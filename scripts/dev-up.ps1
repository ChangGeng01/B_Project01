# F-57：HISTORICAL_WINDOWS_WRAPPER_FOR_LINUX_DEV_ONLY。这是 Windows 开发工作站控制
# Linux 容器开发栈的包装，不是 Windows Server 2022 原生运行、生产启动或发布证据。
# 兼容 Windows PowerShell 5.1 与 pwsh；复用的 compose.yaml 自身是历史 Linux 研究路径。
#
# 退出码：0 成功；64 用法错误；69 无容器引擎；70 起栈/就绪/状态准备失败；72 缺应用镜像。
Set-StrictMode -Version 2.0
$ErrorActionPreference = 'Stop'

$ExitUsage = 64
$ExitNoEngine = 69
$ExitStartFailed = 70
$ExitNoImage = 72

function Show-Usage {
    @'
用法：powershell -File scripts/dev-up.ps1 [--full | --db-only]

  --full     起 PostgreSQL 16 与完整应用栈（默认）。
  --db-only  只起 PostgreSQL 16，供集成测试使用。

退出码：0 成功；64 用法错误；69 无容器引擎；70 起栈、就绪或状态准备失败；72 缺应用镜像。
状态目录由 EP_DEV_STATE_DIR 指定，Windows 默认位于 LocalApplicationData\EP\dev。
'@ | Write-Host
}

$Mode = '--full'
if ($args.Count -gt 1) {
    Show-Usage
    exit $ExitUsage
}
if ($args.Count -eq 1) {
    switch ($args[0]) {
        '--full' { $Mode = '--full' }
        '--db-only' { $Mode = '--db-only' }
        '--help' { Show-Usage; exit 0 }
        '-h' { Show-Usage; exit 0 }
        default { Show-Usage; exit $ExitUsage }
    }
}

$SelfDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = (Resolve-Path (Join-Path $SelfDir '..')).Path
$ComposeFile = Join-Path $RepoRoot 'deploy\compose\compose.yaml'
$QuadletDir = Join-Path $RepoRoot 'deploy\podman'
$Utf8NoBom = New-Object System.Text.UTF8Encoding($false)
. (Join-Path $SelfDir 'dev-state-common.ps1')

if (-not [string]::IsNullOrWhiteSpace($env:EP_DEV_STATE_DIR)) {
    $StateDir = $env:EP_DEV_STATE_DIR
}
elseif (-not [string]::IsNullOrWhiteSpace($env:LOCALAPPDATA)) {
    $StateDir = Join-Path $env:LOCALAPPDATA 'EP\dev'
}
elseif (-not [string]::IsNullOrWhiteSpace($env:USERPROFILE)) {
    $StateDir = Join-Path $env:USERPROFILE '.ep-dev'
}
else {
    [Console]::Error.WriteLine('状态失败  找不到 LocalApplicationData、USERPROFILE 或 EP_DEV_STATE_DIR。')
    exit $ExitStartFailed
}
$UserProfileRoot = [Environment]::GetFolderPath([Environment+SpecialFolder]::UserProfile)
if ([string]::IsNullOrWhiteSpace($UserProfileRoot)) { $UserProfileRoot = $env:USERPROFILE }
if ([string]::IsNullOrWhiteSpace($UserProfileRoot)) {
    [Console]::Error.WriteLine('状态失败  找不到用于保护的用户目录边界。')
    exit $ExitStartFailed
}

$ReadyTimeoutS = 120
if (-not [string]::IsNullOrWhiteSpace($env:EP_DEV_READY_TIMEOUT_S)) {
    $parsedTimeout = 0
    if ((-not [int]::TryParse($env:EP_DEV_READY_TIMEOUT_S, [ref]$parsedTimeout)) -or
        $parsedTimeout -lt 1 -or $parsedTimeout -gt 999999999) {
        [Console]::Error.WriteLine('用法错误  EP_DEV_READY_TIMEOUT_S 必须是 1..999999999 的整数。')
        exit $ExitUsage
    }
    $ReadyTimeoutS = $parsedTimeout
}

$script:ComposeCli = $null
$script:EngineCli = $null
$script:ComposePrefix = @()
$script:ComposeDisplay = $null
$script:LastNativeExit = 0

function Test-NativeCommand([string]$Command, [string[]]$Arguments) {
    & $Command @Arguments *> $null
    return ($LASTEXITCODE -eq 0)
}

function Find-ContainerEngine {
    if ($null -ne (Get-Command docker -ErrorAction SilentlyContinue)) {
        if (Test-NativeCommand 'docker' @('compose', 'version')) {
            $script:EngineCli = 'docker'
            $script:ComposeCli = 'docker'
            $script:ComposePrefix = @('compose')
            $script:ComposeDisplay = 'docker compose'
            return $true
        }
    }
    if ($null -ne (Get-Command podman -ErrorAction SilentlyContinue)) {
        if (Test-NativeCommand 'podman' @('compose', '--help')) {
            $script:EngineCli = 'podman'
            $script:ComposeCli = 'podman'
            $script:ComposePrefix = @('compose')
            $script:ComposeDisplay = 'podman compose'
            return $true
        }
    }
    if (($null -ne (Get-Command podman-compose -ErrorAction SilentlyContinue)) -and
        ($null -ne (Get-Command podman -ErrorAction SilentlyContinue))) {
        $script:EngineCli = 'podman'
        $script:ComposeCli = 'podman-compose'
        $script:ComposePrefix = @()
        $script:ComposeDisplay = 'podman-compose'
        return $true
    }
    return $false
}

function Invoke-Compose([string[]]$Arguments, [switch]$Quiet) {
    $nativeArguments = @($script:ComposePrefix) + @($Arguments)
    if ($Quiet) {
        & $script:ComposeCli @nativeArguments *> $null
    }
    else {
        & $script:ComposeCli @nativeArguments
    }
    $script:LastNativeExit = $LASTEXITCODE
}

function Get-FileSha256([string]$Path) {
    $sha = [Security.Cryptography.SHA256]::Create()
    try {
        $stream = [IO.File]::OpenRead($Path)
        try {
            return ([BitConverter]::ToString($sha.ComputeHash($stream))).Replace('-', '').ToLowerInvariant()
        }
        finally {
            $stream.Dispose()
        }
    }
    finally {
        $sha.Dispose()
    }
}

function Test-VolumePresent {
    & $script:EngineCli volume inspect ep-pgdata *> $null
    return ($LASTEXITCODE -eq 0)
}

function New-DatabasePassword {
    $bytes = New-Object byte[] 16
    $rng = [Security.Cryptography.RandomNumberGenerator]::Create()
    try {
        $rng.GetBytes($bytes)
    }
    finally {
        $rng.Dispose()
    }
    $password = ([BitConverter]::ToString($bytes)).Replace('-', '').ToLowerInvariant()
    if ([Text.Encoding]::ASCII.GetByteCount($password) -ne 32 -or $password -notmatch '^[0-9a-f]{32}$') {
        throw '密码学随机源未产生精确 32 字节安全 ASCII 口令。'
    }
    return $password
}

function Initialize-DevState {
    [void](Initialize-OwnedDevStateRoot $StateDir $RepoRoot $UserProfileRoot)
    $etcDir = Join-Path $StateDir 'etc'
    $secretsDir = Join-Path $StateDir 'secrets'
    foreach ($path in @($StateDir, $etcDir, $secretsDir)) {
        if (Test-ReparsePoint $path) {
            throw "安全状态路径不得是重解析点：$path"
        }
    }
    New-Item -ItemType Directory -Path $etcDir -Force | Out-Null
    New-Item -ItemType Directory -Path $secretsDir -Force | Out-Null
    foreach ($path in @($StateDir, $etcDir, $secretsDir)) {
        if (-not (Protect-SecurePath $StateDir $path $true)) {
            throw "无法把安全状态目录收紧为仅当前用户可访问：$path"
        }
    }

    $env:EP_ETC_DIR = $etcDir
    $env:EP_SECRETS_DIR = $secretsDir
    if ([string]::IsNullOrWhiteSpace($env:EP_IMAGE_PREFIX)) { $env:EP_IMAGE_PREFIX = 'localhost/ep' }
    if ([string]::IsNullOrWhiteSpace($env:EP_IMAGE_TAG)) { $env:EP_IMAGE_TAG = '0.1.0' }

    foreach ($value in @($env:EP_ETC_DIR, $env:EP_SECRETS_DIR, $env:EP_IMAGE_PREFIX, $env:EP_IMAGE_TAG)) {
        if ($value -match "[\r\n]") {
            throw '本地开发环境变量不得含换行符。'
        }
    }

    $envFile = Join-Path $StateDir '.env.dev'
    if (-not (Test-Path -LiteralPath $envFile)) {
        $envText = @(
            "EP_ETC_DIR=$($env:EP_ETC_DIR)"
            "EP_SECRETS_DIR=$($env:EP_SECRETS_DIR)"
            "EP_IMAGE_PREFIX=$($env:EP_IMAGE_PREFIX)"
            "EP_IMAGE_TAG=$($env:EP_IMAGE_TAG)"
        ) -join "`n"
        [IO.File]::WriteAllText($envFile, $envText + "`n", $Utf8NoBom)
        Write-Host "已生成    本地开发环境文件 $envFile"
    }
    if ((Test-ReparsePoint $envFile) -or (-not (Protect-SecurePath $StateDir $envFile $false))) {
        throw '本地开发环境文件不得是重解析点，且必须仅当前用户可访问。'
    }

    $secretPath = Join-Path $secretsDir 'postgres-superuser'
    $bindingPath = Join-Path $secretsDir 'postgres-volume-binding.sha256'
    $hasVolume = Test-VolumePresent
    if (-not (Test-Path -LiteralPath $secretPath)) {
        if ($hasVolume -or (Test-Path -LiteralPath $bindingPath)) {
            throw '数据卷 ep-pgdata 或其绑定记录已存在，但原口令缺失；拒绝生成新口令以免旧库失配。'
        }
        $temporaryPath = "$secretPath.tmp.$PID"
        try {
            $password = New-DatabasePassword
            [IO.File]::WriteAllText($temporaryPath, $password, $Utf8NoBom)
            if (([IO.File]::ReadAllBytes($temporaryPath)).Length -ne 32) {
                throw '生成的数据库口令不是精确 32 字节。'
            }
            if (-not (Protect-SecurePath $StateDir $temporaryPath $false)) {
                throw '无法为数据库口令设置仅当前用户可读写的文件权限。'
            }
            Move-Item -LiteralPath $temporaryPath -Destination $secretPath
        }
        finally {
            if (Test-Path -LiteralPath $temporaryPath) {
                Remove-Item -LiteralPath $temporaryPath -Force
            }
        }
        Write-Host "已生成    开发机数据库超级用户口令 $secretPath"
    }

    $existingSecretBytes = [IO.File]::ReadAllBytes($secretPath)
    $existingSecret = [Text.Encoding]::ASCII.GetString($existingSecretBytes)
    if ($existingSecretBytes.Length -ne 32 -or $existingSecret -notmatch '^[0-9A-Za-z]{32}$') {
        throw '已有数据库口令不是精确 32 个安全 ASCII 字节；为避免数据库失配，不自动改写。'
    }
    if ((Test-ReparsePoint $secretPath) -or (Test-ReparsePoint $bindingPath)) {
        throw '口令与数据卷绑定记录不得是重解析点。'
    }
    if (-not (Protect-SecurePath $StateDir $secretPath $false)) {
        throw '无法为已有数据库口令设置仅当前用户可读写的文件权限。'
    }

    $digest = Get-FileSha256 $secretPath
    $expectedBinding = "sha256:$digest"
    if (Test-Path -LiteralPath $bindingPath) {
        $actualBinding = [IO.File]::ReadAllText($bindingPath, [Text.Encoding]::ASCII)
        if ($actualBinding -ne $expectedBinding) {
            throw '数据库口令与 ep-pgdata 的原绑定不一致。'
        }
    }
    elseif ($hasVolume) {
        throw '数据卷 ep-pgdata 已存在但缺少口令绑定证据；请恢复与该卷匹配的原状态目录。'
    }
    else {
        $temporaryBinding = "$bindingPath.tmp.$PID"
        try {
            [IO.File]::WriteAllText($temporaryBinding, $expectedBinding, $Utf8NoBom)
            if (-not (Protect-SecurePath $StateDir $temporaryBinding $false)) {
                throw '无法保护数据卷与口令绑定记录。'
            }
            Move-Item -LiteralPath $temporaryBinding -Destination $bindingPath
        }
        finally {
            if (Test-Path -LiteralPath $temporaryBinding) {
                Remove-Item -LiteralPath $temporaryBinding -Force
            }
        }
    }
    if (-not (Protect-SecurePath $StateDir $bindingPath $false)) {
        throw '无法把数据卷绑定记录收紧为仅当前用户可访问。'
    }
}

function Test-ImagePresent([string]$Reference) {
    if ($script:EngineCli -eq 'podman') {
        & $script:EngineCli image exists $Reference *> $null
    }
    else {
        & $script:EngineCli image inspect $Reference *> $null
    }
    return ($LASTEXITCODE -eq 0)
}

function Test-AllImagesPresent {
    $missing = 0
    $units = @(Get-ChildItem -LiteralPath $QuadletDir -Filter '*.container' -File)
    foreach ($unit in $units) {
        if ($unit.BaseName -eq 'postgres') { continue }
        $reference = "$($env:EP_IMAGE_PREFIX)/$($unit.BaseName):$($env:EP_IMAGE_TAG)"
        if (-not (Test-ImagePresent $reference)) {
            [Console]::Error.WriteLine("缺镜像    $reference")
            $missing += 1
        }
    }
    if ($missing -gt 0) {
        [Console]::Error.WriteLine("本机缺 $missing 个应用镜像；本脚本不代为构建。只要数据库时使用 --db-only。")
        return $false
    }
    return $true
}

function Wait-PostgresReady {
    $stopwatch = [Diagnostics.Stopwatch]::StartNew()
    Write-Host "等待      PostgreSQL 就绪，最多 $ReadyTimeoutS 秒"
    while ($stopwatch.Elapsed.TotalSeconds -lt $ReadyTimeoutS) {
        & $script:EngineCli exec ep-postgres pg_isready -U postgres *> $null
        if ($LASTEXITCODE -eq 0) {
            $seconds = [Math]::Floor($stopwatch.Elapsed.TotalSeconds)
            Write-Host "已就绪    PostgreSQL 16，等待 $seconds 秒"
            return $true
        }
        Start-Sleep -Seconds 2
    }
    [Console]::Error.WriteLine("未就绪    PostgreSQL 在 $ReadyTimeoutS 秒内没有通过 pg_isready")
    return $false
}

function Get-ComposeServices {
    $nativeArguments = @($script:ComposePrefix) + @('-f', $ComposeFile, 'config', '--services')
    $lines = @(& $script:ComposeCli @nativeArguments 2>$null)
    if ($LASTEXITCODE -ne 0) {
        throw "$script:ComposeDisplay config --services 返回非零。"
    }
    $services = @($lines | ForEach-Object { ([string]$_).Trim() } | Where-Object { $_ -ne '' })
    if ($services.Count -eq 0) { throw 'Compose 没有返回任何必需服务。' }
    foreach ($service in $services) {
        if ($service -notmatch '^[A-Za-z0-9_.-]+$') { throw "Compose 返回非法服务名：$service" }
    }
    return $services
}

# State 为 Ready、Pending 或 Failed。无 healthcheck 的容器以 running 为就绪；
# 有 healthcheck 的容器必须达到 healthy，即统一的 running|healthy 语义。
function Get-RequiredServiceState([string]$Service) {
    $psArguments = @($script:ComposePrefix) + @('-f', $ComposeFile, 'ps', '-a', '-q', $Service)
    $idLines = @(& $script:ComposeCli @psArguments 2>$null)
    if ($LASTEXITCODE -ne 0) {
        return [pscustomobject]@{ State = 'Failed'; Message = "查询必需服务 $Service 的容器失败" }
    }
    $containerId = @($idLines | ForEach-Object { ([string]$_).Trim() } | Where-Object { $_ -ne '' } | Select-Object -First 1)
    if ($containerId.Count -eq 0) {
        return [pscustomobject]@{ State = 'Failed'; Message = "必需服务 $Service 的容器缺失" }
    }

    $selectedContainerId = [string]$containerId[0]
    $inspection = @(& $script:EngineCli inspect --format '{{.State.Status}}|{{if .State.Health}}{{.State.Health.Status}}{{else}}none{{end}}' $selectedContainerId 2>$null)
    if ($LASTEXITCODE -ne 0 -or $inspection.Count -eq 0) {
        return [pscustomobject]@{ State = 'Failed'; Message = "无法读取必需服务 $Service 的容器状态" }
    }
    $parts = ([string]$inspection[0]).Trim() -split '\|', 2
    if ($parts.Count -ne 2) {
        return [pscustomobject]@{ State = 'Failed'; Message = "必需服务 $Service 返回了无法识别的容器状态" }
    }
    $containerState = $parts[0]
    $healthState = $parts[1]
    if ($containerState -eq 'running') {
        if ($healthState -eq 'healthy' -or $healthState -eq 'none') {
            return [pscustomobject]@{ State = 'Ready'; Message = '' }
        }
        if ($healthState -eq 'starting') {
            return [pscustomobject]@{ State = 'Pending'; Message = "$Service 健康检查仍在 starting" }
        }
        return [pscustomobject]@{ State = 'Failed'; Message = "必需服务 $Service 的健康状态为 $healthState" }
    }
    if ($containerState -eq 'created' -or $containerState -eq 'restarting') {
        return [pscustomobject]@{ State = 'Pending'; Message = "$Service 容器状态为 $containerState" }
    }
    return [pscustomobject]@{ State = 'Failed'; Message = "必需服务 $Service 未运行，容器状态为 $containerState" }
}

function Wait-AllRequiredServicesReady([string[]]$Services) {
    $stopwatch = [Diagnostics.Stopwatch]::StartNew()
    Write-Host "等待      必需服务 running/healthy，最多 $ReadyTimeoutS 秒"
    while ($stopwatch.Elapsed.TotalSeconds -lt $ReadyTimeoutS) {
        $pending = $false
        foreach ($service in $Services) {
            $state = Get-RequiredServiceState $service
            if ($state.State -eq 'Failed') {
                [Console]::Error.WriteLine("服务失败  $($state.Message)")
                return $false
            }
            if ($state.State -eq 'Pending') { $pending = $true }
        }
        if (-not $pending) {
            $seconds = [Math]::Floor($stopwatch.Elapsed.TotalSeconds)
            Write-Host "已就绪    全部必需服务均为 running/healthy，等待 $seconds 秒"
            return $true
        }
        Start-Sleep -Seconds 2
    }
    [Console]::Error.WriteLine("未就绪    必需服务在 $ReadyTimeoutS 秒内未全部达到 running/healthy")
    return $false
}

try {
    if (-not (Test-Path -LiteralPath $ComposeFile -PathType Leaf)) {
        [Console]::Error.WriteLine("读不到    $ComposeFile")
        exit $ExitStartFailed
    }
    if ($env:OS -eq 'Windows_NT') {
        $windowsProductType = (Get-CimInstance -ClassName Win32_OperatingSystem).ProductType
        if ([int]$windowsProductType -ne 1) {
            [Console]::Error.WriteLine('拒绝启动  本脚本只是 Windows 开发工作站的 Linux 容器包装，不是 Windows Server 2022 原生运行路径。')
            exit $ExitStartFailed
        }
    }
    # 只读边界校验先于容器引擎探测和任何 New-Item/Set-Acl。
    $StateDir = Resolve-SafeDevStateTarget $StateDir $RepoRoot $UserProfileRoot $true
    if (-not (Find-ContainerEngine)) {
        [Console]::Error.WriteLine('无引擎    本机没有 docker compose、podman compose 或 podman-compose。')
        exit $ExitNoEngine
    }
    Write-Host "引擎      $script:ComposeDisplay"

    Initialize-DevState
    Write-Host "状态目录  $StateDir"

    if ($Mode -eq '--full' -and -not (Test-AllImagesPresent)) {
        exit $ExitNoImage
    }

    $upArguments = @('-f', $ComposeFile, 'up', '-d')
    if ($Mode -eq '--db-only') { $upArguments += 'postgres' }
    Invoke-Compose $upArguments
    if ($script:LastNativeExit -ne 0) {
        [Console]::Error.WriteLine("起栈失败  $script:ComposeDisplay up -d 返回非零")
        exit $ExitStartFailed
    }
    if (-not (Wait-PostgresReady)) { exit $ExitStartFailed }

    if ($Mode -eq '--full') {
        $requiredServices = @(Get-ComposeServices)
    }
    else {
        $requiredServices = @('postgres')
    }
    if (-not (Wait-AllRequiredServicesReady $requiredServices)) { exit $ExitStartFailed }

    Invoke-Compose @('-f', $ComposeFile, 'ps')
    if ($script:LastNativeExit -ne 0) {
        [Console]::Error.WriteLine("状态失败  $script:ComposeDisplay ps 返回非零")
        exit $ExitStartFailed
    }
    if ($Mode -eq '--full') {
        Write-Host "`n全栈已就绪；数据库在 127.0.0.1:5432，超级用户口令见 $(Join-Path $StateDir 'secrets\postgres-superuser')。"
    }
    else {
        Write-Host "`n库已在 127.0.0.1:5432，超级用户口令见 $(Join-Path $StateDir 'secrets\postgres-superuser')。"
    }
    Write-Host '停栈用 powershell -File scripts/dev-down.ps1。'
    exit 0
}
catch {
    [Console]::Error.WriteLine("状态失败  $($_.Exception.Message)")
    exit $ExitStartFailed
}

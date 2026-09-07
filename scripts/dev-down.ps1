# F-57：HISTORICAL_WINDOWS_WRAPPER_FOR_LINUX_DEV_ONLY。只停掉 scripts/dev-up.ps1
# 起的 Linux 容器开发栈，不是 Windows Server 2022 原生生产停机路径。
# 兼容 Windows PowerShell 5.1 与 pwsh。
#
# 退出码：0 成功；64 用法错误；69 无容器引擎；70 停栈失败。
Set-StrictMode -Version 2.0
$ErrorActionPreference = 'Stop'

$ExitUsage = 64
$ExitNoEngine = 69
$ExitFailed = 70

function Show-Usage {
    @'
用法：powershell -File scripts/dev-down.ps1 [--keep-volumes | --purge]

  --keep-volumes  只停容器，命名卷保留（默认）。
  --purge         当前不可用：缺少卷与状态根的来源绑定证据，失败关闭且不访问引擎。

退出码：0 成功；64 用法错误；69 无容器引擎；70 停栈失败。
'@ | Write-Host
}

$Mode = '--keep-volumes'
if ($args.Count -gt 1) {
    Show-Usage
    exit $ExitUsage
}
if ($args.Count -eq 1) {
    switch ($args[0]) {
        '--keep-volumes' { $Mode = '--keep-volumes' }
        '--purge' { $Mode = '--purge' }
        '--help' { Show-Usage; exit 0 }
        '-h' { Show-Usage; exit 0 }
        default { Show-Usage; exit $ExitUsage }
    }
}

if ($Mode -eq '--purge') {
    [Console]::Error.WriteLine('拒绝清卷  尚未交付卷与状态根的来源绑定证据；未访问容器引擎，未删除任何卷。')
    exit $ExitFailed
}

$SelfDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = (Resolve-Path (Join-Path $SelfDir '..')).Path
$ComposeFile = Join-Path $RepoRoot 'deploy\compose\compose.yaml'
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
    exit $ExitFailed
}
$UserProfileRoot = [Environment]::GetFolderPath([Environment+SpecialFolder]::UserProfile)
if ([string]::IsNullOrWhiteSpace($UserProfileRoot)) { $UserProfileRoot = $env:USERPROFILE }
if ([string]::IsNullOrWhiteSpace($UserProfileRoot)) {
    [Console]::Error.WriteLine('状态失败  找不到用于保护的用户目录边界。')
    exit $ExitFailed
}

$script:ComposeCli = $null
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
            $script:ComposeCli = 'docker'
            $script:ComposePrefix = @('compose')
            $script:ComposeDisplay = 'docker compose'
            return $true
        }
    }
    if ($null -ne (Get-Command podman -ErrorAction SilentlyContinue)) {
        if (Test-NativeCommand 'podman' @('compose', '--help')) {
            $script:ComposeCli = 'podman'
            $script:ComposePrefix = @('compose')
            $script:ComposeDisplay = 'podman compose'
            return $true
        }
    }
    if ($null -ne (Get-Command podman-compose -ErrorAction SilentlyContinue)) {
        $script:ComposeCli = 'podman-compose'
        $script:ComposePrefix = @()
        $script:ComposeDisplay = 'podman-compose'
        return $true
    }
    return $false
}

function Invoke-Compose([string[]]$Arguments) {
    $nativeArguments = @($script:ComposePrefix) + @($Arguments)
    & $script:ComposeCli @nativeArguments
    $script:LastNativeExit = $LASTEXITCODE
}

try {
    if (-not (Test-Path -LiteralPath $ComposeFile -PathType Leaf)) {
        [Console]::Error.WriteLine("读不到    $ComposeFile")
        exit $ExitFailed
    }
    if ($env:OS -eq 'Windows_NT') {
        $windowsProductType = (Get-CimInstance -ClassName Win32_OperatingSystem).ProductType
        if ([int]$windowsProductType -ne 1) {
            [Console]::Error.WriteLine('拒绝停机  本脚本只是 Windows 开发工作站的 Linux 容器包装，不是 Windows Server 2022 原生停机路径。')
            exit $ExitFailed
        }
    }
    # 停栈也必须先证明这是本工具已认领的专用目录；错误路径不得触发容器引擎。
    $StateDir = Get-OwnedDevStateRoot $StateDir $RepoRoot $UserProfileRoot
    if (-not (Find-ContainerEngine)) {
        [Console]::Error.WriteLine('无引擎    本机没有 docker compose、podman compose 或 podman-compose。')
        exit $ExitNoEngine
    }

    $env:EP_ETC_DIR = Join-Path $StateDir 'etc'
    $env:EP_SECRETS_DIR = Join-Path $StateDir 'secrets'

    Invoke-Compose @('-f', $ComposeFile, 'down')
    if ($script:LastNativeExit -ne 0) {
        [Console]::Error.WriteLine("停栈失败  $script:ComposeDisplay down 返回非零")
        exit $ExitFailed
    }

    Write-Host '已保留    命名卷，库里的数据还在；当前不提供清卷操作'
    exit 0
}
catch {
    [Console]::Error.WriteLine("停栈失败  $($_.Exception.Message)")
    exit $ExitFailed
}

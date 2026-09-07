# Windows 开发工作站 Linux 容器包装的行为测试。Windows Server 上只验证
# 脚本失败关闭，不把 fake docker 当作原生生产运行证据。
Set-StrictMode -Version 2.0
$ErrorActionPreference = 'Stop'

if ($env:OS -ne 'Windows_NT') {
    Write-Error '本测试需要 Windows PowerShell 5.1 或 Windows 上的 pwsh；当前主机不是 Windows。'
    exit 70
}

$TestDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = (Resolve-Path (Join-Path $TestDir '..\..')).Path
$DevUp = Join-Path $RepoRoot 'scripts\dev-up.ps1'
$DevDown = Join-Path $RepoRoot 'scripts\dev-down.ps1'
$DevStateCommon = Join-Path $RepoRoot 'scripts\dev-state-common.ps1'
. $DevStateCommon
$PowerShellExe = Join-Path $PSHOME 'powershell.exe'
if (-not (Test-Path -LiteralPath $PowerShellExe)) {
    $PowerShellExe = Join-Path $PSHOME 'pwsh.exe'
}
if (-not (Test-Path -LiteralPath $PowerShellExe)) {
    Write-Error '找不到当前 PowerShell 可执行文件。'
    exit 70
}

$TempRoot = Join-Path ([IO.Path]::GetTempPath()) ('ep-dev-controls-' + [Guid]::NewGuid().ToString('N'))
$FakeBin = Join-Path $TempRoot 'bin'
$StateDir = Join-Path $TempRoot 'state'
$EngineLog = Join-Path $TempRoot 'engine.log'
$VolumeMarker = Join-Path $TempRoot 'ep-pgdata.exists'
$Utf8NoBom = New-Object System.Text.UTF8Encoding($false)
$OldPath = $env:PATH
$OldStateDir = $env:EP_DEV_STATE_DIR
$OldEngineLog = $env:EP_TEST_ENGINE_LOG
$OldVolumeMarker = $env:EP_TEST_VOLUME_MARKER
$OldMissingService = $env:EP_TEST_MISSING_SERVICE
$OldExitedService = $env:EP_TEST_EXITED_SERVICE
$OldUnhealthyService = $env:EP_TEST_UNHEALTHY_SERVICE
$OldReadyTimeout = $env:EP_DEV_READY_TIMEOUT_S

function Fail-Test([string]$Message) {
    throw "FAIL: $Message"
}

function Invoke-Target([string]$ScriptPath, [string[]]$ScriptArguments) {
    $lines = @(& $PowerShellExe -NoLogo -NoProfile -ExecutionPolicy Bypass -File $ScriptPath @ScriptArguments 2>&1)
    return @{
        ExitCode = $LASTEXITCODE
        Output = ($lines -join [Environment]::NewLine)
    }
}

function Get-EngineLines {
    if (-not (Test-Path -LiteralPath $EngineLog)) {
        return @()
    }
    return @(Get-Content -LiteralPath $EngineLog)
}

function Assert-ExclusiveAcl([string]$Path) {
    $currentSid = [Security.Principal.WindowsIdentity]::GetCurrent().User
    $acl = Get-Acl -LiteralPath $Path
    $rules = @($acl.Access)
    if (-not $acl.AreAccessRulesProtected) { Fail-Test "$Path 仍继承 ACL" }
    if ($rules.Count -ne 1) { Fail-Test "$Path 显式 ACL 不是唯一当前用户规则" }
    $ruleSid = $rules[0].IdentityReference.Translate([Security.Principal.SecurityIdentifier])
    if ($ruleSid.Value -ne $currentSid.Value -or $rules[0].IsInherited -or
        $rules[0].AccessControlType -ne [Security.AccessControl.AccessControlType]::Allow) {
        Fail-Test "$Path ACL 未收敛到当前用户"
    }
}

try {
    New-Item -ItemType Directory -Path $FakeBin -Force | Out-Null
    [IO.File]::WriteAllText($EngineLog, '', (New-Object Text.UTF8Encoding($false)))
    $FakeDocker = @'
@echo off
if "%1"=="inspect" goto inspect_container
>>"%EP_TEST_ENGINE_LOG%" echo %*
if "%1 %2 %3"=="volume inspect ep-pgdata" goto volume_check
if "%1 %2"=="compose version" exit /b 0
echo %* | findstr /c:" config --services" >nul && goto compose_services
echo %* | findstr /c:" ps -a -q " >nul && goto compose_ps
if "%1"=="compose" if "%4"=="up" type nul >"%EP_TEST_VOLUME_MARKER%"
if "%1"=="compose" if "%4"=="down" echo %* | findstr /c:"--volumes" >nul && del /q "%EP_TEST_VOLUME_MARKER%" 2>nul
exit /b 0
:volume_check
if exist "%EP_TEST_VOLUME_MARKER%" exit /b 0
exit /b 1
:compose_services
for %%s in (postgres core-server job-worker portal-gateway integration-gateway plugin-host ops-agent archive-writer backup-writer) do echo %%s
exit /b 0
:compose_ps
for %%s in (%*) do set service=%%s
if "%EP_TEST_MISSING_SERVICE%"=="%service%" exit /b 0
echo ep-%service%
exit /b 0
:inspect_container
set container=%4
set service=%container:ep-=%
if "%EP_TEST_EXITED_SERVICE%"=="%service%" goto inspect_exited
if "%EP_TEST_UNHEALTHY_SERVICE%"=="%service%" goto inspect_unhealthy
if "%service%"=="postgres" goto inspect_healthy
echo running^|none
exit /b 0
:inspect_exited
echo exited^|none
exit /b 0
:inspect_unhealthy
echo running^|unhealthy
exit /b 0
:inspect_healthy
echo running^|healthy
exit /b 0
'@
    [IO.File]::WriteAllText((Join-Path $FakeBin 'docker.cmd'), $FakeDocker, (New-Object Text.ASCIIEncoding))

    $env:PATH = "$FakeBin;$OldPath"
    $env:EP_DEV_STATE_DIR = $StateDir
    $env:EP_TEST_ENGINE_LOG = $EngineLog
    $env:EP_TEST_VOLUME_MARKER = $VolumeMarker

    $uncRejected = $false
    try {
        [void](Resolve-SafeDevStateTarget '\\server\share\ep-dev' $RepoRoot $env:USERPROFILE $true)
    }
    catch {
        $uncRejected = ($_.Exception.Message -match 'UNC|网络共享|本地磁盘')
    }
    if (-not $uncRejected) {
        Fail-Test 'EP_DEV_STATE_DIR 指向 UNC/网络共享子路径时必须在任何文件或引擎副作用前失败关闭'
    }

    $productType = [int](Get-CimInstance -ClassName Win32_OperatingSystem).ProductType
    if ($productType -ne 1) {
        $result = Invoke-Target $DevUp @('--db-only')
        if ($result.ExitCode -ne 70 -or $result.Output -notmatch 'Windows Server 2022') {
            Fail-Test 'Windows Server 必须拒绝把 Linux 容器开发包装当作原生运行路径'
        }
        Write-Host 'PASS: Windows Server 对历史 Linux 容器开发包装失败关闭。'
        exit 0
    }

    $result = Invoke-Target $DevUp @('--help')
    if ($result.ExitCode -ne 0) { Fail-Test "dev-up.ps1 --help 应返回 0，实际 $($result.ExitCode)" }
    if ((Get-EngineLines).Count -ne 0) { Fail-Test '--help 不应调用容器引擎' }

    $result = Invoke-Target $DevUp @('--unknown')
    if ($result.ExitCode -ne 64) { Fail-Test "未知 up 参数应返回 64，实际 $($result.ExitCode)" }

    [IO.File]::WriteAllText($EngineLog, '', (New-Object Text.UTF8Encoding($false)))
    $env:EP_DEV_READY_TIMEOUT_S = 'not-a-number'
    $result = Invoke-Target $DevUp @('--db-only')
    $env:EP_DEV_READY_TIMEOUT_S = $null
    if ($result.ExitCode -ne 64) { Fail-Test "非法就绪超时应返回 64，实际 $($result.ExitCode)" }
    if ((Get-EngineLines).Count -ne 0) { Fail-Test '非法就绪超时不得调用容器引擎' }

    $unownedState = Join-Path $TempRoot 'unowned-nonempty-state'
    New-Item -ItemType Directory -Path $unownedState | Out-Null
    [IO.File]::WriteAllText((Join-Path $unownedState 'user-file'), 'not an EP state directory', $Utf8NoBom)
    $env:EP_DEV_STATE_DIR = $unownedState
    [IO.File]::WriteAllText($EngineLog, '', (New-Object Text.UTF8Encoding($false)))
    $result = Invoke-Target $DevUp @('--db-only')
    if ($result.ExitCode -ne 70) { Fail-Test '既有非空且无标记目录必须失败关闭' }
    if ((Get-EngineLines).Count -ne 0) { Fail-Test '无标记目录在校验前调用了容器引擎' }
    if (-not (Test-Path -LiteralPath (Join-Path $unownedState 'user-file'))) {
        Fail-Test '拒绝无标记目录时不得清理用户原文件'
    }
    $env:EP_DEV_STATE_DIR = $StateDir

    $result = Invoke-Target $DevUp @('--db-only')
    if ($result.ExitCode -ne 0) { Fail-Test "dev-up.ps1 --db-only 返回 $($result.ExitCode)：$($result.Output)" }
    $secretPath = Join-Path $StateDir 'secrets\postgres-superuser'
    if (-not (Test-Path -LiteralPath $secretPath)) { Fail-Test '未生成数据库口令' }
    $secretBytes = [IO.File]::ReadAllBytes($secretPath)
    if ($secretBytes.Length -ne 32) { Fail-Test "口令必须精确 32 字节，实际 $($secretBytes.Length)" }
    $secret = [Text.Encoding]::ASCII.GetString($secretBytes)
    if ($secret -notmatch '^[0-9a-f]{32}$') { Fail-Test '口令必须是 32 个安全 ASCII 字符' }
    if (-not (Test-Path -LiteralPath (Join-Path $StateDir '.env.dev'))) { Fail-Test '首次启动未生成 .env.dev' }
    $bindingPath = Join-Path $StateDir 'secrets\postgres-volume-binding.sha256'
    if (-not (Test-Path -LiteralPath $bindingPath)) { Fail-Test '未生成数据卷与口令绑定记录' }
    if ([IO.File]::ReadAllText($bindingPath) -notmatch '^sha256:[0-9a-f]{64}$') {
        Fail-Test '数据卷绑定记录形态不正确'
    }
    $ownershipMarker = Join-Path $StateDir '.ep-dev-state-owner-v1'
    if (-not (Test-Path -LiteralPath $ownershipMarker -PathType Leaf)) {
        Fail-Test '首次启动未生成状态目录所有权标记'
    }
    if ([IO.File]::ReadAllText($ownershipMarker, [Text.Encoding]::UTF8) -ne 'enterprise-platform-dev-state-v1') {
        Fail-Test '状态目录所有权标记内容不正确'
    }
    Assert-ExclusiveAcl $StateDir
    Assert-ExclusiveAcl $ownershipMarker
    Assert-ExclusiveAcl (Join-Path $StateDir 'secrets')
    Assert-ExclusiveAcl $secretPath
    Assert-ExclusiveAcl $bindingPath

    $dbLines = Get-EngineLines
    if (-not ($dbLines -match 'compose -f .*compose\.yaml up -d postgres$')) { Fail-Test 'db-only 未只启动 postgres' }
    if (-not ($dbLines -match '^exec ep-postgres pg_isready -U postgres$')) { Fail-Test '未执行 PostgreSQL 就绪探测' }
    if (-not ($dbLines -match 'compose -f .*compose\.yaml ps$')) { Fail-Test '未输出 Compose 状态' }

    $secretBefore = [Convert]::ToBase64String($secretBytes)
    [IO.File]::WriteAllText($EngineLog, '', (New-Object Text.UTF8Encoding($false)))
    $result = Invoke-Target $DevUp @('--full')
    if ($result.ExitCode -ne 0) { Fail-Test "dev-up.ps1 --full 返回 $($result.ExitCode)：$($result.Output)" }
    $fullLines = Get-EngineLines
    if (-not ($fullLines -match '^image inspect ')) { Fail-Test 'full 模式未校验应用镜像' }
    if (-not ($fullLines -match 'compose -f .*compose\.yaml up -d$')) { Fail-Test 'full 模式未启动完整栈' }
    if ($fullLines -match 'compose -f .*compose\.yaml up -d postgres$') { Fail-Test 'full 模式被错误降成 db-only' }
    $secretAfter = [Convert]::ToBase64String([IO.File]::ReadAllBytes($secretPath))
    if ($secretBefore -ne $secretAfter) { Fail-Test '重复启动改写了已有口令' }

    [IO.File]::WriteAllText($EngineLog, '', (New-Object Text.UTF8Encoding($false)))
    $env:EP_TEST_MISSING_SERVICE = 'core-server'
    $result = Invoke-Target $DevUp @('--full')
    $env:EP_TEST_MISSING_SERVICE = $null
    if ($result.ExitCode -ne 70 -or $result.Output -notmatch 'core-server') {
        Fail-Test 'full 模式缺少应用容器时必须指出服务并返回 70'
    }
    if ($result.Output -match '全栈已就绪|库已在') { Fail-Test 'full 模式缺少应用容器时不得打印成功信息' }

    [IO.File]::WriteAllText($EngineLog, '', (New-Object Text.UTF8Encoding($false)))
    $env:EP_TEST_UNHEALTHY_SERVICE = 'job-worker'
    $result = Invoke-Target $DevUp @('--full')
    $env:EP_TEST_UNHEALTHY_SERVICE = $null
    if ($result.ExitCode -ne 70 -or $result.Output -notmatch 'job-worker') {
        Fail-Test 'full 模式应用容器不健康时必须指出服务并返回 70'
    }
    if ($result.Output -match '全栈已就绪|库已在') { Fail-Test 'full 模式应用容器不健康时不得打印成功信息' }

    [IO.File]::WriteAllText($EngineLog, '', (New-Object Text.UTF8Encoding($false)))
    $env:EP_TEST_EXITED_SERVICE = 'portal-gateway'
    $result = Invoke-Target $DevUp @('--full')
    $env:EP_TEST_EXITED_SERVICE = $null
    if ($result.ExitCode -ne 70 -or $result.Output -notmatch 'portal-gateway') {
        Fail-Test 'full 模式应用容器已退出时必须指出服务并返回 70'
    }
    if ($result.Output -match '全栈已就绪|库已在') { Fail-Test 'full 模式应用容器已退出时不得打印成功信息' }

    $switchedState = Join-Path $TempRoot 'switched-state'
    $env:EP_DEV_STATE_DIR = $switchedState
    $result = Invoke-Target $DevUp @('--db-only')
    if ($result.ExitCode -ne 70) { Fail-Test '已有数据卷时切换无原口令的状态目录必须返回 70' }
    if (Test-Path -LiteralPath (Join-Path $switchedState 'secrets\postgres-superuser')) {
        Fail-Test '已有数据卷时不得生成失配口令'
    }
    $env:EP_DEV_STATE_DIR = $StateDir

    [IO.File]::WriteAllText($EngineLog, '', (New-Object Text.UTF8Encoding($false)))
    $result = Invoke-Target $DevDown @()
    if ($result.ExitCode -ne 0) { Fail-Test "dev-down.ps1 默认停止返回 $($result.ExitCode)：$($result.Output)" }
    $downLines = Get-EngineLines
    if (-not ($downLines -match 'compose -f .*compose\.yaml down$')) { Fail-Test '默认停止未执行 down' }
    if ($downLines -match '--volumes') { Fail-Test '默认停止不应删除卷' }

    [IO.File]::WriteAllText($EngineLog, '', (New-Object Text.UTF8Encoding($false)))
    [IO.File]::WriteAllText($VolumeMarker, 'preserve', $Utf8NoBom)
    $result = Invoke-Target $DevDown @('--purge')
    if ($result.ExitCode -ne 70 -or $result.Output -notmatch '来源绑定') {
        Fail-Test "未有来源绑定证据时 --purge 必须失败关闭并返回 70：$($result.Output)"
    }
    if ((Get-EngineLines).Count -ne 0) { Fail-Test '--purge 失败关闭前不得探测或调用容器引擎' }
    if (-not (Test-Path -LiteralPath $VolumeMarker -PathType Leaf)) { Fail-Test '--purge 失败关闭时必须保留数据卷标记' }

    foreach ($misleadingArgs in @(@('--purge=force'), @('--purge', '--keep-volumes'), @('--keep-volumes', '--purge'))) {
        [IO.File]::WriteAllText($EngineLog, '', (New-Object Text.UTF8Encoding($false)))
        $result = Invoke-Target $DevDown $misleadingArgs
        if ($result.ExitCode -ne 64) { Fail-Test "误导/额外 purge 选项必须按用法错误返回 64：$($misleadingArgs -join ' ')" }
        if ((Get-EngineLines).Count -ne 0) { Fail-Test "误导/额外 purge 选项不得触发引擎访问：$($misleadingArgs -join ' ')" }
        if (-not (Test-Path -LiteralPath $VolumeMarker -PathType Leaf)) { Fail-Test '误导/额外 purge 选项不得删除数据卷标记' }
    }

    [IO.File]::WriteAllText($ownershipMarker, 'tampered', (New-Object Text.UTF8Encoding($false)))
    [IO.File]::WriteAllText($EngineLog, '', (New-Object Text.UTF8Encoding($false)))
    $result = Invoke-Target $DevDown @('--keep-volumes')
    if ($result.ExitCode -ne 70) { Fail-Test '标记被篡改后 down 必须失败关闭' }
    if ((Get-EngineLines).Count -ne 0) { Fail-Test '标记被篡改后 down 不得调用容器引擎' }

    Write-Host 'PASS: Windows 本地开发控制脚本未启动容器，参数、口令、状态与卷保留语义均通过。'
    exit 0
}
catch {
    Write-Error $_
    exit 1
}
finally {
    $env:PATH = $OldPath
    $env:EP_DEV_STATE_DIR = $OldStateDir
    $env:EP_TEST_ENGINE_LOG = $OldEngineLog
    $env:EP_TEST_VOLUME_MARKER = $OldVolumeMarker
    $env:EP_TEST_MISSING_SERVICE = $OldMissingService
    $env:EP_TEST_EXITED_SERVICE = $OldExitedService
    $env:EP_TEST_UNHEALTHY_SERVICE = $OldUnhealthyService
    $env:EP_DEV_READY_TIMEOUT_S = $OldReadyTimeout
    if (Test-Path -LiteralPath $TempRoot) {
        Remove-Item -LiteralPath $TempRoot -Recurse -Force
    }
}

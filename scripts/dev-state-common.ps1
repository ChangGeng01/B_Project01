# dev-up.ps1 与 dev-down.ps1 共用的本地开发状态目录安全边界。
# 调用方必须在容器引擎探测及任何 ACL/清理副作用之前完成只读校验。

$script:DevStateMarkerName = '.ep-dev-state-owner-v1'
$script:DevStateMarkerValue = 'enterprise-platform-dev-state-v1'

function Get-NormalizedAbsolutePath([string]$Path, [string]$Label) {
    if ([string]::IsNullOrWhiteSpace($Path) -or $Path -match "[\r\n]") {
        throw "$Label 必须是非空且不含换行符的绝对路径。"
    }
    if (-not [IO.Path]::IsPathRooted($Path) -or $Path -match '^[A-Za-z]:[^\\/]') {
        throw "$Label 必须是绝对路径：$Path"
    }
    $full = [IO.Path]::GetFullPath($Path)
    $root = [IO.Path]::GetPathRoot($full)
    if ($env:OS -eq 'Windows_NT') {
        # 开发状态含数据库超级用户口令。UNC（含扩展 UNC）即使 ACL 可收紧，
        # 仍会把机密与状态流量送出本机；映射盘也必须按真实 DriveType 拒绝。
        if ($full.StartsWith('\\', [StringComparison]::Ordinal) -or
            $root.StartsWith('\\', [StringComparison]::Ordinal)) {
            throw "$Label 必须位于本地磁盘，不得使用 UNC 或网络共享路径：$full"
        }
        $driveType = ([IO.DriveInfo]::new($root)).DriveType
        if ($driveType -eq [IO.DriveType]::Network) {
            throw "$Label 必须位于本地磁盘，不得使用映射网络驱动器：$full"
        }
    }
    if ($full.Length -gt $root.Length) {
        $full = $full.TrimEnd([char[]]@([IO.Path]::DirectorySeparatorChar, [IO.Path]::AltDirectorySeparatorChar))
    }
    return $full
}

function Test-PathSameOrChild([string]$Candidate, [string]$Base) {
    if ([string]::Equals($Candidate, $Base, [StringComparison]::OrdinalIgnoreCase)) { return $true }
    $separator = [IO.Path]::DirectorySeparatorChar
    $prefix = $Base.TrimEnd([char[]]@([IO.Path]::DirectorySeparatorChar, [IO.Path]::AltDirectorySeparatorChar)) + $separator
    return $Candidate.StartsWith($prefix, [StringComparison]::OrdinalIgnoreCase)
}

function Assert-NoReparsePathComponents([string]$Path) {
    $root = [IO.Path]::GetPathRoot($Path)
    $current = $root
    $relative = $Path.Substring($root.Length)
    $separators = [char[]]@([IO.Path]::DirectorySeparatorChar, [IO.Path]::AltDirectorySeparatorChar)
    foreach ($component in $relative.Split($separators, [StringSplitOptions]::RemoveEmptyEntries)) {
        $current = Join-Path $current $component
        if (Test-Path -LiteralPath $current) {
            $item = Get-Item -LiteralPath $current -Force
            if (($item.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
                throw "状态路径的任何组成部分都不得是重解析点：$current"
            }
        }
    }
}

function Test-ReparsePoint([string]$Path) {
    if (-not (Test-Path -LiteralPath $Path)) { return $false }
    $item = Get-Item -LiteralPath $Path -Force
    return (($item.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0)
}

function Test-ExclusiveCurrentUserAcl([string]$Path) {
    if ($env:OS -ne 'Windows_NT') { return $true }
    try {
        $sid = [Security.Principal.WindowsIdentity]::GetCurrent().User
        $acl = Get-Acl -LiteralPath $Path
        $ownerSid = $acl.GetOwner([Security.Principal.SecurityIdentifier])
        $rules = @($acl.Access)
        if (-not $acl.AreAccessRulesProtected -or $rules.Count -ne 1) { return $false }
        $ruleSid = $rules[0].IdentityReference.Translate([Security.Principal.SecurityIdentifier])
        return (($ownerSid.Value -eq $sid.Value) -and ($ruleSid.Value -eq $sid.Value) -and
            (-not $rules[0].IsInherited) -and
            ($rules[0].AccessControlType -eq [Security.AccessControl.AccessControlType]::Allow))
    }
    catch {
        return $false
    }
}

function Test-DevStateMarkerPresent([string]$StateDir) {
    $marker = Join-Path $StateDir $script:DevStateMarkerName
    if (-not (Test-Path -LiteralPath $marker -PathType Leaf) -or (Test-ReparsePoint $marker)) { return $false }
    try {
        $bytes = [IO.File]::ReadAllBytes($marker)
        $expected = [Text.Encoding]::UTF8.GetBytes($script:DevStateMarkerValue)
        if ($bytes.Length -ne $expected.Length) { return $false }
        for ($index = 0; $index -lt $expected.Length; $index += 1) {
            if ($bytes[$index] -ne $expected[$index]) { return $false }
        }
        return $true
    }
    catch {
        return $false
    }
}

function Resolve-SafeDevStateTarget(
    [string]$RawPath,
    [string]$RepoRoot,
    [string]$UserProfile,
    [bool]$AllowUnclaimed
) {
    $state = Get-NormalizedAbsolutePath $RawPath 'EP_DEV_STATE_DIR'
    $repo = Get-NormalizedAbsolutePath $RepoRoot '仓库目录'
    $profile = Get-NormalizedAbsolutePath $UserProfile '用户目录'
    $pathRoot = [IO.Path]::GetPathRoot($state)
    if ([string]::Equals($state, $pathRoot, [StringComparison]::OrdinalIgnoreCase)) {
        throw 'EP_DEV_STATE_DIR 不得是文件系统或共享根目录。'
    }
    if ((Test-PathSameOrChild $state $repo) -or (Test-PathSameOrChild $repo $state)) {
        throw "EP_DEV_STATE_DIR 不得是仓库、仓库子目录或仓库祖先：$state"
    }
    if ([string]::Equals($state, $profile, [StringComparison]::OrdinalIgnoreCase) -or
        (Test-PathSameOrChild $profile $state)) {
        throw "EP_DEV_STATE_DIR 不得是用户目录或其祖先：$state"
    }

    $systemRoots = @()
    $trustedSystemRoots = @()
    if ($env:OS -eq 'Windows_NT') {
        $trustedSystemRoots = @(
            [Environment]::GetFolderPath([Environment+SpecialFolder]::Windows),
            [Environment]::GetFolderPath([Environment+SpecialFolder]::ProgramFiles),
            [Environment]::GetFolderPath([Environment+SpecialFolder]::ProgramFilesX86),
            [Environment]::GetFolderPath([Environment+SpecialFolder]::CommonApplicationData)
        )
    }
    $systemCandidates = @($trustedSystemRoots) + @(
        $env:SystemRoot,
        $env:ProgramFiles,
        ${env:ProgramFiles(x86)},
        $env:ProgramData
    )
    foreach ($candidate in $systemCandidates) {
        if (-not [string]::IsNullOrWhiteSpace($candidate)) {
            $systemRoots += Get-NormalizedAbsolutePath $candidate '系统目录'
        }
    }
    foreach ($protected in $systemRoots) {
        if (Test-PathSameOrChild $state $protected) {
            throw "EP_DEV_STATE_DIR 不得位于系统目录：$state"
        }
    }

    Assert-NoReparsePathComponents $state
    if (Test-Path -LiteralPath $state) {
        if (-not (Test-Path -LiteralPath $state -PathType Container) -or (Test-ReparsePoint $state)) {
            throw "状态路径必须是真实目录：$state"
        }
        if (Test-DevStateMarkerPresent $state) {
            if (-not (Test-ExclusiveCurrentUserAcl $state) -or
                -not (Test-ExclusiveCurrentUserAcl (Join-Path $state $script:DevStateMarkerName))) {
                throw "带标记状态目录及标记必须已经仅当前用户可访问：$state"
            }
        }
        elseif (-not $AllowUnclaimed) {
            throw "状态目录缺少可信所有权标记：$state"
        }
        else {
            # Windows 不自动认领任何既有目录；只有本工具本次创建的目录才能获得首个标记。
            throw "拒绝认领既有且无所有权标记的目录：$state"
        }
    }
    elseif (-not $AllowUnclaimed) {
        throw "状态目录不存在或缺少可信所有权标记：$state"
    }
    return $state
}

function Protect-SecurePath([string]$StateDir, [string]$Path, [bool]$IsDirectory) {
    $state = Get-NormalizedAbsolutePath $StateDir '状态目录'
    $target = Get-NormalizedAbsolutePath $Path '安全路径'
    if (-not (Test-PathSameOrChild $target $state) -or -not (Test-DevStateMarkerPresent $state)) {
        return $false
    }
    Assert-NoReparsePathComponents $target
    if ($env:OS -eq 'Windows_NT') {
        try {
            $sid = [Security.Principal.WindowsIdentity]::GetCurrent().User
            $acl = Get-Acl -LiteralPath $target
            $acl.SetAccessRuleProtection($true, $false)
            foreach ($existingRule in @($acl.Access)) {
                [void]$acl.RemoveAccessRuleAll($existingRule)
            }
            if ($IsDirectory) {
                $inheritance = [Security.AccessControl.InheritanceFlags]::ContainerInherit -bor
                    [Security.AccessControl.InheritanceFlags]::ObjectInherit
                $rule = [Security.AccessControl.FileSystemAccessRule]::new(
                    $sid,
                    [Security.AccessControl.FileSystemRights]::FullControl,
                    $inheritance,
                    [Security.AccessControl.PropagationFlags]::None,
                    [Security.AccessControl.AccessControlType]::Allow
                )
            }
            else {
                $rule = [Security.AccessControl.FileSystemAccessRule]::new(
                    $sid,
                    [Security.AccessControl.FileSystemRights]::FullControl,
                    [Security.AccessControl.AccessControlType]::Allow
                )
            }
            [void]$acl.AddAccessRule($rule)
            Set-Acl -LiteralPath $target -AclObject $acl
            return (Test-ExclusiveCurrentUserAcl $target)
        }
        catch {
            return $false
        }
    }
    if ($null -eq (Get-Command chmod -ErrorAction SilentlyContinue)) { return $false }
    $mode = if ($IsDirectory) { '700' } else { '600' }
    & chmod $mode $target *> $null
    return ($LASTEXITCODE -eq 0)
}

function Initialize-OwnedDevStateRoot([string]$StateDir, [string]$RepoRoot, [string]$UserProfile) {
    $state = Get-NormalizedAbsolutePath $StateDir 'EP_DEV_STATE_DIR'
    $alreadyExists = Test-Path -LiteralPath $state
    if ($alreadyExists) {
        # 已有路径只能通过完整标记和 ACL 校验；不做自动迁移或“顺便”收紧。
        return (Resolve-SafeDevStateTarget $state $RepoRoot $UserProfile $false)
    }

    [void](Resolve-SafeDevStateTarget $state $RepoRoot $UserProfile $true)
    New-Item -ItemType Directory -Path $state -Force | Out-Null
    Assert-NoReparsePathComponents $state
    if (-not (Test-Path -LiteralPath $state -PathType Container) -or (Test-ReparsePoint $state)) {
        throw "无法安全创建状态目录：$state"
    }

    $marker = Join-Path $state $script:DevStateMarkerName
    $stream = $null
    try {
        $stream = [IO.File]::Open($marker, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::None)
        $bytes = [Text.Encoding]::UTF8.GetBytes($script:DevStateMarkerValue)
        $stream.Write($bytes, 0, $bytes.Length)
        $stream.Flush()
    }
    finally {
        if ($null -ne $stream) { $stream.Dispose() }
    }
    if (-not (Test-DevStateMarkerPresent $state)) {
        throw "无法创建状态目录所有权标记：$marker"
    }

    # 标记已原子落盘且内容通过校验后，才允许首次重写 ACL。
    if (-not (Protect-SecurePath $state $state $true) -or
        -not (Protect-SecurePath $state $marker $false)) {
        throw "无法把新状态目录及其所有权标记收紧为仅当前用户可访问：$state"
    }
    return (Resolve-SafeDevStateTarget $state $RepoRoot $UserProfile $false)
}

function Get-OwnedDevStateRoot([string]$StateDir, [string]$RepoRoot, [string]$UserProfile) {
    return (Resolve-SafeDevStateTarget $StateDir $RepoRoot $UserProfile $false)
}

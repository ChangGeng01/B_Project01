#!/usr/bin/env bash
# 本地开发控制脚本的无容器行为测试。所有引擎调用都落到临时 fake，不会启动真实容器。
set -euo pipefail

SELF_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$SELF_DIR/../.." && pwd)
TMP_ROOT=$(mktemp -d "${TMPDIR:-/tmp}/ep-dev-controls.XXXXXX")
TMP_ROOT=$(cd "$TMP_ROOT" && pwd -P)
trap 'rm -rf "$TMP_ROOT"' EXIT

FAKE_BIN=$TMP_ROOT/bin
UNSAFE_BIN=$TMP_ROOT/unsafe-bin
STATE_DIR=$TMP_ROOT/state
ENGINE_LOG=$TMP_ROOT/engine.log
CHMOD_LOG=$TMP_ROOT/chmod.log
VOLUME_MARKER=$TMP_ROOT/ep-pgdata.exists
mkdir -p "$FAKE_BIN" "$UNSAFE_BIN"
: >"$ENGINE_LOG"
: >"$CHMOD_LOG"

# 旧实现固定取一次 512 字节再过滤；这里让那次读取只得到 1 个安全字符，证明它会静默写短口令。
# 修复后的实现改用 od 并循环到精确长度；fake od 每次只给一个字节，强制走满循环分支。
cat >"$FAKE_BIN/head" <<'EOF'
#!/bin/sh
if [ "$1" = "-c" ] && [ "$2" = "512" ] && [ "${3-}" = "/dev/urandom" ]; then
	printf 'a'
	exit 0
fi
exec /usr/bin/head "$@"
EOF

cat >"$FAKE_BIN/od" <<'EOF'
#!/bin/sh
[ "${EP_TEST_OD_EMPTY:-0}" = 1 ] && exit 0
printf 'ab\n'
EOF

cat >"$FAKE_BIN/docker" <<'EOF'
#!/bin/sh
printf '%s\n' "$*" >>"$EP_TEST_ENGINE_LOG"
[ "${EP_TEST_CAPTURE_ENV:-0}" = 1 ] && printf 'env EP_ETC_DIR=%s EP_SECRETS_DIR=%s\n' "${EP_ETC_DIR-}" "${EP_SECRETS_DIR-}" >>"$EP_TEST_ENGINE_LOG"
[ "${EP_TEST_DISABLE_ENGINE:-0}" = 1 ] && exit 1
if [ "${1-} ${2-} ${3-}" = "volume inspect ep-pgdata" ]; then
	[ -e "$EP_TEST_VOLUME_MARKER" ]
	exit $?
fi
if [ "${1-} ${2-}" = "compose version" ]; then
	exit 0
fi
case " $* " in
*" compose "*" config --services "*)
	printf '%s\n' postgres core-server job-worker portal-gateway integration-gateway plugin-host ops-agent archive-writer backup-writer
	exit 0
	;;
*" compose "*" ps -a -q "*)
	service=${!#}
	[ "${EP_TEST_MISSING_SERVICE:-}" = "$service" ] || printf 'ep-%s\n' "$service"
	exit 0
	;;
esac
if [ "${1-}" = inspect ] && [ "${2-}" = --format ]; then
	container=${4-}
	service=${container#ep-}
	if [ "${EP_TEST_EXITED_SERVICE:-}" = "$service" ]; then
		printf 'exited|none\n'
	elif [ "${EP_TEST_UNHEALTHY_SERVICE:-}" = "$service" ]; then
		printf 'running|unhealthy\n'
	elif [ "$service" = postgres ]; then
		printf 'running|healthy\n'
	else
		printf 'running|none\n'
	fi
	exit 0
fi
case " $* " in
*" compose "*" up "*) : >"$EP_TEST_VOLUME_MARKER" ;;
*" compose "*" down "*" --volumes "*) rm -f "$EP_TEST_VOLUME_MARKER" ;;
esac
exit 0
EOF
chmod +x "$FAKE_BIN/head" "$FAKE_BIN/od" "$FAKE_BIN/docker"

# 危险路径负测不能真的创建目录或改权限。这两个替身只记录调用；如果目标脚本在
# 路径校验前触碰任一副作用，测试就会失败。
cat >"$UNSAFE_BIN/mkdir" <<'EOF'
#!/bin/sh
printf '%s\n' "$*" >>"$EP_TEST_CHMOD_LOG"
exit 0
EOF

cat >"$UNSAFE_BIN/chmod" <<'EOF'
#!/bin/sh
printf '%s\n' "$*" >>"$EP_TEST_CHMOD_LOG"
exit 0
EOF
chmod +x "$UNSAFE_BIN/mkdir" "$UNSAFE_BIN/chmod"

fail() {
	printf 'FAIL: %s\n' "$*" >&2
	exit 1
}

assert_log_line() {
	local expected=$1
	grep -Fqx -- "$expected" "$ENGINE_LOG" || fail "引擎调用缺少：$expected"
}

assert_source_order() {
	local file=$1 first=$2 second=$3 label=$4 first_line second_line
	first_line=$(LC_ALL=C grep -nF -- "$first" "$file" | head -n 1 | cut -d: -f1)
	second_line=$(LC_ALL=C grep -nF -- "$second" "$file" | head -n 1 | cut -d: -f1)
	[ -n "$first_line" ] && [ -n "$second_line" ] || fail "$label 缺少静态顺序锚点"
	[ "$first_line" -lt "$second_line" ] || fail "$label 的安全校验没有位于副作用之前"
}

run_up() {
	PATH="$FAKE_BIN:$PATH" \
		EP_DEV_STATE_DIR="$STATE_DIR" \
		EP_TEST_ENGINE_LOG="$ENGINE_LOG" \
		EP_TEST_VOLUME_MARKER="$VOLUME_MARKER" \
		bash "$REPO_ROOT/scripts/dev-up.sh" "$@"
}

run_down() {
	PATH="$FAKE_BIN:$PATH" \
		EP_DEV_STATE_DIR="$STATE_DIR" \
		EP_TEST_ENGINE_LOG="$ENGINE_LOG" \
		EP_TEST_VOLUME_MARKER="$VOLUME_MARKER" \
		bash "$REPO_ROOT/scripts/dev-down.sh" "$@"
}

assert_unsafe_state_rejected() {
	local label=$1 script=$2 state=$3 mode=$4 rc
	: >"$ENGINE_LOG"
	: >"$CHMOD_LOG"
	set +e
	PATH="$UNSAFE_BIN:$FAKE_BIN:$PATH" \
		EP_DEV_STATE_DIR="$state" \
		EP_TEST_ENGINE_LOG="$ENGINE_LOG" \
		EP_TEST_CHMOD_LOG="$CHMOD_LOG" \
		EP_TEST_VOLUME_MARKER="$VOLUME_MARKER" \
		bash "$script" "$mode" >"$TMP_ROOT/unsafe.out" 2>"$TMP_ROOT/unsafe.err"
	rc=$?
	set -e
	[ "$rc" = 70 ] || fail "$label 应失败关闭并返回 70，实际 $rc"
	[ ! -s "$ENGINE_LOG" ] || fail "$label 在路径校验前调用了容器引擎"
	[ ! -s "$CHMOD_LOG" ] || fail "$label 在所有权标记校验前尝试创建目录或改权限"
}

NONEMPTY_STATE=$TMP_ROOT/nonempty-unowned-state
mkdir -p "$NONEMPTY_STATE"
printf 'not an EP state directory' >"$NONEMPTY_STATE/user-file"
LINK_TARGET=$TMP_ROOT/link-target
LINK_PARENT=$TMP_ROOT/link-parent
mkdir -p "$LINK_TARGET"
ln -s "$LINK_TARGET" "$LINK_PARENT"

assert_unsafe_state_rejected 'up 根目录' "$REPO_ROOT/scripts/dev-up.sh" / --db-only
assert_unsafe_state_rejected 'up 用户目录' "$REPO_ROOT/scripts/dev-up.sh" "$HOME" --db-only
assert_unsafe_state_rejected 'up 仓库目录' "$REPO_ROOT/scripts/dev-up.sh" "$REPO_ROOT" --db-only
assert_unsafe_state_rejected 'up 仓库子目录' "$REPO_ROOT/scripts/dev-up.sh" "$REPO_ROOT/.ep-dev-state" --db-only
assert_unsafe_state_rejected 'up 系统目录' "$REPO_ROOT/scripts/dev-up.sh" /etc --db-only
assert_unsafe_state_rejected 'up 链接父目录' "$REPO_ROOT/scripts/dev-up.sh" "$LINK_PARENT/state" --db-only
assert_unsafe_state_rejected 'up 非空且无标记目录' "$REPO_ROOT/scripts/dev-up.sh" "$NONEMPTY_STATE" --db-only
assert_unsafe_state_rejected 'down 根目录' "$REPO_ROOT/scripts/dev-down.sh" / --keep-volumes
assert_unsafe_state_rejected 'down 用户目录' "$REPO_ROOT/scripts/dev-down.sh" "$HOME" --keep-volumes

: >"$ENGINE_LOG"
set +e
EP_DEV_READY_TIMEOUT_S=not-a-number run_up --db-only >"$TMP_ROOT/timeout.out" 2>"$TMP_ROOT/timeout.err"
timeout_rc=$?
set -e
[ "$timeout_rc" = 64 ] || fail "非法就绪超时必须返回 64，实际 $timeout_rc"
[ ! -s "$ENGINE_LOG" ] || fail "非法就绪超时不得调用容器引擎"

run_up --db-only >"$TMP_ROOT/up.out" 2>"$TMP_ROOT/up.err" ||
	fail "dev-up.sh --db-only 返回非零"

OWNERSHIP_MARKER=$STATE_DIR/.ep-dev-state-owner-v1
[ -f "$OWNERSHIP_MARKER" ] || fail "首次启动未生成状态目录所有权标记"
[ ! -L "$OWNERSHIP_MARKER" ] || fail "状态目录所有权标记不得是符号链接"
grep -Fqx 'enterprise-platform-dev-state-v1' "$OWNERSHIP_MARKER" || fail "状态目录所有权标记内容不正确"
marker_mode=$(stat -f '%Lp' "$OWNERSHIP_MARKER" 2>/dev/null || stat -c '%a' "$OWNERSHIP_MARKER")
[ "$marker_mode" = 600 ] || fail "Unix 所有权标记权限必须是 0600，实际 $marker_mode"

SECRET=$STATE_DIR/secrets/postgres-superuser
[ -f "$SECRET" ] || fail "未生成数据库口令"
secret_bytes=$(wc -c <"$SECRET" | tr -d '[:space:]')
[ "$secret_bytes" = 32 ] || fail "口令必须精确 32 字节，实际 $secret_bytes"
LC_ALL=C grep -Eq '^[0-9a-f]{32}$' "$SECRET" || fail "口令必须是 32 个安全 ASCII 字符"
secret_mode=$(stat -f '%Lp' "$SECRET" 2>/dev/null || stat -c '%a' "$SECRET")
[ "$secret_mode" = 600 ] || fail "Unix 口令权限必须是 0600，实际 $secret_mode"
state_mode=$(stat -f '%Lp' "$STATE_DIR" 2>/dev/null || stat -c '%a' "$STATE_DIR")
secrets_mode=$(stat -f '%Lp' "$STATE_DIR/secrets" 2>/dev/null || stat -c '%a' "$STATE_DIR/secrets")
[ "$state_mode" = 700 ] || fail "Unix 状态目录权限必须是 0700，实际 $state_mode"
[ "$secrets_mode" = 700 ] || fail "Unix 机密目录权限必须是 0700，实际 $secrets_mode"
BINDING=$STATE_DIR/secrets/postgres-volume-binding.sha256
[ -f "$BINDING" ] || fail "未生成数据卷与口令绑定记录"
grep -Eq '^sha256:[0-9a-fA-F]{64}$' "$BINDING" || fail "数据卷绑定记录形态不正确"
assert_log_line "compose -f $REPO_ROOT/deploy/compose/compose.yaml up -d postgres"
assert_log_line "exec ep-postgres pg_isready -U postgres"
assert_log_line "compose -f $REPO_ROOT/deploy/compose/compose.yaml ps"

[ -f "$STATE_DIR/.env.dev" ] || fail "首次启动未生成 .env.dev"
grep -Fqx "EP_ETC_DIR=$STATE_DIR/etc" "$STATE_DIR/.env.dev" || fail ".env.dev 缺 EP_ETC_DIR"
grep -Fqx "EP_SECRETS_DIR=$STATE_DIR/secrets" "$STATE_DIR/.env.dev" || fail ".env.dev 缺 EP_SECRETS_DIR"

secret_before=$(LC_ALL=C od -An -tx1 "$SECRET" | tr -d '[:space:]')
run_up --db-only >"$TMP_ROOT/up-second.out" 2>"$TMP_ROOT/up-second.err" ||
	fail "第二次 dev-up.sh --db-only 返回非零"
secret_after=$(LC_ALL=C od -An -tx1 "$SECRET" | tr -d '[:space:]')
[ "$secret_before" = "$secret_after" ] || fail "重复启动改写了已有口令"

: >"$ENGINE_LOG"
run_up --full >"$TMP_ROOT/up-full.out" 2>"$TMP_ROOT/up-full.err" || fail "dev-up.sh --full 返回非零"
assert_log_line "compose -f $REPO_ROOT/deploy/compose/compose.yaml up -d"
if grep -Fqx -- "compose -f $REPO_ROOT/deploy/compose/compose.yaml up -d postgres" "$ENGINE_LOG"; then
	fail "full 模式被错误降成 db-only"
fi
grep -Fq 'image inspect ' "$ENGINE_LOG" || fail "full 模式未校验应用镜像"

: >"$ENGINE_LOG"
set +e
EP_TEST_MISSING_SERVICE=core-server run_up --full >"$TMP_ROOT/up-full-missing.out" 2>"$TMP_ROOT/up-full-missing.err"
missing_service_rc=$?
set -e
[ "$missing_service_rc" = 70 ] || fail "full 模式缺少应用容器时必须返回 70，实际 $missing_service_rc"
grep -Fq 'core-server' "$TMP_ROOT/up-full-missing.err" || fail "full 模式未指出缺失的必需服务"
if grep -Fq '全栈已就绪' "$TMP_ROOT/up-full-missing.out" || grep -Fq '库已在' "$TMP_ROOT/up-full-missing.out"; then
	fail "full 模式缺少应用容器时不得打印成功信息"
fi

: >"$ENGINE_LOG"
set +e
EP_TEST_UNHEALTHY_SERVICE=job-worker run_up --full >"$TMP_ROOT/up-full-unhealthy.out" 2>"$TMP_ROOT/up-full-unhealthy.err"
unhealthy_service_rc=$?
set -e
[ "$unhealthy_service_rc" = 70 ] || fail "full 模式应用容器不健康时必须返回 70，实际 $unhealthy_service_rc"
grep -Fq 'job-worker' "$TMP_ROOT/up-full-unhealthy.err" || fail "full 模式未指出不健康的必需服务"
if grep -Fq '全栈已就绪' "$TMP_ROOT/up-full-unhealthy.out" || grep -Fq '库已在' "$TMP_ROOT/up-full-unhealthy.out"; then
	fail "full 模式应用容器不健康时不得打印成功信息"
fi

: >"$ENGINE_LOG"
set +e
EP_TEST_EXITED_SERVICE=portal-gateway run_up --full >"$TMP_ROOT/up-full-exited.out" 2>"$TMP_ROOT/up-full-exited.err"
exited_service_rc=$?
set -e
[ "$exited_service_rc" = 70 ] || fail "full 模式应用容器已退出时必须返回 70，实际 $exited_service_rc"
grep -Fq 'portal-gateway' "$TMP_ROOT/up-full-exited.err" || fail "full 模式未指出已退出的必需服务"
if grep -Fq '全栈已就绪' "$TMP_ROOT/up-full-exited.out" || grep -Fq '库已在' "$TMP_ROOT/up-full-exited.out"; then
	fail "full 模式应用容器已退出时不得打印成功信息"
fi

: >"$ENGINE_LOG"
run_down >"$TMP_ROOT/down.out" 2>"$TMP_ROOT/down.err" || fail "dev-down.sh 默认停止返回非零"
assert_log_line "compose -f $REPO_ROOT/deploy/compose/compose.yaml down"
if grep -Fq -- '--volumes' "$ENGINE_LOG"; then
	fail "默认停止不应删除卷"
fi

: >"$ENGINE_LOG"
: >"$VOLUME_MARKER"
set +e
run_down --purge >"$TMP_ROOT/purge.out" 2>"$TMP_ROOT/purge.err"
purge_rc=$?
set -e
[ "$purge_rc" = 70 ] || fail "未有来源绑定证据时 --purge 必须失败关闭并返回 70，实际 $purge_rc"
[ ! -s "$ENGINE_LOG" ] || fail "--purge 失败关闭前不得探测或调用容器引擎"
[ -e "$VOLUME_MARKER" ] || fail "--purge 失败关闭时必须保留数据卷标记"
grep -Fq '来源绑定' "$TMP_ROOT/purge.err" || fail "--purge 应说明缺少数据卷来源绑定证据"

for misleading_args in '--purge=force' '--purge --keep-volumes' '--keep-volumes --purge'; do
	: >"$ENGINE_LOG"
	set +e
	# 这里有意按空格展开受控字面量，以覆盖伪装旗标及额外选项组合。
	# shellcheck disable=SC2086
	run_down $misleading_args >"$TMP_ROOT/purge-misleading.out" 2>"$TMP_ROOT/purge-misleading.err"
	misleading_rc=$?
	set -e
	[ "$misleading_rc" = 64 ] || fail "误导/额外 purge 选项必须按用法错误返回 64：${misleading_args}，实际 $misleading_rc"
	[ ! -s "$ENGINE_LOG" ] || fail "误导/额外 purge 选项不得触发任何引擎访问：$misleading_args"
	[ -e "$VOLUME_MARKER" ] || fail "误导/额外 purge 选项不得删除数据标记：$misleading_args"
done

: >"$ENGINE_LOG"
EP_ETC_DIR=/ EP_SECRETS_DIR=/ EP_TEST_CAPTURE_ENV=1 run_down >"$TMP_ROOT/down-env.out" 2>"$TMP_ROOT/down-env.err" ||
	fail "dev-down.sh 拒绝外部目录覆盖时返回非零"
assert_log_line "env EP_ETC_DIR=$STATE_DIR/etc EP_SECRETS_DIR=$STATE_DIR/secrets"

printf 'tampered' >"$OWNERSHIP_MARKER"
assert_unsafe_state_rejected 'down 篡改所有权标记' "$REPO_ROOT/scripts/dev-down.sh" "$STATE_DIR" --keep-volumes
printf 'enterprise-platform-dev-state-v1' >"$OWNERSHIP_MARKER"
chmod 0600 "$OWNERSHIP_MARKER"

chmod 0644 "$OWNERSHIP_MARKER"
assert_unsafe_state_rejected 'down 宽松所有权标记权限' "$REPO_ROOT/scripts/dev-down.sh" "$STATE_DIR" --keep-volumes
chmod 0600 "$OWNERSHIP_MARKER"

REAL_MARKER=$TMP_ROOT/real-marker
mv "$OWNERSHIP_MARKER" "$REAL_MARKER"
ln -s "$REAL_MARKER" "$OWNERSHIP_MARKER"
assert_unsafe_state_rejected 'down 符号链接所有权标记' "$REPO_ROOT/scripts/dev-down.sh" "$STATE_DIR" --keep-volumes
rm "$OWNERSHIP_MARKER"
mv "$REAL_MARKER" "$OWNERSHIP_MARKER"

set +e
run_up --unknown >"$TMP_ROOT/invalid.out" 2>"$TMP_ROOT/invalid.err"
invalid_rc=$?
set -e
[ "$invalid_rc" = 64 ] || fail "未知 up 参数应返回 64，实际 $invalid_rc"

set +e
EP_TEST_DISABLE_ENGINE=1 run_up --db-only >"$TMP_ROOT/no-engine.out" 2>"$TMP_ROOT/no-engine.err"
no_engine_rc=$?
set -e
[ "$no_engine_rc" = 69 ] || fail "无容器引擎应返回 69，实际 $no_engine_rc"

EMPTY_STATE=$TMP_ROOT/empty-random-state
set +e
PATH="$FAKE_BIN:$PATH" \
	EP_DEV_STATE_DIR="$EMPTY_STATE" \
	EP_TEST_ENGINE_LOG="$ENGINE_LOG" \
	EP_TEST_VOLUME_MARKER="$TMP_ROOT/empty-volume.exists" \
	EP_TEST_OD_EMPTY=1 \
	bash "$REPO_ROOT/scripts/dev-up.sh" --db-only >"$TMP_ROOT/empty-random.out" 2>"$TMP_ROOT/empty-random.err"
empty_random_rc=$?
set -e
[ "$empty_random_rc" = 70 ] || fail "安全随机源短读时应失败关闭并返回 70，实际 $empty_random_rc"
[ ! -e "$EMPTY_STATE/secrets/postgres-superuser" ] || fail "安全随机源短读时不得留下短口令文件"

LEGACY_STATE=$TMP_ROOT/legacy-short-state
mkdir -p "$LEGACY_STATE/secrets"
printf 'abc' >"$LEGACY_STATE/secrets/postgres-superuser"
set +e
PATH="$FAKE_BIN:$PATH" \
	EP_DEV_STATE_DIR="$LEGACY_STATE" \
	EP_TEST_ENGINE_LOG="$ENGINE_LOG" \
	EP_TEST_VOLUME_MARKER="$TMP_ROOT/legacy-volume.exists" \
	bash "$REPO_ROOT/scripts/dev-up.sh" --db-only >"$TMP_ROOT/legacy-short.out" 2>"$TMP_ROOT/legacy-short.err"
legacy_short_rc=$?
set -e
[ "$legacy_short_rc" = 70 ] || fail "已有短口令应失败关闭并返回 70，实际 $legacy_short_rc"
[ "$(wc -c <"$LEGACY_STATE/secrets/postgres-superuser" | tr -d '[:space:]')" = 3 ] ||
	fail "脚本不得自动改写可能已经被数据库采用的旧口令"

SWITCHED_STATE=$TMP_ROOT/switched-state
set +e
PATH="$FAKE_BIN:$PATH" \
	EP_DEV_STATE_DIR="$SWITCHED_STATE" \
	EP_TEST_ENGINE_LOG="$ENGINE_LOG" \
	EP_TEST_VOLUME_MARKER="$VOLUME_MARKER" \
	bash "$REPO_ROOT/scripts/dev-up.sh" --db-only >"$TMP_ROOT/switched.out" 2>"$TMP_ROOT/switched.err"
switched_rc=$?
set -e
[ "$switched_rc" = 70 ] || fail "已有数据卷时切换到无原口令的状态目录必须返回 70，实际 $switched_rc"
[ ! -e "$SWITCHED_STATE/secrets/postgres-superuser" ] || fail "已有数据卷时不得为新状态目录生成失配口令"

for ps_script in "$REPO_ROOT/scripts/dev-up.ps1" "$REPO_ROOT/scripts/dev-down.ps1"; do
	[ -f "$ps_script" ] || fail "缺 Windows 控制脚本：$ps_script"
	grep -Fq 'Set-StrictMode -Version 2.0' "$ps_script" || fail "PowerShell 脚本未声明 5.1 可用的严格模式：$ps_script"
done
[ -f "$REPO_ROOT/scripts/dev-state-common.ps1" ] || fail "缺 PowerShell 状态目录公共安全边界"
grep -Fq '.ep-dev-state-owner-v1' "$REPO_ROOT/scripts/dev-state-common.ps1" || fail "PowerShell 公共边界缺所有权标记"
grep -Fq 'Assert-NoReparsePathComponents' "$REPO_ROOT/scripts/dev-state-common.ps1" || fail "PowerShell 公共边界未检查全部重解析点组成"
grep -Fq 'Resolve-SafeDevStateTarget' "$REPO_ROOT/scripts/dev-state-common.ps1" || fail "PowerShell 公共边界未拒绝根、用户、仓库与系统目录"
grep -Fq "StartsWith('\\\\', [StringComparison]::Ordinal)" "$REPO_ROOT/scripts/dev-state-common.ps1" || fail "PowerShell 公共边界未拒绝 UNC/网络共享子路径"
grep -Fq '[IO.DriveType]::Network' "$REPO_ROOT/scripts/dev-state-common.ps1" || fail "PowerShell 公共边界未拒绝映射网络驱动器"
grep -Fq 'Initialize-OwnedDevStateRoot' "$REPO_ROOT/scripts/dev-state-common.ps1" || fail "PowerShell 公共边界未在改 ACL 前认领状态目录"
grep -Fq 'Get-OwnedDevStateRoot' "$REPO_ROOT/scripts/dev-state-common.ps1" || fail "PowerShell 公共边界未要求既有目录带可信标记"
grep -Fq 'Resolve-SafeDevStateTarget' "$REPO_ROOT/scripts/dev-up.ps1" || fail "Windows up 未在引擎调用前校验状态路径"
grep -Fq 'Get-OwnedDevStateRoot' "$REPO_ROOT/scripts/dev-down.ps1" || fail "Windows down 未在引擎调用前要求可信状态标记"
assert_source_order "$REPO_ROOT/scripts/dev-up.ps1" \
	'$StateDir = Resolve-SafeDevStateTarget' 'if (-not (Find-ContainerEngine))' 'Windows up'
assert_source_order "$REPO_ROOT/scripts/dev-down.ps1" \
	'$StateDir = Get-OwnedDevStateRoot' 'if (-not (Find-ContainerEngine))' 'Windows down'
assert_source_order "$REPO_ROOT/scripts/dev-state-common.ps1" \
	'-or -not (Test-DevStateMarkerPresent $state))' 'Set-Acl -LiteralPath $target' 'Windows ACL 写入'
grep -Fq "'--db-only'" "$REPO_ROOT/scripts/dev-up.ps1" || fail "Windows up 缺 --db-only"
grep -Fq "'--full'" "$REPO_ROOT/scripts/dev-up.ps1" || fail "Windows up 缺 --full"
grep -Fq 'RandomNumberGenerator' "$REPO_ROOT/scripts/dev-up.ps1" || fail "Windows up 未使用系统密码学随机源"
grep -Fq 'HISTORICAL_WINDOWS_WRAPPER_FOR_LINUX_DEV_ONLY' "$REPO_ROOT/scripts/dev-up.ps1" || fail "Windows up 未标明仅为 Linux 容器开发包装"
grep -Fq '不是 Windows Server 2022 原生运行路径' "$REPO_ROOT/scripts/dev-up.ps1" || fail "Windows up 未对 Windows Server 原生路径失败关闭"
grep -Fq 'postgres-volume-binding.sha256' "$REPO_ROOT/scripts/dev-up.ps1" || fail "Windows up 未绑定数据卷与原口令"
grep -Fq 'function Get-ComposeServices' "$REPO_ROOT/scripts/dev-up.ps1" || fail "Windows up 未从 Compose 读取必需服务"
grep -Fq 'function Wait-AllRequiredServicesReady' "$REPO_ROOT/scripts/dev-up.ps1" || fail "Windows up 未逐个等待必需服务"
grep -Fq "'ps', '-a', '-q'" "$REPO_ROOT/scripts/dev-up.ps1" || fail "Windows up 未检查已退出或缺失的容器"
grep -Fq 'running|healthy' "$REPO_ROOT/scripts/dev-up.ps1" || fail "Windows up 缺 running/healthy 就绪语义锚点"
grep -Fq 'SetAccessRuleProtection' "$REPO_ROOT/scripts/dev-state-common.ps1" || fail "Windows 公共边界未移除机密路径的继承 ACL"
grep -Fq "'--purge'" "$REPO_ROOT/scripts/dev-down.ps1" || fail "Windows down 缺 --purge 失败关闭入口"
assert_source_order "$REPO_ROOT/scripts/dev-down.ps1" \
	"if (\$Mode -eq '--purge')" 'if (-not (Find-ContainerEngine))' 'Windows purge 失败关闭'

printf 'PASS: Unix 行为负测及 PowerShell 无运行时时的静态安全顺序门禁均通过，未启动真实容器。\n'

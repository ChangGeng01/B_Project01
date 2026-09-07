#!/usr/bin/env bash
# F-57：HISTORICAL_LINUX_DEV_ONLY；不得作为 Windows Server 2022 生产启动、等价性或发布证据。
# 交付物 D-13 本地开发环境：一条命令起 PostgreSQL 16 与全栈。
#
# 起的就是 deploy/compose/compose.yaml，不另写一份开发用编排。理由是另写一份就有了第三套取值，
# 它与生产两套之间的等价性没有任何东西核对，开发机上跑通的与部署出去的会悄悄分叉。
# 开发机与生产机的差别全部由四个环境变量表达，它们的默认值与 Quadlet 一侧的字面量逐字相同：
#   EP_ETC_DIR      配置目录，默认 /etc/ep，开发机指到状态目录下
#   EP_SECRETS_DIR  机密目录，默认 /var/lib/ep/secrets，同上
#   EP_IMAGE_PREFIX 镜像前缀，默认 localhost/ep
#   EP_IMAGE_TAG    镜像标签，默认 0.1.0
#
# 退出码：
#   0   起来了
#   64  用法错误
#   69  本机没有可用的容器引擎
#   70  起栈失败，或任一必需服务在等待窗口内没有就绪
#   72  编排引用的镜像在本机不存在
#
# 镜像缺失时退出码是 72 而不是 0：本脚本不代 D-11 造镜像，也不假装栈起来了。
set -euo pipefail

EXIT_USAGE=64
EXIT_NO_ENGINE=69
EXIT_START_FAILED=70
EXIT_NO_IMAGE=72

SELF_DIR=$(cd "$(dirname "$0")" && pwd -P)
REPO_ROOT=$(cd "$SELF_DIR/.." && pwd -P)
COMPOSE_FILE=$REPO_ROOT/deploy/compose/compose.yaml
QUADLET_DIR=$REPO_ROOT/deploy/podman
PG_VOLUME_NAME=ep-pgdata
. "$SELF_DIR/dev-state-common.sh"

STATE_DIR=${EP_DEV_STATE_DIR:-${XDG_STATE_HOME:-$HOME/.local/state}/ep-dev}
READY_TIMEOUT_S=${EP_DEV_READY_TIMEOUT_S:-120}

usage() {
	cat <<'EOF'
用法：dev-up.sh [--full | --db-only]

  --full     起 PostgreSQL 16 与八个进程（默认）。
  --db-only  只起 PostgreSQL 16。集成测试只要一个库时用这个。

退出码：0 起来了；64 用法错误；69 无容器引擎；70 起栈失败或必需服务未就绪；72 镜像缺失。
状态目录由 EP_DEV_STATE_DIR 指定，默认 ${XDG_STATE_HOME:-~/.local/state}/ep-dev。
EOF
}

# 引擎按 Compose 实现挑，挑到哪个就用哪个，不做第二套编排。
detect_engine() {
	if command -v docker >/dev/null 2>&1 && docker compose version >/dev/null 2>&1; then
		ENGINE_COMPOSE="docker compose"
		ENGINE_CLI=docker
		return 0
	fi
	if command -v podman >/dev/null 2>&1 && podman compose --help >/dev/null 2>&1; then
		ENGINE_COMPOSE="podman compose"
		ENGINE_CLI=podman
		return 0
	fi
	if command -v podman-compose >/dev/null 2>&1; then
		ENGINE_COMPOSE="podman-compose"
		ENGINE_CLI=podman
		return 0
	fi
	return 1
}

image_present() {
	if [ "$ENGINE_CLI" = podman ]; then
		podman image exists "$1"
	else
		docker image inspect "$1" >/dev/null 2>&1
	fi
}

volume_present() {
	$ENGINE_CLI volume inspect "$PG_VOLUME_NAME" >/dev/null 2>&1
}

sha256_file() {
	if command -v sha256sum >/dev/null 2>&1; then
		sha256sum "$1" | awk '{print $1}'
	elif command -v shasum >/dev/null 2>&1; then
		shasum -a 256 "$1" | awk '{print $1}'
	else
		return 1
	fi
}

# 进程清单取自 deploy/podman/ 下的单元文件名，不在本脚本里另抄一份，
# 免得加了进程而这里没跟着改，检查悄悄漏掉一个。
app_services() {
	local path name
	for path in "$QUADLET_DIR"/*.container; do
		name=$(basename "$path" .container)
		[ "$name" = postgres ] || printf '%s\n' "$name"
	done
}

# 16 个随机字节编码成 32 个十六进制 ASCII 字符。通常一次就能取满；循环与最终长度断言
# 让短读、测试夹具或底层读取异常都不能静默落成短口令。
generate_password() {
	local value= chunk= attempts=0
	while [ "${#value}" -lt 32 ] && [ "$attempts" -lt 64 ]; do
		if ! chunk=$(LC_ALL=C od -An -N16 -tx1 /dev/urandom | LC_ALL=C tr -d '[:space:]'); then
			return 1
		fi
		case $chunk in
		'' | *[!0-9a-fA-F]*) return 1 ;;
		esac
		value=$value$chunk
		attempts=$((attempts + 1))
	done
	[ "${#value}" -ge 32 ] || return 1
	printf '%.32s' "$value"
}

prepare_state() {
	umask 077
	if ! ep_claim_state_root "$STATE_DIR" "$REPO_ROOT" "${HOME:-}" ||
		! ep_secure_state_directory "$STATE_DIR" "$STATE_DIR/etc" ||
		! ep_secure_state_directory "$STATE_DIR" "$STATE_DIR/secrets"; then
		printf '状态失败  无法创建本地开发状态目录 %s\n' "$STATE_DIR" >&2
		exit $EXIT_START_FAILED
	fi
	local pw=$STATE_DIR/secrets/postgres-superuser
	local binding=$STATE_DIR/secrets/postgres-volume-binding.sha256
	local env_file=$STATE_DIR/.env.dev
	local tmp pwval secret_bytes digest expected_binding has_volume=0
	volume_present && has_volume=1

	export EP_ETC_DIR=$STATE_DIR/etc
	export EP_SECRETS_DIR=$STATE_DIR/secrets
	export EP_IMAGE_PREFIX=${EP_IMAGE_PREFIX:-localhost/ep}
	export EP_IMAGE_TAG=${EP_IMAGE_TAG:-0.1.0}

	case "$EP_ETC_DIR$EP_SECRETS_DIR$EP_IMAGE_PREFIX$EP_IMAGE_TAG" in
	*$'\n'* | *$'\r'*)
		printf '状态失败  本地开发环境变量不得含换行符\n' >&2
		exit $EXIT_START_FAILED
		;;
	esac

	if [ ! -f "$env_file" ]; then
		tmp=$env_file.tmp.$$
		if ! {
			printf 'EP_ETC_DIR=%s\n' "$EP_ETC_DIR"
			printf 'EP_SECRETS_DIR=%s\n' "$EP_SECRETS_DIR"
			printf 'EP_IMAGE_PREFIX=%s\n' "$EP_IMAGE_PREFIX"
			printf 'EP_IMAGE_TAG=%s\n' "$EP_IMAGE_TAG"
		} >"$tmp"; then
			rm -f "$tmp"
			printf '状态失败  无法生成 %s\n' "$env_file" >&2
			exit $EXIT_START_FAILED
		fi
		if ! chmod 0600 "$tmp" || ! mv "$tmp" "$env_file"; then
			rm -f "$tmp"
			printf '状态失败  无法安全发布 %s\n' "$env_file" >&2
			exit $EXIT_START_FAILED
		fi
		printf '已生成    本地开发环境文件 %s\n' "$env_file"
	fi
	if [ -L "$env_file" ] || ! chmod 0600 "$env_file"; then
		printf '状态失败  %s 不得是符号链接且必须可收紧为 0600\n' "$env_file" >&2
		exit $EXIT_START_FAILED
	fi

	if [ ! -f "$pw" ]; then
		if [ "$has_volume" -eq 1 ] || [ -e "$binding" ]; then
			printf '状态失败  数据卷 %s 或其绑定记录已存在，但原口令缺失；拒绝生成新口令以免旧库失配\n' "$PG_VOLUME_NAME" >&2
			exit $EXIT_START_FAILED
		fi
		# 开发机口令，只在本状态目录内有效，不进仓库也不进任何制品。
		tmp=$pw.tmp.$$
		if ! pwval=$(generate_password); then
			rm -f "$tmp"
			printf '状态失败  无法取得足量安全随机数，未生成数据库口令\n' >&2
			exit $EXIT_START_FAILED
		fi
		if ! printf '%s' "$pwval" >"$tmp"; then
			rm -f "$tmp"
			printf '状态失败  无法写入数据库口令临时文件\n' >&2
			exit $EXIT_START_FAILED
		fi
		secret_bytes=$(wc -c <"$tmp" | tr -d '[:space:]')
		if [ "$secret_bytes" != 32 ]; then
			rm -f "$tmp"
			printf '状态失败  生成的数据库口令不是精确 32 字节\n' >&2
			exit $EXIT_START_FAILED
		fi
		if ! chmod 0600 "$tmp" || ! mv "$tmp" "$pw"; then
			rm -f "$tmp"
			printf '状态失败  无法安全发布数据库口令\n' >&2
			exit $EXIT_START_FAILED
		fi
		printf '已生成    开发机数据库超级用户口令 %s\n' "$pw"
	fi
	if [ -L "$pw" ] || [ -L "$binding" ]; then
		printf '状态失败  口令与数据卷绑定记录不得是符号链接\n' >&2
		exit $EXIT_START_FAILED
	fi

	if ! secret_bytes=$(wc -c <"$pw" | tr -d '[:space:]'); then
		printf '状态失败  无法读取已有数据库口令\n' >&2
		exit $EXIT_START_FAILED
	fi
	if [ "$secret_bytes" != 32 ] || ! LC_ALL=C grep -Eq '^[0-9A-Za-z]{32}$' "$pw"; then
		printf '状态失败  已有数据库口令不是精确 32 个安全 ASCII 字节；为避免数据库失配，不自动改写\n' >&2
		exit $EXIT_START_FAILED
	fi
	if ! chmod 0600 "$pw"; then
		printf '状态失败  无法把数据库口令权限收紧为 0600\n' >&2
		exit $EXIT_START_FAILED
	fi
	if ! digest=$(sha256_file "$pw") || ! printf '%s' "$digest" | LC_ALL=C grep -Eq '^[0-9a-fA-F]{64}$'; then
		printf '状态失败  无法计算数据库口令 SHA-256 绑定\n' >&2
		exit $EXIT_START_FAILED
	fi
	expected_binding="sha256:$digest"
	if [ -f "$binding" ]; then
		if [ "$(cat "$binding")" != "$expected_binding" ]; then
			printf '状态失败  数据库口令与 %s 的原绑定不一致\n' "$PG_VOLUME_NAME" >&2
			exit $EXIT_START_FAILED
		fi
	elif [ "$has_volume" -eq 1 ]; then
		printf '状态失败  数据卷 %s 已存在但缺少口令绑定证据；请恢复与该卷匹配的原状态目录\n' "$PG_VOLUME_NAME" >&2
		exit $EXIT_START_FAILED
	else
		tmp=$binding.tmp.$$
		if ! printf '%s' "$expected_binding" >"$tmp" || ! chmod 0600 "$tmp" || ! mv "$tmp" "$binding"; then
			rm -f "$tmp"
			printf '状态失败  无法安全发布数据卷与口令绑定记录\n' >&2
			exit $EXIT_START_FAILED
		fi
	fi
	if ! chmod 0600 "$binding"; then
		printf '状态失败  无法收紧数据卷绑定记录权限\n' >&2
		exit $EXIT_START_FAILED
	fi
}

check_images() {
	local missing=0 svc ref prefix tag
	prefix=${EP_IMAGE_PREFIX:-localhost/ep}
	tag=${EP_IMAGE_TAG:-0.1.0}
	for svc in $(app_services); do
		ref="$prefix/$svc:$tag"
		if ! image_present "$ref"; then
			printf '缺镜像    %s\n' "$ref" >&2
			missing=$((missing + 1))
		fi
	done
	if [ "$missing" -gt 0 ]; then
		printf '\n本机缺 %d 个进程镜像。八个进程镜像属交付物 D-11，本脚本不代为构建，\n' "$missing" >&2
		printf '也不把缺镜像的一次启动算作成功。只要一个库时改用 --db-only。\n' >&2
		exit $EXIT_NO_IMAGE
	fi
}

wait_ready() {
	local waited=0
	printf '等待      PostgreSQL 就绪，最多 %s 秒\n' "$READY_TIMEOUT_S"
	while [ "$waited" -lt "$READY_TIMEOUT_S" ]; do
		if $ENGINE_CLI exec ep-postgres pg_isready -U postgres >/dev/null 2>&1; then
			printf '已就绪    PostgreSQL 16，等待 %s 秒\n' "$waited"
			return 0
		fi
		sleep 2
		waited=$((waited + 2))
	done
	printf '未就绪    PostgreSQL 在 %s 秒内没有通过 pg_isready\n' "$READY_TIMEOUT_S" >&2
	return 1
}

# Compose 是服务清单的唯一来源；--full 不再靠 Podman 单元名或一份手抄数组猜测
# 应该起来哪些进程。这样新增 Compose 服务后，就绪门禁会自动把它纳入检查。
compose_services() {
	$ENGINE_COMPOSE -f "$COMPOSE_FILE" config --services
}

# 返回值：0 已就绪；1 尚在启动、可继续等待；2 缺失或已进入终止/不健康状态。
# 无 healthcheck 的服务以 running 为就绪；有 healthcheck 的服务必须 healthy。
check_required_service() {
	local service=$1 container_id inspection state health
	if ! container_id=$($ENGINE_COMPOSE -f "$COMPOSE_FILE" ps -a -q "$service" 2>/dev/null | sed -n '1p') ||
		[ -z "$container_id" ]; then
		printf '服务失败  必需服务 %s 的容器缺失\n' "$service" >&2
		return 2
	fi
	if ! inspection=$($ENGINE_CLI inspect --format '{{.State.Status}}|{{if .State.Health}}{{.State.Health.Status}}{{else}}none{{end}}' "$container_id" 2>/dev/null); then
		printf '服务失败  无法读取必需服务 %s 的容器状态\n' "$service" >&2
		return 2
	fi
	inspection=$(printf '%s\n' "$inspection" | sed -n '1p')
	case $inspection in
	*'|'*) ;;
	*)
		printf '服务失败  必需服务 %s 返回了无法识别的容器状态\n' "$service" >&2
		return 2
		;;
	esac
	state=${inspection%%|*}
	health=${inspection#*|}
	case $state in
	running)
		case $health in
		healthy | none) return 0 ;;
		starting) return 1 ;;
		unhealthy)
			printf '服务失败  必需服务 %s 的健康检查为 unhealthy\n' "$service" >&2
			return 2
			;;
		*)
			printf '服务失败  必需服务 %s 的健康状态不可识别：%s\n' "$service" "$health" >&2
			return 2
			;;
		esac
		;;
	created | restarting) return 1 ;;
	exited | dead | removing)
		printf '服务失败  必需服务 %s 未运行，容器状态为 %s\n' "$service" "$state" >&2
		return 2
		;;
	*)
		printf '服务失败  必需服务 %s 的容器状态不可识别：%s\n' "$service" "$state" >&2
		return 2
		;;
	esac
}

wait_required_services() {
	local mode=$1 services service waited=0 pending status
	if [ "$mode" = --db-only ]; then
		services=postgres
	elif ! services=$(compose_services); then
		printf '状态失败  %s config --services 返回非零\n' "$ENGINE_COMPOSE" >&2
		return 1
	fi
	[ -n "$services" ] || {
		printf '状态失败  Compose 没有返回任何必需服务\n' >&2
		return 1
	}
	for service in $services; do
		case $service in
		'' | *[!A-Za-z0-9_.-]*)
			printf '状态失败  Compose 返回非法服务名：%s\n' "$service" >&2
			return 1
			;;
		esac
	done

	printf '等待      必需服务 running/healthy，最多 %s 秒\n' "$READY_TIMEOUT_S"
	while [ "$waited" -lt "$READY_TIMEOUT_S" ]; do
		pending=0
		for service in $services; do
			if check_required_service "$service"; then
				status=0
			else
				status=$?
			fi
			case $status in
			0) ;;
			1) pending=1 ;;
			*) return 1 ;;
			esac
		done
		if [ "$pending" -eq 0 ]; then
			printf '已就绪    全部必需服务均为 running/healthy，等待 %s 秒\n' "$waited"
			return 0
		fi
		sleep 2
		waited=$((waited + 2))
	done
	printf '未就绪    必需服务在 %s 秒内未全部达到 running/healthy\n' "$READY_TIMEOUT_S" >&2
	return 1
}

main() {
	local mode=${1:---full}
	case $mode in
	--full | --db-only) ;;
	-h | --help)
		usage
		exit 0
		;;
	*)
		usage >&2
		exit $EXIT_USAGE
		;;
	esac
	case $READY_TIMEOUT_S in
	'' | *[!0-9]*)
		printf '用法错误  EP_DEV_READY_TIMEOUT_S 必须是正整数。\n' >&2
		exit $EXIT_USAGE
		;;
	esac
	if [ "${#READY_TIMEOUT_S}" -gt 9 ] || [ "$READY_TIMEOUT_S" -lt 1 ]; then
		printf '用法错误  EP_DEV_READY_TIMEOUT_S 必须是 1..999999999 的整数。\n' >&2
		exit $EXIT_USAGE
	fi

	if [ ! -f "$COMPOSE_FILE" ]; then
		printf '读不到    %s\n' "$COMPOSE_FILE" >&2
		exit $EXIT_START_FAILED
	fi
	# 只读校验必须先于容器引擎探测及任何 mkdir/chmod；危险或未标记路径不得产生副作用。
	if ! STATE_DIR=$(ep_validate_state_target "$STATE_DIR" "$REPO_ROOT" "${HOME:-}"); then
		exit $EXIT_START_FAILED
	fi
	if ! detect_engine; then
		printf '无引擎    本机没有 docker compose、podman compose 或 podman-compose 中的任何一个。\n' >&2
		printf '规格第 13.2 章的编排取值是 Docker Compose 或 Podman 加 systemd，二者缺一不可替代。\n' >&2
		exit $EXIT_NO_ENGINE
	fi
	printf '引擎      %s\n' "$ENGINE_COMPOSE"

	prepare_state
	printf '状态目录  %s\n' "$STATE_DIR"

	local -a services=()
	if [ "$mode" = --db-only ]; then
		services=(postgres)
	else
		check_images
	fi

	# 空数组在 bash 3.2 的 set -u 下直接展开会报未绑定变量，故写成带默认的形式。
	if ! $ENGINE_COMPOSE -f "$COMPOSE_FILE" up -d ${services[@]+"${services[@]}"}; then
		printf '起栈失败  %s up -d 返回非零\n' "$ENGINE_COMPOSE" >&2
		exit $EXIT_START_FAILED
	fi

	if ! wait_ready; then
		exit $EXIT_START_FAILED
	fi
	if ! wait_required_services "$mode"; then
		exit $EXIT_START_FAILED
	fi

	if ! $ENGINE_COMPOSE -f "$COMPOSE_FILE" ps; then
		printf '状态失败  %s ps 返回非零\n' "$ENGINE_COMPOSE" >&2
		exit $EXIT_START_FAILED
	fi
	if [ "$mode" = --full ]; then
		printf '\n全栈已就绪；数据库在 127.0.0.1:5432，超级用户口令见 %s/secrets/postgres-superuser。\n' "$STATE_DIR"
	else
		printf '\n库已在 127.0.0.1:5432，超级用户口令见 %s/secrets/postgres-superuser。\n' "$STATE_DIR"
	fi
	printf '库与角色由 db/bootstrap/ 下的引导脚本建，该目录按裁定 C-01 由阶段 2 交付，本脚本不代建。\n'
	printf '停栈用 scripts/dev-down.sh。\n'
}

main "$@"

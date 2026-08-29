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
#   70  起栈失败，或 PostgreSQL 在等待窗口内没有就绪
#   72  编排引用的镜像在本机不存在
#
# 镜像缺失时退出码是 72 而不是 0：本脚本不代 D-11 造镜像，也不假装栈起来了。
set -euo pipefail

EXIT_USAGE=64
EXIT_NO_ENGINE=69
EXIT_START_FAILED=70
EXIT_NO_IMAGE=72

SELF_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$SELF_DIR/.." && pwd)
COMPOSE_FILE=$REPO_ROOT/deploy/compose/compose.yaml
QUADLET_DIR=$REPO_ROOT/deploy/podman

STATE_DIR=${EP_DEV_STATE_DIR:-${XDG_STATE_HOME:-$HOME/.local/state}/ep-dev}
READY_TIMEOUT_S=${EP_DEV_READY_TIMEOUT_S:-120}

usage() {
	cat <<'EOF'
用法：dev-up.sh [--full | --db-only]

  --full     起 PostgreSQL 16 与八个进程（默认）。
  --db-only  只起 PostgreSQL 16。集成测试只要一个库时用这个。

退出码：0 起来了；64 用法错误；69 无容器引擎；70 起栈失败或库未就绪；72 镜像缺失。
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

# 进程清单取自 deploy/podman/ 下的单元文件名，不在本脚本里另抄一份，
# 免得加了进程而这里没跟着改，检查悄悄漏掉一个。
app_services() {
	local path name
	for path in "$QUADLET_DIR"/*.container; do
		name=$(basename "$path" .container)
		[ "$name" = postgres ] || printf '%s\n' "$name"
	done
}

prepare_state() {
	umask 077
	mkdir -p "$STATE_DIR/etc" "$STATE_DIR/secrets"
	local pw=$STATE_DIR/secrets/postgres-superuser
	if [ ! -f "$pw" ]; then
		# 开发机口令，只在本状态目录内有效，不进仓库也不进任何制品。
		# 不用 `tr … | head -c 32`：head 取满即退出，tr 收到 SIGPIPE 返回 141，
		# `set -o pipefail` 把整条管道判非零，`set -e` 于是在首次运行当场中止
		# （141 也不在本脚本第 13-18 行声明的退出码集合内）。改为先取够再截。
		# 不用 `tr … | head -c 32`：head 取满即退出，tr 收到 SIGPIPE 返回 141，
		# `set -o pipefail` 把整条管道判非零，`set -e` 于是在首次运行当场中止。
		# 也不用 `| cut -c1-32`：cut 按行处理会补一个行尾换行，口令文件会变成 33 字节，
		# 而消费方按整文件内容当口令读（F-73 更正 F-68 的这一处）。
		pwval=$(LC_ALL=C head -c 512 /dev/urandom | LC_ALL=C tr -dc 'a-zA-Z0-9')
		printf '%.32s' "$pwval" >"$pw"
		chmod 0600 "$pw"
		printf '已生成    开发机数据库超级用户口令 %s\n' "$pw"
	fi
	export EP_ETC_DIR=$STATE_DIR/etc
	export EP_SECRETS_DIR=$STATE_DIR/secrets
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

	if [ ! -f "$COMPOSE_FILE" ]; then
		printf '读不到    %s\n' "$COMPOSE_FILE" >&2
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

	$ENGINE_COMPOSE -f "$COMPOSE_FILE" ps
	printf '\n库已在 127.0.0.1:5432，超级用户口令见 %s/secrets/postgres-superuser。\n' "$STATE_DIR"
	printf '库与角色由 db/bootstrap/ 下的引导脚本建，该目录按裁定 C-01 由阶段 2 交付，本脚本不代建。\n'
	printf '停栈用 scripts/dev-down.sh。\n'
}

main "$@"

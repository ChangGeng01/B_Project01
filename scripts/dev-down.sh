#!/usr/bin/env bash
# F-57：HISTORICAL_LINUX_DEV_ONLY；不得作为 Windows Server 2022 生产停机或发布证据。
# 停掉 scripts/dev-up.sh 起的本地开发环境。
#
# 默认只停容器，命名卷原样留着：库里的数据是开发过程中攒出来的，
# 一条停栈命令顺手把它删掉会让人第二天从头灌一遍。要清卷得显式加 --purge。
#
# 退出码：0 停干净了；64 用法错误；69 本机没有可用的容器引擎；70 停栈命令返回非零。
set -euo pipefail

EXIT_USAGE=64
EXIT_NO_ENGINE=69
EXIT_FAILED=70

SELF_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$SELF_DIR/.." && pwd)
COMPOSE_FILE=$REPO_ROOT/deploy/compose/compose.yaml

STATE_DIR=${EP_DEV_STATE_DIR:-${XDG_STATE_HOME:-$HOME/.local/state}/ep-dev}

usage() {
	cat <<'EOF'
用法：dev-down.sh [--keep-volumes | --purge]

  --keep-volumes  只停容器，命名卷保留（默认）。
  --purge         连命名卷一起删，下次起是一个空库。

退出码：0 停干净了；64 用法错误；69 无容器引擎；70 停栈命令返回非零。
EOF
}

detect_engine() {
	if command -v docker >/dev/null 2>&1 && docker compose version >/dev/null 2>&1; then
		ENGINE_COMPOSE="docker compose"
		return 0
	fi
	if command -v podman >/dev/null 2>&1 && podman compose --help >/dev/null 2>&1; then
		ENGINE_COMPOSE="podman compose"
		return 0
	fi
	if command -v podman-compose >/dev/null 2>&1; then
		ENGINE_COMPOSE="podman-compose"
		return 0
	fi
	return 1
}

main() {
	local mode=${1:---keep-volumes}
	local -a extra=()
	case $mode in
	--keep-volumes) ;;
	--purge) extra=(--volumes) ;;
	-h | --help)
		usage
		exit 0
		;;
	*)
		usage >&2
		exit $EXIT_USAGE
		;;
	esac

	if ! detect_engine; then
		printf '无引擎    本机没有 docker compose、podman compose 或 podman-compose 中的任何一个\n' >&2
		exit $EXIT_NO_ENGINE
	fi

	# 与 dev-up.sh 取同一组路径变量：Compose 用它们算卷名与绑定源，取值不同会停错东西。
	export EP_ETC_DIR=${EP_ETC_DIR:-$STATE_DIR/etc}
	export EP_SECRETS_DIR=${EP_SECRETS_DIR:-$STATE_DIR/secrets}

	if ! $ENGINE_COMPOSE -f "$COMPOSE_FILE" down ${extra[@]+"${extra[@]}"}; then
		printf '停栈失败  %s down 返回非零\n' "$ENGINE_COMPOSE" >&2
		exit $EXIT_FAILED
	fi

	if [ "$mode" = --purge ]; then
		printf '已删卷    ep-pgdata 等命名卷，下次起是一个空库\n'
	else
		printf '已保留    命名卷，库里的数据还在；要清卷加 --purge\n'
	fi
}

main "$@"

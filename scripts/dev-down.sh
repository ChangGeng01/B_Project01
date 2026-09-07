#!/usr/bin/env bash
# F-57：HISTORICAL_LINUX_DEV_ONLY；不得作为 Windows Server 2022 生产停机或发布证据。
# 停掉 scripts/dev-up.sh 起的本地开发环境。
#
# 默认只停容器，命名卷原样留着。当前没有把卷与状态根做来源绑定的实现，
# 所以 --purge 只保留为显式的失败关闭入口，不能删除任何卷。
#
# 退出码：0 停干净了；64 用法错误；69 本机没有可用的容器引擎；70 停栈命令返回非零。
set -euo pipefail

EXIT_USAGE=64
EXIT_NO_ENGINE=69
EXIT_FAILED=70

SELF_DIR=$(cd "$(dirname "$0")" && pwd -P)
REPO_ROOT=$(cd "$SELF_DIR/.." && pwd -P)
COMPOSE_FILE=$REPO_ROOT/deploy/compose/compose.yaml
. "$SELF_DIR/dev-state-common.sh"

STATE_DIR=${EP_DEV_STATE_DIR:-${XDG_STATE_HOME:-$HOME/.local/state}/ep-dev}

usage() {
	cat <<'EOF'
用法：dev-down.sh [--keep-volumes | --purge]

  --keep-volumes  只停容器，命名卷保留（默认）。
  --purge         当前不可用：缺少卷与状态根的来源绑定证据，失败关闭且不访问引擎。

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
	if [ "$#" -gt 1 ]; then
		usage >&2
		exit $EXIT_USAGE
	fi
	local mode=${1:---keep-volumes}
	case $mode in
	--keep-volumes) ;;
	--purge)
		printf '拒绝清卷  尚未交付卷与状态根的来源绑定证据；未访问容器引擎，未删除任何卷\n' >&2
		exit $EXIT_FAILED
		;;
	-h | --help)
		usage
		exit 0
		;;
	*)
		usage >&2
		exit $EXIT_USAGE
		;;
	esac

	# 停栈也只能使用 dev-up.sh 已认领的状态目录；校验先于引擎探测，避免错误路径触发副作用。
	if ! STATE_DIR=$(ep_require_owned_state_root "$STATE_DIR" "$REPO_ROOT" "${HOME:-}"); then
		exit $EXIT_FAILED
	fi

	if ! detect_engine; then
		printf '无引擎    本机没有 docker compose、podman compose 或 podman-compose 中的任何一个\n' >&2
		exit $EXIT_NO_ENGINE
	fi

	# 与 dev-up.sh 取同一组路径变量：Compose 用它们算卷名与绑定源，取值不同会停错东西。
	export EP_ETC_DIR=$STATE_DIR/etc
	export EP_SECRETS_DIR=$STATE_DIR/secrets

	if ! $ENGINE_COMPOSE -f "$COMPOSE_FILE" down; then
		printf '停栈失败  %s down 返回非零\n' "$ENGINE_COMPOSE" >&2
		exit $EXIT_FAILED
	fi

	printf '已保留    命名卷，库里的数据还在；当前不提供清卷操作\n'
}

main "$@"

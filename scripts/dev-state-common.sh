#!/usr/bin/env bash
# dev-up.sh 与 dev-down.sh 共用的本地开发状态目录边界。
# 本文件只定义函数；调用方必须在任何容器引擎或文件权限副作用之前完成校验。

EP_STATE_MARKER_NAME=.ep-dev-state-owner-v1
EP_STATE_MARKER_VALUE=enterprise-platform-dev-state-v1

ep_path_mode() {
	stat -f '%Lp' "$1" 2>/dev/null || stat -c '%a' "$1"
}

ep_normalize_absolute_path() {
	local input=$1 part normalized= length
	local -a parts=() stack=()
	case $input in
	/*) ;;
	*)
		printf '状态失败  EP_DEV_STATE_DIR 必须是绝对路径：%s\n' "$input" >&2
		return 1
		;;
	esac
	case $input in
	*$'\n'* | *$'\r'*)
		printf '状态失败  EP_DEV_STATE_DIR 不得含换行符\n' >&2
		return 1
		;;
	esac

	IFS=/ read -r -a parts <<<"$input"
	for part in ${parts[@]+"${parts[@]}"}; do
		case $part in
		'' | .) ;;
		..)
			length=${#stack[@]}
			if [ "$length" -gt 0 ]; then
				unset 'stack[length-1]'
			fi
			;;
		*) stack[${#stack[@]}]=$part ;;
		esac
	done
	for part in ${stack[@]+"${stack[@]}"}; do
		normalized=$normalized/$part
	done
	[ -n "$normalized" ] || normalized=/
	printf '%s\n' "$normalized"
}

ep_path_is_same_or_child() {
	local candidate=$1 base=$2
	[ "$candidate" = "$base" ] && return 0
	[ "$base" = / ] && return 0
	case $candidate in
	"$base"/*) return 0 ;;
	esac
	return 1
}

ep_reject_unsafe_state_path() {
	local state=$1 repo=$2 user_home=$3 protected
	local -a system_roots=(
		/etc /private/etc /usr /bin /sbin /lib /lib64 /boot /dev /proc /sys /run /opt
		/System /Library /Applications
		/var/db /var/root /var/run /var/lib /var/log /var/spool
		/private/var/db /private/var/root /private/var/run /private/var/lib /private/var/log /private/var/spool
	)

	if [ "$state" = / ]; then
		printf '状态失败  EP_DEV_STATE_DIR 不得是文件系统根目录\n' >&2
		return 1
	fi
	if ep_path_is_same_or_child "$state" "$repo" || ep_path_is_same_or_child "$repo" "$state"; then
		printf '状态失败  EP_DEV_STATE_DIR 不得是仓库、仓库子目录或仓库祖先：%s\n' "$state" >&2
		return 1
	fi
	if [ -z "$user_home" ] || [ "$state" = "$user_home" ] || ep_path_is_same_or_child "$user_home" "$state"; then
		printf '状态失败  EP_DEV_STATE_DIR 不得是用户目录或其祖先：%s\n' "$state" >&2
		return 1
	fi
	for protected in ${system_roots[@]+"${system_roots[@]}"}; do
		if ep_path_is_same_or_child "$state" "$protected"; then
			printf '状态失败  EP_DEV_STATE_DIR 不得位于系统目录：%s\n' "$state" >&2
			return 1
		fi
	done
	return 0
}

ep_assert_no_symlink_components() {
	local path=$1 rest=${1#/} component cursor=/
	while [ -n "$rest" ]; do
		case $rest in
		*/*)
			component=${rest%%/*}
			rest=${rest#*/}
			;;
		*)
			component=$rest
			rest=
			;;
		esac
		[ "$cursor" = / ] && cursor=/$component || cursor=$cursor/$component
		if [ -L "$cursor" ]; then
			printf '状态失败  状态路径的任何组成部分都不得是符号链接：%s\n' "$cursor" >&2
			return 1
		fi
		if [ -e "$cursor" ] && [ ! -d "$cursor" ] && [ "$cursor" != "$path" ]; then
			printf '状态失败  状态路径的父级不是目录：%s\n' "$cursor" >&2
			return 1
		fi
	done
	return 0
}

ep_directory_is_empty() {
	local path=$1 entry
	for entry in "$path"/* "$path"/.[!.]* "$path"/..?*; do
		if [ -e "$entry" ] || [ -L "$entry" ]; then
			return 1
		fi
	done
	return 0
}

ep_verify_state_marker() {
	local state=$1 marker=$1/$EP_STATE_MARKER_NAME mode bytes value
	if [ ! -d "$state" ] || [ -L "$state" ] || [ ! -O "$state" ]; then
		printf '状态失败  状态目录必须是真实目录且属于当前用户：%s\n' "$state" >&2
		return 1
	fi
	mode=$(ep_path_mode "$state") || return 1
	if [ "$mode" != 700 ]; then
		printf '状态失败  带标记状态目录权限必须已经是 0700（实际 %s）：%s\n' "$mode" "$state" >&2
		return 1
	fi
	if [ -L "$marker" ] || [ ! -f "$marker" ] || [ ! -O "$marker" ]; then
		printf '状态失败  缺少可信的状态目录所有权标记：%s\n' "$marker" >&2
		return 1
	fi
	mode=$(ep_path_mode "$marker") || return 1
	if [ "$mode" != 600 ]; then
		printf '状态失败  状态目录所有权标记权限必须是 0600（实际 %s）：%s\n' "$mode" "$marker" >&2
		return 1
	fi
	bytes=$(wc -c <"$marker" | tr -d '[:space:]') || return 1
	if [ "$bytes" != "${#EP_STATE_MARKER_VALUE}" ]; then
		printf '状态失败  状态目录所有权标记长度不正确：%s\n' "$marker" >&2
		return 1
	fi
	value=$(LC_ALL=C cat "$marker") || return 1
	if [ "$value" != "$EP_STATE_MARKER_VALUE" ]; then
		printf '状态失败  状态目录所有权标记内容不正确：%s\n' "$marker" >&2
		return 1
	fi
	return 0
}

# 只读校验。输出标准化后的路径；不会创建目录、改权限或调用容器引擎。
ep_validate_state_target() {
	local raw=$1 repo_raw=$2 home_raw=$3 state repo user_home marker mode
	state=$(ep_normalize_absolute_path "$raw") || return 1
	repo=$(ep_normalize_absolute_path "$repo_raw") || return 1
	user_home=$(ep_normalize_absolute_path "$home_raw") || return 1
	ep_reject_unsafe_state_path "$state" "$repo" "$user_home" || return 1
	ep_assert_no_symlink_components "$state" || return 1

	if [ -e "$state" ] || [ -L "$state" ]; then
		if [ ! -d "$state" ] || [ -L "$state" ] || [ ! -O "$state" ]; then
			printf '状态失败  状态路径必须是真实目录且属于当前用户：%s\n' "$state" >&2
			return 1
		fi
		marker=$state/$EP_STATE_MARKER_NAME
		if [ -e "$marker" ] || [ -L "$marker" ]; then
			ep_verify_state_marker "$state" || return 1
		else
			mode=$(ep_path_mode "$state") || return 1
			if [ "$mode" != 700 ] || ! ep_directory_is_empty "$state"; then
				printf '状态失败  无所有权标记的状态目录必须为空且权限为 0700，拒绝认领：%s\n' "$state" >&2
				return 1
			fi
		fi
	fi
	printf '%s\n' "$state"
}

# 首次只认领不存在或空且 0700 的安全目录。标记写成并验证后，才允许 chmod。
ep_claim_state_root() {
	local state=$1 repo=$2 user_home=$3 marker mode
	state=$(ep_validate_state_target "$state" "$repo" "$user_home") || return 1
	umask 077
	if [ ! -e "$state" ]; then
		mkdir -p "$state" || return 1
	fi
	ep_assert_no_symlink_components "$state" || return 1
	if [ ! -d "$state" ] || [ -L "$state" ] || [ ! -O "$state" ]; then
		printf '状态失败  无法安全创建并拥有状态目录：%s\n' "$state" >&2
		return 1
	fi
	marker=$state/$EP_STATE_MARKER_NAME
	if [ ! -e "$marker" ] && [ ! -L "$marker" ]; then
		mode=$(ep_path_mode "$state") || return 1
		if [ "$mode" != 700 ] || ! ep_directory_is_empty "$state"; then
			printf '状态失败  状态目录在认领前不再为空或权限不是 0700：%s\n' "$state" >&2
			return 1
		fi
		if ! (set -C; umask 077; printf '%s' "$EP_STATE_MARKER_VALUE" >"$marker"); then
			printf '状态失败  无法独占创建状态目录所有权标记：%s\n' "$marker" >&2
			return 1
		fi
	fi
	ep_verify_state_marker "$state" || return 1
	chmod 0700 "$state" || return 1
	ep_verify_state_marker "$state"
}

ep_require_owned_state_root() {
	local raw=$1 repo_raw=$2 home_raw=$3 state repo user_home
	state=$(ep_normalize_absolute_path "$raw") || return 1
	repo=$(ep_normalize_absolute_path "$repo_raw") || return 1
	user_home=$(ep_normalize_absolute_path "$home_raw") || return 1
	ep_reject_unsafe_state_path "$state" "$repo" "$user_home" || return 1
	ep_assert_no_symlink_components "$state" || return 1
	ep_verify_state_marker "$state" || return 1
	printf '%s\n' "$state"
}

ep_secure_state_directory() {
	local state=$1 path=$2 normalized mode
	ep_verify_state_marker "$state" || return 1
	normalized=$(ep_normalize_absolute_path "$path") || return 1
	if ! ep_path_is_same_or_child "$normalized" "$state"; then
		printf '状态失败  拒绝在已认领状态目录之外创建或改权：%s\n' "$normalized" >&2
		return 1
	fi
	ep_assert_no_symlink_components "$normalized" || return 1
	mkdir -p "$normalized" || return 1
	ep_assert_no_symlink_components "$normalized" || return 1
	if [ ! -d "$normalized" ] || [ -L "$normalized" ] || [ ! -O "$normalized" ]; then
		printf '状态失败  安全目录必须是真实目录且属于当前用户：%s\n' "$normalized" >&2
		return 1
	fi
	chmod 0700 "$normalized" || return 1
	mode=$(ep_path_mode "$normalized") || return 1
	[ "$mode" = 700 ] || {
		printf '状态失败  安全目录 %s 权限不是 0700（实际 %s）\n' "$normalized" "$mode" >&2
		return 1
	}
}

#!/usr/bin/env bash
# F-57：HISTORICAL_LINUX_RESEARCH_ONLY；不得作为当前 P340/Windows 容量或发布证据。
# 连接预算枚举核对（D-11，判据 E-10、DA-06）。
# 输出八进程的连接枚举，并与规格第 7.7 章（口径落 02 计划 §4.5 表）逐项比对：
#   一、八进程的池归属与规模逐行等于规格表；
#   二、常驻连接合计等于 42；
#   三、ep-adapter-db-pg 的 STANDARD_POOL_SPECS 声明与规格表逐池一致；
#   四、portal-gateway 与 plugin-host 不链接 ep-adapter-db-pg（cargo metadata 断言）。
# 任一项不一致即非 0 退出；读不到被测对象是「未覆盖」，不判通过。
set -eu

EXIT_OK=0
EXIT_MISMATCH=2   # 取值不符
EXIT_UNCOVERED=3  # 被测对象读不到，判定未做出
EXIT_USAGE=64

SELF_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "${SELF_DIR}/.." && pwd)

mismatch_count=0
uncovered_count=0

mismatch() {
	printf '不符    %s\n' "${1}" >&2
	mismatch_count=$((mismatch_count + 1))
}

uncovered() {
	printf '未覆盖  %s\n' "${1}" >&2
	uncovered_count=$((uncovered_count + 1))
}

pass() {
	printf '通过    %s\n' "${1}"
}

# 规格表八行：进程名、池与规模描述、常驻连接数。
# archive-writer 的复制连接走 walsender，不计入普通连接预算；
# backup-writer 只在备份窗口内占用 1 连接，非常驻。两者常驻均计 0。
spec_rows() {
	cat <<'EOF'
core-server Rw20+Ro10 30
job-worker Worker5 5
integration-gateway Integ5 5
portal-gateway 无数据库链接 0
plugin-host 无数据库链接 0
ops-agent Ops2 2
archive-writer 复制1常驻加1槽不计普通预算 0
backup-writer 备份窗口内1非常驻 0
EOF
}

EXPECT_RESIDENT_SUM=42

check_enum_and_sum() {
	printf '== 八进程连接枚举（规格第 7.7 章口径）==\n'
	local sum=0 proc spec resident
	while read -r proc spec resident; do
		printf '枚举    %-22s %-28s 常驻 %s\n' "${proc}" "${spec}" "${resident}"
		case ${resident} in
		'' | *[!0-9]*)
			mismatch "${proc} 的常驻连接数 ${resident} 不是非负整数"
			continue
			;;
		esac
		sum=$((sum + resident))
	done <<EOF
$(spec_rows)
EOF
	if [ "${sum}" != "${EXPECT_RESIDENT_SUM}" ]; then
		mismatch "常驻连接合计为 ${sum}，规格第 7.7 章为 ${EXPECT_RESIDENT_SUM}"
	else
		pass "常驻连接合计 ${sum}，与规格一致"
	fi
}

# ep-adapter-db-pg 的池规模声明必须与规格表逐池一致：Rw20/Ro10/Worker5/Integ5/Ops2。
check_pool_specs() {
	printf '== ep-adapter-db-pg 池规模声明比对 ==\n'
	local file=${REPO_ROOT}/crates/adapter/db-pg/src/budget.rs
	if [ ! -r "${file}" ]; then
		uncovered "budget.rs 读不到：${file}"
		return
	fi
	local kind size
	for pair in 'Rw 20' 'Ro 10' 'Worker 5' 'Integ 5' 'Ops 2'; do
		kind=${pair%% *}
		size=${pair##* }
		if grep -A 2 "kind: PoolKind::${kind}," "${file}" | grep -q "max_connections: ${size},"; then
			pass "${kind} 池规模 ${size} 与规格一致"
		else
			mismatch "${kind} 池规模声明缺失或不等于 ${size}"
		fi
	done
}

# portal-gateway 与 plugin-host 不得链接 ep-adapter-db-pg：
# 两个进程的连接预算为零，链接了数据库适配层即是口径漂移。
check_no_db_link() {
	printf '== 无数据库链接断言（cargo metadata）==\n'
	if ! command -v cargo >/dev/null 2>&1; then
		uncovered "cargo 不在 PATH，依赖图无从读取"
		return
	fi
	local meta proc
	if ! meta=$(cd "${REPO_ROOT}" && cargo metadata --format-version=1 --no-deps 2>/dev/null); then
		uncovered "cargo metadata 执行失败，依赖图无从读取"
		return
	fi
	for proc in portal-gateway plugin-host; do
		if printf '%s\n' "${meta}" | grep -q "\"name\":\"${proc}\""; then
			if printf '%s\n' "${meta}" | grep -o "{\"name\":\"${proc}\"[^}]*\"dependencies\":\[[^]]*\]" |
				grep -q 'ep-adapter-db-pg'; then
				mismatch "${proc} 依赖了 ep-adapter-db-pg，与其零连接预算冲突"
			else
				pass "${proc} 未链接 ep-adapter-db-pg"
			fi
		else
			uncovered "cargo metadata 中没有包 ${proc}"
		fi
	done
}

usage() {
	cat <<'EOF'
用法：verify-connection-budget.sh [--self-check | --all]

  --self-check  只做枚举与合计、池规模声明两项静态比对。
  --all         全部三项（默认），含 cargo metadata 的无链接断言。

退出码：0 通过；2 取值不符；3 被测对象读不到、判定未做出；64 用法错误。
EOF
}

main() {
	local mode=${1:---all}
	case ${mode} in
	--self-check)
		check_enum_and_sum
		check_pool_specs
		;;
	--all)
		check_enum_and_sum
		check_pool_specs
		check_no_db_link
		;;
	-h | --help)
		usage
		exit ${EXIT_OK}
		;;
	*)
		usage >&2
		exit ${EXIT_USAGE}
		;;
	esac

	printf '== 结论 ==\n'
	printf '不符 %d 项，未覆盖 %d 项\n' "${mismatch_count}" "${uncovered_count}"
	if [ "${mismatch_count}" -gt 0 ]; then
		printf '判定：不通过（取值不符）\n'
		exit ${EXIT_MISMATCH}
	fi
	if [ "${uncovered_count}" -gt 0 ]; then
		printf '判定：未覆盖，本次核对不成立，不得据此判通过\n'
		exit ${EXIT_UNCOVERED}
	fi
	printf '判定：通过\n'
	exit ${EXIT_OK}
}

main "$@"

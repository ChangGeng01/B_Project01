#!/usr/bin/env bash
# F-57：HISTORICAL_LINUX_RESEARCH_ONLY；不得作为 Windows Server 2022 生产编排证据。
# verify-orchestration-equivalence.py 的负样例集：每条规则一个故意违反的样本，
# 断言的是规则本身——不只看退出码，还要求输出里出现该规则的名字，
# 否则一个「凡事都判不符」的坏脚本也能把这些样例全过掉。
#
# 样本一律在临时目录的 deploy/ 副本上改，经 EP_DEPLOY_ROOT 指过去，仓库里的文件一个字不动。
# 每次改动都先断言基准文本存在：基准文本被人改名后，负样例必须当场红，
# 而不是悄悄退化成一个什么都没改的空样本。
set -euo pipefail

SELF_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$SELF_DIR/.." && pwd)
CHECKER=$SELF_DIR/verify-orchestration-equivalence.py

EXIT_MISMATCH=2
EXIT_UNCOVERED=3

failed=0
passed=0
WORK=$(mktemp -d "${TMPDIR:-/tmp}/ep-orch-neg.XXXXXX")
trap 'rm -rf "$WORK"' EXIT

# 在文件里做一次定点替换，基准文本不存在即失败。
subst() {
	python3 - "$1" "$2" "$3" <<'PY'
import pathlib, sys
path, old, new = pathlib.Path(sys.argv[1]), sys.argv[2], sys.argv[3]
text = path.read_text(encoding="utf-8")
if old not in text:
    sys.exit(f"负样例的基准文本在 {path} 中不存在：{old!r}")
path.write_text(text.replace(old, new, 1), encoding="utf-8")
PY
}

# 用法：expect <样例名> <期望退出码> <期望规则名> <改动命令...>
# 改动命令在副本目录 $DEPLOY 上执行。
expect() {
	local name=$1 want_code=$2 want_rule=$3
	shift 3
	local dir out code
	dir=$(mktemp -d "$WORK/case.XXXXXX")
	cp -R "$REPO_ROOT/deploy" "$dir/deploy"
	DEPLOY=$dir/deploy "$@"
	out=$(EP_DEPLOY_ROOT="$dir/deploy" python3 "$CHECKER" --all 2>&1) && code=0 || code=$?

	if [ "$code" != "$want_code" ]; then
		printf '负样例未达预期  %s：退出码 %s，应为 %s\n' "$name" "$code" "$want_code" >&2
		printf '%s\n' "$out" >&2
		failed=$((failed + 1))
		return
	fi
	if ! printf '%s\n' "$out" | grep -q -F "[$want_rule]"; then
		printf '负样例未达预期  %s：退出码对了，但输出里没有规则 %s，判不符的不是该规则\n' \
			"$name" "$want_rule" >&2
		printf '%s\n' "$out" >&2
		failed=$((failed + 1))
		return
	fi
	printf '负样例已红      %s　→　[%s]，退出码 %s\n' "$name" "$want_rule" "$code"
	passed=$((passed + 1))
}

# ---- 各规则的改动动作，逐个只碰一处 ----

m_tab() { subst "$DEPLOY/compose/compose.yaml" '  postgres:' "$(printf '\tpostgres:')"; }
m_quadlet_key() { subst "$DEPLOY/podman/core-server.container" 'ReadOnly=true' 'Readonly=true'; }
m_compose_key() { subst "$DEPLOY/compose/compose.yaml" '  ops-agent:' '  ops-agent:
    hostname: ops'; }
m_drop_service() { rm "$DEPLOY/podman/ops-agent.container"; }
m_required() {
	subst "$DEPLOY/podman/plugin-host.container" 'Restart=always
' ''
	subst "$DEPLOY/compose/compose.yaml" '    cgroup_parent: app-plugin.slice
    restart: always
' '    cgroup_parent: app-plugin.slice
'
}
m_fact() { subst "$DEPLOY/compose/compose.yaml" '/job-worker:${EP_IMAGE_TAG:-0.1.0}' '/job-worker:${EP_IMAGE_TAG:-0.9.9}'; }
m_volume_decl() { rm "$DEPLOY/podman/ep-files.volume"; }
m_order_paired() { subst "$DEPLOY/podman/backup-writer.container" 'Requires=postgres.service core-server.service
' ''; }
m_quota_in_orch() { subst "$DEPLOY/podman/job-worker.container" 'Slice=app-worker.slice' 'Slice=app-worker.slice
CPUWeight=1000'; }
m_quota_in_unit() { subst "$DEPLOY/systemd/system/app-core.slice" 'Before=slices.target' 'Before=slices.target
CPUWeight=2000'; }
m_unit_missing() { rm "$DEPLOY/systemd/system/app-edge.slice"; }
m_dropin_missing() { rm "$DEPLOY/systemd/system/app-portal.slice.d/10-resource-limits.conf"; }
m_dropin_orphan() {
	subst "$DEPLOY/podman/ops-agent.container" 'Slice=app-edge.slice' 'Slice=app-core.slice'
	subst "$DEPLOY/compose/compose.yaml" '    cgroup_parent: app-edge.slice' '    cgroup_parent: app-core.slice'
}

main() {
	printf '== 正样例，作为对照 ==\n'
	local out code
	out=$(python3 "$CHECKER" --all 2>&1) && code=0 || code=$?
	if [ "$code" != "0" ]; then
		printf '对照失败        仓库现状本应通过，实际退出码 %s\n' "$code" >&2
		printf '%s\n' "$out" >&2
		failed=$((failed + 1))
	else
		printf '对照通过        仓库现状退出码 0\n'
		passed=$((passed + 1))
	fi

	printf '== 负样例，每条规则一个 ==\n'
	expect 制表符缩进 "$EXIT_UNCOVERED" EQ-PARSE m_tab
	expect Quadlet键名拼错 "$EXIT_MISMATCH" EQ-QUADLET-KEY-KNOWN m_quadlet_key
	expect Compose多一个键 "$EXIT_MISMATCH" EQ-COMPOSE-KEY-KNOWN m_compose_key
	expect 一侧少一个服务 "$EXIT_MISMATCH" EQ-SERVICE-SET m_drop_service
	expect 两侧同时漏必备项 "$EXIT_MISMATCH" EQ-REQUIRED-FACT m_required
	expect 一侧镜像标签不同 "$EXIT_MISMATCH" EQ-FACT m_fact
	expect 卷引用了却没声明 "$EXIT_MISMATCH" EQ-VOLUME-DECL m_volume_decl
	expect 有After无Requires "$EXIT_MISMATCH" EQ-QUADLET-ORDER-PAIRED m_order_paired
	expect 编排里写配额 "$EXIT_MISMATCH" SL-QUOTA-IN-ORCH m_quota_in_orch
	expect slice单元里写配额 "$EXIT_MISMATCH" SL-QUOTA-IN-UNIT m_quota_in_unit
	expect 引用的slice无单元 "$EXIT_MISMATCH" SL-UNIT-MISSING m_unit_missing
	expect 引用的slice无dropin "$EXIT_MISMATCH" SL-DROPIN-MISSING m_dropin_missing
	expect dropin无人引用 "$EXIT_MISMATCH" SL-DROPIN-ORPHAN m_dropin_orphan

	printf '== 结论 ==\n'
	printf '达预期 %d 项，未达预期 %d 项\n' "$passed" "$failed"
	if [ "$failed" -gt 0 ]; then
		printf '判定：不通过，有规则没有咬住它该咬的样本\n'
		exit 1
	fi
	printf '判定：通过\n'
}

main "$@"

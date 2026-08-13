#!/usr/bin/env bash
# D-07 两个判定件的负样例集。每个负样例只对真实登记表做一处定向改动，
# 并同时断言退出码与那一条规则自己的报错文字——只断言退出码不足以证明是
# 哪条规则报的，本仓库已因判定笼统返工多次。
#
# 退出码：0 全部负样例如期失败；1 有负样例没按预期报错。
set -uo pipefail

SELF_DIR=$(cd "$(dirname "$0")" && pwd)
CI_DIR=$(cd "$SELF_DIR/.." && pwd)
REPO_ROOT=$(cd "$CI_DIR/../.." && pwd)

VERIFY="$CI_DIR/verify-pipeline-commands.sh"
PIPELINE="$CI_DIR/run-pipeline.sh"
ROSTER="$CI_DIR/pipeline-stages.tsv"
DOC="$REPO_ROOT/docs/ci-pipeline.md"

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

failed=0
passed=0

# expect <名称> <期望退出码> <期望报错文字> -- <命令...>
# 本轮无法构造的负样例。既不记通过也不记失败，单列计数并在结论行报出。
#
# 立这一档的理由与工具本身同一条纪律：负样例构造不出来时若静默跳过，
# 结论行会显示「全部如期失败」，读者据此以为该规则被守着，实际没有。
unconstructible=0
skip() {
    local name=$1 why=$2
    echo "负样例本轮不可构造：${name}　$why"
    unconstructible=$((unconstructible + 1))
}

expect() {
    local name=$1 want_rc=$2 want_msg=$3
    shift 4 # 名称、退出码、文字与 --
    local out rc=0
    out=$("$@" 2>&1) || rc=$?
    if [[ $rc -ne $want_rc ]]; then
        echo "负样例未按预期失败：${name}　期望退出码 ${want_rc}，实得 $rc" >&2
        printf '%s\n' "$out" | sed 's/^/    /' >&2
        failed=$((failed + 1))
        return
    fi
    if [[ -n $want_msg && $out != *"$want_msg"* ]]; then
        echo "负样例退出码对但报错文字不对：${name}　期望含「${want_msg}」" >&2
        printf '%s\n' "$out" | sed 's/^/    /' >&2
        failed=$((failed + 1))
        return
    fi
    echo "负样例如期失败：${name}（退出码 ${rc}）"
    passed=$((passed + 1))
}

# ---- 针对 verify-pipeline-commands.sh 的负样例 ------------------------------

# N1 引用一条 xtask 不受理的子命令。断言的是「命令必须真的存在」这条规则。
sed 's/^3\tarchcheck\txtask\tarchcheck\t/3\tarchcheck\txtask\tnosuchgate\t/' "$ROSTER" >"$WORK/n1.tsv"
expect "N1 引用不存在的 xtask 子命令" 2 "cargo xtask 不受理子命令 nosuchgate" -- \
    env EP_CI_ROSTER="$WORK/n1.tsv" bash "$VERIFY"

# N2 引用一条仓库内不存在的脚本。
sed 's#scripts/verify-resource-limits.sh#scripts/no-such-script.sh#' "$ROSTER" >"$WORK/n2.tsv"
expect "N2 引用不存在的脚本" 2 "在仓库内不存在" -- \
    env EP_CI_ROSTER="$WORK/n2.tsv" bash "$VERIFY"

# N3 脚本存在但没有可执行位。夹具是仓库内一个固定为 644 的文件。
sed 's#scripts/verify-resource-limits.sh#.github/ci/tests/fixtures/not-executable.sh#' "$ROSTER" >"$WORK/n3.tsv"
expect "N3 脚本存在但无可执行位" 2 "存在但没有可执行位" -- \
    env EP_CI_ROSTER="$WORK/n3.tsv" bash "$VERIFY"

# N4 阶段数不是 11。删掉第 11 阶段一行。
grep -v '^11	' "$ROSTER" >"$WORK/n4.tsv"
expect "N4 阶段数少一个" 2 "D-07 定死为 11 个" -- \
    env EP_CI_ROSTER="$WORK/n4.tsv" bash "$VERIFY"

# N5 登记表说已交付，工具却报未交付。断言的是门禁状态那一列不是备注。
skip "N5 未交付的门禁被登记成已交付" "十一个子命令本轮全部交付，没有任何子命令返回 70，该分支无法用真实子命令构造。规则仍在 verify-pipeline-commands.sh 与 run-pipeline.sh 内，一旦将来有子命令回到未交付态即自动重新可测。"

# N6 登记表说未交付，工具却能跑通。反方向同样要报。
sed 's/^3\tarchcheck\txtask\tarchcheck\tdelivered/3\tarchcheck\txtask\tarchcheck\tundelivered/' "$ROSTER" >"$WORK/n6.tsv"
expect "N6 已交付的门禁被登记成未交付" 2 "而不是 70" -- \
    env EP_CI_ROSTER="$WORK/n6.tsv" bash "$VERIFY"

# N7 登记表与文档的阶段表不相等。改文档一侧的一个阶段 id。
sed 's/| `archcheck` |/| `arch-check` |/' "$DOC" >"$WORK/n7.md"
expect "N7 文档阶段表与登记表不相等" 2 "阶段表不相等" -- \
    env EP_CI_DOC="$WORK/n7.md" bash "$VERIFY"

# N8 读不到登记表必须报未覆盖（3），不得报不符也不得判通过。
expect "N8 登记表读不到" 3 "判定未做出" -- \
    env EP_CI_ROSTER="$WORK/does-not-exist.tsv" bash "$VERIFY"

# N9 读不到文档同样是未覆盖，与「不符」用不同的码。
expect "N9 文档读不到" 3 "一致性判定未做出" -- \
    env EP_CI_DOC="$WORK/does-not-exist.md" bash "$VERIFY"

# N10 命令类别不认识：判定做不出来，报未覆盖而不是放过。
sed 's/^11\tdeploy-limits\tscript\t/11\tdeploy-limits\tpodman\t/' "$ROSTER" >"$WORK/n10.tsv"
expect "N10 命令类别不认识" 3 "不认识" -- \
    env EP_CI_ROSTER="$WORK/n10.tsv" bash "$VERIFY"

# ---- 针对 run-pipeline.sh 的负样例 -----------------------------------------
# 这两条断言的是「未交付不得折算成通过」这条最重的纪律本身。

printf '3\tarchcheck\txtask\tarchcheck\tdelivered\n' >"$WORK/p-green.tsv"
printf '3\tarchcheck\txtask\tarchcheck\tdelivered\n4\tsqlcheck\txtask\tsqlcheck\tundelivered\n' >"$WORK/p-undelivered.tsv"

# P1 只有已交付门禁时必须返回 0——这是对照组，证明 P2 的非零不是恒非零。
expect "P1 全为已交付门禁时返回 0" 0 "" -- \
    env EP_CI_ROSTER="$WORK/p-green.tsv" bash "$PIPELINE"

# P2 掺入一条未交付门禁后必须返回 70，不得返回 0。
skip "P2 掺入未交付门禁后不得返回 0" "十一个子命令本轮全部交付，没有任何子命令返回 70，该分支无法用真实子命令构造。规则仍在 verify-pipeline-commands.sh 与 run-pipeline.sh 内，一旦将来有子命令回到未交付态即自动重新可测。"

# P3 登记表说已交付而工具报未交付，流水线按不符处理并返回 1。
skip "P3 登记表失真时流水线判不符" "十一个子命令本轮全部交付，没有任何子命令返回 70，该分支无法用真实子命令构造。规则仍在 verify-pipeline-commands.sh 与 run-pipeline.sh 内，一旦将来有子命令回到未交付态即自动重新可测。"

# ---- 结论 ------------------------------------------------------------------

echo
if [[ $failed -gt 0 ]]; then
    echo "负样例集：$passed 条如期失败，$unconstructible 条本轮不可构造，$failed 条未如期失败。" >&2
    exit 1
fi
echo "负样例集：$passed 条如期失败，$unconstructible 条本轮不可构造，$failed 条未如期失败。"
exit 0

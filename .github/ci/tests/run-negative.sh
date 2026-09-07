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

# N5/P2/P3 需要让指定门禁返回 70，同时其余 cargo 调用仍走真实 cargo。
# 代理只安装到本轮临时工作区，绝不影响仓库或生产执行路径。
REAL_CARGO=$(command -v cargo)
FAKE_CARGO_DIR="$WORK/fake-bin"
mkdir -p "$FAKE_CARGO_DIR"
cp "$SELF_DIR/fixtures/fake-cargo.sh" "$FAKE_CARGO_DIR/cargo"
chmod 755 "$FAKE_CARGO_DIR/cargo"

# 比较器的全表对照使用这组与仓库现状无关的、手工推导的测量状态。只有临时
# fake cargo 明确启用时才读取它；未启用的普通 cargo 调用仍委派真实二进制。
FAKE_XTASK_RESULTS='archcheck=0/0;sqlcheck=0/0;codecheck=1/2;errorcodes=1/4;configdoc=1/7;eventcatalog=1/9'

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

# expect_not_rc <名称> <不得出现的退出码> -- <命令...>
# 用于验证临时代理没有误截获近似但不完全相同的 cargo 调用。
expect_not_rc() {
    local name=$1 forbidden_rc=$2
    shift 3 # 名称、不得出现的退出码与 --
    local out rc=0
    out=$("$@" 2>&1) || rc=$?
    if [[ $rc -eq $forbidden_rc ]]; then
        echo "负样例未按预期失败：${name}　不应退出 ${forbidden_rc}" >&2
        printf '%s\n' "$out" | sed 's/^/    /' >&2
        failed=$((failed + 1))
        return
    fi
    echo "负样例如期失败：${name}（退出码 ${rc}，非 ${forbidden_rc}）"
    passed=$((passed + 1))
}

# run_fake_compare <baseline> <cargo-test 原始 rc> <failed 数> [比较器参数...]
# 所有门禁及 cargo-test 都由临时代理给出确定性结果，避免把测试断言绑定到当前
# 仓库的已登记红数量。
run_fake_compare() {
    local baseline=$1 test_rc=$2 failed_count=$3
    shift 3
    env PATH="$FAKE_CARGO_DIR:$PATH" EP_CI_REAL_CARGO="$REAL_CARGO" \
        EP_CI_FAKE_XTASK_RESULTS="$FAKE_XTASK_RESULTS" EP_CI_FAKE_CARGO_TEST_RC="$test_rc" \
        EP_CI_FAKE_CARGO_TEST_FAILED="$failed_count" EP_RED_BASELINE="$baseline" \
        bash "$COMPARE" "$@"
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

# N5 状态列是封闭集；预检不执行门禁，但必须拒绝不能解释的登记值。
sed 's/^3\tarchcheck\txtask\tarchcheck\tdelivered/3\tarchcheck\txtask\tarchcheck\tmaybe/' "$ROSTER" >"$WORK/n5.tsv"
expect "N5 门禁状态不是封闭值" 2 "不在 {delivered, undelivered} 内" -- \
    env EP_CI_ROSTER="$WORK/n5.tsv" bash "$VERIFY"

# N6 状态列缺失同样不能落到默认值。
sed 's/^3\tarchcheck\txtask\tarchcheck\tdelivered/3\tarchcheck\txtask\tarchcheck\t/' "$ROSTER" >"$WORK/n6.tsv"
expect "N6 门禁状态缺失" 2 "不在 {delivered, undelivered} 内" -- \
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

# N10b 带参数的 xtask 行仍须按第一个词元验证子命令，不得因有参数而跳过。
cp "$ROSTER" "$WORK/n10b.tsv"
printf '6\tregistry-docs\txtask\tnosuchgate --plausible-flag\tdelivered\n' >>"$WORK/n10b.tsv"
expect "N10b 带参数的未知 xtask 子命令" 2 \
    "cargo xtask 不受理子命令 nosuchgate" -- \
    env EP_CI_ROSTER="$WORK/n10b.tsv" bash "$VERIFY"

# N10c configdoc 的参数面是封闭集；拼错旗标不得静默回退到默认 configdoc。
expect "N10c configdoc 拒绝未知参数" 2 "未知参数 --not-a-configdoc-option" -- \
    cargo xtask configdoc --not-a-configdoc-option

# N10d Cargo 行必须验证登记的首子命令；只验证 `cargo` 可执行文件会放过拼错的命令。
sed 's/^5\tcodecheck\tcargo\tclippy /5\tcodecheck\tcargo\tnot-a-cargo-subcommand /' "$ROSTER" >"$WORK/n10d.tsv"
expect "N10d Cargo 行拒绝未知首子命令" 2 "cargo 不受理首子命令 not-a-cargo-subcommand" -- \
    env EP_CI_ROSTER="$WORK/n10d.tsv" bash "$VERIFY"

# N10e 依赖解析型 Cargo 行必须同时携带 --locked/--offline；fmt/list/help 不在此约束内。
sed 's/^5\tcodecheck\tcargo\tclippy --workspace --locked --offline/5\tcodecheck\tcargo\tclippy --workspace --locked/' \
    "$ROSTER" >"$WORK/n10e.tsv"
expect "N10e Cargo 依赖解析行缺 offline" 2 "缺 --locked/--offline" -- \
    env EP_CI_ROSTER="$WORK/n10e.tsv" bash "$VERIFY"

# N10f 预检只问可用性，不执行登记行。fake cargo 记录真实边界；若逐行执行，日志会出现门禁名。
: >"$WORK/verify-cargo.log"
env PATH="$FAKE_CARGO_DIR:$PATH" EP_CI_REAL_CARGO="$REAL_CARGO" \
    EP_CI_FAKE_CARGO_LOG="$WORK/verify-cargo.log" bash "$VERIFY" >/dev/null 2>&1 || true
grep -Fqx -- '--list' "$WORK/verify-cargo.log" || {
    echo '负样例未按预期失败：N10f 预检没有通过 cargo --list 验证首子命令' >&2
    failed=$((failed + 1))
}
if grep -Eq '^xtask (archcheck|sqlcheck|codecheck|errorcodes|eventcatalog|configdoc|sbom|sign|reproduce|coverage|e2e|ci)( |$)' "$WORK/verify-cargo.log"; then
    echo '负样例未按预期失败：N10f 预检执行了登记的 xtask 行' >&2
    failed=$((failed + 1))
else
    echo '负样例如期失败：N10f 预检没有执行登记命令，仅验证可用性'
    passed=$((passed + 1))
fi

# N10g/N10h 检验敏感自托管 workflow 的触发边界，不让人工触发或非 main push 重新出现。
awk '1; /  push:/ { print "  workflow_dispatch:" }' "$REPO_ROOT/.github/workflows/ci.yml" >"$WORK/n10g.yml"
expect "N10g workflow 拒绝人工触发" 2 "只允许 main push" -- \
    env EP_CI_WORKFLOW="$WORK/n10g.yml" bash "$VERIFY"
sed 's/^      - main$/      - feature-x/' "$REPO_ROOT/.github/workflows/ci.yml" >"$WORK/n10h.yml"
expect "N10h workflow 拒绝非 main push" 2 "只允许 main push" -- \
    env EP_CI_WORKFLOW="$WORK/n10h.yml" bash "$VERIFY"
awk '1; /^  pipeline:$/ { print "    if: github.ref == '\''refs/heads/main'\''" }' \
    "$REPO_ROOT/.github/workflows/ci.yml" >"$WORK/n10i.yml"
expect "N10i 唯一 job 不得靠 if 跳过" 2 "唯一 job 不得用 if 跳过" -- \
    env EP_CI_WORKFLOW="$WORK/n10i.yml" bash "$VERIFY"

# ---- 针对 run-pipeline.sh 的负样例 -----------------------------------------
# 这两条断言的是「未交付不得折算成通过」这条最重的纪律本身。

printf '3\tarchcheck\txtask\tarchcheck\tdelivered\n' >"$WORK/p-green.tsv"
printf '3\tarchcheck\txtask\tarchcheck\tdelivered\n4\tsqlcheck\txtask\tsqlcheck\tundelivered\n' >"$WORK/p-undelivered.tsv"
printf '3\tarchcheck\txtask\tarchcheck\tundelivered\n' >"$WORK/p-status-lie.tsv"

# P1 只有已交付门禁时必须返回 0——这是对照组，证明 P2 的非零不是恒非零。
expect "P1 全为已交付门禁时返回 0" 0 "" -- \
    env EP_CI_ROSTER="$WORK/p-green.tsv" bash "$PIPELINE"

# P2 掺入一条未交付门禁后必须返回 70，不得返回 0。
expect "P2 掺入未交付门禁后不得返回 0" 70 "本阶段未交付" -- \
    env PATH="$FAKE_CARGO_DIR:$PATH" EP_CI_REAL_CARGO="$REAL_CARGO" EP_CI_FAKE_CARGO_70_GATE=sqlcheck \
    EP_CI_ROSTER="$WORK/p-undelivered.tsv" bash "$PIPELINE"

# P3 登记表说已交付而工具报未交付，流水线按不符处理并返回 1。
expect "P3 登记表失真时流水线判不符" 1 "登记为已交付却报未交付，登记表失真" -- \
    env PATH="$FAKE_CARGO_DIR:$PATH" EP_CI_REAL_CARGO="$REAL_CARGO" EP_CI_FAKE_CARGO_70_GATE=archcheck \
    EP_CI_ROSTER="$WORK/p-green.tsv" bash "$PIPELINE"

# P3b 反方向同样是登记失真：标未交付的命令若返回 0，不得被累计为通过。
expect "P3b 未交付登记却实跑成功时判失真" 1 "登记为未交付却返回 0，登记表失真" -- \
    env PATH="$FAKE_CARGO_DIR:$PATH" EP_CI_REAL_CARGO="$REAL_CARGO" \
    EP_CI_FAKE_XTASK_RESULTS="$FAKE_XTASK_RESULTS" \
    EP_CI_ROSTER="$WORK/p-status-lie.tsv" bash "$PIPELINE"

# ---- 针对 compare-red-baseline.sh 的负样例 ---------------------------------
# 这一组断言的是「基线红对照表不得变成恒真的挡箭牌」：它必须能报出新回归，
# 也必须在实测取不到时判未覆盖，而不是静默判无回归。
# 各例都用单行基线表加 --gates-only，只跑一道判定面，避免整轮 cargo test。

COMPARE="$CI_DIR/compare-red-baseline.sh"

printf '# 只有注释，一行数据都没有\n' >"$WORK/rb-empty.tsv"

# 这张表是手工列出的完整可测状态；每个新负样例只改一处，避免一个坏字段
# 被另一个缺表项遮蔽。它也让解析校验可在调用任何门禁前完成。
write_complete_baseline() {
    local path=$1
    printf '%s\n' \
        $'gate\texpect_exit\texpect_count\tnote' \
        $'archcheck\t0\t0\tclean' \
        $'sqlcheck\t0\t0\tclean' \
        $'codecheck\t1\t2\tknown-red' \
        $'errorcodes\t1\t4\tknown-red' \
        $'configdoc\t1\t7\tknown-red' \
        $'eventcatalog\t1\t9\tknown-red' \
        $'cargo-test\t1\t3\tknown-red' >"$path"
}

write_complete_baseline "$WORK/rb-complete.tsv"
sed $'s/^codecheck\t1\t2\t/codecheck\t0\t0\t/' "$WORK/rb-complete.tsv" >"$WORK/rb-lie.tsv"
sed '1d' "$WORK/rb-complete.tsv" >"$WORK/rb-no-header.tsv"
printf '%s\n' \
    $'gate\texpect_exit\texpect_count\tnote' \
    $'archcheck\t0\t0\tclean' >"$WORK/rb-partial.tsv"
{
    sed -n 'p' "$WORK/rb-complete.tsv"
    printf '%s\n' $'archcheck\t0\t0\tduplicate'
} >"$WORK/rb-duplicate.tsv"
{
    sed -n 'p' "$WORK/rb-complete.tsv"
    printf '%s\n' $'not-a-gate\t0\t0\tunknown'
} >"$WORK/rb-unknown.tsv"
{
    sed -n 'p' "$WORK/rb-complete.tsv"
    printf '%s\n' $'archcheck\t0\t0'
} >"$WORK/rb-malformed.tsv"
sed $'s/^codecheck\t1\t2\t/codecheck\t70\t2\t/' "$WORK/rb-complete.tsv" >"$WORK/rb-invalid-exit.tsv"
sed $'s/^cargo-test\t1\t3\t/cargo-test\t1\tnot-a-count\t/' "$WORK/rb-complete.tsv" >"$WORK/rb-invalid-count.tsv"
sed $'s/^archcheck\t0\t0\t/archcheck\t1\t1\t/' "$WORK/rb-complete.tsv" >"$WORK/rb-1-to-0.tsv"
sed $'s/^codecheck\t1\t2\t/codecheck\t0\t0\t/' "$WORK/rb-complete.tsv" >"$WORK/rb-0-to-1.tsv"

# N11 基线谎称某面是绿的，实测更红时必须报新回归，不得判一致。
expect "N11 基线谎称绿时报出新回归" 2 "比基线更红" -- \
    run_fake_compare "$WORK/rb-lie.tsv" 101 3

# N12 基线表读不到时判未覆盖，不得当作无回归。
expect "N12 基线表读不到判未覆盖" 3 "读不到基线表" -- \
    env EP_RED_BASELINE="$WORK/nosuch.tsv" bash "$COMPARE" --gates-only

# N13 基线表一行数据都没有时判未覆盖，不得因「零行零回归」判通过。
expect "N13 空基线表判未覆盖" 3 "缺少必需表头" -- \
    env EP_RED_BASELINE="$WORK/rb-empty.tsv" bash "$COMPARE" --gates-only

# N14 未知参数按用法错误处理，不得沉默地按默认路径跑下去。
expect "N14 未知参数报用法错误" 64 "用法错误" -- \
    bash "$COMPARE" --bogus

# N15 跳过某一行即未覆盖：--gates-only 跳过 cargo-test 后不得判无回归。
expect "N15 跳过的行判未覆盖" 3 "判定未做出" -- \
    run_fake_compare "$WORK/rb-complete.tsv" 101 3 --gates-only

# P4 对照组：基线与实测相等时必须返回 0——证明上面四条的非零不是恒非零。
expect "P4 基线与实测相等时返回 0" 0 "与已登记基线逐面相等" -- \
    run_fake_compare "$WORK/rb-complete.tsv" 101 3

# P4b fake cargo 对每次会解析依赖的 run/test 调用强制检查双旗标；缺任一项即以 64 拒绝。
expect "P4b 基线比较器的 Cargo 调用均 locked/offline" 0 "与已登记基线逐面相等" -- \
    env PATH="$FAKE_CARGO_DIR:$PATH" EP_CI_REAL_CARGO="$REAL_CARGO" \
    EP_CI_FAKE_REQUIRE_LOCKED_OFFLINE=1 EP_CI_FAKE_XTASK_RESULTS="$FAKE_XTASK_RESULTS" \
    EP_CI_FAKE_CARGO_TEST_RC=101 EP_CI_FAKE_CARGO_TEST_FAILED=3 \
    EP_RED_BASELINE="$WORK/rb-complete.tsv" bash "$COMPARE"

# N16--N21 的生产破坏分别是：缺失/重复/未知/截断行未被拒绝，或登记的
# exit/count 不再代表一次已测状态。它们必须在跑任何门禁前以未覆盖失败。
expect "N16 缺少必需判定面不能当作完整基线" 3 "缺少必需判定面" -- \
    env EP_RED_BASELINE="$WORK/rb-partial.tsv" bash "$COMPARE" --gates-only

expect "N17 重复判定面不能当作完整基线" 3 "重复出现" -- \
    env EP_RED_BASELINE="$WORK/rb-duplicate.tsv" bash "$COMPARE" --gates-only

expect "N18 未知判定面不能当作完整基线" 3 "未知判定面" -- \
    env EP_RED_BASELINE="$WORK/rb-unknown.tsv" bash "$COMPARE" --gates-only

expect "N19 截断基线行不能当作完整基线" 3 "行格式不正确" -- \
    env EP_RED_BASELINE="$WORK/rb-malformed.tsv" bash "$COMPARE" --gates-only

expect "N20 非测得出口不能作为基线证据" 3 "不是已测得状态" -- \
    env EP_RED_BASELINE="$WORK/rb-invalid-exit.tsv" bash "$COMPARE" --gates-only

expect "N21 非数字基线计数不能作为基线证据" 3 "期望值不是数字" -- \
    env EP_RED_BASELINE="$WORK/rb-invalid-count.tsv" bash "$COMPARE" --gates-only

# N22--N24 直接检验出口类别，而不是把 0/1/3/70 当作有序数字。
expect "N22 不符转通过是收窄" 0 "退出状态由不符转为通过" -- \
    run_fake_compare "$WORK/rb-1-to-0.tsv" 101 3

expect "N23 未覆盖压过同轮回归" 3 "实测取不到" -- \
    run_fake_compare "$WORK/rb-0-to-1.tsv" 70 3

expect "N24 实测未交付不能与基线比较" 3 "实测退出码 70 不属于可比状态" -- \
    env PATH="$FAKE_CARGO_DIR:$PATH" EP_CI_REAL_CARGO="$REAL_CARGO" EP_CI_FAKE_XTASK_RESULTS="$FAKE_XTASK_RESULTS" \
    EP_CI_FAKE_CARGO_70_GATE=archcheck EP_CI_FAKE_CARGO_TEST_RC=101 EP_CI_FAKE_CARGO_TEST_FAILED=3 \
    EP_RED_BASELINE="$WORK/rb-complete.tsv" bash "$COMPARE"

# N25--N27 验证 cargo-test 先看原始 cargo 退出码。即使伪造输出含合法的
# `test result` 与 failed 数，70/3/其他退出码都不可以被归一为已测红。
expect "N25 cargo-test 原始 70 不能归一为红" 3 "cargo-test 原始退出码 70" -- \
    run_fake_compare "$WORK/rb-complete.tsv" 70 3

expect "N26 cargo-test 原始 3 不能归一为红" 3 "cargo-test 原始退出码 3" -- \
    run_fake_compare "$WORK/rb-complete.tsv" 3 3

expect "N27 cargo-test 其他原始退出码不能归一为红" 3 "cargo-test 原始退出码 42" -- \
    run_fake_compare "$WORK/rb-complete.tsv" 42 3

expect "N28 cargo-test 原始 0 与零失败归一为通过" 0 "退出状态由不符转为通过" -- \
    run_fake_compare "$WORK/rb-complete.tsv" 0 0

expect "N29 缺少完整表头不能作为基线" 3 "缺少必需表头" -- \
    env EP_RED_BASELINE="$WORK/rb-no-header.tsv" bash "$COMPARE" --gates-only

# N30/N31 直接运行临时代理：近似 cargo run 不得被截获为 70，自身递归必须快速失败。
expect_not_rc "N30 代理不截获带额外参数的 cargo run" 70 -- \
    env EP_CI_REAL_CARGO="$REAL_CARGO" EP_CI_FAKE_CARGO_70_GATE=archcheck \
    "$FAKE_CARGO_DIR/cargo" run -q --locked --offline -p ep-xtask -- archcheck extra

expect "N31 代理拒绝把自身当作真实 cargo" 64 "与代理自身相同" -- \
    /usr/bin/perl -e 'alarm 2; exec @ARGV' env EP_CI_REAL_CARGO="$FAKE_CARGO_DIR/cargo" \
    "$FAKE_CARGO_DIR/cargo" --version

# N32 即使输出里仍有与基线相等的 failed 汇总，只要同轮出现 Cargo 编译失败，
# cargo-test 就没有形成完整可比证据，必须判未覆盖而不是“与基线一致”。
expect "N32 cargo-test 编译失败不能被既有 failed 数掩盖" 3 "输出含编译失败" -- \
    env PATH="$FAKE_CARGO_DIR:$PATH" EP_CI_REAL_CARGO="$REAL_CARGO" \
    EP_CI_FAKE_XTASK_RESULTS="$FAKE_XTASK_RESULTS" EP_CI_FAKE_CARGO_TEST_RC=101 \
    EP_CI_FAKE_CARGO_TEST_FAILED=3 EP_CI_FAKE_CARGO_TEST_COMPILE_FAILURE=1 \
    EP_RED_BASELINE="$WORK/rb-complete.tsv" bash "$COMPARE"

# N33 一条违规明细可以由仓库内容间接控制；若它伪装成较小的汇总行，比较器不得
# 取第一处数字并把新增红误报为收窄。真实汇总必须是唯一候选，否则证据未覆盖。
expect "N33 多个汇总候选不能用首行伪造计数" 3 "读不到计数" -- \
    env PATH="$FAKE_CARGO_DIR:$PATH" EP_CI_REAL_CARGO="$REAL_CARGO" \
    EP_CI_FAKE_XTASK_RESULTS="$FAKE_XTASK_RESULTS" EP_CI_FAKE_XTASK_DECOY_GATE=configdoc \
    EP_CI_FAKE_CARGO_TEST_RC=101 EP_CI_FAKE_CARGO_TEST_FAILED=3 \
    EP_RED_BASELINE="$WORK/rb-complete.tsv" bash "$COMPARE"

# Completed known failures cannot hide another target with no libtest completion.
for abnormal in signal not-executed; do
    expect "N34 cargo-test known reds plus $abnormal are uncovered" 3 "异常或未执行" -- \
        env PATH="$FAKE_CARGO_DIR:$PATH" EP_CI_REAL_CARGO="$REAL_CARGO" \
        EP_CI_FAKE_XTASK_RESULTS="$FAKE_XTASK_RESULTS" EP_CI_FAKE_CARGO_TEST_RC=101 \
        EP_CI_FAKE_CARGO_TEST_FAILED=3 EP_CI_FAKE_CARGO_TEST_ABNORMAL="$abnormal" \
        EP_RED_BASELINE="$WORK/rb-complete.tsv" bash "$COMPARE"
done

# ---- 结论 ------------------------------------------------------------------

echo
if [[ $failed -gt 0 ]]; then
    echo "负样例集：$passed 条如期失败，$unconstructible 条本轮不可构造，$failed 条未如期失败。" >&2
    exit 1
fi
echo "负样例集：$passed 条如期失败，$unconstructible 条本轮不可构造，$failed 条未如期失败。"
exit 0

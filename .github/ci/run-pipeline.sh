#!/usr/bin/env bash
# 按 pipeline-stages.tsv 逐阶段执行 D-07 的 11 个阶段，并把各命令的退出码归类。
#
# 这个脚本是 ADR-0005 决定二所指的聚合入口 `cargo xtask ci` 的临时替身：
# `cargo xtask ci` **已交付**（xtask/src/ci.rs，`cargo xtask` 用法行已列出 ci），
# workflow 默认调它；本脚本保留为备用调度器，读同一份登记表，只在需要绕开 Rust
# 入口时启用，两者都不作废。它只做退出码归类与汇总，不含任何门禁判定。
# （F-68 更正：原注逐字称「目前不被 xtask 受理」与「一旦交付本脚本连同登记表一并作废」，
# 两句都已被现实取代——实测 `cargo xtask ci` 受理且 ci.yml 第 52-53 行称二者互为备用。）
#
# 退出码归类，四类互不合并：
#   0  该命令通过
#   70 该门禁本阶段未交付（xtask 对未交付子命令的固定退出码）
#   3  该门禁存在不可判定项（archcheck 三态的第三态，计划明令 CI 不得当作通过）
#   其余非零 该门禁判不符
# 全局退出码取最重的一类：不符 1 > 不可判定 3 > 未交付 70 > 全通过 0。
# 未交付的步骤一律不写成 `|| true`，也不折算成通过：那正是静默放行。
set -uo pipefail

SELF_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$SELF_DIR/../.." && pwd)
ROSTER=${EP_CI_ROSTER:-$SELF_DIR/pipeline-stages.tsv}

EXIT_FAIL=1
EXIT_UNDECIDABLE=3
EXIT_UNDELIVERED=70
EXIT_USAGE=64

if [[ $# -gt 0 ]]; then
    echo "用法: $0（无参数）；登记表路径经 EP_CI_ROSTER 覆盖" >&2
    exit "$EXIT_USAGE"
fi

if [[ ! -r $ROSTER ]]; then
    echo "登记表 $ROSTER 读不到，流水线无法调度。" >&2
    exit "$EXIT_FAIL"
fi

rows=$(grep -v -e '^[[:space:]]*#' -e '^[[:space:]]*$' "$ROSTER" || true)
if [[ -z $rows ]]; then
    echo "登记表 $ROSTER 无有效行，流水线无法调度。" >&2
    exit "$EXIT_FAIL"
fi

n_pass=0
n_undelivered=0
n_undecidable=0
n_fail=0
summary=""

cd "$REPO_ROOT"

while IFS=$'\t' read -r stage id kind argv status; do
    [[ -z ${stage:-} ]] && continue

    # shellcheck disable=SC2206  # 登记表的参数列有意按空白拆成 argv
    args=($argv)
    case $kind in
    cargo) cmdline=(cargo "${args[@]}") ;;
    xtask) cmdline=(cargo xtask "${args[@]}") ;;
    script)
        rel=${args[0]}
        cmdline=(bash "$REPO_ROOT/$rel" "${args[@]:1}")
        ;;
    *)
        echo "阶段 ${stage}（${id}）：命令类别 $kind 不认识，无法调度。" >&2
        n_fail=$((n_fail + 1))
        summary+="  阶段 $stage $id  $kind $argv  →  无法调度"$'\n'
        continue
        ;;
    esac

    echo "::: 阶段 ${stage}（${id}） ${cmdline[*]}"
    rc=0
    "${cmdline[@]}" || rc=$?

    case $rc in
    0)
        n_pass=$((n_pass + 1))
        verdict="通过"
        ;;
    "$EXIT_UNDELIVERED")
        n_undelivered=$((n_undelivered + 1))
        verdict="本阶段未交付"
        ;;
    "$EXIT_UNDECIDABLE")
        n_undecidable=$((n_undecidable + 1))
        verdict="存在不可判定项"
        ;;
    *)
        n_fail=$((n_fail + 1))
        verdict="不符（退出码 ${rc}）"
        ;;
    esac

    # 登记表说已交付而工具报未交付，是登记表本身失真，按不符计。
    if [[ $status == delivered && $rc -eq $EXIT_UNDELIVERED ]]; then
        # 该行已在上面的 EXIT_UNDELIVERED 分支计过一次未交付，此处改判为不符时
        # 必须把那一次退回，否则一条命令会在汇总行里同时算进两类（F-68）。
        n_undelivered=$((n_undelivered - 1))
        n_fail=$((n_fail + 1))
        verdict="登记为已交付却报未交付，登记表失真"
    fi

    summary+="  阶段 $stage $id  ${cmdline[*]}  →  $verdict"$'\n'
done <<<"$rows"

echo
echo "==== D-07 流水线汇总 ===="
printf '%s' "$summary"
echo "通过 $n_pass 条，未交付 $n_undelivered 条，不可判定 $n_undecidable 条，不符 $n_fail 条。"

if [[ $n_fail -gt 0 ]]; then
    echo "结论：有门禁判不符，流水线红。" >&2
    exit "$EXIT_FAIL"
fi
if [[ $n_undecidable -gt 0 ]]; then
    echo "结论：仍有不可判定项，按计划第 10 节退出条件 27，不得当作通过。" >&2
    exit "$EXIT_UNDECIDABLE"
fi
if [[ $n_undelivered -gt 0 ]]; then
    echo "结论：$n_undelivered 条门禁本阶段未交付，D-07 的「返回 0」尚不成立。" >&2
    exit "$EXIT_UNDELIVERED"
fi
# 不写死 11：只跑了一部分登记表时（负样例与排障都会这么跑）这句会变成假话。
echo "结论：本次执行的 $((n_pass + n_undelivered + n_undecidable + n_fail)) 条命令与登记表一致。"
exit 0

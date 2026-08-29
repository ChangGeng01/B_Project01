#!/usr/bin/env bash
# 把「已登记的基线红」与「新回归」分开。
#
# 登记红存在期间流水线整体报红，聚合退出码取最重，开发者看不出哪一处红是自己造成的。
# 本脚本按 known-red-baseline.tsv 逐面实测、逐面对照，只回答一个问题：
#   比基线**多**出来的红有没有？
#
# 本脚本不是门禁：它不在 pipeline-stages.tsv 里，不进 ci.yml，不使任何门禁变绿。
#
# 退出码沿用 verify-pipeline-commands.sh 已有的一套，不另立第二套：
#   0  无新回归（收窄单列打印）
#   2  有新回归（某面计数超基线，或退出码与登记不同）
#   3  未覆盖（有面的实测取不到，或用 --gates-only 跳过了 cargo-test 行）
#   64 用法错误
# 「取不到」与「不符」必须是两个不同的码：取不到时一律不得判无回归。
set -euo pipefail

EXIT_OK=0
EXIT_REGRESSION=2
EXIT_UNCOVERED=3
EXIT_USAGE=64

SELF_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$SELF_DIR/../.." && pwd)

# 替代路径只为负样例存在：不设时取仓库内的真实基线表。
BASELINE=${EP_RED_BASELINE:-$SELF_DIR/known-red-baseline.tsv}

GATES_ONLY=0
case "${1:-}" in
    "") ;;
    --gates-only) GATES_ONLY=1 ;;
    -h | --help)
        echo "用法：$0 [--gates-only]"
        echo "  不带参数：连 cargo-test 行一起实测（慢，但判定完整）"
        echo "  --gates-only：只测 xtask 各面，跳过 cargo-test；跳过即未覆盖，退出码 3"
        exit "$EXIT_OK"
        ;;
    *)
        echo "用法错误：未知参数 ${1}" >&2
        exit "$EXIT_USAGE"
        ;;
esac

if [[ ! -r $BASELINE ]]; then
    echo "未覆盖：读不到基线表 $BASELINE，判定未做出，不得视为无回归。" >&2
    exit "$EXIT_UNCOVERED"
fi

# 实测一道 xtask 判定面，回显「退出码 计数」；计数取不到时回显「退出码 -」。
measure_gate() {
    local gate=$1 out rc count
    set +e
    out=$(cd "$REPO_ROOT" && cargo run -q -p ep-xtask -- "$gate" 2>&1 </dev/null)
    rc=$?
    set -e
    # 退出码 0 的面没有不符行，计数即 0。
    if [[ $rc -eq 0 ]]; then
        echo "0 0"
        return
    fi
    # 退出码非 0 时必须读到汇总行，否则判定取不到——不得当作 0。
    # 不用 `| head -1`／`grep -q`：pipefail 下游提前退出会给上游 SIGPIPE，
    # 整条管道返回 141，判定会被误当成「读不到」。一律用 herestring 加 `|| true`。
    count=$(grep -oE '^(不符|不一致)（[0-9]+ 处）' <<<"$out" | grep -oE '[0-9]+' || true)
    count=${count%%$'\n'*}
    if [[ -z $count ]]; then
        echo "$rc -"
        return
    fi
    echo "$rc $count"
}

# 实测 cargo test 失败数，回显「退出码 计数」。
measure_tests() {
    local out rc count
    set +e
    out=$(cd "$REPO_ROOT" && cargo test -q --workspace --no-fail-fast 2>&1 </dev/null)
    rc=$?
    set -e
    # 同上：herestring，不用 `printf | grep -q`。
    if ! grep -q '^test result' <<<"$out"; then
        echo "$rc -"
        return
    fi
    count=$(grep -oE '[0-9]+ failed' <<<"$out" | grep -oE '[0-9]+' |
        awk '{s += $1} END {print s + 0}' || true)
    [[ -z $count ]] && count=0
    # 一个失败都没有时 cargo 退出 0；此处把退出码归一到「有失败即 1」，与基线表同口径。
    if [[ $count -gt 0 ]]; then echo "1 $count"; else echo "0 0"; fi
}

regressions=0
uncovered=0
narrowed=0
checked=0

printf '%-14s %-10s %-10s %s\n' "判定面" "基线" "实测" "结论"
printf -- '---------------------------------------------------------------\n'

while IFS=$'\t' read -r gate expect_exit expect_count _note; do
    [[ -z $gate || $gate == \#* || $gate == "gate" ]] && continue

    if [[ $gate == "cargo-test" ]]; then
        if [[ $GATES_ONLY -eq 1 ]]; then
            printf '%-14s %-10s %-10s %s\n' "$gate" "${expect_exit}/${expect_count}" "跳过" \
                "未覆盖：--gates-only 跳过，判定未做出"
            uncovered=$((uncovered + 1))
            continue
        fi
        read -r got_exit got_count <<<"$(measure_tests)"
    else
        read -r got_exit got_count <<<"$(measure_gate "$gate")"
    fi

    checked=$((checked + 1))

    if [[ $got_count == "-" ]]; then
        printf '%-14s %-10s %-10s %s\n' "$gate" "${expect_exit}/${expect_count}" "${got_exit}/?" \
            "未覆盖：退出码 ${got_exit} 但读不到计数，判定未做出"
        uncovered=$((uncovered + 1))
        continue
    fi

    # 退出码集合无序（0 通过 / 1 不符 / 3 判定未做出 / 70 未交付），不得用大小比较：
    # `-gt` 抓得到 1→3，却抓不到 3→1（未覆盖恶化成确凿不符）。只判是否与登记相同。
    if [[ $got_count -gt $expect_count || $got_exit -ne $expect_exit ]]; then
        printf '%-14s %-10s %-10s %s\n' "$gate" "${expect_exit}/${expect_count}" "${got_exit}/${got_count}" \
            "**新回归**：退出码 ${got_exit}（登记 ${expect_exit}）／计数 ${got_count}（登记 ${expect_count}）"
        regressions=$((regressions + 1))
    elif [[ $got_count -lt $expect_count ]]; then
        printf '%-14s %-10s %-10s %s\n' "$gate" "${expect_exit}/${expect_count}" "${got_exit}/${got_count}" \
            "收窄 $((expect_count - got_count)) 处：须更新基线表并留证"
        narrowed=$((narrowed + 1))
    else
        printf '%-14s %-10s %-10s %s\n' "$gate" "${expect_exit}/${expect_count}" "${got_exit}/${got_count}" \
            "与基线一致"
    fi
done <"$BASELINE"

if [[ $checked -eq 0 ]]; then
    echo "未覆盖：基线表 $BASELINE 里一行都没读到，判定未做出。" >&2
    exit "$EXIT_UNCOVERED"
fi

echo
if [[ $regressions -gt 0 ]]; then
    echo "结论：${regressions} 个判定面比基线更红——这些是本次改动引入的，与登记红无关。" >&2
    exit "$EXIT_REGRESSION"
fi
if [[ $uncovered -gt 0 ]]; then
    echo "结论：${uncovered} 个判定面的实测取不到，判定未做出，不得视为无回归。" >&2
    exit "$EXIT_UNCOVERED"
fi
if [[ $narrowed -gt 0 ]]; then
    echo "结论：无新回归；${narrowed} 个判定面已收窄，请更新 known-red-baseline.tsv 并留证。"
    exit "$EXIT_OK"
fi
echo "结论：无新回归，各判定面与已登记基线逐面相等。"
exit "$EXIT_OK"

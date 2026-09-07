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
    echo "未覆盖：读不到基线表 ${BASELINE}，判定未做出，不得视为无回归。" >&2
    exit "$EXIT_UNCOVERED"
fi

# 基线表本身是证据，不是配置默认值。先完整校验，任何一条不可信都不得开始实测：
# 否则半张表或一条伪造的 3/70 会把未测面悄悄折算成「没有新回归」。
EXPECTED_GATES=(archcheck sqlcheck codecheck errorcodes configdoc eventcatalog cargo-test)
EXPECT_EXIT=()
EXPECT_COUNT=()
SEEN=()

reject_baseline() {
    echo "未覆盖：基线表 ${BASELINE} $1，判定未做出，不得视为无回归。" >&2
    exit "$EXIT_UNCOVERED"
}

line_number=0
header_count=0
while IFS= read -r line || [[ -n $line ]]; do
    line_number=$((line_number + 1))
    [[ -z ${line//[[:space:]]/} || $line == \#* ]] && continue

    if [[ $line == $'gate\texpect_exit\texpect_count\tnote' ]]; then
        header_count=$((header_count + 1))
        [[ $header_count -eq 1 ]] || reject_baseline "第 ${line_number} 行表头重复出现"
        continue
    fi

    tab_characters=${line//[^$'\t']/}
    [[ ${#tab_characters} -eq 3 ]] || reject_baseline "第 ${line_number} 行格式不正确"
    IFS=$'\t' read -r gate expect_exit expect_count _note <<<"$line"

    gate_index=-1
    for index in "${!EXPECTED_GATES[@]}"; do
        if [[ $gate == "${EXPECTED_GATES[$index]}" ]]; then
            gate_index=$index
            break
        fi
    done
    [[ $gate_index -ge 0 ]] || reject_baseline "第 ${line_number} 行含未知判定面 ${gate}"
    [[ ${SEEN[$gate_index]:-0} -eq 0 ]] || reject_baseline "判定面 ${gate} 重复出现"

    if ! [[ $expect_exit =~ ^[0-9]+$ && $expect_count =~ ^[0-9]+$ ]]; then
        reject_baseline "判定面 ${gate} 的期望值不是数字"
    fi
    case "${expect_exit}/${expect_count}" in
        0/0 | 1/[1-9]*) ;;
        *) reject_baseline "判定面 ${gate} 的出口/计数不是已测得状态（只接受 0/0 或 1/正计数）" ;;
    esac

    SEEN[$gate_index]=1
    EXPECT_EXIT[$gate_index]=$expect_exit
    EXPECT_COUNT[$gate_index]=$expect_count
done <"$BASELINE"

[[ $header_count -eq 1 ]] || reject_baseline "缺少必需表头 gate/expect_exit/expect_count/note"

for index in "${!EXPECTED_GATES[@]}"; do
    [[ ${SEEN[$index]:-0} -eq 1 ]] || reject_baseline "缺少必需判定面 ${EXPECTED_GATES[$index]}"
done

# 两个正整数字符串的大小关系。避免把外部表中的数喂给 shell 算术；长度相同时
# 的字典序就是十进制数值序。
compare_counts() {
    local left=$1 right=$2
    if [[ ${#left} -lt ${#right} ]]; then
        printf 'less\n'
    elif [[ ${#left} -gt ${#right} ]]; then
        printf 'greater\n'
    elif [[ $left < $right ]]; then
        printf 'less\n'
    elif [[ $left > $right ]]; then
        printf 'greater\n'
    else
        printf 'equal\n'
    fi
}

# 实测一道 xtask 判定面，回显「退出码 计数」；计数取不到时回显「退出码 -」。
measure_gate() {
    local gate=$1 out rc count summary_lines summary_count
    set +e
    out=$(cd "$REPO_ROOT" && cargo run -q --locked --offline -p ep-xtask -- "$gate" 2>&1 </dev/null)
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
    # 三种汇总行都要认：configdoc/codecheck/eventcatalog 印「不符（N 处）」、
    # errorcodes 印「不一致（N 处）」、archcheck 印「违反明细（N 处）」（xtask/src/main.rs）。
    # 少认一种就会把该面的回归判成「未覆盖」而不是「新回归」——退出码 3 不是 2，
    # 读结论的人会以为只是没测到（F-73 补 archcheck 一种）。
    # 只接受工具自己的整行汇总，且必须恰有一行。若违规明细或被测文档中
    # 恰好含有“ 不符（N 处）”字样，不能让它抢在真实汇总前面伪造计数；
    # 多个候选同样是歧义证据，失败关闭。
    summary_lines=$(grep -E '^(不符|不一致|违反明细)（[0-9]+ 处）：$' <<<"$out" || true)
    summary_count=$(awk 'NF { n += 1 } END { print n + 0 }' <<<"$summary_lines")
    if [[ $summary_count -ne 1 ]]; then
        echo "$rc -"
        return
    fi
    count=$(grep -oE '[0-9]+' <<<"$summary_lines")
    echo "$rc $count"
}

# 实测 cargo test 失败数，回显「退出码 计数」。
measure_tests() {
    local out rc count
    set +e
    out=$(cd "$REPO_ROOT" && cargo test -q --workspace --locked --offline --no-fail-fast 2>&1 </dev/null)
    rc=$?
    set -e
    # cargo test 的原始退出码也是证据的一部分：只有「正常通过」0 和
    # 「测试失败」101 允许归一，未交付/不可判定/进程异常绝不可借一行测试汇总
    # 伪装成已测红。
    case $rc in
        0 | 101) ;;
        *) echo "$rc -"; return ;;
    esac
    # Cargo 的编译失败与测试失败都可能使用 101。只凭后面残留的一行
    # `N failed` 归一会把不完整的构建当成一次可比测试结果，故显式失败关闭。
    if grep -q 'error: could not compile' <<<"$out"; then
        echo "$rc compile-failed"
        return
    fi
    # --no-fail-fast 可先输出旧失败的完整汇总，再有另一个 target 崩溃或
    # 根本未执行。任何进程失败诊断均失败关闭；普通已完成 libtest 的
    # `error: test failed, to rerun pass ...` 不属于这一类。
    # 这是对 Cargo 文本诊断的保守检查，不是结构化的逐 target 完成协议。
    if grep -Eq 'process didn.t exit successfully|process did not exit successfully|could not execute process|could not execute test|failed to (execute|run) (process|test)|never executed|\(signal: [0-9]+|test (binary|executable).*not found' <<<"$out"; then
        echo "$rc abnormal-execution"
        return
    fi
    # 同上：herestring，不用 `printf | grep -q`。
    if ! grep -q '^test result' <<<"$out"; then
        echo "$rc -"
        return
    fi
    count=$(grep -oE '[0-9]+ failed' <<<"$out" | grep -oE '[0-9]+' |
        awk '{s += $1} END {print s + 0}' || true)
    [[ -z $count ]] && count=0
    # 只接受可测的二元组合；例如 rc=0 却带失败数，或 rc=101 却没有失败数，
    # 都是被截断/伪造的证据，保持原始退出码让主循环判为未覆盖。
    case "$rc/$count" in
        0/0) echo "0 0" ;;
        101/[1-9]*) echo "1 $count" ;;
        *) echo "$rc -" ;;
    esac
}

regressions=0
uncovered=0
narrowed=0
checked=0

printf '%-14s %-10s %-10s %s\n' "判定面" "基线" "实测" "结论"
printf -- '---------------------------------------------------------------\n'

for index in "${!EXPECTED_GATES[@]}"; do
    gate=${EXPECTED_GATES[$index]}
    expect_exit=${EXPECT_EXIT[$index]}
    expect_count=${EXPECT_COUNT[$index]}
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

    if [[ $gate == "cargo-test" && $got_count == "compile-failed" ]]; then
        printf '%-14s %-10s %-10s %s\n' "$gate" "${expect_exit}/${expect_count}" "${got_exit}/?" \
            "未覆盖：cargo-test 输出含编译失败，测试结果不是完整可比证据"
        uncovered=$((uncovered + 1))
        continue
    fi
    if [[ $gate == "cargo-test" && $got_count == "abnormal-execution" ]]; then
        printf '%-14s %-10s %-10s %s\n' "$gate" "${expect_exit}/${expect_count}" "${got_exit}/?" \
            "未覆盖：cargo-test 输出含进程异常或未执行，测试结果不是完整可比证据"
        uncovered=$((uncovered + 1))
        continue
    fi

    # 实测出口没有顺序：只有 0（通过）和 1（不符）可与已登记测量相比较。
    # 3/70/其他值都表示这次判定没有拿到可用证据，不能因数字恰好较大/较小而
    # 被误报为回归或收窄。
    if [[ $got_exit != 0 && $got_exit != 1 ]]; then
        if [[ $gate == "cargo-test" ]]; then
            detail="未覆盖：cargo-test 原始退出码 ${got_exit} 不可归一，判定未做出"
        else
            detail="未覆盖：实测退出码 ${got_exit} 不属于可比状态，判定未做出"
        fi
        printf '%-14s %-10s %-10s %s\n' "$gate" "${expect_exit}/${expect_count}" "${got_exit}/?" \
            "$detail"
        uncovered=$((uncovered + 1))
        continue
    fi

    if [[ $got_count == "-" || ! $got_count =~ ^[0-9]+$ ]]; then
        printf '%-14s %-10s %-10s %s\n' "$gate" "${expect_exit}/${expect_count}" "${got_exit}/?" \
            "未覆盖：退出码 ${got_exit} 但读不到计数，判定未做出"
        uncovered=$((uncovered + 1))
        continue
    fi

    if [[ $got_exit == 0 && $got_count != 0 ]]; then
        printf '%-14s %-10s %-10s %s\n' "$gate" "${expect_exit}/${expect_count}" "${got_exit}/${got_count}" \
            "未覆盖：实测通过却带有非零计数，判定未做出"
        uncovered=$((uncovered + 1))
        continue
    fi
    if [[ $got_exit == 1 && ! $got_count =~ ^[1-9][0-9]*$ ]]; then
        printf '%-14s %-10s %-10s %s\n' "$gate" "${expect_exit}/${expect_count}" "${got_exit}/${got_count}" \
            "未覆盖：实测不符却没有正计数，判定未做出"
        uncovered=$((uncovered + 1))
        continue
    fi

    if [[ $expect_exit == 1 && $got_exit == 0 ]]; then
        printf '%-14s %-10s %-10s %s\n' "$gate" "${expect_exit}/${expect_count}" "${got_exit}/${got_count}" \
            "收窄：退出状态由不符转为通过，须更新基线表并留证"
        narrowed=$((narrowed + 1))
    elif [[ $expect_exit == 0 && $got_exit == 1 ]]; then
        printf '%-14s %-10s %-10s %s\n' "$gate" "${expect_exit}/${expect_count}" "${got_exit}/${got_count}" \
            "**新回归**：退出状态由通过变为不符／计数 ${got_count}（登记 ${expect_count}）"
        regressions=$((regressions + 1))
    else
        relation=$(compare_counts "$got_count" "$expect_count")
        case $relation in
            greater)
                printf '%-14s %-10s %-10s %s\n' "$gate" "${expect_exit}/${expect_count}" "${got_exit}/${got_count}" \
                    "**新回归**：不符计数 ${got_count}（登记 ${expect_count}）"
                regressions=$((regressions + 1))
                ;;
            less)
                printf '%-14s %-10s %-10s %s\n' "$gate" "${expect_exit}/${expect_count}" "${got_exit}/${got_count}" \
                    "收窄：不符计数 ${got_count}（登记 ${expect_count}），须更新基线表并留证"
                narrowed=$((narrowed + 1))
                ;;
            equal)
                printf '%-14s %-10s %-10s %s\n' "$gate" "${expect_exit}/${expect_count}" "${got_exit}/${got_count}" \
                    "与基线一致"
                ;;
        esac
    fi
done

if [[ $checked -eq 0 ]]; then
    echo "未覆盖：基线表 $BASELINE 里一行都没读到，判定未做出。" >&2
    exit "$EXIT_UNCOVERED"
fi

echo
if [[ $uncovered -gt 0 ]]; then
    echo "结论：${uncovered} 个判定面的实测取不到，判定未做出，不得视为无回归。" >&2
    exit "$EXIT_UNCOVERED"
fi
if [[ $regressions -gt 0 ]]; then
    echo "结论：${regressions} 个判定面比基线更红——这些是本次改动引入的，与登记红无关。" >&2
    exit "$EXIT_REGRESSION"
fi
if [[ $narrowed -gt 0 ]]; then
    echo "结论：无新回归；${narrowed} 个判定面已收窄，请更新 known-red-baseline.tsv 并留证。"
    exit "$EXIT_OK"
fi
echo "结论：无新回归，各判定面与已登记基线逐面相等。"
exit "$EXIT_OK"

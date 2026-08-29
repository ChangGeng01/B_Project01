#!/usr/bin/env bash
# 判定 CI 流水线登记表引用的每一条命令在本机是否真的存在且可执行，并判定
# 登记表自身与 docs/ci-pipeline.md 的阶段表逐行相等。
#
# 退出码沿用 scripts/verify-resource-limits.sh 已有的一套，不另立第二套：
#   0  全部就位
#   2  不符（读到了被测对象，但取值不对）
#   3  未覆盖（读不到被测对象，判定没有做出）
#   64 用法错误
# 「读不到」与「不符」必须是两个不同的码：读不到被测对象时一律不得判通过。
set -euo pipefail

EXIT_OK=0
EXIT_MISMATCH=2
EXIT_UNCOVERED=3
EXIT_USAGE=64

SELF_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$SELF_DIR/../.." && pwd)

# 两处替代路径只为负样例存在：不设时取仓库内的真实登记表与真实文档。
ROSTER=${EP_CI_ROSTER:-$SELF_DIR/pipeline-stages.tsv}
DOC=${EP_CI_DOC:-$REPO_ROOT/docs/ci-pipeline.md}

# D-07 把阶段数定死为 11，计数漂移在本项目已复发多次，此处做成可机检的。
STAGE_COUNT_EXPECTED=11

if [[ $# -gt 0 ]]; then
    echo "用法: $0（无参数）；登记表与文档路径经 EP_CI_ROSTER 与 EP_CI_DOC 覆盖" >&2
    exit "$EXIT_USAGE"
fi

mismatch=0   # 计「不符」
uncovered=0  # 计「未覆盖」

note_mismatch() { echo "不符　　 $1" >&2; mismatch=$((mismatch + 1)); }
note_uncovered() { echo "未覆盖　 $1" >&2; uncovered=$((uncovered + 1)); }

# ---- 一、读登记表 ----------------------------------------------------------

if [[ ! -r $ROSTER ]]; then
    echo "未覆盖　 登记表 $ROSTER 读不到，命令存在性判定未做出" >&2
    exit "$EXIT_UNCOVERED"
fi

rows=$(grep -v -e '^[[:space:]]*#' -e '^[[:space:]]*$' "$ROSTER" || true)
if [[ -z $rows ]]; then
    echo "未覆盖　 登记表 $ROSTER 无有效行，命令存在性判定未做出" >&2
    exit "$EXIT_UNCOVERED"
fi

# ---- 二、取 xtask 实际受理的子命令表 ---------------------------------------
# 不把子命令表抄一份在本脚本里：抄一份就会与工具漂移，这里直接读工具自己的用法行。

xtask_subcommands=""
xtask_usage=$(cd "$REPO_ROOT" && cargo xtask 2>&1 || true)
if [[ $xtask_usage == *"cargo xtask <"* ]]; then
    xtask_subcommands=$(printf '%s\n' "$xtask_usage" \
        | sed -n 's/.*cargo xtask <\([^>]*\)>.*/\1/p' \
        | tr '|' '\n' | tr -d ' ')
fi
if [[ -z $xtask_subcommands ]]; then
    note_uncovered "读不出 cargo xtask 受理的子命令表，全部 xtask 类命令的存在性判定未做出"
fi

# ---- 三、逐行判定 ----------------------------------------------------------

stage_ids=""     # 「阶段号 空格 阶段 id」去重后的清单，用于与文档比对
checked_xtask="" # 已核对过状态的 xtask 子命令，避免重复执行

while IFS=$'\t' read -r stage id kind argv status; do
    [[ -z ${stage:-} ]] && continue
    label="阶段 ${stage}（${id}）的 $kind 命令「${argv}」"

    if [[ ! $stage =~ ^[0-9]+$ ]]; then
        note_mismatch "登记表首列 $stage 不是阶段号"
        continue
    fi
    case $status in
        delivered | undelivered) ;;
        *) note_mismatch "$label 的门禁状态 $status 不在 {delivered, undelivered} 内" ;;
    esac

    entry="$stage $id"
    case " $stage_ids " in
        *" $entry "*) ;;
        *) stage_ids="$stage_ids $entry" ;;
    esac

    case $kind in
    cargo)
        if ! command -v cargo >/dev/null 2>&1; then
            note_mismatch "${label}：本机没有可执行的 cargo"
        fi
        ;;
    script)
        rel=${argv%% *}
        path="$REPO_ROOT/$rel"
        if [[ ! -e $path ]]; then
            note_mismatch "${label}：$rel 在仓库内不存在"
        elif [[ ! -x $path ]]; then
            note_mismatch "${label}：$rel 存在但没有可执行位"
        fi
        ;;
    xtask)
        sub=${argv%% *}
        if [[ -z $xtask_subcommands ]]; then
            : # 子命令表读不到，已在上面记过一次未覆盖，不重复记
        elif ! printf '%s\n' "$xtask_subcommands" | grep -qx "$sub"; then
            note_mismatch "${label}：cargo xtask 不受理子命令 $sub"
        else
            # 核对门禁状态：真跑一次，看工具是否如登记表所称。
            case " $checked_xtask " in
            *" $sub "*) ;;
            *)
                checked_xtask="$checked_xtask $sub"
                rc=0
                # 带上登记表里的全部参数再跑。只取首词元会让
                # `e2e --profile=t0` 实跑成 `cargo xtask e2e`，那是一次缺必填选项的
                # 用法错误（退出码 2），既非 70 也就被静默放行——本表宣称的
                # 「真跑一次」于是名不副实（F-68）。
                read -r -a sub_args <<<"$argv"
                (cd "$REPO_ROOT" && cargo xtask "${sub_args[@]}" >/dev/null 2>&1) || rc=$?
                if [[ $status == undelivered && $rc -ne 70 ]]; then
                    note_mismatch "$label 登记为未交付，但 cargo xtask $sub 退出码为 $rc 而不是 70"
                elif [[ $status == delivered && $rc -eq 70 ]]; then
                    note_mismatch "$label 登记为已交付，但 cargo xtask $sub 报未交付（70）"
                fi
                ;;
            esac
        fi
        ;;
    *)
        note_uncovered "${label}：命令类别 $kind 不认识，该行的存在性判定未做出"
        ;;
    esac
done <<<"$rows"

# ---- 四、阶段计数 ----------------------------------------------------------

# stage_ids 是「阶段号 阶段 id」成对写入的，两个词元算一个阶段。
stage_count=$(( $(printf '%s\n' "$stage_ids" | wc -w | tr -d ' ') / 2 ))
if [[ $stage_count -ne $STAGE_COUNT_EXPECTED ]]; then
    note_mismatch "登记表共 $stage_count 个阶段，D-07 定死为 $STAGE_COUNT_EXPECTED 个"
fi

# ---- 五、与文档的阶段表逐行比对 --------------------------------------------

if [[ ! -r $DOC ]]; then
    note_uncovered "文档 $DOC 读不到，阶段表一致性判定未做出"
else
    doc_pairs=$(sed -n 's/^|[[:space:]]*\([0-9][0-9]*\)[[:space:]]*|[[:space:]]*`\([a-z0-9-]*\)`[[:space:]]*|.*/\1 \2/p' "$DOC")
    if [[ -z $doc_pairs ]]; then
        note_uncovered "文档 $DOC 中找不到阶段表，一致性判定未做出"
    else
        roster_pairs=$(printf '%s\n' "$stage_ids" | tr ' ' '\n' | awk 'NF' | paste -d' ' - - | sort -n)
        doc_sorted=$(printf '%s\n' "$doc_pairs" | sort -n)
        if [[ $roster_pairs != "$doc_sorted" ]]; then
            note_mismatch "登记表与文档 $DOC 的阶段表不相等"
            diff <(printf '%s\n' "$roster_pairs") <(printf '%s\n' "$doc_sorted") >&2 || true
        fi
    fi
fi

# ---- 六、结论 --------------------------------------------------------------
# 「不符」压过「未覆盖」：有确凿的不符时报 2，只有读不到时报 3，两者都无才是 0。

if [[ $mismatch -gt 0 ]]; then
    echo "结论：不符 $mismatch 处，未覆盖 $uncovered 处。" >&2
    exit "$EXIT_MISMATCH"
fi
if [[ $uncovered -gt 0 ]]; then
    echo "结论：未覆盖 $uncovered 处，判定未做出，不得视为通过。" >&2
    exit "$EXIT_UNCOVERED"
fi
echo "登记表 $stage_count 个阶段引用的全部命令在本机存在且可执行，与文档阶段表逐行相等。"
exit "$EXIT_OK"

#!/usr/bin/env bash
# 仅供 CI 负样例临时工作区使用。普通调用必须穿透到真实 cargo；只有明确指定的
# `cargo xtask <gate>` 才模拟未交付的固定退出码 70。
set -euo pipefail

: "${EP_CI_REAL_CARGO:?EP_CI_REAL_CARGO must name the real cargo executable}"

canonical_path() {
    local path=$1 directory
    directory=$(cd -P -- "$(dirname -- "$path")" && pwd -P)
    printf '%s/%s\n' "$directory" "$(basename -- "$path")"
}

self_path=$(canonical_path "$0")
real_path=$(canonical_path "$EP_CI_REAL_CARGO")
if [[ $self_path == "$real_path" ]]; then
    echo "EP_CI_REAL_CARGO 与代理自身相同，拒绝递归执行。" >&2
    exit 64
fi

if [[ -n ${EP_CI_FAKE_CARGO_LOG:-} ]]; then
    printf '%s\n' "$*" >>"$EP_CI_FAKE_CARGO_LOG"
fi

if [[ ${EP_CI_FAKE_REQUIRE_LOCKED_OFFLINE:-0} == 1 ]]; then
    case " ${1:-} " in
        " run " | " test ")
            case " $* " in
                *" --locked "*) ;;
                *) echo "fake cargo: dependency-resolving invocation lacks --locked: $*" >&2; exit 64 ;;
            esac
            case " $* " in
                *" --offline "*) ;;
                *) echo "fake cargo: dependency-resolving invocation lacks --offline: $*" >&2; exit 64 ;;
            esac
            ;;
    esac
fi

selected_gate=${EP_CI_FAKE_CARGO_70_GATE:-}
selected_argv=${EP_CI_FAKE_CARGO_70_ARGV:-}
matched_gate=""
if [[ $# -eq 2 && ${1:-} == "xtask" ]]; then
    matched_gate=$2
fi

# verify-pipeline-commands.sh 的参数去重负样例需要精确命中一条
# `cargo xtask <subcommand> <flag>`，不能连同名的无旗标入口一起拦截。
if [[ -n $selected_argv && ${1:-} == "xtask" ]]; then
    actual_argv="${*:2}"
    if [[ $actual_argv == "$selected_argv" ]]; then
        exit 70
    fi
fi

# compare-red-baseline.sh 实测 xtask 时使用 `cargo run … -- <gate>`，不是
# `cargo xtask <gate>`；只有目标包、分隔符后恰好一个目标门禁的调用才可截获。
if [[ -z $matched_gate && ${1:-} == "run" ]]; then
    args=("$@")
    package_is_xtask=0
    for ((index = 1; index < ${#args[@]}; index++)); do
        if [[ ${args[$index]} == "-p" && ${args[$((index + 1))]:-} == "ep-xtask" ]]; then
            package_is_xtask=1
        fi
        if [[ ${args[$index]} == "--" ]]; then
            if [[ $package_is_xtask -eq 1 && $((index + 2)) -eq ${#args[@]} && ${args[$((index + 1))]:-} == "$selected_gate" ]]; then
                matched_gate=${args[$((index + 1))]}
            elif [[ $package_is_xtask -eq 1 && $((index + 2)) -eq ${#args[@]} ]]; then
                matched_gate=${args[$((index + 1))]}
            fi
            break
        fi
    done
fi

if [[ -n $selected_gate && $matched_gate == "$selected_gate" ]]; then
    exit 70
fi

# 受控形式为 `gate=exit/count;…`。只要调用形状不是上面两种精确 xtask
# 形状，本表永远不会生效，故普通 cargo run 仍会穿透到真实 cargo。
if [[ -n $matched_gate && -n ${EP_CI_FAKE_XTASK_RESULTS:-} ]]; then
    IFS=';' read -r -a result_rows <<<"$EP_CI_FAKE_XTASK_RESULTS"
    for result_row in "${result_rows[@]}"; do
        [[ ${result_row%%=*} == "$matched_gate" ]] || continue
        result=${result_row#*=}
        result_exit=${result%%/*}
        result_count=${result#*/}
        case "${result_exit}/${result_count}" in
            0/0) exit 0 ;;
            1/[1-9]*)
                if [[ ${EP_CI_FAKE_XTASK_DECOY_GATE:-} == "$matched_gate" ]]; then
                    printf '不符（1 处）：\n'
                fi
                printf '不符（%s 处）：\n' "$result_count"
                exit 1
                ;;
            *)
                echo "EP_CI_FAKE_XTASK_RESULTS 为 ${matched_gate} 给出非法状态 ${result}" >&2
                exit 64
                ;;
        esac
    done
fi

# 仅让负样例精确构造 `cargo test -q --workspace --locked --offline --no-fail-fast` 的原始退出码和
# 汇总行；未设置该测试专用变量时，所有 cargo test 都委派给真实 cargo。
if [[ -n ${EP_CI_FAKE_CARGO_TEST_RC:-} && $# -eq 6 && ${1:-} == "test" && ${2:-} == "-q" \
    && ${3:-} == "--workspace" && ${4:-} == "--locked" && ${5:-} == "--offline" \
    && ${6:-} == "--no-fail-fast" ]]; then
    failed=${EP_CI_FAKE_CARGO_TEST_FAILED:-0}
    printf 'test result: synthetic. 0 passed; %s failed; 0 ignored; 0 measured; 0 filtered out\n' "$failed"
    if [[ ${EP_CI_FAKE_CARGO_TEST_COMPILE_FAILURE:-0} == 1 ]]; then
        printf 'error: could not compile `synthetic-broken-crate` (lib) due to 1 previous error\n' >&2
    fi
    exit "$EP_CI_FAKE_CARGO_TEST_RC"
fi

exec "$EP_CI_REAL_CARGO" "$@"

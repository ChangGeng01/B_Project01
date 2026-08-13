#!/usr/bin/env bash
# 部署与升级各执行一次的资源限额核对。判定分两半：
#   自洽性半边　deploy/ 下八个 drop-in 的两个权重列与规格第 13.1 章配额表该行百分数乘以 100 逐行整数相等；
#   运行期半边　drop-in 的四类取值与 cgroup v2 目录下的实际取值逐行相等。
# 两半的失败不合并成一个退出码：读不到被测对象是「未覆盖」，读到了但不相等才是「不符」，
# 无 cgroup 的机器上一律不得因为读不到而判通过。
set -euo pipefail

EXIT_OK=0
EXIT_MISMATCH=2   # 取值不符
EXIT_UNCOVERED=3  # 被测对象读不到，判定未做出
EXIT_USAGE=64

SELF_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$SELF_DIR/.." && pwd)

# 两个替代根只为负样例存在：不设时取仓库与本机的真实路径。
DROPIN_ROOT=${EP_DROPIN_ROOT:-$REPO_ROOT/deploy/systemd/system}
CGROUP_ROOT=${EP_CGROUP_ROOT:-/sys/fs/cgroup}

DROPIN_NAME=10-resource-limits.conf

# 规格第 13.1 章配额表除第 9 行「内置搜索索引」外的八行：
# slice 名、行号、CPU 份额百分数、磁盘 IO 份额百分数、行名。
# 第 9 行在首版无独立进程与独立 slice，不落 drop-in、不加和、不拆分，因此不在本表。
spec_rows() {
	cat <<'EOF'
app-db 1 44 50 PostgreSQL-16
app-core 2 20 11 Rust核心与集成网关
app-worker 3 10 8 Worker
app-plugin 5 5 4 插件运行时
app-edge 6 3 2 反向代理与运维代理
app-portal 7 3 1 公网门户
app-archive 8 2 6 连续归档与附件增量与审计证据写出
app-backup 9 3 10 全量备份与附件存量搬运与备份校验
EOF
}

# 八行权重之和，缺第 9 行所致，是已披露的既定偏差；写死在此以便有人把缺口悄悄补平时能被检出。
EXPECT_CPU_SUM=9000
EXPECT_IO_SUM=9200

# 只有 app-backup 一行有硬上限；其余各行的突发上限在首版无承载，出现即视为不符。
IOMAX_SLICE=app-backup

mismatch_count=0
uncovered_count=0

mismatch() {
	printf '不符    %s\n' "$1" >&2
	mismatch_count=$((mismatch_count + 1))
}

uncovered() {
	printf '未覆盖  %s\n' "$1" >&2
	uncovered_count=$((uncovered_count + 1))
}

pass() {
	printf '通过    %s\n' "$1"
}

dropin_path() {
	printf '%s/%s.slice.d/%s' "$DROPIN_ROOT" "$1" "$DROPIN_NAME"
}

# 取一个键的唯一取值。键缺失或重复都不给取值，由调用方按不符处理。
read_key() {
	local file=$1 key=$2 lines
	lines=$(grep -c -E "^[[:space:]]*${key}=" "$file" || true)
	if [ "$lines" != "1" ]; then
		return 1
	fi
	grep -E "^[[:space:]]*${key}=" "$file" |
		sed -E "s/^[[:space:]]*${key}=[[:space:]]*//; s/[[:space:]]+$//"
}

# systemd 的 K/M/G/T 后缀按 1024 进制；无后缀即字节。非法取值不返回数字。
parse_size() {
	local raw=$1 num unit
	num=${raw%[KMGTkmgt]}
	unit=${raw#"$num"}
	case $num in
	'' | *[!0-9]*) return 1 ;;
	esac
	case $unit in
	'') printf '%s' "$num" ;;
	K | k) printf '%s' "$((num * 1024))" ;;
	M | m) printf '%s' "$((num * 1024 * 1024))" ;;
	G | g) printf '%s' "$((num * 1024 * 1024 * 1024))" ;;
	T | t) printf '%s' "$((num * 1024 * 1024 * 1024 * 1024))" ;;
	*) return 1 ;;
	esac
}

# 除四类取值之外的键一律不许出现：突发上限一列在首版无承载，
# 有人补上一个 CPUQuota 或 MemoryHigh 时必须在这里红，而不是被当成额外的保险。
allowed_key() {
	local slice=$1 key=$2
	case $key in
	CPUWeight | IOWeight | MemoryLow | MemoryMax) return 0 ;;
	IOReadBandwidthMax | IOWriteBandwidthMax)
		[ "$slice" = "$IOMAX_SLICE" ] && return 0
		return 1
		;;
	*) return 1 ;;
	esac
}

check_one_dropin() {
	local slice=$1 row=$2 cpu_pct=$3 io_pct=$4 name=$5
	local file cpu io mlow mmax key
	file=$(dropin_path "$slice")

	if [ ! -r "$file" ]; then
		mismatch "${slice}　缺少 drop-in 文件 ${file}（配额表第 $row 行 $name 无承载物）"
		return
	fi

	if ! grep -q -E '^[[:space:]]*\[Slice\][[:space:]]*$' "$file"; then
		mismatch "${slice}　drop-in 缺少 [Slice] 节，systemd 会忽略其中全部取值"
		return
	fi

	# 先查有无越界的键，再查取值。
	while IFS= read -r key; do
		if ! allowed_key "$slice" "$key"; then
			mismatch "${slice}　出现四类取值之外的键 ${key}，首版不为该 slice 承载该列"
		fi
	done < <(grep -E '^[[:space:]]*[A-Za-z][A-Za-z0-9]*=' "$file" | sed -E 's/^[[:space:]]*([A-Za-z][A-Za-z0-9]*)=.*/\1/')

	if ! cpu=$(read_key "$file" CPUWeight); then
		mismatch "${slice}　CPUWeight 缺失或重复"
		cpu=
	fi
	if ! io=$(read_key "$file" IOWeight); then
		mismatch "${slice}　IOWeight 缺失或重复"
		io=
	fi
	if ! mlow=$(read_key "$file" MemoryLow); then
		mismatch "${slice}　MemoryLow 缺失或重复"
		mlow=
	fi
	if ! mmax=$(read_key "$file" MemoryMax); then
		mismatch "${slice}　MemoryMax 缺失或重复"
		mmax=
	fi

	local expect_cpu=$((cpu_pct * 100)) expect_io=$((io_pct * 100))

	if [ -n "$cpu" ]; then
		if [ "$cpu" != "$expect_cpu" ]; then
			mismatch "${slice}　CPUWeight=${cpu}，规格第 13.1 章第 $row 行为 ${cpu_pct}%，应为 $expect_cpu"
		elif [ "$cpu" -lt 1 ] || [ "$cpu" -gt 10000 ]; then
			mismatch "${slice}　CPUWeight=$cpu 落在 systemd 取值域 1..10000 之外"
		else
			pass "${slice}　CPUWeight=${cpu}　＝ ${cpu_pct}% × 100"
		fi
	fi

	if [ -n "$io" ]; then
		if [ "$io" != "$expect_io" ]; then
			mismatch "${slice}　IOWeight=${io}，规格第 13.1 章第 $row 行为 ${io_pct}%，应为 $expect_io"
		elif [ "$io" -lt 1 ] || [ "$io" -gt 10000 ]; then
			mismatch "${slice}　IOWeight=$io 落在 systemd 取值域 1..10000 之外"
		else
			pass "${slice}　IOWeight=${io}　＝ ${io_pct}% × 100"
		fi
	fi

	# 内存两列的绝对字节与配额表百分数之间无定义换算，按裁定己-1 只与 drop-in 自身比对：
	# 保底与上限同值，且是一个正整数字节。
	if [ -n "$mlow" ] && [ -n "$mmax" ]; then
		local b_low b_max
		if ! b_low=$(parse_size "$mlow"); then
			mismatch "${slice}　MemoryLow=$mlow 不是合法的字节取值"
		elif ! b_max=$(parse_size "$mmax"); then
			mismatch "${slice}　MemoryMax=$mmax 不是合法的字节取值"
		elif [ "$b_low" != "$b_max" ]; then
			mismatch "${slice}　MemoryLow=$mlow 与 MemoryMax=$mmax 不同值，规格第 13.1 章要求保底与上限同值"
		elif [ "$b_low" -le 0 ]; then
			mismatch "${slice}　MemoryMax=$mmax 不是正数"
		else
			pass "${slice}　MemoryLow＝MemoryMax＝$b_max 字节"
		fi
	fi

	if [ "$slice" = "$IOMAX_SLICE" ]; then
		check_iomax_dropin "$slice" "$file"
	fi
}

# 硬上限的自洽性：两个方向同设备、同取值，即计划所称的「一个 MB/s 数字」。
check_iomax_dropin() {
	local slice=$1 file=$2 r w rdev wdev rval wval
	if ! r=$(read_key "$file" IOReadBandwidthMax); then
		mismatch "${slice}　IOReadBandwidthMax 缺失或重复"
		return
	fi
	if ! w=$(read_key "$file" IOWriteBandwidthMax); then
		mismatch "${slice}　IOWriteBandwidthMax 缺失或重复"
		return
	fi
	rdev=${r%% *}
	wdev=${w%% *}
	rval=${r##* }
	wval=${w##* }
	if [ "$rdev" = "$r" ] || [ "$wdev" = "$w" ]; then
		mismatch "${slice}　IO 硬上限缺少设备路径，systemd 的 io.max 承载指令必须写成「路径 字节每秒」"
		return
	fi
	if [ "$rdev" != "$wdev" ]; then
		mismatch "${slice}　IO 硬上限读写两行设备不同：$rdev 与 $wdev"
		return
	fi
	local b_r b_w
	if ! b_r=$(parse_size "$rval") || ! b_w=$(parse_size "$wval"); then
		mismatch "${slice}　IO 硬上限取值不是合法字节数：$rval 与 $wval"
		return
	fi
	if [ "$b_r" != "$b_w" ]; then
		mismatch "${slice}　IO 硬上限读写两行取值不同：$b_r 与 ${b_w}，计划第 5.6 节只给一个数字"
		return
	fi
	if [ "$b_r" -le 0 ]; then
		mismatch "${slice}　IO 硬上限不是正数：$b_r"
		return
	fi
	pass "${slice}　IO 硬上限 $rdev $b_r 字节每秒（读写同值）"
}

# 多出来的 drop-in 目录同样要红：第 9 行内置搜索索引一旦被人补成第九个 slice，
# 或有人另建一个不在规格表内的 slice，都会在这里检出。
check_no_extra_dropins() {
	local dir base slice known
	for dir in "$DROPIN_ROOT"/*.slice.d; do
		[ -d "$dir" ] || continue
		base=$(basename "$dir")
		slice=${base%.slice.d}
		known=no
		while read -r name _row _cpu _io _label; do
			[ "$name" = "$slice" ] && known=yes
		done <<EOF
$(spec_rows)
EOF
		if [ "$known" = "no" ]; then
			mismatch "${slice}　drop-in 目录不在规格第 13.1 章的八行之内"
		fi
	done
}

check_weight_sums() {
	local cpu_sum=0 io_sum=0 slice file cpu io
	while read -r slice _row _cpu_pct _io_pct _label; do
		file=$(dropin_path "$slice")
		[ -r "$file" ] || return 0 # 缺文件已在逐行检查中报过不符，这里不重复
		cpu=$(read_key "$file" CPUWeight) || return 0
		io=$(read_key "$file" IOWeight) || return 0
		cpu_sum=$((cpu_sum + cpu))
		io_sum=$((io_sum + io))
	done <<EOF
$(spec_rows)
EOF
	if [ "$cpu_sum" != "$EXPECT_CPU_SUM" ]; then
		mismatch "八行 CPUWeight 之和为 ${cpu_sum}，应为 ${EXPECT_CPU_SUM}（第 9 行不落值所致的既定缺口）"
	else
		pass "八行 CPUWeight 之和 ${cpu_sum}，低于满值 10000 是第 9 行未落 drop-in 的既定偏差"
	fi
	if [ "$io_sum" != "$EXPECT_IO_SUM" ]; then
		mismatch "八行 IOWeight 之和为 ${io_sum}，应为 ${EXPECT_IO_SUM}（第 9 行不落值所致的既定缺口）"
	else
		pass "八行 IOWeight 之和 ${io_sum}，低于满值 10000 是第 9 行未落 drop-in 的既定偏差"
	fi
}

check_dropins() {
	printf '== drop-in 自洽性（两个权重列比规格第 13.1 章，三个绝对值只比 drop-in 自身）==\n'
	while read -r slice row cpu_pct io_pct name; do
		check_one_dropin "$slice" "$row" "$cpu_pct" "$io_pct" "$name"
	done <<EOF
$(spec_rows)
EOF
	check_no_extra_dropins
	check_weight_sums
}

# ---- 运行期半边 ----

cgroup_dir() {
	# app-core.slice 是 app.slice 的子片，cgroup 目录按该层级展开。
	printf '%s/app.slice/%s.slice' "$CGROUP_ROOT" "$1"
}

# 读一个 cgroup 接口文件。读不到就没有取值，调用方按未覆盖处理，不得按相等处理。
read_cgroup_file() {
	local path=$1
	[ -r "$path" ] || return 1
	cat "$path"
}

compare_runtime_value() {
	local slice=$1 attr=$2 actual=$3 expect=$4
	if [ "$actual" = "$expect" ]; then
		pass "${slice}　$attr=$actual 与 drop-in 一致"
	else
		mismatch "${slice}　$attr 实际为 ${actual}，drop-in 为 $expect"
	fi
}

check_one_runtime() {
	local slice=$1 dir file raw expect actual
	dir=$(cgroup_dir "$slice")
	file=$(dropin_path "$slice")

	if [ ! -r "$file" ]; then
		uncovered "${slice}　drop-in 读不到，运行期无从比对"
		return
	fi
	if [ ! -d "$dir" ]; then
		uncovered "${slice}　cgroup 目录 $dir 不存在，该 slice 未在本机运行"
		return
	fi

	# drop-in 侧读不出取值时同样是未覆盖：这一半没有比过，不能因为没比而略过。
	if ! expect=$(read_key "$file" CPUWeight); then
		uncovered "${slice}　drop-in 的 CPUWeight 读不出，cpu.weight 无从比对"
	elif actual=$(read_cgroup_file "$dir/cpu.weight"); then
		compare_runtime_value "$slice" cpu.weight "$actual" "$expect"
	else
		uncovered "${slice}　cpu.weight 读不到"
	fi

	if ! expect=$(read_key "$file" IOWeight); then
		uncovered "${slice}　drop-in 的 IOWeight 读不出，io.weight 无从比对"
	else
		if raw=$(read_cgroup_file "$dir/io.weight"); then
			# io.weight 形如「default 100」，另可跟若干按设备的行。
			actual=$(printf '%s\n' "$raw" | awk '$1 == "default" { print $2; found = 1 } END { if (!found) exit 1 }') ||
				actual=
			if [ -n "$actual" ]; then
				compare_runtime_value "$slice" io.weight "$actual" "$expect"
			else
				uncovered "${slice}　io.weight 中没有 default 行，取值无从读出"
			fi
		else
			uncovered "${slice}　io.weight 读不到，io 控制器可能未在该层启用"
		fi
	fi

	check_runtime_memory "$slice" "$file" "$dir" MemoryLow memory.low
	check_runtime_memory "$slice" "$file" "$dir" MemoryMax memory.max

	if [ "$slice" = "$IOMAX_SLICE" ]; then
		check_iomax_runtime "$slice" "$file" "$dir"
	fi
}

# 内存两列的运行期比对。drop-in 侧读不出或解析不出取值时报未覆盖，不静默跳过。
check_runtime_memory() {
	local slice=$1 file=$2 dir=$3 key=$4 attr=$5 raw expect actual
	if ! raw=$(read_key "$file" "$key"); then
		uncovered "${slice}　drop-in 的 $key 读不出，$attr 无从比对"
		return
	fi
	if ! expect=$(parse_size "$raw"); then
		uncovered "${slice}　drop-in 的 $key=$raw 解析不出字节数，$attr 无从比对"
		return
	fi
	if actual=$(read_cgroup_file "$dir/$attr"); then
		compare_runtime_value "$slice" "$attr" "$actual" "$expect"
	else
		uncovered "${slice}　$attr 读不到"
	fi
}

# 把 drop-in 里的设备路径解析成 io.max 用的主次设备号。任一步失败都是未覆盖，不是通过。
resolve_devno() {
	local path=$1 node hex_major hex_minor
	# 无 Linux 块设备的机器上解不出设备号，io.max 那两条比对就无法被负样例证明会红；
	# 该替代取值只顶替设备号一项，比对本身照常，与另外两个替代根同为负样例用途。
	if [ -n "${EP_IOMAX_DEVNO:-}" ]; then
		printf '%s' "$EP_IOMAX_DEVNO"
		return 0
	fi
	command -v df >/dev/null 2>&1 || return 1
	command -v stat >/dev/null 2>&1 || return 1
	[ -e "$path" ] || return 1
	node=$(df -P "$path" 2>/dev/null | awk 'NR == 2 { print $1 }') || return 1
	[ -b "$node" ] || return 1
	hex_major=$(stat -c '%t' "$node" 2>/dev/null) || return 1
	hex_minor=$(stat -c '%T' "$node" 2>/dev/null) || return 1
	[ -n "$hex_major" ] && [ -n "$hex_minor" ] || return 1
	printf '%d:%d' "$((16#$hex_major))" "$((16#$hex_minor))"
}

check_iomax_runtime() {
	local slice=$1 file=$2 dir=$3 r devpath expect devno line rbps wbps
	if ! r=$(read_key "$file" IOReadBandwidthMax); then
		uncovered "${slice}　drop-in 的 IO 硬上限读不出，运行期无从比对"
		return
	fi
	devpath=${r%% *}
	if ! expect=$(parse_size "${r##* }"); then
		uncovered "${slice}　drop-in 的 IO 硬上限取值读不出，运行期无从比对"
		return
	fi
	if ! devno=$(resolve_devno "$devpath"); then
		uncovered "${slice}　$devpath 的主次设备号解析不出，io.max 无从比对"
		return
	fi
	local raw
	if ! raw=$(read_cgroup_file "$dir/io.max"); then
		uncovered "${slice}　io.max 读不到"
		return
	fi
	line=$(printf '%s\n' "$raw" | awk -v d="$devno" '$1 == d { print; found = 1 } END { if (!found) exit 1 }') || line=
	if [ -z "$line" ]; then
		mismatch "${slice}　io.max 中没有设备 $devno 的行，硬上限未生效"
		return
	fi
	rbps=$(printf '%s\n' "$line" | tr ' ' '\n' | awk -F= '$1 == "rbps" { print $2 }')
	wbps=$(printf '%s\n' "$line" | tr ' ' '\n' | awk -F= '$1 == "wbps" { print $2 }')
	compare_runtime_value "$slice" "io.max rbps" "${rbps:-缺}" "$expect"
	compare_runtime_value "$slice" "io.max wbps" "${wbps:-缺}" "$expect"
}

check_runtime() {
	printf '== 运行期 cgroup v2 与 drop-in 逐行比对 ==\n'
	if [ -n "${EP_CGROUP_ROOT:-}" ]; then
		printf '注意    运行期比对使用替代根 %s，非本机 cgroup，仅供负样例\n' "$CGROUP_ROOT"
	fi
	if [ -n "${EP_IOMAX_DEVNO:-}" ]; then
		printf '注意    io.max 的设备号取替代值 %s，非实际解析结果，仅供负样例\n' "$EP_IOMAX_DEVNO"
	fi
	if [ ! -r "$CGROUP_ROOT/cgroup.controllers" ]; then
		uncovered "$CGROUP_ROOT 下没有 cgroup v2（本机为 $(uname -s)），运行期取值一律未读到，本半边未做出判定"
		return
	fi
	while read -r slice _row _cpu_pct _io_pct _label; do
		check_one_runtime "$slice"
	done <<EOF
$(spec_rows)
EOF
}

usage() {
	cat <<'EOF'
用法：verify-resource-limits.sh [--self-check | --runtime | --all]

  --self-check  只做 drop-in 自洽性，不读 cgroup。无 cgroup 的机器上可判。
  --runtime     只做运行期比对。
  --all         两半都做（默认），部署与升级时各执行一次即取此模式。

退出码：0 通过；2 取值不符；3 被测对象读不到、判定未做出；64 用法错误。
读不到 cgroup 时退出码为 3 而不是 0：未覆盖不是通过。
EOF
}

main() {
	local mode=${1:---all}
	case $mode in
	--self-check) check_dropins ;;
	--runtime) check_runtime ;;
	--all)
		check_dropins
		check_runtime
		;;
	-h | --help)
		usage
		exit $EXIT_OK
		;;
	*)
		usage >&2
		exit $EXIT_USAGE
		;;
	esac

	printf '== 结论 ==\n'
	printf '不符 %d 项，未覆盖 %d 项\n' "$mismatch_count" "$uncovered_count"
	if [ "$mismatch_count" -gt 0 ]; then
		printf '判定：不通过（取值不符）\n'
		exit $EXIT_MISMATCH
	fi
	if [ "$uncovered_count" -gt 0 ]; then
		printf '判定：未覆盖，本次核对不成立，不得据此判通过\n'
		exit $EXIT_UNCOVERED
	fi
	printf '判定：通过\n'
	exit $EXIT_OK
}

main "$@"

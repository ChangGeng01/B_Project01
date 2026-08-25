#!/usr/bin/env bash
# F-57：HISTORICAL_LINUX_RELEASE_CHECK；当前发布必须使用 F-57 Windows/HDD/恢复证据门，不能仅凭本脚本放行。
# 客户侧验签。在断网机器上对一份离线升级包做四件事：
#   结构半边　D-11 点名的九个镜像、SBOM、回退说明、校验清单、签名、公钥、签名元数据齐备；
#   清单半边　清单列的每个文件的 SHA-256 与实际相等，且包内没有清单未覆盖的文件；
#   签名半边　MANIFEST.sha256 的 ECDSA-P256 签名以随包公钥验过；
#   来源半边　signing_authority 取 hsm；取 dev 的内部阶段制品要显式 --internal 才放行。
#
# 判定与 xtask sign 同一套：两边的必备项清单必须一致，改一处要同批改另一处。
# 立场不同的只有一点——本脚本在客户机器上跑，只依赖 openssl 与一个 SHA-256 命令，
# 不依赖 Rust 工具链，也一次网络都不出。
#
# 退出码沿用 scripts/verify-resource-limits.sh 已有的一套，不另立第二套：
# 读不到被测对象是「未覆盖」，读到了但不相等才是「不符」，两者绝不合并。
#
# 手写夹具的造法（无真实制品时按此复现一遍本脚本的通过与失败两条路径）：
#   d=$(mktemp -d); mkdir -p "$d/images"
#   for n in archive-writer backup-writer core-server integration-gateway \
#            job-worker ops-agent plugin-host portal-gateway ep-migrate; do
#     printf '镜像 %s 占位\n' "$n" > "$d/images/$n.oci.tar"
#   done
#   printf '{"bomFormat":"CycloneDX","components":[]}\n' > "$d/sbom.cdx.json"
#   printf '# 回退说明\n' > "$d/ROLLBACK.md"
#   printf '{"signing_authority":"dev"}\n' > "$d/signing-metadata.json"
#   (cd "$d" && find images sbom.cdx.json ROLLBACK.md signing-metadata.json -type f |
#      sort | xargs shasum -a 256 > MANIFEST.sha256)
#   openssl ecparam -name prime256v1 -genkey -noout -out "$d/../k.pem"
#   openssl ec -in "$d/../k.pem" -pubout -out "$d/signing-key.pub.pem"
#   openssl dgst -sha256 -sign "$d/../k.pem" \
#     -out "$d/MANIFEST.sha256.sig" "$d/MANIFEST.sha256"
#   scripts/verify-release.sh --dir "$d" --internal   # 0
#   printf x >> "$d/images/core-server.oci.tar"
#   scripts/verify-release.sh --dir "$d" --internal   # 2
set -euo pipefail

EXIT_OK=0
EXIT_MISMATCH=2   # 取值不符，含结构缺项、哈希不等、验签失败
EXIT_UNCOVERED=3  # 被测对象或所需工具读不到，判定未做出
EXIT_USAGE=64

SELF_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$SELF_DIR/.." && pwd)

# 替代根只为夹具与负样例存在：不设时取仓库内的制品落点。
RELEASE_DIR=${EP_RELEASE_DIR:-$REPO_ROOT/target/release-package}
ALLOW_DEV_AUTHORITY=0
# 客户自己保管的可信公钥。不给时退回包内公钥，并明确提示带外核对指纹这一步不可省。
TRUSTED_KEY=""

MANIFEST=MANIFEST.sha256
SIGNATURE=MANIFEST.sha256.sig
PUBLIC_KEY=signing-key.pub.pem
METADATA=signing-metadata.json
ROLLBACK=ROLLBACK.md
SBOM=sbom.cdx.json

# 九个镜像：八个进程加迁移。与 xtask/src/sign.rs 的 IMAGES 逐项一致。
images() {
	cat <<'EOF'
archive-writer
backup-writer
core-server
integration-gateway
job-worker
ops-agent
plugin-host
portal-gateway
ep-migrate
EOF
}

# 签名不覆盖的三个文件：清单不能自证，签名是清单的签名，公钥的信任由带外指纹承载。
# 签名元数据在清单内——它不在清单内时，把 dev 改成 hsm 就能冒充生产制品。
not_in_manifest() {
	printf '%s\n%s\n%s\n' "$MANIFEST" "$SIGNATURE" "$PUBLIC_KEY"
}

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

# SHA-256 命令按可用性择一。两个都没有时不猜、不跳过，判未覆盖。
SHA_CMD=""
pick_sha_cmd() {
	if command -v sha256sum >/dev/null 2>&1; then
		SHA_CMD="sha256sum"
	elif command -v shasum >/dev/null 2>&1; then
		SHA_CMD="shasum -a 256"
	fi
}

sha_of() {
	# shellcheck disable=SC2086
	$SHA_CMD "$1" | awk '{print $1}'
}

# 结构半边。缺任何一项即不符：包在而项缺，不是「读不到」。
check_structure() {
	local missing=0 f
	for f in "$MANIFEST" "$SIGNATURE" "$PUBLIC_KEY" "$METADATA" "$ROLLBACK" "$SBOM"; do
		if [ ! -f "$RELEASE_DIR/$f" ]; then
			mismatch "缺 ${f}，升级包结构不完整"
			missing=$((missing + 1))
		fi
	done
	while read -r n; do
		if [ ! -f "$RELEASE_DIR/images/$n.oci.tar" ]; then
			mismatch "缺 images/$n.oci.tar，升级包结构不完整"
			missing=$((missing + 1))
		fi
	done <<EOF
$(images)
EOF
	if [ "$missing" = "0" ]; then
		pass "结构完整：九个镜像、${SBOM}、${ROLLBACK}、${MANIFEST}、${SIGNATURE}、${PUBLIC_KEY}、$METADATA"
	fi
}

# 清单半边。逐行核对哈希，另查包内是否有清单没覆盖的文件。
check_manifest() {
	local listed_count=0 bad=0 line hash rel actual
	if [ ! -r "$RELEASE_DIR/$MANIFEST" ]; then
		uncovered "$MANIFEST 读不到，清单核对未做出判定"
		return
	fi
	local listed_file
	listed_file=$(mktemp)
	while IFS= read -r line; do
		case $line in
		'' | '#'*) continue ;;
		esac
		hash=${line%% *}
		rel=${line#* }
		rel=${rel# }
		case $hash in
		[0-9a-f]*) ;;
		*)
			mismatch "$MANIFEST 中的哈希不是小写十六进制：$hash"
			bad=$((bad + 1))
			continue
			;;
		esac
		if [ ${#hash} != 64 ]; then
			mismatch "$MANIFEST 中的哈希不是 64 位：$hash"
			bad=$((bad + 1))
			continue
		fi
		printf '%s\n' "$rel" >>"$listed_file"
		listed_count=$((listed_count + 1))
		if [ ! -f "$RELEASE_DIR/$rel" ]; then
			mismatch "$MANIFEST 列了 ${rel}，包内没有这个文件"
			bad=$((bad + 1))
			continue
		fi
		actual=$(sha_of "$RELEASE_DIR/$rel")
		if [ "$actual" != "$hash" ]; then
			mismatch "$rel 的 SHA-256 与清单不符（清单 ${hash}，实际 ${actual}）"
			bad=$((bad + 1))
		fi
	done <"$RELEASE_DIR/$MANIFEST"

	if [ "$listed_count" = "0" ]; then
		mismatch "$MANIFEST 中一条记录都没有。空清单不是通过"
		rm -f "$listed_file"
		return
	fi

	# 夹带进包又不进清单的文件，签名覆盖不到它，只查缺失查不出来。
	local rel2
	while IFS= read -r rel2; do
		if grep -Fxq "$rel2" "$listed_file"; then
			continue
		fi
		if not_in_manifest | grep -Fxq "$rel2"; then
			continue
		fi
		mismatch "$rel2 在包内但不在 $MANIFEST 中，签名覆盖不到它"
		bad=$((bad + 1))
	done <<EOF
$(cd "$RELEASE_DIR" && find . -type f | sed 's|^\./||' | sort)
EOF
	rm -f "$listed_file"
	if [ "$bad" = "0" ]; then
		pass "校验清单 $listed_count 个文件逐项哈希相等，且无夹带文件"
	fi
}

# 签名半边。openssl 缺席是未覆盖，验不过才是不符。
check_signature() {
	local key=$TRUSTED_KEY source_label="客户自备可信公钥"
	if ! command -v openssl >/dev/null 2>&1; then
		uncovered "本机没有 openssl，签名验证未做出判定，不得据此判通过"
		return
	fi
	if [ -z "$key" ]; then
		key=$RELEASE_DIR/$PUBLIC_KEY
		source_label="包内公钥"
		printf '注意    未给 --key，用的是包内公钥。包内公钥无法自证，其指纹必须另行带外核对：\n'
		if [ -r "$key" ]; then
			printf '        %s\n' "$(openssl pkey -pubin -in "$key" -outform DER 2>/dev/null |
				$SHA_CMD 2>/dev/null | awk '{print $1}')"
		fi
	fi
	if [ ! -r "$RELEASE_DIR/$SIGNATURE" ] || [ ! -r "$key" ]; then
		mismatch "签名或公钥读不到，无法验签"
		return
	fi
	if openssl dgst -sha256 -verify "$key" \
		-signature "$RELEASE_DIR/$SIGNATURE" "$RELEASE_DIR/$MANIFEST" >/dev/null 2>&1; then
		pass "$MANIFEST 的 ECDSA 签名以${source_label}验过"
	else
		mismatch "$MANIFEST 的签名以${source_label}验证失败"
	fi
}

# 来源半边。软件密钥签名只允许内部阶段制品使用（计划第 12.1 节 R-06）。
check_authority() {
	local authority
	if [ ! -r "$RELEASE_DIR/$METADATA" ]; then
		mismatch "$METADATA 读不到，签名来源无从判定"
		return
	fi
	authority=$(sed -n 's/.*"signing_authority"[[:space:]]*:[[:space:]]*"\([a-z]*\)".*/\1/p' \
		"$RELEASE_DIR/$METADATA" | head -n 1)
	case $authority in
	hsm)
		pass "signing_authority=hsm"
		;;
	dev)
		if [ "$ALLOW_DEV_AUTHORITY" = "1" ]; then
			pass "signing_authority=dev，已按 --internal 放行；该包只允许内部阶段使用，不得进生产"
		else
			mismatch "signing_authority=dev：软件密钥签名的制品不放行生产，只允许内部阶段使用（--internal）"
		fi
		;;
	'')
		mismatch "$METADATA 中没有 signing_authority 字段，签名来源无从判定"
		;;
	*)
		mismatch "signing_authority=$authority 不在取值域内，只允许 hsm 与 dev"
		;;
	esac
}

usage() {
	cat <<'EOF'
用法：verify-release.sh [--dir <制品目录>] [--key <可信公钥>] [--internal]

  --dir       被验的离线升级包目录。不给时取 <仓库根>/target/release-package，
              也可用环境变量 EP_RELEASE_DIR 指定。
  --key       客户自己保管的可信公钥（PEM）。不给时退回包内公钥，此时脚本打印其
              指纹并提示：包内公钥无法自证，指纹必须另行带外核对。
  --internal  放行 signing_authority=dev 的内部阶段制品。生产收货一律不加此开关。

本脚本一次网络都不出，只用 openssl 与 sha256sum/shasum。

退出码：0 通过；2 取值不符（结构缺项、哈希不等、验签失败、来源不合）；
        3 被测对象或所需工具读不到、判定未做出；64 用法错误。
读不到制品时退出码为 3 而不是 0：未覆盖不是通过。
EOF
}

main() {
	while [ $# -gt 0 ]; do
		case $1 in
		--dir)
			if [ $# -lt 2 ]; then
				usage >&2
				exit $EXIT_USAGE
			fi
			RELEASE_DIR=$2
			shift 2
			;;
		--key)
			if [ $# -lt 2 ]; then
				usage >&2
				exit $EXIT_USAGE
			fi
			TRUSTED_KEY=$2
			shift 2
			;;
		--internal)
			ALLOW_DEV_AUTHORITY=1
			shift
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
	done

	printf '被验制品目录：%s\n' "$RELEASE_DIR"
	if [ ! -d "$RELEASE_DIR" ]; then
		uncovered "$RELEASE_DIR 不存在，升级包验签未做出判定"
	else
		pick_sha_cmd
		if [ -z "$SHA_CMD" ]; then
			uncovered "本机既没有 sha256sum 也没有 shasum，清单核对未做出判定"
		fi
		check_structure
		if [ -n "$SHA_CMD" ]; then
			check_manifest
		fi
		check_signature
		check_authority
	fi

	printf '== 结论 ==\n'
	printf '不符 %d 项，未覆盖 %d 项\n' "$mismatch_count" "$uncovered_count"
	if [ "$mismatch_count" -gt 0 ]; then
		printf '判定：不通过（取值不符）\n'
		exit $EXIT_MISMATCH
	fi
	if [ "$uncovered_count" -gt 0 ]; then
		printf '判定：未覆盖，本次验签不成立，不得据此判通过\n'
		exit $EXIT_UNCOVERED
	fi
	printf '判定：通过\n'
	exit $EXIT_OK
}

main "$@"

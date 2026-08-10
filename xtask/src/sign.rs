//! `xtask sign` —— 升级包结构、校验清单与签名的门禁。
//!
//! 阶段 1 退出条件 15：升级包结构完整，客户侧验签脚本在断网机器上通过，篡改后失败。
//! 交付物 D-11 点名包内含八个进程镜像、迁移镜像、SBOM、签名、校验清单、回退说明。
//!
//! 本模块与 `scripts/verify-release.sh` 判同一套结构，两边的必备项清单必须一致：
//! 前者是发布侧放行门禁，后者是客户侧收货验签，判据相同而立场不同。
//!
//! 三态纪律。制品目录整体缺席时判定未做出（`Outcome::Undecidable`），
//! 因为本阶段还没有任何构建产物；目录一旦存在，其中缺项、哈希不符、签名不过
//! 一律是不符（`Outcome::Violated`）。空目录不得判通过。
//!
//! SHA-256 自带实现而不外调 `shasum`：门禁不该因为一台机器上少一个命令行工具
//! 就退化成不可判定，而且哈希是本模块与 [`crate::reproduce`] 共用的判据基元。

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::archcheck::Outcome;

/// 制品根目录。落在 `target/` 之下：升级包是构建产物，不是仓库顶层源码目录，
/// 技术基线第 1.1 节的顶层目录清单因此一行不动。
pub const RELEASE_DIR: &str = "target/release-package";

pub const MANIFEST: &str = "MANIFEST.sha256";
pub const SIGNATURE: &str = "MANIFEST.sha256.sig";
pub const PUBLIC_KEY: &str = "signing-key.pub.pem";
pub const METADATA: &str = "signing-metadata.json";
pub const ROLLBACK: &str = "ROLLBACK.md";
pub const SBOM_IN_PACKAGE: &str = "sbom.cdx.json";

/// 九个镜像：八个进程加迁移。名字与 `apps/` 下的目录名、systemd 单元名一一对应。
pub const IMAGES: [&str; 9] = [
    "archive-writer",
    "backup-writer",
    "core-server",
    "integration-gateway",
    "job-worker",
    "ops-agent",
    "plugin-host",
    "portal-gateway",
    "ep-migrate",
];

/// 签名不受清单覆盖的三个文件：清单不能自证，签名是清单的签名，
/// 公钥的信任由带外指纹承载而不是由包自身承载。
/// 签名元数据**在**清单内：它不在清单内时，改一个字段就能把 dev 制品冒充成 hsm 制品，
/// 这一处在本轮实测中真的被冒充成功过一次，因此把它收进签名覆盖面。
const NOT_IN_MANIFEST: [&str; 3] = [MANIFEST, SIGNATURE, PUBLIC_KEY];

/// 计划第 12.1 节 R-06：非 HSM 来源的签名只允许内部阶段制品使用，发布流水线拒绝放行。
const AUTHORITY_HSM: &str = "hsm";
const AUTHORITY_DEV: &str = "dev";

#[derive(Debug, Default)]
pub struct Report {
    /// 判不符：被测对象读到了，但不满足判据。
    pub problems: Vec<String>,
    /// 判定未做出：被测对象或判定工具缺席。任何情况下都不折算为通过。
    pub undecidable: Vec<String>,
    /// 已做出的判定，供报告打印。
    pub notes: Vec<String>,
}

impl Report {
    pub fn outcome(&self) -> Outcome {
        if !self.problems.is_empty() {
            Outcome::Violated
        } else if !self.undecidable.is_empty() {
            Outcome::Undecidable
        } else {
            Outcome::Clean
        }
    }
}

pub fn run(root: &Path) -> Report {
    evaluate(&root.join(RELEASE_DIR))
}

/// 判据本体。以一个目录为被测对象，因此负样例可以直接造手写夹具目录喂进来。
pub fn evaluate(dir: &Path) -> Report {
    let mut r = Report::default();
    if !dir.is_dir() {
        r.undecidable.push(format!(
            "制品目录 {} 不存在。本阶段尚未产出任何升级包，结构与验签判定未做出，不得据此判通过",
            dir.display()
        ));
        return r;
    }

    let manifest_text = match fs::read_to_string(dir.join(MANIFEST)) {
        Ok(t) => t,
        Err(e) => {
            r.problems.push(format!("{MANIFEST} 读不到：{e}。目录存在而校验清单缺失即结构不完整"));
            return r;
        }
    };
    let entries = match parse_manifest(&manifest_text) {
        Ok(v) => v,
        Err(e) => {
            r.problems.push(format!("{MANIFEST} 解析失败：{e}"));
            return r;
        }
    };
    if entries.is_empty() {
        r.problems.push(format!("{MANIFEST} 中一条记录都没有。空清单不是通过"));
        return r;
    }

    let listed: Vec<String> = entries.iter().map(|e| e.path.clone()).collect();
    r.problems.extend(structure_problems(&listed));
    r.problems.extend(hash_problems(dir, &entries));
    r.problems.extend(uncovered_files(dir, &listed));

    match fs::read_to_string(dir.join(METADATA)) {
        Ok(t) => r.problems.extend(authority_problems(&t)),
        Err(e) => r.problems.push(format!("{METADATA} 读不到：{e}。签名来源无从判定，按不符处理")),
    }

    let (sig_problems, sig_undecidable) = verify_signature(dir);
    r.problems.extend(sig_problems);
    r.undecidable.extend(sig_undecidable);

    r.notes.push(format!("校验清单覆盖 {} 个文件", entries.len()));
    r
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ManifestEntry {
    pub sha256: String,
    /// 相对制品根的路径。
    pub path: String,
}

/// 清单取 `sha256sum` 的既有格式：64 位小写十六进制、两个空格、相对路径。
/// 不自定义第二种格式，客户侧脚本因此可以直接用 `shasum -c` 之外的任何等价手段核对。
pub fn parse_manifest(text: &str) -> Result<Vec<ManifestEntry>, String> {
    let mut out: Vec<ManifestEntry> = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((hash, path)) = line.split_once("  ") else {
            return Err(format!("第 {} 行不是「哈希两空格路径」形态：{line}", i + 1));
        };
        if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
        {
            return Err(format!("第 {} 行的哈希不是 64 位小写十六进制：{hash}", i + 1));
        }
        let path = path.trim();
        if path.starts_with('/') || path.contains("..") {
            return Err(format!("第 {} 行的路径越出制品根：{path}", i + 1));
        }
        if !seen.insert(path.to_string()) {
            return Err(format!("第 {} 行的路径 {path} 在清单中重复", i + 1));
        }
        out.push(ManifestEntry { sha256: hash.to_string(), path: path.to_string() });
    }
    Ok(out)
}

/// D-11 点名的必备项：九个镜像、SBOM、回退说明，另加签名元数据。
pub fn required_entries() -> Vec<String> {
    let mut v: Vec<String> = IMAGES.iter().map(|n| format!("images/{n}.oci.tar")).collect();
    v.push(SBOM_IN_PACKAGE.to_string());
    v.push(ROLLBACK.to_string());
    v.push(METADATA.to_string());
    v
}

pub fn structure_problems(listed: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let have: BTreeSet<&str> = listed.iter().map(String::as_str).collect();
    for need in required_entries() {
        if !have.contains(need.as_str()) {
            out.push(format!("校验清单中没有 {need}，升级包结构不完整"));
        }
    }
    for skip in NOT_IN_MANIFEST {
        if have.contains(skip) {
            out.push(format!("{skip} 不该出现在校验清单里：清单不能自证，签名与公钥也不由清单覆盖"));
        }
    }
    out
}

fn hash_problems(dir: &Path, entries: &[ManifestEntry]) -> Vec<String> {
    let mut out = Vec::new();
    for e in entries {
        match sha256_file(&dir.join(&e.path)) {
            Err(err) => out.push(format!("清单列了 {}，但读不到：{err}", e.path)),
            Ok(actual) if actual != e.sha256 => out.push(format!(
                "{} 的 SHA-256 与清单不符。\n      清单：{}\n      实际：{actual}",
                e.path, e.sha256
            )),
            Ok(_) => {}
        }
    }
    out
}

/// 清单未覆盖的文件同样是篡改面：只查缺失不查多余，往包里塞一个文件就查不出来。
fn uncovered_files(dir: &Path, listed: &[String]) -> Vec<String> {
    let have: BTreeSet<&str> = listed.iter().map(String::as_str).collect();
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(rd) = fs::read_dir(&d) else {
            out.push(format!("{} 读不到，未覆盖的文件无从枚举", d.display()));
            continue;
        };
        for ent in rd.flatten() {
            let p = ent.path();
            if p.is_dir() {
                stack.push(p);
                continue;
            }
            let Ok(rel) = p.strip_prefix(dir) else { continue };
            let rel = rel.to_string_lossy().replace('\\', "/");
            if NOT_IN_MANIFEST.contains(&rel.as_str()) || have.contains(rel.as_str()) {
                continue;
            }
            out.push(format!("{rel} 在包内但不在校验清单中，签名覆盖不到它"));
        }
    }
    out.sort();
    out
}

/// 元数据只判一件事：签名来源。取值只有 hsm 与 dev 两种，dev 一律不放行。
pub fn authority_problems(meta_text: &str) -> Vec<String> {
    let value: serde_json::Value = match serde_json::from_str(meta_text) {
        Ok(v) => v,
        Err(e) => return vec![format!("{METADATA} 不是合法 JSON：{e}")],
    };
    match value.get("signing_authority").and_then(|v| v.as_str()) {
        None => vec![format!("{METADATA} 中没有 signing_authority 字段，签名来源无从判定")],
        Some(AUTHORITY_HSM) => Vec::new(),
        Some(AUTHORITY_DEV) => vec![format!(
            "signing_authority={AUTHORITY_DEV}：软件密钥签名只允许内部阶段制品使用，\
             发布流水线不放行（计划第 12.1 节 R-06）"
        )],
        Some(other) => vec![format!(
            "signing_authority={other} 不在取值域内，只允许 {AUTHORITY_HSM} 与 {AUTHORITY_DEV}"
        )],
    }
}

/// 验签外调 openssl。缺 openssl 是「判定未做出」，不是「签名不过」，两者退出码不同。
fn verify_signature(dir: &Path) -> (Vec<String>, Vec<String>) {
    for f in [SIGNATURE, PUBLIC_KEY] {
        if !dir.join(f).is_file() {
            return (vec![format!("{f} 不存在，升级包无法验签")], Vec::new());
        }
    }
    let out = Command::new("openssl")
        .arg("dgst")
        .arg("-sha256")
        .arg("-verify")
        .arg(dir.join(PUBLIC_KEY))
        .arg("-signature")
        .arg(dir.join(SIGNATURE))
        .arg(dir.join(MANIFEST))
        .output();
    match out {
        Err(e) => (
            Vec::new(),
            vec![format!("openssl 不可用（{e}），签名验证未做出判定，不得据此判通过")],
        ),
        Ok(o) if o.status.success() => (Vec::new(), Vec::new()),
        Ok(o) => (
            vec![format!(
                "{MANIFEST} 的 ECDSA 签名验证失败：{}",
                String::from_utf8_lossy(&o.stdout).trim()
            )],
            Vec::new(),
        ),
    }
}

// ---------------------------------------------------------------------------
// SHA-256。自实现，规格第 12.3 章把哈希算法定死为 SHA-256。
// ---------------------------------------------------------------------------

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// 增量式：镜像 tar 可以到 GB 量级，不整份读进内存。
struct Sha256 {
    h: [u32; 8],
    buf: [u8; 64],
    buf_len: usize,
    total: u64,
}

impl Sha256 {
    fn new() -> Sha256 {
        Sha256 {
            h: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
                0x5be0cd19,
            ],
            buf: [0; 64],
            buf_len: 0,
            total: 0,
        }
    }

    fn update(&mut self, mut data: &[u8]) {
        self.total = self.total.wrapping_add(data.len() as u64);
        while !data.is_empty() {
            let take = (64 - self.buf_len).min(data.len());
            self.buf[self.buf_len..self.buf_len + take].copy_from_slice(&data[..take]);
            self.buf_len += take;
            data = &data[take..];
            if self.buf_len == 64 {
                let block = self.buf;
                self.compress(&block);
                self.buf_len = 0;
            }
        }
    }

    fn compress(&mut self, block: &[u8; 64]) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                block[4 * i],
                block[4 * i + 1],
                block[4 * i + 2],
                block[4 * i + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = self.h;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (slot, v) in self.h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *slot = slot.wrapping_add(v);
        }
    }

    fn finish(mut self) -> String {
        let bits = self.total.wrapping_mul(8);
        self.update(&[0x80]);
        // 上一行把 total 也加了 1，此处只关心补位长度，长度已在 bits 中定格。
        while self.buf_len != 56 {
            self.update(&[0]);
        }
        let block_tail = bits.to_be_bytes();
        self.buf[56..64].copy_from_slice(&block_tail);
        let block = self.buf;
        self.compress(&block);
        let mut s = String::with_capacity(64);
        for v in self.h {
            s.push_str(&format!("{v:08x}"));
        }
        s
    }
}

pub fn sha256_file(path: &Path) -> Result<String, String> {
    use std::io::Read;
    let mut f = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut s = Sha256::new();
    let mut buf = vec![0u8; 64 * 1024];
    loop {
        let n = f.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        s.update(&buf[..n]);
    }
    Ok(s.finish())
}

#[cfg(test)]
mod hash_negative_samples {
    use super::*;

    fn sha256_hex(data: &[u8]) -> String {
        let mut s = Sha256::new();
        s.update(data);
        s.finish()
    }

    /// 自实现的哈希必须先证明自己是 SHA-256，否则后面所有比对都建在沙上。
    #[test]
    fn negative_sha256_known_vectors() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        // 跨块：56 字节触发补位溢出到第二块，是最易写错的一处。
        assert_eq!(
            sha256_hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
    }

    #[test]
    fn negative_sha256_incremental_equals_oneshot() {
        let data: Vec<u8> = (0u8..=255).cycle().take(100_000).collect();
        let mut s = Sha256::new();
        for chunk in data.chunks(7) {
            s.update(chunk);
        }
        assert_eq!(s.finish(), sha256_hex(&data), "分块喂入与一次喂入必须同值");
    }
}

#[cfg(test)]
mod rule_negative_samples {
    use super::*;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    /// 按目录内容产出一份 `sha256sum` 形态的清单。夹具与真实发布流程用同一套算法，
    /// 否则负样例证明的是另一个东西。
    fn manifest_lines(dir: &Path, paths: &[String]) -> Result<String, String> {
        let mut map: BTreeMap<&str, String> = BTreeMap::new();
        for p in paths {
            map.insert(p.as_str(), sha256_file(&dir.join(p))?);
        }
        Ok(map.into_iter().map(|(p, h)| format!("{h}  {p}\n")).collect())
    }

    fn fixture(tag: &str, files: &[(&str, &str)]) -> PathBuf {
        let root = std::env::temp_dir().join(format!("ep-sign-{tag}"));
        let _ = fs::remove_dir_all(&root);
        for (rel, body) in files {
            let p = root.join(rel);
            fs::create_dir_all(p.parent().expect("有父目录")).expect("建夹具目录");
            fs::write(&p, body).expect("写夹具文件");
        }
        root
    }

    /// 造一个结构完整的包，除签名与公钥外全部就位。
    fn full_package(tag: &str, tamper: Option<&str>) -> PathBuf {
        let mut files: Vec<(String, String)> = IMAGES
            .iter()
            .map(|n| (format!("images/{n}.oci.tar"), format!("镜像 {n} 的占位内容\n")))
            .collect();
        files.push((SBOM_IN_PACKAGE.into(), "{\"bomFormat\":\"CycloneDX\"}\n".into()));
        files.push((ROLLBACK.into(), "# 回退说明\n".into()));
        files.push((
            METADATA.into(),
            format!("{{\"signing_authority\":\"{AUTHORITY_HSM}\"}}\n"),
        ));
        let borrowed: Vec<(&str, &str)> =
            files.iter().map(|(a, b)| (a.as_str(), b.as_str())).collect();
        let dir = fixture(tag, &borrowed);

        let covered: Vec<String> = required_entries();
        let manifest = manifest_lines(&dir, &covered).expect("算清单");
        fs::write(dir.join(MANIFEST), manifest).expect("写清单");
        if let Some(rel) = tamper {
            fs::write(dir.join(rel), "被改过的一个字节\n").expect("改夹具");
        }
        dir
    }

    /// 负样例：制品目录整体缺席时必须是「判定未做出」，绝不是通过。
    #[test]
    fn negative_missing_package_is_undecidable_not_clean() {
        let dir = std::env::temp_dir().join("ep-sign-absent");
        let _ = fs::remove_dir_all(&dir);
        let r = evaluate(&dir);
        assert_eq!(r.outcome(), Outcome::Undecidable);
        assert!(r.problems.is_empty());
        assert!(r.undecidable[0].contains("判定未做出"));
    }

    /// 负样例：包里少一个镜像，结构规则必须点名那一项。
    #[test]
    fn negative_missing_image_is_structural_violation() {
        let mut listed = required_entries();
        listed.retain(|p| p != "images/ep-migrate.oci.tar");
        let p = structure_problems(&listed);
        assert_eq!(p.len(), 1);
        assert!(p[0].contains("images/ep-migrate.oci.tar"));
    }

    /// 负样例：清单把自己也列进去，等于自证。
    #[test]
    fn negative_manifest_covering_itself() {
        let mut listed = required_entries();
        listed.push(MANIFEST.to_string());
        let p = structure_problems(&listed);
        assert_eq!(p.len(), 1);
        assert!(p[0].contains("不能自证"));
    }

    /// 负样例：签名元数据必须在签名覆盖面内，否则把 dev 改成 hsm 就能蒙混过关。
    #[test]
    fn negative_metadata_must_be_signed() {
        assert!(
            required_entries().contains(&METADATA.to_string()),
            "{METADATA} 不在必备项内，改一个字段就能冒充 hsm 制品"
        );
        // 夹具原本标 hsm，改成 dev 即改动了被清单覆盖的字节，反向同理。
        let dir = full_package("metadata-tamper", None);
        fs::write(dir.join(METADATA), format!("{{\"signing_authority\":\"{AUTHORITY_DEV}\"}}\n"))
            .expect("改元数据");
        let r = evaluate(&dir);
        assert!(
            r.problems.iter().any(|p| p.contains(METADATA) && p.contains("SHA-256")),
            "改过的元数据必须被清单查出，实得：{:?}",
            r.problems
        );
    }

    /// 负样例：篡改一个字节，整条规则（不是某个辅助函数）必须判红。
    #[test]
    fn negative_tampered_byte_fails_whole_rule() {
        let dir = full_package("tampered", Some("images/core-server.oci.tar"));
        let r = evaluate(&dir);
        assert_eq!(r.outcome(), Outcome::Violated);
        assert!(
            r.problems.iter().any(|p| p.contains("core-server.oci.tar") && p.contains("SHA-256")),
            "实得：{:?}",
            r.problems
        );
    }

    /// 负样例：往包里塞一个清单没覆盖的文件。
    #[test]
    fn negative_extra_file_outside_manifest() {
        let dir = full_package("extra", None);
        fs::write(dir.join("images/rogue.oci.tar"), "夹带\n").expect("写夹带文件");
        let r = evaluate(&dir);
        assert!(
            r.problems.iter().any(|p| p.contains("images/rogue.oci.tar") && p.contains("签名覆盖不到")),
            "实得：{:?}",
            r.problems
        );
    }

    /// 负样例：软件密钥签名不得放行生产。
    #[test]
    fn negative_dev_authority_rejected() {
        let p = authority_problems(&format!("{{\"signing_authority\":\"{AUTHORITY_DEV}\"}}"));
        assert_eq!(p.len(), 1);
        assert!(p[0].contains("不放行"));
        assert!(authority_problems(&format!("{{\"signing_authority\":\"{AUTHORITY_HSM}\"}}")).is_empty());
        assert_eq!(authority_problems("{}").len(), 1, "字段缺失同样是不符");
    }

    /// 负样例：清单格式。哈希位数、路径越界、重复路径三种都必须报出。
    #[test]
    fn negative_manifest_parse() {
        assert!(parse_manifest("deadbeef  a.txt").is_err(), "哈希位数不足");
        let ok = "0".repeat(64);
        assert!(parse_manifest(&format!("{ok}  ../etc/passwd")).is_err(), "路径越出制品根");
        assert!(
            parse_manifest(&format!("{ok}  a.txt\n{ok}  a.txt")).is_err(),
            "同一路径两条记录"
        );
        assert!(parse_manifest(&format!("{ok}  a.txt")).is_ok());
    }

    /// 负样例：包在但清单空。空清单不是通过。
    #[test]
    fn negative_empty_manifest_is_violation() {
        let dir = fixture("empty-manifest", &[(MANIFEST, "# 只有注释\n")]);
        let r = evaluate(&dir);
        assert_eq!(r.outcome(), Outcome::Violated);
        assert!(r.problems[0].contains("空清单不是通过"));
    }
}

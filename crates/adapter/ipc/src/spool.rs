//! 写出进程的 spool。一帧一行追加到 `pending.jsonl`。
//!
//! 三条纪律：core 不可用时落盘、恢复后按顺序补写并在成功后截断、
//! 超上限丢最旧并记 ERROR。第三条的「记 ERROR」由调用方完成——本 crate
//! 不依赖观测层，因此把被丢弃的条数如实返回，让调用方无从忽略。

use std::io::Write;
use std::path::{Path, PathBuf};

pub const PENDING_FILE: &str = "pending.jsonl";

#[derive(Debug)]
pub enum SpoolError {
    Io {
        path: PathBuf,
        detail: String,
    },
    /// 单条记录本身就超过容量上限，落盘无从谈起。
    RecordTooLarge {
        bytes: usize,
        max_bytes: u64,
    },
}

impl std::fmt::Display for SpoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpoolError::Io { path, detail } => {
                write!(f, "spool {} 操作失败：{detail}", path.display())
            }
            SpoolError::RecordTooLarge { bytes, max_bytes } => {
                write!(f, "单条 {bytes} 字节超过 spool 上限 {max_bytes} 字节")
            }
        }
    }
}

impl std::error::Error for SpoolError {}

/// 一次追加的结果。`evicted` 非零时调用方必须记 ERROR。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AppendOutcome {
    pub evicted: usize,
    pub bytes_after: u64,
}

pub struct Spool {
    dir: PathBuf,
    max_bytes: u64,
}

impl Spool {
    pub fn new(dir: impl Into<PathBuf>, max_bytes: u64) -> Self {
        Self {
            dir: dir.into(),
            max_bytes,
        }
    }

    pub fn file(&self) -> PathBuf {
        self.dir.join(PENDING_FILE)
    }

    pub fn ensure_dir(&self) -> Result<(), SpoolError> {
        std::fs::create_dir_all(&self.dir).map_err(|e| SpoolError::Io {
            path: self.dir.clone(),
            detail: e.to_string(),
        })
    }

    /// 追加一条。超上限时从最旧一条开始丢，直到容得下新的一条。
    /// 绝不阻塞写出：写出进程被 spool 卡住就等于丢的是当下这条。
    pub fn append(&self, record: &str) -> Result<AppendOutcome, SpoolError> {
        self.ensure_dir()?;
        let line = format!("{}\n", record.replace('\n', " "));
        if line.len() as u64 > self.max_bytes {
            return Err(SpoolError::RecordTooLarge {
                bytes: line.len(),
                max_bytes: self.max_bytes,
            });
        }
        let mut lines = self.read_lines()?;
        lines.push(line.trim_end().to_string());
        let mut evicted = 0;
        while total_bytes(&lines) > self.max_bytes {
            lines.remove(0);
            evicted += 1;
        }
        self.rewrite(&lines)?;
        Ok(AppendOutcome {
            evicted,
            bytes_after: total_bytes(&lines),
        })
    }

    /// 按写入顺序读出全部待补写记录。
    pub fn read_lines(&self) -> Result<Vec<String>, SpoolError> {
        let path = self.file();
        match std::fs::read_to_string(&path) {
            Ok(text) => Ok(text
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(str::to_string)
                .collect()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(e) => Err(SpoolError::Io {
                path,
                detail: e.to_string(),
            }),
        }
    }

    /// 补写成功后截断。只在全部补写成功后调用，否则会丢未确认的记录。
    pub fn truncate(&self) -> Result<(), SpoolError> {
        let path = self.file();
        match std::fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(SpoolError::Io {
                path,
                detail: e.to_string(),
            }),
        }
    }

    /// 丢弃已成功补写的前 n 条，保留其余。部分成功时用它，不用 truncate。
    pub fn drop_first(&self, n: usize) -> Result<(), SpoolError> {
        let lines = self.read_lines()?;
        let rest: Vec<String> = lines.into_iter().skip(n).collect();
        if rest.is_empty() {
            return self.truncate();
        }
        self.rewrite(&rest)
    }

    /// 重写整份 spool。**先写临时文件再原子替换**，不原地截断重写。
    ///
    /// 改这一处的理由是裁定 F-08 第 4.3 节第 3 条：原实现直接 `File::create` 覆盖，
    /// 中途失败会留下一份被截断的 spool——而这条路径上的错误会经 `SpoolError::Io`
    /// 归成 `ForwardOutcome::Lost`，那是本 crate 自己注释里写的「连盘都落不下，
    /// 这是最坏的一档」。原地重写在任何平台上都有这个窗口，只是在装了杀毒与备份代理的
    /// 机器上更容易撞上——那些代理会拿着瞬时句柄，使覆盖与删除偶发失败。
    ///
    /// 原子替换把「中途失败」的后果从「spool 被截断」降为「临时文件残留」，
    /// 后者无害且下次重写即被覆盖。
    fn rewrite(&self, lines: &[String]) -> Result<(), SpoolError> {
        let path = self.file();
        let tmp = path.with_extension("jsonl.tmp");
        {
            let mut file = std::fs::File::create(&tmp).map_err(|e| SpoolError::Io {
                path: tmp.clone(),
                detail: e.to_string(),
            })?;
            for l in lines {
                writeln!(file, "{l}").map_err(|e| SpoolError::Io {
                    path: tmp.clone(),
                    detail: e.to_string(),
                })?;
            }
            file.flush().map_err(|e| SpoolError::Io {
                path: tmp.clone(),
                detail: e.to_string(),
            })?;
            // 落盘再替换。少了这一步，掉电后可能得到一个内容为空但已就位的新文件。
            file.sync_all().map_err(|e| SpoolError::Io {
                path: tmp.clone(),
                detail: e.to_string(),
            })?;
        }
        rename_with_retry(&tmp, &path)
    }
}

/// 替换目标文件，失败时有限重试。
///
/// 重试而不是一次定生死，理由同 `rewrite`：外部代理持瞬时句柄会让替换偶发失败，
/// 而这条路径的失败后果是最坏的一档。重试上限取小值——它挡的是瞬时占用，
/// 不是持续占用；持续占用重试多少次都没用，如实失败比吊在这里好。
fn rename_with_retry(from: &Path, to: &Path) -> Result<(), SpoolError> {
    const MAX_ATTEMPTS: u32 = 5;
    let mut last = String::new();
    for attempt in 0..MAX_ATTEMPTS {
        match std::fs::rename(from, to) {
            Ok(()) => return Ok(()),
            Err(e) => {
                last = e.to_string();
                if attempt + 1 < MAX_ATTEMPTS {
                    std::thread::sleep(std::time::Duration::from_millis(20));
                }
            }
        }
    }
    // 替换没成，临时文件不留：留着会在下次 read_lines 时被误当成正文之外的垃圾。
    let _ = std::fs::remove_file(from);
    Err(SpoolError::Io {
        path: to.to_path_buf(),
        detail: format!("原子替换失败，已重试 {MAX_ATTEMPTS} 次：{last}"),
    })
}

fn total_bytes(lines: &[String]) -> u64 {
    lines.iter().map(|l| l.len() as u64 + 1).sum()
}

/// 供测试与调用方判定 spool 目录是否可用。
///
/// **实建一个探针文件再删，不以「能否建子目录」代替「能否建文件」。**
/// 原实现是 `create_dir_all(dir).is_ok()`，那是一条假阳性判据：
/// 建目录与建文件是两个不同的权限位，目录建得出来不等于文件写得进去——
/// 在 NTFS ACL 下尤其容易分开（可以只授 `FILE_ADD_SUBDIRECTORY` 而不授 `FILE_ADD_FILE`），
/// 但这条判据在任何平台上都是错的，不是换平台才暴露的问题。
/// 出处：裁定 F-08 第 4.3 节第 2 条。
///
/// 探针文件名带进程号，避免两个进程同时探测时互相删掉对方的探针。
pub fn is_writable(dir: &Path) -> bool {
    if std::fs::create_dir_all(dir).is_err() {
        return false;
    }
    let probe = dir.join(format!(".ep-writable-probe-{}", std::process::id()));
    match std::fs::write(&probe, b"") {
        Ok(()) => {
            // 删不掉也算可写：能写进去就说明写权限在，删除失败是另一回事，
            // 不该让它把一个可用的目录判成不可用。残留探针无害且下次会被覆盖。
            let _ = std::fs::remove_file(&probe);
            true
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("ep-spool-{}-{name}", std::process::id()));
        std::fs::remove_dir_all(&d).ok();
        d
    }

    #[test]
    fn appended_records_are_read_back_in_order() {
        let dir = temp("order");
        let spool = Spool::new(&dir, 1024);
        spool.append("{\"n\":1}").unwrap();
        spool.append("{\"n\":2}").unwrap();
        assert_eq!(spool.read_lines().unwrap(), ["{\"n\":1}", "{\"n\":2}"]);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn truncate_after_successful_replay() {
        let dir = temp("truncate");
        let spool = Spool::new(&dir, 1024);
        spool.append("a").unwrap();
        spool.truncate().unwrap();
        assert!(spool.read_lines().unwrap().is_empty());
        std::fs::remove_dir_all(&dir).ok();
    }

    // 负样例断言的是「超上限丢最旧」这条规则本身，且丢弃必须被调用方看见。
    #[test]
    fn over_capacity_drops_the_oldest_and_reports_it() {
        let dir = temp("evict");
        // 每条 "aaaa" 加换行是 5 字节，上限 12 字节最多容 2 条。
        let spool = Spool::new(&dir, 12);
        assert_eq!(spool.append("aaaa").unwrap().evicted, 0);
        assert_eq!(spool.append("bbbb").unwrap().evicted, 0);
        let out = spool.append("cccc").unwrap();
        assert_eq!(out.evicted, 1, "超上限必须丢最旧一条并如实返回条数");
        assert_eq!(spool.read_lines().unwrap(), ["bbbb", "cccc"]);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_record_larger_than_the_cap_is_an_error_not_a_silent_drop() {
        let dir = temp("toolarge");
        let spool = Spool::new(&dir, 4);
        let err = spool.append("aaaaaaaa").unwrap_err();
        assert!(matches!(err, SpoolError::RecordTooLarge { .. }));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn partial_replay_keeps_the_unconfirmed_records() {
        let dir = temp("partial");
        let spool = Spool::new(&dir, 1024);
        for n in 0..3 {
            spool.append(&format!("r{n}")).unwrap();
        }
        spool.drop_first(2).unwrap();
        assert_eq!(spool.read_lines().unwrap(), ["r2"]);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn embedded_newlines_cannot_split_one_record_into_two() {
        let dir = temp("newline");
        let spool = Spool::new(&dir, 1024);
        spool.append("{\"a\":\"x\ny\"}").unwrap();
        assert_eq!(
            spool.read_lines().unwrap().len(),
            1,
            "一帧一行，内嵌换行不得撕开一条记录"
        );
        std::fs::remove_dir_all(&dir).ok();
    }
}

#[cfg(test)]
mod f08_defect_tests {
    use super::*;

    fn dir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("ep-spool-f08-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("建夹具目录");
        d
    }

    /// 负样例：只读目录必须判为不可写。
    ///
    /// 这是原实现的假阳性所在——`create_dir_all` 对一个已存在的目录直接返回 Ok，
    /// 于是一个一个字节都写不进去的目录会被判成可写。
    #[cfg(unix)]
    #[test]
    fn readonly_dir_is_not_writable() {
        use std::os::unix::fs::PermissionsExt;
        let d = dir("readonly");
        std::fs::set_permissions(&d, std::fs::Permissions::from_mode(0o500)).expect("设只读");
        let verdict = is_writable(&d);
        // 先恢复权限再断言，否则失败时夹具删不掉。
        let _ = std::fs::set_permissions(&d, std::fs::Permissions::from_mode(0o700));
        assert!(!verdict, "只读目录必须判为不可写；原实现在此返回 true");
    }

    /// 正样例：可写目录判为可写，且探针文件不留。
    #[test]
    fn writable_dir_leaves_no_probe() {
        let d = dir("writable");
        assert!(is_writable(&d));
        let left: Vec<_> = std::fs::read_dir(&d)
            .expect("读夹具目录")
            .filter_map(Result::ok)
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert!(left.is_empty(), "探针文件未清理：{left:?}");
    }

    /// 原子替换：重写后正文正确，且不留 .tmp。
    #[test]
    fn rewrite_is_atomic_and_leaves_no_tmp() {
        let d = dir("rewrite");
        let s = Spool::new(&d, 1024 * 1024);
        for i in 0..5 {
            let out = s.append(&format!("{{\"n\":{i}}}")).expect("追加");
            assert_eq!(out.evicted, 0, "夹具容量足够，不该有淘汰");
        }
        s.drop_first(2).expect("丢弃前两条");
        let rest = s.read_lines().expect("读回");
        assert_eq!(rest.len(), 3, "应剩三条");
        assert!(
            rest[0].contains("\"n\":2"),
            "剩下的应是后三条，实际 {rest:?}"
        );
        let tmp_left: Vec<_> = std::fs::read_dir(&d)
            .expect("读目录")
            .filter_map(Result::ok)
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.ends_with(".tmp"))
            .collect();
        assert!(tmp_left.is_empty(), "临时文件未清理：{tmp_left:?}");
    }
}

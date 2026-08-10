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
    Io { path: PathBuf, detail: String },
    /// 单条记录本身就超过容量上限，落盘无从谈起。
    RecordTooLarge { bytes: usize, max_bytes: u64 },
}

impl std::fmt::Display for SpoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpoolError::Io { path, detail } => write!(f, "spool {} 操作失败：{detail}", path.display()),
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
        Self { dir: dir.into(), max_bytes }
    }

    pub fn file(&self) -> PathBuf {
        self.dir.join(PENDING_FILE)
    }

    pub fn ensure_dir(&self) -> Result<(), SpoolError> {
        std::fs::create_dir_all(&self.dir)
            .map_err(|e| SpoolError::Io { path: self.dir.clone(), detail: e.to_string() })
    }

    /// 追加一条。超上限时从最旧一条开始丢，直到容得下新的一条。
    /// 绝不阻塞写出：写出进程被 spool 卡住就等于丢的是当下这条。
    pub fn append(&self, record: &str) -> Result<AppendOutcome, SpoolError> {
        self.ensure_dir()?;
        let line = format!("{}\n", record.replace('\n', " "));
        if line.len() as u64 > self.max_bytes {
            return Err(SpoolError::RecordTooLarge { bytes: line.len(), max_bytes: self.max_bytes });
        }
        let mut lines = self.read_lines()?;
        lines.push(line.trim_end().to_string());
        let mut evicted = 0;
        while total_bytes(&lines) > self.max_bytes {
            lines.remove(0);
            evicted += 1;
        }
        self.rewrite(&lines)?;
        Ok(AppendOutcome { evicted, bytes_after: total_bytes(&lines) })
    }

    /// 按写入顺序读出全部待补写记录。
    pub fn read_lines(&self) -> Result<Vec<String>, SpoolError> {
        let path = self.file();
        match std::fs::read_to_string(&path) {
            Ok(text) => Ok(text.lines().filter(|l| !l.trim().is_empty()).map(str::to_string).collect()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(e) => Err(SpoolError::Io { path, detail: e.to_string() }),
        }
    }

    /// 补写成功后截断。只在全部补写成功后调用，否则会丢未确认的记录。
    pub fn truncate(&self) -> Result<(), SpoolError> {
        let path = self.file();
        match std::fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(SpoolError::Io { path, detail: e.to_string() }),
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

    fn rewrite(&self, lines: &[String]) -> Result<(), SpoolError> {
        let path = self.file();
        let mut file = std::fs::File::create(&path)
            .map_err(|e| SpoolError::Io { path: path.clone(), detail: e.to_string() })?;
        for l in lines {
            writeln!(file, "{l}").map_err(|e| SpoolError::Io { path: path.clone(), detail: e.to_string() })?;
        }
        file.flush().map_err(|e| SpoolError::Io { path, detail: e.to_string() })
    }
}

fn total_bytes(lines: &[String]) -> u64 {
    lines.iter().map(|l| l.len() as u64 + 1).sum()
}

/// 供测试与调用方判定 spool 目录是否可用。
pub fn is_writable(dir: &Path) -> bool {
    std::fs::create_dir_all(dir).is_ok()
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
        assert_eq!(spool.read_lines().unwrap().len(), 1, "一帧一行，内嵌换行不得撕开一条记录");
        std::fs::remove_dir_all(&dir).ok();
    }
}

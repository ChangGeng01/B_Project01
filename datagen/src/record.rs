//! 数据集的记录模型与确定性编码。
//!
//! 为什么自带一套文本编码而不用 serde_json：本 crate 不新增依赖，且判据是「字节一致」，
//! 需要的是一个字段顺序、换行、空值表示三者全部写死的编码，而不是通用序列化。
//!
//! 为什么 NULL 用 `\N`：这是 PostgreSQL `COPY ... FORMAT text` 的空值记号，
//! 后续阶段把样本档灌进库时不需要再定第二套约定；空字符串与 NULL 因此可区分。

use std::fmt::Write as _;

/// 编码格式版本。改动字段顺序、分隔符或空值记号必须同批递增本值，
/// 否则旧样本档与新生成器在同一 seed 下字节不一致而无人察觉。
pub const FORMAT_VERSION: u32 = 1;

/// 空值记号，与 PostgreSQL COPY 文本格式一致。
pub const NULL_TOKEN: &str = "\\N";

const FIELD_SEP: char = '\t';
const LINE_SEP: char = '\n';

/// 字段取值。`Null` 与空字符串是两回事，不合并。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Value {
    Text(String),
    Int(i64),
    Bool(bool),
    Null,
}

impl Value {
    fn encode(&self) -> String {
        match self {
            Value::Text(s) => s.clone(),
            Value::Int(v) => v.to_string(),
            Value::Bool(v) => if *v { "true" } else { "false" }.to_string(),
            Value::Null => NULL_TOKEN.to_string(),
        }
    }
}

/// 一条记录。`kind` 是逻辑实体名，不是物理表名——阶段 1 不建任何业务表，
/// 物理表名由后续阶段的迁移决定，此处不预先发明。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Record {
    kind: &'static str,
    fields: Vec<(&'static str, Value)>,
}

impl Record {
    /// 构造一条记录。字段顺序即编码顺序，由调用方一次给定，之后不可改。
    pub fn new(kind: &'static str, fields: Vec<(&'static str, Value)>) -> Self {
        Self { kind, fields }
    }

    pub fn kind(&self) -> &'static str {
        self.kind
    }

    pub fn field(&self, name: &str) -> Option<&Value> {
        self.fields.iter().find(|(n, _)| *n == name).map(|(_, v)| v)
    }
}

/// 一个样本档的完整产出。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dataset {
    scale: &'static str,
    seed: u64,
    records: Vec<Record>,
}

/// 编码期可检出的越界输入。分隔符与换行进了取值会静默损坏整行，必须显式失败。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EncodeError {
    IllegalChar { kind: &'static str, field: &'static str, ch: char },
}

impl std::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodeError::IllegalChar { kind, field, ch } => write!(
                f,
                "记录 {kind} 的字段 {field} 含非法字符 {ch:?}：制表符、换行与反斜杠会破坏行结构"
            ),
        }
    }
}

impl std::error::Error for EncodeError {}

impl Dataset {
    pub fn new(scale: &'static str, seed: u64, records: Vec<Record>) -> Self {
        Self { scale, seed, records }
    }

    pub fn records(&self) -> &[Record] {
        &self.records
    }

    /// 按 `kind` 数记录条数，供样本档形状断言使用。
    pub fn count_of(&self, kind: &str) -> usize {
        self.records.iter().filter(|r| r.kind == kind).count()
    }

    /// 确定性编码。同一 `Dataset` 恒等，且不含任何时钟、路径、环境或哈希随机化来源。
    pub fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        let mut out = String::new();
        // 表头让样本档自描述：读到文件的人不必回查生成器就知道它由哪个版本、哪组入参产出。
        // 生成器版本入表头，是因为技术基线第 625 行要求生成器版本化并随认证结论冻结——
        // 换了生成器版本的实测结果不得与旧结论混用，而混用与否要能从文件本身看出来。
        let _ = write!(out, "#! format={FORMAT_VERSION}{LINE_SEP}");
        let _ = write!(out, "#! generator=ep-datagen/{}{LINE_SEP}", env!("CARGO_PKG_VERSION"));
        let _ = write!(out, "#! scale={}{LINE_SEP}", self.scale);
        let _ = write!(out, "#! seed={}{LINE_SEP}", self.seed);
        let _ = write!(out, "#! records={}{LINE_SEP}", self.records.len());

        for record in &self.records {
            let mut line = String::from(record.kind);
            for (name, value) in &record.fields {
                let encoded = value.encode();
                if let Some(ch) = illegal_char(&encoded) {
                    return Err(EncodeError::IllegalChar { kind: record.kind, field: name, ch });
                }
                let _ = write!(line, "{FIELD_SEP}{name}={encoded}");
            }
            line.push(LINE_SEP);
            out.push_str(&line);
        }
        Ok(out.into_bytes())
    }
}

fn illegal_char(s: &str) -> Option<char> {
    s.chars().find(|c| matches!(c, '\t' | '\n' | '\r' | '\\')).and_then(|c| {
        // NULL 记号本身就是反斜杠开头，它由编码器自己产出而不是来自取值，放行。
        if s == NULL_TOKEN {
            None
        } else {
            Some(c)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(text: &str) -> Dataset {
        Dataset::new(
            "unit",
            1,
            vec![Record::new("thing", vec![("code", Value::Text(text.to_string()))])],
        )
    }

    #[test]
    fn encode_is_stable_and_uses_lf_only() {
        let bytes = sample("A-1").encode().expect("合法取值应能编码");
        assert_eq!(bytes, sample("A-1").encode().unwrap());
        assert!(!bytes.contains(&b'\r'), "不得出现 CR，换行必须只有 LF");
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.starts_with("#! format=1\n"));
        assert!(text.ends_with("thing\tcode=A-1\n"));
    }

    /// 表头必须自描述：格式版本、生成器版本、档位、seed 与条数五项齐全。
    #[test]
    fn header_carries_generator_version_and_inputs() {
        let text = String::from_utf8(sample("A-1").encode().unwrap()).unwrap();
        let head: Vec<&str> = text.lines().take(5).collect();
        assert_eq!(
            head,
            vec![
                "#! format=1",
                &format!("#! generator=ep-datagen/{}", env!("CARGO_PKG_VERSION")),
                "#! scale=unit",
                "#! seed=1",
                "#! records=1",
            ]
        );
    }

    /// 负样例：取值里混进制表符必须报错，而不是产出一行看不出错的坏数据。
    #[test]
    fn tab_in_value_is_rejected() {
        let err = sample("A\t1").encode().expect_err("含制表符必须失败");
        assert_eq!(
            err,
            EncodeError::IllegalChar { kind: "thing", field: "code", ch: '\t' }
        );
    }

    /// 负样例：换行同样必须报错。
    #[test]
    fn newline_in_value_is_rejected() {
        assert!(matches!(
            sample("A\n1").encode(),
            Err(EncodeError::IllegalChar { ch: '\n', .. })
        ));
    }

    /// NULL 与空字符串编码后必须不同，不得合并。
    #[test]
    fn null_differs_from_empty_text() {
        let with_null = Dataset::new("unit", 1, vec![Record::new("t", vec![("a", Value::Null)])]);
        let with_empty = Dataset::new(
            "unit",
            1,
            vec![Record::new("t", vec![("a", Value::Text(String::new()))])],
        );
        assert_ne!(with_null.encode().unwrap(), with_empty.encode().unwrap());
    }

    #[test]
    fn count_of_counts_by_kind() {
        let ds = Dataset::new(
            "unit",
            1,
            vec![
                Record::new("a", vec![]),
                Record::new("b", vec![]),
                Record::new("a", vec![]),
            ],
        );
        assert_eq!(ds.count_of("a"), 2);
        assert_eq!(ds.count_of("b"), 1);
        assert_eq!(ds.count_of("c"), 0);
    }
}

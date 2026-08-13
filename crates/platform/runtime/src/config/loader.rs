//! 分层配置加载。顺序为内置默认、主配置、片段目录字典序、环境变量、命令行。
//!
//! 后一层覆盖前一层，表按键递归合并，标量与数组整体替换。数组不做逐元素合并：
//! 合并语义会让「清空一个白名单」变得无法表达。

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use toml::Value;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ConfigError {
    /// 读文件失败。缺主配置文件不算错，缺了指定的文件算错。
    Io { path: PathBuf, detail: String },
    /// TOML 语法错误。
    Syntax { layer: String, detail: String },
    /// 环境变量或命令行的键路径不合法。
    KeyPath {
        layer: String,
        key: String,
        detail: String,
    },
    /// 反序列化失败，含未知键与类型错误。消息里必须带键路径。
    Shape(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io { path, detail } => {
                write!(f, "读取配置 {} 失败：{detail}", path.display())
            }
            ConfigError::Syntax { layer, detail } => write!(f, "配置层 {layer} 语法错误：{detail}"),
            ConfigError::KeyPath { layer, key, detail } => {
                write!(f, "配置层 {layer} 的键 {key} 不合法：{detail}")
            }
            ConfigError::Shape(detail) => write!(f, "配置结构不符：{detail}"),
        }
    }
}

impl std::error::Error for ConfigError {}

/// 已合并的配置树。逐层调用后再 [`ConfigLoader::finish`]。
#[derive(Debug, Default)]
pub struct ConfigLoader {
    table: toml::value::Table,
    /// 记录每个键最后由哪一层写入，供 `--check` 与排障打印来源。
    origin: BTreeMap<String, String>,
}

impl ConfigLoader {
    pub fn new() -> Self {
        Self::default()
    }

    /// 内置默认层与进程固定层都走这里。
    pub fn layer_str(&mut self, layer: &str, text: &str) -> Result<(), ConfigError> {
        let parsed: Value = text.parse::<Value>().map_err(|e| ConfigError::Syntax {
            layer: layer.to_string(),
            detail: e.to_string(),
        })?;
        let Value::Table(t) = parsed else {
            return Err(ConfigError::Syntax {
                layer: layer.to_string(),
                detail: "顶层必须是表".into(),
            });
        };
        merge(&mut self.table, t, layer, "", &mut self.origin);
        Ok(())
    }

    /// 主配置文件。文件不存在时按「本层为空」处理并返回 false，
    /// 因为容器里的默认部署允许全部取内置默认；路径读不动则是错误。
    pub fn layer_file(&mut self, path: &Path) -> Result<bool, ConfigError> {
        match std::fs::read_to_string(path) {
            Ok(text) => {
                self.layer_str(&path.display().to_string(), &text)?;
                Ok(true)
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(ConfigError::Io {
                path: path.to_path_buf(),
                detail: e.to_string(),
            }),
        }
    }

    /// 片段目录，按文件名字典序逐个覆盖，只取 `.toml`。
    pub fn layer_dir(&mut self, dir: &Path) -> Result<usize, ConfigError> {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
            Err(e) => {
                return Err(ConfigError::Io {
                    path: dir.to_path_buf(),
                    detail: e.to_string(),
                })
            }
        };
        let mut files: Vec<PathBuf> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.is_file() && p.extension().is_some_and(|x| x == "toml"))
            .collect();
        files.sort();
        for file in &files {
            self.layer_file(file)?;
        }
        Ok(files.len())
    }

    /// 环境变量层。`EP__DB__POOL__RW_MAX` 映射到 `db.pool.rw_max`。
    pub fn layer_env<I>(&mut self, prefix: &str, vars: I) -> Result<usize, ConfigError>
    where
        I: IntoIterator<Item = (String, String)>,
    {
        let head = format!("{prefix}__");
        let mut applied = 0;
        let mut pairs: Vec<(String, String)> = vars
            .into_iter()
            .filter(|(k, _)| k.starts_with(&head))
            .collect();
        // 字典序，保证同一组环境变量的生效结果与遍历顺序无关。
        pairs.sort();
        for (key, raw) in pairs {
            let path: Vec<String> = key[head.len()..]
                .split("__")
                .map(|s| s.to_ascii_lowercase())
                .collect();
            if path.iter().any(|s| s.is_empty()) {
                return Err(ConfigError::KeyPath {
                    layer: "env".into(),
                    key: key.clone(),
                    detail: "键路径中出现空段".into(),
                });
            }
            self.set_path("env", &path, &raw)?;
            applied += 1;
        }
        Ok(applied)
    }

    /// 命令行层，形如 `--set db.pool.rw_max=30`。
    pub fn layer_cli(&mut self, sets: &[String]) -> Result<usize, ConfigError> {
        for item in sets {
            let Some((key, raw)) = item.split_once('=') else {
                return Err(ConfigError::KeyPath {
                    layer: "cli".into(),
                    key: item.clone(),
                    detail: "形态必须是 <键路径>=<取值>".into(),
                });
            };
            let path: Vec<String> = key.split('.').map(|s| s.trim().to_string()).collect();
            if path.iter().any(|s| s.is_empty()) {
                return Err(ConfigError::KeyPath {
                    layer: "cli".into(),
                    key: key.to_string(),
                    detail: "键路径中出现空段".into(),
                });
            }
            self.set_path("cli", &path, raw)?;
        }
        Ok(sets.len())
    }

    fn set_path(&mut self, layer: &str, path: &[String], raw: &str) -> Result<(), ConfigError> {
        let value = scalar_from_str(raw);
        let mut cursor = &mut self.table;
        for seg in &path[..path.len() - 1] {
            let entry = cursor
                .entry(seg.clone())
                .or_insert_with(|| Value::Table(toml::value::Table::new()));
            if !entry.is_table() {
                return Err(ConfigError::KeyPath {
                    layer: layer.to_string(),
                    key: path.join("."),
                    detail: format!("{seg} 已是标量，不能再向下取键"),
                });
            }
            cursor = entry.as_table_mut().expect("上一行已判定为表");
        }
        let leaf = path[path.len() - 1].clone();
        self.origin.insert(path.join("."), layer.to_string());
        cursor.insert(leaf, value);
        Ok(())
    }

    pub fn origin_of(&self, key: &str) -> Option<&str> {
        self.origin.get(key).map(String::as_str)
    }

    pub fn merged(&self) -> &toml::value::Table {
        &self.table
    }

    /// 落到目标结构。未知键与类型错误在这里被拒，消息里带键路径。
    pub fn finish<T: DeserializeOwned>(self) -> Result<T, ConfigError> {
        Value::Table(self.table)
            .try_into::<T>()
            .map_err(|e| ConfigError::Shape(e.to_string()))
    }
}

/// 环境变量与命令行只给字符串，这里按 TOML 标量文法还原类型。
/// 还原不了就当字符串，不猜——猜错会把 `on` 变成 true。
fn scalar_from_str(raw: &str) -> Value {
    let probe = format!("v = {raw}");
    match probe.parse::<Value>() {
        Ok(Value::Table(t)) => t
            .get("v")
            .cloned()
            .unwrap_or_else(|| Value::String(raw.to_string())),
        _ => Value::String(raw.to_string()),
    }
}

fn merge(
    base: &mut toml::value::Table,
    over: toml::value::Table,
    layer: &str,
    prefix: &str,
    origin: &mut BTreeMap<String, String>,
) {
    for (k, v) in over {
        let path = if prefix.is_empty() {
            k.clone()
        } else {
            format!("{prefix}.{k}")
        };
        match (base.get_mut(&k), v) {
            (Some(Value::Table(existing)), Value::Table(incoming)) => {
                merge(existing, incoming, layer, &path, origin);
            }
            (_, incoming) => {
                origin.insert(path, layer.to_string());
                base.insert(k, incoming);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields, default)]
    struct Pool {
        rw_max: u16,
        ro_max: u16,
    }

    impl Default for Pool {
        fn default() -> Self {
            Self {
                rw_max: 20,
                ro_max: 10,
            }
        }
    }

    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields, default)]
    struct Db {
        host: String,
        pool: Pool,
    }

    impl Default for Db {
        fn default() -> Self {
            Self {
                host: "127.0.0.1".into(),
                pool: Pool::default(),
            }
        }
    }

    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields, default)]
    struct Root {
        db: Db,
        enabled: bool,
    }

    impl Default for Root {
        fn default() -> Self {
            Self {
                db: Db::default(),
                enabled: true,
            }
        }
    }

    #[test]
    fn later_layer_overrides_earlier_one_key_by_key() {
        let mut l = ConfigLoader::new();
        l.layer_str("defaults", "[db]\nhost = \"a\"\n[db.pool]\nrw_max = 20\n")
            .unwrap();
        l.layer_str("main", "[db]\nhost = \"b\"\n").unwrap();
        let cfg: Root = l.finish().unwrap();
        assert_eq!(cfg.db.host, "b");
        assert_eq!(cfg.db.pool.rw_max, 20, "同表内未被覆盖的键必须留存");
    }

    #[test]
    fn env_double_underscore_maps_to_nested_key() {
        let mut l = ConfigLoader::new();
        l.layer_str("defaults", "[db.pool]\nrw_max = 20\n").unwrap();
        l.layer_env(
            "EP",
            [("EP__DB__POOL__RW_MAX".to_string(), "30".to_string())],
        )
        .unwrap();
        assert_eq!(l.origin_of("db.pool.rw_max"), Some("env"));
        let cfg: Root = l.finish().unwrap();
        assert_eq!(cfg.db.pool.rw_max, 30);
    }

    #[test]
    fn cli_layer_wins_over_env_layer() {
        let mut l = ConfigLoader::new();
        l.layer_env("EP", [("EP__DB__HOST".to_string(), "from-env".to_string())])
            .unwrap();
        l.layer_cli(&["db.host=from-cli".to_string()]).unwrap();
        assert_eq!(l.origin_of("db.host"), Some("cli"));
        let cfg: Root = l.finish().unwrap();
        assert_eq!(cfg.db.host, "from-cli");
    }

    #[test]
    fn fragment_dir_is_applied_in_lexicographic_order() {
        let dir = std::env::temp_dir().join(format!("ep-cfg-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("建临时片段目录");
        std::fs::write(dir.join("10-a.toml"), "[db]\nhost = \"first\"\n").unwrap();
        std::fs::write(dir.join("20-b.toml"), "[db]\nhost = \"second\"\n").unwrap();
        // 非 .toml 的文件不参与，避免编辑器备份文件改变生效结果。
        std::fs::write(dir.join("30-c.toml.bak"), "[db]\nhost = \"ignored\"\n").unwrap();
        let mut l = ConfigLoader::new();
        assert_eq!(l.layer_dir(&dir).unwrap(), 2);
        let cfg: Root = l.finish().unwrap();
        assert_eq!(cfg.db.host, "second");
        std::fs::remove_dir_all(&dir).ok();
    }

    // 负样例断言的是 deny_unknown_fields 这条规则本身。
    #[test]
    fn unknown_key_is_rejected_with_its_path() {
        let mut l = ConfigLoader::new();
        l.layer_str("main", "[db]\nhostt = \"typo\"\n").unwrap();
        let err = l.finish::<Root>().expect_err("未知键必须被拒");
        assert!(
            format!("{err}").contains("hostt"),
            "错误消息必须带键路径：{err}"
        );
    }

    #[test]
    fn type_error_message_carries_the_key_path() {
        let mut l = ConfigLoader::new();
        l.layer_cli(&["db.pool.rw_max=not-a-number".to_string()])
            .unwrap();
        let err = l.finish::<Root>().expect_err("类型错误必须被拒");
        assert!(format!("{err}").contains("rw_max"), "{err}");
    }

    #[test]
    fn missing_main_file_is_not_an_error() {
        let mut l = ConfigLoader::new();
        assert!(!l
            .layer_file(Path::new("/nonexistent/ep-core.toml"))
            .unwrap());
    }

    #[test]
    fn scalars_keep_their_toml_types() {
        assert_eq!(scalar_from_str("30"), Value::Integer(30));
        assert_eq!(scalar_from_str("true"), Value::Boolean(true));
        assert_eq!(
            scalar_from_str("[1, 2]"),
            Value::Array(vec![Value::Integer(1), Value::Integer(2)])
        );
        assert_eq!(
            scalar_from_str("127.0.0.1:8080"),
            Value::String("127.0.0.1:8080".into())
        );
    }

    #[test]
    fn empty_key_segment_is_rejected() {
        let mut l = ConfigLoader::new();
        assert!(l.layer_cli(&["db..host=x".to_string()]).is_err());
        assert!(l
            .layer_env("EP", [("EP__DB____HOST".to_string(), "x".to_string())])
            .is_err());
    }
}

//! 启动自检注册表。落点由技术基线第 7.3 节与裁定 C-25 定死在本文件。
//!
//! 判定与分级分开：自检项自己只回答「过、不过、未覆盖、不适用」四种事实，
//! 「不过」要不要拦启动由 severity 决定，由注册表统一映射。让每个项自己
//! 决定拦不拦，等于把闸门散落到十个地方。

use std::sync::Arc;

use serde::Serialize;

use crate::process::ProcessKind;

/// 取值域只有两个，不设第三值。
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Severity {
    Blocking,
    Degrading,
}

/// 自检项自己给出的结论。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Verdict {
    Pass(String),
    /// 判定执行了，结论是不符。
    Fail(String),
    /// 被测对象在本阶段还不存在，或判定手段尚未交付。这是一种如实上报，
    /// 不是空实现，也绝不等同于通过。
    Pending(String),
    /// 本进程不具备该项的前提，例如不持有 SQL 会话。
    NotApplicable(String),
}

/// 报告里的结论。`Fail` 按 severity 落到 FAILED 或 DEGRADED。
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Outcome {
    Passed,
    Failed,
    Degraded,
    Pending,
    NotApplicable,
}

#[async_trait::async_trait]
pub trait SelfCheckRun: Send + Sync {
    async fn run(&self) -> Verdict;
}

/// 一个自检项。字段与技术基线第 7.3 节冻结的四项一致。
#[derive(Clone)]
pub struct SelfCheckItem {
    pub name: &'static str,
    pub title: &'static str,
    pub severity: Severity,
    pub run: Arc<dyn SelfCheckRun>,
}

impl SelfCheckItem {
    pub fn new(
        name: &'static str,
        title: &'static str,
        severity: Severity,
        run: Arc<dyn SelfCheckRun>,
    ) -> Self {
        Self { name, title, severity, run }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DuplicateName(pub &'static str);

impl std::fmt::Display for DuplicateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "自检项 {} 重复注册", self.0)
    }
}

impl std::error::Error for DuplicateName {}

/// 注册顺序即执行顺序，报告按注册顺序输出。
#[derive(Clone, Default)]
pub struct SelfCheckRegistry {
    items: Vec<SelfCheckItem>,
}

impl SelfCheckRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, item: SelfCheckItem) -> Result<(), DuplicateName> {
        if self.items.iter().any(|i| i.name == item.name) {
            return Err(DuplicateName(item.name));
        }
        self.items.push(item);
        Ok(())
    }

    pub fn names(&self) -> Vec<&'static str> {
        self.items.iter().map(|i| i.name).collect()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub async fn run_all(&self, process: ProcessKind, version: &str) -> SelfCheckReport {
        let mut items = Vec::with_capacity(self.items.len());
        for item in &self.items {
            let (outcome, detail) = match item.run.run().await {
                Verdict::Pass(d) => (Outcome::Passed, d),
                Verdict::Fail(d) => match item.severity {
                    Severity::Blocking => (Outcome::Failed, d),
                    Severity::Degrading => (Outcome::Degraded, d),
                },
                Verdict::Pending(d) => (Outcome::Pending, d),
                Verdict::NotApplicable(d) => (Outcome::NotApplicable, d),
            };
            items.push(ItemReport {
                name: item.name,
                title: item.title,
                severity: item.severity,
                outcome,
                detail,
            });
        }
        let overall = overall_of(&items);
        SelfCheckReport { process: process.name(), version: version.to_string(), items, overall }
    }
}

fn overall_of(items: &[ItemReport]) -> Outcome {
    if items.iter().any(|i| i.outcome == Outcome::Failed) {
        Outcome::Failed
    } else if items.iter().any(|i| i.outcome == Outcome::Degraded) {
        Outcome::Degraded
    } else {
        Outcome::Passed
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ItemReport {
    pub name: &'static str,
    pub title: &'static str,
    pub severity: Severity,
    pub outcome: Outcome,
    pub detail: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct SelfCheckReport {
    pub process: &'static str,
    pub version: String,
    pub items: Vec<ItemReport>,
    pub overall: Outcome,
}

impl SelfCheckReport {
    pub fn pending_items(&self) -> usize {
        self.items.iter().filter(|i| i.outcome == Outcome::Pending).count()
    }

    /// 运行期是否允许继续启动：只看 Blocking 项。
    pub fn startup_allowed(&self) -> bool {
        self.overall != Outcome::Failed
    }

    /// `--check` 的判定严于运行期：FAILED 与 DEGRADED 都是非零退出。
    /// Pending 不计入成败，但它在报告里必须显形，见 `pending_items`。
    pub fn check_exit_code(&self) -> u8 {
        match self.overall {
            Outcome::Failed | Outcome::Degraded => crate::lifecycle::EXIT_CONFIG_OR_SELFCHECK,
            _ => 0,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|e| {
            // 报告序列化失败本身也要看得见，绝不能返回一个空对象假装成功。
            format!("{{\"process\":\"{}\",\"error\":\"报告序列化失败: {e}\"}}", self.process)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fixed(Verdict);

    #[async_trait::async_trait]
    impl SelfCheckRun for Fixed {
        async fn run(&self) -> Verdict {
            self.0.clone()
        }
    }

    fn item(name: &'static str, sev: Severity, v: Verdict) -> SelfCheckItem {
        SelfCheckItem::new(name, name, sev, Arc::new(Fixed(v)))
    }

    async fn report(items: Vec<SelfCheckItem>) -> SelfCheckReport {
        let mut reg = SelfCheckRegistry::new();
        for i in items {
            reg.register(i).expect("测试内不重名");
        }
        reg.run_all(ProcessKind::CoreServer, "0.1.0").await
    }

    #[tokio::test]
    async fn blocking_failure_is_failed_and_blocks_startup() {
        let r = report(vec![item("a", Severity::Blocking, Verdict::Fail("坏了".into()))]).await;
        assert_eq!(r.items[0].outcome, Outcome::Failed);
        assert_eq!(r.overall, Outcome::Failed);
        assert!(!r.startup_allowed());
        assert_eq!(r.check_exit_code(), 78);
    }

    #[tokio::test]
    async fn degrading_failure_is_degraded_and_allows_startup_but_fails_check() {
        let r = report(vec![item("a", Severity::Degrading, Verdict::Fail("链断了".into()))]).await;
        assert_eq!(r.items[0].outcome, Outcome::Degraded);
        assert!(r.startup_allowed(), "Degrading 项不阻止启动");
        assert_eq!(r.check_exit_code(), 78, "--check 严于运行期");
    }

    #[tokio::test]
    async fn pending_is_reported_but_not_counted_as_pass_or_fail() {
        let r = report(vec![
            item("a", Severity::Blocking, Verdict::Pass("ok".into())),
            item("b", Severity::Blocking, Verdict::Pending("承接方为阶段 3b".into())),
        ])
        .await;
        assert_eq!(r.pending_items(), 1);
        assert_eq!(r.items[1].outcome, Outcome::Pending);
        assert_eq!(r.overall, Outcome::Passed);
        assert!(r.to_json().contains("PENDING"), "Pending 必须在报告里显形");
    }

    #[tokio::test]
    async fn report_follows_registration_order() {
        let r = report(vec![
            item("first", Severity::Blocking, Verdict::Pass(String::new())),
            item("second", Severity::Blocking, Verdict::Pass(String::new())),
        ])
        .await;
        assert_eq!(r.items.iter().map(|i| i.name).collect::<Vec<_>>(), ["first", "second"]);
    }

    // 负样例断言的是注册表这条规则本身：同名项不得注册两次。
    #[test]
    fn duplicate_registration_is_rejected() {
        let mut reg = SelfCheckRegistry::new();
        reg.register(item("config-parsed", Severity::Blocking, Verdict::Pass(String::new()))).unwrap();
        let err = reg
            .register(item("config-parsed", Severity::Blocking, Verdict::Pass(String::new())))
            .expect_err("重名必须被拒");
        assert_eq!(err, DuplicateName("config-parsed"));
        assert_eq!(reg.len(), 1);
    }

    #[tokio::test]
    async fn not_applicable_neither_passes_nor_fails() {
        let r = report(vec![item("a", Severity::Blocking, Verdict::NotApplicable("本进程不持 SQL 会话".into()))]).await;
        assert_eq!(r.items[0].outcome, Outcome::NotApplicable);
        assert_eq!(r.overall, Outcome::Passed);
        assert_eq!(r.check_exit_code(), 0);
    }
}

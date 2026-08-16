//! ep-platform-recon —— 内部对账与强制不变量校验。
//!
//! 归属阶段 9a（裁定 A-06），注册方固定为阶段 7、8、9b、11 四个。
//!
//! 本轮交付其中**可以脱库判定的那一半**：
//! [`model`] 值类型与豁免纪律、[`gate`] 关账拦截项、
//! [`subject_ref`] 差异事项的键集白名单、[`check`] 与 [`executor`] 两个契约 trait、
//! [`registry`] 校验项注册表与一次运行的汇总语义。
//!
//! 执行器的实现（逐法人遍历、`snapshot_transact` 导出快照、逐批传递）、
//! 三张表的迁移，按 A-06 归阶段 9a 的落库交付，本 crate 不碰数据库。
//!
//! # 为什么这几块值得先做
//!
//! 规格第 10.2 章逐字「差异清零前不得关账」，而**关账是不可逆的**：
//! 已关闭期间不再接受任何凭证写入，首版又不做反结账。
//! 判松了会让一个账实不符的期间被永久关掉，判紧了只是关不成、运维来问一句。
//! **两种错的代价不对称**，所以这里每一条判定都取保守那一侧。
//!
//! 落到具体的三处：
//!
//! 一、[`subject_ref`] 是一道**数据外泄的闸**，不是数据整洁度的规矩。
//! 对账跑在规格第 7.7 章的内部对账系统安全上下文里，该上下文不调用字段投影器；
//! 一个校验项把读到的东西顺手塞进 `subject_ref`，那份数据就落进了差异表。
//!
//! 二、[`executor::summarize_run`] 里「注册表为空」判 `Failed` 而不是 `Completed`。
//! 这不是假想的边界——阶段 9a 交付本体那一刻，十五项里有十一项还不存在。
//!
//! 三、[`gate`] 新增的两条阻断项，期望值来自两个**互相独立**的源。
//! 拿注册表自己的内容当期望值，差集恒空，判定是恒真的。
//!
//! # 未覆盖（明写，不以「校验过了」的外观掩盖）
//!
//! 一、**十五项的名册在本 crate 内没有被测对象。** 十五个实现体分属阶段 7、8、9b、11，
//! 本 crate 内一个都没有；卷内只找得到其中九个的具名码。[`registry`] 因此只提供
//! 按**项数**的谓词，真名册断言的落点在 job-worker 的 wiring。
//!
//! 二、**`UNFINISHED` 与 `FAILED` 的分界是本实现自定的。** 规格把五类终止成因
//! 全部归入「未完成」，全卷 `FAILED` 只在 A-06 那一行 CHECK 里出现过，
//! 阶段 14 的降级 kind 里也只有对应 `UNFINISHED` 的一项。见 [`executor`]。
//!
//! 三、**`ReconRunStatus::Running` 在生产上大概率取不到。** `recon_runs` 按裁定 B-02
//! 登记为仅追加表且可变列白名单为空，而仅追加表的 `BEFORE UPDATE` 触发器一律 raise，
//! `RUNNING` 到终态的更新上线即被拒。连带 [`gate::CloseBlocker::ReconRunning`]
//! 整条是死路径。
//!
//! 四、**`DiscrepancyState` 的三个非 `Open` 取值全卷没有生产者。** 修复中、已修复、
//! 已豁免三个态是运维处置之后的态，而九个阶段计划里没有任何一条给出这三个迁移的
//! 端点、用例或承接方。后果是 [`model::validate_waiver`] 守的规则今天无人能触发，
//! 而「差异清零前不得关账」成了一条**没有解除路径**的约束。
//!
//! 五、**`subject_ref` 十个键的机读名是本实现取的**，计划只给了十个中文词；
//! 其中「凭证号」与「单据编号」的分工判不出来。见 [`subject_ref`]。
//!
//! 六、**`gate` 判的不是规格意义上的「受理前提」的全部。** 规格的受理前提含
//! 「期间是最早的打开期间」与「异步过账队列已清空」两项，两者都是 ledger 侧的事实，
//! recon 取不到，`CloseFacts` 里没有对应字段——判不到的那半今天是恒真的。
//! 本轮把措辞从「受理前提」改口为「关账拦截项」，不再声称自己判全了前提。

pub mod check;
pub mod executor;
pub mod gate;
pub mod model;
pub mod registry;
pub mod subject_ref;

/// `Id<ReconRun>` 的标记类型。
///
/// 落在本 crate 而不是 `ep_foundation::id::marker`：那张表按 `xtask archcheck`
/// 的冻结项逐名断言、不许增删，且它自己声明收的是**跨模块引用**的零大小标记；
/// 一次对账运行的标识只在本 crate 与阶段 9a 之间流转，够不上那个门槛。
/// `Id<T>` 用 `PhantomData<fn() -> T>`，对 `T` 无任何约束，crate 本地标记完全合法。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ReconRun;

pub use check::{validate_fresh_discrepancy, BatchOutcome, ReconCheck};
pub use executor::{summarize_run, validate_run_outcome, ReconExecutor, RunTally};
pub use gate::{check_close_admission, CloseBlocker, CloseFacts};
pub use model::{
    validate_waiver, BatchWindow, DiscrepancyState, ReconCategory, ReconDiscrepancy, ReconRunKind,
    ReconRunOutcome, ReconRunStatus,
};
pub use registry::{ReconRegistry, RegisterCheckError, EXPECTED_REGISTERED_CHECK_COUNT};
pub use subject_ref::{is_allowed, validate_keys, SubjectRefError, ALLOWED_KEYS};

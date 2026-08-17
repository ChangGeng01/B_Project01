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
//! 二、**`FAILED` 已由裁定 F-14 撤销**，`status` 收为 `RUNNING`、`COMPLETED`、
//! `UNFINISHED` 三值。规格把五类终止成因全部归入「未完成」，
//! 阶段 14 的降级 kind 也只有对应 `UNFINISHED` 的一项；归因改由
//! [`model::TerminationCause`] 承担。原「名册不足」那条产生条件前移为起跑前闸门
//! （[`NotReady`]）——那不是一次运行的结果，是这次运行不该开始。
//!
//! 三、**`ReconRunStatus::Running` 是一条活路径**——裁定 F-14 把 `recon_runs`
//! 的登记由 `APPEND_ONLY` 改为 `IMMUTABLE_COLUMNS`（B-02 的 `mode` 本就有这个取值，
//! 带状态机的 `outbox_events` 与 `dead_letters` 用的都是它），
//! 可变列取 `status`、`batch_done`、`finished_at`、`termination_cause` 四列，
//! 证据列仍不可改。**未了结的一件**：谁替崩掉的进程把行推到终态，卷内没有答案，
//! 那是既有缺口（`ledger.period_close_requests` 上同样存在），见 F-14 末节。
//!
//! 四、**`DiscrepancyState` 的三个非 `Open` 取值首版没有生产者**——已由裁定 F-13 处置。
//! 该裁定推翻了「无解除路径」这个说法的前提：关账拦截读的是**本次校验的校验项结论**，
//! 不是差异行的累计集合；解除路径是规格逐字的「期间保持打开、按事项载明的内容修复后
//! 重新发起关账」，即补登与冲正来源事件这类业务动作，不是给差异行置态。
//! PRD 逐字「对账视图不提供任何调整、抹平或忽略差额的操作入口」，故首版无写端点。
//! 三个取值与 `repaired_by`、`approval_ref` 登记为首版不使用，**任何判据不得依赖它们**；
//! 其去留随附录辛第 12 条同批单裁。[`model::validate_waiver`] 因此在首版无调用方，
//! 保留是为了那一天真有豁免通道时规则已经在——但它今天不是一道生效的闸。
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
pub use executor::{summarize_run, validate_run_outcome, NotReady, ReconExecutor, RunTally};
pub use gate::{check_close_admission, CloseBlocker, CloseFacts};
pub use model::{
    BatchWindow, DiscrepancyState, ReconCategory, ReconDiscrepancy, ReconRunKind, ReconRunOutcome,
    ReconRunStatus, TerminationCause,
};
pub use registry::{ReconRegistry, RegisterCheckError, EXPECTED_REGISTERED_CHECK_COUNT};
pub use subject_ref::{is_allowed, validate_keys, SubjectRefError, ALLOWED_KEYS};

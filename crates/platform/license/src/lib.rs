//! ep-platform-license —— 模块许可、模块生命周期与受限运行的判定。
//!
//! 本轮交付其中**可以脱库判定的那一半**：
//! [`status`] 订阅许可状态与受限运行判定、[`restricted`] 受限运行下的动作归类、
//! [`module`] 安装态状态机、[`usage`] 命名用户用量、[`query`] 取用入口的签名。
//!
//! # 本轮最要紧的一处：状态值回答不了「该不该停业务写入」
//!
//! 裁定 A-05 冻结的 [`LicenseStatus`] 是 `Valid`/`ExpiringSoon`/`Expired`/`Revoked`；
//! 规格第 3.4 章的四态是生效/临期告警/**宽限期**/**受限运行**。两组不是同一组：
//! `Expired` 一个变体同时盖住宽限期（规格逐字「全部功能可用」）与受限运行
//! （规格逐字「业务写入、审批、集成出站和自动化任务停止」），两段后果相反、相隔 30 天。
//!
//! 所以本 crate 给出两个函数：[`classify_subscription`] 照 A-05 给状态值，
//! [`in_restricted_run`] 照规格第 3.4 章给后果。**调用方判停写取后者。**
//! 只落前一个函数的话，调用方三条路都错——见 [`status`] 的模块文档。
//!
//! # 三处判错方向的代价不对称，都取了保守侧
//!
//! 一、[`restricted::restricted_run_verdict`] 用**三档**而不是黑名单或白名单。
//! 规格的两张清单加起来不封闭，纯黑名单是恒真放行，纯白名单会拦掉续期文件导入
//! 从而让受限运行不可逆。归不了类的就报归不了类。
//!
//! 二、身份四项（账号停用、口令重置、凭据轮换、权限回收）在任何许可状态下一律放行，
//! 且不为这一组补写反向边界句——规格没写，补一条是自造规格。判紧的代价是
//! 安全事件发生时回收不了一个被盗账号的权限，不可事后补救。
//!
//! 三、[`module::transition`] 查表，查不到即非法，**没有兜底分支**。
//!
//! # 本轮不交付什么
//!
//! [`query::ModuleLicenseQuery`] 只有签名，没有任何实现体，也不留 `todo!()`。
//! 三个方法都要读 `platform_core` 的三张表。运行期的许可导入与模块启停入口
//! 按计划第 3.5.9 节归阶段 13b 与阶段 14。计划第 3.4.11 节的两处过滤点明写
//! 「都在 job-worker」，不在本 crate。
//!
//! # 未覆盖（明写，不以「校验过了」的外观掩盖）
//!
//! 一、**永久授权与维护订阅两项首版不交付**——已由裁定 F-17 处置（不再是未覆盖点）。
//! 本部署为自有企业内部使用，两项不存在；不建 `grant_type` 列，
//! [`classify_subscription`] 的名字因此表示「本部署唯一存在的授权形态」。
//!
//! **同批明写一件本 crate 的实情：许可四态在本部署是一套不会被行使的机制。**
//! 内部使用不存在「让自己公司的许可到期、然后进受限运行」这种情形。
//! 它为规格第 1 章「同时可向其他企业销售」那一面保留，
//! **不是本部署的实效机制**——不得据它推出任何运行期结论。
//!
//! 二、**可信时间构造不出来。** 规格要求判定基准取三者最晚值，其中
//! 「平台本地许可状态存储中按日落盘的最近一次时间戳」在 `license_grants` 上无对应列，
//! job-worker 的任务清单（封闭列举十二项）里也没有按日落盘的任务。
//! 后果是规格「本地系统时间早于可信时间时按可信时间判定」的防时钟回拨能力现在落空。
//! 本 crate 把 `today` 做成入参已是它能做的全部，保证不了传进来的是可信时间。
//!
//! 三、**`is_feature_enabled` 的合成公式没有出处。** `feature_flags.is_enabled`、
//! `requires_license` 与许可状态如何合成，全卷只列列名，无真值表、无布尔式。
//! 自拟一个「看似合理的公式」正是本卷禁止的静默判过，故本轮不实现该方法。
//!
//! 四、**`license_grants` 可有多行而 `license_status()` 不带参数。** 表上唯一约束
//! 只有 `license_no`，无单例约束、无 `is_active` 列；多张凭证取哪一张、零行返回什么，
//! 三个分支全卷未定义。
//!
//! 五、**`signature bytea not null` 从不被校验。** 规格第 3.2 章逐字
//! 「许可证使用数字签名离线验证」是正面能力承诺，但全卷未给算法与密钥引用
//! （对比审计侧两者都有配置键），且验签的自然落点是导入路径，而导入路径无阶段承接。
//!
//! 六、**模块停用的四停两留只落实了两停。** 规格第 5.6 章逐字「停用只停止该模块的
//! 界面入口、写入接口、定时任务和对外事件」；计划只落实了定时任务与对外事件两项，
//! 界面入口与写入接口两停全卷无落点。
//!
//! 七、**许可临期告警没有触发源。** 计划把「许可临期」的触发源指回本阶段，
//! 而本阶段按计划第 3.5.9 节不新增对外端点、按本轮只交付纯判定函数。
//! `notify` 侧的写入接口已备并已按这条告警标定扇出规模。
//!
//! 上述七条中的第一、二、五、七条**没有下游承接方**，不是「留给阶段 13b」。

pub mod module;
pub mod query;
pub mod restricted;
pub mod status;
pub mod usage;

pub use module::{transition, ModuleAction, ModuleState, ModuleTransitionError, TRANSITIONS};
pub use query::ModuleLicenseQuery;
pub use restricted::{
    restricted_run_verdict, RestrictedOp, RestrictedVerdict, ALLOWED_OPS, BLOCKED_OPS,
};
pub use status::{
    classify_subscription, in_restricted_run, LicenseFacts, LicenseStatus,
    EXPIRING_SOON_WINDOW_DAYS, GRACE_PERIOD_DAYS,
};
pub use usage::{named_user_usage_verdict, UsageVerdict};

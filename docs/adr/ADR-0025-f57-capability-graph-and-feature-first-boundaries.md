# ADR-0025 F-57 单一能力图与 feature-first 边界

- 状态：已接受（2026-08-24 用户批准）
- 出处：F-57 总体设计 §4.8、§4.9 与 §15.5
- 关系：窄取代旧计划中“每个业务域默认拆成 contract/domain/application 三个顶层 crate”的物理结构；不改变逻辑分层、事实唯一 owner、强类型命令、单一 PostgreSQL 权威或安全宪法

## 背景

现有 workspace 为多个业务域预建了 contract、domain、application 三套顶层 crate。逻辑分层本身正确，但大量物理 crate 仍是空骨架；继续照此扩展会让 Cargo DAG、版本、脚手架和跨层映射先于业务价值增长。与此同时，API discriminator、component shape、state binding、route、权限、UI、MCP、Excel 和 package manifest 分别保存能力语义，已经出现名称与登记漂移风险。

F-57 需要保留强边界，同时让第一条“客户—合同—订单—采购—交付—开票—收款—证据关闭”闭环尽早出现。因此必须区分逻辑层、物理 crate、运行进程和机器语义源，不能让四者一一映射。

## 决定

一、新业务实现单位为 feature-first bounded context。首条 CTC-01 的物理边界为 `customer-master`、`contracting`、`sales-order`、`procurement`、`inventory-fulfilment`、`sales-invoicing` 和 `receivable-cash`；`service-cycle`、`project-cycle`、`portal-experience` 和 `reporting` 按首次到期阶段加入。`billing-cash` 只可作为产品导航分组，不能成为跨越 invoice/finance 两个事实 owner 的内部直写 crate。不得建立一个包含全部业务政策的 `erp` 巨型 crate，也不得按每张表建立 crate。

二、每个 feature 优先使用一个 `ep-feature-<bounded-capability>` crate，内部组织 `public`、`domain`、`application` 和测试模块：

- `public` 是其他 feature 唯一可依赖的 command/query/fact/identifier 面；
- `domain` 与 `application` 默认私有；
- feature 之间禁止访问对方表、repository 或内部类型；
- 反向关系、循环业务和长链编排通过 committed fact 与 Objective 打断编译依赖环。

三、只有两个以上 feature 共同使用、且不包含具体业务政策的稳定机制才允许进入 `ep-platform-*`。价格、期间、合同、采购、库存、资金和服务状态不得为复用方便下沉到 platform/foundation。

四、adapter 继续按技术和 provider 边界存在，但每个 repository 实现只服务一个 feature-owned schema/port。Integration Gateway、Portal Gateway 和 Extension Host 的零业务 SQL 边界不因 crate 合并而改变。

五、系统建立唯一 `CapabilityGraphV1` authoring model。每个 capability 由唯一 ID、version 和 owning feature 标识；OpenAPI、Rust/TypeScript DTO、UI schema、权限、MCP/Excel、package/provider manifest、状态域与测试 manifest 都是同一 graph digest 的确定性投影，不得分别手工定义业务语义。

六、许可模块、能力包和 provider 的关系固定为：`MODULE_PACKAGE` 只投影许可；`CAPABILITY_PACKAGE` 携带能力图子图；`ProviderManifestV1` 只绑定能力到 carrier、外部契约与资源上限。三者不得重复拥有 command、状态或事实定义。

七、迁移采用 touched-feature 渐进策略，不做一次性大爆炸重排：

1. 禁止新增新的三 crate 空套件；
2. 首条 CTC-01 切片涉及的业务域先迁；
3. 现有 layer-first crate 可暂作兼容 facade，但不得新增业务规则；
4. facade 的删除必须有依赖归零、投影一致和 fresh PostgreSQL 证据；
5. 现有 API 五类 seed 首次确定性导入后降为 `GENERATED_PROJECTION`，ownership、legacy migration、FreshPG 和 CI profile 继续作为交付证据登记。

## 理由

feature-first 让一次业务改变所需的代码、测试和迁移更集中，同时保留 public contract 与事实 owner 的硬边界。单一能力图消除 API、权限、UI、MCP、Excel、包和自动化各自维护能力名称与 schema 的漂移。若继续采用空三 crate 套件和多套手工登记，团队会在尚未证明首条业务闭环前承担长期结构成本，且每增加一个端或 provider 都会放大不一致。

## 后果

正面：首条闭环可以跨 feature 并行开发；生成物可按同一 digest 验证；物理结构更少但边界更清楚；后续行业包和客户定制不需要客户专属内核分支。

代价：需要实现 CapabilityGraph compiler、稳定投影和 drift gate；现有三层骨架要渐进迁移；少数开发者不能再直接手改 OpenAPI、权限表或 UI schema。逻辑层仍存在于 feature 内，不能把“减少 crate”误解为允许 controller、domain 和 repository 混写。

## 影响范围

- 根 `Cargo.toml` workspace、业务 feature crate 与跨 feature dependency gate；
- CapabilityGraph authoring model、OpenAPI/Rust/TypeScript/UI/权限/MCP/Excel/package 投影；
- F-57 Task 1、CTC-01 纵向切片、依赖 DAG 与 L0–L3 CI；
- API 五类 seed 的 authoring/generated 状态；
- `cargo xtask archcheck`、generation digest 和发布证据聚合。

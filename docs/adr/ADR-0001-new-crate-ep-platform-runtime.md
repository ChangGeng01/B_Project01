# ADR-0001 新增 crate `ep-platform-runtime`

- 状态：部分被 ADR-0019 取代；`ep-platform-runtime` 共享库决定继续接受
- 取代说明：[ADR-0019](ADR-0019-f57-runtime-topology-and-measured-connection-budget.md) 只取代本文固定九进程和强制 `ai-inferer` 的表述；crate 职责与依赖方向继续有效
- 出处：阶段 1 计划第 13 节偏离一；技术基线第 1.2 节平台底座清单、第 7.3 节
- 影响的现存文件：`Cargo.toml` 的 `[workspace.dependencies]`、`crates/platform/runtime/`

## 背景

技术基线第 1.2 节的平台底座 crate 清单里没有一个承载进程运行时装配的位置，而同一份基线的第 7.3 节已经把 `SelfCheckRegistry` 的落点写死为 `crates/platform/runtime/src/selfcheck/registry.rs`。两处自相矛盾：按第 1.2 节该 crate 不存在，按第 7.3 节它必须存在且路径已定。

阶段 1 的八个基础进程都需要同一套东西；F-55 又增加第九个且唯一新增的产品常驻进程 `ai-inferer`，并要求在同一实施批次把它加入进程枚举、服务登记、资源单位与门禁。终态九个进程共享分层配置、进程生命周期状态机、启动自检注册表、健康与就绪端点、按需使用的 HTTP 服务器/中间件骨架以及 IPC 服务端接口。这套东西没有归属 crate 时，只能在各二进制里重复实现。

## 决定

新增 crate `ep-platform-runtime`，目录 `crates/platform/runtime/`。阶段 1 先装配进八个基础进程；F-55 阶段 13c 在创建 `ai-inferer` 时同批装配，终态覆盖全部九个产品常驻进程。它承载且只承载六样：进程生命周期状态机、分层配置加载、`SelfCheckRegistry`、健康与就绪端点、HTTP 服务器与中间件栈骨架、以 trait 表达的 IPC 服务端接口。

HTTP 服务器骨架直接构建在第三方 HTTP 库之上，工作区内既不存在也不新增任何 HTTP 系 `ep-adapter-*`。IPC 的具体传输实现留在 `ep-adapter-ipc`，由各进程在 `apps/<proc>/src/wiring/` 目录下注入。

## 理由

不新增的代价是同一套生命周期代码在终态九个二进制里各存一份，与文件规模纪律和单一事实源冲突，并且第 7.3 节写死的自检注册表路径无处安放。

新增的代价只有一条，即基线第 1.2 节的表要多一行。该 crate 只依赖 `ep-foundation` 与其他 `ep-platform-*`，apps 依赖它，不新增任何依赖边的方向，因此 `xtask archcheck` 的 `platform-acyclic`、`platform-no-adapter`、`platform-no-domain-or-app` 三条规则的判定面不因本决定而放宽。

## 后果

正面：终态九个进程的启动路径只有一份实现，自检注册表只有一处注册面，退出条件 6 的「未注册项数量只减不增」这条 CI 断言有唯一的被测对象。

负面：`ep-platform-runtime` 会成为九个进程的共同上游，它的任何破坏性改动一次影响九个二进制；这一点由它自身的覆盖率分档与依赖方向门禁约束，不另设机制。

## 影响范围

技术基线第 1.2 节的平台底座表增加 `ep-platform-runtime` 一行，职责列取本文「决定」一段的六项。该表不补冻结措辞，也不再作为 `xtask archcheck` 的比对面，理由见阶段 1 计划第 10 节退出条件 2：crate 清单逐项一致会把 crate 边界变成必须走基线修订才能移动的冻结物，真正要守的依赖方向由退出条件 3 的七条禁止项守住。

## 与新增 workspace 成员的关系

阶段 1 计划第 13 节新增决定一另外引入了两个工具类 workspace 成员：`xtask` 是纯开发期工具，不进任何制品；`tools/ep-migrate` 是随制品交付的一次性运维工具。当前平台为 Windows Server 原生部署，`ep-migrate` 不注册为 Windows 服务，由升级脚本在迁移窗口内以独立本地账户直接启动、等待退出并原样判定退出码。二者都不是常驻进程，不监听端口，不属于终态九进程清单。它们与本 ADR 的 `ep-platform-runtime` 是三件不同的事：前两者是 workspace 成员而不是被装配的库，后者是被九个产品常驻进程装配的库。此处一并记下，避免把「本阶段新增了哪些 crate」读成只有一条。

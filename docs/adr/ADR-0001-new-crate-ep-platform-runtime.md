# ADR-0001 新增 crate `ep-platform-runtime`

- 状态：已接受
- 出处：阶段 1 计划第 13 节偏离一；技术基线第 1.2 节平台底座清单、第 7.3 节
- 影响的现存文件：`Cargo.toml` 的 `[workspace.dependencies]`、`crates/platform/runtime/`

## 背景

技术基线第 1.2 节的平台底座 crate 清单里没有一个承载进程运行时装配的位置，而同一份基线的第 7.3 节已经把 `SelfCheckRegistry` 的落点写死为 `crates/platform/runtime/src/selfcheck/registry.rs`。两处自相矛盾：按第 1.2 节该 crate 不存在，按第 7.3 节它必须存在且路径已定。

八个进程都需要同一套东西：分层配置加载、进程生命周期状态机、启动自检注册表、健康与就绪端点、HTTP 服务器与中间件栈骨架。这套东西没有归属 crate 时，唯一的落法是在八个二进制里各写一份。

## 决定

新增 crate `ep-platform-runtime`，目录 `crates/platform/runtime/`，装配进全部八个进程。它承载且只承载六样：进程生命周期状态机、分层配置加载、`SelfCheckRegistry`、健康与就绪端点、HTTP 服务器与中间件栈骨架、以 trait 表达的 IPC 服务端接口。

HTTP 服务器骨架直接构建在第三方 HTTP 库之上，工作区内既不存在也不新增任何 HTTP 系 `ep-adapter-*`。IPC 的具体传输实现留在 `ep-adapter-ipc`，由各进程在 `apps/<proc>/src/wiring/` 目录下注入。

## 理由

不新增的代价是同一套生命周期代码在八个二进制里各存一份，与文件规模纪律和单一事实源冲突，并且第 7.3 节写死的自检注册表路径无处安放。

新增的代价只有一条，即基线第 1.2 节的表要多一行。该 crate 只依赖 `ep-foundation` 与其他 `ep-platform-*`，apps 依赖它，不新增任何依赖边的方向，因此 `xtask archcheck` 的 `platform-acyclic`、`platform-no-adapter`、`platform-no-domain-or-app` 三条规则的判定面不因本决定而放宽。

## 后果

正面：八个进程的启动路径只有一份实现，自检注册表只有一处注册面，退出条件 6 的「未注册项数量只减不增」这条 CI 断言有唯一的被测对象。

负面：`ep-platform-runtime` 会成为八个进程的共同上游，它的任何破坏性改动一次影响八个二进制；这一点由它自身的覆盖率分档与依赖方向门禁约束，不另设机制。

## 影响范围

技术基线第 1.2 节的平台底座表增加 `ep-platform-runtime` 一行，职责列取本文「决定」一段的六项。该表不补冻结措辞，也不再作为 `xtask archcheck` 的比对面，理由见阶段 1 计划第 10 节退出条件 2：crate 清单逐项一致会把 crate 边界变成必须走基线修订才能移动的冻结物，真正要守的依赖方向由退出条件 3 的七条禁止项守住。

## 与新增 workspace 成员的关系

阶段 1 计划第 13 节新增决定一另外引入了两个非交付用途的 workspace 成员：`xtask` 是纯开发期工具，不进任何制品；`tools/ep-migrate` 是一次性运维工具，随制品交付并以 systemd 的 oneshot 单元在升级窗口内执行。二者都不是常驻进程，不监听端口，不属于八进程清单。它们与本 ADR 的 `ep-platform-runtime` 是三件不同的事：前两者是 workspace 成员而不是被装配的库，后者是被八个进程装配的库。此处一并记下，避免把「本阶段新增了哪些 crate」读成只有一条。

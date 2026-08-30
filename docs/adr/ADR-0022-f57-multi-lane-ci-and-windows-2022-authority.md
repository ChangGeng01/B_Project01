# ADR-0022 F-57 多 lane CI 与 Windows Server 2022 权威

- 状态：已接受
- 出处：F-57 四端交付、Windows 权威节点与签名发布证据契约
- 取代范围：取代 ADR-0005 的单 Windows runner 和 Windows Server 2019 复核要求；其私有自建默认、薄适配器、Rust 唯一判定与 Authenticode 意图继续有效
- 2026-08-24 澄清：本 ADR 的 `authority lane` 是能够生成 Windows Authority 制品/协议证据的独立受控 Windows evidence environment，不是客户生产 P340；生产 Authority 不兼任通用 CI/build/aggregation runner，只参与本机安装、容量、UPS、存储和恢复证据

## 背景

F-57 同时交付 Windows Server 权威节点、Windows/macOS/iOS/Android Workbench 和跨平台签名证据。单一 Windows runner 无法产生 Apple 与 Android 原生证据；Windows Server 2019 也不是现行生产权威。若任一 runner 自行形成发布结论，四个平台会出现互不一致的门禁与不可聚合证据。

## 决定

一、~~私有、自托管 Forgejo + Woodpecker 继续作为可替换的默认 CI 平台。~~ **已由 [ADR-0027](ADR-0027-ci-platform-github-only.md) 取代（F-77）**：CI 平台单一取 GitHub Actions，执行器一律自托管；本条其余部分（平台只准备环境、注入批准的 secret、不承载业务判定）继续有效。平台配置只准备环境、注入批准的 secret 引用并调用 Rust-owned 命令；阶段、顺序、退出码、证据 schema 与最终 verdict 继续由 Rust 工具唯一拥有。

二、现行 CI 精确包含四条签名 lane：

1. Windows Server 2022 authority/Windows client lane；
2. Apple macOS+iOS lane；
3. Android lane；
4. signed aggregation lane。

每条执行 lane 产生 digest-bound、签名且带代际的独立证据；aggregation lane 拒绝缺失、重复、过期、错代或签名无效的 lane，并且只有聚合结果可以形成发布 verdict。

三、Windows 权威证据只接受 Windows Server 2022。Windows Server 2019 证据即使历史存在也不是现行复核或放行条件；Windows client 证据在同一 Windows lane 生成，但不能替代 Server 2022 的原生 SCM、MSVC/MSI、ACL/pipe、PostgreSQL 与 Authenticode 证据。

Windows Server 2022 evidence runner 与客户生产 Authority 是两个故障域。前者可以执行源码构建、依赖验证和 CI 证据签发；后者只验证并激活 digest 已固定的制品，并产生绑定本机的容量、存储、UPS、安装与恢复证据。任何 pipeline label、runner tag 或历史路径都不得要求生产 P340 承担通用 CI、依赖下载、厂商二进制签名或聚合职责。

四、生产 Windows 制品继续执行 Authenticode 与离线验签意图；CI 平台替换时只能重写薄适配器，不得复制、旁路或改写 Rust-owned verdict。

## 理由

四条 lane 与实际交付平台一一对应，并由单一聚合器保持统一判定。若继续采用单 Windows runner，Apple/Android 只能靠交叉编译或文档声明冒充原生证据；若保留 Server 2019 为现行要求，会同时维护两个互斥权威基线。

## 后果

正面：每个平台都有原生证据，发布结论只有一个；编排面不承载业务判定（F-77：原句「Forgejo/Woodpecker 仍可替换」随决定一被 ADR-0027 取代而失效）。

代价：发布必须协调四条 lane 的签名、时效与代际，任一 lane 缺失都会失败关闭；Windows Server 2022 runner 和 Apple runner 成为必要研发基础设施。

## 影响范围

- CI workflow、pipeline adapter、Rust CI/release aggregator；
- Windows Server 2022/MSVC/MSI/Authenticode 证据；
- macOS+iOS、Android 与 Windows client 构建测试；
- 发布清单、签名证据根与 F-57 最终发布门。

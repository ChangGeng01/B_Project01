# ADR-0004 旧 Linux musl/scratch 交付决定（已被 Windows 原生交付取代）

- 状态：已取代
- 原出处：阶段 1 计划第 13 节新增决定七
- 取代依据：`00c-gap-ruling.md` 裁定 F-08 及其 Windows/CI 最终冻结

## 背景

本 ADR 原先把服务端构建目标固定为 `x86_64-unknown-linux-musl`，并以 `scratch` OCI 镜像交付。服务端目标平台后来正式改为 Windows Server 2019 至 2022 原生服务，认证基线冻结在 Windows Server 2022，且明确不使用 Linux、WSL、Linux 容器，也不把产品服务、数据库或客户主数据卷放入 Hyper-V 或其他虚拟机层。F-55 只增加一个不改变部署形态的窄例外：可选 `LOCAL_WINDOWS_HYPERV_CONTAINER` 可为单次 MCP 插件调用启动短命 Hyper-V-isolated Windows utility VM；它不是产品服务或通用容器平台。因此原 musl/scratch 决定已失去被交付对象。

## 当前决定

| 项 | 现行取值 |
|---|---|
| 服务端构建目标 | `x86_64-pc-windows-msvc` |
| 运行形态 | 产品服务与数据库均为 Windows 服务控制管理器原生进程；唯一 Hyper-V 例外是 F-55 可选、逐调用短命且不承载产品服务/数据库/客户主数据卷的 MCP 插件 utility VM |
| 交付形态 | 同一份 MSI 或压缩包、PE 二进制与服务注册脚本 |
| TLS | rustls 加配置指定的 CA bundle |
| 生产签名 | Authenticode |
| 开发签名 | 内部 ECDSA P-256，仅限非生产制品 |

生产 Authenticode 证书可由软件厂商或客户提供，两种来源走同一签名接口、审计记录与客户侧验签门禁。

## 理由

Windows Server 2019 至 2022 曾是 F-51 冻结的原生服务端区间；[ADR-0022](ADR-0022-f57-multi-lane-ci-and-windows-2022-authority.md) 已进一步把现行生产权威收窄为 Windows Server 2022，Windows Server 2019 只保留历史追溯且不得进入现行发布证据。一份 Linux OCI/scratch 制品不能成为该原生交付路径的交付物；继续保留 musl/scratch 为并行目标会制造第二套部署、签名、备份与认证语义，违反单一实现路径。F-55 的短命插件 utility VM 使用签名 Windows image、由 `plugin-host` 经 HCS 逐调用创建与销毁，不构成第二套产品部署路径。

显式 CA bundle 保留，是为了让客户可使用自己的内网 CA，并使 TLS 信任根不依赖机器全局证书库；保留这一行为不等于保留 scratch 镜像。

## 后果

- 全部现行构建、发布、安装、恢复与认证文档以 Windows PE/MSVC 为目标；musl、glibc、OCI、scratch 与 Linux 文件权限只可作为历史说明，不得作为当前实现要求。
- PE 可复现构建、Windows 服务、DACL、命名管道与 Job Object 的实机行为由 F-08 的 18 项首批实施门禁取证。未执行前不得声称已经通过。
- 本文件名为追溯历史而保留，不表示 musl 仍是现行目标。

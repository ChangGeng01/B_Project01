# deploy/ 单机编排：交付物 D-05 与 D-13

`deploy/README.md` 已交付的是 D-06 的资源限额 drop-in。本文件接它后面那一句，
交付它明确排除在外的三样：八个 slice 单元本身、Podman Quadlet 与 Docker Compose 两套等价编排文件，
以及交付物 D-13 的本地开发环境。

配额一个数都不在本文件涉及的文件里。四类取值只落在 `systemd/system/<slice>.d/10-resource-limits.conf`
一处，编排两套一律只引用 slice 名。哪一侧多写一个 `CPUWeight` 或 `mem_limit`，
`scripts/verify-orchestration-equivalence.py` 都会以 `SL-QUOTA-IN-ORCH` 或 `SL-QUOTA-IN-UNIT` 判红。

## 一、文件清单

| 路径 | 是什么 |
|---|---|
| `systemd/system/app-*.slice`（八个） | 八个 cgroup 分片单元本身，只有 `[Unit]` 一节，不含任何配额取值 |
| `podman/*.container`（九个） | Podman Quadlet 容器单元，PostgreSQL 16 加八个进程 |
| `podman/*.volume`（五个） | Quadlet 卷单元，`VolumeName=` 固定卷名 |
| `compose/compose.yaml` | Docker Compose 一侧，同样九个服务五个卷 |
| `../scripts/verify-orchestration-equivalence.py` | 两套等价性与 slice 引用的核对脚本 |
| `../scripts/verify-orchestration-equivalence-negative.sh` | 每条规则一个故意违反的负样例 |
| `../scripts/dev-up.sh`、`../scripts/dev-down.sh` | 交付物 D-13，本地开发环境的起停 |

## 二、九个服务与八个分片的对应

八个进程取自技术基线第 2 节的进程表，分片取自该表的 cgroup slice 一列，一个字没改。
第九个服务是 PostgreSQL 16，落 `app-db.slice`。

| 服务 | 分片 | 镜像 | 排在其后 |
|---|---|---|---|
| postgres | `app-db.slice` | `docker.io/library/postgres:16` | — |
| core-server | `app-core.slice` | `localhost/ep/core-server:0.1.0` | postgres、plugin-host |
| integration-gateway | `app-core.slice` | `localhost/ep/integration-gateway:0.1.0` | postgres |
| job-worker | `app-worker.slice` | `localhost/ep/job-worker:0.1.0` | postgres |
| plugin-host | `app-plugin.slice` | `localhost/ep/plugin-host:0.1.0` | — |
| ops-agent | `app-edge.slice` | `localhost/ep/ops-agent:0.1.0` | postgres |
| portal-gateway | `app-portal.slice` | `localhost/ep/portal-gateway:0.1.0` | core-server |
| archive-writer | `app-archive.slice` | `localhost/ep/archive-writer:0.1.0` | postgres、core-server |
| backup-writer | `app-backup.slice` | `localhost/ep/backup-writer:0.1.0` | postgres、core-server |

core-server 排在 plugin-host 之后，是因为 `/run/ep/ipc/plugin.sock` 由 plugin-host 建、core-server 连接它；
archive-writer 与 backup-writer 排在 core-server 之后，是因为它们经 `/run/ep/ipc/core.sock` 上报写出结果。
两个写出进程分属两个分片、不共享磁盘 IO 预算，规格第 13.1 章明令不得合并为一个进程。

### 主机网络

九个服务一律 `Network=host` 与 `network_mode: host`，不建容器网络，也不发布任何端口。
理由是技术基线第 2 节的监听地址一律是 `127.0.0.1`，且 `portal.upstream_base_url` 默认
`http://127.0.0.1:8080`、`db.host` 默认 `127.0.0.1`；主机网络下这批默认值原样成立，
不必在编排里再取一套容器名寻址的取值。规格第 13.1 章已声明门户与核心之间只有进程与系统账户边界，
不是网段边界，主机网络不改变该结论，也不构成新的暴露面：这些监听本来就只绑 `127.0.0.1`。

### 只读根文件系统与能力

八个进程容器一律 `ReadOnly=true`、`NoNewPrivileges=true`、`DropCapability=ALL`，
可写处只有各自挂进来的卷。PostgreSQL 官方镜像的入口点要以 root 起再 setuid 到 postgres，
丢弃全部能力会让它起不来，因此 postgres 一个服务不写 `cap_drop`，只保留 `no-new-privileges`。
这一条差别是逐服务写在两套文件里的事实，等价性核对同样逐项比。

## 三、一条命令

生产侧，Podman 加 systemd：

```
install -m 0644 deploy/systemd/system/app-*.slice /etc/systemd/system/
install -m 0644 deploy/podman/*.container deploy/podman/*.volume /etc/containers/systemd/
systemctl daemon-reload
systemctl start core-server portal-gateway integration-gateway job-worker \
                ops-agent archive-writer backup-writer
```

Quadlet 由 `[Install] WantedBy=multi-user.target` 生成 `.service`，
postgres 与 plugin-host 由上列服务的 `Requires=` 带起，不必单独列。

Docker Compose 侧：

```
docker compose -f deploy/compose/compose.yaml up -d
```

本地开发环境（D-13）：

```
scripts/dev-up.sh            # PostgreSQL 16 与八个进程
scripts/dev-up.sh --db-only  # 只要一个库，集成测试用这个
scripts/dev-down.sh          # 停，命名卷保留；要清库加 --purge
```

`dev-up.sh` 起的就是同一个 `compose.yaml`，不另写一份开发用编排。
另写一份就有了第三套取值，而它与生产两套之间没有任何东西核对，开发机上跑通的与部署出去的会悄悄分叉。
开发机与生产机的差别全部由四个环境变量表达，默认值与 Quadlet 一侧的字面量逐字相同：

| 变量 | 默认值 | 开发机取值 |
|---|---|---|
| `EP_ETC_DIR` | `/etc/ep` | 状态目录下的 `etc/` |
| `EP_SECRETS_DIR` | `/var/lib/ep/secrets` | 状态目录下的 `secrets/` |
| `EP_IMAGE_PREFIX` | `localhost/ep` | 同默认 |
| `EP_IMAGE_TAG` | `0.1.0` | 同默认 |

状态目录默认 `${XDG_STATE_HOME:-~/.local/state}/ep-dev`，在仓库之外，不产生未跟踪文件。
数据库超级用户口令由 `dev-up.sh` 首次运行时随机生成并以 0600 写入该目录，不进仓库也不进任何制品。
库与角色由 `db/bootstrap/` 下的引导脚本建，该目录按裁定 C-01 由阶段 2 交付，`dev-up.sh` 不代建。

## 四、等价性怎么判

```
scripts/verify-orchestration-equivalence.py [--equivalence | --slices | --all]
```

两套文件的语法毫无共同之处，直接比文本比不了。脚本把两侧各自压成同一张三元组事实表
`(服务名, 事实名, 事实值)` 后比集合，共十三条规则：

| 规则 | 判什么 |
|---|---|
| `EQ-PARSE` | 两侧都读得到、都解析得了；解析不了即未覆盖，不判通过 |
| `EQ-QUADLET-KEY-KNOWN` | Quadlet 的键名在允许集内。systemd 遇到不认识的键只写日志照常启动，拼错的键只能在这里挡 |
| `EQ-COMPOSE-KEY-KNOWN` | Compose 的键名在允许集内，同理 |
| `EQ-SERVICE-SET` | 两套的服务集合相等 |
| `EQ-REQUIRED-FACT` | 每个服务两侧都给全六项必备事实。两侧同时漏写同一项时集合仍相等，只比集合会把它判成等价 |
| `EQ-FACT` | 两侧事实表逐项相等 |
| `EQ-VOLUME-DECL` | 命名卷声明与引用一致，两侧都不许有引用未声明或声明未引用的卷 |
| `EQ-QUADLET-ORDER-PAIRED` | Quadlet 每个单元的 `After=` 与 `Requires=` 是同一集合 |
| `SL-QUOTA-IN-ORCH` | 编排文件里不出现任何配额取值键 |
| `SL-QUOTA-IN-UNIT` | slice 单元里不出现任何配额取值键 |
| `SL-UNIT-MISSING` | 引用的每个 slice 都有同名单元文件 |
| `SL-DROPIN-MISSING` | 引用的每个 slice 都有 `<slice>.d/10-resource-limits.conf` |
| `SL-DROPIN-ORPHAN` | 没有无人引用的 drop-in，且 drop-in 恰为八个 |

退出码分四种，与 `scripts/verify-resource-limits.sh` 同一套，不合并：

| 退出码 | 含义 |
|---|---|
| 0 | 通过 |
| 2 | 不符 |
| 3 | 被测对象读不到，或含解析器不认识的构造，判定未做出 |
| 64 | 用法错误 |

**读不到即退出码 3 而不是 0。** 本机没有 PyYAML 也没有 `docker compose config` 可调，
脚本自带一个只认本项目所用构造的 YAML 子集解析器；碰上流式写法、锚点、多行标量、制表符或行内注释
一律报未覆盖，不猜。猜出来的解析结果没有判定力。

负样例：

```
scripts/verify-orchestration-equivalence-negative.sh
```

十三条规则各一个样本，一律在临时目录的 `deploy/` 副本上改，经 `EP_DEPLOY_ROOT` 指过去，
仓库里的文件一个字不动。断言的是规则本身——不只看退出码，还要求输出里出现该规则的名字，
否则一个「凡事都判不符」的坏脚本也能把这些样例全过掉。每次改动前先断言基准文本存在，
基准文本被人改名后负样例必须当场红，而不是悄悄退化成一个什么都没改的空样本。

## 五、两套之间消不掉的差别，如实写在这里

**差别一：就绪等待。** Compose 的 `depends_on` 可以写 `condition: service_healthy`，
起 core-server 之前会等 postgres 的健康检查过。Quadlet 只有 `After=` 与 `Requires=`，
那是启动顺序不是就绪门槛，systemd 认为 postgres 的 `.service` 起来了就往下走。
两套在这一点上不等价，事实表把两者都归一成 `after postgres`，因此这条差别核对脚本判不出来。
后果是 Podman 一侧的进程可能在库尚未接受连接时启动，由该进程自身的启动自检与重启承接。
交付材料不得表述为两套就绪语义相同。

**差别二：`${VAR:-默认值}` 只在 Compose 一侧。** Quadlet 是 systemd 单元，不做这类展开，
所以那一侧一律是字面量。核对脚本把 Compose 的默认值取出来再比，两套因此只在默认值上等价；
设了这四个变量之后跑的 Compose 与 Quadlet 不再等价，这正是本地开发环境的用法，不是缺陷。

**差别三：卷名。** Compose 默认给命名卷加项目名前缀，Quadlet 默认加 `systemd-` 前缀。
两侧各写一次固定卷名把前缀关掉，`VolumeName=` 需要 Podman 5.0 及以上；低于该版本的 Podman
会落到另一个卷上，本编排不支持。

## 六、文档未给出、由本次交付暂取的值

以下各处在规格、技术基线、阶段计划与裁定登记中均无取值，落文件时不得不选定。
它们都不是判据，等价性核对只要求两套一致，不与任何上游文件比对，须在认证冻结前复核。

1. **镜像仓库前缀与标签。** 全部上游文件只说「以标准 OCI 容器交付」，没有一处给出镜像命名。
   本次取 `localhost/ep/<进程名>:0.1.0`，标签取工作区 `Cargo.toml` 的 `workspace.package.version`。
   镜像的构建与签名属交付物 D-11，本次不交付，编排只引用。
2. **反向代理没有承载物。** 规格第 13.1 章的配额表第 6 行是「反向代理与运维代理」，
   `app-edge.slice` 因此本该承载两个组件；本仓库没有反向代理的镜像来源，也没有站点、证书与
   访问策略的任何配置来源，凭空写一个 nginx 容器就是发明一套上游没有的取值。
   本次该分片只承载 ops-agent，反向代理属未落实项，不是已覆盖项。终结 TLS、按来源分发、
   门户与员工两类入口的独立站点与独立证书，本编排一样都没有。
3. **停机余量 45 秒。** 配置键 `http.shutdown_drain_ms` 默认 30000，交付物 D-02 要求
   SIGTERM 后 30 秒内退出码 0。编排若也在第 30 秒下杀，两者相撞，因此取 45 秒作余量。
   这 45 不是第二个排空时长，改排空时长时它要跟着改。
4. **服务器之外的落点不由本编排指派。** archive-writer 与 backup-writer 要把归档、附件正文与
   审计证据写到该服务器之外，规格第 13.4 章把落点定为客户提供。本编排只给两个进程各挂一个本机
   暂存卷（配置键 `spool.dir`），落点挂载由部署时按客户环境追加，不在这两套文件里预设一个路径。
5. **容器内运行用户没有写。** 技术基线第 2 节给出八个系统账户名，没有给 uid/gid 分配。
   容器里的 `User=` 要 uid 才稳当，本次不发明一套编号，两套都不写该键，运行用户由镜像自身决定。
   这意味着「以独立系统账户运行」这条在编排层没有承载，只能由 D-11 的镜像承接。
6. **健康检查只有 postgres 一个。** `pg_isready` 在官方镜像里现成。八个进程的健康端点在
   交付物 D-02，探测命令要镜像里有一个能发 HTTP 的东西，那属镜像内容；本次不为此在编排里
   猜一个 curl 路径，八个服务一律不写健康检查。

## 七、本次没有做的事

- **两套编排一次都没有起来过。** 本机（macOS）没有 Docker，也没有 Podman，更没有 systemd 与 cgroup。
  D-05 的判定方式是「一条命令起全栈，`systemctl status` 全部 active」，这条没有验过。
- **八个进程镜像不存在**，属交付物 D-11。`dev-up.sh --full` 在今天的仓库上一定以退出码 72 结束，
  这是它该有的行为：镜像缺失不算起来了。
- **`compose.yaml` 没有经过 `docker compose config` 校验**，只经过本仓库自带的 YAML 子集解析器
  与键名允许集。Quadlet 单元没有经过 `/usr/libexec/podman/quadlet --dryrun` 校验，同理。
- **两个脚本没有接进 CI。** 流水线定义在 `.github/` 下，不在本次交付的路径范围内。

#!/usr/bin/env python3
# F-57：HISTORICAL_LINUX_RESEARCH_ONLY；输出不得作为 Windows Server 2022 生产证据。
"""把两套编排文件各自压成同一张「编排事实表」，供等价性核对逐项比对。

两套文件的语法毫无共同之处：一套是 systemd 单元，一套是 YAML。直接比文本比不了，
比「解析成什么」才比得了，所以这里各写一个解析器，输出同一种三元组
    (服务名, 事实名, 事实值)
再由核对脚本比对两个集合。任何本模块看不懂的构造一律抛 Unsupported 而不是猜，
猜出来的等价没有判定力；核对脚本据此报「未覆盖」并以退出码 3 结束，不判通过。
"""

from __future__ import annotations

import re
from pathlib import Path

# 本项目允许出现在编排文件里的键，逐个列举。允许集之外的键一律判不符：
# systemd 遇到不认识的键只写一条日志照常启动，Compose 遇到不认识的键在部分实现下同样只告警，
# 拼错的键因此不会在部署时暴露，只能在这里挡住。
QUADLET_KEYS = {
    "Unit": {"Description", "Requires", "After"},
    "Container": {
        "ContainerName", "Image", "Network", "Volume", "Environment",
        "ReadOnly", "NoNewPrivileges", "DropCapability",
        "HealthCmd", "HealthInterval", "HealthTimeout", "HealthRetries",
    },
    "Service": {"Slice", "Restart", "TimeoutStopSec"},
    "Volume": {"VolumeName"},
    "Install": {"WantedBy"},
}

COMPOSE_SERVICE_KEYS = {
    "container_name", "image", "network_mode", "cgroup_parent", "restart",
    "stop_grace_period", "read_only", "security_opt", "cap_drop",
    "environment", "volumes", "depends_on", "healthcheck",
}

# 配额取值只许落在 <slice>.d/10-resource-limits.conf 一处。下列键出现在编排文件或 slice 单元里
# 就是第二套取值，改一处漏一处时无从判定哪一处为准。
QUOTA_KEYS = {
    "CPUWeight", "CPUQuota", "CPUShares", "AllowedCPUs", "AllowedMemoryNodes",
    "IOWeight", "IOReadBandwidthMax", "IOWriteBandwidthMax",
    "IOReadIOPSMax", "IOWriteIOPSMax",
    "MemoryLow", "MemoryMin", "MemoryHigh", "MemoryMax", "MemorySwapMax", "MemoryLimit",
    "TasksMax",
    "cpus", "cpu_shares", "cpu_quota", "cpu_period", "cpuset",
    "mem_limit", "mem_reservation", "memswap_limit", "pids_limit",
    "blkio_config", "deploy",
}

# 每个服务两套都必须给全的事实。两套同时漏写一项时集合仍然相等，
# 只比集合相等会把「都没写 slice」判成等价，因此另立本表。
REQUIRED_FACTS = ("container-name", "image", "slice", "network", "restart", "stop-grace-sec")


class Unsupported(Exception):
    """被测文件里有本模块不认识的构造。此时判定未做出，不得按通过处理。"""


# ---------------------------------------------------------------- 通用取值归一

_INTERPOLATION = re.compile(r"\$\{([A-Za-z_][A-Za-z0-9_]*)(:-([^}]*))?\}")


def resolve_defaults(text: str) -> str:
    """把 Compose 的 ${VAR:-默认值} 取其默认值。

    Quadlet 是 systemd 单元，不做这类展开，字面量就是取值；Compose 一侧要留出本地开发环境的
    可变路径，两侧因此只能在默认值上等价。没有默认值的 ${VAR} 无从取值，直接抛。
    """
    def one(m: re.Match) -> str:
        if m.group(2) is None:
            raise Unsupported(f"{m.group(0)} 没有默认值，两套无从在同一取值上比对")
        return m.group(3)
    return _INTERPOLATION.sub(one, text)


def duration_seconds(raw: str) -> str:
    """systemd 的 TimeoutStopSec 与 Compose 的 stop_grace_period 写法不同，一律折成秒。"""
    raw = raw.strip()
    m = re.fullmatch(r"(\d+)(s|sec|seconds)?", raw)
    if not m:
        raise Unsupported(f"时长 {raw!r} 不是本模块认得的整秒写法")
    return m.group(1)


def _volume_name(spec: str) -> str:
    """Quadlet 引用卷单元写 ep-ipc.volume:/path，Compose 写 ep-ipc:/path，归一到后者。"""
    src, _, rest = spec.partition(":")
    if src.endswith(".volume"):
        src = src[: -len(".volume")]
    return f"{src}:{rest}"


# ---------------------------------------------------------------- systemd 单元

def parse_unit(path: Path) -> dict[str, list[tuple[str, str]]]:
    """读一个 systemd 单元。同名键可重复（Volume= 就是），所以每节存的是键值对列表。"""
    sections: dict[str, list[tuple[str, str]]] = {}
    current = None
    for lineno, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        line = raw.strip()
        if not line or line.startswith("#") or line.startswith(";"):
            continue
        if line.startswith("[") and line.endswith("]"):
            current = line[1:-1]
            sections.setdefault(current, [])
            continue
        if current is None:
            raise Unsupported(f"{path}:{lineno} 键出现在任何节之前")
        if "=" not in line:
            raise Unsupported(f"{path}:{lineno} 既不是节头也不是键值对：{line!r}")
        key, _, value = line.partition("=")
        sections[current].append((key.strip(), value.strip()))
    return sections


def _one(pairs: list[tuple[str, str]], key: str) -> str | None:
    hits = [v for k, v in pairs if k == key]
    if len(hits) > 1:
        raise Unsupported(f"键 {key} 重复 {len(hits)} 次，取值不唯一")
    return hits[0] if hits else None


def _many(pairs: list[tuple[str, str]], key: str) -> list[str]:
    return [v for k, v in pairs if k == key]


def quadlet_facts(unit_dir: Path) -> tuple[set[tuple[str, str, str]], dict[str, set[str]]]:
    """从 deploy/podman/ 下的 .container 与 .volume 单元抽事实表与卷声明。"""
    facts: set[tuple[str, str, str]] = set()
    declared_volumes: set[str] = set()

    for path in sorted(unit_dir.glob("*.volume")):
        sections = parse_unit(path)
        name = _one(sections.get("Volume", []), "VolumeName")
        if name is None:
            raise Unsupported(f"{path} 缺 VolumeName=，卷名会被加上 systemd- 前缀，两套对不上")
        declared_volumes.add(name)

    for path in sorted(unit_dir.glob("*.container")):
        svc = path.stem
        sections = parse_unit(path)
        unit = sections.get("Unit", [])
        cont = sections.get("Container", [])
        serv = sections.get("Service", [])

        def add(fact: str, value: str | None) -> None:
            if value is not None:
                facts.add((svc, fact, value))

        add("container-name", _one(cont, "ContainerName"))
        add("image", _one(cont, "Image"))
        add("network", _one(cont, "Network"))
        add("slice", _one(serv, "Slice"))
        add("restart", _one(serv, "Restart"))
        timeout = _one(serv, "TimeoutStopSec")
        if timeout is not None:
            add("stop-grace-sec", duration_seconds(timeout))
        if _one(cont, "ReadOnly") == "true":
            add("read-only", "true")
        if _one(cont, "NoNewPrivileges") == "true":
            add("security-opt", "no-new-privileges:true")
        for value in _many(cont, "DropCapability"):
            for cap in value.split():
                add("cap-drop", cap)
        for value in _many(cont, "Environment"):
            for item in value.split():
                add("env", item)
        for value in _many(cont, "Volume"):
            add("volume", _volume_name(value))
        for value in _many(unit, "After"):
            for dep in value.split():
                add("after", dep[: -len(".service")] if dep.endswith(".service") else dep)
        add("health-cmd", _one(cont, "HealthCmd"))
        for fact, key in (("health-interval-sec", "HealthInterval"),
                          ("health-timeout-sec", "HealthTimeout")):
            value = _one(cont, key)
            if value is not None:
                add(fact, duration_seconds(value))
        add("health-retries", _one(cont, "HealthRetries"))

    return facts, {"declared": declared_volumes}


def quadlet_scan(unit_dir: Path) -> dict:
    """与 Compose 一侧同形的四项：事实表、卷声明、越界键、配额键。"""
    facts, vol = quadlet_facts(unit_dir)
    unknown: list[str] = []
    quota: list[str] = []
    for path in sorted(list(unit_dir.glob("*.container")) + list(unit_dir.glob("*.volume"))):
        for section, pairs in parse_unit(path).items():
            allowed = QUADLET_KEYS.get(section)
            if allowed is None:
                unknown.append(f"{path.name} 的 [{section}] 节")
                continue
            for key, _ in pairs:
                if key in QUOTA_KEYS:
                    quota.append(f"{path.name} [{section}] {key}")
                elif key not in allowed:
                    unknown.append(f"{path.name} [{section}] {key}")
    return {"facts": facts, "volumes": vol["declared"], "unknown": unknown, "quota": quota}


def slice_scan(slice_dir: Path) -> dict:
    """读 .slice 单元本身。配额取值不许出现在这里，只许出现在同名 .d 目录的 drop-in 里。"""
    units: dict[str, list[str]] = {}
    for path in sorted(slice_dir.glob("*.slice")):
        quota = []
        for section, pairs in parse_unit(path).items():
            for key, _ in pairs:
                if key in QUOTA_KEYS:
                    quota.append(f"{path.name} [{section}] {key}")
        units[path.name] = quota
    return units


def quadlet_order_pairs(unit_dir: Path) -> list[tuple[str, set[str], set[str]]]:
    """取每个容器单元的 After 与 Requires 两个集合，供成对性判定。"""
    out = []
    for path in sorted(unit_dir.glob("*.container")):
        unit = parse_unit(path).get("Unit", [])
        after = {d for v in _many(unit, "After") for d in v.split()}
        requires = {d for v in _many(unit, "Requires") for d in v.split()}
        out.append((path.stem, after, requires))
    return out

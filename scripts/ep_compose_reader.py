#!/usr/bin/env python3
# F-57：HISTORICAL_LINUX_RESEARCH_ONLY；输出不得作为 Windows Server 2022 生产证据。
"""读 Compose 文件并抽出与 Quadlet 同形的编排事实表。

本机没有 PyYAML，也没有 docker compose config 可调，所以这里自带一个只认本项目所用构造的
YAML 子集解析器：块映射、块序列、标量，两空格缩进，整行注释。除此之外的一切
（流式写法、锚点、别名、多行标量、制表符、行内注释）一律抛 Unsupported，
由调用方报「未覆盖」并以退出码 3 结束——猜出来的解析结果没有判定力。
"""

from __future__ import annotations

from pathlib import Path

from ep_orchestration_facts import (
    COMPOSE_SERVICE_KEYS,
    QUOTA_KEYS,
    Unsupported,
    duration_seconds,
    resolve_defaults,
)

COMPOSE_TOP_KEYS = {"name", "services", "volumes"}
COMPOSE_VOLUME_KEYS = {"name"}

_FLOW_START = "{[&*!|>%@`"


def _tokens(path: Path) -> list[tuple[int, str, int]]:
    out: list[tuple[int, str, int]] = []
    for lineno, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if "\t" in raw:
            raise Unsupported(f"{path}:{lineno} 含制表符，YAML 缩进不得用制表符")
        stripped = raw.strip()
        if not stripped or stripped.startswith("#"):
            continue
        if stripped.startswith("---") or stripped.startswith("..."):
            raise Unsupported(f"{path}:{lineno} 多文档标记不在本解析器的构造集内")
        if " #" in raw:
            raise Unsupported(f"{path}:{lineno} 行内注释不在本解析器的构造集内，注释请整行写")
        indent = len(raw) - len(raw.lstrip(" "))
        if indent % 2:
            raise Unsupported(f"{path}:{lineno} 缩进 {indent} 不是 2 的倍数")
        out.append((indent, stripped, lineno))
    return out


def _scalar(text: str, lineno: int) -> str:
    if len(text) >= 2 and text.startswith('"') and text.endswith('"'):
        body = text[1:-1]
        if "\\" in body.replace('\\"', "").replace("\\\\", ""):
            raise Unsupported(f"第 {lineno} 行的转义不在本解析器的构造集内")
        return resolve_defaults(body.replace('\\"', '"').replace("\\\\", "\\"))
    if text and text[0] in _FLOW_START:
        raise Unsupported(f"第 {lineno} 行的 {text[0]!r} 起首写法不在本解析器的构造集内")
    if ": " in text:
        raise Unsupported(f"第 {lineno} 行的裸标量含 ': '，无法与映射区分")
    return resolve_defaults(text)


def _parse_block(toks, i: int, indent: int):
    if toks[i][1].startswith("- "):
        return _parse_sequence(toks, i, indent)
    return _parse_mapping(toks, i, indent)


def _parse_sequence(toks, i: int, indent: int):
    items: list[str] = []
    while i < len(toks) and toks[i][0] == indent and toks[i][1].startswith("- "):
        items.append(_scalar(toks[i][1][2:].strip(), toks[i][2]))
        i += 1
    return items, i


def _parse_mapping(toks, i: int, indent: int):
    result: dict[str, object] = {}
    while i < len(toks) and toks[i][0] == indent:
        _, text, lineno = toks[i]
        if text.startswith("- "):
            raise Unsupported(f"第 {lineno} 行在映射位置出现序列项")
        key, sep, rest = text.partition(":")
        if not sep:
            raise Unsupported(f"第 {lineno} 行既不是键值对也不是序列项：{text!r}")
        key = key.strip()
        if key in result:
            raise Unsupported(f"第 {lineno} 行的键 {key} 重复")
        rest = rest.strip()
        if rest:
            result[key] = _scalar(rest, lineno)
            i += 1
            continue
        if i + 1 >= len(toks) or toks[i + 1][0] <= indent:
            raise Unsupported(f"第 {lineno} 行的键 {key} 没有子块，空值不在本解析器的构造集内")
        if toks[i + 1][0] != indent + 2:
            raise Unsupported(f"第 {toks[i + 1][2]} 行缩进跳级，本解析器只认逐级两空格")
        result[key], i = _parse_block(toks, i + 1, indent + 2)
    return result, i


def parse(path: Path) -> dict:
    toks = _tokens(path)
    if not toks:
        raise Unsupported(f"{path} 没有任何可解析的行")
    if toks[0][0] != 0:
        raise Unsupported(f"{path} 首行有缩进")
    doc, i = _parse_mapping(toks, 0, 0)
    if i != len(toks):
        raise Unsupported(f"{path} 第 {toks[i][2]} 行之后的内容没有归属")
    return doc


def _require_mapping(value: object, what: str) -> dict:
    if not isinstance(value, dict):
        raise Unsupported(f"{what} 不是映射")
    return value


def scan(path: Path) -> dict:
    """返回事实表、卷声明、越界键与配额键四项，供核对脚本按规则分别报告。"""
    doc = parse(path)
    unknown: list[str] = []
    quota: list[str] = []
    facts: set[tuple[str, str, str]] = set()

    for key in doc:
        if key not in COMPOSE_TOP_KEYS:
            unknown.append(f"顶层键 {key}")

    volumes = _require_mapping(doc.get("volumes", {}), "顶层 volumes")
    declared = set()
    for name, body in volumes.items():
        body = _require_mapping(body, f"volumes.{name}")
        for key in body:
            if key not in COMPOSE_VOLUME_KEYS:
                unknown.append(f"volumes.{name}.{key}")
        declared.add(body.get("name", name))

    services = _require_mapping(doc.get("services", {}), "顶层 services")
    for svc, body in services.items():
        body = _require_mapping(body, f"services.{svc}")
        for key in body:
            if key in QUOTA_KEYS:
                quota.append(f"services.{svc}.{key}")
            elif key not in COMPOSE_SERVICE_KEYS:
                unknown.append(f"services.{svc}.{key}")
        facts |= _service_facts(svc, body)

    return {"facts": facts, "volumes": declared, "unknown": unknown, "quota": quota}


def _service_facts(svc: str, body: dict) -> set[tuple[str, str, str]]:
    facts: set[tuple[str, str, str]] = set()

    def add(fact: str, value: object) -> None:
        if value is not None:
            facts.add((svc, fact, str(value)))

    add("container-name", body.get("container_name"))
    add("image", body.get("image"))
    add("network", body.get("network_mode"))
    add("slice", body.get("cgroup_parent"))
    add("restart", body.get("restart"))
    if "stop_grace_period" in body:
        add("stop-grace-sec", duration_seconds(str(body["stop_grace_period"])))
    if body.get("read_only") == "true":
        add("read-only", "true")
    for item in body.get("security_opt", []):
        add("security-opt", item)
    for item in body.get("cap_drop", []):
        add("cap-drop", item)
    for key, value in _require_mapping(body.get("environment", {}), "environment").items():
        add("env", f"{key}={value}")
    for item in body.get("volumes", []):
        add("volume", item)
    for dep in _require_mapping(body.get("depends_on", {}), "depends_on"):
        add("after", dep)
    if "healthcheck" in body:
        facts |= _health_facts(svc, _require_mapping(body["healthcheck"], "healthcheck"))
    return facts


def _health_facts(svc: str, hc: dict) -> set[tuple[str, str, str]]:
    test = hc.get("test")
    if not isinstance(test, list) or not test:
        raise Unsupported(f"services.{svc}.healthcheck.test 不是非空序列")
    if test[0] != "CMD":
        raise Unsupported(f"services.{svc}.healthcheck.test 首项不是 CMD，其余写法未归一")
    facts = {(svc, "health-cmd", " ".join(test[1:]))}
    if "interval" in hc:
        facts.add((svc, "health-interval-sec", duration_seconds(str(hc["interval"]))))
    if "timeout" in hc:
        facts.add((svc, "health-timeout-sec", duration_seconds(str(hc["timeout"]))))
    if "retries" in hc:
        facts.add((svc, "health-retries", str(hc["retries"])))
    return facts

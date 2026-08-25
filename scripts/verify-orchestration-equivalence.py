#!/usr/bin/env python3
# F-57：HISTORICAL_LINUX_RESEARCH_ONLY；输出不得作为 Windows Server 2022 生产编排证据。
"""核对两套单机编排是否等价，以及它们引用的 slice 与资源限额 drop-in 是否对得上。

规格第 13.2 章把编排取值定为「Docker Compose 或 Podman 加 systemd」，两者择一部署。
择一的前提是两套等价，而等价靠人眼比对守不住：一套是 YAML，一套是 systemd 单元，
改一处漏一处时两台机器上跑的就是两套东西。本脚本把两套各自压成同一张事实表后逐项比。

退出码分四种，不合并：

    0   通过
    2   不符
    3   被测对象读不到，或含本工具无法解析的构造，判定未做出
    64  用法错误

读不到即退出码 3 而不是 0：未覆盖不是通过。同一次运行里既有不符又有未覆盖时取 2。
"""

from __future__ import annotations

import os
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import ep_compose_reader  # noqa: E402
import ep_orchestration_facts as facts_mod  # noqa: E402
from ep_orchestration_facts import REQUIRED_FACTS, Unsupported  # noqa: E402

EXIT_OK = 0
EXIT_MISMATCH = 2
EXIT_UNCOVERED = 3
EXIT_USAGE = 64

REPO_ROOT = Path(__file__).resolve().parent.parent
# 替代根只为负样例存在：不设时取仓库的真实路径。
DEPLOY_ROOT = Path(os.environ.get("EP_DEPLOY_ROOT", REPO_ROOT / "deploy"))

QUADLET_DIR = DEPLOY_ROOT / "podman"
COMPOSE_FILE = DEPLOY_ROOT / "compose" / "compose.yaml"
SYSTEMD_DIR = DEPLOY_ROOT / "systemd" / "system"
DROPIN_NAME = "10-resource-limits.conf"
EXPECTED_SLICE_COUNT = 8


class Report:
    def __init__(self) -> None:
        self.mismatch = 0
        self.uncovered = 0

    def bad(self, rule: str, detail: str) -> None:
        print(f"不符    [{rule}] {detail}", file=sys.stderr)
        self.mismatch += 1

    def gap(self, rule: str, detail: str) -> None:
        print(f"未覆盖  [{rule}] {detail}", file=sys.stderr)
        self.uncovered += 1

    def ok(self, rule: str, detail: str) -> None:
        print(f"通过    [{rule}] {detail}")


def load(report: Report):
    """读两套编排。任何一侧读不到或解析不了都返回 None，由调用方按未覆盖处理。"""
    if not QUADLET_DIR.is_dir():
        report.gap("EQ-PARSE", f"Quadlet 目录读不到：{QUADLET_DIR}")
        return None
    if not COMPOSE_FILE.is_file():
        report.gap("EQ-PARSE", f"Compose 文件读不到：{COMPOSE_FILE}")
        return None
    if not list(QUADLET_DIR.glob("*.container")):
        report.gap("EQ-PARSE", f"{QUADLET_DIR} 下没有任何 .container 单元，无可比对物")
        return None
    try:
        quadlet = facts_mod.quadlet_scan(QUADLET_DIR)
        compose = ep_compose_reader.scan(COMPOSE_FILE)
    except Unsupported as exc:
        report.gap("EQ-PARSE", f"{exc}")
        return None
    except OSError as exc:
        report.gap("EQ-PARSE", f"读文件失败：{exc}")
        return None
    return quadlet, compose


def check_keys(report: Report, quadlet: dict, compose: dict) -> None:
    for item in quadlet["unknown"]:
        report.bad("EQ-QUADLET-KEY-KNOWN", f"Quadlet 出现允许集之外的键：{item}")
    for item in compose["unknown"]:
        report.bad("EQ-COMPOSE-KEY-KNOWN", f"Compose 出现允许集之外的键：{item}")
    if not quadlet["unknown"]:
        report.ok("EQ-QUADLET-KEY-KNOWN", "Quadlet 键名全部在允许集内")
    if not compose["unknown"]:
        report.ok("EQ-COMPOSE-KEY-KNOWN", "Compose 键名全部在允许集内")


def check_service_set(report: Report, quadlet: dict, compose: dict) -> set[str]:
    q_svc = {svc for svc, _, _ in quadlet["facts"]}
    c_svc = {svc for svc, _, _ in compose["facts"]}
    for svc in sorted(q_svc - c_svc):
        report.bad("EQ-SERVICE-SET", f"{svc} 只有 Quadlet 一侧有")
    for svc in sorted(c_svc - q_svc):
        report.bad("EQ-SERVICE-SET", f"{svc} 只有 Compose 一侧有")
    if q_svc == c_svc:
        report.ok("EQ-SERVICE-SET", f"两套服务集合相等，共 {len(q_svc)} 个")
    return q_svc | c_svc


def check_required(report: Report, services: set[str], quadlet: dict, compose: dict) -> None:
    """两套同时漏写同一项时集合仍相等，因此另判必备项，不能只判集合相等。"""
    missing = False
    for side, data in (("Quadlet", quadlet), ("Compose", compose)):
        present = {(svc, fact) for svc, fact, _ in data["facts"]}
        for svc in sorted(services):
            for fact in REQUIRED_FACTS:
                if (svc, fact) not in present:
                    report.bad("EQ-REQUIRED-FACT", f"{side} 的 {svc} 缺必备事实 {fact}")
                    missing = True
    if not missing:
        report.ok("EQ-REQUIRED-FACT", f"两套各 {len(services)} 个服务的必备事实齐备")


def check_facts(report: Report, quadlet: dict, compose: dict) -> None:
    only_q = sorted(quadlet["facts"] - compose["facts"])
    only_c = sorted(compose["facts"] - quadlet["facts"])
    for svc, fact, value in only_q:
        report.bad("EQ-FACT", f"{svc} 的 {fact}={value} 只有 Quadlet 一侧有")
    for svc, fact, value in only_c:
        report.bad("EQ-FACT", f"{svc} 的 {fact}={value} 只有 Compose 一侧有")
    if not only_q and not only_c:
        report.ok("EQ-FACT", f"两套事实表逐项相等，共 {len(quadlet['facts'])} 条")


def check_volumes(report: Report, quadlet: dict, compose: dict) -> None:
    used = {value.split(":", 1)[0]
            for _, fact, value in quadlet["facts"] | compose["facts"]
            if fact == "volume" and not value.startswith("/")}
    bad = False
    for side, declared in (("Quadlet", quadlet["volumes"]), ("Compose", compose["volumes"])):
        for name in sorted(used - declared):
            report.bad("EQ-VOLUME-DECL", f"{side} 引用了卷 {name} 却没有声明它")
            bad = True
        for name in sorted(declared - used):
            report.bad("EQ-VOLUME-DECL", f"{side} 声明了卷 {name} 却没有任何服务用它")
            bad = True
    if not bad:
        report.ok("EQ-VOLUME-DECL", f"两套的命名卷声明与引用一致，共 {len(used)} 个")


def check_order_paired(report: Report) -> None:
    """Compose 的 depends_on 同时表达「排在其后」与「把它拉起来」，
    Quadlet 要 After= 与 Requires= 两个键才等价，只写一个不等价。"""
    bad = False
    for name, after, requires in facts_mod.quadlet_order_pairs(QUADLET_DIR):
        if after != requires:
            report.bad("EQ-QUADLET-ORDER-PAIRED",
                       f"{name} 的 After={sorted(after)} 与 Requires={sorted(requires)} 不是同一集合")
            bad = True
    if not bad:
        report.ok("EQ-QUADLET-ORDER-PAIRED", "Quadlet 的 After 与 Requires 逐单元成对")


def check_quota_absent(report: Report, quadlet: dict, compose: dict, slices: dict) -> None:
    hits = list(quadlet["quota"]) + list(compose["quota"])
    for item in hits:
        report.bad("SL-QUOTA-IN-ORCH", f"编排文件里出现配额取值键：{item}")
    unit_hits = [item for quota in slices.values() for item in quota]
    for item in unit_hits:
        report.bad("SL-QUOTA-IN-UNIT", f"slice 单元里出现配额取值键：{item}")
    if not hits:
        report.ok("SL-QUOTA-IN-ORCH", "两套编排文件里没有任何配额取值键")
    if not unit_hits:
        report.ok("SL-QUOTA-IN-UNIT", "八个 slice 单元里没有任何配额取值键")


def check_slices(report: Report, referenced: set[str], slices: dict) -> None:
    dropin_dirs = {p.name[: -len(".d")] for p in SYSTEMD_DIR.glob("*.slice.d")
                   if (p / DROPIN_NAME).is_file()}
    for name in sorted(referenced):
        if name not in slices:
            report.bad("SL-UNIT-MISSING", f"编排引用了 {name}，{SYSTEMD_DIR} 下没有该单元文件")
        if name not in dropin_dirs:
            report.bad("SL-DROPIN-MISSING", f"编排引用了 {name}，没有 {name}.d/{DROPIN_NAME}")
    for name in sorted(dropin_dirs - referenced):
        report.bad("SL-DROPIN-ORPHAN", f"{name}.d/{DROPIN_NAME} 存在，却没有任何服务引用 {name}")
    if len(dropin_dirs) != EXPECTED_SLICE_COUNT:
        report.bad("SL-DROPIN-ORPHAN",
                   f"drop-in 共 {len(dropin_dirs)} 个，规格第 13.1 章配额表落 slice 的是 {EXPECTED_SLICE_COUNT} 行")
    if referenced <= slices.keys() and referenced == dropin_dirs:
        report.ok("SL-UNIT-MISSING", f"引用的 {len(referenced)} 个 slice 各有单元文件")
        report.ok("SL-DROPIN-MISSING", f"引用的 {len(referenced)} 个 slice 各有资源限额 drop-in")
        report.ok("SL-DROPIN-ORPHAN", "没有无人引用的 drop-in")


def run_equivalence(report: Report, loaded) -> None:
    print("== 两套编排的等价性 ==")
    quadlet, compose = loaded
    check_keys(report, quadlet, compose)
    services = check_service_set(report, quadlet, compose)
    check_required(report, services, quadlet, compose)
    check_facts(report, quadlet, compose)
    check_volumes(report, quadlet, compose)
    check_order_paired(report)


def run_slices(report: Report, loaded) -> None:
    print("== 引用的 slice 与资源限额 drop-in ==")
    quadlet, compose = loaded
    if not SYSTEMD_DIR.is_dir():
        report.gap("SL-UNIT-MISSING", f"{SYSTEMD_DIR} 读不到，slice 一侧未做出判定")
        return
    try:
        slices = facts_mod.slice_scan(SYSTEMD_DIR)
    except Unsupported as exc:
        report.gap("SL-UNIT-MISSING", f"slice 单元解析不了：{exc}")
        return
    referenced = {value for _, fact, value in quadlet["facts"] | compose["facts"]
                  if fact == "slice"}
    if not referenced:
        report.gap("SL-UNIT-MISSING", "两套编排里一个 slice 引用都没有，本半边未做出判定")
        return
    check_quota_absent(report, quadlet, compose, slices)
    check_slices(report, referenced, slices)


USAGE = """用法：verify-orchestration-equivalence.py [--equivalence | --slices | --all]

  --equivalence  只比两套编排的事实表是否逐项相等。
  --slices       只比编排引用的 slice 名与 deploy/systemd/system/ 下的单元及 drop-in 是否对得上。
  --all          两半都做（默认）。

退出码：0 通过；2 不符；3 被测对象读不到或解析不了、判定未做出；64 用法错误。
环境变量 EP_DEPLOY_ROOT 只为负样例存在，不设时取仓库的 deploy/。
"""


def main(argv: list[str]) -> int:
    mode = argv[1] if len(argv) > 1 else "--all"
    if len(argv) > 2 or mode not in ("--equivalence", "--slices", "--all", "-h", "--help"):
        sys.stderr.write(USAGE)
        return EXIT_USAGE
    if mode in ("-h", "--help"):
        sys.stdout.write(USAGE)
        return EXIT_OK

    report = Report()
    if os.environ.get("EP_DEPLOY_ROOT"):
        print(f"注意    本次比对使用替代根 {DEPLOY_ROOT}，非仓库路径，仅供负样例")
    loaded = load(report)
    if loaded is not None:
        if mode in ("--equivalence", "--all"):
            run_equivalence(report, loaded)
        if mode in ("--slices", "--all"):
            run_slices(report, loaded)

    print("== 结论 ==")
    print(f"不符 {report.mismatch} 项，未覆盖 {report.uncovered} 项")
    if report.mismatch:
        print("判定：不通过（两套不等价，或引用的 slice 对不上）")
        return EXIT_MISMATCH
    if report.uncovered:
        print("判定：未覆盖，本次核对不成立，不得据此判通过")
        return EXIT_UNCOVERED
    print("判定：通过")
    return EXIT_OK


if __name__ == "__main__":
    sys.exit(main(sys.argv))

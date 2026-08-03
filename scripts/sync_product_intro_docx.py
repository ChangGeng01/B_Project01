#!/usr/bin/env python3
"""Synchronize customer-facing copy in the generated DOCX without external helpers."""

from html import unescape
from pathlib import Path
import re
from xml.sax.saxutils import escape
from zipfile import ZIP_DEFLATED, ZipFile


WORKSPACE = Path("/Users/changgeng/Project/B_Project01/B_Project01")
DOCUMENT = WORKSPACE / "docs" / "介绍" / "企业一体化经营管理平台-产品介绍与功能大纲.docx"

REPLACEMENTS = {
    "业务数据持续汇总到统一报表，常用报表通常 10 秒内呈现": "业务数据持续汇总到统一报表；常用报表的性能指标以合同约定的基线环境与验收清单为准",
    "离线只用于现场作业和记录；付款和合同生效需联网并由中心确认，详见第 9 节。": "离线只用于现场作业和记录；合同生效、付款、开票、财务过账、结账和敏感数据导出不得离线完成，必须联网并经人工审批与中心确认。",
    "品牌可定制：每个客户可使用自己的名称、图标、颜色和客户端包。": "品牌可定制：每个客户可使用自己的名称、图标、颜色和客户端包；面向外部人员的应用商店分发使用客户自有开发者账号与证书，审核受阻时按合同切换到 Web/PWA 或其他约定形态并记录能力差异。",
    "支持批量导入、导出、在线查询、受控提交、公式、条件格式和数据透视表。": "电脑端 Office 或 WPS 宿主支持批量导入、导出、在线查询、受控提交、公式、条件格式和数据透视表；移动端按平台能力清单提供查看或原生表单替代。",
    "支持 OCR、批注、版本比较、电子签名、敏感信息遮盖和长期归档。": "电脑端支持模板套用、在线编辑、批注、修订、版本比较、多人协作、电子签名、敏感信息遮盖和长期归档；移动端以查看和批注为主，写入按能力清单转电脑端完成。",
    "AI 不能绕过权限；合同、付款、发票、过账等高风险操作必须确认或审批。": "AI 不能绕过权限；合同生效、付款、开票、财务过账、结账和敏感数据导出等高风险操作必须人工确认或审批。",
    "可连接银行、税务、电子签名、支付、物流、HR、PLM、MES、物联网、GIS、BI 和文档系统。": "可连接银行、税务、电子签名、支付、物流、HR、PLM、MES、物联网、GIS、BI 和文档系统；实际范围取决于对方接口、准入和逐客户联调条件。",
    "四个平台提供等价的业务能力，并根据屏幕和使用场景自动调整。电脑适合批量操作和复杂报表；手机和平板适合审批、查询、扫码、拍照、签到和现场作业。": "四个平台提供按规格定义的等价业务结果，并根据屏幕和使用场景自动调整。电脑适合批量操作、复杂报表、Office/WPS 和配置；手机和平板适合审批、查询、扫码、拍照、签到和现场作业。财务过账与结账、开票与税务写入、报表和仪表盘创作、Office 文档编辑协作、系统配置发布和动态扩展在移动端按清单提供查看、原生表单或不适用的替代路径。",
    "离线范围限于库存、生产、质检和现场工单；财务过账、付款和合同生效需联网并由中心确认。": "离线范围限于库存、生产、质检和现场工单；合同生效、付款、开票、财务过账、结账和敏感数据导出不得离线完成，必须联网并经人工审批与中心确认。",
    "防止外泄：敏感内容可脱敏、水印、限制复制打印、限制导出并追踪去向。": "防止外泄：敏感内容可脱敏、水印、导出审批和追踪去向；桌面端与合规移动端的打印、复制和下载失效控制按策略强制执行，浏览器门户端受浏览器能力限制，相关控制为尽力而为且不作承诺。",
    "合同生效、付款、开票和过账属高风险动作，只生成待审批任务，必须经人工审批后执行。": "合同生效、付款、开票、财务过账、结账和敏感数据导出属高风险动作，只生成待审批任务，必须经人工审批与中心确认后执行。",
    "连接企业已有系统的实际范围取决于对方系统开放的接口能力，需在实施前逐项确认。": "银行、税务与数电发票、支付、电子签章等接入需完成相应准入，并按具体银行和属地逐项开户、报备与联调；未完成时按交付说明和合同采用回单、对账文件导入等替代方式。",
}

TEXT_RUN = re.compile(r"(<w:t\b[^>]*>)(.*?)(</w:t>)", re.S)
PARAGRAPH = re.compile(r"<w:p\b[^>]*>.*?</w:p>", re.S)


def replace_paragraph(match: re.Match[str], changed: set[str]) -> str:
    block = match.group(0)
    runs = list(TEXT_RUN.finditer(block))
    if not runs:
        return block

    original = [unescape(run.group(2)) for run in runs]
    full_text = "".join(original)
    replacement = None
    for old, new in REPLACEMENTS.items():
        if old in full_text:
            replacement = full_text.replace(old, new, 1)
            changed.add(old)
            break
    if replacement is None:
        return block

    chunks: list[str] = []
    cursor = 0
    for index, part in enumerate(original):
        if index == len(original) - 1:
            chunks.append(replacement[cursor:])
        else:
            chunks.append(replacement[cursor : cursor + len(part)])
            cursor += len(part)

    output: list[str] = []
    cursor = 0
    for run, chunk in zip(runs, chunks):
        output.append(block[cursor : run.start(2)])
        output.append(escape(chunk))
        cursor = run.end(2)
    output.append(block[cursor:])
    return "".join(output)


def main() -> None:
    temporary = DOCUMENT.with_name(DOCUMENT.stem + ".sync.tmp.docx")
    changed: set[str] = set()

    with ZipFile(DOCUMENT, "r") as source, ZipFile(temporary, "w", ZIP_DEFLATED) as target:
        for info in source.infolist():
            data = source.read(info.filename)
            if info.filename == "word/document.xml":
                xml = data.decode("utf-8")
                xml = PARAGRAPH.sub(lambda match: replace_paragraph(match, changed), xml)
                data = xml.encode("utf-8")
            target.writestr(info, data)

    missing = [old for old in REPLACEMENTS if old not in changed]
    if missing:
        temporary.unlink()
        raise RuntimeError("DOCX text not found: " + " | ".join(missing))

    temporary.replace(DOCUMENT)
    print(f"Updated {DOCUMENT} ({len(changed)} paragraphs)")


if __name__ == "__main__":
    main()

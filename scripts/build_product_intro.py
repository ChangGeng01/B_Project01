#!/usr/bin/env python3
"""Build the customer-facing product introduction and feature outline."""

from pathlib import Path
import sys

from docx import Document
from docx.enum.section import WD_SECTION
from docx.enum.style import WD_STYLE_TYPE
from docx.enum.table import WD_CELL_VERTICAL_ALIGNMENT
from docx.enum.text import WD_ALIGN_PARAGRAPH, WD_TAB_ALIGNMENT
from docx.oxml import OxmlElement
from docx.oxml.ns import qn
from docx.shared import Inches, Pt, RGBColor


WORKSPACE = Path("/Users/changgeng/Project/B_Project01/B_Project01")
OUTPUT = WORKSPACE / "docs" / "介绍" / "企业一体化经营管理平台-产品介绍与功能大纲.docx"
SKILL_SCRIPTS = Path(
    "/Users/changgeng/.codex/plugins/cache/openai-primary-runtime/"
    "documents/26.715.12143/skills/documents/scripts"
)
sys.path.insert(0, str(SKILL_SCRIPTS))

from table_geometry import apply_table_geometry  # noqa: E402


# launch_messaging_guide -> compact_reference_guide token map.
PAGE_WIDTH_DXA = 9360
TABLE_INDENT_DXA = 120
CELL_MARGINS_DXA = {"top": 80, "bottom": 80, "start": 120, "end": 120}
LIST_MARKER_DXA = 270
LIST_TEXT_DXA = 540
LIST_HANGING_DXA = 270

# A single pan-CJK font prevents missing-glyph boxes while preserving a clean
# sans-serif appearance for Latin text and numerals. The QA renderer receives
# this TrueType font through its explicit SAL_FONTPATH.
LATIN_FONT = "Arial Unicode MS"
CHINESE_FONT = "Arial Unicode MS"

NAVY = "0B2545"
BLUE = "2E74B5"
DARK_BLUE = "1F4D78"
BODY = "202830"
MUTED = "667085"
LIGHT_FILL = "F4F6F9"
TABLE_FILL = "E8EEF5"
WHITE = "FFFFFF"
BORDER = "C9D5E3"


def rgb(hex_color: str) -> RGBColor:
    return RGBColor.from_string(hex_color)


def set_run_font(
    run,
    *,
    size: float | None = None,
    color: str = BODY,
    bold: bool | None = None,
    italic: bool | None = None,
    latin_font: str = LATIN_FONT,
    east_asia_font: str = CHINESE_FONT,
):
    run.font.name = latin_font
    run._element.get_or_add_rPr().rFonts.set(qn("w:ascii"), latin_font)
    run._element.get_or_add_rPr().rFonts.set(qn("w:hAnsi"), latin_font)
    run._element.get_or_add_rPr().rFonts.set(qn("w:eastAsia"), east_asia_font)
    run._element.get_or_add_rPr().rFonts.set(qn("w:cs"), latin_font)
    if size is not None:
        run.font.size = Pt(size)
    run.font.color.rgb = rgb(color)
    if bold is not None:
        run.bold = bold
    if italic is not None:
        run.italic = italic
    return run


def set_style_font(style, *, size: float, color: str, bold: bool = False):
    style.font.name = LATIN_FONT
    style.font.size = Pt(size)
    style.font.bold = bold
    style.font.color.rgb = rgb(color)
    rpr = style._element.get_or_add_rPr()
    rpr.rFonts.set(qn("w:ascii"), LATIN_FONT)
    rpr.rFonts.set(qn("w:hAnsi"), LATIN_FONT)
    rpr.rFonts.set(qn("w:eastAsia"), CHINESE_FONT)
    rpr.rFonts.set(qn("w:cs"), LATIN_FONT)


def configure_styles(doc: Document):
    normal = doc.styles["Normal"]
    set_style_font(normal, size=11, color=BODY)
    normal.paragraph_format.space_before = Pt(0)
    normal.paragraph_format.space_after = Pt(6)
    normal.paragraph_format.line_spacing = 1.25
    normal.paragraph_format.widow_control = True

    heading_specs = {
        "Heading 1": (16, BLUE, 18, 10),
        "Heading 2": (13, BLUE, 14, 7),
        "Heading 3": (12, DARK_BLUE, 10, 5),
    }
    for style_name, (size, color, before, after) in heading_specs.items():
        style = doc.styles[style_name]
        set_style_font(style, size=size, color=color, bold=True)
        style.paragraph_format.space_before = Pt(before)
        style.paragraph_format.space_after = Pt(after)
        style.paragraph_format.line_spacing = 1.0
        style.paragraph_format.keep_with_next = True
        style.paragraph_format.keep_together = True

    for name, size, color, bold, before, after, line in (
        ("Intro Lead", 13.5, NAVY, False, 0, 12, 1.25),
        ("Small Note", 9.5, MUTED, False, 0, 4, 1.15),
        ("Callout", 11.5, NAVY, True, 0, 0, 1.25),
    ):
        if name not in doc.styles:
            style = doc.styles.add_style(name, WD_STYLE_TYPE.PARAGRAPH)
        else:
            style = doc.styles[name]
        set_style_font(style, size=size, color=color, bold=bold)
        style.paragraph_format.space_before = Pt(before)
        style.paragraph_format.space_after = Pt(after)
        style.paragraph_format.line_spacing = line


def configure_section(section):
    section.page_width = Inches(8.5)
    section.page_height = Inches(11)
    section.top_margin = Inches(1.0)
    section.right_margin = Inches(1.0)
    section.bottom_margin = Inches(1.0)
    section.left_margin = Inches(1.0)
    section.header_distance = Inches(0.492)
    section.footer_distance = Inches(0.492)
    section.different_first_page_header_footer = True


def add_page_field(paragraph):
    run = paragraph.add_run()
    begin = OxmlElement("w:fldChar")
    begin.set(qn("w:fldCharType"), "begin")
    instruction = OxmlElement("w:instrText")
    instruction.set(qn("xml:space"), "preserve")
    instruction.text = " PAGE "
    separate = OxmlElement("w:fldChar")
    separate.set(qn("w:fldCharType"), "separate")
    text = OxmlElement("w:t")
    text.text = "1"
    end = OxmlElement("w:fldChar")
    end.set(qn("w:fldCharType"), "end")
    run._r.extend([begin, instruction, separate, text, end])
    set_run_font(run, size=9, color=MUTED)


def configure_header_footer(section):
    header = section.header
    p = header.paragraphs[0]
    p.paragraph_format.space_after = Pt(0)
    p.paragraph_format.line_spacing = 1.0
    left = p.add_run("企业一体化经营管理平台")
    set_run_font(left, size=9, color=MUTED, bold=True)
    p.add_run("\t")
    right = p.add_run("产品介绍与功能大纲")
    set_run_font(right, size=9, color=MUTED)
    p.paragraph_format.tab_stops.add_tab_stop(Inches(6.5), WD_TAB_ALIGNMENT.RIGHT)

    first_header = section.first_page_header
    first_header.paragraphs[0].text = ""

    for footer in (section.footer, section.first_page_footer):
        p = footer.paragraphs[0]
        p.alignment = WD_ALIGN_PARAGRAPH.RIGHT
        p.paragraph_format.space_before = Pt(0)
        p.paragraph_format.space_after = Pt(0)
        p.paragraph_format.line_spacing = 1.0
        prefix = p.add_run("第 ")
        set_run_font(prefix, size=9, color=MUTED)
        add_page_field(p)
        suffix = p.add_run(" 页")
        set_run_font(suffix, size=9, color=MUTED)


def next_numbering_id(numbering_root, tag_name: str) -> int:
    values = []
    for element in numbering_root.findall(qn(tag_name)):
        attr = "w:abstractNumId" if tag_name == "w:abstractNum" else "w:numId"
        raw = element.get(qn(attr))
        if raw is not None:
            values.append(int(raw))
    return max(values, default=0) + 1


def create_numbering(doc: Document, *, decimal: bool) -> int:
    root = doc.part.numbering_part.element
    abstract_id = next_numbering_id(root, "w:abstractNum")
    num_id = next_numbering_id(root, "w:num")

    abstract = OxmlElement("w:abstractNum")
    abstract.set(qn("w:abstractNumId"), str(abstract_id))
    multi = OxmlElement("w:multiLevelType")
    multi.set(qn("w:val"), "singleLevel")
    abstract.append(multi)

    lvl = OxmlElement("w:lvl")
    lvl.set(qn("w:ilvl"), "0")
    start = OxmlElement("w:start")
    start.set(qn("w:val"), "1")
    lvl.append(start)
    num_fmt = OxmlElement("w:numFmt")
    num_fmt.set(qn("w:val"), "decimal" if decimal else "bullet")
    lvl.append(num_fmt)
    lvl_text = OxmlElement("w:lvlText")
    lvl_text.set(qn("w:val"), "%1." if decimal else "•")
    lvl.append(lvl_text)
    lvl_jc = OxmlElement("w:lvlJc")
    lvl_jc.set(qn("w:val"), "left")
    lvl.append(lvl_jc)

    ppr = OxmlElement("w:pPr")
    tabs = OxmlElement("w:tabs")
    tab = OxmlElement("w:tab")
    tab.set(qn("w:val"), "num")
    tab.set(qn("w:pos"), str(LIST_TEXT_DXA))
    tabs.append(tab)
    ppr.append(tabs)
    ind = OxmlElement("w:ind")
    ind.set(qn("w:left"), str(LIST_TEXT_DXA))
    ind.set(qn("w:hanging"), str(LIST_HANGING_DXA))
    ppr.append(ind)
    spacing = OxmlElement("w:spacing")
    spacing.set(qn("w:after"), "80")
    spacing.set(qn("w:line"), "300")
    spacing.set(qn("w:lineRule"), "auto")
    ppr.append(spacing)
    lvl.append(ppr)

    rpr = OxmlElement("w:rPr")
    fonts = OxmlElement("w:rFonts")
    fonts.set(qn("w:ascii"), LATIN_FONT)
    fonts.set(qn("w:hAnsi"), LATIN_FONT)
    fonts.set(qn("w:eastAsia"), CHINESE_FONT)
    rpr.append(fonts)
    lvl.append(rpr)
    abstract.append(lvl)
    root.append(abstract)

    num = OxmlElement("w:num")
    num.set(qn("w:numId"), str(num_id))
    abstract_ref = OxmlElement("w:abstractNumId")
    abstract_ref.set(qn("w:val"), str(abstract_id))
    num.append(abstract_ref)
    root.append(num)
    return num_id


def apply_num(paragraph, num_id: int, *, compact: bool = False):
    ppr = paragraph._p.get_or_add_pPr()
    num_pr = ppr.find(qn("w:numPr"))
    if num_pr is None:
        num_pr = OxmlElement("w:numPr")
        ppr.append(num_pr)
    ilvl = OxmlElement("w:ilvl")
    ilvl.set(qn("w:val"), "0")
    num_id_el = OxmlElement("w:numId")
    num_id_el.set(qn("w:val"), str(num_id))
    num_pr.extend([ilvl, num_id_el])
    paragraph.paragraph_format.left_indent = None
    paragraph.paragraph_format.first_line_indent = None
    paragraph.paragraph_format.space_after = Pt(2 if compact else 4)
    paragraph.paragraph_format.line_spacing = 1.15 if compact else 1.25


def add_bullet(
    doc: Document,
    text: str,
    bullet_num_id: int,
    *,
    bold_lead: str | None = None,
    compact: bool = False,
):
    p = doc.add_paragraph()
    apply_num(p, bullet_num_id, compact=compact)
    font_size = 10.5 if compact else 11
    if bold_lead and text.startswith(bold_lead):
        lead = p.add_run(bold_lead)
        set_run_font(lead, size=font_size, color=BODY, bold=True)
        rest = p.add_run(text[len(bold_lead) :])
        set_run_font(rest, size=font_size, color=BODY)
    else:
        run = p.add_run(text)
        set_run_font(run, size=font_size, color=BODY)
    return p


def add_numbered(
    doc: Document,
    text: str,
    number_num_id: int,
    *,
    bold_lead: str | None = None,
    compact: bool = False,
):
    p = doc.add_paragraph()
    apply_num(p, number_num_id, compact=compact)
    font_size = 10.5 if compact else 11
    if bold_lead and text.startswith(bold_lead):
        lead = p.add_run(bold_lead)
        set_run_font(lead, size=font_size, color=NAVY, bold=True)
        rest = p.add_run(text[len(bold_lead) :])
        set_run_font(rest, size=font_size, color=BODY)
    else:
        run = p.add_run(text)
        set_run_font(run, size=font_size, color=BODY)
    return p


def set_cell_shading(cell, fill: str):
    tc_pr = cell._tc.get_or_add_tcPr()
    shd = tc_pr.find(qn("w:shd"))
    if shd is None:
        shd = OxmlElement("w:shd")
        tc_pr.append(shd)
    shd.set(qn("w:fill"), fill)
    shd.set(qn("w:val"), "clear")


def set_cell_borders(cell, color: str = BORDER, size: int = 6):
    tc_pr = cell._tc.get_or_add_tcPr()
    borders = tc_pr.find(qn("w:tcBorders"))
    if borders is None:
        borders = OxmlElement("w:tcBorders")
        tc_pr.append(borders)
    for edge in ("top", "left", "bottom", "right", "insideH", "insideV"):
        element = borders.find(qn(f"w:{edge}"))
        if element is None:
            element = OxmlElement(f"w:{edge}")
            borders.append(element)
        element.set(qn("w:val"), "single")
        element.set(qn("w:sz"), str(size))
        element.set(qn("w:space"), "0")
        element.set(qn("w:color"), color)


def prevent_row_split(row):
    tr_pr = row._tr.get_or_add_trPr()
    if tr_pr.find(qn("w:cantSplit")) is None:
        tr_pr.append(OxmlElement("w:cantSplit"))


def repeat_header(row):
    tr_pr = row._tr.get_or_add_trPr()
    if tr_pr.find(qn("w:tblHeader")) is None:
        tr_pr.append(OxmlElement("w:tblHeader"))


def format_cell(cell, text: str, *, bold: bool = False, color: str = BODY, size: float = 10.3):
    cell.vertical_alignment = WD_CELL_VERTICAL_ALIGNMENT.CENTER
    p = cell.paragraphs[0]
    p.alignment = WD_ALIGN_PARAGRAPH.LEFT
    p.paragraph_format.space_before = Pt(0)
    p.paragraph_format.space_after = Pt(0)
    p.paragraph_format.line_spacing = 1.15
    p.clear()
    run = p.add_run(text)
    set_run_font(run, size=size, color=color, bold=bold)


def add_table(
    doc: Document,
    headers: list[str] | None,
    rows: list[list[str]],
    widths: list[int],
    *,
    first_column_emphasis: bool = False,
    body_font_size: float = 10.3,
):
    row_count = len(rows) + (1 if headers else 0)
    table = doc.add_table(rows=row_count, cols=len(widths))
    table.style = "Table Grid"
    cursor = 0
    if headers:
        header_row = table.rows[0]
        repeat_header(header_row)
        prevent_row_split(header_row)
        for index, text in enumerate(headers):
            set_cell_shading(header_row.cells[index], TABLE_FILL)
            set_cell_borders(header_row.cells[index])
            format_cell(header_row.cells[index], text, bold=True, color=NAVY, size=10.2)
        cursor = 1
    for row_index, values in enumerate(rows, start=cursor):
        row = table.rows[row_index]
        prevent_row_split(row)
        for col_index, text in enumerate(values):
            cell = row.cells[col_index]
            set_cell_borders(cell)
            if first_column_emphasis and col_index == 0:
                set_cell_shading(cell, LIGHT_FILL)
            format_cell(
                cell,
                text,
                bold=first_column_emphasis and col_index == 0,
                color=NAVY if first_column_emphasis and col_index == 0 else BODY,
                size=body_font_size,
            )
    apply_table_geometry(
        table,
        widths,
        table_width_dxa=PAGE_WIDTH_DXA,
        indent_dxa=TABLE_INDENT_DXA,
        cell_margins_dxa=CELL_MARGINS_DXA,
    )
    after = doc.add_paragraph()
    after.paragraph_format.space_before = Pt(0)
    after.paragraph_format.space_after = Pt(2)
    return table


def add_callout(doc: Document, text: str, *, label: str | None = None):
    p = doc.add_paragraph()
    p.style = doc.styles["Callout"]
    p.paragraph_format.space_before = Pt(2)
    p.paragraph_format.space_after = Pt(6)
    p.paragraph_format.line_spacing = 1.25
    p.paragraph_format.left_indent = Pt(9)
    p.paragraph_format.right_indent = Pt(9)

    p_pr = p._p.get_or_add_pPr()
    shd = OxmlElement("w:shd")
    shd.set(qn("w:fill"), LIGHT_FILL)
    shd.set(qn("w:val"), "clear")
    p_pr.append(shd)
    borders = OxmlElement("w:pBdr")
    for edge in ("top", "left", "bottom", "right"):
        border = OxmlElement(f"w:{edge}")
        border.set(qn("w:val"), "single")
        border.set(qn("w:sz"), "8")
        border.set(qn("w:space"), "6")
        border.set(qn("w:color"), BORDER)
        borders.append(border)
    p_pr.append(borders)

    if label:
        lead = p.add_run(f"{label}  ")
        set_run_font(lead, size=10.5, color=BLUE, bold=True)
    run = p.add_run(text)
    set_run_font(run, size=11.5, color=NAVY, bold=True)
    return p


def add_body(doc: Document, text: str, *, bold_lead: str | None = None):
    p = doc.add_paragraph()
    if bold_lead and text.startswith(bold_lead):
        lead = p.add_run(bold_lead)
        set_run_font(lead, size=11, color=BODY, bold=True)
        rest = p.add_run(text[len(bold_lead) :])
        set_run_font(rest, size=11, color=BODY)
    else:
        run = p.add_run(text)
        set_run_font(run, size=11, color=BODY)
    return p


def add_heading(doc: Document, text: str, level: int, *, page_break_before: bool = False):
    p = doc.add_paragraph(text, style=f"Heading {level}")
    p.paragraph_format.page_break_before = page_break_before
    for run in p.runs:
        set_run_font(
            run,
            size={1: 16, 2: 13, 3: 12}[level],
            color={1: BLUE, 2: BLUE, 3: DARK_BLUE}[level],
            bold=True,
        )
    return p


def build_document():
    doc = Document()
    section = doc.sections[0]
    configure_section(section)
    configure_header_footer(section)
    configure_styles(doc)
    bullet_num_id = create_numbering(doc, decimal=False)
    number_num_id = create_numbering(doc, decimal=True)

    props = doc.core_properties
    props.title = "企业一体化经营管理平台 - 产品介绍与功能大纲"
    props.subject = "面向非技术读者的产品介绍（总体设计阶段，功能以正式发布版本为准）"
    props.author = "Codex"
    props.keywords = "企业管理, CRM, 合同, 订单, 采购, 财务, 售后, 私有化"

    # First-page customer-pack header pattern, without a decorative rule.
    spacer = doc.add_paragraph()
    spacer.paragraph_format.space_after = Pt(10)
    kicker = doc.add_paragraph()
    kicker.paragraph_format.space_before = Pt(0)
    kicker.paragraph_format.space_after = Pt(2)
    run = kicker.add_run("产品介绍 / 功能大纲")
    set_run_font(run, size=10.5, color=BLUE, bold=True)

    title = doc.add_paragraph()
    title.paragraph_format.space_before = Pt(0)
    title.paragraph_format.space_after = Pt(8)
    run = title.add_run("企业一体化经营管理平台")
    set_run_font(run, size=29, color=NAVY, bold=True)

    subtitle = doc.add_paragraph()
    subtitle.style = doc.styles["Intro Lead"]
    subtitle.paragraph_format.space_after = Pt(18)
    run = subtitle.add_run("把客户、合同、订单、采购、库存、财务和售后连成一条线")
    set_run_font(run, size=13.5, color=MUTED)

    add_callout(
        doc,
        "同一份信息只录入一次，后续部门自动接力；每件事都有负责人、进度、审批和完整记录。",
        label="一句话说明",
    )

    add_body(
        doc,
        "本文档说明产品的总体设计和功能规划。产品正在分阶段研发，文中功能不代表当前均已可用，实际范围以正式发布版本和验收清单为准，详见第 9 节。",
    )

    add_heading(doc, "这是一套什么软件？", 1)
    add_body(
        doc,
        "这是一套由企业自己掌控的综合经营管理软件。它把原来分散在不同软件、Excel、邮件、聊天和纸质单据里的工作，放进同一个安全、可追溯的工作体系。",
    )
    add_body(
        doc,
        "一笔业务可以从客户线索开始，一直跟踪到报价、签约、下单、采购或生产、交付、开票、收款、售后和利润分析。企业也可以保留现有专业软件，通过接口连接，不必一次全部替换。",
    )

    add_table(
        doc,
        ["项目", "说明"],
        [
            ["使用设备", "Windows、macOS、iPhone、iPad、Android"],
            ["部署方式", "企业机房、私有云，或企业自己的国内云服务器"],
            ["数据原则", "每个客户独立部署；数据、备份、日志和 AI 上下文由客户控制"],
            ["核心特点", "高度自动化、高度定制、严格权限、完整审计、可连接已有系统"],
            ["适用范围", "首版面向中国大陆业务，仅支持简体中文与人民币；不含多币种、外汇和进出口"],
        ],
        [2700, 6660],
        first_column_emphasis=True,
    )

    add_heading(doc, "适合哪些企业？", 2)
    add_body(
        doc,
        "尤其适合销售、采购、仓库、生产、财务和售后需要紧密协同的企业，也适合多法人、多部门、多地点、重视数据保密或需要大量定制流程的组织。",
    )

    add_heading(doc, "1. 一笔订单怎样走完全程", 1, page_break_before=True)
    add_body(doc, "用最常见的一笔订单来理解这套系统：")
    flow_steps = [
        ("销售建单：", "销售选择客户、产品、数量、价格和交期，系统自动带出已有资料。"),
        ("合同审批：", "关键条款、折扣、付款计划和附件进入对应审批，不能越权跳过。"),
        ("自动分解：", "系统检查库存、产能和采购周期，生成采购、生产、项目或调拨需求。"),
        ("采购或生产：", "采购看到要买什么、何时到货；生产看到要做什么、何时完工。"),
        ("仓库与交付：", "收货、质检、入库、拣货、出库、物流和客户签收全部有记录。"),
        ("财务处理：", "业务单据自动形成应收、应付、开票和会计处理建议；付款需经受控审批后执行。"),
        ("售后服务：", "投诉自动形成工单，关联原订单、合同、产品、批次、设备和保修。"),
        ("管理分析：", "管理层随时查看收入、成本、库存、现金、交付、质量和利润。"),
    ]
    for lead, rest in flow_steps:
        add_numbered(doc, lead + rest, number_num_id, bold_lead=lead, compact=True)

    add_callout(
        doc,
        "客户 -> 合同 -> 订单 -> 采购/生产 -> 仓库交付 -> 财务收付款 -> 售后，全程共用同一条业务记录。",
        label="核心闭环",
    )

    add_heading(doc, "2. 它解决的日常问题", 1)
    add_table(
        doc,
        ["常见情况", "使用平台后", "直接结果"],
        [
            ["客户资料在个人手里", "统一客户档案和全部往来记录", "人员变化也不会丢客户"],
            ["订单靠 Excel 或聊天转发", "订单自动流向采购、仓库、生产和财务", "少重复录入，少漏单错单"],
            ["合同、回款和附件分散", "关键条款、收款计划、版本和附件放在一起", "履约、到期和欠款随时可查"],
            ["采购不知道销售何时要货", "系统按订单、库存和交期形成采购建议", "减少缺料、积压和紧急采购"],
            ["投诉没有负责人或超期", "投诉自动建工单并按服务时限升级", "处理过程清楚，责任明确"],
            ["经营报表月底手工拼", "业务数据实时汇总到统一报表", "管理层更早发现问题"],
        ],
        [2500, 3590, 3270],
        body_font_size=9.9,
    )

    add_heading(doc, "3. 核心业务功能", 1, page_break_before=True)

    add_heading(doc, "客户与销售", 2)
    add_body(doc, "统一管理客户、联系人、线索、商机、跟进、报价、销售目标和预测。")
    add_bullet(doc, "销售可查看客户的历史报价、订单、合同、回款、投诉、设备和服务记录。", bullet_num_id)
    add_bullet(doc, "支持渠道、经销商、线索报备、返利和佣金，也可让客户通过门户自助查询。", bullet_num_id)

    add_heading(doc, "合同管理", 2)
    add_body(doc, "重点把合同的三类信息管清楚：关键条款、收付款信息、合同附件。")
    add_bullet(doc, "支持模板、版本、批注、审批、签章、印章、履约、续签和到期提醒。", bullet_num_id)
    add_bullet(doc, "合同审批后，可自动生成订单、采购需求、项目任务、收款计划和交付节点。", bullet_num_id)

    add_heading(doc, "订单管理", 2)
    add_body(doc, "销售下单后，系统自动检查价格权限、合同、库存和交期。")
    add_bullet(doc, "支持订单变更、分批交付、退货、换货、直运、寄售、订阅和租赁。", bullet_num_id)
    add_bullet(doc, "每次变更都保留版本和审批记录，避免口头修改造成部门信息不一致。", bullet_num_id)

    add_heading(doc, "采购与供应商", 2)
    add_body(doc, "订单、生产、项目或库存不足都可以自动形成采购建议。")
    add_bullet(doc, "覆盖申请、询价、比价、招标、合同、订单、收货、退货、发票和付款。", bullet_num_id)
    add_bullet(doc, "记录供应商资质、价格、交期、质量和风险；供应商也可通过门户协同。", bullet_num_id)

    add_heading(doc, "3. 核心业务功能（续）", 1, page_break_before=True)

    add_heading(doc, "库存、仓库与交付", 2)
    add_body(doc, "管理多仓库、多库位、批次、序列号、保质期、条码、二维码和 RFID。")
    add_bullet(doc, "覆盖收货、质检、上架、预留、拣货、复核、包装、出库、调拨和盘点。", bullet_num_id)
    add_bullet(doc, "发生质量问题时，可追踪受影响的批次、订单、客户、设备和售后记录。", bullet_num_id)

    add_heading(doc, "财务、税务与资金", 2)
    add_body(doc, "业务单据按规则形成财务处理建议，减少财务人员重复录入。")
    add_bullet(doc, "覆盖总账、应收、应付、费用、资产、成本、预算、合并、税务和电子会计档案。", bullet_num_id)
    add_bullet(doc, "支持中国大陆人民币业务、数电发票、银行对账和受控付款审批。", bullet_num_id)
    add_bullet(doc, "首版面向中国大陆业务，不含多币种、外汇、进出口、报关和信用证。", bullet_num_id)

    add_heading(doc, "投诉、工单与售后", 2)
    add_body(doc, "投诉进入系统后自动形成工单，关联客户、订单、合同、产品、批次和保修。")
    add_bullet(doc, "工单可按规则辅助分派给责任人，并按服务时限自动提醒和升级。", bullet_num_id)
    add_bullet(doc, "现场人员可用手机签到、扫码、拍照、换件和客户签字，断网时也能完成现场作业。", bullet_num_id)
    add_bullet(doc, "离线只用于现场作业和记录；付款和合同生效需联网并由中心确认，详见第 9 节。", bullet_num_id)

    add_heading(doc, "生产、项目与质量", 2)
    add_body(doc, "需要时可安装制造、项目、研发、质量和设备管理模块。")
    add_bullet(doc, "支持离散制造、流程制造、项目型制造，以及 MRP、排产、MES 和委外。", bullet_num_id)
    add_bullet(doc, "支持项目预算和利润、图纸与版本、来料/过程/成品质量、实验室和召回。", bullet_num_id)

    add_heading(doc, "4. 按需增加的企业功能", 1, page_break_before=True)
    add_body(
        doc,
        "平台不会把所有企业强行做成同一种管理方式。功能可以按模块安装，也可以连接企业已有系统。停用模块时，历史数据仍按规则安全保留。",
    )

    extension_groups = [
        ("制造与现场：", "MRP/APS、MES、PLM、QMS、LIMS、设备维护、物联网、数字孪生和追溯召回。"),
        ("企业运营：", "人力资源、项目组合、物流运输、EHS、GRC、IT 服务、法务、费用差旅、ESG 与碳管理。"),
        ("商业增长：", "CPQ、订阅、租赁、渠道佣金、电商、POS、市场平台、PIM/DAM、CMS 和 GIS。"),
        ("门户与协作：", "客户、供应商、渠道、员工门户，以及知识、文档、日历、任务、通知和联络中心。"),
        ("分析与计划：", "经营驾驶舱、BI、预算预测、产销协同、流程挖掘、知识图谱、优化和情景模拟。"),
    ]
    for lead, rest in extension_groups:
        add_bullet(doc, lead + rest, bullet_num_id, bold_lead=lead, compact=True)

    add_callout(
        doc,
        "每个业务领域都可以选择：使用系统内置模块，或连接现有专业软件；同一份数据只指定一个主系统，避免互相覆盖。",
        label="自由组合",
    )

    add_heading(doc, "高度定制，但不破坏升级", 1)
    customization_items = [
        ("数据可定制：", "新增业务对象、字段、关系、编号、校验、视图和搜索。"),
        ("界面可定制：", "调整表单、列表、首页、菜单、看板和移动端任务页面。"),
        ("流程可定制：", "设置审批、会签、时限、提醒、升级、自动任务和跨部门动作。"),
        ("权限可定制：", "按法人、部门、岗位、项目、客户、记录和字段控制访问。"),
        ("报表可定制：", "建立企业自己的指标、报表、打印模板和管理驾驶舱。"),
        ("品牌可定制：", "每个客户可使用自己的名称、图标、颜色和客户端包。"),
        ("扩展可安装：", "通过签名模块或受控插件增加特殊能力；配置先验证审批，出现问题可回退。"),
    ]
    for lead, rest in customization_items:
        add_bullet(doc, lead + rest, bullet_num_id, bold_lead=lead, compact=True)

    add_heading(doc, "5. Excel、文档、AI 与外部连接", 1, page_break_before=True)

    add_heading(doc, "Excel 与 WPS", 2)
    add_bullet(doc, "支持批量导入、导出、在线查询、受控提交、公式、条件格式和数据透视表。", bullet_num_id)
    add_bullet(doc, "常用分析可以继续在熟悉的表格方式中完成，再发布为正式报表。", bullet_num_id)
    add_bullet(doc, "为了安全，不执行 VBA、任意宏或未经批准的第三方加载项。", bullet_num_id)

    add_heading(doc, "合同、附件与电子档案", 2)
    add_bullet(doc, "合同、发票、图片、Word/WPS、PDF、OFD 和常见工程文件统一存档。", bullet_num_id)
    add_bullet(doc, "支持 OCR、批注、版本比较、电子签名、敏感信息遮盖和长期归档。", bullet_num_id)
    add_bullet(doc, "大文件支持分片上传、断点续传、去重、安全扫描和权限控制。", bullet_num_id)

    add_heading(doc, "完全本地的 AI 助手", 2)
    add_bullet(doc, "可帮助识别合同和发票、整理客户与工单摘要、搜索知识、填写草稿和预测需求。", bullet_num_id)
    add_bullet(doc, "模型和企业数据在客户控制的环境内运行，不把敏感内容发送给公共 AI。", bullet_num_id)
    add_bullet(doc, "AI 不能绕过权限；合同、付款、发票、过账等高风险操作必须确认或审批。", bullet_num_id)

    add_heading(doc, "MCP、接口与现有软件", 2)
    add_body(
        doc,
        "可以把 MCP 理解为一套受控的“工具连接标准”：企业 AI 只能在员工原有权限内查询资料、生成草稿或发起流程，每次调用都有身份、范围和审计记录。",
    )
    add_bullet(doc, "可连接银行、税务、电子签名、支付、物流、HR、PLM、MES、物联网、GIS、BI 和文档系统。", bullet_num_id)
    add_bullet(doc, "连接失败时支持重试、告警、待修复队列、重放和对账，不让问题悄悄丢失。", bullet_num_id)

    add_heading(doc, "6. 四端使用、私有部署与安全", 1, page_break_before=True)

    add_heading(doc, "Windows、macOS、iOS、Android", 2)
    add_body(
        doc,
        "四个平台提供等价的业务能力，并根据屏幕和使用场景自动调整。电脑适合批量操作和复杂报表；手机和平板适合审批、查询、扫码、拍照、签到和现场作业。",
    )
    add_bullet(doc, "不同设备使用同一套数据、权限和业务规则。", bullet_num_id)
    add_bullet(doc, "仓库、工厂和现场可在受控条件下断网工作，恢复网络后再安全同步。", bullet_num_id)
    add_bullet(doc, "离线范围限于库存、生产、质检和现场工单；财务过账、付款和合同生效需联网并由中心确认。", bullet_num_id)
    add_bullet(doc, "边缘节点默认支持 3 天独立运行，可按容量和业务连续性要求延长。", bullet_num_id)

    add_heading(doc, "企业自己选择部署位置", 2)
    add_bullet(doc, "可部署在企业机房、私有云，或阿里云、腾讯云、华为云等企业自己的国内云环境。", bullet_num_id)
    add_bullet(doc, "数据库、对象存储、密钥服务等既可自建，也可连接已有的合规服务。", bullet_num_id)
    add_bullet(doc, "生产数据、备份、日志、索引和 AI 上下文限定在中国大陆境内。", bullet_num_id)

    add_heading(doc, "高保密设计", 2)
    security_items = [
        ("权限细：", "可以控制到法人、部门、项目、客户、单条记录和单个字段。"),
        ("职责分开：", "系统、数据、安全、审计和密钥管理员相互制约。"),
        ("全程加密：", "传输、数据库、附件、备份和设备缓存均受加密与密钥管理保护。"),
        ("防止外泄：", "敏感内容可脱敏、水印、限制复制打印、限制导出并追踪去向。"),
        ("完整审计：", "关键查看、修改、审批、导出和系统调用均留下防篡改证据。"),
        ("持续可用：", "正式生产采用高可用部署，并配合异地不可变备份和恢复演练。"),
    ]
    for lead, rest in security_items:
        add_bullet(doc, lead + rest, bullet_num_id, bold_lead=lead)

    add_callout(
        doc,
        "云服务器只是可选的部署位置，不代表使用共享 SaaS。每个客户仍拥有独立系统、独立数据、独立密钥和独立升级通道。",
        label="重要区别",
    )

    add_heading(doc, "7. 每个岗位能得到什么", 1, page_break_before=True)
    add_table(
        doc,
        ["岗位", "不再困扰于", "直接得到"],
        [
            ["销售", "客户资料分散、交期不清、回款难追", "客户全景、准确报价、订单进度和回款提醒"],
            ["采购", "临时催料、需求反复变化、供应商难比较", "自动采购建议、比价、交期和供应商绩效"],
            ["仓库", "账实不符、批次不清、出入库靠纸", "实时库存、扫码作业、批次追溯和盘点闭环"],
            ["生产/项目", "物料、进度、成本和变更各自分散", "统一计划、任务、资源、质量、成本和交付"],
            ["财务", "重复录入、业务对不上账、凭证来源不清", "业财联动、自动对账、受控过账和完整证据"],
            ["售后", "投诉丢失、找不到历史、工单超期", "自动建单、规则分派、现场移动作业和服务追踪"],
            ["管理层", "报表滞后、口径不一、无法看到全局", "统一指标、实时经营视图、风险提醒和利润分析"],
        ],
        [1500, 3770, 4090],
    )

    add_heading(doc, "8. 最关键的产品定位", 1)
    add_body(
        doc,
        "这套平台不是要求企业立刻更换所有软件，而是建立一套统一、可控的经营底座。企业可以先启用客户、合同、订单、采购、财务和售后，再逐步增加制造、质量、项目、人力或其他模块。已有专业系统也可以继续使用，通过统一权限、接口和主数据规则连接起来。",
    )
    add_body(
        doc,
        "核心后台采用 Rust 构建，强调稳定、安全和长期可维护；客户差异通过配置、模块和签名插件实现，不为每个客户长期复制一套无法升级的代码。",
    )

    add_callout(
        doc,
        "最终目标：让企业的人、客户、订单、货物、资金、文件和流程在同一个可信体系中协同。",
        label="产品价值",
    )

    add_heading(doc, "9. 版本与适用范围说明", 1, page_break_before=True)
    add_body(
        doc,
        "本节说明本文档的效力范围。产品按阶段研发和交付，文中所列功能不代表当前均已可用，实际交付内容以合同和验收清单为准。",
    )
    scope_items = [
        ("阶段与版本：", "产品处于总体设计与分阶段研发阶段，不设置固定的完整版发布日期；所有正式业务模块完成并通过验收后才发布完整正式版。"),
        ("交付依据：", "每次交付的模块范围、版本和验收标准以双方签署的合同与验收清单为准，本文档不构成功能交付承诺。"),
        ("语言与币种：", "首版仅支持简体中文、人民币和中国标准时间，不支持多币种、外汇、进出口、报关、信用证和产品内容多语言。"),
        ("离线边界：", "库存、生产、质检和现场工单可在受控条件下离线处理，边缘节点默认支持 3 天独立运行并可延长；财务过账、付款和合同生效不得离线完成，必须联网并由中心确认。"),
        ("人工确认：", "AI 与自动化可在授权范围内自动生成单据、任务和建议；合同生效、付款、开票和过账属高风险动作，只生成待审批任务，必须经人工审批后执行。"),
        ("外部连接：", "连接企业已有系统的实际范围取决于对方系统开放的接口能力，需在实施前逐项确认。"),
    ]
    for lead, rest in scope_items:
        add_bullet(doc, lead + rest, bullet_num_id, bold_lead=lead, compact=True)

    add_callout(
        doc,
        "本文所述功能以正式发布版本和验收清单为准；未在合同中列明的能力不构成交付承诺。",
        label="范围说明",
    )

    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    doc.save(OUTPUT)
    print(OUTPUT)


if __name__ == "__main__":
    build_document()

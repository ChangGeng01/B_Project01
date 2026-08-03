#!/usr/bin/env python3
"""Build the customer-facing product introduction and feature outline."""

from pathlib import Path

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


def apply_table_geometry(table, widths, *, table_width_dxa, indent_dxa, cell_margins_dxa):
    """Apply deterministic table geometry without relying on external helpers."""
    table.autofit = False
    table_pr = table._tbl.tblPr

    tbl_width = table_pr.find(qn("w:tblW"))
    if tbl_width is None:
        tbl_width = OxmlElement("w:tblW")
        table_pr.insert(0, tbl_width)
    tbl_width.set(qn("w:w"), str(table_width_dxa))
    tbl_width.set(qn("w:type"), "dxa")

    tbl_layout = table_pr.find(qn("w:tblLayout"))
    if tbl_layout is None:
        tbl_layout = OxmlElement("w:tblLayout")
        table_pr.append(tbl_layout)
    tbl_layout.set(qn("w:type"), "fixed")

    tbl_indent = table_pr.find(qn("w:tblInd"))
    if tbl_indent is None:
        tbl_indent = OxmlElement("w:tblInd")
        table_pr.append(tbl_indent)
    tbl_indent.set(qn("w:w"), str(indent_dxa))
    tbl_indent.set(qn("w:type"), "dxa")

    grid = table._tbl.tblGrid
    for child in list(grid):
        grid.remove(child)
    for width in widths:
        col = OxmlElement("w:gridCol")
        col.set(qn("w:w"), str(width))
        grid.append(col)

    for row in table.rows:
        for cell, width in zip(row.cells, widths):
            tc_pr = cell._tc.get_or_add_tcPr()
            tc_width = tc_pr.find(qn("w:tcW"))
            if tc_width is None:
                tc_width = OxmlElement("w:tcW")
                tc_pr.insert(0, tc_width)
            tc_width.set(qn("w:w"), str(width))
            tc_width.set(qn("w:type"), "dxa")

            margins = tc_pr.find(qn("w:tcMar"))
            if margins is None:
                margins = OxmlElement("w:tcMar")
                tc_pr.append(margins)
            for side, value in cell_margins_dxa.items():
                margin = margins.find(qn(f"w:{side}"))
                if margin is None:
                    margin = OxmlElement(f"w:{side}")
                    margins.append(margin)
                margin.set(qn("w:w"), str(value))
                margin.set(qn("w:type"), "dxa")


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
    # OOXML CT_Numbering 要求全部 w:abstractNum 排在 w:num 之前，追加到末尾会破坏部件结构
    first_num = root.find(qn("w:num"))
    if first_num is None:
        root.append(abstract)
    else:
        first_num.addprevious(abstract)

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
        "一笔业务从销售录入合同开始，一直跟踪到合同审批、订单、采购、收货、交付、开票、收款、付款和售后，并汇总成收入、成本、交付和利润。首版内置的外部对接只有电子签章一类，企业已有系统可作为外部数据源，通过接口和文件导入导出交换数据。",
    )

    add_table(
        doc,
        ["项目", "说明"],
        [
            ["使用设备", "Windows、macOS、iOS（iPhone；iPad 以 iOS 客户端兼容运行）、Android"],
            ["部署方式", "企业机房、私有云，或企业自己的国内云服务器；核心交易数据库为 PostgreSQL，详见第 6 节"],
            ["数据原则", "每个客户独立部署；数据、备份和日志由客户控制"],
            ["核心特点", "合同贯穿全程、单据自动接力、高度定制、严格权限、完整审计"],
            ["适用范围", "首版面向中国大陆业务，仅支持简体中文与人民币；不含多币种、外汇和进出口"],
        ],
        [2700, 6660],
        first_column_emphasis=True,
    )

    add_heading(doc, "适合哪些企业？", 2)
    add_body(
        doc,
        "尤其适合以合同和订单驱动经营，销售、采购、仓库、财务和售后需要紧密协同的企业，也适合多法人、多部门、多地点、重视数据保密或需要大量定制流程的组织。",
    )

    add_heading(doc, "1. 一笔订单怎样走完全程", 1, page_break_before=True)
    add_body(doc, "用最常见的一笔订单来理解这套系统：")
    flow_steps = [
        ("销售建单：", "销售录入合同，选择客户、产品、数量、价格和交期，系统自动带出已有资料，并校验价格权限、库存可用量、交期和客户信用额度。"),
        ("合同审批：", "关键条款、折扣、付款计划和附件按各自审批链审批，不能越权跳过；管理层是合同审批的必经节点，意见、版本和附件全程留痕。"),
        ("合同生效：", "合同生效后自动生成销售订单、采购需求、项目任务、收款计划和交付节点，派生单据与合同双向可追溯。"),
        ("采购订货：", "采购查看已生效的合同与采购需求，按合同下达采购订单并可分批订货，供应商可通过门户确认订单与交期。"),
        ("收货入账：", "收货按物料、批次和序列号写入库存台账；采购发票登记后形成应付明细，并按不含税单价更新库存金额。"),
        ("发票申请与开具：", "销售按合同和订单提交发票申请，管理层审批后由财务登记开具结果，回写申请单状态与剩余可开比例，并形成应收。"),
        ("交付确认：", "发货或合同交付节点确认后确认收入，并同步按加权平均单价结转销货成本。"),
        ("到款与付款登记：", "财务按订单和发票登记到款，按采购发票登记付款，支持分次收付款，未核销部分进入账龄。"),
        ("售后工单：", "售后技术支持记录形成工单，关联原订单、合同、产品、批次、设备和保修，并进入客户档案。"),
        ("管理层看数：", "管理层随时查看收入、成本、交付和利润，并可按期间、客户、产品和合同下钻。"),
    ]
    for lead, rest in flow_steps:
        add_numbered(doc, lead + rest, number_num_id, bold_lead=lead, compact=True)

    add_callout(
        doc,
        "合同 -> 订单 -> 采购收货 -> 交付确认 -> 开票 -> 收付款 -> 售后 -> 经营看数，全程共用同一条业务记录。",
        label="核心闭环",
    )

    add_heading(doc, "2. 它解决的日常问题", 1)
    add_table(
        doc,
        ["常见情况", "使用平台后", "直接结果"],
        [
            ["客户资料在个人手里", "统一客户档案，历史合同、回款、投诉、设备和服务记录集中可查", "人员变化也不会丢客户"],
            ["合同、回款和附件分散", "关键条款、收付款信息、版本和附件放在一起", "履约、到期和欠款随时可查"],
            ["订单靠 Excel 或聊天转发", "合同生效后自动派生订单、采购需求、收款计划和交付节点", "少重复录入，少漏单错单"],
            ["采购不知道销售何时要货", "采购建议按合同、订单和库存不足自动形成", "减少缺料、积压和紧急采购"],
            ["赊销额度靠人工把关", "下单时自动校验客户信用额度，超额按配置阻断或转审批", "减少超额赊销带来的呆账"],
            ["发票和收款对不上", "发票申请、开具登记、收款计划与到款核销逐笔勾稽", "应收余额和账龄一目了然"],
            ["售后找不到单据出处", "工单关联原订单、合同、产品、批次、设备和保修", "处理过程清楚，责任明确"],
            ["经营报表月底手工拼", "收入、成本、交付和利润由业务单据持续汇总；常用报表的性能指标以合同约定的基线环境与验收清单为准", "管理层更早发现问题"],
        ],
        [2500, 3590, 3270],
        body_font_size=9.9,
    )

    add_heading(doc, "3. 核心业务功能", 1, page_break_before=True)

    add_heading(doc, "客户与销售", 2)
    add_body(doc, "统一管理客户档案、联系人和客户 360 视图，销售建单时自动带出客户、产品和价目资料。")
    add_bullet(doc, "销售可查看客户的历史合同、回款、投诉、设备和服务记录。", bullet_num_id)
    add_bullet(doc, "维护产品价目，录入合同和下单时校验价格权限，折扣随合同审批链审批。", bullet_num_id)
    add_bullet(doc, "首版不含线索、商机、漏斗、市场活动、销售预测和渠道佣金。", bullet_num_id)

    add_heading(doc, "合同管理", 2)
    add_body(doc, "重点把合同的三类信息管清楚：关键条款、收付款信息、合同附件。")
    add_bullet(doc, "支持模板、条款、修订版本、批注、审批、电子签章、实体印章、履约和义务跟踪，合同也可以合并。", bullet_num_id)
    add_bullet(doc, "支持合同续签：按原合同派生续签版本，保留与原合同的关联关系，以及原合同的履约记录、收付款计划和已派生单据的追溯链路；续签版本重新审批生效后派生新的订单、收款计划和交付节点。", bullet_num_id)
    add_bullet(doc, "支持合同到期提醒：按合同有效期、交付节点日期和收付款计划到期日生成提醒。", bullet_num_id)
    add_bullet(doc, "合同审批后，可自动生成订单、采购需求、项目任务、收款计划和交付节点。", bullet_num_id)
    add_bullet(doc, "合同变更按新版本重新派生或调整已派生的单据，派生单据与合同双向可追溯。", bullet_num_id)
    add_bullet(doc, "合同生效属高风险操作，须重新确认身份，审批链不能越权跳过。", bullet_num_id)

    add_heading(doc, "订单管理", 2)
    add_body(doc, "销售下单后，系统自动检查价格权限、合同、库存可用量、交期和客户信用额度。")
    add_bullet(doc, "客户信用额度由应收未收金额与在途订单金额推算，超出可用额度时按配置阻断或转审批。", bullet_num_id)
    add_bullet(doc, "支持订单变更、分批交付、退货、换货和直运；订阅、租赁与寄售作为订单类型登记：订阅与租赁登记周期和租期，寄售只登记订单类型，不含寄售在库台账与代销结算，货权转移仍以交付确认为准。", bullet_num_id)
    add_bullet(doc, "每次变更都保留版本和审批记录，避免口头修改造成部门信息不一致。", bullet_num_id)

    add_heading(doc, "采购与供应商", 2)
    add_body(doc, "合同、销售订单、项目任务或库存不足都可以自动形成采购建议。")
    add_bullet(doc, "采购查看审批生效的合同，按合同下达采购订单并可分批订货，覆盖收货、退货、采购发票和付款申请。", bullet_num_id)
    add_bullet(doc, "记录供应商准入与资质，以及价格、交期、质量和风险信息。", bullet_num_id)
    add_bullet(doc, "供应商可通过门户确认采购订单与交期、提交送货通知、上传发票、查询收付款对账并维护自身档案。", bullet_num_id)
    add_bullet(doc, "首版不含询比价、招投标、VMI 和供应商绩效评分模型。", bullet_num_id)

    add_heading(doc, "3. 核心业务功能（续）", 1, page_break_before=True)

    add_heading(doc, "库存与存货计价", 2)
    add_body(doc, "管理仓库与库存台账、收发存记录、可用量查询，以及批次与序列号标识。")
    add_bullet(doc, "收货、出库和退货按物料、批次和序列号登记，可用量直接支撑下单校验和交期判断。", bullet_num_id)
    add_bullet(doc, "存货计价采用移动加权平均一种方法：入库按采购不含税单价计价，出库按加权平均单价结转金额，数量账与金额账同步更新。", bullet_num_id)
    add_bullet(doc, "提供按仓库和物料的库存金额查询与期末库存价值表；退货按当时的加权平均单价回冲，不重算历史成本。", bullet_num_id)
    add_bullet(doc, "首版不含库位、质检、预留、拣货、波次、调拨和盘点，也不含先进先出、标准成本和采购费用分摊。", bullet_num_id)

    add_heading(doc, "财务：应收应付、开票与总账", 2)
    add_body(doc, "业务单据按固定规则生成财务记录，减少重复录入；开票与收付款在系统内登记，不依赖外部账套。")
    add_bullet(doc, "应收台账按客户、合同、订单和发票记录应收明细、收款计划、到款核销和账龄；应付台账按供应商、采购订单和采购发票记录应付明细、付款申请、付款核销和账龄。", bullet_num_id)
    add_bullet(doc, "发票申请、审批、开具登记与合同收款计划的勾稽在系统内连成一条链路；开票有误时可在系统内登记作废或红字冲销。", bullet_num_id)
    add_bullet(doc, "到款和付款按订单与发票登记，支持分次收付款，一笔款项可核销多张发票或订单；银行与现金账户档案和资金流水由人工登记。", bullet_num_id)
    add_bullet(doc, "简易总账提供会计科目表、凭证生成、科目余额表、总账与明细账查询、试算平衡、月度期间开闭和年度损益结转；已过账凭证只能以红字冲销或更正凭证追加更正。", bullet_num_id)
    add_bullet(doc, "成本来源只有两类：交付确认时按加权平均结转的销货成本，以及直接关联合同、订单或项目的采购发票与外购服务费用；可按合同、订单、客户和项目查询成本并得出毛利。", bullet_num_id)
    add_bullet(doc, "收入确认只有一种口径：交付确认时确认收入并同步结转销货成本，开票时形成应收和销项税额。", bullet_num_id)
    add_bullet(doc, "首版为人民币单一账簿，发票开具指按实际开票结果在系统内登记；不含固定资产、预算、费用报销、管理会计、合并报表、多币种、外汇、进出口、信用证、银企直连、自动对账和税务平台直连。", bullet_num_id)

    add_heading(doc, "售后工单与设备台账", 2)
    add_body(doc, "售后技术支持记录形成工单，关联原订单、合同、产品、批次、设备和保修。")
    add_bullet(doc, "设备台账记录设备编号或序列号、型号、所属客户、关联产品与批次、交付与安装日期和当前状态。", bullet_num_id)
    add_bullet(doc, "保修记录起止日期、保修范围和条款文本，建工单时自动读取在保状态。", bullet_num_id)
    add_bullet(doc, "客户投诉与工单进入客户 360 视图，退换修登记与订单的退货、换货打通。", bullet_num_id)
    add_bullet(doc, "工单按流程设定的时限自动提醒和升级；首版不含现场派工调度、服务权益计费和售后知识库。", bullet_num_id)

    add_heading(doc, "报表与经营看板", 2)
    add_body(doc, "报表和看板的数据直接来自业务单据，管理层不必等月底手工汇总。")
    add_bullet(doc, "预置收入、成本、利润、交付四类指标和一个默认管理驾驶舱，并配套应收账龄、应付账龄两张基础表。", bullet_num_id)
    add_bullet(doc, "四类指标可按期间、客户、产品和合同下钻；交付看合同交付节点与订单分批交付的按期完成率和逾期清单。", bullet_num_id)
    add_bullet(doc, "报表设计器支持拖拽设计、筛选、分组、汇总和像素级打印模板，企业可以建立自己的指标。", bullet_num_id)
    add_bullet(doc, "报表和看板的结果继承法人、记录和字段级权限，无权查看的数据不会出现在结果中。", bullet_num_id)
    add_bullet(doc, "首版不含内嵌电子表格、报表订阅与定时分发，以及外部 BI 平台的语义层对接。", bullet_num_id)

    add_heading(doc, "4. 首版模块安装范围与后续版本", 1, page_break_before=True)
    add_body(
        doc,
        "首版按模块交付。首版范围内的模块都可以按许可证安装、启用、停用、再启用和升级；停用只关闭该模块的界面入口、写入接口、定时任务和对外事件，历史数据继续保留，授权范围内仍可查询和审计检索。",
    )

    add_heading(doc, "首版可以安装的能力", 2)
    first_release_modules = [
        ("核心业务：", "主数据、客户档案与客户 360、合同（含续签与到期提醒）、销售订单与客户信用额度校验、采购与供应商、库存台账与存货计价、成本归集、项目任务与交付节点、售后工单与设备台账。"),
        ("财务：", "应收与应付台账、发票申请与开具登记、到款与付款登记、简易总账与期间结账。"),
        ("报表与经营看板：", "报表设计器、仪表盘、企业自定义指标、打印模板，以及收入、成本、交付、利润的预置指标与默认管理驾驶舱。"),
        ("供应商门户：", "供应商查看与确认采购订单和交期、提交送货通知、上传发票、查询收付款对账、维护自身资质与价格交期档案。"),
        ("电子签章连接器：", "合同审批通过后发起签署，回传签署结果与带签章的合同文件，用印留痕并归入合同附件与审计。"),
        ("其他：", "全文检索，以及订阅与租赁两种订单类型。"),
    ]
    for lead, rest in first_release_modules:
        add_bullet(doc, lead + rest, bullet_num_id, bold_lead=lead, compact=True)

    add_heading(doc, "属于后续版本的能力", 2)
    add_body(
        doc,
        "以下能力首版不交付，也不提供安装入口。它们已按原名称逐条登记，后续版本可以按登记恢复；恢复需要重新确定范围，不能在首版通过低代码配置、插件或连接器变相实现。",
    )
    deferred_groups = [
        ("制造与生产：", "MRP、APS、MES、PLM、QMS、LIMS，以及设备点检、维修工单、备件和预测维护。"),
        ("企业运营与治理：", "人力资源、运输管理、EHS、GRC、IT 服务、法务、商旅费控、文档与知识库、内容管理、统一消息中心、联络中心和 GIS。"),
        ("商业与渠道：", "电商、POS、会员、渠道佣金与返利，以及按量计费、独立账单和催收。"),
        ("门户：", "客户门户、经销商门户和员工门户；首版对外门户只有供应商门户。"),
        ("智能与检索：", "本地 AI、OCR、MCP、向量检索、知识图谱、流程挖掘、预测和仿真；首版检索能力只有全文检索。"),
        ("现场与设备联网：", "边缘节点、断网独立运行、工业协议接入、物联网和数字孪生。"),
        ("外部资金与税务：", "银企直连、自动对账、第三方支付渠道，以及数电发票平台与税控设备对接。"),
        ("更深的业务能力：", "完整仓库作业（波次、拣货、盘点、质检、预留、调拨）、寄售在库管理与代销结算、完整财务与管理会计（多账簿、固定资产、预算、费用报销、合并报表、资金管理）、完整成本管理，以及线索、商机、销售预测、询比价和招投标。"),
    ]
    for lead, rest in deferred_groups:
        add_bullet(doc, lead + rest, bullet_num_id, bold_lead=lead, compact=True)

    add_callout(
        doc,
        "首版交付以合同为中心的销售、采购、财务与工单闭环；上面列为后续版本的能力不在首版交付范围内。",
        label="范围提醒",
    )

    add_heading(doc, "高度定制，但不破坏升级", 1)
    customization_items = [
        ("数据可定制：", "新增业务对象、字段、关系、编号、校验、视图和搜索。"),
        ("界面可定制：", "调整表单、列表、首页、菜单、看板和移动端任务页面。"),
        ("流程可定制：", "设置审批、会签、时限、提醒、升级、自动任务和跨部门动作。"),
        ("权限可定制：", "按法人、部门、岗位、项目、客户、记录和字段控制访问。"),
        ("报表可定制：", "建立企业自己的指标、报表、打印模板和管理驾驶舱。"),
        ("品牌可定制：", "每个客户可使用自己的名称、图标、颜色和客户端包；面向外部人员的应用商店分发使用客户自有开发者账号与证书，审核受阻时按合同切换到 Web/PWA 或其他约定形态并记录能力差异。"),
        ("扩展可安装：", "通过签名模块或受控插件，在首版已交付的能力范围内增加特殊能力；配置先验证审批，出现问题可回退。插件默认没有网络、文件、密钥和业务数据权限，需要哪些能力必须先声明，经审批后按最小权限授予。"),
    ]
    for lead, rest in customization_items:
        add_bullet(doc, lead + rest, bullet_num_id, bold_lead=lead, compact=True)

    add_heading(doc, "5. Excel、文档与外部连接", 1, page_break_before=True)

    add_heading(doc, "Excel 与批量数据", 2)
    add_bullet(doc, "首版提供 Excel 导入与导出，用于批量建档、批量录入和把查询结果带出系统，四端能力一致。", bullet_num_id)
    add_bullet(doc, "为了安全，不执行 VBA、任意宏或第三方 Excel 加载项。", bullet_num_id)
    add_bullet(doc, "首版不提供 Excel 或 WPS 加载项，也不提供在表格中实时查询与提交、复杂公式、数据透视和条件格式。", bullet_num_id)

    add_heading(doc, "合同、附件与档案", 2)
    add_bullet(doc, "合同与单据附件支持模板套用、批注、版本与版本比较；DOCX 提供下载与查看。", bullet_num_id)
    add_bullet(doc, "PDF 支持查看、下载、批注、电子签章和归档校验；合同、发票、会计凭证和审计证据保存在经认证的不可篡改存储中。", bullet_num_id)
    add_bullet(doc, "大文件支持分片上传、断点续传、类型识别、恶意内容检查和权限控制。", bullet_num_id)
    add_bullet(doc, "移动端对模板套用、版本比较和 PDF 签章只提供查看与只读批注，写入操作在电脑端完成。", bullet_num_id)
    add_bullet(doc, "首版不提供文档在线编辑、修订与多人协作，也不提供 OCR、PDF/OFD 遮盖与格式转换、CAD 预览。", bullet_num_id)

    add_heading(doc, "接口与外部连接", 2)
    add_body(
        doc,
        "首版随正式版交付的成品连接器只有电子签章一类：合同审批通过后发起签署，回传签署结果与带签章的合同文件，用印留痕并归入合同附件与审计。实际启用需完成签章机构准入，并按客户逐项联调。",
    )
    add_bullet(doc, "银行与税务侧首版不做系统对接：到款、付款与发票开具都在系统内登记，可人工录入或批量导入，闭环不依赖银企直连与税务平台。", bullet_num_id)
    add_bullet(doc, "平台提供 REST 接口、OpenAPI 说明、批量接口、Webhook、业务事件，以及 CSV、Excel、XML 文件导入导出，供企业按需自行对接。", bullet_num_id)
    add_bullet(doc, "对外调用失败时支持超时、退避重试、熔断、死信队列和人工修复，不让问题悄悄丢失。", bullet_num_id)

    add_heading(doc, "6. 四端使用、私有部署与安全", 1, page_break_before=True)

    add_heading(doc, "Windows、macOS、iOS、Android", 2)
    add_body(
        doc,
        "四个平台使用同一套数据、权限和业务规则：同一操作在任一端发起、审批或查询，产生的记录、校验、审计和结果相同；界面按屏幕和使用场景重排，不要求四端外观一致。电脑端适合批量操作、复杂报表和系统配置，手机和平板适合审批、查询、扫码和现场记录。",
    )
    mobile_scope_items = [
        ("移动端完整使用：", "库存台账与收发扫码、售后工单与设备台账、审批待办与站内通知。"),
        ("移动端简化使用：", "客户档案与客户 360 查询、合同条款与电子签章、销售订单与履约、采购与供应商协同、项目任务与交付节点、主数据维护与审批、全文检索；业务对象、权限和流程结果不变，交互形态与单次批量规模受限。"),
        ("移动端只提供查看：", "财务过账与期末结账、到款与付款登记、发票申请与开具登记、报表与像素级打印、文档与附件协作、系统管理与配置发布；对应的写入操作在电脑端完成。"),
        ("移动端不承载：", "扩展插件与动态下发的扩展代码；移动端的设备能力只有相机扫码，随应用版本发布。"),
    ]
    for lead, rest in mobile_scope_items:
        add_bullet(doc, lead + rest, bullet_num_id, bold_lead=lead, compact=True)
    add_bullet(doc, "所有业务写入经中心校验后生效。网络中断期间客户端只能保存本地草稿，恢复连接后由中心重新校验并提交，不在本地产生正式业务记录。首版不提供断网独立运行。", bullet_num_id)
    add_bullet(doc, "合同生效、付款、开票、财务过账、结账和敏感数据导出属高风险操作，必须重新认证后才能提交并进入审批，审批人不得与发起人为同一人，四端口径一致。", bullet_num_id)

    add_heading(doc, "企业自己选择部署位置", 2)
    add_bullet(doc, "可部署在企业机房、私有云，或阿里云、腾讯云、华为云等企业自己的国内云环境。", bullet_num_id)
    add_bullet(doc, "首版核心交易数据库只支持 PostgreSQL 16，可自建，也可部署在企业自有云环境中，版本须在认证版本范围内。", bullet_num_id)
    add_bullet(doc, "企业已有的 Oracle、SQL Server、MySQL 可作为外部数据源接入，不作为核心交易数据库。", bullet_num_id)
    add_bullet(doc, "对象存储、密钥服务等配套组件既可自建，也可连接已有的合规服务。", bullet_num_id)
    add_bullet(doc, "生产数据、备份、日志和索引限定在中国大陆境内。", bullet_num_id)

    add_heading(doc, "高保密设计", 2)
    security_items = [
        ("权限细：", "可以控制到法人、部门、岗位、项目、客户、单条记录和单个字段；策略默认拒绝，不设置全能超级管理员。"),
        ("职责分开：", "系统、数据、安全、审计和密钥管理员相互制约；申请人不可自审，审批链不可越权跳过。"),
        ("全程加密：", "传输、数据库、附件、备份和设备本地缓存均受加密与密钥管理保护，每个法人使用独立的密钥域。"),
        ("控制外泄：", "敏感字段按经批准的清单脱敏；电脑端与移动端强制执行水印、导出审批、打印阻断、剪贴板拦截和分享控制，已下载文件失效在装有原生插件的电脑端和满足设备合规要求的移动端强制执行；浏览器门户端强制执行脱敏投影、水印、导出审批和操作审计，打印阻断、剪贴板拦截与已下载文件失效受浏览器能力限制，为尽力而为，不作承诺。"),
        ("完整审计：", "关键查看、修改、审批、导出和系统调用留下防篡改证据，审计记录只追加、可逐条验证。"),
        ("持续可用：", "正式生产采用单区域三故障域高可用部署，并配合境内异地不可变备份和定期恢复演练。"),
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
            ["销售", "客户资料分散、交期不清、回款难追", "客户全景、合同与订单进度、开票与到款状态、信用额度校验"],
            ["采购", "合同要买什么不清楚、付款进度靠问", "按合同下达的采购订单与分批订货、供应商档案、付款申请与进度"],
            ["仓库", "账实不符、批次不清、出入库靠纸", "实时库存台账、扫码收发、批次与序列号追溯"],
            ["财务", "重复录入、收付款对不上、凭证来源不清", "应收应付台账、分次到款付款核销、按业务事件生成的凭证与账簿"],
            ["售后", "找不到历史、工单没人跟", "关联订单、合同、产品、批次、设备和保修的工单，以及客户服务历史"],
            ["管理层", "报表滞后、口径不一、无法看到全局", "合同与发票申请的审批入口，收入、成本、交付、利润的经营看板"],
        ],
        [1500, 3770, 4090],
    )

    add_heading(doc, "8. 最关键的产品定位", 1)
    add_body(
        doc,
        "首版要解决的是一件具体的事：一笔业务从销售建单和合同审批开始，经采购订货、收货入库、发票申请与开具登记、到款与付款登记、交付确认和售后工单，一直走到管理层看到收入、成本、交付和利润，全过程在同一套系统内完成，不依赖外部系统或线下台账。",
    )
    add_body(
        doc,
        "首版不追求覆盖所有业务领域。制造、企业运营、电商与计费、客户与经销商门户、智能能力，以及银行和税务的系统对接属于后续版本，首版不交付。企业之间的差异通过配置、低代码、签名模块和受控插件实现，不为每个客户长期复制一套无法升级的代码。",
    )
    add_body(
        doc,
        "核心后台采用 Rust 构建，强调稳定、安全和长期可维护；每个客户拥有独立部署、独立数据、独立密钥和独立升级通道。",
    )

    add_callout(
        doc,
        "首版目标：让合同、订单、货物、资金、发票和服务记录在同一套可信记录中衔接，管理层随时看到同一口径的经营结果。",
        label="产品价值",
    )

    add_heading(doc, "9. 版本与适用范围说明", 1, page_break_before=True)
    add_body(
        doc,
        "本节说明本文档的效力范围。产品处于分阶段研发过程中，文中所列功能不代表当前均已可用，实际交付内容以正式发布版本、合同和验收清单为准。",
    )
    scope_items = [
        ("阶段与版本：", "产品处于总体设计与分阶段研发阶段，不设置固定的发布日期；首版范围内的全部模块完成并通过验收后才对外发布首版。"),
        ("交付依据：", "每次交付的模块范围、版本和验收标准以双方签署的合同与验收清单为准，本文档不构成功能交付承诺。"),
        ("首版范围：", "首版是以合同为中心的销售、采购、财务与工单闭环：合同（含续签与到期提醒）、订单与客户信用额度校验、采购与供应商、库存台账与存货计价、成本归集、售后工单与设备台账、应收应付与简易总账、发票申请与开具登记、报表与经营看板，以及权限与定制能力；对外协同只有供应商门户，成品连接器只有电子签章。"),
        ("后续版本：", "制造与生产、企业运营与治理、电商与计费、本地 AI 与 OCR、MCP、向量检索与知识图谱、边缘节点与断网运行、工业接入与物联网、客户门户与经销商门户与员工门户、银企直连与税务平台对接，以及四种国产数据库的适配，首版均不交付；相关条目已逐条登记，后续版本可按登记恢复。"),
        ("语言与币种：", "首版仅支持简体中文、人民币和中国标准时间，不支持多币种、外汇、进出口、报关、信用证和产品内容多语言。"),
        ("联网要求：", "全部业务写入由中心完成，首版不提供断网独立运行；网络中断期间客户端只保存本地草稿，恢复连接后由中心校验通过才写入。"),
        ("人工确认：", "合同生效、付款、开票、财务过账、结账和敏感数据导出属高风险操作，必须重新认证后才能提交并进入审批，审批链不可越权跳过，审批人不得与发起人为同一人。"),
        ("数据库：", "首版核心交易数据库只支持并只认证 PostgreSQL 16；其他数据库产品只能作为外部数据源。"),
        ("外部连接：", "首版成品连接器只有电子签章一类，启用需完成机构准入并按客户逐项联调；银行与税务侧首版不做系统对接，到款、付款与发票开具在系统内登记，可人工录入或批量导入。"),
        ("性能与容量：", "响应时间、并发规模、恢复时间等指标以合同约定的基线环境和验收清单为准，本文档不单独作出量化承诺。"),
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

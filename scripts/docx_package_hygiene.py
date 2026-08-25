#!/usr/bin/env python3
"""Normalize DOCX package metadata and inherited typography.

The generated documents use explicit Windows/Word fonts and language tags.
This final package pass also cleans unsupported/unused style parts retained by
python-docx and removes cached application statistics that are stale as soon as
pagination occurs in a different Word-compatible renderer.
"""

import os
from pathlib import Path
from zipfile import ZipFile

from lxml import etree


WORD_NS = "http://schemas.openxmlformats.org/wordprocessingml/2006/main"
EXTENDED_PROPERTIES_NS = (
    "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"
)
W = f"{{{WORD_NS}}}"
EP = f"{{{EXTENDED_PROPERTIES_NS}}}"

LATIN_FONT = "Arial"
CHINESE_FONT = "Microsoft YaHei"
WESTERN_LANG = "en-US"
EAST_ASIA_LANG = "zh-CN"
BIDI_LANG = "ar-SA"

THEME_FONT_ATTRIBUTES = (
    "asciiTheme",
    "hAnsiTheme",
    "eastAsiaTheme",
    "cstheme",
)
CACHED_STATISTICS = {
    "TotalTime",
    "Pages",
    "Words",
    "Characters",
    "Lines",
    "Paragraphs",
    "CharactersWithSpaces",
}


def _serialize(root) -> bytes:
    return etree.tostring(
        root,
        encoding="UTF-8",
        xml_declaration=True,
        standalone=True,
    )


def _normalize_wordprocessing_xml(data: bytes) -> bytes:
    root = etree.fromstring(data)
    changed = False

    for fonts in root.iter(f"{W}rFonts"):
        for attribute in THEME_FONT_ATTRIBUTES:
            key = f"{W}{attribute}"
            if key in fonts.attrib:
                del fonts.attrib[key]
                changed = True
        hint = f"{W}hint"
        if hint in fonts.attrib:
            del fonts.attrib[hint]
            changed = True
        for attribute, value in (
            ("ascii", LATIN_FONT),
            ("hAnsi", LATIN_FONT),
            ("eastAsia", CHINESE_FONT),
            ("cs", LATIN_FONT),
        ):
            key = f"{W}{attribute}"
            if fonts.get(key) != value:
                fonts.set(key, value)
                changed = True

    for lang in root.iter(f"{W}lang"):
        for attribute, value in (
            ("val", WESTERN_LANG),
            ("eastAsia", EAST_ASIA_LANG),
            ("bidi", BIDI_LANG),
        ):
            key = f"{W}{attribute}"
            if lang.get(key) != value:
                lang.set(key, value)
                changed = True

    return _serialize(root) if changed else data


def _remove_cached_statistics(data: bytes) -> bytes:
    root = etree.fromstring(data)
    changed = False
    for name in CACHED_STATISTICS:
        element = root.find(f"{EP}{name}")
        if element is not None:
            root.remove(element)
            changed = True
    return _serialize(root) if changed else data


def sanitize_docx_package(path: Path) -> None:
    """Atomically sanitize one DOCX after python-docx saves it."""
    path = Path(path)
    temp_path = path.with_name(f".{path.name}.hygiene.tmp")
    try:
        with ZipFile(path, "r") as source, ZipFile(temp_path, "w") as target:
            for info in source.infolist():
                data = source.read(info.filename)
                if info.filename.startswith("word/") and info.filename.endswith(".xml"):
                    data = _normalize_wordprocessing_xml(data)
                elif info.filename == "docProps/app.xml":
                    data = _remove_cached_statistics(data)
                target.writestr(info, data)
        os.replace(temp_path, path)
    finally:
        temp_path.unlink(missing_ok=True)

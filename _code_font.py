"""
The font used for code in the PDFs ("DefCode").

Why a special font:
  A code line copied from a PDF only comes out in one piece when the whole line
  was drawn as one text run. Chrome starts a new run whenever the font changes,
  so a line that mixes ASCII and Japanese (a comment, or a string like
  println!("こんにちは")) must be drawn with ONE font that has both.
  The Japanese fonts that come with Windows draw the backslash as a yen sign
  (C:¥Users...), which is confusing when typing commands, so we make our own:

    ASCII and symbols : Cascadia Mono  (comes with Windows, SIL Open Font License)
    Japanese          : Noto Sans JP   (comes with Windows, SIL Open Font License)

  Japanese characters are widened to exactly two ASCII columns, so text that is
  lined up in the source stays lined up in the PDF.

The merged font is built once into a cache folder outside the repository
(it is not committed); each PDF gets a small subset with only the characters
its code uses (see font_face_css). Needs `pip install fonttools`.
"""
from __future__ import annotations

import base64
import io
import os
import tempfile
from pathlib import Path

from fontTools import subset
from fontTools.ttLib import TTFont, newTable
from fontTools.ttLib.scaleUpem import scale_upem
from fontTools.ttLib.tables import ttProgram
from fontTools.ttLib.tables._c_m_a_p import CmapSubtable
from fontTools.ttLib.tables._g_l_y_f import Glyph, GlyphComponent
from fontTools.varLib import instancer

FAMILY = "DefCode"
WINDOWS_FONTS = Path(os.environ.get("WINDIR", r"C:\Windows")) / "Fonts"
ASCII_FONT = WINDOWS_FONTS / "CascadiaMono.ttf"
JAPANESE_FONT = WINDOWS_FONTS / "NotoSansJP-VF.ttf"
CACHE_DIR = Path(os.environ.get("LOCALAPPDATA", tempfile.gettempdir())) / "def-pdf-build"
CACHE_FONT = CACHE_DIR / f"{FAMILY}-Regular-v1.ttf"


def _static(path: Path) -> TTFont:
    font = TTFont(path)
    if "fvar" in font:
        font = instancer.instantiateVariableFont(font, {"wght": 400})
    return font


def _empty_program() -> ttProgram.Program:
    program = ttProgram.Program()
    program.fromBytecode(b"")
    return program


def build_font(out_path: Path = CACHE_FONT) -> Path:
    """Merge the two fonts into one TrueType font and save it."""
    base = _static(ASCII_FONT)
    jp = _static(JAPANESE_FONT)
    upm = base["head"].unitsPerEm
    if jp["head"].unitsPerEm != upm:
        scale_upem(jp, upm)

    base_cmap = base.getBestCmap()
    half = base["hmtx"][base_cmap[ord("a")]][0]   # width of one ASCII column
    jp_cmap = jp.getBestCmap()
    jp_glyf, glyf, hmtx = jp["glyf"], base["glyf"], base["hmtx"]

    # glyphs that other Japanese glyphs reuse as parts must not be moved in place
    used_as_part = set()
    for name in jp.getGlyphOrder():
        g = jp_glyf[name]
        if g.isComposite():
            used_as_part.update(c.glyphName for c in g.components)

    copied: dict[str, str] = {}

    def copy(name: str) -> str:
        if name in copied:
            return copied[name]
        g = jp_glyf[name]
        new_name = f"jp.{name}"
        if g.isComposite():
            for comp in g.components:
                comp.glyphName = copy(comp.glyphName)
        if hasattr(g, "program"):
            g.program = _empty_program()
        glyf[new_name] = g
        hmtx[new_name] = jp["hmtx"][name]
        copied[name] = new_name
        return new_name

    added: dict[int, str] = {}
    for code, name in sorted(jp_cmap.items()):
        if code in base_cmap:
            continue
        new_name = copy(name)
        advance, lsb = hmtx[new_name]
        target = 2 * half if advance >= upm * 0.9 else half if advance <= upm * 0.55 else advance
        shift = (target - advance) // 2
        if shift:
            if name in used_as_part or glyf[new_name].isComposite():
                # draw the glyph through a wrapper that moves it right
                wrapper = Glyph()
                comp = GlyphComponent()
                comp.glyphName, comp.x, comp.y, comp.flags = new_name, shift, 0, 0x0004
                wrapper.components = [comp]
                wrapper.numberOfContours = -1
                new_name = f"jpw.{name}"
                glyf[new_name] = wrapper
            elif glyf[new_name].numberOfContours > 0:
                glyf[new_name].coordinates.translate((shift, 0))
            hmtx[new_name] = (target, lsb + shift)
        elif target != advance:
            hmtx[new_name] = (target, lsb)
        added[code] = new_name

    base.setGlyphOrder(glyf.glyphOrder)
    for name in glyf.glyphOrder:   # recompute the bounding boxes of moved and wrapped glyphs
        glyf[name].recalcBounds(glyf)

    # character map: BMP table(s) plus a full table for characters beyond U+FFFF
    full = dict(base_cmap)
    full.update(added)
    tables = []
    for t in base["cmap"].tables:
        if t.isUnicode() and t.format == 4:
            t.cmap = {c: n for c, n in full.items() if c <= 0xFFFF}
            tables.append(t)
    t12 = CmapSubtable.newSubtable(12)
    t12.platformID, t12.platEncID, t12.language, t12.cmap = 3, 10, 0, full
    tables.append(t12)
    base["cmap"].tables = tables

    # names: a new family name (the original names are reserved), keep both licences
    notice = (
        f"{FAMILY} is a merge of Cascadia Mono (Copyright (c) Microsoft Corporation) and "
        "Noto Sans JP (Copyright Google LLC / Adobe), both under the SIL Open Font License 1.1, "
        "made only for printing the code in these PDF materials."
    )
    name_table = newTable("name")
    name_table.names = []
    for name_id, value in {
        0: notice, 1: FAMILY, 2: "Regular", 3: f"{FAMILY}-Regular-v1", 4: f"{FAMILY} Regular",
        5: "Version 1.0", 6: f"{FAMILY}-Regular", 13: "SIL Open Font License 1.1",
        14: "https://openfontlicense.org",
    }.items():
        name_table.setName(value, name_id, 3, 1, 0x409)
    base["name"] = name_table
    base["OS/2"].fsType = 0
    base["post"].formatType = 3.0
    for tag in ("DSIG", "STAT", "fvar", "gvar", "avar", "HVAR", "MVAR", "cvar"):
        if tag in base:
            del base[tag]

    out_path.parent.mkdir(parents=True, exist_ok=True)
    base.save(out_path)
    return out_path


def font_face_css(text: str) -> str:
    """An @font-face rule with a subset of DefCode that covers `text` (embedded as data: URL)."""
    if not CACHE_FONT.exists():
        print(f"  building the code font: {CACHE_FONT}")
        build_font()
    options = subset.Options()
    options.hinting = False
    options.name_IDs = ["*"]
    options.notdef_outline = True
    options.layout_features = []
    font = TTFont(CACHE_FONT)
    subsetter = subset.Subsetter(options)
    subsetter.populate(text=text + " ")
    subsetter.subset(font)
    buf = io.BytesIO()
    font.save(buf)
    data = base64.b64encode(buf.getvalue()).decode("ascii")
    return (
        "@font-face { font-family: \"%s\"; font-weight: 400; font-style: normal;"
        " src: url(data:font/ttf;base64,%s) format(\"truetype\"); }\n" % (FAMILY, data)
    )


if __name__ == "__main__":
    print(build_font())

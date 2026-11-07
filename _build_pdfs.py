"""
Convert all chapter Markdown files in this directory into nicely-formatted PDFs.

Pipeline:
  .md  --(python-markdown + extensions)-->  .html (full standalone)
       --(Chrome headless --print-to-pdf)--> .pdf

The inline HTML diagrams in the source MD files are preserved as-is, so they
render as real visual diagrams in the PDF. We add:
  - Japanese/code font stack via webfont CSS
  - Code-block syntax highlighting via Pygments
  - Comfortable page layout (margins, line-height, page-break behaviour)
"""
from __future__ import annotations

import html
import json
import os
import re
import sys
import unicodedata
import shutil
import subprocess
import tempfile
import time
from pathlib import Path
from urllib.parse import quote

import markdown

from _code_font import FAMILY as CODE_FONT, font_face_css

ROOT = Path(__file__).resolve().parent

# --------------------------- Browser detection -------------------------------
CHROME_CANDIDATES = [
    r"C:\Program Files\Google\Chrome\Application\chrome.exe",
    r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
    r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
    r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
]
CHROME = next((p for p in CHROME_CANDIDATES if Path(p).exists()), None)
if CHROME is None:
    print("ERROR: Chrome / Edge not found.", file=sys.stderr)
    sys.exit(1)
print(f"Using browser: {CHROME}")

# --------------------------- HTML template -----------------------------------
# Embedded CSS keeps the diagrams readable and adds typography sensible for
# Japanese reading + code listings.
HTML_TEMPLATE = r"""<!doctype html>
<html lang="ja">
<head>
<meta charset="utf-8">
<title>{title}</title>
<style>
{code_font_face}
@page {{
  size: A4;
  margin: 18mm 16mm 20mm 16mm;
  @bottom-right {{
    content: counter(page);
    font-size: 10px;
    color: #64748b;
  }}
}}

* {{ box-sizing: border-box; }}

html, body {{
  font-family: "Yu Gothic UI", "Yu Gothic", "Meiryo", "Hiragino Sans",
               "Noto Sans JP", "Segoe UI", system-ui, sans-serif;
  font-size: 10.5pt;
  line-height: 1.75;
  color: #1e293b;
  background: #ffffff;
  -webkit-print-color-adjust: exact;
  print-color-adjust: exact;
}}

h1, h2, h3, h4, h5, h6 {{
  color: #0f172a;
  font-weight: 700;
  line-height: 1.4;
  margin: 1.4em 0 0.6em;
  page-break-after: avoid;
}}
h1 {{
  font-size: 22pt;
  border-bottom: 3px solid #1e40af;
  padding-bottom: 0.3em;
  margin-top: 0.4em;
  page-break-before: auto;
}}
h2 {{
  font-size: 16pt;
  border-bottom: 1.5px solid #cbd5e1;
  padding-bottom: 0.25em;
  margin-top: 1.6em;
}}
h3 {{ font-size: 13pt; color: #1e40af; }}
h4 {{ font-size: 12pt; color: #334155; }}

p {{ margin: 0.5em 0 0.8em; }}

a {{ color: #1d4ed8; text-decoration: none; }}
a:hover {{ text-decoration: underline; }}

ul, ol {{ margin: 0.4em 0 0.9em; padding-left: 1.6em; }}
li {{ margin: 0.2em 0; }}

blockquote {{
  border-left: 4px solid #3b82f6;
  background: #eff6ff;
  margin: 0.8em 0;
  padding: 0.6em 1em;
  color: #1e3a8a;
  border-radius: 0 6px 6px 0;
  page-break-inside: avoid;
}}
blockquote p {{ margin: 0.3em 0; }}
blockquote strong {{ color: #1e40af; }}

code {{
  /* DefCode (see _code_font.py) has ASCII and Japanese in ONE font, so a code
     line is drawn as one text run and copies as one line in every PDF viewer. */
  font-family: "{code_font}", "Consolas", monospace;
  font-size: 0.9em;
  background: #f1f5f9;
  color: #be123c;
  padding: 0.12em 0.36em;
  border-radius: 4px;
  /* Long inline code (file paths / URLs) must break instead of overflowing. */
  word-break: break-word;
  overflow-wrap: anywhere;
}}

pre {{
  background: #0f172a;
  color: #e2e8f0;
  border-radius: 8px;
  padding: 12px 14px;
  /* In a PDF there is no horizontal scrolling, so long lines must wrap
     instead of being clipped off the right edge of the page. */
  overflow-x: hidden;
  overflow-y: hidden;
  white-space: pre-wrap;
  word-break: break-word;
  overflow-wrap: anywhere;
  font-size: 9.2pt;
  line-height: 1.5;
  margin: 0.8em 0;
  page-break-inside: avoid;
}}
pre code {{
  background: transparent;
  color: inherit;
  padding: 0;
  font-size: inherit;
  white-space: inherit;
  word-break: inherit;
  overflow-wrap: inherit;
}}

/* The codehilite extension wraps highlighted blocks in a scrolling div; make
   that wrap too so nothing is clipped in the PDF. */
.codehilite {{
  overflow-x: hidden;
  /* overflow-x: hidden alone turns overflow-y into auto, and a glyph from a
     fallback font that is a hair too tall then shows a vertical scrollbar. */
  overflow-y: hidden;
}}
.codehilite pre {{
  white-space: pre-wrap;
  word-break: break-word;
  overflow-wrap: anywhere;
}}

/* No per-token colours: every coloured token became its own text run (and its
   own tag) in the PDF, and some viewers then pasted each token on its own line.
   Only whole comment lines get a colour, so each line is still one run. */
pre .cmt {{ color: #94a3b8; }}

table {{
  border-collapse: collapse;
  width: 100%;
  margin: 0.8em 0;
  font-size: 9.6pt;
  page-break-inside: avoid;
}}
th, td {{
  border: 1px solid #e2e8f0;
  padding: 6px 9px;
  text-align: left;
  vertical-align: top;
}}
th {{
  background: #f1f5f9;
  color: #0f172a;
  font-weight: 700;
}}
tr:nth-child(even) td {{ background: #f8fafc; }}

img {{ max-width: 100%; height: auto; }}

hr {{
  border: none;
  border-top: 1px dashed #cbd5e1;
  margin: 1.6em 0;
}}

/* Diagram blocks (the inline-styled <div>s in the source) */
div[style] {{
  page-break-inside: avoid;
}}

/* Tighten spacing right after headings */
h1 + p, h2 + p, h3 + p, h4 + p {{ margin-top: 0.2em; }}

/* Avoid ugly page breaks inside list items / table rows */
li, tr {{ page-break-inside: avoid; }}
</style>
</head>
<body>
{body}
</body>
</html>
"""

EXTENSIONS = [
    "extra",        # tables, fenced_code, attr_list, footnotes, def_list, abbr, md_in_html
    "sane_lists",
    "toc",
    "codehilite",   # Pygments-based highlighting
]
EXTENSION_CONFIGS = {
    "codehilite": {"guess_lang": False, "noclasses": False},
}

# ---------------------- Copy-friendly code blocks ----------------------------
# Text copied from a PDF keeps every visual line break, so a code line that
# wraps at the page edge becomes two lines when pasted. Each block is therefore
# shrunk until its longest line fits on one line (down to a readable minimum)
# and printed without wrapping. Only blocks that still do not fit wrap.
CODE_WIDTH_PT = 450.0      # usable text width inside a <pre> (A4, with margins/padding/list indent)
CODE_CHAR_EM = 0.586       # DefCode advance width (1200/2048 em); a wide (Japanese) char counts as 2
CODE_MAX_PT = 9.2
CODE_MIN_PT = 5.6        # DefCode at 5.6pt is as narrow as Consolas was at 6pt
CODE_WRAP_PT = 7.5         # size for blocks that must wrap anyway (long logs)

PRE_RE = re.compile(r"<pre([^>]*)>(.*?)</pre>", re.S)
CODE_TEXT_RE = re.compile(r"<code[^>]*>(.*?)</code>", re.S)
# a line that is only a comment: "# ...", "// ...", "-- ...", "<!-- ...", "REM ..." (not "#[derive]" or "#!")
COMMENT_LINE_RE = re.compile(r"^(\s*)((?:#(?=\s|$)|//|-- |<!--|REM\s|::\s).*)$")


def plain_code(inner: str) -> str:
    """Remove the per-token highlight spans and colour whole comment lines only."""
    inner = re.sub(r"<span[^>]*>|</span>", "", inner)
    m = re.fullmatch(r"(\s*<code[^>]*>)(.*?)(</code>\s*)", inner, re.S)
    head, code, tail = (m.group(1), m.group(2), m.group(3)) if m else ("", inner, "")
    out = []
    for line in code.split("\n"):
        if "<" not in line and COMMENT_LINE_RE.match(html.unescape(line)):
            indent = line[: len(line) - len(line.lstrip())]
            line = f'{indent}<span class="cmt">{line.lstrip()}</span>'
        out.append(line)
    return head + "\n".join(out) + tail


def display_width(line: str) -> int:
    return sum(2 if unicodedata.east_asian_width(c) in "WF" else 1 for c in line.replace("\t", "    "))


def copy_friendly_pre(match: re.Match) -> tuple[str, str]:
    """The <pre> for the PDF, and its text (as printed: an empty line holds one space)."""
    attrs, inner = match.group(1), plain_code(match.group(2))
    # An empty line has no glyph, so it vanished when copied. A single space keeps it.
    inner = re.sub(r"\n(?=\n)", "\n ", inner)
    text = html.unescape(re.sub(r"<[^>]+>", "", inner))
    cols = max((display_width(l) for l in text.split("\n")), default=0)
    size = CODE_MAX_PT if cols == 0 else min(CODE_MAX_PT, CODE_WIDTH_PT / (CODE_CHAR_EM * cols))
    if size >= CODE_MIN_PT:
        style = f"font-size:{size:.2f}pt;white-space:pre;word-break:normal;overflow-wrap:normal"
    else:
        style = f"font-size:{CODE_WRAP_PT}pt"
    return f'<pre{attrs} style="{style}">{inner}</pre>', text


# ---------------------- Selecting and copying in the PDF ----------------------
# In VS Code's PDF preview (the vscode-pdf extension, PDF.js 2.10), selecting a code block with the mouse
# and pressing Ctrl+C copies it with its line breaks and indentation (an empty line holds one space),
# once the PDF has no tags and no page numbers in its text (see copy_friendly_text). It cannot give back:
#   - characters that PDF.js always turns into others when it reads the text of a PDF ("…" -> "...",
#     "℃" -> "°C", "µ" -> "μ", "Ⅱ" -> "II" …): PDFJS_NORMALIZED is the keys of its NormalizedUnicodes table;
#   - spaces other than the plain space (a tab or a full-width space comes back as a plain space);
#   - an emoji made of several characters (the zero width joiner, a variation selector, a skin tone …);
#   - "～": the code font draws it with the glyph of "〜", so it comes back as "〜";
#   - a long line that the PDF wraps: each part comes back as a line of its own.
# Such a block gets a short note beside its "コピー" link (which copies it exactly). Which lines wrap is
# only known once Chrome has laid the page out, so the page itself checks that (PDF_WRAP_JS) and then
# shows the note for wrapped lines (data-wrap-note).
PDFJS_NORMALIZED = (
    (0x00A8, 0x00A8), (0x00AF, 0x00AF), (0x00B4, 0x00B5), (0x00B8, 0x00B8), (0x0132, 0x0133), (0x013F, 0x0140),
    (0x0149, 0x0149), (0x017F, 0x017F), (0x01C4, 0x01CC), (0x01F1, 0x01F3), (0x02D8, 0x02DD), (0x037A, 0x037A),
    (0x0384, 0x0384), (0x03D0, 0x03D2), (0x03D5, 0x03D6), (0x03F0, 0x03F2), (0x03F4, 0x03F5), (0x03F9, 0x03F9),
    (0x0587, 0x0587), (0x0675, 0x0678), (0x0E33, 0x0E33), (0x0EB3, 0x0EB3), (0x0EDC, 0x0EDD), (0x0F77, 0x0F77),
    (0x0F79, 0x0F79), (0x1E9A, 0x1E9A), (0x1FBD, 0x1FBD), (0x1FBF, 0x1FC0), (0x1FFE, 0x1FFE), (0x2002, 0x2006),
    (0x2008, 0x200A), (0x2017, 0x2017), (0x2024, 0x2026), (0x2033, 0x2034), (0x2036, 0x2037), (0x203C, 0x203C),
    (0x203E, 0x203E), (0x2047, 0x2049), (0x2057, 0x2057), (0x205F, 0x205F), (0x20A8, 0x20A8), (0x2100, 0x2101),
    (0x2103, 0x2103), (0x2105, 0x2107), (0x2109, 0x2109), (0x2116, 0x2116), (0x2121, 0x2121), (0x2135, 0x2138),
    (0x213B, 0x213B), (0x2160, 0x217F), (0x222C, 0x222D), (0x222F, 0x2230), (0x2474, 0x24B5), (0x2A0C, 0x2A0C),
    (0x2A74, 0x2A76), (0x2E9F, 0x2E9F), (0x2EF3, 0x2EF3), (0x2F00, 0x2FD5), (0x3036, 0x3036), (0x3038, 0x303A),
    (0x309B, 0x309C), (0x3131, 0x318E), (0x3200, 0x321E), (0x3220, 0x3243), (0x32C0, 0x32CB), (0x3358, 0x3370),
    (0x33E0, 0x33FE), (0xFB00, 0xFB06), (0xFB13, 0xFB17), (0xFB4F, 0xFBB1), (0xFBD3, 0xFD3D), (0xFD50, 0xFD8F),
    (0xFD92, 0xFDC7), (0xFE49, 0xFE4F), (0xFE80, 0xFEFC),
)
SHARED_GLYPH_CHARS = "～"


def _copy_changes(c: str) -> str | None:
    """How the character is named in the note, when selecting and copying it in the PDF gives something else."""
    n = ord(c)
    if c == "\t":
        return "タブ"
    if c == "　":
        return "全角の空白"
    if c not in " \n" and (c.isspace() or unicodedata.category(c) == "Zs"):
        return "特別な空白"
    # an emoji made of several characters (joined with U+200D, with a variation selector, a skin tone,
    # a keycap, or a flag) is drawn as one glyph, which comes back as other characters
    if c in "‍⃣" or 0xFE00 <= n <= 0xFE0F or 0x1F3FB <= n <= 0x1F3FF or 0x1F1E6 <= n <= 0x1F1FF:
        return "絵文字"
    if c in SHARED_GLYPH_CHARS or any(a <= n <= b for a, b in PDFJS_NORMALIZED):
        return f"「{c}」"
    return None


def selection_copy_note(text: str, wraps: bool) -> str | None:
    """The note beside the "コピー" link of a block that selecting and copying in the PDF cannot give back."""
    labels = list(dict.fromkeys(l for l in map(_copy_changes, text) if l))
    shown = "・".join(labels[:3]) + ("など" if len(labels) > 3 else "")
    if labels and wraps:
        return f"なぞってコピーすると{shown}が別の文字になり、長い行も分かれます。正しくは →"
    if labels:
        return f"なぞってコピーすると{shown}が別の文字になります。正しくは →"
    if wraps:
        return "長い行を折り返しているので、なぞってコピーすると行が分かれます。正しくは →"
    return None


# ---------------------- Copy buttons (HTML edition) ---------------------------
# Chrome's PDF viewer (pdfium) always drops the indentation and the empty lines
# of copied text, whatever the PDF contains. So every code block gets:
#   - in the HTML edition (<folder>/html/<name>.html): a "コピー" button that
#     copies the exact source text (one click), and an id "code-N";
#   - in the PDF: a small "コピー" link above the block that opens the HTML
#     edition at that block (the link is made relative after printing, so it
#     works wherever the folder is copied to).

PDF_COPY_CSS = """
.code-head { text-align: right; margin: 0.6em 0 -0.55em; page-break-after: avoid; line-height: 1; }
.code-head a.copy-link {
  display: inline-block; font-size: 9pt; font-weight: 700; color: #ffffff; background: #2f80ed;
  border-radius: 4px; padding: 3px 12px; text-decoration: none;
}
.code-head .copy-note { font-size: 7.5pt; color: #b45309; margin-right: 6px; vertical-align: 1px; }
"""

# Runs in the page Chrome prints: a block whose lines take more rows on the page than the code has lines
# (a long line wrapped) shows its wrap note (see selection_copy_note).
PDF_WRAP_JS = """
<script>
document.fonts.ready.then(() => {
  for (const head of document.querySelectorAll(".code-head")) {
    const pre = head.nextElementSibling, note = head.querySelector(".copy-note");
    if (!pre || pre.tagName !== "PRE" || !note) continue;
    const lines = pre.textContent.replace(/\\n$/, "").split("\\n").length;
    const range = document.createRange();
    range.selectNodeContents(pre);
    const tops = [...range.getClientRects()].filter((r) => r.height > 0).map((r) => r.top).sort((a, b) => a - b);
    let rows = tops.length ? 1 : 0;
    for (let i = 1; i < tops.length; i++) if (tops[i] - tops[i - 1] > 3) rows++;
    if (rows > lines) note.textContent = note.dataset.wrapNote;
  }
});
</script>
"""

WEB_CSS = """
@media screen {
  body { max-width: 920px; margin: 0 auto; padding: 20px 20px 120px; font-size: 16px; }
  pre { white-space: pre !important; word-break: normal !important; overflow-wrap: normal !important;
        overflow-x: auto !important; font-size: 14px !important; }
  img { max-width: 100%; height: auto; }
}
.web-bar { display: flex; flex-wrap: wrap; gap: 8px 16px; align-items: center; font-size: 14px;
  padding: 8px 12px; margin-bottom: 16px; background: #eef2f7; border-radius: 8px; }
.web-bar a { color: #1e3a5f; }
.web-note { position: fixed; left: 50%; bottom: 16px; transform: translateX(-50%); z-index: 10; max-width: 90%;
  font-size: 15px; font-weight: 700; background: #fff7ed; border: 2px solid #f2994a; border-radius: 8px;
  padding: 10px 16px; margin: 0; box-shadow: 0 4px 16px rgba(0,0,0,0.15); }
.paste-note { position: fixed; left: 50%; bottom: 16px; transform: translateX(-50%); z-index: 11; width: 640px;
  max-width: calc(100% - 32px); box-sizing: border-box; font-size: 15px; line-height: 1.7; background: #ecfdf5;
  border: 2px solid #27ae60; border-radius: 8px; padding: 12px 40px 12px 16px; box-shadow: 0 4px 16px rgba(0,0,0,0.18); }
.paste-note[hidden] { display: none; }
.paste-note button { position: absolute; top: 4px; right: 6px; border: 0; background: transparent; font-size: 20px;
  line-height: 1; cursor: pointer; color: #1f2937; padding: 4px 6px; }
.code-wrap { position: relative; scroll-margin-top: 16px; }
.code-wrap pre { padding-top: 40px !important; }
.copy-btn { position: absolute; top: 8px; right: 8px; z-index: 2; font-size: 13px; font-weight: 700;
  padding: 4px 12px; border: 0; border-radius: 6px; background: #2f80ed; color: #fff; cursor: pointer;
  font-family: inherit; }
.copy-btn:hover { background: #1c64c8; }
.copy-btn.done { background: #27ae60; }
.code-wrap:target pre { outline: 4px solid #f2994a; outline-offset: 2px; }
.code-wrap:target .copy-btn { box-shadow: 0 0 0 4px #f2994a; }
/* syntax colours on screen (the PDF uses one colour so that each line copies as one piece) */
.codehilite .k, .codehilite .kd, .codehilite .kn, .codehilite .kr { color: #c084fc; }
.codehilite .s, .codehilite .s1, .codehilite .s2, .codehilite .sb { color: #fcd34d; }
.codehilite .c, .codehilite .c1, .codehilite .cm { color: #94a3b8; }
.codehilite .nf, .codehilite .nx { color: #60a5fa; }
.codehilite .nb { color: #34d399; }
.codehilite .mi, .codehilite .mf { color: #f472b6; }
.codehilite .nt { color: #f59e0b; }
.codehilite .na { color: #93c5fd; }
"""

WEB_JS = """
<script>
(function () {
  // The old way (a hidden text box + execCommand) works where the clipboard API is refused.
  function copyOld(text) {
    return new Promise(function (resolve, reject) {
      var ta = document.createElement("textarea");
      ta.value = text; ta.style.position = "fixed"; ta.style.top = "0"; ta.style.opacity = "0";
      document.body.appendChild(ta); ta.focus(); ta.select();
      var ok = false;
      try { ok = document.execCommand("copy"); } catch (e) { ok = false; }
      document.body.removeChild(ta);
      ok ? resolve() : reject();
    });
  }
  function copyText(text) {
    if (navigator.clipboard && window.isSecureContext) {
      return navigator.clipboard.writeText(text).catch(function () { return copyOld(text); });
    }
    return copyOld(text);
  }
  var pasteNote = document.getElementById("paste-note");
  var pasteTimer = null;
  function showPasteNote() {
    if (!pasteNote) { return; }
    var webNote = document.getElementById("web-note");
    if (webNote) { webNote.hidden = true; }
    pasteNote.hidden = false;
    clearTimeout(pasteTimer);
    pasteTimer = setTimeout(function () { pasteNote.hidden = true; }, 15000);
  }
  if (pasteNote) {
    pasteNote.querySelector("button").addEventListener("click", function () { pasteNote.hidden = true; });
  }
  document.querySelectorAll(".copy-btn").forEach(function (btn) {
    btn.addEventListener("click", function () {
      var pre = btn.parentElement.querySelector("pre");
      var text = pre.textContent.replace(/\\n$/, "");
      copyText(text).then(function () {
        btn.textContent = "コピーしました";
        btn.classList.add("done");
        setTimeout(function () { btn.textContent = "コピー"; btn.classList.remove("done"); }, 2000);
        if (text.indexOf("\\n") >= 0) { showPasteNote(); }
      }, function () {
        // could not write to the clipboard: select the code so that Ctrl+C copies it (lines are kept)
        var sel = window.getSelection();
        sel.removeAllRanges();
        var range = document.createRange();
        range.selectNodeContents(pre.querySelector("code") || pre);
        sel.addRange(range);
        btn.textContent = "選びました。Ctrl＋C を押してください";
      });
    });
  });
  if (/^#code-\\d+$/.test(location.hash)) {
    var note = document.getElementById("web-note");
    if (note) { note.hidden = false; }
  }
})();
</script>
"""


def dedent_fences(md_text: str) -> str:
    """A code fence written inside a numbered step (indented by spaces) is shown fine on GitHub, but
    Python-Markdown's fenced_code only reads fences at the start of the line: an indented one became a
    single line of inline code (newlines and indentation lost, no copy button). Move such blocks to the
    start of the line, keeping the indentation of the code itself. Fences shown *inside* another code
    block (a README.md printed as an example) are code, and are left alone."""
    lines = md_text.split("\n")
    out: list[str] = []
    outer = None          # the fence of a block that starts at the start of the line
    i = 0
    while i < len(lines):
        line = lines[i]
        if outer:
            out.append(line)
            if re.match(re.escape(outer[0]) + "{" + str(len(outer)) + r",}[ \t]*$", line):
                outer = None
            i += 1
            continue
        m = re.match(r"(```+|~~~+)", line)
        if m:
            outer = m.group(1)
            out.append(line)
            i += 1
            continue
        m = re.match(r"([ \t]*>[ \t]?)([ \t]*)(```+|~~~+)[ \t]*([\w+-]*)[ \t]*$", line)
        if m:
            # a fence inside a quote (> ```bash): fenced_code cannot see it, so the code became one line of
            # inline code. Write it as an indented code block inside the quote (":::lang" keeps the colours).
            quote, indent, fence, lang = m.groups()
            close = re.compile(r"[ \t]*>[ \t]?" + re.escape(indent) + re.escape(fence) + r"[ \t]*$")
            j = i + 1
            while j < len(lines) and not close.match(lines[j]) and re.match(r"[ \t]*>", lines[j]):
                j += 1
            if j < len(lines) and close.match(lines[j]):
                body = []
                for l in lines[i + 1:j]:
                    l = re.sub(r"^[ \t]*>[ \t]?", "", l)
                    body.append(l[len(indent):] if l.startswith(indent) else l.lstrip(" \t"))
                q = quote.rstrip()
                out.append(q)
                if lang and lang.lower() not in ("text", "txt", "plain"):
                    out.append(f"{q}     :::{lang}")
                out += [f"{q}     {l}" if l.strip() else q for l in body]
                out.append(q)
                i = j + 1
                continue
        m = re.match(r"([ \t]+)(```+|~~~+)(.*)$", line)
        if m:
            indent, fence, info = m.groups()
            close = re.compile(re.escape(indent) + re.escape(fence) + r"[ \t]*$")
            j = i + 1
            while j < len(lines) and not close.match(lines[j]):
                j += 1
            if j < len(lines):
                out += ["", fence + info]
                out += [l[len(indent):] if l.startswith(indent) else l.lstrip(" \t") for l in lines[i + 1:j]]
                out.append(fence)
                i = j + 1
                continue
        out.append(line)
        i += 1
    return "\n".join(out)


# Folders whose in-page links were written for GitHub (`[…](#17-定期メンテナンス…)`): their headings get ids
# that keep the Japanese, like GitHub does. Other folders keep the ASCII-only ids their links point at.
UNICODE_ID_FOLDERS = {"kosu"}
_unicode_ids = False


def github_slug(value: str, separator: str) -> str:
    """The id of a heading, as the links of these documents write it: lower case, punctuation (also full-width)
    and number signs like ① dropped, each space -> "-"."""
    value = re.sub(r"[^\w\- ]", "", value.strip().lower())
    value = "".join(c for c in value if unicodedata.category(c) != "No")
    return value.replace(" ", separator)


def render_body(md_text: str) -> str:
    configs = dict(EXTENSION_CONFIGS)
    if _unicode_ids:
        configs["toc"] = {"slugify": github_slug}
    return markdown.markdown(
        dedent_fences(md_text),
        extensions=EXTENSIONS,
        extension_configs=configs,
        output_format="html5",
    )


def pdf_body(body: str, stem: str) -> str:
    """PDF: one-colour code lines sized to fit, each block with a link to its copy button."""
    n = 0

    def rep(m: re.Match) -> str:
        nonlocal n
        n += 1
        href = f"html/{quote(stem)}.html#code-{n}"
        pre, text = copy_friendly_pre(m)
        note, wrap_note = selection_copy_note(text, False) or "", selection_copy_note(text, True)
        note_html = f'<span class="copy-note" data-wrap-note="{html.escape(wrap_note)}">{html.escape(note)}</span>'
        return f'<div class="code-head">{note_html}<a class="copy-link" href="{href}">コピー</a></div>' + pre

    return PRE_RE.sub(rep, body)


def md_to_html(md_text: str, title: str, stem: str | None = None) -> str:
    body = pdf_body(render_body(md_text), stem or title)
    code_text = "".join(html.unescape(re.sub(r"<[^>]+>", "", t)) for t in CODE_TEXT_RE.findall(body))
    page = HTML_TEMPLATE.format(
        title=title, body=body, code_font=CODE_FONT, code_font_face=font_face_css(code_text)
    )
    page = page.replace("</style>", PDF_COPY_CSS + "</style>", 1)
    return page.replace("</body>", PDF_WRAP_JS + "</body>", 1)


LINK_RE = re.compile(r'\b(src|href)="([^"#:]+?)(#[^"]*)?"')


def web_links(body: str, md_path: Path, out_dir: Path) -> str:
    """Make relative links work from <folder>/html/: images keep pointing at the files, .md links become .html."""
    def rep(m: re.Match) -> str:
        attr, target, frag = m.group(1), m.group(2), m.group(3) or ""
        if target.startswith(("/", "data")):
            return m.group(0)
        src = (md_path.parent / html.unescape(target)).resolve()
        if src.suffix == ".md":
            src = src.parent / "html" / (src.stem + ".html")
        rel = os.path.relpath(src, out_dir).replace(os.sep, "/")
        return f'{attr}="{html.escape(rel)}{frag}"'

    return LINK_RE.sub(rep, body)


def web_html(md_text: str, title: str, md_path: Path, out_dir: Path) -> str:
    body = render_body(md_text)
    n = 0

    def rep(m: re.Match) -> str:
        nonlocal n
        n += 1
        return (f'<div class="code-wrap" id="code-{n}"><button type="button" class="copy-btn">コピー</button>'
                f"{m.group(0)}</div>")

    body = web_links(PRE_RE.sub(rep, body), md_path, out_dir)
    pdf_rel = os.path.relpath(md_path.parent / "pdf" / f"{md_path.stem}.pdf", out_dir).replace(os.sep, "/")
    bar = (f'<div class="web-bar"><a href="index.html">この資料の一覧</a>'
           f'<a href="{quote(pdf_rel)}">PDF 版</a>'
           f"<span>コードの枠の右上の「コピー」を押すと、そのまま 1 回でコピーできます（字下げ・空の行もそのまま）。</span></div>"
           f'<p class="web-note" id="web-note" hidden>色の付いた枠のコードの、右上の「コピー」ボタンを押してください。'
           f"コピーしたら、PDF を見ていた画面に戻ります（ブラウザーで PDF を見ていた時は「戻る」、"
           f"VS Code で見ていた時は VS Code の窓に切り替えます）。</p>"
           f'<div class="paste-note" id="paste-note" role="status" hidden><button type="button" aria-label="閉じる">×</button>'
           f"<b>複数の行をコピーしました。</b>貼り付けた時に、次の確認が出ることがあります。"
           f"どちらもエラーではなく、知らないうちにコマンドが動かないようにするための安全の確認です。"
           f"<br>・コマンド プロンプトや PowerShell の窓（Windows Terminal）:「<b>複数の行を含むテキストを貼り付けようとしています。</b>…"
           f"続行してもよろしいですか?」→ <b>「強制的に貼り付け」</b>を押す"
           f"<br>・VS Code のターミナル:「<b>○ 行のテキストをターミナルに貼り付けますか?</b>」→ <b>「貼り付け」</b>を押す"
           f"（「1 行として貼り付け」は、何行もの文字を 1 行にまとめて貼る物なので、押さないでください）</div>")
    page = HTML_TEMPLATE.format(title=title, body=bar + body, code_font="Consolas", code_font_face="")
    page = page.replace("</style>", WEB_CSS + "</style>", 1)
    return page.replace("</body>", WEB_JS + "</body>", 1)


def write_web_index(folder: Path) -> None:
    """<folder>/html/index.html: the README (if any) or a plain list of the volumes."""
    out_dir = folder / "html"
    out_dir.mkdir(exist_ok=True)
    readme = folder / "README.md"
    if readme.exists():
        md_text = readme.read_text(encoding="utf-8")
    else:
        items = "\n".join(f"- [{p.stem}]({p.name})" for p in sorted(folder.glob("*.md")) if not p.name.startswith("_"))
        md_text = f"# {folder.name}\n\n{items}\n"
    body = web_links(render_body(md_text), readme, out_dir)
    page = HTML_TEMPLATE.format(title=f"{folder.name} の一覧", body=body, code_font="Consolas", code_font_face="")
    (out_dir / "index.html").write_text(page.replace("</style>", WEB_CSS + "</style>", 1), encoding="utf-8")


# The HTML edition on the web (GitHub Pages of this repository: <WEB_BASE>/<folder>/html/<name>.html).
# The PDF's "コピー" links go there. A link to a file in the folder (../html/... or file:///...) does not
# work everywhere: VS Code's PDF preview (the vscode-pdf extension, PDF.js 2.10) only draws http(s)/ftp/
# mailto/tel links, so such a "コピー" was not drawn as a link and pressing it did nothing.
WEB_BASE = "https://oltotlo79-rgb.github.io/education"


def copy_link_url(uri: str, folder: str) -> str | None:
    """The web address for a copy link (file:///…/html/X.html#code-N, ../html/…, or an older web address)."""
    if not uri.startswith(("file:///", "../html/", WEB_BASE + "/")):
        return None
    m = re.match(r".*/html/([^/]+\.html#code-\d+)$", uri)
    return f"{WEB_BASE}/{quote(folder)}/html/{m.group(1)}" if m else None


# Chrome writes a "tagged" PDF: every block of text (and every coloured run, e.g. a comment at the end
# of a code line) is wrapped in marked content (BDC … EMC) for the tag tree. VS Code's PDF preview
# (PDF.js 2.10) then loses the line break after such a run: it puts the end-of-line mark on the
# marked-content marker instead of on the text, so no line break is made there, and text selected and
# copied from the PDF joins that line with the next one ("fn main() {\n    // comment    let a = 1;").
# Without the marked content every line of a code block is copied as its own line, with its indentation.
#
# The page number at the foot of each page is text too, so a code block that goes on to the next page
# was copied with that page's number as a line of its own ("        false\n134\n    }"). The number is
# therefore moved out of the page's text into an annotation that draws the same thing (an annotation's
# drawing is not part of the text that is selected and copied).
MARKED_CONTENT_OPS = ("BDC", "BMC", "EMC")
TEXT_STATE_OPS = {"q", "Q", "cm", "RG", "rg", "G", "g", "K", "k", "CS", "cs", "SC", "SCN", "sc", "scn", "gs",
                  "BT", "ET", "Tf", "Tm", "Td", "TD", "Tc", "Tw", "Tz", "TL", "Ts", "Tr", "Tj", "TJ"}
FOOTER_MAX_Y = 50.0   # pt from the bottom of the page: the page number sits in the 20 mm bottom margin


def _apply(m, x, y):
    a, b, c, d, e, f = (float(v) for v in m)
    return a * x + c * y + e, b * x + d * y + f


def _footer_group(ops):
    """(start, end, (x, y)) of the top-level q … Q group that only draws the page number, or None."""
    if not ops or str(ops[0][1]) != "cm":
        return None
    page_cm = ops[0][0]
    depth, start = 0, None
    for i, (operands, op) in enumerate(ops[1:], 1):
        op = str(op)
        if op == "q":
            if depth == 0:
                start = i
            depth += 1
        elif op == "Q":
            depth -= 1
            if depth == 0 and start is not None:
                group = ops[start:i + 1]
                names = [str(o) for _, o in group]
                if names.count("BT") == 1 and set(names) <= TEXT_STATE_OPS:
                    cm = next((g[0] for g in group if str(g[1]) == "cm"), [1, 0, 0, 1, 0, 0])
                    tm = next((g[0] for g in group if str(g[1]) == "Tm"), None)
                    if tm is not None:
                        x, y = _apply(page_cm, *_apply(cm, float(tm[4]), float(tm[5])))
                        if y < FOOTER_MAX_Y:
                            return start, i, (x, y)
                start = None
        elif depth == 0 and op == "BT":
            return None   # text outside a group: not the layout Chrome writes
    return None


def copy_friendly_text(pdf) -> int:
    """Take the marked content out of the pages (and drop the tag tree), and move each page number into
    an annotation. Returns the number of changes (0 for a PDF that was already done)."""
    import pikepdf
    changes = 0
    seen_forms = set()

    def strip_forms(resources) -> int:
        """The same for the drawings a page uses (form XObjects, e.g. an embedded diagram with text)."""
        n = 0
        for xobj in (resources or {}).get("/XObject", {}).values():
            if xobj.get("/Subtype") != pikepdf.Name.Form or xobj.objgen in seen_forms:
                continue
            seen_forms.add(xobj.objgen)
            fops = pikepdf.parse_content_stream(xobj)
            fkept = [(operands, op) for operands, op in fops if str(op) not in MARKED_CONTENT_OPS]
            if len(fkept) != len(fops):
                xobj.write(pikepdf.unparse_content_stream(fkept))
                n += len(fops) - len(fkept)
            n += strip_forms(xobj.get("/Resources"))
        return n

    for page in pdf.pages:
        changes += strip_forms(page.obj.get("/Resources"))
        ops = pikepdf.parse_content_stream(page)
        kept = [(operands, op) for operands, op in ops if str(op) not in MARKED_CONTENT_OPS]
        changed = len(kept) != len(ops)
        footer = _footer_group(kept)
        if footer:
            start, end, (x, y) = footer
            width = float(page.obj.MediaBox[2])
            rect = [round(x - 6, 2), round(y - 5, 2), round(width - 4, 2), round(y + 12, 2)]
            appearance = pdf.make_stream(
                pikepdf.unparse_content_stream([kept[0]] + kept[start:end + 1]),
                Type=pikepdf.Name.XObject, Subtype=pikepdf.Name.Form, BBox=rect, Resources=page.obj.Resources)
            annot = pdf.make_indirect(pikepdf.Dictionary(
                Type=pikepdf.Name.Annot, Subtype=pikepdf.Name.Watermark, Rect=rect,
                F=4 | 64 | 128,   # print it; read-only and locked (nothing to click or move)
                AP=pikepdf.Dictionary(N=appearance)))
            if "/Annots" not in page.obj:
                page.obj.Annots = pdf.make_indirect(pikepdf.Array())
            page.obj.Annots.append(annot)
            kept = kept[:start] + kept[end + 1:]
            changed = True
        if changed:
            changes += 1 + len(ops) - len(kept)
            page.obj.Contents = pdf.make_stream(pikepdf.unparse_content_stream(kept))
        if "/StructParents" in page.obj:
            del page.obj["/StructParents"]
            changes += 1
        for annot in page.obj.get("/Annots", []):
            if "/StructParent" in annot:
                del annot["/StructParent"]
    for key in ("/StructTreeRoot", "/MarkInfo"):
        if key in pdf.Root:
            del pdf.Root[key]
            changes += 1
    return changes


# ---------------------- "コピー" inside Adobe Acrobat ----------------------------
# Adobe Acrobat and Acrobat Reader run the scripts of a PDF, so there the "コピー" of each code block is a
# button of the PDF itself: it opens a small window with the code in a text box, already all selected,
# and Ctrl+C copies it exactly (line breaks, indentation, tabs, "…", emoji …), with nothing opened on
# the web. A script of a PDF cannot write to the clipboard itself: Acrobat Reader quietly ignores
# app.execMenuItem("Copy") from a document, so the one key press is the user's.
#
# In the file the buttons are hidden ("hidden" and "no view") and lie outside the page, so every other
# viewer leaves them out and keeps the "コピー" link to the web edition: VS Code's PDF preview (PDF.js) and
# Chrome do not draw such a widget, and Chrome (pdfium) would take any form field lying over the link as
# the thing clicked, even a hidden one, and then not follow the link. The document's script, only in
# Acrobat (app.viewerType "Reader" or "Exchange…"), moves each button onto its "コピー" and shows it, and
# then marks the document unchanged again so that closing it asks nothing.
COPY_FIELD_PREFIX = "copy_"
COPY_FIELD_PARKED = [-2, -2, -1, -1]   # where the buttons lie in the file: outside the page, never clicked


def _js(s: str) -> str:
    """A JavaScript string literal in plain ASCII (the script of a PDF is then the same in every encoding)."""
    return json.dumps(s, ensure_ascii=True)


ACROBAT_DOC_JS = """\
function defCopyDialog(code) {
  app.execDialog({
    initialize: function (d) { d.load({"code": code}); },
    description: {
      name: %(title)s,
      elements: [{type: "view", align_children: "align_left", elements: [
        {type: "static_text", name: %(l1)s, bold: true, width: 640},
        {type: "static_text", name: %(l2)s, width: 640},
        {type: "edit_text", item_id: "code", multiline: true, width: 640, height: 300},
        {type: "static_text", name: %(l3)s, width: 640},
        {type: "static_text", name: %(l4)s, width: 640},
        {type: "ok", ok_name: %(ok)s}
      ]}]
    }
  });
}
var defCopyPlaces = __PLACES__;
if (typeof app !== "undefined" && /^(Reader|Exchange)/.test(String(app.viewerType))) {
  for (var name in defCopyPlaces) {
    var f = this.getField(name);
    if (f) { f.rect = defCopyPlaces[name]; f.display = display.visible; }
  }
  this.dirty = false;
}
""" % {
    "title": _js("コードのコピー"),
    "l1": _js("コードが青く選ばれています。このまま Ctrl + C を押すと、コピーできます。"),
    "l2": _js("（中をクリックして青い色が消えた時は、Ctrl + A を押してから Ctrl + C を押します）"),
    "l3": _js("コピーしたら「閉じる」を押して、この窓を閉じます。"),
    "l4": _js("下の赤い字の「警告 : JavaScript ウィンドウ」は、PDF の中のボタンが開いた窓だという印です。危ない物ではありません。"),
    "ok": _js("閉じる"),
}


def code_texts(md_text: str) -> list[str]:
    """The text of each code block, in order (code-1, code-2 …): what the web edition's "コピー" copies."""
    return [re.sub(r"\n$", "", html.unescape(re.sub(r"<[^>]+>", "", m.group(2))))
            for m in PRE_RE.finditer(render_body(md_text))]


def acrobat_copy_buttons(pdf, codes: list[str]) -> int:
    """Give each copy link of the PDF an Acrobat "コピー" button (see above). Returns the number of changes
    (0 when the PDF already has the same buttons and script)."""
    import pikepdf
    Name, String = pikepdf.Name, pikepdf.String
    is_copy_field = lambda a: (a.get("/Subtype") == Name.Widget
                               and str(a.get("/T", "")).startswith(COPY_FIELD_PREFIX))
    changes = 0
    places = {}   # field name -> [upper-left x, upper-left y, lower-right x, lower-right y] (Acrobat's field.rect)
    fields = []
    for page in pdf.pages:
        annots = page.obj.get("/Annots")
        if annots is None:
            continue
        old = {str(a.T): a for a in annots if is_copy_field(a)}
        keep = []
        for a in annots:
            if is_copy_field(a):
                continue
            keep.append(a)
            action = a.get("/A")
            if action is None or action.get("/S") != Name.URI:
                continue
            m = re.search(r"#code-(\d+)$", str(action.URI))
            if not m or not (1 <= int(m.group(1)) <= len(codes)):
                continue
            n = int(m.group(1))
            name = f"{COPY_FIELD_PREFIX}{n}"
            x1, y1, x2, y2 = (round(float(v), 2) for v in a.Rect)
            places[name] = [x1, y2, x2, y1]
            js = f"if (typeof defCopyDialog === \"function\") {{ defCopyDialog({_js(codes[n - 1])}); }}".encode("ascii")
            btn = old.pop(name, None)
            if btn is None or bytes(btn.A.JS.read_bytes()) != js or [float(v) for v in btn.Rect] != COPY_FIELD_PARKED:
                w, h = x2 - x1, y2 - y1
                btn = pdf.make_indirect(pikepdf.Dictionary(
                    Type=Name.Annot, Subtype=Name.Widget, FT=Name.Btn, Ff=1 << 16, T=String(name),
                    TU=String("押すと、このコードをコピーする窓が開きます"),
                    Rect=COPY_FIELD_PARKED, F=2 | 32,   # hidden, no view, outside the page (see above)
                    P=page.obj, H=Name.P, MK=pikepdf.Dictionary(),
                    AP=pikepdf.Dictionary(N=pdf.make_stream(b"", Type=Name.XObject, Subtype=Name.Form,
                                                            BBox=[0, 0, w, h])),   # draws nothing
                    A=pikepdf.Dictionary(S=Name.JavaScript, JS=pdf.make_stream(js))))
                changes += 1
            keep.append(btn)   # after the link: on top of it once Acrobat has moved it there
            fields.append(btn)
        changes += len(old)   # buttons of blocks that are gone
        if [x.objgen for x in keep] != [y.objgen for y in annots]:
            page.obj.Annots = pdf.make_indirect(pikepdf.Array(keep))

    form = pdf.Root.get("/AcroForm")
    others = [f for f in (form.get("/Fields", []) if form is not None else []) if not is_copy_field(f)]
    if not others and not fields:
        if form is not None:   # a PDF without code blocks gets no form (and no script, below)
            del pdf.Root["/AcroForm"]
            changes += 1
    else:
        if form is None:
            pdf.Root.AcroForm = form = pikepdf.Dictionary()
        if [f.objgen for f in form.get("/Fields", [])] != [f.objgen for f in others + fields]:
            form.Fields = pikepdf.Array(others + fields)

    doc_js = ACROBAT_DOC_JS.replace("__PLACES__", json.dumps(places, separators=(",", ":"))).encode("ascii")
    names = pdf.Root.get("/Names")
    old_js = None
    if names is not None and "/JavaScript" in names:
        arr = names.JavaScript.get("/Names", [])
        if len(arr) == 2 and isinstance(arr[1].get("/JS"), pikepdf.Stream):
            old_js = bytes(arr[1].JS.read_bytes())
    if not places:
        if old_js is not None:
            del names["/JavaScript"]
            changes += 1
    elif old_js != doc_js:
        if names is None:
            pdf.Root.Names = names = pikepdf.Dictionary()
        names.JavaScript = pikepdf.Dictionary(Names=pikepdf.Array([String("defcopy"), pdf.make_indirect(
            pikepdf.Dictionary(S=Name.JavaScript, JS=pdf.make_stream(doc_js)))]))
        changes += 1
    return changes


def web_copy_links(src: Path, dest: Path, folder: str, codes: list[str] | None = None) -> int:
    """Point the PDF's copy links at the HTML edition on the web (see WEB_BASE), and take the tags out
    and the page numbers out of the text (see copy_friendly_text), so that text copied in VS Code's PDF
    preview keeps its line breaks and gets no page numbers. With the code blocks' texts (codes), also put
    Acrobat's own "コピー" buttons on the links (see acrobat_copy_buttons).

    Chrome writes the links as absolute file:/// URLs. Reads src (the PDF Chrome printed, or an
    existing PDF for --relink) and writes the result to dest (src and dest may be the same file).
    """
    try:
        import pikepdf
    except ImportError:
        print("  (pikepdf is not installed: the copy links keep absolute paths)")
        if src != dest:
            shutil.copyfile(src, dest)
        return 0
    changed = 0
    with pikepdf.open(src, allow_overwriting_input=(src == dest)) as pdf:
        for page in pdf.pages:
            for annot in page.obj.get("/Annots", []):
                action = annot.get("/A")
                if action is None or action.get("/S") != pikepdf.Name("/URI"):
                    continue
                url = copy_link_url(str(action.URI), folder)
                if url and url != str(action.URI):
                    action.URI = pikepdf.String(url)
                    changed += 1
        changed += copy_friendly_text(pdf)
        if codes is not None:
            changed += acrobat_copy_buttons(pdf, codes)
        if src == dest and changed == 0:
            return 0   # --relink on a PDF with nothing to change: leave the file as it is
        for attempt in range(10):
            try:
                pdf.save(dest)
                break
            except PermissionError:
                time.sleep(1)   # the old PDF may be open for a moment (a viewer, antivirus)
        else:
            raise RuntimeError(f"could not write {dest} (is it open in Adobe Acrobat? Close it there and run again)")
    return changed

def chrome_print_pdf(html_path: Path, pdf_path: Path) -> None:
    """Run headless Chrome to print HTML to PDF.

    Chrome with --headless=new sometimes refuses to exit after writing the PDF.
    We work around this by polling for the PDF file (stable size for 2 ticks)
    and killing the process when done.
    """
    if pdf_path.exists():
        pdf_path.unlink()

    udd = Path(tempfile.mkdtemp(prefix="cdp_user_"))
    try:
        cmd = [
            CHROME,
            "--headless=new",
            "--disable-gpu",
            "--disable-extensions",
            "--no-pdf-header-footer",
            "--no-default-browser-check",
            "--no-first-run",
            f"--user-data-dir={udd}",
            f"--print-to-pdf={pdf_path}",
            "--no-margins",
            "--virtual-time-budget=15000",
            html_path.as_uri(),
        ]
        proc = subprocess.Popen(
            cmd,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )

        # Poll: wait until pdf appears AND its size has been stable for 2 ticks.
        last_size = -1
        stable_ticks = 0
        deadline = time.time() + 120  # hard cap 2 min per file
        while time.time() < deadline:
            time.sleep(1.0)
            if pdf_path.exists():
                try:
                    sz = pdf_path.stat().st_size
                except FileNotFoundError:
                    sz = -1
                if sz > 0 and sz == last_size:
                    stable_ticks += 1
                    if stable_ticks >= 2:
                        break
                else:
                    stable_ticks = 0
                    last_size = sz
            if proc.poll() is not None:
                # Chrome exited on its own — done
                break

        # Force-stop the browser
        try:
            proc.kill()
        except Exception:
            pass

        if not pdf_path.exists() or pdf_path.stat().st_size == 0:
            raise RuntimeError(f"PDF not generated: {pdf_path}")
    finally:
        shutil.rmtree(udd, ignore_errors=True)

def convert(md_path: Path) -> Path:
    title = md_path.stem
    md_text = md_path.read_text(encoding="utf-8")
    html_text = md_to_html(md_text, title, md_path.stem)

    # The HTML edition with copy buttons (<folder>/html/<name>.html)
    web_dir = md_path.parent / "html"
    web_dir.mkdir(exist_ok=True)
    (web_dir / f"{title}.html").write_text(web_html(md_text, title, md_path, web_dir), encoding="utf-8")

    # Write HTML to a temp file under the same parent so relative paths work
    html_path = md_path.with_suffix(".tmp.html")
    html_path.write_text(html_text, encoding="utf-8")

    # PDFs go into a `pdf/` subfolder beside the source .md (e.g. next/pdf,
    # react_native/pdf), so each tutorial folder keeps its own output.
    pdf_dir = md_path.parent / "pdf"
    pdf_dir.mkdir(exist_ok=True)
    pdf_path = pdf_dir / f"{title}.pdf"
    print(f"  -> {pdf_path.relative_to(ROOT)}")
    printed = Path(tempfile.mkdtemp(prefix="pdfprint_")) / "printed.pdf"
    chrome_print_pdf(html_path, printed)
    for attempt in range(10):   # Chrome may hold the file for a moment after printing
        try:
            web_copy_links(printed, pdf_path, md_path.parent.name, code_texts(md_text))
            break
        except PermissionError:
            time.sleep(1)
    shutil.rmtree(printed.parent, ignore_errors=True)

    html_path.unlink(missing_ok=True)
    return pdf_path

def convert_html_only(md_path: Path) -> None:
    """Only the HTML edition (when nothing in the PDF changes, e.g. the copy button's script)."""
    web_dir = md_path.parent / "html"
    web_dir.mkdir(exist_ok=True)
    md_text = md_path.read_text(encoding="utf-8")
    (web_dir / f"{md_path.stem}.html").write_text(web_html(md_text, md_path.stem, md_path, web_dir), encoding="utf-8")


def relink(md_path: Path) -> None:
    """Only redo the finishing of the existing PDF (copy links to the web, no tags, Acrobat's copy buttons),
    without Chrome printing."""
    pdf_path = md_path.parent / "pdf" / f"{md_path.stem}.pdf"
    if pdf_path.exists():
        codes = code_texts(md_path.read_text(encoding="utf-8"))
        n = web_copy_links(pdf_path, pdf_path, md_path.parent.name, codes)
        print(f"  {n} changes (copy links -> {WEB_BASE}/{md_path.parent.name}/html/, tags taken out,"
              f" Acrobat copy buttons)")


def main(targets: list[Path], html_only: bool = False, relink_only: bool = False) -> None:
    global _unicode_ids
    for md in targets:
        _unicode_ids = md.parent.name in UNICODE_ID_FOLDERS
        if relink_only:
            print(f"Relinking: {md.relative_to(ROOT)}")
            relink(md)
            continue
        print(f"Converting: {md.relative_to(ROOT)}" + ("  (HTML only)" if html_only else ""))
        convert_html_only(md) if html_only else convert(md)
    if not relink_only:
        for folder in sorted({md.parent for md in targets}):
            _unicode_ids = folder.name in UNICODE_ID_FOLDERS
            write_web_index(folder)
    print("Done.")

def resolve_targets(args: list[str]) -> list[Path]:
    """Turn CLI args into a list of .md files.

    Each arg may be a folder (e.g. `next` or `react_native`) — all its `*.md`
    files are picked up — or a direct path to a single `.md` file. With no args,
    every tutorial subfolder under ROOT is built.
    """
    files: list[Path] = []
    if args:
        for a in args:
            p = (ROOT / a) if not Path(a).is_absolute() else Path(a)
            if p.is_dir():
                files += sorted(q for q in p.glob("*.md") if not q.name.startswith("_") and q.name != "README.md")
            else:
                files.append(p)
    else:
        for folder in sorted(d for d in ROOT.iterdir() if d.is_dir()):
            files += sorted(q for q in folder.glob("*.md") if not q.name.startswith("_"))
    return files

if __name__ == "__main__":
    # --html-only: rebuild just <folder>/html/*.html (no Chrome printing)
    # --relink:    only redo the finishing of the existing PDFs: copy links to WEB_BASE, tags taken out,
    #              Acrobat's copy buttons (no Chrome printing)
    flags = {"--html-only", "--relink"}
    args = [a for a in sys.argv[1:] if a not in flags]
    main(resolve_targets(args), html_only="--html-only" in sys.argv[1:], relink_only="--relink" in sys.argv[1:])

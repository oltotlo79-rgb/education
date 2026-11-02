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


def copy_friendly_pre(match: re.Match) -> str:
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
    return f'<pre{attrs} style="{style}">{inner}</pre>'


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
  function copyText(text) {
    if (navigator.clipboard && window.isSecureContext) {
      return navigator.clipboard.writeText(text);
    }
    return new Promise(function (resolve, reject) {
      var ta = document.createElement("textarea");
      ta.value = text; ta.style.position = "fixed"; ta.style.opacity = "0";
      document.body.appendChild(ta); ta.select();
      try { document.execCommand("copy") ? resolve() : reject(); } catch (e) { reject(e); }
      document.body.removeChild(ta);
    });
  }
  document.querySelectorAll(".copy-btn").forEach(function (btn) {
    btn.addEventListener("click", function () {
      var pre = btn.parentElement.querySelector("pre");
      var text = pre.textContent.replace(/\\n$/, "");
      copyText(text).then(function () {
        btn.textContent = "コピーしました";
        btn.classList.add("done");
        setTimeout(function () { btn.textContent = "コピー"; btn.classList.remove("done"); }, 2000);
      }, function () { btn.textContent = "コピーできませんでした"; });
    });
  });
  if (/^#code-\\d+$/.test(location.hash)) {
    var note = document.getElementById("web-note");
    if (note) { note.hidden = false; }
  }
})();
</script>
"""


def render_body(md_text: str) -> str:
    return markdown.markdown(
        md_text,
        extensions=EXTENSIONS,
        extension_configs=EXTENSION_CONFIGS,
        output_format="html5",
    )


def pdf_body(body: str, stem: str) -> str:
    """PDF: one-colour code lines sized to fit, each block with a link to its copy button."""
    n = 0

    def rep(m: re.Match) -> str:
        nonlocal n
        n += 1
        href = f"html/{quote(stem)}.html#code-{n}"
        return f'<div class="code-head"><a class="copy-link" href="{href}">コピー</a></div>' + copy_friendly_pre(m)

    return PRE_RE.sub(rep, body)


def md_to_html(md_text: str, title: str, stem: str | None = None) -> str:
    body = pdf_body(render_body(md_text), stem or title)
    code_text = "".join(html.unescape(re.sub(r"<[^>]+>", "", t)) for t in CODE_TEXT_RE.findall(body))
    page = HTML_TEMPLATE.format(
        title=title, body=body, code_font=CODE_FONT, code_font_face=font_face_css(code_text)
    )
    return page.replace("</style>", PDF_COPY_CSS + "</style>", 1)


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
           f"コピーしたら、ブラウザーの「戻る」で PDF に戻れます。</p>")
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


def relative_copy_links(src: Path, dest: Path) -> int:
    """Chrome writes links as absolute file:/// URLs; make the copy links relative to the PDF.

    Reads the PDF Chrome printed (src, a temporary file) and writes the result to dest.
    """
    try:
        import pikepdf
    except ImportError:
        print("  (pikepdf is not installed: the copy links keep absolute paths)")
        shutil.copyfile(src, dest)
        return 0
    changed = 0
    with pikepdf.open(src) as pdf:
        for page in pdf.pages:
            for annot in page.obj.get("/Annots", []):
                action = annot.get("/A")
                if action is None or action.get("/S") != pikepdf.Name("/URI"):
                    continue
                uri = str(action.URI)
                m = re.match(r"file:///.*/html/([^/]+\.html#code-\d+)$", uri)
                if m:
                    action.URI = pikepdf.String("../html/" + m.group(1))
                    changed += 1
        for attempt in range(10):
            try:
                pdf.save(dest)
                break
            except PermissionError:
                time.sleep(1)   # the old PDF may be open for a moment (a viewer, antivirus)
        else:
            raise RuntimeError(f"could not write {dest}")
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
            relative_copy_links(printed, pdf_path)
            break
        except PermissionError:
            time.sleep(1)
    shutil.rmtree(printed.parent, ignore_errors=True)

    html_path.unlink(missing_ok=True)
    return pdf_path

def main(targets: list[Path]) -> None:
    for md in targets:
        print(f"Converting: {md.relative_to(ROOT)}")
        convert(md)
    for folder in sorted({md.parent for md in targets}):
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
    main(resolve_targets(sys.argv[1:]))

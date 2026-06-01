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

import os
import sys
import shutil
import subprocess
import tempfile
import time
from pathlib import Path

import markdown

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
  font-family: "Cascadia Mono", "Consolas", "Source Code Pro", "Courier New", monospace;
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
}}
.codehilite pre {{
  white-space: pre-wrap;
  word-break: break-word;
  overflow-wrap: anywhere;
}}

/* Pygments tokens (a single dark theme) */
.codehilite .k, .codehilite .kd, .codehilite .kn, .codehilite .kr {{ color: #c084fc; }}
.codehilite .s, .codehilite .s1, .codehilite .s2, .codehilite .sb {{ color: #fcd34d; }}
.codehilite .c, .codehilite .c1, .codehilite .cm {{ color: #94a3b8; font-style: italic; }}
.codehilite .nf, .codehilite .nx {{ color: #60a5fa; }}
.codehilite .nb {{ color: #34d399; }}
.codehilite .mi, .codehilite .mf {{ color: #f472b6; }}
.codehilite .o, .codehilite .p {{ color: #cbd5e1; }}
.codehilite .nt {{ color: #f59e0b; }}
.codehilite .na {{ color: #93c5fd; }}

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

def md_to_html(md_text: str, title: str) -> str:
    body = markdown.markdown(
        md_text,
        extensions=EXTENSIONS,
        extension_configs=EXTENSION_CONFIGS,
        output_format="html5",
    )
    return HTML_TEMPLATE.format(title=title, body=body)

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
    html_text = md_to_html(md_text, title)

    # Write HTML to a temp file under the same parent so relative paths work
    html_path = md_path.with_suffix(".tmp.html")
    html_path.write_text(html_text, encoding="utf-8")

    # PDFs go into a `pdf/` subfolder beside the source .md (e.g. next/pdf,
    # react_native/pdf), so each tutorial folder keeps its own output.
    pdf_dir = md_path.parent / "pdf"
    pdf_dir.mkdir(exist_ok=True)
    pdf_path = pdf_dir / f"{title}.pdf"
    print(f"  -> {pdf_path.relative_to(ROOT)}")
    chrome_print_pdf(html_path, pdf_path)

    html_path.unlink(missing_ok=True)
    return pdf_path

def main(targets: list[Path]) -> None:
    for md in targets:
        print(f"Converting: {md.relative_to(ROOT)}")
        convert(md)
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
                files += sorted(q for q in p.glob("*.md") if not q.name.startswith("_"))
            else:
                files.append(p)
    else:
        for folder in sorted(d for d in ROOT.iterdir() if d.is_dir()):
            files += sorted(q for q in folder.glob("*.md") if not q.name.startswith("_"))
    return files

if __name__ == "__main__":
    main(resolve_targets(sys.argv[1:]))

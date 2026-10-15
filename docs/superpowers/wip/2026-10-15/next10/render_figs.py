"""Render figure snippets (figs/*.html) to PNG via Chrome headless -> PDF -> PyMuPDF.

Usage: python render_figs.py [name ...]
Each figs/<name>.html contains the raw <div>...<svg>...</svg></div> block.
"""
import subprocess
import sys
import tempfile
import time
import shutil
from pathlib import Path

import fitz

HERE = Path(__file__).resolve().parent
FIGS = HERE / "figs"
OUT = HERE / "png"
OUT.mkdir(exist_ok=True)
CHROME = r"C:\Program Files\Google\Chrome\Application\chrome.exe"
if not Path(CHROME).exists():
    CHROME = r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe"

TEMPLATE = """<!doctype html><html lang="ja"><head><meta charset="utf-8">
<style>@page {{ size: A4; margin: 18mm 16mm; }} body {{ font-family: 'Yu Gothic UI', 'Meiryo', sans-serif; }}</style>
</head><body>{body}</body></html>"""


def render(name: str) -> None:
    src = FIGS / f"{name}.html"
    html = TEMPLATE.format(body=src.read_text(encoding="utf-8"))
    tmp_html = OUT / f"{name}.tmp.html"
    tmp_html.write_text(html, encoding="utf-8")
    pdf = OUT / f"{name}.pdf"
    if pdf.exists():
        pdf.unlink()
    udd = Path(tempfile.mkdtemp(prefix="cdp_"))
    proc = subprocess.Popen(
        [CHROME, "--headless=new", "--disable-gpu", "--no-pdf-header-footer",
         f"--user-data-dir={udd}", f"--print-to-pdf={pdf}", tmp_html.as_uri()],
        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    deadline = time.time() + 60
    last = -1
    while time.time() < deadline:
        time.sleep(0.7)
        if pdf.exists() and pdf.stat().st_size > 0:
            if pdf.stat().st_size == last:
                break
            last = pdf.stat().st_size
        if proc.poll() is not None and pdf.exists():
            break
    proc.kill()
    shutil.rmtree(udd, ignore_errors=True)
    doc = fitz.open(pdf)
    page = doc[0]
    pix = page.get_pixmap(dpi=110)
    pix.save(OUT / f"{name}.png")
    doc.close()
    print("rendered", name)


if __name__ == "__main__":
    names = sys.argv[1:] or [p.stem for p in sorted(FIGS.glob("*.html"))]
    for n in names:
        render(n)

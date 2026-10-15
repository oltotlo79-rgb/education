"""Render a markdown file to PDF using the repo's _build_pdfs pipeline, then dump page PNGs.

usage: python preview.py <md> [pages like 1,2,5-7 | all]
"""
import sys
from pathlib import Path

sys.path.insert(0, r"C:/Users/oltot/Documents/git-projects/def")
import _build_pdfs as b  # noqa: E402
import fitz  # noqa: E402

md = Path(sys.argv[1]).resolve()
html = b.md_to_html(md.read_text(encoding="utf-8"), md.stem)
html_path = md.with_suffix(".tmp.html")
html_path.write_text(html, encoding="utf-8")
pdf = md.with_suffix(".pdf")
b.chrome_print_pdf(html_path, pdf)
html_path.unlink(missing_ok=True)
doc = fitz.open(pdf)
print("pages:", doc.page_count)
sel = sys.argv[2] if len(sys.argv) > 2 else ""
pages = []
if sel.startswith("find:"):
    keys = sel[5:].split("|")
    for i in range(doc.page_count):
        txt = doc[i].get_text()
        if any(k in txt for k in keys):
            pages.append(i + 1)
elif sel == "all":
    pages = list(range(1, doc.page_count + 1))
elif sel:
    for part in sel.split(","):
        if "-" in part:
            a, c = part.split("-")
            pages += list(range(int(a), int(c) + 1))
        else:
            pages.append(int(part))
outdir = md.parent / (md.stem + "_png")
outdir.mkdir(exist_ok=True)
for p in pages:
    if 1 <= p <= doc.page_count:
        pix = doc[p - 1].get_pixmap(dpi=80)
        out = outdir / f"p{p:03d}.png"
        pix.save(out)
        print("saved", out.name)

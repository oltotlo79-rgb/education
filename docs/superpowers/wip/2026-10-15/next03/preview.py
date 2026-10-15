import sys, pathlib
sys.path.insert(0, r"C:/Users/oltot/Documents/git-projects/def")
import _build_pdfs as b
import fitz
here = pathlib.Path(__file__).parent
figs = sorted((here / "figs").glob("*.html"))
sel = sys.argv[1:] 
parts = []
for f in figs:
    if sel and f.stem not in sel:
        continue
    parts.append(f"## {f.stem}\n\n" + f.read_text(encoding="utf-8").strip() + "\n")
md = here / "preview.md"
md.write_text("\n".join(parts), encoding="utf-8")
html = md.with_suffix(".tmp.html")
html.write_text(b.md_to_html(md.read_text(encoding="utf-8"), "preview"), encoding="utf-8")
pdf = here / "preview.pdf"
b.chrome_print_pdf(html, pdf)
doc = fitz.open(pdf)
outdir = here / "png"; outdir.mkdir(exist_ok=True)
for p in outdir.glob("prev-*.png"): p.unlink()
for i, page in enumerate(doc):
    page.get_pixmap(dpi=110).save(outdir / f"prev-{i+1:02d}.png")
print(len(doc), "pages")

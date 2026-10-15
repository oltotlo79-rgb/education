# usage: python preview.py <md file> <out pdf> [png_prefix pages...]
import sys, pathlib
sys.path.insert(0, r"C:\Users\oltot\Documents\git-projects\def")
import _build_pdfs as b
md = pathlib.Path(sys.argv[1])
out = pathlib.Path(sys.argv[2])
html = b.md_to_html(md.read_text(encoding="utf-8"), md.stem)
hp = md.with_suffix(".tmp.html")
hp.write_text(html, encoding="utf-8")
b.chrome_print_pdf(hp, out)
hp.unlink()
import fitz
doc = fitz.open(str(out))
print("pages:", doc.page_count)

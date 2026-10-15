import sys
from pathlib import Path
sys.path.insert(0, r"C:/Users/oltot/Documents/git-projects/def")
import _build_pdfs as b
md = Path(sys.argv[1]).resolve()
html = b.md_to_html(md.read_text(encoding="utf-8"), md.stem)
hp = md.with_suffix(".tmp.html")
hp.write_text(html, encoding="utf-8")
out = md.with_suffix(".pdf")
b.chrome_print_pdf(hp, out)
print("wrote", out)

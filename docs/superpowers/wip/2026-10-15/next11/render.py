"""Render PDF pages containing given search strings to PNG.

Usage: python render.py <pdf> <outdir> <needle1> [<needle2> ...]
Prints page numbers found for each needle and writes page-<n>.png.
"""
import sys
from pathlib import Path

import fitz

pdf = fitz.open(sys.argv[1])
out = Path(sys.argv[2])
out.mkdir(parents=True, exist_ok=True)
print("pages:", pdf.page_count)
for needle in sys.argv[3:]:
    hits = [i for i, p in enumerate(pdf) if needle in p.get_text()]
    print(needle, "->", [h + 1 for h in hits])
    for h in hits[:2]:
        pix = pdf[h].get_pixmap(dpi=110)
        path = out / f"page-{h + 1:03d}.png"
        pix.save(str(path))
        print("  saved", path)

# usage: python topng.py <pdf> <outprefix> [pages(1-based) ...]  (no pages = all)
import sys, fitz
doc = fitz.open(sys.argv[1])
pages = [int(p) for p in sys.argv[3:]] or list(range(1, doc.page_count + 1))
for p in pages:
    pix = doc[p - 1].get_pixmap(dpi=80)
    pix.save(f"{sys.argv[2]}-{p:03d}.png")
print("ok", pages)

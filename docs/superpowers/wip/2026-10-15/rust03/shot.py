import sys, fitz
pdf, outprefix = sys.argv[1], sys.argv[2]
pages = [int(p) for p in sys.argv[3].split(',')] if len(sys.argv) > 3 else None
doc = fitz.open(pdf)
print("pages:", doc.page_count)
for i in (pages or range(doc.page_count)):
    if i >= doc.page_count: continue
    pix = doc[i].get_pixmap(dpi=80)
    pix.save(f"{outprefix}{i+1:03d}.png")

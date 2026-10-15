import sys, fitz, re
pdf, prefix, pat = sys.argv[1], sys.argv[2], sys.argv[3]
doc = fitz.open(pdf)
print("pages", doc.page_count)
for i, p in enumerate(doc):
    t = p.get_text()
    if re.search(pat, t):
        p.get_pixmap(dpi=80).save(f"{prefix}{i+1:03d}.png"); print(i+1)

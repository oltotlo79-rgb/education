import sys, fitz, re
pdf = sys.argv[1]; outdir = sys.argv[2]
want = sys.argv[3:]  # list of caption prefixes like 図5-1: or 'page:N'
doc = fitz.open(pdf)
print("pages", doc.page_count)
for w in want:
    if w.startswith("page:"):
        idx = [int(w[5:]) - 1]
    else:
        idx = [i for i in range(doc.page_count) if w in doc[i].get_text()]
    for i in idx[:1]:
        pix = doc[i].get_pixmap(dpi=100)
        name = re.sub(r"[^A-Za-z0-9]", "_", w)
        pix.save(f"{outdir}/p_{name}_{i+1}.png")
        print(w, "-> page", i + 1)

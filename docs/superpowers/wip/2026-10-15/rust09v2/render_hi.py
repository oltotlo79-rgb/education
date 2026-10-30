import sys, fitz
pdf, outdir, page, y0, y1 = sys.argv[1], sys.argv[2], int(sys.argv[3]), float(sys.argv[4]), float(sys.argv[5])
doc = fitz.open(pdf)
p = doc[page - 1]
r = p.rect
clip = fitz.Rect(0, r.height * y0, r.width, r.height * y1)
pix = p.get_pixmap(dpi=150, clip=clip)
out = f"{outdir}/hi_{page}_{int(y0*100)}.png"
pix.save(out)
print(out)

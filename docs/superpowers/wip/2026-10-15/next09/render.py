import sys, fitz, re
pdf = fitz.open(r"C:/Users/oltot/Documents/git-projects/def/next/pdf/09-styling.pdf")
out = r"C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/next09/"
print("pages", pdf.page_count)
pats = sys.argv[1:]
for pat in pats:
    for i, page in enumerate(pdf):
        if pat in page.get_text():
            pix = page.get_pixmap(dpi=110)
            fn = out + f"p{i+1:03d}.png"
            pix.save(fn)
            print(pat, "->", i+1, fn)
            break
    else:
        print(pat, "NOT FOUND")

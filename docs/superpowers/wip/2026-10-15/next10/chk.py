import fitz
from PIL import Image
d=fitz.open('next/pdf/10-deployment.pdf'); print('pages', len(d))
out=r'C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/next10/pdfpng'
hit={}
for i,p in enumerate(d):
    t=p.get_text()
    if 'src/' in t or ('src'+chr(92)) in t: print('src on page', i+1)
    for k in ['図10-2:','図10-6:','8.1 プロジェクト構成','8.2 各ファイルの概要']:
        if k in t and k not in hit: hit[k]=i+1
print(hit)
pgs=sorted(set(hit.values())); ims=[]
for pg in pgs:
    d[pg-1].get_pixmap(dpi=80).save(f'{out}/v{pg:02d}.png'); ims.append(Image.open(f'{out}/v{pg:02d}.png'))
for k in range(0,len(ims),2):
    pair=ims[k:k+2]; w=sum(i.width for i in pair)+10; h=max(i.height for i in pair)
    s=Image.new('RGB',(w,h),'white'); x=0
    for i in pair: s.paste(i,(x,0)); x+=i.width+10
    s.save(f'{out}/w{k//2}.png')
print(pgs)

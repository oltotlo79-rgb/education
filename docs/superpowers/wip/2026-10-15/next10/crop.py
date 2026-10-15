import sys
from PIL import Image, ImageChops
from pathlib import Path
out = Path("png")
imgs = []
for n in sys.argv[2:]:
    im = Image.open(out / f"{n}.png").convert("RGB")
    bg = Image.new("RGB", im.size, (255, 255, 255))
    bbox = ImageChops.difference(im, bg).getbbox()
    imgs.append(im.crop(bbox))
w = max(i.width for i in imgs)
h = sum(i.height for i in imgs) + 20 * len(imgs)
sheet = Image.new("RGB", (w, h), (255, 255, 255))
y = 0
for i in imgs:
    sheet.paste(i, (0, y))
    y += i.height + 20
sheet.save(out / f"{sys.argv[1]}.png")
print(sheet.size)

import re, sys
from pathlib import Path
p = Path(sys.argv[1])
raw = p.read_bytes().decode("utf-8")
crlf = "\r\n" in raw
lines = raw.replace("\r\n", "\n").split("\n")
out, in_code, fixed = [], False, []
item = re.compile(r"^>\s*(?:[-*]|\d+\.)\s")
for i, l in enumerate(lines):
    if l.lstrip().startswith("```"):
        in_code = not in_code
    if not in_code and item.match(l) and out:
        prev = out[-1]
        if prev.startswith(">") and prev.strip() != ">" and not item.match(prev):
            out.append(">")
            fixed.append(i + 1)
    out.append(l)
s = "\n".join(out)
if crlf:
    s = s.replace("\n", "\r\n")
p.write_bytes(s.encode("utf-8"))
print("fixed at lines", fixed)

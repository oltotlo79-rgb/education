"""Expand placeholders in a part file.

@@CODE u04/a.rs@@  -> contents of snips/u04/a.rs (without trailing newline)
@@OUT u04/a@@      -> stdout of the run
@@ERR u04/a@@      -> stderr (cargo + rustc), dropping Finished/Running lines
@@RUN u04/a@@      -> stdout followed by stderr minus Compiling/Finished/Running lines
@@ERRBODY u04/a@@  -> stderr minus Compiling/Finished/Running lines

usage: python fill.py parts/unit04.md out/unit04.md
"""
import re
import sys
from pathlib import Path

R = Path(__file__).resolve().parent
S = R / "snips"


def read(p):
    return p.read_text(encoding="utf-8").rstrip("\n")


def strip_status(text, drop_compiling):
    out = []
    for line in text.split("\n"):
        s = line.strip()
        if s.startswith("Finished ") or s.startswith("Running "):
            continue
        if drop_compiling and s.startswith("Compiling "):
            continue
        out.append(line)
    return "\n".join(out).strip("\n")


def repl(m):
    kind, arg = m.group(1), m.group(2)
    if kind == "CODE":
        return read(S / arg)
    base = S / arg
    out = read(Path(str(base) + ".out")) if Path(str(base) + ".out").exists() else ""
    err = read(Path(str(base) + ".err")) if Path(str(base) + ".err").exists() else ""
    if kind == "OUT":
        if not out:
            raise SystemExit(f"empty stdout for {arg}")
        return out
    if kind == "ERR":
        return strip_status(err, drop_compiling=False)
    if kind == "ERRBODY":
        return strip_status(err, drop_compiling=True)
    if kind == "RUN":
        e = strip_status(err, drop_compiling=True)
        if not e:
            return out
        if not out:
            return e
        kept = [
            line
            for line in err.split("\n")
            if not line.strip().startswith(("Finished ", "Running ", "Compiling "))
        ]
        lead = "\n" if kept and kept[0].strip() == "" else ""
        return out + "\n" + lead + e
    raise SystemExit("unknown kind " + kind)


src, dst = sys.argv[1], sys.argv[2]
text = Path(src).read_text(encoding="utf-8")
text = re.sub(r"@@(CODE|OUT|ERR|RUN|ERRBODY) ([^@]+)@@", repl, text)
Path(dst).parent.mkdir(parents=True, exist_ok=True)
Path(dst).write_text(text, encoding="utf-8", newline="\n")
left = re.findall(r"@@[A-Z]+ [^@]+@@", text)
if left:
    raise SystemExit(f"unexpanded: {left}")
print("filled", dst)

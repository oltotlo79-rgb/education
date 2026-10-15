"""Split a bundle file into snippet files.

Bundle format:
//// FILE: name.rs
...code...
//// FILE: other.rs
...
Files are written into the same directory as the bundle.
"""
import os
import sys

bundle = sys.argv[1]
outdir = os.path.dirname(bundle)
name = None
buf = []


def flush():
    if name:
        text = "\n".join(buf).strip("\n") + "\n"
        with open(os.path.join(outdir, name), "w", encoding="utf-8", newline="\n") as f:
            f.write(text)
        print("wrote", name)


for line in open(bundle, encoding="utf-8").read().split("\n"):
    if line.startswith("//// FILE: "):
        flush()
        name = line[len("//// FILE: "):].strip()
        buf = []
    else:
        buf.append(line)
flush()

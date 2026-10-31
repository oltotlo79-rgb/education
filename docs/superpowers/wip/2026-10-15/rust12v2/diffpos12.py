"""2つの Windows の実行ファイル（PE 形式）を比べ、違うバイトがどこにあるかを、場所の名前付きで表示する。
使い方: python diffpos12.py 1つ目.exe 2つ目.exe
"""
import struct
import sys

a = open(sys.argv[1], "rb").read()
b = open(sys.argv[2], "rb").read()
diff = [i for i in range(min(len(a), len(b))) if a[i] != b[i]]

# PE の見出しを読む
pe = struct.unpack_from("<I", a, 0x3C)[0]
n_sections = struct.unpack_from("<H", a, pe + 6)[0]
opt_size = struct.unpack_from("<H", a, pe + 20)[0]
opt = pe + 24
magic = struct.unpack_from("<H", a, opt)[0]
dirs = opt + (112 if magic == 0x20B else 96)
debug_rva, debug_size = struct.unpack_from("<II", a, dirs + 6 * 8)
sections = []
sec = opt + opt_size
for k in range(n_sections):
    off = sec + 40 * k
    vsize, vaddr, rsize, rptr = struct.unpack_from("<IIII", a, off + 8)
    sections.append((vaddr, max(vsize, rsize), rptr))


def rva_to_off(rva):
    for vaddr, size, rptr in sections:
        if vaddr <= rva < vaddr + size:
            return rptr + rva - vaddr
    return None


labels = [(pe + 8, 4, "PE の見出しの、リンクした時刻")]
debug_off = rva_to_off(debug_rva)
if debug_off is not None:
    for k in range(debug_size // 28):
        entry = debug_off + 28 * k
        labels.append((entry + 4, 4, f"デバッグ情報の目次 {k + 1} の時刻"))
        kind = struct.unpack_from("<I", a, entry + 12)[0]
        raw_ptr = struct.unpack_from("<I", a, entry + 24)[0]
        if kind == 2 and a[raw_ptr:raw_ptr + 4] == b"RSDS":
            labels.append((raw_ptr + 4, 16, "PDB と組み合わせる識別番号（GUID）"))


def label(i):
    for start, size, name in labels:
        if start <= i < start + size:
            return name
    return "そのほか"


print(f"違うバイトの数: {len(diff)}")
groups = {}
for i in diff:
    groups.setdefault(label(i), []).append(i)
for name, offsets in groups.items():
    print(f"  {name}: {len(offsets)} バイト")

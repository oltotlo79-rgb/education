"""本の中の、標準ライブラリだけの Rust のコード（fn main があるもの）を、1.99.0 と 1.99.1 でコンパイル・実行して、表示を比べる。

使い方: python compare199.py <巻の md> ...
- コンパイルの表示（エラー・警告）は、そのまま比べる
- 実行の表示は、スレッドの番号（thread 'main' (12345)）だけをそろえてから比べる
- 実行は 5 秒で打ち切る（サーバーや入力待ちのコードがあるため）。打ち切りも「同じ」として比べる
"""
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
EXTERNAL = re.compile(r"^\s*(use|extern crate)\s+(tokio|libsql|serde|serde_json|anyhow|thiserror|topcoat|rand|axum|reqwest|futures|tracing|argon2|chrono|uuid|async_trait|criterion|proptest)\b", re.M)


def run(cmd, cwd, timeout):
    try:
        p = subprocess.run(cmd, cwd=cwd, capture_output=True, timeout=timeout, stdin=subprocess.DEVNULL)
        return p.returncode, p.stdout.decode("utf-8", "replace") + p.stderr.decode("utf-8", "replace")
    except subprocess.TimeoutExpired:
        return "timeout", ""


def norm(text):
    text = re.sub(r"thread '([^']*)' \(\d+\)", r"thread '\1' (N)", text)
    return text.replace("\r", "")


total = same = 0
diffs = []
for md in sys.argv[1:]:
    vol = Path(md).name[:2]
    s = Path(md).read_text(encoding="utf-8")
    blocks = re.findall(r"```rust\n(.*?)```", s, re.S)
    for i, code in enumerate(blocks):
        if "fn main" not in code or EXTERNAL.search(code) or "#[tokio::main]" in code:
            continue
        d = HERE / "work" / f"v{vol}_{i:03}"
        d.mkdir(parents=True, exist_ok=True)
        (d / "main.rs").write_text(code, encoding="utf-8")
        results = []
        for tc in ("1.99.0", "1.99.1"):
            exe = f"m_{tc.replace('.', '')}.exe"
            c_code, c_out = run(["rustup", "run", tc, "rustc", "--edition", "2024", "main.rs", "-o", exe], d, 120)
            if c_code == 0:
                r_code, r_out = run([str(d / exe)], d, 5)
            else:
                r_code, r_out = None, ""
            results.append((c_code, norm(c_out), r_code, norm(r_out)))
        total += 1
        a, b = results
        # コンパイルの表示の中の、実行ファイルの名前の違いはそろえる
        if (a[0], a[1].replace("m_1990", "m_X"), a[2], a[3]) == (b[0], b[1].replace("m_1991", "m_X"), b[2], b[3]):
            same += 1
        else:
            diffs.append((vol, i, a, b))
print(f"programs: {total}, same: {same}, different: {len(diffs)}")
for vol, i, a, b in diffs[:20]:
    print(f"--- vol {vol} block {i}")
    print("1.99.0:", a[0], a[2], repr(a[1][:300]), repr(a[3][:300]))
    print("1.99.1:", b[0], b[2], repr(b[1][:300]), repr(b[3][:300]))

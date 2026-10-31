"""練習のプロジェクト（Cargo の）を、2つの版の組み合わせでビルド・実行・テストして、表示を比べる。

使い方: python cmp_labs.py <名前> <ラボのフォルダ> <組み合わせA> <組み合わせB>
  組み合わせ: "1.99.0" のような Rust の版。"1.99.1+tokio=1.54.1" のように + で依存の版の差し替えも書ける
- それぞれのバイナリを 10 秒で打ち切って動かす。cargo test も動かす
- 時間・スレッドの番号・アドレス・パスは、比べる前にそろえる
"""
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
name, lab, *combos = sys.argv[1:]


def sh(cmd, cwd, timeout=900, env=None):
    try:
        p = subprocess.run(cmd, cwd=cwd, capture_output=True, timeout=timeout, stdin=subprocess.DEVNULL, env=env)
        return p.returncode, (p.stdout + p.stderr).decode("utf-8", "replace").replace("\r", "")
    except subprocess.TimeoutExpired as e:
        out = ((e.stdout or b"") + (e.stderr or b"")).decode("utf-8", "replace").replace("\r", "")
        return "timeout", out


def norm(t, work):
    t = t.replace(str(work), "WORK").replace(str(work).replace("\\", "/"), "WORK")
    t = re.sub(r"thread '([^']*)' \(\d+\)", r"thread '\1' (N)", t)
    t = re.sub(r"\d+(\.\d+)?\s?(ms|µs|us|ns|s)\b", "T", t)
    t = re.sub(r"0x[0-9a-f]{6,}", "0xADDR", t)
    t = re.sub(r"/rustc/[0-9a-f]{40}/", "/rustc/HASH/", t)
    t = re.sub(r"^\s*(Compiling|Finished|Running|Locking|Updating|Downloaded|Downloading|Blocking|Adding)\b.*$", "", t, flags=re.M)
    return "\n".join(l.rstrip() for l in t.splitlines() if l.strip())


results = {}
for combo in combos:
    parts = combo.split("+")
    tc = parts[0]
    # パスが長いとリンクで LNK1104 になるので、短い場所（WORK_ROOT）を使えるようにする
    root = Path(os.environ.get("WORK_ROOT", str(HERE / "labs")))
    work = root / f"{name}_{'new' if '+' in combo or combo != combos[0] else 'old'}"
    if work.exists():
        shutil.rmtree(work)
    shutil.copytree(lab, work, ignore=shutil.ignore_patterns("target", "rust-toolchain.toml", ".git"))
    for dep in parts[1:]:
        crate, ver = dep.split("=")
        # Cargo.toml が "=1.53.3" のように固定していたら、その版の書き方も差し替える
        toml = work / "Cargo.toml"
        t = toml.read_text(encoding="utf-8")
        t = re.sub(r'(^' + re.escape(crate) + r'\s*=\s*(?:\{[^}]*version\s*=\s*)?")=?[0-9.]+(")', r"\g<1>=" + ver + r"\2", t, flags=re.M)
        toml.write_text(t, encoding="utf-8")
        code, out = sh(["cargo", f"+{tc}", "update", "-p", crate, "--precise", ver], work)
        print(f"[{combo}] update {crate} -> {ver}: {code} {out.strip().splitlines()[-1] if out.strip() else ''}")
    env = dict(os.environ, CARGO_TARGET_DIR=str(work / "target"))
    outs = {}
    code, out = sh(["cargo", f"+{tc}", "build", "--bins"], work, env=env)
    outs["build"] = norm(out, work)
    for exe in sorted((work / "target" / "debug").glob("*.exe")):
        c, o = sh([str(exe)], work, timeout=10)
        outs["run " + exe.stem] = f"{c}\n" + norm(o, work)
    c, o = sh(["cargo", f"+{tc}", "test", "--no-fail-fast"], work, env=env)
    outs["test"] = norm(o, work)
    # わざと間違えた例（errs/*.rs）: bin に置いてビルドし、通れば動かす
    errs = os.environ.get("ERRS_DIR")
    if errs:
        for rs in sorted(Path(errs).glob("*.rs")):
            dst = work / "src" / "bin" / rs.name
            shutil.copy(rs, dst)
            c, o = sh(["cargo", f"+{tc}", "build", "--bin", rs.stem], work, env=env)
            text = f"build {c}\n" + norm(o, work)
            if c == 0:
                rc, ro = sh([str(work / "target" / "debug" / f"{rs.stem}.exe")], work, timeout=10)
                text += f"\nrun {rc}\n" + norm(ro, work)
            outs["err " + rs.stem] = text
            dst.unlink()
    results[combo] = outs

a, b = (results[c] for c in combos)
keys = sorted(set(a) | set(b))
diff = [k for k in keys if a.get(k) != b.get(k)]
print(f"{name}: {len(keys)} items, different: {len(diff)}")
for k in diff:
    print(f"--- {k}")
    al = (a.get(k) or "").splitlines(); bl = (b.get(k) or "").splitlines()
    for x, y in zip(al, bl):
        if x != y:
            print("  A:", x[:200]); print("  B:", y[:200]); break
    if len(al) != len(bl):
        print(f"  lines {len(al)} vs {len(bl)}")

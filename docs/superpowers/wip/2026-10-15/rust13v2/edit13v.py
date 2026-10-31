from pathlib import Path

H = Path(__file__).resolve().parent
edits = {
    "parts/unit01.src.md": [
        ("この PC の既定の版は 1.97.1 ですが、RustNote のフォルダでは 1.99.1 が使われています。",
         "PC の既定の版（stable）は、`rustup update` をするたびに新しくなっていきますが、RustNote のフォルダでは、いつも 1.99.1 が使われます（第1巻 UNIT 04）。"),
        ("`rust-toolchain.toml` が無いフォルダや、`cargo +stable build` のように版を指定して、Rust 1.97.1 でビルドしようとした場合です（この PC の既定の版 `stable` は 1.97.1 です）。",
         "`rust-toolchain.toml` が無いフォルダで、古い Rust が既定になっている PC や、`cargo +1.90.0 check` のように版を指定して、古い Rust でビルドしようとした場合です（この PC には、古い 1.90.0 も入っています）。"),
        ("cargo +stable check --locked", "cargo +1.90.0 check --locked"),
        ("「rustc 1.97.1 は、次のパッケージに対応していません」というエラーです。", "「rustc 1.90.0 は、次のパッケージに対応していません」というエラーです。"),
        ("- **直し方:** RustNote のフォルダでは、`+stable` を付けずに `cargo check --locked` とします。", "- **直し方:** RustNote のフォルダでは、`+1.90.0` を付けずに `cargo check --locked` とします。"),
    ],
    "parts/outro.md": [
        ("版が古い（`requires rustc 1.98`）", "版が古い（`requires rustc 1.99`）"),
    ],
}
for name, pairs in edits.items():
    p = H / name
    s = p.read_text(encoding="utf-8")
    for a, b in pairs:
        assert s.count(a) == 1, (name, a[:40], s.count(a))
        s = s.replace(a, b)
    p.write_text(s, encoding="utf-8", newline="\n")
print("ok")

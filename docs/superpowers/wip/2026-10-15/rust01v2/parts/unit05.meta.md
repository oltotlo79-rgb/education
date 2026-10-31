title: MSVC Build Tools と CPU の種類
---checklist
- ☐ Windows の Rust が、MSVC の `link.exe` で実行ファイルを綴じることと、VS Code だけでは足りないことを説明できる（UNIT 05）
- ☐ `vswhere.exe` と MSVC のフォルダで、Build Tools の名前・版・ARM64 用の道具（`Hostarm64`）を確かめられる（UNIT 05）
- ☐ 実行ファイルの印（ARM64 は `0xAA64`、x64 は `0x8664`）を読み、x64 用の道具がエミュレーションで動いていることに気づける（UNIT 05）
- ☐ `rustup target add` と `--target` で別の CPU 用の実行ファイルを作れる。「can't find crate for std」「linker not found」を読んで直せる（UNIT 05）
---glossary
| MSVC | Microsoft Visual C++。Windows で実行ファイルを作る道具一式。リンカー `link.exe` が入っている |
| Build Tools for Visual Studio | Visual Studio を開かずに、MSVC の道具だけを入れる製品 |
| vswhere.exe | 入っている Visual Studio・Build Tools を答える道具 |
| target | 作る実行ファイルが、どの CPU・OS 用か。`aarch64-pc-windows-msvc` など |
| host | rustc が動いている PC の target。`rustc -vV` の `host:` |
| `0xAA64`・`0x8664` | 実行ファイルの中の、CPU の種類の印。ARM64 と x64 |

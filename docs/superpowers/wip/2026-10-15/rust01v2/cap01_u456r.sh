#!/usr/bin/env bash
# 第1巻 UNIT 04〜06 の取り直し。作業フォルダを、UNIT 01 と同じ ~\rust-study にする（終わったら消す）。
# Rust は、PC の Rust を変えないよう、別のフォルダ（~\rsb）に入れたものを使う。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
H="$S/rust01"; O="$H/out"
W=/c/Users/oltot/rust-study
export PYTHONIOENCODING=utf-8
mask() { python "$H/mask01.py"; }
psb() { powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; \$env:RUSTUP_HOME = 'C:\Users\oltot\rsb\rustup'; \$env:CARGO_HOME = 'C:\Users\oltot\rsb\cargo'; \$env:Path = 'C:\Users\oltot\rsb\cargo\bin;' + (((\$env:Path -split ';') | Where-Object { \$_ -notmatch 'cargo' }) -join ';'); $1" 2>&1 | mask; }
psbx() { psb "$1; 'exit=' + \$LASTEXITCODE"; }
pnone() { powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; \$env:Path = ((\$env:Path -split ';') | Where-Object { \$_ -notmatch 'cargo' }) -join ';'; $1" 2>&1 | mask; }
HELLO='C:\Users\oltot\rust-study\hello'
Q='C:\Users\oltot\rust-study\quality'

rm -rf "$W"; mkdir -p "$W"
# 取り直しで、もう一度「入っていない」状態から見るために、前に足した物を外す
psb "rustup toolchain uninstall 1.98.0; rustup toolchain uninstall 1.99.1; rustup target remove x86_64-pc-windows-msvc" > /dev/null

# ================= UNIT 04
pnone "rustc --version" > "$O/u04_notfound.txt"
psb "rustc --version; cargo --version; rustup --version; where.exe rustc cargo rustup" > "$O/u04_versions.txt"
psb "rustup show" > "$O/u04_show.txt"
psb "rustup component list --installed" > "$O/u04_components.txt"
# 受付の3つの .exe（この本の PC の、いつもの .cargo\bin）
powershell -NoProfile -Command "Get-ChildItem ~\.cargo\bin\rustup.exe, ~\.cargo\bin\rustc.exe, ~\.cargo\bin\cargo.exe | ForEach-Object { '{0,-10} {1}' -f \$_.Name, (Get-FileHash \$_.FullName).Hash.Substring(0, 16) }" 2>&1 | mask > "$O/u04_proxies.txt"

psb "Set-Location C:\Users\oltot\rust-study; cargo new hello" > "$O/u04_new.txt"
( cd "$W/hello" && find . -type f -not -path "./.git/*" | sort; [ -d .git ] && echo "（.git フォルダもある）" ) > "$O/u04_tree.txt"
tr -d '\r' < "$W/hello/Cargo.toml" > "$O/u04_cargo_toml.txt"
tr -d '\r' < "$W/hello/src/main.rs" > "$O/u04_main_rs.txt"
tr -d '\r' < "$W/hello/.gitignore" > "$O/u04_gitignore.txt"
psb "Set-Location $HELLO; cargo run" > "$O/u04_run.txt"
psb "Set-Location $HELLO; cargo run" > "$O/u04_run2.txt"
psb "Set-Location $HELLO; Get-ChildItem target\debug -Filter *.exe | Select-Object Name, Length | Format-Table -AutoSize | Out-String; .\target\debug\hello.exe" > "$O/u04_exe.txt"

start=$(date +%s)
psb "& '$(cygpath -w "$H/u04_old.ps1")'" | sed -n '/^----$/,$p' | sed 1d > "$O/u04_pin_missing.txt"
echo "（かかった時間: $(( $(date +%s) - start )) 秒）" >> "$O/u04_pin_missing.txt"
psb "Set-Location $HELLO; rustup show active-toolchain; Set-Location C:\Users\oltot\rust-study\old; rustup show active-toolchain" > "$O/u04_pin_compare.txt"
# 問2: old の中に作ったプロジェクトでは
psb "Set-Location C:\Users\oltot\rust-study\old; cargo new -q pinned; Set-Location pinned; rustc --version" > "$O/u04_q2.txt"

# ================= UNIT 05
cp "$H/u05_machine.ps1" "$W/machine.ps1"
psb "Set-Location $HELLO; cargo build; & ..\machine.ps1 target\debug\hello.exe" > "$O/u05_host_build.txt"
psb "Set-Location $HELLO; cargo build --target x86_64-pc-windows-msvc" > "$O/u05_target_missing.txt"
psb "rustup target add x86_64-pc-windows-msvc" > "$O/u05_target_add.txt"
psb "Set-Location $HELLO; cargo build --target x86_64-pc-windows-msvc; & ..\machine.ps1 target\debug\hello.exe target\x86_64-pc-windows-msvc\debug\hello.exe; .\target\x86_64-pc-windows-msvc\debug\hello.exe" > "$O/u05_x64_build.txt"
psb "Set-Location $HELLO; Get-ChildItem target -Directory -Name" > "$O/u05_target_dirs.txt"
psb "rustup target list --installed" > "$O/u05_target_list.txt"
psb "& C:\Users\oltot\rust-study\machine.ps1 (Get-Command rustc).Source \"\$env:LOCALAPPDATA\Programs\Microsoft VS Code\Code.exe\" (Get-Command git).Source 'C:\Program Files\Git\usr\bin\bash.exe' (Get-Command docker).Source \"\$HOME\.fly\bin\fly.exe\"" > "$O/u05_tools_machine.txt"
psb "Set-Location $HELLO; \$env:RUSTFLAGS = '-C linker=link-missing.exe'; cargo build" > "$O/u05_linker_missing.txt"

# ================= UNIT 06
psbx "Set-Location C:\Users\oltot\rust-study; cargo new --lib quality" > "$O/u06_new.txt"
cp "$H/u06src/lib_before.rs" "$W/quality/src/lib.rs"
psbx "Set-Location $Q; cargo check" > "$O/u06_check.txt"
psbx "Set-Location $Q; cargo fmt --check" > "$O/u06_fmt_check.txt"
psbx "Set-Location $Q; cargo fmt; cargo fmt --check" > "$O/u06_fmt_after.txt"
tr -d '\r' < "$W/quality/src/lib.rs" | sed -n 1,9p > "$O/u06_lib_formatted.txt"
psbx "Set-Location $Q; cargo clippy" > "$O/u06_clippy.txt"
psbx "Set-Location $Q; cargo clippy -- -D warnings" > "$O/u06_clippy_deny.txt"
psbx "Set-Location $Q; cargo test" > "$O/u06_test_fail.txt"
cp "$H/u06src/lib_after.rs" "$W/quality/src/lib.rs"
psb "Set-Location $Q; cargo fmt --check; 'fmt: ' + \$LASTEXITCODE; cargo clippy -q -- -D warnings; 'clippy: ' + \$LASTEXITCODE; cargo test -q; 'test: ' + \$LASTEXITCODE" > "$O/u06_all_ok.txt"
psbx "rustup toolchain install 1.99.1 --profile minimal" > "$O/u06_minimal_install.txt"
psbx "Set-Location $Q; cargo +1.99.1 fmt --check" > "$O/u06_fmt_missing.txt"
psbx "Set-Location $Q; cargo +1.99.1 clippy" > "$O/u06_clippy_missing.txt"
psbx "rustup component add rustfmt clippy --toolchain 1.99.1" > "$O/u06_component_add.txt"
psbx "Set-Location $Q; cargo +1.99.1 fmt --check" > "$O/u06_fmt_fixed.txt"
echo done

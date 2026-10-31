#!/usr/bin/env bash
# 第1巻 UNIT 04 の出力を取る。PC の Rust を変えないよう、別のフォルダ（C:\Users\<名前>\r01\sb）に入れた rustup で試す。
# r01 は、スクラッチの rust01 への短いつなぎ（ジャンクション）。パスを短くしないと、リンクで LNK1104 になる。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
H="$S/rust01"; O="$H/out"
R=/c/Users/oltot/rsb
export PYTHONIOENCODING=utf-8
mask() { python "$H/mask01.py"; }
# 別のフォルダの Rust だけを使う PowerShell（PATH から いつもの .cargo\bin を抜き、sb\cargo\bin を足す）
psb() { powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; \$env:RUSTUP_HOME = 'C:\Users\oltot\rsb\rustup'; \$env:CARGO_HOME = 'C:\Users\oltot\rsb\cargo'; \$env:Path = 'C:\Users\oltot\rsb\cargo\bin;' + (((\$env:Path -split ';') | Where-Object { \$_ -notmatch 'cargo' }) -join ';'); $1" 2>&1 | mask; }
# Rust の入っていない PowerShell（PATH から .cargo\bin を全部抜く）
pnone() { powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; \$env:Path = ((\$env:Path -split ';') | Where-Object { \$_ -notmatch 'cargo' }) -join ';'; $1" 2>&1 | mask; }

# ---- エラー: PATH に無い
pnone "rustc --version" > "$O/u04_notfound.txt"

# ---- 例4-2: 入った版を確かめる
psb "rustc --version; cargo --version; rustup --version; where.exe rustc cargo rustup" > "$O/u04_versions.txt"
psb "rustup show" > "$O/u04_show.txt"
psb "rustup component list --installed" > "$O/u04_components.txt"

# ---- 例4-3: 最初のプロジェクト
rm -rf "$R/w"; mkdir -p "$R/w"
psb "Set-Location C:\Users\oltot\rsb\w; cargo new hello" > "$O/u04_new.txt"
( cd "$R/w/hello" && find . -type f -not -path "./.git/*" | sort ) > "$O/u04_tree.txt"
tr -d '\r' < "$R/w/hello/Cargo.toml" > "$O/u04_cargo_toml.txt"
tr -d '\r' < "$R/w/hello/src/main.rs" > "$O/u04_main_rs.txt"
psb "Set-Location C:\Users\oltot\rsb\w\hello; cargo run" > "$O/u04_run.txt"
psb "Set-Location C:\Users\oltot\rsb\w\hello; cargo run" > "$O/u04_run2.txt"
psb "Set-Location C:\Users\oltot\rsb\w\hello; Get-ChildItem target\debug -Filter *.exe | Select-Object Name, Length | Format-Table -AutoSize | Out-String; .\target\debug\hello.exe" > "$O/u04_exe.txt"

# ---- 例4-4: フォルダごとに版を決める（rust-toolchain.toml）。入っていない版を書いたとき
psb "rustup toolchain uninstall 1.98.0" > /dev/null
mkdir -p "$R/w/old"; printf '[toolchain]\nchannel = "1.98.0"\n' > "$R/w/old/rust-toolchain.toml"
start=$(date +%s)
psb "Set-Location C:\Users\oltot\rsb\w\old; rustup show active-toolchain; rustc --version" > "$O/u04_pin_missing.txt"
echo "（かかった時間: $(( $(date +%s) - start )) 秒）" >> "$O/u04_pin_missing.txt"
psb "Set-Location C:\Users\oltot\rsb\w\hello; rustup show active-toolchain; Set-Location C:\Users\oltot\rsb\w\old; rustup show active-toolchain" > "$O/u04_pin_compare.txt"
psb "rustup toolchain list" > "$O/u04_toolchains_sb.txt"
# この本の PC（いつもの Rust）の一覧
( cd ~ && rustup toolchain list ) 2>&1 | mask > "$O/u04_toolchains_pc.txt"
echo done

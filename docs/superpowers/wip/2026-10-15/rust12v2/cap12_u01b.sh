#!/usr/bin/env bash
# 第12巻 UNIT 01 の取り直しの補い: 比べる道具を本文と同じ相対パスで動かす・束の無いエラーを空のフォルダで確かめる。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
CAP="bash $H/cap.sh"
O="$H/out"
export PYTHONIOENCODING=utf-8
mask() { python "$H/mask12.py" | tr -d '\r'; }
cd "$H/rustnote" || exit 1
mkdir -p builds
cp "$H"/builds/*.exe builds/
$CAP u01_report_normal cargo run -q --example u01_exe_report -- builds/build1.exe builds/build2.exe > /dev/null
$CAP u01_report_brepro cargo run -q --example u01_exe_report -- builds/brepro1.exe builds/brepro2.exe > /dev/null
$CAP u01_report_remap cargo run -q --example u01_exe_report -- builds/remap.exe > /dev/null
rm -rf builds
# エラー1: release の束が無い状態で、空のフォルダから起動する
rm -rf "$H/target/release/assets"
$CAP u01_bundle_debug topcoat asset bundle > /dev/null
R="$H/run_wrong"; rm -rf "$R"; mkdir -p "$R"
( cd "$R" && "$H/target/release/rustnote.exe" > /dev/null 2> "$R/.err" ); echo "（終了コード: $?）" >> "$R/.err"
{ mask < "$R/.err"; echo "（このあとのフォルダの中: $(ls -1 "$R" | tr '\n' ' ')）"; } > "$O/u01_err_wrongbundle.txt"
rm -rf "$R"
CARGO_TARGET_DIR="$H/target" topcoat asset bundle --release > /dev/null 2>&1
echo done

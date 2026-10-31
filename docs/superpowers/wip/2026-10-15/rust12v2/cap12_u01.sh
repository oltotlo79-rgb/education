#!/usr/bin/env bash
# 第12巻 UNIT 01 の出力を取り直す（Rust 1.99.1・topcoat 0.11.0・今の RustNote）。
# 何も無い target から、debug と release を作るところから始める。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
CAP="bash $H/cap.sh"
O="$H/out"
export PYTHONIOENCODING=utf-8
cd "$H/rustnote" || exit 1
check() {
  for f in src tests migrations assets Cargo.toml Cargo.lock rust-toolchain.toml Dockerfile; do
    diff -rq "$f" "$REAL/$f" > /dev/null || { echo "教材と違う: $f"; exit 1; }
  done
  echo "教材と同じ"
}
check
mask() { python "$H/mask12.py" | tr -d '\r'; }
stop_all() { taskkill //F //IM rustnote.exe > /dev/null 2>&1; sleep 1; }
stop_all
rm -rf "$H/target" "$H/target-brepro" "$H/target-remap" "$H/builds" "$H/dist" "$H/run"
mkdir -p "$H/builds"

# ---- 例1-1: 何も無い状態から release と debug を作る
$CAP u01_release cargo build --release --locked > /dev/null
grep -c "Compiling" "$O/u01_release.txt" > "$O/u01_release_count.txt"
$CAP u01_debug cargo build --locked > /dev/null
$CAP u01_bundle topcoat asset bundle --release > /dev/null
# 起動（今いるフォルダに rustnote.db ができる）
mkdir -p "$H/run"
( cd "$H/run" && "$H/target/release/rustnote.exe" > "$H/run/stdout" 2> "$H/run/stderr" & )
for i in $(seq 1 40); do sleep 0.25; curl.exe -s -o /dev/null http://127.0.0.1:3000/health && break; done
curl.exe -s -i http://127.0.0.1:3000/health | tr -d '\r' > "$O/u01_run_health.txt"
# ---- エラー2: もう1つ起動する（ポートが使われている）
( cd "$H/run" && "$H/target/release/rustnote.exe" > "$H/run/stdout2" 2> "$H/run/stderr2" )
echo "（終了コード: $?）" >> "$H/run/stderr2"
mask < "$H/run/stderr2" > "$O/u01_err_port.txt"
stop_all
mask < "$H/run/stderr" > "$O/u01_run_server.txt"
ls -1 "$H/run" | grep -v -E '^(stdout|stderr|stdout2|stderr2)$' > "$O/u01_run_files.txt"
# 大きさ
{
  echo "debug: $(stat -c %s "$H/target/debug/rustnote.exe") バイト"
  echo "release: $(stat -c %s "$H/target/release/rustnote.exe") バイト"
} > "$O/u01_sizes.txt"

# ---- 例1-2: ハッシュ化の速さ
$CAP u01_speed_debug cargo run -q --example u01_hash_speed > /dev/null
$CAP u01_speed_release cargo run -q --release --example u01_hash_speed > /dev/null

# ---- 例1-3: 同じソースから2回作って比べる
cp "$H/target/release/rustnote.exe" "$H/builds/build1.exe"
cargo clean --release -p rustnote --target-dir "$H/target" > /dev/null 2>&1
CARGO_TARGET_DIR="$H/target" cargo build --release --locked > /dev/null 2>&1
cp "$H/target/release/rustnote.exe" "$H/builds/build2.exe"
$CAP u01_report_normal cargo run -q --example u01_exe_report -- "$H/builds/build1.exe" "$H/builds/build2.exe" > /dev/null
python "$H/diffpos12.py" "$H/builds/build1.exe" "$H/builds/build2.exe" > "$O/u01_diffpos.txt"
# /Brepro
for n in 1 2; do
  RUSTFLAGS="-C link-arg=/Brepro" cargo build --release --locked --target-dir "$H/target-brepro" > /dev/null 2>&1
  cp "$H/target-brepro/release/rustnote.exe" "$H/builds/brepro$n.exe"
  cargo clean --release -p rustnote --target-dir "$H/target-brepro" > /dev/null 2>&1
done
$CAP u01_report_brepro cargo run -q --example u01_exe_report -- "$H/builds/brepro1.exe" "$H/builds/brepro2.exe" > /dev/null
# --remap-path-prefix
RUSTFLAGS="-C link-arg=/Brepro --remap-path-prefix=$USERPROFILE=~" cargo build --release --locked --target-dir "$H/target-remap" > /dev/null 2>&1
cp "$H/target-remap/release/rustnote.exe" "$H/builds/remap.exe"
$CAP u01_report_remap cargo run -q --example u01_exe_report -- "$H/builds/remap.exe" > /dev/null

# ---- エラー1: --release を付けずに束を作った
rm -rf "$H/target/release/assets"
$CAP u01_bundle_debug topcoat asset bundle > /dev/null
( cd "$H/run" && "$H/target/release/rustnote.exe" > /dev/null 2> "$H/run/stderr3" ); echo "（終了コード: $?）" >> "$H/run/stderr3"
mask < "$H/run/stderr3" > "$O/u01_err_wrongbundle.txt"
echo "（このあとのフォルダの中: $(ls -1 "$H/run" | grep -v -E '^(stdout|stderr)[0-9]*$' | tr '\n' ' ')）" >> "$O/u01_err_wrongbundle.txt"
CARGO_TARGET_DIR="$H/target" topcoat asset bundle --release > /dev/null 2>&1

# ---- エラー3: 整数のあふれ
$CAP u01_overflow_debug cargo run -q --example u01_overflow > /dev/null
$CAP u01_overflow_release cargo run -q --release --example u01_overflow > /dev/null
CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true $CAP u01_overflow_release_checks cargo run -q --release --example u01_overflow > /dev/null

# ---- 問1: 実行ファイルだけを dist にコピーする
mkdir -p "$H/dist"
cp "$H/target/release/rustnote.exe" "$H/dist/"
( cd "$H/dist" && ./rustnote.exe > /dev/null 2> "$H/dist/err" ); echo "（終了コード: $?）" >> "$H/dist/err"
mask < "$H/dist/err" > "$O/u01_q1_noassets.txt"; rm -f "$H/dist/err"
cp -r "$H/target/release/assets" "$H/dist/assets"
( cd "$H/dist" && ./rustnote.exe > /dev/null 2> "$H/dist/err" & )
for i in $(seq 1 40); do sleep 0.25; curl.exe -s -o /dev/null http://127.0.0.1:3000/health && break; done
echo "（/health: $(curl.exe -s http://127.0.0.1:3000/health)）" > "$H/q1_health"
stop_all
{ mask < "$H/dist/err"; cat "$H/q1_health"; } > "$O/u01_q1_server.txt"
rm -f "$H/dist/err" "$H/q1_health"
echo "（dist の中: $(ls -1 "$H/dist" | tr '\n' ' ')）" >> "$O/u01_q1_server.txt"

# ---- 問2
$CAP u01_q2_debug cargo run -q --example u01_q2 > /dev/null
$CAP u01_q2_release cargo run -q --release --example u01_q2 > /dev/null

# ---- 問3: 記録のファイル（Git で管理したコピーで、本文の PowerShell をそのまま動かす）
R="$H/recdemo/rustnote"
rm -rf "$H/recdemo"; mkdir -p "$R/target/release"
cp -r src tests migrations assets Cargo.toml Cargo.lock rust-toolchain.toml rustfmt.toml "$R/"
cp -p "$H/target/release/rustnote.exe" "$R/target/release/"
printf 'target/\n' > "$R/.gitignore"
(cd "$R" && git init -q && git add -A && git -c user.name=rustnote -c user.email=rustnote@example.invalid commit -q -m "release の記録の練習")
cat > "$R/record.ps1" <<'PS'
$exe = "target\release\rustnote.exe"
@(
    "commit:     $(git rev-parse --short HEAD)"
    "rustc:      $(rustc --version)"
    "Cargo.lock: $((Get-FileHash Cargo.lock).Hash)"
    "rustnote:   $((Get-FileHash $exe).Hash)"
    "built at:   $((Get-Item $exe).LastWriteTimeUtc.ToString('u'))"
) | Set-Content -Encoding utf8 release-record.txt
Get-Content release-record.txt
PS
(cd "$R" && powershell -NoProfile -ExecutionPolicy Bypass -File record.ps1) | tr -d '\r' | sed 's/^\xEF\xBB\xBF//' > "$O/u01_q3_record.txt"
python "$H/linktime12.py" "$R/target/release/rustnote.exe" > "$O/u01_q3_linktime.txt"

check
echo done

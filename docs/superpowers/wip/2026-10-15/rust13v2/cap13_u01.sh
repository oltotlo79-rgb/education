#!/usr/bin/env bash
# 第13巻 UNIT 01 の出力を、今の RustNote（Rust 1.99.1・Topcoat 0.11.0・tokio 1.54.1）で取り直す。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
CAP="bash $H/cap.sh"
E="python $H/../rust10/edit_src.py"
T() { printf 'import runpy, os\nREPLACEMENTS = runpy.run_path(os.path.join(os.path.dirname(__file__), "e13_tables.py"))["%s"]\n' "$1" > "$H/e13_$1.py"; echo "$H/e13_$1.py"; }
cd "$H/rustnote" || exit 1
restore() { cp "$REAL/Cargo.toml" Cargo.toml; cp "$REAL/Cargo.lock" Cargo.lock; cp "$REAL/src/lib.rs" src/lib.rs; }

# 教材の RustNote と同じにする（examples/ の u02_*.rs は、この巻の UNIT 02 の確かめ用に残す）
for f in Cargo.toml Cargo.lock rust-toolchain.toml rustfmt.toml Dockerfile ci.yml fly.toml README.md .gitignore .env.example .dockerignore; do cp "$REAL/$f" "$f"; done
for d in src migrations assets tests; do rm -rf "$d"; cp -r "$REAL/$d" "$d"; done
diff -rq src "$REAL/src" > /dev/null && echo "src は教材と同じ"

# ---- 例1-3: 決めた版でビルドし、3つの確かめ（組み立ての行を全部数えるため、target を空にしてから）
rm -rf "$H/target"
$CAP intro_toolchain rustup show active-toolchain > /dev/null
$CAP u01_build cargo build --locked > /dev/null
grep -c "^   Compiling" "$H/out/u01_build.txt" > "$H/out/u01_build_count.txt"
$CAP u01_fmt cargo fmt --check > /dev/null
$CAP u01_clippy cargo clippy --all-targets --locked -- -D warnings > /dev/null
$CAP u01_test cargo test --locked > /dev/null

# ---- 例1-4・1-6: 依存の一覧と、tokio の機能
$CAP u01_tree cargo tree --locked --depth 1 > /dev/null
$CAP u01_tokio_features cargo tree --locked -p tokio --depth 0 -e normal -f "{p} [{f}]" > /dev/null
$CAP u01_tree_thiserror cargo tree --locked -i thiserror@1.0.69 --edges normal > /dev/null
# RustNote から macros を消しても、ほかのクレートのおかげで組み立てられるか
$E Cargo.toml "$(T NO_MACROS)" > /dev/null
$CAP u01_nomacros_rustnote cargo build --locked > /dev/null
restore

# ---- 例1-5: "=1.53.3" と "1.53.3"（tokio を1つ前の版に固定し直してから試す）
$E Cargo.toml "$(T TOKIO_OLD_PIN)" > /dev/null
$CAP u01_downgrade cargo update -p tokio --precise 1.53.3 > /dev/null
$CAP u01_update_pinned cargo update -p tokio --dry-run > /dev/null
$E Cargo.toml "$(T TOKIO_OLD_CARET)" > /dev/null
$CAP u01_update_caret cargo update -p tokio --dry-run > /dev/null
restore

# ---- エラー1〜4
$CAP u01_err_msrv cargo +stable check --locked > /dev/null
$E Cargo.toml "$(T TYPO)" > /dev/null
$CAP u01_err_typo cargo build --locked > /dev/null
restore
$E Cargo.toml "$(T DOTENVY_OLD)" > /dev/null
$CAP u01_err_locked cargo build --locked > /dev/null
restore
$E Cargo.lock "$(T CHECKSUM)" > /dev/null
$CAP u01_err_checksum cargo build --locked > /dev/null
restore

# ---- エラー5: feat_demo（tokio だけ・macros なし → あり）
cp "$H/feat_demo/Cargo.toml" "$H/feat_demo_keep.toml"
cp "$H/snippets/feat_demo_wrong.toml" "$H/feat_demo/Cargo.toml"
WORKDIR=feat_demo $CAP u01_err_nomacros cargo build > /dev/null
cp "$H/feat_demo_keep.toml" "$H/feat_demo/Cargo.toml"; rm "$H/feat_demo_keep.toml"
WORKDIR=feat_demo $CAP u01_fixed_macros cargo run -q > /dev/null

# ---- 問1・問2
cat "$H/snippets/q1_snippet.rs" >> src/lib.rs
$CAP u01_q1_devdep cargo build --locked > /dev/null
restore
$CAP u01_tree_ring cargo tree --locked -i ring --edges normal > /dev/null

# ---- 問3: git check-ignore
rm -rf "$H/gitdemo" && mkdir -p "$H/gitdemo/rustnote"
cd "$H/gitdemo/rustnote" || exit 1
for f in Cargo.toml Cargo.lock rust-toolchain.toml rustfmt.toml Dockerfile ci.yml fly.toml README.md .gitignore .env.example .dockerignore; do cp "$REAL/$f" .; done
for d in src migrations assets tests; do cp -r "$REAL/$d" .; done
git init -q
cp .env.example .env
printf 'x\n' > rustnote.db
WORKDIR=gitdemo/rustnote $CAP u01_q3_checkignore git check-ignore -v .env rustnote.db rustnote.db-wal target/debug/rustnote.exe .env.example Cargo.lock > /dev/null
WORKDIR=gitdemo/rustnote $CAP u01_q3_status git status --short > /dev/null

cd "$H/rustnote" && diff -rq src "$REAL/src" > /dev/null && diff -q Cargo.toml "$REAL/Cargo.toml" > /dev/null && diff -q Cargo.lock "$REAL/Cargo.lock" > /dev/null && echo "教材と同じに戻した"

#!/usr/bin/env bash
# 第13巻 UNIT 10 の出力を取る（テストと CI の手順）。Docker の分は cap13_u10_docker.sh。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
CAP="bash $H/cap.sh"
O="$H/out"
cd "$H/rustnote" || exit 1
check() { diff -rq src "$REAL/src" > /dev/null && diff -rq tests "$REAL/tests" > /dev/null && diff -q Cargo.toml "$REAL/Cargo.toml" > /dev/null && diff -q Cargo.lock "$REAL/Cargo.lock" > /dev/null && echo "src・tests・Cargo.toml・Cargo.lock は教材と同じ" || { echo "教材と違う"; exit 1; }; }
check
export CARGO_TARGET_DIR="$H/target"
export PYTHONIOENCODING=utf-8
# 練習のプログラム（examples/）は教材に無いので、CI の手順はそれを除いた教材のコピーで動かす
CI="$H/u10ci"
rm -rf "$CI"; mkdir -p "$CI"
cp -r "$REAL/src" "$REAL/tests" "$REAL/migrations" "$REAL/assets" "$CI/"
cp "$REAL/Cargo.toml" "$REAL/Cargo.lock" "$REAL/rust-toolchain.toml" "$REAL/rustfmt.toml" "$CI/"
export WORKDIR=u10ci

# ---- 例10-?: CI と同じ3つの手順
$CAP u10_fmt cargo fmt --check > /dev/null
$CAP u10_clippy cargo clippy --all-targets --locked -- -D warnings > /dev/null
$CAP u10_test cargo test --locked > /dev/null

# ---- 並列とそうでないときの時間（HTTP のテスト）
{
  for threads in default 1; do
    if [ "$threads" = default ]; then args=""; else args="--test-threads=1"; fi
    start=$(date +%s.%N)
    (cd "$CI" && cargo test -q --locked --test http -- $args > /dev/null 2>&1)
    end=$(date +%s.%N)
    python -c "import sys; print(f'--test-threads={sys.argv[1]}: {float(sys.argv[3]) - float(sys.argv[2]):.2f} 秒')" "$threads" "$start" "$end"
  done
} | tr -d '\r' > "$O/u10_threads.txt"

# ---- 問1: テストの DB は、どこに作られて、いつ消えるか
cat > "$CI/tests/u10_tempdir.rs" <<'EOF'
//! 第13巻 UNIT 10 問1: test_db() の一時フォルダが、どこに作られ、いつ消えるかを確かめる。
mod common;

#[tokio::test]
async fn temp_dir_is_removed_on_drop() {
    let test = common::test_db().await;
    let dir = test.db_dir();
    println!("一時フォルダの名前: {}", dir.file_name().unwrap().to_string_lossy().chars().take(4).collect::<String>());
    println!("DB のファイルがある: {}", dir.join("test.db").exists());
    drop(test);
    println!("drop のあと、フォルダがある: {}", dir.exists());
}
EOF
python - "$CI/tests/common/mod.rs" <<'PY'
import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
old = "/// マイグレーション済みの空の DB を作る"
assert s.count(old) == 1
new = """impl TestDb {
    /// （問1の実験のために足した）一時フォルダの場所
    pub fn db_dir(&self) -> std::path::PathBuf {
        self._dir.path().to_path_buf()
    }
}

""" + old
open(p, "w", encoding="utf-8", newline="").write(s.replace(old, new))
PY
$CAP u10_q1 cargo test -q --test u10_tempdir -- --nocapture > /dev/null

rm -rf "$CI"
check

#!/usr/bin/env bash
# 第13巻 UNIT 05 の出力を取る（リポジトリ）。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
CAP="bash $H/cap.sh"
cd "$H/rustnote" || exit 1
check() { diff -rq src "$REAL/src" > /dev/null && diff -rq migrations "$REAL/migrations" > /dev/null && diff -rq tests "$REAL/tests" > /dev/null && echo "src・migrations・tests は教材と同じ" || { echo "教材と違う"; exit 1; }; }
check
export CARGO_TARGET_DIR="$H/target"
cargo build -q --examples || exit 1

# ---- 例5-3: 3つのリポジトリを動かす
$CAP u05_repo cargo run -q --example u05_repo > /dev/null
# ---- escape_like のユニットテスト と リポジトリの結合テスト
$CAP u05_lib_tests cargo test --lib -- repository:: > /dev/null
$CAP u05_repo_tests cargo test --test repository > /dev/null
# ---- 問1
$CAP u05_q1 cargo run -q --example u05_q1 > /dev/null

# ---- エラー1〜3
for e in e05_dyn e05_await e05_params; do
  cp "$H/errs_u05/$e.rs" examples/
  $CAP "u05_err_${e#e05_}" cargo run -q --example "$e" > /dev/null
  rm "examples/$e.rs"
done

# 差し替え: python edit.py <ファイル> <古い> <新しい>（古いものがちょうど1回あること）
edit() {
  python - "$@" <<'PY'
import sys
p, old, new = sys.argv[1], sys.argv[2], sys.argv[3]
s = open(p, encoding="utf-8").read()
assert s.count(old) == 1, old
open(p, "w", encoding="utf-8", newline="").write(s.replace(old, new))
PY
}

# ---- 問2: find の SQL から、持ち主の条件を消す
edit src/repository/notes.rs '        let sql = format!("SELECT {NOTE_COLUMNS} FROM notes WHERE id = ?1 AND user_id = ?2");
        let mut rows = conn
            .query(&sql, libsql::params![id.get(), owner.get()])
            .await?;' '        let sql = format!("SELECT {NOTE_COLUMNS} FROM notes WHERE id = ?1");
        let _ = owner; // 持ち主の条件を消した（問2の実験）
        let mut rows = conn.query(&sql, libsql::params![id.get()]).await?;'
$CAP u05_q2_run cargo run -q --example u05_repo > /dev/null
$CAP u05_q2_test cargo test -q --test repository notes_are_isolated_per_owner > /dev/null
cp "$REAL/src/repository/notes.rs" src/repository/notes.rs

# ---- 問3: update で 0 行のとき、find で見分けずに、いつも VersionMismatch にする
edit src/repository/notes.rs '        match self.find(owner, id).await? {
            Some(_) => Err(RepoError::VersionMismatch),
            None => Err(RepoError::NotFound),
        }' '        Err(RepoError::VersionMismatch)'
$CAP u05_q3_run cargo run -q --example u05_repo > /dev/null
$CAP u05_q3_test cargo test -q --test repository > /dev/null
cp "$REAL/src/repository/notes.rs" src/repository/notes.rs

check

#!/usr/bin/env bash
# 第13巻 UNIT 02 の出力を、今の RustNote で取り直す。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
CAP="bash $H/cap.sh"
E="python $H/../rust10/edit_src.py"
T() { printf 'import runpy, os\nREPLACEMENTS = runpy.run_path(os.path.join(os.path.dirname(__file__), "e13b_tables.py"))["%s"]\n' "$1" > "$H/e13b_$1.py"; echo "$H/e13b_$1.py"; }
cd "$H/rustnote" || exit 1
check() { diff -rq src "$REAL/src" > /dev/null && diff -rq migrations "$REAL/migrations" > /dev/null && echo "src と migrations は教材と同じ" || { echo "教材と違う"; exit 1; }; }
check
rm -f try.db* broken.db* fresh.db*
cargo build -q --examples || exit 1

# 例2-2・2-3
$CAP u02_migrate cargo run -q --example u02_migrate -- try.db > /dev/null
$CAP u02_pragmas cargo run -q --example u02_pragmas > /dev/null

# エラー1: 0003 の列の名前を打ち間違えた
$E migrations/0003_sessions.sql "$(T TYPO_0003)" > /dev/null
$CAP u02_err_migration cargo run -q --example u02_migrate -- broken.db > /dev/null
$CAP u02_after_failure cargo run -q --example u02_state -- broken.db > /dev/null
cp "$REAL/migrations/0003_sessions.sql" migrations/
$CAP u02_recover cargo run -q --example u02_migrate -- broken.db > /dev/null

# エラー2: 適用済みの 0002 を書き換えた
$E migrations/0002_notes.sql "$(T PINNED_0002)" > /dev/null
$CAP u02_edit_applied cargo run -q --example u02_migrate -- try.db > /dev/null
$CAP u02_edit_fresh cargo run -q --example u02_migrate -- fresh.db > /dev/null
cp "$REAL/migrations/0002_notes.sql" migrations/

# 問1: 0004 を足す
cp "$H/snippets/0004_note_pinned.sql" migrations/
$E src/db.rs "$(T ADD_0004)" > /dev/null
$CAP u02_q1 cargo run -q --example u02_migrate -- try.db > /dev/null
$CAP u02_q1_tests cargo test --locked > /dev/null
cp "$REAL/src/db.rs" src/db.rs
rm migrations/0004_note_pinned.sql

# 問2・問3
$CAP u02_q2 cargo run -q --example u02_q2 > /dev/null
$CAP u02_q3 cargo run -q --example u02_q3 > /dev/null

rm -f try.db* broken.db* fresh.db*
check

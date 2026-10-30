#!/usr/bin/env bash
# 第10巻 UNIT 01〜02 の出力を、今の版（Rust 1.99.1・Topcoat 0.11）で取り直す。
# 使い方: bash cap10.sh
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
export VOL=rust10
export CARGO_TARGET_DIR="$S/target10"
CAP="bash $S/capx.sh"
cd "$S/rust10/rustnote" || exit 1
N=src/routes/notes.rs
cp $N /tmp/notes.rs.orig 2>/dev/null || cp $N "$S/rust10/notes.rs.orig"
ORIG="$S/rust10/notes.rs.orig"; [ -f /tmp/notes.rs.orig ] && ORIG=/tmp/notes.rs.orig

# 依存を先に作っておく（エラーの例で Compiling rustnote だけが出るように）
cargo build -q --all-targets || exit 1

$CAP u01_list cargo test --test u01_list -- --nocapture --test-threads=1 > /dev/null
$CAP u01_order cargo run -q --example u01_order > /dev/null
python - "$N" "for note in page.notes.clone()" "for note in page.notes" <<'PY'
import sys
p, a, b = sys.argv[1:]
s = open(p, encoding="utf-8").read()
assert s.count(a) == 1, a
open(p, "w", encoding="utf-8", newline="").write(s.replace(a, b))
PY
$CAP u01_err_moved cargo build > /dev/null
cp "$ORIG" $N
python - "$N" "for note in page.notes.clone()" "for note in &page.notes" <<'PY'
import sys
p, a, b = sys.argv[1:]
s = open(p, encoding="utf-8").read()
assert s.count(a) == 1, a
open(p, "w", encoding="utf-8", newline="").write(s.replace(a, b))
PY
$CAP u01_err_borrow cargo build > /dev/null
cp "$ORIG" $N
cmp $N /c/Users/oltot/Documents/git-projects/def/Rust/rustnote/$N && echo "notes.rs を元に戻した"
cargo build -q || exit 1
$CAP u01_q1 cargo test --test u01_q1 -- --nocapture > /dev/null
$CAP u01_q2 cargo run -q --example u01_q2 > /dev/null
$CAP u01_q3 cargo test --test u01_q3 -- --nocapture > /dev/null

$CAP u02_create cargo test --test u02_create -- --nocapture --test-threads=1 > /dev/null
$CAP u02_reload cargo test --test u02_reload -- --nocapture > /dev/null
cp "$S/rust10/errs/e02_string.rs" examples/e02_string.rs
$CAP u02_err_string cargo build --example e02_string > /dev/null
rm examples/e02_string.rs
$CAP u02_err_name cargo test --test u02_err_name -- --nocapture > /dev/null
$CAP u02_q1 cargo test --test u02_q1 -- --nocapture > /dev/null
$CAP u02_q2 cargo test --test u02_q2 -- --nocapture > /dev/null
$CAP u02_q3 cargo test --test u02_q3 -- --nocapture > /dev/null
diff -rq src /c/Users/oltot/Documents/git-projects/def/Rust/rustnote/src && echo "src は教材と同じ"

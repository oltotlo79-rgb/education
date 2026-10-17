#!/usr/bin/env bash
# Re-capture all outputs for intro + UNIT 01-06 with the pinned toolchain (rust-toolchain.toml)
S="$(cd "$(dirname "$0")" && pwd)"
D=$S/design_lab
cd $D && cargo build -q --bins 2>/dev/null
run() { name=$1; shift; touch $D/src/bin/$1.rs 2>/dev/null; "$S/cap.sh" "$name" "$D" "$@" >/dev/null; }
runbin() { touch $D/src/bin/$1.rs; "$S/cap.sh" "$1" "$D" run --bin "$1" >/dev/null; }
runq() { "$S/cap.sh" "$1" "$D" run -q --bin "$1" >/dev/null; }
for b in u01_ids u01_title u01_domain u02_trait u02_repo u03_generic u03_page u03_service u04_dyn u04_state u05_manual u05_app_error u06_mod; do runbin $b; done
for b in u01_ex1 u01_ex2 u02_ex1 u02_ex2 u02_ex3 u03_ex1 u03_ex2 u04_ex1 u04_ex2 u05_ex1 u05_ex2; do runq $b; done
for e in u01_err_swap u01_err_private u02_err_e0277 u02_err_e0046 u02_err_e0117 u03_err_e0369 u03_err_copy u03_err_e0282 u03_err_e0284 u04_err_e0038 u04_err_self u04_err_e0782 u04_err_send u05_err_from u05_err_e0004 u06_err_private; do "$S/caperr.sh" $e >/dev/null; done
# anyhow
touch $D/src/bin/u05_anyhow.rs; "$S/cap.sh" u05_anyhow_ok $D run --bin u05_anyhow -- seed.txt >/dev/null
touch $D/src/bin/u05_anyhow.rs; "$S/cap.sh" u05_anyhow_missing $D run --bin u05_anyhow -- missing.txt >/dev/null
"$S/cap.sh" u05_anyhow_empty $D run -q --bin u05_anyhow -- empty.txt >/dev/null
# unused Result warning
cp $S/errs/u05_warn_unused.rs $D/src/bin/; touch $D/src/bin/u05_warn_unused.rs; "$S/cap.sh" u05_warn_unused $D run --bin u05_warn_unused >/dev/null; rm $D/src/bin/u05_warn_unused.rs
# notes_app
N=$S/notes_app
cd $N && cargo build -q 2>/dev/null; touch src/main.rs src/lib.rs; "$S/cap.sh" u06_notes_app $N run >/dev/null
for pair in "u06_err_crate:u06_err_crate_main.rs" "u06_err_field:u06_err_field_main.rs"; do
  name=${pair%%:*}; f=${pair#*:}
  cp $S/errs/$f $N/src/main.rs; "$S/cap.sh" $name $N build >/dev/null; cp $S/errs/notes_main_backup.rs $N/src/main.rs
done
cp $S/errs/notes_lib_backup.rs $S/errs/notes_lib_e0583.rs
python - "$S/errs/notes_lib_e0583.rs" <<'PY'
import sys
p=sys.argv[1]; s=open(p,encoding='utf-8').read()
s=s.replace("mod repo;\n","mod repo;\n// まだファイルを作っていない module\nmod session;\n",1)
open(p,'w',encoding='utf-8').write(s)
PY
cp $S/errs/notes_lib_e0583.rs $N/src/lib.rs; "$S/cap.sh" u06_err_e0583 $N build >/dev/null; cp $S/errs/notes_lib_backup.rs $N/src/lib.rs
cd $N && cargo build -q && echo notes_app restored
echo done

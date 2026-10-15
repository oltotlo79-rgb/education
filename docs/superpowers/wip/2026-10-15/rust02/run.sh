#!/bin/bash
# usage: run.sh snips/uNN/name.rs ...   (stdin from name.in if present)
R=C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust02
export CARGO_HOME=$R/cargohome
for f in "$@"; do
  case "$f" in *.rs) ;; *) continue;; esac
  base="${f%.rs}"
  cp "$f" $R/basics/src/main.rs
  inp=/dev/null
  [ -f "$base.in" ] && inp="$base.in"
  ( cd $R/basics && cargo run --offline <"$inp" >"$R/$base.out" 2>"$R/$base.err.raw" )
  python $R/mask.py "$R/$base.err.raw" "$R/$base.err"
  rm -f "$R/$base.err.raw"
  echo "===== $f"
  echo "--- stdout"
  cat "$R/$base.out"
  echo "--- stderr"
  grep -v "^\s*Finished\|^\s*Running\|^\s*Compiling" "$R/$base.err"
done

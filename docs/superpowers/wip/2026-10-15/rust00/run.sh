#!/bin/bash
# usage: run.sh NAME [args...]   (copies ex/NAME.rs to practice/src/main.rs and cargo run)
S=/c/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust00
name=$1; shift
cp "$S/ex/$name.rs" "$S/study/practice/src/main.rs"
cd "$S/study/practice"
cargo run "$@" 2>&1 | PYTHONIOENCODING=utf-8 python "$S/mask.py"
echo "[exit=${PIPESTATUS[0]}]"

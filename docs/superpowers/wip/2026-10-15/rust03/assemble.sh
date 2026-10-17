#!/bin/sh
cd "$(dirname "$0")"
out="C:/Users/oltot/Documents/git-projects/def/Rust/03_Rust中核_所有権_借用_ライフタイム_スマートポインタ.md"
cat parts/00_intro.md parts/01.md parts/02.md parts/03.md parts/04.md parts/05.md parts/06.md parts/07.md parts/08.md parts/09.md parts/10.md parts/99_end.md | sed 's/\r$//' > "$out"
wc -l "$out"

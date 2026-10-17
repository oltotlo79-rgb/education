#!/usr/bin/env bash
# usage: runps.sh <name>   runs ps/<name>.ps1 (adds UTF-8 BOM copy), masks paths, saves out/<name>.txt
here="$(cd "$(dirname "$0")/.." && pwd)"
src="$here/ps/$1.ps1"
bom="$here/ps/_bom_$1.ps1"
printf '\xEF\xBB\xBF' > "$bom"; cat "$src" >> "$bom"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "$(cygpath -w "$bom")" 2>&1 | PYTHONIOENCODING=utf-8 python "$here/mask10.py" > "$here/out/$1.txt"
cat "$here/out/$1.txt"

#!/bin/bash
# usage: dom.sh URL  -> prints final DOM of <main> (after streaming/hydration) and console logs to console.txt
UDD=$(mktemp -d)
timeout 40 "/c/Program Files/Google/Chrome/Application/chrome.exe" --headless=new --disable-gpu --no-first-run --user-data-dir="$UDD" --enable-logging=stderr --v=0 --virtual-time-budget=8000 --dump-dom "$1" > C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/next04/tmp_dom_out.html 2> C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/next04/tmp_dom_err.txt
grep "INFO:CONSOLE" C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/next04/tmp_dom_err.txt | sed -E 's/^.*INFO:CONSOLE:[0-9]+\] "//; s/", source: .*$//' > C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/next04/tmp_console.txt
PYTHONIOENCODING=utf-8 python - <<'PY'
import re
s=open('C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/next04/tmp_dom_out.html',encoding='utf-8').read()
m=re.search(r'<main>.*?</main>',s,re.S)
t=m.group(0) if m else s[:2000]
t=re.sub(r'<!--.*?-->','',t)
print(t)
PY

#!/bin/bash
# usage: ps.sh "<powershell command>"   (runs in current dir, converts CP932 -> UTF-8)
powershell.exe -NoProfile -Command "$1" 2>&1 | python -c "import sys; sys.stdout.buffer.write(sys.stdin.buffer.read().decode('cp932', errors='replace').replace('\r','').encode('utf-8'))"

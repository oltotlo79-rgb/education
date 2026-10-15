#!/bin/bash
cd C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust02 && PYTHONIOENCODING=utf-8 python preview.py "$1" "$2" 2>&1 | grep -v "Using browser"

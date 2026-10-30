#!/usr/bin/env bash
cd /mnt/c/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust11/rustnote_mut || exit 1
cargo mutants --list --file src/repository/notes.rs 2>&1 | head -60

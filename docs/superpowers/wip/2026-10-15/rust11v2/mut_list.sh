#!/usr/bin/env bash
cd /mnt/c/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust11/rustnote || exit 1
cargo mutants --help | grep -n -i -- "--file\|--test-package\|-- \|additional\|test-tool\|--timeout\|--in-place\|cargo_test_args\|--jobs" | head -20
cargo mutants --list --file src/auth.rs 2>&1 | head -40

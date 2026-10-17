#!/usr/bin/env bash
# usage: serve.sh <logname> [ENV=VAL ...]   runs `cargo run` in the RustNote copy, log -> out/<logname>.raw
here="$(cd "$(dirname "$0")" && pwd)"
name="$1"; shift
cd "$here/rustnote" || exit 1
export CARGO_TARGET_DIR="$here/target"
env "$@" cargo +1.99.0 run > "$here/out/$name.raw" 2>&1

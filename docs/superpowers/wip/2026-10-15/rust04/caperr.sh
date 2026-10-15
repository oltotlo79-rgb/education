#!/usr/bin/env bash
# usage: caperr.sh <name>   (copies errs/<name>.rs into design_lab/src/bin, builds, captures, removes)
name="$1"
here="$(cd "$(dirname "$0")" && pwd)"
cp "$here/errs/$name.rs" "$here/design_lab/src/bin/$name.rs"
touch "$here/design_lab/src/bin/$name.rs"
"$here/cap.sh" "$name" "$here/design_lab" build --bin "$name"
rm -f "$here/design_lab/src/bin/$name.rs"

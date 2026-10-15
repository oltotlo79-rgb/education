#!/bin/sh
# usage: ./run.sh harness/run-xxx.tsx
name=$(basename "$1" .tsx)
npx esbuild "$1" --bundle --platform=node --format=esm --packages=external --outfile="out/$name.mjs" --log-level=warning --jsx=automatic && node --import ./harness/setup-entry.ts "out/$name.mjs"

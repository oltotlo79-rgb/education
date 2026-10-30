#!/usr/bin/env bash
# 第11巻 UNIT 05 の出力を取る（Playwright と、パソコンに入っている Chrome で）。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
export VOL=rust11
export CARGO_TARGET_DIR="$S/target11"
CAP="bash $S/capx.sh"
E="python $S/rust10/edit_src.py"
X="$S/rust11/errs"
E2E="$S/rust11/e2e"
cd "$S/rust11/rustnote" || exit 1
check() { diff -rq src "$REAL/src" > /dev/null && diff -rq migrations "$REAL/migrations" > /dev/null && echo "src と migrations は教材と同じ" || { echo "教材と違う"; exit 1; }; }
check
topcoat asset bundle > /dev/null && cargo build -q || exit 1
export RUSTNOTE_EXE="$S/target11/debug/rustnote.exe"

cd "$E2E" || exit 1
# capx.sh は $VOL/$WORKDIR で動かす
export WORKDIR=e2e
rm -rf node_modules package-lock.json test-results
export PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1
$CAP u05_install npm install --save-dev --save-exact @playwright/test@1.64.1 @axe-core/playwright@4.14.0 > /dev/null
$CAP u05_version npx playwright --version > /dev/null
$CAP u05_chrome node chrome_version.mjs > /dev/null

$CAP u05_flow npx playwright test tests/flow.spec.ts > /dev/null
$CAP u05_maxlength npx playwright test tests/maxlength.spec.ts > /dev/null
$CAP u05_a11y npx playwright test tests/a11y.spec.ts > /dev/null

# エラー1・2: 探し方の間違い
for f in e05_strict e05_missing; do
  cp "errs/$f.spec.ts" tests/
  $CAP "u05_err_${f#e05_}" npx playwright test "tests/$f.spec.ts" > /dev/null
  rm "tests/$f.spec.ts"
done
# エラー3: CSS の束（assets/）の無い実行ファイルで起動した
mkdir -p "$S/rust11/e2e_noassets" && cp "$RUSTNOTE_EXE" "$S/rust11/e2e_noassets/"
RUSTNOTE_EXE="$S/rust11/e2e_noassets/rustnote.exe" $CAP u05_err_noassets npx playwright test tests/flow.spec.ts > /dev/null

# 問1: 2つの nav に名前を付けた版で、もう一度 axe
cd "$S/rust11/rustnote"
$E src/views.rs "$X/e05_nav_labels.py"
diff -u --label a/src/views.rs --label b/src/views.rs "$REAL/src/views.rs" src/views.rs > "$S/rust11/out/u05_diff_views.txt"
cargo build -q || exit 1
cd "$E2E"
$CAP u05_a11y_fixed npx playwright test tests/a11y.spec.ts > /dev/null
cd "$S/rust11/rustnote"
cp "$REAL/src/views.rs" src/views.rs
cargo build -q
cd "$E2E"

# 問2・問3
$CAP u05_q2 npx playwright test tests/q2_direct.spec.ts > /dev/null
$CAP u05_q3 npx playwright test tests/q3_tab.spec.ts > /dev/null
rm -rf test-results

cd "$S/rust11/rustnote" && check

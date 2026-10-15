// Fix comments inside ```typescript code blocks of a Markdown file:
//  (1) "orphan" side comments (a `//` line indented >= 30 spaces that trails a code line)
//      are moved ABOVE the code line they describe, re-indented to match it.
//  (2) `//` comment lines that sit in JSX children context (where `//` would be
//      rendered as text / break parsing) are converted to `{/* ... */}`.
// Usage: node fix_comments.mjs <file.md> [--write]
import fs from "node:fs";
import { createRequire } from "node:module";
const require = createRequire(import.meta.url);
const ts = require(process.env.TS_PATH);

const file = process.argv[2];
const write = process.argv.includes("--write");
const text = fs.readFileSync(file, "utf8");
const lines = text.split("\n");

const isComment = (l) => /^\s*\/\//.test(l);
const indentOf = (l) => l.match(/^\s*/)[0];
const ORPHAN_MIN = 30;

function fixBlock(block) {
  // ---- step 1: move orphans
  let out = [];
  for (let i = 0; i < block.length; i++) {
    const l = block[i];
    const ind = indentOf(l).length;
    if (isComment(l) && ind >= ORPHAN_MIN) {
      // find anchor: last non-orphan, non-comment, non-blank line in `out`
      let a = out.length - 1;
      while (a >= 0 && (out[a].trim() === "" || isComment(out[a]))) a--;
      if (a < 0) {
        out.push(l.trimStart());
        continue;
      }
      // anchor might be a continuation; keep simple: insert directly above anchor,
      // but after any comment lines already directly above it
      const anchorIndent = indentOf(out[a]);
      out.splice(a, 0, anchorIndent + l.trim());
      continue;
    }
    out.push(l);
  }
  // ---- step 2: convert JSX-children `//` comments
  const blanked = out.map((l) => (isComment(l) ? "" : l)).join("\n");
  const sf = ts.createSourceFile("x.tsx", blanked, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
  const lineStarts = sf.getLineStarts();
  function deepest(node, pos) {
    let found = node;
    ts.forEachChild(node, (c) => {
      if (c.getFullStart() <= pos && pos < c.end) {
        found = deepest(c, pos);
      }
    });
    return found;
  }
  const converted = [];
  const result = out.map((l, idx) => {
    if (!isComment(l)) return l;
    const pos = lineStarts[idx] + indentOf(l).length;
    const n = deepest(sf, pos);
    const k = n.kind;
    const inChildren =
      k === ts.SyntaxKind.JsxText || k === ts.SyntaxKind.JsxElement || k === ts.SyntaxKind.JsxFragment;
    if (inChildren) {
      let body = l.trim().replace(/^\/\/\s?/, "");
      body = body.replace(
        "JSX の中のコメントは `{/* ... */}` の形にする",
        "JSX の中のコメントは、この行のように波かっこ付きのブロックコメントで書く"
      );
      if (body.includes("*/")) throw new Error("comment contains */: " + l);
      converted.push(l.trim());
      return indentOf(l) + "{/* " + body + " */}";
    }
    return l;
  });
  // diagnostics comparison
  const diagBefore = sf.parseDiagnostics.length;
  const sf2 = ts.createSourceFile("y.tsx", result.join("\n"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
  const diagAfter = sf2.parseDiagnostics.length;
  return { result, converted, diagBefore, diagAfter, changed: result.join("\n") !== block.join("\n") };
}

let i = 0;
const outLines = [];
let report = [];
while (i < lines.length) {
  const l = lines[i];
  const m = l.match(/^```(typescript|tsx|ts)\s*$/);
  if (m) {
    outLines.push(l);
    const start = i + 1;
    let j = start;
    while (!/^```\s*$/.test(lines[j])) j++;
    const block = lines.slice(start, j);
    const r = fixBlock(block);
    if (r.changed) {
      report.push(
        `block@${start + 1}: converted=${r.converted.length} diag ${r.diagBefore}->${r.diagAfter}`
      );
    }
    outLines.push(...r.result);
    outLines.push(lines[j]);
    i = j + 1;
    continue;
  }
  outLines.push(l);
  i++;
}
console.log(report.join("\n"));
console.log("total changed blocks:", report.length);
if (write) fs.writeFileSync(file, outLines.join("\n"));

// ファイルの有無を調べる道具と、パス（住所）を組み立てる道具を読み込む
import fs from "node:fs";
import path from "node:path";

// 探すコマンド名。node find-command.mjs git のように指定できる（省略時は node）
const name = process.argv[2] ?? "node";

// 環境変数 PATH を区切り文字で分割して「フォルダの一覧」にする
// path.delimiter は Windows なら ";"、Mac/Linux なら ":"
const folders = process.env.PATH.split(path.delimiter);

// Windows では実行ファイルに .exe や .cmd が付くので、その候補も用意する
const extensions = process.platform === "win32" ? [".exe", ".cmd"] : [""];

console.log(`「${name}」を PATH の ${folders.length} 個のフォルダから順番に探します`);

// フォルダを上から1つずつ調べ、最初に見つかった時点で探すのをやめる
for (const [index, folder] of folders.entries()) {
  const hit = extensions
    .map((ext) => path.join(folder, name + ext))
    .find((file) => fs.existsSync(file));

  if (hit) {
    console.log(`  ${index + 1}. ${folder} … 見つかった！ → ${hit}`);
    process.exit(0);
  }
  console.log(`  ${index + 1}. ${folder} … なし`);
}

// 最後まで見つからなかった場合（これが command not found の正体）
console.log(`どこにもありませんでした → '${name}' は command not found になります`);

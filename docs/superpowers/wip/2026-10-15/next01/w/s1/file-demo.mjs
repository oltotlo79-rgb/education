// Node.js に最初から入っている「ファイルを読み書きする道具」を読み込む
import fs from "node:fs";

// memo.txt というファイルに文字を書き込む（無ければ新しく作られる）
fs.writeFileSync("memo.txt", "読みたい本: リーダブルコード");

// いま書いたファイルを読み出して、文字列として受け取る
const text = fs.readFileSync("memo.txt", "utf8");

// 読み出した中身を画面に表示する
console.log("memo.txt の中身 →", text);

// Node.js 自身のバージョンと、動いている OS の種類も表示する
console.log("Node.js:", process.version, "/ OS:", process.platform);

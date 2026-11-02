// 付録 B-03: 文字列と数でよく使う書き方
const title = "  治具の製作  ";
const requester = "山田 太郎";
const amount = 1234567.891;

// テンプレート文字列: ` で囲み、${ } の中に値や式を書ける
console.log(`依頼者: ${requester}、件名: ${title.trim()}`);

// よく使う文字列の操作
console.log("[" + title.trim() + "]");                 // 前後の空白を取る
console.log("文字数:", title.trim().length);           // 長さ（日本語も 1 文字は 1）
console.log("REQ-7".padStart(8, "0"));                 // 前を 0 で埋めて 8 文字に
console.log("a,b,c".split(","));                       // 区切って配列に
console.log(["山田", "佐藤"].join("、"));               // 配列をつないで 1 つに
console.log("2026-11-02".replaceAll("-", "/"));        // 全部置きかえる
console.log("approved".toUpperCase());                 // 大文字に
console.log("依頼書.pdf".endsWith(".pdf"), "依頼書.pdf".includes("依頼")); // 終わる？ 含む？
console.log("REQ-0001".slice(4));                      // 4 文字目から後ろ（0 から数える）

// 文字列 → 数: Number() は全体が数でないと NaN（数ではない）になる
console.log(Number("42"), Number("42円"), Number.isNaN(Number("42円")));
console.log(Number.parseInt("42円", 10));              // 先頭の数の部分だけ読む

// 数 → 見やすい文字列
console.log(amount.toFixed(2));                        // 小数点以下 2 けた
console.log(amount.toLocaleString("ja-JP"));           // 3 けたごとの区切り
console.log(Math.round(2.5), Math.floor(2.9), Math.ceil(2.1), Math.max(3, 9, 4));

// 小数の計算の誤差: 0.1 + 0.2 は、ぴったり 0.3 にならない（お金は「円」の整数で持つ）
console.log(0.1 + 0.2, 0.1 + 0.2 === 0.3);

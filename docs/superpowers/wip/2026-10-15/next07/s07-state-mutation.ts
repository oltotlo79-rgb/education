// React は「前の state」と「次の state」を Object.is で比べて、
// 同じ箱（同じオブジェクト）なら「変化なし」とみなして再描画しない。
// その判定をまねる関数
function willRerender(prev: object, next: object): boolean {
  return !Object.is(prev, next);
}

// ❌ NG: 今の state を直接書き換えて、同じ箱をそのまま返す
const prev1 = { title: "", author: "" };
const next1 = prev1;
next1.title = "リーダブルコード";
console.log("NG: 再描画される？", willRerender(prev1, next1), "/ 中身:", next1);

// ✅ OK: スプレッド構文で「新しい箱」を作り、変えたい項目だけ上書きする
const prev2 = { title: "", author: "" };
const next2 = { ...prev2, title: "リーダブルコード" };
console.log("OK: 再描画される？", willRerender(prev2, next2), "/ 中身:", next2);
console.log("OK: 元の箱はそのまま:", prev2);

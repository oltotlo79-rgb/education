// <select> や <input> から来る値は、いつも「文字列」
// "" は「選択してください」を選んだときの値
const inputs = ["3", "5", "", "abc"];

for (const value of inputs) {
  // ❌ 素朴に Number() だけで変換した場合
  const naive = Number(value);
  // ✅ BookForm と同じく「空文字なら null」を先に判定した場合
  const safe = value === "" ? null : Number(value);
  console.log(
    `入力 ${JSON.stringify(value).padEnd(5)}`,
    `| Number()だけ → ${String(naive).padEnd(3)}`,
    `| 安全版 → ${safe}`,
  );
}

// 型も確かめる（typeof は値の種類を文字列で返す）
console.log('typeof "3"         =', typeof "3");
console.log('typeof Number("3") =', typeof Number("3"));

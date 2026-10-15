type Direction = "north" | "south" | "east" | "west";

// 狭い型（Direction）の値
const d: Direction = "north";

// 狭い型 → 広い型（string）への代入は OK
const s: string = d;
console.log(s);

// ユーザーが入力した文字列。どんな文字列が来るか分からないので広い型 string
const input: string = "up";

// 中身が本当に方角かどうかを調べる型ガード関数
function isDirection(value: string): value is Direction {
  return (
    value === "north" ||
    value === "south" ||
    value === "east" ||
    value === "west"
  );
}

if (isDirection(input)) {
  // ここでは input は Direction に絞り込まれているので代入できる
  const d2: Direction = input;
  console.log("方角です:", d2);
} else {
  console.log("方角ではありません:", input);
}

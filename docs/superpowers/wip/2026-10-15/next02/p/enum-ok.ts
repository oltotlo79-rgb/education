// enum の代わりに「as const を付けたオブジェクト」で選択肢をまとめる
const Signal = {
  Red: "RED",
  Yellow: "YELLOW",
  Green: "GREEN",
} as const;

// 値の一覧から型を作る → "RED" | "YELLOW" | "GREEN"
type Signal = (typeof Signal)[keyof typeof Signal];

function canGo(signal: Signal): boolean {
  return signal === Signal.Green;
}

console.log(Signal.Red);
console.log(canGo(Signal.Green));
console.log(canGo("RED"));

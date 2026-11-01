import type { RequestStatus } from "../src/status.ts";

// 間違い 1: 決まっていない状態を入れる
const s: RequestStatus = "承認済み";

// 間違い 2: 数を入れる所に文字を入れる
let count: number = 3;
count = "4";

// 間違い 3: 無い項目を使う
const user = { name: "山田" };
console.log(user.mail);

// 間違い 4: 関数に渡す数の間違い
function greet(name: string): string {
  return `${name} さん`;
}
greet();

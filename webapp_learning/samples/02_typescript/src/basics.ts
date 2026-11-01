// 1. 変数と型
const appName: string = "依頼・承認アプリ"; // 文字
let count: number = 3;                     // 数
const isOpen: boolean = true;              // はい・いいえ
count = count + 1;                         // let は、あとから変えられる

// 2. 配列（同じ型の物の並び）
const approvers: string[] = ["山田", "佐藤", "鈴木"];

// 3. オブジェクト（名前の付いた値のまとまり）
const user = { employeeNo: "012345", name: "山田 太郎", isAdmin: false };

// 4. 関数（引数と戻り値に型を付ける）
function greet(name: string): string {
  return `${name} さん、こんにちは`; // ` で囲むと、${ } の中に値を埋め込める
}

// 5. アロー関数（短い書き方の関数）
const double = (n: number): number => n * 2;

console.log(appName, count, isOpen);
console.log(approvers.length, "人:", approvers.join("、"));
console.log(user.name, user.isAdmin ? "管理者" : "一般");
console.log(greet(user.name));
console.log(double(21));

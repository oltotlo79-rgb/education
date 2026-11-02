// 付録 B-05: オブジェクトと型でよく使う書き方

// type で「形」に名前を付ける。? は「無くてもよい」項目
type User = {
  employeeNo: string;
  name: string;
  mail?: string;
  isAdmin: boolean;
};

// interface でも形を書ける（ほぼ同じ。この資料では type を使う）
interface Approval {
  order: number;
  approver: string;
}

// ユニオン型: 決まった値のどれか 1 つだけを許す
type Status = "回覧中" | "承認済み" | "差し戻し";

const user: User = { employeeNo: "012345", name: "山田 太郎", isAdmin: false };
const approvals: Approval[] = [{ order: 1, approver: "佐藤" }, { order: 2, approver: "鈴木" }];

// ?. は「無ければ、そこで止めて undefined」。?? は「undefined か null なら右の値」
console.log(user.mail?.toUpperCase());
console.log(user.mail ?? "（メール無し）");

// 分割代入: オブジェクトの項目を、同じ名前の変数へ取り出す
const { name, isAdmin } = user;
console.log(name, isAdmin);

// スプレッド構文: 一部だけ変えた「新しい」オブジェクトを作る（元は変えない）
const promoted = { ...user, isAdmin: true };
console.log(user.isAdmin, promoted.isAdmin);

// Object.keys / Object.entries: 項目の名前・名前と値の組を、配列で取り出す
console.log(Object.keys(user));
for (const [key, value] of Object.entries(user)) {
  console.log(`${key} = ${value}`);
}

// 型の絞り込み: switch で Status の全部の場合を書く
function label(s: Status): string {
  switch (s) {
    case "回覧中":
      return "回覧中（承認を待っている）";
    case "承認済み":
      return "承認済み（完了）";
    case "差し戻し":
      return "差し戻し（直して出し直す）";
  }
}
console.log(label("承認済み"));

// typeof で絞り込む: string | number のどちらかを受け取る
function show(value: string | number): string {
  if (typeof value === "number") {
    return value.toLocaleString("ja-JP") + " 円"; // ここでは number として使える
  }
  return value.trim(); // ここでは string として使える
}
console.log(show(30000), show("  見積もり中  "));

// Record<キーの型, 値の型>: 決まったキーをすべて持つ表
const colors: Record<Status, string> = { 回覧中: "青", 承認済み: "緑", 差し戻し: "オレンジ" };
console.log(colors["差し戻し"]);

// as const: 中身を変えられない、決まった値の並びにする
const steps = ["入力", "回覧", "完了"] as const;
console.log(steps.length, steps[2]);

console.log(approvals.map((a) => `${a.order}. ${a.approver}`).join(" → "));

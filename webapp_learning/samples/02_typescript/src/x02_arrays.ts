// 付録 B-04: 配列（並び）でよく使う書き方
type Req = { id: number; title: string; status: "回覧中" | "承認済み" | "差し戻し"; amount: number };

const requests: Req[] = [
  { id: 1, title: "治具の製作", status: "回覧中", amount: 30000 },
  { id: 2, title: "部品の追加工", status: "承認済み", amount: 12000 },
  { id: 3, title: "棚の取り付け", status: "差し戻し", amount: 8000 },
  { id: 4, title: "工具の修理", status: "回覧中", amount: 5000 },
];

// map: 1 つずつ変換して、新しい配列を作る
console.log(requests.map((r) => r.title));

// filter: 条件に合う物だけの、新しい配列を作る
const circulating = requests.filter((r) => r.status === "回覧中");
console.log("回覧中:", circulating.map((r) => r.id));

// find: 最初に条件に合う物（無ければ undefined）。findIndex は、その位置
console.log(requests.find((r) => r.id === 3)?.title, requests.find((r) => r.id === 9));
console.log(requests.findIndex((r) => r.status === "差し戻し"));

// some: 1 つでも合う？ every: 全部合う？ includes: その値がある？
console.log(requests.some((r) => r.amount > 20000), requests.every((r) => r.amount > 1000));
console.log(["山田", "佐藤"].includes("佐藤"));

// reduce: 1 つの値にまとめる（ここでは合計）
const total = requests.reduce((sum, r) => sum + r.amount, 0);
console.log("合計:", total);

// toSorted: 並べかえた「新しい」配列を作る（元の配列は変わらない）。sort は元の配列を変える
const byAmount = requests.toSorted((a, b) => a.amount - b.amount);
console.log("安い順:", byAmount.map((r) => `${r.title}(${r.amount})`).join(" / "));
console.log("元の順のまま:", requests.map((r) => r.id));

// 足す・取り出す（元の配列が変わる）
const names = ["山田"];
names.push("佐藤", "鈴木"); // 後ろに足す
const last = names.pop();   // 後ろから取り出す
console.log(names, last, names.length);

// スプレッド構文 ...: 配列を広げて、新しい配列を作る（元は変えない）
const more = [...names, "高橋"];
console.log(more, names);

// 分割代入: 配列の中身を、順に変数へ取り出す
const [first, second] = more;
console.log(first, second);

// forEach: 1 つずつ処理する（新しい配列は作らない）。番号も受け取れる
requests.forEach((r, i) => {
  if (i < 2) console.log(`${i + 1}件目: ${r.title}`);
});

// 状態ごとに数える（オブジェクトを表のように使う）
const counts: Record<string, number> = {};
for (const r of requests) {
  counts[r.status] = (counts[r.status] ?? 0) + 1;
}
console.log(counts);

import { badgeColor, nextStatus, type RequestStatus } from "./status.ts";

/** 依頼 1 件の形 */
export type RequestItem = {
  id: number;
  title: string;
  status: RequestStatus;
  dueDate: string;      // "2026-11-10" の形の日付
  approvers: string[];  // 回覧者（1〜3 人）
  note?: string;        // ? が付いた項目は、無くてもよい
};

const requests: RequestItem[] = [
  { id: 1, title: "治具の製作", status: "回覧中", dueDate: "2026-11-10", approvers: ["山田", "佐藤"] },
  { id: 2, title: "部品の追加工", status: "回覧完了", dueDate: "2026-10-31", approvers: ["山田"] },
  { id: 3, title: "棚の取り付け", status: "差し戻し中", dueDate: "2026-11-20", approvers: ["佐藤"], note: "寸法を追記" },
  { id: 4, title: "型の改造", status: "回覧中", dueDate: "2026-11-03", approvers: ["鈴木", "山田", "佐藤"] },
];

// filter: 条件に合う物だけ残す
const circulating = requests.filter((r) => r.status === "回覧中");
console.log("回覧中:", circulating.map((r) => r.title));

// map: 1 つずつ、別の形に変える
const rows = requests.map((r) => `${r.id} ${r.title}（${r.status}・${badgeColor(r.status)}）`);
console.log(rows.join("\n"));

// reduce: 全部をまとめて 1 つの値にする（状態ごとの件数）
const counts = requests.reduce<Record<string, number>>((acc, r) => {
  acc[r.status] = (acc[r.status] ?? 0) + 1;
  return acc;
}, {});
console.log("状態ごとの件数:", counts);

// 期限の近い順に並べる（元の配列は変えない）
const byDue = [...requests].sort((a, b) => a.dueDate.localeCompare(b.dueDate));
console.log("期限の近い順:", byDue.map((r) => r.dueDate));

// 状態を変える
console.log(nextStatus("回覧中", "差し戻し"));  // 差し戻し中
console.log(nextStatus("完了", "差し戻し"));    // null（完了した依頼は差し戻せない）

// ? の付いた項目は、無いかもしれないので、確かめてから使う
for (const r of requests) {
  if (r.note !== undefined) console.log(`${r.title} のメモ: ${r.note}`);
}

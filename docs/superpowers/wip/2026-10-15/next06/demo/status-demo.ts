// 読書ステータスとして使ってよい3つの文字列だけを並べた型
type BookStatus = "unread" | "reading" | "finished";

// ステータスを日本語ラベルに変換する対応表
const statusLabels: Record<BookStatus, string> = {
  unread: "未読",
  reading: "読書中",
  finished: "読了",
};

// 3つのステータスを順番に取り出してラベルを表示する
const all: BookStatus[] = ["unread", "reading", "finished"];
for (const status of all) {
  console.log(`${status} → ${statusLabels[status]}`);
}

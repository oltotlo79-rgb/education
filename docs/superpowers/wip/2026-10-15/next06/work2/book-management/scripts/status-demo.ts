// scripts/status-demo.ts
// 読書ステータスの型と、日本語ラベルの対応表を小さく試すスクリプト

// 読書ステータスとして使ってよい3つの文字列だけを並べた型
type BookStatus = "reading" | "completed" | "want_to_read";

// ステータス → 日本語ラベルの対応表（3つすべて書かないと型エラーになる）
const statusLabels: Record<BookStatus, string> = {
  reading: "読書中",
  completed: "読了",
  want_to_read: "読みたい",
};

// 3つのステータスを順番に取り出してラベルを表示する
const all: BookStatus[] = ["reading", "completed", "want_to_read"];
for (const status of all) {
  console.log(`${status} → ${statusLabels[status]}`);
}

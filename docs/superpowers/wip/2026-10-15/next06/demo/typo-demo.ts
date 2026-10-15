// 読書ステータスとして使ってよい3つの文字列
type BookStatus = "unread" | "reading" | "finished";

// ステータス → 日本語ラベルの対応表
const statusLabels: Record<BookStatus, string> = {
  unread: "未読",
  reading: "読書中",
  finished: "読了",
};

// わざと間違えた値（"yomu" は BookStatus の候補に無い）
const myStatus: BookStatus = "yomu";
console.log("ラベル:", statusLabels[myStatus]);

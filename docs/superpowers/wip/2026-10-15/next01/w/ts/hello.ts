// 読書ステータスとして使ってよい値を、この3つだけに限定する
type Status = "want_to_read" | "reading" | "completed";

// 本のデータの「形」を決める（TypeScript の型。第2章で詳しく学びます）
type Book = {
  title: string;
  status: Status;
};

// ステータスの値と、画面に出す日本語の対応表
const labels = {
  want_to_read: "読みたい",
  reading: "読書中",
  completed: "読了",
};

// 決めた形に合わせて、本のデータを3冊ぶん作る
const books: Book[] = [
  { title: "リーダブルコード", status: "completed" },
  { title: "達人プログラマー", status: "reading" },
  { title: "プログラミング TypeScript", status: "want_to_read" },
];

// 1冊ずつ取り出して、タイトルとステータスを表示する
for (const book of books) {
  console.log(`${book.title}: ${labels[book.status]}`);
}

// 本のデータの「形」を決める（TypeScript の型。第2章で詳しく学びます）
type Book = {
  title: string;
  isRead: boolean;
};

// 決めた形に合わせて、本のデータを2冊ぶん作る
const books: Book[] = [
  { title: "リーダブルコード", isRead: true },
  { title: "達人プログラマー", isRead: false },
];

// 1冊ずつ取り出して、読んだかどうかを表示する
for (const book of books) {
  const status = book.isRead ? "読了" : "未読";
  console.log(`${book.title}: ${status}`);
}

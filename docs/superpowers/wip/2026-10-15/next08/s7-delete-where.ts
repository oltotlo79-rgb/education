// .delete() の条件を「正しく付けた」「付け忘れた」「間違えた」ときに、
// どの行が消えるのかを配列で真似して確かめる練習用コード
// （id は読みやすさのため短くしています。本物は UUID です）

type Book = {
  id: string;
  title: string;
  status: string;
};

type DeleteResult = {
  deleted: Book[];
  remaining: Book[];
};

// 毎回同じ状態から試せるよう、テーブルの中身を作り直す
function makeTable(): Book[] {
  return [
    { id: "a1", title: "React実践ガイド", status: "reading" },
    { id: "b2", title: "TypeScript入門", status: "unread" },
    { id: "c3", title: "Next.js実践", status: "finished" },
    { id: "d4", title: "SQL入門", status: "reading" },
  ];
}

// 条件 where に当てはまる行を消す。where が無いと「全行が当てはまる」
function deleteRows(
  table: Book[],
  where?: (book: Book) => boolean,
): DeleteResult {
  const match = where ?? (() => true);
  return {
    deleted: table.filter((book) => match(book)),
    remaining: table.filter((book) => !match(book)),
  };
}

// 結果を見やすく表示する
function report(label: string, result: DeleteResult) {
  const deleted = result.deleted.map((book) => book.title);
  const remaining = result.remaining.map((book) => book.title);
  console.log(label);
  console.log(`  消えた(${deleted.length}件): ${deleted.join(", ") || "なし"}`);
  console.log(`  残った(${remaining.length}件): ${remaining.join(", ") || "なし"}`);
}

// OK: .delete().eq("id", "b2") に相当
report(
  'OK  .eq("id", "b2")',
  deleteRows(makeTable(), (book) => book.id === "b2"),
);

// NG1: .eq() を付け忘れた .delete() に相当（SQL の DELETE FROM books;）
report("NG1 条件なし", deleteRows(makeTable()));

// NG2: id ではなく status で絞ってしまった
report(
  'NG2 .eq("status", "reading")',
  deleteRows(makeTable(), (book) => book.status === "reading"),
);

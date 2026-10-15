// 1冊の本が持つ情報の形（型）
type Book = {
  id: number;
  title: string;
  status: string;
};

// データベースの代わりに、ただの配列を「本棚」として使う
let books: Book[] = [];
// 次に登録する本に付ける番号
let nextId = 1;

// C: Create（作成）… 新しい本を本棚に追加する
function createBook(title: string): void {
  books.push({ id: nextId, title: title, status: "unread" });
  nextId = nextId + 1;
}

// R: Read（読み取り）… 本棚の中身を全部表示する
function listBooks(): void {
  for (const b of books) {
    console.log(`  ${b.id}: ${b.title} [${b.status}]`);
  }
}

// U: Update（更新）… 指定した番号の本の状態を書き換える
function updateStatus(id: number, status: string): void {
  const target = books.find((b) => b.id === id);
  if (target) {
    target.status = status;
  }
}

// D: Delete（削除）… 指定した番号「以外」の本だけを残す
function deleteBook(id: number): void {
  books = books.filter((b) => b.id !== id);
}

createBook("リーダブルコード");
createBook("達人プログラマー");
console.log("① 2冊を登録した直後");
listBooks();

updateStatus(1, "finished");
console.log("② 1番を読了(finished)にした後");
listBooks();

deleteBook(2);
console.log("③ 2番を削除した後");
listBooks();

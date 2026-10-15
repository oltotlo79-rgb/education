// アプリで保存される「本」の形（id はデータベースが付ける番号）
interface Book {
  id: string;
  title: string;
  author: string;
  price: number;
}

// 新規登録の入力: まだ id が無いので、Book から id を取り除いた形
type CreateBookInput = Omit<Book, "id">;

// 更新の入力: 変えたい項目だけ送ればよいので、全部を省略可能にした形
type UpdateBookInput = Partial<CreateBookInput>;

// 入力に id を付けて「本」を完成させる関数
function createBook(input: CreateBookInput, newId: string): Book {
  return { id: newId, ...input };
}

// 元の本に「変えたい項目」だけを上書きした、新しい本を返す関数
function updateBook(book: Book, changes: UpdateBookInput): Book {
  return { ...book, ...changes };
}

const input: CreateBookInput = {
  title: "TypeScript入門",
  author: "山田太郎",
  price: 2980,
};

const created = createBook(input, "book-001");
console.log(created);

// 価格だけを変更する（ほかの項目は書かなくてよい）
const updated = updateBook(created, { price: 1980 });
console.log(updated);

// 元の created は変わっていない
console.log(created.price);

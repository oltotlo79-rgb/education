interface Book {
  id: string;
  title: string;
  author: string;
  price: number;
}
type CreateBookInput = Omit<Book, "id">;
type UpdateBookInput = Partial<CreateBookInput>;

// ① 新規登録の入力に id を書いてしまった
const input: CreateBookInput = {
  id: "book-001",
  title: "TypeScript入門",
  author: "山田太郎",
  price: 2980,
};

// ② 更新の入力で price に文字列を入れてしまった
const changes: UpdateBookInput = { price: "1980" };

// T は「id: string を持っている何かの型」。それ以外の型は受け付けない
function findById<T extends { id: string }>(
  items: T[],
  id: string,
): T | undefined {
  // find は「条件に合う最初の要素」を返す。見つからなければ undefined
  return items.find((item) => item.id === id);
}

interface Book {
  id: string;
  title: string;
  price: number;
}

interface Author {
  id: string;
  name: string;
}

const books: Book[] = [
  { id: "b1", title: "TypeScript入門", price: 2980 },
  { id: "b2", title: "React実践ガイド", price: 3300 },
];

const authors: Author[] = [{ id: "a1", name: "山田太郎" }];

// T = Book と自動で決まる → 戻り値は Book | undefined
const book = findById(books, "b2");
console.log(book?.title);

// T = Author と自動で決まる → 戻り値は Author | undefined
const author = findById(authors, "a1");
console.log(author?.name);

// 見つからないときは undefined
const missing = findById(books, "b999");
console.log(missing);

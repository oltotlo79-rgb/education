import { useState } from "react";

// 本1冊の型
type Book = {
  id: number;
  title: string;
};

// ---- 子1: 検索欄 ----
// 親から「今の検索語」と「検索語が変わったら呼ぶ関数」を受け取る
type SearchBoxProps = {
  query: string;
  onQueryChange: (nextQuery: string) => void;
};

function SearchBox({ query, onQueryChange }: SearchBoxProps) {
  return (
    <input
      value={query}
      // 入力されたら、親から受け取った関数を呼んで「上へ」知らせる
      onChange={(e) => onQueryChange(e.target.value)}
      placeholder="タイトルで検索"
    />
  );
}

// ---- 子2: 検索結果の一覧 ----
// 親から「表示する本の配列」だけを受け取る（自分では state を持たない）
type BookListProps = {
  books: Book[];
};

function BookList({ books }: BookListProps) {
  // 1冊もなければメッセージだけ返す（早期 return）
  if (books.length === 0) {
    return <p>見つかりませんでした</p>;
  }
  return (
    <ul>
      {books.map((book) => (
        <li key={book.id}>{book.title}</li>
      ))}
    </ul>
  );
}

// ---- 親: state を持ち、2つの子に配る ----
const ALL_BOOKS: Book[] = [
  { id: 1, title: "React入門" },
  { id: 2, title: "TypeScript実践" },
  { id: 3, title: "React Hooks 徹底解説" },
];

export default function BookSearchPage() {
  // 兄弟（SearchBox と BookList）で共有したい値は、共通の親が持つ
  const [query, setQuery] = useState<string>("");

  // state から「表示する本」を計算する（これは state にしない）
  const visibleBooks = ALL_BOOKS.filter((book) => book.title.includes(query));

  return (
    <div>
      <h2>書籍検索</h2>
      {/* 下へ: 今の検索語と、変更を知らせてもらう関数を渡す */}
      <SearchBox query={query} onQueryChange={setQuery} />
      <p>{visibleBooks.length}件ヒット</p>
      {/* 下へ: 計算した結果だけを渡す */}
      <BookList books={visibleBooks} />
    </div>
  );
}

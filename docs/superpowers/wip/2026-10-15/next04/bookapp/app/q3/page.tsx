import { LikeButton } from "./LikeButton";

export default async function BooksPage() {
  const books = ["リーダブルコード", "達人プログラマー"];
  return (
    <div>
      <h1>書籍一覧（{books.length}冊）</h1>
      <LikeButton />
    </div>
  );
}

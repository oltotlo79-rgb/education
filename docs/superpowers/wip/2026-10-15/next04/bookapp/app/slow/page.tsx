// app/slow/page.tsx
import { connection } from "next/server";

// 2秒かかるデータ取得のまね（本物ならデータベースへの問い合わせ）
async function getBooksSlowly() {
  await new Promise((resolve) => setTimeout(resolve, 2000));
  return ["リーダブルコード", "達人プログラマー"];
}

export default async function SlowPage() {
  // 「アクセスのたびにサーバーで実行する」と宣言（ビルド時の作り置きをさせない）
  await connection();
  const books = await getBooksSlowly();

  return (
    <ul>
      {books.map((title) => (
        <li key={title}>{title}</li>
      ))}
    </ul>
  );
}

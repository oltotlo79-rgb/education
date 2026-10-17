// src/app/books/[id]/page.tsx
import type { Metadata } from "next";
// サーバー用の Supabase クライアント（第7章で作ったもの）
import { createClient } from "@/lib/supabase/server";

// このページに渡される props の型
type Props = {
  // URL の [id] 部分。Next.js 15 以降は Promise（あとから値が届く箱）で渡される
  params: Promise<{ id: string }>;
};

// generateMetadata = 動的にメタデータを作る関数。Next.js が自動で呼び出す
export async function generateMetadata({ params }: Props): Promise<Metadata> {
  // await で箱の中身を取り出してから id を使う
  const { id } = await params;
  // サーバー用クライアントを作る（cookies() を使うので await が必要）
  const supabase = await createClient();
  // id が一致する書籍を1件だけ取得する
  const { data: book } = await supabase
    .from("books")
    .select("*")
    .eq("id", id)
    .single();

  // 本が見つからなかったときの表示
  if (!book) {
    return { title: "書籍が見つかりません" };
  }

  // 見つかったときは、タイトルと著者名を使ってメタデータを作る
  return {
    title: book.title,
    description: `${book.title}（${book.author}）の詳細情報`,
  };
}

export default async function BookDetailPage({ params }: Props) {
  const { id } = await params;
  const supabase = await createClient();
  const { data: book } = await supabase
    .from("books")
    .select("*")
    .eq("id", id)
    .single();

  return (
    <main className="p-6">
      <h1 className="text-xl font-bold">{book?.title ?? "見つかりません"}</h1>
    </main>
  );
}

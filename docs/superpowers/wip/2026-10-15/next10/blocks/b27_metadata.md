```typescript
// src/app/books/[id]/page.tsx（generateMetadata の部分だけの抜粋。ページ本体の関数は第8章のまま）
import type { Metadata } from "next";
// サーバー用の Supabase クライアント（第7章で作ったもの）
import { createClient } from "@/lib/supabase/server";

// このページに渡される props の型
type Props = {
  // URL の [id] 部分。Next.js 15 以降は Promise（あとから値が届く箱）で渡される
  params: Promise<{ id: string }>;
};

// generateMetadata = 動的にメタデータを作る関数。Next.js が自動で呼び出す
// async = 中で await（非同期処理）を使えるようにする
// Promise<Metadata> = 「Metadata を返す非同期関数」という型注釈
export async function generateMetadata({ params }: Props): Promise<Metadata> {
  // await で箱の中身を取り出してから id を使う
  const { id } = await params;
  // サーバー用クライアントを作る（cookies() を使うので await が必要）
  const supabase = await createClient();
  // id が一致する書籍を1件だけ取得する（.single() = 1件だけ取得）
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
```

> **注意:** 古い記事では `params: { id: string }` と書いて `params.id` を直接使っていますが、Next.js 15 以降では `params` は Promise なので、必ず `await params` で取り出します（1.3 のエラー6 を参照）。このコードは Next.js 16.4.1 でビルドが通ることを確認しています。

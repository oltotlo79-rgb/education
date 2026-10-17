// 書籍一覧を取得する関数（第7章のトップページで使う処理の骨組み）

// 1. サーバー用の Supabase クライアントを作る関数をインポート
import { createClient } from "@/lib/supabase/server";
// Book 型もインポート（型情報のみ。ランタイムには影響なし）
import type { Book } from "@/types/book";

// async: 中で await を使う関数に付ける印。戻り値は「あとで Book の配列が届く約束（Promise）」
export async function getBooks(): Promise<Book[]> {
  // 2. サーバー用の窓口を作る（中で cookies() を待つので await が必要）
  const supabase = await createClient();

  // 3. データベースから書籍一覧を取得
  // 戻り値は { data, error } のオブジェクトなので、分割代入で取り出す
  const { data, error } = await supabase
    // books テーブルを指定（SQL の FROM books に相当）
    .from("books")
    // すべての列を取得（SQL の SELECT * に相当）
    .select("*")
    // 作成日の降順（新しい順）に並び替え（SQL の ORDER BY に相当）
    .order("created_at", { ascending: false });

  // 4. エラーハンドリング
  // error が null でなければ true（エラーが発生した）
  if (error) {
    // サーバーのターミナルにエラーの内容を出す
    console.error("書籍の取得に失敗しました:", error.message);
    // 失敗したときは空の配列を返して、画面が真っ白にならないようにする
    return [];
  }

  // 5. data は書籍の配列。万一 null のときに備えて ?? で空配列にする
  const books: Book[] = data ?? [];
  return books;
}

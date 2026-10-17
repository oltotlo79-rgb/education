// サーバー用の Supabase クライアントを作る関数を読み込む
import { createClient } from "@/lib/supabase/server";

// 書籍をタイトル順に取得する
export async function getBooksSortedByTitle() {
  const supabase = await createClient();
  // AI が提案したコード: findMany は supabase-js には存在しない
  const { data, error } = await supabase
    .from("books")
    .findMany({ orderBy: { title: "asc" } });
  if (error) throw error;
  return data;
}

// サーバー用の Supabase クライアントを作る関数を読み込む
import { createClient } from "@/lib/supabase/server";

// 書籍をタイトル順に取得する
export async function getBooksSortedByTitle() {
  const supabase = await createClient();
  // supabase-js の正しい書き方: select で取得し、order で並べ替える
  const { data, error } = await supabase
    .from("books")
    .select("*")
    .order("title", { ascending: true });
  if (error) throw error;
  return data;
}

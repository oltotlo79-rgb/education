// lib/supabase/server.ts
// サーバーで動く部品（Server Component）から使う Supabase クライアントを作るファイル

// @supabase/ssr から「サーバー用のクライアントを作る関数」を名前付きインポートする
import { createServerClient } from "@supabase/ssr";
// Next.js が用意している「いま届いたリクエストの Cookie を読み書きする関数」
import { cookies } from "next/headers";

// Server Component から呼び出す関数。cookies() を待つので async にする
export async function createClient() {
  // .env.local に書いた接続先 URL を読み出す
  const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL;
  // .env.local に書いた Publishable key（旧 anon key でも可）を読み出す
  const supabaseKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY;

  // どちらかが空なら、原因と直し方が分かるメッセージでわざと止める
  if (!supabaseUrl || !supabaseKey) {
    throw new Error(
      "環境変数 NEXT_PUBLIC_SUPABASE_URL / NEXT_PUBLIC_SUPABASE_ANON_KEY が設定されていません。" +
        ".env.local ファイルを確認してください。"
    );
  }

  // いま処理しているリクエストの Cookie 置き場を受け取る
  const cookieStore = await cookies();

  // URL・キー・Cookie の読み書き方法を渡して、サーバー用のクライアントを作って返す
  return createServerClient(supabaseUrl, supabaseKey, {
    cookies: {
      // Supabase が Cookie を読みたいときに呼ばれる
      getAll() {
        return cookieStore.getAll();
      },
      // Supabase が Cookie を書きたいときに呼ばれる
      setAll(cookiesToSet) {
        try {
          // 受け取った Cookie を1つずつ書き込む
          cookiesToSet.forEach(({ name, value, options }) =>
            cookieStore.set(name, value, options)
          );
        } catch {
          // Server Component の表示中は Cookie を書けないので、ここに来ても無視してよい
          // （このアプリはログイン機能を使わないため、実害はない）
        }
      },
    },
  });
}

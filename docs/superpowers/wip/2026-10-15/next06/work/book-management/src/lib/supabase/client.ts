// src/lib/supabase/client.ts
// ブラウザで動く部品（Client Component）から使う Supabase クライアントを作るファイル

// @supabase/ssr から「ブラウザ用のクライアントを作る関数」を名前付きインポートする
import { createBrowserClient } from "@supabase/ssr";

// Client Component から呼び出す関数。呼ぶたびに Supabase への窓口を作って返す
export function createClient() {
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

  // URL とキーを渡して、ブラウザ用のクライアントを作って返す
  return createBrowserClient(supabaseUrl, supabaseKey);
}

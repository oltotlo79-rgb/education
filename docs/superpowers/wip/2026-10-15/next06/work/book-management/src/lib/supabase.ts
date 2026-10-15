// src/lib/supabase.ts
// このファイルは、アプリ全体で共有する Supabase クライアントを作成・エクスポートする役割

// @supabase/supabase-js パッケージから createClient 関数を「名前付きインポート」する
// {} で囲むのが名前付きインポートの構文。export されている特定の関数だけを取り出す
import { createClient } from "@supabase/supabase-js";

// 環境変数から Supabase の接続情報を取得
// process.env: Node.js 由来のグローバルオブジェクト。.env.local の中身がここに入る
// NEXT_PUBLIC_ で始まる変数は、ビルド時にブラウザ側のコードにも埋め込まれる
// SupabaseのプロジェクトURL
const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL;
// Supabase の Anon Key
const supabaseAnonKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY;

// 環境変数が設定されていない場合にわかりやすいエラーメッセージを表示
// !supabaseUrl は「supabaseUrl が undefined や空文字なら true」になる条件
if (!supabaseUrl) {
  // throw: エラーを「投げる」。アプリの実行を停止し、コンソールにエラーを出力する
  // new Error(...): JavaScript標準のエラーオブジェクトを作る
  throw new Error(
    "環境変数 NEXT_PUBLIC_SUPABASE_URL が設定されていません。" +
      ".env.local ファイルを確認してください。"
  );
}

// AnonKey についても同じく未設定チェックを行う
if (!supabaseAnonKey) {
  throw new Error(
    "環境変数 NEXT_PUBLIC_SUPABASE_ANON_KEY が設定されていません。" +
      ".env.local ファイルを確認してください。"
  );
}

// Supabase クライアントを作成してエクスポート
// アプリ全体で同じインスタンスを使い回す（シングルトンパターン）
// export const: 他のファイルから import できる変数を宣言
// createClient(URL, Key): 第1引数 URL、第2引数 認証キーで Supabase に接続するクライアントを作成
export const supabase = createClient(supabaseUrl, supabaseAnonKey);

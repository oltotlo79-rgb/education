// 01-check.ts : Supabase につながるか確かめる「接続テスト」スクリプト
// 実行方法: node --env-file=.env.local 01-check.ts

// Supabase の公式ライブラリから「クライアントを作る関数」を取り込む
import { createClient } from '@supabase/supabase-js';

// .env.local に書いた接続情報を読み込む（--env-file オプションが読み込んでくれる）
const url = process.env.NEXT_PUBLIC_SUPABASE_URL;
const key = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY;

// 読み込めていなければ、原因が分かるメッセージを出して終了する
if (!url || !key) {
  console.error('環境変数が読み込めていません。--env-file=.env.local を付けましたか？');
  process.exit(1);
}

// 接続窓口（クライアント）を作る
const supabase = createClient(url, key);

// books テーブルから title と status を取り出す（title の順に並べる）
const { data, error } = await supabase
  .from('books')
  .select('title, status')
  .order('title');

// 結果の「2つの箱」を両方とも表示してみる
console.log('data :', data);
console.log('error:', error);

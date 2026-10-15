// 06-ng-ignore-error.ts : error を確認せずに data を使うと何が起きるか（NG 例）
// 実行方法: node --env-file=.env.local 06-ng-ignore-error.ts
import { createClient } from '@supabase/supabase-js';

const supabase = createClient(
  process.env.NEXT_PUBLIC_SUPABASE_URL!,
  process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!
);

// rating に 10（範囲外）を入れて、わざと失敗させる
const { data } = await supabase
  .from('books')
  .insert({ title: 'テスト', author: 'テスト', rating: 10 })
  .select();

// NG: error を見ずに data を使っている
console.log(`${data.length}件追加しました`);

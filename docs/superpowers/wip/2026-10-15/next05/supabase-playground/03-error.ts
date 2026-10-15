// 03-error.ts : わざと失敗させて、error の中身を観察する
// 実行方法: node --env-file=.env.local 03-error.ts
import { createClient } from '@supabase/supabase-js';

const supabase = createClient(
  process.env.NEXT_PUBLIC_SUPABASE_URL!,
  process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!
);

// rating は CHECK 制約で 1〜5 しか入らない。そこへ 10 を入れてみる
const { data, error } = await supabase
  .from('books')
  .insert({ title: 'テスト', author: 'テスト', rating: 10 })
  .select();

// 失敗したら error に「なぜ失敗したか」が入り、data は null になる
if (error) {
  console.log('失敗しました');
  console.log('  code   :', error.code);
  console.log('  message:', error.message);
  console.log('  details:', error.details);
  console.log('  data は:', data);
} else {
  console.log('成功しました:', data);
}

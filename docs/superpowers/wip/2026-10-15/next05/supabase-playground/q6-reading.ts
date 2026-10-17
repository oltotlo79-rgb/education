// q6-reading.ts : 読書中の本の題名を表示する
// 実行方法: node --env-file=.env.local q6-reading.ts
import { createClient } from '@supabase/supabase-js';

const supabase = createClient(
  process.env.NEXT_PUBLIC_SUPABASE_URL!,
  process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!
);

// status が reading の本の title だけを、title の順に取り出す
const { data, error } = await supabase
  .from('books')
  .select('title')
  .eq('status', 'reading')
  .order('title');

// ① 失敗 → ② 0件 → ③ 1件以上 の順に調べる
if (error) {
  console.error('読み込みに失敗しました:', error.message);
} else if (data.length === 0) {
  console.log('読書中の本はありません');
} else {
  for (const book of data) {
    console.log(`- ${book.title}`);
  }
}

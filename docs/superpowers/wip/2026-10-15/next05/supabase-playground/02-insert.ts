// 02-insert.ts : .select() を「付けない insert」と「付けた insert」の違いを見る
// 実行方法: node --env-file=.env.local 02-insert.ts
import { createClient } from '@supabase/supabase-js';

const supabase = createClient(
  process.env.NEXT_PUBLIC_SUPABASE_URL!,
  process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!
);

// (1) .select() を付けずに追加する
const first = await supabase
  .from('books')
  .insert({ title: '銀河鉄道の夜', author: '宮沢賢治' });
console.log('(1) data:', first.data, '/ error:', first.error, '/ status:', first.status);

// (2) .select() を付けて追加する（保存された行が返ってくる）
const second = await supabase
  .from('books')
  .insert({ title: '坊っちゃん', author: '夏目漱石', rating: 4 })
  .select('title, author, rating, status');
console.log('(2) data:', second.data, '/ error:', second.error, '/ status:', second.status);

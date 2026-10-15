// 05-update.ts : 更新の結果を「件数」で確かめる
// 実行方法: node --env-file=.env.local 05-update.ts
import { createClient } from '@supabase/supabase-js';

const supabase = createClient(
  process.env.NEXT_PUBLIC_SUPABASE_URL!,
  process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!
);

// (1) id を「2」のような番号で指定してしまった（id は UUID なので形式が違う）
const wrongType = await supabase
  .from('books')
  .update({ status: 'completed' })
  .eq('id', '2')
  .select();
console.log('(1)', { data: wrongType.data, error: wrongType.error });

// (2) 形式は正しいが、存在しない id を指定した
const notFound = await supabase
  .from('books')
  .update({ status: 'completed' })
  .eq('id', '00000000-0000-0000-0000-000000000000')
  .select();
console.log('(2)', { data: notFound.data, error: notFound.error });

// (3) 実在する本を title で探して更新した
const ok = await supabase
  .from('books')
  .update({ status: 'completed' })
  .eq('title', 'こころ')
  .select('title, status');
console.log('(3)', { data: ok.data, error: ok.error });

// 更新できた件数は data の長さで分かる
console.log('更新された件数:', ok.data?.length ?? 0);

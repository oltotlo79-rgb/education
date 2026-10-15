// 04-single.ts : 0件のときの .single() と .maybeSingle() の違い
// 実行方法: node --env-file=.env.local 04-single.ts
import { createClient } from '@supabase/supabase-js';

const supabase = createClient(
  process.env.NEXT_PUBLIC_SUPABASE_URL!,
  process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!
);

// 存在しない id（形式は正しい UUID）
const missingId = '00000000-0000-0000-0000-000000000000';

// (1) .single() : 「ちょうど1件」でなければエラー
const one = await supabase.from('books').select('title').eq('id', missingId).single();
console.log('(1) single     ->', { data: one.data, error: one.error });

// (2) .maybeSingle() : 0件なら data が null（エラーではない）
const maybe = await supabase.from('books').select('title').eq('id', missingId).maybeSingle();
console.log('(2) maybeSingle->', { data: maybe.data, error: maybe.error });

import { createClient } from '@supabase/supabase-js';
const supabase = createClient(process.env.NEXT_PUBLIC_SUPABASE_URL!, process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!);
{
const { data, error } = await supabase
  .from('books')
  .insert({ title: 'テスト', author: 'テスト', rating: 10 })
  .select();
if (error) {
  console.error('追加に失敗しました:', error.message);
} else {
  console.log(`${data.length}件追加しました`);
}
}
{
const { data, error } = await supabase
  .from('books')
  .insert({ title: 'こころ', author: '夏目漱石' })
  .select('id')
  .single();
if (error) {
  console.error('登録に失敗しました:', error.message);
} else {
  console.log('作られた本の id:', data.id);
}
}
{
const { data, error } = await supabase.from('book').select('*');
console.log({ data, error });
}

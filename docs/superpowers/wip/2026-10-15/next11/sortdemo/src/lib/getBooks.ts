// Supabase に接続するための関数を読み込む
import { createClient } from '@supabase/supabase-js'

// 接続先の URL と公開用キー（本物の値は .env.local から読む）
const supabase = createClient(
  process.env.NEXT_PUBLIC_SUPABASE_URL ?? '',
  process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY ?? '',
)

// 書籍をタイトル順で取得する（AI が提案した、存在しないメソッドを使う版）
export async function getBooksSortedByTitle() {
  const { data, error } = await supabase
    .from('books')
    .findMany({ orderBy: { title: 'asc' } })
  if (error) throw error
  return data
}

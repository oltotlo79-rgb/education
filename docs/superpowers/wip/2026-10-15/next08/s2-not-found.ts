// Supabase の .single() / .maybeSingle() が返す { data, error } の
// 「形」を真似した練習用コード（本物の supabase-js ではありません）

type Book = {
  id: string;
  title: string;
};

type QueryResult = {
  data: Book | null;
  error: { code: string } | null;
};

// books テーブルの代わりの配列（id は UUID 形式の文字列）
const books: Book[] = [
  { id: "3f2a8c1e-5b7d-4e9a-9c01-2b3d4e5f6a7b", title: "React実践ガイド" },
];

// UUID の形（8-4-4-4-12 桁の16進数）かどうかを調べる正規表現
const UUID_PATTERN =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

// .eq("id", id).single() の動きを真似する
function findSingle(id: string): QueryResult {
  // UUID の形をしていない値は、DB が型エラー（22P02）で拒否する
  if (!UUID_PATTERN.test(id)) {
    return { data: null, error: { code: "22P02" } };
  }
  const rows = books.filter((book) => book.id === id);
  // ちょうど1件でなければエラー（PGRST116）
  if (rows.length !== 1) {
    return { data: null, error: { code: "PGRST116" } };
  }
  return { data: rows[0], error: null };
}

// .eq("id", id).maybeSingle() の動きを真似する
function findMaybeSingle(id: string): QueryResult {
  if (!UUID_PATTERN.test(id)) {
    return { data: null, error: { code: "22P02" } };
  }
  const rows = books.filter((book) => book.id === id);
  // 0件のときはエラーにせず data: null を返すのが maybeSingle
  if (rows.length === 0) {
    return { data: null, error: null };
  }
  return { data: rows[0], error: null };
}

// page.tsx の「if (error || !book) notFound();」と同じ判定
function decide(result: QueryResult): string {
  if (result.error || !result.data) {
    return "notFound() → 404 ページ";
  }
  return `詳細ページを表示「${result.data.title}」`;
}

const testIds = [
  // 1. 存在する本の id
  "3f2a8c1e-5b7d-4e9a-9c01-2b3d4e5f6a7b",
  // 2. 形は正しいが、存在しない id
  "00000000-0000-4000-8000-000000000000",
  // 3. そもそも UUID の形をしていない id
  "abc",
];

for (const id of testIds) {
  const single = findSingle(id);
  const maybe = findMaybeSingle(id);
  console.log(`id = ${id}`);
  console.log(
    `  single()      error=${single.error?.code ?? "なし"} → ${decide(single)}`,
  );
  console.log(
    `  maybeSingle() error=${maybe.error?.code ?? "なし"} → ${decide(maybe)}`,
  );
}

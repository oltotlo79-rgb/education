// app/search/page.tsx
// URL の例: /search?q=react&page=2
export default async function SearchPage({
  searchParams,
}: {
  searchParams: Promise<{ q?: string; page?: string }>;
}) {
  // ? 以降の値を取り出す。無いときは右側の値（デフォルト値）を使う
  const { q = "", page = "1" } = await searchParams;
  // URL から来る値は文字列なので、計算に使う前に数値へ変換する
  const pageNumber = Number(page);

  return (
    <div>
      <h1>「{q}」の検索結果</h1>
      <p>
        {pageNumber} ページ目を表示中（次は {pageNumber + 1} ページ目）
      </p>
    </div>
  );
}

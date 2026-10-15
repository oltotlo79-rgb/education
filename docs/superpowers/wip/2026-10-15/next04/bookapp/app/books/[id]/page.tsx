// app/books/[id]/page.tsx（/books/42 などで表示されるページ）
export default async function BookDetailPage({
  params,
}: {
  params: Promise<{ id: string }>;
}) {
  const { id } = await params;
  console.log(`[BookDetailPage] id=${id} のページを描画しました`);

  return <h1>書籍 {id} の詳細</h1>;
}

// app/books/layout.tsx（/books 以下の全ページを包む枠）
export default function BooksLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  console.log("[BooksLayout] サーバーで描画しました");

  return (
    <div className="books-layout">
      <aside>書籍メニュー</aside>
      {/* children の位置に page.tsx の中身が入る */}
      <section>{children}</section>
    </div>
  );
}

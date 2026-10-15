import { createClient } from "@/lib/supabase/server";
import BookCard from "@/components/BookCard";
import type { Book } from "@/types/book";

export default async function HomePage() {
  const supabase = await createClient();
  const { data: books, error } = await supabase
    .from("books")
    .select("*")
    .order("created_at", { ascending: false });

  if (error) {
    console.error("書籍の取得に失敗しました:", error.message);
  }

  const bookList: Book[] = books ?? [];

  return (
    <main className="mx-auto max-w-3xl p-6">
      <h1 className="text-2xl font-bold">書籍管理アプリ</h1>
      <div className="mt-4 grid gap-3">
        {bookList.map((book) => (
          <BookCard key={book.id} book={book} />
        ))}
      </div>
    </main>
  );
}

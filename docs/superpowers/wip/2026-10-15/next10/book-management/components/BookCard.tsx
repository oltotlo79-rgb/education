import Link from "next/link";
import type { Book } from "@/types/book";

export default function BookCard({ book }: { book: Book }) {
  return (
    <Link href={`/books/${book.id}`} className="block rounded border p-4">
      <h2 className="font-bold">{book.title}</h2>
      <p className="text-sm text-gray-500">{book.author}</p>
    </Link>
  );
}

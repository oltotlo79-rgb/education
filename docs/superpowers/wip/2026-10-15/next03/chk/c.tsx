import { useEffect, useState } from "react";
export function Books() {
  const [books, setBooks] = useState<string[]>([]);
  useEffect(async () => {
    const response = await fetch("/api/books");
    setBooks(await response.json());
  }, []);
  return <p>{books.length}</p>;
}

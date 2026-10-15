import { useEffect, useState } from "react";
export function Timer() {
  const [seconds, setSeconds] = useState(0);
  useEffect(() => {
    const id = setInterval(() => {
      setSeconds(seconds + 1);
    }, 1000);
    return () => clearInterval(id);
  }, []);
  return <p>{seconds}</p>;
}
export function Cond({ show }: { show: boolean }) {
  if (show) {
    const [a, setA] = useState(0);
  }
  return <p />;
}
export function Books() {
  const [books, setBooks] = useState<string[]>([]);
  useEffect(async () => {
    const response = await fetch("/api/books");
    setBooks(await response.json());
  }, []);
  return <p>{books.length}</p>;
}

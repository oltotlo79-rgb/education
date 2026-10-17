"use client";

export default async function BooksPage() {
  const books = await fetch("http://localhost:3000/api/none");
  return <div>{books.status}</div>;
}

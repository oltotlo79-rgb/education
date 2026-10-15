interface Book {
  title: string;
}

function getTitle(book: Book | null): string {
  // null かどうかは === null（または !== null）で直接調べる
  if (book !== null) {
    return book.title;
  }
  return "（本がありません）";
}

console.log(getTitle({ title: "TypeScript入門" }));
console.log(getTitle(null));

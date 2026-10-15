interface Book {
  title: string;
}

function getTitle(book: Book | null): string {
  // 「オブジェクトなら」のつもりで typeof を使った
  if (typeof book === "object") {
    return book.title;
  }
  return "（本がありません）";
}

console.log(typeof null);
console.log(getTitle(null));

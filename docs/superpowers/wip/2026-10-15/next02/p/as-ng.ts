interface Book {
  title: string;
  price: number;
}

// サーバーから届いた JSON 文字列（price が抜けている！）
const json = '{"title":"TypeScript入門"}';

// as Book で「これは Book です」と言い切ってしまう
const book = JSON.parse(json) as Book;
console.log(book.price.toLocaleString());

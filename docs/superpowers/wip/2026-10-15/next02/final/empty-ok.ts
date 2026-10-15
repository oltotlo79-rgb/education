// 本棚の「形」を先に型で決めておく
interface Shelf {
  name: string;
  books: string[];
}

// 型注釈を付けたので、books は「string の配列」だと分かる
const shelf: Shelf = {
  name: "本棚A",
  books: [],
};

shelf.books.push("TypeScript入門");
console.log(shelf);

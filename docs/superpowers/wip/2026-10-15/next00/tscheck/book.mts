// 「本」のデータの形（型）を決める。Book という名前を付ける
type Book = {
  title: string;
  author: string;
  rating: number;
};

// 型 Book のルールに従って、本のデータを1冊ぶん作る
const book: Book = {
  title: "リーダブルコード",
  author: "Dustin Boswell",
  rating: 5,
};

// 本を受け取り、紹介文（文字列）を作って返す関数
function describe(b: Book): string {
  return `『${b.title}』（${b.author}）の評価は ★${b.rating} です`;
}

// 関数を呼び出して、返ってきた文字列を変数に入れる
const message = describe(book);
console.log(message);

// 本を2冊並べた配列（リスト）を作る
const shelf: Book[] = [
  book,
  { title: "達人プログラマー", author: "David Thomas", rating: 4 },
];

// 配列の長さ（何冊あるか）を表示する
console.log(`本棚には ${shelf.length} 冊あります`);

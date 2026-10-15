const books = [
  { id: "a1", title: "リーダブルコード" },
  { id: "b2", title: "JavaScript Primer" },
];

// ❌ NG: { } で囲んだのに return を書いていない
const ng = books.map((book) => {
  `カード: ${book.title}`;
});

// ✅ OK その1: ( ) で囲む（または何も囲まない）と、式の値がそのまま返る
const ok1 = books.map((book) => `カード: ${book.title}`);

// ✅ OK その2: { } を使うなら return を書く
const ok2 = books.map((book) => {
  return `カード: ${book.title}`;
});

console.log("NG :", ng);
console.log("OK1:", ok1);
console.log("OK2:", ok2);

// ① 数値（プリミティブ）を別の変数に入れる
let a = 10;
let b = a;
// b だけを 20 に変える
b = 20;
console.log("a =", a, "/ b =", b);

// ② オブジェクトを別の変数に入れる
const book1 = { title: "TypeScript入門", price: 2980 };
const book2 = book1;
// book2 の price だけを変えたつもり
book2.price = 1980;
console.log("book1:", book1);
console.log("book2:", book2);
console.log("book1 と book2 は同じもの？", book1 === book2);

// ③ スプレッド構文 { ...元 } で「中身」をコピーする
const book3 = { ...book1 };
book3.price = 500;
console.log("book1:", book1);
console.log("book3:", book3);
console.log("book1 と book3 は同じもの？", book1 === book3);

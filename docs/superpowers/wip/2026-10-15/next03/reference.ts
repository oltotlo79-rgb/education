// 本のデータの型（info の中にもう1段オブジェクトがある）
type Book = {
  title: string;
  info: { pages: number };
};

// ---- NG: 直接書き換える ----
const before1: Book = { title: "React入門", info: { pages: 200 } };
// 「新しい変数」に入れても、指している箱は同じ
const after1 = before1;
after1.title = "React入門 第2版";
// React は Object.is(前の値, 次の値) で「変わったか」を判定する
console.log("NG: 同じ箱か →", Object.is(before1, after1));
console.log("NG: 元の title →", before1.title);

// ---- OK: スプレッド構文で新しい箱を作る ----
const before2: Book = { title: "React入門", info: { pages: 200 } };
const after2 = { ...before2, title: "React入門 第2版" };
console.log("OK: 同じ箱か →", Object.is(before2, after2));
console.log("OK: 元の title →", before2.title);

// ---- 注意: スプレッド構文は「1段目」しかコピーしない ----
const after3 = { ...before2 };
console.log("info は同じ箱か →", Object.is(before2.info, after3.info));

// ネストした値を変えるときは、その段もスプレッドで作り直す
const after4 = { ...before2, info: { ...before2.info, pages: 250 } };
console.log("作り直した info は同じ箱か →", Object.is(before2.info, after4.info));
console.log("元の pages →", before2.info.pages, "/ 新しい pages →", after4.info.pages);

// Server Action に「どの本か（id）」を渡す 2 つの方法を、
// Node.js の FormData を使って真似する練習用コード

const bookId = "3f2a8c1e-5b7d-4e9a-9c01-2b3d4e5f6a7b";

// 方法A 用: 1番目の引数で id、2番目でフォームの中身を受け取る
async function updateBook(id: string, formData: FormData) {
  const title = formData.get("title");
  console.log(`  → updateBook 実行: id=${id} / title=${title}`);
}

// 方法B 用: フォームの中身（hidden input を含む）だけを受け取る
async function updateBookFromHidden(formData: FormData) {
  const id = formData.get("id");
  const title = formData.get("title");
  // get() の結果は string / File / null のどれかなので確認する
  if (typeof id !== "string" || id === "") {
    console.log("  → id が届いていないので更新しない");
    return;
  }
  console.log(`  → updateBookFromHidden 実行: id=${id} / title=${title}`);
}

// ブラウザが送ってくるフォームの中身を作る（タイトル欄だけ）
const formData = new FormData();
formData.append("title", "React実践ガイド 第2版");

// id だけを受け取る削除用の関数（NG と OK の比較に使う）
async function deleteBook(id: string) {
  console.log(`  → deleteBook 実行: id=${id}`);
}

async function main() {
  console.log("【方法A】bind で id を先に固定する");
  const updateBookWithId = updateBook.bind(null, bookId);
  // 送信されたときに、フォームの中身だけを渡して呼ばれる
  await updateBookWithId(formData);

  console.log("【方法B】hidden input で id も一緒に送る");
  const formWithHidden = new FormData();
  formWithHidden.append("id", bookId);
  formWithHidden.append("title", "React実践ガイド 第2版");
  await updateBookFromHidden(formWithHidden);

  console.log("【方法B の失敗例】hidden input を書き忘れた");
  await updateBookFromHidden(formData);

  console.log("【NG と OK】() を付けるとその場で実行される");
  // OK: bind は「あとで呼ぶための関数」を作るだけ（まだ実行しない）
  const deleteLater = deleteBook.bind(null, bookId);
  console.log("  bind の結果:", typeof deleteLater);
  // NG: () を付けると、この行でもう削除が実行されてしまう
  const deleteNow = deleteBook(bookId);
  console.log("  () の結果:", Object.prototype.toString.call(deleteNow));
}

main();

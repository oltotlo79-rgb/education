// 指定したミリ秒だけ待ってから「完了」になる Promise を返す関数
function wait(ms: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

interface Book {
  id: string;
  title: string;
}

// サーバーから本を取ってくる「ふり」をする関数（300ミリ秒かかる）
async function fetchBook(id: string): Promise<Book> {
  await wait(300);
  return { id, title: `本その${id}` };
}

async function main(): Promise<void> {
  console.log("1. 本の取得を開始します");
  // fetchBook が終わるまで、main の中だけがここで一時停止する
  const book = await fetchBook("1");
  console.log("3. 取得できました:", book.title);
}

main();
// main() は await で止まっている間に呼び出し元へ戻ってくるので、先にこちらが表示される
console.log("2. main() を呼んだ直後の行です");

// 本物の Supabase の代わりに使う「にせものの取得関数」
// 0.3秒後に書籍の配列を返す（ネット越しの問い合わせをまねている）
function fakeSelectBooks(): Promise<{ title: string }[]> {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve([{ title: "リーダブルコード" }, { title: "JavaScript Primer" }]);
    }, 300);
  });
}

// ❌ await を付けずに呼んだ場合
function withoutAwait() {
  const data = fakeSelectBooks();
  console.log("await なし:", data);
}

// ✅ async 関数の中で await を付けて呼んだ場合
async function withAwait() {
  const data = await fakeSelectBooks();
  console.log("await あり:", data);
}

// 2つを順番に試す
async function main() {
  withoutAwait();
  await withAwait();
  console.log("おしまい");
}

main();

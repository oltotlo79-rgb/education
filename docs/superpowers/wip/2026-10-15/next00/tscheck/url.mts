// 分解してみたい URL を2つ、配列に入れて用意する
const addresses: string[] = [
  "https://example.com:443/about?q=book#section1",
  "http://localhost:3000/books/new",
];

// 配列の中身を1つずつ取り出して、同じ処理をくり返す
for (const address of addresses) {
  // new URL(...) で、URL の文字列を「部品ごとに分けた形」に変換する
  const url = new URL(address);

  console.log(`▼ ${address}`);
  console.log(`  スキーム    : ${url.protocol}`);
  console.log(`  ホスト名    : ${url.hostname}`);
  console.log(`  ポート番号  : "${url.port}"`);
  console.log(`  パス        : ${url.pathname}`);
  console.log(`  クエリ文字列: ${url.search}`);
  console.log(`  q の値      : ${url.searchParams.get("q")}`);
  console.log(`  フラグメント: ${url.hash}`);
}

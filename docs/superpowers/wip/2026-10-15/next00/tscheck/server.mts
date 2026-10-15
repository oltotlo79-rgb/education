// Node.js に最初から入っている「HTTP サーバーを作る道具」を読み込む
import { createServer } from "node:http";

// サーバー（お店の厨房）を作る。リクエストが届くたびに中の関数が呼ばれる
const server = createServer((req, res) => {
  // どんな注文（リクエスト）が来たかを記録する
  console.log(`[サーバー] 注文を受けました: ${req.method} ${req.url}`);

  // 「成功(200)」と「中身は HTML です」という情報を先に返す
  res.writeHead(200, { "Content-Type": "text/html; charset=utf-8" });

  // 料理（HTML）を渡して、このやり取りを終える
  res.end("<h1>ようこそ、本の店へ</h1>");
});

// 3000 番の窓口（ポート）で、リクエストの待ち受けを始める
server.listen(3000, async () => {
  console.log("[サーバー] http://localhost:3000 で待っています");

  // ブラウザの代わりに fetch でリクエストを送ってみる
  const response = await fetch("http://localhost:3000/about");
  console.log(`[ブラウザ役] ステータスコード: ${response.status}`);

  // 返ってきた本文（HTML）を文字列として受け取る
  const html = await response.text();
  console.log(`[ブラウザ役] 届いた HTML: ${html}`);

  // 実験が終わったので、サーバーを止める
  server.close();
});

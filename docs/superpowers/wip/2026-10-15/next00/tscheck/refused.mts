// サーバーが待ち受けていない 3999 番の窓口に、リクエストを送ってみる
try {
  await fetch("http://localhost:3999/");
  console.log("つながりました");
} catch (error) {
  // つながらなかったときは、ここに来る
  console.log("つながりませんでした");
  if (error instanceof Error) {
    console.log(`エラーの内容: ${error.message}`);
    // 詳しい原因（cause）の中にある「エラーコード」を取り出す
    const cause = error.cause as { code: string };
    console.log(`原因のコード: ${cause.code}`);
  }
}

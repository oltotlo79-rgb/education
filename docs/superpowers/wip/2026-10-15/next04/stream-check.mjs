// stream-check.mjs（ページが「何秒目に・どんな中身で」届くかを調べる）
const start = Date.now();
const response = await fetch("http://localhost:3000/slow");
const decoder = new TextDecoder();

// レスポンスの本文を「届いた塊（チャンク）ごと」に読む
for await (const chunk of response.body) {
  const text = decoder.decode(chunk, { stream: true });
  const seconds = ((Date.now() - start) / 1000).toFixed(2);
  const found = [];
  if (text.includes("読み込み中です")) found.push("loading.tsx の中身");
  if (text.includes("リーダブルコード")) found.push("page.tsx の中身");
  console.log(`${seconds}秒: ${chunk.length}バイト受信 ${found.join(" / ")}`);
}

// サーバーから JSON を受け取る（fetch と async / await）
import { createServer } from "node:http";
import type { AddressInfo } from "node:net";

type RequestSummary = { id: number; title: string; status: string };

// 1. 練習用に、このパソコンの中で小さなサーバーを動かす（本物のアプリでは Django や Topcoat）
const server = createServer((req, res) => {
  if (req.url === "/api/requests") {
    res.writeHead(200, { "Content-Type": "application/json; charset=utf-8" });
    res.end(JSON.stringify([{ id: 1, title: "治具の製作", status: "回覧中" }]));
  } else {
    res.writeHead(404).end();
  }
});
await new Promise<void>((resolve) => server.listen(0, "127.0.0.1", resolve));
const base = `http://127.0.0.1:${(server.address() as AddressInfo).port}`;

// 2. async の関数の中では、await で「返事が来るまで待つ」と書ける
async function fetchRequests(): Promise<RequestSummary[]> {
  const res = await fetch(`${base}/api/requests`);
  if (!res.ok) throw new Error(`サーバーのエラー: ${res.status}`);
  return (await res.json()) as RequestSummary[];
}

try {
  const list = await fetchRequests();
  console.log("受け取った:", list);
  await fetch(`${base}/api/nothing`).then((r) => console.log("無い URL:", r.status));
} catch (e) {
  console.error("失敗:", e);
} finally {
  server.close();
}

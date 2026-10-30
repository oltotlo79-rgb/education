// Better Auth を Node.js の素の HTTP サーバーで動かす最小構成（比較用）
import http from "node:http";
import { DatabaseSync } from "node:sqlite";
import { betterAuth } from "better-auth";
import { toNodeHandler } from "better-auth/node";
import { getMigrations } from "better-auth/db/migration";

export const auth = betterAuth({
  baseURL: "http://localhost:3400",
  // 本番では環境変数 BETTER_AUTH_SECRET に 32 文字以上のランダムな値を入れる
  secret: "demo-secret-demo-secret-demo-secret-0123",
  database: new DatabaseSync("ba.sqlite"),
  emailAndPassword: { enabled: true },
  // 開発モードでは既定で無効なので、動きを見るために有効にする
  rateLimit: { enabled: true },
});

// テーブル（user / session / account / verification）を作る
const { runMigrations } = await getMigrations(auth.options);
await runMigrations();

http.createServer(toNodeHandler(auth)).listen(3400, () => {
  console.log("Better Auth: http://localhost:3400/api/auth");
});

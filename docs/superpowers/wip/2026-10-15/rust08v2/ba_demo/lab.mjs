// 第8巻 UNIT 08 の、設定の間違いを試すための小さな道具。
// make(options) で、メモリの中の SQLite を使う Better Auth を作り、表も作る。
import { DatabaseSync } from "node:sqlite";
import { betterAuth } from "better-auth";
import { getMigrations } from "better-auth/db/migration";

export async function make(options) {
  const auth = betterAuth({
    baseURL: "http://localhost:3400",
    database: new DatabaseSync(":memory:"),
    emailAndPassword: { enabled: true },
    // 表を作る前の「表が無い」という知らせは、ここでは消しておく
    logger: { disabled: true },
    ...options,
  });
  await (await getMigrations(auth.options)).runMigrations();
  return auth;
}

/// JSON の POST を送り、状態コードと code（無ければ message）を返す
export async function post(auth, path, body, headers = {}) {
  const response = await auth.handler(
    new Request(`http://localhost:3400/api/auth${path}`, {
      method: "POST",
      headers: { "content-type": "application/json", origin: "http://localhost:3400", ...headers },
      body: JSON.stringify(body),
    }),
  );
  const text = await response.text();
  let message = "";
  try {
    const json = JSON.parse(text);
    message = json.code ?? json.message ?? "";
  } catch {
    message = text.slice(0, 40);
  }
  return `${response.status}${message ? `（${message}）` : ""}`;
}

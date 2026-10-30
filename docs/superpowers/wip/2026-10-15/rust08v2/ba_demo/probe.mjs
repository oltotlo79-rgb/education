// 第8巻 UNIT 08: Better Auth に、RustNote と同じ確かめ（登録・重複・ログインの失敗・回数の制限・
// ほかのサイトからの送信・DB の中身・ログアウト）をして、返事を並べる。
// HTTP のサーバーは立てず、auth.handler に Request を直接渡す（RustNote のテストと同じ形）。
// 実行: node probe.mjs
import { rmSync } from "node:fs";
import { DatabaseSync } from "node:sqlite";
import { betterAuth } from "better-auth";
import { getMigrations } from "better-auth/db/migration";

rmSync("ba.sqlite", { force: true });
const db = new DatabaseSync("ba.sqlite");
const auth = betterAuth({
  baseURL: "http://localhost:3400",
  // 本番では環境変数 BETTER_AUTH_SECRET に、32 文字以上のランダムな値を入れる
  secret: "demo-secret-demo-secret-demo-secret-0123",
  database: db,
  emailAndPassword: { enabled: true },
  // 開発のときは既定で無効なので、動きを見るために有効にする
  rateLimit: { enabled: true },
  // 送り元（IP アドレス）を X-Forwarded-For から読む（回数の制限を、送り元ごとに数えるため）
  advanced: { ipAddress: { ipAddressHeaders: ["x-forwarded-for"] } },
  logger: { disabled: true },
});
// 表（user・session・account・verification）を作る
await (await getMigrations(auth.options)).runMigrations();

const url = (path) => `http://localhost:3400/api/auth${path}`;
let nextIp = 1;
// ip を渡さなければ、毎回ちがう送り元にする（回数の制限に引っかからないように）
async function post(path, body, extra = {}, ip = `192.0.2.${nextIp++}`) {
  const headers = { "content-type": "application/json", origin: "http://localhost:3400", "x-forwarded-for": ip, ...extra };
  const started = performance.now();
  const response = await auth.handler(new Request(url(path), { method: "POST", headers, body: JSON.stringify(body) }));
  const ms = Math.round(performance.now() - started);
  const text = await response.text();
  let message = "";
  try {
    const json = JSON.parse(text);
    message = json.code ?? json.message ?? (json.user ? `user ${json.user.email}` : "");
  } catch {
    message = text.slice(0, 40);
  }
  return { response, ms, message };
}
const cookieOf = (response) => {
  const value = response.headers.get("set-cookie") ?? "";
  const first = value.split(/,(?=\s*better-auth)/)[0];
  const [pair, ...attrs] = first.split(";");
  const [name, v] = pair.split("=");
  return { name, value: v, line: `${name}=${v.slice(0, 6)}…;${attrs.join(";")}` };
};
const password = "correct horse battery";

console.log("--- 1. 登録");
let r = await post("/sign-up/email", { email: "alice@example.com", password, name: "alice" });
const alice = cookieOf(r.response);
console.log(`alice@example.com → ${r.response.status}（${r.message}）`);
console.log(`Set-Cookie: ${alice.line}`);
r = await post("/sign-up/email", { email: "alice@example.com", password, name: "alice" });
console.log(`同じメールでもう一度 → ${r.response.status}（${r.message}）`);
r = await post("/sign-up/email", { email: "ALICE@Example.COM", password, name: "alice" });
console.log(`大文字の同じメール → ${r.response.status}（${r.message}）`);
r = await post("/sign-up/email", { email: "  alice@example.com  ", password, name: "alice" });
console.log(`前後に空白の同じメール → ${r.response.status}（${r.message}）`);
r = await post("/sign-up/email", { email: "bob@example.com", password: "short", name: "bob" });
console.log(`短いパスワード → ${r.response.status}（${r.message}）`);

console.log("--- 2. ログインの失敗（交互に4回ずつ）");
const times = { wrong: [], unknown: [] };
let messages = new Set();
for (let n = 0; n < 4; n++) {
  for (const [key, email] of [["unknown", "dave@example.com"], ["wrong", "alice@example.com"]]) {
    r = await post("/sign-in/email", { email, password: "wrong password" });
    times[key].push(`${r.response.status}（${r.ms} ms）`);
    messages.add(r.message);
  }
}
console.log(`登録していないメール: ${times.unknown.join(" / ")}`);
console.log(`パスワードが違う:     ${times.wrong.join(" / ")}`);
console.log(`返事の code: ${[...messages].join("・")}`);

console.log("--- 3. 1つの送り元から、続けて失敗する");
for (let n = 1; n <= 5; n++) {
  r = await post("/sign-in/email", { email: "alice@example.com", password: "wrong password" }, {}, "198.51.100.7");
  console.log(`${n} 回目 → ${r.response.status}（${r.message}）`);
}
r = await post("/sign-in/email", { email: "alice@example.com", password }, {}, "198.51.100.8");
console.log(`別の送り元から、正しいパスワード → ${r.response.status}（${r.message}）`);

console.log("--- 4. ほかのサイトから");
r = await post("/sign-up/email", { email: "carol@example.com", password, name: "carol" }, { origin: "https://evil.example" });
console.log(`Origin: https://evil.example の登録 → ${r.response.status}（${r.message}）`);

console.log("--- 5. DB の中身");
const session = db.prepare("SELECT token, expiresAt, createdAt FROM session ORDER BY createdAt LIMIT 1").get();
console.log(`session.token: ${session.token.slice(0, 6)}…（${session.token.length} 文字）`);
const cookieValue = decodeURIComponent(alice.value);
console.log(`クッキーの値の先頭は session.token と同じ: ${cookieValue.startsWith(session.token)}`);
console.log(`クッキーの値: トークン（${session.token.length} 文字）＋「${cookieValue[session.token.length]}」＋署名（${cookieValue.length - session.token.length - 1} 文字）`);
const days = (new Date(session.expiresAt) - new Date(session.createdAt)) / 86400000;
console.log(`期限まで: ${days} 日`);
const account = db.prepare("SELECT password FROM account LIMIT 1").get();
const [salt, hash] = account.password.split(":");
console.log(`account.password: ${salt.slice(0, 6)}…:${hash.slice(0, 6)}…（salt ${salt.length} 文字・ハッシュ ${hash.length} 文字）`);

console.log("--- 6. ログアウト");
const before = db.prepare("SELECT COUNT(*) AS n FROM session").get().n;
r = await post("/sign-out", {}, { cookie: `${alice.name}=${alice.value}` });
const after = db.prepare("SELECT COUNT(*) AS n FROM session").get().n;
console.log(`sign-out → ${r.response.status}（session: ${before} 行 → ${after} 行）`);

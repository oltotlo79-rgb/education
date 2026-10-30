// 第8巻 UNIT 08 エラー3: 送り元の IP が分からないまま、回数の制限を有効にする
// （手前の中継のサーバーが、送り元の IP を伝えていない場合）。4人が、1回ずつログインに失敗する
// 実行: node e3_no_ip.mjs
import { make, post } from "./lab.mjs";

const auth = await make({
  secret: "demo-secret-demo-secret-demo-secret-0123",
  rateLimit: { enabled: true },
  logger: { level: "warn" },
});
for (let n = 1; n <= 4; n++) {
  const result = await post(auth, "/sign-in/email", { email: `user${n}@example.com`, password: "wrong password" });
  console.log(`user${n} → ${result}`);
}

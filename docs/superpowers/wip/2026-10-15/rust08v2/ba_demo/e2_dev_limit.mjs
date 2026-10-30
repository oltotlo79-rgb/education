// 第8巻 UNIT 08 エラー2: rateLimit を書かずに、開発のとき（NODE_ENV が production でない）に動かす
// 実行: node e2_dev_limit.mjs
import { make, post } from "./lab.mjs";

const auth = await make({ secret: "demo-secret-demo-secret-demo-secret-0123" });
await post(auth, "/sign-up/email", { email: "alice@example.com", password: "correct horse battery", name: "alice" });
for (let n = 1; n <= 6; n++) {
  const result = await post(auth, "/sign-in/email", { email: "alice@example.com", password: "wrong password" }, { "x-forwarded-for": "198.51.100.7" });
  console.log(`${n} 回目 → ${result}`);
}

// 第8巻 UNIT 08 エラー1: secret を設定しないで、NODE_ENV=production で起動する
// 実行: $env:NODE_ENV = "production"; node e1_no_secret.mjs
import { make, post } from "./lab.mjs";

delete process.env.BETTER_AUTH_SECRET;
const auth = await make({});
console.log("作れた");
console.log(await post(auth, "/sign-up/email", { email: "alice@example.com", password: "correct horse battery", name: "alice" }));

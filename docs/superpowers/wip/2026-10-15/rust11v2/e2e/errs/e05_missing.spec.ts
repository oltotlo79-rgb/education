// 第11巻 UNIT 05 エラー2: 画面に無いラベルで、入力欄を探した（「メールアドレス」を「Eメール」と書いた）。
import { test } from "@playwright/test";

test("メールアドレスを入れる", async ({ page }) => {
  test.setTimeout(5_000);
  await page.goto("/login");
  await page.getByLabel("Eメール").fill("alice@example.com");
});

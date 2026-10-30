// 第11巻 UNIT 05 エラー1: ラベルの一部だけで、入力欄を探した。
import { test } from "@playwright/test";

test("パスワードを入れる", async ({ page }) => {
  await page.goto("/signup");
  await page.getByLabel("パスワード").fill("correct horse battery");
});

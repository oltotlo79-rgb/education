// 第11巻 UNIT 05 問2: 入力欄を通らずに、絵文字 80 個の題を、フォームの形で直接送る。
import { expect, test } from "@playwright/test";

test("絵文字 80 個の題を、直接送る", async ({ page }) => {
  await page.goto("/signup");
  await page.getByLabel("メールアドレス").fill(`q2${Date.now()}@example.com`);
  await page.getByLabel("パスワード（8文字以上）").fill("correct horse battery");
  await page.getByLabel("パスワード（確認）").fill("correct horse battery");
  await page.getByRole("button", { name: "登録する" }).click();

  // page.request は、このページと同じクッキーで、HTTP のリクエストを直接送る
  const title = "😀".repeat(80);
  const response = await page.request.post("/notes", {
    form: { title, body: "" },
    maxRedirects: 0,
  });
  console.log(`状態コード: ${response.status()}・移動先: ${response.headers()["location"]}`);
  await page.goto(response.headers()["location"]);
  const shown = await page.getByRole("heading", { level: 1 }).textContent();
  console.log(`詳細の題: ${[...(shown ?? "")].length} 文字（UTF-16 で ${shown?.length}）`);
  expect([...(shown ?? "")].length).toBe(80);
});

// 第11巻 UNIT 05 問3: 一覧の画面で Tab を押していき、フォーカスが移る順番を記録する。
import { test } from "@playwright/test";

test("Tab の順番", async ({ page }) => {
  await page.goto("/signup");
  await page.getByLabel("メールアドレス").fill(`q3${Date.now()}@example.com`);
  await page.getByLabel("パスワード（8文字以上）").fill("correct horse battery");
  await page.getByLabel("パスワード（確認）").fill("correct horse battery");
  await page.getByRole("button", { name: "登録する" }).click();
  await page.getByText("新しいノートを書く").click();
  await page.getByLabel("タイトル（80文字まで）").fill("牛乳を買う");
  await page.getByRole("button", { name: "作成する" }).click();
  await page.goto("/notes");

  for (let i = 1; i <= 8; i++) {
    await page.keyboard.press("Tab");
    // 今フォーカスがある要素を、タグの名前と、見えている文字（無ければ name か placeholder）で表す
    const focused = await page.evaluate(() => {
      const el = document.activeElement as HTMLElement | null;
      if (!el || el === document.body) return "（ページ全体）";
      const text = (el.innerText || el.getAttribute("placeholder") || el.getAttribute("name") || "").trim();
      return `${el.tagName.toLowerCase()}「${text.split("\n")[0]}」`;
    });
    console.log(`Tab ${i} 回目: ${focused}`);
  }
});

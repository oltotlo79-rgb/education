// 第11巻 UNIT 05 例5-2: 題の入力欄（maxlength="80"）に、ブラウザでは何文字まで入るか。
// UNIT 01 で「HTML の決まりでは、maxlength は UTF-16 の単位で数える」と書いたことを、本物の Chrome で確かめる。
import { expect, test } from "@playwright/test";

test.beforeEach(async ({ page }) => {
  // 登録して、一覧の「新しいノートを書く」を開く
  await page.goto("/signup");
  await page.getByLabel("メールアドレス").fill(`user${Date.now()}@example.com`);
  await page.getByLabel("パスワード（8文字以上）").fill("correct horse battery");
  await page.getByLabel("パスワード（確認）").fill("correct horse battery");
  await page.getByRole("button", { name: "登録する" }).click();
  await page.getByText("新しいノートを書く").click();
});

for (const [name, one] of [
  ["半角の a", "a"],
  ["ひらがなの あ", "あ"],
  ["絵文字の 😀", "😀"],
]) {
  test(`${name} を 100 個打つ`, async ({ page }) => {
    const title = page.getByLabel("タイトル（80文字まで）");
    // キーボードで1文字ずつ打つ（人が打つのと同じ）
    await title.pressSequentially(one.repeat(100));
    const value = await title.inputValue();
    console.log(
      `${name}: 入った文字数 ${[...value].length}・UTF-16 の数 ${value.length}`,
    );

    // そのまま作成すると、サーバーはどう受け取るか
    await page.getByRole("button", { name: "作成する" }).click();
    await expect(page.getByRole("heading", { level: 1 })).toHaveText(value);
    console.log(`${name}: 作成した → ${page.url().replace(/^.*(\/notes\/\d+)$/, "$1")}`);
  });
}

test("fill で入れると、maxlength はどうなるか", async ({ page }) => {
  const title = page.getByLabel("タイトル（80文字まで）");
  await title.fill("a".repeat(100));
  const value = await title.inputValue();
  console.log(`fill: 入った文字数 ${value.length}`);
});

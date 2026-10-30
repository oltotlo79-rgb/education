// 第11巻 UNIT 05 例5-1: いちばん大事な流れを、本物のブラウザで1本だけ確かめる。
// 登録 → ノートを作る → 一覧に出る → ログアウト → 一覧はログインを求める
import { expect, test } from "@playwright/test";

test("登録してノートを作り、ログアウトする", async ({ page }) => {
  // 1. 登録する（入力欄は、画面に書かれた名前（ラベル）で探す）
  await page.goto("/signup");
  await page.getByLabel("メールアドレス").fill("alice@example.com");
  await page.getByLabel("パスワード（8文字以上）").fill("correct horse battery");
  await page.getByLabel("パスワード（確認）").fill("correct horse battery");
  await page.getByRole("button", { name: "登録する" }).click();
  await expect(page).toHaveURL("/notes");
  await expect(page.getByText("登録しました。ようこそ RustNote へ！")).toBeVisible();

  // 2. ノートを作る（「新しいノートを書く」を開いてから書く）
  await page.getByText("新しいノートを書く").click();
  await page.getByLabel("タイトル（80文字まで）").fill("牛乳を買う");
  await page.getByLabel("本文（10000文字まで）").fill("低脂肪のもの");
  await page.getByRole("button", { name: "作成する" }).click();
  await expect(page).toHaveURL(/\/notes\/\d+$/);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("牛乳を買う");

  // 3. 一覧に出る
  await page.goto("/notes");
  await expect(page.getByRole("link", { name: "牛乳を買う" })).toBeVisible();

  // 4. ログアウトすると、一覧はログインを求める
  await page.getByRole("button", { name: "ログアウト" }).click();
  await expect(page).toHaveURL("/login");
  await page.goto("/notes");
  await expect(page).toHaveURL("/login");
  await expect(page.getByText("ログインしてください")).toBeVisible();
});

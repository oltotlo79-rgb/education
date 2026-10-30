// 第11巻 UNIT 05 例5-3: axe（アクセシビリティの自動の検査）で、RustNote の画面を調べる。
import AxeBuilder from "@axe-core/playwright";
import { test } from "@playwright/test";

test("4つの画面を axe で調べる", async ({ page }) => {
  // 登録して、ノートを1つ作る（一覧と詳細に、中身がある状態にする）
  await page.goto("/signup");
  await page.getByLabel("メールアドレス").fill(`a11y${Date.now()}@example.com`);
  await page.getByLabel("パスワード（8文字以上）").fill("correct horse battery");
  await page.getByLabel("パスワード（確認）").fill("correct horse battery");
  await page.getByRole("button", { name: "登録する" }).click();
  await page.getByText("新しいノートを書く").click();
  await page.getByLabel("タイトル（80文字まで）").fill("牛乳を買う");
  await page.getByRole("button", { name: "作成する" }).click();
  const detail = new URL(page.url()).pathname;

  for (const path of ["/notes", detail, `${detail}/edit`]) {
    await page.goto(path);
    await report(page, path);
  }
  // ログアウトして、ログインの画面も調べる
  await page.getByRole("button", { name: "ログアウト" }).click();
  await report(page, "/login");
});

async function report(page: import("@playwright/test").Page, path: string) {
  const results = await new AxeBuilder({ page }).analyze();
  console.log(`--- ${path}: 違反 ${results.violations.length} 種類・合格 ${results.passes.length} 種類`);
  for (const violation of results.violations) {
    console.log(`${violation.id}（${violation.impact}）: ${violation.nodes.length} か所`);
    for (const node of violation.nodes) {
      console.log(`  ${node.target.join(" ")}: ${node.failureSummary?.split("\n")[1]?.trim()}`);
    }
  }
}

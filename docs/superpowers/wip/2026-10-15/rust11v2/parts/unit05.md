## UNIT 05: E2E・ブラウザ・アクセシビリティ

### この単元のゴール

- Playwright で、本物の Chrome を動かし、RustNote の**いちばん大事な流れ**（登録 → ノートを作る → 一覧 → ログアウト）を、1本のテストで確かめられる
- テストの前に、RustNote のサーバーを**新しい DB**で起動する設定（`webServer`）を書ける
- 入力欄を、画面に書かれた名前（ラベル・ボタンの文字）で探す理由を、アクセシビリティと結び付けて説明できる
- UNIT 01 で「HTML の決まり」として書いた `maxlength` の数え方（UTF-16）を、本物のブラウザで確かめられる
- axe で画面を自動で調べ、見つかった指摘（`landmark-unique`）を読んで直せる

### たとえ話でつかむ

新しいお店の、開店前の日を考えます。

- これまでの単元は、**部品ごとの点検**でした。レジ（ユニットテスト）・倉庫（リポジトリ）・注文の窓口（HTTP のテスト）を、1つずつ確かめました。
- 開店前の日には、店員が**お客さんの役**をして、入口から入り、品物を選び、会計をして、出口から出るまでを、**通しで**歩いてみます。部品が全部正しくても、「入口のドアが重くて開かない」ことは、通して歩かないと分かりません。これが **E2E**（端から端まで）のテストです。
- 全部の品物で通し歩きをすると、日が暮れます。だから、**いちばん大事な道すじ**だけを歩きます。
- 歩きながら、**目の不自由な人・車いすの人**にも使えるかを見ます。棚の名札は読み上げられるか、段差は無いか。これが**アクセシビリティ**の点検です。点検表（axe）を使うと、見落としにくくなります。

| 開店前の日 | RustNote のテスト |
|---|---|
| お客さんの役で、通しで歩く | E2E のテスト（Playwright で、本物の Chrome を動かす） |
| いちばん大事な道すじだけ | 登録 → 作成 → 一覧 → ログアウト の1本（例5-1） |
| 毎朝、店を空にしてから始める | テストのたびに、新しい DB でサーバーを起動する（`webServer`） |
| 棚の名札で品物を探す | ラベルやボタンの文字で、入力欄を探す（`getByLabel`・`getByRole`） |
| 点検表 | axe（例5-3） |

> **用語:** E2E（End to End：利用者と同じ入口（ブラウザ）から、サーバー・DB まで、全部を本物でつないで確かめるテスト）

<!-- -->

> **用語:** Playwright（プレイライト：ブラウザを、プログラムから動かす道具。Node.js で動き、Chrome・Firefox・Safari の元になるエンジンを動かせる）

<!-- -->

> **用語:** アクセシビリティ（障害のある人や、キーボードだけ・画面の読み上げを使う人も、同じように使えること。a11y と略す）

<!-- -->

> **用語:** ランドマーク（画面の大きな区切り。`header`・`nav`・`main` など。画面の読み上げを使う人は、ランドマークの一覧から、行きたい区切りへ飛ぶ）

### 図で理解する

テストの段と、それぞれの数・時間です（RustNote と、この巻で足したテスト）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<polygon points="360,16 450,76 270,76" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="62" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">E2E</text>
<polygon points="270,82 450,82 520,142 200,142" fill="#f5f3ff" stroke="#8b5cf6" stroke-width="2"/>
<text x="360" y="118" font-size="12" font-weight="700" fill="#6d28d9" text-anchor="middle">HTTP のテスト（UNIT 03）</text>
<polygon points="200,148 520,148 590,208 130,208" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="184" font-size="12" font-weight="700" fill="#1d4ed8" text-anchor="middle">リポジトリの結合テスト（UNIT 02）</text>
<polygon points="130,214 590,214 660,262 60,262" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="244" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">ユニットテスト（UNIT 01）: 関数1つ・19 個で 0.01 秒</text>
<text x="470" y="44" font-size="11" fill="#0f172a">本物の Chrome＋サーバー＋DB</text>
<text x="470" y="62" font-size="11" fill="#0f172a">大事な流れだけ・1本で約 1.3 秒</text>
<text x="540" y="110" font-size="11" fill="#0f172a">ルーターに直接（ポートを開かない）</text>
<text x="540" y="128" font-size="11" fill="#0f172a">tests/http.rs の 14 個で約 2 秒</text>
<text x="610" y="176" font-size="11" fill="#0f172a">本物の libSQL</text>
<text x="610" y="194" font-size="11" fill="#0f172a">8 個で約 0.2 秒</text>
<text x="20" y="40" font-size="11" fill="#c2410c">上ほど: 遅い・壊れやすい・少なく</text>
<text x="20" y="58" font-size="11" fill="#047857">下ほど: 速い・たくさん</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: テストの段。E2E は、ほかの段で確かめられない「ブラウザでの見え方・押せ方」と「全部をつないだ流れ」だけを受け持つ</div>
</div>

Playwright のテストが動く仕組みです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-11-0501" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="80" width="170" height="70" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="95" y="106" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">npx playwright test</text>
<text x="95" y="126" font-size="11" fill="#475569" text-anchor="middle">playwright.config.ts</text>
<text x="95" y="142" font-size="11" fill="#475569" text-anchor="middle">tests/*.spec.ts</text>
<rect x="260" y="16" width="200" height="80" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="40" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">① RustNote のサーバー</text>
<text x="360" y="60" font-size="11" fill="#0f172a" text-anchor="middle">127.0.0.1:3011</text>
<text x="360" y="80" font-size="11" fill="#0f172a" text-anchor="middle">新しい DB ファイル</text>
<rect x="260" y="134" width="200" height="80" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="158" font-size="12" font-weight="700" fill="#1d4ed8" text-anchor="middle">② Chrome</text>
<text x="360" y="178" font-size="11" fill="#0f172a" text-anchor="middle">パソコンに入っているもの</text>
<text x="360" y="198" font-size="11" fill="#0f172a" text-anchor="middle">（channel: "chrome"）</text>
<rect x="540" y="80" width="170" height="70" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="625" y="106" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">③ /health</text>
<text x="625" y="126" font-size="11" fill="#0f172a" text-anchor="middle">200 になるまで待って</text>
<text x="625" y="142" font-size="11" fill="#0f172a" text-anchor="middle">からテストを始める</text>
<line x1="182" y1="100" x2="256" y2="62" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-0501)"/>
<text x="200" y="68" font-size="10" fill="#475569">起動する</text>
<line x1="182" y1="130" x2="256" y2="168" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-0501)"/>
<text x="196" y="176" font-size="10" fill="#475569">動かす</text>
<line x1="360" y1="132" x2="360" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-0501)"/>
<text x="368" y="120" font-size="10" fill="#475569">HTTP（本物のクッキー）</text>
<line x1="462" y1="56" x2="536" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-0501)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: Playwright の webServer。テストのたびに、RustNote を新しい DB で起動し、/health が答えたら、Chrome でテストを始める。終わるとサーバーを止める</div>
</div>

### まず動かす最小コード

#### 例5-1: いちばん大事な流れを、本物のブラウザで確かめる

E2E のテストは、Node.js の道具の Playwright で書きます。RustNote のフォルダとは別に、`e2e` という練習用のフォルダを作り、Playwright と、例5-3で使う axe を入れます。ブラウザは、パソコンに入っている Google Chrome を使うので、`PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1` で、ブラウザのダウンロードを止めておきます。

```powershell
mkdir e2e
cd e2e
npm init -y
$env:PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD = "1"
npm install --save-dev --save-exact @playwright/test@1.64.1 @axe-core/playwright@4.14.0
npx playwright --version
```

```text
added 5 packages, and audited 6 packages in 4s
found 0 vulnerabilities
Version 1.64.1
```

Playwright が実際に使った Chrome の版です（`chrome_version.mjs`）。

```text
Chrome 155.0.8059.80
```

RustNote のほうは、先に CSS の束と実行ファイルを作っておきます（`Rust/rustnote` のコピーで）。

```powershell
topcoat asset bundle
cargo build
```

> **▼ このコードがやること（先に日本語で）:** `playwright.config.ts` は、テストの前に、RustNote の実行ファイルを `127.0.0.1:3011` で起動します。DB は、実行のたびに一時フォルダの新しいファイルにします。`/health` が答えるまで待ってから、パソコンの Chrome でテストを始めます。`flow.spec.ts` は、登録 → ノートを作る → 一覧に出る → ログアウト → 一覧がログインを求める、を1本で確かめます。

```ts
// 第11巻 UNIT 05: Playwright の設定。
// - テストの前に、RustNote のサーバーを、新しい DB ファイルで起動する（webServer）
// - ブラウザは、パソコンに入っている Google Chrome を使う（channel: "chrome"。ブラウザをダウンロードしない）
import { defineConfig } from "@playwright/test";
import { tmpdir } from "node:os";
import { join } from "node:path";

// RustNote の実行ファイル（topcoat asset bundle と cargo build を済ませておく）
const server = process.env.RUSTNOTE_EXE ?? "../rustnote/target/debug/rustnote";
// 実行のたびに、別の DB ファイル（前の実行のデータが残らないように）
const database = join(tmpdir(), `rustnote-e2e-${Date.now()}.db`);

export default defineConfig({
  testDir: "./tests",
  // 1つずつ順に動かす（同じサーバーと DB を使うため）
  workers: 1,
  reporter: "list",
  use: {
    baseURL: "http://127.0.0.1:3011",
    channel: "chrome",
  },
  webServer: {
    command: server,
    url: "http://127.0.0.1:3011/health",
    env: { HOST: "127.0.0.1", PORT: "3011", DATABASE_URL: database },
    reuseExistingServer: false,
  },
});
```

```ts
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
```

```powershell
$env:RUSTNOTE_EXE = "…\target\debug\rustnote.exe"
npx playwright test tests/flow.spec.ts
```

**▼ 実行結果**

```text
[WebServer] マイグレーションを適用しました: 0001_users
[WebServer] マイグレーションを適用しました: 0002_notes
[WebServer] マイグレーションを適用しました: 0003_sessions
[WebServer] RustNote を起動しました: http://127.0.0.1:3011
Running 1 test using 1 worker
  ✓  1 tests\flow.spec.ts:5:5 › 登録してノートを作り、ログアウトする (1.3s)
  1 passed (3.1s)
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: サーバーごと、毎回まっさらに

- `[WebServer]` の行は、Playwright が起動した RustNote が書いた行です。3つのマイグレーションを当てているので、DB が**新しいファイル**だと分かります。前の実行で登録した alice は残っていません（UNIT 02 のエラー2と同じ考え方）。
- 出力からは省きましたが、`[WebServer]` には、アクセスログも出ていました。その中に、次の2行がありました。

```text
[WebServer] GET /_topcoat/assets/style-e9b983efabcfc6c3.css -> 200 OK (814.9µs)
[WebServer] GET /favicon.ico -> 404 Not Found (311.7µs)
```

- ブラウザは、CSS（`/_topcoat/assets/…`）のほかに、頼まれなくても `/favicon.ico`（タブに出す小さな絵）を取りに来ます。RustNote には無いので 404 です。HTTP のテスト（UNIT 03）では見えない、ブラウザならではの動きです。

##### 解説2: 画面に書かれた名前で探す

- `getByLabel("メールアドレス")` は、`<label for="email">メールアドレス</label>` が付いた入力欄を探します。`getByRole("button", { name: "登録する" })` は、「登録する」という名前のボタンを探します。
- この探し方は、**画面の読み上げ**が使うのと同じ名前で探します。ラベルが付いていない入力欄は、テストでも探しにくく、読み上げでも、何を入れる欄なのかが読まれません。テストが書きやすい画面は、アクセシビリティも良い画面です。
- `class` や `id` で探す（`page.locator("#email")`）こともできますが、見た目の都合で `class` を変えると、テストが壊れます。利用者に見えている名前で探すほうが、壊れにくくなります。

##### 解説3: `expect` は、待ってくれる

- `await expect(page).toHaveURL("/notes")` は、URL が `/notes` に**なるまで**、少し待ってくれます（既定で 5 秒まで）。ボタンを押してから、次の画面が出るまでの時間を、自分で `sleep` で待つ必要はありません。
- `toHaveText`・`toBeVisible` も同じです。待たずに1回だけ確かめる書き方だと、速いパソコンでは通り、遅い CI では落ちる、「ときどき失敗するテスト」になります。

### もう一歩: 実務に近い形

#### 例5-2: 題の入力欄に、ブラウザでは何文字まで入るか

UNIT 01 では、「HTML の決まりでは、入力欄の `maxlength` は UTF-16 の単位で数える」と書き、本物のブラウザで確かめるのは、この単元に回しました。RustNote の題の入力欄は `maxlength="80"` です。

> **▼ このコードがやること（先に日本語で）:** 登録して「新しいノートを書く」を開いたあと、題の入力欄に、半角の `a`・ひらがなの `あ`・絵文字の `😀` を、それぞれ 100 個、キーボードで1文字ずつ打ちます（`pressSequentially`）。入った文字数と、UTF-16 の数（JavaScript の `length`）を表示し、そのまま作成して、詳細の画面の題が同じかを確かめます。最後に、`fill`（まとめて入れる）でも試します。

```ts
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
```

**▼ 実行結果**

```text
Running 4 tests using 1 worker
半角の a: 入った文字数 80・UTF-16 の数 80
半角の a: 作成した → /notes/1
  ✓  1 tests\maxlength.spec.ts:20:7 › 半角の a を 100 個打つ (1.0s)
ひらがなの あ: 入った文字数 80・UTF-16 の数 80
ひらがなの あ: 作成した → /notes/2
  ✓  2 tests\maxlength.spec.ts:20:7 › ひらがなの あ を 100 個打つ (987ms)
絵文字の 😀: 入った文字数 40・UTF-16 の数 80
絵文字の 😀: 作成した → /notes/3
  ✓  3 tests\maxlength.spec.ts:20:7 › 絵文字の 😀 を 100 個打つ (996ms)
fill: 入った文字数 80
  ✓  4 tests\maxlength.spec.ts:36:5 › fill で入れると、maxlength はどうなるか (841ms)
  4 passed (5.9s)
```

- `a` と `あ` は 80 個まで入りました。`😀` は、**40 個**で止まりました。`😀` は、UTF-16 では2つ分（サロゲートペア）なので、40 個で 80 になるからです。本物の Chrome でも、HTML の決まりのとおりでした。
- サーバーの `NoteTitle::parse` は、`chars().count()` で数えるので、絵文字も 80 個まで受け付けます（問2）。ブラウザのほうが、**厳しい**ことになります。絵文字だけの題を書く人は、40 個で打てなくなります。
- `fill` でも 80 個で止まりました。Playwright の `fill` も、`maxlength` を守って入れます。

#### 例5-3: axe で、アクセシビリティを自動で調べる

axe（アックス）は、画面を調べて、「ラベルの無い入力欄」「色の差が小さすぎる文字」「名前の無いボタン」など、決まった点検の項目を、自動で確かめる道具です。`@axe-core/playwright` で、Playwright のテストの中から動かせます。

> **▼ このコードがやること（先に日本語で）:** 登録してノートを1つ作り、一覧・詳細・編集の画面と、ログアウトしたあとのログインの画面を、axe で調べます。違反（守れていない点検の項目）と合格の数を表示し、違反があれば、どの要素か（`target`）と、直し方の要約を表示します。

```ts
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
```

**▼ 実行結果**

```text
Running 1 test using 1 worker
--- /notes: 違反 1 種類・合格 28 種類
landmark-unique（moderate）: 1 か所
  header > nav: The landmark must have a unique aria-label, aria-labelledby, or title to make landmarks distinguishable
--- /notes/1: 違反 0 種類・合格 21 種類
--- /notes/1/edit: 違反 0 種類・合格 25 種類
--- /login: 違反 0 種類・合格 31 種類
  ✓  1 tests\a11y.spec.ts:5:5 › 4つの画面を axe で調べる (2.4s)
  1 passed (4.3s)
```

- 違反は、一覧の画面（`/notes`）の `landmark-unique`（ランドマークが見分けられない）の1つだけでした。一覧の画面には、上の帯のメニューと、下のページ送りの、**2つの `nav`** があります。どちらにも名前が無いので、画面の読み上げのランドマークの一覧では、「ナビゲーション」「ナビゲーション」と、同じ名前が2つ並びます。
- 直し方は、`nav` に `aria-label` で名前を付けることです（問1）。詳細・編集・ログインの画面には、`nav` が1つしかないので、違反になりませんでした。
- axe が調べられるのは、決まった点検の項目だけです。「Tab で押せる順番が分かりやすいか」「読み上げで意味が通るか」は、人が確かめます（第10巻 UNIT 10・問3）。

### よくあるエラーと読み方

#### エラー1: ラベルの一部だけで探して、2つ見つかった（strict mode violation）

```ts
// 第11巻 UNIT 05 エラー1: ラベルの一部だけで、入力欄を探した。
import { test } from "@playwright/test";

test("パスワードを入れる", async ({ page }) => {
  await page.goto("/signup");
  await page.getByLabel("パスワード").fill("correct horse battery");
});
```

```text
  ✘  1 tests\e05_strict.spec.ts:4:5 › パスワードを入れる (294ms)
    Error: locator.fill: Error: strict mode violation: getByLabel('パスワード') resolved to 2 elements:
        1) <input required="" id="password" name="password" type="password" autocomplete="new-password"/> aka getByRole('textbox', { name: 'パスワード（8文字以上）' })
        2) <input required="" type="password" id="password_confirm" name="password_confirm" autocomplete="new-password"/> aka getByRole('textbox', { name: 'パスワード（確認）' })
    Call log:
      - waiting for getByLabel('パスワード')
    Error Context: test-results\e05_strict-パスワードを入れる\error-context.md
```

- **何が起きたか:** `getByLabel("パスワード")` は、ラベルに「パスワード」を**含む**入力欄を探すので、登録の画面の「パスワード（8文字以上）」と「パスワード（確認）」の**2つ**が見つかりました。Playwright は、どちらに入れるか決められないので、止まります（strict mode：1つに決まらないときは止まる）。
- **読み方:** `1)`・`2)` の行に、見つかった要素と、それぞれを1つに決める書き方（`aka getByRole('textbox', { name: 'パスワード（8文字以上）' })`）が出ています。
- **直し方:** ラベルを全部書きます（例5-1のように `getByLabel("パスワード（8文字以上）")`）。「ちょうど同じ」にしたいときは、`getByLabel("パスワード", { exact: true })` と書きます。

#### エラー2: 画面に無いラベルで探して、時間切れになった

```ts
// 第11巻 UNIT 05 エラー2: 画面に無いラベルで、入力欄を探した（「メールアドレス」を「Eメール」と書いた）。
import { test } from "@playwright/test";

test("メールアドレスを入れる", async ({ page }) => {
  test.setTimeout(5_000);
  await page.goto("/login");
  await page.getByLabel("Eメール").fill("alice@example.com");
});
```

```text
  ✘  1 tests\e05_missing.spec.ts:4:5 › メールアドレスを入れる (5.0s)
    Test timeout of 5000ms exceeded.
    Error: locator.fill: Test timeout of 5000ms exceeded.
    Call log:
      - waiting for getByLabel('Eメール')
    Error Context: test-results\e05_missing-メールアドレスを入れる\error-context.md
```

- **何が起きたか:** ログインの画面のラベルは「メールアドレス」なので、「Eメール」の入力欄は、いつまでも見つかりません。Playwright は、要素が出てくるのを**待ち続け**、テストの時間の上限（ここでは `test.setTimeout(5_000)` の 5 秒。既定は 30 秒）で止まりました。
- **読み方:** `Call log` の `waiting for getByLabel('Eメール')` が、「何を待っていたか」です。時間切れのエラーは、たいてい「探し方が画面と合っていない」か、「前の操作で、思った画面に来ていない」かのどちらかです。`Error Context` のファイル（`test-results/…/error-context.md`）には、止まったときの画面の中身が書かれているので、それを見ます。

#### エラー3: CSS の束が無い実行ファイルで、サーバーを起動した

`topcoat asset bundle` をせずに、実行ファイルだけを別のフォルダに置いた場合です（`RUSTNOTE_EXE` を、そのフォルダの実行ファイルにしました）。

```text
[WebServer] Error: アセットの束（assets/）がありません。先に `topcoat asset bundle` を実行するか、`topcoat dev` で起動してください
[WebServer] 
[WebServer] Caused by:
[WebServer]     no asset bundle at ...\e2e_noassets\assets[WebServer] 
Error: Process from config.webServer was not able to start. Exit code: 1
```

- **何が起きたか:** RustNote が、起動の途中で「アセットの束（assets/）がありません」と止まり（第9巻 UNIT 10 の診断と同じ文）、Playwright は `Process from config.webServer was not able to start`（サーバーを起動できなかった）と報告しました。テストは1つも動いていません。
- **直し方:** 実行ファイルの隣に `assets/` を作ります（`topcoat asset bundle`）。CI で E2E を動かすときも、「組み立て → 束を作る → テスト」の順番にします（UNIT 10）。

### RustNote ではここで使う

- RustNote には、まだ E2E のテストがありません。この単元の `e2e` フォルダは、練習用に作りました。足すなら、例5-1の1本（大事な流れ）から始め、HTTP のテストで確かめられることは、HTTP のテストに残します。
- **改善点（この単元で見つかったこと）:** 一覧の画面の2つの `nav` に名前が無い（例5-3・問1）。題の入力欄の `maxlength="80"` は、絵文字だと 40 個で止まり、サーバーの決まり（80 文字）と合っていない（例5-2）。`/favicon.ico` が無く、ブラウザが毎回 404 をもらう（解説1。害は無い）。
- RustNote の画面は、入力欄に `<label for>` が付いているので、`getByLabel` で探せました。検索の欄だけは、ラベルが無く、`placeholder`（「キーワードで検索」）だけです。axe はこれを違反にしませんでしたが、`placeholder` は入力を始めると消えるので、ラベルを付けるほうが親切です。

### 練習問題

**問1:** 例5-3の `landmark-unique` を直すために、RustNote のコピーの `src/views.rs` の2つの `nav` に `aria-label` を付け、もう一度 axe で調べてください。

**問2:** 例5-2で、ブラウザでは絵文字が 40 個までしか入りませんでした。入力欄を通らずに、絵文字 80 個の題を、フォームの形で直接送ると、サーバーはどう答えるでしょうか。

**問3:** 一覧の画面で Tab を押していき、フォーカスが移る順番を、Playwright で記録してください。

### 解答と解説

**問1の解答:**

```diff
--- a/src/views.rs
+++ b/src/views.rs
@@ -33,7 +33,7 @@
     Ok(view! {
         <header class="site-header">
             <a class="brand" href=(href!(crate::routes::home))>"RustNote"</a>
-            <nav>
+            <nav aria-label="メニュー">
                 if let Some(user) = user {
                     <a href=(href!(crate::routes::notes::index))>"ノート一覧"</a>
                     <span class="user-email">(user.email.as_str())</span>
@@ -167,7 +167,7 @@
     let next = page.has_next().then(|| link(page.page + 1));
 
     Ok(view! {
-        <nav class="pagination">
+        <nav class="pagination" aria-label="ページ送り">
             if let Some(prev) = prev {
                 <a href=(prev)>"← 前へ"</a>
             }
```

```text
--- /notes: 違反 0 種類・合格 33 種類
--- /notes/1: 違反 0 種類・合格 26 種類
--- /notes/1/edit: 違反 0 種類・合格 30 種類
--- /login: 違反 0 種類・合格 36 種類
  ✓  1 tests\a11y.spec.ts:5:5 › 4つの画面を axe で調べる (2.6s)
```

- 違反は 0 になりました。合格の数が、どの画面でも 5 つずつ増えたのは、`aria-label` が付いたことで、axe が確かめる項目（名前の付け方の項目）が増えたからです。
- 画面の読み上げのランドマークの一覧でも、「メニュー ナビゲーション」「ページ送り ナビゲーション」と、見分けられるようになります。名前は、短く、何の区切りかが分かる言葉にします。

**問2の解答:**

```ts
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
```

```text
状態コード: 303・移動先: /notes/1
詳細の題: 80 文字（UTF-16 で 160）
  ✓  1 tests\q2_direct.spec.ts:4:5 › 絵文字 80 個の題を、直接送る (928ms)
```

- サーバーは `303`（作成）で受け付け、詳細の画面の題は 80 文字（UTF-16 では 160）でした。サーバーの決まりは「80 **文字**」で、ブラウザの `maxlength` は「80 **UTF-16 の単位**」なので、食い違っています。
- `maxlength` は、利用者の打ちまちがいを防ぐための**手助け**で、守りではありません。入力欄を通らない送り方は、いつでもできます。だから、決まりは、必ずサーバーの `parse` で確かめます（第9巻 UNIT 03）。そのうえで、ブラウザの手助けは、サーバーの決まりと**同じ数え方**にそろえるのが理想です（たとえば、`maxlength` をやめて、JavaScript で `[...value].length` を数えて知らせる、など）。

**問3の解答:**

```ts
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
```

```text
Tab 1 回目: a「RustNote」
Tab 2 回目: a「ノート一覧」
Tab 3 回目: button「ログアウト」
Tab 4 回目: input「キーワードで検索」
Tab 5 回目: button「検索」
Tab 6 回目: summary「新しいノートを書く」
Tab 7 回目: a「牛乳を買う」
Tab 8 回目: （ページ全体）
```

- 上の帯（アプリ名 → ノート一覧 → ログアウト）→ 検索の欄 → 検索のボタン → 「新しいノートを書く」→ ノートのリンク、と、画面の上から下へ、HTML に書いた順に移りました。8 回目は、押せる物が終わって、ページ全体（ブラウザのアドレス欄の側）へ戻りました。
- 「新しいノートを書く」（`summary`）にも、Tab で止まれます。閉じた `details` の中の入力欄には、止まりません（開くまで、見えないため）。第10巻 UNIT 10 で、人の手で確かめた順番と同じです。この順番をテストに残しておくと、HTML の並びを変えたときに、気づけます。

### まとめ

- E2E は、Playwright で本物の Chrome を動かし、いちばん大事な流れだけを1本で確かめる。`webServer` で、テストのたびに RustNote を新しい DB で起動し、`/health` が答えてから始める。入力欄は、ラベルやボタンの名前（`getByLabel`・`getByRole`）で探し、`expect` に待たせる。
- 本物の Chrome の `maxlength="80"` は UTF-16 で数え、絵文字は 40 個で止まった。サーバーは 80 文字まで受け付けるので、ブラウザのほうが厳しい。`maxlength` は手助けで、守りはサーバーの `parse` がする。
- axe は、一覧の画面の2つの `nav` に名前が無い（`landmark-unique`）ことを見つけ、`aria-label` で 0 になった。ラベルの一部だけで探すと strict mode で止まり、画面に無いラベルで探すと時間切れになる。CSS の束が無いと、サーバーが起動できない。

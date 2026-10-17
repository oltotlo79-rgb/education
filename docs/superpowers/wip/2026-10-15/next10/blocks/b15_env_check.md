### 1.4.1 環境変数のチェックを自動にする（prebuild スクリプト）

「1.3 エラー3」の補足で見たとおり、この教材のアプリは環境変数が無くても**ビルドは成功してしまい**、本番でページを開いた瞬間に初めてエラーになります。そこで、ビルドの直前に「必要な環境変数がそろっているか」を自動で確かめる小さなスクリプトを用意しましょう。お店の開店前に、レジや釣り銭がそろっているかを確認する「開店前チェック」のようなものです。

> **▼ このコードがやること（先に日本語で）:** 必要な環境変数の名前を一覧にしておき、1つでも値が入っていなければ、分かりやすいメッセージを出して止まります。URL の最後に `/` を付けてしまう、などのよくある書き間違いも見つけます。`package.json` に登録すると、`npm run build` のたびに（Vercel 上のビルドでも）自動で実行されます。

```js
// scripts/check-env.mjs
// ビルドの前に「必要な環境変数がそろっているか」を確かめるスクリプト

// アプリが動くために必要な環境変数の名前
const required = [
  "NEXT_PUBLIC_SUPABASE_URL",
  "NEXT_PUBLIC_SUPABASE_ANON_KEY",
];

// 手元では .env.local を読み込む（Vercel 上にはこのファイルが無い）
try {
  process.loadEnvFile(".env.local");
} catch {
  // ファイルが無ければ何もしない（Vercel では値が最初から入っている）
}

// 値が入っていない名前だけを集める
const missing = required.filter((name) => !process.env[name]);

if (missing.length > 0) {
  console.error("✖ 次の環境変数が設定されていません:");
  for (const name of missing) {
    console.error(`  - ${name}`);
  }
  console.error("→ 手元なら .env.local、Vercel なら Environment Variables を確認してください");
  // 終了コード 1 は「失敗」の合図。npm はここで止まり、ビルドに進まない
  process.exit(1);
}

// URL のよくある書き間違い（末尾の / や http:）もついでに確かめる
const url = process.env.NEXT_PUBLIC_SUPABASE_URL;
if (!url.startsWith("https://") || url.endsWith("/")) {
  console.error(`✖ NEXT_PUBLIC_SUPABASE_URL の形が正しくありません: ${url}`);
  console.error("→ https://xxxx.supabase.co の形（最後に / を付けない）にしてください");
  process.exit(1);
}

console.log(`✔ 必要な環境変数 ${required.length} 個はすべて設定されています`);
```

`package.json` の `"scripts"` に、`"prebuild"` の1行を足します。

```json
{
  "scripts": {
    "dev": "next dev",
    "prebuild": "node scripts/check-env.mjs",
    "build": "next build",
    "start": "next start",
    "lint": "eslint"
  }
}
```

**▼ 実行結果1**（`.env.local` が無い状態で `npm run build`）

```text
> book-management@0.1.0 prebuild
> node scripts/check-env.mjs

✖ 次の環境変数が設定されていません:
  - NEXT_PUBLIC_SUPABASE_URL
  - NEXT_PUBLIC_SUPABASE_ANON_KEY
→ 手元なら .env.local、Vercel なら Environment Variables を確認してください
```

**▼ 実行結果2**（URL の最後に `/` が付いている場合）

```text
> book-management@0.1.0 prebuild
> node scripts/check-env.mjs

✖ NEXT_PUBLIC_SUPABASE_URL の形が正しくありません: https://xyzbookdemo123.supabase.co/
→ https://xxxx.supabase.co の形（最後に / を付けない）にしてください
```

**▼ 実行結果3**（正しく設定されている場合。このあと、いつものビルドが続く）

```text
> book-management@0.1.0 prebuild
> node scripts/check-env.mjs

✔ 必要な環境変数 2 個はすべて設定されています

> book-management@0.1.0 build
> next build

▲ Next.js 16.4.1 (Turbopack)
- Environments: .env.local
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `prebuild` は build の「前」に自動で動く

```json
"prebuild": "node scripts/check-env.mjs",
```

- npm には、`build` という名前のスクリプトを実行するとき、**`prebuild` という名前のスクリプトがあれば先に実行する**という決まりがあります（`pre` は「前の」という意味）。
- `prebuild` が失敗（終了コード 1）すると、`build` は実行されません。Vercel のビルドも `npm run build` で動くので、Vercel 上で環境変数を登録し忘れたときも、デプロイが **Error** で止まり、Build Logs に上のメッセージが出ます。「壊れたまま公開される」よりずっと安全です。

##### 解説2: `.env.local` を読み込む

```js
try {
  process.loadEnvFile(".env.local");
} catch {
  // ファイルが無ければ何もしない
}
```

- `npm run build` の中の Next.js は `.env.local` を自動で読みますが、このスクリプトは Next.js とは別に動く Node.js のプログラムなので、自分で読み込む必要があります。`process.loadEnvFile()` は Node.js（20.12 以降）に入っている「.env ファイルを読み込む」機能です。
- Vercel には `.env.local` が無いので読み込みに失敗しますが、`try { } catch { }` で「失敗しても気にしない」ようにしています。Vercel では、Environment Variables に登録した値が最初から `process.env` に入っています。

##### 解説3: 足りない名前を集めて、止める

```js
const missing = required.filter((name) => !process.env[name]);
```

- `filter` で、「値が入っていない（`!process.env[name]` が true になる）名前」だけを集めています。
- このスクリプトは**ブラウザではなく Node.js で動く**ので、`process.env[name]` のように名前を変数で渡しても正しく読めます（0.3 のつまずきポイントは、ブラウザ用の JS に限った話です）。
- `process.exit(1)` は「失敗としてプログラムを終える」命令です。

### 1.4.2 `.env.local` が Git に入っていないか確かめる

`.env.local` が `.gitignore` で守られているかは、思い込みではなく**コマンドで確かめる**のが確実です。

> **▼ このコードがやること（先に日本語で）:** 「`.env.local` が Git に無視されているか」「どの行のルールで無視されているか」「Git の管理対象に入ってしまっていないか」を、3つのコマンドで確認します。

```powershell
# どのルールで無視されているかを表示する（-v は詳しく表示するオプション）
git check-ignore -v .env.local
# 変更されたファイルの一覧（ここに .env.local が出てはいけない）
git status --short
# Git が管理しているファイルの中に .env.local があるか（何も出なければ OK）
git ls-files .env.local
```

**▼ 実行結果（正しく守られている場合）**

```text
PS> git check-ignore -v .env.local
.gitignore:34:.env*	.env.local
PS> git status --short
PS> git ls-files .env.local
PS>
```

- `.gitignore:34:.env*` は、「`.gitignore` の 34 行目にある `.env*` というルールで無視されている」という意味です。`*` は「何文字でもよい」という記号なので、`.env` で始まるファイルはすべて無視されます（create-next-app が作る `.gitignore` の書き方です）。
- `git status --short` と `git ls-files .env.local` に何も出なければ、`.env.local` は GitHub に送られません。

> **つまずきポイント:** `.gitignore` に書く**前**に一度でもコミットしてしまったファイルは、あとから `.gitignore` に書いても**無視されません**。Git は「すでに管理しているファイル」を追いかけ続けるからです。

**NG: 先にコミットしてしまった `.env.local` の状態**（実験用の小さなリポジトリで、`.env.local` をコミットしたあとに `.gitignore` を作って再現した出力です）

```text
PS> git check-ignore -v .env.local
PS> git ls-files
.env.local
```

`git check-ignore` に何も出ない（＝無視されていない）うえ、`git ls-files` に `.env.local` が出ています。このまま push すると GitHub に送られてしまいます。

**OK: `git rm --cached` で「管理だけ」をやめる**（PCの中のファイルは消えません）

```powershell
git rm --cached .env.local
git status --short
git check-ignore -v .env.local
```

```text
PS> git rm --cached .env.local
rm '.env.local'
PS> git status --short
D  .env.local
?? .gitignore
PS> git check-ignore -v .env.local
.gitignore:1:.env*	.env.local
```

`D` は「Git の管理から外す（Deleted）」という意味です。これをコミットすれば、次からは `.env.local` は送られません。ただし、**すでに GitHub に push してしまった場合は、過去の履歴に値が残っています**。Secret key のような本当に秘密の値だったときは、Supabase の画面でそのキーを無効にして、新しいキーを作り直してください。

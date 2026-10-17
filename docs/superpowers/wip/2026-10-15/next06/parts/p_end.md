@@START: ## まとめ
@@ENDINC: 次の章では、書籍一覧の表示機能を実装します。`BookCard`、`BookList`、`StatusBadge`、`RatingStars`、`LoadingSpinner` の各コンポーネントを作成し、Supabase からデータを取得してトップページに表示します。
## まとめ

この章で実施した内容を振り返ります。

| 項目 | 内容 |
|------|------|
| Next.js プロジェクトの作成 | `create-next-app` の質問で「カスタマイズ」を選び、TypeScript + ESLint + Tailwind CSS + App Router、`src/` なし、Cache Components なしのプロジェクトを作成しました。 |
| Tailwind CSS の基礎 | ユーティリティファーストの概念と、よく使うクラスを学びました。 |
| パッケージのインストール | `@supabase/supabase-js` と `@supabase/ssr` をインストールしました。 |
| ディレクトリ構成の設計 | プロジェクト直下の `app/`（ページ）、`components/`（UI部品）、`lib/`（ユーティリティ）、`types/`（型定義）の4つのディレクトリで整理する構成にしました。 |
| Supabase クライアントの設定 | `.env.local` に URL と Publishable key を設定し、`lib/supabase/server.ts`（サーバー用）と `lib/supabase/client.ts`（ブラウザ用）の2つでクライアントを作れるようにしました。 |
| 型定義の作成 | `Book`, `BookInsert`, `BookUpdate` の3つの型と、ステータス（読書中・読了・読みたい）関連の定数を定義しました。 |
| ルートレイアウトの設定 | 日本語フォント、メタデータ、全ページ共通のレイアウト構造を設定し、`globals.css` を `@import "tailwindcss";` の1行にしました。 |
| ヘッダーコンポーネントの作成 | ナビゲーション付きの共通ヘッダーを作成しました。 |
| 動作確認 | 開発サーバーを起動し、ブラウザで表示を確認しました。 |

**作成したファイル一覧:**

```text
book-management/
├── .env.local            ✅ 作成済み（Git には入らない）
├── app/
│   ├── layout.tsx        ✅ 作成済み
│   ├── page.tsx          ✅ 作成済み（仮の内容）
│   └── globals.css       ✅ 書き換え済み
├── components/
│   └── Header.tsx        ✅ 作成済み（ほかは空のファイル）
├── lib/
│   └── supabase/
│       ├── client.ts     ✅ 作成済み
│       └── server.ts     ✅ 作成済み
└── types/
    └── book.ts           ✅ 作成済み
```

**この章のチェックリスト:**

- [ ] `npm run dev` の表示に `- Environments: .env.local` が出ている
- [ ] `http://localhost:3000`（または表示された URL）で、ヘッダーと「セットアップ完了」のカードが見える
- [ ] `next.config.ts` に `cacheComponents: true` の行が無い
- [ ] `app/`・`components/`・`lib/`・`types/` が `package.json` と同じ場所にある
- [ ] `scripts/typo-demo.ts` を作った場合は削除した

次の章では、書籍一覧の表示機能を実装します。`BookCard`、`BookList`、`StatusBadge`、`RatingStars`、`LoadingSpinner` の各コンポーネントを作成し、Supabase からデータを取得してトップページに表示します。

---

## 練習問題（解答・解説つき）

学んだことを確かめましょう。まず自分で考えてから、下の「解答・解説」を開いて答え合わせをしてください。

### 問題1: create-next-app の答え

`npx create-next-app@latest book-management` を実行したら、最初に「Would you like to use the recommended Next.js defaults?」と聞かれました。本書の手順では何を選び、その後の質問のうち**既定値から変える必要がある**のはどれでしょうか。また、その理由を1つ答えてください。

<details>
<summary>解答・解説</summary>

**解答:** 最初の質問では **「No, customize settings」**（カスタマイズ）を選びます。その後の質問で既定値から変えるのは **⑧ Cache Components を No にする**ことです（⑪ 匿名フィードバックも本書では No にしていますが、こちらはどちらでも構いません）。

**解説:** 推奨設定には Cache Components の有効化が含まれています。Cache Components が有効だと、第7章のトップページのように「サーバーで毎回データを取りに行くページ」が `npm run build` で `Next.js encountered uncached or runtime data during prerendering.` というエラーになります（1.7）。`src/` フォルダの質問⑥は既定値の No のままで、本書の構成（`app/` などをプロジェクト直下に置く）と一致します。
</details>

### 問題2: `@/` の行き先

`tsconfig.json` の `"paths"` が `"@/*": ["./*"]` のとき、`app/books/[id]/edit/page.tsx` の中で次のように書きました。この import は、どのファイルを読み込もうとしますか。また、同じファイルを相対パスで書くとどうなりますか。

```typescript
import Header from "@/components/Header";
```

<details>
<summary>解答・解説</summary>

**解答:** プロジェクト直下の `components/Header.tsx` を読み込みます。相対パスで書くと `import Header from "../../../../components/Header";` です。

**解説:** `@/` は `./`（`tsconfig.json` があるプロジェクト直下）に置き換えられ、`./components/Header` になります。拡張子 `.tsx` は自動で補われます（4.4 の図）。相対パスは「ファイルのあるフォルダ（`edit/`）」から数えるので、`edit/` → `[id]/` → `books/` → `app/` → プロジェクト直下、と4回 `../` でのぼってから `components/Header` に進みます。`@/` を使えば、どの深さのファイルからでも同じ文字列で書けます。
</details>

### 問題3: エラーメッセージを読む

`npm run build` をしたら、次のエラーが出ました。どこをどう直せばよいでしょうか。

```text
./app/layout.tsx:17:1
Error: Module not found: Can't resolve '@/src/components/Header'
```

<details>
<summary>解答・解説</summary>

**解答:** `app/layout.tsx` の17行目の import を `import Header from "@/components/Header";` に直します。

**解説:** `./app/layout.tsx:17:1` は「`app/layout.tsx` の17行目の1文字目」という場所の情報です。本書の構成には `src/` フォルダが無いので、`@/src/components/Header`（＝ `./src/components/Header`）は存在しません（4.4.1）。`src/` を使う構成の記事をまねたときによく起きるミスです。
</details>

### 問題4: どちらの窓口を使う？

次の2つのファイルでは、`lib/supabase/server.ts` と `lib/supabase/client.ts` のどちらの `createClient` を import すべきでしょうか。

- (a) 先頭に `"use client"` が書かれた、登録フォームの部品 `components/BookForm.tsx`
- (b) `"use client"` が無く、`export default async function` で書かれたトップページ `app/page.tsx`

<details>
<summary>解答・解説</summary>

**解答:** (a) は `@/lib/supabase/client`、(b) は `@/lib/supabase/server` です。

**解説:** `"use client"` があるファイルはブラウザで動く Client Component なので、ブラウザ用の `client.ts` を使います。`"use client"` が無いファイルはサーバーで動く Server Component なので、Cookie を扱える `server.ts` を使い、`const supabase = await createClient();` のように `await` を付けて呼びます（5.3 の解説5）。逆にすると、ブラウザでは使えない `cookies()` を呼ぼうとしてエラーになります。
</details>

### 問題5: 環境変数のトラブル

`.env.local` に次のように書いて `npm run dev` したところ、Supabase を使うページで「環境変数 NEXT_PUBLIC_SUPABASE_URL / NEXT_PUBLIC_SUPABASE_ANON_KEY が設定されていません」と表示されました。原因と直し方を答えてください。

```bash
NEXT_PUBLIC_SUPABASE_URL=https://xxxxxxxxxxxxxxxx.supabase.co
NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY=sb_publishable_xxxxxxxxxxxxxxxxxxxxxxxx
```

<details>
<summary>解答・解説</summary>

**解答:** 2行目の変数名が、コードが読む名前（`NEXT_PUBLIC_SUPABASE_ANON_KEY`）と違うのが原因です。`NEXT_PUBLIC_SUPABASE_ANON_KEY=sb_publishable_...` に直して保存し、ターミナルに `Reload env: .env.local` が出るのを確認します（出なければ `Ctrl+C` で止めて `npm run dev` で起動し直します）。

**解説:** `lib/supabase/server.ts` と `client.ts` は `process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY` を読むので、名前が1文字でも違うと値は `undefined` になり、5.3 で書いたチェックがエラーを出します。中身が Publishable key でも、変数名は本書のコードに合わせて `ANON_KEY` にします（5.2.2）。`node --env-file=.env.local scripts/check-env.ts` で事前に確かめる習慣をつけると、起動前に気づけます。
</details>

### 問題6: `NEXT_PUBLIC_` の意味

`.env.local` に `SECRET_TOKEN=abc123`（`NEXT_PUBLIC_` なし）と書きました。この値を、(a) Server Component、(b) Client Component の中で `process.env.SECRET_TOKEN` として読むと、それぞれどうなりますか。

<details>
<summary>解答・解説</summary>

**解答:** (a) Server Component では `"abc123"` が読めます。(b) Client Component（ブラウザで動くとき）では `undefined` になります。

**解説:** Next.js は `.env.local` の値をサーバー側の `process.env` に入れます。ブラウザ用の JavaScript には、`NEXT_PUBLIC_` 付きの変数だけが**文字列として埋め込まれ**ます（5.2 の図）。`NEXT_PUBLIC_` が付いていない値はブラウザに届かないので、秘密にしたい値（Secret key など）は `NEXT_PUBLIC_` を付けずに、サーバーのコードだけで使います。逆に言えば、`NEXT_PUBLIC_` を付けた値は誰でも見られる前提で扱う必要があります。
</details>

### 問題7: Tailwind CSS が効かない

画面の横並び（`flex`）は効いているのに、文字の色や大きさ（`text-gray-600`、`text-3xl`）が全く付きません。`app/globals.css` は次のようになっていました。原因と直し方を答えてください。

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

<details>
<summary>解答・解説</summary>

**解答:** Tailwind CSS v3 の古い書き方になっているのが原因です。`app/globals.css` を `@import "tailwindcss";` の1行に書き換えます。

**解説:** create-next-app が入れる Tailwind CSS v4 では、色や文字サイズの見本帳（テーマ）を `@import "tailwindcss";` で読み込みます。古い3行ではテーマが読み込まれないため、エラーは出ないのに `bg-white` や `text-3xl` などのクラスが作られません（7.2.1 の実験では、できた CSS が 12,332 バイトから 1,236 バイトに減っていました）。
</details>

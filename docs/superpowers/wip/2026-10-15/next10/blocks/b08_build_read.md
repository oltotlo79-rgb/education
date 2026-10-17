#### ▼ 出力の読み方

| 出力の行 | 意味 |
|---|---|
| `▲ Next.js 16.4.1 (Turbopack)` | 使っている Next.js の版と、変換に使う道具（Turbopack：ターボパック） |
| `- Environments: .env.local` | `.env.local` を読み込んだという印。この行が無ければ、環境変数を1つも読めていない |
| `✓ Compiled successfully` | TypeScript や JSX を、JavaScript に変換できた |
| `Running TypeScript ...` | 型チェック。型エラーがあると、ここで止まる（1.3 のエラー1） |
| `Generating static pages` | 先に作っておけるページの HTML を作っている |
| `Route (app)` の表 | ページの一覧。`○` は静的、`ƒ` は動的 |

- `/`（書籍一覧）が `ƒ` なのは、第7章で作ったサーバー用の Supabase クライアント（`lib/supabase/server.ts`）が `cookies()` を使うからです。Cookie はアクセスしてきた人ごとに違うので、Next.js は「このページはアクセスのたびにサーバーで作る」と判断します。だからこそ、本を追加すると一覧にすぐ反映されます。
- `/books/new`（新規登録）が `○` なのは、`"use client"` のフォーム画面で、最初に表示する HTML が誰に対しても同じだからです。データベースへの書き込みは、ボタンを押したあとにブラウザから行われます。
- `/books/[id]` と `/books/[id]/edit` は、URL の `[id]` ごとに中身が変わるので `ƒ` です。
- `/_not-found` は、存在しない URL を開いたときの「404 ページ」で、Next.js が自動で用意してくれます。

> **用語:** 静的（Static）… ビルドのときに HTML を作り置きしておく方式。動的（Dynamic）… アクセスがあるたびにサーバーで HTML を作る方式。

### 1.2.1 本番モードを手元で動かす（npm run start）

`npm run build` が通っても、まだ安心はできません。ビルドしたもの（`.next/` フォルダ）を実際に動かし、ブラウザで全ページを触ってみるのが確実です。`npm run start` を使うと、Vercel にデプロイしたときとほぼ同じ「本番モード」を自分のPCで再現できます。

> **▼ このコードがやること（先に日本語で）:** ビルド結果を使って、本番モードの Web サーバーを手元で起動します。`npm run dev` と違ってファイルを保存しても画面は自動で変わりませんが、その代わり本番と同じ速さ・同じ動きで確認できます。

```powershell
# 1) 本番用にビルドする（まだなら）
npm run build
# 2) ビルド結果を使って、本番モードでサーバーを起動する（ポート番号 3001 を指定）
npm run start -- -p 3001
```

**▼ 実行結果**（`Network` の行の IP アドレスは一部を伏せています）

```text
> book-management@0.1.0 start
> next start -p 3001

▲ Next.js 16.4.1
- Local:         http://localhost:3001
- Network:       http://192.168.x.x:3001
✓ Ready in 175ms
✓ Running next.config.ts took 39ms
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `npm run start` の中身

```powershell
npm run start -- -p 3001
```

- `npm run start` は、`package.json` の `"start": "next start"` を実行します。`next start` は「ビルド済みの `.next/` を使って本番モードのサーバーを起動する」コマンドです。ビルドしていない状態で実行するとエラーになるので、必ず `npm run build` の後に使います。
- `--` は「ここから後ろは npm ではなく `next start` に渡すオプション」という区切りの記号です。`-p 3001` で、使うポート番号を 3001 に変えています。何も付けなければ 3000 番です。
- `✓ Ready` と表示されたら、ブラウザで `http://localhost:3001` を開き、一覧・登録・詳細・編集・削除をひととおり試します。止めるときは、そのターミナルで `Ctrl + C` を押します。

> **用語:** ポート … 1台のPCの中で「どのプログラムに通信を届けるか」を区別する番号。1つの番号は、同時に1つのプログラムしか使えません。

ここで、本番モードを初めて動かすときによくある失敗を1つ見ておきましょう。

> **つまずきポイント:** `npm run dev` を別のターミナルで動かしたまま `npm run start` を実行すると、3000 番がすでに使われているため起動に失敗します。

**NG: 開発サーバーを止めずに、同じ 3000 番で本番モードを起動する**

```powershell
npm run start
```

```text
> book-management@0.1.0 start
> next start

⨯ Failed to start server
Error: listen EADDRINUSE: address already in use :::3000
    at new Promise (<anonymous>) {
  code: 'EADDRINUSE',
  errno: -4091,
  syscall: 'listen',
  address: '::',
  port: 3000
}
```

`EADDRINUSE`（address already in use）は「その住所（ポート）はもう使われています」という意味です。

**OK: 開発サーバーを `Ctrl + C` で止めてから `npm run start` するか、別の番号を指定する**

```powershell
npm run start -- -p 3001
```

### 1.2.2 つまずき: Cache Components を有効にしているとビルドが失敗する

第6章では、`create-next-app` の質問で **Cache Components は No** を選びました。もし Yes を選んだ（または推奨設定のまま作った）場合、`next.config.ts` に `cacheComponents: true` が入り、この教材のトップページ（`/`）が `npm run build` で失敗します。しかも `npm run dev` では画面が普通に表示されるので、**デプロイの直前まで気づかない**のがやっかいな点です。

**NG: `cacheComponents: true` が入った `next.config.ts`**

```ts
// next.config.ts（NG: Cache Components が有効）
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  // 新しいキャッシュの仕組み（Cache Components）を有効にする設定
  cacheComponents: true,
  // Cache Components と組み合わせて使う先読みの設定
  partialPrefetching: true,
  turbopack: {
    rules: {
      "*.css": {
        loaders: ["@tailwindcss/turbopack"],
        as: "*.css",
      },
    },
  },
};

export default nextConfig;
```

**▼ 実行結果（`npm run build`。抜粋）**

```text
▲ Next.js 16.4.1 (Turbopack)
- Environments: .env.local
✓ Running next.config.ts took 38ms
- Cache Components enabled
- Partial Prefetching enabled

  Creating an optimized production build ...
✓ Compiled successfully in 3.7s
  Running TypeScript ...
  Finished TypeScript in 3.2s ...
  Collecting page data using 9 workers ...
  Generating static pages using 9 workers (0/8) ...
  Generating static pages using 9 workers (2/8)
Error: Route "/": Next.js encountered uncached or runtime data during prerendering.

`fetch(...)`, `cookies()`, `headers()`, `params`, `searchParams`, or `connection()` accessed outside of `<Suspense>` prevents the route from being prerendered, blocking the page load and leading to a slower user experience.

Ways to fix this:
  - [stream] Provide a placeholder with `<Suspense fallback={...}>` around the data access
  - [cache] For uncached data (`fetch`, database calls): cache the access with `"use cache"` (does not apply to `connection()`)
  - [block] Set `export const instant = false` to allow a blocking route

Learn more: https://nextjs.org/docs/messages/blocking-prerender-dynamic
    at body (<anonymous>)
    at html (<anonymous>)
（中略）
Error occurred prerendering page "/". Read more: https://nextjs.org/docs/messages/prerender-error
Export encountered an error on /page: /, exiting the build.
⨯ Next.js build worker exited with code: 1 and signal: null
```

- エラー文は「`/` のページで、`cookies()` などのデータを `<Suspense>` の外で使っているので、ページを事前に作れない」という意味です。Cache Components は「どこまでを作り置きし、どこからを毎回作るか」を細かく書き分ける新しい仕組みで、この教材のコードはその書き方に合わせていません。
- `npm run dev` で同じページを開くと、画面は表示されますが、ターミナルに同じ種類のエラー（`Next.js encountered uncached data during prerendering.`）が出ています。開発中もターミナルの赤い文字を見逃さないようにしましょう。

**OK: `cacheComponents` と `partialPrefetching` の2行を消す**（create-next-app で Cache Components に No と答えたときと同じ内容になります）

```ts
// next.config.ts（OK: Cache Components を使わない）
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  turbopack: {
    rules: {
      "*.css": {
        loaders: ["@tailwindcss/turbopack"],
        as: "*.css",
      },
    },
  },
};

export default nextConfig;
```

保存して `npm run build` をやり直すと、1.2 の出力のように最後まで成功します。プロジェクトを作り直せる段階なら、第6章の手順で Cache Components に **No** と答えて作り直しても構いません。

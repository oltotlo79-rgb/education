@@START: ### 5.3 lib/supabase.ts の作成
@@END: ## 6. 型定義ファイルの作成
### 5.3 lib/supabase/client.ts と server.ts の作成

データベースとやり取りするための「窓口（Supabase クライアント）」を作るファイルを2つ用意します。窓口が2つあるのは、Next.js のコードには**サーバーで動くもの**と**ブラウザで動くもの**の2種類があるからです。お店にたとえると、`server.ts` は**店の奥の事務所の電話**、`client.ts` は**店頭のお客さん用の電話**です。どちらも同じ倉庫（Supabase）につながりますが、置き場所と使い方が少し違います。

| ファイル | 使う場所 | 呼び出し方 | 第7章以降の例 |
|----------|---------|-----------|--------------|
| `lib/supabase/server.ts` | Server Component（`"use client"` が無いファイル） | `const supabase = await createClient();` | トップページ `app/page.tsx` で書籍一覧を読む |
| `lib/supabase/client.ts` | Client Component（`"use client"` があるファイル） | `const supabase = createClient();` | 登録フォームや削除ボタンで書き込む |

#### 5.3.1 lib/supabase/client.ts（ブラウザ用）

`lib/supabase/client.ts` ファイルに以下のコードを記述します。

> **▼ このコードがやること（先に日本語で）:** ブラウザで動く部品（Client Component）から使う、Supabase への窓口を作る関数 `createClient` を用意します。さきほど `.env.local` に書いた URL とキーを読み込み、もし設定が抜けていたら、原因がすぐ分かるエラーをわざと出してから止まります。設定がそろっていれば、`@supabase/ssr` の `createBrowserClient` に渡して窓口を作り、返します。

```typescript
// lib/supabase/client.ts
// ブラウザで動く部品（Client Component）から使う Supabase クライアントを作るファイル

// @supabase/ssr から「ブラウザ用のクライアントを作る関数」を名前付きインポートする
import { createBrowserClient } from "@supabase/ssr";

// Client Component から呼び出す関数。呼ぶたびに Supabase への窓口を作って返す
export function createClient() {
  // .env.local に書いた接続先 URL を読み出す
  const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL;
  // .env.local に書いた Publishable key（旧 anon key でも可）を読み出す
  const supabaseKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY;

  // どちらかが空なら、原因と直し方が分かるメッセージでわざと止める
  if (!supabaseUrl || !supabaseKey) {
    throw new Error(
      "環境変数 NEXT_PUBLIC_SUPABASE_URL / NEXT_PUBLIC_SUPABASE_ANON_KEY が設定されていません。" +
        ".env.local ファイルを確認してください。"
    );
  }

  // URL とキーを渡して、ブラウザ用のクライアントを作って返す
  return createBrowserClient(supabaseUrl, supabaseKey);
}
```

#### 5.3.2 lib/supabase/server.ts（サーバー用）

`lib/supabase/server.ts` ファイルに以下のコードを記述します。

> **▼ このコードがやること（先に日本語で）:** サーバーで動く部品（Server Component）から使う窓口を作る関数 `createClient` を用意します。前半の「環境変数を読んで、無ければ止める」部分は `client.ts` と同じです。違うのは後半で、サーバーでは「いま届いたリクエストの Cookie（クッキー：ブラウザが持っている小さなメモ）」を Supabase に読み書きさせる方法も一緒に渡します。Cookie を受け取るのに時間がかかる場合があるので、関数に `async` を付けています。

```typescript
// lib/supabase/server.ts
// サーバーで動く部品（Server Component）から使う Supabase クライアントを作るファイル

// @supabase/ssr から「サーバー用のクライアントを作る関数」を名前付きインポートする
import { createServerClient } from "@supabase/ssr";
// Next.js が用意している「いま届いたリクエストの Cookie を読み書きする関数」
import { cookies } from "next/headers";

// Server Component から呼び出す関数。cookies() を待つので async にする
export async function createClient() {
  // .env.local に書いた接続先 URL を読み出す
  const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL;
  // .env.local に書いた Publishable key（旧 anon key でも可）を読み出す
  const supabaseKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY;

  // どちらかが空なら、原因と直し方が分かるメッセージでわざと止める
  if (!supabaseUrl || !supabaseKey) {
    throw new Error(
      "環境変数 NEXT_PUBLIC_SUPABASE_URL / NEXT_PUBLIC_SUPABASE_ANON_KEY が設定されていません。" +
        ".env.local ファイルを確認してください。"
    );
  }

  // いま処理しているリクエストの Cookie 置き場を受け取る
  const cookieStore = await cookies();

  // URL・キー・Cookie の読み書き方法を渡して、サーバー用のクライアントを作って返す
  return createServerClient(supabaseUrl, supabaseKey, {
    cookies: {
      // Supabase が Cookie を読みたいときに呼ばれる
      getAll() {
        return cookieStore.getAll();
      },
      // Supabase が Cookie を書きたいときに呼ばれる
      setAll(cookiesToSet) {
        try {
          // 受け取った Cookie を1つずつ書き込む
          cookiesToSet.forEach(({ name, value, options }) =>
            cookieStore.set(name, value, options)
          );
        } catch {
          // Server Component の表示中は Cookie を書けないので、ここに来ても無視してよい
          // （このアプリはログイン機能を使わないため、実害はない）
        }
      },
    },
  });
}
```

この2つのファイルは、Next.js 16.4.1・`@supabase/ssr` 0.12.7 で `npm run build`（型チェックを含む本番用ビルド）が通ることを確認しています。

**コードの解説:**

| 部分 | 解説 |
|----|------|
| `import { createBrowserClient } from "@supabase/ssr";` | Supabase の Next.js 向けライブラリから、ブラウザ用の窓口を作る関数を取り出します。中括弧 `{}` で囲むのが「named import（名前付きインポート）」で、ライブラリが公開している特定の関数だけを取り出します。 |
| `import { createServerClient } from "@supabase/ssr";` | 同じライブラリから、サーバー用の窓口を作る関数を取り出します。 |
| `import { cookies } from "next/headers";` | Next.js の、リクエストの Cookie を扱う関数です。サーバーでしか使えません（ブラウザ用の `client.ts` では使いません）。 |
| `process.env.NEXT_PUBLIC_SUPABASE_URL` | `.env.local` に定義した環境変数を読み取ります。`process.env` は環境変数が入っている入れ物です。 |
| `if (!supabaseUrl \|\| !supabaseKey)` | どちらかが未設定（`undefined` や空文字）の場合にエラーを投げます。`.env.local` の作成忘れや変数名の打ち間違いを早く見つけられます。 |
| `export function createClient()` / `export async function createClient()` | 窓口を作る関数を外に公開します。2つのファイルで**同じ名前**にしているので、使う側は import 元（`client` か `server` か）を変えるだけで済みます。 |
| `createServerClient(url, key, { cookies: { getAll, setAll } })` | サーバー用の窓口を作ります。3つ目の引数で「Cookie の読み方（`getAll`）と書き方（`setAll`）」を教えています。 |

> **import 文の種類（重要）:**
> JavaScript/TypeScript には3種類のインポート方法があります。
> - **default import**: `import React from "react";` のように中括弧なし。各ファイルにつき1つだけ `export default` できる主要なものを取り出す。
> - **named import**: `import { useState } from "react";` のように中括弧あり。複数の名前を指定して取り出せる（例: `import { useState, useEffect } from "react";`）。
> - **type import**: `import type { Book } from "@/types/book";` のように `type` キーワード付き。型のみをインポートし、ビルド時に取り除かれる（JavaScriptには型がないため）。

> **なぜ「1つの supabase を共有」ではなく「呼ぶたびに作る関数」なのか:**
> 古い記事では、`export const supabase = createClient(...)` のように**1つだけ作って全ファイルで使い回す**書き方（シングルトンパターン）がよく出てきます。ブラウザだけで動くアプリならそれでも動きますが、Next.js のサーバーは**たくさんの人のリクエストを同時に**処理します。サーバー用の窓口は「いま処理しているリクエストの Cookie」を使うので、1つを全員で共有すると、Aさんの Cookie で Bさんの処理をしてしまう恐れがあります。そこで、**使う場所で `createClient()` を呼んで、その場で窓口を作る**書き方にしています。

> **Server Component と Client Component で Supabase を使うときの違い（重要）:**
>
> Next.js の App Router では、コンポーネントが「Server Component」（サーバー側で実行）か「Client Component」（ブラウザ側で実行）かを区別します。それぞれで Supabase を使う際の注意点が異なります。
>
> | 観点 | Server Component | Client Component |
> |------|------------------|------------------|
> | ファイル先頭の宣言 | （宣言なし。デフォルト） | `"use client"` を書く |
> | 実行場所 | サーバー（あなたのPC or 本番サーバー） | ユーザーのブラウザ |
> | 使う窓口 | `@/lib/supabase/server` の `createClient`（`await` が必要） | `@/lib/supabase/client` の `createClient` |
> | 使える React フック | 使えない（useState など不可） | 使える |
> | データ取得 | `await supabase.from(...)` を直接書ける | ボタンを押したときの処理（イベント）などの中で呼ぶ |
> | 環境変数 | サーバー専用変数（`NEXT_PUBLIC_` なし）も読める | `NEXT_PUBLIC_` 付きの変数のみ |
> | このチュートリアル | 一覧ページ・詳細ページなど、データを**読んで表示**する画面 | 登録フォームや削除ボタンなど、データを**書き込む**操作 |
>
> 第7章では、トップページ（`app/page.tsx`）を Server Component にして `server.ts` の窓口で書籍一覧を読み、登録フォーム（`app/books/new/page.tsx`）を Client Component にして `client.ts` の窓口で書き込みます。

#### ▼ コードを1つずつ分解して解説

上のコードを、初心者がつまずきやすい部分ごとに**1つずつ**ていねいに見ていきましょう。

---

##### 解説1: 窓口を作る関数を「名前付きインポート」する

```typescript
import { createBrowserClient } from "@supabase/ssr";
```

- `import ... from "..."` は、別のファイルやライブラリから機能を**取り込む**ための構文です。
- `{ createBrowserClient }` のように波括弧 `{ }` で囲むのが「名前付きインポート」で、そのライブラリが公開している**特定の名前の機能だけ**をピンポイントで取り出します。
- ここでは `@supabase/ssr`（節3でインストールした Supabase の Next.js 向けライブラリ）から、`createBrowserClient`（ブラウザ用の窓口を作る関数）だけを取り出しています。`server.ts` では同じように `createServerClient` を取り出します。

> **用語:** 「ライブラリ」とは、誰かが作って公開してくれた便利な機能のかたまりのことです。`npm install` で自分のプロジェクトに取り込み、`import` で必要な部分だけ呼び出して使います。

---

##### 解説2: 環境変数から接続情報を読み取る

```typescript
// .env.local に書いた接続先 URL を読み出す
const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL;
// .env.local に書いた Publishable key（旧 anon key でも可）を読み出す
const supabaseKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY;
```

- `process.env`（プロセス・エンブ）は、**環境変数の入れ物**です。さきほど `.env.local` に書いた値が、ここに入ってきます。
- `process.env.NEXT_PUBLIC_SUPABASE_URL` と書くと、`.env.local` の `NEXT_PUBLIC_SUPABASE_URL=...` の値（URL）を取り出せます。
- `process.env.〇〇` は**省略せずにこの形のまま**書くのが大切です。ブラウザ用のファイルでは、Next.js がこの形の部分を探して値に書き換えるので（図の②）、`const env = process.env;` のように一度まとめてから取り出すと、書き換えが働かず `undefined` になってしまいます。
- URL とキーをコードに直接書かず、外部の設定ファイル経由で渡すことで、**秘密情報の漏洩を防ぎ**、環境ごとに値を切り替えられます。

> **用語:** 「環境変数」とは、コードの外側からプログラムへ渡す設定値のことです。API キーのような秘密情報や、開発用と本番用で変えたい値を、コード本体と分けて管理するために使います。

---

##### 解説3: 設定が抜けていたらわざとエラーを出す

```typescript
if (!supabaseUrl || !supabaseKey) {
  throw new Error(
    "環境変数 NEXT_PUBLIC_SUPABASE_URL / NEXT_PUBLIC_SUPABASE_ANON_KEY が設定されていません。" +
      ".env.local ファイルを確認してください。"
  );
}
```

- `!supabaseUrl` の `!`（エクスクラメーション）は「否定」の記号で、「`supabaseUrl` が空っぽ（`undefined` や空文字）なら `true`」という意味になります。`||` は「または」です。
- つまりこの `if` は「URL かキーのどちらかがちゃんと取れなかったとき」だけ中に入ります。
- `throw new Error(...)` は、エラーを**わざと発生させて処理を止める**書き方です。原因（環境変数が未設定）と対処法（`.env.local` を確認）をメッセージに書いておくと、5.2.3 で見たように、エラー画面を見ただけで原因が分かります。
- `+` は文字列をつなげる演算子で、長いメッセージを2行に分けて書いています。
- このチェックを通り抜けた後は、TypeScript も「`supabaseUrl` と `supabaseKey` は空ではない文字列だ」と分かるので、次の行で安心して渡せます。

> **用語:** 「例外（throw / Error）」とは、処理を続けられない問題が起きたときに発生させる中断の合図です。早い段階でわかりやすく失敗させることを「フェイルファスト（fail fast）」と呼び、原因究明を楽にします。

---

##### 解説4: サーバー用は Cookie の読み書き方法も渡す

```typescript
const cookieStore = await cookies();

return createServerClient(supabaseUrl, supabaseKey, {
  cookies: {
    getAll() {
      return cookieStore.getAll();
    },
    setAll(cookiesToSet) {
      try {
        cookiesToSet.forEach(({ name, value, options }) =>
          cookieStore.set(name, value, options)
        );
      } catch {
        // Server Component の表示中は Cookie を書けないので、ここに来ても無視してよい
      }
    },
  },
});
```

- `await cookies()` で、いま処理しているリクエストの Cookie 置き場を受け取ります。`await` を使うので、関数全体に `async` が付いています（だから使う側も `await createClient()` と書きます）。
- `getAll()` は「Cookie を全部見せて」、`setAll()` は「この Cookie を書いておいて」と Supabase から頼まれたときの答え方です。
- `try { ... } catch { ... }` は「やってみて、失敗したら何もしない」という書き方です。Server Component の表示中は Cookie を書き込めない決まりがあり、そのときに出る失敗を無視しています。ログイン機能を使わない本書のアプリでは、ここで困ることはありません。
- ブラウザ用の `createBrowserClient` は、ブラウザが自分で Cookie を扱えるので、この3つ目の引数がいりません。

> **用語:** 「Cookie（クッキー）」とは、Webサイトがブラウザに預けておく小さなメモです。ログイン状態を覚えておく、などに使われます。

---

##### 解説5: 2つのファイルで同じ名前 `createClient` にしている理由

```typescript
// サーバーで動くページでの使い方（第7章のトップページなど）
import { createClient } from "@/lib/supabase/server";

// ブラウザで動く部品での使い方（第7章の登録フォームなど）
import { createClient } from "@/lib/supabase/client";
```

- どちらも関数名は `createClient` です。違うのは **import 元のパス**（`server` か `client` か）だけです。
- 使う側は「このファイルはサーバー？ブラウザ？」を考えて、import 元を選ぶだけで済みます。
- 逆を選ぶとエラーになります。たとえば `"use client"` の部品で `server` の方を import すると、ブラウザでは使えない `cookies()` を呼ぼうとして失敗します。迷ったら、ファイルの先頭に `"use client"` があるかどうかで判断してください。

> **用語:** 「export」は、ファイルの中で作ったものを**外部に公開する**指定です。逆に他ファイルでそれを受け取るのが「import」です。この2つでファイル間の機能の受け渡しを行います。

---


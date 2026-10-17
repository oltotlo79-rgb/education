#### 確認問題の解答と解説

まず自分の言葉で答えてから読んでください。

**問1の解答（Server Component と Client Component の違い）**

- Server Component はサーバーで実行され、Client Component はブラウザでも実行される（最初の1回はサーバーでも描画される。図4-4 の③）
- Server Component はデータベースや秘密のキーに直接アクセスできる。Client Component はできない（書いてもブラウザでは使えず、秘密は漏れる危険がある）
- Client Component は `useState`・`useEffect`・`onClick` などが使える。Server Component は使えない

**解説:** 3つとも「どこで動くか」から説明できます。サーバーで動くから DB に近く秘密を守れる、ブラウザで動くからクリックや入力に反応できる、という関係です。「Client Component はブラウザ**だけ**で動く」と答えた人は、3.1.1 の実験（ターミナルに「実行場所: サーバー」と出た）をもう一度見てください。

**問2の解答（Server / Client の判断）**

- データベースから書籍一覧を取得して表示 → **Server Component**（DB に直接アクセスでき、データ入りの HTML がすぐ届く）
- 検索バー（入力をリアルタイムに反映）→ **Client Component**（`useState` と `onChange` が必要）
- 書籍カードの静的な表示 → **Server Component**（動きが無いので、ブラウザに JavaScript を送る必要が無い）

**解説:** 判断の基準は「その部品に、利用者の操作に反応する動きがあるか」です。動きが無いものは Server Component のまま、動きがある部品だけを小さく切り出して `"use client"` を付けます（3.3 の「つまずきポイント」）。

**問3の解答（環境変数のアクセス範囲）**

- `NEXT_PUBLIC_API_KEY` → Server Component と Client Component の両方で使える。ただし**ビルドのときにブラウザ用の JavaScript に値が書き込まれ、誰でも見られる**
- `API_SECRET_KEY` → サーバー（Server Component・Server Action・Route Handler）だけで使う。Client Component ではブラウザで `undefined` になる

**解説:** 名前に `API_KEY` と付いていても、`NEXT_PUBLIC_` を付けた時点で「公開してよい値」として扱われます（図4-11）。秘密のキーに `NEXT_PUBLIC_` を付けてはいけません。また、`API_SECRET_KEY` を Client Component の中で画面に表示すると、サーバーでの最初の描画で本物の値が HTML に入ってしまう点にも注意しましょう（8.2 の「つまずきポイント」）。

**問4の解答（layout.tsx と page.tsx の違い）**

- `layout.tsx` は、そのフォルダ以下の複数のページで共有される外枠（ヘッダー、サイドバーなど）。`{children}` の位置にページが入る。ページを移動しても作り直されず、状態が保たれる
- `page.tsx` は、その URL 固有の中身。ページを移動すると入れ替わる

**解説:** 図4-6 の「箱の中に箱」を思い出してください。5.1 の実験では、レイアウトの中のカウンターは `<Link>` で移動しても 3 のまま残りました（`<a>` だと 0 に戻る）。

**問5の解答（Server Actions の利点）**

- API（`route.ts`）を別に作らなくても、フォームから直接サーバーの関数を呼べるので、コードが短くなる
- 関数の中身はサーバーで実行されるので、DB の接続情報などの秘密がブラウザに送られない
- `revalidatePath` と組み合わせると、保存と画面の更新が1回の通信で済む（図4-10）
- `<form action>` に渡せば、JavaScript が動かない環境でも送信できる

**解説:** 2つ挙げられれば十分です。ただし、Server Action は「誰でも POST で呼び出せる公開の入り口」でもあるので、本物のアプリでは関数の中で必ずログインの確認や入力チェックをします（Next.js の公式ドキュメントでも強く注意されています）。

## 練習問題（解答・解説つき）

本文の内容を、手を動かして確かめる問題です。まず自分で考えて（できれば紙やエディタに書いて）から、後ろの「解答と解説」を見てください。

### 問題1: ファイルをどこに置く？（2節）

次の画面を表示するには、`app/` フォルダの下にどんな名前のファイルを作ればよいでしょうか。フォルダも含めて答えてください。

1. `/about` のページ
2. `/books/42/edit`（42 は書籍ごとに変わる）の編集ページ
3. `/docs/a`・`/docs/a/b`・`/docs/a/b/c` のどれでも表示される1つのページ
4. `/books` 以下のページでデータを取得している間に出す「読み込み中」の表示

### 問題2: 古いサンプルを今の書き方に直す（2.4節）

インターネットで見つけた次のコードを Next.js 16 で使うと、どうなるでしょうか。また、正しく動くように直してください。

```tsx
// app/books/[id]/page.tsx（古い書き方）
export default function BookDetailPage({
  params,
}: {
  params: { id: string };
}) {
  return <h1>書籍 {params.id} の詳細</h1>;
}
```

### 問題3: ボタンを1つ足したらエラーになった（3節）

次の `app/books/page.tsx` に「いいね」ボタンを足したところ、ページを開くと 500 エラーになりました。データ取得はサーバーで行ったまま、ボタンが動くように直してください。

```tsx
// app/books/page.tsx
export default async function BooksPage() {
  const books = ["リーダブルコード", "達人プログラマー"];
  return (
    <div>
      <h1>書籍一覧（{books.length}冊）</h1>
      <button onClick={() => console.log("いいね")}>いいね</button>
    </div>
  );
}
```

### 問題4: ログはどこに出る？（3.1.1節）

次の Client Component を Server Component のページに置き、`npm run dev` でページを1回開いてから、ボタンを1回押しました。(a)〜(c) のログは、それぞれ「サーバーのターミナル」「ブラウザの Console」のどちらに出るでしょうか（両方の場合もあります）。

```tsx
"use client";

import { useEffect } from "react";

export function Hello() {
  // (a) 描画のたびに出すログ
  console.log("描画しました");
  useEffect(() => {
    // (b) 画面に出た後に出すログ
    console.log("画面に出ました");
  }, []);
  // (c) ボタンを押したときに出すログ
  return <button onClick={() => console.log("押されました")}>押す</button>;
}
```

### 問題5: Link か router.push か（5節）

次の場面では、`<Link>` と `router.push` のどちらを使うのがよいでしょうか。理由も答えてください。

1. 画面上部のメニューにある「書籍一覧」「書籍を追加」
2. 登録フォームで「保存」を押し、保存に成功したら、その本の詳細ページへ移動する
3. 一覧に並んだ書籍カードを押すと、その本の詳細ページへ移動する

### 問題6: 本番だけ一覧が増えない（6.1・7.2節）

次の Server Action で本を追加すると、`npm run dev` では一覧（`/`）を再読み込みすれば増えて見えるのに、`npm run build` → `npm run start` の本番モードでは、何度再読み込みしても増えません。原因と直し方を答えてください（`saveBook` は、データベースに保存する処理のまねをする自作の関数とします）。

```ts
// app/actions.ts
"use server";

import { saveBook } from "@/lib/books";

export async function addBook(formData: FormData) {
  const title = String(formData.get("title") ?? "").trim();
  if (!title) {
    return;
  }
  await saveBook(title);
}
```

### 問題7: 環境変数の仕分け（8節）

`.env.local` に次の4つを書きました。

```bash
NEXT_PUBLIC_SUPABASE_URL="https://abcdefghijklm.supabase.co"
NEXT_PUBLIC_SUPABASE_ANON_KEY="sb_publishable_xxxxxxxxxxxxxxxx"
SUPABASE_SERVICE_ROLE_KEY="sb_secret_xxxxxxxxxxxxxxxx"
NEXT_PUBLIC_ADMIN_PASSWORD="p@ssw0rd"
```

1. ビルド後、ブラウザ用の JavaScript に値が書き込まれるのはどれですか。
2. この中で、設定のしかたが危ないものはどれですか。どう直しますか。
3. ある Client Component に `<p>{process.env.SUPABASE_SERVICE_ROLE_KEY}</p>` と書いてしまいました。ブラウザの画面には何が出て、何が問題ですか。

### 解答と解説

#### 問題1の解答

1. `app/about/page.tsx`
2. `app/books/[id]/edit/page.tsx`
3. `app/docs/[...slug]/page.tsx`
4. `app/books/loading.tsx`

**解説:** URL の区切り（`/`）ごとにフォルダを作り、最後に `page.tsx` を置くのが基本です（図4-3）。変化する部分は `[id]` のように角括弧のフォルダにし、何階層でも受け取りたいときは `[...slug]` にします。`loading.tsx` は、置いたフォルダとその下のページの「読み込み中」表示になるので、`/books` 以下すべてに効かせたいなら `app/books/` に置きます（図4-2）。

#### 問題2の解答

**どうなるか:** 型チェックも `next build` も通ってしまいますが、画面には id が空のまま「書籍  の詳細」と表示され、開発サーバーのターミナルに `` Route "/books/[id]" used `params.id`. `params` is a Promise and must be unwrapped with `await` ... `` というエラーが出ます（2.4 の「つまずきポイント」の実験結果）。

```tsx
// app/books/[id]/page.tsx（直したもの）
export default async function BookDetailPage({
  params,
}: {
  params: Promise<{ id: string }>;
}) {
  const { id } = await params;
  return <h1>書籍 {id} の詳細</h1>;
}
```

**解説:** 直すところは3つです。①関数に `async` を付ける、②型を `Promise<{ id: string }>` にする、③`await params` で封を開けてから `id` を取り出す。`searchParams` も同じように直します。

#### 問題3の解答

**原因:** `"use client"` の無い `page.tsx` は Server Component です。Server Component の中で `onClick` に関数を渡しているので、ターミナルに `Event handlers cannot be passed to Client Component props.` というエラーが出ます（`<button onClick={function onClick} children=...>` の部分が指し示されます。実際に試して確認したエラー文です）。

```tsx
// app/books/LikeButton.tsx（動きが必要なボタンだけを Client Component にする）
"use client";

export function LikeButton() {
  return <button onClick={() => console.log("いいね")}>いいね</button>;
}
```

```tsx
// app/books/page.tsx（ページは Server Component のまま）
import { LikeButton } from "./LikeButton";

export default async function BooksPage() {
  const books = ["リーダブルコード", "達人プログラマー"];
  return (
    <div>
      <h1>書籍一覧（{books.length}冊）</h1>
      <LikeButton />
    </div>
  );
}
```

**解説:** `page.tsx` の先頭に `"use client"` を付けても動きますが、ページ全体が Client Component になり、`async` でのデータ取得や `metadata` が使えなくなります（3.3 の「つまずきポイント」）。動きが必要な部品だけを切り出すのが正解です。

#### 問題4の解答

- (a) `描画しました` … **両方**。最初の HTML を作るためにサーバーで1回（ターミナル）、ハイドレーションのためにブラウザで（Console。開発中は Strict Mode で2回）実行されます。
- (b) `画面に出ました` … **ブラウザの Console だけ**。`useEffect` はブラウザで画面に表示された後にだけ実行されます。
- (c) `押されました` … **ブラウザの Console だけ**。クリックはブラウザで起きるからです。

**解説:** 3.1.1 の実験結果と同じです。「Client Component の関数本体はサーバーでも動く」「`useEffect` とイベントの中はブラウザだけ」の2点が分かっていれば、`window is not defined` のエラー（3.2）も防げます。

#### 問題5の解答

1. **`<Link>`** … いつでも行ける場所への道案内なので。本物の `<a href>` になり、新しいタブで開いたり、検索エンジンがたどったりできる
2. **`router.push`** … 「保存に成功したら」という**処理の結果のあとで**移動したいので（Server Action で保存しているなら、Server Action の中で `redirect()` を使う方法もある）
3. **`<Link>`** … 1 と同じく、利用者が選んで進む道案内なので

**解説:** 迷ったら `<Link>`、「何かが終わってから移動」のときだけ `router.push`（図4-7）。一覧のカードをボタンにして `router.push` で移動させると、右クリックで新しいタブを開けない不便なリンクになってしまいます。

#### 問題6の解答

**原因:** 保存したあとに `revalidatePath` を呼んでいないので、Next.js は一覧（`/`）の作り置きが古くなったことを知りません。本番モードでは、`/` がビルド時に作り置き（`○ Static`）されているので、何度読み込み直しても作り置きが返ります。開発モードではページを毎回作り直すので、気づきにくいのです（7.2 の「つまずきポイント」の実験と同じ）。

```ts
// app/actions.ts（直したもの）
"use server";

import { revalidatePath } from "next/cache";
import { saveBook } from "@/lib/books";

export async function addBook(formData: FormData) {
  const title = String(formData.get("title") ?? "").trim();
  if (!title) {
    return;
  }
  await saveBook(title);
  // 一覧の画面が古くなったことを Next.js に伝える
  revalidatePath("/");
}
```

**解説:** 「データを変えたら、そのデータを表示している画面の URL を `revalidatePath` に渡す」がセットです（図4-10）。詳細ページ `/books/[id]` にも表示しているなら、そのパスも作り直します。

#### 問題7の解答

1. `NEXT_PUBLIC_` が付いた3つ（`NEXT_PUBLIC_SUPABASE_URL`・`NEXT_PUBLIC_SUPABASE_ANON_KEY`・`NEXT_PUBLIC_ADMIN_PASSWORD`）。
2. `NEXT_PUBLIC_ADMIN_PASSWORD` が危険です。管理者のパスワードがブラウザ用の JavaScript に書き込まれ、誰でも読めてしまいます。`NEXT_PUBLIC_` を外して `ADMIN_PASSWORD` にし、サーバー（Server Action など）の中だけで使います。すでに公開してしまった値は、パスワード自体を変更します。
3. ブラウザの画面には（ハイドレーションのあと）何も表示されません（ブラウザでは `undefined`）。しかし、サーバーでの最初の描画では本物の Secret key が読めるので、**サーバーから届く HTML に Secret key がそのまま入ってしまいます**。さらに Hydration エラーも出ます。Client Component では秘密の変数を読まず、必要な処理はサーバー側で行います。

**解説:** `NEXT_PUBLIC_` は「チラシに印刷してよい」という付箋でした（図4-11）。Publishable key（旧 anon key）は Row Level Security で守られている前提なので公開してかまいませんが、Secret key（旧 service_role key）やパスワードは絶対に付箋を付けてはいけません。3 は 8.2 の「つまずきポイント」で実際に確かめた落とし穴です。

### 2-4. 発展: Server Action に id を渡す2つの方法（hidden input と bind）

2-3 の囲み記事で紹介した **Server Action**（サーバーで動く関数をフォームに直接つなぐ仕組み）で編集を書く場合、ひとつ悩むことがあります。`<form action={関数}>` に渡した関数は、送信時に **フォームの中身（FormData）だけ** を受け取って呼ばれます。では「どの本を更新するのか」という **id** は、どうやってサーバー側の関数に届ければよいのでしょうか。方法は2つあります。

> **たとえ話:** 修理に出す荷物に「どの家の荷物か」を伝える方法です。**hidden input** は、箱の中に住所を書いたメモを一緒に入れて送るやり方（中身を開ければ誰でも読めます）。**bind** は、あらかじめ住所が印刷された専用の伝票（関数）を作っておき、その伝票で送るやり方です。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 370" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-08" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="10" width="340" height="34" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="180" y="33" text-anchor="middle" font-size="14" font-weight="700" fill="#1e40af">方法A: bind で id を固定する</text>
<rect x="370" y="10" width="340" height="34" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="540" y="33" text-anchor="middle" font-size="14" font-weight="700" fill="#c2410c">方法B: hidden input で送る</text>
<rect x="10" y="54" width="340" height="96" rx="8" fill="#1e293b"/>
<g font-size="12" font-family="Consolas, monospace" fill="#e2e8f0">
<text x="24" y="76">const action =</text>
<text x="24" y="96">  updateBook.bind(null, id);</text>
<text x="24" y="116">&lt;form action={action}&gt;</text>
<text x="24" y="136">  &lt;input name="title" /&gt;</text>
</g>
<rect x="370" y="54" width="340" height="96" rx="8" fill="#1e293b"/>
<g font-size="12" font-family="Consolas, monospace" fill="#e2e8f0">
<text x="384" y="76">&lt;form action={updateBook}&gt;</text>
<text x="384" y="96">  &lt;input type="hidden"</text>
<text x="384" y="116">    name="id" value={id} /&gt;</text>
<text x="384" y="136">  &lt;input name="title" /&gt;</text>
</g>
<line x1="180" y1="150" x2="180" y2="194" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-08)"/>
<text x="190" y="178" font-size="12" fill="#475569">送信</text>
<line x1="540" y1="150" x2="540" y2="194" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-08)"/>
<text x="550" y="178" font-size="12" fill="#475569">送信</text>
<rect x="10" y="198" width="340" height="72" rx="8" fill="#f8fafc" stroke="#94a3b8"/>
<text x="24" y="218" font-size="12" fill="#475569">サーバー側の関数（actions.ts）</text>
<text x="24" y="240" font-size="12" fill="#1e293b" font-family="Consolas, monospace">updateBook(id, formData)</text>
<text x="24" y="260" font-size="12" fill="#1e40af">id は1番目の引数として届く</text>
<rect x="370" y="198" width="340" height="72" rx="8" fill="#f8fafc" stroke="#94a3b8"/>
<text x="384" y="218" font-size="12" fill="#475569">サーバー側の関数（actions.ts）</text>
<text x="384" y="240" font-size="12" fill="#1e293b" font-family="Consolas, monospace">formData.get("id")</text>
<text x="384" y="260" font-size="12" fill="#c2410c">string か File か null で届く</text>
<text x="24" y="294" font-size="12" fill="#047857">○ HTML の中に id が書き出されない</text>
<text x="24" y="314" font-size="12" fill="#047857">○ 型で「id は string」と決まっている</text>
<text x="24" y="334" font-size="12" fill="#047857">○ フォームの部品を増やさずに済む</text>
<text x="384" y="294" font-size="12" fill="#c2410c">△ HTML の中に id がそのまま見える</text>
<text x="384" y="314" font-size="12" fill="#c2410c">△ 文字列かどうかの確認が必要</text>
<text x="384" y="334" font-size="12" fill="#047857">○ 仕組みが単純で分かりやすい</text>
<text x="360" y="362" text-anchor="middle" font-size="12" fill="#475569">どちらでも id はブラウザ側で書き換えられる前提で、権限の確認はサーバー（RLS など）で行う</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-9: Server Action に id を渡す2つの方法の比較</div>
</div>

#### 動かして確かめる: bind と hidden input を FormData で真似する

> **▼ このコードがやること（先に日本語で）:** Node.js に最初から入っている `FormData`（フォームの中身を入れる入れ物）を使って、ブラウザがフォームを送信したときの「Server Action の呼ばれ方」を真似します。方法A（bind）、方法B（hidden input）、hidden input を書き忘れた場合、そして `bind` の代わりに `()` を付けて呼んでしまった場合の4つを順番に試します。

**ファイル: `bind-hidden.ts`（練習用）**

```ts
// Server Action に「どの本か（id）」を渡す 2 つの方法を、
// Node.js の FormData を使って真似する練習用コード

const bookId = "3f2a8c1e-5b7d-4e9a-9c01-2b3d4e5f6a7b";

// 方法A 用: 1番目の引数で id、2番目でフォームの中身を受け取る
async function updateBook(id: string, formData: FormData) {
  const title = formData.get("title");
  console.log(`  → updateBook 実行: id=${id} / title=${title}`);
}

// 方法B 用: フォームの中身（hidden input を含む）だけを受け取る
async function updateBookFromHidden(formData: FormData) {
  const id = formData.get("id");
  const title = formData.get("title");
  // get() の結果は string / File / null のどれかなので確認する
  if (typeof id !== "string" || id === "") {
    console.log("  → id が届いていないので更新しない");
    return;
  }
  console.log(`  → updateBookFromHidden 実行: id=${id} / title=${title}`);
}

// ブラウザが送ってくるフォームの中身を作る（タイトル欄だけ）
const formData = new FormData();
formData.append("title", "React実践ガイド 第2版");

// id だけを受け取る削除用の関数（NG と OK の比較に使う）
async function deleteBook(id: string) {
  console.log(`  → deleteBook 実行: id=${id}`);
}

async function main() {
  console.log("【方法A】bind で id を先に固定する");
  const updateBookWithId = updateBook.bind(null, bookId);
  // 送信されたときに、フォームの中身だけを渡して呼ばれる
  await updateBookWithId(formData);

  console.log("【方法B】hidden input で id も一緒に送る");
  const formWithHidden = new FormData();
  formWithHidden.append("id", bookId);
  formWithHidden.append("title", "React実践ガイド 第2版");
  await updateBookFromHidden(formWithHidden);

  console.log("【方法B の失敗例】hidden input を書き忘れた");
  await updateBookFromHidden(formData);

  console.log("【NG と OK】() を付けるとその場で実行される");
  // OK: bind は「あとで呼ぶための関数」を作るだけ（まだ実行しない）
  const deleteLater = deleteBook.bind(null, bookId);
  console.log("  bind の結果:", typeof deleteLater);
  // NG: () を付けると、この行でもう削除が実行されてしまう
  const deleteNow = deleteBook(bookId);
  console.log("  () の結果:", Object.prototype.toString.call(deleteNow));
}

main();
```

**▼ 実行結果**（`node bind-hidden.ts`）

```text
【方法A】bind で id を先に固定する
  → updateBook 実行: id=3f2a8c1e-5b7d-4e9a-9c01-2b3d4e5f6a7b / title=React実践ガイド 第2版
【方法B】hidden input で id も一緒に送る
  → updateBookFromHidden 実行: id=3f2a8c1e-5b7d-4e9a-9c01-2b3d4e5f6a7b / title=React実践ガイド 第2版
【方法B の失敗例】hidden input を書き忘れた
  → id が届いていないので更新しない
【NG と OK】() を付けるとその場で実行される
  bind の結果: function
  → deleteBook 実行: id=3f2a8c1e-5b7d-4e9a-9c01-2b3d4e5f6a7b
  () の結果: [object Promise]
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `bind(null, bookId)` は「1番目の引数を固定した新しい関数」を作る

```ts
const updateBookWithId = updateBook.bind(null, bookId);
await updateBookWithId(formData);
```

- `関数.bind(null, 値)` は、元の関数の **1番目の引数をあらかじめ埋めた、新しい関数** を返します（`null` は「`this` は使わない」という決まり文句です）。
- できた `updateBookWithId` を `formData` だけで呼ぶと、中では `updateBook(bookId, formData)` が呼ばれます。実行結果でも id と title の両方が届いています。
- Next.js でも同じで、`<form action={updateBookWithId}>` に渡しておけば、送信時に「固定した id ＋ フォームの中身」でサーバー側の関数が呼ばれます。

##### 解説2: hidden input の値は `formData.get("id")` で取り出し、型を確かめる

```ts
const id = formData.get("id");
if (typeof id !== "string" || id === "") {
  console.log("  → id が届いていないので更新しない");
  return;
}
```

- hidden input（`<input type="hidden">`）は画面に表示されない入力欄で、送信時に他の欄と一緒に送られます。
- `formData.get()` の結果は「文字列・ファイル・`null`（その名前の欄が無い）」のどれかです。そのまま使わず、`typeof id !== "string"` で文字列かどうかを確かめます。
- 3つ目の実行結果のように、hidden input を書き忘れると `null` になり、ここで処理を止められます。

##### 解説3: `()` を付けると、その場で実行されてしまう

```ts
const deleteLater = deleteBook.bind(null, bookId);
const deleteNow = deleteBook(bookId);
```

- `bind` の結果は `function`（まだ実行していない関数）でした。
- 一方 `deleteBook(bookId)` は、**その行で実行されて** `→ deleteBook 実行` が表示され、結果は `Promise`（実行中の処理）です。
- JSX で `action={deleteBook(id)}` と書くと、まさにこの「その場で実行」が起こります。ページを描くたびに削除が走る、という一番危ないバグです（下のつまずきポイント5）。

#### Server Action 版の編集フォーム（参考）

最後に、本物の Next.js で書くとどうなるかを示します。この章のアプリはクライアントコンポーネントから Supabase を直接呼ぶ方式で完成しているので、**以下は組み込まなくて大丈夫な参考コード** です（タイトルだけを書き換える小さなフォームにしています）。

**ファイル: `app/books/actions.ts`（参考）**

> **▼ このコードがやること（先に日本語で）:** サーバー側で動く「タイトル更新」の関数を2種類用意します。1つは bind 用（`id` を1番目の引数で受け取る）、もう1つは hidden input 用（フォームの中から `id` を取り出す）です。どちらも最後は `.eq("id", id)` で1件だけ更新し、キャッシュを捨ててから詳細ページへ移動します。

```ts
// このファイルの関数は、すべてサーバー側で動く Server Action になる
"use server";

// キャッシュを捨てて、次の表示で最新データを取らせる関数
import { revalidatePath } from "next/cache";
// サーバー側からページを移動させる関数
import { redirect } from "next/navigation";
// サーバー用の Supabase クライアント
import { createClient } from "@/lib/supabase/server";

// 方法A（bind）用: 1番目の引数で id、2番目でフォームの中身を受け取る
export async function updateBookTitle(id: string, formData: FormData) {
  // name="title" の欄の値を取り出す（無ければ空文字）
  const title = String(formData.get("title") ?? "").trim();
  if (!title) {
    throw new Error("タイトルは必須です。");
  }

  const supabase = await createClient();
  const { error } = await supabase
    .from("books")
    .update({ title })
    // 必ず id で1件に絞る
    .eq("id", id);

  if (error) {
    throw new Error("書籍の更新に失敗しました。");
  }

  // 詳細ページと一覧ページのキャッシュを捨てる
  revalidatePath(`/books/${id}`);
  revalidatePath("/books");
  // 詳細ページへ移動する（この行より後ろは実行されない）
  redirect(`/books/${id}`);
}

// 方法B（hidden input）用: フォームの中身だけを受け取る
export async function updateBookTitleFromHidden(formData: FormData) {
  const id = formData.get("id");
  // hidden input が無い、または文字列でなければ止める
  if (typeof id !== "string" || id === "") {
    throw new Error("書籍の id が送られていません。");
  }
  // あとは方法A と同じ処理に任せる
  await updateBookTitle(id, formData);
}
```

**ファイル: `components/EditTitleForm.tsx`（参考）**

```tsx
// 上で作った Server Action を読み込む
import {
  updateBookTitle,
  updateBookTitleFromHidden,
} from "@/app/books/actions";

type Props = {
  id: string;
  title: string;
};

// 方法A: bind で id を固定する
export function EditTitleFormWithBind({ id, title }: Props) {
  // id を1番目の引数に固定した新しい関数を作る（まだ実行しない）
  const updateBookTitleWithId = updateBookTitle.bind(null, id);

  return (
    <form action={updateBookTitleWithId}>
      <input name="title" defaultValue={title} />
      <button type="submit">更新する</button>
    </form>
  );
}

// 方法B: hidden input で id も一緒に送る
export function EditTitleFormWithHidden({ id, title }: Props) {
  return (
    <form action={updateBookTitleFromHidden}>
      {/* 画面には出ないが、送信時に id も一緒に送られる */}
      <input type="hidden" name="id" value={id} />
      <input name="title" defaultValue={title} />
      <button type="submit">更新する</button>
    </form>
  );
}
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `"use server"` と `revalidatePath` / `redirect`

```ts
"use server";

revalidatePath(`/books/${id}`);
revalidatePath("/books");
redirect(`/books/${id}`);
```

- ファイルの先頭の `"use server"` で、このファイルの関数はすべて「ブラウザから呼べるサーバー側の関数（Server Action）」になります。Supabase への接続はサーバーの中だけで行われます。
- `revalidatePath` は、この章の `router.refresh()` にあたる処理です。Server Action の中では `router` が使えないので、代わりに「このURLのキャッシュは古くなった」と伝えます。
- `redirect` は `notFound()` と同じく **特別なエラーを投げて** ページを移動させます。そのため `try { … } catch { … }` の中で呼ぶと、つまずきポイント3と同じ理由で効かなくなります。

##### 解説2: フォームは `value` ではなく `defaultValue`

```tsx
<input name="title" defaultValue={title} />
```

- Server Action 版のフォームは `useState` を使わないので、2-2 の囲み記事で説明した **非制御コンポーネント**（`defaultValue`）で書きます。最初に今のタイトルを入れておくだけで、入力された値は送信時に FormData として届きます。
- `name="title"` が、サーバー側の `formData.get("title")` の名前と一致している必要があります。

##### 解説3: どちらの方法でも、サーバー側の確認は省けない

```ts
const id = formData.get("id");
if (typeof id !== "string" || id === "") {
  throw new Error("書籍の id が送られていません。");
}
```

- hidden input の値は、ブラウザの開発者ツールで誰でも書き換えられます。bind の場合も、ブラウザから送られてくる値であることに変わりはありません。
- 「id は書き換えられて届くかもしれない」と考え、**その本を更新してよい人かどうかは、サーバー側（Supabase の RLS ポリシーなど）で確認する** のが基本です（7-4 の本番用ポリシーがその例です）。

> **つまずきポイント5: `action={updateBookTitle(id)}` と `()` を付けて書いてしまう。** これは「関数を渡す」のではなく「関数をその場で実行して、その結果を渡す」意味になります。サーバーコンポーネントの中なら **ページを表示するたびにサーバーで更新処理が走ります**。TypeScript を使っていれば `Expected 2 arguments, but got 1.`（引数が2つ必要なのに1つしか無い）などの型エラーで気付けるので、赤い波線を無視しないでください。

**NG（まちがい・抜粋）:**

```tsx
// () を付けると、描画するたびにその場で実行されてしまう
<form action={updateBookTitle(id)}>
```

**OK（正しい・抜粋）:**

```tsx
// bind は「id を固定した関数」を作るだけ。実行は送信したとき
<form action={updateBookTitle.bind(null, id)}>
```

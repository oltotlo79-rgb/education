## 9. 練習問題（解答・解説つき）

この章の内容を、手を動かさずに頭の中で確かめる問題です。まず自分で答えを考えてから、解答と解説を読んでください。

### 問題1: params の中身

`app/books/[id]/edit/page.tsx` というファイルがあります。ブラウザで `/books/42/edit` を開いたとき、`const p = await params;` の `p` はどれになりますか。

- (a) `{ id: 42 }`
- (b) `{ id: "42" }`
- (c) `{ id: "42/edit" }`
- (d) `{ edit: "42" }`

**▼ 解答:** (b)

**▼ 解説:** params のキーの名前は、フォルダの角括弧の中身（`[id]` → `id`）で決まります。`edit` は角括弧の付いていない普通のフォルダなので、params には入りません。また、URL から取り出した値は数字に見えても **必ず文字列** なので、(a) の `42`（数値）ではなく `"42"` です（0.1 の「動かして確かめる」の実行結果 `/books/42 → {"id":"42"}` を思い出してください）。

### 問題2: バグを探そう（詳細ページ）

Next.js 16 のプロジェクトで、次のコードを書いたところ、TypeScript が赤い波線でエラーを出しました。どこが間違っていて、どう直せばよいでしょうか。

```tsx
type Props = {
  params: Promise<{ id: string }>;
};

export default async function BookDetailPage({ params }: Props) {
  const id = params.id;
  return <p>{id}</p>;
}
```

**▼ 解答:** `params` は Promise なので、`await` で中身を取り出してから使います。

```tsx
export default async function BookDetailPage({ params }: Props) {
  const { id } = await params;
  return <p>{id}</p>;
}
```

**▼ 解説:** 実際に TypeScript でチェックすると、`Property 'id' does not exist on type 'Promise<{ id: string; }>'.`（Promise 型には id というプロパティが無い）というエラーになります。Promise は「あとで値が出てくる箱」なので、箱そのものに `.id` はありません。Next.js 15 までは移行のために古い書き方も一時的に動いていましたが、Next.js 16 では必ず `await` が必要です（つまずきポイント1）。

### 問題3: 危ない削除はどれ？

次のうち、**2件以上の本を消してしまう可能性がある** ものをすべて選んでください。

- (a) `supabase.from("books").delete().eq("id", bookId)`
- (b) `supabase.from("books").delete()`
- (c) `supabase.from("books").delete().eq("status", "finished")`
- (d) `supabase.from("books").delete().eq("id", bookId).select()`

**▼ 解答:** (b) と (c)

**▼ 解説:** (b) は条件が無いので、SQL の `DELETE FROM books;` と同じく全行が対象です（Supabase の安全装置で止まることもありますが、頼ってはいけません）。(c) は条件を付けていますが、「読了の本すべて」が対象なので、読了の本が複数あれば全部消えます。(a) と (d) は主キーの `id` で絞っているので、消えるのは最大1件です。(d) の `.select()` は「消した行を返して」という指定で、消す範囲は (a) と同じです（3-3、3-4）。

### 問題4: 穴埋め（Server Action に id を渡す）

サーバー側に `updateBookTitle(id: string, formData: FormData)` という Server Action があります。フォームの送信時に、この関数へ `id` を渡したいとき、空欄 ①② に入るものを答えてください。

```tsx
const updateBookTitleWithId = updateBookTitle.①(null, id);

return (
  <form action={②}>
    <input name="title" defaultValue={title} />
    <button type="submit">更新する</button>
  </form>
);
```

**▼ 解答:** ① `bind`　② `updateBookTitleWithId`

**▼ 解説:** `bind(null, id)` は「1番目の引数を `id` で固定した新しい関数」を作ります。`action` には **関数そのもの** を渡すので、② は `updateBookTitleWithId` です。ここに `updateBookTitle(id)` と `()` を付けて書くと、送信時ではなく描画した瞬間に実行されてしまいます（2-4、つまずきポイント5）。

### 問題5: null と空文字の変換

DB の行が `{ publisher: null, memo: "読了" }` だったとします。次の①〜③の値はそれぞれ何になりますか。

- ① 編集ページで作る初期値: `book.publisher ?? ""`
- ② ユーザーが出版社欄に空白を2つだけ打って保存したとき: `"  ".trim() || null`
- ③ メモ欄はそのままで保存したとき: `"読了".trim() || null`

**▼ 解答:** ① `""`（空文字）　② `null`　③ `"読了"`

**▼ 解説:** ① `??` は左が `null` か `undefined` のときだけ右を使うので、`null` が空文字になります。② `trim()` で空白を削ると `""` になり、`""` は `||` にとって「空っぽ」なので右の `null` が選ばれます。③ `"読了"` は空ではないので、そのまま左が使われます。行きは `?? ""`、帰りは `|| null` という変換のおかげで、DB は「未入力 = null」のルールを保てます（図8-8）。

### 問題6: 楽観的更新のあと始末

楽観的更新で「読書中」を「読了」に変えて、すぐ画面に「読了」と表示しました。ところがサーバーからはエラーが返ってきました。このとき、プログラムが必ずやるべきことを2つ答えてください。

**▼ 解答:** ① 画面の表示を、変更前の「読書中」に **巻き戻す**。② 保存に失敗したことを **ユーザーに知らせる**（エラーメッセージを出す）。

**▼ 解説:** 楽観的更新は「成功するはず」と考えて先に画面を変える方式なので、失敗したときは画面とデータベースの内容が食い違います。巻き戻さないと、DB は「読書中」なのに画面は「読了」のままになり、ユーザーは保存できたと勘違いします。そのため、変更前の値を覚えておく必要があります（図8-6 と「動かして確かめる」の実行結果の3つ目）。

### 問題7: エラーが出ないのに消えない

`DeleteButton` で削除を実行しました。`error` は `null` で、一覧ページにも戻りましたが、本は一覧に残ったままでした（ページを再読み込みしても残っています）。考えられる原因を2つと、それを確かめる方法を答えてください。

**▼ 解答:** 原因: ① RLS に DELETE を許可するポリシーが無い（または条件に合っていない）。② `.eq("id", bookId)` に渡した `bookId` が、消したい本の id と違っている。確かめる方法: `.select()` を付けて削除された行を返してもらい、`data.length` が 0 かどうかを確かめる（あわせて `console.log` で `bookId` を表示する）。

**▼ 解説:** RLS で許可されていない行は「見えない行」として扱われるので、DELETE は **0件を削除して成功** という結果になり、エラーにはなりません。id 違いも同じく0件で成功です。`if (error)` だけでは見逃すので、件数まで確かめるのが確実です（3-4、つまずきポイント7、7-2）。再読み込みしても残っているので、`router.refresh()` の呼び忘れ（画面が古いだけ）ではない点も手がかりになります。

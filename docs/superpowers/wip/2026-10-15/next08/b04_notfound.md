### 1-4. 存在しない id が開かれたとき（notFound の流れ）

ブックマークしていた本をあとで削除した、URL を1文字打ち間違えた……といった理由で、**存在しない id の URL** が開かれることは普通にあります。詳細ページのコードはこれを `if (error || !book) notFound();` の1行で受け止めていますが、実際には次の3つのパターンがあります。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-04" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="120" y="24" text-anchor="middle" font-size="13" font-weight="700" fill="#475569">開かれた URL</text>
<text x="370" y="24" text-anchor="middle" font-size="13" font-weight="700" fill="#475569">.single() の結果</text>
<text x="610" y="24" text-anchor="middle" font-size="13" font-weight="700" fill="#475569">表示される画面</text>
<rect x="10" y="40" width="220" height="64" rx="8" fill="#f8fafc" stroke="#94a3b8"/>
<text x="120" y="66" text-anchor="middle" font-size="12" fill="#1e293b" font-family="Consolas, monospace">/books/3f2a8c1e-…</text>
<text x="120" y="88" text-anchor="middle" font-size="12" fill="#475569">① 存在する本の id</text>
<rect x="270" y="40" width="200" height="64" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="370" y="66" text-anchor="middle" font-size="12" fill="#047857" font-family="Consolas, monospace">data: { title: … }</text>
<text x="370" y="88" text-anchor="middle" font-size="12" fill="#047857" font-family="Consolas, monospace">error: null</text>
<rect x="510" y="40" width="200" height="64" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="610" y="68" text-anchor="middle" font-size="14" font-weight="700" fill="#047857">詳細ページを表示</text>
<text x="610" y="88" text-anchor="middle" font-size="12" fill="#047857">本の情報が並ぶ</text>
<rect x="10" y="120" width="220" height="64" rx="8" fill="#f8fafc" stroke="#94a3b8"/>
<text x="120" y="146" text-anchor="middle" font-size="12" fill="#1e293b" font-family="Consolas, monospace">/books/00000000-…</text>
<text x="120" y="168" text-anchor="middle" font-size="12" fill="#475569">② 形は正しいが存在しない</text>
<rect x="270" y="120" width="200" height="64" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="370" y="146" text-anchor="middle" font-size="12" fill="#c2410c" font-family="Consolas, monospace">data: null</text>
<text x="370" y="168" text-anchor="middle" font-size="12" fill="#c2410c">error: PGRST116（0件）</text>
<rect x="510" y="120" width="200" height="64" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="610" y="148" text-anchor="middle" font-size="14" font-weight="700" fill="#c2410c" font-family="Consolas, monospace">notFound()</text>
<text x="610" y="168" text-anchor="middle" font-size="12" fill="#c2410c">404 用の画面</text>
<rect x="10" y="200" width="220" height="64" rx="8" fill="#f8fafc" stroke="#94a3b8"/>
<text x="120" y="226" text-anchor="middle" font-size="12" fill="#1e293b" font-family="Consolas, monospace">/books/abc</text>
<text x="120" y="248" text-anchor="middle" font-size="12" fill="#475569">③ UUID の形をしていない</text>
<rect x="270" y="200" width="200" height="64" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="370" y="226" text-anchor="middle" font-size="12" fill="#c2410c" font-family="Consolas, monospace">data: null</text>
<text x="370" y="248" text-anchor="middle" font-size="12" fill="#c2410c">error: 22P02（型が違う）</text>
<rect x="510" y="200" width="200" height="64" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="610" y="228" text-anchor="middle" font-size="14" font-weight="700" fill="#c2410c" font-family="Consolas, monospace">notFound()</text>
<text x="610" y="248" text-anchor="middle" font-size="12" fill="#c2410c">404 用の画面</text>
<line x1="230" y1="72" x2="266" y2="72" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-04)"/>
<line x1="470" y1="72" x2="506" y2="72" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-04)"/>
<line x1="230" y1="152" x2="266" y2="152" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-04)"/>
<line x1="470" y1="152" x2="506" y2="152" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-04)"/>
<line x1="230" y1="232" x2="266" y2="232" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-04)"/>
<line x1="470" y1="232" x2="506" y2="232" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-04)"/>
<text x="360" y="290" text-anchor="middle" font-size="12" fill="#475569">if (error || !book) notFound(); の1行で、② と ③ をまとめて 404 用の画面にしている</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-3: 存在しない id が開かれたときの3つのパターン</div>
</div>

- **① 存在する本の id**: `data` に本が1冊入り、`error` は `null` です。そのまま詳細ページを描きます。
- **② 形は正しいが存在しない id**: `.single()` は「ちょうど1件」を期待しているので、0件だと `PGRST116` というエラーを返します。
- **③ UUID の形をしていない id**（`/books/abc` など）: `books` テーブルの `id` 列は `uuid` 型なので、データベースが「その値は uuid として読めない」という型エラー（`22P02`）を返します。

どのパターンでも「`error` がある、または `book` が無い」なら `notFound()` を呼ぶ、とまとめて書いておけば、②と③を区別せずに 404 用の画面へ送れます。

> **用語:** **PGRST116** は Supabase（の中で動く PostgREST）が「1件だけ返してと言われたのに、0件または2件以上だった」ときに返すエラーコード。**22P02** は PostgreSQL の「入力の形式が型に合わない」エラーコードです。どちらも `error.code` で読めます。

#### 動かして確かめる: `.single()` と `.maybeSingle()` の返り値

> **▼ このコードがやること（先に日本語で）:** Supabase の `.single()` と `.maybeSingle()` が返す `{ data, error }` の **形だけ** を真似した関数を作り、図8-3 の3つの id で呼び出して、詳細ページの判定（`if (error || !book) notFound();`）がどちらに進むかを表示します。本物の通信はしません。

**ファイル: `not-found-check.ts`（練習用）**

```ts
// Supabase の .single() / .maybeSingle() が返す { data, error } の
// 「形」を真似した練習用コード（本物の supabase-js ではありません）

type Book = {
  id: string;
  title: string;
};

type QueryResult = {
  data: Book | null;
  error: { code: string } | null;
};

// books テーブルの代わりの配列（id は UUID 形式の文字列）
const books: Book[] = [
  { id: "3f2a8c1e-5b7d-4e9a-9c01-2b3d4e5f6a7b", title: "React実践ガイド" },
];

// UUID の形（8-4-4-4-12 桁の16進数）かどうかを調べる正規表現
const UUID_PATTERN =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

// .eq("id", id).single() の動きを真似する
function findSingle(id: string): QueryResult {
  // UUID の形をしていない値は、DB が型エラー（22P02）で拒否する
  if (!UUID_PATTERN.test(id)) {
    return { data: null, error: { code: "22P02" } };
  }
  const rows = books.filter((book) => book.id === id);
  // ちょうど1件でなければエラー（PGRST116）
  if (rows.length !== 1) {
    return { data: null, error: { code: "PGRST116" } };
  }
  return { data: rows[0], error: null };
}

// .eq("id", id).maybeSingle() の動きを真似する
function findMaybeSingle(id: string): QueryResult {
  if (!UUID_PATTERN.test(id)) {
    return { data: null, error: { code: "22P02" } };
  }
  const rows = books.filter((book) => book.id === id);
  // 0件のときはエラーにせず data: null を返すのが maybeSingle
  if (rows.length === 0) {
    return { data: null, error: null };
  }
  return { data: rows[0], error: null };
}

// page.tsx の「if (error || !book) notFound();」と同じ判定
function decide(result: QueryResult): string {
  if (result.error || !result.data) {
    return "notFound() → 404 ページ";
  }
  return `詳細ページを表示「${result.data.title}」`;
}

const testIds = [
  // 1. 存在する本の id
  "3f2a8c1e-5b7d-4e9a-9c01-2b3d4e5f6a7b",
  // 2. 形は正しいが、存在しない id
  "00000000-0000-4000-8000-000000000000",
  // 3. そもそも UUID の形をしていない id
  "abc",
];

for (const id of testIds) {
  const single = findSingle(id);
  const maybe = findMaybeSingle(id);
  console.log(`id = ${id}`);
  console.log(
    `  single()      error=${single.error?.code ?? "なし"} → ${decide(single)}`,
  );
  console.log(
    `  maybeSingle() error=${maybe.error?.code ?? "なし"} → ${decide(maybe)}`,
  );
}
```

**▼ 実行結果**（`node not-found-check.ts`）

```text
id = 3f2a8c1e-5b7d-4e9a-9c01-2b3d4e5f6a7b
  single()      error=なし → 詳細ページを表示「React実践ガイド」
  maybeSingle() error=なし → 詳細ページを表示「React実践ガイド」
id = 00000000-0000-4000-8000-000000000000
  single()      error=PGRST116 → notFound() → 404 ページ
  maybeSingle() error=なし → notFound() → 404 ページ
id = abc
  single()      error=22P02 → notFound() → 404 ページ
  maybeSingle() error=22P02 → notFound() → 404 ページ
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `.single()` は「ちょうど1件」でなければエラー

```ts
const rows = books.filter((book) => book.id === id);
if (rows.length !== 1) {
  return { data: null, error: { code: "PGRST116" } };
}
```

- `.single()` は「結果は必ず1件のはず」という **約束** です。0件でも2件以上でも約束違反なので、`error` が返ってきます。
- 2件目のパターン（形は正しいが存在しない id）では、実行結果のとおり `error=PGRST116` になりました。

##### 解説2: `.maybeSingle()` は「0件ならエラーではなく null」

```ts
if (rows.length === 0) {
  return { data: null, error: null };
}
```

- `.maybeSingle()` は「あるかもしれないし、無いかもしれない」ときに使う書き方です。0件は **エラーではない** ので、`error` は `null` のまま `data` が `null` になります。
- 実行結果の2件目では `maybeSingle() error=なし` でした。それでも判定が 404 になったのは、`!result.data`（本が無い）の方で引っかかったからです。

##### 解説3: `error || !data` の両方を見るから、どちらの書き方でも安全

```ts
function decide(result: QueryResult): string {
  if (result.error || !result.data) {
    return "notFound() → 404 ページ";
  }
  return `詳細ページを表示「${result.data.title}」`;
}
```

- 詳細ページでは「エラーがある」**または**「データが無い」のどちらかなら 404 にしています。
- こう書いておけば、`.single()` を使っても `.maybeSingle()` を使っても、3つのパターンすべてで正しく分岐できます。
- 3件目の `abc` はどちらの書き方でも `22P02` です。形の間違いは「0件」ではなく本当のエラーなので、`.maybeSingle()` でも `error` が返ります。

> **つまずきポイント3: `notFound()` を `try { … } catch { … }` の中で呼んでしまう。** `notFound()` は「404 用の画面に切り替えて」という合図を **特別なエラーとして投げる（throw する）** ことで動いています。何でも受け止める `catch` の中にいると、その合図まで受け止めて握りつぶしてしまい、404 用の画面が出なくなります。データの取得だけを `try` に入れるか、この章のように `try` を使わずに `if` で判定してください。

**NG（まちがい・抜粋）:**

```tsx
try {
  const { data: book, error } = await supabase
    .from("books")
    .select("*")
    .eq("id", id)
    .single();
  if (error || !book) {
    // この合図（特別なエラー）が、すぐ下の catch に捕まってしまう
    notFound();
  }
  return <h1>{book.title}</h1>;
} catch (e) {
  console.error(e);
  return <p>エラーが発生しました</p>;
}
```

**OK（正しい・抜粋）:**

```tsx
const { data: book, error } = await supabase
  .from("books")
  .select("*")
  .eq("id", id)
  .single();

// try の外で呼べば、合図がそのまま Next.js まで届く
if (error || !book) {
  notFound();
}
return <h1>{book.title}</h1>;
```

#### 404 用の画面を自分で作る: `not-found.tsx`

`notFound()` を呼んだときに表示される画面は、同じフォルダに `not-found.tsx` を置くと自分好みに作り替えられます。何も置かなければ Next.js 標準の、英語の素っ気ない 404 画面になります。

**ファイル: `app/books/[id]/not-found.tsx`**

> **▼ このコードがやること（先に日本語で）:** 「その本は見つかりませんでした」という日本語の案内と、一覧へ戻るボタンを出す 404 用の画面です。`app/books/[id]/` の中で `notFound()` が呼ばれると（詳細ページでも、その下の編集ページでも）、この画面が表示されます。

```tsx
// 一覧に戻るためのリンク部品
import Link from "next/link";

// not-found.tsx は props を受け取らない決まり（どの id だったかは分からない）
export default function BookNotFound() {
  return (
    <div className="max-w-2xl mx-auto py-16 px-4 text-center">
      {/* 大きく薄い文字で 404 と表示 */}
      <p className="text-5xl font-bold text-gray-300">404</p>
      <h1 className="mt-4 text-xl font-bold text-gray-900">
        書籍が見つかりません
      </h1>
      <p className="mt-2 text-sm text-gray-500">
        URL が間違っているか、すでに削除された可能性があります。
      </p>
      {/* 一覧ページへ戻るボタン */}
      <Link
        href="/books"
        className="mt-6 inline-block rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
      >
        一覧に戻る
      </Link>
    </div>
  );
}
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 置き場所で「どこの 404 か」が決まる

```text
app/books/[id]/not-found.tsx
```

- `not-found.tsx` は、同じフォルダとその下のフォルダで `notFound()` が呼ばれたときに使われます。ここに置けば、詳細ページ（`[id]/page.tsx`）と編集ページ（`[id]/edit/page.tsx`）の両方で共通の 404 画面になります。
- `app/not-found.tsx` に置けば、アプリ全体の 404 画面（存在しない URL を開いたときも含む）になります。

##### 解説2: props を受け取らないので、id は表示できない

```tsx
export default function BookNotFound() {
```

- `not-found.tsx` の関数は **引数（props）を受け取りません**。そのため「id: abc は見つかりません」のように、どの id だったかを表示することはできません。案内文は一般的な言い方にしておきます。

完成すると、存在しない id の URL を開いたときに次のような画面になります。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="700" height="280" rx="10" fill="#ffffff" stroke="#cbd5e1" stroke-width="2"/>
<path d="M10,46 L10,20 Q10,10 20,10 L700,10 Q710,10 710,20 L710,46 Z" fill="#f1f5f9"/>
<line x1="10" y1="46" x2="710" y2="46" stroke="#cbd5e1"/>
<circle cx="32" cy="28" r="5" fill="#f87171"/>
<circle cx="50" cy="28" r="5" fill="#fbbf24"/>
<circle cx="68" cy="28" r="5" fill="#34d399"/>
<rect x="90" y="18" width="600" height="20" rx="10" fill="#ffffff" stroke="#e2e8f0"/>
<text x="104" y="33" font-size="12" fill="#475569" font-family="Consolas, monospace">localhost:3000/books/00000000-0000-4000-8000-000000000000</text>
<text x="360" y="118" text-anchor="middle" font-size="44" font-weight="700" fill="#cbd5e1">404</text>
<text x="360" y="158" text-anchor="middle" font-size="20" font-weight="700" fill="#1e293b">書籍が見つかりません</text>
<text x="360" y="188" text-anchor="middle" font-size="13" fill="#64748b">URL が間違っているか、すでに削除された可能性があります。</text>
<rect x="290" y="210" width="140" height="36" rx="8" fill="#2563eb"/>
<text x="360" y="233" text-anchor="middle" font-size="13" font-weight="700" fill="#ffffff">一覧に戻る</text>
<text x="696" y="278" text-anchor="end" font-size="12" fill="#94a3b8">app/books/[id]/not-found.tsx の画面</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-4: 画面イメージ ― 存在しない id を開いたときの 404 用の画面</div>
</div>

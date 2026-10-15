`/books/abc123` というURLが開かれてから、`page.tsx` の中で `id` が使えるようになるまでを1枚にすると、次のようになります。ポイントは「URL の区画（セグメント）とフォルダが1つずつ対応している」ことと、「Next.js は値を **Promise の箱** に入れて渡してくる」ことの2つです。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 320" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-02" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
<marker id="ar-08-03" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#f97316"/></marker>
</defs>
<text x="20" y="24" font-size="13" font-weight="700" fill="#475569">ブラウザのアドレスバー（URL）</text>
<rect x="20" y="34" width="680" height="50" rx="25" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<rect x="40" y="43" width="200" height="32" rx="6" fill="#ffffff" stroke="#cbd5e1"/>
<text x="140" y="64" text-anchor="middle" font-size="14" fill="#64748b" font-family="Consolas, monospace">localhost:3000</text>
<rect x="270" y="43" width="170" height="32" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="355" y="64" text-anchor="middle" font-size="14" fill="#1e40af" font-family="Consolas, monospace">/books</text>
<rect x="470" y="43" width="200" height="32" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="570" y="64" text-anchor="middle" font-size="14" font-weight="700" fill="#c2410c" font-family="Consolas, monospace">/abc123</text>
<line x1="355" y1="76" x2="355" y2="142" stroke="#3b82f6" stroke-width="2" stroke-dasharray="5 4"/>
<text x="363" y="114" font-size="12" fill="#1e40af">名前が一致</text>
<line x1="570" y1="76" x2="570" y2="142" stroke="#f97316" stroke-width="2" stroke-dasharray="5 4"/>
<text x="578" y="114" font-size="12" fill="#c2410c">[id] に入る</text>
<text x="20" y="136" font-size="13" font-weight="700" fill="#475569">app フォルダの中</text>
<rect x="40" y="144" width="200" height="40" rx="6" fill="#ffffff" stroke="#cbd5e1"/>
<text x="140" y="169" text-anchor="middle" font-size="14" fill="#64748b" font-family="Consolas, monospace">app/</text>
<rect x="270" y="144" width="170" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="355" y="169" text-anchor="middle" font-size="14" fill="#1e40af" font-family="Consolas, monospace">books/</text>
<rect x="470" y="144" width="200" height="40" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="570" y="169" text-anchor="middle" font-size="14" font-weight="700" fill="#c2410c" font-family="Consolas, monospace">[id]/page.tsx</text>
<line x1="570" y1="184" x2="570" y2="212" stroke="#f97316" stroke-width="2" marker-end="url(#ar-08-03)"/>
<text x="560" y="204" text-anchor="end" font-size="12" fill="#c2410c">Next.js が params を作って渡す</text>
<rect x="20" y="216" width="680" height="94" rx="10" fill="#ffffff" stroke="#cbd5e1"/>
<text x="34" y="236" font-size="12" font-weight="700" fill="#475569">page.tsx の中</text>
<rect x="40" y="246" width="210" height="54" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="145" y="267" text-anchor="middle" font-size="13" font-weight="700" fill="#c2410c">params（Promise の箱）</text>
<text x="145" y="288" text-anchor="middle" font-size="12" fill="#c2410c" font-family="Consolas, monospace">Promise&lt;{ id: "abc123" }&gt;</text>
<rect x="290" y="246" width="160" height="54" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="370" y="267" text-anchor="middle" font-size="13" font-weight="700" fill="#1e40af">await で箱を開ける</text>
<text x="370" y="288" text-anchor="middle" font-size="12" fill="#1e40af" font-family="Consolas, monospace">await params</text>
<rect x="490" y="246" width="190" height="54" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="585" y="267" text-anchor="middle" font-size="13" font-weight="700" fill="#047857">id が使えるようになる</text>
<text x="585" y="288" text-anchor="middle" font-size="12" fill="#047857" font-family="Consolas, monospace">id = "abc123"</text>
<line x1="250" y1="273" x2="286" y2="273" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-02)"/>
<line x1="450" y1="273" x2="486" y2="273" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-02)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-2: URL の区画とフォルダが対応し、[id] の位置の値が params に入って届く</div>
</div>

この `id = "abc123"` を、あとで `.eq("id", id)` に渡して「この id の本を1冊ください」と Supabase に頼みます（1-3 で登場します）。

#### 動かして確かめる: URL から id が取り出される仕組み

> **ヒント:** この章の「動かして確かめる」コードは、Next.js のアプリとは別の作業用フォルダ（例: `practice`）に `.ts` ファイルとして保存し、ターミナルで `node ファイル名.ts` と打つと実行できます。Node.js 24 以降なら、型注釈付きの TypeScript をそのまま実行できます（この教材は Node.js 25 で確認しています。バージョンによっては `ExperimentalWarning` という注意書きが出ますが、動作には問題ありません）。どれも本物の Next.js や Supabase は使わず、**仕組みだけを小さく真似した練習用のコード** です。

> **▼ このコードがやること（先に日本語で）:** Next.js が裏側でやっている「URL を `/` で区切って、`[id]` の位置にある値を取り出す」処理を、30行ほどで真似します。さらに、取り出した値を Promise の箱に入れて渡し、`await` を付けた場合と付け忘れた場合で結果がどう変わるかを見比べます。

**ファイル: `route-params.ts`（練習用）**

```ts
// Next.js が「URL から [id] を取り出して params に入れる」動きを
// 小さく真似した練習用コード（本物の Next.js の実装ではありません）

// ルートの形（例: "/books/[id]"）と実際の URL を見比べて、
// [ ] で囲まれた部分の値を集めたオブジェクトを返す。一致しなければ null
function matchRoute(
  pattern: string,
  url: string,
): Record<string, string> | null {
  // "/" で区切ってセグメント（区画）の配列にする。空文字は捨てる
  const patternParts = pattern.split("/").filter((s) => s !== "");
  const urlParts = url.split("/").filter((s) => s !== "");

  // セグメントの数が違えば、このルートとは一致しない
  if (patternParts.length !== urlParts.length) {
    return null;
  }

  // 取り出した値をためていく入れ物
  const params: Record<string, string> = {};

  for (let i = 0; i < patternParts.length; i++) {
    const p = patternParts[i];
    const u = urlParts[i];
    if (p.startsWith("[") && p.endsWith("]")) {
      // "[id]" の角括弧を外した "id" を名前にして、URL 側の値を入れる
      params[p.slice(1, -1)] = u;
    } else if (p !== u) {
      // 固定の部分（"books" など）が違えば不一致
      return null;
    }
  }
  return params;
}

// page.tsx と同じ形の Props 型（params は Promise）
type Props = {
  params: Promise<{ id: string }>;
};

// 正しい書き方: await で Promise の中身を取り出す
async function GoodPage({ params }: Props) {
  const { id } = await params;
  console.log(`[OK] id = "${id}" / 型は ${typeof id}`);
}

// まちがった書き方: await を忘れて Promise から直接 id を読む
async function BadPage({ params }: Props) {
  // Promise 自体には id というプロパティが無いので undefined になる
  const id = (params as unknown as { id?: string }).id;
  console.log(`[NG] id = ${id}`);
}

async function main() {
  // 3 つの URL を "/books/[id]" と見比べてみる
  const urls = ["/books/abc123", "/books/42", "/books/abc123/edit"];
  for (const url of urls) {
    const found = matchRoute("/books/[id]", url);
    console.log(`${url} → ${JSON.stringify(found)}`);
  }

  // Next.js と同じように、取り出した値を Promise に包んで渡す
  const params = Promise.resolve({ id: "42" });
  await GoodPage({ params });
  await BadPage({ params });
}

main();
```

**▼ 実行結果**（`node route-params.ts`）

```text
/books/abc123 → {"id":"abc123"}
/books/42 → {"id":"42"}
/books/abc123/edit → null
[OK] id = "42" / 型は string
[NG] id = undefined
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: URL を `/` で区切って、ルートの形と見比べる

```ts
const patternParts = pattern.split("/").filter((s) => s !== "");
const urlParts = url.split("/").filter((s) => s !== "");

if (patternParts.length !== urlParts.length) {
  return null;
}
```

- `"/books/[id]".split("/")` は `["", "books", "[id]"]` になります。先頭の空文字は `filter` で捨てて、`["books", "[id]"]` という「区画の並び」にします。
- 区画の数が違う URL は、そもそも別のページです。だから `/books/abc123/edit`（3区画）は `/books/[id]`（2区画）とは一致せず、実行結果でも `null` になりました。この URL は `app/books/[id]/edit/page.tsx` という別のファイルが受け持ちます。

> **用語:** **セグメント（segment）** は URL を `/` で区切った1区画のこと。`/books/abc123` なら `books` と `abc123` の2つです。

##### 解説2: `[id]` の位置にある値を `id` という名前で入れる

```ts
if (p.startsWith("[") && p.endsWith("]")) {
  params[p.slice(1, -1)] = u;
} else if (p !== u) {
  return null;
}
```

- フォルダ名が `[` で始まり `]` で終わっていれば「ここは変わる部分」です。`p.slice(1, -1)` で角括弧を外した `"id"` をキー（名前）にして、URL 側の値（`"abc123"`）を入れます。
- つまり **params のキーの名前は、フォルダの角括弧の中身で決まります。** フォルダが `[bookId]` なら、キーも `bookId` になります（下のつまずきポイント2）。
- 実行結果の `/books/42 → {"id":"42"}` に注目してください。数字に見えても、URL から来る値は **いつも文字列** です。

##### 解説3: `await` を付けた場合と、付け忘れた場合

```ts
const params = Promise.resolve({ id: "42" });
await GoodPage({ params });
await BadPage({ params });
```

- `Promise.resolve(...)` で「中身がすでに決まっている Promise の箱」を作り、Next.js と同じ形で渡しています。
- `GoodPage` は `await params` で箱を開けてから `id` を取り出すので `"42"` が取れます。
- `BadPage` は箱を開けずに `params.id` を読んでいるので `undefined` です。本物のアプリでは、この `undefined` がそのまま `.eq("id", undefined)` に渡り、データが見つからずに 404 になる、という分かりにくい失敗につながります。

> **つまずきポイント1: `await params` を忘れて id が undefined になる。** Next.js 15 以降、`params` は Promise です（Next.js 16 では、`await` せずに読む古い書き方の互換措置も完全に無くなりました）。TypeScript を使っていれば `Property 'id' does not exist on type 'Promise<{ id: string; }>'.`（Promise には id というプロパティが無い）というエラーで気付けます。

**NG（まちがい・抜粋）:**

```tsx
export default async function BookDetailPage({ params }: Props) {
  // Promise の箱を開けずに読んでいる
  const id = params.id;
}
```

**OK（正しい・抜粋）:**

```tsx
export default async function BookDetailPage({ params }: Props) {
  // await で箱を開けてから取り出す
  const { id } = await params;
}
```

> **つまずきポイント2: フォルダ名とキーの名前がずれている。** フォルダを `app/books/[bookId]/page.tsx` という名前で作ったのに、コードでは `{ id }` を取り出そうとすると、`id` は `undefined` になります。params のキーは **フォルダの角括弧の中身と完全に同じ名前** です。

**NG（まちがい）:** フォルダは `app/books/[bookId]/page.tsx`

```tsx
type Props = {
  // フォルダ名は [bookId] なのに id と書いている
  params: Promise<{ id: string }>;
};
```

**OK（正しい）:** フォルダは `app/books/[id]/page.tsx`

```tsx
type Props = {
  // フォルダ名 [id] と同じ名前にそろえる
  params: Promise<{ id: string }>;
};
```

> **ヒント:** Next.js 15.5 以降では、`next dev` や `next build` を実行すると、ルートの形から型を自動で作ってくれる `PageProps` という型が使えます。`export default async function Page(props: PageProps<"/books/[id]">)` のように書くと、`props.params` の中身が `{ id: string }` だと自動で分かるので、フォルダ名とキー名のずれも型エラーで気付けます。この章では仕組みが見えやすいように、`Props` 型を自分で書く方法で進めます。

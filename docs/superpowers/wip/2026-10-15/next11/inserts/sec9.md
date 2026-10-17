### 9-7. 実践ウォークスルー：「確かめ方」つきで並べ替え関数を作る

ここまでの道具（4要素の指示・プランモード・Git・テスト）を全部使って、書籍管理アプリに並べ替え関数を足す流れを通しでやってみます。4-1 で計画した「手順1（sortBooks とテスト）」の部分です。

#### 手順1: ブランチを切り、確かめ方つきで依頼する

```bash
# 作業用ブランチを作って移動する（0.4.1）
git switch -c feature/sort-books
```

```text
src/lib/sortBooks.ts に、書籍の配列を並べ替える関数 sortBooks を作ってください。

目的:
トップページ（/）の書籍一覧を、利用者が好きな順番で見られるようにする準備です。

仕様:
- 並べ方は "title"（タイトルのあいうえお順）、"rating"（評価の高い順。評価なしは最後）、"newest"（登録が新しい順）の3種類
- 引数で受け取った配列そのものは書き換えず、並べ替えた新しい配列を返す

確かめ方:
- 同じフォルダに Vitest のテスト sortBooks.test.ts を作り、3種類の並べ方と「元の配列を書き換えないこと」をテストする
- npx vitest run を実行し、全テストが通るまで直してから報告する
- テストの期待値は変えないこと。期待値が間違っていると思ったら、直す前に理由を説明して止まる

範囲:
- この2ファイル以外は変更しない
```

#### 手順2: Claude Code が作ったコードとテスト

> **▼ このコードがやること（先に日本語で）:** `sortBooks` は、書籍の配列と並べ方（3種類のどれか）を受け取り、並べ替えた**新しい配列**を返す関数です。テストのほうは、3冊ぶんの見本データを使って「タイトル順・評価順・新しい順に正しく並ぶか」「元の配列が書き換わっていないか」を自動で確かめます。

**`src/lib/sortBooks.ts`**

```ts
// 並べ替えに使う項目だけを持つ「書籍の形」
export interface SortableBook {
  title: string;
  rating: number | null;
  created_at: string;
}

// 並べ替えの種類は3通りだけに限定する
export type SortKey = "title" | "rating" | "newest";

// 書籍の配列を受け取り、並べ替えた「新しい配列」を返す
export function sortBooks<T extends SortableBook>(books: T[], key: SortKey): T[] {
  // 元の配列を書き換えないよう、コピーしてから並べ替える
  const copied = [...books];

  if (key === "title") {
    // 日本語の読み順（あいうえお順）で比べる
    return copied.sort((a, b) => a.title.localeCompare(b.title, "ja"));
  }

  if (key === "rating") {
    // 評価なし（null）は 0 とみなし、評価の高い順に並べる
    return copied.sort((a, b) => (b.rating ?? 0) - (a.rating ?? 0));
  }

  // 新しい順: 登録日時を数値（ミリ秒）に直して大きい順に並べる
  return copied.sort(
    (a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime(),
  );
}
```

**`src/lib/sortBooks.test.ts`**

```ts
// Vitest のテスト用の関数を読み込む
import { describe, expect, it } from "vitest";
// テストしたい関数を読み込む
import { sortBooks } from "./sortBooks";

// テストで使う3冊ぶんのデータ
const books = [
  { title: "すいみん学", rating: 3, created_at: "2026-01-10T09:00:00Z" },
  { title: "あめの本", rating: null, created_at: "2026-03-05T09:00:00Z" },
  { title: "かぜの歌", rating: 5, created_at: "2026-02-20T09:00:00Z" },
];

describe("sortBooks", () => {
  it("タイトルのあいうえお順に並ぶ", () => {
    const result = sortBooks(books, "title");
    expect(result.map((b) => b.title)).toEqual(["あめの本", "かぜの歌", "すいみん学"]);
  });

  it("評価の高い順に並び、評価なしは最後になる", () => {
    const result = sortBooks(books, "rating");
    expect(result.map((b) => b.rating)).toEqual([5, 3, null]);
  });

  it("登録が新しい順に並ぶ", () => {
    const result = sortBooks(books, "newest");
    expect(result[0].title).toBe("あめの本");
  });

  it("元の配列は書き換えない", () => {
    sortBooks(books, "title");
    expect(books[0].title).toBe("すいみん学");
  });
});
```

**▼ 実行結果（`npx vitest run`。Vitest 3.2.7、パスの一部は伏せています）**

```text

 RUN  v3.2.7 C:/…/my-books-app

 ✓ src/lib/sortBooks.test.ts (4 tests) 9ms

 Test Files  1 passed (1)
      Tests  4 passed (4)
   Start at  11:29:28
   Duration  1.18s (transform 127ms, setup 0ms, collect 127ms, tests 9ms, environment 0ms, prepare 637ms)
```

`✓` が付いて「4 passed（4つ成功）」と出れば、Claude Code の報告どおりに動いていることを**あなた自身が**確かめられたことになります。テストを動かす準備（Vitest のインストールと設定）は 10-1 を参照してください。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 型で「受け取れる値」をしぼる

```ts
export type SortKey = "title" | "rating" | "newest";
```

- `SortKey` は「この3つの文字列のどれか」しか受け付けない型です。`sortBooks(books, "price")` のように存在しない並べ方を渡すと、型チェック（`npx tsc --noEmit`）でエラーになります。
- `<T extends SortableBook>` は「`title`・`rating`・`created_at` を持っていれば、ほかの項目（`author` や `memo` など）があってもよい」という意味です。書籍アプリの `Book` 型をそのまま渡せます。

##### 解説2: `[...books]` で配列をコピーしてから並べ替える

```ts
const copied = [...books];
```

- `...`（スプレッド構文）で、配列の中身を新しい配列に並べ直してコピーしています。
- JavaScript の `sort` は**元の配列そのものを並べ替えてしまう**ので、コピーしないと、呼び出した側のデータまで順番が変わってしまいます（下の「よくある展開」を参照）。

##### 解説3: 3種類の並べ方

```ts
return copied.sort((a, b) => a.title.localeCompare(b.title, "ja"));
```

- `localeCompare(…, "ja")` は、日本語の読み順で2つの文字列を比べる関数です。
- 評価順では `b.rating ?? 0` のように、評価なし（`null`）を 0 とみなして最後に回しています。`??` は「左が `null` か `undefined` なら右を使う」という演算子です。

##### 解説4: テストは「見本データ」と「期待する結果」のペア

```ts
it("元の配列は書き換えない", () => {
  sortBooks(books, "title");
  expect(books[0].title).toBe("すいみん学");
});
```

- `it("説明", () => { … })` が1つのテストです。`expect(実際の値).toBe(期待する値)` で、結果が期待どおりかを確かめます。
- 「元の配列を書き換えない」のように、**仕様に書いた約束ごと**をテストにしておくと、AI が後でコードを書き換えても約束が守られているかを自動で確かめられます。

#### 手順3（よくある展開）: 最初の版でテストが落ちたら

実際の作業では、Claude Code の最初の版がテストに落ちることもよくあります。たとえば、コピーを忘れて次のように書いてしまった場合です。

**NG（最初の版。`sortBooks.ts` の抜粋）:** 配列をコピーせずに並べ替えている

```ts
// （最初の版）配列をそのまま並べ替えている
const copied = books;
```

**▼ 実行結果（`npx vitest run`。パスの一部は伏せています）**

```text

 RUN  v3.2.7 C:/…/my-books-app

 ❯ src/lib/sortBooks.test.ts (4 tests | 1 failed) 13ms
   ✓ sortBooks > タイトルのあいうえお順に並ぶ 7ms
   ✓ sortBooks > 評価の高い順に並び、評価なしは最後になる 0ms
   ✓ sortBooks > 登録が新しい順に並ぶ 1ms
   × sortBooks > 元の配列は書き換えない 4ms
     → expected 'あめの本' to be 'すいみん学' // Object.is equality

⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  src/lib/sortBooks.test.ts > sortBooks > 元の配列は書き換えない
AssertionError: expected 'あめの本' to be 'すいみん学' // Object.is equality

Expected: "すいみん学"
Received: "あめの本"

 ❯ src/lib/sortBooks.test.ts:31:28
     29|   it("元の配列は書き換えない", () => {
     30|     sortBooks(books, "title");
     31|     expect(books[0].title).toBe("すいみん学");
       |                            ^
     32|   });
     33| });

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/1]⎯


 Test Files  1 failed (1)
      Tests  1 failed | 3 passed (4)
   Start at  11:29:34
   Duration  933ms (transform 52ms, setup 0ms, collect 59ms, tests 13ms, environment 0ms, prepare 84ms)
```

**OK（直した版）:** コピーしてから並べ替える

```ts
// 元の配列を書き換えないよう、コピーしてから並べ替える
const copied = [...books];
```

- `Expected`（期待した値）が「すいみん学」、`Received`（実際の値）が「あめの本」です。タイトル順に並べ替えたせいで、元の `books` の先頭まで入れ替わってしまったことが分かります。
- 指示に「全テストが通るまで直してから報告する」と書いておいたので、Claude Code はこの失敗を自分で読み、`[...books]` に直して再実行し、成功してから報告してきます。**確かめ方を渡しておくと、AI が自分のミスを自分で見つけて直せる**、というのがこの例のポイントです。

#### 手順4: 差分を読んでコミットする

最後に、0.4.1 の手順で `git status` と `git diff` を確認し、頼んだ2ファイルだけをコミットします。次の「画面につなぐ」作業（`SortSelect` と `page.tsx`）は、また別の指示・別のコミットにします。

### 9-8. AI の出力を鵜呑みにしない：検証チェックリスト

テストが通っても、それだけで安心はできません。空港の保安検査のように、AI の変更は**いくつもの関所**を通してから採用します。どの関所も、それぞれ別の種類の間違いを見つけるのが得意です。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 346" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-11-13" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="230" y="8" width="260" height="32" rx="16" fill="#fff7ed" stroke="#f97316" stroke-width="1.5"/>
<text x="360" y="29" text-anchor="middle" font-size="13" font-weight="700" fill="#c2410c">AI が作った変更（未コミット）</text>
<rect x="120" y="58" width="400" height="46" rx="8" fill="#f8fafc" stroke="#3b82f6" stroke-width="1.5"/>
<rect x="120" y="58" width="80" height="46" rx="8" fill="#3b82f6"/>
<text x="160" y="86" text-anchor="middle" font-size="13" font-weight="700" fill="#ffffff">関所 1</text>
<text x="214" y="77" font-size="13" font-weight="700" fill="#0f172a">目で読む（差分レビュー）</text>
<text x="214" y="95" font-size="12" fill="#475569">git status・git diff。頼んでいない変更はないか</text>
<rect x="120" y="120" width="400" height="46" rx="8" fill="#f8fafc" stroke="#1e40af" stroke-width="1.5"/>
<rect x="120" y="120" width="80" height="46" rx="8" fill="#1e40af"/>
<text x="160" y="148" text-anchor="middle" font-size="13" font-weight="700" fill="#ffffff">関所 2</text>
<text x="214" y="139" font-size="13" font-weight="700" fill="#0f172a">機械に調べさせる</text>
<text x="214" y="157" font-size="12" fill="#475569">npx tsc --noEmit（型）・npm run lint</text>
<rect x="120" y="182" width="400" height="46" rx="8" fill="#f8fafc" stroke="#f97316" stroke-width="1.5"/>
<rect x="120" y="182" width="80" height="46" rx="8" fill="#f97316"/>
<text x="160" y="210" text-anchor="middle" font-size="13" font-weight="700" fill="#ffffff">関所 3</text>
<text x="214" y="201" font-size="13" font-weight="700" fill="#0f172a">動かして確かめる</text>
<text x="214" y="219" font-size="12" fill="#475569">npm run dev → ブラウザで実際に操作する</text>
<rect x="120" y="244" width="400" height="46" rx="8" fill="#f8fafc" stroke="#10b981" stroke-width="1.5"/>
<rect x="120" y="244" width="80" height="46" rx="8" fill="#10b981"/>
<text x="160" y="272" text-anchor="middle" font-size="13" font-weight="700" fill="#ffffff">関所 4</text>
<text x="214" y="263" font-size="13" font-weight="700" fill="#0f172a">テストで確かめる</text>
<text x="214" y="281" font-size="12" fill="#475569">npx vitest run（テストは書き換えさせない）</text>
<rect x="230" y="306" width="260" height="32" rx="16" fill="#ecfdf5" stroke="#10b981" stroke-width="1.5"/>
<text x="360" y="327" text-anchor="middle" font-size="13" font-weight="700" fill="#047857">すべて OK → 採用してコミット</text>
<line x1="320" y1="40" x2="320" y2="56" stroke="#475569" stroke-width="1.5" marker-end="url(#ar-11-13)"/>
<line x1="320" y1="104" x2="320" y2="118" stroke="#475569" stroke-width="1.5" marker-end="url(#ar-11-13)"/>
<line x1="320" y1="166" x2="320" y2="180" stroke="#475569" stroke-width="1.5" marker-end="url(#ar-11-13)"/>
<line x1="320" y1="228" x2="320" y2="242" stroke="#475569" stroke-width="1.5" marker-end="url(#ar-11-13)"/>
<line x1="320" y1="290" x2="320" y2="304" stroke="#475569" stroke-width="1.5" marker-end="url(#ar-11-13)"/>
<text x="330" y="116" font-size="12" font-weight="700" fill="#047857">OK</text>
<text x="330" y="178" font-size="12" font-weight="700" fill="#047857">OK</text>
<text x="330" y="240" font-size="12" font-weight="700" fill="#047857">OK</text>
<text x="330" y="302" font-size="12" font-weight="700" fill="#047857">OK</text>
<line x1="520" y1="81" x2="552" y2="81" stroke="#c2410c" stroke-width="1.5"/>
<line x1="520" y1="143" x2="552" y2="143" stroke="#c2410c" stroke-width="1.5"/>
<line x1="520" y1="205" x2="552" y2="205" stroke="#c2410c" stroke-width="1.5"/>
<line x1="520" y1="267" x2="552" y2="267" stroke="#c2410c" stroke-width="1.5"/>
<line x1="552" y1="81" x2="552" y2="267" stroke="#c2410c" stroke-width="1.5"/>
<line x1="552" y1="175" x2="572" y2="175" stroke="#c2410c" stroke-width="1.5" marker-end="url(#ar-11-13)"/>
<text x="540" y="72" font-size="12" font-weight="700" fill="#c2410c">NG</text>
<rect x="574" y="118" width="136" height="114" rx="8" fill="#fff7ed" stroke="#c2410c" stroke-width="1.5" stroke-dasharray="5 4"/>
<text x="642" y="143" text-anchor="middle" font-size="12" fill="#c2410c">エラー全文と</text>
<text x="642" y="165" text-anchor="middle" font-size="12" fill="#c2410c">期待する動きを</text>
<text x="642" y="187" text-anchor="middle" font-size="12" fill="#c2410c">伝えて直させ、</text>
<text x="642" y="209" text-anchor="middle" font-size="12" fill="#c2410c">関所1からやり直す</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図11-13: AI の変更を採用するまでの4つの関所。どこかで NG なら、直してもらって最初からやり直します</div>
</div>

**▼ AI の出力を採用する前のチェックリスト**

1. □ `git status` に、頼んでいないファイルが混ざっていない（関所1）
2. □ `git diff` の全行を読み、説明できない行がない。分からない行は「この行は何をしていますか？」と Claude に聞いて理解してから進む（関所1）
3. □ 実在しない関数・オプション・パッケージを使っていない。怪しいものは公式ドキュメントや型チェックで確かめる（関所1・2）
4. □ `npx tsc --noEmit` と `npm run lint` がエラーなく通る（関所2）
5. □ `npm run dev` で実際に画面を操作した。成功するとき・失敗するとき・データが0件のとき、の3パターンを試した（関所3）
6. □ `npx vitest run` が通る。**テストの期待値が勝手に書き換えられていない**（関所4）
7. □ API キーやパスワードがコードに直接書かれていない。`.env.local` に置くべき値が混ざっていない
8. □ Claude の「完了しました」という報告の内容と、実際の差分が一致している

> **ヒント:** チェックリストの2番は、とても大事です。AI が書いたコードを説明できないまま採用すると、次に不具合が起きたとき、あなたも AI も原因を推測するしかなくなります。「動いたから OK」ではなく「説明できるから OK」を合格ラインにしましょう。

#### 型チェックで「存在しないメソッド」を見つける

> **▼ このコードがやること（先に日本語で）:** 書籍をタイトル順に取り出す関数です。NG 版は、AI が別のライブラリ（Prisma など）の書き方と混同して、Supabase には**存在しない** `findMany` というメソッドを使っています。OK 版は Supabase の正しい書き方（`select` で取り出し、`order` で並べ替え）です。型チェックの結果を比べると、ハルシネーションが機械的に見つかることが分かります。

**NG（`src/lib/getBooks.ts`。AI が提案した版）**

```ts
// サーバー用の Supabase クライアントを作る関数を読み込む
import { createClient } from "@/lib/supabase/server";

// 書籍をタイトル順に取得する
export async function getBooksSortedByTitle() {
  const supabase = await createClient();
  // AI が提案したコード: findMany は supabase-js には存在しない
  const { data, error } = await supabase
    .from("books")
    .findMany({ orderBy: { title: "asc" } });
  if (error) throw error;
  return data;
}
```

**▼ 実行結果（`npx tsc --noEmit`。supabase-js 2.117.3 / TypeScript 7.0.2）**

```text
src/lib/getBooks.ts(10,6): error TS2339: Property 'findMany' does not exist on type 'PostgrestQueryBuilder<any, any, any, "books", unknown>'.
```

**OK（直した版）**

```ts
// サーバー用の Supabase クライアントを作る関数を読み込む
import { createClient } from "@/lib/supabase/server";

// 書籍をタイトル順に取得する
export async function getBooksSortedByTitle() {
  const supabase = await createClient();
  // supabase-js の正しい書き方: select で取得し、order で並べ替える
  const { data, error } = await supabase
    .from("books")
    .select("*")
    .order("title", { ascending: true });
  if (error) throw error;
  return data;
}
```

**▼ 実行結果（`npx tsc --noEmit`）**

```text
（何も表示されずに終了）
```

（型チェックのためだけに、`@/lib/supabase/server` を supabase-js で作った簡易版に置き換えた練習用フォルダで実行した結果です。第7章の本物の `createClient` でも、`findMany` については同じエラーになります。）

#### ▼ コードを1つずつ分解して解説

##### 解説1: エラーメッセージの読み方

```text
src/lib/getBooks.ts(10,6): error TS2339: Property 'findMany' does not exist on type 'PostgrestQueryBuilder<…>'.
```

- `(10,6)` は「10行目の6文字目」、`TS2339` は TypeScript のエラー番号です。
- 「`findMany` というプロパティ（メソッド）は、`PostgrestQueryBuilder`（Supabase のクエリを組み立てる部品）には存在しない」という意味です。つまり **AI が存在しない書き方をした**ことが、型チェックだけで分かります。

##### 解説2: 正しい書き方は公式ドキュメントで確かめる

```ts
.select("*")
.order("title", { ascending: true });
```

- Supabase では、`from("テーブル名")` の後に `select`（取り出す）、`order`（並べ替え）、`eq`（条件）などをつなげて書きます（第7章で `order("created_at", { ascending: false })` を使いました）。
- AI に「findMany は存在しないとエラーが出ました。supabase-js の公式ドキュメントに沿った書き方で直してください」とエラー全文を貼って頼むのが近道です。

> **つまずきポイント⑩: AI が「それっぽい」存在しないメソッドを使う**
> AI は、似たライブラリ（Prisma、Drizzle など）の書き方を混ぜてしまうことがあります。見た目は自然なので、読んだだけでは気づきにくいのが厄介な点です。「型チェック（`npx tsc --noEmit`）を通す」を確かめ方に入れておけば、存在しないメソッドは上のように機械的に見つかります。

**NG（存在しないメソッドを信じる）:**

```ts
// Prisma 風の書き方が混ざっている
const { data } = await supabase.from("books").findMany({ orderBy: { title: "asc" } });
```

**OK（supabase-js の書き方）:**

```ts
// select で取得し、order で並べ替える
const { data } = await supabase.from("books").select("*").order("title", { ascending: true });
```

> **つまずきポイント⑪: 「テストが通るようにして」と頼み、テストのほうを書き換えられる**
> テストが失敗したときに「テストが通るようにして」とだけ頼むと、AI がコードではなく**テストの期待値**を書き換えて、無理やり成功させることがあります。こうなるとテストは何も守ってくれません。9-7 の指示のように「テストの期待値は変えない」と書き、テストファイルの差分も必ず読みましょう。

**NG（テストを書き換えて通した差分）:** 9-7 の「最初の版」のバグが残ったまま、テストが「成功」してしまいます。

```diff
diff --git a/src/lib/sortBooks.test.ts b/src/lib/sortBooks.test.ts
index ae01d85..6283a3c 100644
--- a/src/lib/sortBooks.test.ts
+++ b/src/lib/sortBooks.test.ts
@@ -26,8 +26,8 @@ describe("sortBooks", () => {
     expect(result[0].title).toBe("あめの本");
   });
 
-  it("元の配列は書き換えない", () => {
+  it("並べ替え後は先頭が変わる", () => {
     sortBooks(books, "title");
-    expect(books[0].title).toBe("すいみん学");
+    expect(books[0].title).toBe("あめの本");
   });
 });
```

**OK（期待値を守らせる指示）:**

```text
npx vitest run で「元の配列は書き換えない」のテストが失敗しています。
テストファイル（sortBooks.test.ts）は変更せず、sortBooks.ts のほうを直してください。
テストの期待値が間違っていると思う場合は、直す前に理由を説明して止まってください。
```


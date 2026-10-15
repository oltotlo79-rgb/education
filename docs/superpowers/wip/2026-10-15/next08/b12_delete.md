### 3-3. `.eq("id", id)` を付け忘れると何が起きるか

`DeleteButton` のコメントにも「`.eq()` を付けずに `.delete()` だけを実行すると全データが削除される」と書きました。ここでは、それがどういうことかを図とコードで確かめます。

> **たとえ話:** `.delete()` は「捨ててください」、`.eq("id", "b2")` は「カード番号 b2 のものを」という指定です。指定なしで「捨ててください」とだけ言うと、司書さんは **棚のカードを全部** 捨ててよいと受け取ります。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="340" height="34" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="180" y="33" text-anchor="middle" font-size="14" font-weight="700" fill="#c2410c">NG: .delete() だけ（条件なし）</text>
<rect x="370" y="10" width="340" height="34" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="540" y="33" text-anchor="middle" font-size="14" font-weight="700" fill="#047857">OK: .eq("id", "b2") で1件に絞る</text>
<text x="20" y="66" font-size="12" fill="#1e293b" font-family="Consolas, monospace">supabase.from("books").delete()</text>
<text x="380" y="66" font-size="12" fill="#1e293b" font-family="Consolas, monospace">.delete().eq("id", "b2")</text>
<rect x="20" y="78" width="320" height="26" fill="#f1f5f9" stroke="#cbd5e1"/>
<text x="34" y="96" font-size="12" font-weight="700" fill="#475569">id</text>
<text x="84" y="96" font-size="12" font-weight="700" fill="#475569">title</text>
<text x="250" y="96" font-size="12" font-weight="700" fill="#475569">status</text>
<rect x="20" y="104" width="320" height="32" fill="#fff7ed" stroke="#fdba74"/>
<text x="34" y="125" font-size="12" fill="#c2410c" font-family="Consolas, monospace">a1</text>
<text x="84" y="125" font-size="12" fill="#c2410c">React実践ガイド</text>
<text x="250" y="125" font-size="12" fill="#c2410c" font-family="Consolas, monospace">reading</text>
<line x1="28" y1="120" x2="332" y2="120" stroke="#c2410c" stroke-width="2"/>
<rect x="20" y="136" width="320" height="32" fill="#fff7ed" stroke="#fdba74"/>
<text x="34" y="157" font-size="12" fill="#c2410c" font-family="Consolas, monospace">b2</text>
<text x="84" y="157" font-size="12" fill="#c2410c">TypeScript入門</text>
<text x="250" y="157" font-size="12" fill="#c2410c" font-family="Consolas, monospace">unread</text>
<line x1="28" y1="152" x2="332" y2="152" stroke="#c2410c" stroke-width="2"/>
<rect x="20" y="168" width="320" height="32" fill="#fff7ed" stroke="#fdba74"/>
<text x="34" y="189" font-size="12" fill="#c2410c" font-family="Consolas, monospace">c3</text>
<text x="84" y="189" font-size="12" fill="#c2410c">Next.js実践</text>
<text x="250" y="189" font-size="12" fill="#c2410c" font-family="Consolas, monospace">finished</text>
<line x1="28" y1="184" x2="332" y2="184" stroke="#c2410c" stroke-width="2"/>
<rect x="20" y="200" width="320" height="32" fill="#fff7ed" stroke="#fdba74"/>
<text x="34" y="221" font-size="12" fill="#c2410c" font-family="Consolas, monospace">d4</text>
<text x="84" y="221" font-size="12" fill="#c2410c">SQL入門</text>
<text x="250" y="221" font-size="12" fill="#c2410c" font-family="Consolas, monospace">reading</text>
<line x1="28" y1="216" x2="332" y2="216" stroke="#c2410c" stroke-width="2"/>
<text x="20" y="258" font-size="12" font-weight="700" fill="#c2410c" font-family="Consolas, monospace">DELETE FROM books;</text>
<text x="20" y="280" font-size="12" fill="#c2410c">→ 4件すべてが「消す対象」になる</text>
<rect x="380" y="78" width="320" height="26" fill="#f1f5f9" stroke="#cbd5e1"/>
<text x="394" y="96" font-size="12" font-weight="700" fill="#475569">id</text>
<text x="444" y="96" font-size="12" font-weight="700" fill="#475569">title</text>
<text x="610" y="96" font-size="12" font-weight="700" fill="#475569">status</text>
<rect x="380" y="104" width="320" height="32" fill="#ffffff" stroke="#e2e8f0"/>
<text x="394" y="125" font-size="12" fill="#1e293b" font-family="Consolas, monospace">a1</text>
<text x="444" y="125" font-size="12" fill="#1e293b">React実践ガイド</text>
<text x="610" y="125" font-size="12" fill="#1e293b" font-family="Consolas, monospace">reading</text>
<rect x="380" y="136" width="320" height="32" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="394" y="157" font-size="12" fill="#c2410c" font-family="Consolas, monospace">b2</text>
<text x="444" y="157" font-size="12" fill="#c2410c">TypeScript入門</text>
<text x="610" y="157" font-size="12" fill="#c2410c" font-family="Consolas, monospace">unread</text>
<line x1="388" y1="152" x2="692" y2="152" stroke="#c2410c" stroke-width="2"/>
<rect x="380" y="168" width="320" height="32" fill="#ffffff" stroke="#e2e8f0"/>
<text x="394" y="189" font-size="12" fill="#1e293b" font-family="Consolas, monospace">c3</text>
<text x="444" y="189" font-size="12" fill="#1e293b">Next.js実践</text>
<text x="610" y="189" font-size="12" fill="#1e293b" font-family="Consolas, monospace">finished</text>
<rect x="380" y="200" width="320" height="32" fill="#ffffff" stroke="#e2e8f0"/>
<text x="394" y="221" font-size="12" fill="#1e293b" font-family="Consolas, monospace">d4</text>
<text x="444" y="221" font-size="12" fill="#1e293b">SQL入門</text>
<text x="610" y="221" font-size="12" fill="#1e293b" font-family="Consolas, monospace">reading</text>
<text x="380" y="258" font-size="12" font-weight="700" fill="#047857" font-family="Consolas, monospace">DELETE FROM books WHERE id = 'b2';</text>
<text x="380" y="280" font-size="12" fill="#047857">→ 消えるのは b2 の1件だけ</text>
<text x="360" y="318" text-anchor="middle" font-size="12" fill="#475569">※ Supabase の API には条件なしの DELETE を止める安全装置もあるが、頼らずに必ず .eq() を付ける</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-11: Before/After ― 条件なしの削除（左）と、id で1件に絞った削除（右）。取り消し線の行が消える対象</div>
</div>

#### 動かして確かめる: 条件の付け方で、消える行がどう変わるか

> **▼ このコードがやること（先に日本語で）:** 4冊の本が入った配列を「books テーブル」に見立てて、`.delete()` の条件を「id で絞る（正しい）」「付け忘れる」「id ではなく status で絞ってしまう」の3通りで実行し、どの行が消えて、どの行が残るかを表示します。データベースには一切触らないので、安心して試せます。

**ファイル: `delete-where.ts`（練習用）**

```ts
// .delete() の条件を「正しく付けた」「付け忘れた」「間違えた」ときに、
// どの行が消えるのかを配列で真似して確かめる練習用コード
// （id は読みやすさのため短くしています。本物は UUID です）

type Book = {
  id: string;
  title: string;
  status: string;
};

type DeleteResult = {
  deleted: Book[];
  remaining: Book[];
};

// 毎回同じ状態から試せるよう、テーブルの中身を作り直す
function makeTable(): Book[] {
  return [
    { id: "a1", title: "React実践ガイド", status: "reading" },
    { id: "b2", title: "TypeScript入門", status: "unread" },
    { id: "c3", title: "Next.js実践", status: "finished" },
    { id: "d4", title: "SQL入門", status: "reading" },
  ];
}

// 条件 where に当てはまる行を消す。where が無いと「全行が当てはまる」
function deleteRows(
  table: Book[],
  where?: (book: Book) => boolean,
): DeleteResult {
  const match = where ?? (() => true);
  return {
    deleted: table.filter((book) => match(book)),
    remaining: table.filter((book) => !match(book)),
  };
}

// 結果を見やすく表示する
function report(label: string, result: DeleteResult) {
  const deleted = result.deleted.map((book) => book.title);
  const remaining = result.remaining.map((book) => book.title);
  console.log(label);
  console.log(`  消えた(${deleted.length}件): ${deleted.join(", ") || "なし"}`);
  console.log(`  残った(${remaining.length}件): ${remaining.join(", ") || "なし"}`);
}

// OK: .delete().eq("id", "b2") に相当
report(
  'OK  .eq("id", "b2")',
  deleteRows(makeTable(), (book) => book.id === "b2"),
);

// NG1: .eq() を付け忘れた .delete() に相当（SQL の DELETE FROM books;）
report("NG1 条件なし", deleteRows(makeTable()));

// NG2: id ではなく status で絞ってしまった
report(
  'NG2 .eq("status", "reading")',
  deleteRows(makeTable(), (book) => book.status === "reading"),
);
```

**▼ 実行結果**（`node delete-where.ts`）

```text
OK  .eq("id", "b2")
  消えた(1件): TypeScript入門
  残った(3件): React実践ガイド, Next.js実践, SQL入門
NG1 条件なし
  消えた(4件): React実践ガイド, TypeScript入門, Next.js実践, SQL入門
  残った(0件): なし
NG2 .eq("status", "reading")
  消えた(2件): React実践ガイド, SQL入門
  残った(2件): TypeScript入門, Next.js実践
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 条件が無いと「全行が当てはまる」

```ts
const match = where ?? (() => true);
```

- `where`（条件の関数）が渡されなかったときは、`() => true`（どの行にも「当てはまる」と答える関数）を使います。
- これは SQL の `DELETE FROM books;` と同じ意味で、WHERE が無い DELETE は **テーブルの全行** が対象です。実行結果の NG1 では4件すべてが消えました。

##### 解説2: `filter` で「消える行」と「残る行」に分ける

```ts
deleted: table.filter((book) => match(book)),
remaining: table.filter((book) => !match(book)),
```

- `filter` は、条件が `true` になった要素だけを集めた新しい配列を作ります。`!match(book)`（当てはまらない）で集めたものが「残る行」です。

##### 解説3: 条件を「付けた」だけでは安心できない

```ts
deleteRows(makeTable(), (book) => book.status === "reading"),
```

- NG2 は条件を付けていますが、`id` ではなく `status` で絞っています。「読書中」の本は2冊あるので、**2冊とも** 消えてしまいました。
- 1冊だけを消したいときは、**1行に1つしか無い値（主キーの `id`）** で絞るのが鉄則です。

> **注意:** Supabase の API（supabase-js からの呼び出し）では、WHERE の無い UPDATE / DELETE を拒否する安全装置（`safeupdate`）が働くように設定されていることが多く、その場合は条件なしの `.delete()` は `DELETE requires a WHERE clause` というエラーで止まります。ただし、NG2 のような **条件の書き間違い** は止めてくれませんし、SQL Editor で直接実行した `DELETE FROM books;` も止まりません。安全装置に頼らず、UPDATE と DELETE には必ず `.eq("id", id)` を付けてください。

> **つまずきポイント6: UPDATE / DELETE の `.eq("id", ...)` を付け忘れる・別の列で絞ってしまう。** 書き換えや削除は「どの行に対してか」がすべてです。`.update(...)` や `.delete()` を書いたら、**次の行に必ず `.eq("id", ...)` を書く** と手に覚えさせましょう。

**NG（まちがい・抜粋）:**

```tsx
const { error } = await supabase
  .from("books")
  // 条件が無いので、全行が対象になってしまう
  .delete();
```

**OK（正しい・抜粋）:**

```tsx
const { error } = await supabase
  .from("books")
  .delete()
  // 主キーの id で1件だけに絞る
  .eq("id", bookId);
```

### 3-4. 「エラーは出ないのに消えていない」を見逃さない

もう1つ、削除と更新でよく出会う不思議な現象があります。**エラーは出ないし一覧にも戻るのに、データが消えていない（変わっていない）** というものです。原因の多くは次の2つです。

- **RLS で許可されていない**: DELETE（や UPDATE）を許すポリシーが無いと、Supabase は「見えない行は対象外」として扱い、**0件を削除して成功** という結果を返します。エラーにはなりません（7-2 も参照）。
- **id がずれている**: 渡した `bookId` が別の値になっていると、当てはまる行が0件のまま成功します。

どちらも `error` は `null` なので、`if (error)` だけでは気付けません。そこで `.select()` を付けて「実際に消えた行」を返してもらい、その件数を確かめます。

#### 動かして確かめる: 返ってきた行の数で成功を判定する

> **▼ このコードがやること（先に日本語で）:** `.delete().eq("id", id).select()` が返す `{ data, error }` の形を3パターン用意し、「本当に消えたのか」を判定する関数に通します。`error` が無くても、`data` が空配列なら失敗として扱うのがポイントです。

**ファイル: `affected.ts`（練習用）**

```ts
// .delete().eq("id", id).select() の返り値 { data, error } から、
// 「本当に1件消えたのか」を判定する練習用コード
// （返り値の形だけを真似しています）

type Book = {
  id: string;
  title: string;
};

type MutationResult = {
  data: Book[] | null;
  error: { code: string } | null;
};

// 返り値を見て、画面に出すメッセージを決める
function judge(result: MutationResult): string {
  // ① Supabase がエラーを返した
  if (result.error) {
    return `失敗: エラーコード ${result.error.code}`;
  }
  // ② エラーは無いが 1 件も消えていない（id 違い・RLS で見えない行など）
  if (!result.data || result.data.length === 0) {
    return "失敗: 対象の行が 0 件でした（id や RLS を確認）";
  }
  // ③ ちゃんと消えた
  return `成功: 「${result.data[0].title}」を削除しました`;
}

// よくある 3 つの返り値
const cases: MutationResult[] = [
  { data: [{ id: "b2", title: "TypeScript入門" }], error: null },
  { data: [], error: null },
  { data: null, error: { code: "42501" } },
];

for (const result of cases) {
  console.log(judge(result));
}
```

**▼ 実行結果**（`node affected.ts`）

```text
成功: 「TypeScript入門」を削除しました
失敗: 対象の行が 0 件でした（id や RLS を確認）
失敗: エラーコード 42501
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: まず `error` を見る

```ts
if (result.error) {
  return `失敗: エラーコード ${result.error.code}`;
}
```

- 通信の失敗や、権限エラー（`42501` は PostgreSQL の「権限が足りない」エラーコード）など、はっきりした失敗はここで捕まえます。今の `DeleteButton` がやっている `if (error) { throw error; }` と同じ確認です。

##### 解説2: 次に「何件だったか」を見る

```ts
if (!result.data || result.data.length === 0) {
  return "失敗: 対象の行が 0 件でした（id や RLS を確認）";
}
```

- `.select()` を付けると、`data` には **実際に削除された行の配列** が入ります。空配列（`[]`）なら、1件も消えていません。
- 2つ目の実行結果のように、`error` が `null` でもここで失敗だと判定できます。これが「エラーは出ないのに消えていない」の正体を見破る方法です。

> **つまずきポイント7: `error` が無いことだけで「成功」と判断してしまう。** RLS で許可されていない DELETE / UPDATE は、エラーではなく「0件成功」になります。大事な操作では `.select()` で結果の行を返してもらい、件数まで確かめましょう（`DeleteButton` の try の中を、次のように書き換えられます）。

**NG（まちがい・抜粋）:**

```tsx
const { error } = await supabase.from("books").delete().eq("id", bookId);

// 0件しか消えていなくても、error は null なので「成功」扱いになる
if (error) {
  throw error;
}
router.push("/books");
```

**OK（正しい・抜粋）:**

```tsx
const { data, error } = await supabase
  .from("books")
  .delete()
  .eq("id", bookId)
  // 削除した行を返してもらう
  .select();

if (error) {
  throw error;
}
// エラーが無くても 0 件なら、消えていないので失敗として扱う
if (!data || data.length === 0) {
  throw new Error("削除できませんでした（対象が無いか、権限がありません）");
}
router.push("/books");
```

> **ヒント:** `.select()` で削除した行を返してもらうには、その行を **読む（SELECT）権限** も必要です。開発用の「全部許可」ポリシー（7-2 の `FOR ALL`）なら読み取りも許可されているので、そのまま動きます。

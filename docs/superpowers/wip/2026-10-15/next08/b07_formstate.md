#### 動かして確かめる: `handleChange` が state を作り替える様子

> **▼ このコードがやること（先に日本語で）:** `BookForm` の `handleChange` がやっている「1つの欄だけを差し替えた **新しいオブジェクト** を作る」処理を、React を使わずに実行して中身を確かめます。あわせて、元のオブジェクトを直接書き換えた場合との違いと、解説2で出てきた `??` と `||` の違いも表示します。

**ファイル: `form-state.ts`（練習用）**

```ts
// BookForm の handleChange が state をどう作り替えるのかを、
// React を使わずに確かめる練習用コード

type BookFormData = {
  title: string;
  author: string;
  publisher: string;
  published_date: string;
  rating: number | null;
  status: string;
  memo: string;
};

// 編集ページから渡ってくる既存データ（initialData）の例
const initialData: BookFormData = {
  title: "React実践ガイド",
  author: "山田太郎",
  publisher: "技術評論社",
  published_date: "2024-01-15",
  rating: 4,
  status: "reading",
  memo: "",
};

// handleChange と同じく「1項目だけ差し替えた新しいオブジェクト」を作る
function applyChange(
  prev: BookFormData,
  name: string,
  value: string,
): BookFormData {
  return {
    ...prev,
    [name]: value,
  };
}

// 「ステータス欄を読了に変えた」という入力を再現する
const next = applyChange(initialData, "status", "finished");
console.log("変更前の status:", initialData.status);
console.log("変更後の status:", next.status);
console.log("title はそのまま:", next.title);
// React は「前と別のオブジェクトか」で変化を見分ける
console.log("新しいオブジェクトになった?:", !Object.is(initialData, next));

// NG: 元のオブジェクトを直接書き換えると「同じオブジェクト」のまま
const sameObject = initialData;
sameObject.status = "finished";
console.log(
  "直接書き換えたら別物になった?:",
  !Object.is(initialData, sameObject),
);

// ?? と || の違い（左がどんな値のときに右へ置き換わるか）
const samples = [null, undefined, "", 0];
for (const v of samples) {
  const label = String(JSON.stringify(v)).padEnd(9);
  const withNullish = JSON.stringify(v ?? "既定値");
  const withOr = JSON.stringify(v || "既定値");
  console.log(`${label} ?? → ${withNullish} / || → ${withOr}`);
}
```

**▼ 実行結果**（`node form-state.ts`）

```text
変更前の status: reading
変更後の status: finished
title はそのまま: React実践ガイド
新しいオブジェクトになった?: true
直接書き換えたら別物になった?: false
null      ?? → "既定値" / || → "既定値"
undefined ?? → "既定値" / || → "既定値"
""        ?? → "" / || → "既定値"
0         ?? → 0 / || → "既定値"
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `...prev` で全部コピーし、`[name]` の欄だけ上書きする

```ts
return {
  ...prev,
  [name]: value,
};
```

- `...prev` で7つの欄をすべてコピーしてから、`[name]: value` で1つだけ上書きします。`name` が `"status"` なら `status: "finished"` と書いたのと同じです。
- 実行結果で `title はそのまま: React実践ガイド` と出ているとおり、上書きしなかった欄は元の値が残ります。

##### 解説2: React は「別のオブジェクトになったか」で画面を描き直す

```ts
console.log("新しいオブジェクトになった?:", !Object.is(initialData, next));
```

- `Object.is(a, b)` は「a と b がまったく同じもの（同じ箱）か」を調べます。React も state が変わったかどうかを、これと同じ考え方で判断しています。
- `applyChange` は新しいオブジェクトを返すので `true`（別物になった）＝ React は「変わった」と気付いて画面を描き直します。

##### 解説3: 直接書き換えると「同じ箱のまま」なので気付いてもらえない

```ts
const sameObject = initialData;
sameObject.status = "finished";
```

- `sameObject` は `initialData` と **同じ箱を指す別名** にすぎません。中身を書き換えても箱は同じなので、結果は `false`（別物になっていない）です。
- React の state でこれをやると、値は変わっているのに **画面が更新されない** という不思議なバグになります（下のつまずきポイント4）。

##### 解説4: `??` は null/undefined だけ、`||` は空文字や 0 も置き換える

```ts
const withNullish = JSON.stringify(v ?? "既定値");
const withOr = JSON.stringify(v || "既定値");
```

- 実行結果の下4行を見比べてください。`null` と `undefined` ではどちらも「既定値」になりますが、`""`（空文字）と `0` では結果が分かれます。
- 編集ページで `book.publisher ?? ""` と `??` を使うのは、「`null` だけを空文字に直したい」からです。逆に `BookForm` の `formData.author.trim() || null` で `||` を使うのは、「空文字を `null` に直したい」からです。目的に合わせて使い分けています。

> **つまずきポイント4: state のオブジェクトを直接書き換えてしまう。** `formData.title = "新しいタイトル"` のように書いても、React は変化に気付かないので画面が更新されません（しかも、次に `setFormData` を呼んだときに思わぬ値が混ざる原因になります）。state を変えるときは、必ず **set 関数に新しいオブジェクトを渡します**。

**NG（まちがい・抜粋）:**

```tsx
const handleTitleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
  // 今の state を直接書き換えている（React は気付かない）
  formData.title = e.target.value;
  setFormData(formData);
};
```

**OK（正しい・抜粋）:**

```tsx
const handleTitleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
  // コピーした新しいオブジェクトを渡す
  setFormData((prev) => ({ ...prev, title: e.target.value }));
};
```

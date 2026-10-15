#### 図で整理: `null` と空文字の行き来

編集機能では、同じデータが「DB の行 → フォームの値 → UPDATE で送る中身」と姿を変えながら一周します。DB では「未入力 = `null`」、フォームでは「未入力 = 空文字 `""`」と決まりが違うので、**行きと帰りで1回ずつ変換** しているのがポイントです。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-07" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="105" y="24" text-anchor="middle" font-size="13" font-weight="700" fill="#475569">① DB の行</text>
<text x="360" y="24" text-anchor="middle" font-size="13" font-weight="700" fill="#1e40af">② フォームの値</text>
<text x="615" y="24" text-anchor="middle" font-size="13" font-weight="700" fill="#047857">③ UPDATE で送る中身</text>
<rect x="10" y="36" width="190" height="150" rx="8" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<rect x="265" y="36" width="190" height="150" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<rect x="520" y="36" width="190" height="150" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<g font-size="12" font-family="Consolas, monospace" fill="#1e293b">
<text x="22" y="62">title: "React実践ガイド"</text>
<text x="22" y="88">publisher: <tspan fill="#c2410c" font-weight="700">null</tspan></text>
<text x="22" y="114">published_date: <tspan fill="#c2410c" font-weight="700">null</tspan></text>
<text x="22" y="140">memo: <tspan fill="#c2410c" font-weight="700">null</tspan></text>
<text x="22" y="166">rating: 4</text>
<text x="277" y="62">title: "React実践ガイド"</text>
<text x="277" y="88">publisher: <tspan fill="#1e40af" font-weight="700">""</tspan></text>
<text x="277" y="114">published_date: <tspan fill="#1e40af" font-weight="700">""</tspan></text>
<text x="277" y="140">memo: <tspan fill="#1e40af" font-weight="700">""</tspan></text>
<text x="277" y="166">rating: 4</text>
<text x="532" y="62">title: "React実践ガイド"</text>
<text x="532" y="88">publisher: <tspan fill="#047857" font-weight="700">"技術評論社"</tspan></text>
<text x="532" y="114">published_date: <tspan fill="#c2410c" font-weight="700">null</tspan></text>
<text x="532" y="140">memo: <tspan fill="#c2410c" font-weight="700">null</tspan></text>
<text x="532" y="166">rating: 4</text>
</g>
<line x1="200" y1="111" x2="261" y2="111" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-07)"/>
<text x="231" y="101" text-anchor="middle" font-size="12" fill="#475569" font-family="Consolas, monospace">?? ""</text>
<text x="231" y="130" text-anchor="middle" font-size="12" fill="#475569">null→空</text>
<line x1="455" y1="111" x2="516" y2="111" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-07)"/>
<text x="486" y="101" text-anchor="middle" font-size="12" fill="#475569" font-family="Consolas, monospace">|| null</text>
<text x="486" y="130" text-anchor="middle" font-size="12" fill="#475569">空→null</text>
<text x="360" y="206" text-anchor="middle" font-size="12" fill="#475569">（ユーザーが出版社だけ入力し、メモは空白だけ打った）</text>
<text x="105" y="206" text-anchor="middle" font-size="12" fill="#64748b">編集ページで取得</text>
<text x="615" y="206" text-anchor="middle" font-size="12" fill="#64748b">BookForm が送信</text>
<text x="360" y="244" text-anchor="middle" font-size="12" fill="#475569">DB は「未入力 = null」、フォームは「未入力 = 空文字」。行きと帰りで1回ずつ変換する</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-8: 編集でデータが一周するときの null と空文字の変換（オレンジ = null、青 = 空文字）</div>
</div>

#### 動かして確かめる: DB の行 ⇄ フォームの値 の変換

> **▼ このコードがやること（先に日本語で）:** 編集ページの `initialData` 作り（`null` → `""`）と、`BookForm` の `.update({ ... })` に渡す中身作り（`""` → `null`、前後の空白を削る）の2つを関数として取り出し、図8-8 と同じデータで実行します。画面や Supabase を使わずに、変換の結果だけを目で確かめられます。

**ファイル: `convert.ts`（練習用）**

```ts
// 編集ページと BookForm がやっている「値の変換」だけを取り出して
// 動かしてみる練習用コード

// Supabase から返ってくる books テーブルの1行の形
type BookRow = {
  id: string;
  title: string;
  author: string | null;
  publisher: string | null;
  published_date: string | null;
  rating: number | null;
  status: string | null;
  memo: string | null;
};

// フォームが扱う値の形（文字の欄は必ず string）
type BookFormData = {
  title: string;
  author: string;
  publisher: string;
  published_date: string;
  rating: number | null;
  status: string;
  memo: string;
};

// 編集ページの initialData 作りと同じ変換（null → ""）
function toFormData(book: BookRow): BookFormData {
  return {
    title: book.title ?? "",
    author: book.author ?? "",
    publisher: book.publisher ?? "",
    published_date: book.published_date ?? "",
    rating: book.rating ?? null,
    status: book.status ?? "unread",
    memo: book.memo ?? "",
  };
}

// BookForm が .update() に渡す中身と同じ変換（"" → null、空白を除去）
function toUpdatePayload(form: BookFormData) {
  return {
    title: form.title.trim(),
    author: form.author.trim() || null,
    publisher: form.publisher.trim() || null,
    published_date: form.published_date || null,
    rating: form.rating,
    status: form.status,
    memo: form.memo.trim() || null,
  };
}

// DB にこんな行が入っていたとする（出版社・出版日・メモは未入力）
const row: BookRow = {
  id: "3f2a8c1e-5b7d-4e9a-9c01-2b3d4e5f6a7b",
  title: "React実践ガイド",
  author: "山田太郎",
  publisher: null,
  published_date: null,
  rating: 4,
  status: "reading",
  memo: null,
};

// ① DB の行 → フォームの初期値
const form = toFormData(row);
console.log("① フォームの初期値:", form);

// ② ユーザーが出版社を入力し、メモに空白だけ打ったとする
const edited: BookFormData = {
  ...form,
  publisher: "  技術評論社  ",
  memo: "   ",
};

// ③ フォームの値 → UPDATE で送る中身
console.log("③ UPDATE で送る中身:", toUpdatePayload(edited));
```

**▼ 実行結果**（`node convert.ts`）

```text
① フォームの初期値: {
  title: 'React実践ガイド',
  author: '山田太郎',
  publisher: '',
  published_date: '',
  rating: 4,
  status: 'reading',
  memo: ''
}
③ UPDATE で送る中身: {
  title: 'React実践ガイド',
  author: '山田太郎',
  publisher: '技術評論社',
  published_date: null,
  rating: 4,
  status: 'reading',
  memo: null
}
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 行き（DB → フォーム）は `?? ""` で null を空文字に

```ts
publisher: book.publisher ?? "",
published_date: book.published_date ?? "",
```

- DB で `null` だった出版社・出版日・メモが、実行結果①ではすべて `''`（空文字）になっています。
- 入力欄の `value` に `null` を渡すと React の「制御コンポーネント」として正しく動かないため、文字の欄は必ず文字列にそろえます（2-3 の解説2と同じ理由です）。

##### 解説2: 帰り（フォーム → DB）は `trim()` と `|| null` で空を null に

```ts
publisher: form.publisher.trim() || null,
memo: form.memo.trim() || null,
```

- `"  技術評論社  ".trim()` は前後の空白を削った `"技術評論社"` になります。
- `"   ".trim()` は `""`（空文字）になり、`"" || null` は `null` です。だから空白だけ打ったメモは、実行結果③で `null` として送られます。
- こうしておくと、DB には「空白だけの文字列」という見えないゴミが入らず、「未入力は必ず `null`」というルールが守られます。

##### 解説3: `rating` は変換しない

```ts
rating: book.rating ?? null,
```

- 評価はもともと「数値 または `null`」で、フォームの `<select>` 側で `value={formData.rating ?? ""}` と表示用に変換しています。データとしては `null` のまま持ち運ぶので、往復のどちらでも値が変わっていません（①③とも `rating: 4`）。

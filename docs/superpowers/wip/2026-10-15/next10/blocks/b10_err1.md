**▼ 実際の出力**（Next.js 16.4.1。`BookCard.tsx` で `book.title` を `book.titel` と打ち間違えた場合）

```text
  Running TypeScript ...
src/components/BookCard.tsx(7,39): error TS2551: Property 'titel' does not exist on type 'Book'. Did you mean 'title'?
Failed to type check.
```

> **つまずきポイント:** 型エラーの多くは「型定義が間違っている」のではなく、**使う側の打ち間違い**です。`Did you mean 'title'?`（もしかして title のこと？）というヒントが出ていたら、まずはその通りに直してみましょう。型定義の方を書き換えて黙らせるのは最後の手段です。

**NG: プロパティ名を打ち間違えている**

```tsx
// src/components/BookCard.tsx の一部（NG: titel は Book 型に無い）
<h2 className="font-bold">{book.titel}</h2>
```

**OK: 型定義どおりの名前を使う**

```tsx
// src/components/BookCard.tsx の一部（OK: Book 型の title）
<h2 className="font-bold">{book.title}</h2>
```

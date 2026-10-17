// 新規追加ページ。"use client" を書かないので Server
// app/books/new/page.tsx（Server Component）

// 同じフォルダの actions.ts から Server Action を取り込む
import { createBook } from "./actions";

// 同期関数でOK（このページ自体は async データ取得をしない）
export default function NewBookPage() {
  // <form action> に直接渡す Server Action は「何も返さない」形にする
  // createBook は結果（error など）を返すので、何も返さない関数で包む
  async function submitBook(formData: FormData) {
    // この関数の中身もサーバーで実行される、という印
    "use server";
    await createBook(formData);
  }

  return (
    <div>
      <h1>書籍を追加</h1>

      {/* `<form action={...}>` に関数を渡せるのが Next.js の拡張 */}
      {/* JavaScript が無効でも動作する（Progressive Enhancement） */}
      {/* 送信時に submitBook（→ createBook）がサーバーで実行される */}
      <form action={submitBook}>
        <div>
          {/* `htmlFor` は HTML の `for` 属性のJSX版（forはJSの予約語） */}
          <label htmlFor="title">タイトル</label>
          <input
            // テキスト入力
            type="text"
            // label と紐付けるための id
            id="title"
            // ※ formData.get で取り出すキー名はこの name 属性
            name="title"
            // 入力必須（ブラウザの標準バリデーション）
            required
          />
        </div>

        <div>
          <label htmlFor="author">著者</label>
          <input
            type="text"
            id="author"
            name="author"
            required
          />
        </div>

        <div>
          <label htmlFor="publishedYear">出版年</label>
          <input
            // 数値入力欄。上下ボタンが付く
            type="number"
            id="publishedYear"
            name="publishedYear"
          />
        </div>

        {/* submit ボタン */}
        <button type="submit">追加する</button>
      </form>
    </div>
  );
}
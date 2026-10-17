// クライアント側でローディング状態を扱えるフォーム
// components/BookFormClient.tsx
// useActionState は React のフックなので Client 必須
"use client";

// React 19 で追加された Server Action 連携用フック
// 旧名は useFormState（同じ用途）
import { useActionState } from "react";
// 別ファイルの Server Action を取り込む
import { createBook } from "@/app/books/new/actions";

// フォームの状態（Server Action が返した結果）の型
type FormState = { error?: string; success?: boolean } | null;

// useActionState に渡す関数は「(前回の状態, フォームの中身)」の2つを受け取る形にする
async function submitBook(
  _prevState: FormState,
  formData: FormData
): Promise<FormState> {
  // 中身は Server Action の createBook に任せる
  return createBook(formData);
}

export function BookFormClient() {
  // 戻り値は [現在のstate, ラップ済み formAction, 送信中フラグ]
  // 第2引数の null は state の初期値
  // formAction は <form action={...}> に渡せるよう加工された関数
  const [state, formAction, isPending] = useActionState(submitBook, null);

  return (
    // 元の createBook ではなく、ラップ済みの formAction を渡す
    <form action={formAction}>
      <div>
        <label htmlFor="title">タイトル</label>
        {/* name="title" が formData のキーになる */}
        <input type="text" id="title" name="title" required />
      </div>

      <div>
        <label htmlFor="author">著者</label>
        <input type="text" id="author" name="author" required />
      </div>

      <div>
        <label htmlFor="publishedYear">出版年</label>
        <input type="number" id="publishedYear" name="publishedYear" />
      </div>

      {/* エラーメッセージの表示 */}
      {/* `?.` はオプショナルチェイニング（state が null でも安全） */}
      {state?.error && (
        // Server Action が返した error メッセージを表示
        <p className="error-message">{state.error}</p>
      )}

      {/* 送信中はボタンを無効化 */}
      {/* `disabled={isPending}` で送信中はクリック不可 */}
      <button type="submit" disabled={isPending}>
        {/* ボタンラベルも切り替え */}
        {isPending ? "追加中..." : "追加する"}
      </button>
    </form>
  );
}
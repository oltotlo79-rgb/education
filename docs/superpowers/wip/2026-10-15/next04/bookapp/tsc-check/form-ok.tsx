// components/BookFormClient.tsx
"use client";

import { useActionState } from "react";
import { createBook } from "./actions";

// フォームの状態（Server Action が返した結果）の型
type FormState = { error?: string; success?: boolean } | null;

// useActionState に渡す関数は「(前回の状態, フォームの中身)」の2つを受け取る形にする
async function submitBook(
  _prevState: FormState,
  formData: FormData
): Promise<FormState> {
  return createBook(formData);
}

export function BookFormClient() {
  const [state, formAction, isPending] = useActionState(submitBook, null);

  return (
    <form action={formAction}>
      <input type="text" id="title" name="title" required />
      {state?.error && <p className="error-message">{state.error}</p>}
      <button type="submit" disabled={isPending}>
        {isPending ? "追加中..." : "追加する"}
      </button>
    </form>
  );
}

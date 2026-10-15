// app/ok-func/DeleteButton.tsx（Client Component）
"use client";

import { deleteBook } from "./actions";

export function DeleteButton({ title }: { title: string }) {
  // クリックされたときの処理は Client Component の中で作る
  const handleClick = async () => {
    await deleteBook(title);
  };

  return <button onClick={handleClick}>「{title}」を削除</button>;
}

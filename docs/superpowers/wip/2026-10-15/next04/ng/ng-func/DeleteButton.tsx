// app/ng-func/DeleteButton.tsx（Client Component）
"use client";

export function DeleteButton({
  title,
  onDelete,
}: {
  title: string;
  onDelete: () => void;
}) {
  return <button onClick={onDelete}>「{title}」を削除</button>;
}

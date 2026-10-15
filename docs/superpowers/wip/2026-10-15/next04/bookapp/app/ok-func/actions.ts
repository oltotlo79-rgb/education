// app/ok-func/actions.ts
"use server";

export async function deleteBook(title: string) {
  // 本物のアプリならここでデータベースから削除する
  console.log(`[deleteBook] 「${title}」を削除しました`);
}

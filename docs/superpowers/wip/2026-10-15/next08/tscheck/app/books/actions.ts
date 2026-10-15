// このファイルの関数は、すべてサーバー側で動く Server Action になる
"use server";

// キャッシュを捨てて、次の表示で最新データを取らせる関数
import { revalidatePath } from "next/cache";
// サーバー側からページを移動させる関数
import { redirect } from "next/navigation";
// サーバー用の Supabase クライアント
import { createClient } from "@/lib/supabase/server";

// 方法A（bind）用: 1番目の引数で id、2番目でフォームの中身を受け取る
export async function updateBookTitle(id: string, formData: FormData) {
  // name="title" の欄の値を取り出す（無ければ空文字）
  const title = String(formData.get("title") ?? "").trim();
  if (!title) {
    throw new Error("タイトルは必須です。");
  }

  const supabase = await createClient();
  const { error } = await supabase
    .from("books")
    .update({ title })
    // 必ず id で1件に絞る
    .eq("id", id);

  if (error) {
    throw new Error("書籍の更新に失敗しました。");
  }

  // 詳細ページと一覧ページのキャッシュを捨てる
  revalidatePath(`/books/${id}`);
  revalidatePath("/books");
  // 詳細ページへ移動する（この行より後ろは実行されない）
  redirect(`/books/${id}`);
}

// 方法B（hidden input）用: フォームの中身だけを受け取る
export async function updateBookTitleFromHidden(formData: FormData) {
  const id = formData.get("id");
  // hidden input が無い、または文字列でなければ止める
  if (typeof id !== "string" || id === "") {
    throw new Error("書籍の id が送られていません。");
  }
  // あとは方法A と同じ処理に任せる
  await updateBookTitle(id, formData);
}

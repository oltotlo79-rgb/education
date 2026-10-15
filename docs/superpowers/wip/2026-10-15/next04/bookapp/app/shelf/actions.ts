// app/shelf/actions.ts
"use server";

import { revalidatePath } from "next/cache";
import { shelf } from "@/lib/shelf";

export async function addBook(formData: FormData) {
  // name="title" の入力欄の値を取り出し、前後の空白を取り除く
  const title = String(formData.get("title") ?? "").trim();
  if (!title) {
    return;
  }

  // データを保存する（本物のアプリでは Supabase に INSERT する）
  shelf.push(title);
  console.log(`[addBook] 「${title}」を追加しました（全${shelf.length}冊）`);

  // 「/shelf の画面は古くなった」と Next.js に伝え、作り直してもらう
  revalidatePath("/shelf");
}

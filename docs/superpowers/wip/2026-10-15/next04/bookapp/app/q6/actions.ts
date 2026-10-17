"use server";

import { revalidatePath } from "next/cache";
import { saveBook } from "@/lib/books";

export async function addBook(formData: FormData) {
  const title = String(formData.get("title") ?? "").trim();
  if (!title) {
    return;
  }
  await saveBook(title);
  // 一覧の画面が古くなったことを Next.js に伝える
  revalidatePath("/");
}

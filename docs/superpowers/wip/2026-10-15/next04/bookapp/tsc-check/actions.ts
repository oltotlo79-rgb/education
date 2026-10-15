// app/books/new/actions.ts
"use server";

export async function createBook(formData: FormData) {
  const title = formData.get("title") as string;
  const author = formData.get("author") as string;
  const publishedYear = Number(formData.get("publishedYear"));

  if (!title || !author) {
    return {
      error: "タイトルと著者は必須です",
    };
  }

  console.log("書籍を追加:", { title, author, publishedYear });
  return { success: true };
}

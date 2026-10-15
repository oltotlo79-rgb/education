// app/books/new/page.tsx（Server Component）
import { createBook } from "./actions";

export default function NewBookPage() {
  // <form action> に直接渡す Server Action は「何も返さない」形にする
  async function submitBook(formData: FormData) {
    "use server";
    await createBook(formData);
  }

  return (
    <div>
      <h1>書籍を追加</h1>
      <form action={submitBook}>
        <input type="text" id="title" name="title" required />
        <button type="submit">追加する</button>
      </form>
    </div>
  );
}

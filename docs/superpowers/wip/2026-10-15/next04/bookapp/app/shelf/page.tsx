// app/shelf/page.tsx（Server Component）
import { shelf } from "@/lib/shelf";
import { addBook } from "./actions";

export default function ShelfPage() {
  return (
    <div>
      <h1>本棚（{shelf.length}冊）</h1>
      <ul>
        {shelf.map((title) => (
          <li key={title}>{title}</li>
        ))}
      </ul>
      {/* action に Server Action を渡す */}
      <form action={addBook}>
        <input name="title" placeholder="書名" />
        <button type="submit">追加</button>
      </form>
    </div>
  );
}

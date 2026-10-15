import { updateBookTitle } from "@/app/books/actions";

export function Ng({ id }: { id: string }) {
  return (
    <form action={updateBookTitle(id)}>
      <button type="submit">x</button>
    </form>
  );
}

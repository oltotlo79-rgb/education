type BooksState =
  | { status: "loading" }
  | { status: "success"; books: string[] }
  | { status: "error"; message: string };

function render(state: BooksState): string {
  switch (state.status) {
    case "loading":
      return "読み込み中…";
    case "success":
      return `${state.books.length}冊: ${state.books.join(", ")}`;
    case "error":
      return `エラー: ${state.message}`;
    default: {
      // ここに来ることは無いはず。来たら型エラーで教えてもらう
      const unreachable: never = state;
      return unreachable;
    }
  }
}

const loaded: BooksState = {
  status: "success",
  books: ["TypeScript入門", "React実践ガイド"],
};

console.log(render({ status: "loading" }));
console.log(render(loaded));
console.log(render({ status: "error", message: "通信に失敗しました" }));

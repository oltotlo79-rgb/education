// ===== 読書ステータスの定義 =====

// 3つの文字列リテラルだけを許す型。
type ReadingStatus = "want_to_read" | "reading" | "completed";

// 書籍の基本型。
interface Book {
  id: string;
  title: string;
  author: string;
  // ステータスは ReadingStatus 型に限定。
  status: ReadingStatus;
}

// --- 正しい使い方 ---
const book1: Book = {
  id: "1",
  title: "TypeScript入門",
  author: "山田太郎",
  // 許可された値。
  status: "reading",
};

const book2: Book = {
  id: "2",
  title: "React実践ガイド",
  author: "佐藤花子",
  status: "completed",
};

const book3: Book = {
  id: "3",
  title: "Next.js入門",
  author: "鈴木一郎",
  status: "want_to_read",
};

// ===== ステータスに応じた処理 =====

// ステータスを日本語ラベルに変換する関数。
function getStatusLabel(status: ReadingStatus): string {
  // switch 文で値ごとに分岐。
  switch (status) {
    // status が "want_to_read" のとき。
    case "want_to_read":
      // return すると関数が即終了。break 不要。
      return "読みたい";
    case "reading":
      return "読書中";
    case "completed":
      return "読了";
  }
  // すべてのケースを処理しているため、default 不要
  // ReadingStatus の3値すべてを書いたので、TypeScript は「漏れなし」と判断する。
  // TypeScript がすべてのケースが網羅されていることを保証
}

// ステータス → 色コード（#で始まる16進数）の変換。
function getStatusColor(status: ReadingStatus): string {
  switch (status) {
    case "want_to_read":
      // オレンジ
      return "#f39c12";
    case "reading":
      // 青
      return "#3498db";
    case "completed":
      // 緑
      return "#27ae60";
  }
}

// ステータス → アイコン名の変換（map 方式）。
function getStatusIcon(status: ReadingStatus): string {
  // Record<K, V> は「K 型のキーを全部持ち、値がすべて V 型」のオブジェクトを表す Utility Type。
  const icons: Record<ReadingStatus, string> = {
    // キー名が普通の識別子（英数字と _ だけ）なので、クォートは省略できる。
    // "want-to-read" のようにハイフンを含むキーなら "..." で囲む必要がある。
    want_to_read: "bookmark",
    reading: "book-open",
    completed: "check-circle",
  };
  // status をキーにして値を取り出す。Record で全キー網羅されているので必ず文字列が取れる。
  return icons[status];
}

// ===== フィルタリング =====

// 書籍配列を「指定ステータスのものだけ」に絞り込む関数。
function filterBooksByStatus(
  // 書籍の配列を受け取る。
  books: Book[],
  // 絞り込みたいステータス。
  status: ReadingStatus
// 戻り値は Book の配列。
): Book[] {
  // filter で「book.status と引数 status が一致する要素」だけ残す。
  return books.filter((book) => book.status === status);
}

// 3冊をまとめた配列。
const allBooks: Book[] = [book1, book2, book3];
// "reading" だけ残す → [book1]。
const readingBooks = filterBooksByStatus(allBooks, "reading");
// "completed" だけ残す → [book2]。
const completedBooks = filterBooksByStatus(allBooks, "completed");

// ===== ステータスの変更 =====

// book に新しい status を当てた新しい book を返す関数。
function updateBookStatus(
  book: Book,
  newStatus: ReadingStatus
): Book {
  // ... はスプレッド構文（オブジェクト版）。book の全プロパティをコピーして、後ろの status を上書き。元の book は壊さない（イミュータブル）。
  return { ...book, status: newStatus };
}

// book1 の status だけ "completed" にした新オブジェクト。
const updatedBook = updateBookStatus(book1, "completed");
// "completed"
console.log(updatedBook.status);

// 無効なステータスへの変更はコンパイルエラー
// ReadingStatus にない値なのでエラー。
// updateBookStatus(book1, "abandoned");
// エラー: Argument of type '"abandoned"' is not assignable to type 'ReadingStatus'

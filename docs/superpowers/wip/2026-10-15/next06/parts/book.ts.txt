// types/book.ts
// 書籍データに関する型と定数をまとめて定義するファイル

/**
 * 読書ステータスの型定義
 *
 * - "reading"      : 読書中（今読んでいる途中）
 * - "completed"    : 読了（読み終わった）
 * - "want_to_read" : 読みたい（これから読む）
 */
// type: TypeScript の型エイリアス（型に名前を付ける構文）
// "reading" | "completed" | "want_to_read": ユニオン型。この3つの文字列のいずれかしか取れない
// export: 他のファイルからこの型をインポートできるようにする
export type BookStatus = "reading" | "completed" | "want_to_read";

/**
 * 書籍データの型定義（データベースから取得した完全なデータ）
 *
 * Supabase の books テーブル（第5章で作成）の1行に対応します。
 * すべての列が含まれており、データベースから取得した
 * 書籍データはこの型に一致します。
 */
// type Book = { ... }: オブジェクトの形を表す型を Book という名前で定義
export type Book = {
  /** 書籍ID（UUID形式、Supabase が自動生成） */
  // UUID は "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx" 形式の文字列
  id: string;

  /** 書籍のタイトル（必須） */
  // 例: "リーダブルコード"
  title: string;

  /** 著者名（必須） */
  // 例: "Dustin Boswell"
  author: string;

  /** 出版社（任意、未入力の場合は null） */
  // string | null: 文字列か null のどちらか（ユニオン型）
  publisher: string | null;

  /** 出版日（任意。"2024-01-15" のような日付の文字列、未入力なら null） */
  // データベースの date 型は、JavaScript では文字列として届く
  published_date: string | null;

  /** 評価（1〜5の整数、未評価の場合は null） */
  // number | null: 数値か null のどちらか
  rating: number | null;

  /** 読書ステータス（デフォルト: "want_to_read"） */
  // 上で定義した BookStatus 型（3つの文字列のどれか）
  status: BookStatus;

  /** メモ・感想（任意、未入力の場合は null） */
  // 文字列または null
  memo: string | null;

  /** 作成日時（ISO 8601 形式の文字列、Supabase が自動生成） */
  // 例: "2026-10-15T10:30:00.000Z"
  created_at: string;

  /** 更新日時（ISO 8601 形式の文字列、Supabase が自動生成） */
  // 行が更新されるたびに Supabase のトリガーが書き換える
  updated_at: string;
};

/**
 * 書籍の新規登録時に送るデータの型定義
 *
 * id, created_at, updated_at はデータベースが自動生成するため、
 * 送信データには含めません。
 * title と author は必須、それ以外はオプショナル（省略可能）です。
 */
export type BookInsert = {
  /** 書籍のタイトル（必須） */
  // 必須項目（? が付いていない）
  title: string;

  /** 著者名（必須） */
  // 必須項目。空文字や null にせず、必ず名前を入れて送る
  author: string;

  /** 出版社（省略時は null） */
  // ?: オプショナル。省略可能
  publisher?: string | null;

  /** 出版日（省略時は null） */
  // 省略してもよいし、明示的に null を送ってもよい
  published_date?: string | null;

  /** 評価（1〜5、省略時は null） */
  // 同上
  rating?: number | null;

  /** 読書ステータス（省略時は "want_to_read" がデフォルト） */
  // 省略時の値はDB側のデフォルトで補う
  status?: BookStatus;

  /** メモ・感想（省略時は null） */
  // 同上
  memo?: string | null;
};

/**
 * 書籍の更新時に送るデータの型定義
 *
 * 更新したい列だけを送ればよいため、
 * すべての項目がオプショナルです。
 * Partial<BookInsert> と同じ意味ですが、明示的に定義しています。
 */
export type BookUpdate = {
  /** 書籍のタイトル */
  // すべて ? 付き（更新時は変更したい項目のみ送る）
  title?: string;

  /** 著者名 */
  // 同上（送る場合は空にしない）
  author?: string;

  /** 出版社（null で消去） */
  // 同上
  publisher?: string | null;

  /** 出版日（null で消去） */
  // 同上
  published_date?: string | null;

  /** 評価（1〜5、null で評価をクリア） */
  // null を明示的に送ると評価をリセットできる
  rating?: number | null;

  /** 読書ステータス */
  // 同上
  status?: BookStatus;

  /** メモ・感想（null でメモをクリア） */
  // 同上
  memo?: string | null;
};

/**
 * ステータスの表示ラベル定義
 *
 * コンポーネントでステータスを日本語表示するときに使います。
 * 例: statusLabels["reading"] → "読書中"
 */
// Record<BookStatus, string>: TypeScript の組み込み型。
// 「キーが BookStatus 型、値が string 型のオブジェクト」を意味する
// 3つのキー（reading, completed, want_to_read）すべてが必須になる
export const statusLabels: Record<BookStatus, string> = {
  // "reading" の表示用ラベル
  reading: "読書中",
  // "completed" の表示用ラベル
  completed: "読了",
  // "want_to_read" の表示用ラベル
  want_to_read: "読みたい",
};

/**
 * ステータスに対応する色のクラス定義
 *
 * StatusBadge コンポーネントでバッジの色を切り替えるときに使います。
 * Tailwind CSS のクラス名を直接指定しています（第7章の StatusBadge と同じ色）。
 */
// Record<BookStatus, { bg: string; text: string }>:
// キーが BookStatus、値が { bg: string; text: string } 型のオブジェクト
// 各ステータスごとに「背景色クラス」と「文字色クラス」を持つ
export const statusColors: Record<
  BookStatus,
  { bg: string; text: string }
> = {
  // 読書中: 青系（進行中の印象）
  reading: {
    // 背景は薄い青
    bg: "bg-blue-100",
    // 文字は濃い青
    text: "text-blue-800",
  },
  // 読了: 緑系（完了の印象）
  completed: {
    // 背景は薄い緑
    bg: "bg-green-100",
    // 文字は濃い緑
    text: "text-green-800",
  },
  // 読みたい: 黄系（これから、の印象）
  want_to_read: {
    // 背景は薄い黄
    bg: "bg-yellow-100",
    // 文字は濃い黄
    text: "text-yellow-800",
  },
};

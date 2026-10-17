// サーバー用 Supabase クライアント
import { createClient } from "@/lib/supabase/server";
// 詳細ページ等への遷移
import Link from "next/link";
// 検索バー（クライアントコンポーネント）
import SearchBar from "@/components/SearchBar";
// ソート用セレクト（クライアントコンポーネント）
import SortSelect from "@/components/SortSelect";

// -------------------------------------------------
// 型定義
// -------------------------------------------------
// Next.js App Router のページコンポーネントは、
// searchParams プロパティでURLの検索パラメータを
// 受け取ることができます。
// Next.js 15 以降は Promise でラップされる点に注意。
// -------------------------------------------------
type Props = {
  // URL の ? 以降のパラメータ
  searchParams: Promise<{
    // 検索クエリ（省略可）
    q?: string;
    // ステータスフィルタ
    status?: string;
    // ソート対象カラム
    sort?: string;
    // 昇順/降順
    order?: string;
  }>;
};

// -------------------------------------------------
// ステータス表示用のヘルパー関数
// -------------------------------------------------
// 詳細ページで使ったものと同じ。
// 共通化したい場合は別ファイルに抽出するとよい。
// -------------------------------------------------
function getStatusLabel(status: string): string {
  const statusMap: Record<string, string> = {
    want_to_read: "読みたい",
    reading: "読書中",
    completed: "読了",
  };
  return statusMap[status] || status;
}

function getStatusColor(status: string): string {
  switch (status) {
    case "want_to_read":
      return "bg-yellow-100 text-yellow-800";
    case "reading":
      return "bg-blue-100 text-blue-800";
    case "completed":
      return "bg-green-100 text-green-800";
    default:
      return "bg-gray-100 text-gray-800";
  }
}

// -------------------------------------------------
// 評価を星マークで表示する関数
// -------------------------------------------------
function renderStars(rating: number | null): string {
  // 未評価のときはハイフン
  if (rating === null || rating === undefined) return "-";
  return "★".repeat(rating) + "☆".repeat(5 - rating);
}

// -------------------------------------------------
// 書籍一覧ページ（Server Component）
// -------------------------------------------------
export default async function BooksPage({ searchParams }: Props) {
  // Promise を解決して各パラメータを取り出し
  const { q, status, sort, order } = await searchParams;
  // サーバー用クライアント
  const supabase = await createClient();

  // -------------------------------------------------
  // Supabase クエリの構築
  // -------------------------------------------------
  // 基本のクエリを作成し、検索条件やフィルタ条件が
  // ある場合は動的にクエリを組み立てます。
  // let で宣言して、後から条件を追加していく形にします。
  // -------------------------------------------------
  // ベースのクエリ（全件取得）
  let query = supabase.from("books").select("*");

  // -------------------------------------------------
  // テキスト検索
  // -------------------------------------------------
  // q パラメータが指定されている場合、タイトルまたは
  // 著者名で部分一致検索を行います。
  //
  // Supabase の ilike は大文字小文字を区別しない
  // LIKE 検索です。% はワイルドカードで、
  // %検索語% は「検索語を含む」という意味になります。
  //
  // .or() を使って、タイトルまたは著者名のいずれかに
  // マッチする条件を設定します。
  // -------------------------------------------------
  if (q) {
    // title OR author で部分一致
    query = query.or(`title.ilike.%${q}%,author.ilike.%${q}%`);
  }

  // -------------------------------------------------
  // ステータスフィルタ
  // -------------------------------------------------
  // status パラメータが指定されている場合、
  // そのステータスの書籍のみを取得します。
  // -------------------------------------------------
  if (status) {
    // WHERE status = :status
    query = query.eq("status", status);
  }

  // -------------------------------------------------
  // ソート
  // -------------------------------------------------
  // sort パラメータでソート対象のカラムを、
  // order パラメータでソート順（昇順/降順）を
  // 指定します。デフォルトは作成日の降順です。
  // -------------------------------------------------
  // 指定が無ければ created_at
  const sortColumn = sort || "created_at";
  // "asc" のときだけ昇順
  const ascending = order === "asc";
  // ORDER BY :sortColumn :order
  query = query.order(sortColumn, { ascending });

  // -------------------------------------------------
  // クエリ実行
  // -------------------------------------------------
  // ここで初めてサーバーへリクエストが飛ぶ。
  // -------------------------------------------------
  const { data: books, error } = await query;

  if (error) {
    console.error("書籍の取得に失敗:", error);
  }

  // -------------------------------------------------
  // レンダリング
  // -------------------------------------------------
  return (
    <div className="max-w-4xl mx-auto py-8 px-4">
      {/* ページヘッダー */}
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-gray-900">書籍一覧</h1>
        <Link
          href="/books/new"
          className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700"
        >
          <svg
            className="w-4 h-4 mr-2"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              // + アイコン
              d="M12 4v16m8-8H4"
            />
          </svg>
          新規登録
        </Link>
      </div>

      {/* 検索バーとソート */}
      <SearchBar />
      <SortSelect />

      {/* 検索結果の件数表示 */}
      <p className="text-sm text-gray-500 mb-4">
        {books ? `${books.length}件の書籍が見つかりました` : "読み込み中..."}
        {q && (
          <span className="ml-2">
            （検索: 「{q}」）
          </span>
        )}
        {status && (
          <span className="ml-2">
            （フィルタ: {getStatusLabel(status)}）
          </span>
        )}
      </p>

      {/* 書籍一覧 */}
      {/* データが無いとき */}
      {!books || books.length === 0 ? (
        <div className="text-center py-12 bg-white rounded-lg shadow">
          <svg
            className="mx-auto h-12 w-12 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              // 開いた本のアイコン
              d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
            />
          </svg>
          <h3 className="mt-2 text-sm font-medium text-gray-900">
            書籍が見つかりません
          </h3>
          <p className="mt-1 text-sm text-gray-500">
            {q || status
              ? "検索条件を変更してお試しください。"
              : "新しい書籍を登録してみましょう。"}
          </p>
          {/* 検索条件無しで 0 件のときだけ「最初の書籍を登録」を表示 */}
          {!q && !status && (
            <div className="mt-6">
              <Link
                href="/books/new"
                className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
              >
                最初の書籍を登録する
              </Link>
            </div>
          )}
        </div>
      ) : (
        <div className="bg-white shadow overflow-hidden sm:rounded-lg">
          <ul className="divide-y divide-gray-200">
            {/* 配列を map で展開してリスト要素を生成 */}
            {books.map((book) => (
              // key は React がリスト要素を識別するために必要
              <li key={book.id}>
                <Link
                  // クリックで詳細ページへ
                  href={`/books/${book.id}`}
                  className="block hover:bg-gray-50 transition-colors duration-150"
                >
                  <div className="px-4 py-4 sm:px-6">
                    <div className="flex items-center justify-between">
                      <div className="flex-1 min-w-0">
                        <p className="text-sm font-medium text-blue-600 truncate">
                          {book.title}
                        </p>
                        <p className="mt-1 text-sm text-gray-500">
                          {book.author || "著者不明"}
                          {/* 出版社があれば併記 */}
                          {book.publisher && ` / ${book.publisher}`}
                        </p>
                      </div>
                      <div className="ml-4 flex flex-col items-end space-y-1">
                        <span
                          className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(book.status)}`}
                        >
                          {getStatusLabel(book.status)}
                        </span>
                        {book.rating && (
                          <span className="text-yellow-500 text-xs">
                            {renderStars(book.rating)}
                          </span>
                        )}
                      </div>
                    </div>
                  </div>
                </Link>
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}

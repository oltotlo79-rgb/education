// サーバー用 Supabase クライアント
import { createClient } from "@/lib/supabase/server";
// 404 表示用
import { notFound } from "next/navigation";
// 戻るリンク用
import Link from "next/link";
// 上で更新した共通フォーム
import BookForm from "@/components/BookForm";

// -------------------------------------------------
// 型定義
// -------------------------------------------------
// 動的セグメント [id] の値を Promise として受け取る
// （Next.js 15 以降の仕様）。
// -------------------------------------------------
type Props = {
  params: Promise<{ id: string }>;
};

// -------------------------------------------------
// 編集ページコンポーネント（Server Component）
// -------------------------------------------------
// このコンポーネントは以下の処理を行います:
// 1. URLのパラメータから書籍IDを取得
// 2. Supabase から既存の書籍データを取得
// 3. データが存在しなければ 404 を表示
// 4. BookForm コンポーネントに既存データを渡して
//    レンダリング
// -------------------------------------------------
export default async function BookEditPage({ params }: Props) {
  // -------------------------------------------------
  // 1. params から書籍IDを取得
  // -------------------------------------------------
  // await を忘れると id が undefined になり、
  // データ取得時にエラーになります。
  // -------------------------------------------------
  const { id } = await params;

  // -------------------------------------------------
  // 2. Supabase から既存データを取得
  // -------------------------------------------------
  // クライアントを準備
  const supabase = await createClient();

  const { data: book, error } = await supabase
    .from("books")
    .select("*")
    // URL の id と一致するレコード
    .eq("id", id)
    // 単一レコードとして取得
    .single();

  // -------------------------------------------------
  // 3. エラーハンドリング
  // -------------------------------------------------
  // 存在しない ID にアクセスされた場合や、
  // 何らかのエラーが起きた場合は 404 を表示します。
  // -------------------------------------------------
  if (error || !book) {
    notFound();
  }

  // -------------------------------------------------
  // 4. BookForm に渡す初期データを整形
  // -------------------------------------------------
  // データベースから取得したデータをフォームの型に
  // 合わせて整形します。null の値は空文字に変換し、
  // フォームの入力フィールドで正しく表示されるように
  // します。
  //
  // ?? は左が null/undefined のときだけ右の値を使う
  // 演算子です（nullish coalescing）。
  // -------------------------------------------------
  const initialData = {
    // null なら空文字に変換
    title: book.title ?? "",
    author: book.author ?? "",
    publisher: book.publisher ?? "",
    published_date: book.published_date ?? "",
    // null はそのまま
    rating: book.rating ?? null,
    // status は null だと困るのでデフォルトを設定
    status: book.status ?? "want_to_read",
    memo: book.memo ?? "",
  };

  // -------------------------------------------------
  // 5. レンダリング
  // -------------------------------------------------
  return (
    <div className="max-w-2xl mx-auto py-8 px-4">
      {/* 戻るリンク */}
      <Link
        // 編集元の詳細ページへ
        href={`/books/${id}`}
        className="inline-flex items-center text-sm text-blue-600 hover:text-blue-800 mb-6"
      >
        <svg
          className="w-4 h-4 mr-1"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            // 左向き矢印
            d="M15 19l-7-7 7-7"
          />
        </svg>
        詳細に戻る
      </Link>

      {/* ページタイトル */}
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-gray-900">書籍を編集</h1>
        <p className="mt-1 text-sm text-gray-600">
          {/* 既存タイトルを表示して、編集対象を明示 */}
          「{book.title}」の情報を編集します。
        </p>
      </div>

      {/* -------------------------------------------
          BookForm コンポーネント
          initialData: 既存の書籍データ
          isEdit: true（編集モード）
          bookId: 対象の書籍ID
          サーバーコンポーネントからクライアント
          コンポーネントへ props を渡す例。
      ------------------------------------------- */}
      <div className="bg-white shadow rounded-lg p-6">
        <BookForm initialData={initialData} isEdit={true} bookId={id} />
      </div>
    </div>
  );
}

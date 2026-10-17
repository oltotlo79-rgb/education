// 一覧に戻るためのリンク部品
import Link from "next/link";

// not-found.tsx は props を受け取らない決まり（どの id だったかは分からない）
export default function BookNotFound() {
  return (
    <div className="max-w-2xl mx-auto py-16 px-4 text-center">
      {/* 大きく薄い文字で 404 と表示 */}
      <p className="text-5xl font-bold text-gray-300">404</p>
      <h1 className="mt-4 text-xl font-bold text-gray-900">
        書籍が見つかりません
      </h1>
      <p className="mt-2 text-sm text-gray-500">
        URL が間違っているか、すでに削除された可能性があります。
      </p>
      {/* 一覧ページへ戻るボタン */}
      <Link
        href="/"
        className="mt-6 inline-block rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
      >
        一覧に戻る
      </Link>
    </div>
  );
}

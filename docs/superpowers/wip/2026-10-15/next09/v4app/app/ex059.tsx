// components/EmptyState.tsx
// ↑ データが0件のときに「まだ何もありません」と分かりやすく案内するコンポーネント。
//   ただ空白にするより、ユーザーに次の行動を促すボタンを置くのが UX 的に親切。

"use client";

import Link from "next/link";

// props 型。? が付いているプロパティはすべて省略可能
type EmptyStateProps = {
  // タイトル
  title?: string;
  // 補足説明文
  message?: string;
  // ボタンの文字
  actionLabel?: string;
  // ボタンのリンク先
  actionHref?: string;
  // 3種類から選択
  icon?: "book" | "search" | "error";
};

// 引数のデフォルト値を分割代入で指定。
// 呼び出し側が省略してもこれらの値が使われる
export function EmptyState({
  title = "書籍が見つかりません",
  message = "まだ書籍が登録されていません。最初の一冊を登録してみましょう。",
  actionLabel = "書籍を登録する",
  actionHref = "/books/new",
  icon = "book",
}: EmptyStateProps) {
  // アイコンの描画
  const renderIcon = () => {
    switch (icon) {
      case "book":
        return (
          <svg
            className="h-16 w-16"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={1}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M12 6.042A8.967 8.967 0 006 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 016 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 016-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0018 18a8.967 8.967 0 00-6 2.292m0-14.25v14.25"
            />
          </svg>
        );
      case "search":
        return (
          <svg
            className="h-16 w-16"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={1}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"
            />
          </svg>
        );
      case "error":
        return (
          <svg
            className="h-16 w-16"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={1}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z"
            />
          </svg>
        );
    }
  };

  return (
    <div
      className="
        animate-fade-in
        flex flex-col items-center justify-center
        rounded-xl
        border-2 border-dashed border-gray-200
        bg-gray-50/50
        px-6 py-16
        text-center
        dark:border-gray-700
        dark:bg-gray-800/50
      "
      /* ▼ クラスの読み解き ▼
         animate-fade-in              : フェードイン
         flex flex-col items-center   : 縦並びで中央寄せ
         justify-center               : 縦方向も中央
         rounded-xl                   : 大きめの角丸
         border-2 border-dashed       : 2px の点線枠線（dashed:破線）
         bg-gray-50/50                : 灰色の透明度50%（うっすら）
         px-6 py-16                   : 横24px、縦64px の余白
         text-center                  : テキスト中央揃え
         dark:* で各色のダーク版指定
      */
    >
      {/* アイコン */}
      <div
        className="
          mb-4
          rounded-full
          bg-gray-100
          p-4
          text-gray-400
          dark:bg-gray-700
          dark:text-gray-500
        "
        /* mb-4         : 下マージン 1rem (16px)
           rounded-full : 完全な円形（円形バッジの容器）
           bg-gray-100  : 薄い灰色背景
           p-4          : padding 16px
           text-gray-400: アイコン色 */
      >
        {renderIcon()}
      </div>

      {/* タイトル */}
      <h3
        className="
          mb-2
          text-lg font-semibold
          text-gray-900
          dark:text-gray-100
        "
        /* mb-2: 下16px / text-lg: 18px / font-semibold: 600 */
      >
        {title}
      </h3>

      {/* メッセージ */}
      <p
        className="
          mb-6
          max-w-sm
          text-sm text-gray-500
          dark:text-gray-400
        "
        /* mb-6      : 下24px
           max-w-sm  : 最大幅 24rem (384px)。長い文章でも適度に折り返す
           text-sm   : 14px
           text-gray-500 : 補足色 */
      >
        {message}
      </p>

      {/* アクションボタン（actionHref があれば表示） */}
      {actionHref && (
        <Link href={actionHref} className="btn-primary gap-2">
          {/* btn-primary: globals.css のプライマリボタン
              gap-2     : flex 子要素間 8px（アイコンと文字の間） */}
          <svg
            className="h-4 w-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={2}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              /* 「+」プラスマーク */
              d="M12 4.5v15m7.5-7.5h-15"
            />
          </svg>
          {actionLabel}
        </Link>
      )}
    </div>
  );
}

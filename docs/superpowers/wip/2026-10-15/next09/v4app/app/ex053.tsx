// components/Pagination.tsx
// ↑ ページネーション（複数ページを切り替えるナビゲーション）コンポーネント。
//   1   2   3 ... 10  のような表示を作る。

// クライアントコンポーネント（useSearchParams を使う）
"use client";

// Next.js 用リンクコンポーネント（高速遷移）
import Link from "next/link";
// URL のクエリパラメータ取得 Hook
import { useSearchParams } from "next/navigation";

// コンポーネントが受け取る props 型
type PaginationProps = {
  // 現在表示中のページ番号（1 始まり）
  currentPage: number;
  // 全体のページ数
  totalPages: number;
  // ベースとなるURLパス（例: "/books"）
  basePath: string;
};

export function Pagination({
  currentPage,
  totalPages,
  basePath,
}: PaginationProps) {
  // 現在の URL のクエリパラメータ（?key=val&...）を取得
  const searchParams = useSearchParams();

  // ページ番号から URL を生成するヘルパー関数
  // 既存の検索条件（?search=xxx 等）を保ちつつ page だけ書き換える
  const createPageUrl = (page: number): string => {
    // URLSearchParams: クエリ文字列を扱う標準 API
    // 既存のパラメータを複製してから page を上書き
    const params = new URLSearchParams(searchParams.toString());
    // 数値を文字列にして set
    params.set("page", String(page));
    // 例: "/books?page=2&search=react"
    return `${basePath}?${params.toString()}`;
  };

  // 表示するページ番号のリストを計算する関数
  // 例: currentPage=5, totalPages=10 → [1, "ellipsis", 4, 5, 6, "ellipsis", 10]
  // 返却型: number または "ellipsis"（省略記号）の配列
  const getPageNumbers = (): (number | "ellipsis")[] => {
    const pages: (number | "ellipsis")[] = [];
    // 表示するページ番号の最大数（省略記号除く）
    const maxVisible = 5;

    if (totalPages <= maxVisible + 2) {
      // 全ページ数が少ない場合（7ページ以下なら全部表示）
      for (let i = 1; i <= totalPages; i++) {
        pages.push(i);
      }
    } else {
      // 多い場合は「先頭 ... 真ん中 ... 末尾」形式
      // 常に最初のページを表示
      pages.push(1);

      // 現在のページが最初の方（1〜3）にある場合
      // → [1, 2, 3, 4, ..., totalPages]
      if (currentPage <= 3) {
        pages.push(2, 3, 4);
        pages.push("ellipsis");
      }
      // 現在のページが最後の方（totalPages-2 以降）にある場合
      // → [1, ..., totalPages-3, totalPages-2, totalPages-1, totalPages]
      else if (currentPage >= totalPages - 2) {
        pages.push("ellipsis");
        pages.push(totalPages - 3, totalPages - 2, totalPages - 1);
      }
      // 現在のページが中間にある場合
      // → [1, ..., current-1, current, current+1, ..., totalPages]
      else {
        pages.push("ellipsis");
        pages.push(currentPage - 1, currentPage, currentPage + 1);
        pages.push("ellipsis");
      }

      // 常に最後のページを表示
      pages.push(totalPages);
    }

    return pages;
  };

  // ページが1ページしかない場合は表示しない（早期リターン）
  if (totalPages <= 1) return null;

  // ページ番号リストを計算
  const pageNumbers = getPageNumbers();
  // 最初のページか
  const isFirstPage = currentPage === 1;
  // 最後のページか
  const isLastPage = currentPage === totalPages;

  // ----- スタイル定数（複数箇所で使い回すために変数化）-----
  // 共通のページ番号ボタンスタイル
  const basePageClass = `
    inline-flex h-10 w-10 items-center justify-center
    rounded-lg text-sm font-medium
    transition-all duration-200
    focus-visible:outline-none focus-visible:ring-2
    focus-visible:ring-primary-500 focus-visible:ring-offset-2
  `;
  /* ▼ クラスの読み解き ▼
     inline-flex h-10 w-10        : 40x40 の正方形ボタン
     items-center justify-center  : 縦横中央に数字を配置
     rounded-lg                   : 角丸 8px
     text-sm font-medium          : 14px 太さ500
     transition-all duration-200  : 全プロパティ 200ms で滑らかに変化
     focus-visible:*              : キーボードフォーカス時のリング
  */

  // 現在のページ（アクティブ）用スタイル
  const activePageClass = `
    ${basePageClass}
    bg-primary-600 text-white shadow-md
    dark:bg-primary-500
  `;
  /* インディゴ背景＋白文字＋影。現在地が視覚的に分かるように */

  // 他のページ番号用スタイル
  const inactivePageClass = `
    ${basePageClass}
    text-gray-600 hover:bg-gray-100
    dark:text-gray-400 dark:hover:bg-gray-800
  `;
  /* 普段は灰色、ホバーで薄い灰色背景 */

  // 「前へ」「次へ」が無効化されているとき用
  const disabledNavClass = `
    inline-flex h-10 items-center justify-center
    rounded-lg px-3 text-sm font-medium
    text-gray-300 cursor-not-allowed
    dark:text-gray-600
  `;
  /* text-gray-300       : とても薄い灰色（押せない感じ） */
  /* cursor-not-allowed : マウスカーソルが禁止マークに */

  // 「前へ」「次へ」が有効なとき用
  const enabledNavClass = `
    inline-flex h-10 items-center justify-center
    rounded-lg px-3 text-sm font-medium
    text-gray-600 hover:bg-gray-100
    transition-all duration-200
    dark:text-gray-400 dark:hover:bg-gray-800
    focus-visible:outline-none focus-visible:ring-2
    focus-visible:ring-primary-500 focus-visible:ring-offset-2
  `;

  return (
    // <nav> 要素はナビゲーションを表すセマンティック要素
    <nav
      role="navigation"
      // スクリーンリーダー向けの説明
      aria-label="ページネーション"
      className="flex items-center justify-center gap-1 py-8"
      /* flex items-center justify-center: 中央寄せ
         gap-1                            : 子要素間 4px
         py-8                             : 上下パディング 2rem (32px) */
    >
      {/* 前のページボタン */}
      {/* 最初のページなら無効化された span を、それ以外なら Link を表示 */}
      {isFirstPage ? (
        <span className={disabledNavClass} aria-disabled="true">
          <svg
            /* mr-1: 右マージン 4px */
            className="mr-1 h-4 w-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={2}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              /* 左向きの「<」アイコン */
              d="M15.75 19.5L8.25 12l7.5-7.5"
            />
          </svg>
          前へ
        </span>
      ) : (
        // Link は Next.js のクライアントサイドナビゲーション用
        <Link
          // 一つ前のページURL
          href={createPageUrl(currentPage - 1)}
          className={enabledNavClass}
          aria-label="前のページへ"
        >
          <svg
            className="mr-1 h-4 w-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={2}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M15.75 19.5L8.25 12l7.5-7.5"
            />
          </svg>
          前へ
        </Link>
      )}

      {/* ページ番号リスト */}
      <div className="flex items-center gap-1">
        {/* pageNumbers 配列を1件ずつ展開 */}
        {pageNumbers.map((page, index) => {
          // 要素が省略記号 "ellipsis" の場合 → "..." を表示
          if (page === "ellipsis") {
            return (
              <span
                // 同じキーが重複しないよう index を含める
                key={`ellipsis-${index}`}
                className="inline-flex h-10 w-10 items-center justify-center text-gray-400"
                // 省略記号は装飾なのでスクリーンリーダーには無視させる
                aria-hidden="true"
              >
                ...
              </span>
            );
          }

          // 数値の場合 → 現在のページか他のページかで表示分岐
          const isActive = page === currentPage;

          return isActive ? (
            // アクティブ（現在）ページは span（リンクではない）
            <span
              key={page}
              className={activePageClass}
              // ARIA 属性: 「現在のページ」と伝える
              aria-current="page"
              aria-label={`${page}ページ目（現在のページ）`}
            >
              {page}
            </span>
          ) : (
            // 他のページは Link で遷移可能に
            <Link
              key={page}
              href={createPageUrl(page)}
              className={inactivePageClass}
              aria-label={`${page}ページ目へ`}
            >
              {page}
            </Link>
          );
        })}
      </div>

      {/* 次のページボタン（最後のページなら無効化） */}
      {isLastPage ? (
        <span className={disabledNavClass} aria-disabled="true">
          次へ
          <svg
            /* ml-1: 左マージン 4px */
            className="ml-1 h-4 w-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={2}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              /* 右向きの「>」アイコン */
              d="M8.25 4.5l7.5 7.5-7.5 7.5"
            />
          </svg>
        </span>
      ) : (
        <Link
          // 次のページURL
          href={createPageUrl(currentPage + 1)}
          className={enabledNavClass}
          aria-label="次のページへ"
        >
          次へ
          <svg
            className="ml-1 h-4 w-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={2}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M8.25 4.5l7.5 7.5-7.5 7.5"
            />
          </svg>
        </Link>
      )}
    </nav>
  );
}

// components/Header.tsx
// 全ページ上部に表示する共通ヘッダー（ナビゲーションバー）

// "use client": このファイルを Client Component として宣言する特別な文字列
// 必ずファイルの一番上（importよりも前）に書く
// これがあるとブラウザで実行され、React フックや状態管理が使える
"use client";

// default import: Next.js の Link コンポーネント
// HTML の <a> よりも高速にページ遷移できる（SPA的な切り替え）
import Link from "next/link";

// named import: usePathname フック（現在のURLパスを取得）
// 注意: import元は "next/navigation"（App Router用）。Pages Router の "next/router" とは別物
import { usePathname } from "next/navigation";

/**
 * 共通ヘッダーコンポーネント
 *
 * すべてのページの上部に表示されるナビゲーションバーです。
 * - アプリ名（クリックでトップページに遷移）
 * - 「書籍を登録する」ボタン
 * を含みます。
 *
 * "use client" を使用する理由:
 * usePathname() フックを使用するため、Client Component にする必要があります。
 * Server Component では React のフック（useState, useEffect, usePathname 等）は使えません。
 */
// export default: このファイルを import するときの主要な対象
// function Header(): 関数コンポーネントの宣言。引数なし、JSX を返す
export default function Header() {
  // 現在のURLパスを取得（アクティブなナビゲーションリンクのスタイル変更に使用）
  // usePathname() の戻り値は "/" や "/books/new" のような文字列
  const pathname = usePathname();

  // JSX を return（ヘッダー全体の構造）
  return (
    // <header>: HTML5 のセマンティックタグ。ページ上部のヘッダーを示す
    // bg-white: 白い背景 / shadow-sm: 控えめな影 / border-b: 下方向に1pxのボーダー / border-gray-200: ボーダー色
    <header className="bg-white shadow-sm border-b border-gray-200">
      {/* 内側の div: ヘッダーの中身を中央寄せ＆最大幅制限する */}
      {/* max-w-7xl: 最大幅1280px / mx-auto: 左右マージンautoで中央寄せ */}
      {/* px-4 sm:px-6 lg:px-8: 画面サイズに応じて左右パディング */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* flex 行: ロゴと右側ナビを横並びに、両端配置、高さ64px */}
        {/* flex: フレックスボックス / items-center: 垂直中央揃え */}
        {/* justify-between: 左右に均等配置（ロゴが左、ナビが右） / h-16: 高さ64px */}
        <div className="flex items-center justify-between h-16">
          {/* アプリ名（ロゴ） */}
          {/* Link: ページリロードなしで遷移する Next.js のコンポーネント */}
          {/* href: 遷移先のパス */}
          <Link
            href="/"
            // text-xl: フォントサイズ20px / font-bold: 太字 / text-gray-900: 濃いグレー
            // hover:text-blue-600: ホバー時に青 / transition duration-200: 200msで滑らかに変化
            className="text-xl font-bold text-gray-900 hover:text-blue-600 transition duration-200"
          >
            書籍管理アプリ
          </Link>

          {/* ナビゲーション */}
          {/* nav: HTML5 のナビゲーション用セマンティックタグ */}
          {/* space-x-4: 子要素間に16pxの水平方向マージン */}
          <nav className="flex items-center space-x-4">
            {/* 書籍一覧リンク */}
            <Link
              href="/"
              // テンプレートリテラル ${} で動的にクラスを切り替える
              // pathname === "/" が true なら青系（現在地ハイライト）、false ならグレー系
              className={`px-3 py-2 rounded-md text-sm font-medium transition duration-200 ${
                pathname === "/"
                  // 現在地：薄青背景＋青文字
                  ? "bg-blue-50 text-blue-700"
                  // 通常：グレー文字、ホバーで反応
                  : "text-gray-600 hover:text-gray-900 hover:bg-gray-50"
              }`}
            >
              書籍一覧
            </Link>

            {/* 書籍登録ボタン */}
            <Link
              href="/books/new"
              // px-4: 左右パディング16px / py-2: 上下パディング8px / rounded-md: 角丸
              // text-sm: 文字サイズ14px / font-medium: やや太字 / transition: 滑らか変化
              className={`px-4 py-2 rounded-md text-sm font-medium transition duration-200 ${
                pathname === "/books/new"
                  // 現在地：濃い青背景＋白文字
                  ? "bg-blue-700 text-white"
                  // 通常：青背景、ホバーで濃く
                  : "bg-blue-600 text-white hover:bg-blue-700"
              }`}
            >
              書籍を登録する
            </Link>
          </nav>
        </div>
      </div>
    </header>
  );
}

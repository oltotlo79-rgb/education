// app/layout.tsx
// 全ページに適用される「ルートレイアウト」。<html> と <body> はここでしか書かない

// type import: Metadata 型だけをインポート（実体はビルド後のコードに残らない）
import type { Metadata } from "next";

// named import: next/font/google から Noto_Sans_JP 関数だけを取り出す
// next/font は Next.js が用意した、フォント最適化のための機能
import { Noto_Sans_JP } from "next/font/google";

// CSS のサイドエフェクトインポート（左辺なし）
// このファイルを読み込んだだけで CSS が適用される
import "./globals.css";

// default import: Header コンポーネントを取り込む
// @/components/Header はプロジェクト直下の components/Header（パスエイリアス @/ = プロジェクト直下）
import Header from "@/components/Header";

/**
 * フォント設定
 *
 * Google Fonts から Noto Sans JP（日本語対応フォント）を読み込みます。
 * next/font/google を使うことで、フォントファイルがビルド時に
 * 自動的に最適化・セルフホスティングされます。
 * （外部の Google Fonts サーバーにリクエストが飛ばないため高速）
 */
const notoSansJP = Noto_Sans_JP({
  // 読み込むサブセット（文字種）。日本語はデフォルトで含まれる
  subsets: ["latin"],
  // 使用するフォントの太さ（通常 / やや太字 / 太字）
  weight: ["400", "500", "700"],
  // フォント読み込み中は代替フォントで表示し、完了後に切り替える
  display: "swap",
  // 重要なフォントなので、HTMLの <link rel="preload"> で先読みする
  preload: true,
});

/**
 * メタデータ設定
 *
 * ブラウザのタブに表示されるタイトルや、
 * 検索エンジンに表示される説明文を設定します。
 */
// export const metadata: Next.js が自動でこの変数を読み取って <head> に反映する
// 型は Metadata（先ほど type import したもの）
export const metadata: Metadata = {
  // <title> タグの中身（ブラウザのタブ表示）
  title: "書籍管理アプリ",
  // <meta name="description"> の中身（検索結果に出る説明文）
  description:
    "読んだ本、読んでいる本、これから読む本を管理するWebアプリケーション",
};

/**
 * ルートレイアウトコンポーネント
 *
 * すべてのページはこのレイアウトの中にレンダリングされます。
 * { children } には各ページのコンテンツが入ります。
 *
 * 構造:
 *   <html>
 *     <body>
 *       <Header />           ← 全ページ共通のヘッダー
 *       <main>{children}</main> ← 各ページの内容
 *     </body>
 *   </html>
 */
// export default function: ファイルの主要な関数として1つだけ default エクスポート
// RootLayout: コンポーネントの名前。Next.js が自動でこの関数を呼ぶ
// { children }: 分割代入で props から children を取り出している
// Readonly<{ children: React.ReactNode }>: 引数の型。children は読み取り専用、React の任意のノード
export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  // JSX を return する。1つの親要素（<html>）で全体を包む必要がある
  return (
    // lang="ja": HTML の言語属性を日本語に。スクリーンリーダーや検索エンジン用
    <html lang="ja">
      {/* body の className: notoSansJP のクラス名 + Tailwind の背景色と最小高さ */}
      {/* テンプレートリテラル `${...}` で文字列を結合 */}
      <body className={`${notoSansJP.className} bg-gray-50 min-h-screen`}>
        {/* 共通ヘッダーを最上部に配置 */}
        <Header />
        {/* main: ページのメインコンテンツ領域 */}
        {/* max-w-7xl: 最大幅1280px / mx-auto: 左右マージン自動で中央寄せ */}
        {/* px-4 sm:px-6 lg:px-8: 画面サイズに応じて左右パディングを変える（モバイル16px → タブレット24px → PC32px） */}
        {/* py-8: 上下パディング32px */}
        <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
          {/* children: 各ページの中身がここに差し込まれる */}
          {children}
        </main>
      </body>
    </html>
  );
}

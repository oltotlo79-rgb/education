// app/layout.tsx での使用例
// ↑ Next.js App Router のルートレイアウト。
//   全ページ共通の <html><body> 構造と、ヘッダー・フッターなどを定義する。

import { PageTransition } from "@/components/PageTransition";
import { ToastProvider } from "@/contexts/ToastContext";
import { ToastContainer } from "@/components/Toast";
import { Header } from "@/components/Header";
import { Footer } from "@/components/Footer";
// グローバル CSS は必ずここで import
import "./globals.css";

export default function RootLayout({
  // 各ページの内容がここに入る
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    // suppressHydrationWarning: ダークモード切替時のSSR/CSR差異警告を抑制
    <html lang="ja" suppressHydrationWarning>
      {/* flex min-h-screen flex-col:
            縦並びで最低でも画面高さ100%を占める → フッターを下端に固定できる */}
      <body className="flex min-h-screen flex-col">
        {/* ToastProvider: 内側の全コンポーネントから useToast を使えるようにする */}
        <ToastProvider>
          <Header />
          {/* main: メインコンテンツのセマンティック要素
              container: tailwind.config の container 設定でレスポンシブ最大幅
              flex-1: 余り領域を全て使う（ヘッダーとフッターの間を埋める）
              py-8: 上下 32px の余白 */}
          <main className="container flex-1 py-8">
            <PageTransition>{children}</PageTransition>
          </main>
          <Footer />
          {/* ToastContainer はトーストを画面右上に固定表示する別ツリー */}
          <ToastContainer />
        </ToastProvider>
      </body>
    </html>
  );
}

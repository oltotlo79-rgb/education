import type { Metadata } from "next";
import Link from "next/link";
import "./globals.css";

export const metadata: Metadata = {
  title: "実験用アプリ",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  console.log("[RootLayout] サーバーで描画しました");
  return (
    <html lang="ja">
      <body>
        <header>
          <Link href="/">ホーム</Link>
        </header>
        <main>{children}</main>
      </body>
    </html>
  );
}

// app/demo/layout.tsx（/demo/a と /demo/b で共通の枠）
import Link from "next/link";
import { Counter } from "./Counter";

export default function DemoLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div>
      {/* レイアウトの中のカウンター（ページを移動しても残ってほしい） */}
      <Counter />
      <nav>
        {/* ❌ ふつうの a タグ */}
        <a href="/demo/b">a タグで B へ</a>
        {/* ✅ Next.js の Link */}
        <Link href="/demo/b">Link で B へ</Link>
        <Link href="/demo/a">Link で A へ</Link>
      </nav>
      {children}
    </div>
  );
}

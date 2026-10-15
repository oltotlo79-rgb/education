// app/nav-demo/page.tsx（Server Component）
import Link from "next/link";
import { SaveButton } from "./SaveButton";

export default function NavDemoPage() {
  return (
    <div>
      {/* Link は「ここへ行けます」と画面に置いておくリンク */}
      <Link href="/books/1">1冊目の詳細へ（Link）</Link>
      {/* router.push は「処理が終わったら移動する」ためのボタン */}
      <SaveButton />
    </div>
  );
}

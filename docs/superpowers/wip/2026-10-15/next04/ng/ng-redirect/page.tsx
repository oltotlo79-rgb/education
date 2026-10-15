// app/ng-redirect/page.tsx
import { redirect } from "next/navigation";

export default async function NgRedirectPage() {
  try {
    // 何かの処理に失敗した、という想定で一覧へ転送したい
    redirect("/shelf");
  } catch (error) {
    console.log("catch で捕まえたもの:", error);
  }
  return <p>ここが表示されてしまう</p>;
}

// app/nav-demo/SaveButton.tsx（Client Component）
"use client";

import { useRouter } from "next/navigation";

export function SaveButton() {
  const router = useRouter();

  const handleClick = async () => {
    // 本当はここで保存処理をする（今回は 0.5 秒待つだけ）
    await new Promise((resolve) => setTimeout(resolve, 500));
    // 保存が終わったら、プログラムから一覧ページへ移動する
    router.push("/books/1");
  };

  return <button onClick={handleClick}>保存して詳細へ（router.push）</button>;
}

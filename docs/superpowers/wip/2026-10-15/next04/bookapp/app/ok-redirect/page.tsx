// app/ok-redirect/page.tsx
import { redirect } from "next/navigation";

// 失敗するかもしれない処理のまね（今回は必ず失敗する）
async function loadSomething(): Promise<boolean> {
  throw new Error("読み込みに失敗しました");
}

export default async function OkRedirectPage() {
  let loaded = false;
  try {
    loaded = await loadSomething();
  } catch (error) {
    console.log("catch で捕まえたもの:", (error as Error).message);
  }

  // redirect は try / catch の「外」で呼ぶ
  if (!loaded) {
    redirect("/shelf");
  }
  return <p>読み込みに成功しました</p>;
}

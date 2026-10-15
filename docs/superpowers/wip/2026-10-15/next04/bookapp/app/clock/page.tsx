// app/clock/page.tsx（OK: アクセスのたびに作り直す）
import { connection } from "next/server";

export default async function ClockPage() {
  // 「このページはアクセスのたびにサーバーで実行する」と宣言する
  await connection();
  const now = new Date().toLocaleTimeString("ja-JP");
  return <p>このページを作った時刻: {now}</p>;
}

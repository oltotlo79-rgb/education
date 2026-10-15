// app/where/ClientLogger.tsx（Client Component）
"use client";

import { useEffect, useState } from "react";

export function ClientLogger() {
  const [count, setCount] = useState(0);

  // window が存在するかどうかで「今どこで動いているか」を判定する
  const place = typeof window === "undefined" ? "サーバー" : "ブラウザ";
  // 描画（関数の実行）のたびにログを出す
  console.log(`[ClientLogger] 描画しました（実行場所: ${place}）`);

  // useEffect の中身は「ブラウザで画面に出た後」にだけ実行される
  useEffect(() => {
    console.log("[ClientLogger] useEffect を実行しました");
  }, []);

  return (
    <button onClick={() => setCount(count + 1)}>押した回数: {count}</button>
  );
}

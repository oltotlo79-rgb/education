"use client";

import { useEffect } from "react";

export function Hello() {
  // (a) 描画のたびに出すログ
  console.log("描画しました");
  useEffect(() => {
    // (b) 画面に出た後に出すログ
    console.log("画面に出ました");
  }, []);
  // (c) ボタンを押したときに出すログ
  return <button onClick={() => console.log("押されました")}>押す</button>;
}

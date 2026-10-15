// app/ok-window/WindowWidth.tsx（Client Component）
"use client";

import { useEffect, useState } from "react";

export function WindowWidth() {
  // 最初は「まだ分からない」を表す null にしておく（サーバーでも同じ値になる）
  const [width, setWidth] = useState<number | null>(null);

  // useEffect の中はブラウザでだけ実行されるので window を安全に使える
  useEffect(() => {
    setWidth(window.innerWidth);
  }, []);

  if (width === null) {
    return <p>画面の幅を調べています...</p>;
  }
  return <p>画面の幅は {width}px です</p>;
}

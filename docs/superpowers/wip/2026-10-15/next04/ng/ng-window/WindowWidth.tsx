// app/ng-window/WindowWidth.tsx（Client Component）
"use client";

export function WindowWidth() {
  // ブラウザの画面幅を読む
  const width = window.innerWidth;
  return <p>画面の幅は {width}px です</p>;
}

// 付録 B-08: useState とイベント（ボタンを押すと、画面の数が変わる）
import { useState } from "react";

export function X01Counter() {
  // useState(最初の値) → [今の値, 値を変える関数]
  const [count, setCount] = useState(0);
  const [open, setOpen] = useState(false);

  return (
    <div className="x-card">
      <p>承認した数: <b>{count}</b> 件</p>
      {/* onClick に「押された時にすること」を渡す。setCount で値を変えると、画面が描き直される */}
      <button onClick={() => setCount(count + 1)}>＋1</button>{" "}
      <button onClick={() => setCount(0)} disabled={count === 0}>
        0 に戻す
      </button>

      {/* 条件で表示を変える: 条件 ? A : B と、条件 && A */}
      <p>{count >= 3 ? "たくさん承認しました" : "まだ少しです"}</p>
      <button onClick={() => setOpen(!open)}>{open ? "説明を閉じる" : "説明を開く"}</button>
      {open && <p className="x-note">開いている時だけ、この説明が出ます。</p>}
    </div>
  );
}

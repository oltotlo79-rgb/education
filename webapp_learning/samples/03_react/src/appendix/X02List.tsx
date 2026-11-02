// 付録 B-09: 配列を一覧にする（map と key）・絞り込み・空の時の表示
import { useState } from "react";

type Item = { id: number; title: string; status: "回覧中" | "差し戻し中" | "完了" };

const ITEMS: Item[] = [
  { id: 1, title: "治具の製作", status: "回覧中" },
  { id: 2, title: "部品の追加工", status: "完了" },
  { id: 3, title: "棚の取り付け", status: "差し戻し中" },
  { id: 4, title: "治具の改造", status: "回覧中" },
];

export function X02List() {
  const [word, setWord] = useState("");
  const [onlyOpen, setOnlyOpen] = useState(false);

  // 表示する物を、毎回の描画のたびに計算する（state にしない）
  const shown = ITEMS.filter((it) => it.title.includes(word)).filter((it) => !onlyOpen || it.status !== "完了");

  return (
    <div className="x-card">
      <input value={word} onChange={(e) => setWord(e.target.value)} placeholder="件名で探す" />{" "}
      <label>
        <input type="checkbox" checked={onlyOpen} onChange={(e) => setOnlyOpen(e.target.checked)} /> 完了を隠す
      </label>
      <p>{shown.length} 件</p>
      {shown.length === 0 ? (
        <p className="x-note">合う依頼がありません</p>
      ) : (
        <ul>
          {/* map で 1 件ずつ <li> にする。key には、変わらない番号（id）を渡す */}
          {shown.map((it) => (
            <li key={it.id}>
              {it.title}（{it.status}）
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}

import { useEffect, useState } from "react";
import { fetchRequests } from "../api.ts";
import type { RequestItem } from "../types.ts";
import { StatusBadge } from "./StatusBadge.tsx";

/** 自分の依頼の一覧。開いたときにサーバーから読み込む */
export function RequestList() {
  const [items, setItems] = useState<RequestItem[] | null>(null); // null = まだ読み込み中
  const [keyword, setKeyword] = useState("");

  useEffect(() => {
    fetchRequests().then(setItems);
  }, []); // [] = 画面を開いたときに 1 回だけ

  if (items === null) return <p>読み込み中…</p>;

  const shown = items.filter((r) => r.title.includes(keyword));
  return (
    <section>
      <h1>自分の依頼</h1>
      <input
        className="search"
        placeholder="件名でしぼる"
        value={keyword}
        onChange={(e) => setKeyword(e.target.value)}
      />
      <table>
        <thead>
          <tr><th>番号</th><th>件名</th><th>希望の納期</th><th>状態</th></tr>
        </thead>
        <tbody>
          {shown.map((r) => (
            <tr key={r.id}>
              <td>{r.id}</td>
              <td>{r.title}</td>
              <td>{r.dueDate}</td>
              <td><StatusBadge status={r.status} /></td>
            </tr>
          ))}
        </tbody>
      </table>
      <p className="hint">{shown.length} 件</p>
    </section>
  );
}

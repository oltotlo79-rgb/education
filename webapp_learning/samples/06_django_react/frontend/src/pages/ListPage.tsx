import { useEffect, useState } from "react";
import { api } from "../api.ts";
import { StatusBadge } from "../components/StatusBadge.tsx";
import type { Me, RequestRow } from "../types.ts";

/** 依頼の一覧。一般の社員は「自分の依頼」、担当者と管理者は「全部」も見られる */
export function ListPage({ me }: { me: Me }) {
  const canSeeAll = me.role !== "member";
  const [mine, setMine] = useState(true);
  const [rows, setRows] = useState<RequestRow[] | null>(null);
  const [error, setError] = useState("");

  useEffect(() => {
    setRows(null);
    api.requests(mine).then(setRows).catch((e: Error) => setError(e.message));
  }, [mine]); // mine が変わったら、読み直す

  return (
    <section>
      <h1>{mine ? "自分の依頼" : "すべての依頼"}</h1>
      <div className="toolbar">
        <a className="btn-primary" href="#/new">新しい依頼</a>
        {canSeeAll && (
          <label><input type="checkbox" checked={!mine} onChange={(e) => setMine(!e.target.checked)} /> すべての依頼を見る</label>
        )}
      </div>
      {error && <p className="errors">{error}</p>}
      {rows === null ? <p>読み込み中…</p> : (
        <table>
          <thead><tr><th>番号</th><th>件名</th><th>希望の納期</th><th>状態</th></tr></thead>
          <tbody>
            {rows.map((r) => (
              <tr key={r.id}>
                <td>{r.id}</td>
                <td><a href={`#/requests/${r.id}`}>{r.title}</a></td>
                <td>{r.due_date}</td>
                <td>
                  <StatusBadge status={r.status} label={r.status_label} />
                  {r.waiting_for && <span className="hint"> {r.waiting_for} の承認待ち</span>}
                </td>
              </tr>
            ))}
            {rows.length === 0 && <tr><td colSpan={4}>依頼はまだありません</td></tr>}
          </tbody>
        </table>
      )}
    </section>
  );
}

import { useCallback, useEffect, useState } from "react";
import { api } from "../api.ts";
import { DrawingPad } from "../components/DrawingPad.tsx";
import { StatusBadge } from "../components/StatusBadge.tsx";
import type { Me, RequestDetail } from "../types.ts";

const time = (s: string | null) => (s ? new Date(s).toLocaleString("ja-JP", { dateStyle: "short", timeStyle: "short" }) : "");

/** 依頼の詳細。回覧の進み具合・差し戻しのコメント・添付・記録と、今できる操作のボタン */
export function DetailPage({ id, me }: { id: number; me: Me }) {
  const [r, setR] = useState<RequestDetail | null>(null);
  const [message, setMessage] = useState("");
  const load = useCallback(() => api.request(id).then(setR).catch((e: Error) => setMessage(e.message)), [id]);
  useEffect(() => { load(); }, [load]);

  async function run(action: string, done: string) {
    try {
      await api.action(id, action);
      setMessage(done);
      await load();
    } catch (e) {
      setMessage(e instanceof Error ? e.message : "できませんでした。");
    }
  }

  if (r === null) return <p>{message || "読み込み中…"}</p>;
  const mine = r.requester_name === me.name;
  const editable = mine && (r.status === "returned" || r.status === "pulled_back");
  return (
    <section>
      <h1>{r.id} {r.title} <StatusBadge status={r.status} label={r.status_label} /></h1>
      {message && <p className="message">{message}</p>}
      <div className="actions left">
        {mine && r.status === "circulating" && <button className="btn-secondary" onClick={() => run("pull-back", "引き戻しました。")}>引き戻す</button>}
        {editable && <a className="btn-primary" href={`#/requests/${r.id}/edit`}>直して出し直す</a>}
        {editable && <button className="btn-secondary" onClick={() => run("withdraw", "取り下げました。")}>取り下げる</button>}
        {me.role !== "member" && r.status === "approved" && <button className="btn-primary" onClick={() => run("assign", "担当になりました。")}>担当になる</button>}
        {r.status === "in_progress" && r.assignee_name === me.name && <button className="btn-primary" onClick={() => run("complete", "完了にしました。")}>完了にする</button>}
      </div>
      <table className="kv">
        <tbody>
          <tr><th>依頼者</th><td>{r.requester_name}</td></tr>
          <tr><th>希望の納期</th><td>{r.due_date}</td></tr>
          <tr><th>内容</th><td className="pre">{r.detail}</td></tr>
          {r.assignee_name && <tr><th>担当者</th><td>{r.assignee_name}</td></tr>}
          {r.return_comments && <tr><th>差し戻しのコメント</th><td className="pre">{r.return_comments}</td></tr>}
          <tr><th>添付</th><td>{r.attachments.length === 0 ? "なし" : r.attachments.map((a) => (
            <a key={a.id} className="file" href={`/api/attachments/${a.id}/`}>{a.original_name}（{Math.ceil(a.size / 1024)} KB）</a>
          ))}</td></tr>
        </tbody>
      </table>
      <h2>回覧（{r.round} 回目）</h2>
      <ol className="steps">
        {r.steps.map((s) => (
          <li key={s.order} className={`step-${s.decision}`}>
            回覧者{s.order} {s.approver_name} — {s.decision_label} {time(s.decided_at)}
          </li>
        ))}
      </ol>
      {r.drawing && r.drawing.shapes.length > 0 && (<><h2>図面</h2><DrawingPad value={r.drawing} /></>)}
      <h2>記録</h2>
      <table>
        <tbody>
          {r.histories.map((h, i) => (
            <tr key={i}><td>{time(h.at)}</td><td>{h.actor_name}</td><td>{h.action}</td><td>{h.note}</td></tr>
          ))}
        </tbody>
      </table>
    </section>
  );
}

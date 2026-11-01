import { useState, type FormEvent } from "react";
import { createRequest, employees } from "../api.ts";

type Props = { onCreated: (id: number) => void };

/** 依頼の入力。入力の値は、全部 state に持つ（制御されたコンポーネント） */
export function RequestForm({ onCreated }: Props) {
  const [title, setTitle] = useState("");
  const [detail, setDetail] = useState("");
  const [dueDate, setDueDate] = useState("");
  const [approvers, setApprovers] = useState<string[]>(["", "", ""]);
  const [errors, setErrors] = useState<string[]>([]);
  const [sending, setSending] = useState(false);

  function setApprover(index: number, value: string) {
    setApprovers((prev) => prev.map((v, i) => (i === index ? value : v)));
  }

  function validate(): string[] {
    const list: string[] = [];
    if (title.trim() === "") list.push("件名を入れてください");
    if (title.length > 100) list.push("件名は 100 文字までです");
    if (dueDate === "") list.push("希望の納期を入れてください");
    const chosen = approvers.filter((a) => a !== "");
    if (chosen.length === 0) list.push("回覧者を 1 人以上選んでください");
    if (new Set(chosen).size !== chosen.length) list.push("同じ人を 2 回選べません");
    return list;
  }

  async function handleSubmit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault(); // ブラウザが自分でページを送り直すのを止める
    const found = validate();
    setErrors(found);
    if (found.length > 0) return;
    setSending(true);
    const created = await createRequest({ title, detail, dueDate, approvers: approvers.filter((a) => a !== "") });
    setSending(false);
    onCreated(created.id);
  }

  return (
    <form onSubmit={handleSubmit}>
      <h1>依頼の入力</h1>
      {errors.length > 0 && (
        <ul className="errors" role="alert">
          {errors.map((m) => <li key={m}>{m}</li>)}
        </ul>
      )}
      <div className="form-grid">
        <label htmlFor="title" className="required">件名</label>
        <input id="title" value={title} onChange={(e) => setTitle(e.target.value)} placeholder="例: 治具の製作" />

        <label htmlFor="detail">内容</label>
        <textarea id="detail" rows={3} value={detail} onChange={(e) => setDetail(e.target.value)} />

        <label htmlFor="due" className="required">希望の納期</label>
        <input id="due" type="date" value={dueDate} onChange={(e) => setDueDate(e.target.value)} />

        <label className="required">回覧者</label>
        <div className="approvers">
          {approvers.map((value, i) => (
            <select key={i} aria-label={`回覧者${i + 1}`} value={value} onChange={(e) => setApprover(i, e.target.value)}>
              <option value="">{i === 0 ? "回覧者1（必須）" : `回覧者${i + 1}（なし）`}</option>
              {employees.map((emp) => (
                <option key={emp.no} value={emp.no}>{emp.name}（{emp.dept}）</option>
              ))}
            </select>
          ))}
        </div>
      </div>
      <div className="actions">
        <button type="submit" className="btn-primary" disabled={sending}>
          {sending ? "送っています…" : "依頼を送る"}
        </button>
      </div>
    </form>
  );
}

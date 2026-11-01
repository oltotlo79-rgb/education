import { useEffect, useState, type FormEvent } from "react";
import { api } from "../api.ts";
import { ApproverPicker } from "../components/ApproverPicker.tsx";
import { DrawingPad, emptyDrawing } from "../components/DrawingPad.tsx";
import type { Drawing, Employee } from "../types.ts";

const MAX_FILE = 10 * 1024 * 1024;
const OK_EXT = /\.(jpe?g|png|pdf)$/i;

/** 依頼の入力。editId があれば「直して出し直す」 */
export function FormPage({ editId }: { editId?: number }) {
  const [title, setTitle] = useState("");
  const [detail, setDetail] = useState("");
  const [dueDate, setDueDate] = useState("");
  const [approvers, setApprovers] = useState<Employee[]>([]);
  const [drawing, setDrawing] = useState<Drawing>(emptyDrawing);
  const [files, setFiles] = useState<File[]>([]);
  const [errors, setErrors] = useState<string[]>([]);
  const [sending, setSending] = useState(false);

  useEffect(() => {
    if (editId === undefined) return;
    api.request(editId).then((r) => {
      setTitle(r.title);
      setDetail(r.detail);
      setDueDate(r.due_date);
      setDrawing(r.drawing ?? emptyDrawing);
      setApprovers(r.steps.map((s) => ({ id: s.approver_id, name: s.approver_name, dept: s.approver_dept, job_title: "" })));
    });
  }, [editId]);

  function validate(): string[] {
    const list: string[] = [];
    if (!title.trim()) list.push("件名を入れてください。");
    if (!dueDate) list.push("希望の納期を入れてください。");
    if (approvers.length === 0) list.push("回覧者を 1 人以上選んでください。");
    for (const f of files) {
      if (!OK_EXT.test(f.name)) list.push(`${f.name}: 添付できるのは JPG・PNG・PDF だけです。`);
      if (f.size > MAX_FILE) list.push(`${f.name}: 1 ファイル 10MB までです。`);
    }
    return list;
  }

  async function handleSubmit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    const found = validate();
    setErrors(found);
    if (found.length > 0) return;
    setSending(true);
    try {
      const input = { title, detail, due_date: dueDate, approver_ids: approvers.map((a) => a.id), drawing };
      let id = editId;
      if (id === undefined) {
        id = (await api.create(input)).id;
      } else {
        await api.action(id, "resubmit", input);
      }
      for (const f of files) await api.upload(id, f); // 添付は、依頼を作ってから 1 つずつ送る
      location.hash = `#/requests/${id}`;
    } catch (err) {
      setErrors([err instanceof Error ? err.message : "送れませんでした。"]);
    } finally {
      setSending(false);
    }
  }

  return (
    <form onSubmit={handleSubmit}>
      <h1>{editId === undefined ? "依頼の入力" : `依頼 ${editId} を直して出し直す`}</h1>
      {errors.length > 0 && <ul className="errors" role="alert">{errors.map((m) => <li key={m}>{m}</li>)}</ul>}
      <div className="form-grid">
        <label htmlFor="title" className="required">件名</label>
        <input id="title" value={title} maxLength={100} onChange={(e) => setTitle(e.target.value)} />
        <label htmlFor="detail">内容</label>
        <textarea id="detail" rows={3} value={detail} onChange={(e) => setDetail(e.target.value)} />
        <label htmlFor="due" className="required">希望の納期</label>
        <input id="due" type="date" value={dueDate} onChange={(e) => setDueDate(e.target.value)} />
        <label className="required">回覧者</label>
        <ApproverPicker value={approvers} onChange={setApprovers} />
        <label htmlFor="files">添付ファイル</label>
        <input id="files" type="file" multiple accept=".jpg,.jpeg,.png,.pdf" onChange={(e) => setFiles([...(e.target.files ?? [])])} />
      </div>
      <h2>図面（ドラッグで四角を描く）</h2>
      <DrawingPad value={drawing} onChange={setDrawing} />
      <div className="actions">
        <button type="button" className="btn-secondary" onClick={() => setDrawing(emptyDrawing)}>図面を消す</button>
        <button type="submit" className="btn-primary" disabled={sending}>{sending ? "送っています…" : "依頼を送る"}</button>
      </div>
    </form>
  );
}

// 付録 B-10: フォーム（入力欄の値を state で持つ・入力の検査・送る）
import { useState, type FormEvent } from "react";

type Form = { title: string; kind: string; urgent: boolean };

export function X03Form() {
  // 入力欄の値を、1 つのオブジェクトの state で持つ
  const [form, setForm] = useState<Form>({ title: "", kind: "new", urgent: false });
  const [error, setError] = useState("");
  const [sent, setSent] = useState<Form | null>(null);

  function handleSubmit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault(); // ブラウザがページを移動しないようにする（React で送る時の決まり）
    if (form.title.trim() === "") {
      setError("件名を入れてください");
      return;
    }
    setError("");
    setSent(form); // 本当のアプリでは、ここで fetch してサーバーへ送る
  }

  return (
    <form className="x-card" onSubmit={handleSubmit}>
      <label>
        件名{" "}
        {/* value と onChange の組: 入力欄の中身は、いつも state と同じになる */}
        <input value={form.title} onChange={(e) => setForm({ ...form, title: e.target.value })} />
      </label>{" "}
      <select value={form.kind} onChange={(e) => setForm({ ...form, kind: e.target.value })}>
        <option value="new">新しく作る</option>
        <option value="fix">直す</option>
      </select>{" "}
      <label>
        <input
          type="checkbox"
          checked={form.urgent}
          onChange={(e) => setForm({ ...form, urgent: e.target.checked })}
        />{" "}
        至急
      </label>{" "}
      <button type="submit">送る</button>
      {error && <p className="x-error">{error}</p>}
      {sent && (
        <p className="x-note">
          送った内容: {sent.title.trim()}／{sent.kind === "new" ? "新しく作る" : "直す"}／
          {sent.urgent ? "至急" : "通常"}
        </p>
      )}
    </form>
  );
}

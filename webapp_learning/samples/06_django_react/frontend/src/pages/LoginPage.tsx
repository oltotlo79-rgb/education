import { useState, type FormEvent } from "react";
import { api } from "../api.ts";

export function LoginPage({ onLogin }: { onLogin: () => void }) {
  const [employeeNo, setEmployeeNo] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");

  async function handleSubmit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setError("");
    try {
      await api.login(employeeNo, password);
      onLogin();
    } catch (err) {
      setError(err instanceof Error ? err.message : "ログインできませんでした。");
    }
  }

  return (
    <form onSubmit={handleSubmit} className="narrow">
      <h1>ログイン</h1>
      {error && <p className="errors" role="alert">{error}</p>}
      <div className="form-grid">
        <label htmlFor="no">従業員番号</label>
        <input id="no" value={employeeNo} onChange={(e) => setEmployeeNo(e.target.value)} autoComplete="username" required />
        <label htmlFor="pw">パスワード</label>
        <input id="pw" type="password" value={password} onChange={(e) => setPassword(e.target.value)} autoComplete="current-password" required />
      </div>
      <div className="actions"><button type="submit" className="btn-primary">ログイン</button></div>
      <p className="hint">60 分操作が無いと、自動でログアウトします。</p>
    </form>
  );
}

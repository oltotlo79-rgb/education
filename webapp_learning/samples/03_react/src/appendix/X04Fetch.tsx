// 付録 B-11: useEffect で、画面が出たあとにデータを取ってくる（読み込み中・失敗・成功）
import { useEffect, useState } from "react";

type Row = { id: number; title: string };

export function X04Fetch({ url }: { url: string }) {
  const [rows, setRows] = useState<Row[] | null>(null); // null は「まだ届いていない」
  const [error, setError] = useState("");

  // useEffect(する事, [見張る値]): 最初の描画のあと、url が変わった時に動く
  useEffect(() => {
    let ignore = false; // 古い結果を捨てるための印
    setRows(null);
    setError("");
    fetch(url)
      .then((res) => {
        if (!res.ok) throw new Error(`サーバーの返事が ${res.status} でした`);
        return res.json() as Promise<Row[]>;
      })
      .then((data) => {
        if (!ignore) setRows(data);
      })
      .catch((e: unknown) => {
        if (!ignore) setError(e instanceof Error ? e.message : String(e));
      });
    // 後片づけ: url が変わった・画面から消えた時に呼ばれる
    return () => {
      ignore = true;
    };
  }, [url]);

  if (error) return <p className="x-card x-error">読み込めませんでした: {error}</p>;
  if (rows === null) return <p className="x-card">読み込み中…</p>;
  return (
    <ul className="x-card">
      {rows.map((r) => (
        <li key={r.id}>
          {r.id}. {r.title}
        </li>
      ))}
    </ul>
  );
}

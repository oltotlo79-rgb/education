import { useEffect, useState } from "react";
import { api } from "../api.ts";
import type { Employee } from "../types.ts";

type Props = { value: Employee[]; onChange: (list: Employee[]) => void; max?: number };

/** 回覧者を、氏名・所属・役職で探して、順番に 3 人まで選ぶ */
export function ApproverPicker({ value, onChange, max = 3 }: Props) {
  const [q, setQ] = useState("");
  const [found, setFound] = useState<Employee[]>([]);

  useEffect(() => {
    // 打つたびにすぐ呼ばず、0.3 秒止まってから探す（サーバーにお願いしすぎない）
    const timer = setTimeout(() => {
      api.employees(q).then(setFound).catch(() => setFound([]));
    }, 300);
    return () => clearTimeout(timer);
  }, [q]);

  const chosenIds = new Set(value.map((e) => e.id));
  return (
    <div className="picker">
      <ol className="chosen">
        {value.map((e, i) => (
          <li key={e.id}>
            回覧者{i + 1}: {e.name}（{e.dept} {e.job_title}）
            <button type="button" className="link" onClick={() => onChange(value.filter((x) => x.id !== e.id))}>外す</button>
          </li>
        ))}
        {value.length === 0 && <li className="hint">まだ選んでいません（1〜{max} 人）</li>}
      </ol>
      {value.length < max && (
        <>
          <input placeholder="氏名・所属・役職で探す" value={q} onChange={(e) => setQ(e.target.value)} aria-label="回覧者を探す" />
          <ul className="found">
            {found.filter((e) => !chosenIds.has(e.id)).slice(0, 5).map((e) => (
              <li key={e.id}>
                <button type="button" className="link" onClick={() => onChange([...value, e])}>
                  ＋ {e.name}（{e.dept} {e.job_title}）
                </button>
              </li>
            ))}
          </ul>
        </>
      )}
    </div>
  );
}

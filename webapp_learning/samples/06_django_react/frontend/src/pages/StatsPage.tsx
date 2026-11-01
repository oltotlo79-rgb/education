import { useEffect, useState } from "react";
import { api } from "../api.ts";

type Bar = { label: string; count: number };

/** 横向きの棒グラフ（部品を使わず、SVG で描く） */
function BarChart({ title, bars }: { title: string; bars: Bar[] }) {
  const max = Math.max(1, ...bars.map((b) => b.count));
  const rowH = 32;
  return (
    <figure className="chart">
      <figcaption>{title}</figcaption>
      <svg viewBox={`0 0 600 ${bars.length * rowH + 10}`} role="img" aria-label={title}>
        {bars.map((b, i) => (
          <g key={b.label} transform={`translate(0 ${i * rowH + 5})`}>
            <text x="110" y="18" textAnchor="end" className="axis">{b.label}</text>
            <rect x="120" y="4" height="20" rx="4" width={(b.count / max) * 420} className="bar" />
            <text x={128 + (b.count / max) * 420} y="19" className="value">{b.count} 件</text>
          </g>
        ))}
      </svg>
    </figure>
  );
}

export function StatsPage() {
  const [data, setData] = useState<{ byStatus: Bar[]; byAssignee: Bar[] } | null>(null);

  useEffect(() => {
    api.stats().then((s) =>
      setData({
        byStatus: s.by_status.map((x) => ({ label: x.status, count: x.count })),
        byAssignee: s.done_by_assignee.map((x) => ({ label: x.name, count: x.count })),
      }),
    );
  }, []);

  if (data === null) return <p>読み込み中…</p>;
  return (
    <section>
      <h1>グラフ</h1>
      <BarChart title="状態ごとの依頼の件数" bars={data.byStatus} />
      <BarChart title="担当者ごとの完了の件数" bars={data.byAssignee} />
    </section>
  );
}

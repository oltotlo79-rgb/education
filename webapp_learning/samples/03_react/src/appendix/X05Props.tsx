// 付録 B-12: 部品に分ける（props で渡す・子から親へ知らせる・children）
import { useState, type ReactNode } from "react";

// 子の部品 1: 値と「押された時の関数」を props で受け取る
function ApproveButton({ name, onApprove }: { name: string; onApprove: (name: string) => void }) {
  return <button onClick={() => onApprove(name)}>{name} さんとして承認</button>;
}

// 子の部品 2: children は、タグの間に書いた中身
function Panel({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section className="x-card">
      <h3>{title}</h3>
      {children}
    </section>
  );
}

export function X05Props() {
  // 状態は「使う部品たちの、いちばん近い親」に置く（ここでは X05Props）
  const [log, setLog] = useState<string[]>([]);

  function handleApprove(name: string) {
    setLog((prev) => [...prev, `${name} さんが承認しました`]); // 前の値から作る時は関数で渡す
  }

  return (
    <Panel title="回覧">
      <ApproveButton name="佐藤" onApprove={handleApprove} />{" "}
      <ApproveButton name="鈴木" onApprove={handleApprove} />
      <ol>
        {log.map((line, i) => (
          <li key={i}>{line}</li>
        ))}
      </ol>
    </Panel>
  );
}

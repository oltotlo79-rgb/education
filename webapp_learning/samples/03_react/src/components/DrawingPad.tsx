import { useState, type PointerEvent } from "react";

type Rect = { kind: "rect"; x: number; y: number; width: number; height: number };
type Drawing = { version: 1; widthMm: number; heightMm: number; shapes: Rect[] };

const sample: Drawing = {
  version: 1,
  widthMm: 297,
  heightMm: 210,
  shapes: [
    { kind: "rect", x: 20, y: 20, width: 120, height: 70 },
    { kind: "rect", x: 160, y: 40, width: 90, height: 120 },
  ],
};

/** 図面を描く。ドラッグで四角を足し、図面は JSON で持つ */
export function DrawingPad() {
  const [drawing, setDrawing] = useState<Drawing>(sample);
  const [start, setStart] = useState<{ x: number; y: number } | null>(null);

  // 画面の上の位置 → 図面の上の位置（mm）
  function toMm(e: PointerEvent<SVGSVGElement>) {
    const box = e.currentTarget.getBoundingClientRect();
    return {
      x: Math.round(((e.clientX - box.left) / box.width) * drawing.widthMm),
      y: Math.round(((e.clientY - box.top) / box.height) * drawing.heightMm),
    };
  }

  function handleUp(e: PointerEvent<SVGSVGElement>) {
    if (start === null) return;
    const end = toMm(e);
    const rect: Rect = {
      kind: "rect",
      x: Math.min(start.x, end.x),
      y: Math.min(start.y, end.y),
      width: Math.abs(end.x - start.x),
      height: Math.abs(end.y - start.y),
    };
    setStart(null);
    if (rect.width < 2 || rect.height < 2) return; // 小さすぎる物は無視
    setDrawing({ ...drawing, shapes: [...drawing.shapes, rect] }); // 新しいオブジェクトを作って入れる
  }

  const json = JSON.stringify(drawing);
  return (
    <section>
      <h1>図面</h1>
      <p className="hint">ドラッグで四角を描きます（{drawing.shapes.length} 個）。</p>
      <svg
        className="pad"
        viewBox={`0 0 ${drawing.widthMm} ${drawing.heightMm}`}
        onPointerDown={(e) => setStart(toMm(e))}
        onPointerUp={handleUp}
      >
        {drawing.shapes.map((s, i) => (
          <rect key={i} x={s.x} y={s.y} width={s.width} height={s.height} />
        ))}
      </svg>
      <div className="actions">
        <button type="button" className="btn-secondary" onClick={() => setDrawing({ ...drawing, shapes: [] })}>全部消す</button>
      </div>
      <h2>保存する JSON（{new TextEncoder().encode(json).length} バイト）</h2>
      <pre className="json">{JSON.stringify(drawing, null, 2)}</pre>
    </section>
  );
}

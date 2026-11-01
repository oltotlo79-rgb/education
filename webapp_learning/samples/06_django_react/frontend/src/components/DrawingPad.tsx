import { useState, type PointerEvent } from "react";
import type { Drawing } from "../types.ts";

export const emptyDrawing: Drawing = { version: 1, widthMm: 297, heightMm: 210, shapes: [] };

type Props = { value: Drawing; onChange?: (d: Drawing) => void };

/** 図面。onChange を渡すとドラッグで四角を描ける。渡さなければ見るだけ（03 の DrawingPad を、親が値を持つ形にした物） */
export function DrawingPad({ value, onChange }: Props) {
  const [start, setStart] = useState<{ x: number; y: number } | null>(null);

  function toMm(e: PointerEvent<SVGSVGElement>) {
    const box = e.currentTarget.getBoundingClientRect();
    return {
      x: Math.round(((e.clientX - box.left) / box.width) * value.widthMm),
      y: Math.round(((e.clientY - box.top) / box.height) * value.heightMm),
    };
  }

  function handleUp(e: PointerEvent<SVGSVGElement>) {
    if (start === null || !onChange) return;
    const end = toMm(e);
    setStart(null);
    const width = Math.abs(end.x - start.x);
    const height = Math.abs(end.y - start.y);
    if (width < 2 || height < 2) return;
    onChange({
      ...value,
      shapes: [...value.shapes, { kind: "rect", x: Math.min(start.x, end.x), y: Math.min(start.y, end.y), width, height }],
    });
  }

  return (
    <svg
      className="pad"
      viewBox={`0 0 ${value.widthMm} ${value.heightMm}`}
      onPointerDown={onChange ? (e) => setStart(toMm(e)) : undefined}
      onPointerUp={handleUp}
    >
      {value.shapes.map((s, i) =>
        s.kind === "rect" ? <rect key={i} x={s.x} y={s.y} width={s.width} height={s.height} /> : null,
      )}
    </svg>
  );
}

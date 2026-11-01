// 図面を JSON で保存する・読み込む

/** 線・四角・文字の 3 種類の図形。kind の値で、どの種類かが決まる */
type Line = { kind: "line"; x1: number; y1: number; x2: number; y2: number };
type Rect = { kind: "rect"; x: number; y: number; width: number; height: number };
type Label = { kind: "text"; x: number; y: number; text: string };
export type Shape = Line | Rect | Label;

export type Drawing = { version: 1; widthMm: number; heightMm: number; shapes: Shape[] };

/** 図形の説明の文を作る。kind で分けると、その種類の項目だけが使える */
export function describe(shape: Shape): string {
  switch (shape.kind) {
    case "line":
      return `線 (${shape.x1},${shape.y1})→(${shape.x2},${shape.y2})`;
    case "rect":
      return `四角 ${shape.width}×${shape.height}`;
    case "text":
      return `文字「${shape.text}」`;
  }
}

/** JSON から読み込んだ物が、本当に Drawing の形かを確かめる（型ガード） */
export function isDrawing(value: unknown): value is Drawing {
  if (typeof value !== "object" || value === null) return false;
  const v = value as Record<string, unknown>;
  return v.version === 1 && typeof v.widthMm === "number" && typeof v.heightMm === "number" && Array.isArray(v.shapes);
}

const drawing: Drawing = {
  version: 1,
  widthMm: 297,
  heightMm: 210,
  shapes: [
    { kind: "rect", x: 10, y: 10, width: 120, height: 60 },
    { kind: "line", x1: 10, y1: 80, x2: 130, y2: 80 },
    { kind: "text", x: 12, y: 95, text: "穴 φ8 × 4" },
  ],
};

// 保存するとき: オブジェクト → JSON の文字
const json = JSON.stringify(drawing);
const bytes = new TextEncoder().encode(json).length;
console.log(json);
console.log(`大きさ: ${bytes} バイト（上限 5MB = ${5 * 1024 * 1024} バイト）`);

// 読み込むとき: JSON の文字 → 何が入っているか分からない物（unknown）→ 確かめてから使う
const loaded: unknown = JSON.parse(json);
if (isDrawing(loaded)) {
  console.log(loaded.shapes.map(describe).join(" / "));
}
console.log(isDrawing(JSON.parse('{"version": 2}'))); // false（形が違う）

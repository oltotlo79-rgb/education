// 図面のパッド: SVG の上をドラッグして四角を描き、JSON を隠し項目（#drawing）に入れる。
// JSON の形は 06 の React 版（DrawingPad.tsx）と同じ。確かめは、サーバー（drawing.rs）でもう一度する。
(() => {
  const pad = document.getElementById("pad");
  const input = document.getElementById("drawing");
  if (!pad || !input) return;

  const NS = "http://www.w3.org/2000/svg";
  let drawing = { version: 1, widthMm: 297, heightMm: 210, shapes: [] };
  try {
    if (input.value) drawing = JSON.parse(input.value); // 直すときは、今の図面から始める
  } catch {
    // 読めなければ、空から始める
  }

  // 画面の位置（px）を、図面の位置（mm）に直す
  const toMm = (e) => {
    const box = pad.getBoundingClientRect();
    return {
      x: Math.round(((e.clientX - box.left) / box.width) * drawing.widthMm),
      y: Math.round(((e.clientY - box.top) / box.height) * drawing.heightMm),
    };
  };

  const el = (name, attrs) => {
    const node = document.createElementNS(NS, name);
    for (const [k, v] of Object.entries(attrs)) node.setAttribute(k, v);
    return node;
  };

  // 図形を描き直し、隠し項目を新しくする（図形が無ければ空にする = 図面なし）
  function render() {
    pad.replaceChildren();
    for (const s of drawing.shapes) {
      if (s.kind === "rect") pad.append(el("rect", { x: s.x, y: s.y, width: s.width, height: s.height }));
      if (s.kind === "line") pad.append(el("line", { x1: s.x1, y1: s.y1, x2: s.x2, y2: s.y2 }));
      if (s.kind === "text") {
        const t = el("text", { x: s.x, y: s.y });
        t.textContent = s.text; // textContent なので、文字は文字のまま（HTML にならない）
        pad.append(t);
      }
    }
    input.value = drawing.shapes.length ? JSON.stringify(drawing) : "";
  }

  let start = null;
  pad.addEventListener("pointerdown", (e) => {
    start = toMm(e);
    pad.setPointerCapture(e.pointerId);
  });
  pad.addEventListener("pointerup", (e) => {
    if (!start) return;
    const end = toMm(e);
    const width = Math.abs(end.x - start.x);
    const height = Math.abs(end.y - start.y);
    if (width >= 2 && height >= 2) {
      drawing.shapes.push({ kind: "rect", x: Math.min(start.x, end.x), y: Math.min(start.y, end.y), width, height });
    }
    start = null;
    render();
  });
  document.getElementById("pad-undo")?.addEventListener("click", () => {
    drawing.shapes.pop();
    render();
  });
  document.getElementById("pad-clear")?.addEventListener("click", () => {
    drawing.shapes = [];
    render();
  });
  render();
})();

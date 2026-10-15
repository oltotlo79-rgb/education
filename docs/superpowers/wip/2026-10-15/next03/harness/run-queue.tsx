import { act, useState } from "react";
import { createRoot } from "react-dom/client";
import { click } from "./util.ts";
function C() {
  const [a, setA] = useState(0);
  const [b, setB] = useState(0);
  const [c, setC] = useState(0);
  return (
    <div>
      <button id="a" onClick={() => { setA(a + 1); console.log("直後の a:", a); setA(a + 1); }}>A</button>
      <button id="b" onClick={() => { setB((p) => p + 1); setB((p) => p + 1); }}>B</button>
      <button id="c" onClick={() => { setC(5); setC((p) => p * 2); }}>C</button>
      <p id="out">{a},{b},{c}</p>
    </div>
  );
}
const root = createRoot(document.getElementById("root")!);
await act(async () => root.render(<C />));
await click(document.getElementById("a")!);
await click(document.getElementById("b")!);
await click(document.getElementById("c")!);
console.log(document.getElementById("out")!.textContent);

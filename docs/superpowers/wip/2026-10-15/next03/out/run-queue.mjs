// harness/run-queue.tsx
import { act as act2, useState } from "react";
import { createRoot } from "react-dom/client";

// harness/util.ts
import { act } from "react";
async function click(el) {
  await act(async () => {
    el.dispatchEvent(new window.MouseEvent("click", { bubbles: true }));
  });
}

// harness/run-queue.tsx
import { jsx, jsxs } from "react/jsx-runtime";
function C() {
  const [a, setA] = useState(0);
  const [b, setB] = useState(0);
  const [c, setC] = useState(0);
  return /* @__PURE__ */ jsxs("div", { children: [
    /* @__PURE__ */ jsx("button", { id: "a", onClick: () => {
      setA(a + 1);
      console.log("\u76F4\u5F8C\u306E a:", a);
      setA(a + 1);
    }, children: "A" }),
    /* @__PURE__ */ jsx("button", { id: "b", onClick: () => {
      setB((p) => p + 1);
      setB((p) => p + 1);
    }, children: "B" }),
    /* @__PURE__ */ jsx("button", { id: "c", onClick: () => {
      setC(5);
      setC((p) => p * 2);
    }, children: "C" }),
    /* @__PURE__ */ jsxs("p", { id: "out", children: [
      a,
      ",",
      b,
      ",",
      c
    ] })
  ] });
}
var root = createRoot(document.getElementById("root"));
await act2(async () => root.render(/* @__PURE__ */ jsx(C, {})));
await click(document.getElementById("a"));
await click(document.getElementById("b"));
await click(document.getElementById("c"));
console.log(document.getElementById("out").textContent);

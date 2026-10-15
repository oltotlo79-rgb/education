// harness/run-q.tsx
import { act as act2, useEffect, useState } from "react";
import { createRoot } from "react-dom/client";

// harness/util.ts
import { act } from "react";
async function click(el) {
  await act(async () => {
    el.dispatchEvent(new window.MouseEvent("click", { bubbles: true }));
  });
}

// harness/run-q.tsx
import { jsx } from "react/jsx-runtime";
function Clock({ city }) {
  console.log("A: \u30EC\u30F3\u30C0\u30FC", city);
  useEffect(() => {
    console.log("B: effect", city);
    return () => console.log("C: \u7247\u4ED8\u3051", city);
  }, [city]);
  return /* @__PURE__ */ jsx("p", { children: city });
}
var root = createRoot(document.getElementById("root"));
await act2(async () => root.render(/* @__PURE__ */ jsx(Clock, { city: "\u6771\u4EAC" })));
await act2(async () => root.render(/* @__PURE__ */ jsx(Clock, { city: "\u5927\u962A" })));
await act2(async () => root.render(/* @__PURE__ */ jsx("div", {})));
function Q3() {
  const [count, setCount] = useState(0);
  const handleClick = () => {
    setCount(count + 1);
    setCount(count + 1);
    setCount((prev) => prev + 1);
  };
  return /* @__PURE__ */ jsx("button", { onClick: handleClick, children: count });
}
await act2(async () => root.render(/* @__PURE__ */ jsx(Q3, {})));
await click(document.querySelector("button"));
console.log("Q3:", document.querySelector("button").textContent);

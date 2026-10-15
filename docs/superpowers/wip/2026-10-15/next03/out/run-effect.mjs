// harness/run-effect.tsx
import { act as act2, StrictMode } from "react";
import { createRoot } from "react-dom/client";

// effect-log.tsx
import { useEffect, useState } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
function CountLogger({ count }) {
  console.log(`\u2460 \u30EC\u30F3\u30C0\u30FC\uFF08count = ${count}\uFF09`);
  useEffect(() => {
    console.log(`\u2461 effect \u5B9F\u884C\uFF08count = ${count}\uFF09`);
    return () => {
      console.log(`\u2462 \u30AF\u30EA\u30FC\u30F3\u30A2\u30C3\u30D7\uFF08count = ${count} \u306E effect \u3092\u7247\u4ED8\u3051\uFF09`);
    };
  }, [count]);
  return /* @__PURE__ */ jsxs("p", { children: [
    "count: ",
    count
  ] });
}
function EffectTimingDemo() {
  const [count, setCount] = useState(0);
  const [isVisible, setIsVisible] = useState(true);
  return /* @__PURE__ */ jsxs("div", { children: [
    isVisible && /* @__PURE__ */ jsx(CountLogger, { count }),
    /* @__PURE__ */ jsx("button", { onClick: () => setCount(count + 1), children: "+1" }),
    /* @__PURE__ */ jsx("button", { onClick: () => setIsVisible(false), children: "\u6D88\u3059" })
  ] });
}

// harness/util.ts
import { act } from "react";
async function click(el) {
  await act(async () => {
    el.dispatchEvent(new window.MouseEvent("click", { bubbles: true }));
  });
}

// harness/run-effect.tsx
import { jsx as jsx2 } from "react/jsx-runtime";
var strict = process.argv.includes("--strict");
var root = createRoot(document.getElementById("root"));
console.log("--- \u521D\u56DE\u8868\u793A ---");
await act2(async () => root.render(strict ? /* @__PURE__ */ jsx2(StrictMode, { children: /* @__PURE__ */ jsx2(EffectTimingDemo, {}) }) : /* @__PURE__ */ jsx2(EffectTimingDemo, {})));
var [plus, hide] = [...document.querySelectorAll("button")];
console.log("--- +1 \u3092\u62BC\u3059 ---");
await click(plus);
console.log("--- \u6D88\u3059 \u3092\u62BC\u3059 ---");
await click(hide);

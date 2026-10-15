// harness/run-warn.tsx
import { act as act2, useState, useEffect } from "react";
import { createRoot } from "react-dom/client";

// harness/util.ts
import { act } from "react";
async function typeInto(el, value) {
  const setter = Object.getOwnPropertyDescriptor(window.HTMLInputElement.prototype, "value").set;
  await act(async () => {
    setter.call(el, value);
    el.dispatchEvent(new window.Event("input", { bubbles: true }));
  });
}

// harness/run-warn.tsx
import { jsx } from "react/jsx-runtime";
var origErr = console.error;
console.error = (...args) => {
  origErr("[console.error]", String(args[0]).slice(0, 400), args.slice(1).map(String).join(" | ").slice(0, 200));
};
var which = process.argv[2];
function T4() {
  const [count, setCount] = useState(0);
  return /* @__PURE__ */ jsx("button", { onClick: setCount(count + 1), children: "+1" });
}
function T5() {
  const [text] = useState("");
  return /* @__PURE__ */ jsx("input", { value: text });
}
function T8() {
  const fruits = ["\u308A\u3093\u3054", "\u30D0\u30CA\u30CA"];
  return /* @__PURE__ */ jsx("ul", { children: fruits.map((f) => /* @__PURE__ */ jsx("li", { children: f })) });
}
function T9({ show }) {
  if (show) {
    const [a] = useState(0);
  }
  const [b] = useState(1);
  return /* @__PURE__ */ jsx("p", { children: b });
}
function T6() {
  const [seconds, setSeconds] = useState(0);
  useEffect(() => {
    const id = setInterval(() => {
      setSeconds(seconds + 1);
    }, 100);
    return () => clearInterval(id);
  }, []);
  return /* @__PURE__ */ jsx("p", { id: "sec", children: seconds });
}
var root = createRoot(document.getElementById("root"), { onUncaughtError: (e) => origErr("[uncaught]", e.message) });
try {
  if (which === "T4") await act2(async () => root.render(/* @__PURE__ */ jsx(T4, {})));
  if (which === "T5") {
    await act2(async () => root.render(/* @__PURE__ */ jsx(T5, {})));
    await typeInto(document.querySelector("input"), "abc");
    console.log("value after typing:", JSON.stringify(document.querySelector("input").value));
  }
  if (which === "T8") await act2(async () => root.render(/* @__PURE__ */ jsx(T8, {})));
  if (which === "T9") {
    await act2(async () => root.render(/* @__PURE__ */ jsx(T9, { show: false })));
    await act2(async () => root.render(/* @__PURE__ */ jsx(T9, { show: true })));
  }
  if (which === "T6") {
    await act2(async () => root.render(/* @__PURE__ */ jsx(T6, {})));
    await act2(async () => {
      await new Promise((r) => setTimeout(r, 550));
    });
    console.log("seconds:", document.getElementById("sec").textContent);
  }
} catch (e) {
  origErr("[thrown]", e.message);
}

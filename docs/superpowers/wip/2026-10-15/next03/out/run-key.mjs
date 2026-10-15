// harness/run-key.tsx
import { act as act2, useState } from "react";
import { createRoot } from "react-dom/client";

// harness/util.ts
import { act } from "react";
async function click(el) {
  await act(async () => {
    el.dispatchEvent(new window.MouseEvent("click", { bubbles: true }));
  });
}
async function typeInto(el, value) {
  const setter = Object.getOwnPropertyDescriptor(window.HTMLInputElement.prototype, "value").set;
  await act(async () => {
    setter.call(el, value);
    el.dispatchEvent(new window.Event("input", { bubbles: true }));
  });
}

// harness/run-key.tsx
import { jsx, jsxs } from "react/jsx-runtime";
function L({ useId }) {
  const [items, setItems] = useState([{ id: 1, name: "\u308A\u3093\u3054" }, { id: 2, name: "\u30D0\u30CA\u30CA" }]);
  return /* @__PURE__ */ jsxs("div", { children: [
    /* @__PURE__ */ jsx("button", { onClick: () => setItems([{ id: 3, name: "\u307F\u304B\u3093" }, ...items]), children: "add" }),
    items.map((f, i) => /* @__PURE__ */ jsxs("div", { children: [
      /* @__PURE__ */ jsx("span", { children: f.name }),
      /* @__PURE__ */ jsx("input", {})
    ] }, useId ? f.id : i))
  ] });
}
for (const useId of [false, true]) {
  const el = document.createElement("div");
  document.body.appendChild(el);
  const root = createRoot(el);
  await act2(async () => root.render(/* @__PURE__ */ jsx(L, { useId })));
  const inputs = el.querySelectorAll("input");
  await typeInto(inputs[0], "\u8D64\u3044");
  await typeInto(inputs[1], "\u9EC4\u8272\u3044");
  await click(el.querySelector("button"));
  console.log(useId ? "id:" : "index:", [...el.querySelectorAll("div > div")].map((d) => d.querySelector("span").textContent + "=" + d.querySelector("input").value).join(", "));
}

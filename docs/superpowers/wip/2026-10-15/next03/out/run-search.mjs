// harness/run-search.tsx
import { act as act2 } from "react";
import { createRoot } from "react-dom/client";

// search.tsx
import { useState } from "react";
import { jsx, jsxs } from "react/jsx-runtime";
function SearchBox({ query, onQueryChange }) {
  return /* @__PURE__ */ jsx(
    "input",
    {
      value: query,
      onChange: (e) => onQueryChange(e.target.value),
      placeholder: "\u30BF\u30A4\u30C8\u30EB\u3067\u691C\u7D22"
    }
  );
}
function BookList({ books }) {
  if (books.length === 0) {
    return /* @__PURE__ */ jsx("p", { children: "\u898B\u3064\u304B\u308A\u307E\u305B\u3093\u3067\u3057\u305F" });
  }
  return /* @__PURE__ */ jsx("ul", { children: books.map((book) => /* @__PURE__ */ jsx("li", { children: book.title }, book.id)) });
}
var ALL_BOOKS = [
  { id: 1, title: "React\u5165\u9580" },
  { id: 2, title: "TypeScript\u5B9F\u8DF5" },
  { id: 3, title: "React Hooks \u5FB9\u5E95\u89E3\u8AAC" }
];
function BookSearchPage() {
  const [query, setQuery] = useState("");
  const visibleBooks = ALL_BOOKS.filter((book) => book.title.includes(query));
  return /* @__PURE__ */ jsxs("div", { children: [
    /* @__PURE__ */ jsx("h2", { children: "\u66F8\u7C4D\u691C\u7D22" }),
    /* @__PURE__ */ jsx(SearchBox, { query, onQueryChange: setQuery }),
    /* @__PURE__ */ jsxs("p", { children: [
      visibleBooks.length,
      "\u4EF6\u30D2\u30C3\u30C8"
    ] }),
    /* @__PURE__ */ jsx(BookList, { books: visibleBooks })
  ] });
}

// harness/util.ts
import { act } from "react";
async function typeInto(el, value) {
  const setter = Object.getOwnPropertyDescriptor(window.HTMLInputElement.prototype, "value").set;
  await act(async () => {
    setter.call(el, value);
    el.dispatchEvent(new window.Event("input", { bubbles: true }));
  });
}

// harness/run-search.tsx
import { jsx as jsx2 } from "react/jsx-runtime";
var root = createRoot(document.getElementById("root"));
await act2(async () => root.render(/* @__PURE__ */ jsx2(BookSearchPage, {})));
var show = (label) => {
  const div = document.getElementById("root");
  const lines = [...div.querySelectorAll("h2,p,li")].map((e) => (e.tagName === "LI" ? "\u30FB" : "") + e.textContent);
  console.log(label, JSON.stringify(lines));
};
show("\u521D\u671F");
var input = document.querySelector("input");
await typeInto(input, "React");
show("React");
await typeInto(input, "Vue");
show("Vue");

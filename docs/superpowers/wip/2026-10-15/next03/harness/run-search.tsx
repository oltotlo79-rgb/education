
import { act } from "react";
import { createRoot } from "react-dom/client";
import BookSearchPage from "../search.tsx";
import { typeInto } from "./util.ts";

const root = createRoot(document.getElementById("root")!);
await act(async () => root.render(<BookSearchPage />));
const show = (label: string) => {
  const div = document.getElementById("root")!;
  const lines = [...div.querySelectorAll("h2,p,li")].map((e) => (e.tagName === "LI" ? "・" : "") + e.textContent);
  console.log(label, JSON.stringify(lines));
};
show("初期");
const input = document.querySelector("input")! as HTMLInputElement;
await typeInto(input, "React");
show("React");
await typeInto(input, "Vue");
show("Vue");

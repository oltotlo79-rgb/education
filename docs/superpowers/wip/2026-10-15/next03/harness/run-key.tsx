import { act, useState } from "react";
import { createRoot } from "react-dom/client";
import { click, typeInto } from "./util.ts";
type F = { id: number; name: string };
function L({ useId }: { useId: boolean }) {
  const [items, setItems] = useState<F[]>([{ id: 1, name: "りんご" }, { id: 2, name: "バナナ" }]);
  return (
    <div>
      <button onClick={() => setItems([{ id: 3, name: "みかん" }, ...items])}>add</button>
      {items.map((f, i) => (
        <div key={useId ? f.id : i}><span>{f.name}</span><input /></div>
      ))}
    </div>
  );
}
for (const useId of [false, true]) {
  const el = document.createElement("div"); document.body.appendChild(el);
  const root = createRoot(el);
  await act(async () => root.render(<L useId={useId} />));
  const inputs = el.querySelectorAll("input");
  await typeInto(inputs[0] as HTMLInputElement, "赤い");
  await typeInto(inputs[1] as HTMLInputElement, "黄色い");
  await click(el.querySelector("button")!);
  console.log(useId ? "id:" : "index:", [...el.querySelectorAll("div > div")].map((d) => d.querySelector("span")!.textContent + "=" + (d.querySelector("input") as HTMLInputElement).value).join(", "));
}

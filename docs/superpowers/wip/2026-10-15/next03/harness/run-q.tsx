import { act, useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import { click } from "./util.ts";
function Clock({ city }: { city: string }) {
  console.log("A: レンダー", city);
  useEffect(() => {
    console.log("B: effect", city);
    return () => console.log("C: 片付け", city);
  }, [city]);
  return <p>{city}</p>;
}
const root = createRoot(document.getElementById("root")!);
await act(async () => root.render(<Clock city="東京" />));
await act(async () => root.render(<Clock city="大阪" />));
await act(async () => root.render(<div />));
function Q3() {
  const [count, setCount] = useState(0);
  const handleClick = () => {
    setCount(count + 1);
    setCount(count + 1);
    setCount((prev) => prev + 1);
  };
  return <button onClick={handleClick}>{count}</button>;
}
await act(async () => root.render(<Q3 />));
await click(document.querySelector("button")!);
console.log("Q3:", document.querySelector("button")!.textContent);

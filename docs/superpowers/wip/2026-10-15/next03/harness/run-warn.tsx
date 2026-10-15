import { act, useState, useEffect } from "react";
import { createRoot } from "react-dom/client";
import { typeInto } from "./util.ts";
const origErr = console.error;
console.error = (...args: any[]) => { origErr("[console.error]", String(args[0]).slice(0, 400), args.slice(1).map(String).join(" | ").slice(0,200)); };
const which = process.argv[2];
function T4() {
  const [count, setCount] = useState(0);
  // @ts-ignore
  return <button onClick={setCount(count + 1)}>+1</button>;
}
function T5() {
  const [text] = useState("");
  return <input value={text} />;
}
function T8() {
  const fruits = ["りんご", "バナナ"];
  return <ul>{fruits.map((f) => <li>{f}</li>)}</ul>;
}
function T9({ show }: { show: boolean }) {
  if (show) {
    const [a] = useState(0);
  }
  const [b] = useState(1);
  return <p>{b}</p>;
}
function T6() {
  const [seconds, setSeconds] = useState(0);
  useEffect(() => {
    const id = setInterval(() => {
      setSeconds(seconds + 1);
    }, 100);
    return () => clearInterval(id);
  }, []);
  return <p id="sec">{seconds}</p>;
}
const root = createRoot(document.getElementById("root")!, { onUncaughtError: (e: any) => origErr("[uncaught]", e.message) } as any);
try {
  if (which === "T4") await act(async () => root.render(<T4 />));
  if (which === "T5") { await act(async () => root.render(<T5 />)); await typeInto(document.querySelector("input")!, "abc"); console.log("value after typing:", JSON.stringify((document.querySelector("input") as HTMLInputElement).value)); }
  if (which === "T8") await act(async () => root.render(<T8 />));
  if (which === "T9") { await act(async () => root.render(<T9 show={false} />)); await act(async () => root.render(<T9 show={true} />)); }
  if (which === "T6") { await act(async () => root.render(<T6 />)); await act(async () => { await new Promise((r) => setTimeout(r, 550)); }); console.log("seconds:", document.getElementById("sec")!.textContent); }
} catch (e: any) { origErr("[thrown]", e.message); }

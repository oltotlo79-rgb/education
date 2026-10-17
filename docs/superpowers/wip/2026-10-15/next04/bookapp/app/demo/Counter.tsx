// app/demo/Counter.tsx（Client Component）
"use client";

import { useState } from "react";

export function Counter() {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount(count + 1)}>カウント: {count}</button>;
}

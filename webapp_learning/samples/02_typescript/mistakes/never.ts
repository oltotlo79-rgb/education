type S = "a" | "b" | "c";
export function f(s: S): number {
  switch (s) {
    case "a": return 1;
    case "b": return 2;
    default: { const x: never = s; return x; }
  }
}

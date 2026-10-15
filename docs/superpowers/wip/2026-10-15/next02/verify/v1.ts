type Shape = "circle" | "square" | "triangle" | "pentagon";
function getArea(shape: Shape): number {
  switch (shape) {
    case "circle":
      return Math.PI * 10 * 10;
    case "square":
      return 10 * 10;
    case "triangle":
      return (10 * 10) / 2;
    default:
      const _exhaustiveCheck: never = shape;
      return _exhaustiveCheck;
  }
}
const emptyList = [];
emptyList.push("a");
const emptyStrings = [];
emptyStrings.push("hello");
const s = emptyStrings;
const state = { items: [] };
state.items.push("x");
interface User { name: string; email?: string }
const user: User = { name: "田中" };
console.log(user.email.toUpperCase());
function findItem(id: number): string | null { return id > 0 ? "a" : null; }
const item = findItem(-1);
console.log(item.toUpperCase());
export {};

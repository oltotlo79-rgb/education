function toLabel(value: string | number | boolean): string {
  if (typeof value === "string") {
    // ここでは string
    return `文字列: ${value}`;
  }
  if (typeof value === "number") {
    // ここでは number
    return `数値: ${value.toFixed(1)}`;
  }
  // 残りは boolean だけ
  return `真偽値: ${value ? "はい" : "いいえ"}`;
}

console.log(toLabel("TypeScript"));
console.log(toLabel(3));
console.log(toLabel(false));

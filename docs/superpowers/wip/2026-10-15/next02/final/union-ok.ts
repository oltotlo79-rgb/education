function shout(value: string | number): string {
  // まず typeof で「今は string か？」を確かめる
  if (typeof value === "string") {
    return value.toUpperCase();
  }
  // ここでは number に絞り込まれている
  return value.toString();
}

console.log(shout("hello"));
console.log(shout(42));

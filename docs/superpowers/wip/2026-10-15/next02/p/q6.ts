function last<T>(items: T[]): T | undefined {
  // 要素が無ければ undefined、あれば最後の要素を返す
  if (items.length === 0) {
    return undefined;
  }
  return items[items.length - 1];
}

const lastTitle = last(["TypeScript入門", "React実践ガイド"]);
const lastPrice = last([2980, 3300, 1980]);

console.log(lastTitle);
console.log(lastPrice);
console.log(last<string>([]));

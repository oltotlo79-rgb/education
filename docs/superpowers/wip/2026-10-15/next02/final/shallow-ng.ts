const original = {
  title: "TypeScript入門",
  author: { name: "山田太郎" },
};

// スプレッドでコピーしたつもり
const copy = { ...original };
// コピーの著者名だけを変えたつもり
copy.author.name = "佐藤花子";

console.log(original.author.name);

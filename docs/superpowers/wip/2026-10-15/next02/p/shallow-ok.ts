const original = {
  title: "TypeScript入門",
  author: { name: "山田太郎" },
};

// 方法1: 中のオブジェクトもスプレッドでコピーし直す
const copy1 = { ...original, author: { ...original.author } };
copy1.author.name = "佐藤花子";

// 方法2: structuredClone で丸ごと（深く）コピーする
const copy2 = structuredClone(original);
copy2.author.name = "鈴木一郎";

console.log(original.author.name);
console.log(copy1.author.name);
console.log(copy2.author.name);

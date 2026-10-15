// 書き方1: 分割代入の「外側」に、まとめて型を書く
function describeBook({ title, price }: { title: string; price: number }) {
  return `${title}（${price}円）`;
}

// 書き方2: 型に名前を付けてから使う（React の Props でよく見る形）
interface BookSummary {
  title: string;
  price: number;
}

function describeBook2({ title, price }: BookSummary) {
  return `${title}（${price}円）`;
}

console.log(describeBook({ title: "TypeScript入門", price: 2980 }));
console.log(describeBook2({ title: "React実践ガイド", price: 3300 }));

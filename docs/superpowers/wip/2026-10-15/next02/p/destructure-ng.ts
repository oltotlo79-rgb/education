// 本の情報を受け取って1行の紹介文を作る関数（のつもり）
function describeBook({ title: string, price: number }) {
  return `${title}（${price}円）`;
}

console.log(describeBook({ title: "TypeScript入門", price: 2980 }));

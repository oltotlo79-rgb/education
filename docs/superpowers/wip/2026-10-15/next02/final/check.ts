// 本の価格に送料 500 円を足して返す関数
// 引数 price は number（数値）だけ受け取る、と型で宣言している
function addShipping(price: number): number {
  return price + 500;
}

// 正しい呼び出し: 数値を渡している
console.log(addShipping(1000));

// 間違った呼び出し: 文字列 "1000" を渡している
console.log(addShipping("1000"));

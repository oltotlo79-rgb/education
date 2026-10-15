function addShipping(price: number): number {
  return price + 500;
}

// 文字列は Number() で数値に変換してから渡す
console.log(addShipping(1000));
console.log(addShipping(Number("1000")));

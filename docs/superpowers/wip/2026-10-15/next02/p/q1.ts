// price（単価）・quantity（冊数）・discount（値引き額。省略したら 0）
function calcTotal(
  price: number,
  quantity: number,
  discount: number = 0,
): number {
  return price * quantity - discount;
}

console.log(calcTotal(1000, 3));
console.log(calcTotal(1000, 3, 500));

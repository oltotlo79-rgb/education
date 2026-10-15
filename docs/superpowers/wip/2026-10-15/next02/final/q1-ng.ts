function calcTotal(
  price: number,
  quantity: number,
  discount: number = 0,
): number {
  return price * quantity - discount;
}

console.log(calcTotal("1000", 3));

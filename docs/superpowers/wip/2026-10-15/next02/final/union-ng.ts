function shout(value: string | number): string {
  // string か number か分からないまま、string 専用のメソッドを呼んでいる
  return value.toUpperCase();
}

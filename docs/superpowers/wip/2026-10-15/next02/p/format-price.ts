// 価格は「数値」「文字列（フォームに入力されたまま）」「未定（null）」のどれか
function formatPrice(price: number | string | null): string {
  // ここでの price の型: number | string | null
  if (price === null) {
    // ここでの price の型: null
    return "価格未定";
  }

  // ここまで来たら null の可能性は消えた → number | string
  if (typeof price === "string") {
    // ここでの price の型: string
    // 文字列を数値に変換してから表示する
    const num = Number(price);
    return `${num.toLocaleString("ja-JP")}円`;
  }

  // 残りは number だけ → 数値専用の toLocaleString が使える
  return `${price.toLocaleString("ja-JP")}円`;
}

console.log(formatPrice(2980));
console.log(formatPrice("12800"));
console.log(formatPrice(null));

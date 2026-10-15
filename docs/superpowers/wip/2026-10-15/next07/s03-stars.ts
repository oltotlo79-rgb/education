// RatingStars と同じ考え方で、評価を「★☆」の文字列に変える関数
function ratingToStars(rating: number | null): string {
  // 評価が無いときは早期 return
  if (rating === null) {
    return "評価なし";
  }
  // 1〜5 の範囲に収める（クランプ）
  const clamped = Math.min(5, Math.max(1, rating));
  // 1〜5 の番号ごとに「塗る★／塗らない☆」を決めて、1本の文字列につなげる
  const stars = [1, 2, 3, 4, 5]
    .map((star) => (star <= clamped ? "★" : "☆"))
    .join("");
  return `${stars} (${clamped})`;
}

// いろいろな評価値で試す（0 と 9 は範囲外、null は未入力）
const samples = [4, 1, 5, 0, 9, null];

for (const rating of samples) {
  console.log(String(rating).padStart(4), "→", ratingToStars(rating));
}

// Tailwind の初期ブレークポイント（この幅「以上」で効く）
const breakpoints: Record<string, number> = {
  sm: 640,
  md: 768,
  lg: 1024,
  xl: 1280,
};

// className の中から「その画面幅で最終的に効くクラス」を探す関数
function activeClass(className: string, width: number): string {
  let winner = "";
  let winnerMin = -1;
  // スペースで区切って、クラスを1つずつ調べる
  for (const token of className.split(" ")) {
    // "lg:grid-cols-3" → prefix = "lg", name = "grid-cols-3"
    const [prefix, name] = token.includes(":")
      ? token.split(":")
      : ["", token];
    // 接頭辞なしは 0px 以上＝いつでも効く
    const minWidth = prefix === "" ? 0 : breakpoints[prefix];
    // 条件を満たすもののうち「より大きい画面向け」が勝つ
    // （Tailwind は sm → md → lg → xl の順に CSS を出力するため）
    if (width >= minWidth && minWidth > winnerMin) {
      winner = name;
      winnerMin = minWidth;
    }
  }
  return winner;
}

// OK: 本章の BookGrid と同じ「小さい画面から順に足す」書き方
const good = "grid-cols-1 md:grid-cols-2 lg:grid-cols-3";
// NG: 「大きい画面から書いて、小さい画面で減らそう」とした書き方
const bad = "grid-cols-4 md:grid-cols-2 sm:grid-cols-1";

for (const width of [500, 800, 1100]) {
  const label = String(width).padStart(4, " ");
  console.log(
    `${label}px  OK→ ${activeClass(good, width).padEnd(11)}  NG→ ${activeClass(bad, width)}`,
  );
}

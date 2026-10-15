// contrast.ts
// "#4f46e5" のような16進数カラーを、0〜1 の「明るさ（相対輝度）」に変える
function luminance(hex: string): number {
  // "#4f46e5" → [0x4f, 0x46, 0xe5] → 0〜1 の範囲に直す
  const [r, g, b] = [1, 3, 5].map(
    (i) => parseInt(hex.slice(i, i + 2), 16) / 255,
  );
  // 人の目の感じ方に合わせて補正する（WCAG の計算式）
  const linear = (c: number) =>
    c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4;
  // 緑を一番明るく、青を一番暗く感じるので重みを変えて足す
  return 0.2126 * linear(r) + 0.7152 * linear(g) + 0.0722 * linear(b);
}

// 2色のコントラスト比（1〜21）を求める
function contrast(text: string, background: string): number {
  const a = luminance(text);
  const b = luminance(background);
  const light = Math.max(a, b);
  const dark = Math.min(a, b);
  return (light + 0.05) / (dark + 0.05);
}

// 調べたい「文字色 / 背景色」の組み合わせ
const pairs: [string, string, string][] = [
  ["本文（ライト）", "#0f172a", "#ffffff"],
  ["本文（ダーク）", "#f8fafc", "#0f172a"],
  ["補足文（ライト）", "#64748b", "#ffffff"],
  ["補足文（ダーク）", "#94a3b8", "#0f172a"],
  ["ボタン文字", "#ffffff", "#4f46e5"],
  ["エラー文字", "#991b1b", "#fef2f2"],
  ["エラー文字（ダーク）", "#fca5a5", "#7f1d1d"],
  ["白文字 / blue-500 のボタン", "#ffffff", "#3b82f6"],
  ["薄すぎる灰色の文字", "#9ca3af", "#ffffff"],
];

for (const [label, text, background] of pairs) {
  const ratio = contrast(text, background);
  // 本文の合格ラインは 4.5:1 以上（WCAG 2.1 AA）
  const result = ratio >= 4.5 ? "合格" : "不合格";
  // 比率を右寄せ5桁で揃えてから、判定と名前を後ろに並べる
  console.log(`${ratio.toFixed(2).padStart(5, " ")}:1  ${result}  ${label}`);
}

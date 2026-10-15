type Props = { color: "blue" | "green" | "yellow"; label: string };

// OK: 完成したクラス名を「辞書」に丸ごと書いておく
const colorClasses = {
  blue: "bg-blue-100 text-blue-700",
  green: "bg-green-100 text-green-700",
  yellow: "bg-yellow-100 text-yellow-700",
};

export function StatusBadge({ color, label }: Props) {
  return <span className={colorClasses[color]}>{label}</span>;
}

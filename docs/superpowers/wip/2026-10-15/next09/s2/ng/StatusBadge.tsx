type Props = { color: "blue" | "green" | "yellow"; label: string };

export function StatusBadge({ color, label }: Props) {
  // NG: クラス名を文字列の組み立てで作っている
  return <span className={`bg-${color}-100 text-${color}-700`}>{label}</span>;
}

// OK: 完成したクラス名を「辞書」に丸ごと書いておき、そこから選ぶ
const colorClasses = {
  blue: "bg-blue-100 text-blue-700",
  green: "bg-green-100 text-green-700",
};
<span className={colorClasses[color]}>{label}</span>;

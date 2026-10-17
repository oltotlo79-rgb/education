// NG: クラス名を文字列の組み立てで作っている
// → ファイルの中に "bg-blue-100" という文字が無いので、CSS が作られない
<span className={`bg-${color}-100 text-${color}-700`}>{label}</span>

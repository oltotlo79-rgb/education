// cn.ts
// 条件つきでクラス名をつなげる小さな関数
// false / null / undefined / "" はまとめて捨てる
function cn(...classes: (string | false | null | undefined)[]): string {
  return classes.filter(Boolean).join(" ");
}

// ページ番号ボタンのクラスを作る関数
function pageButtonClass(isCurrent: boolean, isDisabled: boolean): string {
  return cn(
    // どの状態でも共通のクラス
    "rounded-lg px-3 py-1 text-sm",
    // 現在のページだけ濃い色
    isCurrent && "bg-indigo-600 text-white",
    // それ以外は薄い色
    !isCurrent && "bg-gray-100 text-gray-700",
    // 押せないときは半透明
    isDisabled && "opacity-50",
  );
}

console.log(pageButtonClass(true, false));
console.log(pageButtonClass(false, false));
console.log(pageButtonClass(false, true));

// クリックされたときに呼ぶ関数の型（関数 か、何もなし）
type ClickHandler = (() => void) | undefined;

// <button> の代わりになる小さな模型
// 「描画」のときに onClick を受け取って覚えておき、クリックされたら呼ぶ
function renderButton(label: string, onClick: ClickHandler) {
  console.log(`[描画] ${label} ボタン（onClick の中身: ${typeof onClick}）`);
  return {
    click() {
      console.log(`[クリック] ${label} ボタン`);
      if (onClick) {
        onClick();
      } else {
        console.log("  → onClick が空なので何も起きない");
      }
    },
  };
}

// 本を削除する関数（ここではメッセージを出すだけ）
function deleteBook(id: number): void {
  console.log(`  → 本 ${id} を削除しました`);
}

// OK: アロー関数で包んで「クリックされたら deleteBook(1) を呼ぶ関数」を渡す
const okButton = renderButton("OK", () => deleteBook(1));
// NG: deleteBook(2) を「今ここで」呼び、その戻り値（undefined）を渡してしまう
const ngButton = renderButton("NG", deleteBook(2));

console.log("--- ここでユーザーがクリック ---");
okButton.click();
ngButton.click();

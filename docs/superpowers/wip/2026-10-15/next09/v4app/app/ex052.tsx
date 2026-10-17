// 任意のコンポーネントから呼び出す例
"use client";

// Context にアクセスするカスタムフック
import { useToast } from "@/contexts/ToastContext";

// 削除ボタンコンポーネント。受け取った bookId の書籍を削除する
export function BookDeleteButton({ bookId }: { bookId: string }) {
  // useToast() でトースト操作のためのオブジェクトを取得
  // toast.success / toast.error などのメソッドが使える
  const toast = useToast();

  // 非同期削除処理。async を付けると関数内で await が使える
  const handleDelete = async () => {
    try {
      // fetch で DELETE リクエスト送信。バッククォート ` ` の中で ${} で値を埋め込み可能
      const res = await fetch(`/api/books/${bookId}`, { method: "DELETE" });
      // res.ok は HTTP ステータスが 200-299 なら true
      if (!res.ok) throw new Error("削除に失敗しました");
      // 成功時のトースト（緑色）を表示
      toast.success("削除完了", "書籍が正常に削除されました。");
    } catch (err) {
      // エラー時のトースト（赤色）を表示
      toast.error("エラー", "書籍の削除に失敗しました。もう一度お試しください。");
    }
  };

  return (
    // btn-danger は globals.css で定義した赤い削除ボタン
    <button onClick={handleDelete} className="btn-danger">
      削除
    </button>
  );
}

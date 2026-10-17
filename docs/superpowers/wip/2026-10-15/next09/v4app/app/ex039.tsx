// types/toast.ts
// ↑ トースト通知に使う型をまとめて定義するファイル。
//   型を別ファイルに切り出すと、Context・コンポーネント間で共有しやすい。

// ToastType: トーストの「種類」を表す文字列リテラル型のユニオン（| で連結した複数候補）
// この4つ以外の文字列は代入できなくなる（タイポ防止）
export type ToastType = "success" | "error" | "warning" | "info";

// Toast: 1件のトーストデータを表す型（オブジェクト型）
export type Toast = {
  // 一意な識別子（削除時に使う）
  id: string;
  // 上で定義した4種のどれか
  type: ToastType;
  // 通知のタイトル（必須）
  title: string;
  // 補足メッセージ。? は「省略可能（undefined を許容）」の意味
  message?: string;
  // 表示時間（ミリ秒）。省略時はデフォルト 5000ms = 5秒
  duration?: number;
};

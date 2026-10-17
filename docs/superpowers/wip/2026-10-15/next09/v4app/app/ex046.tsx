// components/Toast.tsx
// ↑ トーストの実際の見た目（UIコンポーネント）を定義するファイル。
//   ToastContext からトースト配列を受け取って画面に描画する。

// クライアントコンポーネント宣言（state を使うため）
"use client";

// React Hooks
import { useState, useEffect, useCallback } from "react";
// 型を別名 ToastType で取り込む（同名のローカル定義と区別するため as でリネーム）
import { Toast as ToastType } from "@/types/toast";
// Context Hook
import { useToast } from "@/contexts/ToastContext";

// 各トーストタイプのアイコンとスタイルをまとめた設定オブジェクト
// 4種類（success/error/warning/info）ごとにアイコンと色を変える
// このように設定を1か所にまとめておくと、新しい種類を追加しやすい（DRY原則）
const toastConfig = {
  // ─── 成功（チェックマークアイコン+緑色） ───
  success: {
    icon: (
      <svg
        /* 20x20 サイズ */
        className="h-5 w-5"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        strokeWidth={2}
        aria-hidden="true"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          /* チェックマーク + 円形のパス */
          d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
        />
      </svg>
    ),
    /* 各色は緑系の組み合わせ。ライトモードは薄背景+濃文字、ダークモードは逆 */
    containerClass:
      "border-green-200 bg-green-50 dark:border-green-800 dark:bg-green-950",
    iconClass: "text-green-600 dark:text-green-400",
    titleClass: "text-green-800 dark:text-green-200",
    messageClass: "text-green-700 dark:text-green-300",
    /* プログレスバーは中間色 */
    progressClass: "bg-green-500",
  },
  // ─── エラー（!アイコン+赤色） ───
  error: {
    icon: (
      <svg
        className="h-5 w-5"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        strokeWidth={2}
        aria-hidden="true"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          /* 円の中に「!」を描くパス */
          d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z"
        />
      </svg>
    ),
    containerClass:
      "border-red-200 bg-red-50 dark:border-red-800 dark:bg-red-950",
    iconClass: "text-red-600 dark:text-red-400",
    titleClass: "text-red-800 dark:text-red-200",
    messageClass: "text-red-700 dark:text-red-300",
    progressClass: "bg-red-500",
  },
  // ─── 警告（三角!アイコン+黄色） ───
  warning: {
    icon: (
      <svg
        className="h-5 w-5"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        strokeWidth={2}
        aria-hidden="true"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          /* 三角形の中に「!」（道路標識のような形） */
          d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z"
        />
      </svg>
    ),
    containerClass:
      "border-yellow-200 bg-yellow-50 dark:border-yellow-800 dark:bg-yellow-950",
    iconClass: "text-yellow-600 dark:text-yellow-400",
    titleClass: "text-yellow-800 dark:text-yellow-200",
    messageClass: "text-yellow-700 dark:text-yellow-300",
    progressClass: "bg-yellow-500",
  },
  // ─── 情報（iアイコン+青色） ───
  info: {
    icon: (
      <svg
        className="h-5 w-5"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        strokeWidth={2}
        aria-hidden="true"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          /* 円の中に「i」（インフォメーション） */
          d="M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z"
        />
      </svg>
    ),
    containerClass:
      "border-blue-200 bg-blue-50 dark:border-blue-800 dark:bg-blue-950",
    iconClass: "text-blue-600 dark:text-blue-400",
    titleClass: "text-blue-800 dark:text-blue-200",
    messageClass: "text-blue-700 dark:text-blue-300",
    progressClass: "bg-blue-500",
  },
};

// ----------------------------------------------------------------------------
// 個別のトーストアイテム（1件分の見た目）
// ----------------------------------------------------------------------------
function ToastItem({ toast }: { toast: ToastType }) {
  // Context から「削除関数」を取り出す
  const { removeToast } = useToast();
  // isExiting: 閉じるアニメーション中かどうか（true で出ていくアニメに切替）
  const [isExiting, setIsExiting] = useState(false);
  // 種類に応じた設定（アイコン・色）を取得
  const config = toastConfig[toast.type];

  // 閉じるボタンクリック時のハンドラ
  // useCallback で関数をメモ化（依存が変わらない限り同じ関数を使い回す）
  const handleClose = useCallback(() => {
    // アニメーション開始
    setIsExiting(true);
    // 300ms（アニメーション時間）待ってから state から削除
    setTimeout(() => {
      removeToast(toast.id);
    }, 300);
  }, [removeToast, toast.id]);

  // プログレスバー用のアニメーション時間（指定がなければ5秒）
  // || 演算子: 左が falsy（undefined や 0）なら右を使う
  const duration = toast.duration || 5000;

  return (
    <div
      // ARIA ロール: 警告メッセージとして扱う
      role="alert"
      // スクリーンリーダーに即読み上げさせる
      aria-live="assertive"
      // 内容変更時に全体を読み直す（部分だけ読まない）
      aria-atomic="true"
      className={`
        relative
        flex w-full max-w-sm items-start gap-3
        overflow-hidden
        rounded-lg border
        p-4
        shadow-lg
        ${config.containerClass}
        ${isExiting ? "animate-toast-out" : "animate-toast-in"}
      `}
      /* ▼ クラスの読み解き ▼
         relative                : 子要素の absolute（プログレスバー）の基準
         flex w-full max-w-sm    : flex 横並び、幅100%、最大幅 24rem (384px)
         items-start             : 縦方向の上揃え（アイコンとテキストを上揃え）
         gap-3                   : 子要素間 0.75rem (12px)
         overflow-hidden         : プログレスバーがはみ出ないように
         rounded-lg border       : 角丸 + 枠線
         p-4                     : 内側余白 1rem (16px)
         shadow-lg               : 大きめの影
         ${config.containerClass}: 種類ごとの色設定（緑/赤/黄/青）
         三項演算子で「閉じる時 → toast-out / 開く時 → toast-in」アニメ切替
      */
    >
      {/* アイコン */}
      <div className={`flex-shrink-0 ${config.iconClass}`}>
        {/* flex-shrink-0: flex 子要素が縮まないようにする（テキストが長くてもアイコンサイズ維持） */}
        {config.icon}
      </div>

      {/* テキスト内容 */}
      <div className="flex-1 min-w-0">
        {/* flex-1: 余り領域を全て使う
            min-w-0: flex 子要素がデフォルトで min-width: auto になり改行されない問題を回避 */}
        <p className={`text-sm font-semibold ${config.titleClass}`}>
          {toast.title}
        </p>
        {/* message があるときだけ補足行を表示 */}
        {toast.message && (
          <p className={`mt-1 text-sm ${config.messageClass}`}>
            {/* mt-1: 上マージン 0.25rem (4px) */}
            {toast.message}
          </p>
        )}
      </div>

      {/* 閉じるボタン */}
      <button
        // フォーム内に置かれても submit にならないよう明示
        type="button"
        onClick={handleClose}
        className={`
          flex-shrink-0
          rounded-md p-1
          opacity-70
          transition-opacity
          hover:opacity-100
          focus:outline-none focus:ring-2 focus:ring-offset-2
          ${config.iconClass}
        `}
        /* opacity-70 → hover:opacity-100: 普段は薄め、ホバーでくっきり */
        // スクリーンリーダー用のボタン説明
        aria-label="通知を閉じる"
      >
        <svg
          // 16x16 の小さい × アイコン
          className="h-4 w-4"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
          strokeWidth={2}
          aria-hidden="true"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            // 「×」を描く2本の斜め線
            d="M6 18L18 6M6 6l12 12"
          />
        </svg>
      </button>

      {/* プログレスバー（残り時間表示） */}
      <div className="absolute bottom-0 left-0 right-0 h-1">
        {/* absolute bottom-0 left-0 right-0: 親の下端いっぱいに横一杯
            h-1: 高さ 0.25rem (4px) */}
        <div
          className={`h-full ${config.progressClass} opacity-30`}
          style={{
            // インラインアニメーション。CSS の animation プロパティを直接指定
            // shrinkWidth は別途 @keyframes で定義（幅を100%→0%に縮める）
            animation: `shrinkWidth ${duration}ms linear forwards`,
          }}
        />
      </div>
    </div>
  );
}

// ----------------------------------------------------------------------------
// トーストコンテナ（画面右上に固定配置するラッパー）
// ----------------------------------------------------------------------------
// レイアウトのルートに1つ置いて、全トーストをここに集約する
export function ToastContainer() {
  // 表示中のトースト配列を取得
  const { toasts } = useToast();

  // 0件なら null を返して描画しない（DOMにも残さない）
  if (toasts.length === 0) return null;

  return (
    <div
      aria-label="通知"
      className="
        fixed
        right-4
        top-4
        z-50
        flex
        flex-col
        gap-3
        sm:right-6
        sm:top-6
      "
      /* ▼ クラスの読み解き ▼
         fixed              : 画面に固定（スクロールしても位置不変）
         right-4 top-4      : 右上から 16px 離れた位置
         z-50               : z-index: 50（他の要素より前面に）
         flex flex-col      : 縦並び
         gap-3              : トースト間 12px の隙間
         sm:right-6 sm:top-6: 640px 以上では 24px に広げる
      */
    >
      {/* トースト配列を1件ずつ ToastItem に展開 */}
      {toasts.map((toast) => (
        <ToastItem key={toast.id} toast={toast} />
      ))}
    </div>
  );
}

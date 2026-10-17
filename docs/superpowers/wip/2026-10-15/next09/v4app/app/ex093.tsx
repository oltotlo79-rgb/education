// components/BookCard.tsx に追加するキーボード対応

// カードが Link で囲まれている場合、Enter/Space で遷移が可能（Link のデフォルト動作）
// それ以外のインタラクティブ要素にも対応する

// 例: 削除確認ダイアログのキーボード対応
// components/ConfirmDialog.tsx
// ↑ モーダル（modal：背景を暗くして他操作を遮るダイアログ）型の確認ボックス。
//   削除など重要操作の前にユーザーに最終確認を求める用途。

"use client";

import { useEffect, useRef, type ReactNode } from "react";

type ConfirmDialogProps = {
  // ダイアログを表示するかどうか
  isOpen: boolean;
  // タイトル
  title: string;
  // 本文メッセージ
  message: string;
  // 確認ボタンの文字（既定: "確認"）
  confirmLabel?: string;
  // キャンセルボタンの文字（既定: "キャンセル"）
  cancelLabel?: string;
  // 確認ボタンの色
  variant?: "danger" | "primary";
  // 確認時のコールバック
  onConfirm: () => void;
  // キャンセル時のコールバック
  onCancel: () => void;
};

export function ConfirmDialog({
  isOpen,
  title,
  message,
  confirmLabel = "確認",
  cancelLabel = "キャンセル",
  variant = "primary",
  onConfirm,
  onCancel,
}: ConfirmDialogProps) {
  // useRef: DOM要素への参照を保持する Hook
  // .current で実際の DOM ノードにアクセスできる
  // ダイアログ本体への参照
  const dialogRef = useRef<HTMLDivElement>(null);
  // キャンセルボタンへの参照
  const cancelButtonRef = useRef<HTMLButtonElement>(null);

  // ダイアログが開いたらキャンセルボタンにフォーカス
  // ※ 確認ボタンではなくキャンセルに当てるのは「誤操作で確定しない」ための配慮
  useEffect(() => {
    if (isOpen) {
      // ?. は null/undefined チェック付きアクセス
      cancelButtonRef.current?.focus();
    }
  }, [isOpen]);

  // Escape キーで閉じる + Tab キーでのフォーカストラップ（ダイアログ内に閉じ込める）
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!isOpen) return;

      // Escape キーでキャンセル
      if (e.key === "Escape") {
        onCancel();
      }

      // Tab キーのフォーカストラップ
      // → Tab/Shift+Tab を押してもダイアログの外に出ないようにする
      if (e.key === "Tab" && dialogRef.current) {
        // ダイアログ内でフォーカス可能な要素を全て取得
        // querySelectorAll: CSS セレクタで子要素を検索する API
        const focusableElements = dialogRef.current.querySelectorAll<HTMLElement>(
          'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
        );
        // 最初
        const firstElement = focusableElements[0];
        // 最後
        const lastElement = focusableElements[focusableElements.length - 1];

        if (e.shiftKey) {
          // Shift+Tab: 逆方向に移動
          // 最初の要素でさらに戻ろうとしたら、最後の要素にラップ
          if (document.activeElement === firstElement) {
            e.preventDefault();
            lastElement.focus();
          }
        } else {
          // Tab: 順方向
          // 最後の要素でさらに進もうとしたら、最初の要素にラップ
          if (document.activeElement === lastElement) {
            e.preventDefault();
            firstElement.focus();
          }
        }
      }
    };

    // document 全体で keydown を監視
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [isOpen, onCancel]);

  // body のスクロールを無効化
  // モーダル表示中に背景をスクロールできてしまうと混乱するため
  useEffect(() => {
    if (isOpen) {
      // スクロール禁止
      document.body.style.overflow = "hidden";
    } else {
      // 元に戻す
      document.body.style.overflow = "";
    }
    return () => {
      // クリーンアップでも復元
      document.body.style.overflow = "";
    };
  }, [isOpen]);

  // 閉じている時は何も描画しない
  if (!isOpen) return null;

  return (
    <div
      className="
        fixed inset-0 z-50
        flex items-center justify-center
        p-4
      "
      /* fixed inset-0 : 画面全体に固定（top:0 right:0 bottom:0 left:0）
         z-50         : 最前面
         flex ... center: 中身を縦横中央に
         p-4          : 画面端からの余白 */
      role="dialog"
      /* 背景操作を遮るモーダルであると伝える */
      aria-modal="true"
      /* タイトルとなる要素のid */
      aria-labelledby="dialog-title"
      /* 説明文となる要素のid */
      aria-describedby="dialog-description"
    >
      {/* オーバーレイ（背景を暗くする半透明の幕） */}
      <div
        className="
          absolute inset-0
          bg-black/50
          animate-fade-in
          backdrop-blur-sm
        "
        /* bg-black/50      : 黒の透明度50%
           backdrop-blur-sm : 背景を少しぼかす */
        /* オーバーレイクリックでキャンセル */
        onClick={onCancel}
        /* スクリーンリーダーから隠す（装飾要素） */
        aria-hidden="true"
      />

      {/* ダイアログ本体 */}
      <div
        /* DOMへの参照を取得 */
        ref={dialogRef}
        className="
          relative
          w-full max-w-md
          animate-scale-in
          rounded-xl
          bg-[var(--color-card)]
          p-6
          shadow-xl
        "
        /* relative                  : オーバーレイより手前に重ねる
           w-full max-w-md           : 幅100%（ただし最大448px）
           animate-scale-in          : 拡大しながら表示
           rounded-xl                : 大きめ角丸
           bg-[var(--color-card)]    : カード背景色（ダーク対応） */
      >
        <h2
          /* aria-labelledby で参照される */
          id="dialog-title"
          className="text-lg font-semibold text-[var(--color-foreground)]"
        >
          {title}
        </h2>

        <p
          /* aria-describedby で参照される */
          id="dialog-description"
          className="mt-2 text-sm text-[var(--color-muted)]"
        >
          {message}
        </p>

        <div className="mt-6 flex justify-end gap-3">
          {/* mt-6: 上マージン 24px / justify-end: 右寄せ / gap-3: ボタン間 12px */}
          <button
            /* ボタンへの参照（初期フォーカス用） */
            ref={cancelButtonRef}
            onClick={onCancel}
            /* 控えめなボタン */
            className="btn-ghost"
          >
            {cancelLabel}
          </button>

          <button
            onClick={onConfirm}
            /* variant に応じて btn-danger（赤）または btn-primary（インディゴ） */
            className={variant === "danger" ? "btn-danger" : "btn-primary"}
          >
            {confirmLabel}
          </button>
        </div>
      </div>
    </div>
  );
}

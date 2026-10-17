// components/AnimatedButton.tsx
// ↑ 押した時のスケール変化、ローディング表示、バリアント切り替えを備えた汎用ボタン。

"use client";

// ButtonHTMLAttributes: <button> 要素が持つ全ての属性（onClick, type, name など）の型
// type を付けて import すると「型のみ」インポート（実行時に消える）
import { type ButtonHTMLAttributes, type ReactNode } from "react";

// props 型: <button> の全属性に独自プロパティを追加する（インターセクション型 &）
type AnimatedButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  // ボタンの種類
  variant?: "primary" | "secondary" | "danger" | "ghost";
  // ボタンの大きさ
  size?: "sm" | "md" | "lg";
  // 処理中フラグ（trueでスピナー表示）
  isLoading?: boolean;
  // ボタン内のラベル
  children: ReactNode;
};

export function AnimatedButton({
  // 既定はプライマリ
  variant = "primary",
  // 既定は中サイズ
  size = "md",
  // 既定は処理していない
  isLoading = false,
  children,
  // 追加クラスを受け取れるように（既定は空）
  className = "",
  // 標準の disabled も受け取る
  disabled,
  // 残りの<button>属性をまとめて受け取る（rest演算子）
  ...props
}: AnimatedButtonProps) {
  // バリアントごとのスタイル（オブジェクトで管理→ variantStyles[variant] で参照）
  const variantStyles = {
    primary: `
      bg-primary-600 text-white
      hover:bg-primary-700
      active:bg-primary-800
      focus-visible:ring-primary-500
      shadow-md hover:shadow-lg
    `,
    secondary: `
      bg-secondary-600 text-white
      hover:bg-secondary-700
      active:bg-secondary-800
      focus-visible:ring-secondary-500
    `,
    danger: `
      bg-red-600 text-white
      hover:bg-red-700
      active:bg-red-800
      focus-visible:ring-red-500
    `,
    ghost: `
      text-gray-600 hover:bg-gray-100
      dark:text-gray-400 dark:hover:bg-gray-800
      focus-visible:ring-gray-500
    `,
  };

  // サイズごとのスタイル
  const sizeStyles = {
    /* 小: 横12px 縦6px 文字12px */
    sm: "px-3 py-1.5 text-xs gap-1.5",
    /* 中: 横16px 縦10px 文字14px */
    md: "px-4 py-2.5 text-sm gap-2",
    /* 大: 横24px 縦12px 文字16px */
    lg: "px-6 py-3 text-base gap-2.5",
  };

  return (
    <button
      className={`
        inline-flex items-center justify-center
        rounded-lg font-medium
        transition-all duration-200 ease-out
        focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-offset-2
        disabled:pointer-events-none disabled:opacity-50
        active:scale-[0.97]
        ${variantStyles[variant]}
        ${sizeStyles[size]}
        ${className}
      `}
      /* active:scale-[0.97] : クリック中（押している瞬間）に97%サイズに縮める → 押下感の演出 */
      /* 任意値構文 [ ] でTailwindにないサイズを直接指定できる */
      /* 処理中も無効化 */
      disabled={disabled || isLoading}
      /* onClick等の追加属性をスプレッドで渡す */
      {...props}
    >
      {/* isLoading が true の時だけスピナーを表示 */}
      {isLoading && (
        <svg
          /* animate-spin: 1秒ループ回転 */
          className="h-4 w-4 animate-spin"
          viewBox="0 0 24 24"
          fill="none"
          aria-hidden="true"
        >
          {/* 円弧（薄い背景の円） */}
          <circle
            /* 透明度25%（薄く） */
            className="opacity-25"
            /* 中心 x座標 */
            cx="12"
            /* 中心 y座標 */
            cy="12"
            /* 半径10 */
            r="10"
            stroke="currentColor"
            strokeWidth="4"
          />
          {/* 回転する濃い部分（パス） */}
          <path
            /* 透明度75%（濃いめ） */
            className="opacity-75"
            fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
          />
        </svg>
      )}
      {children}
    </button>
  );
}

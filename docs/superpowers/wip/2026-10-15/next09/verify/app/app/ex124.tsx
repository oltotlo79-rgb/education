// components/ThemeToggle.tsx
// ↑ テーマ（ライト/ダーク/システム連動）を切り替えるボタンコンポーネント。

"use client";

import { useState, useEffect } from "react";

// Theme 型: 3つのテーマ候補
type Theme = "light" | "dark" | "system";

export function ThemeToggle() {
  // theme: 現在のユーザー選択（初期値は system）
  const [theme, setTheme] = useState<Theme>("system");
  // mounted: コンポーネントがマウント済みかどうか
  // → SSR（サーバーサイドレンダリング）とCSR（クライアント）で
  //   初期描画が異なるとReactが警告を出すので、それを避けるための仕組み
  const [mounted, setMounted] = useState(false);

  // コンポーネントがマウントされた後にテーマを読み込む
  // （SSR 時にミスマッチ＝ハイドレーションエラーが起きるのを防ぐため）
  useEffect(() => {
    // クライアントで初めて true になる
    setMounted(true);

    // localStorage: ブラウザに永続的に値を保存する仕組み（タブを閉じても消えない）
    // as Theme | null: TypeScript の型アサーション。null も許容
    const savedTheme = localStorage.getItem("theme") as Theme | null;
    if (savedTheme) {
      setTheme(savedTheme);
      applyTheme(savedTheme);
    } else {
      applyTheme("system");
    }
  // [] で初回マウント時のみ実行
  }, []);

  // システムのカラースキーム変更を監視
  // ユーザーが OS の設定でダーク↔ライトを切り替えたら自動連動する
  useEffect(() => {
    // system モード以外なら監視不要
    if (theme !== "system") return;

    // matchMedia: メディアクエリをJSから扱う API
    // (prefers-color-scheme: dark): OSがダークモード設定かどうか
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    const handleChange = () => applyTheme("system");

    // OS設定変更時に handleChange を呼ぶ
    mediaQuery.addEventListener("change", handleChange);
    // クリーンアップ: コンポーネントがアンマウントされる時にリスナー解除
    return () => mediaQuery.removeEventListener("change", handleChange);
  // theme が変わるたびに再実行
  }, [theme]);

  // テーマを実際にDOMに適用する関数
  const applyTheme = (newTheme: Theme) => {
    // <html> 要素を取得
    const root = document.documentElement;
    // isDark: 実際にダークモードにすべきかの判定
    // - "dark" 選択時 → true
    // - "system" 選択時 → OS設定に応じて
    const isDark =
      newTheme === "dark" ||
      (newTheme === "system" &&
        window.matchMedia("(prefers-color-scheme: dark)").matches);

    // <html> の class に "dark" を付けたり外したり
    if (isDark) {
      root.classList.add("dark");
    } else {
      root.classList.remove("dark");
    }
  };

  // テーマを切り替えるボタンハンドラ
  // クリックするたび light → dark → system → light → ... と循環
  const toggleTheme = () => {
    const nextTheme: Theme =
      theme === "light" ? "dark" : theme === "dark" ? "system" : "light";

    // state 更新
    setTheme(nextTheme);
    // 永続化
    localStorage.setItem("theme", nextTheme);
    // DOM 反映
    applyTheme(nextTheme);
  };

  // マウント前はプレースホルダーを表示（ハイドレーションエラー防止）
  if (!mounted) {
    return (
      <button
        className="
          inline-flex h-10 w-10 items-center justify-center
          rounded-lg
          text-gray-500
        "
        aria-label="テーマ切り替え"
      >
        <div className="h-5 w-5" />
        {/* 中身は空（プレースホルダー） */}
      </button>
    );
  }

  // テーマに応じたアイコン
  const themeIcon = () => {
    switch (theme) {
      case "light":
        return (
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
              d="M12 3v2.25m6.364.386l-1.591 1.591M21 12h-2.25m-.386 6.364l-1.591-1.591M12 18.75V21m-4.773-4.227l-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z"
            />
          </svg>
        );
      case "dark":
        return (
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
              d="M21.752 15.002A9.718 9.718 0 0118 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 003 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 009.002-5.998z"
            />
          </svg>
        );
      case "system":
        return (
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
              d="M9 17.25v1.007a3 3 0 01-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0115 18.257V17.25m6-12V15a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 15V5.25m18 0A2.25 2.25 0 0018.75 3H5.25A2.25 2.25 0 003 5.25m18 0V12a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 12V5.25"
            />
          </svg>
        );
    }
  };

  // テーマのラベル（画面表示と aria-label に使う）
  const themeLabel =
    theme === "light" ? "ライト" : theme === "dark" ? "ダーク" : "システム";

  return (
    <button
      onClick={toggleTheme}
      className="
        inline-flex h-10 items-center justify-center gap-2
        rounded-lg px-3
        text-gray-600
        transition-colors duration-200
        hover:bg-gray-100 hover:text-gray-900
        dark:text-gray-400
        dark:hover:bg-gray-800 dark:hover:text-gray-100
        focus-visible:outline-hidden focus-visible:ring-2
        focus-visible:ring-primary-500 focus-visible:ring-offset-2
      "
      /* ▼ クラスの読み解き ▼
         inline-flex h-10 ...     : 40px高さの横並びボタン
         gap-2                    : アイコンとラベルの間 8px
         rounded-lg px-3          : 角丸 + 左右パディング 12px
         transition-colors        : 色変化をトランジション
         hover/dark バリアントの組み合わせで多状態対応
      */
      aria-label={`テーマ切り替え（現在: ${themeLabel}モード）`}
      /* マウスを乗せたときのツールチップ */
      title={`${themeLabel}モード`}
    >
      {themeIcon()}
      {/* ラベルは小画面で隠す（hidden）、640px以上で inline 表示 */}
      <span className="hidden text-sm font-medium sm:inline">
        {themeLabel}
      </span>
    </button>
  );
}

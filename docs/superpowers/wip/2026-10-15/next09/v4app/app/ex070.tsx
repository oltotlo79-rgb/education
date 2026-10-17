// components/PageTransition.tsx
// ↑ ページ遷移時に「ふわっ」とフェードイン＋上昇するアニメーションを付けるラッパー。
//   <PageTransition>{children}</PageTransition> のように使う。

"use client";

// 現在の URL パス取得 Hook
import { usePathname } from "next/navigation";
// React の基本 Hook と型
import { useEffect, useState, type ReactNode } from "react";

// type ReactNode: JSXとして描画可能なあらゆる値（文字列・JSX・配列など）の型
type PageTransitionProps = {
  children: ReactNode;
};

export function PageTransition({ children }: PageTransitionProps) {
  // 現在のパス（/books など）
  const pathname = usePathname();
  // 表示状態（アニメ用フラグ）
  const [isVisible, setIsVisible] = useState(false);
  // 表示中の子要素
  const [displayChildren, setDisplayChildren] = useState(children);

  useEffect(() => {
    // pathname または children が変わったときに実行される

    // パスが変わるたびにアニメーションをリセット
    // 一旦非表示状態に
    setIsVisible(false);
    // 子要素を更新
    setDisplayChildren(children);

    // requestAnimationFrame: ブラウザの次の描画タイミングで実行されるコールバック
    // → ブラウザが「非表示状態」を一度描画してから、すぐに「表示」に切り替えるため
    //   アニメーションが確実に発火する
    const timer = requestAnimationFrame(() => {
      setIsVisible(true);
    });

    // クリーンアップ: 次の useEffect 実行前 or アンマウント時にタイマーをキャンセル
    return () => cancelAnimationFrame(timer);
  // この配列の値が変わるたびに再実行
  }, [pathname, children]);

  return (
    <div
      className={`
        transition-all duration-300 ease-out
        ${
          isVisible
            // 表示中 → 元位置 / 不透明
            ? "translate-y-0 opacity-100"
            // 非表示 → 8px下 / 透明
            : "translate-y-2 opacity-0"
        }
      `}
    >
      {displayChildren}
    </div>
  );
}

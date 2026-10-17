// components/BookCardSkeleton.tsx
// ↑ ローディング中（データ読込中）に表示する「骨組み」コンポーネント。
//   実際のコンテンツと同じ大きさのグレーのプレースホルダーを表示することで
//   「もうすぐ何かが表示される」ことをユーザーに伝える。

// 1件分のスケルトン
export function BookCardSkeleton() {
  return (
    <div className="card overflow-hidden">
      {/* サムネイルスケルトン（書籍画像の代わり） */}
      <div
        className="
          aspect-[3/4] w-full
          bg-gradient-to-r from-gray-200 via-gray-100 to-gray-200
          bg-[length:200%_100%]
          animate-shimmer
          dark:from-gray-700 dark:via-gray-600 dark:to-gray-700
        "
        /* ▼ クラスの読み解き ▼
           aspect-[3/4] w-full                : 縦横比 3:4、幅100%
           bg-gradient-to-r ... from/via/to   : 左→右の3色グラデーション（中央だけ明るい）
           bg-[length:200%_100%]              : 背景画像のサイズを横200%に
                                                 → 余分な部分が画面外にあり、shimmer で動かして光が流れる演出
           animate-shimmer                    : tailwind.config で定義したアニメーションを再生
        */
      />

      {/* テキストスケルトン */}
      <div className="flex flex-col gap-2 p-4">
        {/* タイトル行 */}
        <div
          className="
            h-4 w-3/4 rounded
            bg-gradient-to-r from-gray-200 via-gray-100 to-gray-200
            bg-[length:200%_100%]
            animate-shimmer
            dark:from-gray-700 dark:via-gray-600 dark:to-gray-700
          "
          /* h-4: 16px / w-3/4: 幅75% / rounded: 角丸 */
        />
        {/* 著者行（少し遅らせて動かす） */}
        <div
          className="
            h-3 w-1/2 rounded
            bg-gradient-to-r from-gray-200 via-gray-100 to-gray-200
            bg-[length:200%_100%]
            animate-shimmer
            dark:from-gray-700 dark:via-gray-600 dark:to-gray-700
          "
          /* 0.1秒遅れて開始（複数行が同じタイミングだと不自然なため） */
          style={{ animationDelay: "0.1s" }}
        />
        {/* 評価行 */}
        <div
          className="
            mt-2 h-3 w-1/3 rounded
            bg-gradient-to-r from-gray-200 via-gray-100 to-gray-200
            bg-[length:200%_100%]
            animate-shimmer
            dark:from-gray-700 dark:via-gray-600 dark:to-gray-700
          "
          /* さらに遅らせる */
          style={{ animationDelay: "0.2s" }}
        />
      </div>
    </div>
  );
}

// グリッド全体のスケルトン（複数のスケルトンを並べる）
// count はデフォルト 8、? は省略可能を意味する
export function BookGridSkeleton({ count = 8 }: { count?: number }) {
  return (
    <div
      className="
        grid grid-cols-1 gap-4
        sm:grid-cols-2 sm:gap-5
        lg:grid-cols-3 lg:gap-6
        xl:grid-cols-4
      "
      /* BookGrid と同じレスポンシブグリッド設定にすることで、
         ローディング中も実際の表示と同じ配置になる */
    >
      {/* Array.from({ length: count }) で count 個の空配列を作り、map で展開する慣用句 */}
      {/* _ は使わない引数の慣例的名前（i だけ使う） */}
      {Array.from({ length: count }).map((_, i) => (
        <BookCardSkeleton key={i} />
      ))}
    </div>
  );
}

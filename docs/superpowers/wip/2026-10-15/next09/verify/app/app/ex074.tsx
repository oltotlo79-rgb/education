// components/BookCard.tsx
// ↑ 書籍1冊分の見た目を表示するカードコンポーネント。
//   一覧画面 (BookGrid) の中で1セルずつ並ぶ部品。

// "use client" は Next.js App Router で「これはクライアントコンポーネント」と宣言するディレクティブ
// useState / useEffect / onClick などを使うなら必要
"use client";

// 書籍データの型を import（@/ はプロジェクトルートを指すパスエイリアス）
import { Book } from "@/types/book";

// このコンポーネントが受け取る props（外から渡される値）の型を定義
type BookCardProps = {
  // Book 型の書籍データを 1 件
  book: Book;
};

// export function: 名前付きエクスポート（他ファイルから { BookCard } で取り出す）
// 引数の { book } は分割代入：props の book プロパティを直接取り出している
export function BookCard({ book }: BookCardProps) {
  // 読了状況（status）に応じてバッジのスタイルを切り替えるヘルパー関数
  // ※ 関数のなかでJSXを return して、 {statusBadge()} で呼び出す
  const statusBadge = () => {
    switch (book.status) {
      case "reading":
        // badge-primary はインディゴ色のバッジ（globals.css で定義）
        return <span className="badge-primary">読書中</span>;
      case "completed":
        // badge-success は緑色のバッジ
        return <span className="badge-success">読了</span>;
      case "want_to_read":
        // badge-warning は黄色のバッジ
        return <span className="badge-warning">読みたい</span>;
      default:
        // 該当ステータスがなければ何も表示しない
        return null;
    }
  };

  return (
    // <article> は「独立した記事的なコンテンツ」を表すHTML5 タグ
    // 書籍1冊分の情報のかたまりとして意味的に正しい
    <article
      className="
        card-hover
        animate-fade-in
        flex flex-col
        overflow-hidden
        p-0
      "
      /* ▼ クラスの読み解き ▼
         card-hover     : .card（カードのベース）にホバー時の浮き上がりを追加
         animate-fade-in: 表示時に下からフェードインするアニメーション
         flex flex-col  : flex レイアウトで子要素を縦に並べる
         overflow-hidden: はみ出た要素を非表示（画像のはみ出し対策）
         p-0            : padding を 0 に（card のデフォルトpadding解除）
      */
    >
      {/* 書籍のサムネイルエリア */}
      <div
        className="
          relative
          aspect-[3/4]
          w-full
          overflow-hidden
          bg-gradient-to-br from-primary-100 to-primary-200
          dark:from-primary-900 dark:to-primary-800
        "
        /* ▼ クラスの読み解き ▼
           relative      : position: relative。子要素の absolute 配置の基準にする
           aspect-[3/4]  : 縦横比 3:4（本の縦長プロポーション）。任意値構文 [ ]
           w-full        : 幅100%
           overflow-hidden: はみ出し非表示（hover で画像がズームしてもカードからはみ出ない）
           bg-gradient-to-br: 背景を「左上 → 右下」のグラデーションに（br = bottom-right）
           from-primary-100 to-primary-200 : 開始色と終了色
           dark:from-primary-900 dark:to-primary-800: ダークモード時のグラデーション
        */
      >
        {/* book.thumbnailUrl があれば画像を表示、なければプレースホルダーを表示する分岐 */}
        {/* JSX 内の {} は JavaScript 式を埋め込む構文。三項演算子 (条件 ? A : B) で出し分け */}
        {book.thumbnailUrl ? (
          <img
            // 画像URL
            src={book.thumbnailUrl}
            // 代替テキスト（読み上げ・画像非表示時用）
            alt={`「${book.title}」の表紙`}
            className="
              h-full w-full
              object-cover
              transition-transform duration-300
              group-hover:scale-105
            "
            /* ▼ クラスの読み解き ▼
               h-full w-full          : 親要素いっぱいに広げる
               object-cover           : 縦横比を保ちながら親要素を完全に覆う（はみ出る部分はカット）
               transition-transform   : transform プロパティに transition を適用
               duration-300           : 300ms かけて変化
               group-hover:scale-105  : 親要素のうち className="group" を持つもののホバー時に
                                        この要素を 105% に拡大。BookGrid 側の <Link> に group を付けている
            */
            // 画面外の画像は遅延読み込み（パフォーマンス向上）
            loading="lazy"
          />
        ) : (
          /* プレースホルダー（画像がない場合の代替表示） */
          <div
            className="
              flex h-full w-full
              flex-col items-center justify-center
              gap-2
              p-4
              text-primary-400
              dark:text-primary-600
            "
            /* ▼ クラスの読み解き ▼
               flex h-full w-full           : flex レイアウトで親全体に広げる
               flex-col                      : 縦並び
               items-center justify-center   : 縦横とも中央寄せ
               gap-2                         : 子要素間に 0.5rem (8px) の隙間
               p-4                           : 内側余白 1rem (16px)
               text-primary-400              : アイコン色（薄めのインディゴ）
               dark:text-primary-600         : ダーク時の色
            */
          >
            {/* SVG（Scalable Vector Graphics：拡大しても綺麗な画像形式）でアイコンを描く */}
            <svg
              // 48x48 サイズ
              className="h-12 w-12"
              // 塗りつぶしなし
              fill="none"
              // SVG の内部座標系（24x24 のキャンバス）
              viewBox="0 0 24 24"
              // 線の色は親要素の文字色（text-primary-400）を継承
              stroke="currentColor"
              // 線の太さ
              strokeWidth={1.5}
              // スクリーンリーダーに無視させる（装飾アイコンのため）
              aria-hidden="true"
            >
              <path
                // 線の端を丸く
                strokeLinecap="round"
                // 線の折れ角を丸く
                strokeLinejoin="round"
                d="M12 6.042A8.967 8.967 0 006 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 016 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 016-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0018 18a8.967 8.967 0 00-6 2.292m0-14.25v14.25"
                /* ↑ d 属性は SVG のパス（描画命令）。M=移動、A=円弧、C=曲線など */
              />
            </svg>
            <span className="text-xs font-medium">No Image</span>
            {/*  text-xs: 12px / font-medium: 500 の太さ */}
          </div>
        )}

        {/* ステータスバッジ（カード右上に絶対配置） */}
        <div className="absolute right-2 top-2">
          {/* absolute        : position: absolute。親 .relative を基準に配置
              right-2 top-2  : 右から 0.5rem、上から 0.5rem の位置 */}
          {statusBadge()}
        </div>
      </div>

      {/* 書籍情報エリア */}
      <div className="flex flex-1 flex-col gap-1.5 p-4">
        {/* ▼ クラスの読み解き ▼
            flex flex-1 : flex の子で、残り領域を全部使う（伸縮自在）
            flex-col    : 子要素を縦並びに
            gap-1.5     : 子要素間に 0.375rem (6px) の隙間
            p-4         : 内側余白 1rem (16px) */}

        {/* タイトル */}
        <h3
          className="
            line-clamp-2
            text-sm font-semibold
            leading-tight
            text-[var(--color-foreground)]
            transition-colors duration-200
            group-hover:text-primary-600
            dark:group-hover:text-primary-400
            sm:text-base
          "
          /* ▼ クラスの読み解き ▼
             line-clamp-2                    : 2 行を超えたら省略「…」（Tailwind 標準のクラス）
             text-sm font-semibold           : 14px の太字
             leading-tight                   : 行間を狭め（line-height: 1.25）
             text-[var(--color-foreground)]  : 文字色をCSS変数で指定（ダーク対応のため）
             transition-colors duration-200  : 色変化を 200ms かけて滑らかに
             group-hover:text-primary-600    : 親（.group）ホバー時にインディゴへ変色
             dark:group-hover:text-primary-400: ダーク × ホバー時の色
             sm:text-base                    : 640px 以上で 16px に拡大
          */
        >
          {book.title}
        </h3>

        {/* 著者 */}
        <p
          className="
            line-clamp-1
            text-xs text-[var(--color-muted)]
            sm:text-sm
          "
          /* line-clamp-1: 1行で省略
             text-xs (12px) → sm:text-sm (14px に拡大)
             text-[var(--color-muted)]: 補足テキスト用の薄めの色 */
        >
          {book.author}
        </p>

        {/* 評価（星表示） */}
        {/* 条件付きレンダリング: rating が定義済みで 0 より大きい時のみ描画 */}
        {/* && は短絡評価。左が true なら右の JSX を返す、false なら何も返さない */}
        {book.rating !== undefined && book.rating > 0 && (
          <div className="mt-auto flex items-center gap-1 pt-2">
            {/* mt-auto: 上方向の margin を auto に → flex 子要素を一番下に押し下げる
                flex items-center: 横並びで縦中央
                gap-1: 子要素間 0.25rem (4px)
                pt-2: 上パディング 0.5rem (8px) */}

            {/* [1,2,3,4,5] の配列を map で星アイコン5つにループ展開 */}
            {[1, 2, 3, 4, 5].map((star) => (
              <svg
                // React リストの一意キー（必須）
                key={star}
                /* テンプレートリテラル ` ` の中で ${} を使い動的にクラスを切替 */
                /* star が現在の評価以下なら金色（塗る）、それより大きいなら灰色（空き） */
                className={`h-3.5 w-3.5 ${
                  // ! は「null/undefined でない」と保証する非 null アサーション
                  star <= book.rating!
                    ? "text-yellow-400"
                    : "text-gray-300 dark:text-gray-600"
                }`}
                // 塗り色＝親要素の文字色
                fill="currentColor"
                viewBox="0 0 20 20"
                // スクリーンリーダーから隠す（後ろの数値で代替）
                aria-hidden="true"
              >
                {/* 星型のパス */}
                <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
              </svg>
            ))}
            {/* 評価の数値（4.0 など）も併記 */}
            <span className="ml-1 text-xs text-[var(--color-muted)]">
              {/* ml-1: 左マージン 0.25rem (4px) */}
              {book.rating}
            </span>
          </div>
        )}
      </div>
    </article>
  );
}

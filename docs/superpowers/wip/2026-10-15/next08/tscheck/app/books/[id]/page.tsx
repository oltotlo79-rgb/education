// サーバー側で動く Supabase クライアントを作る関数を取り込む（@/ はプロジェクトルートを指すエイリアス）
import { createClient } from "@/lib/supabase/server";
// 404 ページを表示するための Next.js 組み込み関数
import { notFound } from "next/navigation";
// クライアントサイド遷移ができる Next.js のリンクコンポーネント（<a> よりも高速）
import Link from "next/link";
// この後作る削除ボタン（クライアントコンポーネント）
import DeleteButton from "@/components/DeleteButton";

// -------------------------------------------------
// 型定義
// -------------------------------------------------
// Next.js App Router のページコンポーネントは、
// params プロパティを受け取ります。
// [id] フォルダに配置されているので、params.id で
// URLの動的部分を取得できます。
// Next.js 15 以降は params が Promise でラップ
// されている点に注意してください。
// -------------------------------------------------
type Props = {
  // params は「{ id: string } をいずれ返す Promise」型
  params: Promise<{ id: string }>;
};

// -------------------------------------------------
// ステータス表示用のヘルパー関数
// -------------------------------------------------
// データベースに保存されている英語のステータス値を
// 日本語の表示ラベルに変換します。
// 例: "want_to_read" → "読みたい"
// 値とラベルは第7章の StatusBadge と同じにそろえています。
// -------------------------------------------------
// 引数 status は文字列、戻り値も文字列
function getStatusLabel(status: string): string {
  // Record<キーの型, 値の型> は「キーと値の型を指定したオブジェクト」を表す型
  const statusMap: Record<string, string> = {
    // DB の値 "want_to_read" を「読みたい」に対応付け
    want_to_read: "読みたい",
    // DB の値 "reading" を「読書中」に対応付け
    reading: "読書中",
    // DB の値 "completed" を「読了」に対応付け
    completed: "読了",
  };
  // 該当があれば日本語、無ければ元の値をそのまま返す（フォールバック）
  return statusMap[status] || status;
}

// -------------------------------------------------
// ステータスに応じたバッジの色を返すヘルパー関数
// -------------------------------------------------
// ステータスごとに異なる色のバッジを表示することで、
// ひと目で書籍の読書状態がわかるようにします。
// Tailwind CSS のクラス名を文字列として返します。
// -------------------------------------------------
function getStatusColor(status: string): string {
  // status の値で分岐
  switch (status) {
    // 読みたい（まだ読んでいない）とき
    case "want_to_read":
      // 背景は薄い黄、文字は濃い黄（第7章の StatusBadge と同じ色）
      return "bg-yellow-100 text-yellow-800";
    // 読書中のとき
    case "reading":
      // 背景は薄い青、文字は濃い青
      return "bg-blue-100 text-blue-800";
    // 読了のとき
    case "completed":
      // 背景は薄い緑、文字は濃い緑
      return "bg-green-100 text-green-800";
    // 想定外の値が来たとき
    default:
      // フォールバックとしてグレー
      return "bg-gray-100 text-gray-800";
  }
}

// -------------------------------------------------
// 評価を星マークで表示するヘルパー関数
// -------------------------------------------------
// 数値の評価（1〜5）を視覚的な星マーク（★☆）に
// 変換して表示します。
// 例: 3 → "★★★☆☆"
// -------------------------------------------------
// rating は数値または null（評価なし）
function renderStars(rating: number | null): string {
  // null/undefined のときは「未評価」と表示
  if (rating === null || rating === undefined) return "未評価";
  // 評価の数だけ「★」を繰り返す（例: 3 なら "★★★"）
  const filled = "★".repeat(rating);
  // 残りの数だけ「☆」を繰り返す（例: 3 なら "☆☆"）
  const empty = "☆".repeat(5 - rating);
  // 「★★★」+「☆☆」=「★★★☆☆」を返す
  return filled + empty;
}

// -------------------------------------------------
// 書籍詳細ページコンポーネント（Server Component）
// -------------------------------------------------
// このコンポーネントはサーバーサイドで実行されます。
// async 関数として定義することで、コンポーネント内で
// 直接 Supabase へのデータ取得を行えます。
// "use client" を書いていない＝Server Component です。
// -------------------------------------------------
// 分割代入で props から params を取り出し
export default async function BookDetailPage({ params }: Props) {
  // -------------------------------------------------
  // 1. params から書籍IDを取得
  // -------------------------------------------------
  // Next.js 15 以降、params は Promise として渡される
  // ため、await で解決する必要があります。
  // ここで await を忘れると id が undefined になり、
  // データ取得が失敗します。
  // -------------------------------------------------
  // Promise を解決して { id } を取り出す
  const { id } = await params;

  // -------------------------------------------------
  // 2. Supabase クライアントの作成とデータ取得
  // -------------------------------------------------
  // サーバーコンポーネント用の Supabase クライアントを
  // 作成し、指定されたIDの書籍データを取得します。
  // .single() を使うことで、配列ではなく単一の
  // オブジェクトとしてデータを受け取ります。
  // -------------------------------------------------
  // Cookie 連携などの非同期処理があるため await が必要
  const supabase = await createClient();

  // data を book という名前にリネームして受け取る（分割代入のリネーム構文）
  const { data: book, error } = await supabase
    // 操作対象は books テーブル
    .from("books")
    // 全カラムを取得（"id, title" のようにカラム名を絞ることも可能）
    .select("*")
    // WHERE id = :id 条件（URL から取った id と一致するレコード）
    .eq("id", id)
    // 結果を配列ではなく単一オブジェクトとして受け取る（0件や2件以上ならエラー）
    .single();

  // -------------------------------------------------
  // 3. エラーハンドリング
  // -------------------------------------------------
  // データが取得できなかった場合（IDが存在しない、
  // ネットワークエラーなど）、Next.js の notFound()
  // 関数を呼び出して 404 ページを表示します。
  //
  // notFound() は例外をスローする関数なので、
  // この行以降のコードは実行されません。
  // try/catch で囲うとエラー扱いになってしまうので、
  // 直接呼び出すのがポイントです。
  // -------------------------------------------------
  // エラーがある、または book が空のとき
  if (error || !book) {
    // 404 ページに即遷移（以降のコードは実行されない）
    notFound();
  }

  // -------------------------------------------------
  // 4. 日付のフォーマット
  // -------------------------------------------------
  // データベースに保存されている日付文字列を
  // 日本語の表示形式に変換します。
  // 例: "2024-01-15" → "2024年1月15日"
  // -------------------------------------------------
  // アロー関数で日付フォーマッタを定義
  const formatDate = (dateString: string | null): string => {
    // null や空文字なら「未設定」を返す
    if (!dateString) return "未設定";
    // 文字列から Date オブジェクトを生成
    const date = new Date(dateString);
    // 日本ロケールで整形（"ja-JP" は日本語表記）
    return date.toLocaleDateString("ja-JP", {
      // 年は数字
      year: "numeric",
      // 月は「1月」のような長い形式
      month: "long",
      // 日は数字
      day: "numeric",
    });
  };

  // -------------------------------------------------
  // 5. ページのレンダリング
  // -------------------------------------------------
  // 詳細ページは以下のようなレイアウトで表示されます:
  //
  // カード形式のレイアウトで、書籍の全情報を
  // 見やすく表示します。
  // -------------------------------------------------
  return (
    // 最大幅 2xl、左右中央寄せ、上下に余白
    <div className="max-w-2xl mx-auto py-8 px-4">
      {/* -------------------------------------------
          戻るリンク
          一覧ページに戻るためのリンクです。
          ページ上部に配置して、ユーザーがすぐに
          一覧に戻れるようにします。
      ------------------------------------------- */}
      <Link
        // 遷移先の URL
        href="/"
        // インラインフレックスで矢印とテキストを横並びに
        className="inline-flex items-center text-sm text-blue-600 hover:text-blue-800 mb-6"
      >
        <svg
          // 16x16 のサイズ、右に少しマージン
          className="w-4 h-4 mr-1"
          // 塗りつぶしなし
          fill="none"
          // 線の色は親要素の文字色を引き継ぐ
          stroke="currentColor"
          // SVG の表示領域
          viewBox="0 0 24 24"
        >
          <path
            // 線の端を丸く
            strokeLinecap="round"
            // 線同士の結合点も丸く
            strokeLinejoin="round"
            // 線の太さ
            strokeWidth={2}
            // ← の形を描画するパス
            d="M15 19l-7-7 7-7"
          />
        </svg>
        一覧に戻る
      </Link>

      {/* -------------------------------------------
          書籍詳細カード
          白背景のカードに影をつけて、情報を
          グループ化して表示します。
      ------------------------------------------- */}
      {/* 白背景、強めの影、角丸、はみ出し非表示 */}
      <div className="bg-white shadow-lg rounded-lg overflow-hidden">
        {/* -------------------------------------------
            ヘッダー部分
            書籍タイトルとステータスバッジを
            表示します。背景色をグラデーションに
            して視覚的に目立たせます。
        ------------------------------------------- */}
        {/* 左から右へ青のグラデーション */}
        <div className="bg-gradient-to-r from-blue-600 to-blue-800 px-6 py-8">
          {/* タイトルとバッジを左右に振り分ける */}
          <div className="flex items-start justify-between">
            {/* book.title を白文字で大きく表示 */}
            <h1 className="text-2xl font-bold text-white">{book.title}</h1>
            <span
              // テンプレートリテラルで動的にクラスを合成
              className={`inline-flex items-center px-3 py-1 rounded-full text-sm font-medium ${getStatusColor(book.status)}`}
            >
              {/* 日本語ラベルを表示 */}
              {getStatusLabel(book.status)}
            </span>
          </div>
          {/* book.author が真値（空でない）のときだけ描画する短絡評価 */}
          {book.author && (
            <p className="mt-2 text-blue-100 text-lg">{book.author}</p>
          )}
        </div>

        {/* -------------------------------------------
            詳細情報部分
            書籍の各項目を定義リスト風のレイアウトで
            表示します。ラベルと値を横並びにして、
            見やすく整理します。
        ------------------------------------------- */}
        <div className="px-6 py-6">
          {/* dl は定義リスト。子要素間に区切り線を入れる */}
          <dl className="divide-y divide-gray-200">
            {/* 出版社 */}
            {/* sm 以上で 3 カラムグリッド */}
            <div className="py-4 sm:grid sm:grid-cols-3 sm:gap-4">
              {/* dt は定義の見出し（ラベル） */}
              <dt className="text-sm font-medium text-gray-500">出版社</dt>
              {/* dd は定義の本体（値）。2カラム分使う */}
              <dd className="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
                {/* publisher が空なら「未設定」を表示 */}
                {book.publisher || "未設定"}
              </dd>
            </div>

            {/* 出版日 */}
            <div className="py-4 sm:grid sm:grid-cols-3 sm:gap-4">
              <dt className="text-sm font-medium text-gray-500">出版日</dt>
              <dd className="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
                {/* 上で定義したフォーマッタで整形 */}
                {formatDate(book.published_date)}
              </dd>
            </div>

            {/* 評価 */}
            <div className="py-4 sm:grid sm:grid-cols-3 sm:gap-4">
              <dt className="text-sm font-medium text-gray-500">評価</dt>
              <dd className="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
                {/* 星は黄色、少し大きめ */}
                <span className="text-yellow-500 text-lg">
                  {/* 数値を ★★★☆☆ 形式に変換 */}
                  {renderStars(book.rating)}
                </span>
              </dd>
            </div>

            {/* ステータス */}
            <div className="py-4 sm:grid sm:grid-cols-3 sm:gap-4">
              <dt className="text-sm font-medium text-gray-500">ステータス</dt>
              <dd className="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
                <span
                  className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(book.status)}`}
                >
                  {getStatusLabel(book.status)}
                </span>
              </dd>
            </div>

            {/* メモ */}
            <div className="py-4 sm:grid sm:grid-cols-3 sm:gap-4">
              <dt className="text-sm font-medium text-gray-500">メモ</dt>
              <dd className="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
                {/* メモがあれば本文を、なければグレーで「メモはありません」を表示する三項演算子 */}
                {book.memo ? (
                  // whitespace-pre-wrap で改行を保持
                  <div className="bg-gray-50 rounded-md p-4 whitespace-pre-wrap">
                    {book.memo}
                  </div>
                ) : (
                  <span className="text-gray-400">メモはありません</span>
                )}
              </dd>
            </div>

            {/* 登録日時 */}
            <div className="py-4 sm:grid sm:grid-cols-3 sm:gap-4">
              <dt className="text-sm font-medium text-gray-500">登録日時</dt>
              <dd className="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
                {/* DB が自動で入れる作成日時 */}
                {formatDate(book.created_at)}
              </dd>
            </div>
          </dl>
        </div>

        {/* -------------------------------------------
            アクションボタン部分
            編集と削除の操作ボタンを表示します。
            編集はリンク（青色）、削除はボタン
            （赤色）で表示して、操作を視覚的に
            区別します。
        ------------------------------------------- */}
        {/* 右寄せでボタンを横並び、要素間隔 3 */}
        <div className="bg-gray-50 px-6 py-4 flex items-center justify-end space-x-3">
          <Link
            // テンプレートリテラルで動的に編集ページの URL を組み立て
            href={`/books/${book.id}/edit`}
            className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
          >
            <svg
              className="w-4 h-4 mr-2"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                // 鉛筆（編集）アイコンのパス
                d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
              />
            </svg>
            編集する
          </Link>

          {/* ------------------------------------------
              DeleteButton はクライアントコンポーネント
              です。window.confirm による確認ダイアログ
              やクリックイベントの処理はブラウザ側で
              行う必要があるためです。
              サーバーコンポーネントから props を渡せる
              点に注目（id と title を渡している）。
          ------------------------------------------ */}
          <DeleteButton bookId={book.id} bookTitle={book.title} />
        </div>
      </div>
    </div>
  );
}

// このファイルはブラウザ側で動くクライアントコンポーネント。useState/useRouter を使うために必須
"use client";

// 状態管理フック
import { useState } from "react";
// ページ遷移フック（App Router 用）
import { useRouter } from "next/navigation";
// ブラウザ向けの Supabase クライアント
import { createClient } from "@/lib/supabase/client";

// -------------------------------------------------
// 型定義
// -------------------------------------------------
// BookFormData: フォームで扱うデータの型
// initialData に渡す型でもあり、フォームの状態
// 管理にも使用します。
// 数値は null も許容（評価が未入力の場合用）。
// -------------------------------------------------
type BookFormData = {
  // タイトル（必須）
  title: string;
  // 著者
  author: string;
  // 出版社
  publisher: string;
  // 出版日（YYYY-MM-DD 形式の文字列）
  published_date: string;
  // 評価（1〜5 または null）
  rating: number | null;
  // ステータス（"unread" / "reading" / "finished"）
  status: string;
  // メモ
  memo: string;
};

// -------------------------------------------------
// Props の型定義
// -------------------------------------------------
// initialData: 編集時に既存のデータを渡すための
//              プロパティ。新規登録時は undefined。
// isEdit:      編集モードかどうかを示すフラグ。
//              true の場合は UPDATE、false の場合は
//              INSERT を実行します。
// bookId:      編集時に対象の書籍IDを渡すための
//              プロパティ。UPDATE の WHERE 句で使用。
// 末尾の ? は「省略可能」を表す TypeScript の記法です。
// -------------------------------------------------
type Props = {
  // 省略可（新規登録時は渡さない）
  initialData?: BookFormData;
  // 省略可（デフォルト false）
  isEdit?: boolean;
  // 省略可（編集時のみ必要）
  bookId?: string;
};

// -------------------------------------------------
// デフォルトの初期値
// -------------------------------------------------
// 新規登録時に使用するフォームの初期値です。
// すべてのフィールドを空またはデフォルト値で
// 初期化します。
// -------------------------------------------------
const defaultFormData: BookFormData = {
  // 空文字
  title: "",
  // 空文字
  author: "",
  // 空文字
  publisher: "",
  // 空文字（date 入力欄の空状態）
  published_date: "",
  // 評価なし
  rating: null,
  // デフォルトは「未読」
  status: "unread",
  // 空文字
  memo: "",
};

export default function BookForm({
  // 編集時の初期データ
  initialData,
  // デフォルト値を指定（渡されなければ false）
  isEdit = false,
  // 編集対象の ID
  bookId,
}: Props) {
  // 遷移用のルーターを取得
  const router = useRouter();

  // -------------------------------------------------
  // フォームの状態管理
  // -------------------------------------------------
  // initialData が渡された場合（編集モード）は
  // 既存データを初期値として使用します。
  // 渡されなかった場合（新規登録モード）は
  // defaultFormData を初期値として使用します。
  // ?? は「左が null/undefined なら右を使う」演算子（nullish coalescing）です。
  // -------------------------------------------------
  const [formData, setFormData] = useState<BookFormData>(
    // initialData が undefined なら defaultFormData
    initialData ?? defaultFormData
  );
  // 送信中フラグ（二重送信防止）
  const [isSubmitting, setIsSubmitting] = useState(false);
  // エラーメッセージ（無ければ null）
  const [error, setError] = useState<string | null>(null);

  // -------------------------------------------------
  // フォームフィールドの値変更ハンドラ
  // -------------------------------------------------
  // 各入力フィールドの onChange イベントで呼ばれ、
  // フォームの状態を更新します。
  // name 属性をキーとして、対応するフィールドの
  // 値だけを更新します。
  // -------------------------------------------------
  const handleChange = (
    // ChangeEvent は React の入力変化イベントの型
    e: React.ChangeEvent<
      // input / select / textarea のいずれかに対応
      HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement
    >
  ) => {
    // イベントの発生元から name と value を取り出す
    const { name, value } = e.target;
    // 関数形式の setState（前の状態を引数に取れる）
    setFormData((prev) => ({
      // スプレッド構文で既存の値を全部展開（他のフィールドはそのまま）
      ...prev,
      // 計算プロパティ名で name に対応するフィールドだけ上書き
      [name]: value,
    }));
  };

  // -------------------------------------------------
  // 評価の変更ハンドラ
  // -------------------------------------------------
  // 評価は数値として扱うため、専用のハンドラを
  // 用意します。空文字の場合は null を設定し、
  // それ以外は数値に変換します。
  // select の value は常に文字列で来るので、明示的に
  // parseInt で数値化する必要があります。
  // -------------------------------------------------
  const handleRatingChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    // 選択された値（文字列）
    const value = e.target.value;
    setFormData((prev) => ({
      ...prev,
      // 空なら null、そうでなければ 10 進数で整数化
      rating: value === "" ? null : parseInt(value, 10),
    }));
  };

  // -------------------------------------------------
  // フォーム送信ハンドラ
  // -------------------------------------------------
  // isEdit フラグに応じて INSERT または UPDATE を
  // 実行します。
  // async 関数として定義し、await で非同期処理の
  // 完了を待ちます。
  // -------------------------------------------------
  const handleSubmit = async (e: React.FormEvent) => {
    // フォームのデフォルト送信（ページリロード）をキャンセル
    e.preventDefault();
    // 送信中状態に
    setIsSubmitting(true);
    // 過去のエラーをクリア
    setError(null);

    // -------------------------------------------------
    // バリデーション
    // -------------------------------------------------
    // タイトルは必須項目です。空の場合はエラーを
    // 表示して処理を中断します。
    // trim() で前後の空白を除去してからチェックする
    // ことで「スペースだけ」の入力もエラー扱いに。
    // -------------------------------------------------
    // タイトルが空白のみ または 空文字の場合
    if (!formData.title.trim()) {
      setError("タイトルは必須です。");
      // 送信中フラグを戻す
      setIsSubmitting(false);
      // 処理を中断
      return;
    }

    try {
      // クライアント用 Supabase インスタンスを作成
      const supabase = createClient();

      // 編集モード かつ ID がある場合
      if (isEdit && bookId) {
        // -------------------------------------------------
        // 編集モード: UPDATE
        // -------------------------------------------------
        // 既存の書籍データを更新します。
        // .eq("id", bookId) で対象のレコードを指定し、
        // フォームの内容で上書きします。
        // .eq() を省略すると全件更新されてしまうので
        // 絶対に忘れないこと。
        // -------------------------------------------------
        const { error: updateError } = await supabase
          // books テーブルに対して
          .from("books")
          // UPDATE を発行
          .update({
            // 前後の空白を除去
            title: formData.title.trim(),
            // 空文字なら null（DB をクリーンに保つ）
            author: formData.author.trim() || null,
            publisher: formData.publisher.trim() || null,
            // 空文字なら null
            published_date: formData.published_date || null,
            // null または数値
            rating: formData.rating,
            status: formData.status,
            memo: formData.memo.trim() || null,
          })
          // WHERE id = bookId
          .eq("id", bookId);

        // UPDATE 中にエラーが起きたら
        if (updateError) {
          // catch ブロックに飛ばす
          throw updateError;
        }

        // -------------------------------------------------
        // 更新成功: 詳細ページにリダイレクト
        // -------------------------------------------------
        // 更新が完了したら、その書籍の詳細ページに
        // 遷移します。ユーザーは更新後の情報を
        // すぐに確認できます。
        // refresh() でサーバーコンポーネントのキャッシュ
        // も更新し、最新のデータが表示されるようにします。
        // -------------------------------------------------
        // 詳細ページに遷移
        router.push(`/books/${bookId}`);
        // サーバー側データを再取得
        router.refresh();
      } else {
        // -------------------------------------------------
        // 新規登録モード: INSERT
        // -------------------------------------------------
        // 新しい書籍データを挿入します。
        // .select() を付けることで、挿入されたデータ
        // （自動生成されたIDを含む）を取得できます。
        // -------------------------------------------------
        const { data, error: insertError } = await supabase
          .from("books")
          // INSERT 文に相当
          .insert({
            title: formData.title.trim(),
            author: formData.author.trim() || null,
            publisher: formData.publisher.trim() || null,
            published_date: formData.published_date || null,
            rating: formData.rating,
            status: formData.status,
            memo: formData.memo.trim() || null,
          })
          // 挿入したレコードを返してもらう
          .select()
          // 1 件だけ取得
          .single();

        if (insertError) {
          throw insertError;
        }

        // -------------------------------------------------
        // 登録成功: 一覧ページにリダイレクト
        // -------------------------------------------------
        router.push("/books");
        router.refresh();
      }
    } catch (err) {
      // 開発者向けにコンソールへエラー出力
      console.error("保存エラー:", err);
      setError(
        isEdit
          // 編集時のメッセージ
          ? "書籍の更新に失敗しました。もう一度お試しください。"
          // 新規登録時のメッセージ
          : "書籍の登録に失敗しました。もう一度お試しください。"
      );
    } finally {
      // 成功・失敗どちらでも、送信中フラグを戻す（finally で必ず実行）
      setIsSubmitting(false);
    }
  };

  // -------------------------------------------------
  // フォームのレンダリング
  // -------------------------------------------------
  return (
    // form の onSubmit で送信時に handleSubmit が呼ばれる
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* エラーメッセージ */}
      {/* error が null でないときだけ描画 */}
      {error && (
        // 赤系の警告ボックス
        <div className="bg-red-50 border-l-4 border-red-400 p-4 rounded">
          <div className="flex">
            <div className="flex-shrink-0">
              <svg
                className="h-5 w-5 text-red-400"
                viewBox="0 0 20 20"
                fill="currentColor"
              >
                <path
                  fillRule="evenodd"
                  // ×印アイコンのパス
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                  clipRule="evenodd"
                />
              </svg>
            </div>
            <div className="ml-3">
              {/* エラー文言を表示 */}
              <p className="text-sm text-red-700">{error}</p>
            </div>
          </div>
        </div>
      )}

      {/* タイトル（必須） */}
      <div>
        <label
          // この label がどの input と紐づくか（id と一致させる）
          htmlFor="title"
          className="block text-sm font-medium text-gray-700"
        >
          {/* 赤いアスタリスクで必須を示す */}
          タイトル <span className="text-red-500">*</span>
        </label>
        <input
          // テキスト入力欄
          type="text"
          // label の htmlFor と一致
          id="title"
          // handleChange で参照される名前
          name="title"
          // HTML 側の必須チェック（ブラウザがエラー表示）
          required
          // 制御コンポーネント方式：state の値を反映
          value={formData.title}
          // 入力のたびに state を更新
          onChange={handleChange}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
          // 未入力時のヒント
          placeholder="書籍のタイトルを入力"
        />
      </div>

      {/* 著者 */}
      <div>
        <label
          htmlFor="author"
          className="block text-sm font-medium text-gray-700"
        >
          著者
        </label>
        <input
          type="text"
          id="author"
          // name="author" → handleChange で formData.author が更新される
          name="author"
          value={formData.author}
          onChange={handleChange}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
          placeholder="著者名を入力"
        />
      </div>

      {/* 出版社 */}
      <div>
        <label
          htmlFor="publisher"
          className="block text-sm font-medium text-gray-700"
        >
          出版社
        </label>
        <input
          type="text"
          id="publisher"
          name="publisher"
          value={formData.publisher}
          onChange={handleChange}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
          placeholder="出版社名を入力"
        />
      </div>

      {/* 出版日 */}
      <div>
        <label
          htmlFor="published_date"
          className="block text-sm font-medium text-gray-700"
        >
          出版日
        </label>
        <input
          // 日付ピッカー（YYYY-MM-DD 形式の文字列を扱う）
          type="date"
          id="published_date"
          name="published_date"
          value={formData.published_date}
          onChange={handleChange}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
        />
      </div>

      {/* 評価 */}
      <div>
        <label
          htmlFor="rating"
          className="block text-sm font-medium text-gray-700"
        >
          評価
        </label>
        <select
          id="rating"
          name="rating"
          // null のとき "" を渡す（select は string しか扱えない）
          value={formData.rating ?? ""}
          // 専用ハンドラで数値化
          onChange={handleRatingChange}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
        >
          {/* 空文字を選択肢として用意 */}
          <option value="">未評価</option>
          <option value="1">★☆☆☆☆（1）</option>
          <option value="2">★★☆☆☆（2）</option>
          <option value="3">★★★☆☆（3）</option>
          <option value="4">★★★★☆（4）</option>
          <option value="5">★★★★★（5）</option>
        </select>
      </div>

      {/* ステータス */}
      <div>
        <label
          htmlFor="status"
          className="block text-sm font-medium text-gray-700"
        >
          ステータス
        </label>
        <select
          id="status"
          name="status"
          value={formData.status}
          // ステータスは文字列のままなので共通ハンドラで OK
          onChange={handleChange}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
        >
          <option value="unread">未読</option>
          <option value="reading">読書中</option>
          <option value="finished">読了</option>
        </select>
      </div>

      {/* メモ */}
      <div>
        <label
          htmlFor="memo"
          className="block text-sm font-medium text-gray-700"
        >
          メモ
        </label>
        <textarea
          id="memo"
          name="memo"
          // 表示行数の目安
          rows={4}
          value={formData.memo}
          onChange={handleChange}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
          placeholder="読書メモや感想を入力"
        />
      </div>

      {/* 送信ボタン */}
      {/* ボタンを右寄せで横並び */}
      <div className="flex items-center justify-end space-x-3">
        <button
          // type="button" は「フォーム送信をしないボタン」
          type="button"
          // クリック時に 1 つ前のページへ戻る
          onClick={() => router.back()}
          className="inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
        >
          キャンセル
        </button>
        <button
          // type="submit" でフォームの onSubmit を発火させる
          type="submit"
          // 送信中はクリック不可（二重送信防止）
          disabled={isSubmitting}
          className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {/* 送信中なら... */}
          {isSubmitting
            /* かつ編集モードなら */
            ? isEdit
              /* 「更新中...」 */
              ? "更新中..."
              /* それ以外は「登録中...」 */
              : "登録中..."
            /* 送信中でなくて編集モードなら */
            : isEdit
              /* 「更新する」 */
              ? "更新する"
              /* 新規登録なら「登録する」 */
              : "登録する"}
        </button>
      </div>
    </form>
  );
}

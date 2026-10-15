// セレクト変更時に router.push を呼ぶのでクライアントコンポーネント
"use client";

// ルーティング関連フック
import { useRouter, useSearchParams, usePathname } from "next/navigation";

// -------------------------------------------------
// ソート選択コンポーネント
// -------------------------------------------------
// 書籍一覧の並び順を変更するためのセレクトボックスです。
// 以下のソート基準に対応しています:
// - 登録日（新しい順/古い順）
// - タイトル（昇順/降順）
// - 評価（高い順/低い順）
// -------------------------------------------------
export default function SortSelect() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const pathname = usePathname();

  // -------------------------------------------------
  // 現在のソート状態をURLから取得
  // -------------------------------------------------
  // URL に sort/order が無ければデフォルト値を使う。
  // -------------------------------------------------
  // デフォルトは登録日
  const currentSort = searchParams.get("sort") || "created_at";
  // デフォルトは降順
  const currentOrder = searchParams.get("order") || "desc";
  // select の value 形式に変換
  const currentValue = `${currentSort}-${currentOrder}`;

  // -------------------------------------------------
  // ソート変更ハンドラ
  // -------------------------------------------------
  // セレクトボックスの値が変更されたときに呼ばれます。
  // 値は "カラム名-昇順/降順" の形式（例: "title-asc"）
  // になっており、これを分解して URL パラメータに
  // 設定します。
  // -------------------------------------------------
  const handleSortChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    // 例: "rating-desc"
    const value = e.target.value;
    // ["rating", "desc"] に分解
    const [sort, order] = value.split("-");

    // 現在のクエリをコピー
    const params = new URLSearchParams(searchParams.toString());
    // sort を上書き
    params.set("sort", sort);
    // order を上書き
    params.set("order", order);

    // 新しい URL に遷移
    router.push(`${pathname}?${params.toString()}`);
  };

  // -------------------------------------------------
  // レンダリング
  // -------------------------------------------------
  return (
    <div className="flex items-center justify-end mb-4">
      <label
        htmlFor="sort"
        className="text-sm text-gray-500 mr-2"
      >
        並び順:
      </label>
      <select
        id="sort"
        // URL の値を反映
        value={currentValue}
        // 変更時に URL を更新
        onChange={handleSortChange}
        className="block w-48 py-1.5 px-3 border border-gray-300 bg-white rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 text-sm"
      >
        <option value="created_at-desc">登録日（新しい順）</option>
        <option value="created_at-asc">登録日（古い順）</option>
        <option value="title-asc">タイトル（A→Z）</option>
        <option value="title-desc">タイトル（Z→A）</option>
        <option value="rating-desc">評価（高い順）</option>
        <option value="rating-asc">評価（低い順）</option>
      </select>
    </div>
  );
}

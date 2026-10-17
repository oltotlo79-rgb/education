// 入力イベントや setState を扱うのでクライアントコンポーネント
"use client";

// ルーター、URL クエリ、パス取得用フック
import { useRouter, useSearchParams, usePathname } from "next/navigation";
// 状態管理・副作用・関数メモ化
import { useState, useEffect, useCallback } from "react";

// -------------------------------------------------
// 検索バーコンポーネント
// -------------------------------------------------
// このコンポーネントは以下の機能を提供します:
// 1. テキスト入力による検索
// 2. ステータスによるフィルタリング
// 3. URLの検索パラメータとの同期
// 4. デバウンス処理（入力のたびにクエリを
//    発行しないように、一定時間待ってから検索）
// -------------------------------------------------
export default function SearchBar() {
  // URL を変更するため
  const router = useRouter();
  // 現在の ?q=... を読むため
  const searchParams = useSearchParams();
  // 現在のパス（一覧ページなら /）を取得
  const pathname = usePathname();

  // -------------------------------------------------
  // URL の検索パラメータから初期値を取得
  // -------------------------------------------------
  // ページをリロードしたり、検索結果のURLに直接
  // アクセスしたりした場合でも、検索状態が保持
  // されるようにします。
  // -------------------------------------------------
  const [searchQuery, setSearchQuery] = useState(
    // ?q= の値、無ければ空文字
    searchParams.get("q") ?? ""
  );
  const [statusFilter, setStatusFilter] = useState(
    // ?status= の値、無ければ空文字
    searchParams.get("status") ?? ""
  );

  // -------------------------------------------------
  // URL の検索パラメータを更新する関数
  // -------------------------------------------------
  // 検索クエリやフィルタが変更されたときに、
  // URL の検索パラメータを更新してページを
  // 再レンダリングします。
  // useCallback で関数をメモ化することで、
  // 依存配列が変わらない限り同じ関数インスタンスを
  // 使い回します。
  // -------------------------------------------------
  const updateSearchParams = useCallback(
    (query: string, status: string) => {
      // 現在のクエリを元にコピーを作成
      const params = new URLSearchParams(searchParams.toString());

      // -------------------------------------------------
      // 検索クエリの設定
      // -------------------------------------------------
      // 値が空の場合はパラメータを削除し、
      // URLをクリーンに保ちます。
      // 例: /?q=React&status=reading
      //     検索クエリが空なら /?status=reading
      // -------------------------------------------------
      // 空白以外の文字が含まれていれば
      if (query.trim()) {
        // ?q= を設定
        params.set("q", query.trim());
      } else {
        // 空なら ?q= を削除
        params.delete("q");
      }

      // ステータスフィルタの設定
      if (status) {
        params.set("status", status);
      } else {
        params.delete("status");
      }

      // -------------------------------------------------
      // URL を更新
      // -------------------------------------------------
      // router.push() を使って URL を更新すると、
      // Next.js が自動的にサーバーコンポーネントを
      // 再レンダリングし、新しい searchParams で
      // データを再取得します。
      // -------------------------------------------------
      // "q=React&status=reading" など
      const queryString = params.toString();
      // クエリがあれば付加、なければパスだけ
      const newUrl = queryString ? `${pathname}?${queryString}` : pathname;
      // URL を更新
      router.push(newUrl);
    },
    // これらが変わったら関数を作り直す
    [router, pathname, searchParams]
  );

  // -------------------------------------------------
  // デバウンス処理
  // -------------------------------------------------
  // ユーザーがキーボードを打つたびに検索クエリを
  // 発行すると、不必要なリクエストが大量に発生
  // します。デバウンスを使って、ユーザーが入力を
  // 止めてから 300ms 後に検索を実行します。
  // useEffect は「描画後に副作用を実行する」フック。
  // -------------------------------------------------
  useEffect(() => {
    // 300ms 後に実行する予約
    const timer = setTimeout(() => {
      updateSearchParams(searchQuery, statusFilter);
    }, 300);

    // -------------------------------------------------
    // クリーンアップ関数
    // -------------------------------------------------
    // 次の入力が行われた場合、前のタイマーを
    // キャンセルします。これにより、最後の入力
    // から 300ms 後にのみ検索が実行されます。
    // -------------------------------------------------
    // 副作用の片付け
    return () => clearTimeout(timer);
  // これらが変わるたびに発火
  }, [searchQuery, statusFilter, updateSearchParams]);

  // -------------------------------------------------
  // 検索条件クリア
  // -------------------------------------------------
  // 入力欄を空にして、URL も検索条件無しに戻します。
  // -------------------------------------------------
  const handleClear = () => {
    // 検索クエリをクリア
    setSearchQuery("");
    // フィルタをクリア
    setStatusFilter("");
    // ?xxx を全部外す
    router.push(pathname);
  };

  // -------------------------------------------------
  // レンダリング
  // -------------------------------------------------
  return (
    <div className="bg-white shadow rounded-lg p-4 mb-6">
      {/* モバイルは縦、PC は横並び */}
      <div className="flex flex-col sm:flex-row gap-4">
        {/* 検索入力フィールド */}
        <div className="flex-1">
          {/* sr-only は視覚的には非表示、スクリーンリーダー用 */}
          <label htmlFor="search" className="sr-only">
            検索
          </label>
          <div className="relative">
            {/* 虫眼鏡アイコン */}
            {/* クリックを通過させる */}
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              <svg
                className="h-5 w-5 text-gray-400"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  // 虫眼鏡のパス
                  d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                />
              </svg>
            </div>
            <input
              type="text"
              id="search"
              // state を反映（制御コンポーネント）
              value={searchQuery}
              // 入力のたびに state を更新
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="タイトルまたは著者名で検索..."
              className="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
            />
          </div>
        </div>

        {/* ステータスフィルタ */}
        <div className="sm:w-48">
          <label htmlFor="status-filter" className="sr-only">
            ステータスフィルタ
          </label>
          <select
            id="status-filter"
            value={statusFilter}
            onChange={(e) => setStatusFilter(e.target.value)}
            className="block w-full py-2 px-3 border border-gray-300 bg-white rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
          >
            <option value="">すべてのステータス</option>
            <option value="want_to_read">読みたい</option>
            <option value="reading">読書中</option>
            <option value="completed">読了</option>
          </select>
        </div>

        {/* クリアボタン */}
        {/* どちらか入っているときだけ表示 */}
        {(searchQuery || statusFilter) && (
          <button
            onClick={handleClear}
            className="inline-flex items-center px-3 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
          >
            <svg
              className="w-4 h-4 mr-1"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                // × アイコン
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
            クリア
          </button>
        )}
      </div>
    </div>
  );
}

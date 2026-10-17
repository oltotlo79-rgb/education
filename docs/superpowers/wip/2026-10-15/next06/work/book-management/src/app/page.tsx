// src/app/page.tsx
// URL "/" にアクセスしたときに表示されるトップページ

// export default function: ファイルのメインコンポーネントとしてエクスポート
// HomePage: コンポーネント名（任意の名前でOK。Next.jsはdefault exportを自動的にページとして扱う）
export default function HomePage() {
  // JSX を return（このページの中身）
  return (
    // 全体を div で囲む。複数の要素を return するには親要素が必要
    <div>
      {/* h1: 大見出し / text-3xl: 30px / font-bold: 太字 / text-gray-900: 濃いグレー / mb-4: 下マージン16px */}
      <h1 className="text-3xl font-bold text-gray-900 mb-4">
        書籍一覧
      </h1>
      {/* p: 段落 / text-gray-600: 中間グレー / mb-8: 下マージン32px */}
      <p className="text-gray-600 mb-8">
        登録された書籍がここに表示されます。
      </p>

      {/* 動作確認用: Tailwind CSS のスタイルが正しく適用されるかチェック */}
      {/* bg-white: 白背景 / rounded-lg: 角丸大 / shadow-sm: 控えめな影 */}
      {/* border border-gray-200: 1pxの薄いグレーボーダー / p-6: 内側パディング24px */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        {/* h2: 中見出し / text-lg: 18px / font-semibold: やや太字（600） */}
        <h2 className="text-lg font-semibold text-gray-800 mb-2">
          セットアップ完了
        </h2>
        {/* text-sm: 14px の小さめテキスト */}
        <p className="text-gray-600 text-sm">
          Next.js + Tailwind CSS + Supabase の開発環境が正しくセットアップされました。
          次の章で、このページに実際の書籍一覧を表示します。
        </p>
        {/* バッジを横並びにする領域 */}
        {/* mt-4: 上マージン16px / flex: 横並び / space-x-2: 子要素間に8px間隔 */}
        <div className="mt-4 flex space-x-2">
          {/* 読みたいバッジ：黄系 */}
          {/* inline-block: インラインブロック表示 / bg-yellow-100: 薄い黄の背景 / text-yellow-800: 濃い黄の文字 */}
          {/* text-xs: 12px / px-2.5 py-1: パディング / rounded-full: 完全な丸（カプセル形） / font-medium: 中太字 */}
          <span className="inline-block bg-yellow-100 text-yellow-800 text-xs px-2.5 py-1 rounded-full font-medium">
            読みたい
          </span>
          {/* 読書中バッジ：青系 */}
          <span className="inline-block bg-blue-100 text-blue-800 text-xs px-2.5 py-1 rounded-full font-medium">
            読書中
          </span>
          {/* 読了バッジ：緑系 */}
          <span className="inline-block bg-green-100 text-green-800 text-xs px-2.5 py-1 rounded-full font-medium">
            読了
          </span>
        </div>
      </div>
    </div>
  );
}

{/* 基本パターン: 通常のクラスの後に dark: プレフィックスを付ける */}
{/* bg-white: 通常は白背景 / dark:bg-gray-900: ダーク時は濃灰背景 */}
<div className="bg-white dark:bg-gray-900">
  {/* text-gray-900: 通常は濃灰文字 / dark:text-gray-100: ダーク時は薄灰文字 */}
  <h1 className="text-gray-900 dark:text-gray-100">タイトル</h1>
  {/* 補足テキスト: 通常は中間グレー / ダーク時は薄め */}
  <p className="text-gray-600 dark:text-gray-400">本文テキスト</p>
</div>

{/* ボーダーの例: 通常は薄い枠 / ダーク時は濃い枠 */}
<div className="border border-gray-200 dark:border-gray-700">
  ...
</div>

{/* ホバーとの組み合わせ（dark: と hover: は並べて書ける） */}
<button className="
  bg-blue-500 hover:bg-blue-600
  dark:bg-blue-600 dark:hover:bg-blue-700
">
  {/* bg-blue-500           : 通常背景 */}
  {/* hover:bg-blue-600     : ホバー時に濃く */}
  {/* dark:bg-blue-600      : ダーク時の背景 */}
  {/* dark:hover:bg-blue-700: ダーク × ホバー時 */}
  ボタン
</button>

{/* リングとフォーカスとの組み合わせ */}
<input className="
  focus:ring-blue-500
  dark:focus:ring-blue-400
" />
{/* focus:ring-blue-500       : フォーカス時のリング色 */}
{/* dark:focus:ring-blue-400  : ダーク × フォーカス時のリング色 */}

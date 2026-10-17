{/*
  ▼ Flexbox（フレックスボックス）の基本セット
    ・flex          : 中の子要素を「横並び」にする
    ・items-center  : 縦方向（高さ方向）の中央に揃える
    ・justify-between : 左右の端に寄せて、間を空ける
    ・flex-wrap     : 横幅が足りなくなったら自動で折り返す
    ・flex-1        : 余ったすき間を吸い込んで広がる
*/}
<div className="flex flex-wrap items-center justify-between gap-3">
  {/* 左側：タイトル。flex-1 で余白を吸って広がる */}
  <h2 className="flex-1 text-lg font-bold">本棚</h2>

  {/* 右側：ボタンを2つ横並びに。間に gap-2 のすき間 */}
  <div className="flex items-center gap-2">
    <button className="rounded-md bg-gray-200 px-3 py-1">並び替え</button>
    <button className="rounded-md bg-blue-500 px-3 py-1 text-white">追加</button>
  </div>
</div>

{/* NG: 画像を包む div に relative が無い */}
{/* → バッジの基準がページ全体になり、画面の右上に飛んでいく */}
<div className="inline-block">
  <img src="/book.jpg" alt="表紙" className="rounded" />
  <span className="absolute -top-2 -right-2 rounded-full bg-red-500 px-2 text-xs text-white">
    NEW
  </span>
</div>

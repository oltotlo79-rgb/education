{/* OK: 画像を包む div に relative を付けて「ここが基準」と宣言する */}
{/* → バッジが画像の右上の角にぴったり乗る */}
<div className="relative inline-block">
  <img src="/book.jpg" alt="表紙" className="rounded" />
  <span className="absolute -top-2 -right-2 rounded-full bg-red-500 px-2 text-xs text-white">
    NEW
  </span>
</div>

{/*
  ▼ ポジショニングの基本セット
    ・relative : 子要素の「基準」になる箱（自分は動かない）
    ・absolute : 基準の箱を起点に、好きな位置へ浮かせて置く
    ・inset-*  : 上下左右からの距離をまとめて指定
    ・z-*      : どれを手前に表示するか（重なり順）
*/}
<div className="relative inline-block">
  {/* 商品画像（基準になる箱） */}
  <img src="/book.jpg" alt="表紙" className="rounded" />

  {/* 右上に重ねる「NEW」バッジ */}
  <span className="absolute -top-2 -right-2 z-10 rounded-full bg-red-500 px-2 py-0.5 text-xs text-white">
    NEW
  </span>
</div>

{/*
  ▼ sticky : スクロールしても画面上部に貼り付く見出し
    ・top-0 : 上端 0px の位置で固定される
*/}
<h3 className="sticky top-0 bg-white py-2 font-bold">あ行</h3>

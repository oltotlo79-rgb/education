// components/BookCard.tsx 内のホバーエフェクト部分（上のシンプル版に加えて、より凝った演出を追加した例）

// カード全体のホバー（上に浮かぶ + 影が深くなる）
<article
  className="
    card
    overflow-hidden
    transition-all duration-300 ease-out
    hover:-translate-y-1.5
    hover:shadow-card-hover
  "
  /* ▼ クラスの読み解き ▼
     card                       : globals.css のカード基本スタイル
     overflow-hidden            : 中身がはみ出さない（画像ズーム時のため）
     transition-all duration-300: 全プロパティを 300ms かけて変化
     ease-out                   : 最初速く・最後ゆっくり（自然な減速カーブ）
     hover:-translate-y-1.5     : ホバーで上に 6px 移動（浮き上がる）
     hover:shadow-card-hover    : ホバーで大きな影に
  */
>
  {/* サムネイル画像のホバー（ズームイン） */}
  <div className="relative aspect-[3/4] overflow-hidden">
    <img
      src={book.thumbnailUrl}
      alt={book.title}
      className="
        h-full w-full object-cover
        transition-transform duration-500 ease-out
        group-hover:scale-110
      "
      /* group-hover:scale-110 : 親（.group）のホバー時に画像を 110% にズーム */
    />

    {/* オーバーレイ（ホバー時にフェードイン） */}
    <div
      className="
        absolute inset-0
        bg-gradient-to-t from-black/60 via-transparent to-transparent
        opacity-0
        transition-opacity duration-300
        group-hover:opacity-100
      "
      /* ▼ クラスの読み解き ▼
         absolute inset-0          : 親いっぱいに広げる（top:0 right:0 bottom:0 left:0）
         bg-gradient-to-t          : 下→上方向のグラデーション
         from-black/60 ... transparent: 下側は黒60%、上側は透明
         opacity-0                 : 通常は透明（非表示）
         group-hover:opacity-100   : 親ホバー時に表示
      */
    />

    {/* 「詳細を見る」テキスト（ホバー時に下からスライドイン） */}
    <div
      className="
        absolute bottom-0 left-0 right-0
        p-4
        text-white
        translate-y-4 opacity-0
        transition-all duration-300
        group-hover:translate-y-0 group-hover:opacity-100
      "
      /* translate-y-4 opacity-0       : 通常は 16px 下にずれて透明（見えない）
         group-hover:translate-y-0 ... : ホバー時に元位置 + 不透明（スライドアップで登場）
      */
    >
      <span className="text-sm font-medium">詳細を見る →</span>
    </div>
  </div>
</article>

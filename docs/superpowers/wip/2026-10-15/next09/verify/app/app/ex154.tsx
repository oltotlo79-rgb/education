{/*
  ▼ CSS Grid（グリッド）の基本セット
    ・grid          : 格子レイアウトにする
    ・grid-cols-3   : 列を3つに分ける
    ・gap-4         : マス同士のすき間を 16px に
*/}
<div className="grid grid-cols-3 gap-4">
  <div className="rounded bg-blue-100 p-4">1</div>
  <div className="rounded bg-blue-100 p-4">2</div>
  <div className="rounded bg-blue-100 p-4">3</div>
  <div className="rounded bg-blue-100 p-4">4</div>
  <div className="rounded bg-blue-100 p-4">5</div>
  <div className="rounded bg-blue-100 p-4">6</div>
</div>

{/*
  ▼ 列の自動調整（画面幅に応じて列数が変わる）
    ・grid-cols-[repeat(auto-fill,minmax(8rem,1fr))]
      → 「最低 8rem ある列を、入るだけ自動で並べる」指定
*/}
<div className="grid grid-cols-[repeat(auto-fill,minmax(8rem,1fr))] gap-4">
  <div className="rounded bg-green-100 p-4">A</div>
  <div className="rounded bg-green-100 p-4">B</div>
  <div className="rounded bg-green-100 p-4">C</div>
</div>

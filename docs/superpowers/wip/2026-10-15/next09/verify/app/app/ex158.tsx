{/*
  ▼ 状態に応じた見た目の変化
    ・hover:bg-blue-600 : マウスを乗せた時だけ濃い青に
    ・focus:ring-2      : クリック/選択した時だけ枠を光らせる
    ・before:/after:    : HTMLに無い「飾り」を前後に足す
*/}
<button className="rounded bg-blue-500 px-4 py-2 text-white hover:bg-blue-600 focus:ring-2 focus:ring-blue-300">
  保存
</button>

<input
  className="rounded border px-3 py-2 focus:border-blue-500 focus:outline-hidden"
  placeholder="検索…"
/>

{/* 必須マーク（赤い * ）を after で自動的に付ける */}
<label className="after:ml-1 after:text-red-500 after:content-['*']">
  タイトル
</label>

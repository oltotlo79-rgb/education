{/* NG: 縦並びのボタンを「横の中央」に寄せたくて justify-center を付けた */}
{/* → flex-col の主軸は縦なので横には効かず、ボタンは横いっぱいに伸びたまま */}
<div className="flex flex-col justify-center gap-2">
  <button>保存</button>
  <button>キャンセル</button>
</div>

{/* OK: flex-col のときの「横」は交差軸 → items-center を使う */}
{/* → ボタンが中身の幅に縮み、横方向の真ん中にそろう */}
<div className="flex flex-col items-center gap-2">
  <button>保存</button>
  <button>キャンセル</button>
</div>

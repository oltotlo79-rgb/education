// React の「state 更新の予約」をまねした小さな模型
// （React 本体のコードではなく、考え方をつかむためのものです）

// 予約できるのは「次の値そのもの」か「前の値から次の値を作る関数」
type Update = number | ((prev: number) => number);

// 予約を前から順番に処理して、次のレンダーで使う値を求める
function applyUpdates(current: number, queue: Update[]): number {
  let value = current;
  for (const update of queue) {
    // 関数なら「ここまでの最新の値」を渡して計算し、値ならそのまま置き換える
    value = typeof update === "function" ? update(value) : update;
  }
  return value;
}

// このレンダーの count は 0 で固定されている（スナップショット）
const count = 0;

// ---- パターンA: setCount(count + 1) を2回 ----
const queueA: Update[] = [];
// 1回目: count は 0 なので「1 にして」と予約
queueA.push(count + 1);
// 2回目: count はまだ 0 なので、また「1 にして」と予約
queueA.push(count + 1);
console.log("A: 予約の中身 =", queueA);
console.log("A: 次のレンダーの count =", applyUpdates(count, queueA));

// ---- パターンB: setCount((prev) => prev + 1) を2回 ----
const queueB: Update[] = [];
queueB.push((prev) => prev + 1);
queueB.push((prev) => prev + 1);
console.log("B: 次のレンダーの count =", applyUpdates(count, queueB));

// ---- パターンC: setCount(5) のあとに setCount((prev) => prev * 2) ----
const queueC: Update[] = [];
queueC.push(5);
queueC.push((prev) => prev * 2);
console.log("C: 次のレンダーの count =", applyUpdates(count, queueC));

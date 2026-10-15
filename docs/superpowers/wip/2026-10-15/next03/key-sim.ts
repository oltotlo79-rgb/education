// 画面に並んでいる「行」1つ分（React が作った DOM の代わり）
type Row = {
  // この行に付いている key
  key: string;
  // 行の中の入力欄にユーザーが打った文字（DOM が覚えている状態）
  memo: string;
};

// 表示したいデータ1件分
type Fruit = {
  id: number;
  name: string;
};

// key の決め方（index を使うか、id を使うか）を関数で受け取る
type GetKey = (fruit: Fruit, index: number) => string;

// 新しいデータを描くとき、同じ key の行があればそれを使い回す（React のまね）
function rerender(prevRows: Row[], fruits: Fruit[], getKey: GetKey): Row[] {
  return fruits.map((fruit, index) => {
    const key = getKey(fruit, index);
    // 同じ key の行を探す。見つかればその行（メモ入り）をそのまま使う
    const reused = prevRows.find((row) => row.key === key);
    const memo = reused ? reused.memo : "";
    console.log(`  ${fruit.name}: key=${key} → メモ欄「${memo || "(空)"}」`);
    return { key, memo };
  });
}

const before: Fruit[] = [
  { id: 1, name: "りんご" },
  { id: 2, name: "バナナ" },
];
// 先頭に「みかん」を追加したあとのデータ
const after: Fruit[] = [{ id: 3, name: "みかん" }, ...before];

// ---- index を key にした場合 ----
const byIndex: GetKey = (_fruit, index) => String(index);
// 追加前の画面: りんごの行に「赤い」、バナナの行に「黄色い」とメモしてある
const indexRows: Row[] = [
  { key: "0", memo: "赤い" },
  { key: "1", memo: "黄色い" },
];
console.log("【index を key にした場合】");
rerender(indexRows, after, byIndex);

// ---- id を key にした場合 ----
const byId: GetKey = (fruit) => String(fruit.id);
const idRows: Row[] = [
  { key: "1", memo: "赤い" },
  { key: "2", memo: "黄色い" },
];
console.log("【id を key にした場合】");
rerender(idRows, after, byId);

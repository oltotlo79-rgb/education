const fruits: string[] = ["りんご", "みかん", "バナナ"];

// 方法1: 取り出したあと undefined でないか確かめてから使う
const fourth = fruits[3];
if (fourth !== undefined) {
  console.log(fourth.toUpperCase());
} else {
  console.log("4番目の果物はありません");
}

// 方法2: at() は型が string | undefined になるので、?? で代わりの値を用意する
const last = fruits.at(-1) ?? "なし";
console.log(last);

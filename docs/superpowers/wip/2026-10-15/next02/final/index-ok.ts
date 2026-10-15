const fruits: string[] = ["りんご", "みかん", "バナナ"];

// 取り出したあと、undefined でないか確かめてから使う
const fourth = fruits[3];
if (fourth !== undefined) {
  console.log(fourth.toUpperCase());
} else {
  console.log("4番目の果物はありません");
}

// ?? で「無かったときの代わりの値」を用意しておく
const fifth = fruits[4] ?? "（なし）";
console.log(fifth);

// 在庫数。0 は「在庫が 0 冊」という正しい値
const stock: number | null = 0;

// || は 0 や "" も「値がない」扱いにしてしまう
console.log(stock || "在庫情報なし");

// ?? は null と undefined のときだけ右側を使う
console.log(stock ?? "在庫情報なし");

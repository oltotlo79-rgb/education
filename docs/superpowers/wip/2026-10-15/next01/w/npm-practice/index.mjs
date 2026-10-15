// node_modules にインストールした ms パッケージを読み込む
// （"ms" とだけ書けば、Node.js が node_modules の中から探してくれる）
import ms from "ms";

// 返却期限「2週間」をミリ秒（1000分の1秒）に変換する
console.log("2週間 =", ms("14 days"), "ミリ秒");

// 逆に、ミリ秒の数字を人間が読みやすい長さに変換する
console.log("600000ミリ秒 =", ms(600000));
console.log("3600000ミリ秒 =", ms(3600000, { long: true }));

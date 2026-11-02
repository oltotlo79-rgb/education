// 付録 B-07: クラス・Map・Set・日付でよく使う書き方

// クラス: データと、それを扱う関数（メソッド）をまとめる。# の付いた項目は外から見えない
class RequestCounter {
  #counts = new Map<string, number>();

  add(status: string): void {
    this.#counts.set(status, (this.#counts.get(status) ?? 0) + 1);
  }

  // get を付けると、() 無しで値のように読める
  get total(): number {
    let sum = 0;
    for (const n of this.#counts.values()) sum += n;
    return sum;
  }

  summary(): string {
    return [...this.#counts].map(([s, n]) => `${s}:${n}`).join(" ");
  }
}

const counter = new RequestCounter();
for (const s of ["回覧中", "承認済み", "回覧中"]) counter.add(s);
console.log(counter.summary(), "合計", counter.total);

// Map: キー → 値の表。キーに何でも使え、入れた順に回せる
const approvers = new Map<number, string>([
  [1, "佐藤"],
  [2, "鈴木"],
]);
approvers.set(3, "高橋");
console.log(approvers.get(2), approvers.has(9), approvers.size);
for (const [order, name] of approvers) console.log(`${order}番目: ${name}`);

// Set: 同じ値は 1 つしか入らない（重複を消す）
const depts = new Set(["設計", "製造", "設計", "品質"]);
console.log([...depts], depts.has("製造"));

// 日付: 月は 0 から数える（10 は 11 月）。時刻のずれを避けるため、日本時間を指定して表示する
const due = new Date(Date.UTC(2026, 10, 10, 0, 0, 0)); // 2026-11-10 00:00（世界標準時）
console.log(due.toISOString());
console.log(due.toLocaleDateString("ja-JP", { timeZone: "Asia/Tokyo" }));
console.log(due.toLocaleDateString("ja-JP", { timeZone: "Asia/Tokyo", dateStyle: "full" }));

// 日数の差: ミリ秒の差を、1 日のミリ秒で割る
const today = new Date(Date.UTC(2026, 10, 2));
const days = Math.round((due.getTime() - today.getTime()) / (24 * 60 * 60 * 1000));
console.log(`締め切りまで ${days} 日`);

// 数を見やすく: Intl.NumberFormat（通貨の形）
const yen = new Intl.NumberFormat("ja-JP", { style: "currency", currency: "JPY" });
console.log(yen.format(30000));

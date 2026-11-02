// 付録 B-06: 非同期（async/await）・エラー・JSON でよく使う書き方
import { setTimeout as sleep } from "node:timers/promises";

// 時間のかかる処理のまね（ms ミリ秒待ってから、値を返す）
async function fetchName(id: number, ms: number): Promise<string> {
  await sleep(ms);
  if (id === 0) {
    throw new Error(`id ${id} の人は見つかりません`);
  }
  return `社員${id}`;
}

// 自分のエラーの種類を作る
class ValidationError extends Error {
  field: string;
  constructor(field: string, message: string) {
    super(message);
    this.name = "ValidationError";
    this.field = field;
  }
}

function checkTitle(title: string): string {
  if (title.trim() === "") {
    throw new ValidationError("title", "件名を入れてください");
  }
  return title.trim();
}

async function main() {
  // 1. await: 終わるまで待って、結果を受け取る
  const a = await fetchName(1, 100);
  console.log("1:", a);

  // 2. Promise.all: いくつかを同時に待つ（いちばん遅い物の時間で終わる）
  const start = Date.now();
  const names = await Promise.all([fetchName(2, 200), fetchName(3, 200)]);
  console.log("2:", names, Date.now() - start < 350 ? "同時に待った" : "順番に待った");

  // 3. try / catch / finally: 失敗を受け止める
  try {
    await fetchName(0, 50);
  } catch (e) {
    // catch の e は何が来るか分からないので、Error か確かめてから使う
    console.log("3: 失敗:", e instanceof Error ? e.message : e);
  } finally {
    console.log("3: finally は、成功しても失敗しても最後に動く");
  }

  // 4. 自分のエラーの種類で分ける
  for (const t of ["  治具の製作 ", "   "]) {
    try {
      console.log("4: OK:", checkTitle(t));
    } catch (e) {
      if (e instanceof ValidationError) {
        console.log(`4: 入力の間違い（${e.field}）: ${e.message}`);
      } else {
        throw e; // 思っていないエラーは、上に投げ直す
      }
    }
  }

  // 5. JSON: 文字列 ⇔ オブジェクト。壊れた JSON は JSON.parse でエラーになる
  const text = JSON.stringify({ id: 1, title: "治具", approvers: ["佐藤", "鈴木"] });
  console.log("5:", text);
  const back = JSON.parse(text) as { id: number; title: string; approvers: string[] };
  console.log("5:", back.approvers[1]);
  try {
    JSON.parse("{ id: 1 }");
  } catch (e) {
    console.log("5: 壊れた JSON:", e instanceof SyntaxError);
  }
}

await main();

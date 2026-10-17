// 「サーバーの返事を待ってから画面を変える（通常の更新）」と
// 「先に画面を変えてしまう（楽観的更新）」の違いを、
// 経過時間つきで確かめる練習用コード

type ScreenState = {
  status: string;
  note: string;
};

// 計測を始めた時刻（ミリ秒）
let startedAt = 0;

// 経過時間（100ms 単位に丸める）と、いまの画面の様子を表示する
function show(screen: ScreenState) {
  const elapsed = Math.round((Date.now() - startedAt) / 100) * 100;
  const time = String(elapsed).padStart(4);
  console.log(`  ${time}ms 画面: ${screen.status} ${screen.note}`);
}

// 返事に 800ms かかるサーバーの代わり。shouldFail が true なら失敗する
function fakeServerUpdate(shouldFail: boolean): Promise<void> {
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      if (shouldFail) {
        reject(new Error("更新に失敗しました"));
      } else {
        resolve();
      }
    }, 800);
  });
}

// 通常の更新（この章の BookForm と同じ考え方）
async function normalUpdate(shouldFail: boolean) {
  startedAt = Date.now();
  // 押した直後は「保存中...」を出すだけで、値はまだ変えない
  show({ status: "読書中", note: "（保存中...）" });
  try {
    await fakeServerUpdate(shouldFail);
    // サーバーが OK と言ってから、新しい値を表示する
    show({ status: "読了", note: "（保存しました）" });
  } catch {
    show({ status: "読書中", note: "（エラーを表示）" });
  }
}

// 楽観的更新（成功するだろうと考えて先に表示する）
async function optimisticUpdate(shouldFail: boolean) {
  startedAt = Date.now();
  // 失敗したときに戻せるよう、変更前の値を覚えておく
  const before: ScreenState = { status: "読書中", note: "" };
  // 押した直後に、新しい値をもう表示してしまう
  show({ status: "読了", note: "（先に表示）" });
  try {
    await fakeServerUpdate(shouldFail);
    show({ status: "読了", note: "（サーバーも成功）" });
  } catch {
    // 失敗したら、覚えておいた変更前の値に巻き戻す
    show({ ...before, note: "（巻き戻してエラーを表示）" });
  }
}

async function main() {
  console.log("■ 通常の更新（成功）");
  await normalUpdate(false);
  console.log("■ 楽観的更新（成功）");
  await optimisticUpdate(false);
  console.log("■ 楽観的更新（失敗）");
  await optimisticUpdate(true);
}

main();

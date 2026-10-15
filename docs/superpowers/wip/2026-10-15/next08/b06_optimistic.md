「待ってから反映する（通常の更新）」と「先に反映してしまう（楽観的更新）」の違いは、時間の流れで見ると一目で分かります。ここでは、ステータスを「読書中」から「読了」に変えて保存し、サーバーの返事が 800 ミリ秒後に返ってくる場面を考えます。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="150" y="40" text-anchor="middle" font-size="12" fill="#475569">0ms クリック</text>
<text x="600" y="40" text-anchor="middle" font-size="12" fill="#475569">800ms サーバーの返事</text>
<line x1="150" y1="48" x2="150" y2="262" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 4"/>
<line x1="600" y1="48" x2="600" y2="262" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 4"/>
<text x="700" y="22" text-anchor="end" font-size="12" fill="#64748b">時間の流れ →</text>
<text x="16" y="88" font-size="13" font-weight="700" fill="#475569">通常の更新</text>
<text x="16" y="104" font-size="12" fill="#64748b">（この章の方式）</text>
<rect x="150" y="66" width="450" height="44" fill="#f1f5f9" stroke="#94a3b8"/>
<text x="375" y="93" text-anchor="middle" font-size="13" fill="#475569">読書中（保存中...）</text>
<rect x="600" y="66" width="100" height="44" fill="#ecfdf5" stroke="#10b981"/>
<text x="650" y="93" text-anchor="middle" font-size="13" font-weight="700" fill="#047857">読了</text>
<text x="16" y="158" font-size="13" font-weight="700" fill="#475569">楽観的更新</text>
<text x="16" y="174" font-size="12" fill="#64748b">（成功した場合）</text>
<rect x="150" y="136" width="450" height="44" fill="#ecfdf5" stroke="#10b981"/>
<text x="375" y="163" text-anchor="middle" font-size="13" fill="#047857">読了（先に表示）</text>
<rect x="600" y="136" width="100" height="44" fill="#ecfdf5" stroke="#10b981"/>
<text x="650" y="163" text-anchor="middle" font-size="13" font-weight="700" fill="#047857">読了</text>
<text x="16" y="228" font-size="13" font-weight="700" fill="#475569">楽観的更新</text>
<text x="16" y="244" font-size="12" fill="#64748b">（失敗した場合）</text>
<rect x="150" y="206" width="450" height="44" fill="#ecfdf5" stroke="#10b981"/>
<text x="375" y="233" text-anchor="middle" font-size="13" fill="#047857">読了（先に表示）</text>
<rect x="600" y="206" width="100" height="44" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="650" y="233" text-anchor="middle" font-size="13" font-weight="700" fill="#c2410c">読書中</text>
<text x="650" y="268" text-anchor="middle" font-size="12" fill="#c2410c">巻き戻し</text>
<text x="360" y="292" text-anchor="middle" font-size="12" fill="#475569">楽観的更新は待ち時間が無いように見えるが、失敗したら元に戻す処理が必ず必要になる</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-6: 通常の更新と楽観的更新の時系列（帯は、その時点で画面に出ている表示）</div>
</div>

- **通常の更新**: クリックしてから返事が来るまでの 800ms は「保存中...」のまま待ち、成功が分かってから「読了」に変えます。画面に出る内容は常に「データベースと同じ」なので、うそを表示することがありません。
- **楽観的更新**: クリックした瞬間に「読了」を出してしまうので、待ち時間がゼロに見えます。その代わり、失敗したときは **「読了」と表示していたのを「読書中」に巻き戻す** 処理が必要です。

#### 動かして確かめる: 通常の更新と楽観的更新を時間つきで比べる

> **▼ このコードがやること（先に日本語で）:** 返事に 800 ミリ秒かかる「サーバーの代わり」の関数を作り、通常の更新と楽観的更新で「いつ、画面に何が出るか」を経過時間つきで表示します。楽観的更新については、サーバーが失敗した場合の巻き戻しも試します。図8-6 をコードで再現したものです。

**ファイル: `optimistic.ts`（練習用）**

```ts
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
```

**▼ 実行結果**（`node optimistic.ts`。時間は 100ms 単位に丸めています）

```text
■ 通常の更新（成功）
     0ms 画面: 読書中 （保存中...）
   800ms 画面: 読了 （保存しました）
■ 楽観的更新（成功）
     0ms 画面: 読了 （先に表示）
   800ms 画面: 読了 （サーバーも成功）
■ 楽観的更新（失敗）
     0ms 画面: 読了 （先に表示）
   800ms 画面: 読書中 （巻き戻してエラーを表示）
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 返事が遅いサーバーを `setTimeout` で作る

```ts
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
```

- `setTimeout(処理, 800)` は「800 ミリ秒後に処理を実行して」という予約です。その中で `resolve()`（成功）か `reject(...)`（失敗）を呼ぶので、`await fakeServerUpdate(...)` は 800 ミリ秒待たされます。
- 本物の Supabase への通信も、こうして「少し待たされてから成功か失敗が返ってくる」Promise です。

##### 解説2: 通常の更新は「成功してから」新しい値を出す

```ts
show({ status: "読書中", note: "（保存中...）" });
try {
  await fakeServerUpdate(shouldFail);
  show({ status: "読了", note: "（保存しました）" });
```

- `await` の **後ろ** で新しい値を表示しているのがポイントです。`await` の行で 800ms 止まるので、成功が確定するまで画面は「読書中（保存中...）」のままです。
- この章の `BookForm` も同じで、`await supabase.from("books").update(...)` が終わってから `router.push` で詳細ページに移ります。

##### 解説3: 楽観的更新は「先に出して、失敗したら戻す」

```ts
const before: ScreenState = { status: "読書中", note: "" };
show({ status: "読了", note: "（先に表示）" });
try {
  await fakeServerUpdate(shouldFail);
  show({ status: "読了", note: "（サーバーも成功）" });
} catch {
  show({ ...before, note: "（巻き戻してエラーを表示）" });
}
```

- `await` の **前** で新しい値を出しているので、待ち時間なしに見えます。
- その代わり、変更前の値を `before` に覚えておき、`catch`（失敗）に来たらそれを表示し直しています。この「巻き戻し」を忘れると、**データベースは「読書中」なのに画面は「読了」** という食い違いが残ってしまいます。

> **用語:** **楽観的更新（optimistic update）** は「たぶん成功する」と楽観的に考えて先に画面を変える方式。反対に、返事を待ってから変える方式を **悲観的更新（pessimistic update）** と呼ぶことがあります。

> **ヒント:** React 19 には、楽観的更新のための `useOptimistic` というフックがあります。処理中だけ「仮の値」を表示し、処理が終わると親から渡された本当の値の表示に自動で戻ります（成功してデータが取り直されていれば新しい値、失敗なら古い値）。そのため、巻き戻しを自分で書かずに済みます。まずはこの章の「待ってから反映する」方式に慣れ、物足りなくなったら試してみてください。

## UNIT 05: 完了の切り替え・1文の SQL

### この単元のゴール

- 完了の切り替え（`POST /notes/{id}/toggle`）が、1つの `UPDATE` の中で `done` を反転し、`version` と `updated_at` も進めて、303 で詳細へ戻ることを確かめられる
- 「今の値を読んでから、Rust で反転して書く」形にすると、同時に2つ届いたときに**1回分が消える**ことを、実際に確かめて説明できる
- `done = 1 - done` と、`CHECK (done IN (0, 1))` が、どう守り合っているかを言える。`done + 1` や `!done` と書いたときの 500 を、ログから読める
- 「反転する」SQL と「値を決めて入れる」SQL の違い（2回送ったときの結果）を説明できる

### たとえ話でつかむ

体育館の照明を、2人の係が同時に操作する場面を考えます。

- **見てから決める係:** 2人とも照明を見て「消えている」と思い、「つけよう」と決めて、それぞれスイッチを「入」にします。照明は「入」。2人が操作したのに、1回分の操作しか起きていません。見てから操作するまでの間に、相手が動いたことに気づけないからです。
- **押すたびに切り替わるボタン:** 2人が順にボタンを押すと、「入」→「切」と、ちゃんと2回切り替わります。ボタンの中の仕組みが、「今の状態を見て、反対にする」を**一度に**やっているので、間に誰かが入り込めません。

| 体育館の照明 | RustNote の完了の切り替え |
|---|---|
| 見てから「入」にする係 | `SELECT done` で読み、Rust で `!done` にして、`UPDATE … SET done = ?` で書く（エラー1） |
| 押すたびに切り替わるボタン | `UPDATE notes SET done = 1 - done …`（1つの SQL の中で読んで、書く） |
| スイッチに「入」と「切」しかない | `CHECK (done IN (0, 1))`（0 と 1 以外は DB が受け付けない） |

> **用語:** 競合状態（race condition：2つの処理が同時に進むとき、どちらが先に進むかによって、結果が変わってしまうこと）

<!-- -->

> **用語:** 1文の SQL（ここでは、「読む」と「書く」を、1つの `UPDATE` の中で済ませること。SQLite は、1つの文を途中で止めずに、まとめて行う）

<!-- -->

> **用語:** CHECK 制約（表の列に「この値しか入れてはいけない」と決めておく決まり。破る値を入れようとすると、DB がエラーにする。第7巻 UNIT 01）

### 図で理解する

同じノートに、切り替えが2つ同時に届いたときの流れです（はじめは `done=0`）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="180" y="22" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">読んでから書く（エラー1）</text>
<rect x="20" y="34" width="320" height="250" rx="10" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="100" y="60" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">1つ目</text>
<text x="260" y="60" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">2つ目</text>
<text x="100" y="92" font-size="11" fill="#0f172a" text-anchor="middle">① 読む: done=0</text>
<text x="260" y="122" font-size="11" fill="#0f172a" text-anchor="middle">② 読む: done=0</text>
<text x="100" y="152" font-size="11" fill="#0f172a" text-anchor="middle">③ 書く: done=1</text>
<text x="260" y="182" font-size="11" fill="#0f172a" text-anchor="middle">④ 書く: done=1</text>
<line x1="180" y1="70" x2="180" y2="200" stroke="#fca5a5" stroke-width="2" stroke-dasharray="4 4"/>
<text x="180" y="226" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">最後は done=1</text>
<text x="180" y="248" font-size="11" fill="#475569" text-anchor="middle">2回押したのに、1回分しか切り替わらない</text>
<text x="180" y="266" font-size="11" fill="#475569" text-anchor="middle">（②は、③で変わる前の値を読んでいる）</text>
<text x="540" y="22" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">1文の SQL（RustNote）</text>
<rect x="380" y="34" width="320" height="250" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="460" y="60" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">1つ目</text>
<text x="620" y="60" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">2つ目</text>
<rect x="400" y="80" width="120" height="44" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="460" y="98" font-size="11" fill="#0f172a" text-anchor="middle">① 1 - done</text>
<text x="460" y="114" font-size="11" fill="#0f172a" text-anchor="middle">0 → 1</text>
<rect x="560" y="140" width="120" height="44" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="620" y="158" font-size="11" fill="#0f172a" text-anchor="middle">② 1 - done</text>
<text x="620" y="174" font-size="11" fill="#0f172a" text-anchor="middle">1 → 0</text>
<line x1="540" y1="70" x2="540" y2="200" stroke="#86efac" stroke-width="2" stroke-dasharray="4 4"/>
<text x="540" y="226" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">最後は done=0</text>
<text x="540" y="248" font-size="11" fill="#475569" text-anchor="middle">読むと書くの間に、ほかの処理が入れない</text>
<text x="540" y="266" font-size="11" fill="#475569" text-anchor="middle">2回押せば、2回切り替わる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: 切り替えが2つ同時に届いたとき。読んでから書くと、2つ目が古い値を読んで1回分が消える。1文の SQL なら、読むと書くの間が無いので、2回きちんと切り替わる</div>
</div>

切り替えの SQL が、1回でやっていることです（`src/repository/notes.rs` の `toggle_done`）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="16" width="430" height="96" rx="8" fill="#0f172a"/>
<text x="36" y="40" font-size="12" fill="#e2e8f0" font-family="Consolas, monospace">UPDATE notes</text>
<text x="36" y="58" font-size="12" fill="#e2e8f0" font-family="Consolas, monospace">   SET <tspan fill="#fbbf24">done = 1 - done</tspan>, version = version + 1,</text>
<text x="36" y="76" font-size="12" fill="#e2e8f0" font-family="Consolas, monospace">       updated_at = datetime('now')</text>
<text x="36" y="94" font-size="12" fill="#e2e8f0" font-family="Consolas, monospace"> WHERE id = ?1 AND user_id = ?2 RETURNING …</text>
<rect x="470" y="16" width="230" height="96" rx="8" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="585" y="40" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">1 - done の値</text>
<text x="585" y="66" font-size="12" fill="#0f172a" text-anchor="middle">done=0 → 1 - 0 = 1</text>
<text x="585" y="90" font-size="12" fill="#0f172a" text-anchor="middle">done=1 → 1 - 1 = 0</text>
<rect x="20" y="130" width="220" height="86" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="130" y="154" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">version が 1 進む</text>
<text x="130" y="176" font-size="11" fill="#0f172a" text-anchor="middle">開いていた編集のフォームは</text>
<text x="130" y="194" font-size="11" fill="#0f172a" text-anchor="middle">古い版になる（問1）</text>
<rect x="250" y="130" width="220" height="86" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="154" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">updated_at が今になる</text>
<text x="360" y="176" font-size="11" fill="#0f172a" text-anchor="middle">一覧（新しい順）の</text>
<text x="360" y="194" font-size="11" fill="#0f172a" text-anchor="middle">いちばん上に移る（問3）</text>
<rect x="480" y="130" width="220" height="86" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="590" y="154" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">0 行なら 404</text>
<text x="590" y="176" font-size="11" fill="#0f172a" text-anchor="middle">無いノート・他人のノートは</text>
<text x="590" y="194" font-size="11" fill="#0f172a" text-anchor="middle">RETURNING が何も返さない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: 切り替えの SQL が1回でやること。done の反転・version と updated_at の更新・持ち主の確かめを、1つの UPDATE で行い、変わったあとの行を RETURNING で受け取る</div>
</div>

### まず動かす最小コード

#### 例5-1: 完了の切り替えを2回送る

切り替えの画面は、`src/routes/notes/id/toggle.rs` です。画面を返さず、成功したら必ず 303 で詳細へ戻すので、返す型は `Result<()>` です。

```rust
// 成功すると必ず 303 なので、画面として返す中身は無い（`()`）
#[page(POST)]
pub async fn toggle(cx: &Cx) -> Result<()> {
    let user = require_user(cx).await?;
    let id = *path_param::<Id>(cx)?;
    // 見つからなければ RepoError::NotFound → AppError::NotFound（404）
    app_state(cx)
        .notes
        .toggle_done(user.id, id)
        .await
        .map_err(AppError::from)?;
    flash::set(cx, Flash::NoteToggled);
    Err(see_other(href!(super::show, Id(id)).resolve(cx)).into())
}
```

> **▼ このコードがやること（先に日本語で）:** alice がノートを1件作り、`done` と `version` を表示します。切り替えの POST を2回送り、それぞれの返事（状態コード・移動先・お知らせのクッキー）を表示します。ブラウザと同じように、お知らせのクッキーも付けて移動先（詳細）を開き、完了の印・ボタンの文字・お知らせがあるかと、DB の `done` と `version` を表示します。

```rust
@@FILE:rustnote/tests/u05_toggle.rs@@
```

```powershell
cargo test --test u05_toggle -- --nocapture
```

```text
@@OUTGREP:u05_toggle:^(?!   Compiling)@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 1回ごとに done が反転し、version が進む

- 1回目のあと `done=1 version=2`、2回目のあと `done=0 version=3` でした。`1 - done` は、0 なら 1、1 なら 0 です。`version = version + 1` も同じ文の中にあるので、切り替えのたびに 1 進みます。
- 詳細の画面は、1回目のあとは「完了」の印があり、ボタンが「未完了に戻す」に、2回目のあとは印が消えて「完了にする」に戻りました。ボタンの文字は、`if note.done { "未完了に戻す" } else { "完了にする" }` で、DB の値から作っています（UNIT 03 の例3-2）。

##### 解説2: 読み直さずに、変わったあとの行を受け取る

- `toggle_done` の SQL の最後の `RETURNING {NOTE_COLUMNS}` は、「更新したあとの行を、そのまま返して」という指定です。変わったあとの値を知るために、もう一度 `SELECT` する必要がありません。
- 1行も返らなければ、id と持ち主の条件に合うノートが無かった、ということです。リポジトリは `RepoError::NotFound` を返し、画面は 404 になります（UNIT 03 の問3）。
- お知らせ（`flash=note_toggled`）は「完了の状態を切り替えました」です。「完了にしました」と書かないのは、この POST が、完了にするのか、戻すのかを、**送る側が決めていない**からです（問2）。

### もう一歩: 実務に近い形

#### 例5-2: 切り替えを2つ同時に送る

ボタンを素早く2回押した（ダブルクリックした）ときや、2つのタブで同時に押したときを、`tokio::join!`（2つの処理を同時に進める。第5巻 UNIT 01）で再現します。

> **▼ このコードがやること（先に日本語で）:** 未完了のノートを1件作り、そのノートへの切り替えの POST を、`tokio::join!` で2つ同時に送ります。両方の返事が来たら、DB の `done` と `version` を読みます。これを、毎回新しいノートで 20 回くり返し、最後が `done=0`（2回切り替わって元に戻った）だった回数と、`done=1`（1回分が消えた）だった回数を数えます。最初の3回は、返事と DB の値も表示します。

```rust
@@FILE:rustnote/tests/u05_race.rs@@
```

```powershell
cargo test --test u05_race -- --nocapture
```

```text
@@OUTGREP:u05_race:^\d+ 回目|^20 回@@
```

（返事と数の行だけを抜き出しています）

- 20 回とも、最後は `done=0`、`version=3` でした。2つの切り替えが、2回とも働いています。
- SQLite は、書き込みを1つずつ順に行います。2つの `UPDATE` が同時に届いても、片方が終わってから、もう片方が動きます。そして、それぞれの `UPDATE` は、**動いたときの今の `done`** を見て反転します。だから、どちらが先でも、2回分の切り替えになります。
- 2回押して元に戻るのは、利用者から見ると「何も起きなかった」ように見えます。でも、表示と DB は食い違っていません。2回目の返事の移動先を開けば、元に戻った状態が表示されます。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: 読んでから、Rust で反転して書いた

`toggle_done` を、「`find` で今のノートを読み、`!current.done` を `SET done = ?1` で書く」形にした場合です。読むと書くの間に、DB が遠いとき（Turso など）の待ち時間の代わりとして、50 ミリ秒の `sleep` を入れています（練習用のコピーの `src/repository/notes.rs` を書き換え、確かめたら元に戻します）。

```rust
let conn = self.db.connect().await?;
// わざと間違えた版: 今の done を読み、Rust で反転してから、書き込む
let current = self.find(owner, id).await?.ok_or(RepoError::NotFound)?;
// DB が遠い（Turso など）ときの、読むと書くの間の待ち時間の代わり
tokio::time::sleep(std::time::Duration::from_millis(50)).await;
let sql = format!(
    "UPDATE notes
        SET done = ?1, version = version + 1, updated_at = datetime('now')
      WHERE id = ?2 AND user_id = ?3
      RETURNING {NOTE_COLUMNS}"
);
let mut rows = conn
    .query(&sql, libsql::params![!current.done, id.get(), owner.get()])
    .await?;
```

例5-2のテストを、そのまま動かします。

```text
@@OUTGREP:u05_err_race:^\d+ 回目|^20 回@@
```

- **何が起きたか:** 20 回とも、最後が `done=1` でした。2回押したのに、1回分の切り替えが消えています。返事は2つとも `303`、お知らせも2つとも「切り替えました」なので、利用者にもテストの状態コードにも、何も知らされません。
- **version だけは 3:** `version = version + 1` は SQL の中で計算しているので、2回とも進みました。`done` だけが、Rust で計算した値（どちらも、読んだときの `0` を反転した `1`）で上書きされたのです。同じ1つの `UPDATE` の中でも、**DB の中で計算した値**と、**前に読んで Rust で計算した値**とで、結果が分かれました。
- **直し方:** RustNote のとおり、`SET done = 1 - done` と、DB の中で反転させます。どうしても Rust で計算したいときは、UNIT 04 の楽観的ロックのように、`WHERE … AND version = ?` を付けて、読んだあとに変わっていたら 0 行になるようにします。

#### エラー2: done + 1 と書いた（CHECK constraint failed）

`SET done = 1 - done` を `SET done = done + 1` と書いた場合です。例5-1のテストを動かします。

```text
@@OUTGREP:u05_err_check:^(?!   Compiling|    Finished|     Running|running|\s*$|test |exit=|POST /signup|POST /notes ->)@@
```

（切り替えの行とログの行だけを抜き出しています）

- **何が起きたか:** 1回目（0 → 1）は成功し、**2回目に 500** になりました。ログの `<-` を右へたどると、`CHECK constraint failed: done IN (0, 1)` です。`1 + 1 = 2` を入れようとして、表の決まり（`migrations/0002_notes.sql` の `CHECK (done IN (0, 1))`）に止められました。
- **守られたこと:** DB は `done=1 version=2` のままで、`2` という、ありえない値は入っていません。もし CHECK 制約が無かったら、2回目は成功して `done=2` になります。`row_to_note` は `row.get::<i64>(4)? != 0` で `bool` に変えるので、`2` は「完了」と読まれ、次の切り替えでも `3` になって、**いつまでも未完了に戻せないノート**ができてしまいます。
- **学べること:** 決まりを、Rust の型（`bool`）と、DB の CHECK 制約の**両方**に書いておくと、SQL を書き間違えても、おかしな値は保存されません。

#### エラー3: !done と書いた（unrecognized token）

Rust の書き方につられて、`SET done = !done` と書いた場合です。

```text
@@OUTGREP:u05_err_bang:^(?!   Compiling|    Finished|     Running|running|\s*$|test |exit=|POST /signup|POST /notes ->)@@
```

- **何が起きたか:** 1回目から 500 で、ログは `SQLite failure: unrecognized token: "!"`（「`!` という記号は知りません」）です。SQL には、`!` という否定の書き方がありません（SQL の否定は `NOT` です）。
- **気づくのが遅い理由:** SQL は、Rust の文字列（`format!`）の中に書いているので、コンパイラーは中身を調べません。間違いが分かるのは、その SQL を実際に動かしたときです。だから、SQL ごとに、一度は動かすテストを用意しておきます（第11巻 UNIT 02 のリポジトリのテスト）。

### RustNote ではここで使う

- 切り替えは `src/routes/notes/id/toggle.rs` の `toggle` と、`src/repository/notes.rs` の `toggle_done` です。SQL は1つで、`find_own_note` を通りません（UNIT 03 の問3）。
- `done` の列は `migrations/0002_notes.sql` で、`done INTEGER NOT NULL DEFAULT 0 CHECK (done IN (0, 1))` です。SQLite には真偽値の型が無いので、0 と 1 の整数で持ち、Rust の `row_to_note` で `bool` に変えています。
- 詳細の画面の切り替えのボタンは、`<form method="post">` の中の `<button>` です（GET のリンクではありません。UNIT 03 の例3-2）。

### 練習問題

**問1:** 編集のフォームを開いたあとで、別のタブで完了を切り替えました。それから、開いておいた編集のフォームで保存すると、どうなるでしょうか。タイトルも本文も、誰も変えていないのに、そうなるのはなぜでしょうか。

**問2:** 「反転する」SQL（`SET done = 1 - done`）と、「値を決めて入れる」SQL（`SET done = 1`）を、それぞれ2回ずつ実行すると、`done` はどうなるでしょうか。ダブルクリックに強いのは、どちらでしょうか。

**問3:** 1件目・2件目・3件目の順に作ったノートの一覧で、いちばん古い1件目の完了を切り替えると、一覧の順番はどうなるでしょうか。

### 解答と解説

```rust
@@FILE:rustnote/tests/u05_q.rs@@
```

```powershell
cargo test --test u05_q -- --nocapture --test-threads=1
```

```text
@@OUTGREP:u05_q:^問@@
```

（問題の答えの行だけを抜き出しています）

**問1の解答:**

- **`409 Conflict`** でした。切り替えの SQL が `version = version + 1` で版を進めたので、開いておいたフォームの `version=1` は古くなったからです。
- タイトルと本文は変わっていないので、保存しても、誰の内容も消えません。それでも 409 にするのは、`version` が「ノートのどこかが変わった」印だからです。「完了の印も含めて、最新を見てから保存してください」という、安全なほうの決め方です。もし切り替えで `version` を進めなければ、この保存は通り、困ることもありません。どちらにするかは、設計の選び方です。

**問2の解答:**

- 反転する SQL は、1回目のあと `1`、2回目のあと `0` でした。値を決めて入れる SQL は、1回目も2回目も `1` です。
- 値を決めて入れるほうは、何回送っても結果が同じです（**冪等**。UNIT 02 の用語）。ボタンを「完了にする」と「未完了に戻す」の2種類にし、フォームで `done=1` か `done=0` を送る作りにすれば、ダブルクリックしても、利用者が思ったとおりの状態になります。
- RustNote は、URL を1つ（`/toggle`）で済ませるために、反転する形にしています。その代わり、お知らせを「完了の状態を切り替えました」として、「完了にしました」と言い切らないようにしています。

**問3の解答:**

- 切り替える前は `["3件目", "2件目", "1件目"]`（新しい順）で、1件目を切り替えると `["1件目（完了）", "3件目", "2件目"]` になりました。
- 切り替えで `updated_at` が今になり、一覧は `updated_at DESC, id DESC`（UNIT 01）で並ぶので、いちばん上に移りました。「最近さわったノートが上」という並びなので、完了にしたノートも上に来ます。完了したノートを下に集めたいなら、`ORDER BY done, updated_at DESC, id DESC` のように、並びの決まりを変えます。

### まとめ

- 完了の切り替えは、`UPDATE notes SET done = 1 - done, version = version + 1, updated_at = datetime('now') WHERE id = ?1 AND user_id = ?2 RETURNING …` の1文で、反転・版・日時・持ち主の確かめを一度に行い、変わったあとの行を受け取る。
- 読んでから Rust で反転して書くと、同時に2つ届いたときに1回分が消える（20 回中 20 回）。DB の中で計算させれば、読むと書くの間が無いので、2回きちんと切り替わる。
- `CHECK (done IN (0, 1))` が、`done + 1` のような書き間違いから DB を守る。SQL の文字列はコンパイラーが調べないので、`!done` のような間違いは、動かして初めて 500 で分かる。反転より、値を決めて入れるほうが、ダブルクリックに強い（冪等）。

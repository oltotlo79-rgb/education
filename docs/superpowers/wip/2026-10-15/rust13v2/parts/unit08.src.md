## UNIT 08: ノートの CRUD

### この単元のゴール

- `src/routes/notes.rs` と `src/routes/notes/` の4つのファイルが、ノートの一覧・作成・詳細・編集・切り替え・削除（CRUD）を、どの URL・どのメソッド・どの状態コードで扱うかを、本物のサーバーの応答で言える
- `module_param!` で URL の `{id}` を `NoteId` に変え、数字でない・0・他人のノート・無いノートを、どれも 404 にするしくみを説明できる
- 編集の楽観的ロックで、2つのタブから保存したときに 409 になり、もう一度保存すると上書きされる流れを確かめられる
- `href!` に ID を渡し忘れると、コンパイルは通るのに、表示のときにパニックして 500 になることを、テストが止めることと合わせて確かめられる

### たとえ話でつかむ

会社の、1人1枚の掲示板を考えます。

- 自分の掲示板には、**番号付きの付箋**（ノート）を貼ります。付箋を見る・書く・書き直す・済の印を付ける・はがす、の5つができます。
- 「7番の付箋を見せて」と頼まれても、それが**他人の掲示板の付箋**なら、「7番はありません」と答えます。「7番は田中さんのです」とは言いません。「あいうえお番」や「0番」のような番号も、「ありません」です。
- 付箋を書き直すときは、「**何版目を見て書き直したか**」を添えます。2人が同じ版を見て書き直すと、あとの人には「先に書き直されています。今の題は『…』です。それでも書き直しますか？」と聞きます。
- 付箋の一覧は、**20 枚ずつのページ**に分けます。

| 掲示板 | RustNote（`src/routes/notes.rs`・`src/routes/notes/`） |
|---|---|
| 付箋の一覧と、書く | `GET /notes`・`POST /notes`（`notes.rs`） |
| 1枚を見る | `GET /notes/{id}`（`notes/id.rs`） |
| 書き直す | `GET`・`POST /notes/{id}/edit`（`notes/id/edit.rs`） |
| 済の印を付ける・外す | `POST /notes/{id}/toggle`（`notes/id/toggle.rs`） |
| はがす | `POST /notes/{id}/delete`（`notes/id/delete.rs`） |
| 他人の付箋・おかしな番号は「ありません」 | どれも 404 |
| 何版目を見て書き直したか | 隠し項目 `version`（違えば 409） |

> **用語:** CRUD（クラッド：Create（作る）・Read（読む）・Update（書き直す）・Delete（消す）の頭文字。データを扱う画面の、基本の4つの操作）

<!-- -->

> **用語:** `module_param!`（モジュール・パラム：Topcoat のマクロ。そのモジュールの URL の区切りを `{id}` のような変数にし、文字列を決まった型に変える）

### 図で理解する

ノートの URL と、それぞれの結果です（例8-4の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="700" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="22" y="32" font-size="12" font-weight="700" fill="#1d4ed8">notes.rs</text>
<text x="190" y="32" font-size="11" fill="#0f172a">GET /notes（?q=・?page=）→ 200　｜　POST /notes → 303 /notes/{id}・400</text>
<rect x="40" y="56" width="670" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="52" y="78" font-size="12" font-weight="700" fill="#047857">notes/id.rs</text>
<text x="220" y="78" font-size="11" fill="#0f172a">GET /notes/{id} → 200（module_param! で {id} を NoteId に）</text>
<rect x="70" y="102" width="640" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="82" y="124" font-size="12" font-weight="700" fill="#0f172a">notes/id/edit.rs</text>
<text x="250" y="124" font-size="11" fill="#0f172a">GET → 200　｜　POST → 303・400・409（version が古い）</text>
<rect x="70" y="146" width="640" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="82" y="168" font-size="12" font-weight="700" fill="#0f172a">notes/id/toggle.rs</text>
<text x="250" y="168" font-size="11" fill="#0f172a">POST → 303 /notes/{id}（完了 ⇔ 未完了）</text>
<rect x="70" y="190" width="640" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="82" y="212" font-size="12" font-weight="700" fill="#0f172a">notes/id/delete.rs</text>
<text x="250" y="212" font-size="11" fill="#0f172a">POST → 303 /notes</text>
<rect x="10" y="238" width="700" height="44" rx="6" fill="#fef2f2" stroke="#ef4444" stroke-width="1.5"/>
<text x="22" y="257" font-size="11" fill="#b91c1c">{id} のある URL はどれも: 数字でない（abc）・0 以下・無い（999）・他人のノート → 404</text>
<text x="22" y="274" font-size="11" fill="#b91c1c">ログインしていない → 303 /login（require_user）。?page=abc → 400（query_params）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: ノートの URL の地図。ファイルの入れ子が URL の入れ子で、{id} の下に、編集・切り替え・削除が並ぶ</div>
</div>

2つのタブで、同じノートを編集したときの流れです（例8-4の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-0802" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="80" y="24" font-size="13" font-weight="700" fill="#1d4ed8" text-anchor="middle">タブ A</text>
<text x="360" y="24" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">DB の notes.version</text>
<text x="640" y="24" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">タブ B</text>
<line x1="80" y1="32" x2="80" y2="240" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 4"/>
<line x1="360" y1="32" x2="360" y2="240" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 4"/>
<line x1="640" y1="32" x2="640" y2="240" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 4"/>
<rect x="320" y="40" width="80" height="24" rx="4" fill="#ecfdf5" stroke="#10b981"/>
<text x="360" y="57" font-size="12" fill="#047857" text-anchor="middle">1</text>
<text x="90" y="58" font-size="11" fill="#0f172a">編集を開く（version 1）</text>
<text x="630" y="58" font-size="11" fill="#0f172a" text-anchor="end">編集を開く（version 1）</text>
<line x1="80" y1="90" x2="356" y2="90" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0802)"/>
<text x="90" y="84" font-size="11" fill="#1d4ed8">「土曜」・version 1 で保存 → 303</text>
<rect x="320" y="96" width="80" height="24" rx="4" fill="#ecfdf5" stroke="#10b981"/>
<text x="360" y="113" font-size="12" fill="#047857" text-anchor="middle">2</text>
<line x1="640" y1="146" x2="364" y2="146" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0802)"/>
<text x="630" y="140" font-size="11" fill="#c2410c" text-anchor="end">「日曜」・version 1 で保存 → 409</text>
<text x="630" y="164" font-size="10" fill="#c2410c" text-anchor="end">最新の題「買い物（土曜）」を知らせ、隠し項目を 2 に</text>
<line x1="640" y1="200" x2="364" y2="200" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0802)"/>
<text x="630" y="194" font-size="11" fill="#c2410c" text-anchor="end">もう一度保存（version 2）→ 303・上書き</text>
<rect x="320" y="206" width="80" height="24" rx="4" fill="#ecfdf5" stroke="#10b981"/>
<text x="360" y="223" font-size="12" fill="#047857" text-anchor="middle">3</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: 2つのタブの編集。あとから保存したタブ B は、いったん 409 で止まり、最新の題を見たうえで、もう一度押すと上書きする</div>
</div>

### まず動かす最小コード

#### 例8-1: src/routes/notes.rs の全文

> **▼ このファイルがやること（先に日本語で）:** ノートの一覧（`GET /notes`）と作成（`POST /notes`）です。一覧は、URL の `?q=`（キーワード）と `?page=`（ページの番号）で検索し、検索のフォーム・作成のフォーム・ノートの一覧・ページ送りを出します。作成は、検査に通れば 303 でそのノートの詳細へ、通らなければ 400 で入力を残したまま一覧と一緒に再表示します。

```rust
@@FILE:rustnote/src/routes/notes.rs@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: URL のクエリを読む

- `#[query_params(error = bad_request)]` を付けた `NotesParams` は、`?q=…&page=…` を読む型です。`page` は `Option<u32>` なので、無ければ `None`、`abc` のように数でなければ、ページの処理の前に 400 になります（例8-4）。
- 読んだ値は、`NoteQuery::new(params.q.as_deref(), params.page)` で、検査済みの検索の条件にします（キーワードの trim・100 文字まで、ページの 0 を 1 に。UNIT 04）。

##### 解説2: 部品の中で DB を検索する

- `notes_index` は `#[component]` で、しかも `async` なので、部品の中で `app_state(cx).notes.search(…)` を `.await` できます。一覧のページ（`index`）と、作成に失敗したときの再表示（`create`）の両方が、この部品を使います。
- `for note in page.notes.clone()` の前の `#[key(note.id.get())]` は、くり返しの1つ1つに、ノートの ID を「目印」として付けます。
- `class!("note-item", "done" if note.done)` は、完了したノートにだけ `done` のクラスを足します（UNIT 09 の CSS で、打ち消し線などを付ける）。
- 作成のフォームは `<details>` の中にあり、ふだんは閉じています。`open=(form.title_error.is_some() || form.body_error.is_some())` で、入力にエラーがあるときだけ開いた状態で返します（例8-4）。

##### 解説3: 作成は PRG、だめなら 400 で再表示

- `create` は、`NewNote::parse` に通れば保存し、`flash::set(cx, Flash::NoteCreated)` と、`href!(id::show, id::Id(note.id))` の 303 で、作ったノートの詳細へ移ります（PRG。UNIT 07）。
- 通らなければ、`NoteFormState::from_input(…).with_errors(&errors)` で、入力とエラー文を入れたフォームを作り、400 で一覧ごと再表示します。タイトルと本文の両方の問題が、それぞれの欄の下に一度に出ます（UNIT 04 の `NewNote::parse`）。

#### 例8-2: src/routes/notes/id.rs の全文

> **▼ このファイルがやること（先に日本語で）:** ノートの詳細（`GET /notes/{id}`）です。`module_param!` で、このモジュールの URL の区切りを `{id}` にし、`NoteId` に変えます。自分のノートを探す `find_own_note` も、ここにあり、編集でも使います。

```rust
@@FILE:rustnote/src/routes/notes/id.rs@@
```

##### 解説4: {id} は module_param! で NoteId に

- `module_param!(pub id: NoteId, error = not_found);` は、このモジュール（`notes/id.rs`）の URL の区切りを `{id}` にし、`Id` という型を作ります。文字列は、UNIT 04 の `NoteId` の `FromStr` で変換し、失敗（数字でない・0 以下）なら `NotFoundError`（404）にします。
- 読むときは `path_param::<Id>(cx)?` です。返ってくるのは `&NoteId`（借り物）なので、`*` を付けて `NoteId` の値にします（`NoteId` は `Copy`）。`NoteId` の型そのものでは読めません（エラー2）。
- リンクを作るときは `href!(edit::edit_page, Id(id))` のように、`Id` で包んだ値を渡します。渡し忘れても、コンパイルは通ってしまいます（エラー1）。

##### 解説5: 他人のノートも 404

- `find_own_note` は、`notes.find(owner, id)` が `None` なら `AppError::NotFound` を返します。リポジトリの SQL が持ち主で絞っているので（UNIT 05）、他人のノートも「無い」のと同じ 404 です。403（見る権利がない）にすると、「その番号のノートは、だれかのものとしてある」と知らせてしまいます。
- 本文は `<div class="note-body">(note.body)</div>` で、自動でエスケープされます。改行は、HTML の `<br>` に変えずに、CSS の `white-space: pre-wrap` で表示します（UNIT 09）。
- 「完了にする」と「削除」は、どちらも `POST` のフォームのボタンです。状態を変える操作は、リンク（GET）にしません（UNIT 07 のログアウトと同じ）。

#### 例8-3: 編集・切り替え・削除の全文

> **▼ このファイルがやること（先に日本語で）:** `edit.rs` は、編集のフォーム（今の `version` を隠し項目に入れる）と保存（楽観的ロック）です。`toggle.rs` は完了と未完了の切り替え、`delete.rs` は削除で、どちらも成功したら 303 で移ります。

```rust
@@FILE:rustnote/src/routes/notes/id/edit.rs@@
```

```rust
@@FILE:rustnote/src/routes/notes/id/toggle.rs@@
```

```rust
@@FILE:rustnote/src/routes/notes/id/delete.rs@@
```

##### 解説6: 編集の保存は、404 → 400 → 409 の順に見る

- `update` は、まず `find_own_note` で、他人のノート・無いノートを 404 にします。入力の検査より先にするのは、他人のノートの ID に、わざとまちがった入力を送られても、「400（入力が違う）」と答えて、ノートがあることを知らせないためです。
- 次に、`NewNote::parse` が通らなければ 400 で、入力とエラー文を残して再表示します。
- 通れば、`notes.update(…, input.version)` で保存します。`VersionMismatch` なら、最新のノートを読み直し、「最新のタイトルは『…』です」というお知らせと、**最新の `version`** を隠し項目に入れて、409 で再表示します。入力した内容は残っているので、もう一度「保存する」を押すと、今度は最新の `version` で保存され、上書きになります（例8-4・問2）。

##### 解説7: 切り替えと削除は、画面を返さない

- `toggle` と `delete` は、`Result<()>` を返す `#[page(POST)]` です。成功したら必ず `Err(see_other(…))` の 303 なので、画面として返す中身がありません。
- 他人のノート・無いノートは、リポジトリが `RepoError::NotFound` を返し、`AppError::from` で 404 になります。

### もう一歩: 実務に近い形

#### 例8-4: 本物のサーバーで、ノートの CRUD を確かめる

UNIT 07 と同じく、release の実行ファイルを一時フォルダで起動し、`curl` で送りました（本文は `<main>` の中だけを、タグの区切りで改行）。最初に alice で登録し、そのクッキーを使っています。

**▼ ① 作成（タイトル「買い物」、本文「牛乳」改行「卵」）と、その詳細**

```text
@@OUT:u08_create_ok@@
```

```text
@@OUT:u08_show@@
```

**▼ ② タイトルが空で、本文が 10,001 文字の作成**

```text
@@OUT:u08_create_bad@@
```

**▼ ③ 一覧（`GET /notes`）と、数でないページの番号（`?page=abc`）**

```text
@@OUT:u08_list@@
```

```text
@@OUT:u08_page_abc@@
```

**▼ ④ 編集のフォーム**

```text
@@OUT:u08_edit@@
```

**▼ ⑤ 2つのタブ: A が version 1 で「買い物（土曜）」を保存し、B も version 1 で「買い物（日曜）」を保存する**

```text
@@OUT:u08_save_a@@
```

```text
@@OUT:u08_save_b@@
```

**▼ ⑥ B が、そのまま、もう一度「保存する」を押す（隠し項目は version 2）**

```text
@@OUT:u08_save_b2@@
```

**▼ ⑦ 完了に切り替えて、詳細を開く**

```text
@@OUT:u08_toggle@@
```

```text
@@OUT:u08_show_done@@
```

**▼ ⑧ bob（別のユーザー）で、alice のノート 1 を開く・切り替える・保存する**

```text
@@OUT:u08_bob_show@@
```

```text
@@OUT:u08_bob_toggle@@
```

```text
@@OUT:u08_bob_edit@@
```

**▼ ⑨ alice で、`/notes/abc`・`/notes/0`・`/notes/999` を開く**

```text
@@OUT:u08_abc@@
```

```text
@@OUT:u08_zero@@
```

```text
@@OUT:u08_missing@@
```

**▼ ⑩ タイトルが NUL（`%00`）1文字の作成**

```text
@@OUT:u08_nul@@
```

**▼ ⑪ 削除と、そのあと**

```text
@@OUT:u08_delete@@
```

```text
@@OUT:u08_after_delete@@
```

**▼ この間のサーバーのログ**

```text
@@OUT:u08_server_log@@
```

- ①の作成は 303 で `/notes/1` へ移りました。本文の改行（フォームの `\r\n`）は、`\n` にそろえて保存され、HTML でもそのまま改行として入っています。
- ②は 400 で、`<details>` が `open` になり、2つのエラー文が欄の下に出て、入力（10,001 文字の本文）も残りました。下には、今の一覧（ノート1件）も出ています。
- ⑤の B は 409 で、「最新のタイトルは『買い物（土曜）』です」と知らされ、隠し項目が `2` に変わりました。⑥でもう一度押すと 303 になり、⑦の詳細のタイトルは「買い物（日曜）」でした。A の「土曜」は、B が知らされたうえで上書きしました。
- ⑧の bob は、見る・切り替える・保存する、のどれも 404 でした。⑨のおかしな ID も、無い ID も、同じ 404 です。
- ⑩の NUL のタイトルは、`NewNote::parse` を通って DB の `CHECK` で断られ、500 になりました（UNIT 04・第11巻 UNIT 04）。ログには、`[error]` の行に、原因の鎖（`CHECK constraint failed: length(title) BETWEEN 1 AND 80`）が書かれ、画面には「時間をおいてもう一度お試しください」だけが出ました。

### よくあるエラーと読み方

#### エラー1: 一覧のリンクの `href!` に、ID を渡し忘れた（コンパイルは通る）

教材のコードのコピーで、一覧のリンクを `href!(id::show, id::Id(note.id))` から `href!(id::show)` に変えて、`cargo build` し、サーバーで試しました。

```text
@@OUT:u08_err1_build@@
```

**▼ ノートが0件の一覧 → ノートを1件作る → もう一度、一覧**

```text
@@OUT:u08_err1_empty@@
```

```text
@@OUT:u08_err1_list@@
```

```text
@@OUTTAIL:u08_err1_log:6@@
```

- **何が起きたか:** コンパイルは通り（`exit=0`）、ノートが0件の一覧も表示できました。ノートを1件作ると、一覧を表示する所で、`href!` が「`/notes/{id}` の `id` に値がありません」とパニックし、500 が返りました。
- **読み方:** `href!` は、行き先がページの関数かどうかはコンパイルのときに確かめますが（UNIT 07 のエラー3）、`{id}` の値を渡したかどうかは、URL を組み立てる（表示する）ときに確かめます。しかも、この形のリンクは、ノートがあるときにしか作られないので、0件の画面を見ただけでは気づけません。
- **見つけ方:** パニックした `GET /notes` は、アクセスログの行がありません（パニックで、`access_log` の続きが動かなかった）。HTTP のテストは、ノートを 22 件作ってから一覧を見るので、ここで止まりました。

```text
@@OUTGREP:u08_err1_test:^(running|list_searches|thread|no value|assertion|  left|  right|test result)@@
```

- **直し方:** `href!(id::show, id::Id(note.id))` のように、`{id}` の値を `Id` で包んで渡します。

#### エラー2: URL の `{id}` を、`Id` ではなく `NoteId` の型で読もうとした（E0277）

```rust
@@FILE:errs_u08/e08_path.rs@@
```

```text
@@OUTHEAD:u08_err_path:18@@
  …（同じエラーが、もう1つ続く）…
@@OUTGREP:u08_err_path:^error: could not@@
```

- **何が起きたか:** 「`NoteId` は `PathParam`（URL の変数として読める型）ではありません」というエラーです。`help:` が「`PathParam` は `Id` に実装されています」と、`module_param!` の行を指しています。
- **直し方:** `path_param::<Id>(cx)?` で読み、`*` で `NoteId` を取り出します。`Id` は、`module_param!` が作った「このモジュールの `{id}`」専用の型です。

#### エラー3: フォームを読む型に、`#[derive(Deserialize)]` を付け忘れた（E0277）

```rust
@@FILE:errs_u08/e08_form.rs@@
```

```text
@@OUTHEAD:u08_err_form:20@@
  …（同じ形のエラーが、もう1つ続く）…
@@OUTGREP:u08_err_form:^(   = note: for local types|error: could not)@@
```

- **何が起きたか:** 「`NoteForm` は `DeserializeOwned`（送られてきたデータから作れる型）ではありません」というエラーです。`Form<T>` は、フォームの中身を `serde` で `T` に読むので、`T` に `Deserialize` が要ります。
- **直し方:** 2つ目のエラーの `note:` のとおり、`#[derive(Deserialize)]`（`use serde::Deserialize;`）を付けます。項目の名前は、フォームの `name` 属性と同じにします。

### RustNote ではここで使う

- **ログインが必要:** 7つのページ（一覧・作成・詳細・編集のフォーム・保存・切り替え・削除）は、どれも最初に `require_user(cx)` を呼びます。ログインしていなければ、303 でログインの画面へ移ります（UNIT 06）。
- **テスト:** `tests/http.rs` の 14 のテストのうち、この単元の分は8つです（UNIT 10）。

```text
@@OUTGREP:u08_http_tests:^(running|test |test result)@@
```

- **改善点（この単元で見つかったこと）:**
  - ページの番号が、最後のページより大きいと、ノートがあるのに「まだノートがありません。」と出ます（問1）。
  - 作成日時と更新日時は、DB の `datetime('now')` の UTC のまま表示されます（問3）。
  - 「削除」のボタンは、確かめずに、すぐ消します。
  - NUL のタイトルで 500 になります（⑩）。

### 練習問題

**問1:** 例8-4のノートが1件の状態で、`/notes?page=99` を開くと、何が出るでしょうか。どう直せばよいでしょうか。

**問2:** 例8-4の⑤⑥で、タブ B は、409 のあと、もう一度「保存する」を押しただけで、A の保存を上書きしました。これは良い作りでしょうか。

**問3:** 例8-4の①の詳細の「作成: 2026-10-30 21:08:30」は、何時でしょうか。このとき、PC の時計は次のとおりでした。

```text
@@OUT:u08_clock@@
```

### 解答と解説

**問1の解答:**

```text
@@OUT:u08_page_99@@
```

- ノートが1件あるのに、「まだノートがありません。最初の1件を書いてみましょう。」と出ました。ページ送りも「99 / 1 ページ」で、98 ページへの「前へ」が出ています。99 ページ目には、表示するノートが無いだけです。

教材のコードのコピーで、空のときの文を、「総数が 0 でなければ『このページにはノートがありません。』」にして試しました。

```diff
-                if keyword.is_some() {
+                if page.total_count > 0 {
+                    "このページにはノートがありません。"
+                } else if keyword.is_some() {
                     "条件に合うノートはありません。"
                 } else {
```

```text
@@OUTGREP:u08_q1_page_99:^(<p class="empty|<a href|<span>)@@
```

```text
@@OUTGREP:u08_q1_page_1:^(<input name="q"|<p class="empty|<span>)@@
```

```text
@@OUTGREP:u08_q1_test:^(running|test result)@@
```

- `?page=99` は「このページにはノートがありません。」になり、検索で何も当たらないとき（`?q=zzz`）は、今までどおり「条件に合うノートはありません。」でした。HTTP のテストは 14 個とも通りました。
- さらに良くするなら、最後のページより大きい番号は、303 で最後のページへ移す、という作り方もあります。

**問2の解答:**

- 409 の画面は、「先に更新された」ことと、最新のタイトルを知らせ、入力した内容を残したまま、最新の `version` を隠し項目に入れます。B が、それを読んだうえで「保存する」を押したなら、上書きは B の意思なので、良い作りと言えます。
- ただし、知らせるのはタイトルだけなので、A が**本文**を変えていた場合、B は、何が変わったのかを、画面からは知れません。⑤では、A は本文を「牛乳」にしていましたが、B の 409 の画面には出ていません。本文も見せる・違う所を並べて見せる、などが考えられます。
- 何も知らせずに上書きする（version を比べない）作りと比べると、少なくとも「気づかずに消してしまう」ことは防げています（UNIT 05）。

**問3の解答:**

- 「21:08:30」は UTC（世界の基準の時刻）です。同じときの PC の時計（日本時間）は 06:08:31 で、9 時間ずれています。DB の `datetime('now')` は UTC を返し、RustNote は、その文字列をそのまま表示しているからです（`Note` の `created_at` のコメントにも「UTC」とあります）。
- 日本の利用者が見ると、夜の 9 時に書いたように見えてしまいます。表示するときに、利用者の時間帯に直す必要があります。日本の利用者だけなら 9 時間足す、いろいろな国の利用者がいるなら、ブラウザの時間帯で表示する、などの方法があります。

### まとめ

- `notes.rs` は、一覧（`?q=`・`?page=`。数でなければ 400）と作成（PRG の 303、だめなら `<details>` を開いて 400）。一覧の本体は、DB を検索する async の部品 `notes_index`。
- `notes/id.rs` は、`module_param!` で `{id}` を `NoteId` にし（読むのは `path_param::<Id>`。`NoteId` では E0277）、数字でない・0・無い・他人のノートを、どれも 404 にする。
- `edit.rs` は 404 → 400 → 409 の順に見て、409 では最新のタイトルと `version` を知らせ、もう一度押せば上書き。`toggle.rs`・`delete.rs` は `Result<()>` で、必ず 303。
- `href!` に `{id}` の値を渡し忘れても、コンパイルは通り、ノートがあるときだけ表示でパニックして 500 になる。アクセスログにも残らず、22 件のノートを作る HTTP のテストが止めた。
- 改善点: 最後より大きいページで「まだノートがありません」、日時が UTC のまま、削除の確かめが無い、NUL のタイトルで 500。

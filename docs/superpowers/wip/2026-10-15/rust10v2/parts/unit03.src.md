## UNIT 03: 詳細・URL の ID・見つからないとき

### この単元のゴール

- 詳細の画面（`GET /notes/{id}`）が、ログインの確かめ → URL の ID を `NoteId` に変える → 持ち主の条件付きの SQL で探す、の3つの関門を順に通ることを説明できる
- 無いノート・他人のノート・おかしな ID（`abc`・`0`・大きすぎる数）が、**どれも同じ 404** になることを、テストで確かめられる
- 他人のノートを 403 にすると、ID を順に試すだけで**ノートの数が分かってしまう**ことを、実際に確かめて説明できる
- `module_param!` が作る `Id` 型と、`href!(edit::edit_page, Id(id))` の書き方、`NoteId` をそのまま渡したときの `E0277` を読める

### たとえ話でつかむ

ホテルの部屋の鍵を考えます。

- フロントで「305 号室の鍵をください」と頼むと、フロントの人は、まず**泊まっている人か**を確かめます。泊まっていなければ、鍵の話の前に、受付へ案内します。
- 次に、部屋の番号が**番号として正しいか**を見ます。「あいう号室」や「0 号室」は、探すまでもなく「その部屋はありません」です。
- 最後に、台帳で「**この人が泊まっている** 305 号室」を探します。305 号室がほかの人の部屋なら、「その部屋はありません」と答えます。「ほかの方のお部屋です」とは言いません。言ってしまうと、305 号室に誰かが泊まっていることが、ばれるからです。

| ホテルのフロント | RustNote の詳細（GET /notes/{id}） |
|---|---|
| 泊まっている人か | `require_user`（ログインしていなければ 303 でログイン画面へ） |
| 部屋の番号として正しいか | `module_param!` が `{id}` を `NoteId` に変える（数字でない・0 以下なら 404） |
| 「この人の」305 号室を台帳で探す | `WHERE id = ?1 AND user_id = ?2`（持ち主の条件付きの SQL） |
| ほかの人の部屋でも「ありません」 | 他人のノートも 404（403 にしない） |

> **用語:** パスの引数（URL の `/notes/{id}` の `{id}` のように、URL の一部を値として受け取るもの。Topcoat では `module_param!` で型を決める。第6巻 UNIT 04）

<!-- -->

> **用語:** 404 Not Found（「その URL のものはありません」という返事。RustNote では、無いノートと他人のノートの両方に使う）

<!-- -->

> **用語:** 403 Forbidden（「あるけれど、あなたには見せられません」という返事。「ある」ことは伝わってしまう）

### 図で理解する

`GET /notes/{id}` が、詳細の画面にたどり着くまでの3つの関門です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-10-0301" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="40" width="120" height="60" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="70" y="66" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">GET</text>
<text x="70" y="86" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">/notes/{id}</text>
<rect x="160" y="40" width="160" height="60" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="240" y="64" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">① require_user</text>
<text x="240" y="84" font-size="11" fill="#475569" text-anchor="middle">ログインしているか</text>
<rect x="350" y="40" width="160" height="60" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="430" y="64" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">② path_param::&lt;Id&gt;</text>
<text x="430" y="84" font-size="11" fill="#475569" text-anchor="middle">{id} を NoteId に変える</text>
<rect x="540" y="40" width="170" height="60" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="625" y="64" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">③ find_own_note</text>
<text x="625" y="84" font-size="11" fill="#475569" text-anchor="middle">id と持ち主の両方で探す</text>
<line x1="130" y1="70" x2="156" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0301)"/>
<line x1="320" y1="70" x2="346" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0301)"/>
<line x1="510" y1="70" x2="536" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-10-0301)"/>
<line x1="240" y1="100" x2="240" y2="146" stroke="#f97316" stroke-width="2" marker-end="url(#ar-10-0301)"/>
<line x1="430" y1="100" x2="430" y2="146" stroke="#f97316" stroke-width="2" marker-end="url(#ar-10-0301)"/>
<line x1="600" y1="100" x2="600" y2="146" stroke="#f97316" stroke-width="2" marker-end="url(#ar-10-0301)"/>
<rect x="160" y="150" width="160" height="70" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="240" y="176" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">303 → /login</text>
<text x="240" y="198" font-size="11" fill="#475569" text-anchor="middle">ログインしていない</text>
<rect x="350" y="150" width="160" height="70" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="430" y="176" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">404</text>
<text x="430" y="196" font-size="11" fill="#475569" text-anchor="middle">abc・0・-1・大きすぎる数</text>
<rect x="540" y="150" width="125" height="70" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="602" y="174" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">404</text>
<text x="602" y="192" font-size="11" fill="#475569" text-anchor="middle">無いノート</text>
<text x="602" y="208" font-size="11" fill="#475569" text-anchor="middle">他人のノート</text>
<line x1="692" y1="100" x2="692" y2="236" stroke="#10b981" stroke-width="2" marker-end="url(#ar-10-0301)"/>
<rect x="590" y="240" width="120" height="50" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="650" y="270" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">200 詳細の画面</text>
<text x="350" y="262" font-size="11" fill="#475569" text-anchor="middle">②と③の 404 は、同じ画面（中身まで同じ）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: 詳細の画面までの3つの関門。ログインしていなければ 303、ID の形が正しくなければ 404、自分のノートが見つからなければ 404。②と③の 404 は見分けがつかない</div>
</div>

他人のノートを 403 にしたときと、404 にしたときに、bob に伝わることの違いです。alice のノートが3件（id は 1・2・3）あるとします。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="360" y="22" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">bob が /notes/1 から /notes/5 を順に開く</text>
<text x="40" y="70" font-size="12" font-weight="700" fill="#b91c1c">403 にした</text>
<rect x="140" y="46" width="84" height="40" rx="6" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="182" y="72" font-size="12" fill="#b91c1c" text-anchor="middle">1: 403</text>
<rect x="234" y="46" width="84" height="40" rx="6" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="276" y="72" font-size="12" fill="#b91c1c" text-anchor="middle">2: 403</text>
<rect x="328" y="46" width="84" height="40" rx="6" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="370" y="72" font-size="12" fill="#b91c1c" text-anchor="middle">3: 403</text>
<rect x="422" y="46" width="84" height="40" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="464" y="72" font-size="12" fill="#475569" text-anchor="middle">4: 404</text>
<rect x="516" y="46" width="84" height="40" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="558" y="72" font-size="12" fill="#475569" text-anchor="middle">5: 404</text>
<text x="370" y="108" font-size="11" fill="#b91c1c" text-anchor="middle">→ 「1〜3 は誰かのノートがある。4 から先は無い」と分かる</text>
<text x="40" y="160" font-size="12" font-weight="700" fill="#047857">404 にした</text>
<rect x="140" y="136" width="84" height="40" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="182" y="162" font-size="12" fill="#475569" text-anchor="middle">1: 404</text>
<rect x="234" y="136" width="84" height="40" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="276" y="162" font-size="12" fill="#475569" text-anchor="middle">2: 404</text>
<rect x="328" y="136" width="84" height="40" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="370" y="162" font-size="12" fill="#475569" text-anchor="middle">3: 404</text>
<rect x="422" y="136" width="84" height="40" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="464" y="162" font-size="12" fill="#475569" text-anchor="middle">4: 404</text>
<rect x="516" y="136" width="84" height="40" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="558" y="162" font-size="12" fill="#475569" text-anchor="middle">5: 404</text>
<text x="370" y="198" font-size="11" fill="#047857" text-anchor="middle">→ どれも同じ答え。ほかの人のノートがあるかどうかは、何も分からない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図10-?: 403 と 404 で、bob に伝わることの違い。403 は「ある」ことを知らせてしまうので、ID を順に試すだけで、ほかの人のノートの数が分かる</div>
</div>

### まず動かす最小コード

#### 例3-1: 自分・他人・無い・おかしな ID を、表にして確かめる

詳細の画面は、`src/routes/notes/id.rs` です。関門の部分だけを抜き出します。

```rust
// URL の {id} を NoteId に変換する。`Id` という型ができ、path_param::<Id>(cx) で読める。
// 変換に失敗したら NotFoundError（404）になる。
module_param!(pub id: NoteId, error = not_found);

/// 自分のノートを探す。無い・他人のノートなら AppError::NotFound（404）
pub async fn find_own_note(cx: &Cx, owner: UserId, id: NoteId) -> Result<Note, AppError> {
    app_state(cx)
        .notes
        .find(owner, id)
        .await?
        .ok_or(AppError::NotFound)
}

#[page]
pub async fn show(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let id = *path_param::<Id>(cx)?;
    let note = find_own_note(cx, user.id, id).await?;
    // …（画面を組み立てる。例3-2）
}
```

`notes.find` の SQL は、`src/repository/notes.rs` にあります。

```rust
let sql = format!("SELECT {NOTE_COLUMNS} FROM notes WHERE id = ?1 AND user_id = ?2");
```

> **▼ このコードがやること（先に日本語で）:** alice と bob を登録し、alice のノートを1件作ります。次に、alice・bob・ログインしていない人（guest）が、いろいろな URL を開いたときの状態コードを、表にして表示します。最後に、bob から見た「他人のノート（`/notes/1`）」と「無いノート（`/notes/999`）」の答えを、状態コード・本文の長さ・本文がまったく同じか、で比べます。

```rust
@@FILE:rustnote/tests/u03_detail.rs@@
```

```powershell
cargo test --test u03_detail -- --nocapture --test-threads=1
```

**▼ 実行結果**（`GET /notes/1 -> …` のような行は、RustNote のアクセスログです）

```text
@@OUTGREP:u03_detail:^(?!   Compiling)@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 200 になるのは、alice が自分のノートを開いたときだけ

- 表の中で `200 OK` は1行だけです。bob の `/notes/1`（alice のノート）も、誰の `/notes/999`（無いノート）も、`404 Not Found` でした。
- `abc`・`0`・`-1`・`9223372036854775808`（`i64` に入らない大きな数）も 404 です。`NoteId` の `FromStr`（`src/domain.rs`）が、数字でなければ `NotANumber`、0 以下なら `NotPositive` を返し、`module_param!` の `error = not_found` が、それを 404 にします。DB には問い合わせていません。
- ログインしていない guest は、`303 See Other → /login` でした。①の `require_user` が、②や③より先に動くからです。

##### 解説2: bob から見ると、2つの 404 は1バイトも違わない

- 最後の比べでは、状態コードも、本文の長さ（690 バイト）も同じで、`本文がまったく同じ: true` でした。bob には、`/notes/1` に alice のノートが**ある**のか、`/notes/999` のように**無い**のか、区別がつきません。
- 区別がつかないのは、③で「id が 1 **で、しかも持ち主が bob の**ノート」を探しているからです。SQL の答えは、どちらも「0 行」です。Rust のコードは、「無い」と「他人」を、そもそも知りません。
- 比べるときは、どちらも bob で開いています。画面の上の部分（ヘッダー）には、ログイン中の人に合わせた表示があるので、alice と bob の画面を比べても、意味がないからです。

### もう一歩: 実務に近い形

#### 例3-2: 詳細の画面の中身と、次の操作への入り口

> **▼ このコードがやること（先に日本語で）:** タイトルに `<b>牛乳</b>を買う`、本文に改行と `&` を入れたノートを作り、詳細の画面の HTML から、ノートの部分（`<article>`）と、次の操作への入り口（編集のリンク・完了と削除のフォーム・一覧へ戻るリンク）を抜き出して表示します。

```rust
@@FILE:rustnote/tests/u03_page.rs@@
```

```powershell
cargo test --test u03_page -- --nocapture
```

```text
@@OUTGREP:u03_page:^(?!   Compiling|POST |GET |test |running|\s*$|    Finished|     Running|test result|exit=)@@
```

（cargo の行とアクセスログの行は省いています）

- タイトルの `<b>` は `&lt;b&gt;` に、本文の `&` は `&amp;` に置き換わりました。`view!` の `(note.title)` は、値を**文字として**入れるので、HTML のタグとしては働きません（第6巻 UNIT 05）。
- 本文の改行は、HTML の中でも改行のまま入っています。HTML は、ふつうは改行を空白1つとして表示しますが、`assets/style.css` の `.note-body { white-space: pre-wrap; }` が、改行をそのまま見せています。`<br>` に置き換える必要はありません。
- 入り口の URL（`/notes/1/edit`・`/notes/1/toggle`・`/notes/1/delete`）は、どれも `href!` で作っています。手で `format!("/notes/{}/edit", id)` と書くと、URL の形を変えたときに、直し忘れが起きます。`href!` なら、行き先のページ（`edit::edit_page`）が無くなったり、引数の型が違ったりすると、コンパイルが止まります（エラー3）。
- 完了と削除は `<form method="post">` で、リンク（GET）ではありません。GET はリンクを開いただけで動いてしまうので、DB を変える操作には使いません（UNIT 05・06）。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: SQL から持ち主の条件を外した

`notes.find` の SQL を、`WHERE id = ?1` だけにした場合です（練習用のコピーの `src/repository/notes.rs` を書き換え、確かめたら元に戻します）。

```rust
let sql = format!("SELECT {NOTE_COLUMNS} FROM notes WHERE id = ?1");
```

例3-1のテストを、そのまま動かします（表と比べの行だけを抜き出しています）。

```text
@@OUTGREP:u03_err_owner:^(alice|bob|guest) |^状態|^本文@@
```

- **何が起きたか:** コンパイルも、テストの実行も、エラーなく通りました。それなのに、**bob の `/notes/1` が `200 OK`** になり、alice のノートの画面（1066 バイト）が見えています。
- **気づきにくい理由:** SQL から `?2` を消しても、`libsql::params![id.get(), owner.get()]` は2つの値を渡したままです。libsql は、使われない値があっても、エラーにしませんでした。型の検査も通ります。だから、この間違いは、**他人として開いてみるテスト**でしか見つかりません。
- **直し方:** 持ち主の条件は、SQL の `WHERE` に書きます。「id で探してから、Rust で `note.user_id == owner` を比べる」形にすると、比べるのを1か所でも忘れたら、同じ事故になります（第8巻 UNIT 01・07（準備中））。

#### エラー2（エラーが出ない間違い）: 他人のノートを 403 にした

「他人のノートなら『権限がありません』と言うほうが親切だ」と考えて、`find_own_note` を、見つからなかったときに、もう一度 id だけで探し、あれば 403 を返すようにした場合です（練習用のコピーの `src/error.rs` に `Forbidden`（403）を足し、`src/routes/notes/id.rs` を書き換えています）。

```rust
pub async fn find_own_note(cx: &Cx, owner: UserId, id: NoteId) -> Result<Note, AppError> {
    if let Some(note) = app_state(cx).notes.find(owner, id).await? {
        return Ok(note);
    }
    // わざと間違えた版: ほかの人のノートなら 403、本当に無いときだけ 404
    let conn = app_state(cx).db.connect().await.map_err(AppError::internal)?;
    let mut rows = conn
        .query("SELECT 1 FROM notes WHERE id = ?1", libsql::params![id.get()])
        .await
        .map_err(AppError::internal)?;
    if rows.next().await.map_err(AppError::internal)?.is_some() {
        Err(AppError::Forbidden)
    } else {
        Err(AppError::NotFound)
    }
}
```

alice のノートを3件作り、bob が `/notes/1` から `/notes/5` を順に開くテスト（`tests/u03_count.rs`）です。

```rust
@@FILE:rustnote/tests/u03_count.rs@@
```

まず、RustNote のコード（404 の版）です。

```text
@@OUTGREP:u03_count:^(alice|bob) @@
```

次に、403 にした版です。

```text
@@OUTGREP:u03_err_403:^(alice|bob) @@
```

- **何が起きたか:** 403 の版では、`1`・`2`・`3` が `403 Forbidden`、`4`・`5` が `404 Not Found` でした。bob は、ノートの中身は見られませんが、「**id が 1〜3 のノートが、誰かのものとしてある**」「4 から先は、まだ無い」ことを知りました。
- **困ること:** ID を順に試すだけで、ほかの人のノートの数や、ノートが作られる速さ（毎日どれだけ増えるか）が分かります。ノートの題が分からなくても、知られたくない情報です。RustNote の 404 の版では、5つとも同じ答えなので、何も分かりません。
- **学べること:** 「見せない」ときは、「無い」と同じ答えにします。同じ考えで、ログイン画面も「メールアドレスまたはパスワードが違います」と、どちらが違うかを言いません（第8巻 UNIT 04（準備中））。

#### エラー3: href! に NoteId をそのまま渡した（E0277）

編集のリンクを `href!(edit::edit_page, id)` と書いた場合です（`Id(id)` の `Id(…)` を忘れた）。

```text
@@OUTHEAD:u03_err_href:20@@
...
@@OUTTAIL:u03_err_href:3@@
```

（同じ形のエラーが、`view!` の所にもう1つ出ます。間は省いています）

- **何が起きたか:** 「`NoteId` は `HrefParam` を実装していません」というエラーです。`href!` に渡せる値は、`HrefParam` という印の付いた型だけです。
- **読み方:** 2つ目の `help:` が答えです。「`HrefParam` を実装しているほかの型」として、27 行目の `module_param!(pub id: NoteId, …)` が作った **`id::Id`** を挙げています。`module_param!` は、`NoteId` を包む `Id` という型を作り、`{id}` の場所に入る値は、この `Id` で渡す決まりにしています。
- **直し方:** `href!(edit::edit_page, Id(id))` と、`Id(…)` で包みます。こうしておくと、別の URL の部品（たとえば、将来ユーザーの ID を URL に入れる `{user_id}`）と取り違えたときにも、コンパイルが止まります。

### RustNote ではここで使う

- 詳細の画面は `src/routes/notes/id.rs` の `show` です。`find_own_note` は、編集の画面（`src/routes/notes/id/edit.rs`）でも、表示・保存・競合のときの読み直しの3か所で使っています（UNIT 04）。
- 完了の切り替えと削除は、`find_own_note` を通らず、`UPDATE … WHERE id = ?1 AND user_id = ?2`・`DELETE … WHERE id = ?1 AND user_id = ?2` で、1つの SQL の中に持ち主の条件を入れています（問3・UNIT 05・06）。
- `src/error.rs` の先頭の表に「`NotFound` … 存在しないノート、**他人のノート**（あること自体を隠す）」と書いてあります。決まりを1か所に書いておくと、新しい画面を足すときにも迷いません。

### 練習問題

**問1:** ログインしていない人が `/notes/abc`（おかしな ID）と `/notes/999`（無いノート）を開くと、何が返るでしょうか。404 にならないのは、なぜでしょうか。

**問2:** alice が、同じノートを `/notes/1`・`/notes/01`・`/notes/+1`・`/notes/%31`・`/notes/1.0`・`/notes/%201`（`%20` は空白）で開くと、それぞれ何が返るでしょうか。予想してから、確かめてください。

**問3:** 完了の切り替え（`POST /notes/1/toggle`）と削除（`POST /notes/1/delete`）は、`find_own_note` を通りません。bob がこの2つを alice のノートに送ると、どうなるでしょうか。alice のノートは無事でしょうか。

### 解答と解説

```rust
@@FILE:rustnote/tests/u03_q.rs@@
```

```powershell
cargo test --test u03_q -- --nocapture --test-threads=1
```

```text
@@OUTGREP:u03_q:^問@@
```

（問題の答えの行だけを抜き出しています）

**問1の解答:**

- どちらも `303 See Other → /login` です。①の `require_user` が、②の ID の変換より先に動くので、ID がおかしいかどうかを調べる前に、ログイン画面へ送られます。
- この順番にも意味があります。ログインしていない人には、「その ID の形が正しいか」さえ教えません。どの URL でも同じ 303 なので、外から試しても、何も分かりません。

**問2の解答:**

- `/notes/01`・`/notes/+1`・`/notes/%31` も、`/notes/1` と同じノートが `200 OK` で開きました。`NoteId` の `FromStr` は、Rust の `i64` の `parse` を使っていて、`parse` は先頭の `0` や `+` を受け付けるからです。`%31` は URL の中での `1` の書き方で、Topcoat が `1` に戻してから渡しています。
- `1.0`（小数）と `%201`（先頭に空白がある `" 1"`）は 404 です。`parse` は、小数点や空白を受け付けません。
- どの書き方で開いても、画面の中の入り口は `/notes/1/edit` でした。`href!` は `NoteId` の `Display`（`write!(f, "{}", self.0)`）で URL を作るので、いつも同じ形にそろいます。同じノートに URL が何通りもあるのは、困ることではありませんが、気になるなら、`FromStr` で「数字だけ・先頭が 0 でない」を確かめる決まりを足します。

**問3の解答:**

- どちらも `404 Not Found` でした。そのあと alice が開くと `200 OK` で、完了の印も付いていません（`false`）。alice のノートは、変わらずに残っています。
- `toggle_done` と `delete` の SQL が、どちらも `WHERE id = ?1 AND user_id = ?2` になっているからです。bob の ID では、変える行も消す行も、0 行です。0 行のとき、リポジトリは `RepoError::NotFound` を返し、それが `AppError::NotFound`（404）になります。
- 「読んで、持ち主を確かめてから、変える」形にすると、確かめと変更の間に時間ができます。1つの SQL の中に条件を入れておけば、その間がありません（UNIT 05 でくわしく扱います）。

### まとめ

- 詳細の画面は、①ログイン（303）→ ②ID の形（`module_param!` と `NoteId` の `FromStr`。404）→ ③持ち主の条件付きの SQL（404）の順に通る。200 になるのは、自分のノートのときだけ。
- 無いノートと他人のノートは、同じ 404 で、本文まで1バイトも違わない。403 にすると、ID を順に試すだけで、ほかの人のノートの数が分かってしまう。持ち主の条件を SQL から外しても、コンパイルもテストも通るので、他人として開くテストで確かめる。
- URL は `href!` で作り、`{id}` には `module_param!` が作った `Id(…)` で渡す。`NoteId` をそのまま渡すと `E0277` で止まる。

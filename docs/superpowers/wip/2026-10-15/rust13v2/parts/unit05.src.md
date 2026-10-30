## UNIT 05: リポジトリ

### この単元のゴール

- `src/repository.rs` が「約束（trait）」と「libSQL の実装」を組にして置き、`AppState` が `Arc<dyn … + Send + Sync>` で持つ理由と、`#[async_trait]` が要る理由を説明できる
- `src/repository/notes.rs` のどの SQL にも、持ち主の条件（`user_id = ?`）が入っていることと、それを消すと何が起きるか（どのテストが止めるか）を、本物の実行で確かめられる
- 楽観的ロック（`version`）の `update` が、0 行のときに `find` で「無い」と「先に更新された」を見分ける流れを説明できる
- LIKE の `%` と `_` を escape する理由を、escape しない検索との比較で示せる
- `users.rs`・`sessions.rs` が、パスワードのハッシュとセッションのトークンのハッシュを、どう保存し、どう探すかを言える

### たとえ話でつかむ

図書館の、本の出し入れの窓口を考えます。

- 利用者（ルート）は、**窓口**（trait）に「この本を出してください」と頼むだけで、書庫の中の棚の場所（SQL）は知りません。窓口の係が、だれ（libSQL の実装・テスト用の実装）でも、頼み方は同じです。
- 係は、本を出すとき、必ず**貸出カードの名前**（`user_id`）を確かめます。他人が借りている本を頼まれたら、「ありません」と答えます。「ありますが、他人のです」とは言いません。
- 本には、**版の番号**（`version`）が書いてあります。書き直しを頼むときは、「私が読んだのは第2版です」と伝えます。もう第3版になっていたら、係は書き直さず、「先に別の人が直しました」と答えます（楽観的ロック）。
- 書き直せなかったとき、係は、**もう一度、棚を見て**、「本が無い」のか「版が違う」のかを確かめてから答えます。
- 「`100%` という言葉を含む本」を探すとき、`%` を「何でもよい」の印と取り違えないよう、**ただの文字だと印を付けて**から探します。

| 図書館 | RustNote（`src/repository.rs`・`src/repository/`） |
|---|---|
| 窓口（頼み方） | trait（`NoteRepository`・`UserRepository`・`SessionRepository`） |
| 窓口の係 | `LibsqlNoteRepository` など（libSQL で実装したもの） |
| 係を選んで置いておく | `AppState` の `Arc<dyn NoteRepository + Send + Sync>` |
| 貸出カードの名前を確かめる | どの SQL にも `user_id = ?`（持ち主の条件） |
| 版の番号 | `notes.version`（更新のたびに 1 増える） |
| 版が違えば書き直さない | `UPDATE … WHERE … AND version = ?` |
| もう一度、棚を見る | 0 行のときの `self.find(owner, id)` |
| ただの文字だと印を付ける | `escape_like` と `ESCAPE '\'` |

> **用語:** リポジトリ（repository：データのしまい方・取り出し方を引き受ける部品。ほかの部品は、SQL を知らずに「保存して」「探して」と頼む）

<!-- -->

> **用語:** 楽観的ロック（らっかんてきロック：「同じデータを同時に直す人は、めったにいない」と考えて、先に鍵をかけず、保存するときに版の番号を比べて、食い違いを見つける方法）

### 図で理解する

ルートから DB までの層と、エラーが戻る道です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-0501" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="30" width="150" height="56" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="85" y="54" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">ルート</text>
<text x="85" y="74" font-size="11" fill="#475569" text-anchor="middle">routes/notes.rs など</text>
<rect x="190" y="30" width="160" height="56" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="270" y="54" font-size="13" font-weight="700" fill="#1d4ed8" text-anchor="middle">AppState</text>
<text x="270" y="74" font-size="11" fill="#1d4ed8" text-anchor="middle">Arc&lt;dyn NoteRepository…&gt;</text>
<rect x="380" y="30" width="170" height="56" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="465" y="54" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">LibsqlNoteRepository</text>
<text x="465" y="74" font-size="11" fill="#047857" text-anchor="middle">src/repository/notes.rs</text>
<rect x="580" y="30" width="130" height="56" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="645" y="54" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">libSQL</text>
<text x="645" y="74" font-size="11" fill="#c2410c" text-anchor="middle">notes の表</text>
<line x1="160" y1="50" x2="186" y2="50" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0501)"/>
<line x1="350" y1="50" x2="376" y2="50" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0501)"/>
<line x1="550" y1="50" x2="576" y2="50" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0501)"/>
<line x1="580" y1="70" x2="554" y2="70" stroke="#b91c1c" stroke-width="2" stroke-dasharray="4 3" marker-end="url(#ar-13-0501)"/>
<line x1="380" y1="70" x2="354" y2="70" stroke="#b91c1c" stroke-width="2" stroke-dasharray="4 3" marker-end="url(#ar-13-0501)"/>
<line x1="190" y1="70" x2="164" y2="70" stroke="#b91c1c" stroke-width="2" stroke-dasharray="4 3" marker-end="url(#ar-13-0501)"/>
<text x="85" y="110" font-size="11" fill="#0f172a" text-anchor="middle">trait のメソッドを</text>
<text x="85" y="126" font-size="11" fill="#0f172a" text-anchor="middle">呼ぶだけ（SQL は知らない）</text>
<text x="270" y="110" font-size="11" fill="#0f172a" text-anchor="middle">どの実装を使うかを</text>
<text x="270" y="126" font-size="11" fill="#0f172a" text-anchor="middle">起動のときに決めて持つ</text>
<text x="465" y="110" font-size="11" fill="#0f172a" text-anchor="middle">SQL を書き、行を型に変える</text>
<text x="465" y="126" font-size="11" fill="#b91c1c" text-anchor="middle">WHERE に必ず user_id = ?</text>
<text x="645" y="110" font-size="11" fill="#0f172a" text-anchor="middle">UNIQUE・CHECK・</text>
<text x="645" y="126" font-size="11" fill="#0f172a" text-anchor="middle">外部キーで最後に止める</text>
<rect x="10" y="150" width="700" height="62" rx="6" fill="#fef2f2" stroke="#ef4444" stroke-width="1.5"/>
<text x="24" y="172" font-size="12" fill="#b91c1c">点線: 失敗の戻り道。libsql::Error → RepoError（From。UNIQUE なら UniqueViolation）→ AppError（UNIT 04）</text>
<text x="24" y="196" font-size="12" fill="#b91c1c">0 行の更新・削除は、リポジトリが NotFound か VersionMismatch に分けて返す</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: リポジトリの層。ルートは trait だけを知り、AppState が実装を持つ。持ち主の条件は、いちばん DB に近い SQL で必ず付ける</div>
</div>

`update`（楽観的ロック）が、0 行のときに理由を見分ける流れです（例5-4の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-0502" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="20" width="250" height="70" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="135" y="44" font-size="12" font-weight="700" fill="#1d4ed8" text-anchor="middle">UPDATE notes SET …</text>
<text x="135" y="62" font-size="11" fill="#1d4ed8" text-anchor="middle">WHERE id AND user_id AND version = 期待</text>
<text x="135" y="80" font-size="11" fill="#1d4ed8" text-anchor="middle">RETURNING …</text>
<rect x="320" y="20" width="390" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="515" y="42" font-size="12" fill="#047857" text-anchor="middle">1 行返った → Ok(Note)（version が 1 増える）</text>
<rect x="320" y="76" width="170" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="405" y="98" font-size="12" fill="#0f172a" text-anchor="middle">0 行 → find(owner, id)</text>
<rect x="520" y="76" width="190" height="56" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="615" y="98" font-size="12" fill="#c2410c" text-anchor="middle">Some → VersionMismatch</text>
<text x="615" y="118" font-size="11" fill="#c2410c" text-anchor="middle">→ Conflict（409）</text>
<rect x="520" y="146" width="190" height="56" rx="6" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="615" y="168" font-size="12" fill="#b91c1c" text-anchor="middle">None → NotFound</text>
<text x="615" y="188" font-size="11" fill="#b91c1c" text-anchor="middle">→ NotFound（404）</text>
<line x1="260" y1="40" x2="316" y2="38" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0502)"/>
<line x1="260" y1="74" x2="316" y2="92" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0502)"/>
<line x1="490" y1="96" x2="516" y2="102" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0502)"/>
<line x1="470" y1="110" x2="516" y2="170" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0502)"/>
<text x="10" y="130" font-size="11" fill="#475569">例: 古い version 1 で更新 → VersionMismatch</text>
<text x="10" y="150" font-size="11" fill="#475569">例: 完了の切り替え（version が上がる）のあと → VersionMismatch</text>
<text x="10" y="170" font-size="11" fill="#475569">例: bob が alice のノートを更新 → NotFound</text>
<text x="10" y="196" font-size="11" fill="#b91c1c">find も user_id で絞るので、他人のノートは「無い」と同じになる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: 楽観的ロックの update。0 行のときだけ、もう一度 find して、「先に更新された」と「無い（他人のも含む）」を分ける</div>
</div>

### まず動かす最小コード

#### 例5-1: src/repository.rs の全文

> **▼ このファイルがやること（先に日本語で）:** 3つのリポジトリ（`notes.rs`・`users.rs`・`sessions.rs`）を部品として読み込み、その trait と実装を `pub use` で外に見せます。時刻を「1970 年からの秒数」に変える `unix_seconds` と、DB から読んだ値の変換の失敗を `InvalidData` にする `invalid_data` もあります。

```rust
@@FILE:rustnote/src/repository.rs@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 約束（trait）と実装を組にする

- 3つのファイルは `mod notes;` と、`pub` を付けずに読み込みます。外に見せるのは、`pub use` で選んだ trait・実装・`UserRecord` だけです。外からは `rustnote::repository::NoteRepository` と、1段浅い名前で使えます。
- ルートは trait（`NoteRepository`）のメソッドだけを呼びます。どの実装を使うかは、`src/state.rs` の `AppState::new` が決め、`Arc<dyn NoteRepository + Send + Sync>` で持ちます（UNIT 06）。`Send + Sync` は「ほかのスレッドへ渡しても、同時に使っても安全」という印です。Topcoat の `app_context` は、リクエストを複数のスレッドで同時に処理するので、登録する値に `Send + Sync` を求めます。
- trait のメソッドは、DB を待つので `async fn` です。ところが、`async fn` を持つ trait は、そのままでは `dyn` で使えません（エラー1）。`#[async_trait]` は、`async fn` を「`Box` に入れた Future を返す普通の関数」に書き換えて、`dyn` で使える形にします（第9巻 UNIT 04）。

##### 解説2: 小さな2つの関数

- `unix_seconds` は、`SystemTime` を「1970-01-01 からの秒数」の `i64` にします。`sessions.expires_at` の列の形です。1970 年より前なら 0、`i64` に入らないほど先なら `i64::MAX` にして、パニックしません。
- `invalid_data` は、DB から読んだ値が型の検査に通らなかったとき（たとえば `users.email` が `Email::parse` に通らない）に、どの列かと理由を付けて `RepoError::InvalidData` にします。DB に変な値が入っていても、パニックせずに 500 で返し、ログに理由を残せます。

#### 例5-2: src/repository/notes.rs の全文

> **▼ このファイルがやること（先に日本語で）:** ノートの追加・1件の検索・一覧の検索・更新・完了の切り替え・削除の6つの約束（`NoteRepository`）と、それを libSQL で実装したもの（`LibsqlNoteRepository`）です。どの SQL にも、持ち主の条件（`user_id = ?`）を入れます。更新は、`version` を比べる楽観的ロックです。最後に、LIKE の特別な文字を escape する関数と、そのテストがあります。

```rust
@@FILE:rustnote/src/repository/notes.rs@@
```

##### 解説3: どの SQL にも、持ち主の条件

- `find`・`update`・`toggle_done`・`delete` の `WHERE` には `user_id = ?`、`search` の条件には `user_id = ?1` が入っています。`insert` は、持ち主を `user_id` の列に書きます。
- 持ち主の条件を SQL に入れておくと、他人のノートは「行が無い」のと同じになります。ルートは、「無い」と「他人の」を区別できないので、どちらも 404 を返し、他人のノートがあること自体を知らせません（UNIT 04 の `NotFound`）。条件を1つ消すと何が起きるかは、問2で確かめます。

##### 解説4: RETURNING と、列の順番

- `insert`・`update`・`toggle_done` は、SQL の終わりに `RETURNING {NOTE_COLUMNS}` を付けています。書き込んだ行（DB が決めた `id`・`created_at`・新しい `version` など）を、その場で受け取れるので、もう一度 `SELECT` しなくて済みます。
- 読む列の並びは、定数の `NOTE_COLUMNS` 1か所に書き、`row_to_note` は、その順番どおりに `row.get(0)`・`row.get(1)`…と読みます。`id` と `user_id` は、`NoteId::new`・`UserId::new` で検査し直し、通らなければ `InvalidData` にします。`title` と `body` は、DB の `CHECK` が守っているので、`String` のまま読みます。

##### 解説5: 一覧の検索は2回の問い合わせ

- 1回目は `COUNT(*)` で、条件に合う総数を数えます（ページ数の計算に使う）。2回目は、`ORDER BY updated_at DESC, id DESC` で新しく更新した順に並べ、`LIMIT 20 OFFSET …` で、そのページの分だけを読みます。
- `updated_at` は、`datetime('now')` の秒までの時刻なので、同じ秒に更新したノートは、`id` の大きい順になります。
- キーワードが無いときは、パターンを `%`（何にでも一致）にして、同じ SQL を使います。キーワードがあれば `%キーワード%` にしますが、その前に `escape_like` で、`%`・`_`・`\` に `\` を付けます。SQL の `ESCAPE '\'` は「`\` の次の文字は、ただの文字」という意味です（問1）。

##### 解説6: 楽観的ロックの update

- `UPDATE … WHERE id = ?3 AND user_id = ?4 AND version = ?5` は、版の番号が、編集の画面を開いたときと同じときだけ書き込み、`version = version + 1` で番号を上げます。
- 1 行も返らなかったときは、理由が2つあります。「ノートが無い（他人のノートも含む）」と「ほかの画面で先に更新された」です。`self.find(owner, id)` でもう一度探し、見つかれば `VersionMismatch`（409）、無ければ `NotFound`（404）にします（2つ目の図・問3）。
- `toggle_done` は、版の番号を比べずに切り替えますが、`version` は上げます。だから、編集の画面を開いたあとに、一覧で完了を切り替えると、編集の保存は `VersionMismatch` になります（例5-4）。
- `delete` は、`execute` が返す「消した行の数」が 0 なら `NotFound` にします。

#### 例5-3: src/repository/users.rs と src/repository/sessions.rs の全文

> **▼ このファイルがやること（先に日本語で）:** `users.rs` は、ユーザーの登録と、メールでの検索です。同じメールの登録は、DB の `UNIQUE` で `UniqueViolation` になります。`sessions.rs` は、ログイン中のセッションの記録・検索・削除・期限切れの掃除です。トークンそのものではなく、トークンの SHA-256（32 バイト）を保存します。

```rust
@@FILE:rustnote/src/repository/users.rs@@
```

```rust
@@FILE:rustnote/src/repository/sessions.rs@@
```

##### 解説7: パスワードのハッシュは、要る所でだけ

- `User`（`id` と `email`）には、パスワードのハッシュが入っていません。ハッシュ付きの `UserRecord` を返すのは、ログインの照合に使う `find_by_email` だけです。ふだん持ち回る `User` には、ハッシュが無いので、うっかり画面やログに出すこともありません。
- `row_to_user` は、`pub(super)`（親の `repository` の中だけで使える）で、`sessions.rs` の `find_user` からも使います。`users.email` も、`Email::parse` で検査し直します。
- `find_by_email` の `[email.as_str()]` のように、値が1つのときは、配列で SQL の値を渡せます。2つ以上なら `libsql::params![…]` を使います。

##### 解説8: セッションは、トークンのハッシュで探す

- Topcoat は、ログインのトークンを発行して、クッキーで運ぶだけです。どのトークンがだれのものかの記録は、アプリが `sessions` の表に持ちます（第8巻）。
- 保存するのは、トークンの SHA-256 の `TokenHash`（32 バイト）を `to_vec()` で `BLOB` にしたものです。DB の中身が漏れても、トークンそのものは分かりません。
- `find_user` は、`sessions` と `users` を `JOIN` し、`expires_at > 今` の行だけを見るので、期限切れのトークンでは、ユーザーが見つかりません。期限切れの行は、`delete_expired` が、ログインのたびにまとめて消します（UNIT 06 の `src/auth.rs`）。

### もう一歩: 実務に近い形

#### 例5-4: 3つのリポジトリを、本物の DB で動かす

> **▼ このコードがやること（先に日本語で）:** 一時フォルダに DB のファイルを作り、マイグレーションをかけ、本番と同じ `AppState::new` で3つのリポジトリを用意します。そして、ユーザーの登録（同じメールの2回目も）、ノートの追加と持ち主ごとの検索、楽観的ロックの更新、一覧の検索とページ送り、削除、セッションの期限を、順に試して表示します。

```rust
@@FILE:rustnote/examples/u05_repo.rs@@
```

```powershell
cargo run -q --example u05_repo
```

```text
@@OUT:u05_repo@@
```

- 同じメールの2回目は、`UniqueViolation` でした。中の文は、SQLite の `UNIQUE constraint failed: users.email` です（UNIT 04 の `From<libsql::Error>` が、この文で見分けます）。
- bob が alice のノートを探すと `None`、無い ID（999）も `None` で、**同じ答え**でした。更新も、bob は `NotFound` です。
- 古い `version 1` での更新と、完了の切り替え（version が 3 に上がる）のあとの `version 2` での更新は、どちらも `VersionMismatch` でした。
- 26 件のノートは、20 件と 6 件の2ページに分かれました。キーワード `100%` は「100%オフ」の1件だけ、`100` は「1000円」も含めて2件でした。bob の一覧は 0 件です。
- 期限切れのトークンではユーザーが見つからず、掃除で1件消え、ログアウト（`delete`）のあとは、期限内のトークンでも見つかりませんでした。

### よくあるエラーと読み方

#### エラー1: `#[async_trait]` を付けずに、`async fn` を持つ trait を `dyn` で使った（E0038）

```rust
@@FILE:errs_u05/e05_dyn.rs@@
```

```text
@@OUT:u05_err_dyn@@
```

- **何が起きたか:** 「trait `NoteCounter` は dyn で使えません」というエラーです。`...because method count is async`（`count` が `async` だから）と、理由が書いてあります。
- **読み方:** `dyn` で使うには、メソッドの表（vtable）を作れる必要があります。`async fn` は、実装ごとに大きさの違う Future を返すので、表に入れられません。
- **直し方:** RustNote のように、trait と実装の両方に `#[async_trait]` を付けます。実装が1つしか無く、差し替える予定も無いなら、`help:` のとおり、`dyn` を使わずに、その型を直接使う手もあります。

#### エラー2: `connect()` に `.await` を付け忘れた（E0277）

```rust
@@FILE:errs_u05/e05_await.rs@@
```

```text
@@OUT:u05_err_await@@
```

- **何が起きたか:** 「`?` は、`Try` を実装した値にしか使えません」というエラーです。`connect()` は `async fn` なので、呼んだだけでは、まだ実行されていない Future（`impl Future<Output = Result<…>>`）が返ります。
- **直し方:** `help:` のとおり、`?` の前に `.await` を付けます。リポジトリのメソッドも全部 `async fn` なので、呼ぶ所では `.await?` がひと組です。

#### エラー3: SQL の値に、`UserId` をそのまま渡した（E0599）

```rust
@@FILE:errs_u05/e05_params.rs@@
```

```text
@@OUTHEAD:u05_err_params:10@@
  …（満たされていない条件の一覧が続く）…
@@OUTGREP:u05_err_params:^error: could not@@
```

- **何が起きたか:** `libsql::params!` は、中の値を `into_value` で libSQL の値（`Value`）に変えます。`UserId` には、その変換（`Into<Value>`）がありません。
- **読み方:** 長いエラーですが、`doesn't satisfy UserId: Into<Value>` の1行が要点です。
- **直し方:** RustNote のように `owner.get()` で中の `i64` を取り出して渡します。DB に渡す所でだけ、検査済みの型から中身を出します。

### RustNote ではここで使う

- **共有の状態:** `src/state.rs` の `AppState::new` が、3つの libSQL の実装を作って、`Arc<dyn …>` で持ちます（UNIT 06）。
- **ルート:** ノートの一覧・作成・編集・切り替え・削除のルートが `state.notes` を（UNIT 08）、登録が `state.users.create` を、ログインの照合が `find_by_email` を、セッションの確認が `state.sessions` を使います（UNIT 06・07）。
- **テスト:** `escape_like` のユニットテストが1つと、`tests/repository.rs` の結合テストが8つあります（UNIT 10）。結合テストは、一時フォルダに本物の DB を作って試します。

```text
@@OUTGREP:u05_lib_tests:^(running|test |test result)@@
```

```text
@@OUTGREP:u05_repo_tests:^(running|test |test result)@@
```

### 練習問題

**問1:** `escape_like` を使わずに、キーワードをそのまま `%…%` に入れて LIKE で探すと、`100%` と `a_b` の検索は、どうなるでしょうか。

**問2:** `notes.rs` の `find` の SQL から、持ち主の条件（`AND user_id = ?2`）を消すと、何が起きるでしょうか。どのテストが止めてくれるでしょうか。

**問3:** `update` で 0 行だったとき、`find` で見分けずに、いつも `VersionMismatch` を返すようにすると、何が困るでしょうか。

### 解答と解説

**問1の解答:**

同じ4つのタイトルに対して、escape しないパターンと、escape したパターン（`ESCAPE '\'` 付き）で探しました。

```rust
@@FILE:rustnote/examples/u05_q1.rs@@
```

```text
@@OUT:u05_q1@@
```

- escape しないと、`100%` の `%` が「何文字でもよい」になり、「1000円」まで当たりました。`a_b` の `_` は「どれか1文字」になり、「acb の計算」まで当たりました。
- 利用者は「`100%` という文字」を探したいので、escape した結果が正しい答えです。RustNote の `search` は、escape してから探すので、例5-4のとおり、`100%` は1件だけでした。

**問2の解答:**

教材のコードのコピーで、`find` の SQL を `WHERE id = ?1` だけにして、例5-4とリポジトリのテストを動かしました。

```text
@@OUTGREP:u05_q2_run:^(alice が探す|bob が)@@
```

```text
@@OUTGREP:u05_q2_test:^(running|notes_are|thread|assertion|test result)@@
```

- bob が、alice のノートを**見つけられる**ようになりました。ノートの画面のルートは、`find` で自分のノートを探すので（UNIT 08 の `find_own_note`）、bob が `/notes/1` を開けば、alice のノートが表示されることになります。
- 更新の SQL には持ち主の条件が残っているので、bob の更新は書き込まれません。ただし、0 行のあとの `find` が見つけてしまうので、答えが `NotFound` から `VersionMismatch`（409）に変わりました。
- 結合テストの `notes_are_isolated_per_owner` が、78 行目（「bob からは見えない」）で止めました。持ち主の条件は、1つ消しただけでは、コンパイルも、ほかのテストも通ってしまいます。**「他人からは見えない」をテストに書いておく**ことが、この種のまちがいを止める、ほとんど唯一の方法です。

**問3の解答:**

教材のコードのコピーで、`update` の最後を `Err(RepoError::VersionMismatch)` だけにして、例5-4とリポジトリのテストを動かしました。

```text
@@OUTGREP:u05_q3_run:^(古い version|bob が|切り替えのあと)@@
```

```text
@@OUTGREP:u05_q3_test:^(running|notes_are|thread|assertion|    Err|test result)@@
```

- bob が alice のノートを更新しようとすると、`NotFound`（404）ではなく `VersionMismatch`（409「ほかの画面で先に更新されました。読み込み直してください」）になりました。存在しない ID なら 404 なのに、alice のノートの ID なら 409 なので、bob は、「その番号のノートが、だれかのものとして**ある**」ことを知ってしまいます。
- `notes_are_isolated_per_owner` が、82 行目（「他人は更新できない（NotFound）」）で止めました。ほかの 7 つのテストは通りました。
- `find` をもう一度呼ぶのは、更新が失敗したときだけなので、ふだんの更新は、問い合わせ1回のままです。

### まとめ

- `src/repository.rs` は、3つの trait と libSQL の実装を `pub use` で公開する。ルートは trait だけを知り、`AppState` が `Arc<dyn … + Send + Sync>` で実装を持つ。`async fn` の trait を dyn で使うには `#[async_trait]` が要る（無いと E0038）。
- `notes.rs` は、どの SQL にも `user_id = ?` を入れ、他人のノートを「無い」と同じにする。条件を消すと、コンパイルもほかのテストも通り、`notes_are_isolated_per_owner` だけが止めた。
- `update` は `version` を比べる楽観的ロック。0 行なら `find` で `VersionMismatch`（409）と `NotFound`（404）を分ける。分けないと、他人のノートの存在が 409 で漏れる。`toggle_done` も `version` を上げる。
- 一覧は `COUNT` と `LIMIT/OFFSET` の2回。LIKE は `escape_like` と `ESCAPE '\'` で `%`・`_` をただの文字にする（しないと `100` が「1000円」に当たる）。
- `users.rs` はハッシュ付きの `UserRecord` をログインの照合のときだけ返し、`sessions.rs` はトークンの SHA-256 を `BLOB` で保存して、期限内の行だけを `JOIN` で探す。

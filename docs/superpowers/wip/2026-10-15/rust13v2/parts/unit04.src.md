## UNIT 04: ドメインの型とエラーの分類

### この単元のゴール

- `src/domain.rs` の「検査済みの型」（`NoteTitle`・`Email`・`NoteId` など）が、中身を private にして、`parse` を通らずには作れないようにしている仕組みを説明できる
- タイトル・本文・メール・パスワード・ID の、それぞれの「数え方」と「そろえ方」（trim するか・小文字にするか・改行をそろえるか）を、いろいろな入力の実際の結果で言える
- `src/error.rs` の2つの層（DB の `RepoError` と、利用者への返事の `AppError`）と、`From` による変換、`status_code` と `public_message` の役目を説明できる
- 型が守れていない約束（`NoteQuery` の `pub` な項目、NUL の題）を、本物の実行で確かめられる

### たとえ話でつかむ

宅配便の受付の窓口を考えます。

- 窓口では、荷物の**大きさと重さを測り**、合格した荷物にだけ「**検査済み**」のシールを貼ります。シールは**窓口の係しか貼れません**。だから、奥で荷物を運ぶ人は、シールを見るだけで、測り直しません。
- 送り状の「届け先」と「依頼主」は、どちらも住所ですが、**欄が分かれている**ので、取り違えて書くことはありません。
- 断るときは、「重さも大きさも、両方オーバーです」と、**理由を全部まとめて**伝えます。1つ伝えて直してもらい、また次を伝えるのでは、お客さんが何度も並び直すことになります。
- 奥でトラックが故障しても、窓口では「時間をおいて、もう一度お越しください」とだけ伝え、**詳しい事情は業務日誌**に書きます。
- 「その荷物はありません」と「それは他人の荷物です」は、**同じ返事**にします。他人の荷物があること自体を、知らせないためです。

| 宅配の窓口 | RustNote（`src/domain.rs`・`src/error.rs`） |
|---|---|
| 大きさと重さを測る | `NoteTitle::parse` など（検査に通れば `Ok`、だめなら `Err`） |
| 検査済みのシール | `NoteTitle` などの型（中身は private） |
| シールは窓口の係しか貼れない | 中身が private なので、`domain` の外では `NoteTitle(…)` と書けない（エラー1） |
| 届け先と依頼主の欄 | `UserId` と `NoteId`（中身は同じ `i64` でも、別の型。エラー2） |
| 理由を全部まとめて伝える | `NewNote::parse`（タイトルと本文の問題を両方集める） |
| 窓口での説明と、業務日誌 | `public_message()`（500 では中身を隠す）と、ログ |
| 「ありません」と「他人のです」を同じに | 他人のノートも `AppError::NotFound`（404） |

> **用語:** 検査済みの型（けんさずみのかた：決まり（文字数など）の検査に合格した値だけが入る型。中身を private にして、検査をする関数（`parse`）を通らないと作れないようにする）

<!-- -->

> **用語:** `thiserror`（ジス・エラー：`#[derive(Error)]` と `#[error("…")]` で、エラーの型と、その文（`Display`）を短く書けるようにするクレート）

### 図で理解する

外から来た値が検査済みの型に変わる流れ（上）と、失敗が利用者への返事に変わる流れ（下）です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 340" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-0401" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="20" y="22" font-size="13" font-weight="700" fill="#0f172a">① 外から来た値を、検査済みの型に変える（src/domain.rs）</text>
<rect x="20" y="40" width="170" height="56" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="105" y="63" font-size="12" fill="#0f172a" text-anchor="middle">フォームの文字列</text>
<text x="105" y="83" font-size="12" fill="#475569" text-anchor="middle">「  買い物  」</text>
<rect x="240" y="40" width="170" height="56" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="325" y="63" font-size="12" fill="#1d4ed8" text-anchor="middle">NoteTitle::parse</text>
<text x="325" y="83" font-size="11" fill="#1d4ed8" text-anchor="middle">trim → 文字数を数える</text>
<rect x="460" y="34" width="240" height="30" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="580" y="54" font-size="12" fill="#047857" text-anchor="middle">Ok(NoteTitle("買い物"))</text>
<rect x="460" y="74" width="240" height="30" rx="6" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="580" y="94" font-size="12" fill="#b91c1c" text-anchor="middle">Err(TitleError::Empty など)</text>
<line x1="190" y1="68" x2="236" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0401)"/>
<line x1="410" y1="62" x2="456" y2="50" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0401)"/>
<line x1="410" y1="76" x2="456" y2="88" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0401)"/>
<text x="20" y="128" font-size="11" fill="#475569">NoteTitle の中身は private。parse を通らずには作れないので、NoteTitle の値がある ＝ 1〜80 文字の検査に合格した</text>
<text x="20" y="146" font-size="11" fill="#b91c1c">ただし NoteQuery は項目が pub なので、new を通らずに page: 0 を作れる（問2）</text>
<line x1="20" y1="162" x2="700" y2="162" stroke="#cbd5e1" stroke-width="1"/>
<text x="20" y="186" font-size="13" font-weight="700" fill="#0f172a">② 失敗を、利用者への返事に分類する（src/error.rs）</text>
<rect x="20" y="204" width="180" height="40" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="110" y="229" font-size="12" fill="#0f172a" text-anchor="middle">TitleError など（入力）</text>
<rect x="20" y="262" width="180" height="40" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="110" y="287" font-size="12" fill="#0f172a" text-anchor="middle">RepoError（DB の層）</text>
<rect x="260" y="228" width="140" height="52" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="330" y="251" font-size="13" font-weight="700" fill="#1d4ed8" text-anchor="middle">AppError</text>
<text x="330" y="270" font-size="11" fill="#1d4ed8" text-anchor="middle">#[from] と From</text>
<rect x="460" y="204" width="240" height="40" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="580" y="229" font-size="12" fill="#c2410c" text-anchor="middle">status_code(): 400・401・404・409・500</text>
<rect x="460" y="262" width="240" height="40" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="580" y="287" font-size="12" fill="#c2410c" text-anchor="middle">public_message(): 500 は中身を隠す</text>
<line x1="200" y1="224" x2="256" y2="246" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0401)"/>
<line x1="200" y1="282" x2="256" y2="264" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0401)"/>
<line x1="400" y1="246" x2="456" y2="226" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0401)"/>
<line x1="400" y1="264" x2="456" y2="282" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-0401)"/>
<text x="20" y="326" font-size="11" fill="#475569">src/views.rs の error_page が、この2つで、状態コード付きのエラー画面を作る（UNIT 07）。500 の原因はログにだけ書く</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: 検査済みの型とエラーの分類。外から来た値は parse で型に変え、失敗は AppError にまとめてから、状態コードと画面の文に変える</div>
</div>

### まず動かす最小コード

#### 例4-1: src/domain.rs の全文

> **▼ このファイルがやること（先に日本語で）:** RustNote が扱う「もの」の型を決めます。外から来た文字列（フォームの入力・URL の `{id}`）は、ここの `parse` を通して、検査済みの型（`NoteTitle`・`NoteBody`・`Email`・`Password`・`NoteId`）に変えます。ユーザーとノート（`User`・`Note`）、作成と編集の入力のまとまり（`NewNote`）、一覧の検索の条件とページ送り（`NoteQuery`・`NotePage`）も、ここにあります。最後に、13 のユニットテストのうち 11 があります。

```rust
@@FILE:rustnote/src/domain.rs@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 中身が private な「検査済みの型」

- `pub struct NoteTitle(String);` の `String` には `pub` が付いていません。型の名前は公開されていますが、**中身は `domain` の中からしか触れません**。だから、ほかのファイルでは `NoteTitle(…)` と書いて作ることも、`.0` で中身を書き換えることもできず、`NoteTitle::parse` で作り、`as_str()` で読むしかありません（エラー1）。
- こうすると、「`NoteTitle` 型の値がある」なら「1〜80 文字の検査に合格した」が、必ず成り立ちます。ルートやリポジトリは、`NoteTitle` を受け取るだけで、測り直しません（第9巻 UNIT 03 の「不変条件を型の検査で守る」）。
- `UserId` と `NoteId` は、中身はどちらも `i64` ですが、別の型です。ユーザーの ID を、ノートの ID の代わりに渡すと、コンパイルエラーになります（エラー2）。
- `NoteId` は `FromStr` を実装しているので、`"42".parse::<NoteId>()` と書けます。Topcoat の `module_param!` も、この `FromStr` で、URL の `/notes/{id}` の `{id}` を `NoteId` に変えます（UNIT 08）。

##### 解説2: 数え方と、そろえ方

| 型 | そろえ方 | 数え方と上限 | Debug と Display |
|---|---|---|---|
| `NoteTitle` | 前後の空白を除く（`trim`） | 文字数（`chars().count()`）で 1〜80 | どちらもあり |
| `NoteBody` | `\r\n` を `\n` にそろえる。trim しない | 文字数で 0〜10,000（空でよい） | Debug だけ |
| `Email` | trim して、小文字にそろえる | バイト数（`len()`）で 254 まで。`@` が1つ・ドメインに `.` など | どちらもあり |
| `Password` | そろえない（空白も、利用者が決めたパスワードの一部） | 文字数で 8〜128 | Debug は `Password(***)`。Display は無い |
| `NoteId`・`UserId` | なし | 1 以上の `i64` | どちらもあり |

- タイトルを `len()`（バイト数）で数えると、日本語は1文字が3バイトなので、27 文字で 80 を超えてしまいます。だから `chars().count()` で数えます（テスト `note_title_counts_characters_not_bytes`）。
- ブラウザのフォームは、改行を `\r\n` で送ってきます。本文は `\n` にそろえてから数えるので、改行を1文字と数えます。
- メールは、同じ人を同じ値にするために、小文字にそろえます。`Alice@Example.COM` と `alice@example.com` は、同じユーザーです。
- `Password` の `Debug` は、手で書いて `Password(***)` とだけ表示します。`#[derive(Debug)]` のままだと、`{:?}` でログに出したときに、パスワードがそのまま書かれてしまうからです（UNIT 03 の `Secret` と同じ考え方）。

##### 解説3: NewNote::parse は、問題を全部集める

- `match (NoteTitle::parse(title), NoteBody::parse(body))` で、2つを**両方**検査してから、結果の組み合わせを見ます。両方 `Ok` なら `NewNote`、どちらかが `Err` なら、`NoteInputErrors` に、それぞれの問題（無ければ `None`）を入れて返します。
- こうすると、「タイトルが空」と「本文が長すぎる」を、一度に画面に出せます（問1で、`?` で書いた版と比べます）。

##### 解説4: 一覧の検索の条件とページ送り

- `NoteQuery::new` は、URL の `?q=` と `?page=` から作ります。キーワードは trim して、空なら `None`（全部表示）、長ければ先頭の 100 文字だけにします。ページは、無ければ 1、0 なら 1 にそろえます（`max(1)`）。
- `offset` は、SQL の `OFFSET`（何件読み飛ばすか）で、`(page - 1) × 20` です。掛け算は `u64` でするので、大きなページの番号でも、あふれません。
- `NotePage::total_pages` は、`div_ceil`（割って切り上げ）で、全部で何ページあるかを数えます。0 件でも 1 ページです。`has_prev` と `has_next` は、「前へ」「次へ」のリンクを出すかどうかに使います（UNIT 07 の `pagination`）。
- **注意:** `NoteQuery` の項目（`keyword`・`page`）は `pub` です。ファイルの先頭のコメントには「検査を通らずに作る方法はない」とありますが、`NoteQuery` だけは、`new` を通らずに作れます（問2・第11巻 UNIT 04）。

#### 例4-2: src/error.rs の全文

> **▼ このファイルがやること（先に日本語で）:** 失敗を2つの層に分けます。`RepoError` は DB の層の失敗（一意制約の違反・version の食い違い・見つからない・値を読めない・DB の故障）です。`AppError` は、利用者への返事を決めるための分類で、それぞれが HTTP の状態コードに対応します。`RepoError` を `AppError` に変える決まり（`From`）と、状態コード（`status_code`）、画面に出してよい文（`public_message`）も、ここにあります。

```rust
@@FILE:rustnote/src/error.rs@@
```

##### 解説5: 2つの層と、From による変換

- **`RepoError`（DB の層）:** libsql のエラーは、`From<libsql::Error>` で `RepoError` に変わります。文に `UNIQUE constraint failed` があれば `UniqueViolation`、それ以外は `Database` です。この文は、手元の SQLite でも Turso でも同じなので、文で見分けています。`Database(#[source] libsql::Error)` の `#[source]` は、元のエラーを「原因」として持っておく印で、ログに原因まで書けます。
- **`AppError`（返事の分類）:** `InvalidTitle(#[from] TitleError)` の `#[from]` は、`From<TitleError> for AppError` を作ります。だから、`AppError` を返す関数の中では、`NoteTitle::parse(…)?` と `?` を付けるだけで、`TitleError` が `AppError::InvalidTitle` に変わります（`#[from]` が無い型に `?` を付けると、エラー3）。`#[error(transparent)]` は、「文は、中の `TitleError` の文をそのまま使う」という意味です。
- **`From<RepoError> for AppError`:** `VersionMismatch` は `Conflict`（409）、`NotFound` は `NotFound`（404）、それ以外は全部 `Internal`（500）です。`UniqueViolation` も、ここでは `Internal` になります。登録済みのメールを 409 にするのは、`src/routes/signup.rs` が、変換する**前**に `UniqueViolation` を見分けているからです（UNIT 07）。

```rust
// src/routes/signup.rs の register から（抜き出し）
let user = match app_state(cx).users.create(&email, &password_hash).await {
    Ok(user) => user,
    // users.email の UNIQUE に引っかかった ＝ 登録済みのメール
    Err(RepoError::UniqueViolation(_)) => {
        form.email_error = Some(AppError::EmailTaken.to_string());
        return Ok(SignupOutcome::Rejected(StatusCode::CONFLICT, form));
    }
    Err(other) => return Err(other.into()),
};
```

##### 解説6: status_code と public_message

- `status_code` の `match` には、`_`（そのほか全部）が**ありません**。`AppError` に種類を足すと、ここがコンパイルエラーになり、「新しい種類は、何番の状態コードか」を決めるまで、先へ進めません（問3）。
- `public_message` は、`Internal` だけ「時間をおいてもう一度お試しください」にして、ほかは `#[error("…")]` の文をそのまま返します。500 の原因（DB の故障の文など）は、画面には出さず、ログにだけ書きます（第11巻 UNIT 07）。
- `AppError::internal(…)` は、`Into<BoxError>` にできる物なら何でも `Internal` に包みます。`&str`（`"disk full"`）も、`std::error::Error` を実装した型も渡せます。

### もう一歩: 実務に近い形

#### 例4-3: いろいろな入力で確かめる

> **▼ このコードがやること（先に日本語で）:** 検査済みの型の `parse` に、ふつうの入力と、まぎらわしい入力（NUL・絵文字・全角の文字・前後の空白・とても大きな数）を渡して、結果を表示します。最後に、いろいろな失敗を `AppError` に変えて、状態コードと、画面に出す文を表示します。

```rust
@@FILE:rustnote/examples/u04_types.rs@@
```

```powershell
cargo run -q --example u04_types
```

```text
@@OUT:u04_types@@
```

- **NUL の題（`"\0"`）が合格しました。** `NoteTitle::parse` は、NUL（番号 0 の文字）も1文字と数えます。ところが、DB の `CHECK (length(title) BETWEEN 1 AND 80)` の SQLite の `length()` は、NUL より前しか数えないので、この題は DB で断られて 500 になります（UNIT 02 の注意・第11巻 UNIT 04）。`MAX_TITLE_CHARS` のコメントの「SQLite の `length()` と同じ」は、NUL では成り立ちません（改善点）。
- **絵文字 80 個（`😀×80`）も合格しました。** `chars().count()` は、絵文字1つを1文字と数えます。ブラウザの入力欄の `maxlength="80"` は UTF-16 の単位で数えるので、ブラウザでは 40 個までしか入りません（第11巻 UNIT 05）。
- **全角の `ａｌｉｃｅ@example.com` も合格しました。** `to_lowercase` は大文字と小文字をそろえるだけで、全角を半角にはしません。`alice@example.com` とは別のユーザーになります。
- パスワードは、前後の空白もそのまま残ります（`Password(***)` と隠れて表示）。日本語の 8 文字も合格です。
- `NoteId` は、`"01"` を 1 と読み、前に空白がある `" 1"` と、`i64` に入らない `9223372036854775808` は `NotANumber` でした。
- `NoteQuery::new` は、150 文字のキーワードを 100 文字に切り、空白だけのキーワードを `None` に、`page` の 0 を 1 にそろえました。
- 失敗の分類では、`UniqueViolation` が、`From` では `Internal`（500）になりました（解説5）。`Internal` は、`disk full` のような原因を、画面の文に出しません。

### よくあるエラーと読み方

#### エラー1: 検査を通らずに、`NoteId` を直接作ろうとした（E0423）

```rust
@@FILE:errs_u04/e04_private.rs@@
```

```text
@@OUT:u04_err_private@@
```

- **何が起きたか:** 「private な項目を含むタプル構造体は、作れません」というエラーです。`note:` が、`src/domain.rs` の `pub struct NoteId(i64);` の `i64` を指して、`private field` と教えています。
- **読み方:** 中身が private なのは、**わざと**です。`NoteId(0)` のような、検査に通らない値を作らせないためです。
- **直し方:** `NoteId::new(5)` か `"5".parse::<NoteId>()` で作り、`Result` を受け取ります。

#### エラー2: ユーザーの ID を、ノートの ID の代わりに渡した（E0308）

```rust
@@FILE:errs_u04/e04_mixid.rs@@
```

```text
@@OUT:u04_err_mixid@@
```

- **何が起きたか:** 「型が合いません。`NoteId` のはずが、`UserId` でした」というエラーです。
- **読み方:** 中身がどちらも `i64` でも、型が違えば渡せません。ID を全部 `i64` のままにしていたら、このまちがいはコンパイルを通り、「ユーザー 7 番の ID で、ノート 7 番を表示する」という、見つけにくいバグになります。
- **直し方:** 渡す値を、正しい型（ノートの ID）にします。

#### エラー3: `AppError` を返す関数の中で、数の変換の失敗に `?` を付けた（E0277）

```rust
@@FILE:errs_u04/e04_question.rs@@
```

```text
@@OUT:u04_err_question@@
```

- **何が起きたか:** 「`?` が、エラーを `AppError` に変換できません」というエラーです。`?` は、エラーを `From` で、関数の返す型のエラーに変えます。`AppError` には `From<ParseIntError>` がありません。
- **読み方:** `help:` に、`AppError` が実装している `From` の一覧（`BodyError`・`EmailError`・`PasswordError`・`RepoError`・`TitleError`）が出ています。`#[from]` を付けた4つと、手で書いた `From<RepoError>` です。
- **直し方:** どの分類にするかを決めて、`map_err` で変えます。たとえば、利用者の入力のまちがいなら、入力のエラーの型を作って `#[from]` を足し、想定外の失敗なら `.map_err(AppError::internal)?` とします。

### RustNote ではここで使う

- **入力の検査:** 登録のフォームは `Email::parse`・`Password::parse`（UNIT 07）、ログインの照合は `Email::parse`（UNIT 06 の `src/auth.rs`）、ノートの作成と編集のフォームは `NewNote::parse`（UNIT 08）、URL の `{id}` は `NoteId` の `FromStr`（UNIT 08）を通ります。
- **DB の失敗:** リポジトリは `RepoError` を返します（UNIT 05）。ルートは、それを `?` や `map_err(AppError::from)` で `AppError` に変えます。
- **エラー画面:** `src/views.rs` の `error_page` が、`status_code` と `public_message` で、状態コード付きの画面を作ります。500 のときは、原因をログにだけ書きます（UNIT 07）。
- **テスト:** 2つのファイルのユニットテストは、13 個です（`cargo test --lib` の 19 個のうち、`domain::` と `error::` の分）。

```powershell
cargo test --lib -- domain:: error::
```

```text
@@OUTGREP:u04_lib_tests:^(running|test |test result)@@
```

- **改善点（この単元で見つかったこと）:**
  - NUL の題が `NoteTitle::parse` を通り、DB で断られて 500 になる（例4-3）。
  - `NoteQuery` の項目が `pub` なので、`page: 0` を作れる（問2）。
  - `src/views.rs` の `error_page` は、`RepoError` を、`From<RepoError> for AppError` と**同じ表を、もう一度書いて**分類しています（`NotFound` は 404、`VersionMismatch` は 409、ほかは 500）。表を変えるときは、2か所を直す必要があります。

### 練習問題

**問1:** `NewNote::parse` は、なぜ `?` を使わずに、`match` で2つの結果を組み合わせているのでしょうか。`?` で書いた版と、実際に比べてください。

**問2:** 例4-3では、`NoteQuery::new(…, Some(0))` の `page` は 1 にそろいました。では、`NoteQuery { keyword: None, page: 0 }` と項目を直接書いて作り、`offset()` を呼ぶと、どうなるでしょうか。`cargo run` と、`cargo run --release` の両方で確かめ、どう直せばよいかを考えてください。

**問3:** `AppError` に、新しい種類 `TooManyNotes`（ノートの数が上限に達した）を足すと、どこでコンパイルエラーになるでしょうか。`public_message` は、どうなるでしょうか。

### 解答と解説

**問1の解答:**

```rust
@@FILE:rustnote/examples/u04_q1.rs@@
```

```text
@@OUT:u04_q1@@
```

- `?` で書いた版は、タイトルの問題で止まるので、本文の問題（10,001 文字）は伝わりませんでした。利用者は、タイトルを直して送り直してから、初めて本文の問題を知ります。
- 教材の版は、両方の問題を集めて返しました。作成のフォームは、この2つを、それぞれの入力欄の下に、一度に表示します（UNIT 08）。

**問2の解答:**

```rust
@@FILE:rustnote/examples/u04_q2.rs@@
```

```text
@@OUT:u04_q2@@
```

```text
@@OUT:u04_q2_release@@
```

- `cargo run`（debug）では、`offset` の `self.page - 1`（`u32` の 0 から 1 を引く）が、`attempt to subtract with overflow`（引き算であふれた）でパニックしました。
- `--release` では、あふれの検査が無いので、0 − 1 が `u32` のいちばん大きな数（4,294,967,295）に回り込み、`offset` は 85,899,345,900（その 20 倍）になりました。パニックはしませんが、SQL はそれだけ読み飛ばすので、**エラーも無く、空のページ**になります。
- 今の RustNote は、`NoteQuery` を `NoteQuery::new` でしか作っていないので、どちらも起きません。ただし、**型が、約束（`page` は 1 以上）を守っていません**。
- **直し方:** 項目を `pub` でなくし、`keyword()`・`page()` のような読むためのメソッドを足して、`NoteQuery::new` だけで作れるようにします。項目を直接読んでいる所（`src/repository/notes.rs`・`src/routes/notes.rs`・`src/views.rs`）は、メソッドを呼ぶように直します。教材のコードは、まだ直していません（巻末の総合演習で扱います）。

**問3の解答:**

`src/error.rs` の `Conflict,` の次に、`#[error("ノートの数が上限に達しました")]` と `TooManyNotes,` を足して、`cargo build` しました。

```text
@@OUTTAIL:u04_q3:23@@
```

- コンパイルエラーは、`status_code` の `match`（E0004、「`&AppError::TooManyNotes` が扱われていません」）の1か所でした。`_` が無いので、新しい種類の状態コードを決めるまで、先へ進めません。`help:` は `todo!()` の腕を足す案を出していますが、実際には、409 や 422 など、意味に合う状態コードを書きます。
- `public_message` は、`other => other.to_string()` の腕があるので、エラーになりません。`#[error("…")]` の文（「ノートの数が上限に達しました」）が、そのまま画面に出ます。画面に出してよい文かどうかは、`#[error]` を書くときに決めておきます。
- `_` や `other` を使うと、書く量は減りますが、種類を足したときに、コンパイラが教えてくれなくなります。返事の分類のように、「種類ごとに必ず決めてほしい」所では、`_` を使わずに全部の種類を書きます。

### まとめ

- `src/domain.rs` の検査済みの型（`NoteTitle`・`NoteBody`・`Email`・`Password`・`NoteId`・`UserId`）は、中身が private なので、`parse`・`new` を通らずには作れない（E0423）。`UserId` と `NoteId` は取り違えられない（E0308）。
- タイトルは trim して文字数で 1〜80、本文は改行をそろえて 10,000 まで、メールは trim と小文字、パスワードはそろえずに 8〜128 文字で `Debug` を隠す。`NewNote::parse` は問題を全部集める。
- `src/error.rs` は、DB の `RepoError` と、返事の `AppError` の2つの層。`#[from]` と `From` で `?` が変換し（無いと E0277）、`status_code` は `_` 無しの `match`（足し忘れは E0004）、`public_message` は 500 の中身を隠す。
- 改善点: NUL の題が型の検査を通って 500 になる。`NoteQuery` の `pub` な項目で `page: 0` を作ると、debug ではパニック、release では空のページ。`error_page` が分類の表を二重に持つ。

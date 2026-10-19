## UNIT 09: Repository・行の変換・N+1

### この単元のゴール

- `notes` 表の SQL を **Repository**（リポジトリ）という1つの部品に集め、ページの処理（ハンドラー）から SQL を追い出せる
- `NOTE_COLUMNS` と `row_to_note` で、DB の行を検証済みの型（`NoteId`・`NoteTitle` など）に変換できる。変換に失敗したら、「どの表のどの列の、どの行か」を添えたエラーにできる
- `#[async_trait]` を付けた trait を `Arc<dyn NoteRepository + Send + Sync>` として持ち、`RepoError` を利用者への返事（404・409・500）に変換できる
- **N+1 問題**を SQL の回数で見つけ、`JOIN`・`GROUP BY`・`IN` で1回にまとめられる

### たとえ話でつかむ

図書館のカウンターを思い浮かべてください。

- 利用者（**ハンドラー**）は、「この本を貸してください」と頼むだけです。書庫のどの棚にあるか（**SQL**）は知りません。
- 司書（**Repository**）は、書庫から本を取ってきて、ページが破れていないかを確かめてから（**行の変換と検証**）渡します。破れていたら、「3階の B 棚の、この本のこのページが破れています」と、場所をはっきり書いて報告します。
- 書庫の並べ方を変えても（表の列が増えても）、利用者の頼み方は変わりません。直すのは司書の仕事の中だけです。

もう1つ、買い物で考えます。買う物が 10 品あるとき、1品ずつ「家とお店を往復」すると 10 回往復します。メモを持って1回で全部買えば、1回で済みます。プログラムが DB に SQL を1件ずつ送って、何百回も往復してしまうのが **N+1 問題**です。

> **用語:** Repository（リポジトリ：データをどこに・どうしまうかを引き受ける部品。ほかの部品は「ノートを探して」と頼むだけで、SQL を知らなくてよい）

<!-- -->

> **用語:** N+1 問題（一覧を読む SQL を1回送った後、その N 件それぞれについて SQL をもう1回ずつ送ってしまい、合わせて N+1 回 DB と往復すること。件数が増えるほど遅くなる）

### 図で理解する

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-091" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="16" width="240" height="56" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="140" y="40" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">ハンドラー（ページの処理）</text>
<text x="140" y="60" font-size="11" fill="#475569" text-anchor="middle">SQL を1行も知らない</text>
<rect x="20" y="112" width="240" height="56" rx="10" fill="#fefce8" stroke="#eab308" stroke-width="2" stroke-dasharray="6 3"/>
<text x="140" y="136" font-size="13" font-weight="700" fill="#854d0e" text-anchor="middle">trait NoteRepository</text>
<text x="140" y="156" font-size="11" fill="#475569" text-anchor="middle">約束: find・rename など</text>
<rect x="20" y="208" width="240" height="56" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="140" y="232" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">LibsqlNoteRepository</text>
<text x="140" y="252" font-size="11" fill="#475569" text-anchor="middle">SQL はここにだけ書く</text>
<line x1="140" y1="72" x2="140" y2="108" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-091)"/>
<text x="148" y="94" font-size="11" fill="#475569">呼ぶ</text>
<line x1="140" y1="168" x2="140" y2="204" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-091)"/>
<text x="148" y="190" font-size="11" fill="#475569">約束を守る実装</text>
<rect x="320" y="208" width="120" height="56" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="380" y="232" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">libSQL</text>
<text x="380" y="252" font-size="11" fill="#475569" text-anchor="middle">notes 表</text>
<line x1="260" y1="226" x2="316" y2="226" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-091)"/>
<text x="288" y="218" font-size="10" fill="#475569" text-anchor="middle">SQL＋値</text>
<line x1="320" y1="250" x2="264" y2="250" stroke="#10b981" stroke-width="2" marker-end="url(#ar-07-091)"/>
<text x="292" y="274" font-size="10" fill="#047857" text-anchor="middle">行（Row）</text>
<rect x="470" y="16" width="235" height="120" rx="10" fill="#ffffff" stroke="#3b82f6"/>
<text x="587" y="38" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">行 → 型（row_to_note）</text>
<text x="484" y="62" font-size="11" fill="#0f172a">Row（列の番号で値を持つ）</text>
<text x="484" y="82" font-size="11" fill="#475569">↓ get・NoteId::new・検証</text>
<text x="484" y="102" font-size="11" fill="#0f172a">Note（検証済みの型）</text>
<text x="484" y="124" font-size="10" fill="#b91c1c">失敗 → InvalidData（表.列 と id）</text>
<rect x="470" y="150" width="235" height="134" rx="10" fill="#ffffff" stroke="#f97316"/>
<text x="587" y="172" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">失敗の変換</text>
<text x="484" y="196" font-size="11" fill="#0f172a">libsql::Error → RepoError</text>
<text x="484" y="216" font-size="11" fill="#0f172a">RepoError → AppError</text>
<text x="494" y="236" font-size="11" fill="#475569">NotFound → 404</text>
<text x="494" y="254" font-size="11" fill="#475569">VersionMismatch → 409</text>
<text x="494" y="272" font-size="11" fill="#475569">それ以外 → 500（原因はログへ）</text>
<line x1="440" y1="236" x2="466" y2="110" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 3"/>
<line x1="260" y1="44" x2="466" y2="210" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 3"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: Repository を使った層の分け方。ハンドラーは trait の約束だけを使い、SQL と行の変換は LibsqlNoteRepository の中に閉じ込める。失敗は RepoError から AppError へ変換して、404・409・500 を決める</div>
</div>

### まず動かす最小コード

#### 例9-1: notes 表の SQL を NoteRepo に集める

> **▼ このコードがやること（先に日本語で）:** 第4巻で作ったドメインの型（`UserId`・`NoteId`・`NoteTitle`・`Note`）を小さくして用意します。`notes` 表の SQL をすべて `NoteRepo` の中に書き、「持ち主のノートを新しい順に全部」と「持ち主のノートを1件」の2つの読み方を作ります。読んだ行は `row_to_note` で `Note` に変換し、変換できない値があれば `RepoError::InvalidData` にします。

```rust
use libsql::{Builder, Connection, Row};
use thiserror::Error;

// ---- ドメインの型（第4巻 UNIT 01 の形を小さくしたもの） ----
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NoteId(i64);

impl NoteId {
    // 1 以上の数だけを受け付ける
    fn new(value: i64) -> Result<NoteId, String> {
        if value <= 0 {
            return Err(format!("{value} は 1 以上ではありません"));
        }
        Ok(NoteId(value))
    }
}

#[derive(Debug)]
struct NoteTitle(String);

impl NoteTitle {
    // 1〜80 文字だけを受け付ける
    fn parse(input: &str) -> Result<NoteTitle, String> {
        let len = input.chars().count();
        if len == 0 || len > 80 {
            return Err(format!("タイトルは 1〜80 文字です（{len} 文字）"));
        }
        Ok(NoteTitle(input.to_string()))
    }
}

#[derive(Debug)]
struct Note {
    id: NoteId,
    owner: UserId,
    title: NoteTitle,
    done: bool,
}

// ---- 保存先の層の失敗 ----
#[derive(Debug, Error)]
enum RepoError {
    #[error("DB の値を読み取れません: {0}")]
    InvalidData(String),
    #[error("DB の操作に失敗しました: {0}")]
    Database(#[from] libsql::Error),
}

// ---- Repository: notes 表の SQL はすべてここに集める ----
struct NoteRepo {
    conn: Connection,
}

// SELECT で読む列。row_to_note の番号と順番をそろえる
const NOTE_COLUMNS: &str = "id, user_id, title, done";

impl NoteRepo {
    // 自分のノートを新しい順に全部返す
    async fn list_by_owner(&self, owner: UserId) -> Result<Vec<Note>, RepoError> {
        let sql = format!("SELECT {NOTE_COLUMNS} FROM notes WHERE user_id = ?1 ORDER BY id DESC");
        let mut rows = self.conn.query(&sql, [owner.0]).await?;
        let mut notes = Vec::new();
        while let Some(row) = rows.next().await? {
            notes.push(row_to_note(&row)?);
        }
        Ok(notes)
    }

    // 自分のノートを1件探す。他人のノートや無いノートは None
    async fn find(&self, owner: UserId, id: NoteId) -> Result<Option<Note>, RepoError> {
        let sql = format!("SELECT {NOTE_COLUMNS} FROM notes WHERE id = ?1 AND user_id = ?2");
        let mut rows = self.conn.query(&sql, [id.0, owner.0]).await?;
        match rows.next().await? {
            Some(row) => Ok(Some(row_to_note(&row)?)),
            None => Ok(None),
        }
    }
}

// 1行を Note に変える。変換できなければ「どの列か」を添えて InvalidData にする
fn row_to_note(row: &Row) -> Result<Note, RepoError> {
    let id = NoteId::new(row.get::<i64>(0)?)
        .map_err(|e| RepoError::InvalidData(format!("notes.id: {e}")))?;
    let title = NoteTitle::parse(&row.get::<String>(2)?)
        .map_err(|e| RepoError::InvalidData(format!("notes.title (id={}): {e}", id.0)))?;
    Ok(Note {
        id,
        owner: UserId(row.get::<i64>(1)?),
        title,
        done: row.get::<i64>(3)? != 0,
    })
}

#[tokio::main]
async fn main() -> Result<(), RepoError> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    let repo = NoteRepo { conn };
    let alice = UserId(1);

    println!("--- Alice のノート一覧 ---");
    for note in repo.list_by_owner(alice).await? {
        let mark = if note.done { "済" } else { "  " };
        println!(
            "[{mark}] #{} {}（持ち主 {}）",
            note.id.0, note.title.0, note.owner.0
        );
    }

    println!("--- 1件だけ探す ---");
    let mine = repo.find(alice, NoteId(2)).await?;
    println!("Alice が #2 を探す: {mine:?}");
    let others = repo.find(alice, NoteId(3)).await?;
    println!("Alice が #3（Bob のノート）を探す: {others:?}");
    Ok(())
}
```

`src/bin/u09_repo.rs` として保存して実行します。

```powershell
cargo run -q --bin u09_repo
```

**▼ 実行結果**

```text
--- Alice のノート一覧 ---
[  ] #7 100% 達成（持ち主 1）
[  ] #6 SQL の練習（持ち主 1）
[  ] #4 読書メモ（持ち主 1）
[済] #2 Rust の勉強（持ち主 1）
[  ] #1 買い物リスト（持ち主 1）
--- 1件だけ探す ---
Alice が #2 を探す: Some(Note { id: NoteId(2), owner: UserId(1), title: NoteTitle("Rust の勉強"), done: true })
Alice が #3（Bob のノート）を探す: None
```

`main` には SQL が1行もありません（表と練習用データを用意する `execute_batch` を除きます）。`main` は「Alice のノートの一覧」「Alice の #2」と頼むだけで、どんな SQL が送られたかを知りません。

#### ▼ コードを1つずつ分解して解説

##### 解説1: ドメインの型で、DB の値に意味を持たせる

- `UserId(i64)` と `NoteId(i64)` は、中身はどちらも整数ですが、別の型です（第4巻の newtype）。`repo.find(alice, NoteId(2))` の2つの引数を逆に書くと、コンパイルエラーになります。
- `NoteId::new` は 1 以上、`NoteTitle::parse` は 1〜80 文字の値だけを受け付けます。`Note` を作れたら、その中身は約束を満たしている、と信じて使えます。

##### 解説2: NOTE_COLUMNS で「読む列」を1か所に書く

- `const NOTE_COLUMNS: &str = "id, user_id, title, done";` を、`SELECT {NOTE_COLUMNS} FROM …` のように `format!` で埋め込んでいます。どの SQL でも、同じ列が同じ順番で返ってきます。
- `row_to_note` は、この順番どおりに `get(0)`（id）・`get(1)`（user_id）・`get(2)`（title）・`get(3)`（done）と読みます。列を足すときは、`NOTE_COLUMNS` と `row_to_note` の2か所だけを直します。
- `format!` で SQL に混ぜているのは、プログラムの中の定数 `NOTE_COLUMNS` だけです。利用者から来る値（`owner`・`id`）は、UNIT 07 のとおり `?1`・`?2` で bind しています。

##### 解説3: row_to_note は、失敗に「場所」を添える

- `NoteId::new(…)` や `NoteTitle::parse(…)` が失敗したら、`map_err` で `RepoError::InvalidData("notes.title (id=4): …")` のように、**表の名前・列の名前・行の id** を添えたエラーに変えます。
- DB には `CHECK` 制約があるので、ふつうは失敗しません。それでも、制約を足す前に入った古い行や、手で直した行が混ざることがあります。そのとき「どの行を直せばよいか」がすぐ分かるように、場所を書いておきます（例9-2 の⑤で、実際に壊れた行を読みます）。
- `done` は DB では 0 と 1 の整数なので、`row.get::<i64>(3)? != 0` で `bool` にしています（UNIT 07 の `InvalidColumnType` を避ける読み方です）。

##### 解説4: find は持ち主の条件込みで、Option を返す

- `find` の SQL は `WHERE id = ?1 AND user_id = ?2` です。UNIT 03 で学んだ**持ち主の条件**を、Repository の中に必ず入れています。
- Bob のノート #3 を Alice が探すと、エラーではなく `None` が返りました。呼ぶ側から見ると、「他人のノート」と「存在しないノート」は区別できません。これが、他人のノートを 404 にするための仕組みです（練習問題3）。

##### 解説5: ? だけで libsql のエラーを RepoError に変える

- `Database(#[from] libsql::Error)` の `#[from]` で、`From<libsql::Error> for RepoError` が作られます。だから、`self.conn.query(…).await?` の `?` が、`libsql::Error` を自動で `RepoError::Database` に変えてくれます。
- `main` の戻り値も `Result<(), RepoError>` なので、`Builder::new_local(…).build().await?` の失敗も同じように変換されます。

### もう一歩: 実務に近い形

#### 例9-2: trait で層を分け、失敗を 404・409・500 に変える

RustNote のハンドラーは、本物の DB を使う `LibsqlNoteRepository` を、**trait の型**（`dyn NoteRepository`）として受け取ります。こうしておくと、ハンドラーを書き換えずに、DB を使わない別の実装（テスト用の実装など）に差し替えられる形になります。

trait のメソッドを `async fn` にして `dyn` で使うために、`async-trait` クレートを足します。

```powershell
cargo add async-trait@0.1.92
```

```text
    Updating crates.io index
      Adding async-trait v0.1.92 to dependencies
    Updating crates.io index
     Locking 1 package to highest Rust 1.98 compatible version
      Adding async-trait v0.1.92
```

> **▼ このコードがやること（先に日本語で）:** `NoteRepository` という trait（約束）に「1件探す」と「タイトルを変える」の2つを決め、libSQL で実装します。タイトルの変更は UNIT 05 の楽観的ロック（`version` が一致したときだけ更新）で行います。SQL を知らないハンドラーのつもりの関数 `show_note`・`rename_note` から、5つの場面（自分のノート・他人のノート・正しい version・古い version・壊れた行）を試し、それぞれの結果を「ステータスコード 文章」の形で表示します。500 のときは、原因をログ（標準エラー出力）にだけ書きます。

```rust
use std::error::Error as _;
use std::sync::Arc;

use async_trait::async_trait;
use libsql::{Builder, Connection, Row};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
struct UserId(i64);

#[derive(Debug, Clone, Copy)]
struct NoteId(i64);

#[derive(Debug)]
struct Note {
    id: NoteId,
    title: String,
    version: i64,
}

// ---- 保存先の層の失敗（RustNote の RepoError と同じ分類） ----
#[derive(Debug, Error)]
enum RepoError {
    #[error("version が一致しません")]
    VersionMismatch,
    #[error("対象のデータが見つかりません")]
    NotFound,
    #[error("DB の値を読み取れません: {0}")]
    InvalidData(String),
    #[error("DB の操作に失敗しました")]
    Database(#[from] libsql::Error),
}

// ---- アプリの層の失敗（利用者への返事を決める） ----
#[derive(Debug, Error)]
enum AppError {
    #[error("ページが見つかりません")]
    NotFound,
    #[error("ほかの画面で先に更新されました。読み込み直してください")]
    Conflict,
    #[error("サーバーの中でエラーが起きました")]
    Internal(#[source] RepoError),
}

impl From<RepoError> for AppError {
    fn from(error: RepoError) -> Self {
        match error {
            RepoError::VersionMismatch => AppError::Conflict,
            RepoError::NotFound => AppError::NotFound,
            other => AppError::Internal(other),
        }
    }
}

impl AppError {
    fn status_code(&self) -> u16 {
        match self {
            AppError::NotFound => 404,
            AppError::Conflict => 409,
            AppError::Internal(_) => 500,
        }
    }
}

// ---- Repository の約束（trait）。async fn を dyn で使うため #[async_trait] を付ける ----
#[async_trait]
trait NoteRepository {
    async fn find(&self, owner: UserId, id: NoteId) -> Result<Option<Note>, RepoError>;
    async fn rename(
        &self,
        owner: UserId,
        id: NoteId,
        title: &str,
        expected_version: i64,
    ) -> Result<Note, RepoError>;
}

// ---- libSQL で約束を守る実装。SQL はこの中にしか書かない ----
struct LibsqlNoteRepository {
    conn: Connection,
}

const NOTE_COLUMNS: &str = "id, title, version";

#[async_trait]
impl NoteRepository for LibsqlNoteRepository {
    async fn find(&self, owner: UserId, id: NoteId) -> Result<Option<Note>, RepoError> {
        let sql = format!("SELECT {NOTE_COLUMNS} FROM notes WHERE id = ?1 AND user_id = ?2");
        let mut rows = self.conn.query(&sql, [id.0, owner.0]).await?;
        match rows.next().await? {
            Some(row) => Ok(Some(row_to_note(&row)?)),
            None => Ok(None),
        }
    }

    async fn rename(
        &self,
        owner: UserId,
        id: NoteId,
        title: &str,
        expected_version: i64,
    ) -> Result<Note, RepoError> {
        // version が一致したときだけ更新し、version を 1 増やす（UNIT 05 の楽観的ロック）
        let sql = format!(
            "UPDATE notes SET title = ?1, version = version + 1
              WHERE id = ?2 AND user_id = ?3 AND version = ?4
              RETURNING {NOTE_COLUMNS}"
        );
        let mut rows = self
            .conn
            .query(
                &sql,
                libsql::params![title, id.0, owner.0, expected_version],
            )
            .await?;
        if let Some(row) = rows.next().await? {
            return row_to_note(&row);
        }
        // 1行も更新されなかった: ノートが無いのか、version が違うのかを見分ける
        match self.find(owner, id).await? {
            Some(_) => Err(RepoError::VersionMismatch),
            None => Err(RepoError::NotFound),
        }
    }
}

// 1行を Note に変える。おかしな値なら、どの列のどの行かを添えて InvalidData にする
fn row_to_note(row: &Row) -> Result<Note, RepoError> {
    let id = row.get::<i64>(0)?;
    let title = row.get::<String>(1)?;
    let len = title.chars().count();
    if !(1..=80).contains(&len) {
        return Err(RepoError::InvalidData(format!(
            "notes.title (id={id}): 1〜80 文字ではありません（{len} 文字）"
        )));
    }
    Ok(Note {
        id: NoteId(id),
        title,
        version: row.get::<i64>(2)?,
    })
}

// ---- ハンドラーのつもりの関数。SQL を1行も知らず、trait だけを使う ----
async fn show_note(
    repo: &(dyn NoteRepository + Send + Sync),
    owner: UserId,
    id: NoteId,
) -> Result<String, AppError> {
    let note = repo.find(owner, id).await?.ok_or(AppError::NotFound)?;
    Ok(format!(
        "#{} {}（version {}）",
        note.id.0, note.title, note.version
    ))
}

async fn rename_note(
    repo: &(dyn NoteRepository + Send + Sync),
    owner: UserId,
    id: NoteId,
    title: &str,
    expected_version: i64,
) -> Result<String, AppError> {
    let note = repo.rename(owner, id, title, expected_version).await?;
    Ok(format!(
        "保存しました: {}（version {}）",
        note.title, note.version
    ))
}

// 結果を「ステータス 文章」の形で表示する。500 のときは原因をログ（標準エラー）へ
fn report(label: &str, result: Result<String, AppError>) {
    match result {
        Ok(message) => println!("{label} → 200 {message}"),
        Err(error) => {
            println!("{label} → {} {error}", error.status_code());
            if let Some(source) = error.source() {
                eprintln!("  [log] 原因: {source}");
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;

    // 本物の実装を、trait の型（dyn）として持つ。RustNote の AppState と同じ形
    let repo: Arc<dyn NoteRepository + Send + Sync> =
        Arc::new(LibsqlNoteRepository { conn: conn.clone() });
    let alice = UserId(1);

    report(
        "① 自分の #2 を表示",
        show_note(&*repo, alice, NoteId(2)).await,
    );
    report(
        "② Bob の #3 を表示",
        show_note(&*repo, alice, NoteId(3)).await,
    );
    report(
        "③ #2 を version 3 で更新",
        rename_note(&*repo, alice, NoteId(2), "Rust の復習", 3).await,
    );
    report(
        "④ #2 を古い version 3 で更新",
        rename_note(&*repo, alice, NoteId(2), "別のタブから", 3).await,
    );

    // CHECK 制約を一時的に無視して、壊れた行をわざと作る（実験のためだけ）
    conn.execute_batch(
        "PRAGMA ignore_check_constraints = ON;
         UPDATE notes SET title = '' WHERE id = 4;
         PRAGMA ignore_check_constraints = OFF;",
    )
    .await?;
    report(
        "⑤ 壊れた #4 を表示",
        show_note(&*repo, alice, NoteId(4)).await,
    );
    Ok(())
}
```

`src/bin/u09_layers.rs` として保存して実行します。

```powershell
cargo run -q --bin u09_layers
```

**▼ 実行結果**（`[log]` の行は標準エラー出力）

```text
① 自分の #2 を表示 → 200 #2 Rust の勉強（version 3）
② Bob の #3 を表示 → 404 ページが見つかりません
③ #2 を version 3 で更新 → 200 保存しました: Rust の復習（version 4）
④ #2 を古い version 3 で更新 → 409 ほかの画面で先に更新されました。読み込み直してください
⑤ 壊れた #4 を表示 → 500 サーバーの中でエラーが起きました
  [log] 原因: DB の値を読み取れません: notes.title (id=4): 1〜80 文字ではありません（0 文字）
```

| 場面 | Repository が返したもの | AppError | 利用者が見るもの |
|---|---|---|---|
| ① 自分のノート | `Ok(Some(note))` | （成功） | 200 とノート |
| ② 他人のノート | `Ok(None)` | `NotFound` | 404 |
| ③ 正しい version | `Ok(note)`（version 4） | （成功） | 200 |
| ④ 古い version | `Err(VersionMismatch)` | `Conflict` | 409「読み込み直してください」 |
| ⑤ 壊れた行 | `Err(InvalidData(…))` | `Internal` | 500。原因はログにだけ |

#### ▼ コードを1つずつ分解して解説

##### 解説1: #[async_trait] で、async fn の trait を dyn で使えるようにする

- trait のメソッドを `async fn` で書くことは、今の Rust でもできます。ただし、そのままでは `dyn NoteRepository`（実行するときに中身の型を選ぶ使い方）にできません（よくあるエラー1）。
- `#[async_trait]` を trait と `impl` の**両方**に付けると、`async fn` が「`Box` に包んだ Future を返す普通のメソッド」に書き換えられ、`dyn` で使えるようになります。
- `Arc<dyn NoteRepository + Send + Sync>` は、「`NoteRepository` を守る何かを、複数のスレッドから共有して使う」という型です。RustNote の `AppState` の `notes` も、この形です。

##### 解説2: rename は「0 行更新」の理由を見分ける

- `UPDATE … WHERE id = ?2 AND user_id = ?3 AND version = ?4 RETURNING …` は、UNIT 05 の楽観的ロックです。version が合えば、更新した行が `RETURNING` で返ってきます。
- 1行も返ってこなかったときは、理由が2つ考えられます。ノートが無い（または他人のもの）か、version が古いかです。そこで `find` で探し直し、見つかれば `VersionMismatch`、無ければ `NotFound` にしています。

##### 解説3: ハンドラーは SQL を知らない

- `show_note`・`rename_note` の引数は `&(dyn NoteRepository + Send + Sync)` で、`LibsqlNoteRepository` という名前すら出てきません。使っているのは `find`・`rename` という約束だけです。
- `repo.find(owner, id).await?` の `?` は、`RepoError` を `AppError` に変えます（次の `From`）。`.ok_or(AppError::NotFound)?` で、`None` を 404 にしています。
- `main` の `&*repo` は、`Arc` の中身を借りて `&dyn NoteRepository` として渡す書き方です。

##### 解説4: From<RepoError> for AppError が、返事の種類を決める

- `VersionMismatch` は `Conflict`（409）、`NotFound` は `NotFound`（404）、それ以外は全部 `Internal`（500）にします。
- `Internal(#[source] RepoError)` は、元の `RepoError` を「原因」として中に持っています。利用者に見せる文は `サーバーの中でエラーが起きました` だけで、原因は `error.source()` で取り出してログに書きます。DB の内部の事情（表や列の名前）を、利用者の画面に出さないためです。

##### 解説5: 壊れた行は 500 にし、場所をログに残す

- ⑤では、実験のために `PRAGMA ignore_check_constraints = ON` で `CHECK` 制約を一時的に無効にして、タイトルが空の行を作りました。**本物のアプリでは使ってはいけない命令**です。
- `row_to_note` がこの行を見つけ、`notes.title (id=4)` と場所を添えた `InvalidData` を返しました。利用者には 500、ログには「どの行のどの列が、なぜおかしいか」が残るので、管理者は #4 の行を直せば済みます。
- 壊れた値をそのまま画面に出したり、`unwrap` で panic したりせず、**境界（DB から読んだところ）で検証して、分かる失敗に変える**のが、この単元の大事な考え方です。

#### 例9-3: N+1 を SQL の回数で見つけて、1回にまとめる

管理画面で「会員ごとのノートの数と、完了したノートの数」を一覧にする場面を考えます。会員の一覧を読み、1人ずつノートを数えると、会員が3人なら 1 + 3 = 4 回、1,000 人なら 1,001 回、DB と往復します。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-092" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="170" y="22" font-size="14" font-weight="700" fill="#b91c1c" text-anchor="middle">N+1（1 + 会員の数 回）</text>
<text x="560" y="22" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">JOIN（1 回）</text>
<text x="50" y="46" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">アプリ</text>
<text x="290" y="46" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">DB</text>
<line x1="50" y1="54" x2="50" y2="226" stroke="#cbd5e1" stroke-width="3"/>
<line x1="290" y1="54" x2="290" y2="226" stroke="#cbd5e1" stroke-width="3"/>
<line x1="54" y1="72" x2="284" y2="72" stroke="#f97316" stroke-width="2" marker-end="url(#ar-07-092)"/>
<text x="170" y="66" font-size="10" fill="#c2410c" text-anchor="middle">① 会員の一覧（users）</text>
<line x1="286" y1="86" x2="56" y2="86" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 3" marker-end="url(#ar-07-092)"/>
<line x1="54" y1="112" x2="284" y2="112" stroke="#f97316" stroke-width="2" marker-end="url(#ar-07-092)"/>
<text x="170" y="106" font-size="10" fill="#c2410c" text-anchor="middle">② 会員 1 のノートを数える</text>
<line x1="286" y1="126" x2="56" y2="126" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 3" marker-end="url(#ar-07-092)"/>
<line x1="54" y1="152" x2="284" y2="152" stroke="#f97316" stroke-width="2" marker-end="url(#ar-07-092)"/>
<text x="170" y="146" font-size="10" fill="#c2410c" text-anchor="middle">③ 会員 2 のノートを数える</text>
<line x1="286" y1="166" x2="56" y2="166" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 3" marker-end="url(#ar-07-092)"/>
<line x1="54" y1="192" x2="284" y2="192" stroke="#f97316" stroke-width="2" marker-end="url(#ar-07-092)"/>
<text x="170" y="186" font-size="10" fill="#c2410c" text-anchor="middle">④ 会員 3 のノートを数える</text>
<line x1="286" y1="206" x2="56" y2="206" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 3" marker-end="url(#ar-07-092)"/>
<text x="170" y="252" font-size="12" fill="#b91c1c" text-anchor="middle">会員が 1,000 人なら 1,001 回の往復</text>
<line x1="360" y1="40" x2="360" y2="240" stroke="#e2e8f0" stroke-width="1.5"/>
<text x="440" y="46" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">アプリ</text>
<text x="680" y="46" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">DB</text>
<line x1="440" y1="54" x2="440" y2="226" stroke="#cbd5e1" stroke-width="3"/>
<line x1="680" y1="54" x2="680" y2="226" stroke="#cbd5e1" stroke-width="3"/>
<line x1="444" y1="112" x2="674" y2="112" stroke="#10b981" stroke-width="2" marker-end="url(#ar-07-092)"/>
<text x="560" y="84" font-size="10" fill="#047857" text-anchor="middle">users LEFT JOIN notes</text>
<text x="560" y="98" font-size="10" fill="#047857" text-anchor="middle">GROUP BY で会員ごとに数える</text>
<line x1="676" y1="138" x2="446" y2="138" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 3" marker-end="url(#ar-07-092)"/>
<text x="560" y="156" font-size="10" fill="#475569" text-anchor="middle">全員分の結果がまとめて返る</text>
<text x="560" y="252" font-size="12" fill="#047857" text-anchor="middle">会員が何人でも 1 回の往復</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図7-?: N+1 と JOIN の往復の回数。N+1 は会員の数だけ SQL が増えるが、JOIN と GROUP BY なら何人でも1回で済む</div>
</div>

> **▼ このコードがやること（先に日本語で）:** 送った SQL の回数を数える小さな包み紙 `CountingConn` を作ります。会員ごとの集計を、N+1 の書き方と `LEFT JOIN` ＋ `GROUP BY` の書き方の両方で作り、SQL の回数と結果を比べます。続けて、「最近見たノート」のように決まった id の集まりを `IN` で1回で読みます。最後に会員を 1,000 人増やして、2つの書き方の回数と時間を比べます。

```rust
use std::cell::Cell;
use std::time::Instant;

use libsql::params::IntoParams;
use libsql::{Builder, Connection, Rows};

// 投げた SQL の回数を数える接続（実験用の包み紙）
struct CountingConn {
    conn: Connection,
    count: Cell<usize>,
}

impl CountingConn {
    async fn query(&self, sql: &str, params: impl IntoParams) -> Result<Rows, libsql::Error> {
        self.count.set(self.count.get() + 1);
        self.conn.query(sql, params).await
    }

    // 回数を 0 に戻し、それまでの回数を返す
    fn take_count(&self) -> usize {
        self.count.replace(0)
    }
}

// 会員ごとの集計（管理画面の1行分）
#[derive(Debug, PartialEq)]
struct Summary {
    email: String,
    notes: i64,
    done: i64,
}

// ❌ N+1: 会員の一覧で1回 + 会員1人ごとに1回
async fn summaries_n_plus_1(db: &CountingConn) -> Result<Vec<Summary>, libsql::Error> {
    let mut users = db
        .query("SELECT id, email FROM users ORDER BY id", ())
        .await?;
    let mut list = Vec::new();
    while let Some(user) = users.next().await? {
        let id = user.get::<i64>(0)?;
        // ループの中で SQL を投げている（ここが N 回）
        let mut rows = db
            .query(
                "SELECT COUNT(*), COALESCE(SUM(done), 0) FROM notes WHERE user_id = ?1",
                [id],
            )
            .await?;
        let row = rows.next().await?.expect("COUNT は必ず1行を返す");
        list.push(Summary {
            email: user.get::<String>(1)?,
            notes: row.get::<i64>(0)?,
            done: row.get::<i64>(1)?,
        });
    }
    Ok(list)
}

// ✅ JOIN + GROUP BY: 1回で全員分
async fn summaries_join(db: &CountingConn) -> Result<Vec<Summary>, libsql::Error> {
    let mut rows = db
        .query(
            "SELECT u.email, COUNT(n.id), COALESCE(SUM(n.done), 0)
               FROM users AS u
               LEFT JOIN notes AS n ON n.user_id = u.id
              GROUP BY u.id
              ORDER BY u.id",
            (),
        )
        .await?;
    let mut list = Vec::new();
    while let Some(row) = rows.next().await? {
        list.push(Summary {
            email: row.get::<String>(0)?,
            notes: row.get::<i64>(1)?,
            done: row.get::<i64>(2)?,
        });
    }
    Ok(list)
}

// ✅ IN: 決まった id の集まりを1回で読む（「最近見たノート」など）
async fn titles_by_ids(
    db: &CountingConn,
    owner: i64,
    ids: &[i64],
) -> Result<Vec<(i64, String)>, libsql::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    // ?2, ?3, ?4 … を、id の数だけ作る（値そのものは SQL に書き込まない）
    let marks: Vec<String> = (0..ids.len()).map(|i| format!("?{}", i + 2)).collect();
    let sql = format!(
        "SELECT id, title FROM notes WHERE user_id = ?1 AND id IN ({}) ORDER BY id",
        marks.join(", ")
    );
    // 値は ?1 = 持ち主、?2 以降 = id の順に並べて渡す
    let mut values = vec![libsql::Value::Integer(owner)];
    values.extend(ids.iter().map(|&id| libsql::Value::Integer(id)));
    println!("  SQL: {sql}");
    let mut rows = db.query(&sql, values).await?;
    let mut list = Vec::new();
    while let Some(row) = rows.next().await? {
        list.push((row.get::<i64>(0)?, row.get::<String>(1)?));
    }
    Ok(list)
}

#[tokio::main]
async fn main() -> Result<(), libsql::Error> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;
    conn.execute_batch(include_str!("../../sql/schema.sql"))
        .await?;
    conn.execute_batch(include_str!("../../sql/seed.sql"))
        .await?;
    let db = CountingConn {
        conn,
        count: Cell::new(0),
    };

    println!("--- 会員 3 人で比べる ---");
    let a = summaries_n_plus_1(&db).await?;
    println!("N+1 版: SQL {} 回", db.take_count());
    let b = summaries_join(&db).await?;
    println!("JOIN 版: SQL {} 回", db.take_count());
    for s in &b {
        println!("  {} ノート {} 件（完了 {} 件）", s.email, s.notes, s.done);
    }
    println!("結果は同じ？ {}", a == b);

    println!("--- IN で「最近見たノート」をまとめて読む ---");
    let recent = titles_by_ids(&db, 1, &[6, 1, 4]).await?;
    println!("  {recent:?}（SQL {} 回）", db.take_count());

    println!("--- 会員を 1,000 人増やして比べる ---");
    db.conn
        .execute_batch(
            "WITH RECURSIVE seq(i) AS (SELECT 1 UNION ALL SELECT i + 1 FROM seq WHERE i < 1000)
             INSERT INTO users (email, password_hash)
             SELECT 'user' || i || '@example.com', 'dummy' FROM seq;
             INSERT INTO notes (user_id, title, done)
             SELECT u.id, 'メモ', u.id % 2 FROM users AS u, (SELECT 1 UNION ALL SELECT 2) WHERE u.id > 3;",
        )
        .await?;
    let start = Instant::now();
    let a = summaries_n_plus_1(&db).await?;
    println!("N+1 版: SQL {} 回、{:?}", db.take_count(), start.elapsed());
    let start = Instant::now();
    let b = summaries_join(&db).await?;
    println!("JOIN 版: SQL {} 回、{:?}", db.take_count(), start.elapsed());
    println!("会員 {} 人分。結果は同じ？ {}", b.len(), a == b);
    Ok(())
}
```

`src/bin/u09_n_plus_1.rs` として保存して実行します。

```powershell
cargo run -q --bin u09_n_plus_1
```

**▼ 実行結果**（時間は実行のたびに変わります）

```text
--- 会員 3 人で比べる ---
N+1 版: SQL 4 回
JOIN 版: SQL 1 回
  alice@example.com ノート 5 件（完了 1 件）
  bob@example.com ノート 3 件（完了 1 件）
  carol@example.com ノート 0 件（完了 0 件）
結果は同じ？ true
--- IN で「最近見たノート」をまとめて読む ---
  SQL: SELECT id, title FROM notes WHERE user_id = ?1 AND id IN (?2, ?3, ?4) ORDER BY id
  [(1, "買い物リスト"), (4, "読書メモ"), (6, "SQL の練習")]（SQL 1 回）
--- 会員を 1,000 人増やして比べる ---
N+1 版: SQL 1004 回、268.5556ms
JOIN 版: SQL 1 回、6.5576ms
会員 1003 人分。結果は同じ？ true
```

結果はまったく同じなのに、会員が 1,003 人になると、N+1 版は SQL を 1,004 回送り、JOIN 版の約40倍の時間がかかりました。

#### ▼ コードを1つずつ分解して解説

##### 解説1: CountingConn で「SQL の回数」を見える化する

- `CountingConn` は、本物の `Connection` を包んで、`query` が呼ばれるたびに `count` を1つ増やすだけの部品です。
- `Cell<usize>` は、`&self`（書き換えられない借用）からでも中の数を書き換えられる入れ物です。数を数えるだけの、この実験用の部品にはちょうどよい道具です。
- 「遅い」と感じたとき、まず**1回の画面表示で SQL を何回送っているか**を数えるのが、N+1 を見つける近道です。件数に比例して回数が増えていたら、N+1 を疑います。

##### 解説2: N+1 はループの中の SQL から生まれる

- `summaries_n_plus_1` は、`while let Some(user) = users.next().await?` のループの中で、もう一度 `db.query(…)` を呼んでいます。会員が増えるほど、SQL の回数も増えます。
- コードとしては読みやすく、会員が3人なら困りません。データが増えてから急に遅くなるので、気づくのが遅れがちです。

##### 解説3: LEFT JOIN ＋ GROUP BY で、全員分を1回で数える

- `users AS u LEFT JOIN notes AS n ON n.user_id = u.id` で会員とノートをつなぎ、`GROUP BY u.id` で会員ごとにまとめて数えます（UNIT 02 の `JOIN`・集計関数）。
- `LEFT JOIN` なので、ノートが 0 件の carol も残ります。carol の行では `n.id` も `n.done` も `NULL` なので、`COUNT(n.id)` は 0、`SUM(n.done)` は `NULL` です。`COALESCE(…, 0)` で `NULL` を 0 に置き換えています。
- `JOIN` と書いたり、`COUNT(*)` と書いたりすると、結果が変わります（よくあるエラー2）。

##### 解説4: IN の ? は、値の数だけ作って bind する

- `(0..ids.len()).map(|i| format!("?{}", i + 2))` で `?2, ?3, ?4` を作り、`IN (…)` に並べます。SQL に埋め込むのは**場所取りの記号だけ**で、id の値そのものは書き込みません（UNIT 07 の bind の約束）。
- 値は `Vec<libsql::Value>` にまとめ、`?1` に持ち主、`?2` 以降に id の順で並べて渡します。数が決まっていない値の並びは、`params!` ではなく `Vec<Value>` で渡せます。
- `ids` が空のときは `IN ()` という正しくない SQL になるので、最初に `return Ok(Vec::new())` で抜けています。

##### 解説5: 往復の回数は、リモートではもっと効く

- この実験は、メモリの中の DB（ネットを使わない）で、debug ビルドでの結果です。それでも 1,004 回の往復は、1回の JOIN の約40倍かかりました。
- UNIT 08 のリモート（Turso）では、SQL を1回送るたびにネットを往復します。N+1 の1回ごとにその待ち時間が足されるので、差はもっと大きくなります。

### よくあるエラーと読み方

#### エラー1: E0038 — async fn の trait を dyn で使おうとした

`#[async_trait]` を付けずに、`async fn` を持つ trait を `dyn` で使おうとしたときのエラーです。

```rust
use std::sync::Arc;

struct Note {
    title: String,
}

// async fn をそのまま trait に書いた（#[async_trait] なし）
trait NoteRepository {
    async fn find(&self, id: i64) -> Option<Note>;
}

struct MemoryRepo;

impl NoteRepository for MemoryRepo {
    async fn find(&self, id: i64) -> Option<Note> {
        (id == 1).then(|| Note {
            title: "買い物リスト".to_string(),
        })
    }
}

#[tokio::main]
async fn main() {
    // 実装を dyn（trait の型）として持とうとする
    let repo: Arc<dyn NoteRepository + Send + Sync> = Arc::new(MemoryRepo);
    if let Some(note) = repo.find(1).await {
        println!("{}", note.title);
    }
}
```

```text
error[E0038]: the trait `NoteRepository` is not dyn compatible
  --> src\bin\u09_err_dyn.rs:25:23
   |
25 |     let repo: Arc<dyn NoteRepository + Send + Sync> = Arc::new(MemoryRepo);
   |                       ^^^^^^^^^^^^^^ `NoteRepository` is not dyn compatible
   |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
  --> src\bin\u09_err_dyn.rs:9:14
   |
 8 | trait NoteRepository {
   |       -------------- this trait is not dyn compatible...
 9 |     async fn find(&self, id: i64) -> Option<Note>;
   |              ^^^^ ...because method `find` is `async`
   = help: consider moving `find` to another trait
   = help: only type `MemoryRepo` implements `NoteRepository`; consider using it directly instead.

For more information about this error, try `rustc --explain E0038`.
error: could not compile `sql-lab` (bin "u09_err_dyn") due to 1 previous error
```

- **何が起きたか:** 「`NoteRepository` は dyn で使えない（dyn compatible でない）」と言われました。理由は `...because method `find` is `async``、つまり `find` が `async fn` だからです。
- **読み方:** `vtable`（ブイテーブル）は、`dyn` で使うときに「どの実装の関数を呼ぶか」を引くための表です。`async fn` は、実装ごとに違う型の Future を返すので、この表を作れません。
- **直し方:** trait と `impl` の両方に `#[async_trait]` を付けます（例9-2 の形）。`help:` の「`MemoryRepo` を直接使う」も1つの方法ですが、RustNote のように `AppState` に実装を入れておき、ハンドラーを変えずに差し替えられるようにするには `dyn` が便利です。

#### エラー2: JOIN と COUNT(*) で、集計の結果が変わる

N+1 を JOIN に直すときに起きやすい間違いを、道具 `sql` で確かめます。

```sql
-- ① N+1 を直すつもりで、LEFT JOIN ではなく JOIN と書いてしまった
SELECT u.email, COUNT(n.id) AS notes, COALESCE(SUM(n.done), 0) AS done
  FROM users AS u
  JOIN notes AS n ON n.user_id = u.id
 GROUP BY u.id
 ORDER BY u.id;

-- ② LEFT JOIN にしたが、COUNT(n.id) ではなく COUNT(*) と書いてしまった
SELECT u.email, COUNT(*) AS notes, COALESCE(SUM(n.done), 0) AS done
  FROM users AS u
  LEFT JOIN notes AS n ON n.user_id = u.id
 GROUP BY u.id
 ORDER BY u.id;
```

```powershell
cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u09_inner_join.sql
```

```text
（準備: sql/schema.sql の 2 文を実行しました）
（準備: sql/seed.sql の 2 文を実行しました）
sql> SELECT u.email, COUNT(n.id) AS notes, COALESCE(SUM(n.done), 0) AS done …
+-------------------+-------+------+
| email             | notes | done |
+-------------------+-------+------+
| alice@example.com | 5     | 1    |
| bob@example.com   | 3     | 1    |
+-------------------+-------+------+
(2 行)

sql> SELECT u.email, COUNT(*) AS notes, COALESCE(SUM(n.done), 0) AS done …
+-------------------+-------+------+
| email             | notes | done |
+-------------------+-------+------+
| alice@example.com | 5     | 1    |
| bob@example.com   | 3     | 1    |
| carol@example.com | 1     | 0    |
+-------------------+-------+------+
(3 行)
```

- **何が起きたか:** ①は、ノートが 0 件の carol が**一覧から消えました**。`JOIN`（INNER JOIN）は、相手（ノート）がいない会員の行を残さないからです。②は carol が残りましたが、ノートが **1 件**と数えられました。`COUNT(*)` は「行の数」を数えるので、ノートの列が全部 `NULL` の carol の1行も数えてしまいます。
- どちらも**エラーにならない**ので、N+1 版と結果を比べないと気づけません。例9-3 で `a == b` を表示していたのは、このためです。
- **直し方:** 相手がいない行も残したいときは `LEFT JOIN`、数えるのは `COUNT(*)` ではなく `COUNT(n.id)`（相手の主キー）にします。

#### エラー3: 列の順番の取り違えは、型が同じだとエラーにならない

`SELECT` の列の順番を変えたのに、読む側の番号を直し忘れた場面です。

```rust
// SELECT の列の順番を変えたのに、読む側の番号を直し忘れた
let mut rows = conn
    .query(
        "SELECT user_id, id, title FROM notes WHERE user_id = ?1 ORDER BY id DESC",
        [1],
    )
    .await?;
while let Some(row) = rows.next().await? {
    // 読む側は「0 = id、1 = user_id」のつもりのまま
    let id = row.get::<i64>(0)?;
    let owner = row.get::<i64>(1)?;
    let title = row.get::<String>(2)?;
    println!("#{id} {title}（持ち主 {owner}）");
}
```

（`src/bin/u09_err_columns.rs`。前半は例9-1 と同じく、メモリの DB に `schema.sql` と `seed.sql` を流しています。）

```text
#1 100% 達成（持ち主 7）
#1 SQL の練習（持ち主 6）
#1 読書メモ（持ち主 4）
#1 Rust の勉強（持ち主 2）
#1 買い物リスト（持ち主 1）
```

- **何が起きたか:** id と持ち主が入れ替わり、全部のノートが「#1」で、持ち主が 7・6・4… という、でたらめな結果になりました。`user_id` も `id` も整数なので、`get::<i64>` は**どちらも成功してしまい**、エラーも panic も起きません（型が違えば、UNIT 07 のエラー4 のように panic します）。
- このまま「#1 を編集」のリンクを作ると、全部のノートが #1 を指してしまいます。
- **直し方:** 例9-1 のように、読む列を `NOTE_COLUMNS` の1か所に書き、それを読む関数も `row_to_note` の1つだけにします。さらに、テストで「読んだノートの id と持ち主が、入れたものと同じか」を確かめておくと、取り違えをすぐ見つけられます。

#### エラー4: IN に並べる ? が多すぎる

`IN` の `?` は、いくつでも並べられるわけではありません。

```rust
for count in [32766, 32767] {
    // ?1, ?2, … を count 個作り、id として 1, 2, … を渡す
    let marks: Vec<String> = (1..=count).map(|i| format!("?{i}")).collect();
    let sql = format!(
        "SELECT COUNT(*) FROM notes WHERE id IN ({})",
        marks.join(", ")
    );
    let values: Vec<Value> = (1..=count).map(|i| Value::Integer(i64::from(i))).collect();
    match conn.query(&sql, values).await {
        Ok(mut rows) => {
            let row = rows.next().await?.expect("COUNT は必ず1行を返す");
            println!("{count} 個: 見つかったノート {} 件", row.get::<i64>(0)?);
        }
        Err(e) => println!("{count} 個: エラー: {e}"),
    }
}
```

（`src/bin/u09_err_in_limit.rs`。`use libsql::{Builder, Value};` で、前半は例9-1 と同じ準備です。）

```text
32766 個: 見つかったノート 8 件
32767 個: エラー: SQLite failure: `variable number must be between ?1 and ?32766`
```

- **何が起きたか:** 「`?` の番号は `?1` から `?32766` まで」と言われました。この本の libsql 0.10（SQLite 3.47.0）では、1つの SQL で使える `?` は 32,766 個までです。
- **直し方:** 数がとても多くなりうるときは、たとえば 500 個ずつに分けて送ります。そもそも何万個もの id をアプリから渡す場面は、`IN` の中に `SELECT` を書く（`WHERE id IN (SELECT note_id FROM …)`）か、`JOIN` で DB の中だけで済ませられないかを考えましょう。

### RustNote ではここで使う

RustNote のリポジトリは、表ごとに「約束（trait）」と「libSQL の実装」の組で置いてあります（`Rust/rustnote/src/repository.rs` と `src/repository/` の下）。

| ファイル | trait | 実装 |
|---|---|---|
| `notes.rs` | `NoteRepository` | `LibsqlNoteRepository` |
| `users.rs` | `UserRepository` | `LibsqlUserRepository` |
| `sessions.rs` | `SessionRepository` | `LibsqlSessionRepository` |

ページの処理（`src/routes/`）は trait だけを知っていて、`AppState` の `notes: Arc<dyn NoteRepository + Send + Sync>` を通して使います。例9-2 と同じ形です。

`notes.rs` の読む列と行の変換は、次のとおりです。

```rust
/// SELECT や RETURNING で読む列（row_to_note の順番と合わせる）
const NOTE_COLUMNS: &str = "id, user_id, title, body, done, version, created_at, updated_at";
```

```rust
fn row_to_note(row: &Row) -> Result<Note, RepoError> {
    let id = NoteId::new(row.get::<i64>(0)?).map_err(|e| invalid_data("notes.id", e))?;
    let owner = UserId::new(row.get::<i64>(1)?).map_err(|e| invalid_data("notes.user_id", e))?;
    Ok(Note {
        id,
        owner,
        title: row.get::<String>(2)?,
        body: row.get::<String>(3)?,
        done: row.get::<i64>(4)? != 0,
        version: row.get::<i64>(5)?,
        created_at: row.get::<String>(6)?,
        updated_at: row.get::<String>(7)?,
    })
}
```

- 8つの列を `NOTE_COLUMNS` に1か所だけ書き、`row_to_note` が同じ順番で読みます。`invalid_data("notes.id", e)` は、`RepoError::InvalidData("notes.id: …")` を作る小さな関数で、例9-1 と同じく「どの列か」を添えています。
- 例9-1 との違いは、読んだ `title` を `NoteTitle` ではなく `String` のまま持っていることです。書き込むときは `NewNote`（`title: NoteTitle`）で必ず検証し、DB の `CHECK` 制約も 1〜80 文字を守らせているので、読むときは id と持ち主だけを型で確かめる、という分担にしています。
- どのメソッドも、`WHERE` に持ち主の条件 `user_id = ?` を入れています。他人のノートは `None` になり、ハンドラーが 404 にします。

`src/error.rs` の変換も、例9-2 と同じ分類です。

```rust
impl From<RepoError> for AppError {
    fn from(error: RepoError) -> Self {
        match error {
            RepoError::VersionMismatch => AppError::Conflict,
            RepoError::NotFound => AppError::NotFound,
            other => AppError::Internal(Box::new(other)),
        }
    }
}
```

- `Internal` のときは、画面には「時間をおいてもう一度お試しください」とだけ出し（`public_message`）、原因のつながりは `src/views.rs` の `log_if_internal` が `[error] …` の形でログに書きます。
- ノートの一覧（`search`）は、「件数を数える SQL」と「そのページの分を読む SQL」の **2回だけ**です。ノートが何件あっても回数は変わらないので、N+1 になっていません。

### 練習問題

**問題1:** 例9-1 の `NoteRepo` に、持ち主のノートの数を返すメソッド `async fn count_by_owner(&self, owner: UserId) -> Result<i64, RepoError>` を足し、`main` の最後で Alice・Bob・Carol（会員 1・2・3）の数を表示してください。

**問題2:** 次の関数は、新しいノート5件を「タイトル（作者のメールアドレス）」の形で返しますが、N+1 になっています。`JOIN` を使って1回の SQL に直し、例9-3 の `CountingConn` で、直す前と後の SQL の回数を比べてください。

```rust
// ❌ 問題のコード: ノートの一覧で1回 + ノート1件ごとに作者を1回
async fn latest_n_plus_1(db: &CountingConn) -> Result<Vec<(String, String)>, libsql::Error> {
    let mut notes = db
        .query(
            "SELECT user_id, title FROM notes ORDER BY id DESC LIMIT 5",
            (),
        )
        .await?;
    let mut list = Vec::new();
    while let Some(note) = notes.next().await? {
        let mut users = db
            .query(
                "SELECT email FROM users WHERE id = ?1",
                [note.get::<i64>(0)?],
            )
            .await?;
        let user = users.next().await?.expect("作者は必ずいる");
        list.push((note.get::<String>(1)?, user.get::<String>(0)?));
    }
    Ok(list)
}
```

**問題3:** 例9-2 では、Alice が Bob のノート #3 を表示しようとすると 404「ページが見つかりません」になりました。「あなたにはこのノートを見る権限がありません」（403 Forbidden）にしないのはなぜでしょうか。

### 解答と解説

**解答1:** `NoteRepo` の `impl` に、次のメソッドを足します。

```rust
// 自分のノートの数を返す。0 件でも 0 を返す
async fn count_by_owner(&self, owner: UserId) -> Result<i64, RepoError> {
    let mut rows = self
        .conn
        .query("SELECT COUNT(*) FROM notes WHERE user_id = ?1", [owner.0])
        .await?;
    match rows.next().await? {
        Some(row) => Ok(row.get::<i64>(0)?),
        None => Ok(0),
    }
}
```

`main` の最後（`Ok(())` の前）には、次を足します。

```rust
println!("--- ノートの数 ---");
for (name, owner) in [
    ("Alice", UserId(1)),
    ("Bob", UserId(2)),
    ("Carol", UserId(3)),
] {
    println!("{name}: {} 件", repo.count_by_owner(owner).await?);
}
```

```powershell
cargo run -q --bin u09_q1
```

```text
--- Alice のノート一覧 ---
[  ] #7 100% 達成（持ち主 1）
[  ] #6 SQL の練習（持ち主 1）
[  ] #4 読書メモ（持ち主 1）
[済] #2 Rust の勉強（持ち主 1）
[  ] #1 買い物リスト（持ち主 1）
--- 1件だけ探す ---
Alice が #2 を探す: Some(Note { id: NoteId(2), owner: UserId(1), title: NoteTitle("Rust の勉強"), done: true })
Alice が #3（Bob のノート）を探す: None
--- ノートの数 ---
Alice: 5 件
Bob: 3 件
Carol: 0 件
```

- `COUNT(*)` は1行の数を返すだけなので、`row_to_note` は使いません。`row.get::<i64>(0)?` の `?` は `libsql::Error` を返すので、`Ok(…?)` と書いて `RepoError` に変換しています。
- `main` から見ると、`count_by_owner` も `list_by_owner` と同じ「頼むだけ」の使い方です。SQL を足したのは `NoteRepo` の中だけです。

**解答2:** `notes` と `users` を `JOIN` でつなぎ、1回で読みます。

```rust
// ✅ 直した版: JOIN で1回
async fn latest_join(db: &CountingConn) -> Result<Vec<(String, String)>, libsql::Error> {
    let mut rows = db
        .query(
            "SELECT n.title, u.email
               FROM notes AS n
               JOIN users AS u ON u.id = n.user_id
              ORDER BY n.id DESC
              LIMIT 5",
            (),
        )
        .await?;
    let mut list = Vec::new();
    while let Some(row) = rows.next().await? {
        list.push((row.get::<String>(0)?, row.get::<String>(1)?));
    }
    Ok(list)
}
```

`main` で両方を呼び、`take_count()` で回数を表示して、結果が同じかを比べます（`src/bin/u09_q2.rs`）。

```text
N+1 版: SQL 6 回
JOIN 版: SQL 1 回
  ジムの予定（bob@example.com）
  100% 達成（alice@example.com）
  SQL の練習（alice@example.com）
  買い物（bob@example.com）
  読書メモ（alice@example.com）
結果は同じ？ true
```

- N+1 版は、ノートの一覧で1回、ノート5件のそれぞれで1回ずつの、合わせて6回でした。
- ここでは `LEFT JOIN` ではなく `JOIN` で正しい答えになります。ノートには必ず作者がいる（`user_id` は `NOT NULL` で、外部キーで `users` を指している）ので、作者のいないノートの行が消える心配がないからです。「相手がいない行を残したいか」で、`JOIN` と `LEFT JOIN` を選びます。

**解答3:** 403 にすると、「#3 というノートは**存在する**が、あなたのものではない」と教えてしまうからです。

- 404 なら、「存在しない」と「他人のもの」の区別がつきません。番号を 1, 2, 3… と順に試されても、どの番号に他人のノートがあるかは分かりません。
- RustNote では、Repository の SQL に必ず持ち主の条件（`AND user_id = ?`）を入れているので、他人のノートは最初から「見つからない」（`None`）になります。ハンドラーがうっかり 403 を返す余地も、他人のノートの中身を読んでしまう余地もありません（UNIT 03 で学んだ形を、Repository で必ず守る作りです）。

### まとめ

- `notes` 表の SQL は Repository の中に集め、ハンドラーは trait の約束（`find`・`rename` など）だけを使います。ハンドラーを変えずに、別の実装に差し替えられます。
- 読む列は `NOTE_COLUMNS` の1か所に書き、`row_to_note` で同じ順番に読みます。変換に失敗したら、「表.列」と行の id を添えた `RepoError::InvalidData` にし、利用者には 500、ログには場所を残します。
- `async fn` を持つ trait を `dyn` で使うには `#[async_trait]` を付けます。`RepoError` は `From` で `AppError` に変え、`NotFound` は 404、`VersionMismatch` は 409、それ以外は 500 にします。
- ループの中で SQL を送ると N+1 になります。SQL の回数を数えて見つけ、`LEFT JOIN`・`GROUP BY`・`IN` で1回にまとめます。`JOIN` と `LEFT JOIN`、`COUNT(*)` と `COUNT(列)` の違いに気をつけ、直す前と後の結果を比べて確かめます。

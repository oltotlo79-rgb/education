## UNIT 09: Repository・行の変換・N+1

<!-- TODO: 未完。本文は未執筆。以下は構成案と、実行済みプログラム・実出力の記録（停止指示のため書き出し）。 -->
<!-- 実行済みのコード: scratchpad/rust07/sql-lab/src/bin/u09_repo.rs, u09_layers.rs, u09_n_plus_1.rs（rustfmt 済み・Rust 1.99.0 で実行確認）。エラー例: scratchpad/rust07/errs09/u09_err_dyn.rs -->
<!-- sql-lab/Cargo.toml に async-trait = "=0.1.92" を追加済み（cargo add async-trait@=0.1.92 → "Adding async-trait =0.1.92 to dependencies"） -->

### 構成案（TODO: 本文にする）

- ゴール: SQL を Repository に集める / NOTE_COLUMNS と row_to_note で行→ドメイン型、失敗は RepoError::InvalidData に「表.列 (id=..)」の文脈 / N+1 をクエリ回数で見つけ JOIN・IN でまとめる / RepoError→AppError（VersionMismatch→409、NotFound→404、他→500）
- たとえ話: 図書館のカウンター（利用者は棚の場所を知らない）、買い物を1品ずつ往復する（N+1）vs リストを渡す
- 図1: handler → trait NoteRepository → LibsqlNoteRepository → SQL → libSQL、行 → row_to_note → Note。図2: N+1（1+N 回の往復）と JOIN（1回）の比較
- まず動かす最小コード: u09_repo.rs（NoteRepo の list_by_owner / find、row_to_note で NoteId::new と NoteTitle::parse）
- もう一歩: u09_layers.rs（#[async_trait] の trait、Arc<dyn NoteRepository + Send + Sync>、SQL を知らないハンドラー関数、AppError への変換、壊れた行で 500）、u09_n_plus_1.rs（CountingConn で回数を数える、LEFT JOIN + GROUP BY、IN の ?2.. を数だけ作る）
- よくあるエラー: ① E0038（async fn の trait を dyn で使う）② INNER JOIN だとノート 0 件の carol が消える（LEFT JOIN にする。TODO: sql ツールで実出力を取る）③ 列の順番の取り違え（i64 同士だとエラーにならず id と持ち主が入れ替わる。型が違えば UNIT 07 の panic）④ IN の変数が多すぎる（?32767 以上）
- RustNote ではここで使う: Rust/rustnote/src/repository/notes.rs の NOTE_COLUMNS・row_to_note・invalid_data、src/error.rs の From<libsql::Error>（UNIQUE 文言で UniqueViolation）と From<RepoError> for AppError。RustNote の Note.title は String（DB の CHECK に任せ、id だけ NoteId::new / UserId::new で検査）。第4巻の基礎形との違い（rustnote-design-notes.md の7項目）を表で橋渡し
- 練習問題（TODO）: count_by_owner を足す / ノート一覧に作者メールを出す N+1 を JOIN に直して回数を確かめる / 他人のノートを 403 でなく 404 にする理由

### 実出力（実行済み）

u09_repo:

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

u09_layers（[log] の行は標準エラー）:

```text
① 自分の #2 を表示 → 200 #2 Rust の勉強（version 3）
② Bob の #3 を表示 → 404 ページが見つかりません
③ #2 を version 3 で更新 → 200 保存しました: Rust の復習（version 4）
④ #2 を古い version 3 で更新 → 409 ほかの画面で先に更新されました。読み込み直してください
⑤ 壊れた #4 を表示 → 500 サーバーの中でエラーが起きました
  [log] 原因: DB の値を読み取れません: notes.title (id=4): 1〜80 文字ではありません（0 文字）
```

u09_n_plus_1（時間は実行ごとに変わる）:

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
N+1 版: SQL 1004 回、259.1711ms
JOIN 版: SQL 1 回、4.1231ms
会員 1003 人分。結果は同じ？ true
```

E0038（errs09/u09_err_dyn.rs を src/bin に置いて cargo build）:

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

IN の変数の上限（実験用プログラムは削除済み。conn.query の Err の表示）:

```text
32766: ok Some(Ok(8))
32767: SQLite failure: `variable number must be between ?1 and ?32766`
40000: SQLite failure: `variable number must be between ?1 and ?32766`
```

列の取り違え（SELECT user_id, id, title, done を「id, user_id, …」のつもりで読む。エラーにならない）:

```text
1 7 100% 達成
1 6 SQL の練習
1 4 読書メモ
1 2 Rust の勉強
1 1 買い物リスト
```

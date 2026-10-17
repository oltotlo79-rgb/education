## UNIT 07: libsql Rust SDK の接続とパラメーターの bind

<!-- TODO: 未完。本文は未執筆。以下は構成案と、実行確認済みのコード・出力の所在だけ。 -->

<!--
構成案（このとおり書く予定だった）:
- ゴール: Builder → Database → Connection → execute / query の流れ、rows.next().await? / row.get::<T>(i)、NULL は Option、?1・params!・タプル・配列・named_params! で bind、libsql::Error の読み分け
- たとえ話: Database=図書館の建物、Connection=窓口、SQL=申込用紙、bind=記入欄に値を書く（インジェクション=名前欄に指示を書くと係員が指示として読んでしまう）
- 図1: Builder → Database → Connection → execute(u64) / query(Rows → Row → get::<T>)
- 図2: 文字列連結（入力が SQL の文法の一部になる）と bind（SQL の文と値が別々に届く）の比較
- 例7-1: src/bin/u07_first.rs（u07.db を消してから2回実行。出力: 1回目「追加した行数: 1 / 新しい id: 1」、2回目は id 2 と2行）
- 例7-2: u07_params.rs（出力は scratchpad/rust07/out/u07_params.txt と同じことを 2026-10-18 に再確認）
- 例7-3: u07_injection.rs（out/u07_injection.txt と同じことを再確認。"x' OR '1'='1" で3人分、"0 OR 1=1 --" で8件全部消える。bind 版は [] と 404 扱い）
- sql.rs の要所（prepare、stmt.columns().is_empty() で execute/query を分ける、get_value と Value の match）
- よくあるエラー:
  1. E0277 `!: FromValue`（errs/u07_ce_infer.rs → out/u07_ce_infer.txt）
  2. E0308 配列に型の違う値（errs/u07_ce_params.rs → out/u07_ce_params.txt）→ params! かタプル
  3. 実行時 Err: NullValue / 存在しない列も NullValue / InvalidColumnType / ExecuteReturnedRows（u07_err_get.rs、再確認済み）
  4. panic「internal error: entered unreachable code: invalid value type」（u07_err_type.rs、TEXT を i64 で get。Option<i64> でも panic することを u07_q3 で確認）
  5. bind の数の食い違いはエラーにならない: 足りない→NULL 扱い（UPDATE は Ok(0)、INSERT は NOT NULL 違反 1299）、多い→Ok(1) で黙って無視（u07_err_bind.rs を今回 ③ を追加して再実行済み）。対策 stmt.parameter_count()
  6. エラー番号の読み分け（新規 u07_err_codes.rs、実行済み）: 2067 UNIQUE / 1555 主キー重複 / 1299 NOT NULL / 275 CHECK / 787 FK / 1 構文。Debug は SqliteFailure(番号, 文)、Display は SQLite failure: `文`。リモートでは Hrana(...) になるので RustNote は文言 "UNIQUE constraint failed" で見分ける（src/error.rs）
- RustNote: Db::connect を操作ごと、libsql::params![owner.get(), ...]、RETURNING で id を受け取る、done は row.get::<i64>(4)? != 0、NOTE_COLUMNS の format! は定数だけなので安全
- 練習問題（解答は実行済み）:
  Q1 count_done(conn, owner)（u07_q1.rs）→ 会員 1: 1 件、会員 2: 1 件、会員 3: 0 件
  Q2 検索を bind に直す（u07_q2.rs）→ "買い物" → ["買い物リスト"]、"' OR 1=1 --" → []
  Q3 get の結果を予想（u07_q3.rs）→ (a) Ok("メモ") (b) Err(NullValue) (c) Ok(None) (d) Ok(true) (e) 'abc' を Option<i64> で → panic（invalid value type）
-->

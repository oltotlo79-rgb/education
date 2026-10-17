## UNIT 05: transaction・ACID・競合

<!-- TODO: 未完（本文は未執筆。以下は構成メモと検証済みの結果の置き場所） -->

<!--
構成メモ（停止時点）
- たとえ話: 買い物かごとレジ（BEGIN=かごに入れる、COMMIT=支払う、ROLLBACK=棚に戻す）。ACID を短く
- 図 ar-07-051: BEGIN→INSERT→ROLLBACK（8件のまま）/ COMMIT（9件）
- 図 ar-07-052: 接続 a（tx の中で 9 件が見える）と b（8 件）。a が書き込みの鍵 → b は database is locked、busy_timeout なら待つ
- 図 ar-07-053: 楽観的ロック（タブA・B が version 1 で開く→A 保存で v2→B は 0 行→409→読み直して v3）
- 例5-1: u05_tx.sql / 例5-2: u05_partial.sql（途中の文が失敗しても tx は続く。COMMIT すると1件目だけ確定）。出力 = out456/u05_*.txt（旧 out/ と一致）
- 例5-3: u05_drop.rs（drop でロールバック、is_autocommit、save_two の ? で自動取り消し）/ 例5-4: u05_locked.rs（SqliteFailure(5, "database is locked")、busy_timeout(5秒) で 328ms 待って Ok(1)。tokio::time は libsql 経由で time 機能が有効なので動く→ヒントで注記）/ 例5-5: u05_conflict.rs（Ok(2) → Err(VersionMismatch) → 409、他人のノート → 404）。再実行の出力は会話ログのとおり（警告は出ない）
- エラー: u05_errors.sql（cannot commit - no transaction is active / cannot start a transaction within a transaction / cannot rollback - no transaction is active）＋ database is locked
- RustNote: notes.rs update の WHERE … AND version = ?5 RETURNING、edit.rs で 409 と最新 version で再表示、db.rs の WAL と busy_timeout = 5000、migrate の tx
- 練習: Q1 sql/u05_q1.sql（答え 9）/ Q2 sql/u05_q2.sql（ROLLBACK で 0 行）/ Q3 sql/u05_q3.sql（変更 1 行 → 0 行 → 読み直して 1 行、version 3）
-->

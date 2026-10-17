## UNIT 06: migration と schema version

<!-- TODO: 未完（本文は未執筆。以下は構成メモと検証済みの結果の置き場所） -->

<!--
構成メモ（停止時点）
- たとえ話: 家の増改築の工事台帳（番号つきの工事と、済んだ工事の記録）
- 図 ar-07-061: migrations/0001〜0005 と schema_migrations の記録を比べ、未適用だけを1つずつ tx で適用
- 図 ar-07-062: 空の開発 DB では 0005 が通る（u06_fresh）が、行のある本番 DB では失敗（u06_run4）
- 図 ar-07-063: 表の作り直し（notes_new 作成→写す→DROP→RENAME→索引作り直し、全部 BEGIN…COMMIT の中）
- 例6-1: u06_migrate.rs（3件版 = out/u06_migrate_3.rs.txt。RustNote の db.rs と同じ）。手順と再実行結果（すべて旧 out/ と一致）: run1 3件適用 → run2 なし → u06_data.sql → setmig 4 → run3 0004 → u06_after4（pinned=0）→ setmig 5 → run4 失敗（Cannot add a NOT NULL column with default value NULL）→ u06_check（tags 無し=ロールバック、記録は 0004 まで）→ u06.db 削除で fresh は 5件とも成功
- 例6-2: u06_rebuild.sql（8 行写して CHECK length(body) <= 10000 が効く）
- エラー: u06_errors.sql（duplicate column name: pinned / Cannot add a NOT NULL column with default value NULL / DEFAULT 'none' で成功）＋ include_str! のパス間違い（couldn't read `src\bin\../migrations/0001_users.sql`: 指定されたパスが見つかりません。 (os error 3)、help で ../../ を提案。ファイルは削除済み）
- RustNote: db.rs の migrate、main.rs で起動時に適用して「マイグレーションを適用しました: …」。複数マシン同時起動の競合・Turso Cloud の PRAGMA user_version 読み取り専用・本番前のバックアップ（UNIT 10）
- 練習: Q1 0005 に DEFAULT 'none' を付けて直すと、行のある DB でも適用される（sql/u06_q1.sql で color = none を確認。0005_tags.sql は元に戻した）/ Q2 適用済みファイルを書き換えてはいけない理由 / Q3 版の名前を 0004_pinned に変えると再適用されて duplicate column name: pinned（確認済み、元に戻した）
- sql-lab の状態: u06_migrate.rs は 3件版に戻した。追加ファイル: sql/u04_size.sql, u04_q1.sql, u04_q2.sql, u05_q1〜q3.sql, u06_q1.sql、出力は scratchpad/rust07/out456/
-->

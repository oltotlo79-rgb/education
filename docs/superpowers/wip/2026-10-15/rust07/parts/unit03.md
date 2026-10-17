## UNIT 03: INSERT・UPDATE・DELETE と持ち主の条件

<!-- TODO: 未完（本文は未執筆。以下は構成メモ） -->
<!--
構成案（素材はすべて実行済み。出力は scratchpad/rust07/out/u03_*.txt、追加分は下記 SQL を --setup sql/schema.sql --setup sql/seed.sql で再実行）:
- たとえ話: 貸金庫（番号だけでなく本人確認がそろって初めて開く）。影響行数＝受付票
- 図 ar-07-031: Alice が /notes/3/edit を送る → NG: WHERE id = 3 で Bob のノートが書き換わる / OK: WHERE id = 3 AND user_id = 1 で 0 行 → アプリは 404
- 図 ar-07-032: 会員削除時の ON DELETE CASCADE（ノートも消える）と指定なし（FOREIGN KEY constraint failed）
- まず動かす: 例3-1 sql/u03_write.sql（INSERT … RETURNING で id 9 / version+1 で 2 / done = 1 - done で 1・version 3 / DELETE で found 0）
- もう一歩: 例3-2 sql/u03_owner.sql（変更 1 行 → 乗っ取り、持ち主条件つきは変更 0 行）、例3-3 sql/u03_cascade.sql（Bob 削除でノート 3 件も消える）、sql/u03_fk_delete.sql（--setup sql/u01_create.sql。ON DELETE 指定なしで FOREIGN KEY constraint failed）、例3-4 sql/u03_preview.sql（消す前に同じ WHERE で SELECT → 1 行 → DELETE 変更 1 行）
- よくあるエラー: sql/u03_errors.sql（UPDATE でも CHECK / NOT NULL notes.body / "2 values for 3 columns" / UPDATE の FK 違反 / WHERE 忘れで 8 行全部 done=1、エラーにならない）
- RustNote: notes.rs の delete（変更 0 行なら RepoError::NotFound → 404）、toggle_done（done = 1 - done, version + 1, updated_at = datetime('now') … RETURNING）、insert の RETURNING。version = ?5 の楽観的ロックは UNIT 05 で扱うと予告のみ
- 練習: sql/u03_q.sql（実行済み: Bob がノート 8 を完了 → RETURNING で done 1・version 2 / Alice が Bob のノート 5 を DELETE → 変更 0 行で残る / Alice が本文更新 RETURNING で version 2・updated_at は UTC の実行時刻 2026-10-17 21:35:21）
-->

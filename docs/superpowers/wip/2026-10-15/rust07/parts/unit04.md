## UNIT 04: index と EXPLAIN QUERY PLAN

<!-- TODO: 未完（本文は未執筆。以下は構成メモと検証済みの結果の置き場所） -->

<!--
構成メモ（停止時点）
- ゴール: 索引=並べ替えて別に持つ目次 / EXPLAIN QUERY PLAN で SCAN と SEARCH を見分ける / WHERE・ORDER BY に合わせた複合索引の列順 / 費用を実測で説明
- たとえ話: 料理本の巻末索引（SCAN=1ページ目から全部めくる、SEARCH=索引から直接飛ぶ）。名簿（組→出席番号）で列の順番
- 図 ar-07-041: SCAN（10万行を全部見る→TEMP B-TREE で並べ替え）と SEARCH（会員42の所へ直接→先頭20件で終わり）。会員42のノートは id 41, 1041, …, 99041
- 図 ar-07-042: (user_id, updated_at DESC, id DESC) は会員42が固まって並ぶ / (updated_at DESC, user_id) は散らばる
- 図 ar-07-043: 費用の棒グラフ（読み出し 10.4ms→0.049ms 約216倍 / 1,000件追加 13.8ms→18.3ms / 2055→2899ページ（4096B）約8.4MB→11.9MB）
- 例4-1: u04_plan.sql（--setup sql/schema.sql --setup sql/u04_big.sql）。出力 = scratchpad/rust07/out456/u04_plan.txt（旧 out/ と一致）
- 例4-2: u04_more.sql（+ --setup sql/u04_index.sql）5ケース: 先頭列なし→SCAN / user_id+LIKE→SEARCH(user_id=?) / email→COVERING INDEX sqlite_autoindex_users_1 / lower(email)→SCAN USING COVERING INDEX / COUNT→COVERING。RustNote の Email::parse は trim+to_lowercase 済みなので lower() 不要
- 例4-3: u04_timing.rs（再実行: 索引なし 10.4124ms / あり 48.766µs / 約216倍、追加 13.7967ms → 18.2773ms）＋ 新規 sql/u04_size.sql（page_size 4096, page_count 2055 → 2899）
- エラー: u04_errors.sql（already exists / no such column: usr_id / 列順逆→SCAN USING INDEX + USE TEMP B-TREE FOR LAST TERM OF ORDER BY / no such index: idx_nope）。直し方 IF NOT EXISTS / IF EXISTS
- RustNote: 0002 の idx_notes_user_updated が /notes の検索 SQL（user_id = ?1 AND (title LIKE … OR body LIKE …) ORDER BY updated_at DESC, id DESC LIMIT ?3 OFFSET ?4）に効く。email の UNIQUE 自動索引はログイン。sessions の idx_sessions_expires は delete_expired（expires_at <= ?1）、idx_sessions_user は ON DELETE CASCADE 用
- 練習: Q1 sql/u04_q1.sql（done=0 を足しても SEARCH … (user_id=?)）/ Q2 sql/u04_q2.sql（DELETE … expires_at <= … が SCAN → 索引後 SEARCH … (expires_at<?)。<= でも表示は < ）/ Q3 全列に索引の提案への意見（上の実測値で）
-->

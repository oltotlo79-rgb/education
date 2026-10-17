## UNIT 02: SELECT・WHERE・ORDER BY・LIMIT

<!-- TODO: 未完（本文は未執筆。以下は構成メモ） -->
<!--
構成案（素材はすべて実行済み。出力は scratchpad/rust07/out/u02_*.txt、追加分は下記コマンドで再取得できる）:
- 冒頭: sql/schema.sql と sql/seed.sql を全文提示し、--setup の使い方（cargo run -q --bin sql -- --setup sql/schema.sql --setup sql/seed.sql sql/u02_select.sql）を説明。seed の 8 ノート（Alice=1 が 5 件、Bob=2 が 3 件、Carol=3 は 0 件。5 と 8 は updated_at が同じ 2026-10-07 19:30:00。7 のタイトルに % 記号）
- たとえ話: 司書への注文書（FROM=棚、WHERE=条件、ORDER BY=並べ方、LIMIT=冊数、SELECT 列=写す項目）
- 図 ar-07-021: 書く順と処理の順（FROM→WHERE→SELECT→ORDER BY→LIMIT、行数 8→5→…→2）
- 図 ar-07-022: ページ送りと同点（Alice 5 件を 2 件ずつ: 7,6 / 4,2 / 1。Bob の 5 と 8 は同点 → id DESC で決める）
- 図 ar-07-023: JOIN と LEFT JOIN（Carol が LEFT JOIN だけに残り、相手は NULL）
- まず動かす: 例2-1 sql/u02_select.sql（out/u02_select.txt）
- もう一歩: 例2-2 sql/u02_more.sql（同点・LIMIT/OFFSET・LIKE・%%% の罠・ESCAPE '\'）、例2-3 sql/u02_join.sql（JOIN、LEFT JOIN + GROUP BY + COUNT(notes.id)）、例2-4 sql/u02_null.sql（LEFT JOIN + IS NULL で Carol、'Rust' LIKE 'rust' = 1 / = は 0、LIKE は英字の大小を区別しない）
- よくあるエラー: sql/u02_errors.sql（no such column: titel / no such table: note / ambiguous column name: id / NULL = NULL は NULL / "title" は列名扱いで全 8 行、"買い物" は列が無いので文字列扱いで 1 行）
- RustNote: src/repository/notes.rs の search（user_id = ?1 AND (title LIKE ?2 ESCAPE '\' OR body LIKE ?2 ESCAPE '\')、ORDER BY updated_at DESC, id DESC LIMIT ?3 OFFSET ?4、PER_PAGE=20、COUNT(*) で総数、escape_like）
- 練習: sql/u02_q.sql（実行済み: Alice の done=1 は 1 件 / Bob を created_at ASC, id ASC で 3,5,8 / Alice の 3 ページ目 LIMIT 2 OFFSET 4 で id 1 のみ）
-->

## この巻のまとめ

この巻では、ノートを DB に安全に預けるための道具を、SQL の基本から、安全に変える仕組み、Rust からの使い方の順にそろえました。次のチェックリストで、できるようになったことを確かめてください。自信がない項目は、括弧の中の単元に戻って、SQL やコードを動かし直しましょう。

**SQL で表を扱う**

- ☐ 表・行・列・主キー・外部キーの意味を、RustNote の `users` と `notes` で説明できる（UNIT 01）
- ☐ `CREATE TABLE` で `NOT NULL`・`UNIQUE`・`CHECK`・`DEFAULT`・`REFERENCES` を書き、制約違反のメッセージから破った約束を言える（UNIT 01）
- ☐ アプリの検証（`NoteTitle::parse`）と DB の制約を両方置く理由を説明できる。`STRICT` と `PRAGMA foreign_keys` の注意点を知っている（UNIT 01）
- ☐ `SELECT`・`WHERE`・`ORDER BY`・`LIMIT`・`OFFSET`・`COUNT(*)` で、必要な行を決まった順番で少しずつ読める（UNIT 02）
- ☐ 並べるときは最後に主キー（tie-breaker）を足し、ページ送りで行がずれないようにできる（UNIT 02）
- ☐ `LIKE` の `%`・`_` と `ESCAPE`、`IS NULL`、`'…'` と `"…"` の違い、`JOIN` と `LEFT JOIN` の違いを説明できる（UNIT 02）
- ☐ `INSERT`・`UPDATE`・`DELETE` と `RETURNING` を書き、影響行数で「対象が無かった」ことを見分けられる（UNIT 03）
- ☐ 書き換えの `WHERE` に持ち主の条件（`AND user_id = ?`）を入れ、他人のノートは 404 にできる（UNIT 03）

**速く・安全に変える**

- ☐ `EXPLAIN QUERY PLAN` で `SCAN` と `SEARCH` を見分け、`WHERE` と `ORDER BY` に合った複合索引を作れる（UNIT 04）
- ☐ 索引の費用（書き込みが遅くなる・DB が大きくなる）を、実測の数字で説明できる（UNIT 04）
- ☐ `BEGIN`・`COMMIT`・`ROLLBACK` で、複数の変更を「全部か無しか」にまとめられる。途中の文が失敗しても自動では取り消されないことを知っている（UNIT 05）
- ☐ libsql の `Transaction` が drop で自動的に取り消されることを使い、`?` で安全に途中で抜けられる（UNIT 05）
- ☐ `database is locked` と `busy_timeout`、`version` 列の楽観的ロックと 409 を説明できる（UNIT 05）
- ☐ `schema_migrations` で適用済みの版を記録し、未適用の migration だけを1つずつ transaction で適用するプログラムを読める（UNIT 06）
- ☐ 行がある表への列の追加と表の作り直しを、安全な手順で書ける。適用済みの migration を書き換えない（UNIT 06）

**Rust から使う**

- ☐ `Builder` → `Database` → `Connection` で DB を開き、`execute` と `query` を使い分けて、`row.get::<T>(i)` で値を受け取れる（UNIT 07）
- ☐ 値を `?1`・`params!`・`named_params!` で bind し、文字列連結が SQL インジェクションを生む理由を説明できる（UNIT 07）
- ☐ `NullValue`・`InvalidColumnType`・`ExecuteReturnedRows`・エラー番号を読み分けられる。型の取り違えは panic し、bind の数の食い違いはエラーにならないことを知っている（UNIT 07）

**置き場所を選ぶ**

- ☐ ローカル・リモート・組み込みレプリカの3つの形で、データの場所と読み書きの行き先を図で説明し、アプリを動かす場所に合わせて選べる（UNIT 08）
- ☐ read-your-writes と `sync` の関係を説明でき、ほかのサーバーの書き込みが `sync` まで見えないことを知っている（UNIT 08）
- ☐ DB の URL とトークンを環境変数から読み、トークンを `Secret` 型に包んで画面やログに出さない。足りない設定は起動時に止められる（UNIT 08）
- ☐ `os error 10061`・`Hrana`・`Sync is not supported`・E0599（features の不足）・debug ビルドのスタックあふれを見分けて直せる（UNIT 08）

**SQL を1か所に集める**

- ☐ 表の SQL を Repository に集め、ハンドラーが SQL を知らずに trait の約束だけで動くように書ける（UNIT 09）
- ☐ `NOTE_COLUMNS` と `row_to_note` で行を検証済みの型に変換し、失敗したら「表.列」と行の id を添えた `InvalidData` にできる（UNIT 09）
- ☐ `#[async_trait]` の trait を `Arc<dyn … + Send + Sync>` で持ち、`RepoError` を 404・409・500 に変換できる。E0038 の意味が分かる（UNIT 09）
- ☐ N+1 を SQL の回数で見つけ、`LEFT JOIN`・`GROUP BY`・`IN` で1回にまとめられる。`JOIN` と `LEFT JOIN`、`COUNT(*)` と `COUNT(列)` の違いを説明できる（UNIT 09）

> **お知らせ:** この版には UNIT 01〜09 を収録しています。UNIT 10（backup・token・監視・障害対応）は準備中で、次の更新で追加します。

次の第8巻では、この巻で作った `users` 表と `sessions` 表を使って、メールアドレスとパスワードによる登録・ログイン・ログアウト（認証）と、「その人がその操作をしてよいか」の確認（認可）を学びます。

## 用語ミニ辞典

この巻で出てきた主な用語です。

| 用語 | 読み・意味 | 出てくる単元 |
|---|---|---|
| データベース（DB） | データを整理して保存し、取り出せるようにする仕組み | 巻頭 |
| SQLite | エスキューライト。1つのファイルに表を保存する、小さな DB の仕組み | 巻頭 |
| libSQL | SQLite をもとに機能を足した、オープンソースの DB | 巻頭 |
| Turso | ターソ。libSQL をクラウドで動かすサービス | 巻頭 |
| SQL | エスキューエル。DB に表の作成や読み書きを頼むための言葉 | UNIT 01 |
| 表（テーブル） | 同じ形の行の集まり。`users`・`notes` など | UNIT 01 |
| 行・列 | 表の1件分の記入欄（行、レコード）と、その項目（列、カラム） | UNIT 01 |
| 主キー（PRIMARY KEY） | 行を1つに決める列。同じ値の行は存在できない | UNIT 01 |
| 外部キー（FOREIGN KEY） | ほかの表の主キーを指す列。指す先が実在しない値は入れられない | UNIT 01・03 |
| 制約 | `NOT NULL`・`UNIQUE`・`CHECK` など、DB が守らせる約束 | UNIT 01 |
| NULL | ヌル。「値が無い」を表す特別な印。0 や `''` とは別物で、`IS NULL` で比べる | UNIT 01・02 |
| ストレージクラス | SQLite の値の種類（integer・real・text・blob・null） | UNIT 01・07 |
| STRICT | 列の型と違う値を断るようにする、表の指定 | UNIT 01 |
| AUTOINCREMENT | 消した番号を使い回さない自動採番 | UNIT 01 |
| `PRAGMA` | DB の設定を読んだり変えたりする命令（`foreign_keys`・`busy_timeout` など） | UNIT 01・05 |
| クエリ | DB への問い合わせ。特に `SELECT` 文 | UNIT 02 |
| 集計関数 | `COUNT`・`SUM`・`MAX` など、たくさんの行を1つの値にまとめる関数 | UNIT 02 |
| tie-breaker | タイブレーカー。並べ替えで同点の行の順番を決める最後の列（主キー） | UNIT 02 |
| ページ送り | `LIMIT` と `OFFSET` で、一覧を少しずつ表示すること | UNIT 02 |
| ワイルドカード | `LIKE` の `%`（何文字でも）と `_`（1文字） | UNIT 02 |
| JOIN / LEFT JOIN | 2つの表をつないで読むこと。LEFT JOIN は相手がいない左の行も残す | UNIT 02 |
| RETURNING | `INSERT`・`UPDATE`・`DELETE` の後に、変わった行を返してもらう指定 | UNIT 03 |
| 影響行数 | 書き換えの文が実際に変えた行の数。0 なら「対象が無かった」 | UNIT 03・07 |
| 持ち主の条件 | `WHERE id = ? AND user_id = ?` のように、ログイン中の人の行だけを対象にする条件 | UNIT 03 |
| 認可 | ログインした人が、その操作をしてよいかを確かめること | UNIT 03 |
| ON DELETE CASCADE | 親の行を消すと、それを指す子の行も一緒に消す指定 | UNIT 03 |
| index（索引） | 列の値を並べ替えて、行の場所と一緒に持つ別のデータ。検索を速くする | UNIT 04 |
| 複合索引 | 複数の列を並べた索引。先頭の列から順にしか使えない | UNIT 04 |
| EXPLAIN QUERY PLAN | 文を実行せずに、DB がどう探すつもりか（計画）を見せる命令 | UNIT 04 |
| SCAN / SEARCH | 全部を見る／索引で目的の場所へ直接飛ぶ | UNIT 04 |
| COVERING INDEX | 索引だけで答えが出せたので、表を見に行かなかったこと | UNIT 04 |
| transaction | トランザクション。複数の SQL を「全部確定か、全部取り消しか」にまとめる仕組み | UNIT 05 |
| COMMIT / ROLLBACK | transaction の変更を確定する／全部取り消す | UNIT 05 |
| autocommit | transaction を書かないとき、1文ずつ自動で確定すること | UNIT 05 |
| ACID | transaction が守る4つの性質（全部か無しか・制約の維持・分離・永続） | UNIT 05 |
| database is locked | ほかの接続が書き込みの鍵を持っているときのエラー | UNIT 05 |
| 楽観的ロック | 鍵をかけずに編集させ、保存するときに `version` で食い違いを見つける方法 | UNIT 05 |
| WAL | Write-Ahead Logging。書き込み中でも読み込みを待たせにくくする記録方式 | UNIT 05 |
| スキーマ | DB にどんな表・列・制約があるか、という形の定義 | UNIT 06 |
| migration | マイグレーション。スキーマの変更を番号付きの手順として管理し、未適用の分だけを順に適用する仕組み | UNIT 06 |
| schema_migrations | 適用済みの migration の版を記録する表 | UNIT 06 |
| 冪等 | べきとう。何度実行しても同じ結果になること | UNIT 06 |
| `include_str!` | コンパイル時にファイルを読み、文字列としてプログラムに埋め込むマクロ | UNIT 06 |
| Builder / Database / Connection | libsql で、DB の開き方を決める部品／DB そのもの／DB と話す窓口 | UNIT 07 |
| execute / query | 行を返さない文を実行して変更した行数を返す／行を返す文を実行して `Rows` を返す | UNIT 07 |
| bind | プレースホルダー（`?1` など）に、値を別に渡して当てはめること | UNIT 07 |
| プレースホルダー | SQL の中で、後から値を当てはめる場所取り（`?1`・`:owner`） | UNIT 02・07 |
| SQL インジェクション | 入力が SQL の文法として解釈され、意図と違う SQL が実行される脆弱性 | UNIT 07 |
| `Value` | libsql で、種類の分からない値をそのまま受け取る enum | UNIT 07 |
| `SqliteFailure` | libsql のエラーの種類の1つ。SQLite のエラー番号と文を持つ | UNIT 07 |
| ローカル / リモート | 手元のファイルの DB を開く形（`new_local`）／ネットの向こうの DB に毎回問い合わせる形（`new_remote`） | UNIT 08 |
| 組み込みレプリカ | embedded replica。Turso の DB のコピーを手元のファイルに置き、読むのはコピー・書くのは Turso にする形 | UNIT 08 |
| primary | プライマリ。コピーの元になる本体の DB | UNIT 08 |
| sync / sync_interval | 手元のコピーを最新にすること／それを裏で自動的に行う間隔 | UNIT 08 |
| read-your-writes | 書き込みが成功したら、書いたアプリ自身のコピーにはすぐ反映されること | UNIT 08 |
| 環境変数 | プログラムの外から渡す設定の値。`DATABASE_URL`・`DATABASE_AUTH_TOKEN` など | UNIT 08 |
| トークン | Turso の DB に入るための秘密の文字列。コードにもログにも出さない | UNIT 08 |
| `Secret` 型 | 秘密の値を包み、`{:?}` で表示しても中身を `********` で隠す型 | UNIT 08 |
| Hrana | libsql がリモートの DB とやり取りするときの通信の仕組み。リモートのエラーは `Hrana(…)` で返る | UNIT 07・08 |
| features | クレートの機能を部品ごとに選ぶ仕組み。libsql の `core`・`remote`・`replication`・`tls` など | UNIT 08 |
| スタックあふれ | 関数の作業用のメモリ（スタック）が足りなくなって、プログラムが異常終了すること | UNIT 08 |
| Repository | リポジトリ。データのしまい方・取り出し方を引き受ける部品。SQL はこの中にだけ書く | UNIT 09 |
| 行の変換（row_to_note） | DB の1行を、検証済みのドメインの型に変える関数。失敗したら場所を添えたエラーにする | UNIT 09 |
| `#[async_trait]` | `async fn` を持つ trait を `dyn` で使えるように書き換えるマクロ（async-trait クレート） | UNIT 09 |
| dyn compatible | trait を `dyn`（実行時に実装を選ぶ形）で使える性質。満たさないと E0038 になる | UNIT 09 |
| N+1 問題 | 一覧で1回、その N 件それぞれで1回ずつ SQL を送り、DB と N+1 回往復してしまうこと | UNIT 09 |
| `COALESCE` | 最初の `NULL` でない値を返す関数。`COALESCE(SUM(done), 0)` で `NULL` を 0 にする | UNIT 09 |
| `IN` | 列の値が、並べた値のどれかと一致するかを調べる条件。`?` は 32,766 個まで | UNIT 01・09 |

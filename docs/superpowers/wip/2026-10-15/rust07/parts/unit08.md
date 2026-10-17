## UNIT 08: local・remote・embedded replica

<!-- TODO: 未完。本文は未執筆。以下は構成案と、実行確認済みのコード・出力の所在だけ。 -->

<!--
構成案:
- ゴール: 3つの形（ローカル・リモート・組み込みレプリカ）の違い（データの置き場所、読み書きの行き先、速さ、オフライン）、配置に合わせた選び方、URL とトークンは環境変数、features と clang、read-your-writes・sync・sync_interval
- たとえ話: ローカル=自分の手帳、リモート=毎回本社に電話、レプリカ=本社台帳のコピーを手元に置き、読むのはコピー・書くときは本社に電話（sync=コピーの取り寄せ）
- 図1: 3つの形の読み書きの矢印。図2: 2台のアプリでの read-your-writes と sync の時間の流れ
- 例8-1: src/bin/u08_local.rs（実行済み）。出力（1回目）:
    u08_local.db: 1回目に開いたとき 1 件 / 開き直したとき 1 件 / sync エラー: Sync is not supported in databases opened in File mode.
    :memory:: 1回目に開いたとき 1 件 / 開き直したとき エラー: SQLite failure: `no such table: memos` / sync エラー: Sync is not supported in databases opened in Memory mode.
  2回目の実行では u08_local.db が 2 件になる
- 例8-2: src/bin/u08_open.rs（環境変数 DATABASE_URL / DATABASE_AUTH_TOKEN / DATABASE_REPLICA_PATH で形を選ぶ。実行済み）:
    何も設定しない → 選んだ形: ローカル / SELECT 1 + 1 = 2（3 ミリ秒）
    DATABASE_URL=libsql://rustnote-example.turso.io（トークンなし）→ 設定エラー: ... DATABASE_AUTH_TOKEN が必要です（exit 1）
    DATABASE_URL=http://127.0.0.1:9 → リモート / 失敗（2030 ミリ秒）: Hrana: `http error: `error trying to connect: tcp connect error: 対象のコンピューターによって拒否されたため、接続できませんでした。 (os error 10061)``
    + DATABASE_REPLICA_PATH=u08_replica.db → 組み込みレプリカ / 失敗（2054 ミリ秒）: sync error: error trying to connect: ... (os error 10061)（build の時点で初回 sync が走る）
- 未作成: 例8-3 レプリカの使い方（Turso 本物向け。コンパイルのみ確認の予定）、features の Cargo.toml（default = core, replication, remote, sync, tls / core だけならローカル専用で clang 不要: ref/libsql-core-noclang-probe で検証済み）
- よくあるエラー（素材）:
  1. os error 10061（上記、約2秒）
  2. debug ビルドの main スレッドでリモートの execute → "thread 'main' (...) has overflowed its stack"（examples/probe_remote2.rs exec で再現。tokio::spawn の中なら Hrana のエラーが返る。release での回避は未再確認）
  3. Windows ARM64 で clang が無いと ring のビルド失敗（docs/superpowers/wip/2026-10-15/rust09/out/u05_err_clang.txt の実出力）
  4. core だけの features で new_remote を使うと E0599（未作成・未実行）
- 未検証: Turso 本物・sqld（turso dev）への接続成功時の動き、トークン不正時のエラー文。公式ドキュメント（scratchpad/turso/features_embedded-replicas_introduction.md）: 読みは手元、書きはリモートの primary、read-your-writes は既定で有効（Builder の read_your_writes(true)）、sync_interval は Database が生きている間だけ裏で動く、同期中にローカルファイルを別に開くと壊れる。Turso は新規には turso クレート（Turso Sync）を推奨、embedded replica も本番で引き続きサポート。
-->

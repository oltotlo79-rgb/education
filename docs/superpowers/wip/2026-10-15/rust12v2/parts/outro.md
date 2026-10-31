## この巻のまとめ

この巻の UNIT 01〜10 で、次のことができるようになったかを確かめましょう。できたものに印を付けてください。

**UNIT 01: release ビルドと再現性**

- ☐ debug と release の違いを、大きさ（約 21.7 MB と 11.8 MB）・速さ（ハッシュ化 約 400 ms と 20 ms）・整数のあふれ（panic と折り返し）で説明できる
- ☐ `cargo build --release --locked` → `topcoat asset bundle --release` の順で本番用の実行ファイルを作り、起動して `GET /health` で確かめられる
- ☐ 実行ファイルは「自分の隣の `assets\`」を、DB は「今いるフォルダ」（または `DATABASE_URL`）を使うことを説明できる
- ☐ `no asset bundle at ...` と `os error 10048` のエラーを読み、直し方を言える
- ☐ 同じソースから作った実行ファイルの中身が違う理由（リンクした時刻・識別番号）と、作る場所が中身に入り込むこと（ユーザーフォルダの場所・`asset!` のクレートの場所）を、実際の比べ方で説明できる
- ☐ release でも整数のあふれに気づけるよう、大きい型・`checked_*`・`overflow-checks = true` のどれかを選べる
- ☐ コミット・Rust の版・`Cargo.lock` と実行ファイルの指紋を、`release-record.txt` に記録できる

**UNIT 02: multi-stage の Dockerfile**

- ☐ RustNote の Dockerfile の builder の段と実行用の段が、それぞれ何を入れ、何を入れないかを、docker history の結果で説明できる
- ☐ 1段だけの Dockerfile と比べて、大きさ（3.98GB と 170MB）・中の道具・動かす利用者（root と app）の違いを言える
- ☐ Docker のキャッシュが段ごとに効くことと、ソースを1行変えると cargo build が全部やり直しになる理由を説明できる
- ☐ キャッシュのマウントで target/ を残し、やり直しを短くできる
- ☐ 実行用の段の Linux のちがい（alpine で no such file or directory）・COPY --from の場所・/data の持ち主のまちがいが、それぞれどこで、どう止まるかを言える

**UNIT 03: アセットの束を本番用に作る**

- ☐ topcoat asset bundle --release が、中で cargo build をして、実行ファイルの asset! の印から束を作ることと、先に --locked でビルドしておく理由を言える
- ☐ manifest.toml の id・file・hash・content_type と、指紋付きの名前の決まり方を説明できる
- ☐ 束が 1 年間 immutable のキャッシュと gzip・br の圧縮で配られることを、本物の応答で確かめられる
- ☐ CSS を変えて束を作り直し忘れると、エラーも無く古い CSS のままになることを示せる
- ☐ topcoat CLI と topcoat クレートの版がずれたときの警告を読み、CLI をそろえられる

**UNIT 04: fly launch・fly.toml・ポート**

- ☐ fly.toml の各項目（app・primary_region・[build]・[env]・[http_service]・ヘルスチェック・[[vm]]）と、RustNote の設定の対応を説明できる
- ☐ ブラウザから RustNote までの道筋（443 → 入口 → internal_port 8080 → 0.0.0.0:8080）を言える
- ☐ fly launch がすること（調べる・名前と場所・fly.toml・公開）と --no-deploy を、公式のドキュメントにそって説明できる
- ☐ fly.toml と Dockerfile の食い違い（PORT・force_https・[env] の秘密）を、手元の道具とコンテナで見つけられる

**UNIT 05: secrets・設定・入れ替え**

- ☐ RustNote の設定の値を、秘密の値（トークン）と秘密でない値に分け、置き場所（fly secrets・[env]・ENV）と勝ち負けを言える
- ☐ ENV で焼き込んだトークンが docker image inspect と docker history で読めることを示し、漏れたら入れ替える理由を言える
- ☐ .dockerignore が無いと .env が builder の段に入りビルドに送られることを、実際のビルドで示せる
- ☐ fly secrets set・--stage・fly secrets deploy・list・unset と、Turso のトークンの入れ替えの順番を説明できる

**UNIT 06: health check と起動の順番**

- ☐ RustNote の /health が DB に SELECT 1 をして 200・503 を返すことを、sqld を止めたり動かしたりして確かめられる
- ☐ Fly.io のヘルスチェックが失敗したマシンには振り分けないが、動かし直しはしないことを、公式のドキュメントにそって言える
- ☐ DB が無いまま起動すると、マイグレーションで止まって終了コード 1 になり、restart policy（on-fail）で動かし直されることを説明できる
- ☐ 起動から 200 までの時間を測って、grace_period を決められる

**UNIT 07: migration を出す順番と release command**

- ☐ 起動のたびに migrate をかける作りで、同時に起動したマシンがぶつかる理由を、例7-1の結果で説明できる
- ☐ Fly.io の release_command のしくみ（新しいイメージで1回・失敗で公開を止める・volume は無い）を言え、--migrate-only の動かし方を足して確かめられる
- ☐ 足すだけのマイグレーションと、名前を変えるマイグレーションで、古い版が動くかの違いを示せる
- ☐ 広げてから縮める（expand / contract）の4つの手順を言える

**UNIT 08: deploy・ログ・動作の確かめ・独自ドメイン**

- ☐ fly deploy がすること（イメージ・release command・rolling・起動の直後の見張り）を、公式のドキュメントにそって説明できる
- ☐ 本物の URL で7つを順に試すスモークテストを作り、正しい公開と壊れた公開の両方で結果を読める
- ☐ アクセスログで、スモークテストの1つ1つのリクエストを見つけられる
- ☐ 独自ドメインの手順（fly certs add・CNAME か A/AAAA・証明書の自動発行）を説明できる

**UNIT 09: 台数・地域・費用・Turso との距離**

- ☐ DB までの遅れを入れた実験で、1回の表示の時間が、往復の回数の分だけ増えることを示せる
- ☐ RustNote が DB に送ったリクエストを数えて、読む SQL 1つで describe と cursor の2回になることを確かめられる
- ☐ マシンと DB を、利用者の近くの同じ地域（nrt・aws-ap-northeast-1）に置く理由を説明できる
- ☐ 台数（止める／動かす・いつも1台・2台以上）と費用（Fly.io・Turso の公式の料金）の釣り合いを考えられる

**UNIT 10: rollback・backup・障害からの復旧・更新**

- ☐ ほかの接続が開いている間に DB のファイルだけをコピーすると、-wal の分が抜けることを示し、VACUUM INTO でバックアップを取れる
- ☐ 戻すときは rustnote.db・-wal・-shm を全部よけてから写しを置き、件数と integrity_check で確かめられる
- ☐ 壊れた版を、前の版のイメージで戻せる。Fly.io の fly releases --image・fly deploy --image と、戻らない物（DB・secrets・fly.toml）を言える
- ☐ 依存の更新を、dry-run → 別のフォルダで更新 → テスト → 公開の順に進め、版を書く場所を組で直せる

## 用語ミニ辞典

この巻（UNIT 01〜10）に出てきた用語です。

### UNIT 01 の用語

| 用語 | 意味 |
|---|---|
| debug ビルド | `cargo build` で作る試し用の実行ファイル。最適化なし、整数のあふれを確かめる。`target\debug` にできる |
| release ビルド | `cargo build --release` で作る本番用の実行ファイル。最適化あり、既定では整数のあふれを確かめない。`target\release` にできる |
| 最適化 | 同じ動きのまま、速く小さくなるように、コンパイラーがプログラムを作り直すこと |
| `cfg!(debug_assertions)` | debug ビルドなら `true`、release ビルドなら `false` になる式 |
| 整数のあふれ（オーバーフロー） | 計算の答えが、型で表せる範囲を超えること。debug は panic、release は既定で折り返す |
| `checked_add` / `checked_mul` | あふれたら `None` を返す計算。あふれたことをプログラムで扱える |
| `overflow-checks` | `[profile.release]` に書くと、release でもあふれを確かめる設定 |
| アセットの束 | `topcoat asset bundle` が作る、CSS などのファイルと目録（`manifest.toml`）。実行ファイルの隣の `assets\` に置く |
| 再現性 | 同じソース・同じ道具・同じ部品から、いつ誰が作っても同じ物ができること |
| SHA-256 の指紋 | ファイルの中身から計算した長い値。1 バイトでも違えば、まったく別の値になる。`Get-FileHash` で表示できる |
| `/Brepro` | Windows のリンカーに、時刻の代わりに中身から計算した値を使わせる指定 |
| `--remap-path-prefix` | 実行ファイルに入れるフォルダの場所の文字を、別の文字に置き換える rustc の指定 |
| `RUSTFLAGS` | cargo が rustc に渡す追加の指定を書く環境変数。変えると全部のクレートが作り直しになる |
| os error 10048 | Windows の「そのアドレスとポートは、もう使われている」エラー |

### UNIT 02 の用語

| 用語 | 意味 |
|---|---|
| multi-stage ビルド | 1つの Dockerfile に FROM を2つ以上書き、前の段で作った物から要る物だけを COPY --from で運ぶ作り方 |
| レイヤー | イメージを作る1つ1つの手順の結果。同じ手順・同じ材料なら、前のビルドの結果（キャッシュ）を使い回す |
| `CACHED` | docker build の記録で、その手順の結果を前のビルドから使い回したという印 |
| glibc | Linux の C の基本のライブラリ。新しい glibc で作った実行ファイルは、古い glibc の上では動かないことがある |
| `docker history` | イメージのレイヤーと、それぞれの大きさ・作った手順を並べて見るコマンド |
| キャッシュのマウント | `RUN --mount=type=cache,…` で、ビルドをまたいでフォルダ（target/ など）を取っておくしくみ。イメージには残らない |
| 最小権限 | プログラムに、要る分だけの権限しか与えない考え方。RustNote は app で動き、書けるのは /data だけ |

### UNIT 03 の用語

| 用語 | 意味 |
|---|---|
| アセットの束 | CSS などを指紋付きの名前にして、目録と一緒にまとめたフォルダ。RustNote では実行ファイルの隣の assets/ |
| `manifest.toml` | 束の目録。asset! の id と、束の中のファイルの名前・指紋・種類の対応を書く |
| 指紋（ハッシュ） | ファイルの中身から計算した値。中身が1文字でも変われば、別の値になる |
| 圧縮 | 同じ情報のまま、送る中身を小さくすること。HTTP では gzip と br（Brotli）がよく使われる |
| `Accept-Encoding` | ブラウザが、読める圧縮の方式を伝える見出し |
| `vary` | 同じ URL でも、ある見出し（ここでは Accept-Encoding）によって中身が違う、という印 |

### UNIT 04 の用語

| 用語 | 意味 |
|---|---|
| fly.toml | Fly.io でアプリを動かすときの設定ファイル（TOML）。名前・地域・ポート・ヘルスチェック・マシンの大きさを書く |
| リージョン | Fly.io のマシンを置く場所。3文字の名前（nrt は東京） |
| `internal_port` | Fly.io の入口が、マシンの中のどのポートへリクエストを届けるか |
| `force_https` | http:// のリクエストを https:// へ移す設定 |
| `auto_stop_machines` | 使われていない間、マシンを止めるか（"off"・"stop"・"suspend"） |
| `tomllib` | Python の標準の、TOML を読むための部品 |

### UNIT 05 の用語

| 用語 | 意味 |
|---|---|
| secret | パスワードやトークンのような秘密の設定の値。Fly.io では fly secrets set で保管し、起動のときに環境変数で渡す |
| 入れ替え（ローテーション） | 秘密の値を、決まった時期や漏れたおそれがあるときに、新しい値に替えること |
| `fly secrets set` | 秘密の値を Fly.io に保管し、マシンを入れ替えて渡すコマンド（`--stage` で、登録だけ） |
| `docker image inspect` | イメージの設定（環境変数・利用者・ポートなど）を見るコマンド |
| `turso db tokens invalidate` | その DB（同じグループ）のトークンを全部使えなくする Turso のコマンド。短い停止がある |

### UNIT 06 の用語

| 用語 | 意味 |
|---|---|
| ヘルスチェック | 動いているアプリに決まった間をおいてリクエストを送り、正しく答えられるかを確かめること |
| sqld | libSQL のサーバー。Turso と同じ形（Hrana）で、ネットごしに SQL を受け付ける。手元の Docker で動かせる |
| Hrana | libSQL のクライアントとサーバーの間の、HTTP などでの話し方の決まり |
| restart policy | マシン（コンテナ）のプロセスが終わったとき、動かし直すかの決まり。Fly.io の既定は on-fail |
| `grace_period` | 起動してから、最初のヘルスチェックまで待つ時間 |
| 503 | Service Unavailable。今は仕事ができない、という状態コード |

### UNIT 07 の用語

| 用語 | 意味 |
|---|---|
| release command | 新しい版のマシンを入れ替える前に、新しいイメージで1回だけ動かすコマンド。マイグレーションをここで済ませる |
| expand / contract | DB の形を変えるとき、まず足し（広げ）、全部のコードが新しい形になってから古い物を消す（縮める）進め方 |
| `--migrate-only` | この巻で RustNote のコピーに足した、マイグレーションだけをして終わる動かし方 |
| rolling | マシンを順番に入れ替える公開のやり方 |
| `ALTER TABLE … ADD COLUMN` | 表に列を足す SQL。古い版のコードはその列を知らなくても動く |

### UNIT 08 の用語

| 用語 | 意味 |
|---|---|
| スモークテスト | 公開した直後に、大事な使い方をひととおり、本物の URL で試すこと |
| 独自ドメイン | 自分のドメインで、アプリを開けるようにすること。DNS のレコードで Fly.io に向ける |
| CNAME | DNS のレコードの1つ。名前を、別の名前に向ける（サブドメインに使う） |
| A・AAAA | DNS のレコード。名前を、IPv4・IPv6 のアドレスに向ける |
| Let's Encrypt | HTTPS の証明書を自動で無料で発行するしくみ。Fly.io が使う |
| `fly logs` | Fly.io のマシンのログを見るコマンド |

### UNIT 09 の用語

| 用語 | 意味 |
|---|---|
| レイテンシ | 1回の往復にかかる時間。距離が遠いほど長くなる |
| toxiproxy | 2つのプログラムの間にはさんで、通信をわざと遅くしたり切ったりできる道具 |
| Hrana | libSQL のサーバーと HTTP で話すときの決まり。`/v3/pipeline`・`/v3/cursor` に JSON を送る |
| リージョン | マシンや DB を置く地域。Fly.io の `nrt`、Turso の `aws-ap-northeast-1` など |
| `min_machines_running` | いつも動かしておくマシンの最小の数。0 なら、使われていない間は全部止まる |
| 読んだ行・書いた行 | Turso の費用を決める量。SQL が読んだ行の数と、書いた行の数 |

### UNIT 10 の用語

| 用語 | 意味 |
|---|---|
| WAL | 書き込みを、まず -wal のファイルに書き、あとで本体に書き戻す SQLite のしくみ |
| `-wal`・`-shm` | WAL の書き込みの記録と、その目次のファイル。DB の一部なので、本体と組で扱う |
| `VACUUM INTO` | 動いている DB の、ある瞬間の全体を、1つのファイルに写す SQL |
| `PRAGMA integrity_check` | DB のファイルの形が壊れていないかを確かめる SQL。壊れていなければ ok |
| rollback | 新しい版に問題があったとき、前の版（イメージ）に戻すこと。DB の中身は戻らない |
| point-in-time recovery | Turso で、過去の時刻の状態の DB を、新しい DB として作ること |

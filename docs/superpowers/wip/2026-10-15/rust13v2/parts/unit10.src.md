## UNIT 10: テスト・Docker・Fly.io・CI

### この単元のゴール

- `tests/common/mod.rs` の道具（一時フォルダの DB・テスト用のアセットの設定・ポートを開かない `Router::handle`）と、`tests/repository.rs`・`tests/http.rs` が確かめていることを説明できる
- CI（`ci.yml`）と同じ3つの手順（整形の確認・Clippy・テスト）を、手元で動かして結果を読める
- `Dockerfile` の2段階（multi-stage）のビルドと `.dockerignore` を説明し、本物の Docker でイメージを作って起動し、`/health` を確かめられる
- `fly.toml` の項目が、RustNote のどの設定（ポート・`/health`・HTTPS と `Secure` のクッキー）と結びついているかを言える

### たとえ話でつかむ

お弁当の工場を考えます。

- 出荷の前に、3つの**検査**をします。おかずを1つずつ調べる（ユニットテスト）、冷蔵庫に入れて取り出せるかを調べる（リポジトリの結合テスト）、お客さんの注文から箱を渡すまでを、窓口を開けずに通しで調べる（HTTP のテスト）。
- 検査は、毎回**新しいまな板**（一時フォルダの DB）で始め、終わったら捨てます。前の検査の残りが、次の検査に混ざりません。
- 箱詰めは、**調理場**（builder の段）と**お弁当箱**（実行用の段）を分けます。お弁当箱には、できたおかず（実行ファイル）と、ふりかけ（CSS の束）だけを入れ、包丁や鍋（コンパイラ）は入れません。
- **配送の指示書**（`fly.toml`）には、「受け取り口は 8080 番」「時々、元気かを `/health` で聞く」と書きます。
- 毎回、調理を変えるたびに、**同じ検査を自動で**やり直します（CI）。

| お弁当の工場 | RustNote（`tests/`・`Dockerfile`・`fly.toml`・`ci.yml`） |
|---|---|
| おかずを1つずつ | `src/*.rs` の `#[cfg(test)]`（19 個） |
| 冷蔵庫に入れて取り出す | `tests/repository.rs`（8 個） |
| 注文から箱まで通しで | `tests/http.rs`（14 個） |
| 新しいまな板 | `common::test_db()`（一時フォルダ） |
| 調理場とお弁当箱 | `Dockerfile` の builder の段と実行用の段 |
| 配送の指示書 | `fly.toml` |
| 毎回、同じ検査を自動で | `ci.yml`（GitHub Actions） |

> **用語:** CI（シーアイ、継続的インテグレーション：コードを送るたびに、整形・静的な検査・テスト・ビルドを、自動で動かすしくみ。RustNote は GitHub Actions の設定の例を `ci.yml` に置いている）

<!-- -->

> **用語:** コンテナのイメージ（アプリと、動かすのに要るものを1つにまとめた箱の型。`docker build` で作り、`docker run` で動かす）

### 図で理解する

テストの3つの層と、CI の手順です（例10-4の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-1001" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="20" y="22" font-size="13" font-weight="700" fill="#0f172a">テストの3つの層（cargo test で 41 個）</text>
<rect x="20" y="34" width="320" height="50" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="32" y="55" font-size="12" font-weight="700" fill="#1d4ed8">ユニットテスト（19）</text>
<text x="32" y="74" font-size="11" fill="#1d4ed8">src の中。型の検査・エラーの表・設定</text>
<rect x="20" y="94" width="320" height="50" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="32" y="115" font-size="12" font-weight="700" fill="#047857">リポジトリの結合テスト（8）</text>
<text x="32" y="134" font-size="11" fill="#047857">一時フォルダの本物の libSQL ファイル</text>
<rect x="20" y="154" width="320" height="50" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="32" y="175" font-size="12" font-weight="700" fill="#c2410c">HTTP のテスト（14）</text>
<text x="32" y="194" font-size="11" fill="#c2410c">Router::handle に直接渡す（ポートを開かない）</text>
<text x="20" y="232" font-size="11" fill="#475569">下に行くほど、本物に近く、遅い。どの層も、テストごとに新しい DB</text>
<text x="380" y="22" font-size="13" font-weight="700" fill="#0f172a">CI（ci.yml）の手順</text>
<rect x="380" y="34" width="320" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="540" y="56" font-size="11" fill="#0f172a" text-anchor="middle">push・pull request</text>
<rect x="380" y="86" width="320" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="540" y="108" font-size="11" fill="#1d4ed8" text-anchor="middle">test: cargo fmt --check → clippy -D warnings → test</text>
<rect x="380" y="138" width="320" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="540" y="160" font-size="11" fill="#047857" text-anchor="middle">docker: docker build（test が通ったら）</text>
<line x1="540" y1="68" x2="540" y2="82" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-1001)"/>
<line x1="540" y1="120" x2="540" y2="134" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-1001)"/>
<text x="380" y="198" font-size="11" fill="#b91c1c">ci.yml は、.github/workflows/ に置いたときだけ動く</text>
<text x="380" y="216" font-size="11" fill="#475569">（RustNote のフォルダには「設定の例」として置いてある）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: テストの層と CI。CI は、手元と同じ3つの手順を、コードを送るたびに動かし、通ったら Docker のイメージも作ってみる</div>
</div>

`Dockerfile` の2つの段と、それぞれに入るものです（例10-5の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-1002" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="310" height="200" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="175" y="44" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">① builder（rust:1.99.1-trixie）</text>
<text x="36" y="72" font-size="11" fill="#0f172a">cargo install topcoat-cli 0.11.0</text>
<text x="36" y="96" font-size="11" fill="#0f172a">COPY . .（.dockerignore で target/ などを除く）</text>
<text x="36" y="120" font-size="11" fill="#0f172a">cargo build --release --locked</text>
<text x="36" y="144" font-size="11" fill="#0f172a">topcoat asset bundle --release</text>
<text x="36" y="180" font-size="11" fill="#475569">コンパイラ・ソース・途中の成果物が全部入る</text>
<text x="36" y="200" font-size="11" fill="#475569">（もとのイメージだけで 2.22GB）</text>
<rect x="400" y="20" width="300" height="200" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="550" y="44" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">② 実行用（debian:trixie-slim）</text>
<text x="416" y="72" font-size="11" fill="#0f172a">ca-certificates（Turso へ HTTPS）</text>
<text x="416" y="96" font-size="11" fill="#0f172a">利用者 app（uid 10001）・/data</text>
<text x="416" y="120" font-size="11" fill="#047857">/app/rustnote（実行ファイル）</text>
<text x="416" y="144" font-size="11" fill="#047857">/app/assets/（CSS の束）</text>
<text x="416" y="168" font-size="11" fill="#0f172a">HOST=0.0.0.0・PORT=8080</text>
<text x="416" y="200" font-size="11" fill="#047857">できたイメージ: 170MB</text>
<line x1="330" y1="120" x2="396" y2="120" stroke="#475569" stroke-width="2" marker-end="url(#ar-13-1002)"/>
<text x="363" y="112" font-size="10" fill="#475569" text-anchor="middle">COPY</text>
<text x="363" y="138" font-size="10" fill="#475569" text-anchor="middle">--from</text>
<text x="20" y="250" font-size="11" fill="#0f172a">実行用の段には、builder の段から、実行ファイルと assets/ の2つだけをコピーする。コンパイラは入らない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: 2段階のビルド。大きな builder の段で作り、小さな実行用の段に、要るものだけを移す</div>
</div>

### まず動かす最小コード

#### 例10-1: tests/common/mod.rs の全文

> **▼ このファイルがやること（先に日本語で）:** テストで共通に使う道具です。一時フォルダにマイグレーション済みの DB を作る `test_db`、本物の CSS の束の代わりの設定 `test_assets`、ルーターを組み立てる `test_app`、GET とフォームの POST を送る関数、登録してクッキーを受け取る `sign_up`、ノートを作る `create_note`、応答から本文・`Location`・セッションのクッキーを取り出す関数があります。

```rust
@@FILE:rustnote/tests/common/mod.rs@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: テストごとに、新しい DB

- `test_db` は、`tempfile::tempdir()` で一時フォルダを作り、その中に `test.db` を開いて、マイグレーションをかけます。`TestDb` が `_dir`（`TempDir`）を持っているので、`TestDb` が捨てられる（drop）と、フォルダごと消えます（問1）。
- テストは、それぞれ自分の DB を持つので、同じ時に動いても、ほかのテストのデータが混ざりません。`cargo test` が、テストを並列に動かせるのは、このためです（例10-4）。

##### 解説2: ポートを開かずに、ルーターに直接渡す

- `test_app` は、本番と同じ `routes::build_router` に、テスト用の DB と `test_assets()` を渡します。`test_assets` は、`STYLESHEET` の ID と仮のファイル名だけの目録（Manifest）で、CSS の URL を `/test-assets/style.css` に決めます。本物の束（`topcoat asset bundle`）が無くても、画面を組み立てられます。
- `get` と `post_form` は、`Request` を組み立てて `self.router.handle(…)` に渡し、`Response` を受け取ります。ネットワークのポートを開かないので速く、ポートの取り合いも起きません。
- `session_cookie` は、`Set-Cookie` を `get_all` で**全部**見て、`__Host-session=` で始まり、消す指示（`Max-Age=0`）でないものを探します。`Set-Cookie` の順番は決まっていないので（UNIT 06）、1つ目だけを見ると、ときどき失敗します（第11巻 UNIT 03）。
- `encode` は、フォームの値を URL の形にする簡易版です。テストで使う文字（英数字・空白・日本語など）だけを扱います。

#### 例10-2: tests/repository.rs と tests/http.rs の全文

> **▼ このファイルがやること（先に日本語で）:** `repository.rs` は、リポジトリを一時フォルダの本物の DB で動かす結合テスト（8個）です。`http.rs` は、HTTP のリクエストをルーターに渡して、状態コード・リダイレクト先・画面の文字を確かめるテスト（14個）です。

```rust
@@FILE:rustnote/tests/repository.rs@@
```

```rust
@@FILE:rustnote/tests/http.rs@@
```

##### 解説3: 何を確かめているか

| ファイル | 確かめていること（この巻の単元） |
|---|---|
| `repository.rs` | マイグレーションは1回だけ（UNIT 02）・同じメールは `UniqueViolation`・持ち主ごとの分離・楽観的ロック・切り替えと削除・検索とページ送り（`%` と `_` も）・セッションの期限とログアウト（UNIT 05） |
| `http.rs` | `/health` の JSON・トップのリンクと CSS の URL・未ログインは 303・登録の 400 と 409・ログインの 401 とログアウト（UNIT 07）・PRG とエスケープ・空のタイトルの 400・他人のノートの 404・おかしな ID の 404・古い version の 409・切り替えと削除・検索とページ送り・ほかのサイトからの POST の 403・お知らせは一度だけ（UNIT 08） |

- `http.rs` の `assert!(html.contains(…))` は、画面の文字を探します。`create_note_follows_prg_and_escapes_html` は、`<script>` が `&lt;script&gt;` になっていて、生の `<script>alert(1)</script>` が**無い**ことまで確かめます。
- この巻の単元で、教材のコードのコピーを壊したとき、止めてくれたのも、このテストたちでした（UNIT 05 の持ち主の条件・UNIT 08 の `href!`）。

#### 例10-3: Dockerfile・.dockerignore・fly.toml・ci.yml の全文

> **▼ このファイルがやること（先に日本語で）:** `Dockerfile` は、RustNote のコンテナのイメージを、2つの段で作る手順です。`.dockerignore` は、イメージを作るときに Docker へ送らないファイルの一覧です。`fly.toml` は、Fly.io で動かすときの設定（待ち受けのポート・HTTPS・ヘルスチェック・マシンの大きさ）です。`ci.yml` は、GitHub Actions で、テストと Docker のビルドを自動で動かす設定の例です。

```dockerfile
@@FILE:rustnote/Dockerfile@@
```

```text
@@FILE:rustnote/.dockerignore@@
```

```toml
@@FILE:rustnote/fly.toml@@
```

```yaml
@@FILE:rustnote/ci.yml@@
```

最後に、RustNote の説明書 `README.md` です（中にコードのかこみがあるので、外側のかこみの記号を4つにしています）。

````markdown
@@FILE:rustnote/README.md@@
````

##### 解説4: Dockerfile の2つの段

- **builder の段**（`FROM rust:1.99.1-trixie AS builder`）は、`rust-toolchain.toml` と同じ Rust 1.99.1 の入ったイメージです。`cargo install topcoat-cli --version 0.11.0 --locked` で CSS の束を作る道具を入れ、`COPY . .` でソースを入れ、`cargo build --release --locked` で、`Cargo.lock` の版のままビルドします。最後に `topcoat asset bundle --release` で、実行ファイルの隣に `assets/` を作ります。
- `cargo install` を `COPY . .` より**前**に書いているのは、Docker が段ごとに結果を覚えておく（キャッシュ）からです。ソースを変えても、topcoat-cli のインストールは、やり直しになりません。
- **実行用の段**（`FROM debian:trixie-slim`）は、小さな Debian です。Turso に HTTPS でつなぐための `ca-certificates` を入れ、root でない利用者 `app`（uid 10001）と、ローカルの DB を置く `/data` を作ります。builder の段からは、`COPY --from=builder` で、実行ファイルと `assets/` の2つだけを持ってきます。`AssetBundle::load()` は「実行ファイルの隣の `assets/`」を読むので（UNIT 03）、`/app/rustnote` と `/app/assets` に並べます。
- `ENV` で `HOST=0.0.0.0`・`PORT=8080`・`DATABASE_URL=/data/rustnote.db` を決めます。`0.0.0.0` は「どのネットワークの入口からでも受け付ける」で、コンテナの外から届くために要ります（問3）。

##### 解説5: .dockerignore・fly.toml・ci.yml

- `.dockerignore` は、`target/`（ビルドの成果物）・DB のファイル・`.env`（秘密の値）・`.git/` を、Docker に送りません。送る量が減って速くなり、秘密の値がイメージに入るのも防ぎます。
- `fly.toml` の `internal_port = 8080` は、`Dockerfile` の `PORT=8080` と合わせます。`force_https = true` は、`http://` で来たら `https://` へ移します。セッションのクッキーは `Secure`（HTTPS でしか送らない）なので、HTTPS が必須です（UNIT 06）。`[[http_service.checks]]` は、30 秒ごとに `GET /health` を送り、200 なら正常とみなします（UNIT 07）。
- `fly.toml` の `[env]` には秘密でない値だけを書き、DB の URL とトークンは `fly secrets set` で渡します（UNIT 03）。この巻では、Fly.io のアカウントを使った公開はしません（第12巻）。
- `ci.yml` は、`test` のジョブで、整形の確認（`cargo fmt --check`）・Clippy（`-D warnings` で警告もエラー）・テスト（`--locked`）を順に動かし、通ったら `docker` のジョブで `docker build` をします。GitHub Actions は、リポジトリの `.github/workflows/` の中の設定しか読まないので、使うときは、そこへコピーします。
- `README.md` は、RustNote を初めて見る人のための説明書です。できること・使っている版・準備（Windows ARM64 では LLVM が要る）・起動のしかた・環境変数・URL の一覧・状態コード・テスト・フォルダの構成・公開の流れ・設計のメモを、この巻の単元と同じ順番でまとめています。Docker のビルドと起動を確かめたので、「公開する」の注意書きを、「Docker は確かめた・Fly.io と Turso への公開は未検証」に直しました。

#### 例10-4: CI と同じ3つの手順を、手元で動かす

教材のフォルダ（`Rust/rustnote` の `src`・`tests`・`migrations`・`assets` と設定ファイル）をコピーし、CI と同じコマンドを動かしました。

```powershell
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

```text
@@OUT:u10_fmt@@
```

```text
@@OUT:u10_clippy@@
```

```text
@@OUTGREP:u10_test:^(running|test result|     Running|   Doc-tests)@@
```

- 整形の確認と Clippy は、何も言わずに通りました（`exit=0`）。テストは、ユニットテスト 19・HTTP 14・リポジトリ 8 の、合わせて 41 個が通りました。
- HTTP のテストは 2.03 秒かかりました。登録とログインのたびに、Argon2id を debug のビルドで計算するからです（1回 0.4 秒ほど。第11巻 UNIT 08）。

テストを1つずつ順に動かす（`--test-threads=1`）と、どれくらい変わるかも測りました。

```text
@@OUT:u10_threads@@
```

- 並列（既定）のほうが、4 倍ほど速くなりました。テストごとに一時フォルダの DB を持つので、並列に動かしても、結果は変わりません。

### もう一歩: 実務に近い形

#### 例10-5: 本物の Docker で、イメージを作って起動する

RustNote の `Dockerfile` は、はじめ Debian 12（bookworm）のイメージを使っていました。Debian の今の版は 13（trixie）です。そこで、builder と実行用の `FROM` の2行を trixie に替えた版も作り、両方を、本物の Docker（Windows の Docker Desktop。中は linux/arm64）でビルドして起動しました。

```text
@@OUT:u10_docker_version@@
```

| | bookworm（Debian 12） | trixie（Debian 13） |
|---|---|---|
| builder のもとのイメージ（`rust:1.99.1-…`） | 2.08GB | 2.22GB |
| 実行用のもとのイメージ（`debian:…-slim`） | 138MB | 142MB |
| 最初のビルドの時間（キャッシュ無し・この PC） | 140 秒 | 235 秒 |
| できたイメージ | 172MB | 170MB |
| 実行用の glibc | 2.36 | 2.41 |
| `/health` | 200 `{"status":"ok"}` | 200 `{"status":"ok"}` |

（表の数は、`docker images`・`docker build`・`ldd --version`・`curl` の結果から写しました。ビルドの時間は、ネットワークからクレートを取ってくる時間も含むので、毎回ぶれます。）

- どちらも、ビルドも起動も問題ありませんでした。イメージの大きさは、ほとんど同じです。builder の段（2GB 以上）は、できたイメージ（170MB 前後）には入っていません。
- そこで、教材の `Dockerfile` を trixie に替えました（`FROM` の2行と、「builder と実行用を同じ Debian にそろえる」理由のコメント）。Rust の実行ファイルは、作った環境の glibc（C の基本のライブラリ）を使うので、builder と実行用の Debian の版は、そろえておきます。

替えた `Dockerfile` で、教材のフォルダから、もう一度ビルドしました（2回目なので、Docker のキャッシュが効きます）。

```powershell
docker build --progress=plain -t rustnote:u10 .
```

```text
@@OUT:u10_build_u10_result@@
```

```text
@@OUT:u10_build_u10_steps@@
```

- `cargo install topcoat-cli` の段は `CACHED`（前の結果をそのまま使った）でした。`COPY . .` より前に置いた効き目です。`COPY . .` のあとの `cargo build --release --locked` は、やり直しになり、81 秒でした。
- 最後の `COPY --from=builder … /app/rustnote` も `CACHED` でした。教材のフォルダからビルドし直した実行ファイルが、前に教材のコピーからビルドしたものと、**1バイトも違わなかった**ので、Docker は同じものとして扱いました（同じ場所・同じ道具で作ると、同じ物ができる。第12巻 UNIT 01）。
- Docker に送った中身（`transferring context`）は、最初のビルドで 258.90kB（bookworm）でした。`.dockerignore` が `target/` などを除いているので、ソースと設定ファイルだけです。2回目は、変わった分だけを送るので 3.68kB でした。

できたイメージを調べ、起動して `/health` を開きました。

```text
@@OUT:u10_run_u10@@
```

- 利用者は `app`（uid 10001。root ではない）、`/app` には実行ファイル（約 17MB）と `assets/` の2つだけでした。
- 起動のログは、マイグレーションの3つと「起動しました: http://0.0.0.0:8080」で、`/health` は 200 でした。DB は、`Dockerfile` の `DATABASE_URL=/data/rustnote.db` の、コンテナの中のファイルです（問2）。

### よくあるエラーと読み方

#### エラー1: `Cargo.toml` だけ変えて、`--locked` でビルドした

教材のコードのコピーで、`Cargo.toml` の `tempfile = "=3.27.0"` を `"=3.26.0"` にし、`Cargo.lock` は変えずに、`Dockerfile` と同じ `cargo build --release --locked` をしました。

```text
@@OUT:u10_err_locked@@
```

- **何が起きたか:** `Cargo.toml` の版の指定と、`Cargo.lock` の版が食い違うので、`Cargo.lock` を書き換える必要がありますが、`--locked` が書き換えを禁じているので止まりました。
- **読み方:** `--locked` は、「`Cargo.lock` のとおりにビルドし、勝手に版を変えない」という約束です。CI と Docker のビルドで付けておくと、手元と違う版でビルドされることを防げます。
- **直し方:** 手元で `cargo build`（`--locked` なし）か `cargo update -p tempfile` をして `Cargo.lock` を書き換え、`Cargo.toml` と一緒に送ります。

#### エラー2: 実行用の段に、`assets/` をコピーし忘れた

`Dockerfile` から `COPY --from=builder /app/target/release/assets /app/assets` の1行を消してイメージを作り、起動しました。

```text
@@OUT:u10_err_noassets@@
```

- **何が起きたか:** `docker build` は通りましたが、起動すると「アセットの束（assets/）がありません」（原因は `no asset bundle at /app/assets`）と止まり、終了コードは 1 でした。UNIT 03 の、束の無いフォルダの実行ファイルと同じです。DB を開く前に止まるので、マイグレーションのログも出ていません。
- **直し方:** `COPY --from=builder` で、実行ファイルと同じ並び（`/app/rustnote` と `/app/assets`）にコピーします。

#### エラー3: コンテナの中のポートと、ちがう番号につないだ

イメージを `docker run -p 18080:3000`（外の 18080 を、コンテナの 3000 へ）で起動し、`curl` で `/health` を開きました。

```text
@@OUT:u10_err_port@@
```

- **何が起きたか:** RustNote は、コンテナの中で 8080 で待ち受けている（ログのとおり）のに、3000 につないだので、応答がありませんでした（状態 000）。curl の終了コード 52 は「サーバーから空の返事が来た」という意味です。Docker が外の 18080 で受けた接続を、コンテナの 3000 へ渡そうとしても、そこにはだれもいないので、すぐ閉じられました。
- **直し方:** `-p 18080:8080` のように、右側（コンテナの中）を `PORT` と同じ 8080 にします。Fly.io でも、`fly.toml` の `internal_port` を `PORT` と合わせます。

### RustNote ではここで使う

- **毎回の確かめ:** コードを変えたら、`cargo fmt --check`・`cargo clippy --all-targets --locked -- -D warnings`・`cargo test --locked` の3つを動かします（例10-4）。CI を設定すれば、GitHub に送るたびに、同じことが自動で動きます。
- **公開:** 第12巻で、この `Dockerfile` と `fly.toml` を使って、Fly.io に公開する手順を学びます。DB は Turso につなぎ、URL とトークンは `fly secrets set` で渡します。
- **改善点（この単元で見つかったこと）:**
  - `Dockerfile` の `DATABASE_URL=/data/rustnote.db` は、Turso の設定（`fly secrets set`）を忘れると、そのまま使われます。コンテナの中のファイルは、作り直すと消えるので、データが残りません（問2）。
  - `ci.yml` は、`.github/workflows/` にコピーしないと動きません。

### 練習問題

**問1:** `test_db()` の一時フォルダは、どこに作られ、いつ消えるでしょうか。テストを書いて確かめてください。

**問2:** 例10-5のイメージで、コンテナを起動して登録し、コンテナを止めて（`--rm` なので消える）、もう一度起動すると、登録したユーザーでログインできるでしょうか。volume（`-v rustnote-u10:/data`）を付けたときと比べてください。

**問3:** `HOST=127.0.0.1` を付けてコンテナを起動すると、外から `/health` は開けるでしょうか。

### 解答と解説

**問1の解答:**

教材のコードのコピーで、`TestDb` に、一時フォルダの場所を返す `db_dir` を足し（実験のためだけ）、次のテストを書きました。

```rust
// tests/u10_tempdir.rs（実験のためのテスト）
mod common;

#[tokio::test]
async fn temp_dir_is_removed_on_drop() {
    let test = common::test_db().await;
    let dir = test.db_dir();
    println!("一時フォルダの名前: {}", dir.file_name().unwrap().to_string_lossy().chars().take(4).collect::<String>());
    println!("DB のファイルがある: {}", dir.join("test.db").exists());
    drop(test);
    println!("drop のあと、フォルダがある: {}", dir.exists());
}
```

```text
@@OUTGREP:u10_q1:^(running|一時|DB の|drop|test result)@@
```

- 一時フォルダは、OS の一時フォルダの中に、`.tmp` で始まる名前で作られ（名前の残りは毎回ランダム）、`test.db` が入っていました。`drop(test)` で `TestDb` を捨てると、フォルダごと消えました。
- テストが失敗して途中で止まっても（パニックしても）、Rust は、持っている値を捨てる処理（drop）を動かすので、一時フォルダは残りません。

**問2の解答:**

例10-5のイメージを `-p 18080:8080 --rm` で起動して登録とログインをし、止めて（`--rm` なのでコンテナは消える）、もう一度起動してログインしました。次に、名前付きの volume（`rustnote-u10`）を `/data` に付けて、同じことをしました。

```text
@@OUT:u10_q2@@
```

- volume なしでは、作り直したコンテナでログインできず（401）、起動のログに、マイグレーションの3つが**もう一度**出ました。`/data/rustnote.db` はコンテナの中のファイルなので、コンテナと一緒に消え、空の DB から始まったのです。
- volume ありでは、作り直したあとも 303（ログインできた）で、マイグレーションのログは出ませんでした（もう適用済み）。DB のファイルが、コンテナの外（volume）に残ったからです。
- Fly.io のマシンのディスクも、作り直すと消えます。RustNote は、本番では Turso（`DATABASE_URL=libsql://…`）を使います（第12巻）。`fly secrets set` を忘れると、`Dockerfile` の既定の `/data/rustnote.db` で動いてしまい、公開し直すたびにデータが消えます。

**問3の解答:**

```text
@@OUT:u10_q3@@
```

- ログには「起動しました: http://127.0.0.1:8080」と出ましたが、外からは応答がありませんでした（状態 000・終了コード 52。エラー3と同じ形）。`127.0.0.1` は「このコンテナの中だけ」の入口なので、Docker がコンテナの外から送ったリクエストは、届きません。
- コンテナや Fly.io のマシンでは、`HOST=0.0.0.0` にします（`Dockerfile` の `ENV` と `fly.toml` の `[env]`）。手元の開発（`.env.example`）では、ほかの PC から開けないよう、`127.0.0.1` のままにします。

### まとめ

- `tests/common/mod.rs` は、テストごとに一時フォルダの DB を作り（drop で消える）、本番と同じ `build_router` を `Router::handle` で直接動かす。`Set-Cookie` は `get_all` で全部見る。
- `cargo test --locked` は 41 個（ユニット 19・HTTP 14・リポジトリ 8）。並列で約 3 秒、1つずつだと約 12 秒。CI と同じ3つの手順（fmt・clippy・test）は、手元で動かせる。`--locked` は `Cargo.lock` の食い違いで止める。
- `Dockerfile` は、builder の段で Rust と topcoat-cli でビルドし、実行用の段に、実行ファイルと `assets/` だけを移す。`HOST=0.0.0.0`・`PORT=8080`・root でない利用者・`/data`。assets を忘れると起動で止まり、ポートを間違えると届かない。
- `fly.toml` は `internal_port`・`force_https`・`/health` のチェックを、`Dockerfile` と RustNote の設定に合わせる。コンテナの中の DB は作り直すと消えるので、本番は Turso（か volume）を使う。

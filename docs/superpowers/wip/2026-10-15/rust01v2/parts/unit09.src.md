## UNIT 09: Turso CLI と libSQL の確認

### この単元のゴール

- Turso・libSQL・libSQL のサーバー（sqld）・Turso CLI の関係を説明できる
- Turso CLI の公式の入れ方（WSL の中で、インストールのスクリプトを動かす）と、そのスクリプトがすることを説明できる。アカウントが要るコマンドを区別できる
- 手元で libSQL のサーバーを動かし、RustNote をつないで、`/health`・登録・SQL の結果を確かめられる
- `DATABASE_URL` の書き間違い（`http://` の付け忘れ・トークンの付け忘れ）のエラーを読んで直せる

### たとえ話でつかむ

**本棚と司書さん**を考えます。

- **libSQL** は、本の並べ方の決まり（データベースの形式）です。SQLite という、とても広く使われている決まりを、もとにしています。
- 本棚が**自分の部屋**にあれば、自分で本を出し入れします（ローカルのファイルの DB。`rustnote.db`）。
- 本棚が**別の建物**にあると、**司書さん**（libSQL のサーバー、`sqld`）に、窓口（HTTP）で「この本を出して」とお願いします。
- **Turso** は、司書さんのいる図書館を、インターネットの上で貸してくれる会社です。図書館を使うには、**会員証**（トークン）が要ります。
- **Turso CLI**（`turso`）は、その会社への電話です。図書館を作る（`turso db create`）のも、会員証を作るのも、この電話でします。電話をかけるには、会員になって（`turso auth signup`）、名乗る（`turso auth login`）必要があります。
- 練習のために、**自分の家に司書さんを呼ぶ**こともできます（`turso dev`。この単元では、同じ司書さんを Docker で呼びます）。

| 本棚と司書さん | この単元の言葉 |
|---|---|
| 本の並べ方の決まり | libSQL（SQLite がもと） |
| 自分の部屋の本棚 | ローカルのファイル（`DATABASE_URL=rustnote.db`） |
| 司書さんと窓口 | libSQL のサーバー `sqld` と、HTTP（`http://127.0.0.1:8080`） |
| 図書館を貸す会社 | Turso（`libsql://…turso.io`） |
| 会員証 | トークン（`DATABASE_AUTH_TOKEN`） |
| 会社への電話 | Turso CLI（`turso`） |

> **用語:** libSQL（リブ・エス・キュー・エル：SQLite をもとにしたデータベース。ファイルとしても、サーバー（sqld）越しにも使える。Turso のデータベースも libSQL）

<!-- -->

> **用語:** sqld（エス・キュー・エル・ディー：libSQL のサーバーのプログラム。HTTP で SQL を受け取る。Docker のイメージ名は `libsql-server`）

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-091" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="100" width="160" height="90" rx="8" fill="#e0f2fe" stroke="#0284c7" stroke-width="1.5"/>
<text x="100" y="132" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">RustNote</text>
<text x="100" y="156" font-size="11" fill="#0369a1" text-anchor="middle">DATABASE_URL で</text>
<text x="100" y="174" font-size="11" fill="#0369a1" text-anchor="middle">つなぐ先を決める</text>
<rect x="260" y="20" width="200" height="66" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="360" y="46" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">ファイル（rustnote.db）</text>
<text x="360" y="68" font-size="11" fill="#475569" text-anchor="middle">rustnote.db（既定）</text>
<rect x="260" y="112" width="200" height="66" rx="8" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
<text x="360" y="138" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">手元の sqld（この単元）</text>
<text x="360" y="160" font-size="11" fill="#15803d" text-anchor="middle">http://127.0.0.1:8080</text>
<rect x="260" y="204" width="200" height="66" rx="8" fill="#fef3c7" stroke="#d97706" stroke-width="1.5"/>
<text x="360" y="230" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">Turso のクラウド</text>
<text x="360" y="252" font-size="11" fill="#b45309" text-anchor="middle">libsql://…（トークンが要る）</text>
<line x1="182" y1="130" x2="256" y2="60" stroke="#475569" stroke-width="1.6" marker-end="url(#ar-01-091)"/>
<line x1="182" y1="145" x2="256" y2="145" stroke="#475569" stroke-width="1.6" marker-end="url(#ar-01-091)"/>
<line x1="182" y1="160" x2="256" y2="232" stroke="#475569" stroke-width="1.6" marker-end="url(#ar-01-091)"/>
<rect x="500" y="112" width="200" height="66" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="600" y="138" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">Turso CLI（turso）</text>
<text x="600" y="160" font-size="11" fill="#475569" text-anchor="middle">WSL の中で使う</text>
<line x1="600" y1="180" x2="466" y2="230" stroke="#475569" stroke-width="1.4" stroke-dasharray="5 4" marker-end="url(#ar-01-091)"/>
<text x="610" y="214" font-size="11" fill="#475569">クラウドを操作する</text>
<text x="610" y="230" font-size="11" fill="#475569">（ログインが要る）</text>
<line x1="498" y1="145" x2="464" y2="145" stroke="#475569" stroke-width="1.4" stroke-dasharray="5 4" marker-end="url(#ar-01-091)"/>
<text x="600" y="98" font-size="11" fill="#475569" text-anchor="middle">turso dev で、手元にも sqld を動かせる</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: RustNote の DB の3つのつなぎ先。手元の sqld でもクラウドの Turso でも、RustNote から見れば同じ libSQL</div>
</div>

### まず動かす最小コード

#### 例9-1: Turso CLI の入れ方（公式のドキュメントで確かめた手順）

Turso CLI には Windows 版が無く、公式のドキュメントでは、WSL（UNIT 07）の中で、次のコマンドを打つように案内されています（2026-10-31 に確かめたもの）。入ったかどうかは、**新しいシェル**を開いて `turso` と打って確かめます。

```powershell
# PowerShell から Ubuntu に入る
wsl
```

```bash
# ここからは Ubuntu の中
curl -sSfL https://get.tur.so/install.sh | bash
```

`curl … | bash` は、「インターネットから取ってきたスクリプトを、そのまま動かす」という意味です。動かす前に、中身を読んでおくのが安全です。この本で、スクリプト（191 行）を取ってきて読むと、次のことをしていました。

- `uname -m` と `uname -s` で、CPU（`aarch64` なら arm64）と OS（Linux）を調べる
- GitHub の Turso の公式のリポジトリから、libSQL のサーバー `sqld` と、Turso CLI の `turso` を取ってきて、ホームフォルダの `.turso` に置く
- `~/.bashrc` の最後に、`.turso` を PATH に足す行を書き足す（だから、新しいシェルで使えるようになる）
- 最後に、`turso auth signup`（会員になる）と `turso auth login`（ログイン）を案内する

2026-10-31 の Turso CLI のいちばん新しい版は **v1.0.34** でした（GitHub の公開の一覧で確かめたもの）。**この巻では、Turso CLI を動かした結果は載せません。** アカウントにつながる操作（`turso auth login`・`turso db create` など）は、第12巻と同じく、手順の説明だけにします。手元での確かめは、Turso CLI が入れるのと同じ libSQL のサーバー（sqld）を、Docker で動かして行います（例9-2）。

> **ヒント:** 公式のドキュメント（Local Development）によると、`turso dev` は手元で libSQL のサーバーを動かし、`turso dev --db-file local.db` とすると、中身をファイルに残せます。つなぐ先の例は `http://127.0.0.1:8080` です。ログインは要りません、とは書かれていないので、使うときに確かめてください。

#### 例9-2: 手元で libSQL のサーバーを動かす

> **▼ この手順がやること（先に日本語で）:** libSQL のサーバー（sqld）の公式のイメージを、Docker で動かします。PC のポート 8080 を、コンテナの 8080 につなぎます。動いたら、`/health`（元気か）と `/version`（版）を開きます。

```powershell
docker run -d --name u09sqld -p 8080:8080 ghcr.io/tursodatabase/libsql-server:latest
docker ps --filter name=u09sqld --format '{{.Image}}  {{.Status}}  {{.Ports}}'
curl.exe -s -o NUL -w '%{http_code}' http://127.0.0.1:8080/health
curl.exe -s http://127.0.0.1:8080/version
```

**▼ 実行結果**

```text
@@OUT:u09_sqld_run@@
```

```text
@@OUT:u09_sqld@@
```

- 1行目は、作られたコンテナの ID です。`0.0.0.0:8080->8080/tcp` は、PC の 8080 がコンテナの 8080 につながっている、という意味です。`/health` は 200 で、サーバーの版は sqld 0.24.33 でした。

#### 例9-3: RustNote を、そのサーバーにつなぐ

> **▼ この手順がやること（先に日本語で）:** RustNote のフォルダで、本番用の作り方（release）のアセットの束を作り、`DATABASE_URL` を `http://127.0.0.1:8080` にしてから `cargo run --release` で起動します（開発用の `cargo run` ではない理由は、エラー1で説明します）。別の窓から、`/health` を開き、ユーザーを1人登録します。最後に、libSQL のサーバーに HTTP で直接 SQL を送って、登録されたかを確かめます。

```powershell
# RustNote のフォルダで（1つ目の窓）
topcoat asset bundle --release
$env:DATABASE_URL = "http://127.0.0.1:8080"
cargo run --release
```

**▼ 実行結果**（RustNote のログ。最初のビルドの行は省いています）

```text
@@OUTGREP:u09_rustnote_log:マイグレーション|起動|->@@
```

```powershell
# 2つ目の窓
curl.exe -s -i http://127.0.0.1:3000/health
curl.exe -s -o NUL -w '%{http_code} %{redirect_url}' -d 'email=alice%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery' http://127.0.0.1:3000/signup
```

**▼ 実行結果**

```text
@@OUT:u09_rustnote_check@@
```

```powershell
# libSQL のサーバーに、HTTP で SQL を2つ送る
$body = '{"statements":["SELECT id, email FROM users","SELECT version FROM schema_migrations ORDER BY version"]}'
Invoke-RestMethod -Method Post -Uri http://127.0.0.1:8080/ -Body $body -ContentType application/json | ConvertTo-Json -Depth 6
```

**▼ 実行結果**

```text
@@OUT:u09_http_sql@@
```

#### ▼ 1つずつ分解して解説

##### 解説1: 起動のときのマイグレーション

- RustNote は、起動のたびに、DB に足りない表を作ります（マイグレーション。第7巻・第9巻）。空のサーバーだったので、`0001_users`・`0002_notes`・`0003_sessions` の3つが適用されました。

##### 解説2: `/health` と登録

- `/health` は 200 で、本文は `{"status":"ok"}` でした。RustNote の `/health` は、DB に `SELECT 1` を送って確かめるので、サーバーまで届いています（第9巻 UNIT 10）。
- `POST /signup` は 303（別のページへ移ってください）で、移る先は `/notes` でした。登録してログインできた、という意味です（第8巻）。

##### 解説3: libSQL のサーバーは、HTTP で SQL を受け取る

- サーバーに JSON で SQL を送ると、1つの SQL ごとに `results` が返り、列（`columns`）と行（`rows`）が入っていました。`users` には alice が1人、`schema_migrations` には3つの版が記録されていました。（`value` と `Count` は、PowerShell が受け取った JSON を表示し直すときに付けた物です。）
- RustNote も、中では同じように HTTP で SQL を送っています（第12巻 UNIT 09）。本物の Turso のデータベースも、同じ libSQL です。つなぐ先の URL とトークンが変わるだけで、RustNote のコードは変えません。

### よくあるエラーと読み方

#### エラー1: 開発用のビルドで、スタックがあふれる（Windows）

この本の PC で、開発用の作り方（`cargo run`）のまま、手元のサーバーにつなぎました。

```powershell
topcoat asset bundle
$env:DATABASE_URL = "http://127.0.0.1:8080"
cargo run
```

```text
@@OUT:u09_debug_overflow@@
```

- **何が起きたか:** 起動してすぐ、`thread 'main' has overflowed its stack`（main の**スタック**があふれた）で止まりました。`STATUS_STACK_OVERFLOW` は、Windows がスタックのあふれを見つけて止めた、という意味です。マイグレーションの行も出ていないので、DB につなぐ途中で止まっています。
- **なぜ:** スタックは、関数を呼ぶたびに、一時的な値を置いていく作業台です。Windows では、プログラムの最初のスレッド（main）のスタックは、MSVC のリンカーの既定で 1 MB です。開発用のビルドは最適化をしないので、一度に使う作業台が大きくなりがちで、リモートの libSQL につなぐ処理で足りなくなりました。同じ設定でも、release のビルド（例9-3）では止まりませんでした。また、ローカルのファイルの DB（`DATABASE_URL` を付けない）なら、開発用のビルドでも動きました。
- **直し方1:** `cargo run --release` で動かします（例9-3）。
- **直し方2:** 開発用のビルドのまま、main のスタックを 8 MiB にします。`RUSTFLAGS` を変えると、部品も全部ビルドし直しになります。

```powershell
$env:RUSTFLAGS = "-C link-arg=/STACK:8388608"
cargo run
```

```text
@@OUTGREP:u09_stack8:Running|起動@@
```

```text
@@OUT:u09_stack8_health@@
```

#### エラー2: `http://` を付け忘れた（エラーにならないのが落とし穴）

```powershell
$env:DATABASE_URL = "127.0.0.1:8080"
cargo run --release
```

```text
@@OUTGREP:u09_no_scheme:Running|起動@@
```

```text
@@OUT:u09_no_scheme_health@@
```

**エラーにならず、起動してしまいました。** RustNote のフォルダを見ると、`127.0.0.1` という名前のファイルができていました。

```powershell
Get-ChildItem -Name | Where-Object { $_ -like '127*' }
Get-Item .\127.0.0.1 -Stream * | Select-Object Stream, Length | Format-Table -AutoSize
```

```text
@@OUT:u09_no_scheme_stream@@
```

- **何が起きたか:** `http://` も `libsql://` も付いていないので、RustNote は `127.0.0.1:8080` を**ファイルの名前**として扱い、ローカルのファイルの DB を作りました。Windows のファイルの仕組み（NTFS）では、`名前:別の名前` は「ファイルの中の、別の流れ（代替データストリーム）」という意味になります。だから、中身が 0 バイトの `127.0.0.1` と、その中の `8080` という流れ（49,152 バイトの DB）ができました。
- 手元のサーバーには何も届かず、登録したユーザーも、このファイルの中に入ります。**エラーが出ないので、気づきにくい**落とし穴です。
- **直し方:** `http://127.0.0.1:8080` のように、`http://` から書きます。できてしまったファイルは、RustNote を止めてから `Remove-Item -LiteralPath .\127.0.0.1` で消します。

#### エラー3: Turso の URL なのに、トークンが無い

```powershell
$env:DATABASE_URL = "libsql://rustnote-example.turso.io"
cargo run --release
```

```text
@@OUTGREP:u09_no_token:Error|Caused|DATABASE_URL|exit@@
```

- **何が起きたか:** `libsql://` で始まる URL は、インターネットの向こうの Turso です。RustNote は、そのときはトークン（`DATABASE_AUTH_TOKEN`）を必須にしていて、つなぐ前に止まりました（第9巻）。
- **直し方:** Turso の会員証（トークン）を `DATABASE_AUTH_TOKEN` に入れます。トークンは秘密の値なので、`.env` に書くか（UNIT 02。Git には入れない）、公開のときは `fly secrets` で渡します（第12巻 UNIT 05）。

### RustNote ではここで使う

- RustNote の DB のつなぎ先は、`DATABASE_URL` の1つで決まります。何も書かなければ `rustnote.db`（ファイル）、`http://…` なら手元のサーバー、`libsql://…` なら Turso です（図）。
- **RustNote の改善点（この単元で見つかったこと）:** Windows で、開発用のビルドのまま、リモートの libSQL（`http://`・`libsql://`）につなぐと、main のスタックがあふれます（エラー1）。この巻では RustNote のコードは変えず、`cargo run --release` か `RUSTFLAGS` で動かします。また、`DATABASE_URL` の書き間違い（`http://` 忘れ）を、起動のときに注意する仕組みもありません（エラー2）。
- 第12巻では、手元の sqld（この単元と同じイメージ）で、ヘルスチェックやマイグレーションの順番を確かめました。そのときも、release の RustNote か、Docker の中（Linux）の RustNote を使っていました。
- 練習が終わったら、サーバーを止めます（`docker rm -f u09sqld`）。

### 練習問題

**問1:** 例9-3のあと、RustNote を止めて、`DATABASE_URL` を消してから（`Remove-Item Env:DATABASE_URL`）、もう一度起動しました。登録した alice でログインできるでしょうか。

**問2:** `curl … | bash` で入れる手順を、そのまま打つ前に確かめたいことは何でしょうか。

**問3:** 例9-2のサーバーを `docker rm -f u09sqld` で消してから、同じコマンドで動かし直しました。alice は残っているでしょうか。

### 解答と解説

**問1の解答:**

- できません。`DATABASE_URL` が無いと、RustNote はフォルダの `rustnote.db`（ファイル）を使います（RustNote の `src\config.rs` の既定の値）。alice が登録されたのは、手元のサーバー（sqld）の方なので、ファイルの DB にはいません（図の3つのつなぎ先）。

**問2の解答（例）:**

- そのスクリプトが**どこから来たか**（公式のドキュメントに書かれた URL か）と、**何をするか**です。`curl -sSfL https://get.tur.so/install.sh -o install.sh` で先にファイルに保存して、中身を読んでから `bash install.sh` で動かすと、安全です。例9-1のように、取ってくる物・置く場所・書き換えるファイル（`~/.bashrc`）が分かります。

**問3の解答:**

- 残っていません。この本で、コンテナを消して動かし直してから、例9-3と同じ SQL を送ると、次のようになりました。

```text
@@OUT:u09_q3@@
```

- `no such table: users`（`users` という表が無い）で、マイグレーションの前の、空のサーバーに戻っていました。この例では、サーバーの中身をコンテナの中にしか置いていないので、コンテナを消すと一緒に消えます。残したいときは、ボリューム（`-v`）で、コンテナの外に置きます（第12巻 UNIT 10）。

### まとめ

- libSQL は SQLite がもとの DB。sqld はその HTTP のサーバー（この本では 0.24.33）、Turso はそれを貸すクラウド、Turso CLI はクラウドを操作する道具（WSL の中で使う。ログインが要る）。
- Turso CLI は、WSL の中で `curl -sSfL https://get.tur.so/install.sh | bash` で入れる。スクリプトは sqld と turso を `~/.turso` に置き、`~/.bashrc` に PATH を足す。2026-10-31 の最新は v1.0.34。
- 手元の sqld（Docker の `libsql-server`）に、`DATABASE_URL=http://127.0.0.1:8080` と `cargo run --release` で RustNote をつなぐと、3つのマイグレーションが適用され、登録は 303。サーバーに HTTP で SQL を送って確かめられる。
- Windows の開発用ビルドでは、リモートの DB につなぐと main のスタックがあふれる（release か `/STACK:8388608` で動く）。`http://` を忘れるとエラーにならずに `127.0.0.1` のファイル（の別の流れ）に DB ができ、`libsql://` でトークンを忘れると、つなぐ前に止まる。

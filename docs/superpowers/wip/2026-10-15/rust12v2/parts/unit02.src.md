## UNIT 02: multi-stage の Dockerfile

### この単元のゴール

- RustNote の `Dockerfile` の2つの段（作る段と動かす段）が、それぞれ何を入れ、何を入れないかを、本物のビルドと `docker history` で説明できる
- 1段だけの Dockerfile と比べて、イメージの大きさ・入っている物・動かす利用者が、どう違うかを言える
- Docker のキャッシュが段ごとに効くことと、ソースを1行変えたときに何がやり直しになるかを、ビルドの記録で読める。キャッシュのマウントで、やり直しを短くできる
- 実行用の段の決まり（同じ Debian にそろえる・`COPY --from` の場所・`/data` の持ち主）を守らないと何が起きるかを、エラーで確かめられる

### たとえ話でつかむ

パン屋さんの、**工房**と**お店の棚**を考えます。

- 工房（builder の段）には、オーブン・こね台・粉の袋が、全部そろっています。パンを焼くには、どれも要ります。
- お店の棚（実行用の段）に並べるのは、**焼き上がったパンと、値札だけ**です。オーブンや粉の袋まで棚に置いたら、場所を取るうえに、お客さんが勝手に触ってしまうかもしれません。
- 店員さん（利用者 `app`）は、棚の商品（`/app`）を作り替えることはできず、**在庫の置き場**（`/data`）だけを使います。
- 工房とお店の**電気の方式**（Linux の種類と glibc）は、そろえておきます。少し古い方式のお店でも動くことはありますが、約束はありません。方式がまるで違うお店では、まったく動きません（エラー1）。
- 粉の袋の中身（依存クレート）が変わらないなら、**仕込みの途中までを取っておく**と、毎回はじめからこねなくて済みます（キャッシュ）。

| パン屋さん | RustNote の `Dockerfile` |
|---|---|
| 工房 | `FROM rust:1.99.1-trixie AS builder`（Rust と topcoat CLI が入った大きな段） |
| 焼き上がったパンと値札 | 実行ファイル `rustnote` と、CSS の束 `assets/` |
| お店の棚 | `FROM debian:trixie-slim`（小さな Debian の段） |
| 工房から棚へ運ぶ | `COPY --from=builder …`（要る物だけを運ぶ） |
| 店員さん | 利用者 `app`（uid 10001。root ではない） |
| 在庫の置き場 | `/data`（DB のファイルを置く、`app` が書ける唯一の場所） |
| 電気の方式をそろえる | builder と実行用を、同じ Debian 13（trixie）にする |

> **用語:** multi-stage ビルド（マルチステージ・ビルド：1つの Dockerfile に `FROM` を2つ以上書き、前の段で作った物の中から、要る物だけを次の段へ `COPY --from` で運ぶ作り方）

<!-- -->

> **用語:** レイヤー（Docker のイメージを作る1つ1つの段の結果。`RUN`・`COPY` ごとに1つでき、同じ手順・同じ材料なら、前のビルドの結果（キャッシュ）を使い回す）

<!-- -->

> **用語:** glibc（ジー・リブ・シー：Linux の C の基本のライブラリ。Rust の実行ファイルも、ふつうはこれを使う。新しい glibc で作った実行ファイルは、古い glibc の上では動かないことがある）

### 図で理解する

教材の `Dockerfile` の2つの段と、それぞれに入る物です（例2-1・例2-2の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-12-0201" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="300" height="200" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="170" y="44" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">① builder（rust:1.99.1-trixie・2.22GB）</text>
<text x="36" y="72" font-size="11" fill="#0f172a">cargo・rustc（Rust の道具）</text>
<text x="36" y="94" font-size="11" fill="#0f172a">topcoat CLI 0.11.0</text>
<text x="36" y="116" font-size="11" fill="#0f172a">ソース（/app/src など）</text>
<text x="36" y="138" font-size="11" fill="#0f172a">target/（途中の成果物）</text>
<text x="36" y="166" font-size="11" fill="#047857">target/release/rustnote</text>
<text x="36" y="188" font-size="11" fill="#047857">target/release/assets/</text>
<rect x="420" y="20" width="280" height="200" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="560" y="44" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">② 実行用（debian:trixie-slim）</text>
<text x="436" y="72" font-size="11" fill="#0f172a">Debian の土台: 110MB</text>
<text x="436" y="94" font-size="11" fill="#0f172a">ca-certificates: 4.66MB</text>
<text x="436" y="116" font-size="11" fill="#0f172a">利用者 app・/data: 73.7kB</text>
<text x="436" y="138" font-size="11" fill="#047857">/app/rustnote: 17.1MB</text>
<text x="436" y="160" font-size="11" fill="#047857">/app/assets/: 24.6kB</text>
<text x="436" y="196" font-size="12" font-weight="700" fill="#047857">できたイメージ: 170MB</text>
<line x1="320" y1="176" x2="416" y2="150" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0201)"/>
<text x="368" y="146" font-size="10" fill="#475569" text-anchor="middle">COPY --from</text>
<rect x="20" y="240" width="680" height="48" rx="6" fill="#fef2f2" stroke="#ef4444" stroke-width="1.5"/>
<text x="34" y="260" font-size="12" fill="#b91c1c">比べるための1段だけの Dockerfile: イメージは 3.98GB、root で動き、cargo・rustc・ソース・target/ が</text>
<text x="34" y="278" font-size="12" fill="#b91c1c">全部入ったまま（例2-2）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図12-?: 2つの段。大きな builder の段で作り、実行用の段には、実行ファイルと assets/ の2つだけを運ぶ。数は docker history と docker image ls の結果</div>
</div>

ソースを1行変えたとき、どの段がやり直しになるかです（例2-3の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 240" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-12-0202" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="140" height="44" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="90" y="38" font-size="11" fill="#047857" text-anchor="middle">FROM・WORKDIR</text>
<text x="90" y="56" font-size="11" font-weight="700" fill="#047857" text-anchor="middle">CACHED</text>
<rect x="180" y="20" width="160" height="44" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="260" y="38" font-size="11" fill="#047857" text-anchor="middle">cargo install topcoat-cli</text>
<text x="260" y="56" font-size="11" font-weight="700" fill="#047857" text-anchor="middle">CACHED</text>
<rect x="360" y="20" width="110" height="44" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="415" y="38" font-size="11" fill="#c2410c" text-anchor="middle">COPY . .</text>
<text x="415" y="56" font-size="11" font-weight="700" fill="#c2410c" text-anchor="middle">変わった</text>
<rect x="490" y="20" width="210" height="44" rx="6" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="595" y="38" font-size="11" fill="#b91c1c" text-anchor="middle">cargo build --release --locked</text>
<text x="595" y="56" font-size="11" font-weight="700" fill="#b91c1c" text-anchor="middle">全部やり直し 101 秒</text>
<line x1="160" y1="42" x2="176" y2="42" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0202)"/>
<line x1="340" y1="42" x2="356" y2="42" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0202)"/>
<line x1="470" y1="42" x2="486" y2="42" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0202)"/>
<text x="20" y="96" font-size="12" fill="#0f172a">COPY より前の段は、材料が同じなので使い回す。COPY で材料が変わると、そのあとの段は全部やり直し。</text>
<text x="20" y="118" font-size="12" fill="#0f172a">cargo build は、まっさらな target/ から始まるので、依存クレートも含めた 286 個を、もう一度コンパイルする。</text>
<rect x="20" y="140" width="680" height="84" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="34" y="164" font-size="12" font-weight="700" fill="#1d4ed8">キャッシュのマウント（問2）</text>
<text x="34" y="186" font-size="12" fill="#1d4ed8">RUN --mount=type=cache,…,target=/app/target で、target/ をビルドをまたいで取っておく。</text>
<text x="34" y="208" font-size="12" fill="#1d4ed8">ソースを1行変えたときの cargo build: 14.1 秒（依存クレートはコンパイルし直さない）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図12-?: Docker のキャッシュは段ごと。材料（COPY）が変わった段から下は全部やり直しになる。target/ をキャッシュのマウントで残すと、Rust のビルドは変わった所だけで済む</div>
</div>

### まず動かす最小コード

#### 例2-1: 教材の Dockerfile でイメージを作る

> **▼ このファイルがやること（先に日本語で）:** builder の段で、Rust 1.99.1 の入った Debian 13 のイメージに topcoat CLI を入れ、ソースをコピーして release ビルドとアセットの束を作ります。実行用の段では、小さな Debian に証明書と利用者 `app` と `/data` を用意し、builder の段から実行ファイルと `assets/` だけをコピーして、`0.0.0.0:8080` で動かします。

```dockerfile
@@FILE:rustnote/Dockerfile@@
```

教材のフォルダの中身（`.dockerignore` も含む）をコピーしたフォルダで、ビルドしました。

```powershell
docker build --progress=plain -t rustnote:u02 .
```

```text
@@OUT:u02_docker_version@@
```

```text
@@OUT:u02_build_result@@
```

```text
@@OUT:u02_build_steps@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 2つの段は、並んで進む

- 記録の `[builder 3/6]` は builder の段の3つ目の手順、`[stage-1 2/6]` は実行用の段（名前を付けていないので `stage-1`）の2つ目の手順です。2つの段は、互いに待たなくてよい所は、同時に進みます。実行用の段の `COPY --from=builder` だけは、builder の段が終わるのを待ちます。
- `CACHED` は、前のビルド（第13巻 UNIT 10）の結果を、そのまま使ったという意味です。この PC では、`cargo install topcoat-cli` も、実行用の段の準備も、前に作ってあったので、やり直したのは `COPY . .` からあとだけでした。
- `cargo build --release --locked` は 86.8 秒でした。そのあとの `COPY --from=builder … /app/rustnote` まで `CACHED` だったのは、できた実行ファイルが、前のビルドと**1バイトも違わなかった**からです（UNIT 01 で見た再現性。Docker の中では、作る場所と道具がいつも同じ）。

##### 解説2: 実行用の段の中身を見る

`docker history` で、できたイメージのレイヤーと大きさを、上（新しい）から並べました。

```powershell
docker history rustnote:u02 --format "{{.Size}}`t{{.CreatedBy}}"
```

```text
@@OUT:u02_history@@
```

```text
@@OUT:u02_sizes@@
```

- いちばん大きいのは、Debian の土台（110MB）です。RustNote の実行ファイルは 17.1MB、CSS の束は 24.6kB、証明書は 4.66MB でした。builder の段の 2GB 以上は、どこにも入っていません。
- `docker image ls` の 170MB と、`docker history` の合計が少し違うのは、数え方（圧縮・メタデータ）の違いです。

イメージの中で、だれとして動き、どこに書けるかを確かめました。

```powershell
docker run --rm rustnote:u02 sh -c 'id; ls -la /app /data; touch /app/x; touch /data/x && echo "/data に書けた"'
```

```text
@@OUT:u02_inside@@
```

- 利用者は `app`（uid 10001）です。`/app` の中（実行ファイルと assets）は root の物なので、`app` は書き換えられません（`Permission denied`）。書けるのは、`chown app:app` した `/data` だけです。
- もし RustNote に、ファイルを書き換えられてしまうような弱点があっても、実行ファイルそのものは書き換えられません。権限を必要な分だけにしておく考え方です（最小権限）。

### もう一歩: 実務に近い形

#### 例2-2: 1段だけの Dockerfile と比べる

比べるために、作る道具ごとそのまま動かす、1段だけの Dockerfile を書いて、ビルドしました。

```dockerfile
# 比べるための、1段だけの Dockerfile（作る道具ごと、そのまま動かす）
FROM rust:1.99.1-trixie
WORKDIR /app
RUN cargo install topcoat-cli --version 0.11.0 --locked
COPY . .
RUN cargo build --release --locked && topcoat asset bundle --release
ENV HOST=0.0.0.0 PORT=8080 DATABASE_URL=/app/rustnote.db
CMD ["/app/target/release/rustnote"]
```

```text
@@OUT:u02_single_result@@
```

```text
@@OUT:u02_single_inside@@
```

- イメージは **3.98GB** で、2段のイメージ（170MB）の 20 倍以上でした。Rust の道具（2.22GB）に、`target/` の途中の成果物まで、全部入っているからです。Fly.io に送るたび、起動するたびに、この大きさを運ぶことになります。
- **root** で動き、`cargo` と `rustc` が入ったまま、`/app` にはソース・`tests`・`target` が残っています。万一、中に入り込まれたら、その場でプログラムを作って動かせてしまいます。

#### 例2-3: ソースを1行変えて、ビルドし直す

`src/routes/health.rs` の最後に、コメントを1行足して、教材の Dockerfile でビルドし直しました。

```text
@@OUT:u02_rebuild_result@@
```

```text
@@OUTGREP:u02_rebuild_steps:^#(8|9|10|11|12) @@
```

- `COPY . .` より前（`WORKDIR`・`cargo install topcoat-cli`）は `CACHED` でした。`COPY . .` で材料が変わったので、そのあとの `cargo build` は、依存クレートを含めて全部やり直しになり、101.1 秒かかりました（`Compiling` の行が 286 行）。
- `cargo build` は、レイヤーの中で、まっさらな `target/` から始まるからです。手元の PC の `cargo build` なら、変わった RustNote の部分だけをコンパイルし直すので、数秒で終わります。
- ビルドの記録には、`info: downloading 2 components` という行もありました。`rust-toolchain.toml` の `components = ["rustfmt", "clippy"]` のため、イメージに入っていない rustfmt と clippy を、ビルドのたびに取りに行っているのです。実行ファイルを作るだけなら要らない物です（改善点）。

### よくあるエラーと読み方

#### エラー1: 実行用の段を alpine にした（exec … no such file or directory）

小さなイメージとしてよく使われる alpine を、実行用の段に使ってみました（alpine には `useradd` が無いので、利用者を作る行も alpine の書き方に直しました）。

```diff
@@OUT:u02_err_alpine_diff@@
```

```text
@@OUT:u02_err_alpine_build_result@@
```

```text
@@OUT:u02_err_alpine@@
```

- **何が起きたか:** ビルドは通りましたが、起動すると `exec /app/rustnote: no such file or directory`（そのファイルがありません）で止まりました。ところが、ファイルはあります。

```text
@@OUT:u02_err_alpine_ls@@
```

- **読み方:** 実行ファイルは `/app/rustnote` にあります。無いのは、実行ファイルを読み込むための道具（glibc の `ld-linux-aarch64.so.1`）です。alpine は glibc ではなく musl という別の C のライブラリを使うので、glibc 向けに作った RustNote の実行ファイルは動きません。「ファイルが無い」という文は、実行ファイルそのものではなく、それを読み込む道具が無いという意味でした。
- **直し方:** 実行用の段は、builder の段と同じ Debian（`debian:trixie-slim`）にします。どうしても alpine にしたいなら、builder の段も musl 向け（この PC の Docker なら `aarch64-unknown-linux-musl` のターゲット）で作ります。

比べるために、実行用の段だけを1つ古い Debian 12（`debian:bookworm-slim`。glibc 2.36）にしたイメージも作りました。

```text
@@OUT:u02_err_glibc@@
```

- こちらは、glibc 2.41 の trixie で作った実行ファイルが、glibc 2.36 の上でも、エラー無く動きました。RustNote が、新しい glibc にしか無い機能を使っていなかったからです。ただし、これはたまたまで、Rust や依存クレートの版が上がると、`GLIBC_2.38 not found` のようなエラーで止まることがあります。だから、builder と実行用は、同じ版にそろえておきます（`Dockerfile` の先頭のコメントのとおり）。

#### エラー2: `COPY --from` の場所を間違えた

実行用の段の `COPY --from=builder /app/target/release/rustnote` を、`/app/target/rustnote`（`release` が抜けた）にしてビルドしました。

```text
@@OUT:u02_err_copy@@
```

- **何が起きたか:** ビルドの最後の段で、「`/app/target/rustnote` が見つかりません（not found）」と止まりました。builder の段の中に、その場所のファイルが無いからです。
- **読み方:** `failed to calculate checksum of ref …` は、Docker がコピーする物の指紋を計算しようとして、物が無かった、という意味です。止まるのはビルドのときなので、壊れたイメージが公開されることはありません。CI の `docker build` で、毎回確かめられます。
- **直し方:** builder の段で、物がどこにできるかを確かめます（release ビルドは `target/release/`）。

#### エラー3: `/data` を作らずに、root でない利用者で動かした

実行用の段の `RUN useradd … && mkdir -p /data && chown app:app /data` から、`/data` を作る部分を消してビルドし、起動しました。

```text
@@OUT:u02_err_nodata@@
```

- **何が起きたか:** ビルドは通りましたが、起動すると「DB（/data/rustnote.db）を開けませんでした」で止まりました。原因の `14` は、SQLite の「ファイルを開けません（SQLITE_CANTOPEN）」の番号です。
- **読み方:** `/data` が無いので、DB のファイルを作る場所がありません。root なら `/` の下に作れるかもしれませんが、利用者 `app` には書く権限がありません。最小権限にしたので、書ける場所を、こちらで用意する必要があるのです。
- **直し方:** `mkdir -p /data && chown app:app /data` で、`app` が書けるフォルダを用意します。Fly.io で volume を使うときも、同じ `/data` に付けます。本番の RustNote は Turso を使うので、`/data` は使いません（UNIT 05・09）。

### RustNote ではここで使う

- **CI:** `ci.yml` の `docker` のジョブは、テストが通ったあとに `docker build -t rustnote Rust/rustnote` をします。Dockerfile が壊れていないか（エラー2のような書き間違い）を、毎回確かめられます（第13巻 UNIT 10）。
- **Fly.io:** `fly deploy` は、`fly.toml` の `[build] dockerfile = "Dockerfile"` で、この Dockerfile からイメージを作って公開します（UNIT 04・08）。
- **改善点（この単元で見つかったこと）:**
  - ソースを1行変えるだけで、依存クレートまで全部コンパイルし直します（例2-3）。キャッシュのマウントで短くできます（問2）。
  - `rust-toolchain.toml` の `components` のため、ビルドのたびに rustfmt と clippy をダウンロードしています。

### 練習問題

**問1:** 教材の Dockerfile で、`RUN cargo install topcoat-cli …` を `COPY . .` より**前**に書いているのは、なぜでしょうか。例2-3の記録から考えてください。

**問2:** ソースを1行変えたときのビルドを短くするために、`cargo build` の `target/` を、ビルドをまたいで取っておく（キャッシュのマウント）ように、Dockerfile を直してください。直す前と後で、時間を比べます。

**問3:** 例2-1で、`/app` に書けず、`/data` には書けました。もし実行用の段で `USER app` を書き忘れると、どうなるでしょうか。

### 解答と解説

**問1の解答:**

- Docker は、段を上から順に見て、「手順も材料も前と同じ」ならキャッシュを使います。1つでも変わると、そこから下は全部やり直しです。
- `cargo install topcoat-cli` は、ソースと関係が無く、数分かかる手順です（第13巻 UNIT 10 の最初のビルドでは 98 秒）。`COPY . .` より前に置くと、ソースを変えても `CACHED` のままです（例2-3）。もし `COPY . .` のあとに置いていたら、ソースを1行変えるたびに、topcoat CLI のインストールからやり直しになります。
- 時間のかかる手順で、あまり変わらない物ほど、上に置きます。

**問2の解答:**

`cargo build` の `RUN` に、ダウンロードしたクレートの置き場（`/usr/local/cargo/registry`）と `target/` を、キャッシュのマウント（`--mount=type=cache`）として付けました。マウントした所はイメージに残らないので、できた実行ファイルと `assets/` は、同じ `RUN` の中で `/out` にコピーし、実行用の段は `/out` から運びます。`id=` は、ほかのプロジェクトのキャッシュと混ざらないための名前です。

```diff
@@OUT:u02_cache_diff@@
```

```text
@@OUT:u02_cache_first_result@@
```

```text
@@OUTGREP:u02_cache_first_steps:^#[0-9]+ \[builder 5/@@
```

ソースを1行変えて、もう一度ビルドしました。

```text
@@OUT:u02_cache_second_result@@
```

```text
@@OUT:u02_cache_check@@
```

- 1回目（キャッシュが空）は、`cargo build` の段が 95.0 秒（依存クレートを含めて全部）でした。ソースを1行変えた2回目は **14.1 秒**で、コンパイルしたのは RustNote の1つだけ（`Compiling` の行が1行）でした。ビルド全体も 100 秒 → 24 秒です。
- できたイメージは 170MB で、`/health` も 200 でした。キャッシュのマウントは、イメージの中身を変えずに、作る時間だけを短くします。
- ただし、キャッシュのマウントは、ビルドする PC（や CI の機械）の中に残る物です。毎回まっさらな機械でビルドする CI では、効きません。その場合は、依存クレートだけを先にビルドする段を分ける（`cargo-chef` などの道具を使う）方法があります。

**問3の解答:**

- 例2-2の1段だけのイメージと同じく、root で動きます。`/app` の実行ファイルも、`/etc` の中も、RustNote のプロセスから書き換えられる状態になります。
- `USER app` は、RustNote の動きには何の影響も無い1行ですが、「万一のときに、被害を小さくする」ための大事な1行です。例2-1の `id` の結果（`uid=10001(app)`）で、毎回確かめられます。

### まとめ

- RustNote の `Dockerfile` は、builder の段（`rust:1.99.1-trixie`・2.22GB）で release ビルドとアセットの束を作り、実行用の段（`debian:trixie-slim`）に、実行ファイル（17.1MB）と `assets/`（24.6kB）だけを運ぶ。できたイメージは 170MB で、利用者 `app` が動かし、書けるのは `/data` だけ。1段だけだと 3.98GB・root・道具入り。
- Docker のキャッシュは段ごと。`COPY . .` で材料が変わると、そのあとの `cargo build` は依存クレートから全部やり直し（101 秒）。時間がかかって変わらない手順は上に置き、`target/` はキャッシュのマウントで残せる。
- builder と実行用の Linux はそろえる。alpine（musl）では `exec … no such file or directory` で動かない。1つ古い Debian では今は動いたが、約束は無い。`COPY --from` の場所の書き間違いはビルドで止まり、`/data` を作り忘れると起動で止まる（SQLite の 14）。

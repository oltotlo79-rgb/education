## UNIT 10: health・初回起動・診断

### この単元のゴール

- `GET /health` が何を確かめているか、200 と 503 がそれぞれ何を伝えるかを、テストと `curl.exe` で、外から確かめられる
- Fly.io のヘルスチェックの設定（`fly.toml` の `[[http_service.checks]]`）を読み、`path` がずれると何が起きるかを説明できる
- 起動に要るもの（設定・アセットの束・DB・マイグレーション・待ち受ける場所）を、起動の順番どおりに全部確かめる**診断のプログラム**を書き、表示から原因を切り分けられる
- `main` の `?` は**最初の失敗で止まり**、診断は**全部の問題を並べる**、という違いを使い分けられる
- 症状（すぐ終わる・503・404・500）から、まず見る場所を決められる

### たとえ話でつかむ

病院には、2種類の確かめがあります。

- 入院している人の**脈**を、看護師さんが決まった間隔で測ります。短く、軽く、何度も測ります。脈が無ければ、すぐに人を呼びます。このとき、腕に触らずに「元気そうですね」と言うだけでは、測ったことになりません。
- **健康診断**は、たまに、時間をかけて、項目を順に全部測ります。血圧が高くても、そこで帰らずに最後まで測り、結果を1枚の紙に並べます。
- 結果の紙には、項目ごとに「基準内」「要再検査」と、**次に何をすればよいか**が書いてあります。

| 病院 | RustNote |
|---|---|
| 看護師さんが脈を測る（決まった間隔で） | Fly.io が `GET /health` を送る（`fly.toml` の `interval = "30s"`） |
| 腕に触って測る | `db.ping()`（`SELECT 1`）で、DB に本当に問い合わせる |
| 脈が無い → 人を呼ぶ | 503 → Fly.io が、そのマシンにリクエストを送らなくなる |
| 健康診断で、項目を順に全部 | `examples/u10_doctor.rs`（起動に要るものを、順に全部） |
| 「要再検査」と、次にすること | `✗` と、直し方のヒント |

> **用語:** ヘルスチェック（health check：アプリが動いているかを、外から決まった間隔で確かめること。RustNote では `GET /health` に答える）

<!-- -->

> **用語:** 503 Service Unavailable（「今は、仕事を受けられません」という状態コード。サーバーは返事ができるが、頼っている物（ここでは DB）が使えないときに返す）

<!-- -->

> **用語:** 診断（ここでは、起動に要るものを1つずつ確かめて、どこが悪いかを表示する小さなプログラム。英語の道具では doctor という名前がよく使われる）

### 図で理解する

`GET /health` の流れです。Fly.io（第12巻）は、決まった間隔で RustNote に `GET /health` を送り、答えの状態コードで、そのマシンにリクエストを送るかを決めます。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs><marker id="u10a" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker></defs>
<rect x="20" y="30" width="190" height="110" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="115" y="58" font-size="13" font-weight="700" fill="#1d4ed8" text-anchor="middle">Fly.io のプロキシ</text>
<text x="115" y="82" font-size="12" fill="#0f172a" text-anchor="middle">30 秒ごとに</text>
<text x="115" y="102" font-size="12" fill="#0f172a" text-anchor="middle">GET /health を送る</text>
<text x="115" y="122" font-size="11" fill="#475569" text-anchor="middle">（5 秒で時間切れ）</text>
<line x1="212" y1="85" x2="262" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#u10a)"/>
<rect x="265" y="30" width="190" height="110" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="58" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">RustNote</text>
<text x="360" y="82" font-size="12" fill="#0f172a" text-anchor="middle">routes/health.rs</text>
<text x="360" y="102" font-size="12" fill="#0f172a" text-anchor="middle">db.ping() を呼ぶ</text>
<text x="360" y="122" font-size="11" fill="#475569" text-anchor="middle">アクセスログには書かない</text>
<line x1="457" y1="85" x2="507" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#u10a)"/>
<text x="482" y="76" font-size="10" fill="#475569" text-anchor="middle">SELECT 1</text>
<rect x="510" y="30" width="190" height="110" rx="10" fill="#f1f5f9" stroke="#64748b" stroke-width="2"/>
<text x="605" y="58" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">DB</text>
<text x="605" y="82" font-size="12" fill="#0f172a" text-anchor="middle">手元では SQLite のファイル</text>
<text x="605" y="102" font-size="12" fill="#0f172a" text-anchor="middle">本番では Turso</text>
<line x1="330" y1="142" x2="200" y2="176" stroke="#475569" stroke-width="2" marker-end="url(#u10a)"/>
<line x1="390" y1="142" x2="520" y2="176" stroke="#475569" stroke-width="2" marker-end="url(#u10a)"/>
<rect x="20" y="180" width="330" height="92" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="185" y="206" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">DB に届いた → 200 {"status":"ok"}</text>
<text x="185" y="230" font-size="12" fill="#0f172a" text-anchor="middle">合格: このマシンにリクエストを送る</text>
<text x="185" y="252" font-size="11" fill="#475569" text-anchor="middle">合否は状態コードで決まる（200 番台なら合格）</text>
<rect x="370" y="180" width="330" height="92" rx="10" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="535" y="206" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">届かない → 503 {"status":"unavailable"}</text>
<text x="535" y="230" font-size="12" fill="#0f172a" text-anchor="middle">不合格: 送らない（再起動はしない）</text>
<text x="535" y="252" font-size="11" fill="#475569" text-anchor="middle">RustNote のログに [error] の行が出る</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: GET /health の流れ。RustNote は DB に SELECT 1 を問い合わせ、届けば 200、届かなければ 503 を返す。Fly.io は状態コードで合否を決め、不合格のマシンにはリクエストを送らない</div>
</div>

動かないときに、まず見る場所の地図です。この単元の例と、UNIT 06 で、それぞれ本物の表示を確かめます。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 340" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="16" width="680" height="34" rx="6" fill="#0f172a"/>
<text x="130" y="38" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">症状</text>
<text x="410" y="38" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">まず見る所</text>
<text x="630" y="38" font-size="12" font-weight="700" fill="#ffffff" text-anchor="middle">確かめる所</text>
<rect x="20" y="54" width="680" height="54" rx="6" fill="#f8fafc" stroke="#e2e8f0"/>
<text x="34" y="86" font-size="12" fill="#0f172a">「Error:」と出て、すぐ終わる</text>
<text x="260" y="78" font-size="11" fill="#0f172a">エラーの文と Caused by を読む。</text>
<text x="260" y="96" font-size="11" fill="#0f172a">問題を全部並べたいときは、診断を動かす</text>
<text x="630" y="86" font-size="11" fill="#475569" text-anchor="middle">例10-2・UNIT 06</text>
<rect x="20" y="112" width="680" height="54" rx="6" fill="#ffffff" stroke="#e2e8f0"/>
<text x="34" y="136" font-size="12" fill="#0f172a">「起動しました」のあとに Error</text>
<text x="34" y="154" font-size="11" fill="#475569">（os error 10048）</text>
<text x="260" y="136" font-size="11" fill="#0f172a">待ち受ける場所。ほかのプログラムが</text>
<text x="260" y="154" font-size="11" fill="#0f172a">同じポートを使っていないか</text>
<text x="630" y="144" font-size="11" fill="#475569" text-anchor="middle">UNIT 06・エラー3</text>
<rect x="20" y="170" width="680" height="54" rx="6" fill="#f8fafc" stroke="#e2e8f0"/>
<text x="34" y="202" font-size="12" fill="#0f172a">/health が 503</text>
<text x="260" y="194" font-size="11" fill="#0f172a">DB（URL・トークン・届くか）。</text>
<text x="260" y="212" font-size="11" fill="#0f172a">ログの [error] の行に、届かない理由</text>
<text x="630" y="202" font-size="11" fill="#475569" text-anchor="middle">例10-1</text>
<rect x="20" y="228" width="680" height="54" rx="6" fill="#ffffff" stroke="#e2e8f0"/>
<text x="34" y="260" font-size="12" fill="#0f172a">/health が 404</text>
<text x="260" y="252" font-size="11" fill="#0f172a">確かめる側の path（fly.toml）。</text>
<text x="260" y="270" font-size="11" fill="#0f172a">アプリは元気なのに、不合格にされる</text>
<text x="630" y="260" font-size="11" fill="#475569" text-anchor="middle">エラー2</text>
<rect x="20" y="286" width="680" height="54" rx="6" fill="#f8fafc" stroke="#e2e8f0"/>
<text x="34" y="318" font-size="12" fill="#0f172a">画面は出るのに、ボタンで 500</text>
<text x="260" y="310" font-size="11" fill="#0f172a">まず /health。503 なら DB。200 なら、</text>
<text x="260" y="328" font-size="11" fill="#0f172a">ログの [error] の行の「&lt;-」をたどる</text>
<text x="630" y="318" font-size="11" fill="#475569" text-anchor="middle">問1</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: 症状から、まず見る場所を決める地図。起動の前の失敗は、エラーの文と診断で、起動のあとの失敗は、/health の状態コードとログの [error] の行で切り分ける</div>
</div>

### まず動かす最小コード

#### 例10-1: /health が、DB に届くかどうかで答えを変えるか確かめる

RustNote の `src/routes/health.rs` です（全部です）。

```rust
@@FILE:book/rustnote/src/routes/health.rs@@
```

> **▼ このコードがやること（先に日本語で）:** テスト用の DB で RustNote のルーターを作り、`GET /health` を送って、状態コードと本文を表示します。次に、何も待ち受けていない `http://127.0.0.1:9` を DB の場所にしたルーターを作り、同じように `GET /health` を送ります。1つ目は 200、2つ目は 503 になることを、`assert_eq!` でも確かめます。

```rust
@@FILE:book/rustnote/tests/u10_health.rs@@
```

```powershell
cargo test --test u10_health -- --nocapture
```

```text
@@OUT:u10_health@@
```

本物のサーバーでも確かめます。`cargo run` で RustNote を起動し、別の窓から送ります。

```text
@@OUT:u10_run4_health@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: DB に届けば 200、届かなければ 503

- ふつうの DB では `200 OK {"status":"ok"}`、届かない DB では `503 Service Unavailable {"status":"unavailable"}` でした。`health` は、`app_state(cx).db.ping()`（`src/db.rs` の `Db::ping`。`SELECT 1` を問い合わせる）の結果で、答えを分けています。
- 503 のとき、RustNote の窓（標準エラー出力）に `[error] ヘルスチェックで DB に接続できません: …` の行が出ました。`os error 10061` は、Windows の「接続を拒否されました」です。9 番ポートでは、何も待ち受けていないからです。**状態コードは「悪いかどうか」、ログの行は「なぜ悪いか」**を伝えます。
- `Db::open` は、リモートの DB では、まだ接続しません（`expect` の文のとおり、開くだけなら成功します）。だから、起動のときに DB が止まっていても、気づけるのは `/health` か、最初の DB の操作のときです。

##### 解説2: 届かないと分かるまでの時間

- テスト全体は **2.08 秒**かかりました（あとのエラー1の、DB に問い合わせない版は 0.03 秒です）。この PC では、拒否されたと分かるまでに、約2秒かかっています。
- `fly.toml` のヘルスチェックは `timeout = "5s"` です。ネットワークの向こうの DB が、拒否もせずに黙ってしまうと、`/health` の答えが5秒を超え、Fly.io は時間切れとして不合格にします。どちらにしても、そのマシンには、リクエストが送られなくなります。

### もう一歩: 実務に近い形

#### 例10-2: 起動に要るものを、順に全部確かめる診断

UNIT 06 で見たとおり、`main` は、起動の順番（設定 → アセットの束 → DB → マイグレーション → 待ち受け）の途中で失敗すると、`?` で**その場で止まります**。止まるのは正しいのですが、問題が2つあるときは、1つ直して起動し、また止まって2つ目に気づく、を繰り返すことになります。そこで、**同じ順番で、全部を確かめて並べる**プログラムを作ります。

> **▼ このコードがやること（先に日本語で）:** `.env` と環境変数から設定を作り、①設定 ②アセットの束 ③DB を開く ④DB に届く（`ping`）⑤マイグレーションがいくつ適用済みか ⑥待ち受ける場所（`HOST:PORT`）を押さえられるか、を順に確かめます。1つごとに `✓`（通った）か `✗`（だめ。直し方のヒント付き）を表示し、1つでも `✗` があれば、終了コード 1 で終わります。設定が作れないときだけは、そのあとを確かめられないので、そこで終わります。

```rust
@@FILE:book/rustnote/examples/u10_doctor.rs@@
```

`examples/` に置いたファイルは、`cargo run --example 名前` で動かせる、アプリとは別の小さな実行ファイルになります（`rustnote` の `lib.rs` の公開された部品を、そのまま使えます）。ふだんの `dev.db` で動かします。

```powershell
$env:DATABASE_URL = "dev.db"
cargo run --example u10_doctor
```

```text
@@OUT:u10_doctor_ok@@
```

次に、**2つ同時に壊します**。アセットの束を消し（`topcoat asset clean`）、DB の場所を、無いフォルダの中にします。まず、ふつうの `cargo run`、次に診断です。

```text
@@OUT:u10_two_problems@@
```

##### 解説1: main は1つ目で止まり、診断は2つとも見つける

- `cargo run` は、アセットの束が無いことだけを伝えて止まりました。DB の場所の間違いは、束を作り直して、もう一度起動するまで分かりません。
- 診断は、`✗ アセットの束` と `✗ DB を開く` の**2つを並べ**、それぞれに直し方を付けました。終了コードは、どちらも 1 です。
- 診断は、問題があっても先へ進むように、1つの確かめごとに `Result` を受け取り、`report` で表示するだけにしています。`?` を使わないのが、`main` との違いです。

##### 解説2: 壊し方ごとの、✗ の出る場所

ほかの壊し方でも、1つずつ動かしました（表は、本物の表示の要点です）。

| 壊し方 | ✗ になった行 | 表示の要点 |
|---|---|---|
| `DATABASE_URL` を `libsql://rustnote-example.turso.io` にして、トークンを入れない | `✗ 設定`（ここで終わる） | `DATABASE_AUTH_TOKEN が設定されていません` |
| `topcoat asset clean` で束を消す | `✗ アセットの束` | `無い（topcoat asset bundle を実行する）` |
| `DATABASE_URL` を `nofolder/dev.db` にする | `✗ DB を開く` | `Unable to open connection to local database nofolder/dev.db: 14` |
| `DATABASE_URL` を `http://127.0.0.1:9` にする | `✗ DB に届く`（開くのは `✓`） | `os error 10061` |

- リモートの DB は、**開くのは成功し、届くかで初めて分かる**ことが、ここでも見えます（例10-1の解説1）。だから、診断は「開く」と「届く」を別の行にしています。
- 診断は、本物のアプリと同じ `Config::from_env`・`Db::open`・`listen_address` を呼びます。確かめ方をアプリと別に書くと、アプリだけが変わったときに、診断が古い決まりで「✓」を出してしまうからです。

> **注意:** アセットの束は、アプリ（`target/debug/rustnote.exe`）の隣の `assets` を見ます。Topcoat の `AssetBundle::load()` は「今動いている実行ファイルの隣」を見るので、`target/debug/examples/` にある診断から呼ぶと、違う場所を見てしまいます。そこで、診断では `AssetBundle::load_dir` に、1つ上のフォルダの `assets` を渡しています。

#### 例10-3: 初回起動を、診断から通しで行う

新しい DB（`first.db`）で、初めて起動する流れです。順に実行します。

1. アセットの束を作る

```text
@@OUT:u10_run1_bundle@@
```

2. 起動の前に、診断を動かす

```text
@@OUT:u10_run2_doctor@@
```

3. 別の窓で `cargo run` を実行して起動し、そのままにしておく（あとで止めたときまでの表示です）

```text
@@OUT:u10_run3_server@@
```

4. もう1つの窓から、外から確かめる（例10-1と同じ `curl.exe -i http://127.0.0.1:3000/health` で、`200 OK` と `{"status":"ok"}`）

5. ブラウザで `http://127.0.0.1:3000/` を開き、新規登録から始める（第10巻）

- 2の診断では、マイグレーションが `0/3` でした。まだ表が無い、新しい DB だからです。これは `✗` ではありません。3の起動で、`0001_users`・`0002_notes`・`0003_sessions` の3つが流れました（UNIT 07）。
- 診断は、DB を開いただけで `first.db` を作りました（`Test-Path first.db` が `True`）。ローカルの SQLite は、開くときにファイルが無ければ作ります（UNIT 06）。
- 3の表示に、`/health` の行はありません。待つため・確かめるために何度も送りましたが、アクセスログは `/health` を書きません（問3）。`GET /healthz` と `GET /login` の行は、このあとのエラー2と問3で送ったものです。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: DB に問い合わせない /health

`/health` を「サーバーが返事をすればよい」と考えて、DB に触らずに、いつも `ok` を返すように書いた場合です（作業用のコピーで、`health` の中身だけを変えています）。

```rust
#[route(GET)]
pub async fn health(_cx: &Cx) -> Result<(StatusCode, Json<Health>)> {
    // DB には問い合わせず、いつも ok と答える
    Ok((StatusCode::OK, Json(Health { status: "ok" })))
}
```

```text
@@OUT:u10_err_naive@@
```

- **何が起きたか:** コンパイルは、問題なく通ります。届かない DB でも `200 OK {"status":"ok"}` でした。気づけたのは、例10-1のテストに `assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE)` があったからです（`left: 200` が実際の値、`right: 503` が期待した値）。`println!` だけのテストなら、このテストも「ok」で通ってしまいます。
- **困ること:** Fly.io は、DB が使えないマシンにも「合格」として、リクエストを送り続けます。使う人には、ログインのボタンを押したときに 500 が返ります（問1）。脈を測るふりをして、腕に触っていない状態です。
- **直し方:** `/health` では、アプリが仕事をするのに**欠かせない物**（RustNote では DB）に、軽く問い合わせます。重い処理（全部のノートを数えるなど）は入れません。30 秒ごとに呼ばれるからです。

#### エラー2（エラーが出ない間違い）: fly.toml の path を /healthz と書いた

ヘルスチェックの URL を、ほかのアプリの習慣で `/healthz` と書いてしまった場合です。Fly.io の本物のアカウントでは動かさず、Fly.io が送るのと同じリクエストを、手元で送って確かめます。

```toml
  [[http_service.checks]]
    grace_period = "10s"
    interval = "30s"
    method = "GET"
    path = "/healthz"   # 間違い。RustNote の URL は /health
    timeout = "5s"
```

```text
@@OUT:u10_run5_healthz@@
```

- **何が起きたか:** 404 です。RustNote は元気で、DB にも届いているのに、確かめる側が、無い URL を見ています。Fly.io の説明書（docs.fly.io の health checks）では、合格は 200 番台の答えだけで、不合格のマシンには、合格するまでリクエストを送りません。つまり、**元気なアプリに、誰もたどり着けなく**なります。
- **気づき方:** 例10-3の3の表示に、`GET /healthz -> 404 Not Found` の行がありました。アクセスログが書かないのは `/health` ちょうどだけなので、`/healthz` は書かれます。ログに、30 秒ごとに同じ 404 の行が並んでいたら、これを疑います。
- **直し方:** `fly.toml` の `path` を、`routes` の一覧（UNIT 08）と同じ `/health` にします。手元で `curl.exe -i` を送り、200 になる URL を書きます。

#### エラー3（読み違え）: 動いている RustNote の横で、診断を動かした

例10-3の3で RustNote を動かしたまま、もう1つの窓で診断を動かした場合です。

```text
@@OUT:u10_run6_doctor_busy@@
```

- **何が起きたか:** `✗ 待ち受ける場所` です。`os error 10048` は、「そのアドレスは、もう使われています」です。使っているのは、隣の窓の RustNote 自身です。**これは問題ではありません。**
- **読み方:** 診断の `✗` は「このまま、もう1つ起動すると、ここで止まる」という意味です。今動いている RustNote を止めるか、別のポートで確かめます。同じ窓のまま（`DATABASE_URL` は `first.db` のまま）、`PORT` だけを変えると、`✓` になります。

```text
@@OUT:u10_run6_doctor_3001@@
```

- UNIT 06 のエラーで見たとおり、ふつうの `cargo run` は、ここで「起動しました」と出してから止まります。診断なら、起動する前に、ポートが使えるかを確かめられます。

### RustNote ではここで使う

- `GET /health` は `src/routes/health.rs` です（例10-1）。`AppState` が `db` を持っているのは、このためです（`src/state.rs` のコメント「DB そのもの（GET /health の確認に使う）」）。
- アクセスログ（`src/routes.rs` の `access_log`）は、`/health` だけを書きません。

```rust
/// アクセスログ: 「GET /notes -> 200 OK (1.2ms)」のように標準エラー出力へ書く
#[layer]
async fn access_log(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let started = std::time::Instant::now();
    let result = next.run(cx, body).await;
    // ヘルスチェックは数十秒ごとに来るので、ログに出さない
    if uri(cx).path() != "/health" {
        // …（状態コードを文字にして、1行書く）
    }
    result
}
```

- `fly.toml` のヘルスチェックは、次のとおりです。`grace_period = "10s"` は、マシンが起動してから最初に確かめるまで待つ時間です。マイグレーションを流している間に、不合格にされないための余裕です。

```toml
  # ヘルスチェック: GET /health が 200 を返せば正常
  [[http_service.checks]]
    grace_period = "10s"
    interval = "30s"
    method = "GET"
    path = "/health"
    timeout = "5s"
```

- `README.md` の URL の表にも、`GET /health` が「200 `{"status":"ok"}`（DB に届かなければ 503）」と書いてあります。本物の Fly.io での確かめは、第12巻 UNIT 06（準備中）で扱います。
- 診断（`examples/u10_doctor.rs`）は、この本の例として、コピーした RustNote に足したものです。`Rust/rustnote` には入れていません（RustNote のコードは変えていません）。

### 練習問題

**問1:** DB に届かないとき、`/health` 以外の URL は何を返すでしょうか。`GET /login`・`GET /notes`（ログインしていない）・`POST /login`（メールアドレスとパスワードを送る）を予想してから、例10-1と同じ「届かない DB」のルーターで確かめてください。

**問2:** `/health` は、DB に届かないとき 503 を返します。500（Internal Server Error）や、`200 {"status":"unavailable"}` にしないのは、なぜでしょうか。

**問3:** Fly.io は、30 秒ごとに `/health` を送ります。アクセスログが `/health` を書かないことで、マシン1台あたり、1日に何行が減るでしょうか。また、書かないことで、困ることはないでしょうか。

### 解答と解説

**問1の解答:**

```rust
@@FILE:book/rustnote/tests/u10_q.rs@@
```

```text
@@OUT:u10_q@@
```

- `GET /login` は **200**、`GET /notes` は **303**（ログイン画面へ）でした。どちらも、DB に問い合わせずに答えられるからです。`src/auth.rs` の `load_current_user` は、クッキーのトークンが無ければ、DB に問い合わせずに「ログインしていない」と答えます。
- `POST /login` は、メールアドレスで DB を探そうとして、**約2秒待ったあとに 500** になりました。ログの `<-` の鎖（第4巻 UNIT 05）を右へたどると、`DB の操作に失敗しました` の理由が `os error 10061` だと分かります。
- つまり、**画面が出ることは、動いている証拠になりません**。使う人は、ログインの画面までは進めて、ボタンを押したところで失敗します。DB の具合を伝えているのは `/health` の 503 だけです。図9-26の「画面は出るのに、ボタンで 500」が、この状態です。

**問2の解答:**

- 500 は「このリクエストの処理の途中で、思っていなかった失敗が起きた」です。RustNote では、`AppError` の内側のエラーが 500 になり、ログに `<-` の鎖が出ます（問1の `POST /login`）。直すべき**不具合か、事故**の印です。
- 503 は「サーバーは返事ができるが、今は仕事を受けられない」です。頼っている DB が戻れば、コードを直さなくても合格に戻ります。`/health` が伝えたいのは、こちらの意味です。
- `200 {"status":"unavailable"}` は、いちばん困ります。Fly.io のヘルスチェックは、**状態コードで**合否を決めます（200 番台なら合格。設定の項目に、本文を確かめるものはありません）。本文の `unavailable` は人のための文で、機械には合格に見えます。エラー1と同じく、DB の無いマシンにリクエストが送られ続けます。

**問3の解答:**

- 1日は 86,400 秒なので、86,400 ÷ 30 = **2,880 行**です（マシンが動いている間。ヘルスチェック1つあたり）。台数が増えれば、その分だけ増えます。ほかのリクエストの行が、ヘルスチェックの行に埋もれてしまいます。
- 手元でも確かめました。`/health` を5回送ってから、ログイン画面を1回開きました。

```text
@@OUT:u10_run7_client@@
```

- 例10-3の3の RustNote の表示には、`/health` の行が1つも無く、`GET /login -> 200 OK` だけが出ています。
- 困ることは、ほとんどありません。**不合格のとき**は、`health.rs` が `[error] ヘルスチェックで DB に接続できません: …` を自分で書くからです（例10-1）。書かないのは「いつもどおり合格」の行だけで、知りたい行は残ります。ただし、`/health` に来たリクエストの速さを調べたいときには、この行が無いので、別の方法（第11巻 UNIT 07（準備中）のメトリクス）を使います。

### まとめ

- `GET /health` は、DB に `SELECT 1` を問い合わせ、届けば 200、届かなければ 503 を返す。Fly.io は状態コードで合否を決め、不合格のマシンにはリクエストを送らない。DB に触らない `/health` や、`path` のずれは、エラーが出ないまま事故になるので、テストの `assert_eq!` と、`curl.exe -i` で確かめる。
- 診断（`examples/u10_doctor.rs`）は、起動と同じ順番で、設定・アセットの束・DB を開く・届く・マイグレーション・待ち受ける場所を全部確かめ、`✓`・`✗` とヒントを並べる。`main` は最初の失敗で止まり、診断は全部の問題を一度に見せる。
- 初回起動は「束を作る → 診断 → 起動（マイグレーションが流れる）→ /health を外から確かめる → ブラウザ」の順。動かないときは、起動の前ならエラーの文と診断、起動のあとなら /health の状態コードとログの [error] の行で切り分ける。

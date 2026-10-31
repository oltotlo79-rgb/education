## UNIT 06: health check と起動の順番

### この単元のゴール

- RustNote の `GET /health` が何を確かめ、どんなときに 200・503 を返すかを、DB を止めたり動かしたりして確かめられる
- Fly.io のヘルスチェックが失敗したとき、何が起き、何が起きないか（振り分けを止める・マシンは動かし直さない）を、公式のドキュメントにそって言える
- 起動の順番（設定 → 束 → DB → マイグレーション → 待ち受け）のどこで止まると、プロセスが終わり、Fly.io の「動かし直し」（restart policy）に任されるかを説明できる
- `grace_period` を、実際に測った起動の時間から決められる

### たとえ話でつかむ

**お店の開店と、見回りの係**を考えます。

- 開店の前に、店長は、決まりの確認 → 看板 → 倉庫を開ける → 棚を整える、の順に準備します。倉庫の鍵が開かなければ、店は開けず、その日はいったん帰ります（プロセスが終わる）。本部は「店が閉まっていたら、少し待って、もう一度開けに行かせる」と決めています（restart policy）。
- 開店のあと、本部の**見回りの係**は、30 秒ごとに「倉庫まで歩いて行けますか」と聞きます（`GET /health`）。倉庫が使えなければ「使えません」（503）と答えます。
- 見回りの係が「使えません」を聞いたら、本部は、その店に**お客さんを案内しなくなります**。でも、店を閉めさせたり、店長を替えたりはしません。倉庫が直れば、また案内を始めます。
- 店を開けた直後は、まだ準備中かもしれないので、見回りは**少し待ってから**始めます（`grace_period`）。

| お店の開店と見回り | RustNote と Fly.io |
|---|---|
| 開店の準備の順番 | `main.rs` の起動の順番（第13巻 UNIT 03） |
| 倉庫の鍵が開かず、いったん帰る | DB に届かず、マイグレーションで止まって、終了コード 1 |
| もう一度開けに行かせる | Machine の restart policy（`on-fail`。5 分で最大 10 回） |
| 見回りの「倉庫まで歩けますか」 | `[[http_service.checks]]` の `GET /health`（DB に `SELECT 1`） |
| 「使えません」と答える | `503 {"status":"unavailable"}` |
| お客さんを案内しなくなる | 失敗しているマシンには、リクエストを振り分けない |
| 開店の直後は待つ | `grace_period = "10s"` |

> **用語:** ヘルスチェック（health check：動いているアプリに、決まった間をおいてリクエストを送り、正しく答えられるかを確かめること。RustNote では `GET /health`）

<!-- -->

> **用語:** sqld（エス・キュー・エル・ディー：libSQL のサーバー。Turso の DB と同じ形（Hrana という HTTP の決まり）で、ネットごしに SQL を受け付ける。この単元では、Turso の代わりに、手元の Docker で動かす）

### 図で理解する

DB が止まったときと、DB が無いまま起動したときの流れです（例6-2・エラーの結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-12-0601" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="20" y="22" font-size="13" font-weight="700" fill="#0f172a">① 動いている間に DB が止まった</text>
<rect x="20" y="34" width="150" height="50" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="95" y="56" font-size="11" fill="#047857" text-anchor="middle">/health 200</text>
<text x="95" y="74" font-size="10" fill="#047857" text-anchor="middle">振り分けられる</text>
<rect x="210" y="34" width="150" height="50" rx="6" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="285" y="56" font-size="11" fill="#b91c1c" text-anchor="middle">DB が止まる → 503</text>
<text x="285" y="74" font-size="10" fill="#b91c1c" text-anchor="middle">振り分けを止める</text>
<rect x="400" y="34" width="150" height="50" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="475" y="56" font-size="11" fill="#047857" text-anchor="middle">DB が戻る → 200</text>
<text x="475" y="74" font-size="10" fill="#047857" text-anchor="middle">また振り分ける</text>
<line x1="170" y1="59" x2="206" y2="59" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0601)"/>
<line x1="360" y1="59" x2="396" y2="59" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0601)"/>
<text x="570" y="52" font-size="10" fill="#475569">プロセスは動いたまま</text>
<text x="570" y="68" font-size="10" fill="#475569">（動かし直さない）</text>
<line x1="20" y1="104" x2="700" y2="104" stroke="#cbd5e1" stroke-width="1"/>
<text x="20" y="128" font-size="13" font-weight="700" fill="#0f172a">② DB が無いまま起動した</text>
<rect x="20" y="140" width="120" height="44" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="80" y="166" font-size="11" fill="#0f172a" text-anchor="middle">設定・束</text>
<rect x="160" y="140" width="150" height="44" rx="6" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="235" y="160" font-size="11" fill="#b91c1c" text-anchor="middle">マイグレーション</text>
<text x="235" y="176" font-size="10" fill="#b91c1c" text-anchor="middle">失敗 → 終了コード 1</text>
<rect x="330" y="140" width="170" height="44" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="415" y="160" font-size="11" fill="#c2410c" text-anchor="middle">restart policy</text>
<text x="415" y="176" font-size="10" fill="#c2410c" text-anchor="middle">on-fail（5 分で最大 10 回）</text>
<rect x="520" y="140" width="180" height="44" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="610" y="160" font-size="11" fill="#047857" text-anchor="middle">DB が来たら起動</text>
<text x="610" y="176" font-size="10" fill="#047857" text-anchor="middle">（手元では 2 回目のあと）</text>
<line x1="140" y1="162" x2="156" y2="162" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0601)"/>
<line x1="310" y1="162" x2="326" y2="162" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0601)"/>
<line x1="500" y1="162" x2="516" y2="162" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0601)"/>
<path d="M415,184 V208 H235 V188" fill="none" stroke="#c2410c" stroke-width="1.5" stroke-dasharray="4 3" marker-end="url(#ar-12-0601)"/>
<text x="250" y="224" font-size="10" fill="#c2410c">もう一度起動</text>
<rect x="20" y="240" width="680" height="48" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="34" y="260" font-size="12" fill="#1d4ed8">起動から /health が 200 になるまで、手元では約 1.1 秒。grace_period = "10s" は十分に長い</text>
<text x="34" y="280" font-size="12" fill="#1d4ed8">（Turso までの距離・マイグレーションの数が増えると長くなるので、測ってから決める）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図12-?: ヘルスチェックと起動の順番。動いている間の DB の故障は「振り分けを止める」、起動の時の故障は「プロセスが終わって動かし直す」で扱われる</div>
</div>

### まず動かす最小コード

#### 例6-1: sqld につないで起動し、/health が 200 になるまでの時間を測る

Turso の代わりに、手元の Docker で sqld（`ghcr.io/tursodatabase/libsql-server`）を動かし、RustNote（UNIT 02 のイメージ）を、同じ Docker のネットワークの中から `http://sqld:8080` でつなぎました。`http://` はトークンが要らない、手元の sqld 用の形です（第13巻 UNIT 03）。

```powershell
docker network create rn-net
docker run -d --name sqld --network rn-net ghcr.io/tursodatabase/libsql-server:latest
docker run -d --name rn-app --network rn-net -p 18080:8080 -e DATABASE_URL=http://sqld:8080 rustnote:u02
```

```text
sqld のイメージ: arm64・latest
```

```text
起動してから /health が 200 になるまで: 約 1.1 秒
/health: 200 {"status":"ok"}
--- RustNote のログ
マイグレーションを適用しました: 0001_users
マイグレーションを適用しました: 0002_notes
マイグレーションを適用しました: 0003_sessions
RustNote を起動しました: http://0.0.0.0:8080
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: /health がすること

- `src/routes/health.rs` の `health` は、`app_state(cx).db.ping()` で、DB に `SELECT 1` を送ります。答えが返れば `200 {"status":"ok"}`、返らなければ、ログに `[error] ヘルスチェックで DB に接続できません: …` と書いて、`503 {"status":"unavailable"}` を返します（第13巻 UNIT 07）。
- 「プロセスが動いているか」だけでなく、「DB まで届くか」まで確かめるので、DB が止まったときも、ヘルスチェックで分かります。
- 起動してから 200 になるまでは、約 1.1 秒でした（0.1 秒おきに `/health` を開いて測った）。マイグレーションの3つも、この中に入っています。

### もう一歩: 実務に近い形

#### 例6-2: 動いている間に DB を止め、また動かす

`docker stop sqld` で DB を止めて、`/health` を1秒おきに3回と、トップページを開きました。そのあと `docker start sqld` で動かし直しました。

```text
--- sqld を止めたあと
/health: 503 {"status":"unavailable"}
/health: 503 {"status":"unavailable"}
/health: 503 {"status":"unavailable"}
--- そのときの / （トップページ）: 200
```

```text
--- sqld を動かし直したあと
/health: 200 {"status":"ok"}
--- RustNote のログ（最後の5行）
[error] ヘルスチェックで DB に接続できません: Hrana: `http error: `error trying to connect: dns error: failed to lookup address information: Name or service not known``
[error] ヘルスチェックで DB に接続できません: Hrana: `http error: `error trying to connect: dns error: failed to lookup address information: Name or service not known``
[error] ヘルスチェックで DB に接続できません: Hrana: `http error: `error trying to connect: dns error: failed to lookup address information: Name or service not known``
[error] ヘルスチェックで DB に接続できません: Hrana: `http error: `error trying to connect: dns error: failed to lookup address information: Name or service not known``
GET / -> 200 OK (4.069134ms)
```

- DB を止めると、`/health` は 503 になりました。ログには、DB の名前（`sqld`）が引けない（`dns error`）という原因が書かれました。止めたコンテナは、ネットワークから消えるからです。
- そのときでも、トップページは 200 でした。ログインしていない人のトップページは DB を使わず、画面の枠の `current_user` も、DB の失敗を「ログインしていない」と同じに扱うからです（第13巻 UNIT 07）。**ページが開けても、DB が使えるとは限りません**。ヘルスチェックの URL を、DB まで確かめる `/health` にしておく理由です。
- DB を動かし直すと、RustNote を動かし直さなくても、`/health` は 200 に戻りました。RustNote は、リクエストのたびに接続を作るからです（第13巻 UNIT 02）。

##### 解説2: Fly.io のヘルスチェックが失敗すると

公式のドキュメントで確かめた、Fly.io の動きです。

- `[[http_service.checks]]` に失敗したマシンは、入口（プロキシ）から「元気でない」とされ、**チェックが通るようになるまで、リクエストを振り分けられません**。
- ただし、チェックの失敗で、マシンが**動かし直されたり止められたりはしません**。プロセスが動いたまま DB だけが使えない、という状態は、例6-2のように、DB が戻れば自然に直ります。
- マシンが1台だけなら、振り分け先が無くなるので、そのあいだ、利用者には、Fly.io の入口からのエラーが返ります。DB が止まらないこと（Turso の側の守り）と、台数（UNIT 09）の両方が要ります。

### よくあるエラーと読み方

#### エラー: DB が動いていないときに、RustNote を起動した

sqld を止めたまま、RustNote を起動しました。

```text
Error: マイグレーションに失敗しました

Caused by:
    0: schema_migrations を準備できません
    1: Hrana: `http error: `error trying to connect: dns error: failed to lookup address information: Name or service not known``
（終了コード: 1）
```

- **何が起きたか:** 起動の順番（設定 → 束 → DB → マイグレーション）のうち、マイグレーションの最初（`schema_migrations` の表を用意する所）で、DB に届かず止まり、終了コード 1 で終わりました。
- **読み方:** `Caused by:` の `0:` と `1:` は、原因の鎖です。`0` が RustNote の言葉（`schema_migrations を準備できません`）、`1` が libsql の言葉（Hrana の HTTP のエラー・名前が引けない）です。下ほど、元の原因に近づきます。
- **Fly.io では:** プロセスが 0 でない終了コードで終わると、Machine の restart policy（`fly deploy` で作ったマシンの既定は `on-fail`）で、動かし直されます。公式のドキュメントによると、5 分のうちに最大 10 回まで試し、それでもだめなら止まったままになります。

Fly.io の `on-fail` の代わりに、Docker の `--restart on-failure:5` で起動し、4 秒たってから sqld を動かしました。

```powershell
docker run -d --name rn-app --network rn-net -p 18080:8080 --restart on-failure:5 -e DATABASE_URL=http://sqld:8080 rustnote:u02
# 4 秒後に
docker start sqld
```

```text
/health: 200 {"status":"ok"}
動かし直した回数: 2
--- RustNote のログ
Error: マイグレーションに失敗しました

Caused by:
    0: schema_migrations を準備できません
    1: Hrana: `http error: `error trying to connect: dns error: failed to lookup address information: Name or service not known``
Error: マイグレーションに失敗しました

Caused by:
    0: schema_migrations を準備できません
    1: Hrana: `http error: `error trying to connect: dns error: failed to lookup address information: Name or service not known``
RustNote を起動しました: http://0.0.0.0:8080
```

- 2回失敗して動かし直されたあと、3回目の起動で sqld に届き、起動しました。マイグレーションのログが無いのは、例6-1で、もう適用してあったからです。
- **直し方:** DB が先に動いていることを確かめてから、アプリを出します。Turso はいつも動いている前提ですが、トークンの入れ替え（UNIT 05）や、Turso の障害のときには、この形になります。

### RustNote ではここで使う

- **fly.toml:** `grace_period = "10s"`・`interval = "30s"`・`timeout = "5s"`・`path = "/health"` です（UNIT 04）。手元の起動は約 1.1 秒なので、10 秒の待ちは十分です。
- **起動の順番:** 設定・束の失敗は DB に触る前に止まり（第13巻 UNIT 03・本巻 UNIT 01）、DB の失敗はマイグレーションで止まります。どれも終了コード 1 なので、Fly.io の restart policy に任されます。
- **改善点:** `PORT` がおかしいときは、DB を作ってから止まります（第13巻 UNIT 03。直し方は第13巻の総合演習2）。

### 練習問題

**問1:** 例6-2で、DB が止まっている間も、トップページは 200 でした。もし `fly.toml` のヘルスチェックの `path` を `/`（トップページ）にしていたら、どうなるでしょうか。

**問2:** 起動に 15 秒かかるアプリで、`grace_period = "10s"` のままにすると、何が起きるでしょうか。

**問3:** `/health` で、DB に `SELECT 1` を送るのは、良いことばかりでしょうか。困る場面を考えてください。

### 解答と解説

**問1の解答:**

- DB が止まっていても、ヘルスチェックは通り続けます。Fly.io の入口は、そのマシンに振り分け続け、ログインやノートの一覧（DB を使うページ）は、500 になります。DB の故障に、ヘルスチェックでは気づけません。
- ヘルスチェックの URL は、アプリが本当に仕事をできるか（RustNote なら DB まで届くか）を確かめる物にします。

**問2の解答:**

- 起動の途中の 10 秒の時点で、最初のチェックが送られ、まだ待ち受けていないので失敗します。失敗したマシンには振り分けられないので、起動が終わってチェックが通るまで、そのマシンは使われません。deploy の確かめ方によっては、新しい版が「元気でない」とみなされ、公開が止まることもあります。
- `grace_period` は、測った起動の時間に、余裕を足して決めます。RustNote は約 1.1 秒でしたが、Turso までの距離やマイグレーションの数で変わるので、公開したあとのログ（`fly logs`。UNIT 08）でも確かめます。

**問3の解答（例）:**

- 30 秒ごとに、マシンの台数ぶんの問い合わせが DB に届きます。Turso は、読んだ行や問い合わせの数で費用が決まる面があるので、台数が多いと、ヘルスチェックだけで問い合わせが増えます（UNIT 09）。
- DB が一時的に遅いだけで、`timeout`（5 秒）を超えると、全部のマシンが一度に「元気でない」になり、だれにも振り分けられなくなることがあります。DB の故障のときに、アプリ全体を止める方が良いか、DB を使わないページだけでも出す方が良いかは、アプリによって決めます。RustNote は、DB が無いと何もできないので、DB まで確かめています。

### まとめ

- RustNote の `/health` は、DB に `SELECT 1` をして、届けば 200、届かなければ 503 とログ。手元の sqld で試すと、起動から 200 まで約 1.1 秒、DB を止めると 503（トップページは 200 のまま）、DB を戻すと、動かし直さずに 200 に戻った。
- Fly.io は、ヘルスチェックに失敗したマシンには振り分けないが、動かし直しはしない。プロセスが終了コード 1 で終わったときは、restart policy（`on-fail`。5 分で最大 10 回）で動かし直す。
- DB が無いまま起動すると、マイグレーションで止まって終了コード 1。`--restart on-failure` では、DB が来たところで起動できた。`grace_period` は、測った起動の時間に余裕を足して決める。

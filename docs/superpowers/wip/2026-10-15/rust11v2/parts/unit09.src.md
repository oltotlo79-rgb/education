## UNIT 09: 負荷試験・背圧・容量

### この単元のゴール

- 負荷をかける道具 oha で、release の RustNote に何千ものリクエストを送り、1秒あたりの回数・応答時間の分布・状態コードの内訳を読める
- ログイン（Argon2）を同時に何人がすると、1秒あたりの回数とメモリがどう変わるかを測り、「同時の数を増やしても速くならない所」を見つけられる
- Argon2 を同時に計算する数に上限を付け、超えた分を待たせる**背圧**で、メモリの上限を決められる
- 負荷試験の結果を、状態コードの内訳まで確かめずに信じると、まちがえる理由を言える

### たとえ話でつかむ

ラーメン屋の、お昼どきを考えます。

- お客さんが少ないうちは、来た順にすぐ作れます。お客さんが増えると、1時間に作れる杯数が増えていきますが、**コンロの数**（CPU の数）を超えると、それ以上は増えません。
- コンロが足りないのに、注文を全部いっぺんに受けて、全部の鍋を火にかけようとすると、鍋を置く場所（メモリ）が足りなくなり、1杯ずつの時間も遅くなります。
- そこで、**整理券**を配ります。コンロの数だけ注文を受け、ほかのお客さんには、店の外で順番を待ってもらいます（背圧）。待つ時間はかかりますが、店の中はあふれず、1時間の杯数はかえって増えます。
- 「今日は1時間に 1000 杯さばけた」と思ったら、実は全部「売り切れです」と断っていただけ、ということもあります。**何を返したか**まで数えます。

| ラーメン屋 | RustNote |
|---|---|
| お客さんの数 | 同時のリクエストの数（oha の `-c`） |
| 1時間に作れる杯数 | 1秒あたりの回数（Requests/sec） |
| コンロの数 | CPU の数（このパソコンは 12） |
| 鍋を置く場所 | メモリ（Argon2 は1回 19 MiB。UNIT 08） |
| 整理券 | `tokio::sync::Semaphore`（同時の数の上限。例9-3） |
| 「売り切れです」 | 403・429・503 などの、断る返事（エラー1） |

> **用語:** 負荷試験（たくさんのリクエストを、決まった数・決まった同時の数で送り、速さと、どこで限界が来るかを測ること）

<!-- -->

> **用語:** 背圧（はいあつ。back pressure：受け付けられる量を超えた分を、待たせたり断ったりして、中に入れすぎないようにすること）

<!-- -->

> **用語:** 容量（キャパシティ：1台のマシンで、どのくらいの量まで受け付けられるか。CPU・メモリ・DB のどれが先に足りなくなるかで決まる）

<!-- -->

> **用語:** パーセンタイル（応答時間を短い順に並べたとき、何 % の所の値か。`95.00% in 1.8 ms` は、95% のリクエストが 1.8 ms 以内だった、という意味）

### 図で理解する

ログイン（`POST /login`）を、同時の数を変えて送ったときの、1秒あたりの回数と、サーバーのメモリのピークです（例9-2・例9-3の数）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="20" y="24" font-size="12" font-weight="700" fill="#0f172a">1秒あたりのログインの回数</text>
<line x1="60" y1="150" x2="340" y2="150" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="75" y="128" width="40" height="22" fill="#93c5fd"/>
<text x="95" y="122" font-size="10" fill="#0f172a" text-anchor="middle">14.7</text>
<rect x="135" y="94" width="40" height="56" fill="#93c5fd"/>
<text x="155" y="88" font-size="10" fill="#0f172a" text-anchor="middle">37.6</text>
<rect x="195" y="58" width="40" height="92" fill="#93c5fd"/>
<text x="215" y="52" font-size="10" fill="#0f172a" text-anchor="middle">61.4</text>
<rect x="255" y="80" width="40" height="70" fill="#fca5a5"/>
<text x="275" y="74" font-size="10" fill="#b91c1c" text-anchor="middle">46.6</text>
<text x="95" y="166" font-size="10" fill="#475569" text-anchor="middle">同時 1</text>
<text x="155" y="166" font-size="10" fill="#475569" text-anchor="middle">4</text>
<text x="215" y="166" font-size="10" fill="#475569" text-anchor="middle">12</text>
<text x="275" y="166" font-size="10" fill="#475569" text-anchor="middle">48</text>
<text x="60" y="190" font-size="11" fill="#b91c1c">同時 48 では、12 のときより減った</text>
<text x="380" y="24" font-size="12" font-weight="700" fill="#0f172a">メモリのピーク（MiB）</text>
<line x1="400" y1="150" x2="700" y2="150" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="415" y="144" width="40" height="6" fill="#86efac"/>
<text x="435" y="138" font-size="10" fill="#0f172a" text-anchor="middle">42</text>
<rect x="475" y="137" width="40" height="13" fill="#86efac"/>
<text x="495" y="131" font-size="10" fill="#0f172a" text-anchor="middle">90</text>
<rect x="535" y="126" width="40" height="24" fill="#86efac"/>
<text x="555" y="120" font-size="10" fill="#0f172a" text-anchor="middle">160</text>
<rect x="595" y="50" width="40" height="100" fill="#fca5a5"/>
<text x="615" y="44" font-size="10" fill="#b91c1c" text-anchor="middle">654</text>
<text x="435" y="166" font-size="10" fill="#475569" text-anchor="middle">同時 1</text>
<text x="495" y="166" font-size="10" fill="#475569" text-anchor="middle">4</text>
<text x="555" y="166" font-size="10" fill="#475569" text-anchor="middle">12</text>
<text x="615" y="166" font-size="10" fill="#475569" text-anchor="middle">48</text>
<rect x="20" y="210" width="680" height="76" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="36" y="234" font-size="12" font-weight="700" fill="#047857">背圧あり（Argon2 は同時に 12 まで。例9-3）</text>
<text x="36" y="256" font-size="11" fill="#0f172a">同時 12: 1秒に 76.6 回・ピーク 177 MiB　／　同時 48: 1秒に 69.5 回・ピーク 218 MiB</text>
<text x="36" y="276" font-size="11" fill="#047857">同時 48 でも、メモリは 654 → 218 MiB に収まり、回数は 46.6 → 69.5 回に増えた</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: ログインの同時の数と、回数・メモリ。CPU の数（12）を超えて同時に計算させると、回数は減り、メモリだけが増える。背圧で同時の数をそろえると、どちらも良くなる</div>
</div>

背圧の仕組みです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 200" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-11-0901" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="60" width="150" height="80" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="85" y="88" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">ログインが 48 人</text>
<text x="85" y="110" font-size="11" fill="#475569" text-anchor="middle">（oha -c 48）</text>
<rect x="210" y="40" width="190" height="120" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="305" y="66" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">整理券（Semaphore）</text>
<text x="305" y="90" font-size="11" fill="#0f172a" text-anchor="middle">12 枚だけ</text>
<text x="305" y="112" font-size="11" fill="#0f172a" text-anchor="middle">券が無い 36 人は、</text>
<text x="305" y="132" font-size="11" fill="#0f172a" text-anchor="middle">ここで await して待つ</text>
<rect x="450" y="40" width="260" height="120" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="580" y="66" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">Argon2 の計算（spawn_blocking）</text>
<text x="580" y="90" font-size="11" fill="#0f172a" text-anchor="middle">同時に 12 まで ＝ CPU の数</text>
<text x="580" y="112" font-size="11" fill="#0f172a" text-anchor="middle">メモリは 12 × 19 MiB まで</text>
<text x="580" y="132" font-size="11" fill="#0f172a" text-anchor="middle">終わったら券を返す（_slot が drop）</text>
<line x1="162" y1="100" x2="206" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-0901)"/>
<line x1="402" y1="100" x2="446" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-0901)"/>
<text x="360" y="186" font-size="11" fill="#475569" text-anchor="middle">待っている間は、メモリをほとんど使わない（Future が止まっているだけ）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: 背圧。整理券の数で、同時に計算する数とメモリの上限を決める。超えた分は、計算を始める前に待つ</div>
</div>

### まず動かす最小コード

#### 例9-1: oha で、ふつうのページに負荷をかける

oha は、Rust で作られた、負荷をかける道具です。`cargo install` で入れます。

```powershell
cargo install oha --version 1.16.0 --locked
```

```text
@@OUT:u09_machine@@
```

RustNote は、本番と同じ release で組み立てて、`127.0.0.1:3013` で起動します（新しい DB。UNIT 08 のエラー1のとおり、debug で測っても意味がありません）。

```powershell
cargo build --release
topcoat asset bundle --release
$env:PORT = "3013"
.\target\release\rustnote.exe
```

> **▼ このコマンドがやること（先に日本語で）:** 別の画面から、`GET /login` を 5000 回、同時に 10 本ずつ送ります。`--no-tui` は、動いている間のグラフの画面を出さずに、終わってから結果だけを表示する指定です。

```powershell
oha --no-tui -n 5000 -c 10 http://127.0.0.1:3013/login
```

**▼ 実行結果**

```text
@@OUTGREP:u09_get_login:^(Summary|  Success|  Total:|  Slowest|  Fastest|  Average|  Requests/sec|Response time distribution|  [0-9.]+% in|Status code|  \[)@@
```

#### ▼ 結果を1つずつ分解して解説

##### 解説1: Summary（まとめ）

- `Success rate: 100.00%` は、**返事が返ってきた**割合です（エラー1のとおり、200 かどうかではありません）。`Requests/sec` が、1秒あたりの回数です。ログインの画面は、1秒に 12,710 回返せました。
- `Average`（平均）は 0.78 ms ですが、`Slowest`（いちばん遅い）は 10.1 ms でした。平均だけでは、遅い回が見えません。

##### 解説2: パーセンタイル

- `Response time distribution` は、応答時間を短い順に並べたときの、10%・25%・…・99.99% の所の値です。`50.00% in 0.5753 ms` は「半分は 0.58 ms 以内」、`99.00% in 2.5172 ms` は「100 回に 1 回は 2.5 ms より遅い」という意味です。
- UNIT 07 の SLO（「95% が 50 ms 以内」）は、この `95.00%` の行で確かめられます。

##### 解説3: ログインした人のページ

ログインした人の一覧（`GET /notes`、ノート 20 件）でも測りました。セッションのクッキーは `-H "Cookie: __Host-session=…"` で付けました。

```text
@@OUTGREP:u09_get_notes:^(  Success|  Average|  Requests/sec|  50.00%|  95.00%|  99.00%|  \[)@@
```

- 1秒に 914 回で、ログインの画面の約 14 分の 1 でした。一覧は、セッションの確かめ（DB）・ノートの読み出し（DB）・20 件の画面づくりをするからです（問3）。

### もう一歩: 実務に近い形

#### 例9-2: ログインを、同時の数を変えて送る

ログイン（`POST /login`）は、Argon2 の照合があるので重い操作です（UNIT 08）。同時の数を 1・4・12・48 と変えて、それぞれ送りました（12 はこのパソコンの CPU の数です）。回数は、同時の数の 10 倍（少なくとも 60 回）にしました。1回ごとに、サーバーのメモリのピーク（`PeakWorkingSet64`）も読みました。

```powershell
oha --no-tui -n 480 -c 48 -m POST -T application/x-www-form-urlencoded `
  -d "email=alice%40example.com&password=correct+horse+battery" http://127.0.0.1:3013/login
```

**▼ 実行結果（同時 48）**

```text
@@OUTGREP:u09_login_c48:^(  Success|  Total:|  Slowest|  Fastest|  Average|  Requests/sec|  50.00%|  95.00%|  99.00%|  \[)@@
```

4つの同時の数を、表にまとめます（それぞれの出力の数）。

| 同時の数 | 1秒あたり | 平均 | 95% | メモリのピーク |
|---|---|---|---|---|
| 1 | 14.7 回 | 67.9 ms | 106.9 ms | 42 MiB |
| 4 | 37.6 回 | 105.2 ms | 213.6 ms | 90 MiB |
| 12 | 61.4 回 | 189.2 ms | 559.9 ms | 160 MiB |
| 48 | 46.6 回 | 986 ms | 1763 ms | 654 MiB |

```text
@@OUT:u09_peak_plain@@
```

- 同時の数を 1 → 4 → 12 と増やすと、1秒あたりの回数は増えました。CPU の数（12）を超えて 48 にすると、回数は 61.4 → **46.6 に減り**、平均は 1 秒近くになりました。12 個の CPU で 48 個の Argon2 を同時に計算すると、切り替えが増え、1つずつの計算が遅くなるからです。
- メモリのピークは、同時 48 で **654 MiB** になりました。Argon2 は1回に 19 MiB 使うので、48 個を同時に計算すると、それだけで 900 MiB 近く要る計算です（実際は、全部が同じ瞬間に重なるわけではないので、654 MiB でした）。
- RustNote の `spawn_blocking` は、重い計算を別のスレッドに出しますが、**いくつ同時に出すか**には上限がありません（tokio の既定では、こうしたスレッドを 512 本まで作ります）。

#### 例9-3: 背圧で、同時に計算する数をそろえる

Argon2 を同時に計算する数に、CPU の数までという上限を付けます。練習用のコピーの `src/auth.rs` に、整理券（`tokio::sync::Semaphore`）を足しました（教材の `Rust/rustnote` は変えていません）。

```diff
@@FILE:out/u09_diff_auth.txt@@
```

> **▼ このコードがやること（先に日本語で）:** `ARGON2_SLOTS` は、CPU の数（`available_parallelism`。このパソコンでは 12）の整理券です。`hash_password` と `verify_password` は、計算を始める前に `acquire().await` で券を1枚もらい、無ければ、券が返ってくるまで待ちます。`_slot` は、関数の終わりで片付けられる（drop される）ときに、券を返します。

同じ負荷（同時 12 と 48）を、組み立て直したサーバーに送りました。

```text
@@OUTGREP:u09_slots_c48:^(  Success|  Average|  Requests/sec|  50.00%|  95.00%|  99.00%|  \[)@@
@@OUT:u09_peak_slots@@
```

| | 同時 12 | 同時 48 |
|---|---|---|
| 背圧なし | 61.4 回/秒・160 MiB | 46.6 回/秒・654 MiB |
| 背圧あり | 76.6 回/秒・177 MiB | 69.5 回/秒・218 MiB |

- 同時 48 で、メモリのピークは 654 → **218 MiB** に下がり、1秒あたりの回数は 46.6 → **69.5 回**に増えました。95% の応答時間も、1.76 秒 → 1.14 秒に短くなりました。整理券を待つ間は、`await` で止まっているだけなので、メモリも CPU もほとんど使いません。
- 同時 12 でも、回数が少し増えました（61.4 → 76.6）。どれも1回ずつ測った数なので、ばらつきがあります。差が小さいときは、何回か測り直して確かめます。
- 背圧は、**待たせる**か**断る**かを選べます。ここでは待たせました。待つ時間が長すぎるときは、`try_acquire` で券が無ければ、すぐ `503`（今は受け付けられない）を返す作りもあります（問1）。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: 全部が断られていたのに、速いと思った

ログインの負荷を、ほかのサイトのページから送られた形（`Sec-Fetch-Site: cross-site`）で送った場合です。

```powershell
oha --no-tui -n 2000 -c 10 -m POST -T application/x-www-form-urlencoded `
  -H "Sec-Fetch-Site: cross-site" -d "email=…&password=…" http://127.0.0.1:3013/login
```

```text
@@OUTGREP:u09_err_403:^(  Success|  Requests/sec|  Average|Status code|  \[)@@
```

- **何が起きたか:** `Success rate: 100.00%`・1秒に **65,403 回**と、ログインがとても速く見えます。ところが、`Status code distribution` は `[403] 2000 responses` で、**全部が断られて**いました。Topcoat の Origin の検査が、Argon2 を計算する前に 403 を返したからです（第8巻 UNIT 06）。
- サーバーのアクセスログを数えると、`POST /login` の行は、例9-2の 720 回分だけで、この 2000 回は1行もありませんでした（UNIT 07 と第8巻 UNIT 10 のとおり、403 はアクセスログの層より前で返る）。

```text
@@OUT:u09_plain_logcount@@
```

- **直し方:** 負荷試験の結果は、必ず `Status code distribution` を見て、思った状態コード（ここでは 303）だけが返っているかを確かめます。断る返事（403・429・503）は、計算をしないので、とても速く見えます。

#### エラー2（エラーが出ない間違い）: サーバーを起動し忘れた

```powershell
oha --no-tui -n 20 -c 2 http://127.0.0.1:3013/login
```

```text
@@OUTGREP:u09_err_refused:^(  Success|  Total:|  Average|  Requests/sec|Error distribution|  \[|exit=)@@
```

- **何が起きたか:** 20 回とも「対象のコンピューターによって拒否されたため、接続できませんでした（os error 10061）」で、`Success rate: 0.00%` でした。平均などは `NaN`（数ではない）になりました。Windows では、断られた接続のたびに約 2 秒待ってから諦めるので（第9巻 UNIT 10）、同時 2 本で 20 回に、約 20 秒かかりました。
- **気をつけること:** oha そのものは `exit=0`（成功）で終わりました。CI などで負荷試験を自動で回すときは、終わり方だけでなく、`Success rate` と状態コードの内訳を読んで、合否を決めます。

#### エラー3（思い込み）: Secure のクッキーは、http では送られないと思った

例9-1の `GET /notes` では、セッションのクッキーを、登録の返事の `Set-Cookie` から取り出して `-H "Cookie: …"` で付けました。「`__Host-session` は `Secure` なので、http の curl のクッキーの入れ物（jar）では送られないだろう」と考えたからです。確かめると、違いました。

```text
@@OUT:u09_err_jar@@
```

- **何が起きたか:** curl のクッキーの入れ物（`-c jar.txt`・`-b jar.txt`）でも、`127.0.0.1` へは `Secure` のクッキーが送られ、一覧は 200 でした。curl も、Chrome と同じく（第10巻）、`127.0.0.1` への http には、`Secure` のクッキーを送りました。
- **学べること:** 「たぶんこうなる」と思ったら、小さく試して確かめます。ただし、本番の `https://` の名前や、ほかのパソコンの `http://` の名前では、`Secure` のクッキーは送られません。負荷試験の手順も、その場所で確かめてから使います。

### RustNote ではここで使う

- 今の RustNote は、Argon2 を同時に計算する数に上限がありません。このパソコン（12 コア・32 GB）では、同時 48 のログインで 654 MiB でした。小さなマシンでは、同時のログインが増えると、メモリが足りなくなります（問2）。例9-3の整理券を足すと、メモリの上限が「CPU の数 × 19 MiB」で決まります（改善点）。
- ログインと登録には、回数の制限もありません（第8巻 UNIT 04 の改善点）。背圧は「入ってきた物をどう待たせるか」、回数の制限は「同じ人から入りすぎないようにする」もので、両方を組み合わせます。
- ログインのたびに、セッションの行を DB に書きます（`log_in`）。DB に書く仕事は、SQLite では一度に1つずつなので、同時の数を増やしても、ここは速くなりません（問3も参照）。
- 公開するマシンの大きさを決めるとき（第12巻）は、この単元の数（1秒あたりの回数・メモリのピーク）を、そのマシンで測り直します。

### 練習問題

**問1:** 例9-3の背圧は、整理券が無いと**待たせ**ました。代わりに、券が無ければすぐ `503 Service Unavailable` を返す（**断る**）と、利用者と、サーバーにとって、それぞれ何が変わるでしょうか。

**問2:** メモリが 256 MiB のマシンで、RustNote（起動したては約 30 MiB。例9-2の `start`）を動かします。背圧なしで、ログインが同時に 12 人来ると、Argon2 だけで何 MiB 要るでしょうか。CPU が 1 つのマシンで、例9-3の背圧（CPU の数まで）を入れると、どうなるでしょうか。

**問3:** 例9-1で、`GET /login` は 1秒に 12,710 回、`GET /notes` は 914 回でした。なぜ、14 倍も違うのでしょうか。

### 解答と解説

**問1の解答:**

- **待たせる（例9-3）:** 利用者は、少し待てばログインできます。ただし、混みすぎると、ブラウザや、前に置いた入口（プロキシ）の時間切れまで待たされ、結局エラーになります。サーバーは、待っている人の分の接続を持ち続けます（メモリは少ないですが、数には上限があります）。
- **断る（503）:** 利用者は、すぐに「混んでいます」と分かり、少しあとで試し直せます（`Retry-After` ヘッダーで、何秒後かを伝えられます）。サーバーは、仕事を増やさずに済みます。ただし、断った分は、UNIT 07 の SLO では失敗（5xx）に数えます。
- 実際には、「少しだけ待たせ、それでも空かなければ断る」（`tokio::time::timeout` で `acquire` に時間の上限を付ける）がよく使われます。

**問2の解答:**

- 背圧なしで 12 人なら、Argon2 だけで 12 × 19 ＝ **228 MiB** です。起動したての 30 MiB を足すと、258 MiB で、256 MiB を超えます。実際には、全部が同じ瞬間に重なるとは限りませんが（例9-2の同時 12 は 160 MiB）、混んだ時間には、メモリが足りなくなって、プログラムが止められるおそれがあります。
- CPU が 1 つのマシンで背圧（CPU の数まで）を入れると、Argon2 は同時に 1 つだけで、19 MiB です。30 ＋ 19 ＝ 約 49 MiB で収まります。代わりに、ログインは1つずつ順番になるので、1秒にできる回数は、1 つの CPU の速さで決まります（このパソコンの 1 コアなら、例9-2の同時 1 の 14.7 回くらい）。
- 容量は、「メモリが足りなくなる人数」と「待ち時間が長すぎる人数」の、**少ないほう**で決まります。

**問3の解答:**

- `GET /login`（ログインしていない人）は、`login_page` の最初で `current_user` を呼びますが、セッションのクッキーが無いので、DB には問い合わせずに「ログインしていない」と分かります（`token_hash` が `None`）。小さな画面を作って返すだけです。
- `GET /notes` は、①クッキーのトークンのハッシュで、セッションを DB で探す、②ノートを 20 件と、全部の件数を DB で読む（UNIT 02 の `search` は、件数と一覧の2回）、③20 件の画面を作る、をします。DB に3回問い合わせ、画面も大きいので、1回あたりの時間が長くなります。
- どちらも、まずは十分速い数です。遅いページを探すときは、UNIT 08 のように、部分ごとに測ってから直します。

### まとめ

- oha で、release の RustNote に負荷をかけた。ログインの画面は1秒に 12,710 回、ログインした人の一覧は 914 回だった。結果は、平均だけでなくパーセンタイル（95%・99%）と、状態コードの内訳まで読む。全部 403 でも `Success rate: 100.00%` と出て、速く見える。サーバーが無くても oha は `exit=0` で終わる。
- ログイン（Argon2）は、同時の数を CPU の数（12）まで増やすと回数が増え、48 にすると回数が減り、メモリのピークは 654 MiB になった。`spawn_blocking` には、同時の数の上限が無い。
- 整理券（`Semaphore`）で、Argon2 を同時に計算する数を CPU の数までにする背圧を入れると、同時 48 で 218 MiB・1秒に 69.5 回になった。背圧は待たせるか断るかを選び、容量は、メモリと待ち時間の、先に限界が来るほうで決まる。

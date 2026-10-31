## UNIT 07: migration を出す順番と release command

### この単元のゴール

- RustNote が起動のたびにマイグレーションをかける作りのままで、何台ものマシンが同時に起動すると何が起きるかを、手元で確かめられる
- Fly.io の `release_command`（公開の前に、新しいイメージで1回だけ動かすコマンド）のしくみを、公式のドキュメントにそって説明し、RustNote に「マイグレーションだけをして終わる」動かし方を足して、その形を手元で確かめられる
- 「足すだけ」のマイグレーションと「名前を変える・消す」マイグレーションで、古い版のコードが動き続けられるかの違いを、実際に動かして言える
- 新しい列や表を、古い版を壊さずに出す順番（広げてから、縮める）を説明できる

### たとえ話でつかむ

**新しいお店の、棚の入れ替え**を考えます。

- 開店の朝、店員が3人、同時に来て、それぞれ「棚を組み立てる係」の紙を見て、同時に棚を組み立て始めました。2人目は、組み立てようとした棚が、もう1人目の手で置かれていたので、「もうあります」と言って、帰ってしまいました（起動のたびのマイグレーション）。
- そこで、**段取り係**を1人決めて、開店の前に、棚の組み立てだけを済ませてもらいます。店員たちは、そのあとで来て、もう組み立て終わった棚を使うだけです（release command）。
- 棚を**足す**だけなら、まだ古い手順書を持っている店員も、知らない棚を気にせず、いつもどおり働けます。
- ところが、棚の**名前を付け替える**と、古い手順書の「本の棚に置く」が、どの棚か分からなくなり、仕事ができません。
- 名前を替えたいときは、まず新しい名前の棚を**足して**、両方に置くようにし、全員の手順書を新しくしてから、古い棚を**片付けます**（広げてから、縮める）。

| 棚の入れ替え | RustNote と Fly.io |
|---|---|
| 店員それぞれが組み立てる | 起動のたびに `db::migrate`（`main.rs`） |
| 「もうあります」と帰る | `table notes already exists` で、終了コード 1（例7-1） |
| 段取り係 | `fly.toml` の `[deploy] release_command` |
| 棚の組み立てだけを済ませる | `rustnote --migrate-only`（この単元で足した動かし方） |
| 棚を足す | `ALTER TABLE … ADD COLUMN`（古い版も動く。例7-3） |
| 名前の付け替え | `ALTER TABLE … RENAME COLUMN`（古い版が 500。エラー） |
| 広げてから、縮める | expand / contract の順番 |

> **用語:** release command（リリースコマンド：Fly.io で、新しい版のマシンを入れ替える前に、新しいイメージで1回だけ動かすコマンド。マイグレーションを、ここで済ませる）

<!-- -->

> **用語:** expand / contract（エクスパンド・コントラクト：DB の形を変えるとき、まず新しい列や表を足し（広げる）、全部のコードが新しい形を使うようになってから、古い物を消す（縮める）、という2段階の進め方）

### 図で理解する

起動のたびにマイグレーションをかける形と、release command の形の違いです（例7-1・例7-2の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-12-0701" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="20" y="22" font-size="13" font-weight="700" fill="#0f172a">① 起動のたびに migrate（今の RustNote）</text>
<rect x="20" y="34" width="150" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="95" y="58" font-size="11" fill="#047857" text-anchor="middle">1台目: 3つを適用・200</text>
<rect x="20" y="82" width="150" height="40" rx="6" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="95" y="100" font-size="11" fill="#b91c1c" text-anchor="middle">2台目: 0002 で失敗</text>
<text x="95" y="115" font-size="10" fill="#b91c1c" text-anchor="middle">already exists・終了</text>
<rect x="20" y="130" width="150" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="95" y="154" font-size="11" fill="#047857" text-anchor="middle">3台目: 適用済み・200</text>
<rect x="200" y="34" width="120" height="136" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="260" y="98" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">同じ DB</text>
<text x="260" y="118" font-size="10" fill="#475569" text-anchor="middle">（sqld）</text>
<line x1="170" y1="54" x2="196" y2="70" stroke="#475569" stroke-width="1.5" marker-end="url(#ar-12-0701)"/>
<line x1="170" y1="102" x2="196" y2="102" stroke="#475569" stroke-width="1.5" marker-end="url(#ar-12-0701)"/>
<line x1="170" y1="150" x2="196" y2="134" stroke="#475569" stroke-width="1.5" marker-end="url(#ar-12-0701)"/>
<text x="380" y="22" font-size="13" font-weight="700" fill="#0f172a">② release command で先に1回</text>
<rect x="380" y="34" width="320" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="540" y="52" font-size="11" fill="#1d4ed8" text-anchor="middle">rustnote --migrate-only（新しいイメージで1回）</text>
<text x="540" y="67" font-size="10" fill="#1d4ed8" text-anchor="middle">4つを適用・終了コード 0</text>
<rect x="380" y="100" width="100" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="430" y="124" font-size="11" fill="#047857" text-anchor="middle">1台目 200</text>
<rect x="490" y="100" width="100" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="540" y="124" font-size="11" fill="#047857" text-anchor="middle">2台目 200</text>
<rect x="600" y="100" width="100" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="650" y="124" font-size="11" fill="#047857" text-anchor="middle">3台目 200</text>
<line x1="540" y1="74" x2="540" y2="96" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0701)"/>
<text x="380" y="166" font-size="11" fill="#475569">そのあとの起動では、もう適用済みなので、何もしない</text>
<rect x="20" y="194" width="680" height="84" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="1.5"/>
<text x="34" y="216" font-size="12" font-weight="700" fill="#c2410c">古い版と新しい DB（例7-3・エラー）</text>
<text x="34" y="240" font-size="12" fill="#0f172a">足すだけ（ADD COLUMN pinned）→ 古い v1 も、登録・作成・一覧が動く</text>
<text x="34" y="264" font-size="12" fill="#b91c1c">名前を変える（RENAME COLUMN body TO content）→ 古い v1 は、作成も一覧も 500</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図12-?: マイグレーションを出す順番。起動のたびにかけると、同時に起動したマシンがぶつかる。先に1回だけかけ、DB の形は「足すだけ」で変えると、古い版も動き続けられる</div>
</div>

### まず動かす最小コード

#### 例7-1: 空の DB に、3台を同時に起動する

手元の Docker で sqld を動かし（UNIT 06）、Windows の release の RustNote（教材のまま。v1）を、ポートを変えて3つ、同時に起動しました。どれも同じ sqld（`http://127.0.0.1:18081`）につなぎ、起動のときにマイグレーションをかけます。

```text
@@OUT:u07_race@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: なぜ、ぶつかったのか

- RustNote の `db::migrate` は、①適用済みの版を `schema_migrations` から読み、②まだの版を、1つずつトランザクションの中で適用します（第13巻 UNIT 02）。
- 3台が同時に①をすると、どれも「まだ何も適用されていない（か、途中まで）」と読みます。そのあと、②で同じ `CREATE TABLE notes …` を実行しようとし、先に済ませた1台以外は、`table notes already exists` で失敗します。トランザクションは、1つの版を「全部か、無しか」にはしてくれますが、ほかのマシンと「だれがやるか」を決めてはくれません。
- 2台目は終了コード 1 で終わりました。Fly.io なら、restart policy で動かし直され（UNIT 06）、2回目には適用済みなので起動できます。今回はたまたま無事に済みますが、マイグレーションが「途中まで動いて失敗すると困る」種類（データを書き換える物）だと、危険です。
- 3台目は、ほかの2台より少し遅れて①をしたので、もう適用済みと読み、何もせずに起動しました。どれが失敗するかは、毎回の動くタイミングしだいです。

### もう一歩: 実務に近い形

#### 例7-2: マイグレーションだけをして終わる動かし方を足し、release command の形で試す

Fly.io の `release_command` は、公式のドキュメントによると、次のように動きます。

- 新しいイメージから、一時的なマシンを作り、公開のたびに1回だけ動かす（`CMD` の代わりに、このコマンドを動かす）。
- 0 でない終了コードで終わったら、**公開を止める**（古い版のマシンは、そのまま動き続ける）。
- secrets と環境変数は受け取り、ネットにもつながる。ただし、**volume は付かない**（一時的なマシンのファイルは残らない）。時間の上限は、既定で 5 分。

RustNote の DB は Turso（ネットの向こう）なので、`release_command` でマイグレーションをかけられます（`/data` のローカルの DB では、volume が付かないので使えません）。ところが、今の RustNote には「マイグレーションだけをして終わる」動かし方がありません。教材のコードのコピーの `src/main.rs` に、`--migrate-only` を足しました（v2）。

```diff
@@OUT:u07_diff_main@@
```

`fly.toml` には、次のように書きます（この巻では Fly.io に公開しない）。

```toml
[deploy]
  # 新しい版のマシンを入れ替える前に、新しいイメージで1回だけ、マイグレーションをかける
  release_command = "/app/rustnote --migrate-only"
```

手元では、空の sqld に、まず `rustnote --migrate-only` を1回動かし（release command の代わり）、そのあと3台を同時に起動しました。v2 には、足すだけのマイグレーション `0004_pinned`（例7-3）も入れてあります。

```text
@@OUT:u07_bins@@
```

```text
@@OUT:u07_release_command@@
```

- `--migrate-only` は、4つの版を適用して、終了コード 0 で終わりました。待ち受けもせず、アセットの束も読みません。
- そのあとの3台は、どれもマイグレーションのログを出さず（適用済み）、3台とも `/health` が 200 でした。ぶつかる所がありません。
- 起動のときの `db::migrate` は残してあるので、手元の開発（`topcoat dev`）では、今までどおり自動でかかります。本番では、release command が先に済ませているので、何もしません。

#### 例7-3: 足すだけのマイグレーションのあとで、古い版を動かす

v2 のマイグレーション `0004_pinned` は、ノートに列を1つ足します。

```sql
@@FILE:u07src/0004_pinned.sql@@
```

空の sqld で v2 を起動して登録とノートの作成をし、止めてから、同じ DB で古い v1（教材のまま）を動かしました。

```text
@@OUT:u07_compat_add@@
```

- v1 は、`pinned` の列を知りませんが、登録（同じメールなので 409 で、ログインは 303）・ノートの作成・一覧が、全部動きました。v1 の `INSERT` は `pinned` を書かないので、既定値の 0 が入ります。
- しかも、v1 の `db::migrate` は、自分の知らない版（`0004_pinned`）が `schema_migrations` にあっても、気にせず起動します（第13巻 UNIT 02）。だから、v2 を出したあとで v1 に戻しても（UNIT 10）、動きます。

### よくあるエラーと読み方

#### エラー: 名前を変えるマイグレーションのあとで、古い版を動かした

v3 には、本文の列の名前を `body` から `content` に変えるマイグレーションを入れました（ここでは、マイグレーションの影響を見るためだけの版で、コードの直しは省いています）。

```sql
@@FILE:u07src/0004_rename_body.sql@@
```

空の sqld で v3 を起動して `0004_rename_body` を当て、止めてから、同じ DB で古い v1 を動かしました。

```text
@@OUT:u07_compat_rename@@
```

- **何が起きたか:** v1 は、登録はできましたが（`users` の表は変わっていない）、ノートの作成と一覧が 500 になりました。ログには、`table notes has no column named body`（`body` という列が無い）と `no such column: body` が出ています。v1 の SQL は、`body` の列を使うからです。
- **なぜ困るか:** Fly.io の公開は、ふつう、マシンを順番に入れ替えます（rolling）。release command で `0004_rename_body` を当てた瞬間から、まだ入れ替わっていない古いマシンが、全部 500 を返し始めます。前の版に戻しても（UNIT 10）、DB は戻らないので、直りません。
- **直し方（広げてから、縮める）:**
  1. **広げる:** `content` の列を**足す**マイグレーションを出し、新しい版のコードは、`body` と `content` の両方に書く。古い版も動く。
  2. **写す:** 前からあるノートの `body` を、`content` に写す。
  3. **読む所を替える:** 次の版で、読むのを `content` に替える。
  4. **縮める:** すべてのマシンが新しい版になり、もう戻さないと決めてから、最後の版で `body` を消す。

### RustNote ではここで使う

- **今の形:** RustNote は、起動のたびに `db::migrate` をかけます。マシンが1台で、`min_machines_running = 0`（UNIT 04）なら、ぶつかることはまずありません。台数を増やすとき（UNIT 09）は、例7-2のような、マイグレーションだけをする動かし方と release command を用意します。
- **改善点（この単元で見つかったこと）:** 同時に起動すると、マイグレーションがぶつかります（例7-1）。`--migrate-only` の動かし方がありません（教材のコードは変えず、コピーで試しました）。
- **マイグレーションの書き方:** 新しい版を出すときは、「古い版のコードが、新しい DB の形でも動くか」を、例7-3とエラーの例のように確かめてから出します。

### 練習問題

**問1:** 例7-1で、2台目が失敗したのは `0001_users` ではなく `0002_notes` でした。どんな順番で起きたと考えられるでしょうか。

**問2:** release command が失敗して、公開が止まりました。DB と、動いているマシンは、どうなっているでしょうか。

**問3:** マイグレーションの SQL を `CREATE TABLE IF NOT EXISTS notes …` にすれば、例7-1のぶつかりは無くなるでしょうか。

### 解答と解説

**問1の解答（考えられる順番）:**

- 2台目が `schema_migrations` を読んだとき、1台目は、もう `0001_users` を確定させていて、`0002_notes` は、まだ確定していなかったと考えられます。2台目は `0001` を飛ばし、`0002` の `CREATE TABLE notes` を実行しましたが、その直前に、1台目の `0002` が確定していて、ぶつかりました。
- どの版でぶつかるかは、毎回の動くタイミングしだいです。「たまたま」で結果が変わること自体が、問題の印です。

**問2の解答:**

- 公開は止まり、古い版のマシンは、そのまま動き続けます（公式のドキュメント）。新しい版のマシンは作られません。
- DB は、どこまで適用されたかによります。RustNote の `db::migrate` は、1つの版ずつトランザクションの中で適用するので、失敗した版は取り消され、その前の版までは確定しています（第13巻 UNIT 02）。足すだけのマイグレーションなら、古い版は、確定した分の形でも動きます（例7-3）。直してから、もう一度公開します。

**問3の解答:**

- `CREATE TABLE` のぶつかりは無くなりますが、それだけでは足りません。2台が同じ版を適用すると、そのあとの `INSERT INTO schema_migrations` が、主キー（`version`）の重なりで失敗します。`ALTER TABLE … ADD COLUMN` のように、`IF NOT EXISTS` を付けられない SQL もあります。
- 「1回だけ・1台だけが、マイグレーションをかける」ようにするのが、確実な方法です。release command（例7-2）か、DB の中でロック（だれがやるかの札）を取る方法があります。

### まとめ

- RustNote は起動のたびに `db::migrate` をかけるので、空の DB に3台を同時に起動すると、1台が `table notes already exists` で終了コード 1 になった（どれが失敗するかはタイミングしだい）。
- Fly.io の `release_command` は、新しいイメージで公開のたびに1回だけ動き、失敗すれば公開を止める（volume は無いので、Turso のように外の DB 向け）。コピーに `--migrate-only` を足し、先に1回かけてから3台を起動すると、ぶつからなかった。
- 足すだけのマイグレーション（`ADD COLUMN`）なら古い版も動き、名前を変える（`RENAME COLUMN`）と古い版は 500。DB の形は、広げてから縮める順番で変える。

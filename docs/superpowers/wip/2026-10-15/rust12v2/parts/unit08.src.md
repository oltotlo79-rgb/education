## UNIT 08: deploy・ログ・動作の確かめ・独自ドメイン

### この単元のゴール

- `fly deploy` がすること（イメージを決める・マシンを順番に入れ替える・起動の直後を見張る）を、公式のドキュメントにそって説明できる（この巻では実行しない）
- 公開したあとに「本当に動いているか」を、決まった順番で確かめるスモークテストを作り、手元のコンテナで、正しい公開と壊れた公開の両方に試せる
- RustNote のアクセスログを読んで、スモークテストの1つ1つのリクエストを見つけられる
- 独自ドメインを使う手順（`fly certs add`・DNS のレコード・証明書）を説明できる

### たとえ話でつかむ

**新しい自動販売機を、駅に置く**ところを考えます。

- 置いたら、まず**電源が入っているか**（`/health`）、**表の見本が光っているか**（トップページ）を見ます。
- それだけでは足りません。実際に**お金を入れて、ボタンを押し、品物が出てくるか**（登録・ノートの作成・表示）まで、1回やってみます。これがスモークテストです。
- 確かめる順番と、合格の基準（どのボタンで、何が出れば良いか）は、**紙に書いて**おき、だれがやっても同じ確かめになるようにします。
- 販売機の**記録**（アクセスログ）には、いつ・どのボタンが押され・何が出たかが残っています。確かめの1回1回を、記録の中に見つけられます。
- 駅の名前（`アプリ名.fly.dev`）のほかに、**自分のお店の看板**（独自ドメイン）を掲げることもできます。看板には、正しい店だという**証明書**が要ります。

| 自動販売機 | RustNote の公開 |
|---|---|
| 置く | `fly deploy`（この巻では実行しない） |
| 電源と見本 | `GET /health`・`GET /` |
| お金を入れて品物が出るか | 登録 → 一覧 → 作成 → 作ったノートを開く → ログアウト |
| 紙に書いた確かめの順番 | スモークテストのスクリプト（`smoke.ps1`） |
| 販売機の記録 | アクセスログ（`docker logs`・Fly.io では `fly logs`） |
| 自分のお店の看板と証明書 | 独自ドメイン（`fly certs add`・DNS・Let's Encrypt） |

> **用語:** スモークテスト（smoke test：公開した直後に、大事な使い方をひととおり、本物の URL で試すこと。機械に電源を入れて、煙が出ないかを見ることから、こう呼ぶ）

<!-- -->

> **用語:** 独自ドメイン（自分で持っているドメイン（例: `notes.example.com`）で、アプリを開けるようにすること。DNS のレコードで、名前を Fly.io のアプリに向ける）

### 図で理解する

公開してから、確かめ終わるまでの流れです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-12-0801" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="20" width="160" height="90" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="90" y="44" font-size="12" font-weight="700" fill="#1d4ed8" text-anchor="middle">fly deploy</text>
<text x="22" y="66" font-size="10" fill="#1d4ed8">イメージを決める</text>
<text x="22" y="82" font-size="10" fill="#1d4ed8">（Dockerfile から作る）</text>
<text x="22" y="98" font-size="10" fill="#1d4ed8">release command（UNIT 07）</text>
<rect x="200" y="20" width="160" height="90" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="280" y="44" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">rolling で入れ替え</text>
<text x="212" y="66" font-size="10" fill="#c2410c">1台ずつ新しい版に</text>
<text x="212" y="82" font-size="10" fill="#c2410c">起動の直後 10 秒ほど見張る</text>
<text x="212" y="98" font-size="10" fill="#c2410c">（落ち続けたら止める）</text>
<rect x="390" y="20" width="150" height="90" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="465" y="44" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">ヘルスチェック</text>
<text x="402" y="66" font-size="10" fill="#047857">30 秒ごとに /health</text>
<text x="402" y="82" font-size="10" fill="#047857">通ったマシンに</text>
<text x="402" y="98" font-size="10" fill="#047857">振り分け（UNIT 06）</text>
<rect x="570" y="20" width="140" height="90" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="640" y="44" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">スモークテスト</text>
<text x="582" y="66" font-size="10" fill="#0f172a">本物の URL で</text>
<text x="582" y="82" font-size="10" fill="#0f172a">7つを順に</text>
<text x="582" y="98" font-size="10" fill="#0f172a">→ OK か NG か</text>
<line x1="170" y1="65" x2="196" y2="65" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0801)"/>
<line x1="360" y1="65" x2="386" y2="65" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0801)"/>
<line x1="540" y1="65" x2="566" y2="65" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0801)"/>
<rect x="10" y="140" width="700" height="114" rx="8" fill="#f8fafc" stroke="#94a3b8" stroke-width="1.5"/>
<text x="24" y="162" font-size="12" font-weight="700" fill="#0f172a">スモークテストの7つ（例8-1）</text>
<text x="24" y="186" font-size="11" fill="#0f172a">① GET /health 200　② GET / 200　③ POST /signup 303（新しいメール）　④ GET /notes 200</text>
<text x="24" y="208" font-size="11" fill="#0f172a">⑤ POST /notes 303　⑥ GET 作ったノート 200（⑤の Location）　⑦ POST /logout 303</text>
<text x="24" y="234" font-size="11" fill="#b91c1c">/health だけでは分からない故障（ログイン・保存・表示）も、⑥まで通れば、ひととおり動いている</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図12-?: 公開から確かめまで。Fly.io の見張り（起動の直後とヘルスチェック）のあとに、本物の使い方をひととおり試す</div>
</div>

### まず動かす最小コード

#### fly deploy がすること（この巻では実行しない）

公式のドキュメントで確かめた、`fly deploy` の動きです。

- **イメージを決める:** `--image` か `fly.toml` の `[build]` の指定、なければ `Dockerfile` から作ります。RustNote は `[build] dockerfile = "Dockerfile"`（UNIT 02・04）です。
- **release command:** `fly.toml` に `[deploy] release_command` があれば、マシンを入れ替える前に1回動かし、失敗すれば止めます（UNIT 07）。
- **入れ替え方（strategy）:** 既定は `rolling` で、マシンを1台ずつ新しい版にし、1台が成功してから次に進みます。ほかに、`canary`（1台だけ試してから）・`immediate`（全部いっぺんに）・`bluegreen`（新しいマシンを横に並べ、全部のヘルスチェックが通ってから切り替える）があります。
- **起動の直後の見張り（smoke checks）:** 新しいマシンが起動してから 10 秒ほど見張り、0 でない終了コードで落ち続けるなら、公開を止めます。RustNote の起動の失敗（設定・束・DB。UNIT 06）は、ここで止まります。

```powershell
# RustNote のフォルダで（この巻では実行しない。Fly.io のアカウントでのログインが要る）
fly deploy                # Dockerfile からイメージを作り、rolling で入れ替える
fly status                # マシンと、ヘルスチェックの状態を見る
fly logs                  # マシンのログ（RustNote のアクセスログなど）を見る
```

#### 例8-1: スモークテストを作る

公開したあとに動かす、スモークテストの PowerShell のスクリプトです。Fly.io の代わりに、手元のコンテナ（UNIT 02 のイメージ）に試しました。

> **▼ このコードがやること（先に日本語で）:** 渡された URL（`-Base`）に、`curl.exe` で7つのリクエストを順に送り、期待した状態コードかを、1行ずつ OK・NG で表示します。クッキーは一時ファイル（`New-TemporaryFile`）に入れて、登録のあとのリクエストに持ち回ります。登録には、毎回違うメール（日時入り）を使います。ノートを作ったら、303 の行き先（`redirect_url`）を開きます。1つでも NG なら、終了コード 1 で終わります。

```powershell
@@FILE:smoke.ps1@@
```

```powershell
docker run -d --name rn-u08 -p 18080:8080 rustnote:u02
powershell -NoProfile -ExecutionPolicy Bypass -File smoke.ps1 -Base http://127.0.0.1:18080
```

```text
@@OUT:u08_smoke_ok@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: スモークテストの作り方

- **本物の使い方を、ひととおり:** `/health` は「DB まで届くか」しか見ません（UNIT 06）。登録（Argon2・セッション）・作成（入力の検査・DB への書き込み）・表示（エスケープ・CSS の URL）は、実際に使って初めて分かります。
- **毎回違うメール:** 同じメールで2回目を試すと、409（登録済み）になります（第13巻 UNIT 07）。本番の DB に、確かめ用のユーザーとノートが残るので、確かめ用と分かる名前（`smoke-…@example.com`）にしておきます。消し方も、決めておきます。
- **リダイレクトは追いかけない:** `curl.exe` に `-L` を付けないので、303 は 303 のまま確かめられます。行き先は `%{redirect_url}` で受け取り、⑥で開いています。
- **表示は `Write-Host`:** PowerShell の関数の中で `Write-Output` を使うと、その文字が関数の戻り値に混ざってしまいます。結果の行は `Write-Host` で表示し、戻り値は `curl.exe` の結果だけにしています。

##### 解説2: アクセスログで、確かめを見つける

スモークテストのあとの、コンテナのログです（Fly.io では `fly logs` で見られる、同じ標準エラー出力）。

```text
@@OUT:u08_logs@@
```

- スモークテストの7つのうち、6つがログに出ています。`GET /health` だけは、`access_log` が、わざと書かない決まりだからです（第13巻 UNIT 07）。
- 登録（`POST /signup`）は 40 ミリ秒ほどで、ほかより長いのは Argon2 です（第13巻 UNIT 06）。公開したあと、このくらいの時間で返っているかも、確かめの1つです。

### もう一歩: 実務に近い形

#### 例8-2: 壊れた公開に、スモークテストをかける

`HOST=127.0.0.1` で起動して、外から届かない（UNIT 04・第13巻 UNIT 10）コンテナに、同じスモークテストをかけました。

```powershell
docker run -d --name rn-u08 -p 18080:8080 -e HOST=127.0.0.1 rustnote:u02
powershell -NoProfile -ExecutionPolicy Bypass -File smoke.ps1 -Base http://127.0.0.1:18080
```

```text
@@OUT:u08_smoke_broken@@
```

- 6つとも NG（状態 000 は、返事が無かった）で、終了コード 1 でした。⑥は、⑤が失敗して行き先が無いので、動かしていません。
- 終了コードで成功・失敗が分かるので、CI（GitHub Actions）の最後の段に置けば、公開のあとに自動で確かめられます。失敗したら、UNIT 10 の手順で前の版に戻します。

### よくあるエラーと読み方

#### エラー: スモークテストが、CSS の故障を見逃す

UNIT 03 のように、束を作り直し忘れて古い CSS のまま公開しても、スモークテストの7つは全部 OK になります。状態コードしか見ていないからです。トップページの `<link>` の CSS の URL も、開いて確かめておくと安心です。

```text
@@OUT:u08_css@@
```

- **読み方:** この結果は、CSS が**ある**ことまでは確かめますが、**新しい** CSS かどうかは分かりません。CSS を変えた版を出したときは、指紋（`e9b983efabcfc6c3` の所）が変わったかも見ます（UNIT 03）。
- **直し方:** スモークテストに、「`<link>` の CSS を開いて 200 か」を足します。確かめたいことが増えたら、手で確かめる前に、スクリプトに足していきます。

### RustNote ではここで使う

#### 独自ドメインを使う（この巻では実行しない）

公式のドキュメントで確かめた手順です。

```powershell
# 1. アプリに、自分のドメインを足す（必要な DNS のレコードが表示される）
fly certs add notes.example.com

# 2. DNS の会社の画面で、レコードを作る
#    サブドメイン（notes.example.com など）: CNAME で、アプリの .fly.dev の名前に向ける
#    いちばん上（example.com）: A と AAAA のレコードで、アプリの IP アドレスに向ける
#      （IP アドレスが無ければ fly ips allocate で割り当てる）

# 3. 確かめる（DNS が届いたら、Let's Encrypt の証明書が自動で発行される）
fly certs check notes.example.com
```

- 証明書は、DNS が正しく向いたあとに、Fly.io が Let's Encrypt で自動で発行します。発行には回数の制限があるので、DNS の間違いを直してから、確かめ直します。
- RustNote の `__Host-session` クッキーは、ドメインを指定しない（`__Host-` の決まり）ので、`アプリ名.fly.dev` と独自ドメインでは、別々のログインになります。利用者には、どちらか1つの URL を案内します。

#### 公開のたびの確かめの順番

1. `fly deploy` の出力で、すべてのマシンが `success` になったか。
2. `fly status` で、ヘルスチェックが通っているか。
3. スモークテスト（例8-1）を、本物の URL で動かす。
4. `fly logs` で、スモークテストのリクエストと、`[error]` の行が無いかを見る。

### 練習問題

**問1:** スモークテストが、本番の DB に確かめ用のユーザーとノートを残します。困ることと、対策を考えてください。

**問2:** `/health` が 200 なのに、スモークテストの③（登録）だけが NG（500）になりました。何が考えられるでしょうか。ログのどこを見ればよいでしょうか。

**問3:** 例8-1のスクリプトに、`-L`（リダイレクトを追いかける）を付けた `curl.exe` を使うと、何が困るでしょうか。

### 解答と解説

**問1の解答:**

- 公開のたびにユーザーが1人ずつ増え、利用者の数を数えるとき、確かめ用のユーザーが混ざります。メールの形（`smoke-…@example.com`）で見分けられるようにし、決まった時期にまとめて消すか、スモークテストの最後に、自分で作ったノートを消す（`POST /notes/{id}/delete`）手順を足します。
- RustNote には、ユーザーを消す画面が無いので、消すときは DB を直接操作することになります。手順と、だれがするかを決めておきます。

**問2の解答:**

- `/health` は DB に `SELECT 1` をするだけなので、DB に**書く**ことや、Argon2・セッションのことは見ていません。登録の 500 は、たとえば、DB への書き込みができない（Turso のトークンが読み取り専用・容量の上限）、`sessions` の表がない（マイグレーションの抜け。UNIT 07）、などが考えられます。
- ログ（`fly logs`）で、`POST /signup -> 500` の直前の `[error]` の行を見ます。RustNote は、500 のときだけ、原因の鎖を `[error] … <- … <- …` の形で書きます（第13巻 UNIT 07）。

**問3の解答:**

- 303 を追いかけてしまうので、`POST /signup` の結果が、行き先の `GET /notes` の 200 になります。「登録が 303 で成功したか」と「一覧が開けたか」の区別がつかず、たとえば、登録に失敗してログインの画面へ戻されても、200 になって見逃すことがあります。
- 1つのリクエストで、1つのことを確かめるのが、スモークテストの基本です。

### まとめ

- `fly deploy` は、イメージを決め、release command を動かし、既定の `rolling` でマシンを1台ずつ入れ替え、起動の直後を 10 秒ほど見張る（落ち続けたら止める）。結果は `fly status` と `fly logs` で見る。
- スモークテストは、本物の URL で、大事な使い方の7つ（health・トップ・登録・一覧・作成・表示・ログアウト）を順に試し、終了コードで成功・失敗を返す。手元のコンテナでは全部 OK、`HOST=127.0.0.1` の壊れた公開では6つとも NG（000）だった。
- アクセスログで、確かめの1つ1つを見つけられる（`/health` は書かない決まり）。状態コードだけでは古い CSS を見逃すので、確かめは足していく。独自ドメインは `fly certs add` と DNS のレコードで、証明書は自動で発行される。

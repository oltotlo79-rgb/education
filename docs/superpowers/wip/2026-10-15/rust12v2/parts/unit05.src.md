## UNIT 05: secrets・設定・入れ替え

### この単元のゴール

- RustNote の設定のうち、秘密の値（Turso のトークン）と、秘密でない値（`HOST`・`PORT` など）を見分け、それぞれをどこに書くか（`fly secrets`・`fly.toml` の `[env]`・`Dockerfile`）を決められる
- 秘密の値をイメージに焼き込むと、だれが・どこから読めてしまうかを、`docker image inspect` と `docker history` で確かめられる
- `.dockerignore` が、手元の `.env`（秘密の値）を、ビルドに送らないための守りであることを、実際のビルドで示せる
- `fly secrets set`・`--stage`・`fly secrets deploy`・トークンの入れ替えの手順を、公式のドキュメントにそって説明できる（この巻では実行しない）

### たとえ話でつかむ

会社の**金庫の暗証番号**を考えます。

- 暗証番号を、社員みんなが見る**業務マニュアル**（イメージ）に書き込んではいけません。マニュアルは何冊も印刷して配られ（イメージは何度もコピーされ）、後から消すことができません。
- 暗証番号は、**その日に金庫の前に立つ人にだけ**、口頭で伝えます（動かすときに環境変数で渡す）。マニュアルには「暗証番号は、当日に伝えます」とだけ書きます。
- 手元のメモ帳（`.env`）に暗証番号を書いておくのは構いませんが、マニュアルを印刷所に送るとき（`docker build`）、メモ帳まで一緒に送らないように、**送らない物の一覧**（`.dockerignore`）を作っておきます。
- 暗証番号は、**ときどき変えます**。この金庫は、番号を変えると、それまでの番号が全部使えなくなるしくみなので、変える間だけ、少し金庫が開かない時間ができます。

| 金庫の暗証番号 | RustNote |
|---|---|
| 暗証番号 | `DATABASE_AUTH_TOKEN`（Turso のトークン） |
| 業務マニュアル | Docker のイメージ（`Dockerfile` の `ENV` も含む） |
| 当日に口頭で伝える | `fly secrets set`（Fly.io が、マシンの起動のときに環境変数として渡す） |
| 「当日に伝えます」とだけ書く | `fly.toml` の `[env]` には、秘密でない値だけ |
| 手元のメモ帳 | `.env`（開発用。Git にも入れない） |
| 送らない物の一覧 | `.dockerignore` |
| ときどき変える | トークンの入れ替え（rotation） |

> **用語:** secret（シークレット：パスワードやトークンのような、秘密の設定の値。Fly.io では `fly secrets set` で、暗号にして保管され、マシンが動くときに環境変数として渡される）

<!-- -->

> **用語:** 入れ替え（ローテーション：秘密の値を、決まった時期や、漏れたおそれがあるときに、新しい値に替えること）

### 図で理解する

RustNote の設定の値が、どこに書かれ、どうやってアプリに届くかです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-12-0501" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="200" height="66" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="120" y="44" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">Dockerfile の ENV</text>
<text x="120" y="66" font-size="11" fill="#475569" text-anchor="middle">HOST・PORT・DATABASE_URL（既定）</text>
<rect x="20" y="102" width="200" height="66" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="120" y="126" font-size="12" font-weight="700" fill="#1d4ed8" text-anchor="middle">fly.toml の [env]</text>
<text x="120" y="148" font-size="11" fill="#1d4ed8" text-anchor="middle">HOST・PORT（秘密でない）</text>
<rect x="20" y="184" width="200" height="66" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="120" y="204" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">fly secrets set</text>
<text x="120" y="222" font-size="11" fill="#c2410c" text-anchor="middle">DATABASE_URL</text>
<text x="120" y="240" font-size="11" fill="#c2410c" text-anchor="middle">DATABASE_AUTH_TOKEN</text>
<rect x="300" y="90" width="170" height="90" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="385" y="116" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">マシンの起動</text>
<text x="385" y="138" font-size="11" fill="#047857" text-anchor="middle">環境変数として渡す</text>
<text x="385" y="158" font-size="10" fill="#047857" text-anchor="middle">（secrets ＞ [env] ＞ ENV）</text>
<rect x="530" y="90" width="170" height="90" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="615" y="116" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">RustNote</text>
<text x="615" y="138" font-size="11" fill="#0f172a" text-anchor="middle">Config::from_env</text>
<text x="615" y="158" font-size="11" fill="#0f172a" text-anchor="middle">トークンは Secret に包む</text>
<line x1="220" y1="53" x2="296" y2="110" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0501)"/>
<line x1="220" y1="135" x2="296" y2="135" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0501)"/>
<line x1="220" y1="217" x2="296" y2="160" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0501)"/>
<line x1="470" y1="135" x2="526" y2="135" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0501)"/>
<text x="20" y="272" font-size="11" fill="#b91c1c">イメージ（ENV）と fly.toml は、たくさんの人が読める物。秘密の値は、起動のときにだけ渡す secrets に置く</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図12-?: 設定の置き場所。同じ名前があれば、secrets が [env] に、[env] が Dockerfile の ENV に勝つ</div>
</div>

### まず動かす最小コード

#### 例5-1: Turso の URL なのに、トークンが無い

Turso の URL（`libsql://…`）だけを渡して、トークンを渡さずに、UNIT 02 のイメージを起動しました。設定の段で止まるので、ネットにはつながりません。

```powershell
docker run --rm -e DATABASE_URL=libsql://rustnote-example.turso.io rustnote:u02
```

```text
@@OUT:u05_missing_token@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 秘密の値と、そうでない値

| 値 | 秘密か | 置き場所 |
|---|---|---|
| `HOST`・`PORT` | 秘密でない | `Dockerfile` の `ENV`・`fly.toml` の `[env]` |
| `DATABASE_URL` | 秘密に近い（どの DB かが分かる） | `fly secrets set`（`Dockerfile` には手元用の既定値だけ） |
| `DATABASE_AUTH_TOKEN` | **秘密** | `fly secrets set` だけ |

- RustNote の `Config::from_env` は、Turso の URL（`libsql://`・`https://`・`wss://`）なのにトークンが無いと、DB に触る前に止まります（第13巻 UNIT 03）。設定を忘れたことに、起動の時点で気づけます。
- トークンは、読んだらすぐ `Secret` 型に包むので、`{:?}` でログに出しても `Secret(********)` としか表示されません（第13巻 UNIT 03）。

### もう一歩: 実務に近い形

#### 例5-2: トークンをイメージに焼き込むと（まちがった例）

UNIT 02 のイメージに、`ENV` でトークン（ここでは、本物ではない、ためしの文字列）を足しただけのイメージを作り、だれでもできる2つのコマンドで、中を見ました。

```dockerfile
# まちがった例: 秘密の値を ENV でイメージに書き込む
FROM rustnote:u02
ENV DATABASE_AUTH_TOKEN=u05-fake-token-not-a-real-secret
```

```powershell
docker image inspect rustnote:baked --format "{{range .Config.Env}}{{println .}}{{end}}"
docker history rustnote:baked --format "{{.CreatedBy}}" --no-trunc
```

```text
@@OUT:u05_baked@@
```

- イメージの設定にも、作った手順の記録にも、トークンがそのまま出ました。イメージを手に入れた人（Fly.io のレジストリに送ったイメージ・CI の記録・同じ PC の人）は、だれでも読めます。あとから `ENV` の行を消して作り直しても、前のイメージを持っている人の手元からは消えません。

#### 例5-3: 動かすときに渡すと

同じトークンを、イメージには書かず、`docker run -e` で渡しました。Fly.io の `fly secrets set` も、マシンの起動のときに、環境変数として渡すしくみです。

```powershell
docker run -d --name rn-u05 -e DATABASE_AUTH_TOKEN=u05-fake-token-not-a-real-secret rustnote:u02
```

```text
@@OUT:u05_runtime@@
```

- イメージ（`rustnote:u02`）の環境変数には、トークンがありません。動いているコンテナの設定（`docker inspect rn-u05`）にはあるので、そのコンテナを見られる人（管理する人）には読めますが、イメージを配っても漏れません。
- `DATABASE_URL` がローカルのファイルなので、RustNote はトークンを使わずに起動しました。

#### 例5-4: 手元の .env が、ビルドにまぎれ込むか

`.env` に、Turso の URL とトークン（ためしの文字列）を書いたフォルダから、`.dockerignore` がある場合と無い場合で、ビルドしました。builder の段だけのイメージ（`--target builder`）と、できたイメージの両方を調べました。

```text
@@OUT:u05_dotenv@@
```

- `.dockerignore` が無いと、builder の段の `/app/.env` に、トークンの行ごと入りました。`COPY . .` が、フォルダの中身を全部コピーするからです。
- できたイメージには、どちらも `.env` はありません。実行用の段には、実行ファイルと `assets/` しか運ばないからです（UNIT 02）。multi-stage が、ここでも守りになっています。
- それでも、`.env` は、ビルドのときに Docker へ送られています。Fly.io の `fly deploy` は、ふつう、フォルダの中身を Fly.io のビルド用のマシンへ送ってビルドするので、`.dockerignore` が無いと、手元の秘密の値まで送ってしまいます。RustNote の `.dockerignore` は、`.env` を送らない一覧に入れています（第13巻 UNIT 10）。

### よくあるエラーと読み方

#### エラー1: 秘密の値を設定し忘れた

例5-1のとおり、Turso の URL だけでトークンが無いと、`DATABASE_AUTH_TOKEN が設定されていません` で止まります。Fly.io では、マシンが起動のたびに止まり、ヘルスチェックも通りません（UNIT 06）。`fly secrets list` で、名前が登録されているかを確かめます（値は表示されません）。

#### エラー2: secrets を設定したのに、効かない

- `fly secrets set --stage` で登録したときは、まだどのマシンにも渡っていません。`fly secrets deploy` か `fly deploy` で、マシンを入れ替えたときに渡ります（公式のドキュメント）。
- 同じ名前を `[env]` にも書いていると、secrets が勝ちます。`[env]` を見て「ここが効いているはず」と思い込まないようにします。

### RustNote ではここで使う

公式のドキュメントで確かめた、RustNote の公開のときの secrets の手順です（この巻では実行しない。Fly.io のアカウントでのログインが要る）。

```powershell
# Turso の DB の URL とトークンを、Fly.io の金庫に入れる（入れると、マシンが入れ替わる）
fly secrets set DATABASE_URL=libsql://自分のDB名-組織名.turso.io DATABASE_AUTH_TOKEN=トークン

# 登録されている名前を見る（値は表示されない）
fly secrets list

# いらなくなったら消す
fly secrets unset DATABASE_AUTH_TOKEN
```

- `fly secrets set` は、値を Fly.io の金庫に暗号にして保管し、そのアプリのマシンを全部入れ替えます（入れ替えのとき、マシンの一時的なディスクは消えます）。値は、マシンの起動のときに環境変数として渡され、イメージには入りません。
- 入れ替えずに登録だけしたいとき（ほかの変更と一緒に出したいとき）は `--stage` を付け、あとで `fly secrets deploy`（イメージを作り直さずに出し直す）か `fly deploy` をします。
- PowerShell の画面に打ったトークンは、コマンドの履歴に残ります。共有の PC では、履歴を消すか、ファイルから読む方法を使います。

**トークンの入れ替え（ローテーション）の順番:**

Turso の `turso db tokens invalidate DB名` は、トークンを1つずつではなく、その DB（と同じグループの DB）のトークンを**全部**使えなくします。トークンに署名する鍵を作り直すからです。公式のドキュメントは、短い停止が要ると書いています。だから、「新しいトークンを先に作って渡してから、古いトークンを止める」という順番は、Turso では使えません（先に作った新しいトークンも、一緒に使えなくなります）。

1. 次の3つのコマンドを、先に用意しておく（停止を短くするため）。
2. `turso db tokens invalidate DB名` で、今のトークンを全部使えなくする（ここから、RustNote は DB に入れず、`/health` は 503 になる。UNIT 06）。
3. `turso db tokens create DB名` で、新しいトークンを作る（`--expiration 7d` のように、期限を付けることもできる）。
4. `fly secrets set DATABASE_AUTH_TOKEN=新しいトークン` で渡す（マシンが新しいトークンで起動し直す）。
5. UNIT 08 のスモークテストで、動いていることを確かめる。

- 漏れたおそれがあるときは、すぐに 2 から始めます。決まった時期に入れ替えるときは、利用者の少ない時間に行います。

### 練習問題

**問1:** 例5-2のイメージを、Fly.io のレジストリに送ってしまいました。`ENV` の行を消して作り直し、もう一度送れば、大丈夫でしょうか。

**問2:** RustNote の `Dockerfile` には `ENV DATABASE_URL=/data/rustnote.db` があります。これは、秘密の値をイメージに書いていることになるでしょうか。

**問3:** 例5-4で、`.dockerignore` が無くても、できたイメージに `.env` は入りませんでした。それなら、`.dockerignore` は要らないでしょうか。

### 解答と解説

**問1の解答:**

- 大丈夫ではありません。前に送ったイメージ（と、それを取り込んだ人の手元）には、トークンが残っています。イメージは、作り直しても、前の物が上書きで消えるとは限りません。
- 秘密の値がイメージに入ったと分かったら、**その値は漏れたもの**として扱い、入れ替え（ローテーション）をします。新しいトークンを secrets で渡し、古いトークンを使えなくします。

**問2の解答:**

- なりません。`/data/rustnote.db` は、ローカルのファイルの場所（既定値）で、だれに知られても困りません。本番では、`fly secrets set DATABASE_URL=libsql://…` で上書きします（secrets が `ENV` に勝つ）。
- ただし、secrets を設定し忘れると、この既定値のまま、マシンの中のファイルで動いてしまいます。マシンを入れ替えるたびにデータが消えるので、気づきにくい事故になります（第13巻 UNIT 10 の問2）。

**問3の解答:**

- 要ります。例5-4のとおり、`.dockerignore` が無いと、`.env` は builder の段に入り、ビルドのたびに Docker（Fly.io のビルド用のマシンを含む）へ送られます。builder の段の結果（キャッシュ）は、ビルドしたマシンに残ります。
- さらに、`target/`（数 GB になることもある）や DB のファイルも送らずに済むので、ビルドも速くなります。「できたイメージに入らなければよい」ではなく、「秘密の値は、ビルドに送らない」が守り方です。

### まとめ

- 秘密の値（Turso のトークン）は `fly secrets set` だけに置き、`Dockerfile` の `ENV` と `fly.toml` の `[env]` には、秘密でない値だけを書く。同じ名前なら secrets ＞ `[env]` ＞ `ENV`。
- `ENV` で焼き込んだトークンは、`docker image inspect` と `docker history` で、だれでも読めた。`docker run -e`（Fly.io では secrets）で渡せば、イメージには残らない。漏れたら、作り直しではなく、入れ替える。
- `.dockerignore` が無いと、`.env` は builder の段に入り、ビルドに送られる（できたイメージには入らない）。Turso のトークンの入れ替えは、全部を使えなくする → 新しいトークンを作る → secrets で渡す → 確かめる、の順（短い停止がある）。

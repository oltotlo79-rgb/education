## UNIT 04: fly launch・fly.toml・ポート

### この単元のゴール

- `fly.toml` の各項目（`app`・`primary_region`・`[build]`・`[env]`・`[http_service]`・ヘルスチェック・`[[vm]]`）が、RustNote のどの設定と結びついているかを説明できる
- ブラウザから RustNote に届くまでの、アドレスとポートの道筋（443 → Fly.io の入口 → `internal_port` 8080 → `0.0.0.0:8080`）を言える
- `fly launch` が何をするかを、公式のドキュメントにそって説明できる（この巻では実行しない）
- `fly.toml` と `Dockerfile` を読んで食い違いを見つける小さな道具で、手元のコンテナと合わせて確かめられる

### たとえ話でつかむ

大きな**マンションの宅配の受付**を考えます。

- 荷物（リクエスト）は、まずマンションの**正面の受付**（Fly.io の入口）に届きます。受付は、外からの荷物を、どの部屋に回すかを知っています。
- 部屋の中の住人（RustNote）は、**部屋番号 8080 の扉**の前で待っています。受付の帳簿（`fly.toml` の `internal_port`）に書いた番号と、住人が待つ扉の番号（`PORT`）が違うと、荷物は届きません。
- 住人が「この部屋の中の人からだけ受け取る」（`127.0.0.1`）と決めていたら、受付からの荷物も受け取れません。「どこからでも受け取る」（`0.0.0.0`）にします。
- 受付は、鍵の付いていない裏口（`http://`）に来た人を、**鍵付きの正面**（`https://`）へ回します（`force_https`）。
- 受付の人は、30 秒ごとに部屋をノックして「元気ですか」と聞きます（ヘルスチェック）。返事が無ければ、その部屋には荷物を回しません。
- 帳簿は、だれでも見られる**掲示板**に貼り出す物です。合鍵の番号（トークン）を書いてはいけません。

| マンションの受付 | Fly.io と RustNote |
|---|---|
| 正面の受付 | Fly.io の入口（プロキシ。`https://アプリ名.fly.dev` の 443 番） |
| 受付の帳簿の部屋番号 | `fly.toml` の `[http_service] internal_port = 8080` |
| 住人が待つ扉の番号 | RustNote が読む `PORT`（`[env]` と `Dockerfile` の `PORT=8080`） |
| どこからでも受け取る | `HOST=0.0.0.0` |
| 裏口から正面へ回す | `force_https = true` |
| 30 秒ごとのノック | `[[http_service.checks]]` の `GET /health` |
| 掲示板に貼る帳簿 | `fly.toml`（Git に入れる。秘密は書かない） |

> **用語:** fly.toml（フライ・トムル：Fly.io でアプリを動かすときの設定ファイル。TOML という書き方で、アプリの名前・地域・ポート・ヘルスチェック・マシンの大きさなどを書く）

<!-- -->

> **用語:** リージョン（region：Fly.io の、マシンを置く場所（都市）。`nrt` は東京。3文字の名前で表す）

### 図で理解する

ブラウザから RustNote まで、リクエストが届く道筋です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-12-0401" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="30" width="130" height="70" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="75" y="58" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">ブラウザ</text>
<text x="75" y="80" font-size="10" fill="#475569" text-anchor="middle">https://…fly.dev</text>
<rect x="190" y="30" width="170" height="70" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="275" y="54" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">Fly.io の入口</text>
<text x="275" y="74" font-size="10" fill="#c2410c" text-anchor="middle">443 で受けて HTTPS をほどく</text>
<text x="275" y="90" font-size="10" fill="#c2410c" text-anchor="middle">http:// は https:// へ</text>
<rect x="410" y="20" width="300" height="160" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="560" y="42" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">マシン（primary_region = "nrt"）</text>
<rect x="430" y="56" width="260" height="56" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="1.5"/>
<text x="560" y="78" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">RustNote（Docker のイメージ）</text>
<text x="560" y="98" font-size="11" fill="#0f172a" text-anchor="middle">HOST=0.0.0.0・PORT=8080 で待ち受け</text>
<text x="430" y="138" font-size="11" fill="#047857">internal_port = 8080 へ届ける</text>
<text x="430" y="160" font-size="11" fill="#047857">[[vm]] shared・1 CPU・512mb</text>
<line x1="140" y1="65" x2="186" y2="65" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0401)"/>
<line x1="360" y1="65" x2="426" y2="80" stroke="#475569" stroke-width="2" marker-end="url(#ar-12-0401)"/>
<text x="393" y="58" font-size="10" fill="#475569" text-anchor="middle">8080</text>
<line x1="275" y1="100" x2="275" y2="196" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 3"/>
<line x1="275" y1="196" x2="670" y2="196" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 3"/>
<line x1="670" y1="196" x2="670" y2="116" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 3" marker-end="url(#ar-12-0401)"/>
<text x="300" y="214" font-size="11" fill="#475569">ヘルスチェック: 起動から 10 秒待ってから、30 秒ごとに GET /health（5 秒で時間切れ）</text>
<text x="10" y="248" font-size="11" fill="#b91c1c">PORT と internal_port が違う・HOST が 127.0.0.1 → 入口から届かず、ヘルスチェックも失敗（エラー）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図12-?: リクエストの道筋。HTTPS は Fly.io の入口でほどかれ、マシンの中の RustNote には internal_port の 8080 で届く</div>
</div>

### まず動かす最小コード

#### 例4-1: fly.toml を読む

> **▼ このファイルがやること（先に日本語で）:** アプリの名前と置く場所（東京）、ビルドに使う Dockerfile、秘密でない環境変数（`HOST`・`PORT`）、外からのリクエストの受け方（8080 番へ・HTTPS へ回す・使われていないときは止める）、ヘルスチェック、マシンの大きさを決めます。DB の URL とトークンは、ここには書かず `fly secrets set` で渡します（UNIT 05）。

```toml
# Fly.io の設定ファイル
# app の名前は世界で1つだけなので、自分用の名前に変えてから `fly launch --no-deploy` / `fly deploy` する。
# DB（Turso）の URL とトークンは秘密の値なので、ここには書かず `fly secrets set` で渡す:
#   fly secrets set DATABASE_URL=libsql://<DB名>-<組織名>.turso.io DATABASE_AUTH_TOKEN=<トークン>

app = "rustnote-example"
primary_region = "nrt"

[build]
  dockerfile = "Dockerfile"

# アプリに渡す環境変数（秘密でないもの）
[env]
  HOST = "0.0.0.0"
  PORT = "8080"

[http_service]
  # アプリが待ち受けるポート（PORT と同じ）
  internal_port = 8080
  # http:// で来たら https:// へ転送する（セッションのクッキーは Secure なので HTTPS が必須）
  force_https = true
  # アクセスが無いときはマシンを止め、来たら起動する
  auto_stop_machines = "stop"
  auto_start_machines = true
  min_machines_running = 0

  # ヘルスチェック: GET /health が 200 を返せば正常
  [[http_service.checks]]
    grace_period = "10s"
    interval = "30s"
    method = "GET"
    path = "/health"
    timeout = "5s"

[[vm]]
  memory = "512mb"
  cpu_kind = "shared"
  cpus = 1
```

TOML として正しく読めるかを、Python の標準の `tomllib` で読んで、JSON の形で表示しました（Fly.io にはつながりません）。

```powershell
python -c "import tomllib, json, sys; print(json.dumps(tomllib.load(open(sys.argv[1], 'rb')), ensure_ascii=False, indent=2))" fly.toml
```

```text
{
  "app": "rustnote-example",
  "primary_region": "nrt",
  "build": {
    "dockerfile": "Dockerfile"
  },
  "env": {
    "HOST": "0.0.0.0",
    "PORT": "8080"
  },
  "http_service": {
    "internal_port": 8080,
    "force_https": true,
    "auto_stop_machines": "stop",
    "auto_start_machines": true,
    "min_machines_running": 0,
    "checks": [
      {
        "grace_period": "10s",
        "interval": "30s",
        "method": "GET",
        "path": "/health",
        "timeout": "5s"
      }
    ]
  },
  "vm": [
    {
      "memory": "512mb",
      "cpu_kind": "shared",
      "cpus": 1
    }
  ]
}
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: アプリの名前・場所・ビルド

- `app` は、Fly.io の中で**世界に1つだけ**の名前です。`rustnote-example` は教材の例なので、自分で公開するときは、自分用の名前に変えます。公開すると、`https://アプリ名.fly.dev` で開けるようになります。
- `primary_region = "nrt"` は、マシンを東京に置くという意味です。`fly deploy` は、ここにマシンを作ります（公式のドキュメント）。DB（Turso）も近くに置くと、速くなります（UNIT 09）。
- `[build] dockerfile = "Dockerfile"` で、UNIT 02 の Dockerfile からイメージを作ります。

##### 解説2: [env] と [http_service]

- `[env]` には、秘密でない環境変数だけを書きます。値は文字列で書きます（`PORT = "8080"`）。同じ名前の secret（`fly secrets set`）があると、そちらが勝ちます（公式のドキュメント）。
- `internal_port = 8080` は、Fly.io の入口が、マシンの中の**どのポートへ**リクエストを届けるかです。RustNote は `PORT` で待ち受けるので、`[env]` の `PORT`・`Dockerfile` の `ENV PORT=8080`・`internal_port` の3つを、同じ 8080 にそろえます。
- `force_https = true` は、`http://` で来たリクエストを `https://` へ移します。RustNote のログインのクッキー `__Host-session` は `Secure`（HTTPS でしか送らない）なので、HTTPS でないと、ログインしても次のリクエストでクッキーが送られません（第8巻）。
- `auto_stop_machines = "stop"` と `auto_start_machines = true` は、使われていない間はマシンを止め、リクエストが来たら動かす設定です。値は `"off"`・`"stop"`・`"suspend"` のどれかで、公式のドキュメントは、止める・動かすの両方を有効にするか、両方を無効にするかを勧めています。`min_machines_running = 0` なので、だれも使っていなければ、0 台になります（問2）。

##### 解説3: ヘルスチェックとマシンの大きさ

- `[[http_service.checks]]` は、起動してから `grace_period`（10 秒）待ってから、`interval`（30 秒）ごとに `GET /health` を送り、`timeout`（5 秒）以内に 2xx が返れば正常とみなします。リダイレクト（301・302）は追いかけません（公式のドキュメント）。RustNote の `/health` は、DB に `SELECT 1` をして 200 か 503 を返します（UNIT 06）。
- `[[vm]]` は、マシンの大きさです。`cpu_kind = "shared"`（ほかと分け合う CPU）を 1 つ、メモリ 512MB です。Argon2 は1回に 19 MiB 使うので、同時のログインが多いと足りなくなることがあります（第11巻 UNIT 09）。

#### fly launch がすること（この巻では実行しない）

公式のドキュメントによると、`fly launch` は、次の順に進みます。

1. ソースを調べて、どんなアプリかを見分けます。`Dockerfile` があるときは、ふつう、それをそのまま使います。
2. 組織・アプリの名前（フォルダの名前から作る。世界で1つだけ）・リージョン・マシンの大きさを提案し、変えるかを聞きます。
3. `fly.toml` を作ります。すでにあるときは、その設定を新しいアプリに写すかを聞きます。
4. そのまま公開（deploy）まで進みます。`--no-deploy` を付けると、公開の前で止まります。

```powershell
# RustNote のフォルダで（この巻では実行しない。Fly.io のアカウントでのログインが要る）
fly launch --no-deploy --name 自分用のアプリ名
```

- RustNote には、`Dockerfile` と `fly.toml` がもう用意してあるので、`fly launch` は、アプリを Fly.io に登録して、`fly.toml` の `app` を新しい名前にするところまでで済みます。DB の URL とトークンを `fly secrets set` で渡してから（UNIT 05）、`fly deploy` で公開します（UNIT 08）。

### もう一歩: 実務に近い形

#### 例4-2: fly.toml と Dockerfile の食い違いを、手元で確かめる道具

Fly.io に出す前に、`fly.toml` と `Dockerfile` がそろっているかを確かめる、小さな Python の道具を作りました。ヘルスチェックの `path` は、手元で動かしているコンテナにも送ってみます。

> **▼ このコードがやること（先に日本語で）:** `fly.toml` を `tomllib` で、`Dockerfile` を文字列として読み、`internal_port`・`[env]` の `PORT`・`Dockerfile` の `PORT` がそろっているか、`HOST` が `0.0.0.0` か、`force_https` が有効か、`[env]` に秘密の値を書いていないかを確かめます。URL を渡すと、ヘルスチェックの `path` を、実際に開いてみます。1つでもだめなら、終了コード 1 で終わります。

```python
"""fly.toml と Dockerfile が、RustNote の待ち受けの設定とそろっているかを確かめる（Fly.io にはつながない）。
使い方: python check_fly.py [fly.toml のあるフォルダ] [確かめる URL（例: http://127.0.0.1:18080）]
"""
import re
import sys
import tomllib
import urllib.request
from pathlib import Path

folder = Path(sys.argv[1] if len(sys.argv) > 1 else ".")
base = sys.argv[2] if len(sys.argv) > 2 else None
fly = tomllib.loads((folder / "fly.toml").read_text(encoding="utf-8"))
docker = (folder / "Dockerfile").read_text(encoding="utf-8")
problems = 0


def check(ok, message):
    global problems
    print(("OK  " if ok else "NG  ") + message)
    if not ok:
        problems += 1


service = fly.get("http_service", {})
env = fly.get("env", {})
port = service.get("internal_port")
docker_port = re.search(r"PORT=(\d+)", docker)
docker_host = re.search(r"HOST=([0-9.]+)", docker)

check(port is not None, f"[http_service] internal_port = {port}")
check(env.get("PORT") == str(port), f"[env] PORT = {env.get('PORT')!r} と internal_port がそろっている")
check(docker_port is not None and docker_port.group(1) == str(port),
      f"Dockerfile の PORT = {docker_port.group(1) if docker_port else None} と internal_port がそろっている")
check(env.get("HOST") == "0.0.0.0", f"[env] HOST = {env.get('HOST')!r}（外から届くには 0.0.0.0）")
check(docker_host is not None and docker_host.group(1) == "0.0.0.0",
      f"Dockerfile の HOST = {docker_host.group(1) if docker_host else None}")
check(service.get("force_https") is True, "force_https = true（__Host-session クッキーは Secure なので HTTPS が要る）")
secrets_in_env = [k for k in env if "TOKEN" in k or "SECRET" in k or k == "DATABASE_URL"]
check(not secrets_in_env, f"[env] に秘密の値を書いていない（見つかったもの: {secrets_in_env}）")
checks = service.get("checks", [])
for c in checks:
    path = c.get("path")
    if base and path:
        try:
            with urllib.request.urlopen(base + path, timeout=5) as r:
                code = r.status
        except Exception as error:  # 届かない・4xx・5xx
            code = getattr(error, "code", type(error).__name__)
        check(code == 200, f"ヘルスチェック {c.get('method', 'GET')} {path} → {code}（{base}）")
    else:
        check(bool(path), f"ヘルスチェックの path = {path}")
print(f"だめだった数: {problems}")
sys.exit(1 if problems else 0)
```

UNIT 02 のイメージを `-p 18080:8080`（外の 18080 を、コンテナの 8080 へ）で動かして、教材の `fly.toml` と `Dockerfile` を確かめました。

```powershell
docker run -d --name rn-u04 -p 18080:8080 rustnote:u02
python check_fly.py . http://127.0.0.1:18080
```

```text
Python 3.12.0
```

```text
OK  [http_service] internal_port = 8080
OK  [env] PORT = '8080' と internal_port がそろっている
OK  Dockerfile の PORT = 8080 と internal_port がそろっている
OK  [env] HOST = '0.0.0.0'（外から届くには 0.0.0.0）
OK  Dockerfile の HOST = 0.0.0.0
OK  force_https = true（__Host-session クッキーは Secure なので HTTPS が要る）
OK  [env] に秘密の値を書いていない（見つかったもの: []）
OK  ヘルスチェック GET /health → 200（http://127.0.0.1:18080）
だめだった数: 0
（終了コード: 0）
```

- 8つとも OK でした。手元のコンテナは、Fly.io の入口の代わりに、Docker が 18080 を 8080 へつないでいます。

### よくあるエラーと読み方

#### エラー: fly.toml に、よくある書き間違いを3つ入れた

`[env]` の `PORT` を 3000 に、`force_https` を `false` に変え、`[env]` にトークンを書いた `fly.toml` を作りました。

```diff
--- a/fly.toml
+++ b/fly.toml
@@ -12,13 +12,14 @@
 # アプリに渡す環境変数（秘密でないもの）
 [env]
   HOST = "0.0.0.0"
-  PORT = "8080"
+  DATABASE_AUTH_TOKEN = "u04-fake-token"
+  PORT = "3000"
 
 [http_service]
   # アプリが待ち受けるポート（PORT と同じ）
   internal_port = 8080
   # http:// で来たら https:// へ転送する（セッションのクッキーは Secure なので HTTPS が必須）
-  force_https = true
+  force_https = false
   # アクセスが無いときはマシンを止め、来たら起動する
   auto_stop_machines = "stop"
   auto_start_machines = true
```

Fly.io と同じように、`[env]` の `PORT=3000` をコンテナに渡して動かし（Docker は、外の 18080 を `internal_port` の 8080 へつなぐ）、道具で確かめました。

```text
OK  [http_service] internal_port = 8080
NG  [env] PORT = '3000' と internal_port がそろっている
OK  Dockerfile の PORT = 8080 と internal_port がそろっている
OK  [env] HOST = '0.0.0.0'（外から届くには 0.0.0.0）
OK  Dockerfile の HOST = 0.0.0.0
NG  force_https = true（__Host-session クッキーは Secure なので HTTPS が要る）
NG  [env] に秘密の値を書いていない（見つかったもの: ['DATABASE_AUTH_TOKEN']）
NG  ヘルスチェック GET /health → RemoteDisconnected（http://127.0.0.1:18080）
だめだった数: 4
（終了コード: 1）
```

```text
RustNote を起動しました: http://0.0.0.0:3000
```

- **ポートの食い違い:** RustNote は `PORT` を読んで、コンテナの中の **3000** で待ち受けました（ログ）。ところが、入口は `internal_port` の **8080** へ届けるので、そこにはだれもいません。ヘルスチェックの `GET /health` は、つながってすぐ切られました（`RemoteDisconnected`）。Fly.io でも、ヘルスチェックが失敗し続け、そのマシンにはリクエストが回されません。
- **force_https = false:** `http://` のまま使えてしまいます。RustNote のログインのクッキーは `Secure` なので、ブラウザは `http://` ではクッキーを送らず、ログインしても、ログインしていない扱いになります。
- **[env] のトークン:** `fly.toml` は Git に入れて、だれでも読める物です。トークンは `fly secrets set` で渡します（UNIT 05）。
- **直し方:** 道具が NG と言った所を、例4-2の OK の形に戻します。CI で、この道具を動かすこともできます。

### RustNote ではここで使う

- **fly.toml と Dockerfile:** `internal_port = 8080`・`[env] PORT = "8080"`・`ENV PORT=8080` と、`HOST=0.0.0.0` の組です（第13巻 UNIT 10）。手元の開発では、`.env.example` のとおり `127.0.0.1:3000` です。
- **ヘルスチェック:** `src/routes/health.rs` の `GET /health`（UNIT 06）。
- **このあとの単元:** 秘密の値の渡し方（UNIT 05）、ヘルスチェックと起動の順番（UNIT 06）、公開と確かめ（UNIT 08）、地域と費用（UNIT 09）。

### 練習問題

**問1:** `primary_region` を、東京（`nrt`）から、アメリカのリージョンに変えると、日本の利用者にとって、何が変わるでしょうか。

**問2:** `auto_stop_machines = "stop"`・`min_machines_running = 0` のとき、しばらくだれも使わなかったあとの、最初のリクエストは、どうなるでしょうか。

**問3:** 例4-2の道具に、もう1つ確かめを足すとしたら、何を確かめますか。

### 解答と解説

**問1の解答:**

- 日本からアメリカまでの往復の時間が、毎回のリクエストに加わります。さらに、RustNote は1回のページの表示で、DB に何回も問い合わせるので、DB（Turso）をどこに置くかも大事です。マシンと DB の距離が、ページの表示の時間に何倍にもなって効くことを、UNIT 09 で測ります。
- 利用者が日本にいるなら、マシンも DB も、日本の近くに置きます。

**問2の解答:**

- マシンが止まっているので、Fly.io の入口がマシンを動かしてから、リクエストを届けます。そのぶん、最初のリクエストだけ遅くなります（RustNote が起動して、DB を開き、マイグレーションを確かめる時間。UNIT 06 で測ります）。
- 1台も止めたくなければ `min_machines_running = 1` にしますが、そのぶん、動かしている時間の費用がかかります（UNIT 09）。

**問3の解答（例）:**

- `[[http_service.checks]]` の `path` が、RustNote にある URL か（打ち間違いで `/healthz` にすると、ずっと失敗する）。例4-2の道具は、実際に開いて確かめているので、これはもう確かめています。
- `grace_period` が、RustNote の起動にかかる時間より長いか（短いと、起動の途中で「元気でない」とされる）。起動の時間は、UNIT 06 で測ります。
- `[[vm]] memory` が、同時に来るログインの Argon2 の分（1回 19 MiB）より十分に大きいか（第11巻 UNIT 09）。

### まとめ

- `fly.toml` は、アプリの名前（世界で1つ）・場所（`nrt`）・Dockerfile・`[env]`（秘密でない値だけ）・`[http_service]`（`internal_port`・`force_https`・止める／動かす）・ヘルスチェック（10 秒待って 30 秒ごとに `GET /health`）・マシンの大きさ（shared 1 CPU・512MB）を決める。
- 届く道筋は、ブラウザ → 443 の入口（HTTPS をほどく）→ `internal_port` 8080 → `HOST=0.0.0.0`・`PORT=8080` の RustNote。`PORT` と `internal_port` がずれると、待ち受けている所に届かず、ヘルスチェックも失敗する。
- `fly launch` は、ソースを調べ、名前と場所を決め、`fly.toml` を作って（あれば写して）、公開まで進む（`--no-deploy` で止める）。出す前に、`fly.toml` と `Dockerfile` の食い違いを、手元の道具とコンテナで確かめておく。

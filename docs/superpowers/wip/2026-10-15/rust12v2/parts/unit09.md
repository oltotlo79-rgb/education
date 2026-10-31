## UNIT 09: 台数・地域・費用・Turso との距離

### この単元のゴール

- RustNote と DB（Turso）の間の距離が、1回のページの表示に何回分も効くことを、遅れを入れた実験で示せる
- RustNote が DB に送ったリクエストを数えて、1回のページの表示で何回往復しているかを確かめられる
- マシンと DB の置き場所（リージョン）を、利用者の場所から決められる。台数（`min_machines_running`、止める／動かす）を、速さと費用の釣り合いで考えられる
- Fly.io と Turso の費用の決まり方を、公式の料金のページ（2026-10-31 に確かめたもの）で読める

### たとえ話でつかむ

**図書館のカウンターと、離れた書庫**を考えます。

- 利用者が「この本の一覧を見せて」と頼むと、カウンターの人は書庫まで何度か往復します。会員証を確かめに1回、本が何冊あるかを数えに1回、20 冊分を取りに1回……。
- 書庫がカウンターの**すぐ裏**なら、往復は一瞬です。書庫が**隣の町**にあると、1回の往復が長く、それが何回も重なります。
- だから書庫はカウンターの近くに置き、カウンターは利用者の近くに置きます。
- カウンターの人を何人置くか（台数）も決めます。だれも来ない夜はカウンターを閉めて（マシンを止めて）お金を節約できますが、朝いちばんの人は、開けるまで少し待ちます。
- 書庫が遠すぎると、見回りの係（ヘルスチェック）が「返事が無い」と判断してしまいます。

| 図書館 | RustNote と Fly.io・Turso |
|---|---|
| カウンター | RustNote のマシン（Fly.io の `primary_region`） |
| 書庫 | DB（Turso のリージョン） |
| 書庫まで往復する回数 | 1回のリクエストで、DB に送るリクエストの数（例9-2） |
| 隣の町の書庫 | 遠いリージョンの DB（例9-1の遅れ） |
| カウンターを閉める夜 | `auto_stop_machines = "stop"`・`min_machines_running = 0` |
| 「返事が無い」と判断する | ヘルスチェックの `timeout = "5s"`（エラー） |

> **用語:** レイテンシ（latency：1回の往復にかかる時間。遠いほど長くなる。光の速さには上限があるので、距離による遅れは、速い回線にしても無くならない）

<!-- -->

> **用語:** toxiproxy（トキシプロキシ：2つのプログラムの間にはさんで、通信をわざと遅くしたり切ったりできる道具。この単元では、RustNote と sqld の間に置いて、DB までの距離の代わりにする）

### 図で理解する

DB までの片道の遅れを増やしたときの、応答の時間です（例9-1の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="20" y="22" font-size="13" font-weight="700" fill="#0f172a">GET /notes（ログイン中・25 件）の応答の時間（7回の中央値）</text>
<text x="20" y="58" font-size="12" fill="#0f172a">ファイル</text>
<rect x="110" y="44" width="5" height="22" rx="3" fill="#a7f3d0"/>
<text x="122" y="60" font-size="11" fill="#0f172a">5 ms</text>
<text x="20" y="92" font-size="12" fill="#0f172a">遅れ 0 ms</text>
<rect x="110" y="78" width="16" height="22" rx="3" fill="#a7f3d0"/>
<text x="132" y="94" font-size="11" fill="#0f172a">19 ms</text>
<text x="20" y="126" font-size="12" fill="#0f172a">遅れ 10 ms</text>
<rect x="110" y="112" width="87" height="22" rx="3" fill="#fde68a"/>
<text x="203" y="128" font-size="11" fill="#0f172a">102 ms</text>
<text x="20" y="160" font-size="12" fill="#0f172a">遅れ 50 ms</text>
<rect x="110" y="146" width="312" height="22" rx="3" fill="#fed7aa"/>
<text x="428" y="162" font-size="11" fill="#0f172a">367 ms</text>
<text x="20" y="194" font-size="12" fill="#0f172a">遅れ 100 ms</text>
<rect x="110" y="180" width="566" height="22" rx="3" fill="#fecaca"/>
<text x="600" y="196" font-size="11" fill="#0f172a">666 ms</text>
<rect x="20" y="220" width="680" height="68" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="34" y="242" font-size="12" fill="#1d4ed8">片道が 100 ms 増えると、GET /notes は約 650 ms 増えた → 1回の表示で、DB と約 6.5 回往復している</text>
<text x="34" y="264" font-size="12" fill="#1d4ed8">数えると、待っている往復は 6 回（読む SQL が 3つ × 2回ずつ。例9-2）。遅れは、往復の回数だけ重なる</text>
<text x="34" y="282" font-size="10" fill="#475569">（「ファイル」は DB がコンテナの中のファイル /data/rustnote.db のとき。「遅れ 0 ms」は sqld に Docker のネットワークでつないだとき）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図12-?: DB までの距離と応答の時間。遅れは、1回のリクエストの中で DB と往復する回数だけ、何倍にもなって効く</div>
</div>

1回の `GET /notes` で、RustNote が DB に送ったリクエストです（例9-2の結果）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 340" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-12-09a" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#1d4ed8"/></marker>
<marker id="ar-12-09b" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#94a3b8"/></marker>
</defs>
<rect x="20" y="16" width="130" height="270" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="85" y="40" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">RustNote</text>
<rect x="570" y="16" width="130" height="270" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="635" y="40" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">DB（sqld）</text>
<text x="360" y="62" font-size="11" fill="#0f172a" text-anchor="middle">① describe（ログイン中のユーザーを探す SQL の形を聞く）</text>
<line x1="155" y1="68" x2="565" y2="68" stroke="#1d4ed8" stroke-width="1.5" marker-end="url(#ar-12-09a)"/>
<text x="360" y="90" font-size="11" fill="#0f172a" text-anchor="middle">② cursor（その SQL を実行して、行を受け取る）</text>
<line x1="155" y1="96" x2="565" y2="96" stroke="#1d4ed8" stroke-width="1.5" marker-end="url(#ar-12-09a)"/>
<text x="360" y="118" font-size="11" fill="#64748b" text-anchor="middle">③ close（この接続はおしまい。返事を待たない）</text>
<line x1="155" y1="124" x2="565" y2="124" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="5 4" marker-end="url(#ar-12-09b)"/>
<text x="360" y="146" font-size="11" fill="#0f172a" text-anchor="middle">④ describe（ノートの数を数える SQL）</text>
<line x1="155" y1="152" x2="565" y2="152" stroke="#1d4ed8" stroke-width="1.5" marker-end="url(#ar-12-09a)"/>
<text x="360" y="174" font-size="11" fill="#0f172a" text-anchor="middle">⑤ cursor（数を受け取る）</text>
<line x1="155" y1="180" x2="565" y2="180" stroke="#1d4ed8" stroke-width="1.5" marker-end="url(#ar-12-09a)"/>
<text x="360" y="202" font-size="11" fill="#0f172a" text-anchor="middle">⑥ describe（1ページ分を読む SQL）</text>
<line x1="155" y1="208" x2="565" y2="208" stroke="#1d4ed8" stroke-width="1.5" marker-end="url(#ar-12-09a)"/>
<text x="360" y="230" font-size="11" fill="#0f172a" text-anchor="middle">⑦ cursor（20 件を受け取る）</text>
<line x1="155" y1="236" x2="565" y2="236" stroke="#1d4ed8" stroke-width="1.5" marker-end="url(#ar-12-09a)"/>
<text x="360" y="258" font-size="11" fill="#64748b" text-anchor="middle">⑧ close（返事を待たない）</text>
<line x1="155" y1="264" x2="565" y2="264" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="5 4" marker-end="url(#ar-12-09b)"/>
<rect x="20" y="298" width="680" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="360" y="320" font-size="12" fill="#1d4ed8" text-anchor="middle">青い矢印 6 本は、返事が来るまで次へ進めない → DB が遠いと、6 回分の往復を待つ</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図12-?: GET /notes 1回で、RustNote が DB に送った8つのリクエスト。読む SQL 1つにつき、describe と cursor の2回の往復になる</div>
</div>

### まず動かす最小コード

#### 例9-1: RustNote と DB の間に遅れを入れて、応答の時間を測る

UNIT 06 と同じく、手元の Docker で sqld を動かし、RustNote（UNIT 02 のイメージ）との間に toxiproxy をはさみました。toxiproxy の `latency` で、sqld から RustNote へ返る方向（片道）に、0・10・50・100 ms の遅れを入れました。それぞれ7回ずつ測った中央値です。ノートは 25 件作ってあります。

```powershell
docker network create rn-net
docker run -d --name sqld --network rn-net ghcr.io/tursodatabase/libsql-server:latest
docker run -d --name toxiproxy --network rn-net -p 18474:8474 ghcr.io/shopify/toxiproxy:latest
# toxiproxy に「28080 で受けて sqld:8080 へつなぐ」道を作る
curl.exe -X POST http://127.0.0.1:18474/proxies -d '{\"name\":\"sqld\",\"listen\":\"0.0.0.0:28080\",\"upstream\":\"sqld:8080\"}'
# RustNote は、toxiproxy をはさんで sqld につなぐ
docker run -d --name rn-app --network rn-net -p 18080:8080 -e DATABASE_URL=http://toxiproxy:28080 rustnote:u02
# 片道 50 ms の遅れを入れる（ほかの値も同じように）
curl.exe -X POST http://127.0.0.1:18474/proxies/sqld/toxics -d '{\"name\":\"lag\",\"type\":\"latency\",\"stream\":\"downstream\",\"attributes\":{\"latency\":50,\"jitter\":0}}'
```

```text
toxiproxy のイメージ: arm64
```

```text
遅れ（片道）| GET /health | GET /notes（ログイン中・25 件）| POST /login
0 ms | 9 ms | 19 ms | 43 ms
10 ms | 36 ms | 102 ms | 100 ms
50 ms | 131 ms | 367 ms | 288 ms
100 ms | 231 ms | 666 ms | 511 ms
```

比べるために、DB をコンテナの中のファイル（UNIT 02 の既定の `/data/rustnote.db`）にしたときも測りました。

```text
ローカルのファイル（/data/rustnote.db）| 3 ms | 5 ms | 33 ms
```

#### 例9-2: RustNote が DB に送ったリクエストを数える

例9-1の遅れから、往復の回数は見積もれます。本当にその回数なのかを、数えて確かめます。RustNote と sqld の間に、届いたリクエストを1行ずつ書き出すだけの「数える係」（Python の小さなプログラム）をはさみました。SQL の値（メールアドレスやトークン）は書き出さず、SQL の文の先頭だけを書きます。

`count_proxy.py`:

```python
# RustNote と sqld の間に置いて、RustNote が sqld に送った HTTP のリクエストを1行ずつ書き出す。
# 使い方: python count_proxy.py 待ち受けのポート 送り先のURL ログのファイル
import http.client
import json
import sys
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import urlsplit

listen_port = int(sys.argv[1])
upstream = urlsplit(sys.argv[2])
log_path = sys.argv[3]
lock = threading.Lock()


def short(sqls) -> str:
    text = " ".join(" ".join((s or "").split()) for s in sqls)
    return text[:32] + "…" if len(text) > 32 else text


def describe(body: bytes) -> str:
    """Hrana の中身を「describe『SQL』, close」のように短く書く（値は書かない）"""
    try:
        data = json.loads(body)
    except ValueError:
        return f"{len(body)} バイト"
    if "batch" in data and "requests" not in data:
        # /v3/cursor: 1つの batch を送り、結果を順に受け取る
        steps = data["batch"].get("steps", [])
        return f"cursor「{short(step.get('stmt', {}).get('sql') for step in steps)}」"
    parts = []
    for req in data.get("requests", []):
        kind = req.get("type", "?")
        sqls = []
        if kind == "execute":
            sqls = [req.get("stmt", {}).get("sql")]
        elif kind == "batch":
            sqls = [step.get("stmt", {}).get("sql") for step in req.get("batch", {}).get("steps", [])]
        elif kind in ("sequence", "describe"):
            sqls = [req.get("sql")]
        text = short(sqls)
        parts.append(f"{kind}「{text}」" if text else kind)
    return ", ".join(parts) if parts else "（requests なし）"


class Handler(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def log_message(self, *args):
        pass

    def read_body(self) -> bytes:
        if self.headers.get("Transfer-Encoding", "").lower() == "chunked":
            chunks = []
            while True:
                size = int(self.rfile.readline().strip().split(b";")[0], 16)
                if size == 0:
                    self.rfile.readline()
                    break
                chunks.append(self.rfile.read(size))
                self.rfile.readline()
            return b"".join(chunks)
        return self.rfile.read(int(self.headers.get("Content-Length", 0)))

    def forward(self):
        body = self.read_body()
        conn = http.client.HTTPConnection(upstream.hostname, upstream.port, timeout=30)
        headers = {k: v for k, v in self.headers.items() if k.lower() not in ("host", "transfer-encoding", "content-length", "connection")}
        conn.request(self.command, self.path, body=body if body else None, headers=headers)
        resp = conn.getresponse()
        data = resp.read()
        with lock, open(log_path, "a", encoding="utf-8") as log:
            line = f"{self.command} {self.path} → {resp.status}"
            if body:
                line += f": {describe(body)}"
            log.write(line + "\n")
        self.send_response(resp.status)
        for k, v in resp.getheaders():
            if k.lower() not in ("transfer-encoding", "content-length", "connection"):
                self.send_header(k, v)
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)
        conn.close()

    do_GET = forward
    do_POST = forward


ThreadingHTTPServer(("127.0.0.1", listen_port), Handler).serve_forever()
```

```powershell
# 1つ目の PowerShell: sqld を動かし、その手前で「数える係」を待ち受けさせる
docker run -d --name sqld -p 18081:8080 ghcr.io/tursodatabase/libsql-server:latest
python count_proxy.py 28181 http://127.0.0.1:18081 proxy.log
```

```powershell
# 2つ目の PowerShell: release の RustNote を「数える係」につなぐ
$env:PORT = "18090"
$env:DATABASE_URL = "http://127.0.0.1:28181"
.\target\release\rustnote.exe
```

登録してノートを 25 件作ったあと、3つのリクエストを1つずつ送り、その間に `proxy.log` に増えた行を見ました。

```text
--- GET /health（状態 200）: sqld へのリクエスト 3 回
 1. POST /v3/pipeline → 200: describe「SELECT 1」
 2. POST /v3/cursor → 200: cursor「SELECT 1;」
 3. POST /v3/pipeline → 200: close
--- GET /notes（ログイン中・25 件）（状態 200）: sqld へのリクエスト 8 回
 1. POST /v3/pipeline → 200: describe「SELECT u.id, u.email FROM sessio…」
 2. POST /v3/cursor → 200: cursor「SELECT u.id, u.email FROM sessio…」
 3. POST /v3/pipeline → 200: close
 4. POST /v3/pipeline → 200: describe「SELECT COUNT(*) FROM notes WHERE…」
 5. POST /v3/cursor → 200: cursor「SELECT COUNT (*) FROM notes WHER…」
 6. POST /v3/pipeline → 200: describe「SELECT id, user_id, title, body,…」
 7. POST /v3/cursor → 200: cursor「SELECT id, user_id, title, body,…」
 8. POST /v3/pipeline → 200: close
--- POST /login（状態 303）: sqld へのリクエスト 5 回
 1. POST /v3/pipeline → 200: describe「SELECT id, email, password_hash …」
 2. POST /v3/cursor → 200: cursor「SELECT id, email, password_hash …」
 3. POST /v3/pipeline → 200: close
 4. POST /v3/pipeline → 200: batch「INSERT INTO sessions (token_hash…」, get_autocommit, close
 5. POST /v3/pipeline → 200: batch「DELETE FROM sessions WHERE expir…」, get_autocommit, close
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 遅れが、何倍にもなる

- 片道の遅れを 100 ms 増やすと、`GET /notes` は 19 ms → 666 ms と、約 650 ms 増えました。1回の往復ごとに 100 ms 増えるので、`GET /notes` は DB と**約 6.5 回**往復している、と見積もれます。
- 同じように、`POST /login` は 43 ms → 511 ms で約 4.7 回、`/health` は 9 ms → 231 ms で約 2.2 回でした。
- 遅れが小さいとき（10 ms）は、ほかの時間が混ざって、割り算の答えがずれます。いちばん大きい 100 ms の行で見積もるのが確かです。
- ファイルの DB（5 ms）と、遅れ 0 ms の sqld（19 ms）の差も、HTTP で DB と話す分の時間です。

##### 解説2: 数えると、読む SQL 1つで2回の往復

- RustNote が使う libSQL の crate は、リモートの DB（`http://`・`libsql://`）とは、HTTP の `/v3/pipeline` と `/v3/cursor` に JSON を送って話します。この決まりを Hrana と呼びます。
- 読む SQL（`conn.query`）を1つ送るたびに、`describe`（SQL の形を聞く）と `cursor`（実行して行を受け取る）の **2回** のリクエストを送っていました。書く SQL（`conn.execute`）は、`batch` の **1回** でした。
- `GET /notes` は、読む SQL が3つ（ログイン中のユーザーを探す・ノートの数を数える・1ページ分を読む）なので、3 × 2 ＝ 6 回です。これに、接続を閉じる `close` が2回付いて、全部で 8 回でした。
- `close` の返事は、応答の時間に入っていません。`/health` は3回送りましたが、片道 100 ms のときの時間は 231 ms で、3回分（300 ms 以上）より短いからです。待っている往復は、`/health` が2回・`GET /notes` が6回・`POST /login` が4回（探す SQL の2回、セッションを作る `INSERT` と期限切れを消す `DELETE` の1回ずつ）です。例9-1の見積もり（約 2.2・6.5・4.7）の方が少し多めですが、だいたい合います。
- ログイン中のユーザーは、画面の枠とページの両方で使いますが、探す SQL は1回だけでした。`#[memoize]` で、1つのリクエストの中では覚えておいた結果を使うからです（第13巻 UNIT 06）。

##### 解説3: 置き場所の決め方

- 利用者が日本にいるなら、マシンは東京（Fly.io の `nrt`）に、DB も東京に置きます。Turso の東京は `aws-ap-northeast-1` です（Turso の公式のドキュメントの、場所の一覧で確かめました）。同じ地域なら、往復は短く済みます。
- マシンを東京、DB を海外の遠い地域に置くと、例9-1の「遅れ 50〜100 ms」の行のような速さになります。ページの表示ごとに 0.3〜0.7 秒ほど遅くなる計算です。
- 地域（リージョン）は、Fly.io では `fly.toml` の `primary_region` で、Turso では DB を作るとき（`turso db create`）に決めます。

### もう一歩: 実務に近い形

#### 台数の決め方

- **止める／動かすの1台（`min_machines_running = 0`）:** 使われていない間は0台になり、費用が少なくて済みます。そのかわり、止まったあとの最初のリクエストは、マシンの起動と RustNote の起動（UNIT 06 では約 1.1 秒）を待ちます。
- **いつも1台（`min_machines_running = 1`）:** 最初のリクエストも待たせませんが、いつも動いている分の費用がかかります。
- **2台以上:** 1台が止まったり入れ替わったりしても、もう1台が答えられます（UNIT 06・08）。ただし RustNote は起動のたびにマイグレーションをかけるので、同時の起動でぶつからないよう、release command（UNIT 07）を用意します。DB がマシンの中のファイル（`/data`）だと、マシンごとに別の DB になってしまいます。2台以上なら、共通の DB（Turso）が前提です。
- **マシンの大きさ:** Argon2 は1回に 19 MiB 使うので、同時のログインが多いと、512MB では足りなくなることがあります。第11巻 UNIT 09 では、同時に 48 のログインを送るとメモリが 654 MiB まで増え、整理券（同時の数の上限）を付けると 218 MiB に収まりました。

#### 費用の決まり方（2026-10-31 に、公式の料金のページで確かめたもの）

**Fly.io**（マシンの料金は地域で違います。下の値は、公式のページに書かれた、いちばん安い地域（`iad`・`ewr`）のものです）

| 項目 | 料金の決まり方 |
|---|---|
| マシン（動いている間） | 動いた時間で数える。shared-cpu-1x・512MB は月 $3.69 相当 |
| マシン（止めている間） | ルートのファイルシステムの分だけ、1GB あたり月 $0.15 |
| 入ってくる通信 | 無料 |
| 外への通信 | アジア太平洋・オセアニア・南アメリカの地域は、1GB あたり $0.04 |
| IP アドレス | 共有の IPv4 と IPv6 は無料。専用の IPv4 は月 $2（無くてもよい） |

**Turso**

| プラン | 中身 |
|---|---|
| Free（月 $0） | DB 100 個・保存 5GB・1か月に読む行 5 億・書く行 1,000 万・過去に戻せるのは 1 日 |
| Developer（月 $4.99） | DB の数は無制限・保存 9GB・読む行 25 億・書く行 2,500 万・過去に戻せるのは 10 日。超えた分は、保存 1GB $0.75・読む行 10 億 $1・書く行 100 万 $1 |

- Turso は、**読んだ行・書いた行**の数で量が決まります。`GET /notes` の `COUNT(*)` は、その人のノートの行を数えるために読みます。ヘルスチェックの `SELECT 1` も、30 秒ごとに台数分の問い合わせになります（UNIT 06 の問3）。
- 料金は変わることがあります。公開する前に、その時の公式のページで確かめます。

### よくあるエラーと読み方

#### エラー: DB が遠すぎ（遅すぎ）て、ヘルスチェックが時間切れになる

例9-1の遅れを、片道 1,500 ms と 3,000 ms にしました。`fly.toml` の `timeout = "5s"` と同じく、5 秒で打ち切る `curl.exe -m 5` で `/health` を開きました。

```text
遅れ（片道）1500 ms: 状態 200・かかった時間 3.021576 秒・curl の終了コード 0
遅れ（片道）3000 ms: 状態 000・かかった時間 5.006041 秒・curl の終了コード 28
```

- **何が起きたか:** 1,500 ms では約 3 秒で 200 が返りましたが、3,000 ms では5秒で打ち切られました（状態 000、curl の終了コード 28 は時間切れ）。`/health` は2回の往復を待つので、片道 3 秒なら 6 秒以上かかるからです。
- **Fly.io では:** ヘルスチェックが時間切れで失敗し続け、そのマシンには振り分けられなくなります（UNIT 06）。DB が遠いうえに、DB の側が一時的に遅くなると、全部のマシンが一度に「元気でない」になることがあります。
- **直し方:** DB をマシンの近くに置きます。`timeout` を長くするだけでは、利用者の待ち時間は変わりません。

### RustNote ではここで使う

- **fly.toml:** `primary_region = "nrt"`（東京）、`auto_stop_machines = "stop"`・`min_machines_running = 0`、`[[vm]]` は shared 1 CPU・512MB です（UNIT 04）。
- **Turso:** DB は東京（`aws-ap-northeast-1`）に作ると、マシンと同じ地域になります。場所は `turso db create` のときに選びます（この巻では実行しません）。
- **改善点（この単元で見つかったこと）:** `GET /notes` は、DB と6回の往復を待ちます。遠い DB では、この回数がそのまま待ち時間になります。問い合わせの数を減らすと、遠い DB でも速くなります（問3）。

### 練習問題

**問1:** 例9-1の結果から、マシンを東京に、DB を片道 80 ms の所に置いたとき、`GET /notes` はだいたい何 ms になると見積もれるでしょうか。

**問2:** 1日にほとんど使われない、自分だけのメモアプリとして RustNote を公開します。台数とマシンの大きさを、どう決めますか。

**問3:** `GET /notes` の DB との往復を減らすには、RustNote のどこを変えればよいでしょうか。

### 解答と解説

**問1の解答:**

- 遅れ 0 ms で 19 ms でした。片道が 1 ms 増えるごとに約 6.5 ms 増える（100 ms で約 650 ms 増えた）ので、19 ＋ 80 × 6.5 ≒ **540 ms** と見積もれます。例9-1の 50 ms（367 ms）と 100 ms（666 ms）の間に入ります。
- 実際の回線では、遅れにばらつき（jitter）があったり、混んでいたりします。公開したあとに、ログの時間（UNIT 08）で確かめます。

**問2の解答（例）:**

- `min_machines_running = 0` で止める／動かすを有効にしたまま、1台にします。使われていない間の費用は、ルートのファイルシステムの分（1GB あたり月 $0.15）だけになります。最初のリクエストだけ、起動の分（約 1 秒）待ちます。
- 大きさは shared 1 CPU・512MB で十分です。自分だけなら、同時のログインは1つです。DB は Turso の Free プランで、上限（1か月に読む行 5 億）には、まず届きません。

**問3の解答（例）:**

- 一覧の `COUNT(*)` と `SELECT … LIMIT 20` は、同じ接続の、別々の問い合わせです（第13巻 UNIT 05）。例9-2のとおり、読む SQL は1つで2回の往復なので、ここで4回です。
- `COUNT(*) OVER ()`（ウィンドウ関数）を使えば、1ページ分の行と一緒に、条件に合う全部の数も1回の問い合わせで読めます。待つ往復は 6 回から 4 回に減るはずです。
- ただし、最後より後のページでは行が1つも返らないので、全部の数も分からなくなります。その時だけ数え直す、などの工夫が要ります。
- 減ったかどうかは、例9-2で数え、例9-1と同じ実験で、遅れ 100 ms のときの時間を比べれば確かめられます。

### まとめ

- RustNote と DB の間に遅れを入れると、片道 100 ms で `GET /notes` は 19 → 666 ms（約 6.5 回分の往復）、`POST /login` は約 4.7 回分、`/health` は約 2.2 回分になった。遠い DB の遅れは、往復の回数だけ重なる。
- 数えると、libSQL の crate は、読む SQL 1つに `describe` と `cursor` の2回、書く SQL 1つに `batch` の1回を送っていた。`GET /notes` は8回送り、そのうち6回の返事を待つ（`close` は待たない）。
- マシンと DB は、利用者の近くの同じ地域（Fly.io の `nrt`・Turso の `aws-ap-northeast-1`）に置く。台数は、止める／動かすで費用を抑えるか、いつも動かして待たせないかで決める。2台以上なら Turso と release command が前提。
- Fly.io は、動いた時間・止めている間のディスク・外への通信で、Turso は、読んだ行・書いた行・保存の量で費用が決まる（2026-10-31 の公式のページ）。DB が遅すぎると、`/health` が `timeout` の 5 秒に届き、振り分けられなくなる。

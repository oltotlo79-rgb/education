## UNIT 05: ブラウザ・URL・HTTP

### この単元のゴール

- **URL** を「スキーム・ホスト・ポート・パス・クエリ」の部品に分けて読める
- ブラウザに URL を入れてから画面が出るまでに起きることを、順番に説明できる
- **HTTP** の**リクエスト**と**レスポンス**の中身（メソッド・パス・ヘッダー・本文・ステータスコード）を読める
- `curl.exe` を使って、ブラウザの裏で行われているやりとりを自分の目で見られる

### たとえ話でつかむ

Web のしくみは、**決まった書式の注文書を郵送して、返事をもらう**ことにそっくりです。

- **URL は宛先**です。「どの会社（ホスト）の、どの窓口（ポート）の、どの担当（パス）に、どんな条件（クエリ）で」お願いするかが全部書いてあります。
- **DNS（ディーエヌエス）は電話帳**です。`example.com` のような会社名を、郵便で実際に使う番地（**IP アドレス**、例 `172.66.147.243`）に引き直してくれます。
- **HTTP リクエストは注文書**です。1行目に「何をしてほしいか（例: `GET /notes` ＝ ノートの一覧を見せて）」、続けて添え状（**ヘッダー**）、必要なら中身（**本文**）を書きます。書式が決まっているので、世界中のどのサーバーにも通じます。
- **HTTP レスポンスは返事**です。1行目に結果のスタンプ（**ステータスコード**、例 `200 OK` ＝ 成功、`404 Not Found` ＝ 見つからない）、続けて添え状、最後に中身（多くは **HTML**）が入っています。
- **ブラウザは秘書**です。注文書を書いて送り、届いた HTML を読んで、文字や色や画像をきれいに並べた画面にしてくれます。

> **用語:** **HTTP（エイチティーティーピー）** … HyperText Transfer Protocol の略。ブラウザとサーバーがやりとりするときの「注文書と返事の書き方の決まり（**プロトコル**）」です。**HTTPS** は、そのやりとりを暗号化して、途中で盗み見や書き換えをされないようにしたものです。
>
> **用語:** **HTML（エイチティーエムエル）** … Web ページの中身を書く言葉。`<h1>見出し</h1>` のように、タグで文章の役割を示します。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="14" y="20" width="692" height="54" rx="10" fill="#0c1222"/>
<text x="360" y="55" font-size="19" text-anchor="middle" font-family="Cascadia Mono, Consolas, monospace"><tspan fill="#60a5fa">https</tspan><tspan fill="#94a3b8">://</tspan><tspan fill="#fb923c">your-app.fly.dev</tspan><tspan fill="#94a3b8">:</tspan><tspan fill="#facc15">443</tspan><tspan fill="#34d399">/notes</tspan><tspan fill="#94a3b8">?</tspan><tspan fill="#f472b6">q=rust&amp;page=2</tspan></text>
<rect x="14" y="100" width="120" height="104" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="74" y="124" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">スキーム</text>
<text x="74" y="146" font-size="12" fill="#334155" text-anchor="middle">通信のやり方</text>
<text x="74" y="166" font-size="12" fill="#334155" text-anchor="middle">https = 暗号化</text>
<text x="74" y="186" font-size="12" fill="#334155" text-anchor="middle">http = そのまま</text>
<rect x="144" y="100" width="150" height="104" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="219" y="124" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">ホスト</text>
<text x="219" y="146" font-size="12" fill="#334155" text-anchor="middle">どのコンピュータか</text>
<text x="219" y="166" font-size="12" fill="#334155" text-anchor="middle">DNS で IP アドレス</text>
<text x="219" y="186" font-size="12" fill="#334155" text-anchor="middle">に引き直される</text>
<rect x="304" y="100" width="120" height="104" rx="8" fill="#fefce8" stroke="#ca8a04"/>
<text x="364" y="124" font-size="13" font-weight="700" fill="#a16207" text-anchor="middle">ポート</text>
<text x="364" y="146" font-size="12" fill="#334155" text-anchor="middle">どの窓口か</text>
<text x="364" y="166" font-size="12" fill="#334155" text-anchor="middle">https は 443</text>
<text x="364" y="186" font-size="12" fill="#334155" text-anchor="middle">（ふつう省略）</text>
<rect x="434" y="100" width="120" height="104" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="494" y="124" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">パス</text>
<text x="494" y="146" font-size="12" fill="#334155" text-anchor="middle">どのページ・</text>
<text x="494" y="166" font-size="12" fill="#334155" text-anchor="middle">どの担当か</text>
<rect x="564" y="100" width="142" height="104" rx="8" fill="#fdf2f8" stroke="#db2777"/>
<text x="635" y="124" font-size="13" font-weight="700" fill="#be185d" text-anchor="middle">クエリ</text>
<text x="635" y="146" font-size="12" fill="#334155" text-anchor="middle">追加の条件</text>
<text x="635" y="166" font-size="12" fill="#334155" text-anchor="middle">名前=値 を</text>
<text x="635" y="186" font-size="12" fill="#334155" text-anchor="middle">&amp; でつなぐ</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図5-1: URL の分解。「rust を含むノートの2ページ目を見せて」というお願いになっている</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-13" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="20" width="130" height="250" rx="12" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="79" y="48" font-size="15" font-weight="700" fill="#1e40af" text-anchor="middle">ブラウザ</text>
<rect x="576" y="20" width="130" height="250" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="641" y="48" font-size="15" font-weight="700" fill="#c2410c" text-anchor="middle">サーバー</text>
<rect x="290" y="20" width="140" height="44" rx="10" fill="#f8fafc" stroke="#475569"/>
<text x="360" y="47" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">DNS（電話帳）</text>
<line x1="146" y1="40" x2="286" y2="40" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-13)"/>
<text x="216" y="32" font-size="12" fill="#334155" text-anchor="middle">① 名前は？</text>
<line x1="286" y1="56" x2="146" y2="56" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-13)"/>
<text x="216" y="74" font-size="12" fill="#334155" text-anchor="middle">IP アドレス</text>
<line x1="146" y1="104" x2="572" y2="104" stroke="#475569" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-00-13)"/>
<text x="360" y="96" font-size="12" fill="#334155" text-anchor="middle">② 接続する（https なら暗号化の準備も）</text>
<line x1="146" y1="148" x2="572" y2="148" stroke="#1e40af" stroke-width="2.5" marker-end="url(#ar-00-13)"/>
<text x="360" y="140" font-size="12" fill="#1e40af" text-anchor="middle">③ リクエスト: GET /notes HTTP/1.1 ＋ ヘッダー</text>
<rect x="592" y="164" width="98" height="40" rx="8" fill="#ffffff" stroke="#fdba74"/>
<text x="641" y="181" font-size="12" fill="#9a3412" text-anchor="middle">④ 処理して</text>
<text x="641" y="197" font-size="12" fill="#9a3412" text-anchor="middle">HTML を作る</text>
<line x1="572" y1="222" x2="146" y2="222" stroke="#c2410c" stroke-width="2.5" marker-end="url(#ar-00-13)"/>
<text x="360" y="214" font-size="12" fill="#c2410c" text-anchor="middle">⑤ レスポンス: 200 OK ＋ ヘッダー ＋ HTML</text>
<rect x="30" y="234" width="98" height="28" rx="6" fill="#ffffff" stroke="#93c5fd"/>
<text x="79" y="253" font-size="12" fill="#1e40af" text-anchor="middle">⑥ 画面に描く</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図5-2: URL を入れてから画面が出るまで。不具合はこの①〜⑥のどこで起きたかで切り分ける</div>
</div>

HTTP のリクエストとレスポンスは、どちらも「1行目・ヘッダー・空行・本文」という同じ形をしています。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="180" y="22" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">リクエスト（ブラウザ → サーバー）</text>
<rect x="14" y="32" width="332" height="250" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<rect x="24" y="44" width="312" height="30" rx="4" fill="#dbeafe"/>
<text x="32" y="64" font-size="13" font-weight="700" fill="#1e3a8a" font-family="Cascadia Mono, Consolas, monospace">POST /notes HTTP/1.1</text>
<text x="32" y="96" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">Host: your-app.fly.dev</text>
<text x="32" y="116" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">Cookie: session=...</text>
<text x="32" y="136" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">Content-Type: application/</text>
<text x="32" y="152" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">  x-www-form-urlencoded</text>
<text x="32" y="176" font-size="12" fill="#94a3b8">（空行 ＝ ヘッダーの終わり）</text>
<rect x="24" y="188" width="312" height="30" rx="4" fill="#ffffff" stroke="#93c5fd"/>
<text x="32" y="208" font-size="12" fill="#0f172a" font-family="Cascadia Mono, Consolas, monospace">title=牛乳を買う&amp;body=</text>
<text x="32" y="242" font-size="12" fill="#1e40af">1行目: メソッド・パス・HTTP の版</text>
<text x="32" y="262" font-size="12" fill="#1e40af">本文: フォームに入力した内容</text>
<text x="540" y="22" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">レスポンス（サーバー → ブラウザ）</text>
<rect x="374" y="32" width="332" height="250" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<rect x="384" y="44" width="312" height="30" rx="4" fill="#ffedd5"/>
<text x="392" y="64" font-size="13" font-weight="700" fill="#7c2d12" font-family="Cascadia Mono, Consolas, monospace">HTTP/1.1 200 OK</text>
<text x="392" y="96" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">Content-Type: text/html;</text>
<text x="392" y="112" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">  charset=utf-8</text>
<text x="392" y="132" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">Content-Length: 1834</text>
<text x="392" y="176" font-size="12" fill="#94a3b8">（空行 ＝ ヘッダーの終わり）</text>
<rect x="384" y="188" width="312" height="30" rx="4" fill="#ffffff" stroke="#fdba74"/>
<text x="392" y="208" font-size="12" fill="#0f172a" font-family="Cascadia Mono, Consolas, monospace">&lt;!doctype html&gt;&lt;html&gt;...</text>
<text x="392" y="242" font-size="12" fill="#c2410c">1行目: HTTP の版・ステータスコード</text>
<text x="392" y="262" font-size="12" fill="#c2410c">本文: 画面に描く HTML</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図5-3: HTTP のリクエストとレスポンスの形。どちらも「1行目・ヘッダー・空行・本文」（値は例）</div>
</div>

**メソッド**は「何をしてほしいか」を表す動詞です。RustNote で使うのは主に次の2つです。

| メソッド | 意味 | 例 |
|---|---|---|
| `GET` | 見せて（データを変えない） | `GET /notes` ノート一覧を見せて |
| `POST` | 送るので処理して（データが変わる） | `POST /notes` このノートを作って |

**ステータスコード**は返事の結果を表す3桁の数字で、最初の桁でおおまかな意味が分かります。

| コード | 名前 | 意味 | RustNote での例 |
|---|---|---|---|
| `200` | OK | 成功 | 一覧ページを返した |
| `303` | See Other | 「処理したので、こちらの URL を見に行って」 | ノート作成後に一覧へ移動させる |
| `400` | Bad Request | リクエストの形がおかしい | 送られてきた値が読めない |
| `401` / `403` | Unauthorized / Forbidden | ログインが必要／権限がない | （UNIT 08 で説明） |
| `404` | Not Found | 見つからない | 存在しない、または**他人の**ノート |
| `500` | Internal Server Error | サーバー側の失敗 | プログラムの不具合 |

2xx は成功、3xx は「別の場所へ」、4xx は**頼んだ側**の問題、5xx は**サーバー側**の問題、と覚えておくと便利です。

### まず動かす最小コード

#### `curl.exe` でブラウザの裏側をのぞく

**curl（カール）** は、ブラウザの代わりに HTTP のリクエストを送り、返ってきたものを**そのまま文字で見せてくれる**道具です。Windows 10 以降には最初から入っています（UNIT 03 で見たとおり、PowerShell では必ず `curl.exe` と書きます）。

> **▼ このコマンドがやること（先に日本語で）:** 説明用に用意されている公式のサイト `example.com` に HTTP のリクエストを送り、送った注文書（リクエスト）と、返ってきた返事（レスポンス）のヘッダーを表示します。本文の HTML は表示せずに捨てます。

```powershell
curl.exe -4 -sv http://example.com/ -o NUL
```

**▼ 実行結果**（あなたの PC のアドレスの部分は伏せています）

```text
* Host example.com:80 was resolved.
* IPv6: (none)
* IPv4: 172.66.147.243, 104.20.23.154
*   Trying 172.66.147.243:80...
* Established connection to example.com (172.66.147.243 port 80) from 192.168.x.x port 56035
* using HTTP/1.x
> GET / HTTP/1.1
> Host: example.com
> User-Agent: curl/8.21.0
> Accept: */*
>
* Request completely sent off
< HTTP/1.1 200 OK
< Date: Thu, 15 Oct 2026 11:38:59 GMT
< Content-Type: text/html; charset=utf-8
< Transfer-Encoding: chunked
< Connection: keep-alive
< Age: 9
< Server: cloudflare
< Last-Modified: Tue, 13 Oct 2026 22:05:48 GMT
< Allow: GET, HEAD
< Accept-Ranges: bytes
< cf-cache-status: HIT
< CF-RAY: a4ae8a66ff859649-KIX
< alt-svc: h3=":443"; ma=86400
<
{ [589 bytes data]
* Connection #0 to host example.com:80 left intact
```

#### ▼ 出力を1つずつ分解して解説

##### 解説1: コマンドのオプション

```powershell
curl.exe -4 -sv http://example.com/ -o NUL
```

- `-v`（verbose: 詳しく）は、やりとりの中身を表示するオプションです。`-s`（silent）は進み具合の表示を消します。2つまとめて `-sv` と書けます。
- `-4` は「IPv4 という形式のアドレスで接続して」という指定です。表示を短くするために付けています。
- `-o NUL` は「本文は `NUL`（何も残らない特別な場所）に捨てて」という意味です。bash では `-o /dev/null` と書きます。

##### 解説2: `*` の行 — 名前を引いて接続する（図5-2 の①②）

```text
* Host example.com:80 was resolved.
* IPv4: 172.66.147.243, 104.20.23.154
* Established connection to example.com (172.66.147.243 port 80) from 192.168.x.x port 56035
```

- `*` の行は curl 自身の報告です。
- `was resolved` は「DNS で名前を引けた」という意味で、`example.com` の IP アドレスが2つ見つかりました。
- `Established connection` は「接続できた」。`port 80` は、`http` の URL でポートを省略したときに使われる番号です（`https` なら 443）。

##### 解説3: `>` の行 — 送った注文書（図5-2 の③）

```text
> GET / HTTP/1.1
> Host: example.com
> User-Agent: curl/8.21.0
> Accept: */*
>
```

- `>` の行は、**curl がサーバーへ送ったリクエスト**そのものです。
- 1行目 `GET / HTTP/1.1` は「HTTP/1.1 の決まりで、パス `/`（トップページ）を GET（見せて）」。
- `Host:` はどのサイト宛てか、`User-Agent:` は送り主のソフトの名前、`Accept: */*` は「どんな種類の中身でも受け取ります」というヘッダーです。
- 最後の `>` だけの行が**空行**で、「ヘッダーはここまで」の合図です。GET なので本文はありません。

##### 解説4: `<` の行 — 返ってきた返事（図5-2 の⑤）

```text
< HTTP/1.1 200 OK
< Content-Type: text/html; charset=utf-8
< Server: cloudflare
<
{ [589 bytes data]
```

- `<` の行は、**サーバーから返ってきたレスポンス**です。
- 1行目の `200 OK` が結果のスタンプで、「成功」です。
- `Content-Type: text/html; charset=utf-8` は「本文は HTML で、文字コードは UTF-8 です」という添え状です。ブラウザはこれを見て、本文を HTML として描きます。
- 空行の後の `{ [589 bytes data]` は、「本文を 589 バイト受け取った（が `-o NUL` で捨てた）」という報告です。
- `-v` の代わりに `-i` を付けると、ヘッダーと本文の HTML をそのまま表示できます。

#### URL を部品に分けるプログラム

> **▼ このコードがやること（先に日本語で）:** 図5-1 の URL を、文字の区切り（`://`・`/`・`?`・`&`・`=`）を手がかりにして、スキーム・ホスト・パス・クエリに分けて表示します。URL が「決まった区切りを持つ、ただの文字」であることを確かめます。

```rust
fn main() {
    let url = "https://your-app.fly.dev/notes?q=rust&page=2";
    println!("URL 全体: {url}");

    // 「://」の前がスキーム（通信のやり方）
    let (scheme, rest) = url.split_once("://").unwrap();
    println!("スキーム: {scheme}");

    // 最初の「/」の前がホスト（どのコンピュータか）
    let (host, path_and_query) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };
    println!("ホスト  : {host}");

    // 「?」の前がパス（どのページか）、後ろがクエリ（追加の条件）
    let (path, query) = path_and_query
        .split_once('?')
        .unwrap_or((path_and_query, ""));
    println!("パス    : {path}");

    // クエリは「名前=値」が & でつながっている
    for pair in query.split('&') {
        if let Some((name, value)) = pair.split_once('=') {
            println!("クエリ  : {name} = {value}");
        }
    }
}
```

**▼ 実行結果**

```text
URL 全体: https://your-app.fly.dev/notes?q=rust&page=2
スキーム: https
ホスト  : your-app.fly.dev
パス    : /notes
クエリ  : q = rust
クエリ  : page = 2
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 区切りで2つに分ける `split_once`

```rust
let (scheme, rest) = url.split_once("://").unwrap();
```

- `split_once("://")` は、文字列を**最初に見つかった** `://` の前後の2つに分けます。結果は `("https", "your-app.fly.dev/notes?q=rust&page=2")` です。
- 区切りが見つからないこともあるので結果は `Option` で、`.unwrap()` で中身を取り出しています（見つからなければパニックします。練習用の割り切りです）。
- `let (scheme, rest) = ...` は、2つ組の値を2つの変数に一度に受け取る書き方です。

##### 解説2: 位置を探して切り出す

```rust
let (host, path_and_query) = match rest.find('/') {
    Some(i) => (&rest[..i], &rest[i..]),
    None => (rest, "/"),
};
```

- `rest.find('/')` は、最初の `/` が**何文字目（何バイト目）にあるか**を探します。
- `&rest[..i]` は「先頭から i の手前まで」、`&rest[i..]` は「i から最後まで」の切り出しです。
- `/` が無い URL（`https://example.com` など）のときは、パスを `/` とみなしています。

##### 解説3: クエリを1組ずつ読む

```rust
for pair in query.split('&') {
    if let Some((name, value)) = pair.split_once('=') {
        println!("クエリ  : {name} = {value}");
    }
}
```

- `query.split('&')` は `&` で区切った部品を1つずつ取り出します（`q=rust`、`page=2`）。
- `if let Some(...) = ...` は「区切りが見つかったときだけ中を実行する」という書き方です。

> **注意:** 本物の URL には、日本語や記号を `%E7%89%9B` のような形に置き換える決まり（**URL エンコード**）などもあります。実際のアプリでは、こうした細かい決まりまで正しく扱ってくれる部品（Topcoat など）に任せます。ここでは「URL は区切りの決まった文字列だ」と分かれば十分です。

### もう一歩: 実務に近い形

#### HTTP の注文書を、自分の手で書いて送る

ブラウザや curl がしていることは、結局「決まった形の文字を送って、文字を受け取る」だけです。それを Rust で直接やってみます。

> **▼ このコードがやること（先に日本語で）:** `example.com` の 80 番ポートに接続し、HTTP のリクエストを**文字として**組み立てて送ります。返ってきた文字を全部受け取り、空行を境にヘッダーと本文に分けて表示します。

```rust
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // example.com の 80 番ポート（http の窓口）に接続する
    let mut stream = TcpStream::connect("example.com:80")?;

    // HTTP リクエストは「決まった形の文字」。1 行ずつ \r\n で区切り、空行で終わる
    let request = "GET / HTTP/1.1\r\n\
                   Host: example.com\r\n\
                   User-Agent: rust-study\r\n\
                   Connection: close\r\n\
                   \r\n";
    stream.write_all(request.as_bytes())?;

    // サーバーからの返事（レスポンス）を最後まで受け取る
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // 空行（\r\n\r\n）の前がヘッダー、後ろが本文
    let (head, body) = response.split_once("\r\n\r\n").unwrap_or((&response, ""));
    println!("----- ヘッダー部分 -----");
    println!("{head}");
    println!("----- 本文の先頭 60 文字 -----");
    let preview: String = body.chars().take(60).collect();
    println!("{preview}");
    Ok(())
}
```

**▼ 実行結果**

```text
----- ヘッダー部分 -----
HTTP/1.1 200 OK
Date: Sat, 17 Oct 2026 02:30:02 GMT
Content-Type: text/html; charset=utf-8
Transfer-Encoding: chunked
Connection: close
Server: cloudflare
Last-Modified: Tue, 13 Oct 2026 20:08:28 GMT
Allow: GET, HEAD
Accept-Ranges: bytes
Age: 11679
cf-cache-status: HIT
CF-RAY: a4bbe10c7ef7440f-KIX
alt-svc: h3=":443"; ma=86400
----- 本文の先頭 60 文字 -----
241
<!doctype html><html lang=en><head><meta charset=utf-8>
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 接続する `TcpStream::connect`

```rust
let mut stream = TcpStream::connect("example.com:80")?;
```

- **TCP（ティーシーピー）** は、2台のコンピュータの間に「文字を順番どおり確実に届ける管」をつなぐしくみです。HTTP はこの管の上を流れます。
- `connect("example.com:80")` は、名前を引いて（DNS）、80 番ポートに接続します。図5-2 の①②を1行でやっています。
- 接続できたら、`stream` に書けば相手に届き、`stream` から読めば相手からの文字が受け取れます。

##### 解説2: 注文書を文字で組み立てる

```rust
let request = "GET / HTTP/1.1\r\n\
               Host: example.com\r\n\
               User-Agent: rust-study\r\n\
               Connection: close\r\n\
               \r\n";
```

- HTTP の決まりでは、行の区切りは `\r\n`（**CR LF**：復帰と改行の2文字）です。
- 最後の `\r\n` だけの行が空行で、「ヘッダーはここまで」の合図です。これを忘れると、サーバーは続きを待ち続けます。
- 行末の `\` は「ソースコード上の改行と、次の行の先頭の空白を無視して1つの文字列につなげて」という Rust の書き方です。
- `Connection: close` は「返事を送ったら接続を切ってください」というお願いです。これで `read_to_string` が「最後まで読んだ」と分かります。

##### 解説3: 返事を受け取って分ける

```rust
stream.read_to_string(&mut response)?;
let (head, body) = response.split_once("\r\n\r\n").unwrap_or((&response, ""));
```

- `read_to_string` で、接続が切れるまでの全部を `response` に読み込みます。
- 最初の空行（`\r\n\r\n`）でヘッダーと本文を分けています。
- 本文の先頭の `241` は HTML ではありません。ヘッダーに `Transfer-Encoding: chunked` とあるとおり、このサーバーは本文を「塊（chunk）」に分けて送っていて、`241` は「次の塊は 16 進数で 241（＝577）バイト」という目印です。ブラウザや curl は、こうした細かい決まりも自動で処理してくれています。

### よくあるエラーと読み方

#### 名前が引けない（curl のエラー 6）

```text
PS C:\Users\you> curl.exe -sS http://exampel-typo.invalid/
curl: (6) Could not resolve host: exampel-typo.invalid
```

**何が起きたか:** 図5-2 の①で失敗しています。DNS（電話帳）に `exampel-typo.invalid` という名前が載っていませんでした。`(6)` は curl のエラー番号です。

**直し方:** URL の綴りを確かめます。綴りが正しいなら、インターネットにつながっているかを確かめます。

#### 接続できない（curl のエラー 7）

```text
PS C:\Users\you> curl.exe -sS http://127.0.0.1:8080/
curl: (7) Failed to connect to 127.0.0.1:8080 after 2030 ms: Could not connect to server
```

**何が起きたか:** 図5-2 の②で失敗しています。名前は分かった（ここでは `127.0.0.1` を直接指定）けれど、その番地の 8080 番の窓口で**誰も待っていなかった**のです。サーバーを起動し忘れている、ポート番号を間違えている、のどちらかがほとんどです（UNIT 06 で、このポートで待つサーバーを作ります）。

**直し方:** サーバーが起動しているか、ポート番号が合っているかを確かめます。

#### 返事は来たが 404

```text
PS C:\Users\you> curl.exe -s -i http://example.com/notes-not-here
HTTP/1.1 404 Not Found
Date: Thu, 15 Oct 2026 11:39:00 GMT
Content-Type: text/html; charset=utf-8
Transfer-Encoding: chunked
Connection: keep-alive
Server: cloudflare
Age: 9487
cf-cache-status: HIT
```

（本文の HTML は省略しています）

**何が起きたか:** これは**通信としては成功**しています。サーバーまで届き、サーバーが「そのパスのページはありません（404）」と返事をくれました。①〜⑤のうち④の結果です。

**直し方:** パスの綴りを確かめます。自分のサーバーなら、そのパスを受け付ける処理（**ルート**）を作ったかを確かめます。

> **つまずきポイント:** 「ページが表示されない」とひとことで言っても、①名前が引けない、②接続できない、④サーバーが 404 や 500 を返した、⑥ HTML の書き方がおかしい、では原因も直し方もまったく違います。`curl.exe -v` で**どこまで進んだか**を確かめるのが、Web の不具合を調べる第一歩です。

### RustNote ではここで使う

- RustNote のページは、すべて「メソッド＋パス」の組で決まっています（巻頭の表）。たとえば一覧は `GET /notes`、作成は `POST /notes`、検索は `GET /notes?q=買い物` です。
- フォームの送信（POST）に成功したら、RustNote は HTML を直接返さずに **303 See Other** で「`GET /notes` を見に行って」と返します。こうしておくと、ブラウザの再読み込みボタンで同じノートが二重に作られる事故を防げます。この作法を **PRG（Post/Redirect/Get）** と呼びます（第6巻・第10巻）。
- 他の人のノートの URL を開かれたときは、**404** を返します。理由は UNIT 08 で説明します。

### 練習問題

**問題1:** 次の URL を、スキーム・ホスト・ポート・パス・クエリに分けてください。

```text
http://127.0.0.1:8080/notes/42/edit?from=list
```

**問題2:** URL を分けるプログラムを改造して、ホストとポートを分けて表示してください。ポートが書かれていない URL のときは、`http` なら `80`、`https` なら `443` と表示します。

**問題3:** 次の場面で返すステータスコードとして、最もふさわしいものを選んでください（200・303・404・500）。

- a. ノート一覧のページを表示する
- b. ノートを作成したので、一覧のページへ移動してもらう
- c. `/notes/9999` が開かれたが、そんな番号のノートは無い
- d. プログラムの不具合で、処理の途中で失敗した

### 解答と解説

**問題1の解答:** スキームは `http`、ホストは `127.0.0.1`、ポートは `8080`、パスは `/notes/42/edit`、クエリは `from=list` です。ホストに IP アドレスを直接書くこともできます。

**問題2の解答:**

```rust
fn main() {
    let url = "http://127.0.0.1:8080/notes/42/edit?from=list";

    let (scheme, rest) = url.split_once("://").unwrap();
    let (host_and_port, path_and_query) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };

    // 「ホスト:ポート」を「:」で分ける。ポートが書かれていなければ既定値を使う
    let (host, port) = match host_and_port.split_once(':') {
        Some((host, port)) => (host, port),
        None if scheme == "https" => (host_and_port, "443"),
        None => (host_and_port, "80"),
    };

    let (path, query) = path_and_query
        .split_once('?')
        .unwrap_or((path_and_query, ""));

    println!("スキーム: {scheme}");
    println!("ホスト  : {host}");
    println!("ポート  : {port}");
    println!("パス    : {path}");
    println!("クエリ  : {query}");
}
```

**▼ 実行結果**

```text
スキーム: http
ホスト  : 127.0.0.1
ポート  : 8080
パス    : /notes/42/edit
クエリ  : from=list
```

`None if scheme == "https" =>` は「`None` で、しかも条件が成り立つとき」という書き方です。`match` は上から順に調べるので、https でなければ最後の `None =>` に進みます。

**問題3の解答:** a は `200`、b は `303`、c は `404`、d は `500`。4xx は「頼み方（URL）の問題」、5xx は「サーバー側の問題」です。

### まとめ

- URL は「スキーム・ホスト・ポート・パス・クエリ」からなる宛先で、DNS がホスト名を IP アドレスに引き直します。
- HTTP のリクエストとレスポンスは「1行目・ヘッダー・空行・本文」の決まった形の**文字**で、メソッド（GET・POST）とステータスコード（200・303・404・500 など）が要です。
- 表示されないときは `curl.exe -v` で、名前解決・接続・返事のどこまで進んだかを確かめます。


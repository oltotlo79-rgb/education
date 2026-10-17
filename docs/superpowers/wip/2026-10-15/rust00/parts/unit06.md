## UNIT 06: サーバー・ポート・localhost

### この単元のゴール

- **サーバー**とは「リクエストを待ち続けるプロセス」だと説明できる
- **IP アドレス**・**ポート**・**localhost（127.0.0.1）**・**0.0.0.0** の違いを説明できる
- Rust の標準ライブラリだけで小さな Web サーバーを作り、`curl.exe` とブラウザからアクセスできる
- 「ポートが使用中」「実行ファイルを上書きできない」というエラーに対処できる

### たとえ話でつかむ

ネットワークの住所は、**大きなマンション**で考えると分かりやすくなります。

- **IP アドレスはマンションの住所**です。インターネットにつながったコンピュータ1台1台に付いています（例 `172.66.147.243`）。
- **ポートは部屋番号**です。1つのマンション（1台の PC）には 1〜65535 番までの部屋があり、それぞれの部屋に別々の住人（プロセス）が住めます。同じ住所に届いた荷物も、部屋番号で届け先のプロセスが決まります。
- **サーバーは、部屋で来客を待ち続けている住人**です。「8080 号室で待っています」と管理人（OS）に届け出ておくと、8080 号室宛ての訪問者が来るたびに取り次いでもらえます。この届け出を **bind（バインド）**、待つことを **listen（リッスン）** と呼びます。1つの部屋に住めるのは1人だけです。
- **localhost（ローカルホスト）＝ 127.0.0.1 は「このマンションの中」**を表す特別な住所です。内線電話のようなもので、外の人はこの番号にかけられません。開発中は、自分の PC の中だけでサーバーとブラウザをやりとりさせるのに使います。
- **0.0.0.0 で待つのは「どの入口から来た客も受け付ける」**という届け出です。127.0.0.1 で待つと内線（同じ PC の中）からしか来客がありませんが、0.0.0.0 で待つと、ネットワークの外からの来客も受け付けます。

> **用語:** **サーバー** … 頼まれごと（リクエスト）を待ち受けて、返事（レスポンス）を返す側のプログラム、またはそれが動いているコンピュータ。頼む側（ブラウザなど）は **クライアント** と呼びます。
>
> **用語:** **IP アドレス** … ネットワーク上のコンピュータの住所。`127.0.0.1` のような4つの数字の形（IPv4）と、`::1` のような長い形（IPv6）があります。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-14" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="250" y="14" width="456" height="262" rx="14" fill="#f8fafc" stroke="#475569" stroke-width="2"/>
<text x="478" y="40" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">あなたの PC（IP アドレス = 住所）</text>
<rect x="270" y="56" width="132" height="96" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="336" y="80" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">3000 番</text>
<text x="336" y="104" font-size="12" fill="#334155" text-anchor="middle">RustNote</text>
<text x="336" y="122" font-size="12" fill="#334155" text-anchor="middle">（開発中）</text>
<rect x="414" y="56" width="132" height="96" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="3"/>
<text x="480" y="80" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">8080 番</text>
<text x="480" y="104" font-size="12" fill="#334155" text-anchor="middle">練習サーバー</text>
<text x="480" y="122" font-size="12" fill="#334155" text-anchor="middle">practice.exe</text>
<rect x="558" y="56" width="132" height="96" rx="10" fill="#ffffff" stroke="#cbd5e1" stroke-dasharray="5 4"/>
<text x="624" y="80" font-size="13" font-weight="700" fill="#94a3b8" text-anchor="middle">5000 番</text>
<text x="624" y="104" font-size="12" fill="#94a3b8" text-anchor="middle">空き部屋</text>
<text x="624" y="122" font-size="12" fill="#94a3b8" text-anchor="middle">（誰も待っていない）</text>
<rect x="270" y="170" width="420" height="92" rx="10" fill="#eff6ff" stroke="#3b82f6"/>
<text x="286" y="194" font-size="13" font-weight="700" fill="#1e40af">ブラウザ・curl（同じ PC の中のクライアント）</text>
<text x="286" y="218" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">http://127.0.0.1:8080/  → 8080 番へ</text>
<text x="286" y="238" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">http://localhost:8080/  → 同じ意味</text>
<text x="286" y="256" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">http://127.0.0.1:5000/  → 誰もいない</text>
<line x1="480" y1="168" x2="480" y2="156" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-14)"/>
<rect x="14" y="56" width="216" height="206" rx="12" fill="#ffffff" stroke="#94a3b8"/>
<text x="122" y="82" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">覚えておきたい番号</text>
<text x="30" y="110" font-size="12" fill="#334155">80　 http の既定</text>
<text x="30" y="134" font-size="12" fill="#334155">443　https の既定</text>
<text x="30" y="158" font-size="12" fill="#334155">3000 Topcoat の既定</text>
<text x="30" y="182" font-size="12" fill="#334155">8080 Fly.io の RustNote</text>
<text x="30" y="206" font-size="12" fill="#334155">　　 ／ この巻の練習</text>
<text x="30" y="236" font-size="12" fill="#64748b">1つのポートで待てるのは</text>
<text x="30" y="254" font-size="12" fill="#64748b">1つのプロセスだけ</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図6-1: ポートは部屋番号。URL の「:8080」で、どのプロセスに届けるかが決まる</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-15" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
<marker id="ar-00-16" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#dc2626"/></marker>
</defs>
<text x="180" y="22" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">127.0.0.1:8080 で待つ（開発中）</text>
<rect x="14" y="34" width="332" height="170" rx="12" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="30" y="58" font-size="12" font-weight="700" fill="#1e40af">あなたの PC</text>
<rect x="190" y="74" width="140" height="50" rx="8" fill="#ffffff" stroke="#3b82f6"/>
<text x="260" y="104" font-size="12" fill="#1e40af" text-anchor="middle">サーバー</text>
<rect x="30" y="74" width="110" height="50" rx="8" fill="#ffffff" stroke="#3b82f6"/>
<text x="85" y="104" font-size="12" fill="#1e40af" text-anchor="middle">ブラウザ</text>
<line x1="142" y1="99" x2="186" y2="99" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-15)"/>
<text x="30" y="150" font-size="12" fill="#047857">同じ PC の中からは届く</text>
<line x1="180" y1="236" x2="250" y2="132" stroke="#dc2626" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar-00-16)"/>
<text x="30" y="178" font-size="12" fill="#b91c1c">外の PC からは</text>
<text x="30" y="196" font-size="12" fill="#b91c1c">届かない ×</text>
<rect x="110" y="236" width="140" height="40" rx="8" fill="#ffffff" stroke="#94a3b8"/>
<text x="180" y="261" font-size="12" fill="#334155" text-anchor="middle">別の PC・スマホ</text>
<text x="540" y="22" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">0.0.0.0:8080 で待つ（Fly.io）</text>
<rect x="374" y="34" width="332" height="170" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="390" y="58" font-size="12" font-weight="700" fill="#c2410c">Fly.io の上のマシン</text>
<rect x="550" y="74" width="140" height="50" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="620" y="96" font-size="12" fill="#c2410c" text-anchor="middle">RustNote</text>
<text x="620" y="114" font-size="12" fill="#c2410c" text-anchor="middle">HOST=0.0.0.0</text>
<rect x="390" y="74" width="120" height="50" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="450" y="96" font-size="12" fill="#c2410c" text-anchor="middle">Fly.io の</text>
<text x="450" y="114" font-size="12" fill="#c2410c" text-anchor="middle">受付（443）</text>
<line x1="512" y1="99" x2="546" y2="99" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-15)"/>
<text x="620" y="150" font-size="12" fill="#047857" text-anchor="middle">外からの https を</text>
<text x="620" y="168" font-size="12" fill="#047857" text-anchor="middle">8080 番へ取り次ぐ</text>
<line x1="540" y1="236" x2="460" y2="130" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-15)"/>
<rect x="470" y="236" width="140" height="40" rx="8" fill="#ffffff" stroke="#94a3b8"/>
<text x="540" y="261" font-size="12" fill="#334155" text-anchor="middle">世界中のブラウザ</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図6-2: 待ち受ける住所の違い。127.0.0.1 は自分の PC の中だけ、0.0.0.0 は外からも受け付ける</div>
</div>

> **つまずきポイント:** `0.0.0.0` は「**どの入口でも待つ**」という待ち受け側の指定で、ブラウザに打つ宛先ではありません。開発中のサーバーにアクセスするときは `http://127.0.0.1:3000` や `http://localhost:3000` を使います。

### まず動かす最小コード

#### Rust だけで作る、いちばん小さな Web サーバー

> **▼ このコードがやること（先に日本語で）:** 自分の PC（127.0.0.1）の 8080 番ポートで来客を待ちます。リクエストが来るたびに、その1行目を画面に記録し、「Hello from Rust!」という見出しの HTML を HTTP の決まった形で返します。止めるまでずっと待ち続けます。

```rust
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    // 自分の PC（127.0.0.1）の 8080 番ポートで待ち受けを始める
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("http://127.0.0.1:8080 で待っています（止めるには Ctrl+C）");

    // 接続が来るたびに、1 つずつ順番に処理する
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut reader = BufReader::new(&stream);

        // リクエストの 1 行目（例: GET / HTTP/1.1）を読む
        let mut first_line = String::new();
        reader.read_line(&mut first_line)?;
        println!("受け取った: {}", first_line.trim_end());

        // 残りのヘッダー行を、空行が来るまで読み飛ばす
        loop {
            let mut line = String::new();
            let n = reader.read_line(&mut line)?;
            if n == 0 || line == "\r\n" {
                break;
            }
        }

        // 返事（レスポンス）を HTTP の決まった形で組み立てて送る
        let body = "<h1>Hello from Rust!</h1>\n";
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(response.as_bytes())?;
    }
    Ok(())
}
```

PowerShell の窓を**2つ**開きます。1つ目の窓でサーバーを起動します。

```powershell
cargo run
```

サーバーは止まらずに待ち続けます。2つ目の窓から、curl でアクセスします。

```powershell
curl.exe -s -i http://127.0.0.1:8080/
curl.exe -s http://localhost:8080/hello
```

**▼ 実行結果**（2つ目の窓・curl 側）

```text
HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8
Content-Length: 26

<h1>Hello from Rust!</h1>
<h1>Hello from Rust!</h1>
```

**▼ 実行結果**（1つ目の窓・サーバー側の記録）

```text
http://127.0.0.1:8080 で待っています（止めるには Ctrl+C）
受け取った: GET / HTTP/1.1
受け取った: GET /hello HTTP/1.1
```

ブラウザで `http://127.0.0.1:8080/` を開いても、同じ見出しが表示されます。止めるときは、サーバーの窓で **Ctrl キーを押しながら C** を押します。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 部屋を借りて待ち始める（bind）

```rust
let listener = TcpListener::bind("127.0.0.1:8080")?;
```

- `TcpListener::bind` は、OS に「127.0.0.1 の 8080 番で待たせてください」と届け出ます。
- 届け出が通ると `listener`（待ち受け係）が手に入ります。他のプロセスがすでに 8080 番を使っていると、ここで失敗します（「よくあるエラー」で見ます）。

##### 解説2: 来客を1人ずつ迎える

```rust
for stream in listener.incoming() {
    let mut stream = stream?;
    // ...
}
```

- `listener.incoming()` は「来客（接続）の列」です。`for` で1人ずつ取り出し、来客がいないときは**次の来客まで待ちます**。だからこのプログラムは終わりません。
- `stream` は UNIT 05 の `TcpStream` と同じで、読めば相手からの文字、書けば相手への文字になります。今回はサーバー側なので、まず読んでから書きます。

##### 解説3: 注文書を読む

```rust
let mut first_line = String::new();
reader.read_line(&mut first_line)?;
// ...
loop {
    let mut line = String::new();
    let n = reader.read_line(&mut line)?;
    if n == 0 || line == "\r\n" {
        break;
    }
}
```

- `BufReader` は、届いた文字を1行ずつ読みやすくする道具です。
- 1行目（`GET / HTTP/1.1`）を読んで記録したあと、空行（`"\r\n"`）が来るまでヘッダーを読み飛ばします。
- ヘッダーを読まずに返事をして接続を閉じると、相手側で「接続が途中で切られた」というエラーになることがあるため、最後まで読んでいます。
- `n == 0` は「相手が何も送らずに接続を閉じた」場合です。

##### 解説4: 返事を書く

```rust
let response = format!(
    "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
    body.len(),
    body
);
stream.write_all(response.as_bytes())?;
```

- `format!` は、`{}` の場所に値を埋め込んだ文字列を作ります（`println!` の「表示せずに文字列を作る」版）。
- UNIT 05 の図5-3 と同じ「1行目・ヘッダー・空行・本文」の形です。
- `Content-Length` は本文の**バイト数**です。`body.len()` はバイト数を返すので、ここに使えます（文字数とバイト数の違いは UNIT 10 で見ます）。
- `write_all` で相手に送ります。`stream` の処理が終わると接続は自動で閉じられます。

### もう一歩: 実務に近い形

本物の Web アプリは、URL のパスによって返す内容を変えます。この振り分けを **ルーティング** と呼びます。

#### パスで返事を変える（ルーティング）

> **▼ このコードがやること（先に日本語で）:** リクエストの1行目からメソッドとパスを取り出し、`GET /` ならトップページ、`GET /health` なら `ok`、それ以外なら「404 ページが見つかりません」を返します。どのリクエストに何を返したかを、サーバーの画面に記録します。

```rust
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("http://127.0.0.1:8080 で待っています（止めるには Ctrl+C）");

    for stream in listener.incoming() {
        handle(stream?)?;
    }
    Ok(())
}

// 1 つの接続（1 回のリクエスト）を処理する
fn handle(mut stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(&stream);

    // 1 行目「GET /health HTTP/1.1」を、空白で 3 つに分ける
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");

    // 残りのヘッダー行は空行まで読み飛ばす
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        if n == 0 || line == "\r\n" {
            break;
        }
    }

    // メソッドとパスの組み合わせで、返す内容を決める（ルーティング）
    let (status, body) = match (method, path) {
        ("GET", "/") => ("200 OK", "<h1>トップページ</h1>\n"),
        ("GET", "/health") => ("200 OK", "ok\n"),
        _ => ("404 Not Found", "<h1>ページが見つかりません</h1>\n"),
    };
    println!("{method} {path} -> {status}");

    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(response.as_bytes())?;
    Ok(())
}
```

サーバーを起動したまま、別の窓から curl で3回アクセスし、最後にブラウザで `http://127.0.0.1:8080/` を開きます。

```powershell
curl.exe -s http://127.0.0.1:8080/health
curl.exe -s -i http://127.0.0.1:8080/notes
curl.exe -s -i -X POST http://127.0.0.1:8080/health
```

**▼ 実行結果**（curl 側）

```text
ok
HTTP/1.1 404 Not Found
Content-Type: text/html; charset=utf-8
Content-Length: 43

<h1>ページが見つかりません</h1>

HTTP/1.1 404 Not Found
Content-Type: text/html; charset=utf-8
Content-Length: 43

<h1>ページが見つかりません</h1>
```

**▼ 実行結果**（サーバー側の記録）

```text
http://127.0.0.1:8080 で待っています（止めるには Ctrl+C）
GET /health -> 200 OK
GET /notes -> 404 Not Found
POST /health -> 404 Not Found
GET / -> 200 OK
GET /favicon.ico -> 404 Not Found
  -> 404 Not Found
```

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="340" height="230" rx="10" fill="#ffffff" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="10" y="10" width="340" height="36" rx="10" fill="#e2e8f0"/>
<rect x="10" y="36" width="340" height="10" fill="#e2e8f0"/>
<circle cx="30" cy="28" r="5" fill="#f87171"/>
<circle cx="46" cy="28" r="5" fill="#fbbf24"/>
<circle cx="62" cy="28" r="5" fill="#34d399"/>
<rect x="78" y="17" width="262" height="22" rx="11" fill="#ffffff" stroke="#cbd5e1"/>
<text x="90" y="33" font-size="12" fill="#334155">127.0.0.1:8080</text>
<text x="26" y="86" font-size="24" font-weight="700" fill="#0f172a">トップページ</text>
<text x="26" y="222" font-size="12" fill="#94a3b8">（CSS が無いので、飾りのない見た目）</text>
<rect x="370" y="10" width="340" height="230" rx="10" fill="#0c1222" stroke="#334155"/>
<text x="386" y="34" font-size="12" fill="#94a3b8">PowerShell（サーバー側）</text>
<g font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" font-size="12" fill="#e2e8f0" xml:space="preserve" style="white-space: pre">
<text x="386" y="62">http://127.0.0.1:8080 で待っています</text>
<text x="386" y="84">GET /health -&gt; 200 OK</text>
<text x="386" y="106">GET /notes -&gt; 404 Not Found</text>
<text x="386" y="128">POST /health -&gt; 404 Not Found</text>
<text x="386" y="150" fill="#facc15">GET / -&gt; 200 OK</text>
<text x="386" y="172" fill="#facc15">GET /favicon.ico -&gt; 404 Not Found</text>
<text x="386" y="194" fill="#facc15">  -&gt; 404 Not Found</text>
</g>
<text x="386" y="226" font-size="12" fill="#facc15">黄色 = ブラウザで1回開いたときの記録</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図6-3: ブラウザで開いた画面（左）と、そのときのサーバーの記録（右）</div>
</div>

#### ▼ コードを1つずつ分解して解説

##### 解説1: 処理を関数に分ける

```rust
for stream in listener.incoming() {
    handle(stream?)?;
}
```

- 1回分のリクエストの処理を `handle` という関数に切り出しました。`main` は「待って、来たら `handle` に任せる」だけになり、読みやすくなります。
- `fn handle(mut stream: TcpStream) -> std::io::Result<()>` は、「`TcpStream` を1つ受け取り、成功か失敗を返す関数」という意味です。

##### 解説2: メソッドとパスを取り出す

```rust
let mut parts = first_line.split_whitespace();
let method = parts.next().unwrap_or("");
let path = parts.next().unwrap_or("");
```

- `split_whitespace()` は空白で区切って、部品を1つずつ取り出せるようにします。`GET /health HTTP/1.1` なら `GET`、`/health`、`HTTP/1.1` の順です。
- `.next()` で次の部品を1つ取り出します。無ければ `unwrap_or("")` で空文字にします。

##### 解説3: `match` で振り分ける

```rust
let (status, body) = match (method, path) {
    ("GET", "/") => ("200 OK", "<h1>トップページ</h1>\n"),
    ("GET", "/health") => ("200 OK", "ok\n"),
    _ => ("404 Not Found", "<h1>ページが見つかりません</h1>\n"),
};
```

- `(method, path)` の組を、上から順に比べます。`_` は「それ以外ぜんぶ」です。
- `POST /health` が 404 になったのは、`("GET", "/health")` とメソッドが違うからです。**同じパスでもメソッドが違えば別の注文**として扱われます。
- これが **ルーティング** の正体です。Topcoat などのフレームワークは、これをもっと便利で安全な形で用意してくれます。

##### 解説4: ブラウザは頼んでいないものも頼む

- ブラウザで1回開いただけなのに、記録が3行増えました。
- `GET /favicon.ico` は、ブラウザがタブに表示する小さなアイコン（**ファビコン**）を自動で取りに来たものです。
- 最後の `  -> 404 Not Found`（メソッドもパスも空）は、ブラウザが「次に使うかもしれない」と先回りして開いた予備の接続が、何も送らずに閉じられたものです（解説3 の `n == 0` の場合）。
- サーバーの記録には、こうした「人が頼んでいないリクエスト」も混ざります。記録を読むときに慌てないようにしましょう。

### よくあるエラーと読み方

#### ポートがすでに使われている

サーバーを動かしたまま、同じプログラムをもう1つ起動すると、次のエラーになります。

```text
Error: Os { code: 10048, kind: AddrInUse, message: "通常、各ソケット アドレスに対してプロトコル、ネットワーク アドレス、またはポートのどれか 1 つのみを使用できます。" }
```

**何が起きたか:** `bind` で 8080 番を借りようとしましたが、すでに別のプロセスが住んでいました。`AddrInUse`（アドレス使用中）、Windows のエラー番号は 10048 です。

**直し方:** 先に動いているサーバーを止めます（その窓で Ctrl+C）。どのプロセスが使っているか分からないときは、次のコマンドで調べられます。

```powershell
Get-NetTCPConnection -LocalPort 8080 -State Listen | Select-Object LocalAddress, LocalPort, State, OwningProcess
Get-Process -Id (Get-NetTCPConnection -LocalPort 8080 -State Listen).OwningProcess
netstat -ano | findstr :8080
```

**▼ 実行結果**

```text
LocalAddress LocalPort  State OwningProcess
------------ ---------  ----- -------------
127.0.0.1         8080 Listen         54588


Handles  NPM(K)    PM(K)      WS(K)     CPU(s)     Id  SI ProcessName
-------  ------    -----      -----     ------     --  -- -----------
     67       6      864       6396       0.00  54588   1 practice


  TCP    127.0.0.1:8080         0.0.0.0:0              LISTENING       54588
```

- 1つ目のコマンドは「8080 番で待ち受けている（`Listen`）接続」を探し、持ち主のプロセス番号（`OwningProcess`）を表示します。
- 2つ目は、その番号のプロセスの情報を表示します。`ProcessName` が `practice` なので、さっき起動した自分のサーバーだと分かります。
- 3つ目の `netstat` は昔からあるコマンドで、最後の数字が同じプロセス番号です。

自分のサーバーなら、`Stop-Process -Id 54588` で止めることもできます。

#### サーバーを動かしたまま `cargo build` した

```text
   Compiling practice v0.1.0 (C:\Users\you\rust-study\practice)
error: failed to remove file `C:\Users\you\rust-study\practice\target\debug\practice.exe`

Caused by:
  アクセスが拒否されました。 (os error 5)
```

**何が起きたか:** Cargo は新しい `practice.exe` を作る前に古いファイルを消そうとしますが、Windows では**実行中のプログラムのファイルは消せません**。そのため「アクセスが拒否されました（OS のエラー 5）」になりました。`Caused by:`（原因は）の下に、OS からの本当の理由が書かれています。

**直し方:** サーバーを Ctrl+C で止めてから、もう一度 `cargo build`（または `cargo run`）します。サーバーを作っているときは「直す → 止める → 起動し直す」の順番を習慣にしましょう。

### RustNote ではここで使う

- RustNote は Topcoat の `topcoat::start` でサーバーを起動します。待ち受ける場所は環境変数 `HOST` と `PORT` で決まり、何も設定しなければ **`127.0.0.1:3000`** です。開発中はブラウザで **`http://127.0.0.1:3000`** を開きます。この巻の練習サーバーが 8080 番なのは、RustNote とぶつからないようにするためです。
- Fly.io の上では、`HOST=0.0.0.0`・`PORT=8080` を設定して起動します。0.0.0.0 にしないと、Fly.io の受付からの取り次ぎを受けられません（第12巻）。
- RustNote にも、`ok` と返すだけの `GET /health` を作ります。Fly.io はこの URL に定期的にアクセスして、「サーバーが生きているか」を確かめます（**ヘルスチェック**、第9巻・第12巻）。

### 練習問題

**問題1:** 「もう一歩」のサーバーに、`GET /about` で「このサーバーについて」という見出しを返すルートを追加し、curl で確かめてください。

**問題2:** 次の説明のうち正しいものをすべて選んでください。

- a. `127.0.0.1:8080` で待っているサーバーには、同じ Wi-Fi の別の PC からアクセスできる
- b. `localhost` と `127.0.0.1` は、どちらも自分の PC を指す
- c. 1つのポートでは、1つのプロセスしか待ち受けられない
- d. ブラウザで `http://0.0.0.0:3000` を開くのが、Fly.io にアクセスする正しい方法である

**問題3:** `cargo run` したら `AddrInUse` のエラーが出ました。調べて解決する手順を書いてください。

### 解答と解説

**問題1の解答:** `match` に1行追加します（他の部分は「もう一歩」のコードと同じです）。

```rust
    let (status, body) = match (method, path) {
        ("GET", "/") => ("200 OK", "<h1>トップページ</h1>\n"),
        ("GET", "/health") => ("200 OK", "ok\n"),
        ("GET", "/about") => ("200 OK", "<h1>このサーバーについて</h1>\n"),
        _ => ("404 Not Found", "<h1>ページが見つかりません</h1>\n"),
    };
```

```powershell
curl.exe -s -i http://127.0.0.1:8080/about
```

**▼ 実行結果**

```text
HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8
Content-Length: 40

<h1>このサーバーについて</h1>
```

サーバー側には `GET /about -> 200 OK` と記録されます。追加する場所は `_ =>` の行より**上**です。`_` は「それ以外ぜんぶ」なので、その下に書いた行には決して届きません。

**問題2の解答:** 正しいのは **b と c** です。a は誤りで、127.0.0.1 は自分の PC の中だけです。d も誤りで、0.0.0.0 は待ち受け側の指定です。Fly.io のアプリには `https://アプリ名.fly.dev` の URL でアクセスします。

**問題3の解答:**

1. `Get-NetTCPConnection -LocalPort ポート番号 -State Listen` で、そのポートを使っているプロセスの番号（OwningProcess）を調べる
2. `Get-Process -Id 番号` で、それが何のプログラムかを確かめる
3. 自分が前に起動したサーバーなら、その窓で Ctrl+C するか `Stop-Process -Id 番号` で止める。別の大事なアプリなら、自分のサーバーのポート番号のほうを変える

### まとめ

- サーバーは、OS に届け出たポート（部屋番号）で来客を待ち続けるプロセスです。1つのポートで待てるのは1つのプロセスだけです。
- `127.0.0.1`（localhost）は自分の PC の中だけ、`0.0.0.0` で待つと外からも受け付けます。RustNote は手元で `127.0.0.1:3000`、Fly.io で `0.0.0.0:8080` です。
- パスとメソッドで処理を振り分けるのがルーティングです。ポートの使用中・実行ファイルのロックは「先に動いているものを止める」で解決します。


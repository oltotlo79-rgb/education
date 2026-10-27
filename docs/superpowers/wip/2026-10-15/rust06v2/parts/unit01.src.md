## UNIT 01: HTTP の要求・応答・状態コード

### この単元のゴール

- HTTP の**要求**（要求行・ヘッダー・空行・本文）と**応答**（状態行・ヘッダー・空行・本文）の形を、実際に届いた文字で説明できる
- 標準ライブラリだけの小さなサーバーを動かし、`curl.exe -i` と `-v` の表示を読める
- 状態コード（200・301・303・403・404・405）が、それぞれ何を伝えるのかを説明できる
- GET（見るだけ）と POST（変える）の違いと、POST の後に 303 で GET の画面へ移す（PRG）理由を説明できる

### たとえ話でつかむ

役所の窓口に、申請書を出す場面を考えます。

- **申請書**のいちばん上には、「何をしたいか」（閲覧したい・登録したい）と「どの書類について」（住民票・印鑑登録）を書きます。その下に、名前や連絡先のような**添え書き**を並べ、1行空けてから、**中身**（登録する内容）を書きます。
- 窓口の人は、申請書を読んで、**返事の紙**を返します。いちばん上には「受け付けました」「その書類はありません」「その窓口ではできません」のような**結果の番号**が書いてあり、その下に添え書き、1行空けて中身（書類の写しなど）が続きます。
- 「登録」の申請が終わったら、窓口の人は「控えは、隣の閲覧の窓口で受け取ってください」と案内します。登録の窓口に何度も並び直すと、同じ登録が二重にされてしまうからです。

Web のブラウザとサーバーのやり取り（HTTP）も、同じ形をしています。

| 役所の窓口 | HTTP |
|---|---|
| 申請書のいちばん上（何を・どの書類について） | **要求行**（メソッドとパス。`GET /hello HTTP/1.1`） |
| 申請書の添え書き | **ヘッダー**（`Host:`・`User-Agent:`・`Content-Length:` など） |
| 1行空けてから書く中身 | **本文**（フォームの値など。GET には、ふつう無い） |
| 返事の紙の結果の番号 | **状態コード**（`200 OK`・`404 Not Found` など） |
| 閲覧の窓口・登録の窓口 | **GET**（見るだけ）・**POST**（変える） |
| 「控えは閲覧の窓口で」 | **303 See Other**（POST の後に、GET の画面へ移す。PRG） |

> **用語:** HTTP（エイチティーティーピー。HyperText Transfer Protocol：ブラウザとサーバーが、要求と応答をやり取りするときの決まり）

<!-- -->

> **用語:** 要求（ようきゅう。request：ブラウザなどからサーバーへの頼み。リクエストとも言う）／応答（おうとう。response：サーバーからの返事。レスポンスとも言う）

<!-- -->

> **用語:** 状態コード（じょうたいコード。status code：応答の1行目にある3桁の数。結果の種類を表す。ステータスコードとも言う）

### 図で理解する

要求と応答は、どちらも「1行目・ヘッダー・空行・本文」の4つの部分でできています。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 320" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0101" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="180" y="22" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">要求（curl → サーバー）</text>
<rect x="20" y="34" width="320" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="34" y="56" font-size="12" fill="#0f172a" font-family="Consolas, monospace">POST /notes HTTP/1.1</text>
<rect x="20" y="76" width="320" height="110" rx="6" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="34" y="98" font-size="12" fill="#0f172a" font-family="Consolas, monospace">Host: 127.0.0.1:3000</text>
<text x="34" y="120" font-size="12" fill="#0f172a" font-family="Consolas, monospace">User-Agent: curl/8.21.0</text>
<text x="34" y="142" font-size="12" fill="#0f172a" font-family="Consolas, monospace">Content-Length: 24</text>
<text x="34" y="164" font-size="11" fill="#0f172a" font-family="Consolas, monospace">Content-Type: application/x-www-...</text>
<rect x="20" y="194" width="320" height="24" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="1" stroke-dasharray="4 3"/>
<text x="180" y="211" font-size="11" fill="#475569" text-anchor="middle">（空行: ヘッダーの終わり）</text>
<rect x="20" y="226" width="320" height="34" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="34" y="248" font-size="12" fill="#0f172a" font-family="Consolas, monospace">title=shopping&amp;body=milk</text>
<text x="540" y="22" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">応答（サーバー → curl）</text>
<rect x="380" y="34" width="320" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="394" y="56" font-size="12" fill="#0f172a" font-family="Consolas, monospace">HTTP/1.1 200 OK</text>
<rect x="380" y="76" width="320" height="110" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="394" y="98" font-size="11" fill="#0f172a" font-family="Consolas, monospace">Content-Type: text/plain; charset=utf-8</text>
<text x="394" y="120" font-size="12" fill="#0f172a" font-family="Consolas, monospace">Content-Length: 26</text>
<rect x="380" y="194" width="320" height="24" rx="6" fill="#f8fafc" stroke="#94a3b8" stroke-width="1" stroke-dasharray="4 3"/>
<text x="540" y="211" font-size="11" fill="#475569" text-anchor="middle">（空行: ヘッダーの終わり）</text>
<rect x="380" y="226" width="320" height="34" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="394" y="248" font-size="12" fill="#0f172a">こんにちは、HTTP！</text>
<text x="180" y="290" font-size="12" fill="#1e40af" text-anchor="middle">1行目 = メソッド・パス・版</text>
<text x="540" y="290" font-size="12" fill="#047857" text-anchor="middle">1行目 = 版・状態コード・説明</text>
<text x="360" y="312" font-size="12" fill="#0f172a" text-anchor="middle">Content-Length は本文の「バイト数」。受け取る側は、この数だけ本文を読む</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: HTTP の要求と応答の形。どちらも「1行目・ヘッダー・空行・本文」でできている。この単元の例で、実際に届いた・返した文字</div>
</div>

- **1行目**だけが、要求と応答で違います。要求は「メソッド（`GET`・`POST` など）・パス（`/notes`）・版」、応答は「版・状態コード（`200`）・その説明（`OK`）」です。
- **ヘッダー**は「名前: 値」の行の並びです。**空行**（何も書いていない1行）で、ヘッダーが終わったことを知らせます。
- **本文**は、空行の後ろです。どこまでが本文かは、ヘッダーの `Content-Length`（本文の**バイト数**）で伝えます（エラー1）。

状態コードは、最初の数字で、おおまかな種類が分かります。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="20" width="160" height="200" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="100" y="48" font-size="16" font-weight="800" fill="#047857" text-anchor="middle">2xx</text>
<text x="100" y="70" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">うまくいった</text>
<text x="100" y="104" font-size="12" fill="#0f172a" text-anchor="middle">200 OK</text>
<text x="100" y="124" font-size="11" fill="#475569" text-anchor="middle">頼まれた物を返す</text>
<rect x="196" y="20" width="160" height="200" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="276" y="48" font-size="16" font-weight="800" fill="#1e40af" text-anchor="middle">3xx</text>
<text x="276" y="70" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">別の場所へ</text>
<text x="276" y="104" font-size="12" fill="#0f172a" text-anchor="middle">301 Moved</text>
<text x="276" y="120" font-size="12" fill="#0f172a" text-anchor="middle">Permanently</text>
<text x="276" y="138" font-size="11" fill="#475569" text-anchor="middle">ずっと引っ越した</text>
<text x="276" y="170" font-size="12" fill="#0f172a" text-anchor="middle">303 See Other</text>
<text x="276" y="188" font-size="11" fill="#475569" text-anchor="middle">GET で別の所を見て</text>
<rect x="372" y="20" width="160" height="200" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="452" y="48" font-size="16" font-weight="800" fill="#c2410c" text-anchor="middle">4xx</text>
<text x="452" y="70" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">頼み方が違う</text>
<text x="452" y="100" font-size="12" fill="#0f172a" text-anchor="middle">400 Bad Request</text>
<text x="452" y="124" font-size="12" fill="#0f172a" text-anchor="middle">403 Forbidden</text>
<text x="452" y="148" font-size="12" fill="#0f172a" text-anchor="middle">404 Not Found</text>
<text x="452" y="172" font-size="12" fill="#0f172a" text-anchor="middle">405 Method</text>
<text x="452" y="188" font-size="12" fill="#0f172a" text-anchor="middle">Not Allowed</text>
<rect x="548" y="20" width="152" height="200" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="624" y="48" font-size="16" font-weight="800" fill="#b91c1c" text-anchor="middle">5xx</text>
<text x="624" y="70" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">サーバーの失敗</text>
<text x="624" y="104" font-size="12" fill="#0f172a" text-anchor="middle">500 Internal</text>
<text x="624" y="120" font-size="12" fill="#0f172a" text-anchor="middle">Server Error</text>
<text x="624" y="148" font-size="12" fill="#0f172a" text-anchor="middle">503 Service</text>
<text x="624" y="164" font-size="12" fill="#0f172a" text-anchor="middle">Unavailable</text>
<text x="360" y="242" font-size="12" fill="#0f172a" text-anchor="middle">4xx は「頼んだ側を直せば通る」、5xx は「頼んだ側には直せない」</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: 状態コードの種類。最初の数字で、うまくいった（2）・別の場所へ（3）・頼み方が違う（4）・サーバーの失敗（5）が分かる</div>
</div>

### まず動かす最小コード

#### 例1-1: 届いた要求を、そのまま表示するサーバー

> **▼ このコードがやること（先に日本語で）:** `127.0.0.1:3000` で接続を待ちます。接続が来たら、要求の1行目とヘッダーを空行まで1行ずつ読み、`Content-Length` があれば、その長さだけ本文も読みます。読んだ要求をそのまま画面に表示し、「こんにちは、HTTP！」という文字を、HTTP の応答の形にして返します。外のクレートは使いません。

```rust
@@FILE:http_lab/src/bin/u01_raw.rs@@
```

練習用のプロジェクトの `src/bin/u01_raw.rs` として保存し、1つ目の PowerShell の窓で起動します。

```powershell
cargo run --bin u01_raw
```

2つ目の PowerShell の窓で、`curl.exe` を使って、GET の要求と POST の要求を1回ずつ送ります。`-i` は、応答の1行目とヘッダーも表示する指定、`-d` は、本文にフォームの値を入れて POST で送る指定です。

```powershell
curl.exe -i "http://127.0.0.1:3000/hello?name=ada"
curl.exe -i -d "title=shopping&body=milk" http://127.0.0.1:3000/notes
```

**▼ 実行結果（curl の窓）**

```text
@@OUT:u01_raw_curl@@
```

```text
@@OUT:u01_raw_post@@
```

**▼ 実行結果（サーバーの窓。3つ目の要求は、問3の `curl.exe -v` で送ったものです）**

```text
@@OUT:u01_raw_server@@
```

> **注意:** PowerShell 5.1 で、curl の窓の日本語が `縺薙ｓ縺ｫ縺｡縺ｯ` のように化けるときは、最初の `curl.exe` の前に `[Console]::OutputEncoding = [System.Text.Encoding]::UTF8` を実行します。サーバーが返した UTF-8 の文字を、PowerShell が別の文字コードで読んでいるのが原因です。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 要求は、ただの文字の並び

- サーバーの窓には、curl が送ってきた文字が、そのまま表示されました。1行目の `GET /hello?name=ada HTTP/1.1` が要求行で、その下の `Host:`・`User-Agent:`・`Accept:` がヘッダーです。curl は、何も指定しなくても、この3つのヘッダーを付けて送ります。
- URL の `?name=ada` の部分（クエリ）は、パスの一部として要求行に入っています。
- POST のほうには、`Content-Length: 24` と `Content-Type: application/x-www-form-urlencoded`（フォームの形の本文です）が加わり、空行の後ろに、本文の `title=shopping&body=milk` が届きました。24 は、この本文のバイト数です。

##### 解説2: 応答も、自分で書いた文字

- curl の窓の `HTTP/1.1 200 OK` から `こんにちは、HTTP！` までは、`format!` で作った文字列そのものです。行の終わりは `\r\n`（改行の前に「行の先頭へ戻る」を付けた2文字）で、HTTP の決まりです。
- このサーバーは、どんなパスやメソッドが来ても、同じ応答を返します。`/notes` に POST しても、何も保存していません。「どのパスに、どう答えるか」を決めるのが、次の例1-2と、UNIT 02 から使うフレームワーク（Topcoat）の役目です。

##### 解説3: TcpListener は、接続を1つずつ受け取る

- `TcpListener::bind("127.0.0.1:3000")` は、「この PC の 3000 番の窓口で、接続を待つ」という意味です。`127.0.0.1` は「この PC 自身」を表す特別なアドレスで、ほかの PC からはつながりません。
- `listener.incoming()` は、接続が来るたびに1つずつ取り出します。このサーバーは、1つの接続を処理し終えるまで、次の接続を受け付けません。たくさんの人が同時に使うサーバーでは、async（第5巻）で同時にさばきます。Topcoat は、その部分を引き受けてくれます。

### もう一歩: 実務に近い形

#### 例1-2: メソッドとパスで、返す状態コードを変える

> **▼ このコードがやること（先に日本語で）:** 要求行を空白で区切って、メソッドとパスを取り出します。その組み合わせで、200（トップページ）・405（使えないメソッド）・403（見られないページ）・301（引っ越したページ）・303（伝言を受け取ったので、一覧を見てください）・404（無いページ）を返し分けます。`POST /messages` で受け取った伝言は、プログラムの中の `Vec` に貯め、`GET /messages` で一覧にします。

```rust
@@FILE:http_lab/src/bin/u01_status.rs@@
```

`src/bin/u01_status.rs` として保存し、例1-1のサーバーを Ctrl+C で止めてから起動します。2つ目の窓から、いろいろな要求を送ります。`-X DELETE` はメソッドを DELETE にする指定、`-I` は HEAD（ヘッダーだけをください）で送る指定、`-L` は 3xx の応答が来たら `Location` の場所へ自動でたどる指定です。

```powershell
cargo run --bin u01_status
```

**▼ 実行結果（curl の窓。`###` の行は、送ったコマンドです）**

```text
@@OUT:u01_status_curl@@
```

**▼ 実行結果（サーバーの窓）**

```text
@@OUT:u01_status_server@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 404・403・405 は、それぞれ違う「断り方」

- `/nothing` は、そもそも無いページなので **404 Not Found** です。`/admin` は、ページはあるが見せられないので **403 Forbidden** です。
- `DELETE /` は、ページはあるが、その**メソッド**では使えないので **405 Method Not Allowed** です。405 のときは、`Allow: GET, HEAD` のヘッダーで「使えるメソッドはこれです」と教えるのが HTTP の決まりです。
- どれも「頼んだ側を直せば通る」4xx の仲間ですが、直し方が違います（404 は URL を、403 は権限を、405 はメソッドを直す）。だから、使い分けると、使う人に正しい直し方が伝わります。

##### 解説2: 301 と 303 は、Location で行き先を教える

- `/old-page` は **301 Moved Permanently** と `Location: /` を返しました。「このページは、ずっと `/` に引っ越しました」という意味です。ブラウザや検索エンジンは、次からは新しい場所を使います。
- `POST /messages` は **303 See Other** と `Location: /messages` を返しました。「受け取りました。結果は `GET /messages` で見てください」という意味です。`-L` を付けた最後のコマンドでは、curl が `Location` をたどって、自動で `GET /messages` を送り、`伝言は 2 件: ["hello", "world"]` を受け取りました。サーバーの窓にも、`POST /messages` の後に `GET /messages` が届いています。

##### 解説3: POST の後に 303 で GET へ移す（PRG）

- もし `POST /messages` が、303 ではなく 200 で一覧を返していたら、ブラウザの画面は「POST の結果」のままです。利用者が再読み込み（F5）を押すと、ブラウザは**同じ POST をもう一度送り**、伝言が二重に保存されます。
- 303 で GET の画面へ移しておけば、再読み込みで送り直されるのは `GET /messages`（見るだけ）なので、何も増えません。この「POST → Redirect（303）→ GET」の形を、頭文字をとって **PRG** と呼びます。RustNote のフォームも、すべてこの形です（第9巻 UNIT 02）。
<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0102" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="60" y="14" width="140" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="130" y="36" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">ブラウザ</text>
<rect x="520" y="14" width="140" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="590" y="36" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">サーバー</text>
<line x1="130" y1="48" x2="130" y2="320" stroke="#94a3b8" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="590" y1="48" x2="590" y2="320" stroke="#94a3b8" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="130" y1="78" x2="586" y2="78" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0102)"/>
<text x="360" y="70" font-size="12" fill="#0f172a" text-anchor="middle">① POST /messages（message=hello）</text>
<text x="600" y="98" font-size="11" fill="#047857">伝言を保存</text>
<line x1="590" y1="122" x2="134" y2="122" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0102)"/>
<text x="360" y="114" font-size="12" fill="#0f172a" text-anchor="middle">② 303 See Other（Location: /messages）</text>
<line x1="130" y1="166" x2="586" y2="166" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0102)"/>
<text x="360" y="158" font-size="12" fill="#0f172a" text-anchor="middle">③ GET /messages（ブラウザが自動で送る）</text>
<line x1="590" y1="210" x2="134" y2="210" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0102)"/>
<text x="360" y="202" font-size="12" fill="#0f172a" text-anchor="middle">④ 200 OK（伝言の一覧）</text>
<text x="20" y="246" font-size="11" fill="#c2410c">再読み込み</text>
<line x1="130" y1="254" x2="586" y2="254" stroke="#c2410c" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-06-0102)"/>
<text x="360" y="246" font-size="12" fill="#c2410c" text-anchor="middle">⑤ もう一度送られるのは GET /messages だけ</text>
<line x1="590" y1="290" x2="134" y2="290" stroke="#c2410c" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-06-0102)"/>
<text x="360" y="282" font-size="12" fill="#c2410c" text-anchor="middle">⑥ 200 OK（一覧を見るだけ。伝言は増えない）</text>
<text x="360" y="316" font-size="11" fill="#475569" text-anchor="middle">②で 200 の一覧を返していたら、再読み込みで ① の POST が送り直され、伝言が二重になる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-?: PRG（POST → Redirect → GET）の流れ。POST の返事を 303 にして GET の画面へ移すので、再読み込みで送り直されるのは見るだけの GET になる</div>
</div>

- GET は「見るだけ」で、何回送っても DB を変えない（**安全**な）メソッドとして扱います。ブラウザは、リンクの先を前もって読み込んだり、再読み込みで送り直したりするので、GET で何かを変えると、利用者が知らないうちに変わってしまいます。

##### 解説4: HEAD は、ヘッダーだけを返す

- `curl.exe -I`（HEAD）には、`200 OK` と `Content-Length: 25` だけが返り、本文はありませんでした。`respond` の中の `if request.method != "HEAD"` で、本文を付けないようにしているからです。HEAD は「本文の大きさや種類だけを、先に知りたい」ときに使います。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: Content-Length に文字数を入れた

例1-1の `reply.len()`（バイト数）を、`reply.chars().count()`（文字数）にしてしまった場合です（`src/bin/u01_len_bug.rs`）。

```rust
@@FILE:http_lab/src/bin/u01_len_bug.rs@@
```

```powershell
cargo run --bin u01_len_bug
```

別の窓から:

```powershell
curl.exe -i http://127.0.0.1:3000/
```

```text
@@OUT:u01_len_bug_curl@@
```

- **何が起きたか:** 本文が「こんにち」で切れました。エラーは、どこにも出ていません。`Content-Length: 12` は、`こんにちは、HTTP！\n` の**文字数**（12 文字）です。ところが、日本語の1文字は UTF-8 で3バイトなので、本当の本文は 26 バイトあります。curl は、`Content-Length` を信じて 12 バイトだけを本文として読み、残りを捨てました。12 バイトは、日本語4文字ぶん（こ・ん・に・ち）です。
- **英語だけで試すと気づかない:** 英数字は1文字1バイトなので、文字数とバイト数が同じになり、この間違いは見えません。日本語のアプリでは、日本語で試すことが大切です（第2巻 UNIT 05、第9巻 UNIT 03）。
- **直し方:** `Content-Length` には `reply.len()`（`String` の `len()` はバイト数）を入れます。UNIT 02 から使う Topcoat は、この長さを自動で正しく計算してくれます。

#### エラー2: 誰も待っていないポートに送った（curl: (7)）

サーバーを止めたまま、`curl.exe` で要求を送った場合です。

```powershell
curl.exe -i http://127.0.0.1:3000/
```

```text
@@OUT:u01_refused@@
```

- **何が起きたか:** 「127.0.0.1 の 3000 番に、約2秒試しても接続できませんでした」というエラーです。`(7)` は curl のエラーの番号で、「接続できなかった」を表します。HTTP の状態コード（404 など）は出ていません。状態コードは、サーバーが応答を返したときにだけ届くものだからです。
- **確かめること:** サーバーの窓で、`http://127.0.0.1:3000 で待っています` が表示されているか、ポートの番号が同じかを確かめます。サーバーが起動の途中でエラーになって止まっていることもあります。
- **404 との違い:** 404 は「サーバーは動いていて、そのページが無い」、curl の `(7)` は「サーバーにたどり着けない」です。どちらの段階で失敗しているかを、最初に見分けます。

#### エラー3: 同じポートで2つ起動した（os error 10048）

例1-1のサーバーを止めずに、別の窓から例1-2のサーバーを起動すると、`TcpListener::bind` が失敗します。Windows では `os error 10048`（そのアドレスとポートは、もう使われています）、Linux では `os error 98` のエラーになります。先に動いているサーバーを Ctrl+C で止めてから、起動し直します（RustNote で同じエラーを見た例は、第12巻 UNIT 01 のエラー2にあります）。

### RustNote ではここで使う

RustNote は、例1-2のような「メソッドとパスで返事を分ける」処理を、Topcoat に任せています。RustNote の `src/routes.rs` の先頭のコメントには、ファイルと URL の対応の表があります（抜き出しています）。

```rust
//! | ファイル | URL |
//! |---|---|
//! | routes.rs                 | GET /（トップ）、全ページ共通のレイアウト、404 の受け皿 |
//! | routes/health.rs          | GET /health |
//! | routes/signup.rs          | GET /signup・POST /signup |
//! | routes/login.rs           | GET /login・POST /login |
//! | routes/logout.rs          | POST /logout |
//! | routes/notes.rs           | GET /notes（?q= 検索・?page= ページ送り）・POST /notes |
```

- 例1-2の `match (method, path)` の1行ずつが、RustNote では1つの関数（`#[page]` や `#[page(POST)]` を付けたもの）になります。無いページの 404、使えないメソッドの 405 は、Topcoat が自動で返します。
- RustNote のフォームの POST は、成功すると必ず 303 で GET の画面へ移します（PRG）。たとえば、ノートの作成（`POST /notes`）は、303 で、作ったノートの詳細（`GET /notes/{id}`）へ移ります。

RustNote が返す状態コードの決まりは、`src/error.rs` の先頭のコメントにまとめてあります（抜き出しています）。

```rust
//! | AppError | ステータス | 起きる場面 |
//! |---|---|---|
//! | `InvalidTitle` など入力の検証エラー | 400 | 空のタイトル、形式の違うメール |
//! | `Unauthorized` | 401 | ログインしていない（通常はログイン画面へのリダイレクトで扱う） |
//! | `NotFound` | 404 | 存在しないノート、**他人のノート**（あること自体を隠す） |
//! | `EmailTaken` / `Conflict` | 409 | 登録済みのメール、ほかの画面で先に更新された |
//! | `Internal` | 500 | DB の故障など。画面には詳細を出さず、ログに原因を書く |
```

- 例1-2では出てこなかった **400**（入力の形が違う）・**401**（ログインが必要）・**409**（ほかの更新とぶつかった）・**500**（サーバーの中の失敗）も使っています。他人のノートを 403 ではなく 404 にしている理由は、第8巻 UNIT 01 で扱いました。

### 練習問題

**問1:** 例1-2のサーバーに、`GET /health` を足してください。`200 OK` で、本文は `{"status":"ok"}`、`Content-Type` は `application/json`（JSON の形の本文です）にします。足したら、`GET /health` と `POST /health` を送って、それぞれの応答を確かめてください。`POST /health` の応答は、HTTP の決まりとして正しいでしょうか。

**問2:** 「伝言を `GET /messages?add=hello` で足せる」ように作ったとします。この作りには、どんな困ったことが起きるでしょうか。解説3の内容をもとに、2つ挙げてください。

**問3:** 例1-1で取った `curl.exe -v` の表示（下）の中から、curl が送った要求の部分と、サーバーが返した応答の部分を見分けてください。それぞれ、どの記号で始まっていますか。また、`* ` で始まる行は何を表しているでしょうか。

```text
@@OUT:u01_raw_verbose@@
```

### 解答と解説

**問1の解答:**

`match` の、`// 見るだけの操作（GET と HEAD）` の行の前に、次の腕を足します（足したものを `src/bin/u01_q1.rs` として保存しました）。`respond` は `Content-Type` を `text/plain` に決めているので、この腕では、応答の文字を自分で組み立てます。

```rust
@@FILE:q1_branch.rs@@
```

```powershell
cargo run --bin u01_q1
```

```text
@@OUT:u01_q1_curl@@
```

- `GET /health` には、`Content-Type: application/json` と `{"status":"ok"}` が返りました。`Content-Length: 15` は、`{"status":"ok"}` のバイト数です。`r#"..."#` は、中に `"` を含む文字列を、`\"` と書かずにそのまま書ける Rust の書き方（生の文字列リテラル）です。
- `POST /health` には **404** が返りました。`match` のどの腕にも当たらず、最後の `_` に落ちたからです。HTTP の決まりとしては、`/health` というページはあるので、**405 Method Not Allowed** と `Allow: GET` を返すのが正確です。例1-2の `/` と同じように、`(_, "/health")` の腕を足せば直せます。こうした決まりを、ページごとに自分で書き続けるのは大変です。Topcoat は、ページに付けたメソッドから、405 と `Allow` を自動で返してくれます（第9巻 UNIT 02 で、RustNote の 405 を確かめました）。
- RustNote の `GET /health` も、同じ `{"status":"ok"}` を返します。公開したあと、外から「動いているか」を確かめるのに使います（第12巻 UNIT 01）。

**問2の解答:**

1. **再読み込みで、伝言が増える:** `GET /messages?add=hello` の画面で再読み込みを押すと、ブラウザは同じ GET をもう一度送るので、伝言がもう1件増えます。GET は「何回送っても変わらない」ことが前提なので、ブラウザは確かめずに送り直します。
2. **リンクを踏んだだけで、伝言が増える:** GET は、リンク（`<a href="...">`）や画像（`<img src="...">`）でも送られます。ほかのサイトに `<img src="http://127.0.0.1:3000/messages?add=spam">` と書かれていたら、そのページを開いただけで、利用者のブラウザが伝言を足してしまいます。ブラウザが「先読み」（リンクの先を前もって読み込むこと）で送ることもあります。

- だから、何かを**変える**操作は POST（や PUT・DELETE）にして、GET は**見るだけ**にします。POST にしても、ほかのサイトからフォームで送らせる攻撃（CSRF）は残るので、その守りを足します（第8巻 UNIT 01、この巻の UNIT 10（準備中））。

**問3の解答:**

- **`> ` で始まる行**が、curl が送った要求です（`> GET /hello?name=ada HTTP/1.1` から、空の `> ` の行まで）。**`< ` で始まる行**が、サーバーが返した応答の1行目とヘッダーです（`< HTTP/1.1 200 OK` から、空の `< ` の行まで）。矢印の向き（`>` は外へ、`<` は内へ）と覚えます。
- **`* ` で始まる行**は、curl が自分の動きを説明しているメモです。`Trying 127.0.0.1:3000...`（つなごうとしています）、`Established connection`（つながりました）、`Request completely sent off`（要求を送り終えました）、`Connection #0 to host 127.0.0.1:3000 left intact`（接続は閉じずに残しました）のように、HTTP の文字ではなく、その外側で起きたことを表します。
- `{ [26 bytes data]` は、本文を 26 バイト受け取ったという印です。`-v` と一緒に `-o NUL`（本文は捨てる）を付けたので、本文そのものは表示されていません。

### まとめ

- HTTP の要求と応答は、どちらも「1行目・ヘッダー・空行・本文」の文字の並び。要求の1行目はメソッドとパス、応答の1行目は状態コード。本文の長さは `Content-Length` に**バイト数**で書く。
- 状態コードは、2xx（うまくいった）・3xx（別の場所へ。`Location` で行き先を教える）・4xx（頼み方が違う。404・403・405 は直し方が違う）・5xx（サーバーの失敗）。curl の `(7)` は、そもそもサーバーにたどり着けていない。
- GET は見るだけ、POST は変える。POST が成功したら 303 で GET の画面へ移す（PRG）と、再読み込みで二重に保存されない。

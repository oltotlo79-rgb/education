<div style="page-break-after: always; text-align: center; margin: 0; padding: 0;">
<img src="assets/cover.jpg" alt="表紙" style="width: 100%; max-height: 175mm; object-fit: cover; border-radius: 10px;">
<div style="margin-top: 14px; font-size: 11px; letter-spacing: 0.18em; color: #c2410c; font-weight: 700;">VOLUME 06 / RUST BACKEND MASTERY</div>
<div style="font-size: 26px; font-weight: 800; color: #0f172a; margin-top: 6px; line-height: 1.35;">Web 基礎と Topcoat 0.10 完全ガイド</div>
<div style="font-size: 13px; color: #475569; margin-top: 8px;">HTTP の要求と応答から、Topcoat のページ・ルーター・view!・フォーム・アセット・セッションまでを、動かして確かめる</div>
</div>

# 第6巻: Web 基礎と Topcoat 0.10 完全ガイド

## この巻で身に付けること

第5巻までで、Rust の文法・所有権・設計・async を学びました。この巻では、それを使って **Web のサーバー**を作ります。ブラウザから届いた「この画面を見せて」という頼みに、Rust のプログラムが HTML を作って返す仕組みです。

Web のサーバーは、いきなり道具（フレームワーク）から始めると、何が起きているのかが見えにくくなります。次のような場面で、困ることがよくあります。

- ブラウザでは何も表示されないのに、どこが悪いのか見当がつかない
- 「404」「405」「303」のような数字が返ってきたが、それが何を意味するのか分からない
- フォームを送ったあとに再読み込みしたら、同じデータがもう1件できてしまった
- 日本語の文字が、途中で切れて表示される

そこで、この巻の最初の単元では、道具を使わずに、**標準ライブラリだけ**で小さなサーバーを書き、HTTP の要求と応答が「ただの文字のやり取り」であることを、自分の目で確かめます。そのうえで、UNIT 02 から、RustNote でも使っている Web フレームワーク **Topcoat 0.10** の使い方を、1つずつ学びます。

### 到達目標

この巻を終えると、次のことができるようになります。

- HTTP の要求（メソッド・パス・ヘッダー・本文）と応答（状態コード・ヘッダー・本文）の形を説明し、`curl.exe -i` や `-v` の表示を読める
- 状態コード（200・303・403・404・405 など）と、GET・POST の使い分けを説明できる
- Topcoat 0.10 で、ページ（`#[page]`）・ルーター・レイアウト・URL の部品の読み取り・`view!` とコンポーネントを使った画面を作れる
- フォームを受け取り、成功したら 303 で GET の画面へ移す（PRG）形を作れる
- 共有の状態（`app_context`）・アセット（CSS の束）・エラー・クッキー・セッション・ほかのサイトからの送信の守りを、Topcoat の仕組みで扱える

### 学習地図

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 400" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-001" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="6" y="30" width="708" height="90" rx="12" fill="#eff6ff"/>
<rect x="6" y="140" width="708" height="90" rx="12" fill="#fff7ed"/>
<rect x="6" y="250" width="708" height="90" rx="12" fill="#ecfdf5"/>
<text x="360" y="20" font-size="13" fill="#475569" text-anchor="middle">上の段から順に進みます（各段の中は左から右へ）</text>
<text x="65" y="70" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">HTTP と</text>
<text x="65" y="90" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">最初の一歩</text>
<text x="65" y="180" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">画面と</text>
<text x="65" y="200" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">フォーム</text>
<text x="65" y="290" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">仕上げの</text>
<text x="65" y="310" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">仕組み</text>
<rect x="135" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="200" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 01</text>
<text x="200" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">HTTP の形</text>
<text x="200" y="99" font-size="12" fill="#475569" text-anchor="middle">要求・応答・状態</text>
<rect x="282" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="347" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 02</text>
<text x="347" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">最小のアプリ</text>
<text x="347" y="99" font-size="12" fill="#475569" text-anchor="middle">Topcoat 0.10</text>
<rect x="429" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="494" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 03</text>
<text x="494" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">ルーター</text>
<text x="494" y="99" font-size="12" fill="#475569" text-anchor="middle">page・layout</text>
<rect x="576" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="641" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 04</text>
<text x="641" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">URL の部品</text>
<text x="641" y="99" font-size="12" fill="#475569" text-anchor="middle">path・query・Cx</text>
<line x1="265" y1="75" x2="280" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-001)"/>
<line x1="412" y1="75" x2="427" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-001)"/>
<line x1="559" y1="75" x2="574" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-001)"/>
<path d="M641,110 V130 H200 V146" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-001)"/>
<rect x="135" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="200" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 05</text>
<text x="200" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">view!</text>
<text x="200" y="209" font-size="12" fill="#475569" text-anchor="middle">部品・エスケープ</text>
<rect x="282" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="347" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 06</text>
<text x="347" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">フォーム</text>
<text x="347" y="209" font-size="12" fill="#475569" text-anchor="middle">JSON・PRG</text>
<rect x="429" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="494" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 07</text>
<text x="494" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">共有の状態</text>
<text x="494" y="209" font-size="12" fill="#475569" text-anchor="middle">app_context</text>
<rect x="576" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="641" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 08</text>
<text x="641" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">アセット</text>
<text x="641" y="209" font-size="12" fill="#475569" text-anchor="middle">CSS の束</text>
<line x1="265" y1="185" x2="280" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-001)"/>
<line x1="412" y1="185" x2="427" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-001)"/>
<line x1="559" y1="185" x2="574" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-001)"/>
<path d="M641,220 V240 H200 V256" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-001)"/>
<rect x="135" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="200" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 09</text>
<text x="200" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">runtime</text>
<text x="200" y="319" font-size="12" fill="#475569" text-anchor="middle">signal・procedure</text>
<rect x="282" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="347" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 10</text>
<text x="347" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">エラー・守り</text>
<text x="347" y="319" font-size="12" fill="#475569" text-anchor="middle">cookie・session</text>
<line x1="265" y1="295" x2="280" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-001)"/>
<rect x="429" y="260" width="277" height="70" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-dasharray="5 4"/>
<text x="567" y="284" font-size="12" fill="#475569" text-anchor="middle">RustNote では</text>
<text x="567" y="304" font-size="12" fill="#475569" text-anchor="middle">src/routes/・src/views.rs・assets/</text>
<text x="360" y="372" font-size="13" fill="#475569" text-anchor="middle">各単元の流れ: たとえ話 → 図 → 最小コード → 実務の形 → よくあるエラー → 練習問題</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図6-1: 第6巻の学習地図。道具を使わずに HTTP の形を確かめてから、Topcoat で画面とフォームを作り、共有の状態・アセット・エラーと守りの仕組みで仕上げる</div>
</div>

| 単元 | テーマ | ひとことで言うと |
|---|---|---|
| UNIT 01 | HTTP の要求・応答・状態コード | 標準ライブラリだけのサーバーで、HTTP がただの文字だと確かめる |
| UNIT 02 | Topcoat 0.10 の最小のアプリ | ページを1つ作って起動する |
| UNIT 03 | ルーター・page・route・layout（準備中） | URL ごとに処理を分け、共通の外枠で包む |
| UNIT 04 | URL の部品（path・query）と Cx（準備中） | `/notes/{id}` や `?page=2` を型にして読む |
| UNIT 05 | view!・コンポーネント・HTML のエスケープ（準備中） | 画面を部品に分け、入力を安全に表示する |
| UNIT 06 | フォーム・JSON・PRG（準備中） | 送られてきた値を型にし、303 で移る |
| UNIT 07 | app_context・memoize（準備中） | DB などを全部の処理で共有し、同じ問い合わせを1回にする |
| UNIT 08 | アセット・CSS・束（準備中） | `asset!` と `topcoat asset bundle` |
| UNIT 09 | runtime・signal・procedure・shard（準備中） | ブラウザの中で動く部分と、その守り方 |
| UNIT 10 | エラー・クッキー・セッション・CSRF（準備中） | 400・404・リダイレクトと、ログインの仕組みの土台 |

> **お知らせ:** この版には UNIT 01〜02 を収録しています。UNIT 03〜10 は準備中で、次の更新から順に追加します。以前の版（Topcoat 0.5 の版）の PDF は、この版がそろうまで残してあります。Topcoat は 0.5 から 0.10 までに書き方が大きく変わっているので、以前の版のコードは、0.10 では動かないことがあります。

### 前提知識（先に読んでおく巻）

- **第2巻（Rust 基礎）:** `String` と `&str`、`match`、`Vec`、`for`。特に、日本語の文字列の「文字数」と「バイト数」の違い（第2巻 UNIT 05）。UNIT 01 のエラー1で、この違いが原因の事故を見ます。
- **第4巻（設計）:** `Result` と `?`、標準ライブラリの入出力（`std::io`）。
- **第5巻（async）:** UNIT 02 から使う `#[tokio::main]` と `async fn`。

### この巻の進め方

各単元では、次の順番で進めてください。

1. **予想する:** コマンドを実行する前に、どの状態コードが返り、何が表示されるかを予想します。
2. **そのまま動かす:** サーバーを起動し、もう1つの PowerShell の窓から `curl.exe` で要求を送ります。
3. **両方の窓を見る:** サーバーの窓（届いた要求）と、curl の窓（返ってきた応答）を、並べて見比べます。
4. **1か所だけ変える:** 「よくあるエラーと読み方」の節のとおりに書き換えて、何が変わるかを見ます。

> **ヒント:** この巻では、サーバーを動かす窓と、`curl.exe` を打つ窓の2つの PowerShell を使います。サーバーを止めるときは、サーバーの窓で Ctrl+C を押します。止め忘れたまま、別のサーバーを同じポートで起動しようとすると、エラーになります。

### 使うツールと版

この巻のコードは、すべて次の環境で実際にコンパイル・実行し、実行結果とエラーメッセージは実際の出力をそのまま載せています（パソコンのユーザー名などを含む部分は `...` に置き換えています）。

| 項目 | 版・内容 |
|---|---|
| OS | Windows 11（ARM64）。コマンドは PowerShell を主に、WSL / macOS / Linux は補足で示します |
| Rust | **1.98 以上**（本書シリーズの基準）。動作確認は rustc 1.99.0 / cargo 1.99.0（`aarch64-pc-windows-msvc`） |
| edition | 2024 |
| curl | 8.21.0（Windows に入っている `curl.exe`。PowerShell では `curl` だけだと別のコマンドになるので、必ず `curl.exe` と打ちます） |
| topcoat | 0.10.0（UNIT 02 から使います。UNIT 01 では使いません。2026-10 には 0.11.0 も出ていますが、この本は 0.10.0 に固定しています） |

### 練習用のプロジェクトを用意する

UNIT 01 では、外のクレートを1つも使いません。練習用のプロジェクトを1つ作り、`src/bin/` に、単元ごとの小さなサーバーを足していきます。

```powershell
cargo new http_lab
cd http_lab
# このフォルダでは Rust 1.99.0 を使う（第13巻 UNIT 01）
Set-Content rust-toolchain.toml "[toolchain]`nchannel = `"1.99.0`""
New-Item -ItemType Directory src\bin
```

`src/bin/u01_raw.rs` のように置いたファイルは、`cargo run --bin u01_raw` で、それぞれ別のプログラムとして動かせます。

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
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-2: HTTP の要求と応答の形。どちらも「1行目・ヘッダー・空行・本文」でできている。この単元の例で、実際に届いた・返した文字</div>
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
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-3: 状態コードの種類。最初の数字で、うまくいった（2）・別の場所へ（3）・頼み方が違う（4）・サーバーの失敗（5）が分かる</div>
</div>

### まず動かす最小コード

#### 例1-1: 届いた要求を、そのまま表示するサーバー

> **▼ このコードがやること（先に日本語で）:** `127.0.0.1:3000` で接続を待ちます。接続が来たら、要求の1行目とヘッダーを空行まで1行ずつ読み、`Content-Length` があれば、その長さだけ本文も読みます。読んだ要求をそのまま画面に表示し、「こんにちは、HTTP！」という文字を、HTTP の応答の形にして返します。外のクレートは使いません。

```rust
//! HTTP の要求と応答が「ただの文字」であることを確かめる、とても小さなサーバー。
//! 届いた要求をそのまま表示し、決まった文字を返す（標準ライブラリだけで書く）。
//! 実行: cargo run --bin u01_raw

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

/// 1つの接続を処理する: 要求を読んで表示し、応答を返す
fn handle(mut stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(&stream);

    // 1行目（要求行）とヘッダーを、空行が来るまで1行ずつ読む
    let mut head = Vec::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let line = line.trim_end().to_string();
        if line.is_empty() {
            break;
        }
        head.push(line);
    }

    // Content-Length のヘッダーがあれば、その長さ（バイト数）だけ本文を読む
    let mut length = 0;
    for line in &head {
        if let Some((name, value)) = line.split_once(':')
            && name.eq_ignore_ascii_case("content-length")
        {
            length = value.trim().parse().unwrap_or(0);
        }
    }
    let mut body = vec![0; length];
    reader.read_exact(&mut body)?;

    println!("--- 届いた要求 ---");
    for line in &head {
        println!("{line}");
    }
    if !body.is_empty() {
        println!("（空行）");
        println!("{}", String::from_utf8_lossy(&body));
    }

    // 応答を「文字列」として組み立てて送り返す。Content-Length は本文のバイト数
    let reply = "こんにちは、HTTP！\n";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
        reply.len(),
        reply
    );
    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    println!("http://127.0.0.1:3000 で待っています（Ctrl+C で終了）");
    // 接続が来るたびに、1つずつ処理する
    for stream in listener.incoming() {
        handle(stream?)?;
    }
    Ok(())
}
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
HTTP/1.1 200 OK
Content-Type: text/plain; charset=utf-8
Content-Length: 26

こんにちは、HTTP！
```

```text
HTTP/1.1 200 OK
Content-Type: text/plain; charset=utf-8
Content-Length: 26

こんにちは、HTTP！
```

**▼ 実行結果（サーバーの窓。3つ目の要求は、問3の `curl.exe -v` で送ったものです）**

```text
http://127.0.0.1:3000 で待っています（Ctrl+C で終了）
--- 届いた要求 ---
GET /hello?name=ada HTTP/1.1
Host: 127.0.0.1:3000
User-Agent: curl/8.21.0
Accept: */*
--- 届いた要求 ---
POST /notes HTTP/1.1
Host: 127.0.0.1:3000
User-Agent: curl/8.21.0
Accept: */*
Content-Length: 24
Content-Type: application/x-www-form-urlencoded
（空行）
title=shopping&body=milk
--- 届いた要求 ---
GET /hello?name=ada HTTP/1.1
Host: 127.0.0.1:3000
User-Agent: curl/8.21.0
Accept: */*
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
//! 要求のメソッドとパスを見て、いろいろな状態コードを返すサーバー（標準ライブラリだけで書く）。
//! 実行: cargo run --bin u01_status

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

/// 届いた要求（要求行・本文）
struct Request {
    method: String,
    path: String,
    body: String,
}

/// 要求を読む: 1行目（要求行）・ヘッダー・本文
fn read_request(stream: &TcpStream) -> std::io::Result<Request> {
    let mut reader = BufReader::new(stream);
    let mut first = String::new();
    reader.read_line(&mut first)?;
    // 「GET /messages HTTP/1.1」を空白で区切る
    let mut parts = first.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("").to_string();

    let mut length = 0;
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(':')
            && name.eq_ignore_ascii_case("content-length")
        {
            length = value.trim().parse().unwrap_or(0);
        }
    }
    let mut body = vec![0; length];
    reader.read_exact(&mut body)?;
    let body = String::from_utf8_lossy(&body).to_string();
    Ok(Request { method, path, body })
}

/// 応答を組み立てて送る。HEAD のときは、ヘッダーだけを送る
fn respond(
    stream: &mut TcpStream,
    request: &Request,
    status: &str,
    headers: &[(&str, &str)],
    body: &str,
) -> std::io::Result<()> {
    let mut text = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\n",
        body.len()
    );
    for (name, value) in headers {
        text.push_str(&format!("{name}: {value}\r\n"));
    }
    text.push_str("\r\n");
    if request.method != "HEAD" {
        text.push_str(body);
    }
    stream.write_all(text.as_bytes())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    println!("http://127.0.0.1:3000 で待っています（Ctrl+C で終了）");
    // POST /messages で受け取った伝言を、ここに貯める
    let mut messages: Vec<String> = Vec::new();

    for stream in listener.incoming() {
        let mut stream = stream?;
        let request = read_request(&stream)?;
        println!("{} {}", request.method, request.path);

        match (request.method.as_str(), request.path.as_str()) {
            // 見るだけの操作（GET と HEAD）
            ("GET" | "HEAD", "/") => {
                respond(&mut stream, &request, "200 OK", &[], "トップページです\n")?
            }
            // / は GET と HEAD でしか使えない
            (_, "/") => respond(
                &mut stream,
                &request,
                "405 Method Not Allowed",
                &[("Allow", "GET, HEAD")],
                "その方法は使えません\n",
            )?,
            ("GET", "/admin") => respond(
                &mut stream,
                &request,
                "403 Forbidden",
                &[],
                "ここは見られません\n",
            )?,
            // 引っ越したページ: 新しい場所を Location で教える
            ("GET", "/old-page") => respond(
                &mut stream,
                &request,
                "301 Moved Permanently",
                &[("Location", "/")],
                "",
            )?,
            // 伝言を受け取ったら、303 で「GET /messages を見てください」と返す
            ("POST", "/messages") => {
                let text = request.body.split_once('=').map(|(_, value)| value);
                messages.push(text.unwrap_or("").to_string());
                respond(
                    &mut stream,
                    &request,
                    "303 See Other",
                    &[("Location", "/messages")],
                    "",
                )?
            }
            ("GET", "/messages") => {
                let list = format!("伝言は {} 件: {:?}\n", messages.len(), messages);
                respond(&mut stream, &request, "200 OK", &[], &list)?
            }
            _ => respond(
                &mut stream,
                &request,
                "404 Not Found",
                &[],
                "ページが見つかりません\n",
            )?,
        }
    }
    Ok(())
}
```

`src/bin/u01_status.rs` として保存し、例1-1のサーバーを Ctrl+C で止めてから起動します。2つ目の窓から、いろいろな要求を送ります。`-X DELETE` はメソッドを DELETE にする指定、`-I` は HEAD（ヘッダーだけをください）で送る指定、`-L` は 3xx の応答が来たら `Location` の場所へ自動でたどる指定です。

```powershell
cargo run --bin u01_status
```

**▼ 実行結果（curl の窓。`###` の行は、送ったコマンドです）**

```text
### curl.exe -i http://127.0.0.1:3000/
HTTP/1.1 200 OK
Content-Type: text/plain; charset=utf-8
Content-Length: 25

トップページです

### curl.exe -i http://127.0.0.1:3000/nothing
HTTP/1.1 404 Not Found
Content-Type: text/plain; charset=utf-8
Content-Length: 34

ページが見つかりません

### curl.exe -i http://127.0.0.1:3000/admin
HTTP/1.1 403 Forbidden
Content-Type: text/plain; charset=utf-8
Content-Length: 28

ここは見られません

### curl.exe -i http://127.0.0.1:3000/old-page
HTTP/1.1 301 Moved Permanently
Content-Type: text/plain; charset=utf-8
Content-Length: 0
Location: /


### curl.exe -i -X DELETE http://127.0.0.1:3000/
HTTP/1.1 405 Method Not Allowed
Content-Type: text/plain; charset=utf-8
Content-Length: 31
Allow: GET, HEAD

その方法は使えません

### curl.exe -I http://127.0.0.1:3000/
HTTP/1.1 200 OK
Content-Type: text/plain; charset=utf-8
Content-Length: 25

### curl.exe -i -d message=hello http://127.0.0.1:3000/messages
HTTP/1.1 303 See Other
Content-Type: text/plain; charset=utf-8
Content-Length: 0
Location: /messages


### curl.exe -i -L -d message=world http://127.0.0.1:3000/messages
HTTP/1.1 303 See Other
Content-Type: text/plain; charset=utf-8
Content-Length: 0
Location: /messages

HTTP/1.1 200 OK
Content-Type: text/plain; charset=utf-8
Content-Length: 36

伝言は 2 件: ["hello", "world"]
```

**▼ 実行結果（サーバーの窓）**

```text
http://127.0.0.1:3000 で待っています（Ctrl+C で終了）
GET /
GET /nothing
GET /admin
GET /old-page
DELETE /
HEAD /
POST /messages
POST /messages
GET /messages
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
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-4: PRG（POST → Redirect → GET）の流れ。POST の返事を 303 にして GET の画面へ移すので、再読み込みで送り直されるのは見るだけの GET になる</div>
</div>

- GET は「見るだけ」で、何回送っても DB を変えない（**安全**な）メソッドとして扱います。ブラウザは、リンクの先を前もって読み込んだり、再読み込みで送り直したりするので、GET で何かを変えると、利用者が知らないうちに変わってしまいます。

##### 解説4: HEAD は、ヘッダーだけを返す

- `curl.exe -I`（HEAD）には、`200 OK` と `Content-Length: 25` だけが返り、本文はありませんでした。`respond` の中の `if request.method != "HEAD"` で、本文を付けないようにしているからです。HEAD は「本文の大きさや種類だけを、先に知りたい」ときに使います。

### よくあるエラーと読み方

#### エラー1（エラーが出ない間違い）: Content-Length に文字数を入れた

例1-1の `reply.len()`（バイト数）を、`reply.chars().count()`（文字数）にしてしまった場合です（`src/bin/u01_len_bug.rs`）。

```rust
//! エラー1の例: Content-Length に、バイト数ではなく文字数を入れてしまったサーバー。
//! 実行: cargo run --bin u01_len_bug

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    println!("http://127.0.0.1:3000 で待っています（Ctrl+C で終了）");
    for stream in listener.incoming() {
        let mut stream = stream?;
        // 要求の1行目とヘッダーを、空行まで読み飛ばす
        let mut reader = BufReader::new(&stream);
        loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            if line.trim_end().is_empty() {
                break;
            }
        }

        let reply = "こんにちは、HTTP！\n";
        // 間違い: chars().count() は「文字数」。Content-Length に入れるのは「バイト数」
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            reply.chars().count(),
            reply
        );
        stream.write_all(response.as_bytes())?;
    }
    Ok(())
}
```

```powershell
cargo run --bin u01_len_bug
```

別の窓から:

```powershell
curl.exe -i http://127.0.0.1:3000/
```

```text
HTTP/1.1 200 OK
Content-Type: text/plain; charset=utf-8
Content-Length: 12

こんにち
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
curl: (7) Failed to connect to 127.0.0.1:3000 after 2029 ms: Could not connect to server
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
*   Trying 127.0.0.1:3000...
* Established connection to 127.0.0.1 (127.0.0.1 port 3000) from 127.0.0.1 port 55388
* using HTTP/1.x
> GET /hello?name=ada HTTP/1.1
> Host: 127.0.0.1:3000
> User-Agent: curl/8.21.0
> Accept: */*
> 
* Request completely sent off
< HTTP/1.1 200 OK
< Content-Type: text/plain; charset=utf-8
< Content-Length: 26
< 
{ [26 bytes data]
* Connection #0 to host 127.0.0.1:3000 left intact
```

### 解答と解説

**問1の解答:**

`match` の、`// 見るだけの操作（GET と HEAD）` の行の前に、次の腕を足します（足したものを `src/bin/u01_q1.rs` として保存しました）。`respond` は `Content-Type` を `text/plain` に決めているので、この腕では、応答の文字を自分で組み立てます。

```rust
            // 問1: 動いているかを外から確かめる URL。JSON で返すので Content-Type も変える
            ("GET", "/health") => {
                let body = r#"{"status":"ok"}"#;
                let text = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
                    body.len()
                );
                stream.write_all(text.as_bytes())?
            }
```

```powershell
cargo run --bin u01_q1
```

```text
### curl.exe -i http://127.0.0.1:3000/health
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 15

{"status":"ok"}

### curl.exe -i -X POST http://127.0.0.1:3000/health
HTTP/1.1 404 Not Found
Content-Type: text/plain; charset=utf-8
Content-Length: 34

ページが見つかりません
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

## UNIT 02: Topcoat 0.10 の最小のアプリ

### この単元のゴール

- `cargo add` で Topcoat 0.10.0 と Tokio を足し、ページが1つだけの Web アプリを作って起動できる
- `#[page("/")]`・`Router::builder().discover().build()`・`topcoat::start` の3つが、それぞれ何をしているかを説明できる
- UNIT 01 で自分で書いた処理（`Content-Length`・404・405・HEAD）を、Topcoat が代わりにしてくれることを、`curl.exe` の表示で確かめられる
- `HOST` と `PORT` の環境変数で待ち受ける場所を変え、`.await` の書き忘れ・`use` の書き忘れ・ポートの指定の間違いのエラーを読める

### たとえ話でつかむ

UNIT 01 では、お店の窓口を1人で切り盛りしました。お客さんの申込書を1行ずつ読み、「その窓口ではできません」「その書類はありません」を自分で書き、返事の紙の枚数（`Content-Length`）まで自分で数えました。

UNIT 02 では、**受付の係を雇います**。

- 受付の係は、申込書を読み、どの担当に回すかを決め、担当がいなければ「その書類はありません」（404）、担当はいても方法が違えば「その方法は使えません」（405）と、決まった形で返事をしてくれます。返事の紙の枚数も、係が数えます。
- あなたがすることは、**担当者の名札**を書くことだけです。「`/` に見に来た人には、この人が答えます」と書いた名札を、担当者に付けておきます。
- 開店のとき、受付の係は、名札を付けた担当者を**全員集めて**、名簿を作ります。
- お店の**住所と窓口の番号**（`HOST` と `PORT`）は、開店の前に掲示板（環境変数）に書いておけば、係がそれを読みます。

| お店 | Topcoat のアプリ |
|---|---|
| 受付の係 | **Topcoat**（ルーターと、待ち受けの仕組み） |
| 担当者に付ける名札 | `#[page("/")]`（この関数は `GET /` に答える、という印） |
| 名札の付いた担当者を集めて名簿を作る | `Router::builder().discover().build()` |
| 開店して、窓口を開ける | `topcoat::start(router).await` |
| 掲示板に書いた住所と窓口の番号 | 環境変数 `HOST` と `PORT`（無ければ `127.0.0.1` と `3000`） |
| 決まった形の断り方 | 無い URL は 404、使えないメソッドは 405 と `allow` |

> **用語:** Web フレームワーク（ウェブフレームワーク：要求を読み、URL ごとの処理に振り分け、応答を返す、といった決まった仕事を引き受けてくれる道具。Topcoat もその1つ）

<!-- -->

> **用語:** ルーター（router：要求の URL とメソッドを見て、どの関数に処理させるかを決める仕組み）

### 図で理解する

最小のアプリの部品と、要求が届いてから応答を返すまでの流れです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0201" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="200" height="70" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="120" y="46" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">#[page("/")]</text>
<text x="120" y="66" font-size="11" fill="#0f172a" text-anchor="middle">async fn home()</text>
<text x="120" y="82" font-size="11" fill="#475569" text-anchor="middle">→ view! で HTML</text>
<rect x="260" y="20" width="200" height="70" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="46" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">Router::builder()</text>
<text x="360" y="66" font-size="11" fill="#0f172a" text-anchor="middle">.discover()</text>
<text x="360" y="82" font-size="11" fill="#0f172a" text-anchor="middle">.build()</text>
<rect x="500" y="20" width="200" height="70" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="600" y="46" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">topcoat::start(router)</text>
<text x="600" y="66" font-size="11" fill="#0f172a" text-anchor="middle">HOST・PORT を読む</text>
<text x="600" y="82" font-size="11" fill="#0f172a" text-anchor="middle">待ち受けを始める</text>
<line x1="220" y1="55" x2="256" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0201)"/>
<line x1="460" y1="55" x2="496" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0201)"/>
<text x="238" y="44" font-size="10" fill="#475569" text-anchor="middle">集める</text>
<text x="478" y="44" font-size="10" fill="#475569" text-anchor="middle">渡す</text>
<rect x="20" y="150" width="680" height="120" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2" stroke-dasharray="6 4"/>
<text x="360" y="174" font-size="12" font-weight="700" fill="#475569" text-anchor="middle">要求が届いたら（Topcoat が引き受ける）</text>
<rect x="40" y="190" width="150" height="60" rx="8" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="115" y="216" font-size="12" fill="#0f172a" text-anchor="middle">GET /</text>
<text x="115" y="236" font-size="11" fill="#047857" text-anchor="middle">→ home を呼ぶ → 200</text>
<rect x="210" y="190" width="150" height="60" rx="8" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="285" y="216" font-size="12" fill="#0f172a" text-anchor="middle">GET /nothing</text>
<text x="285" y="236" font-size="11" fill="#c2410c" text-anchor="middle">→ 404 not found</text>
<rect x="380" y="190" width="150" height="60" rx="8" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="455" y="216" font-size="12" fill="#0f172a" text-anchor="middle">POST /</text>
<text x="455" y="236" font-size="11" fill="#c2410c" text-anchor="middle">→ 405・allow: GET, HEAD</text>
<rect x="550" y="190" width="130" height="60" rx="8" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="615" y="216" font-size="12" fill="#0f172a" text-anchor="middle">HEAD /</text>
<text x="615" y="236" font-size="11" fill="#047857" text-anchor="middle">→ ヘッダーだけ</text>
<line x1="600" y1="90" x2="600" y2="146" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0201)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-5: 最小のアプリの部品。名札（#[page]）を付けた関数を discover で集めてルーターを作り、start に渡す。要求の振り分けと、404・405・HEAD の返事は Topcoat がする</div>
</div>

- 自分で書くのは、`#[page("/")]` を付けた関数 `home` と、`main` の2行だけです。
- `discover()` は、プログラムの中の `#[page]` を付けた関数を、**全部自動で集めます**。ページを増やすときは、`#[page("/about")]` を付けた関数を足すだけで、`main` を書き換える必要はありません（問1）。
- 無い URL の 404、使えないメソッドの 405 と `allow`、HEAD のときに本文を付けないこと、`Content-Length` の計算は、どれも Topcoat がします。UNIT 01 で自分で書いた処理です。

### まず動かす最小コード

#### 例2-1: プロジェクトを作り、Topcoat と Tokio を足す

> **▼ このコマンドがやること（先に日本語で）:** 新しいプロジェクトを作り、Rust の版を 1.99.0 に固定してから、`cargo add` で Topcoat（版を 0.10.0 に固定）と Tokio（非同期のランタイム）を足します。

```powershell
cargo new web_lab
cd web_lab
Set-Content rust-toolchain.toml "[toolchain]`nchannel = `"1.99.0`""
cargo add topcoat@=0.10.0
cargo add tokio@=1.53.3 --features rt-multi-thread,macros
```

**▼ 実行結果**（`cargo add topcoat@=0.10.0` の表示）

```text
    Updating crates.io index
      Adding topcoat =0.10.0 to dependencies
             Features:
             + asset
             + compression
             + cookie
             + discover
             + font
             + icon
             + router
             + runtime
             + serve
             + session
             + view
             - alpine-ajax
             - anyhow
             - datastar
             - font-fontsource
             - fs
             - full
             - htmx
             - icon-iconify
             - mail
             - mail-smtp
             - multipart
             - sitemap
             - sse
             - tailwind
             - tower
             - ui
             - websocket
    Updating crates.io index
     Locking 214 packages to highest Rust 1.99.0 compatible versions
      Adding generic-array v0.14.7 (available: v0.14.9)
      Adding topcoat v0.10.0 (available: v0.11.0)
```

- `+` の付いた features（`asset`・`router`・`view` など）が、既定で有効になる機能です。`-` の付いたもの（`tailwind`・`websocket` など）は、必要なときに自分で足します。
- 最後の `Adding topcoat v0.10.0 (available: v0.11.0)` は、「0.10.0 を入れましたが、0.11.0 も出ています」というお知らせです。この本は、すべての巻で **0.10.0 に固定**しています。`@=0.10.0` の `=` を付けているので、勝手に 0.11.0 にはなりません（第13巻 UNIT 01）。Topcoat は、版が上がると書き方が変わることがあるので、版を上げるときは、自分で決めて、まとめて直します。

できあがった `Cargo.toml` です（この単元の最後に、もう1つの機能を足したあとの形です。例2-3で説明します）。

```toml
[package]
name = "web_lab"
version = "0.1.0"
edition = "2024"

[dependencies]
tokio = { version = "=1.53.3", features = ["rt-multi-thread", "macros", "net"] }
topcoat = "=0.10.0"
```

#### 例2-2: ページが1つだけのアプリ

> **▼ このコードがやること（先に日本語で）:** `GET /` に答える関数 `home` に `#[page("/")]` の名札を付け、`view!` で HTML を作って返します。`main` では、名札の付いた関数を `discover()` で集めてルーターを作り、`topcoat::start` に渡して、待ち受けを始めます。

```rust
//! Topcoat 0.10 の、いちばん小さな Web アプリ。GET / に HTML を返す。
//! 実行: cargo run

use topcoat::{
    Result,
    router::{Router, RouterBuilderDiscoverExt, page},
    view::{View, view},
};

// URL「/」に GET が来たら、この関数が HTML を作って返す
#[page("/")]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"はじめての Topcoat"</title>
            </head>
            <body>
                <h1>"こんにちは、Topcoat！"</h1>
                <p>"この HTML は Rust のプログラムが作りました。"</p>
            </body>
        </html>
    })
}

#[tokio::main]
async fn main() {
    // #[page] の付いた関数を、プログラム全体から集めて、ルーターを作る
    let router = Router::builder().discover().build();
    // 環境変数 HOST と PORT の場所（無ければ 127.0.0.1:3000）で待ち受ける
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
cargo run --bin web_lab
```

最初の1回は、Topcoat が使うクレートをコンパイルするので、少し時間がかかります（この PC では、`Compiling` の行が 164 行出て、約 31 秒でした）。

```text
   Compiling web_lab v0.1.0 (...\web_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 31.48s
```

**サーバーの窓には、何も表示されません。** `topcoat::start` は、起動しても何も表示しないからです。止まっているように見えますが、待ち受けています。2つ目の窓から、`curl.exe` で確かめます。

```powershell
curl.exe -i http://127.0.0.1:3000/
```

**▼ 実行結果**

```text
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-length: 228
date: Wed, 28 Oct 2026 21:52:33 GMT

<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><title>はじめての Topcoat</title></head><body><h1>こんにちは、Topcoat！</h1><p>この HTML は Rust のプログラムが作りました。</p></body></html>
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: #[page("/")] は、URL の名札

- `#[page("/")]` は、「この関数は `GET /` に答える」という印です。`#[page]` は、ふつうの関数を、ページの処理に変える**属性マクロ**です。何も指定しなければ GET に答え、HEAD にも自動で答えます。
- 関数の名前（`home`）は、URL には関係ありません。URL を決めるのは、`#[page(...)]` の中の文字列です。
- 戻り値の `Result<impl View>` は、「HTML になる何か（`View`）を返す。失敗することもある」という意味です。`view!` の中には、HTML をほぼそのまま書けます。文字は `"..."` で囲みます（UNIT 05（準備中））。

##### 解説2: discover は、名札を自動で集める

- `Router::builder().discover().build()` は、プログラムの中にある `#[page]` の関数を全部集めて、URL の名簿（ルーター）を作ります。
- `discover` は、`RouterBuilderDiscoverExt` という trait のメソッドです。だから、`use topcoat::router::RouterBuilderDiscoverExt` が要ります（エラー2）。

##### 解説3: Topcoat が付けてくれたヘッダー

- 応答のヘッダーは、`content-type: text/html; charset=utf-8`（HTML です。文字は UTF-8 です）・`content-length: 228`・`date`（応答を作った時刻、UTC）のほかに、`vary` が2行付いています（問3）。UNIT 01 では、`Content-Length` を自分で数えて、日本語で間違えました。Topcoat は、本文のバイト数を正しく数えてくれます。
- ヘッダーの名前が小文字（`content-type`）になっています。HTTP のヘッダーの名前は、大文字と小文字を区別しない決まりなので、`Content-Type` と同じ意味です。
- 本文の HTML は、`view!` に書いたとおりの1行になっています。字下げや改行は、`view!` が取り除いて、短くしています。

#### ▼ Topcoat が引き受けてくれる返事

UNIT 01 の例1-2で、自分で書いた返事を、Topcoat が返すか確かめます。

```powershell
curl.exe -i http://127.0.0.1:3000/nothing
curl.exe -i -X POST http://127.0.0.1:3000/
curl.exe -I http://127.0.0.1:3000/
```

**▼ 実行結果**

```text
### curl.exe -i http://127.0.0.1:3000/nothing
HTTP/1.1 404 Not Found
content-type: text/plain; charset=utf-8
content-length: 9
date: Wed, 28 Oct 2026 21:52:34 GMT

not found

### curl.exe -i -X POST http://127.0.0.1:3000/
HTTP/1.1 405 Method Not Allowed
allow: GET, HEAD
content-length: 0
date: Wed, 28 Oct 2026 21:52:34 GMT



### curl.exe -I http://127.0.0.1:3000/
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-length: 228
date: Wed, 28 Oct 2026 21:52:34 GMT
```

- 無い URL（`/nothing`）は `404 Not Found` で、本文は `not found` だけです。ページに名札を付けていない URL は、Topcoat が自動で 404 にします。自分の画面（ヘッダーやトップへのリンクの付いたもの）で 404 を出したいときは、`not_found!()` とレイアウトを使います（UNIT 03・10（準備中））。
- `POST /` は `405 Method Not Allowed` と `allow: GET, HEAD` です。`#[page]` は GET（と HEAD）だけに答えるので、Topcoat が「使えるのは GET と HEAD です」と教えてくれました。UNIT 01 の問1で「本当は 405 と Allow を返すべき」と書いたことを、Topcoat は何も書かなくてもしてくれます。
- `HEAD /`（`-I`）は、GET と同じヘッダー（`content-length: 228` も同じ）で、本文はありません。

### もう一歩: 実務に近い形

#### 例2-3: 待ち受けの場所を押さえてから、「待っています」と表示する

`topcoat::start` は、何も表示しません。「起動しました」と表示したくて、`start` の前に `println!` を書くと、困ったことが起きます。ポートを押さえる**前**に表示するので、ほかのプログラムがポートを使っていて起動に失敗したときも、「起動しました」と出てしまうのです（第12巻 UNIT 01 のエラー2で、RustNote が実際にそうなっているのを見ました）。

そこで、待ち受けの場所を**自分で先に押さえて**から表示し、押さえた場所を Topcoat に渡す書き方をします。`topcoat::start` の中身（Topcoat 0.10.0 の `serve.rs`）も、`TcpListener::bind` で押さえてから `serve` を呼ぶ形になっています。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0202" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="180" y="22" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">start の前に表示する（困る形）</text>
<rect x="20" y="36" width="320" height="40" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="180" y="61" font-size="12" fill="#0f172a" text-anchor="middle">① 「起動しました」と表示</text>
<rect x="20" y="96" width="320" height="56" rx="6" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="180" y="118" font-size="12" fill="#0f172a" text-anchor="middle">② topcoat::start の中で</text>
<text x="180" y="138" font-size="11" fill="#475569" text-anchor="middle">bind（押さえる）→ serve（さばく）</text>
<rect x="20" y="172" width="320" height="56" rx="6" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="180" y="194" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">bind に失敗しても、①はもう出ている</text>
<text x="180" y="214" font-size="11" fill="#475569" text-anchor="middle">表示と本当の状態が食い違う</text>
<line x1="180" y1="76" x2="180" y2="92" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0202)"/>
<line x1="180" y1="152" x2="180" y2="168" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0202)"/>
<text x="540" y="22" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">押さえてから表示する（例2-3）</text>
<rect x="380" y="36" width="320" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="540" y="61" font-size="12" fill="#0f172a" text-anchor="middle">① TcpListener::bind で押さえる</text>
<rect x="380" y="96" width="320" height="56" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="540" y="118" font-size="12" fill="#0f172a" text-anchor="middle">② 押さえられたときだけ</text>
<text x="540" y="138" font-size="11" fill="#475569" text-anchor="middle">「待っています」と表示</text>
<rect x="380" y="172" width="320" height="56" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="540" y="194" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">③ topcoat::serve(listener, router)</text>
<text x="540" y="214" font-size="11" fill="#475569" text-anchor="middle">失敗したら①で止まり、②は出ない</text>
<line x1="540" y1="76" x2="540" y2="92" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0202)"/>
<line x1="540" y1="152" x2="540" y2="168" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0202)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-6: 「起動しました」を表示する場所の違い。start は押さえるのとさばくのを中でまとめてするので、間に表示を入れられない。自分で押さえてから表示し、serve に渡せば、表示と状態が食い違わない</div>
</div>

> **▼ このコードがやること（先に日本語で）:** Tokio の `TcpListener::bind` で `127.0.0.1:3000` を押さえます。押さえられなかったら、そこでエラーとして終わります。押さえられたときだけ、「待っています」と表示してから、`topcoat::serve` に、押さえた場所とルーターを渡します。

```rust
//! 先に待ち受けの場所（127.0.0.1:3000）を押さえてから、「待っています」と表示する版。
//! 実行: cargo run --bin u02_serve

use tokio::net::TcpListener;
use topcoat::{
    Result,
    router::{Router, RouterBuilderDiscoverExt, page},
    view::{View, view},
};

#[page("/")]
async fn home() -> Result<impl View> {
    Ok(view! {
        <h1>"u02_serve で動いています"</h1>
    })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let router = Router::builder().discover().build();
    // 先にアドレスとポートを押さえる。ほかのプログラムが使っていたら、ここでエラーになって止まる
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    // 押さえられたときだけ、ここまで来る
    println!(
        "http://{} で待っています（Ctrl+C で止めます）",
        listener.local_addr()?
    );
    topcoat::serve(listener, router).await
}
```

このプログラムは `tokio::net::TcpListener` を使うので、Tokio の `net` の機能を、自分の `Cargo.toml` に足します。Topcoat がすでに `net` を有効にしているので、足さなくてもビルドは通りますが、**自分が使う機能は自分の `Cargo.toml` に書く**のが決まりです（第13巻 UNIT 01）。

```powershell
cargo add tokio@=1.53.3 --features rt-multi-thread,macros,net
```

`src/bin/u02_serve.rs` として保存し、1つ目の窓で起動します。続けて、2つ目の窓で、同じものをもう1つ起動します。

```powershell
cargo run --bin u02_serve
```

**▼ 実行結果（1つ目）**

```text
http://127.0.0.1:3000 で待っています（Ctrl+C で止めます）
```

**▼ 実行結果（2つ目。ポートがもう使われている）**

```text
Error: Os { code: 10048, kind: AddrInUse, message: "通常、各ソケット アドレスに対してプロトコル、ネットワーク アドレス、またはポートのどれか 1 つのみを使用できます。" }
```

- 1つ目は、ポートを押さえてから「待っています」と表示しました。2つ目は、`TcpListener::bind` で失敗したので、「待っています」を表示せずに、エラーで終わりました。表示と、本当の状態が食い違いません。
- `Error: Os { code: 10048, kind: AddrInUse, ... }` は、`main` が返した `Err` を、Rust が `{:?}` の形で表示したものです。`AddrInUse` は「アドレスはもう使われています」という意味です（UNIT 01 のエラー3と同じ原因です）。
- `local_addr()` は、実際に押さえた場所を返します。ポートに 0 を指定すると、空いている番号を OS が選んでくれるので、そのときに「何番になったか」を知るのにも使えます。

### よくあるエラーと読み方

#### エラー1: start の .await を書き忘れた（E0599）

`topcoat::start(router)` の次の行の `.await` を消してしまった場合です。

```rust
    topcoat::start(router)
        .expect("サーバーを起動できませんでした");
```

```powershell
cargo build
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error[E0599]: no method named `expect` found for opaque type `impl Future<Output = Result<(), std::io::Error>>` in the current scope
  --> src\main.rs:34:10
   |
33 | /     topcoat::start(router)
34 | |         .expect("サーバーを起動できませんでした");
   | |         -^^^^^^ method not found in `impl Future<Output = Result<(), std::io::Error>>`
   | |_________|
   |
   |
help: consider `await`ing on the `Future` and calling the method on its `Output`
   |
34 |         .await.expect("サーバーを起動できませんでした");
   |          ++++++

For more information about this error, try `rustc --explain E0599`.
error: could not compile `web_lab` (bin "web_lab") due to 1 previous error
```

- **何が起きたか:** 「`impl Future<Output = Result<(), std::io::Error>>` に、`expect` というメソッドはありません」というエラーです。`topcoat::start` は async の関数なので、呼んだだけでは、まだ動いていない「これから動く仕事」（`Future`）が返るだけです。`expect` は `Result` のメソッドなので、`Future` には使えません（第5巻 UNIT 01）。
- **直し方:** `help:` のとおり、`.await` を付けます。`.await` で仕事を最後まで動かすと、`Result` が手に入り、`expect` が使えます。

#### エラー2: discover の trait を use し忘れた（E0599）

`use topcoat::router::{Router, RouterBuilderDiscoverExt, page}` から、`RouterBuilderDiscoverExt` を消してしまった場合です。

```powershell
cargo build
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error[E0599]: no method named `discover` found for struct `RouterBuilder` in the current scope
  --> src\main.rs:31:36
   |
31 |     let router = Router::builder().discover().build();
   |                                    ^^^^^^^^
   |
  ::: ...\router.rs:14:8
   |
14 |     fn discover(self) -> Self;
   |        -------- the method is available for `RouterBuilder` here
   |
   = help: items from traits can only be used if the trait is in scope
help: trait `RouterBuilderDiscoverExt` which provides `discover` is implemented but not in scope; perhaps you want to import it
   |
 4 + use topcoat::router::RouterBuilderDiscoverExt;
   |
help: there is a method `discover_fonts` with a similar name
   |
31 |     let router = Router::builder().discover_fonts().build();
   |                                            ++++++

For more information about this error, try `rustc --explain E0599`.
error: could not compile `web_lab` (bin "web_lab") due to 1 previous error
```

- **何が起きたか:** 「`RouterBuilder` に、`discover` というメソッドが見つかりません」というエラーです。`discover` は、`RouterBuilder` 自身のメソッドではなく、`RouterBuilderDiscoverExt` という trait が足しているメソッドです。trait のメソッドは、その trait を `use` しているときだけ使えます（`help: items from traits can only be used if the trait is in scope`）。
- **直し方:** 2つ目の `help:` のとおり、`use topcoat::router::RouterBuilderDiscoverExt;` を足します。3つ目の `help:` の `discover_fonts`（名前が似ているメソッド）は、別の機能なので、案内に従わないでください。

#### エラー3: PORT に数ではない値を入れた

PowerShell で `$env:PORT = "abc"` としてから起動した場合です。

```powershell
$env:PORT = "abc"
cargo run --bin web_lab
```

```text

thread 'main' (154412) panicked at src\main.rs:35:10:
サーバーを起動できませんでした: Custom { kind: InvalidInput, error: "PORT must be a valid port number: invalid digit found in string" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- **何が起きたか:** `topcoat::start` が、環境変数 `PORT` を数として読めずに、`InvalidInput`（入力の値が不正）のエラーを返しました。`PORT must be a valid port number: invalid digit found in string` は「PORT は正しいポートの番号でなければなりません。文字列に数字ではない文字があります」です。`main` の `.expect(...)` が、このエラーを受けて panic しました。
- **直し方:** `Remove-Item Env:PORT` で環境変数を消す（既定の 3000 に戻る）か、`$env:PORT = "3001"` のように正しい番号を入れます。PowerShell で入れた環境変数は、その窓を閉じるまで残るので、ほかのプログラムにも影響することがあります。

#### エラー4: 実行ファイルが2つになって、cargo run が迷った

`src/bin/u02_serve.rs` を足したあとに、`--bin` を付けずに `cargo run` とした場合です。

```powershell
cargo run
```

```text
error: `cargo run` could not determine which binary to run. Use the `--bin` option to specify a binary, or the `default-run` manifest key.
available binaries: u02_serve, web_lab
```

- **何が起きたか:** 「`cargo run` は、どの実行ファイルを動かせばよいか決められません。`--bin` で指定するか、`default-run` を書いてください」というエラーです。`src/main.rs`（`web_lab`）と `src/bin/u02_serve.rs`（`u02_serve`）の2つの実行ファイルがあるからです。
- **直し方:** `cargo run --bin web_lab` のように指定します。いつも同じものを動かすなら、`Cargo.toml` の `[package]` に `default-run = "web_lab"` と書きます。RustNote の `Cargo.toml` に `default-run = "rustnote"` があるのは、このためです（第13巻 UNIT 01）。

### RustNote ではここで使う

RustNote の `src/main.rs` の最後の部分です（抜き出しています）。

```rust
    let router = routes::build_router(AppState::new(db), assets);

    eprintln!(
        "RustNote を起動しました: http://{}",
        config::listen_address()
    );
    topcoat::start(router)
        .await
        .context("サーバーを起動できませんでした（HOST / PORT を確認してください）")?;
```

- 例2-2と同じく、`topcoat::start(router).await` で待ち受けています。`HOST` と `PORT` を読むのも同じです。公開するとき（Fly.io）は `HOST=0.0.0.0`・`PORT=8080` にします（第12巻）。
- 例2-3で見たとおり、`eprintln!` で「起動しました」を、`start` の**前**に表示しています。ポートが使われていると、この表示のあとにエラーで止まります。例2-3の形（`TcpListener::bind` で押さえてから表示し、`topcoat::serve` に渡す）にすれば直せます（RustNote のコードは変えていません）。
- RustNote のルーターは、`discover()` ではなく、`module_router!()` で作っています（`src/routes.rs` の `build_router`）。ファイルの並びをそのまま URL の並びにする書き方で、UNIT 03（準備中）で扱います。

### 練習問題

**問1:** 例2-2のアプリに、2つ目のページ `GET /about` を足してください。トップページから `/about` へのリンクも付けます。足したら、`curl.exe -i http://127.0.0.1:3000/about` で確かめてください。

**問2:** `$env:PORT = "3001"` としてから例2-2のアプリを起動し、`http://127.0.0.1:3001/` と `http://127.0.0.1:3000/` に `curl.exe` で要求を送ってください。それぞれ、どうなるでしょうか。

**問3:** Topcoat の応答には、`vary: accept-encoding` というヘッダーが付いていました。`curl.exe -i --compressed http://127.0.0.1:3000/about` で、圧縮してほしいと伝えて要求を送り、ヘッダーがどう変わるかを確かめてください。`vary` は何を伝えているのでしょうか。

### 解答と解説

**問1の解答:**

`#[page("/about")]` を付けた関数を足すだけです（足したものを `src/bin/u02_q1.rs` として保存しました）。`main` は変えません。

```rust
//! 問1の解答: 例2-1のアプリに、2つ目のページ（/about）を足したもの。
//! 実行: cargo run --bin u02_q1

use topcoat::{
    Result,
    router::{Router, RouterBuilderDiscoverExt, page},
    view::{View, view},
};

#[page("/")]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"はじめての Topcoat"</title>
            </head>
            <body>
                <h1>"こんにちは、Topcoat！"</h1>
                <p><a href="/about">"このアプリについて"</a></p>
            </body>
        </html>
    })
}

// 2つ目のページ。#[page] に URL を書いた関数を足すだけで、discover が見つけてくれる
#[page("/about")]
async fn about() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"このアプリについて"</title>
            </head>
            <body>
                <h1>"このアプリについて"</h1>
                <p>"第6巻 UNIT 02 の練習で作りました。"</p>
                <p><a href="/">"トップへ戻る"</a></p>
            </body>
        </html>
    })
}

#[tokio::main]
async fn main() {
    let router = Router::builder().discover().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
cargo run --bin u02_q1
```

```text
### curl.exe -i http://127.0.0.1:3000/about
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-length: 258
date: Wed, 28 Oct 2026 21:55:22 GMT

<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><title>このアプリについて</title></head><body><h1>このアプリについて</h1><p>第6巻 UNIT 02 の練習で作りました。</p><p><a href="/">トップへ戻る</a></p></body></html>
```

- `discover()` が、新しい `about` も自動で集めたので、`main` を書き換えずに、`/about` が `200 OK` になりました。
- トップページのリンクは、`<a href="/about">` と URL を文字列で書いています。ページの名前を変えたときに、リンクを直し忘れても、コンパイルでは気づけません。Topcoat には、関数を指定してリンクの URL を作る `href!` があり、RustNote はそちらを使っています（第9巻 UNIT 04・この巻の UNIT 03（準備中））。

**問2の解答:**

```powershell
$env:PORT = "3001"
cargo run --bin web_lab
```

別の窓から:

```text
### curl.exe -s -o NUL -w "%{http_code}" http://127.0.0.1:3001/
200
### curl.exe -s -o NUL -w "%{http_code}" http://127.0.0.1:3000/
000
(curl の終了コード: 7)
```

- 3001 番は `200` でした。3000 番は、何も待ち受けていないので、`-w "%{http_code}"` の表示は `000`（応答が無い）で、curl は終了コード 7（接続できない）で終わりました（UNIT 01 のエラー2）。
- `topcoat::start` は、`PORT` を読んで、その番号で待ち受けます。プログラムを書き換えずに、待ち受ける場所を変えられるのです。確かめたら、`Remove-Item Env:PORT` で元に戻します。

**問3の解答:**

```powershell
curl.exe -i --compressed http://127.0.0.1:3000/about
```

```text
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-encoding: gzip
transfer-encoding: chunked
date: Wed, 28 Oct 2026 21:55:22 GMT
```

- `--compressed` を付けると、curl は `Accept-Encoding`（受け取れる圧縮の形式）のヘッダーを付けて送ります。Topcoat は、本文を gzip で圧縮し、`content-encoding: gzip`（本文は gzip で圧縮してあります）を付けて返しました。圧縮したので、`content-length` の代わりに、`transfer-encoding: chunked`（本文を区切って少しずつ送ります）になっています。curl は、受け取った本文を元に戻して表示します（ここでは、ヘッダーの部分だけを載せています）。
- `vary: accept-encoding` は、「この応答は、要求の `Accept-Encoding` によって中身が変わります」という印です。途中で応答を覚えておく仕組み（キャッシュ）が、圧縮できない相手に、圧縮した応答を渡してしまわないように付けます。`vary: Accept` も同じで、要求の `Accept`（受け取れる種類）によって中身が変わることがある、という印です。
- ブラウザは、いつも `Accept-Encoding` を付けて送るので、ブラウザから見るときは、圧縮された応答が届きます。Topcoat の既定の機能（`compression`）が、これをしています。

### まとめ

- `cargo add topcoat@=0.10.0` と Tokio を足し、`#[page("/")]` を付けた関数と、`Router::builder().discover().build()`・`topcoat::start(router).await` の数行で、Web アプリが動く。版は `=` で固定する（0.11.0 が出ていても、0.10.0 のまま）。
- 要求の振り分け・`Content-Length`・404・405 と `allow`・HEAD・圧縮は、Topcoat がする。`topcoat::start` は `HOST` と `PORT` を読み、起動しても何も表示しない。
- 「待っています」と表示したいときは、`TcpListener::bind` で先に押さえてから表示し、`topcoat::serve` に渡す。`.await` の書き忘れ、trait の `use` の書き忘れ、実行ファイルが2つあるときの `--bin` のエラーを読めるようにしておく。

## この巻のまとめ

この版に収録した UNIT 01〜02 で、次のことができるようになったかを確かめましょう。できたものに印を付けてください。

**UNIT 01: HTTP の要求・応答・状態コード**

- ☐ HTTP の要求と応答が「1行目・ヘッダー・空行・本文」の文字の並びであることを、標準ライブラリだけのサーバーで確かめられる
- ☐ 要求の1行目（メソッド・パス・版）と、応答の1行目（版・状態コード・説明）を読める
- ☐ `Content-Length` に入れるのが文字数ではなくバイト数である理由と、まちがえたときに本文が切れることを説明できる
- ☐ 200・301・303・403・404・405 の違いと、405 のときに `Allow` を付ける理由を言える
- ☐ GET は見るだけ、POST は変えるという使い分けと、POST の後に 303 で GET へ移す PRG の理由を説明できる
- ☐ `curl.exe` の `-i`・`-d`・`-X`・`-I`・`-L`・`-v` を使い分け、`-v` の `>`・`<`・`*` の行を読み分けられる
- ☐ curl の `(7)`（たどり着けない）と、404（たどり着いたが、ページが無い）を見分けられる

**UNIT 02: Topcoat 0.10 の最小のアプリ**

- ☐ `cargo add topcoat@=0.10.0` と Tokio で、ページが1つのアプリを作って起動できる。`=` で版を固定する理由を言える
- ☐ `#[page("/")]`・`Router::builder().discover().build()`・`topcoat::start(router).await` の役割を説明できる
- ☐ 404・405 と `allow`・HEAD・`Content-Length`・圧縮を、Topcoat が代わりにしてくれることを、`curl.exe` で確かめられる
- ☐ `HOST` と `PORT` の環境変数で待ち受ける場所を変えられる。`PORT` が数でないときのエラーを読める
- ☐ `topcoat::start` が何も表示しないことと、`TcpListener::bind` で押さえてから表示して `topcoat::serve` に渡す書き方を説明できる
- ☐ `.await` の書き忘れ（E0599）、`RouterBuilderDiscoverExt` の `use` の書き忘れ（E0599）、実行ファイルが2つあるときの `--bin` のエラーを読める

> **お知らせ:** この版には UNIT 01〜02 を収録しています。UNIT 03（ルーター・page・route・layout）・UNIT 04（URL の部品と Cx）・UNIT 05（view!・コンポーネント・HTML のエスケープ）・UNIT 06（フォーム・JSON・PRG）・UNIT 07（app_context・memoize）・UNIT 08（アセット・CSS・束）・UNIT 09（runtime・signal・procedure・shard）・UNIT 10（エラー・クッキー・セッション・CSRF）は準備中で、次の更新から順に追加します。それまでの間、Topcoat を使った完成したアプリの例は、教材のフォルダの `Rust/rustnote/src/routes.rs` から読み始められます。

## 用語ミニ辞典

この巻（UNIT 01〜02）に出てきた用語です。

### UNIT 01 の用語

| 用語 | 意味 |
|---|---|
| HTTP | ブラウザとサーバーが、要求と応答をやり取りするときの決まり |
| 要求（request） | サーバーへの頼み。1行目（メソッド・パス・版）・ヘッダー・空行・本文でできている |
| 応答（response） | サーバーからの返事。1行目（版・状態コード・説明）・ヘッダー・空行・本文でできている |
| メソッド | 要求の種類。GET（見る）・POST（変える）・HEAD（ヘッダーだけ見る）・DELETE（消す）など |
| パス | 要求の1行目の、`/notes` のような場所の部分。`?` の後ろはクエリ |
| ヘッダー | 「名前: 値」の行の並び。空行で終わる |
| `Content-Length` | 本文のバイト数。受け取る側は、この数だけ本文を読む |
| 状態コード | 応答の1行目の3桁の数。2xx 成功・3xx 別の場所へ・4xx 頼み方が違う・5xx サーバーの失敗 |
| `Location` | 3xx の応答で、行き先を教えるヘッダー |
| `Allow` | 405 の応答で、使えるメソッドを教えるヘッダー |
| PRG | POST → Redirect（303）→ GET。再読み込みで二重に送られないようにする形 |
| 安全なメソッド | 何回送っても、サーバーの中のデータを変えないメソッド（GET・HEAD） |
| `TcpListener` | 決まったアドレスとポートで、接続を待つ標準ライブラリの型 |
| `127.0.0.1` | 「この PC 自身」を表すアドレス。ほかの PC からはつながらない |
| curl の `(7)` | サーバーに接続できなかったときの curl のエラー |

### UNIT 02 の用語

| 用語 | 意味 |
|---|---|
| Web フレームワーク | 要求を読み、URL ごとの処理に振り分け、応答を返す、といった決まった仕事を引き受けてくれる道具 |
| ルーター | 要求の URL とメソッドを見て、どの関数に処理させるかを決める仕組み |
| `#[page("/")]` | この関数は `GET /`（と HEAD）に答える、という Topcoat の属性マクロ |
| `discover()` | プログラムの中の `#[page]` の関数を、全部自動で集める（`RouterBuilderDiscoverExt` を use して使う） |
| `topcoat::start` | 環境変数 `HOST`・`PORT`（無ければ 127.0.0.1:3000）で待ち受けを始める。起動しても何も表示しない |
| `topcoat::serve` | 自分で押さえた待ち受けの場所（`TcpListener` など）とルーターを受け取って、要求をさばく |
| `view!` | HTML をほぼそのまま書いて、ページの中身を作るマクロ |
| `vary` | この応答が、要求のどのヘッダーによって変わるかを伝えるヘッダー |
| `content-encoding: gzip` | 本文を gzip で圧縮してある、という印 |
| `default-run` | 実行ファイルが2つ以上あるとき、`cargo run` で動かすものを決める `Cargo.toml` の設定 |

<div style="page-break-after: always; text-align: center; margin: 0; padding: 0;">
<img src="assets/cover.jpg" alt="表紙" style="width: 100%; max-height: 175mm; object-fit: cover; border-radius: 10px;">
<div style="margin-top: 14px; font-size: 11px; letter-spacing: 0.18em; color: #c2410c; font-weight: 700;">VOLUME 06 / RUST BACKEND MASTERY</div>
<div style="font-size: 26px; font-weight: 800; color: #0f172a; margin-top: 6px; line-height: 1.35;">Web 基礎と Topcoat 0.11 完全ガイド</div>
<div style="font-size: 13px; color: #475569; margin-top: 8px;">HTTP の要求と応答から、Topcoat のページ・ルーター・view!・フォーム・アセット・セッションまでを、動かして確かめる</div>
</div>

# 第6巻: Web 基礎と Topcoat 0.11 完全ガイド

## この巻で身に付けること

第5巻までで、Rust の文法・所有権・設計・async を学びました。この巻では、それを使って **Web のサーバー**を作ります。ブラウザから届いた「この画面を見せて」という頼みに、Rust のプログラムが HTML を作って返す仕組みです。

Web のサーバーは、いきなり道具（フレームワーク）から始めると、何が起きているのかが見えにくくなります。次のような場面で、困ることがよくあります。

- ブラウザでは何も表示されないのに、どこが悪いのか見当がつかない
- 「404」「405」「303」のような数字が返ってきたが、それが何を意味するのか分からない
- フォームを送ったあとに再読み込みしたら、同じデータがもう1件できてしまった
- 日本語の文字が、途中で切れて表示される

そこで、この巻の最初の単元では、道具を使わずに、**標準ライブラリだけ**で小さなサーバーを書き、HTTP の要求と応答が「ただの文字のやり取り」であることを、自分の目で確かめます。そのうえで、UNIT 02 から、RustNote でも使っている Web フレームワーク **Topcoat 0.11** の使い方を、1つずつ学びます。

### 到達目標

この巻を終えると、次のことができるようになります。

- HTTP の要求（メソッド・パス・ヘッダー・本文）と応答（状態コード・ヘッダー・本文）の形を説明し、`curl.exe -i` や `-v` の表示を読める
- 状態コード（200・303・403・404・405 など）と、GET・POST の使い分けを説明できる
- Topcoat 0.11 で、ページ（`#[page]`）・ルーター・レイアウト・URL の部品の読み取り・`view!` とコンポーネントを使った画面を作れる
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
<text x="347" y="99" font-size="12" fill="#475569" text-anchor="middle">Topcoat 0.11</text>
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
| UNIT 02 | Topcoat 0.11 の最小のアプリ | ページを1つ作って起動する |
| UNIT 03 | ルーター・page・route・layout | URL ごとに処理を分け、共通の外枠で包む |
| UNIT 04 | URL の部品（path・query）と Cx | `/notes/{id}` や `?page=2` を型にして読む |
| UNIT 05 | view!・コンポーネント・HTML のエスケープ | 画面を部品に分け、入力を安全に表示する |
| UNIT 06 | フォーム・JSON・PRG | 送られてきた値を型にし、303 で移る |
| UNIT 07 | app_context・memoize | DB などを全部の処理で共有し、同じ問い合わせを1回にする |
| UNIT 08 | アセット・CSS・束 | `asset!` と `topcoat asset bundle` |
| UNIT 09 | runtime・signal・procedure・shard | ブラウザの中で動く部分と、その守り方 |
| UNIT 10 | エラー・クッキー・セッション・CSRF | 400・404・リダイレクトと、ログインの仕組みの土台 |

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
| Rust | **1.99 以上**（本書シリーズの基準。Topcoat 0.11 が 1.99 以上を求めます）。動作確認は rustc 1.99.1 / cargo 1.99.1（`aarch64-pc-windows-msvc`） |
| edition | 2024 |
| curl | 8.21.0（Windows に入っている `curl.exe`。PowerShell では `curl` だけだと別のコマンドになるので、必ず `curl.exe` と打ちます） |
| topcoat | 0.11.0（2026-10-24 に出た版。UNIT 02 から使います。UNIT 01 では使いません） |

### 練習用のプロジェクトを用意する

UNIT 01 では、外のクレートを1つも使いません。練習用のプロジェクトを1つ作り、`src/bin/` に、単元ごとの小さなサーバーを足していきます。

```powershell
cargo new http_lab
cd http_lab
# このフォルダでは Rust 1.99.1 を使う（第13巻 UNIT 01）
Set-Content rust-toolchain.toml "[toolchain]`nchannel = `"1.99.1`""
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

- だから、何かを**変える**操作は POST（や PUT・DELETE）にして、GET は**見るだけ**にします。POST にしても、ほかのサイトからフォームで送らせる攻撃（CSRF）は残るので、その守りを足します（第8巻 UNIT 01、この巻の UNIT 10）。

**問3の解答:**

- **`> ` で始まる行**が、curl が送った要求です（`> GET /hello?name=ada HTTP/1.1` から、空の `> ` の行まで）。**`< ` で始まる行**が、サーバーが返した応答の1行目とヘッダーです（`< HTTP/1.1 200 OK` から、空の `< ` の行まで）。矢印の向き（`>` は外へ、`<` は内へ）と覚えます。
- **`* ` で始まる行**は、curl が自分の動きを説明しているメモです。`Trying 127.0.0.1:3000...`（つなごうとしています）、`Established connection`（つながりました）、`Request completely sent off`（要求を送り終えました）、`Connection #0 to host 127.0.0.1:3000 left intact`（接続は閉じずに残しました）のように、HTTP の文字ではなく、その外側で起きたことを表します。
- `{ [26 bytes data]` は、本文を 26 バイト受け取ったという印です。`-v` と一緒に `-o NUL`（本文は捨てる）を付けたので、本文そのものは表示されていません。

### まとめ

- HTTP の要求と応答は、どちらも「1行目・ヘッダー・空行・本文」の文字の並び。要求の1行目はメソッドとパス、応答の1行目は状態コード。本文の長さは `Content-Length` に**バイト数**で書く。
- 状態コードは、2xx（うまくいった）・3xx（別の場所へ。`Location` で行き先を教える）・4xx（頼み方が違う。404・403・405 は直し方が違う）・5xx（サーバーの失敗）。curl の `(7)` は、そもそもサーバーにたどり着けていない。
- GET は見るだけ、POST は変える。POST が成功したら 303 で GET の画面へ移す（PRG）と、再読み込みで二重に保存されない。

## UNIT 02: Topcoat 0.11 の最小のアプリ

### この単元のゴール

- `cargo add` で Topcoat 0.11.0 と Tokio を足し、ページが1つだけの Web アプリを作って起動できる
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

> **▼ このコマンドがやること（先に日本語で）:** 新しいプロジェクトを作り、Rust の版を 1.99.1 に固定してから、`cargo add` で Topcoat（版を 0.11.0 に固定）と Tokio（非同期のランタイム）を足します。

```powershell
cargo new web_lab
cd web_lab
Set-Content rust-toolchain.toml "[toolchain]`nchannel = `"1.99.1`""
cargo add topcoat@=0.11.0
cargo add tokio@=1.54.1 --features rt-multi-thread,macros
```

**▼ 実行結果**（`cargo add topcoat@=0.11.0` の表示）

```text
    Updating crates.io index
      Adding topcoat =0.11.0 to dependencies
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
             22 deactivated features
    Updating crates.io index
     Locking 220 packages to highest Rust 1.99.1 compatible versions
      Adding generic-array v0.14.7 (available: v0.14.9)
```

- `+` の付いた features（`asset`・`router`・`view` など）が、既定で有効になる機能です。そのほかの機能（`tailwind`・`websocket`・`l10n` など）は既定では無効で、数が多いので、`22 deactivated features`（無効の機能が 22 個）とだけ表示されます。一覧は `cargo info -v topcoat@0.11.0` で見られます。必要なときに、自分で足します。
- `Locking 220 packages to highest Rust 1.99.1 compatible versions` は、「Rust 1.99.1 で使える範囲で、いちばん新しい版を選んで、220 個のクレートの版を `Cargo.lock` に書きました」という意味です。`Adding generic-array v0.14.7 (available: v0.14.9)` は、「もっと新しい 0.14.9 もありますが、0.14.7 にしました」というお知らせです。間接的に使う `crypto-common` が `=0.14.7` と版を固定しているからで、気にしなくて大丈夫です。
- `@=0.11.0` の `=` を付けているので、Topcoat に新しい版が出ても、勝手には上がりません（第13巻 UNIT 01）。Topcoat は、版が上がると書き方が変わることがあるので、版を上げるときは、自分で決めて、まとめて直します。この本も、0.10.0 から 0.11.0 に上げたときに、すべての巻のコードを動かし直しました。

できあがった `Cargo.toml` です（この単元の最後に、もう1つの機能を足したあとの形です。例2-3で説明します）。

```toml
[package]
name = "web_lab"
version = "0.1.0"
edition = "2024"

[dependencies]
tokio = { version = "=1.54.1", features = ["rt-multi-thread", "macros", "net"] }
topcoat = "=0.11.0"
```

#### 例2-2: ページが1つだけのアプリ

> **▼ このコードがやること（先に日本語で）:** `GET /` に答える関数 `home` に `#[page("/")]` の名札を付け、`view!` で HTML を作って返します。`main` では、名札の付いた関数を `discover()` で集めてルーターを作り、`topcoat::start` に渡して、待ち受けを始めます。

```rust
//! Topcoat 0.11 の、いちばん小さな Web アプリ。GET / に HTML を返す。
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

最初の1回は、Topcoat が使うクレートをコンパイルするので、少し時間がかかります（この PC では、`Compiling` の行が 164 行出て、約 22 秒でした）。

```text
   Compiling web_lab v0.1.0 (...\web_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 21.90s
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
date: Fri, 30 Oct 2026 11:31:05 GMT

<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><title>はじめての Topcoat</title></head><body><h1>こんにちは、Topcoat！</h1><p>この HTML は Rust のプログラムが作りました。</p></body></html>
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: #[page("/")] は、URL の名札

- `#[page("/")]` は、「この関数は `GET /` に答える」という印です。`#[page]` は、ふつうの関数を、ページの処理に変える**属性マクロ**です。何も指定しなければ GET に答え、HEAD にも自動で答えます。
- 関数の名前（`home`）は、URL には関係ありません。URL を決めるのは、`#[page(...)]` の中の文字列です。
- 戻り値の `Result<impl View>` は、「HTML になる何か（`View`）を返す。失敗することもある」という意味です。`view!` の中には、HTML をほぼそのまま書けます。文字は `"..."` で囲みます（UNIT 05）。

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
date: Fri, 30 Oct 2026 11:31:05 GMT

not found

### curl.exe -i -X POST http://127.0.0.1:3000/
HTTP/1.1 405 Method Not Allowed
allow: GET, HEAD
content-length: 0
date: Fri, 30 Oct 2026 11:31:05 GMT



### curl.exe -I http://127.0.0.1:3000/
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-length: 228
date: Fri, 30 Oct 2026 11:31:05 GMT
```

- 無い URL（`/nothing`）は `404 Not Found` で、本文は `not found` だけです。ページに名札を付けていない URL は、Topcoat が自動で 404 にします。自分の画面（ヘッダーやトップへのリンクの付いたもの）で 404 を出したいときは、`not_found!()` とレイアウトを使います（UNIT 03・10）。
- `POST /` は `405 Method Not Allowed` と `allow: GET, HEAD` です。`#[page]` は GET（と HEAD）だけに答えるので、Topcoat が「使えるのは GET と HEAD です」と教えてくれました。UNIT 01 の問1で「本当は 405 と Allow を返すべき」と書いたことを、Topcoat は何も書かなくてもしてくれます。
- `HEAD /`（`-I`）は、GET と同じヘッダー（`content-length: 228` も同じ）で、本文はありません。

### もう一歩: 実務に近い形

#### 例2-3: 待ち受けの場所を押さえてから、「待っています」と表示する

`topcoat::start` は、何も表示しません。「起動しました」と表示したくて、`start` の前に `println!` を書くと、困ったことが起きます。ポートを押さえる**前**に表示するので、ほかのプログラムがポートを使っていて起動に失敗したときも、「起動しました」と出てしまうのです（第12巻 UNIT 01 のエラー2で、RustNote が実際にそうなっているのを見ました）。

そこで、待ち受けの場所を**自分で先に押さえて**から表示し、押さえた場所を Topcoat に渡す書き方をします。`topcoat::start` の中身（Topcoat 0.11.0 の `serve.rs`）も、`TcpListener::bind` で押さえてから `serve` を呼ぶ形になっています。

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
cargo add tokio@=1.54.1 --features rt-multi-thread,macros,net
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

thread 'main' (202724) panicked at src\main.rs:35:10:
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
- RustNote のルーターは、`discover()` ではなく、`module_router!()` で作っています（`src/routes.rs` の `build_router`）。ファイルの並びをそのまま URL の並びにする書き方で、UNIT 03 で扱います。

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
date: Fri, 30 Oct 2026 11:31:29 GMT

<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><title>このアプリについて</title></head><body><h1>このアプリについて</h1><p>第6巻 UNIT 02 の練習で作りました。</p><p><a href="/">トップへ戻る</a></p></body></html>
```

- `discover()` が、新しい `about` も自動で集めたので、`main` を書き換えずに、`/about` が `200 OK` になりました。
- トップページのリンクは、`<a href="/about">` と URL を文字列で書いています。ページの名前を変えたときに、リンクを直し忘れても、コンパイルでは気づけません。Topcoat には、関数を指定してリンクの URL を作る `href!` があり、RustNote はそちらを使っています（第9巻 UNIT 04・この巻の UNIT 03）。

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
date: Fri, 30 Oct 2026 11:31:29 GMT
```

- `--compressed` を付けると、curl は `Accept-Encoding`（受け取れる圧縮の形式）のヘッダーを付けて送ります。Topcoat は、本文を gzip で圧縮し、`content-encoding: gzip`（本文は gzip で圧縮してあります）を付けて返しました。圧縮したので、`content-length` の代わりに、`transfer-encoding: chunked`（本文を区切って少しずつ送ります）になっています。curl は、受け取った本文を元に戻して表示します（ここでは、ヘッダーの部分だけを載せています）。
- `vary: accept-encoding` は、「この応答は、要求の `Accept-Encoding` によって中身が変わります」という印です。途中で応答を覚えておく仕組み（キャッシュ）が、圧縮できない相手に、圧縮した応答を渡してしまわないように付けます。`vary: Accept` も同じで、要求の `Accept`（受け取れる種類）によって中身が変わることがある、という印です。
- ブラウザは、いつも `Accept-Encoding` を付けて送るので、ブラウザから見るときは、圧縮された応答が届きます。Topcoat の既定の機能（`compression`）が、これをしています。

### まとめ

- `cargo add topcoat@=0.11.0` と Tokio を足し、`#[page("/")]` を付けた関数と、`Router::builder().discover().build()`・`topcoat::start(router).await` の数行で、Web アプリが動く。版は `=` で固定する（新しい版が出ても、勝手には上がらない）。
- 要求の振り分け・`Content-Length`・404・405 と `allow`・HEAD・圧縮は、Topcoat がする。`topcoat::start` は `HOST` と `PORT` を読み、起動しても何も表示しない。
- 「待っています」と表示したいときは、`TcpListener::bind` で先に押さえてから表示し、`topcoat::serve` に渡す。`.await` の書き忘れ、trait の `use` の書き忘れ、実行ファイルが2つあるときの `--bin` のエラーを読めるようにしておく。

## UNIT 03: ルーター・page・route・layout

### この単元のゴール

- `module_router!()` で、Rust のモジュールの並びを、そのまま URL の並びにできる（`about` → `/about`、`blog_posts` → `/blog-posts`）
- 画面を返す `#[page]` と、文字や JSON を返す `#[route(GET)]` を使い分けられる
- `#[layout]` で全ページに共通の外枠（HTML の骨組み・メニュー）を作り、入れ子のレイアウトと、処理を包む `#[layer]` の順番を説明できる
- `href!` でリンク先を作る理由と、同じ URL のページが2つあるときの起動時の panic・`(slot)` の書き忘れ・`href!` の間違いを読める

### たとえ話でつかむ

大きなデパートを考えます。

- デパートは、**階**と**売り場**に分かれています。「5階の、おもちゃ売り場」のように、場所の並びで売り場が決まります。案内板は、その並びをそのまま書いています。
- どの売り場にも、**同じ建物の壁と天井**があります。売り場の人は、壁や天井を自分で作りません。建物（外枠）の中に、売り場の中身だけを並べます。
- 5階の売り場には、さらに**5階だけの飾り付け**があることもあります。建物の外枠の中に、5階の飾り付けがあり、その中に売り場があります。
- 入口には**警備員**がいて、入ってきた人と出ていった人を、全員記録します。売り場の中で何が起きても、出入りは必ず警備員の前を通ります。
- 店内の案内は、「5階のおもちゃ売り場」と**売り場の名前**で書きます。売り場が引っ越しても、名前で書いた案内なら、案内板を見れば新しい場所が分かります。

| デパート | Topcoat のアプリ |
|---|---|
| 階と売り場の並び | Rust の**モジュールの並び**（`mod about` → `/about`） |
| 案内板 | `module_router!()`（モジュールの並びから、URL の名簿を作る） |
| 売り場 | `#[page]`（画面を返す）と `#[route(GET)]`（文字や JSON を返す） |
| 建物の壁と天井 | `#[layout]`（どのページにも共通の、HTML の外枠） |
| 5階だけの飾り付け | 入れ子のレイアウト（`/settings` より下だけの外枠） |
| 入口の警備員 | `#[layer]`（要求の処理を、前と後ろで包む） |
| 売り場の名前で書いた案内 | `href!(about::about_page)`（関数の名前からリンク先を作る） |

> **用語:** モジュールルーティング（module routing：ファイルやモジュールの並びから、URL を決める書き方。Topcoat が勧める書き方で、RustNote もこれを使う）

<!-- -->

> **用語:** レイアウト（layout：いくつものページを包む、共通の外枠。HTML の `<html>`・`<head>`・メニューなどを1か所に書く）

<!-- -->

> **用語:** レイヤー（layer：要求の処理の前と後ろに、決まった仕事（時間を測る・記録する など）をはさむもの）

### 図で理解する

例3-1のアプリの、モジュールと URL の対応です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0301" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="160" y="22" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">Rust のモジュール（u03_site.rs）</text>
<text x="560" y="22" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">URL</text>
<rect x="20" y="36" width="280" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="160" y="54" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">（ファイルの一番上）</text>
<text x="160" y="70" font-size="11" fill="#475569" text-anchor="middle">module_router!() を呼ぶ・#[layout]・#[page] home</text>
<rect x="60" y="88" width="240" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="180" y="110" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">mod about（#[page]）</text>
<rect x="60" y="134" width="240" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="180" y="156" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">mod blog_posts（#[page]）</text>
<rect x="60" y="180" width="240" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="180" y="202" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">mod api</text>
<rect x="100" y="222" width="200" height="34" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="200" y="244" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">mod health（#[route(GET)]）</text>
<rect x="440" y="36" width="250" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="565" y="61" font-size="13" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">GET /</text>
<rect x="440" y="88" width="250" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="565" y="110" font-size="13" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">GET /about</text>
<rect x="440" y="134" width="250" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="565" y="156" font-size="13" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">GET /blog-posts</text>
<rect x="440" y="222" width="250" height="34" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="565" y="244" font-size="13" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">GET /api/health（文字）</text>
<line x1="300" y1="56" x2="436" y2="56" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0301)"/>
<line x1="300" y1="105" x2="436" y2="105" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0301)"/>
<line x1="300" y1="151" x2="436" y2="151" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0301)"/>
<line x1="300" y1="239" x2="436" y2="239" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0301)"/>
<text x="368" y="138" font-size="11" fill="#c2410c" text-anchor="middle">_ → -</text>
<text x="368" y="198" font-size="10" fill="#475569" text-anchor="middle">api は区切りだけ</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-7: モジュールと URL の対応。module_router!() を呼んだ場所が / になり、モジュールが1つ深くなるたびに、URL の区切りが1つ増える。モジュール名の _ は - になる</div>
</div>

- `module_router!()` を呼んだモジュール（ここではファイルの一番上）が `/` になります。`mod about` の中の `#[page]` は `/about` に、`mod api` の中の `mod health` の `#[route(GET)]` は `/api/health` になります。
- **関数の名前は、URL に関係ありません**。`about_page` という名前でも、URL は `/about` です。
- モジュールの名前の `_` は、URL では `-` になります（`blog_posts` → `/blog-posts`）。URL では、単語を `-` でつなぐのがふつうだからです。

1つの要求が、レイヤー・レイアウト・ページを通る順番です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="20" width="680" height="210" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="40" y="44" font-size="13" font-weight="700" fill="#c2410c">#[layer] timing（前に時刻を記録 → 中を動かす → 後ろで時間を表示）</text>
<rect x="50" y="60" width="620" height="155" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="70" y="84" font-size="13" font-weight="700" fill="#1e40af">#[layout] root_layout（/ より下のすべて）: &lt;html&gt;・見出し・(slot)</text>
<rect x="80" y="100" width="560" height="100" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="100" y="124" font-size="13" font-weight="700" fill="#047857">#[layout] settings_layout（/settings より下だけ）: メニュー・(slot)</text>
<rect x="110" y="140" width="500" height="44" rx="8" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="360" y="167" font-size="13" fill="#0f172a" text-anchor="middle">#[page] profile（GET /settings/profile）: &lt;h1&gt;プロフィール&lt;/h1&gt;</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-8: 入れ子の順番。レイヤーがいちばん外で要求の処理を包み、その中で、広い範囲のレイアウトから狭い範囲のレイアウトへ、最後にページが入る</div>
</div>

### まず動かす最小コード

#### 例3-1: モジュールで URL を作り、レイアウトで包む

> **▼ このコードがやること（先に日本語で）:** ファイルの一番上に、全ページを包むレイアウト `root_layout` と、トップページ `home` を書きます。`mod about`・`mod blog_posts` の中にページを1つずつ、`mod api` の中の `mod health` に、文字だけを返す `#[route(GET)]` を書きます。レイアウトのメニューのリンク先は `href!` で作り、今開いているページのリンクには `aria-current="page"` と `active` のクラスを付けます。`main` では `module_router!()` でルーターを作ります。

```rust
//! 第6巻 UNIT 03: モジュールの並びを、そのまま URL の並びにする（module_router!）。
//! 実行: cargo run --bin u03_site

use topcoat::{
    Result,
    context::Cx,
    router::{Slot, href, layout, module_router, page},
    view::{View, class, view},
};

/// すべてのページを包むレイアウト（HTML の外枠と、上のメニュー）
#[layout]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    // 「この場所について」のリンク先と、いまそのページを開いているか
    let about = href!(about::about_page);
    let on_about = about.is_current(cx);
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"UNIT 03 の練習"</title>
            </head>
            <body>
                <nav>
                    <a href=(href!(home))>"トップ"</a>
                    <a
                        href=(about)
                        aria-current=(on_about.then_some("page"))
                        class=(class!("nav", "active" if on_about))
                    >
                        "この場所について"
                    </a>
                    <a href=(href!(blog_posts::list))>"記事"</a>
                </nav>
                (slot)
            </body>
        </html>
    })
}

/// GET /
#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <h1>"トップ"</h1> })
}

mod about {
    use super::*;

    /// GET /about
    #[page]
    pub async fn about_page() -> Result<impl View> {
        Ok(view! { <h1>"この場所について"</h1> })
    }
}

mod blog_posts {
    use super::*;

    /// GET /blog-posts（モジュール名の _ は - になる）
    #[page]
    pub async fn list() -> Result<impl View> {
        Ok(view! { <h1>"記事の一覧"</h1> })
    }
}

mod api {
    mod health {
        use topcoat::{Result, router::route};

        /// GET /api/health（HTML ではなく、文字だけを返す）
        #[route(GET)]
        async fn health() -> Result<&'static str> {
            Ok("ok")
        }
    }
}

#[tokio::main]
async fn main() {
    // このファイル（クレートの一番上）が / になる
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

UNIT 02 の `web_lab` の `src/bin/u03_site.rs` として保存し、起動します。

```powershell
cargo run --bin u03_site
```

2つ目の窓から、4つの URL を開きます。

```powershell
curl.exe -i http://127.0.0.1:3000/
curl.exe -i http://127.0.0.1:3000/about
curl.exe -i http://127.0.0.1:3000/blog-posts
curl.exe -i http://127.0.0.1:3000/api/health
```

**▼ 実行結果**

```text
### curl.exe -i http://127.0.0.1:3000/
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-length: 261
date: Fri, 30 Oct 2026 11:37:48 GMT

<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><title>UNIT 03 の練習</title></head><body><nav><a href="/">トップ</a><a href="/about" class="nav">この場所について</a><a href="/blog-posts">記事</a></nav><h1>トップ</h1></body></html>

### curl.exe -i http://127.0.0.1:3000/about
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-length: 303
date: Fri, 30 Oct 2026 11:37:48 GMT

<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><title>UNIT 03 の練習</title></head><body><nav><a href="/">トップ</a><a href="/about" aria-current="page" class="nav active">この場所について</a><a href="/blog-posts">記事</a></nav><h1>この場所について</h1></body></html>

### curl.exe -i http://127.0.0.1:3000/blog-posts
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-length: 267
date: Fri, 30 Oct 2026 11:37:48 GMT

<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><title>UNIT 03 の練習</title></head><body><nav><a href="/">トップ</a><a href="/about" class="nav">この場所について</a><a href="/blog-posts">記事</a></nav><h1>記事の一覧</h1></body></html>

### curl.exe -i http://127.0.0.1:3000/api/health
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 2
date: Fri, 30 Oct 2026 11:37:49 GMT

ok
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: module_router!() は、モジュールの並びから名簿を作る

- `module_router!()` を呼んだ場所（このファイルの一番上）が `/` になり、その中の `#[page] home` が `GET /` に答えました。`mod about` の中のページは `/about`、`mod blog_posts` の中のページは `/blog-posts` です。
- `#[page]` に URL の文字列を書いていないことに注目してください。UNIT 02 では `#[page("/")]` と書きましたが、モジュールルーティングでは、**モジュールの場所が URL を決める**ので、書きません。
- `module_router!()` は、ファイルを探して回るわけではありません。Rust が `mod about` などの宣言でコンパイルしたモジュールだけが、名簿に入ります。別のファイル（`src/about.rs`）に分けたときは、`mod about;` の宣言を忘れないようにします（宣言を忘れると、そのページは 404 になります）。

##### 解説2: #[page] は画面、#[route(GET)] は文字や JSON

- `/api/health` は、`content-type: text/plain; charset=utf-8` で、本文は `ok` の2文字だけでした。`#[route(GET)]` の関数は、`&'static str`（文字）や `Json`（第6巻 UNIT 06）などを返し、それがそのまま応答になります。**レイアウトにも包まれません**。
- `#[page]` は、`View`（画面）を返し、レイアウトに包まれます。人が見る画面は `#[page]`、プログラムやヘルスチェックが読む応答は `#[route]`、と使い分けます。
- `#[route]` は、メソッドを必ず書きます（`#[route(GET)]`・`#[route(POST)]`、いくつかなら `#[route([GET, POST])]`）。`#[page]` は、書かなければ GET です。

##### 解説3: レイアウトは (slot) の所にページを入れる

- 4つのページのどれにも、`<nav>`（メニュー）が付いています。レイアウト `root_layout` が、`(slot)` の所に、それぞれのページの中身（`<h1>…</h1>`）を入れたからです。ページの関数には、`<html>` も `<nav>` も書いていません。
- `/about` を開いたときだけ、「この場所について」のリンクに `aria-current="page"` と `class="nav active"` が付きました。`href!(about::about_page).is_current(cx)` が「今、このリンク先を開いている」を `true` で返したからです。`aria-current="page"` は、画面の読み上げソフトに「これが今のページです」と伝える印で、`active` のクラスは、見た目（CSS）で強調するために使います。
- `class!("nav", "active" if on_about)` は、`on_about` が `true` のときだけ `active` を足して、`class` の値を作ります。`aria-current=(on_about.then_some("page"))` は、`None` のときは属性そのものを書きません（`/` の表示では、`aria-current` がありません）。

##### 解説4: リンク先は href! で作る

- メニューのリンク先は、`"/about"` と文字列で書かずに、`href!(about::about_page)` と、**ページの関数の名前**で書きました。モジュールの名前を変えて URL が変わっても、`href!` は新しい URL を作るので、リンクを直し忘れません。名前の付け間違いは、コンパイルで分かります（エラー2）。

### もう一歩: 実務に近い形

#### 例3-2: レイアウトを入れ子にし、レイヤーで時間を測る

`/settings` より下のページにだけ、設定のメニューを付けます。全部の要求には、処理にかかった時間を表示するレイヤーを付けます。

> **▼ このコードがやること（先に日本語で）:** ファイルの一番上に、全部の要求を包むレイヤー `timing`（処理の前に時刻を覚え、処理のあとで、メソッド・パス・状態コード・かかった時間を表示する）と、外側のレイアウトを書きます。`mod settings` の中に、内側のレイアウト `settings_layout` と、`/settings` のページを、さらにその中の `mod profile` に `/settings/profile` のページを書きます。

```rust
//! 第6巻 UNIT 03 の例3-2: レイアウトを入れ子にし、レイヤーで処理の時間を測る。
//! 実行: cargo run --bin u03_nested

use topcoat::{
    Result,
    context::Cx,
    router::{
        Body, Next, Slot, layer, layout, module_router, page,
        request::{method, uri},
        response::Response,
    },
    view::{View, view},
};

/// すべての要求を包むレイヤー: 処理にかかった時間を表示する
#[layer]
async fn timing(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let started = std::time::Instant::now();
    let result = next.run(cx, body).await;
    let status = match &result {
        Ok(response) => response.status().to_string(),
        Err(error) => format!("エラー（{error}）"),
    };
    println!(
        "{} {} -> {status}（{:?}）",
        method(cx),
        uri(cx).path(),
        started.elapsed()
    );
    result
}

/// 外側のレイアウト（/ より下の、すべてのページ）
#[layout]
async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <html>
            <body>
                <header>"サイトの見出し"</header>
                (slot)
            </body>
        </html>
    })
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <h1>"トップ"</h1> })
}

mod settings {
    use super::*;

    /// 内側のレイアウト（/settings より下のページだけ）
    #[layout]
    async fn settings_layout(slot: Slot<'_>) -> Result<impl View> {
        Ok(view! {
            <div class="settings">
                <nav>"設定のメニュー"</nav>
                (slot)
            </div>
        })
    }

    /// GET /settings
    #[page]
    async fn settings_top() -> Result<impl View> {
        Ok(view! { <h1>"設定"</h1> })
    }

    mod profile {
        use super::*;

        /// GET /settings/profile
        #[page]
        async fn profile() -> Result<impl View> {
            Ok(view! { <h1>"プロフィール"</h1> })
        }
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
cargo run --bin u03_nested
```

**▼ 実行結果（curl.exe -i の表示。date と vary の行は省いています）**

```text
### /
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 82

<html><body><header>サイトの見出し</header><h1>トップ</h1></body></html>

### /settings
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 139

<html><body><header>サイトの見出し</header><div class="settings"><nav>設定のメニュー</nav><h1>設定</h1></div></body></html>

### /settings/profile
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 151

<html><body><header>サイトの見出し</header><div class="settings"><nav>設定のメニュー</nav><h1>プロフィール</h1></div></body></html>

### /settings/nothing
HTTP/1.1 404 Not Found
content-type: text/plain; charset=utf-8
content-length: 9

not found
```

**▼ サーバーの窓の表示**

```text
GET / -> 200 OK（321.8µs）
GET /settings -> 200 OK（108.5µs）
GET /settings/profile -> 200 OK（110.3µs）
```

- `/settings/profile` の HTML は、外側のレイアウト（`<header>サイトの見出し</header>`）の中に、内側のレイアウト（`<div class="settings"><nav>設定のメニュー</nav>`）があり、その中にページ（`<h1>プロフィール</h1>`）が入っています。**範囲の広いレイアウトが外側、狭いレイアウトが内側**です。`/`（トップ）には、内側のレイアウトは付きません。
- レイヤーは、3つの要求のそれぞれで、時間を表示しました（数百マイクロ秒）。`next.run(cx, body).await` の前と後ろに書いたことが、ページの処理を**はさんで**動きます。
- `/settings/nothing`（無い URL）の 404 は、**サーバーの窓に表示されていません**。モジュールに書いたレイヤーは、そのモジュールより下で**見つかったルート**を包むので、どこにも当たらない要求は通らないのです。RustNote は、`not_found!()` を書いて、見つからない URL も自分の 404 の画面とアクセスログに通しています（UNIT 10）。

> **ヒント:** レイヤーは、ログ・時間の計測・決まったヘッダーを足す、のような「どの要求にも同じことをする」仕事に使います。「ログインしているか」の確かめは、レイヤーではなく、ページの最初で関数（RustNote の `require_user`）を呼ぶのが Topcoat の考え方です（第8巻 UNIT 01・UNIT 07）。

### よくあるエラーと読み方

#### エラー1: 同じ URL のページが2つあって、起動した途端に止まった

`mod about` の中に、「お問い合わせ」のページを書き足してしまった場合です（`errs/e03_conflict.rs`）。

```rust
//! 同じモジュールに、GET のページを2つ書いてしまった
use topcoat::{
    Result,
    router::{module_router, page},
    view::{View, view},
};

#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <h1>"トップ"</h1> })
}

mod about {
    use super::*;

    #[page]
    async fn about_page() -> Result<impl View> {
        Ok(view! { <h1>"この場所について"</h1> })
    }

    // 書き足したつもりが、同じモジュール（/about）に入れてしまった
    #[page]
    async fn contact_page() -> Result<impl View> {
        Ok(view! { <h1>"お問い合わせ"</h1> })
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

コンパイルは通ります。起動すると、すぐに止まります。

```text

thread 'main' (212584) panicked at src\bin\e03_conflict.rs:30:35:
duplicate route registered for `GET /about`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- **何が起きたか:** 「`GET /about` に、ルートが二重に登録されました」という panic です。同じモジュールの `#[page]` は、どれも同じ URL（`/about`）になるので、`about_page` と `contact_page` が、同じ `GET /about` を取り合いました。30 行目の `module_router!().build()` が、名簿を作るときに見つけて止めました。
- **直し方:** 「お問い合わせ」用に `mod contact` を作り、そこにページを移します（問1）。同じモジュールに置いてよいのは、**メソッドが違う**ページだけです（RustNote の `routes/notes.rs` の `GET /notes` と `POST /notes` のように）。
- **学べること:** この間違いはコンパイルでは見つからず、**起動したとき**に分かります。名簿を作る `build()` は、起動の最初に1回だけ動くので、テスト（第11巻 UNIT 03（準備中））でルーターを組み立てるだけでも見つけられます。

#### エラー2: href! に、#[page] を付け忘れた関数を渡した（E0573）

`about_page` の `#[page]` を付け忘れたまま、`href!(about_page)` でリンクを作った場合です（`errs/e03_href.rs`）。

```rust
//! href! に、ページではない、ふつうの関数の名前を渡してしまった
use topcoat::{
    Result,
    router::{href, module_router, page},
    view::{View, view},
};

/// ページではない、ただの関数（#[page] を付け忘れた）
async fn about_page() -> Result<String> {
    Ok("この場所について".to_string())
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <h1>"トップ"</h1>
        <a href=(href!(about_page))>"この場所について"</a>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
cargo build --bin e03_href
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error[E0573]: cannot find type `about_page` in this scope
  --> src\bin\e03_href.rs:17:24
   |
17 |         <a href=(href!(about_page))>"この場所について"</a>
   |                        ^^^^^^^^^^ not found in this scope
   |
   = note: a function named `about_page` exists in another namespace

For more information about this error, try `rustc --explain E0573`.
error: could not compile `web_lab` (bin "e03_href") due to 1 previous error
```

- **何が起きたか:** 「`about_page` という**型**が見つかりません」というエラーです。`note:` は「`about_page` という名前の**関数**なら、別の名前の空間にあります」と言っています。`#[page]` は、関数と同じ名前の**型**も作り、`href!` はその型から URL を作ります。`#[page]` が無いと型が作られないので、「型が無い」と言われるのです。
- **直し方:** `about_page` に `#[page]` を付けます。`href!` に渡せるのは、`#[page]`・`#[route]` などが付いた関数だけです。
- **学べること:** 文字列で `"/about"` と書いていたら、このページが無いことにコンパイルは気づけず、リンクを押したときに初めて 404 になります。`href!` なら、**リンク先が本当にあるか**をコンパイルが確かめます。

#### エラー3（エラーが出ない間違い）: レイアウトに (slot) を書き忘れた

レイアウトの `view!` に、`(slot)` を書き忘れた場合です（`errs/e03_noslot.rs`。抜き出しています）。

```rust
#[layout]
async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <html>
            <body>
                <header>"サイトの見出し"</header>
            </body>
        </html>
    })
}
```

ビルドすると、エラーではなく、**警告**が1つ出ます。

```text
   Compiling web_lab v0.1.0 (...\web_lab)
warning: unused variable: `slot`
 --> src\bin\e03_noslot.rs:9:22
  |
9 | async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
  |                      ^^^^ help: if this is intentional, prefix it with an underscore: `_slot`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `web_lab` (bin "e03_noslot") generated 1 warning (run `cargo fix --bin "e03_noslot" -p web_lab` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.45s
```

起動して開くと、200 なのに、ページの中身（`<h1>トップ</h1>`）がありません。

```text
### curl.exe -i http://127.0.0.1:3000/
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 64

<html><body><header>サイトの見出し</header></body></html>
```

- **何が起きたか:** レイアウトは、受け取ったページの中身（`slot`）を、どこにも入れませんでした。どのページを開いても、見出しだけの同じ画面になります。状態コードは 200 なので、ログを見ても気づけません。
- **気づき方:** `warning: unused variable: slot`（`slot` という変数を使っていません）が、ちゃんと教えてくれていました。`help:` の「わざとなら `_slot` にしてください」に従うのではなく、**なぜ使っていないのか**を考えます。警告も、エラーと同じように読むくせを付けましょう（`cargo clippy -- -D warnings` で、警告をエラーとして扱うこともできます。第1巻 UNIT 06（準備中））。

### RustNote ではここで使う

RustNote のルーターは、`src/routes.rs` の `build_router` で作っています（抜き出しています）。

```rust
pub fn build_router(state: AppState, assets: impl Into<AssetConfig>) -> Router {
    module_router!()
        // クッキーを読み書きできるようにする（セッションとフラッシュメッセージで使う）
        .cookies()
        // セッション（ログイン）。トークンは __Host-session クッキーで運ばれる
        .sessions(SessionConfig::default())
        // CSS を /_topcoat/assets/... で配信する
        .assets(assets)
        // どのページからも app_state(cx) で借りられるようにする
        .app_context(state)
        .build()
}
```

- 例3-1と同じく、`module_router!()` で名簿を作っています。`.cookies()`・`.sessions(…)`・`.assets(…)`・`.app_context(…)` は、ページから使う道具を、ルーターに登録しています（UNIT 07・08・10）。
- `routes.rs` の先頭のコメントには、ファイルと URL の対応の表があります。`routes/notes/id/edit.rs` が `/notes/{id}/edit` になる、のように、**ファイルの並びが、そのまま URL の並び**です。`{id}` の部分は、UNIT 04 で扱います。

同じファイルに、全ページを包むレイアウトと、アクセスログのレイヤーもあります（抜き出しています）。

```rust
/// アクセスログ: 「GET /notes -> 200 OK (1.2ms)」のように標準エラー出力へ書く
#[layer]
async fn access_log(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let started = std::time::Instant::now();
    let result = next.run(cx, body).await;
    // ヘルスチェックは数十秒ごとに来るので、ログに出さない
    if uri(cx).path() != "/health" {
```

- 例3-2の `timing` と同じ形です。`/health`（`routes/health.rs` の `#[route(GET)]`）だけはログに出さないように、パスで分けています。公開したあとは、Fly.io が数十秒ごとに `/health` を確かめに来るので、全部書くとログが埋まってしまうからです（第12巻 UNIT 06（準備中））。
- `root_layout` は、`<head>` に CSS へのリンクを入れ、`<body>` の中に、ヘッダー（`site_header`）と、`error_boundary`（ページで起きたエラーを、エラーの画面に変える部品。UNIT 10）で包んだ `(slot)` を置いています。

### 練習問題

**問1:** 例3-1のアプリに、「お問い合わせ」のページ（`/contact`）を足し、メニューにもリンクを足してください（エラー1の正しい直し方です）。

**問2:** 例3-1のアプリで、`/about/`（最後に `/` が付いている）と、`/blog_posts`（`-` ではなく `_`）を `curl.exe -i` で開くと、それぞれどうなるでしょうか。

**問3:** 例3-1の `/api/health` に、`curl.exe -I`（HEAD）と `curl.exe -i -X POST`（POST）で要求を送ると、どうなるでしょうか。`#[route(GET)]` には、GET しか書いていません。

### 解答と解説

**問1の解答:**

`mod contact` を作り、その中に `#[page]` の関数を書き、メニューに `href!(contact::contact_page)` のリンクを足します（`src/bin/u03_q1.rs`。足した所だけを抜き出しています）。

```rust
                    <a href=(href!(blog_posts::list))>"記事"</a>
                    <a href=(href!(contact::contact_page))>"お問い合わせ"</a>
```

```rust
mod contact {
    use super::*;

    /// GET /contact（足したページ）
    #[page]
    pub async fn contact_page() -> Result<impl View> {
        Ok(view! { <h1>"お問い合わせ"</h1> })
    }
}
```

```powershell
cargo run --bin u03_q1
```

```text
### curl.exe -i http://127.0.0.1:3000/contact
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
vary: Accept
vary: accept-encoding
content-length: 311
date: Fri, 30 Oct 2026 11:37:53 GMT

<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><title>UNIT 03 の練習</title></head><body><nav><a href="/">トップ</a><a href="/about" class="nav">この場所について</a><a href="/blog-posts">記事</a><a href="/contact">お問い合わせ</a></nav><h1>お問い合わせ</h1></body></html>
```

- `main` も `module_router!()` も変えずに、`/contact` が 200 になり、メニューにリンクが増えました。モジュールを1つ足すと、URL が1つ増えます。

**問2の解答:**

```text
### curl.exe -i http://127.0.0.1:3000/about/
HTTP/1.1 308 Permanent Redirect
location: /about
content-length: 0
date: Fri, 30 Oct 2026 11:37:49 GMT



### curl.exe -i http://127.0.0.1:3000/blog_posts
HTTP/1.1 404 Not Found
content-type: text/plain; charset=utf-8
content-length: 9
date: Fri, 30 Oct 2026 11:37:49 GMT

not found
```

- `/about/` は `308 Permanent Redirect` で、`location: /about` でした。Topcoat は、最後の `/` があるかないかだけが違う URL を、**書いたとおりの形**（ここでは `/` なし）へ移します。`308` は「ずっと、こちらに移りました。同じメソッドで送り直してください」という意味です（UNIT 06 の 303 と違い、POST は POST のまま送り直されます）。ブラウザは、自動で `/about` を開き直します。
- `/blog_posts` は 404 です。URL は `/blog-posts` だけで、`_` の形は用意されません。リンクを `href!` で作っていれば、この取り違えは起きません。

**問3の解答:**

```text
### curl.exe -I http://127.0.0.1:3000/api/health
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 2
date: Fri, 30 Oct 2026 11:37:49 GMT

### curl.exe -i -X POST http://127.0.0.1:3000/api/health
HTTP/1.1 405 Method Not Allowed
allow: GET, HEAD
content-length: 0
date: Fri, 30 Oct 2026 11:37:49 GMT
```

- HEAD は 200 で、GET と同じヘッダー（`content-length: 2`）で本文がありません。`GET` を書いたルートには、Topcoat が HEAD にも自動で答えます。
- POST は `405 Method Not Allowed` と `allow: GET, HEAD` でした。UNIT 02 の `#[page]` と同じく、書いていないメソッドは、Topcoat が 405 で断ってくれます。

### まとめ

- `module_router!()` を呼んだモジュールが `/` になり、`mod` が1つ深くなるたびに URL の区切りが1つ増える。関数の名前は URL に関係なく、モジュール名の `_` は `-` になる。
- 画面は `#[page]`（レイアウトに包まれる）、文字や JSON は `#[route(GET)]`（そのまま応答になる）。リンクは `href!` で関数の名前から作り、無いページへのリンクをコンパイルで見つける。
- レイアウトは広い範囲が外側、狭い範囲が内側に入れ子になり、`(slot)` の所にページが入る。レイヤーは処理を前と後ろではさむ。同じ URL・同じメソッドのページが2つあると、起動したときに panic で止まる。

## UNIT 04: URL の部品（path・query）と Cx

### この単元のゴール

- `/notes/7` の `7` のような**パスの部品**を、`module_param!` と `path_param::<Id>(cx)` で、型の付いた値として読める
- `?q=rust&page=2` のような**クエリ**を、`#[query_params]` を付けた構造体と `query_params::<T>(cx)` で読み、無い・空のときに `None` になることを説明できる
- 読めなかったときの返事を `error = not_found`（404）・`error = bad_request`（400）で選び、自分で作った型（1 以上の数だけの `NoteId`）で読める
- `cx: &Cx` から、メソッド・URL・ヘッダーを読む関数を書き、型の取り違え（E0308）・`Option` の付け忘れ・`view!` に入れられない型（E0277）を読める

### たとえ話でつかむ

宅配便の伝票を考えます。

- 伝票には、**住所**を書く欄があります。「東京都 / 港区 / 7番地」のように、上から順に、決まった並びで書きます。この並びが違えば、別の家に届きます。
- 住所とは別に、**備考欄**があります。「午前中に届けて」「置き配でよい」のように、書いても書かなくてもよいことを、名前と中身の組で書きます。
- 受付の人は、伝票を読むとき、「7番地」の「7」を**数として**読みます。「七」や「ABC」と書いてあったら、「そんな番地はありません」と返します。
- 備考欄の「午前中」が空欄なら、「指定なし」として扱います。
- 受付の人の机には、**伝票そのもの**が置いてあります。配達の係も、仕分けの係も、その伝票を見に来れば、必要なことが読めます。

| 宅配便 | Topcoat の要求 |
|---|---|
| 住所の欄（決まった並び） | URL の**パス**（`/notes/7`）。`module_param!` で、区切りを1つ `{id}` にする |
| 「7番地」を数として読む | `path_param::<Id>(cx)`（`u64` や `NoteId` に変える） |
| 「そんな番地はありません」 | `error = not_found`（読めなければ 404） |
| 備考欄（名前と中身の組） | URL の**クエリ**（`?q=rust&page=2`）。`#[query_params]` の構造体で読む |
| 空欄なら「指定なし」 | `Option<T>` の項目。無い・空なら `None` |
| 机の上の伝票 | `cx: &Cx`（今の要求の中身。どの関数からでも読める） |

> **用語:** パスパラメーター（path parameter：URL のパスの中で、変わる部分。`/notes/7` の `7`。Topcoat では `{id}` のように書く）

<!-- -->

> **用語:** クエリ文字列（query string：URL の `?` より後ろの、`名前=中身` を `&` でつないだ部分。検索の言葉やページの番号に使う）

<!-- -->

> **用語:** Cx（コンテキスト：今の要求の中身と、アプリで共有する値をまとめて持っている入れ物。`cx: &Cx` を引数にすると、Topcoat が渡してくれる）

### 図で理解する

URL のどの部分を、どう読むかです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0401" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="680" height="44" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="40" y="48" font-size="14" fill="#475569" font-family="Consolas, monospace">http://127.0.0.1:3000</text>
<text x="246" y="48" font-size="14" fill="#1e40af" font-weight="700" font-family="Consolas, monospace">/notes/7</text>
<text x="330" y="48" font-size="14" fill="#c2410c" font-weight="700" font-family="Consolas, monospace">?q=rust&amp;page=2</text>
<rect x="40" y="100" width="300" height="150" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="190" y="124" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">パス（住所）</text>
<text x="60" y="150" font-size="12" fill="#0f172a" font-family="Consolas, monospace">mod notes { mod id {</text>
<text x="60" y="170" font-size="12" fill="#0f172a" font-family="Consolas, monospace">  module_param!(id: u64,</text>
<text x="60" y="190" font-size="12" fill="#0f172a" font-family="Consolas, monospace">    error = not_found);</text>
<text x="60" y="214" font-size="12" fill="#0f172a" font-family="Consolas, monospace">path_param::&lt;Id&gt;(cx)?</text>
<text x="190" y="238" font-size="11" fill="#1e40af" text-anchor="middle">→ &amp;u64 の 7（読めなければ 404）</text>
<rect x="380" y="100" width="300" height="150" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="530" y="124" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">クエリ（備考欄）</text>
<text x="400" y="150" font-size="12" fill="#0f172a" font-family="Consolas, monospace">#[query_params(error = bad_request)]</text>
<text x="400" y="170" font-size="12" fill="#0f172a" font-family="Consolas, monospace">struct NotesQuery {</text>
<text x="400" y="190" font-size="12" fill="#0f172a" font-family="Consolas, monospace">  q: Option&lt;String&gt;, page: Option&lt;u32&gt; }</text>
<text x="400" y="214" font-size="12" fill="#0f172a" font-family="Consolas, monospace">query_params::&lt;NotesQuery&gt;(cx)?</text>
<text x="530" y="238" font-size="11" fill="#c2410c" text-anchor="middle">→ q = Some("rust")・page = Some(2)</text>
<line x1="280" y1="64" x2="200" y2="96" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0401)"/>
<line x1="420" y1="64" x2="510" y2="96" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0401)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-9: URL の部品の読み方。パスの区切りは module_param! で型を決めて path_param で、? より後ろは #[query_params] の構造体と query_params で読む</div>
</div>

- パスの部品は、**モジュールの並び**で場所が決まります。`mod notes` の中の `mod id` に `module_param!(id: u64, …)` を書くと、そのモジュールの区切りが `{id}` になり、URL は `/notes/{id}` になります（UNIT 03 のモジュールルーティングの続きです）。
- クエリは、URL の場所には関係ありません。構造体の項目の名前（`q`・`page`）が、`?` より後ろの名前と合うものを読みます。

読めたとき・読めなかったときの返事です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 210" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="120" y="24" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">URL</text>
<text x="360" y="24" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">読んだ結果</text>
<text x="600" y="24" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">返事</text>
<rect x="20" y="36" width="200" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="120" y="58" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">/notes/7</text>
<rect x="260" y="36" width="200" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="58" font-size="12" fill="#047857" text-anchor="middle">Ok(&amp;7)</text>
<rect x="500" y="36" width="200" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="600" y="58" font-size="12" fill="#047857" text-anchor="middle">ページを動かす → 200</text>
<rect x="20" y="82" width="200" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="120" y="104" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">/notes/abc</text>
<rect x="260" y="82" width="200" height="34" rx="6" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="360" y="104" font-size="12" fill="#b91c1c" text-anchor="middle">Err（数ではない）</text>
<rect x="500" y="82" width="200" height="34" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="600" y="104" font-size="12" fill="#c2410c" text-anchor="middle">error = not_found → 404</text>
<rect x="20" y="128" width="200" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="120" y="150" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">/notes?page=abc</text>
<rect x="260" y="128" width="200" height="34" rx="6" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="360" y="150" font-size="12" fill="#b91c1c" text-anchor="middle">Err（page が数ではない）</text>
<rect x="500" y="128" width="200" height="34" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="600" y="150" font-size="12" fill="#c2410c" text-anchor="middle">error = bad_request → 400</text>
<rect x="20" y="174" width="200" height="34" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="120" y="196" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">/notes?page=</text>
<rect x="260" y="174" width="200" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="196" font-size="12" fill="#047857" text-anchor="middle">page = None（空は「無い」）</text>
<rect x="500" y="174" width="200" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="600" y="196" font-size="12" fill="#047857" text-anchor="middle">1 ページ目として 200</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-10: 読んだ結果と返事。読めなかったときは、宣言に書いた error = … の返事になり、ページの関数は動かない。Option の項目は、無いときも空のときも None になる</div>
</div>

### まず動かす最小コード

#### 例4-1: パス・クエリ・要求の中身を読む

> **▼ このコードがやること（先に日本語で）:** 4つのページを作ります。`/notes` は、クエリの `q`（検索の言葉）と `page`（ページの番号）を読んで表示します。`/notes/{id}` は、パスの `{id}` を `u64` として読みます。`/users/{name}` は、パスの `{name}` を文字のまま読みます。`/whoami` は、`cx` から、メソッド・URL・`User-Agent` ヘッダー（どんなプログラムから来たか）を読んで表示します。

```rust
//! 第6巻 UNIT 04: URL の部品（パスの {id}・クエリの ?q=&page=）と、要求の中身を Cx から読む。
//! 実行: cargo run --bin u04_params

use topcoat::{
    Result,
    context::Cx,
    router::{
        module_router, page, query_params,
        request::{headers, method, uri},
    },
    view::{View, view},
};

/// 要求の中身を、1行の文字にする（&Cx があれば、どこからでも読める）
fn request_summary(cx: &Cx) -> String {
    let user_agent = headers(cx)
        .get("user-agent")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("（なし）");
    format!("{} {} / User-Agent: {user_agent}", method(cx), uri(cx))
}

/// GET /whoami: 要求の中身を見せる
mod whoami {
    use super::*;

    #[page]
    async fn whoami(cx: &Cx) -> Result<impl View> {
        Ok(view! { <p>(request_summary(cx))</p> })
    }
}

mod notes {
    use super::*;

    /// URL のクエリ（?q=...&page=...）。読めなければ 400
    #[query_params(error = bad_request)]
    struct NotesQuery {
        q: Option<String>,
        page: Option<u32>,
    }

    /// GET /notes
    #[page]
    async fn list(cx: &Cx) -> Result<impl View> {
        let query = query_params::<NotesQuery>(cx)?;
        let keyword = query.q.clone().unwrap_or_else(|| "（なし）".to_string());
        let page = query.page.unwrap_or(1);
        Ok(view! {
            <p>"検索: " (keyword) " / ページ: " (page)</p>
        })
    }

    mod id {
        use topcoat::{
            Result,
            context::Cx,
            router::{module_param, page, path_param},
            view::{View, view},
        };

        // このモジュールの URL の区切りを {id} にし、u64 として読む。読めなければ 404
        module_param!(id: u64, error = not_found);

        /// GET /notes/{id}
        #[page]
        async fn show(cx: &Cx) -> Result<impl View> {
            let id = path_param::<Id>(cx)?;
            Ok(view! { <h1>"ノート " (id) " 番"</h1> })
        }
    }
}

mod users {
    mod name {
        use topcoat::{
            Result,
            context::Cx,
            router::{module_param, page, path_param},
            view::{View, view},
        };

        // 型を書かないと、区切りの文字（%XX を戻したもの）が &str で読める
        module_param!(name);

        /// GET /users/{name}
        #[page]
        async fn profile(cx: &Cx) -> Result<impl View> {
            let name: &str = path_param::<Name>(cx);
            Ok(view! { <h1>(name) " さんのページ"</h1> })
        }
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
cargo run --bin u04_params
```

2つ目の窓から、4つの URL を開きます（URL に `&` が入るので、`"…"` で囲みます。`%E5%A4%AA%E9%83%8E` は「太郎」を UTF-8 で書いたものです）。

```powershell
curl.exe -i "http://127.0.0.1:3000/notes"
curl.exe -i "http://127.0.0.1:3000/notes?q=rust&page=2"
curl.exe -i "http://127.0.0.1:3000/notes/7"
curl.exe -i "http://127.0.0.1:3000/users/%E5%A4%AA%E9%83%8E"
```

**▼ 実行結果**（date と vary の行は省いています）

```text
### curl.exe -i "http://127.0.0.1:3000/notes"
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 42

<p>検索: （なし） / ページ: 1</p>


### curl.exe -i "http://127.0.0.1:3000/notes?q=rust&page=2"
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 34

<p>検索: rust / ページ: 2</p>


### curl.exe -i "http://127.0.0.1:3000/notes/7"
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 24

<h1>ノート 7 番</h1>


### curl.exe -i "http://127.0.0.1:3000/users/%E5%A4%AA%E9%83%8E"
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 34

<h1>太郎 さんのページ</h1>
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: module_param! は、区切りを {id} にして、型を決める

- `mod notes` の中の `mod id` に `module_param!(id: u64, error = not_found)` と書きました。これで、`mod id` の区切りは `id` ではなく `{id}` になり、URL は `/notes/{id}` です。`/notes/7` の `7` が、`{id}` に当たります。
- `module_param!` は、`id` から `Id` という型も作ります（名前の頭を大文字にした形）。`path_param::<Id>(cx)` と書くと、「`Id` の部品を読んで」という意味になり、`u64` に変えた値（`&u64`）が返ります。
- `error = not_found` があるので、読めなかったとき、`path_param::<Id>(cx)?` の `?` で、404 の返事になります（例4-1の次の「読めなかったとき」で確かめます）。

##### 解説2: 型を書かなければ、文字のまま読める

- `/users/{name}` の `module_param!(name)` には、型を書いていません。このときは、区切りの文字が `&str` のまま返ります。数に変えないので、失敗することもありません（`error = …` も書けません）。
- `%E5%A4%AA%E9%83%8E` は、`%` の後ろの2文字ずつが1バイトを表す書き方（パーセントエンコーディング）で、「太郎」の UTF-8 の6バイトです。Topcoat は、これを元に戻してから渡すので、ページでは「太郎」として読めました。

##### 解説3: #[query_params] は、構造体に ? より後ろを読み込む

- `#[query_params(error = bad_request)]` を付けた `NotesQuery` は、`q` と `page` の2つの項目を持ちます。`query_params::<NotesQuery>(cx)?` で、`?q=rust&page=2` が `q = Some("rust")`・`page = Some(2)` になりました。
- `/notes`（クエリが無い）では、どちらも `None` で、`unwrap_or_else`・`unwrap_or(1)` の値（「（なし）」と 1）が使われました。**クエリは、いつも付いているとは限らない**ので、項目は `Option<T>` にします（エラー1）。
- クエリの構造体は、URL の場所とは関係ありません。`&Cx` を持っている関数なら、どこからでも同じ構造体を読めます。1つの要求の中では、最初の1回だけ読み込んで、あとは覚えておいた結果を返します。

##### 解説4: &Cx があれば、要求の中身を読める

- `request_summary(cx)` は、ページではない、ふつうの関数です。`cx: &Cx` を受け取れば、`method(cx)`（`GET` など）・`uri(cx)`（`/whoami` など）・`headers(cx)`（ヘッダーの一覧）を読めます。

```powershell
curl.exe -i -A "RustNote-test/1.0" http://127.0.0.1:3000/whoami
```

```text
### curl.exe -i -A "RustNote-test/1.0" http://127.0.0.1:3000/whoami
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 50

<p>GET /whoami / User-Agent: RustNote-test/1.0</p>
```

- `-A` で、curl が送る `User-Agent` を変えました。ページの関数ではなく、そこから呼んだ `request_summary` が、`cx` から読んでいます。**`cx` を渡せば、どの関数でも要求の中身を読める**ので、「要求の中身を1か所で用意して、全部の関数に配る」ような仕組み（ミドルウェア）を作らなくて済みます（UNIT 07）。

#### ▼ 読めなかったときの返事

```powershell
curl.exe -i "http://127.0.0.1:3000/notes?page=abc"
curl.exe -i "http://127.0.0.1:3000/notes?page="
curl.exe -i "http://127.0.0.1:3000/notes/abc"
```

```text
### curl.exe -i "http://127.0.0.1:3000/notes?page=abc"
HTTP/1.1 400 Bad Request
content-type: text/plain; charset=utf-8
content-length: 75

bad request: invalid query value: invalid digit found in string (at `page`)


### curl.exe -i "http://127.0.0.1:3000/notes?page="
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 42

<p>検索: （なし） / ページ: 1</p>


### curl.exe -i "http://127.0.0.1:3000/notes/abc"
HTTP/1.1 404 Not Found
content-type: text/plain; charset=utf-8
content-length: 9

not found
```

- `?page=abc` は 400 で、本文は ``bad request: invalid query value: invalid digit found in string (at `page`)`` でした。「クエリの値が不正です。文字列に数字ではない文字があります（`page` の所で）」という意味で、**どの項目が読めなかったか**まで教えてくれます。
- `?page=`（空）は、エラーにならず、1 ページ目でした。ブラウザのフォームは、空の入力欄も `page=` として送ってくるので、Topcoat は「空」を `None`（無い）として扱います。
- `/notes/abc` は 404 です。`abc` は `u64` に変えられないので、`error = not_found` の返事になりました。ページの関数は動いていません。

### もう一歩: 実務に近い形

#### 例4-2: 自分で作った型で読む（1 以上の数だけの NoteId）

`u64` では、`/notes/0` も読めてしまいます。ノートの ID は 1 から始まるので、0 は「無いノート」です。**1 以上の数しか作れない型**を作り、その型で読みます。RustNote の `NoteId` と同じ考え方です（第4巻 UNIT 01）。

> **▼ このコードがやること（先に日本語で）:** `NoteId` という型を作り、文字から作る `FromStr`（数でなければ `NotANumber`、1 未満なら `NotPositive` のエラー）と、表示の `Display` を書きます。`module_param!(id: NoteId, error = not_found)` で、URL の `{id}` を `NoteId` として読みます。

```rust
//! 第6巻 UNIT 04 の例4-2: URL の {id} を、自分で作った型（1 以上の数だけ）で読む。
//! 実行: cargo run --bin u04_noteid

use std::fmt;
use std::str::FromStr;

use topcoat::{
    Result,
    context::Cx,
    router::{module_router, page, path_param},
    view::{View, view},
};

/// ノートの ID。0 や負の数は作れない
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoteId(i64);

/// ID を作れなかった理由
#[derive(Debug)]
pub enum IdError {
    NotANumber,
    NotPositive,
}

// "42".parse::<NoteId>() と書けるようにする。module_param! もこれを使う
impl FromStr for NoteId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: i64 = s.parse().map_err(|_| IdError::NotANumber)?;
        if value <= 0 {
            return Err(IdError::NotPositive);
        }
        Ok(NoteId(value))
    }
}

// 画面に出すときと、href! で URL を作るときに使う
impl fmt::Display for NoteId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

mod notes {
    mod id {
        use super::super::*;
        use topcoat::router::module_param;

        // 区切りの文字を NoteId::from_str で読む。読めなければ 404
        module_param!(id: NoteId, error = not_found);

        /// GET /notes/{id}
        #[page]
        async fn show(cx: &Cx) -> Result<impl View> {
            let id: &NoteId = path_param::<Id>(cx)?;
            // view! に入れられるのは文字や数などなので、to_string() で文字にする
            Ok(view! { <h1>"ノート " (id.to_string()) " 番"</h1> })
        }
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
cargo run --bin u04_noteid
```

```text
### curl.exe -i http://127.0.0.1:3000/notes/7
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 24

<h1>ノート 7 番</h1>


### curl.exe -i http://127.0.0.1:3000/notes/0
HTTP/1.1 404 Not Found
content-type: text/plain; charset=utf-8
content-length: 9

not found


### curl.exe -i http://127.0.0.1:3000/notes/-5
HTTP/1.1 404 Not Found
content-type: text/plain; charset=utf-8
content-length: 9

not found


### curl.exe -i http://127.0.0.1:3000/notes/abc
HTTP/1.1 404 Not Found
content-type: text/plain; charset=utf-8
content-length: 9

not found
```

- `/notes/7` は 200、`/notes/0`・`/notes/-5`・`/notes/abc` は、どれも 404 になりました。`module_param!` は、型に書いてある `FromStr` を使って文字を変えるので、**「1 以上」という決まりを、型に書いておくだけで**、URL の読み取りにも効きます。
- ページの関数の中では、`id` は必ず 1 以上の `NoteId` です。関数の中で「0 だったら…」と確かめる必要はありません。
- 画面に出すときは `id.to_string()` にしています。`view!` に `(id)` とそのまま入れるとエラーになるからです（エラー3）。

### よくあるエラーと読み方

#### エラー1: クエリの項目を Option にし忘れた

`page: Option<u32>` を、`page: u32` と書いてしまった場合です（`errs/e04_required.rs`）。コンパイルは通ります。

```rust
//! クエリの項目を Option にし忘れた（page: u32）
use topcoat::{
    Result,
    context::Cx,
    router::{module_router, page, query_params},
    view::{View, view},
};

mod notes {
    use super::*;

    #[query_params(error = bad_request)]
    struct NotesQuery {
        page: u32, // Option<u32> にし忘れた
    }

    #[page]
    async fn list(cx: &Cx) -> Result<impl View> {
        let query = query_params::<NotesQuery>(cx)?;
        Ok(view! { <p>"ページ: " (query.page)</p> })
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```text
### curl.exe -i "http://127.0.0.1:3000/notes"
HTTP/1.1 400 Bad Request
content-type: text/plain; charset=utf-8
content-length: 63

bad request: invalid query value: missing field `page` (at `.`)


### curl.exe -i "http://127.0.0.1:3000/notes?page=2"
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 19

<p>ページ: 2</p>
```

- **何が起きたか:** `?page=2` を付けたときは読めましたが、付けないで `/notes` を開くと、400 になりました。``missing field `page` ``（`page` の項目がありません）です。`Option` ではない項目は、「必ず付いている」という決まりになるからです。
- **直し方:** 付いていなくてもよい項目は、`Option<u32>` にして、ページの関数で `unwrap_or(1)` のように決まった値を使います。
- **学べること:** 一覧のページは、ふつう、クエリを付けずに開きます（メニューのリンクなど）。クエリの項目は、ほぼ全部 `Option` にすると考えてかまいません。

#### エラー2: path_param の結果を、そのまま u64 として受け取った（E0308）

`error = …` を書かない `module_param!(id: u64)` で、結果を `u64` の変数に入れようとした場合です（`errs/e04_type.rs`。抜き出しています）。

```rust
        module_param!(id: u64); // error = ... を書いていない

        #[page]
        async fn show(cx: &Cx) -> Result<impl View> {
            let id: u64 = path_param::<Id>(cx);
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error[E0308]: mismatched types
  --> src\bin\e04_type.rs:17:27
   |
17 |             let id: u64 = path_param::<Id>(cx);
   |                     ---   ^^^^^^^^^^^^^^^^^^^^ expected `u64`, found `Result<&u64, &ParseIntError>`
   |                     |
   |                     expected due to this
   |
   = note: expected type `u64`
              found enum `std::result::Result<&u64, &ParseIntError>`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `web_lab` (bin "e04_type") due to 1 previous error
```

- **何が起きたか:** 「型が合いません。`u64` のはずが、`Result<&u64, &ParseIntError>` です」というエラーです。`path_param` は、**読めなかったかもしれない**ので、`Result` を返します。中身は `u64` そのものではなく、借りた `&u64` です。
- **直し方:** 読めなかったときの返事を宣言で決める（`module_param!(id: u64, error = not_found)` にして `path_param::<Id>(cx)?`）か、呼んだ所で決めます（`path_param::<Id>(cx).ok_or_not_found()?`。`RouterErrorExt` を `use` します）。値が `u64` で欲しいときは、`*` を付けて `let id: u64 = *path_param::<Id>(cx)?;` と書きます（RustNote の `show` も `*` を付けています）。

#### エラー3: 自分で作った型を、view! にそのまま入れた（E0277）

例4-2で、`(id.to_string())` ではなく、`(id)` と書いた場合です（`errs/e04_display.rs`）。

```rust
            Ok(view! { <h1>"ノート " (id) " 番"</h1> })
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error[E0277]: the trait bound `&NoteId: topcoat::view::internal::NodeClassify` is not satisfied
  --> src\bin\e04_display.rs:57:36
   |
57 |             Ok(view! { <h1>"ノート " (id) " 番"</h1> })
   |                -----------------------^^--------------
   |                |                      |
   |                |                      the trait `NodeViewParts` is not implemented for `&NoteId`
   |                required by a bound introduced by this call
   |
   = help: the following other types implement trait `NodeViewParts`:
             &&'b T
             &bool
             &char
             &f32
             &f64
             &i128
             &i16
             &i32
           and 56 others
   = note: required for `&NoteId` to implement `topcoat::view::internal::NodeClassify`

```

（このあとに、同じ形の `E0277` がもう1つ続きます）

- **何が起きたか:** 「`&NoteId` に、`NodeViewParts` という trait が実装されていません」というエラーです。`view!` の `( )` に入れられるのは、文字（`&str`・`String`）・数（`i32`・`u64` など）・`bool` など、Topcoat が**画面に出し方を知っている型**だけです（`help:` に、入れられる型の一部が並んでいます）。`NoteId` に `Display` を書いても、それだけでは入れられません。
- **直し方:** `(id.to_string())` で文字にしてから入れます。数を入れたいなら `(id.get())` のように、中の数を取り出してもかまいません。
- **学べること:** 「どう表示するか」を `view!` が勝手に決めないので、HTML に入る文字は、いつも自分で選んだものになります。エスケープ（危ない文字の書き換え）の話は UNIT 05 で扱います。

### RustNote ではここで使う

ノートの一覧（`src/routes/notes.rs`）は、クエリを次の構造体で読んでいます（抜き出しています）。

```rust
/// URL のクエリ（?q=...&page=...）。page が数字でなければ 400
#[query_params(error = bad_request)]
pub struct NotesParams {
    q: Option<String>,
    page: Option<u32>,
}

#[page]
pub async fn index(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let params = query_params::<NotesParams>(cx)?;
    let query = NoteQuery::new(params.q.as_deref(), params.page);
```

- 例4-1の `NotesQuery` と同じ形です。読んだ値を、そのまま SQL に使うのではなく、`NoteQuery::new` で、「前後の空白を取る」「空白だけなら検索しない」「ページは 1 以上」と整えてから使います（第10巻 UNIT 01）。
- 最初に `require_user(cx)` でログインを確かめてから、クエリを読んでいます。どちらも `cx` から読むので、順番は自分で決められます。

ノートの詳細（`src/routes/notes/id.rs`）は、`{id}` を `NoteId` で読んでいます（抜き出しています）。

```rust
// URL の {id} を NoteId に変換する。`Id` という型ができ、path_param::<Id>(cx) で読める。
// 変換に失敗したら NotFoundError（404）になる。
module_param!(pub id: NoteId, error = not_found);

#[page]
pub async fn show(cx: &Cx) -> Result<impl View> {
    let user = require_user(cx).await?;
    let id = *path_param::<Id>(cx)?;
```

- 例4-2と同じく、`NoteId` の `FromStr`（`src/domain.rs`）が、数でない・1 未満の ID を断ります。`pub` が付いているのは、ほかのファイル（編集・完了・削除のページ）からも `Id` を使うからです。`/notes/{id}/edit` は `notes/id/edit.rs` で、親のモジュールの `Id` を読みます。
- 他人のノートの ID を開いても、無い ID と同じ 404 にしています（第8巻 UNIT 01・第10巻 UNIT 03）。

### 練習問題

**問1:** 例4-1のアプリで、`curl.exe -i "http://127.0.0.1:3000/notes?q=a+b%26c"` を開くと、「検索:」のあとに何が出るでしょうか。`+` と `%26` は、それぞれ何に変わるでしょうか。

**問2:** 例4-1のアプリで、`/users/a%2Fb`・`/users/a/b`・`/notes/99999999999999999999` を開くと、それぞれどうなるでしょうか。

**問3:** 例4-2の `module_param!` の `error = not_found` を `error = bad_request` に変えると、`/notes/0` と `/notes/abc` の返事はどう変わるでしょうか。どちらを使うのがよいか、理由も考えてください。

### 解答と解説

**問1の解答:**

```text
### curl.exe -i "http://127.0.0.1:3000/notes?q=a+b%26c"
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 39

<p>検索: a b&amp;c / ページ: 1</p>
```

- `+` は空白に、`%26` は `&` に戻り、検索の言葉は `a b&c` になりました。クエリの中では、空白を `+` で、`&`（項目の区切りに使う文字）を `%26` で書く決まりだからです。`&` をそのまま書くと、`c` が別の項目の名前として読まれてしまいます。
- HTML の中では `a b&amp;c` になっています。`&` は HTML で特別な意味を持つので、`view!` が `&amp;` に書き換えました（エスケープ。UNIT 05）。ブラウザには `a b&c` と表示されます。

**問2の解答:**

```text
### curl.exe -i "http://127.0.0.1:3000/users/a%2Fb"
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 31

<h1>a/b さんのページ</h1>


### curl.exe -i "http://127.0.0.1:3000/users/a/b"
HTTP/1.1 404 Not Found
content-type: text/plain; charset=utf-8
content-length: 9

not found


### curl.exe -i "http://127.0.0.1:3000/notes/99999999999999999999"
HTTP/1.1 404 Not Found
content-type: text/plain; charset=utf-8
content-length: 9

not found
```

- `/users/a%2Fb` は、1つの区切りの中の `%2F` が `/` に戻り、名前は `a/b` になりました。区切りの `/` とは別物として扱われます。
- `/users/a/b` は、区切りが1つ多いので、`/users/{name}` に当たらず、404 です。
- `/notes/99999999999999999999` は、`u64` に入りきらない大きな数なので、変えられずに 404 でした。「数字だけ」でも読めるとは限りません。型に任せておけば、こうした端の値も断ってくれます（第11巻 UNIT 01 の境界値）。

**問3の解答:**

`src/bin/u04_q3.rs` として、`error = bad_request` に変えたものを保存しました。

```text
### curl.exe -i "http://127.0.0.1:3000/notes/0"
HTTP/1.1 400 Bad Request
content-type: text/plain; charset=utf-8
content-length: 50

bad request: invalid value for path parameter "id"


### curl.exe -i "http://127.0.0.1:3000/notes/abc"
HTTP/1.1 400 Bad Request
content-type: text/plain; charset=utf-8
content-length: 50

bad request: invalid value for path parameter "id"
```

- どちらも 400 になり、本文は `bad request: invalid value for path parameter "id"`（パスの `id` の値が不正です）でした。
- RustNote は、**404 を選んでいます**。`/notes/0` や `/notes/abc` は「そんなノートは無い」と同じ意味で、無い ID・他人の ID と同じ返事にそろえておけば、外から「この ID の形は正しい・正しくない」の違いを探られにくくなります。クエリの `?page=abc` は、ページ自体はあるので、400（送られてきた内容がおかしい）が合います。

### まとめ

- パスの部品は `module_param!(id: 型, error = …)` で区切りを `{id}` にして、`path_param::<Id>(cx)?` で読む。型に `FromStr` を書けば、その決まり（1 以上など）が URL の読み取りにも効く。
- クエリは `#[query_params]` の構造体で読む。項目は `Option<T>` にし、無い・空は `None`。読めなかったときは、宣言の `error = …` の返事（404・400 など）になり、ページの関数は動かない。
- `cx: &Cx` があれば、どの関数からでも、メソッド・URL・ヘッダー・パス・クエリを読める。`view!` に入れられるのは文字や数などで、自分の型は `to_string()` などで変えてから入れる。

## UNIT 05: view!・コンポーネント・HTML のエスケープ

### この単元のゴール

- `view!` の中で、文字（`"…"`）・式（`( )`）・属性・`if`・`for`・`match` を使って、HTML を組み立てられる
- `#[component]` で画面を部品に分け、名前付きの引数と、中身（`child`）を渡して呼び出せる
- 利用者が書いた文字が、**置く場所ごと**にどうエスケープされるか（本文・属性の値・属性の名前）を、実際の HTML で確かめられる
- `"…"` の付け忘れ・閉じタグの書き忘れ・`view!` に入れた値の移動（E0382）のエラーを読める

### たとえ話でつかむ

お弁当屋さんの、詰め合わせの作業を考えます。

- お弁当には、**決まった仕切り**（ご飯の場所・おかずの場所）があります。仕切りの形は、いつも同じです。これが HTML のタグです。
- 仕切りの中には、**その日の材料**を入れます。材料は日によって変わります。これが `( )` に入れる式です。
- 「卵焼き」「唐揚げ」の小分けのカップは、**何度も使う部品**です。カップの作り方を一度決めておけば、どのお弁当にも同じように入れられます。これがコンポーネントです。
- お客さんから持ち込まれた材料は、そのまま入れずに、**入れる場所に合わせて包み直します**。汁物の仕切りに入れるなら汁がこぼれないように、揚げ物の仕切りに入れるなら油がしみないように。これがエスケープです。
- 「この材料で、仕切りそのものを作って」と頼まれたら断ります。仕切りが壊れると、お弁当の形が保てないからです。

| お弁当屋さん | Topcoat の view! |
|---|---|
| 決まった仕切り | HTML のタグ（`<ul>`・`<li>` など） |
| その日の材料 | `( )` に入れた式（`(title)`・`(count)`） |
| 小分けのカップ | `#[component]`（何度も使う画面の部品） |
| 入れる場所に合わせて包み直す | エスケープ（本文では `<` を `&lt;`、属性の値では `"` を `&quot;` に） |
| 仕切りそのものは作らせない | 属性の名前・タグの名前に危ない文字が来たら、panic で止める |

> **用語:** コンポーネント（component：画面の一部を作る、何度も使える部品。Topcoat では `#[component]` を付けた async 関数）

<!-- -->

> **用語:** エスケープ（escape：HTML で特別な意味を持つ文字を、ただの文字として表示される書き方に置き換えること。`<` → `&lt;` など）

<!-- -->

> **用語:** XSS（クロスサイトスクリプティング：利用者が書いた文字にプログラム（`<script>` など）を混ぜて、ほかの人の画面で動かす攻撃）

### 図で理解する

`view!` の中で使う書き方の一覧です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="20" width="330" height="46" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="36" y="40" font-size="12" fill="#0f172a" font-family="Consolas, monospace">&lt;h1&gt;"ノート"&lt;/h1&gt;</text>
<text x="36" y="58" font-size="11" fill="#475569">決まった文字は "…" で囲む</text>
<rect x="370" y="20" width="330" height="46" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="386" y="40" font-size="12" fill="#0f172a" font-family="Consolas, monospace">&lt;p&gt;(title)&lt;/p&gt;</text>
<text x="386" y="58" font-size="11" fill="#475569">変わる値は ( ) に入れる（エスケープされる）</text>
<rect x="20" y="78" width="330" height="46" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="36" y="98" font-size="12" fill="#0f172a" font-family="Consolas, monospace">data-tag=(tag)</text>
<text x="36" y="116" font-size="11" fill="#475569">None なら、属性そのものを書かない</text>
<rect x="370" y="78" width="330" height="46" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="386" y="98" font-size="12" fill="#0f172a" font-family="Consolas, monospace">disabled=(is_pending)</text>
<text x="386" y="116" font-size="11" fill="#475569">true なら disabled=""、false なら書かない</text>
<rect x="20" y="136" width="330" height="46" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="36" y="156" font-size="12" fill="#0f172a" font-family="Consolas, monospace">if done { " ✓" }</text>
<text x="36" y="174" font-size="11" fill="#475569">if・else・match で、出す部品を選ぶ</text>
<rect x="370" y="136" width="330" height="46" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="386" y="156" font-size="12" fill="#0f172a" font-family="Consolas, monospace">#[key(note.id)] for note in notes { … }</text>
<text x="386" y="174" font-size="11" fill="#475569">for で並べる。key は1件ずつを見分ける印</text>
<rect x="20" y="194" width="680" height="80" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="36" y="216" font-size="12" fill="#0f172a" font-family="Consolas, monospace">panel(title: "一覧", &lt;ul&gt; … &lt;/ul&gt;)</text>
<text x="36" y="236" font-size="11" fill="#475569">コンポーネントは、名前付きの引数で呼ぶ。引数のあとに書いた部品は、child として渡る</text>
<text x="36" y="258" font-size="12" fill="#0f172a" font-family="Consolas, monospace">#[component] async fn panel(title: &amp;str, #[default] child: Child&lt;'_&gt;)</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-11: view! の書き方。HTML とほぼ同じ形で書き、決まった文字は "…"、変わる値は ( ) に入れる。属性の値に Option や bool を入れると、無いときは属性ごと消える</div>
</div>

利用者が書いた文字を、置く場所ごとに、どう書き換えるかです（Topcoat 0.11 の `topcoat-view` の `escape.rs` に書いてある決まりです）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="150" y="26" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">置く場所</text>
<text x="330" y="26" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">&lt;</text>
<text x="400" y="26" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">&gt;</text>
<text x="470" y="26" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">&amp;</text>
<text x="540" y="26" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">"</text>
<text x="630" y="26" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">理由</text>
<rect x="20" y="38" width="680" height="50" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="150" y="60" font-size="12" fill="#0f172a" text-anchor="middle">本文（タグの間）</text>
<text x="150" y="78" font-size="11" fill="#475569" text-anchor="middle" font-family="Consolas, monospace">&lt;p&gt;(title)&lt;/p&gt;</text>
<text x="330" y="68" font-size="12" fill="#b91c1c" text-anchor="middle" font-family="Consolas, monospace">&amp;lt;</text>
<text x="400" y="68" font-size="12" fill="#b91c1c" text-anchor="middle" font-family="Consolas, monospace">&amp;gt;</text>
<text x="470" y="68" font-size="12" fill="#b91c1c" text-anchor="middle" font-family="Consolas, monospace">&amp;amp;</text>
<text x="540" y="68" font-size="12" fill="#475569" text-anchor="middle">そのまま</text>
<text x="630" y="68" font-size="11" fill="#475569" text-anchor="middle">&lt; はタグの始まり</text>
<rect x="20" y="98" width="680" height="50" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="150" y="120" font-size="12" fill="#0f172a" text-anchor="middle">属性の値（"…" の中）</text>
<text x="150" y="138" font-size="11" fill="#475569" text-anchor="middle" font-family="Consolas, monospace">title=(title)</text>
<text x="330" y="128" font-size="12" fill="#475569" text-anchor="middle">そのまま</text>
<text x="400" y="128" font-size="12" fill="#475569" text-anchor="middle">そのまま</text>
<text x="470" y="128" font-size="12" fill="#b91c1c" text-anchor="middle" font-family="Consolas, monospace">&amp;amp;</text>
<text x="540" y="128" font-size="12" fill="#b91c1c" text-anchor="middle" font-family="Consolas, monospace">&amp;quot;</text>
<text x="630" y="128" font-size="11" fill="#475569" text-anchor="middle">" で値が終わる</text>
<rect x="20" y="158" width="680" height="56" rx="6" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="150" y="180" font-size="12" fill="#0f172a" text-anchor="middle">属性の名前・タグの名前</text>
<text x="150" y="198" font-size="11" fill="#475569" text-anchor="middle" font-family="Consolas, monospace">&lt;p (attr)="1"&gt;</text>
<text x="470" y="186" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">書き換えずに、panic で止める（空白・= ・&lt; ・&gt; ・" ・' ・/ など）</text>
<text x="470" y="204" font-size="11" fill="#475569" text-anchor="middle">名前は書き換えても元に戻らないので、危ない文字が来たら作らない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-12: 置く場所ごとのエスケープ。本文ではタグが始まらないように &lt; と &gt; を、属性の値では値が終わらないように " を書き換える。名前の場所には危ない文字を入れさせない</div>
</div>

### まず動かす最小コード

#### 例5-1: 部品に分けて、一覧の画面を作る

> **▼ このコードがやること（先に日本語で）:** ノート1件を表示する部品 `note_card`（完了なら `done` のクラスと「✓」を付け、タグがあれば `data-tag` 属性を付ける）と、見出しと中身を枠で囲む部品 `panel` を作ります。トップページでは、2件のノートを `for` で並べ、全部が完了かどうかを `match` で分けて表示し、最後に `checked` と `disabled` を式で書いたチェックボックスを置きます。

```rust
//! 第6巻 UNIT 05: view! の書き方（式・属性・if・for・match）と、コンポーネントで画面を部品に分ける。
//! 実行: cargo run --bin u05_view

use topcoat::{
    Result,
    router::{module_router, page},
    view::{Child, View, class, component, view},
};

/// 一覧に並べるノート（本物のアプリでは DB から読む）
struct Note {
    id: u32,
    title: &'static str,
    done: bool,
    tag: Option<&'static str>,
}

/// ノート1件を表示する部品
#[component]
async fn note_card(title: &str, done: bool, tag: Option<&str>) -> Result<impl View> {
    Ok(view! {
        <li class=(class!("note", "done" if done)) data-tag=(tag)>
            (title)
            if done {
                " ✓"
            }
        </li>
    })
}

/// 見出しと中身を枠で囲む部品（child に、呼んだ側の中身が入る）
#[component]
async fn panel(title: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <section class="panel">
            <h2>(title)</h2>
            (child)
        </section>
    })
}

/// GET /
#[page]
async fn home() -> Result<impl View> {
    let notes = vec![
        Note {
            id: 1,
            title: "牛乳を買う",
            done: true,
            tag: Some("買い物"),
        },
        Note {
            id: 2,
            title: "本を返す",
            done: false,
            tag: None,
        },
    ];
    let count = notes.len();
    let status = if notes.iter().all(|n| n.done) {
        "all"
    } else {
        "some"
    };

    Ok(view! {
        <h1>"ノート（" (count) " 件）"</h1>
        panel(
            title: "一覧",
            <ul>
                #[key(note.id)]
                for note in notes {
                    note_card(title: note.title, done: note.done, tag: note.tag)
                }
            </ul>
        )
        match status {
            "all" => <p>"すべて完了しました"</p>,
            _ => <p>"まだ残っています"</p>,
        }
        <input type="checkbox" checked=(count > 1) disabled=(false)>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
cargo run --bin u05_view
curl.exe -s http://127.0.0.1:3000/
```

**▼ 実行結果**

```text
### curl.exe -s "http://127.0.0.1:3000/"
<h1>ノート（2 件）</h1><section class="panel"><h2>一覧</h2><ul><li class="note done" data-tag="買い物">牛乳を買う ✓</li><li class="note">本を返す</li></ul></section><p>まだ残っています</p><input type="checkbox" checked="">
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 文字は "…"、変わる値は ( )

- `<h1>"ノート（" (count) " 件）"</h1>` は、決まった文字を `"…"` で囲み、変わる値（`count`）を `( )` に入れています。結果は `<h1>ノート（2 件）</h1>` です。`"…"` で囲むのは、Rust のコードと HTML の文字を見分けるためです（エラー1）。
- `view!` の中の空白や改行は、HTML には出ません。字下げは、読みやすさのために自由に入れられます。

##### 解説2: 属性に Option・bool を入れると、無いときは書かない

- 1件目は `data-tag="買い物"` が付き、2件目（`tag: None`）には `data-tag` そのものがありません。属性の値の式が `None` のときは、**属性ごと書かない**決まりです。
- `checked=(count > 1)` は `true` なので `checked=""`、`disabled=(false)` は `false` なので、書かれていません。`checked`・`disabled` のような「あるかないか」だけで意味が決まる属性（ブール属性）は、`bool` の式で書きます（問2）。
- `class=(class!("note", "done" if done))` は、`done` が `true` のときだけ `done` を足して、`class="note done"` を作りました。UNIT 03 のメニューでも使いました。

##### 解説3: for・if・match は、Rust と同じ形

- `for note in notes { note_card(…) }` で、ノートの数だけ `<li>` を作りました。`#[key(note.id)]` は、1件ずつを見分ける印です。並び順が変わっても、同じノートを同じ部品として扱えるようにします（ブラウザの中で動く部分を使うときに大事になります。UNIT 09）。
- `if done { " ✓" }` は、完了のときだけ「✓」を足しました。`match status { "all" => …, _ => … }` は、2件とも完了ではないので、`_` の側（「まだ残っています」）になりました。

##### 解説4: コンポーネントは、名前付きの引数で呼ぶ

- `note_card(title: note.title, done: note.done, tag: note.tag)` のように、**引数の名前を書いて**呼びます。順番を間違えても、名前で合うので安心です。
- `panel(title: "一覧", <ul>…</ul>)` は、名前付きの引数のあとに、`<ul>…</ul>` を書きました。これは `panel` の `child` という引数に入り、`panel` の中の `(child)` の所に置かれました。`#[default]` が付いているので、中身なしで `panel(title: "空")` と呼ぶこともできます。
- コンポーネントは async 関数なので、中で DB を読むこともできます。RustNote の `notes_index` は、コンポーネントの中でノートを検索しています（第10巻 UNIT 01）。

### もう一歩: 実務に近い形

#### 例5-2: 利用者が書いた危ない文字を、置く場所ごとに確かめる

利用者がノートのタイトルに、`<script>alert("x")</script> & 'quote'` と書いた、と考えます。これを、本文と、属性の値の両方に置いてみます。さらに、利用者の文字から**属性の名前**を作ってしまう、してはいけない例も試します。

> **▼ このコードがやること（先に日本語で）:** トップページでは、危ない文字を含むタイトルを、`<p>` の本文と、`<a>` の `title` 属性の値に置きます。`/name?attr=…` のページでは、クエリの `attr` の文字を、そのまま `<p>` の属性の**名前**に使います。

```rust
//! 第6巻 UNIT 05 の例5-2: 利用者が書いた文字を、置く場所ごとにどうエスケープするかを確かめる。
//! 実行: cargo run --bin u05_escape

use topcoat::{
    Result,
    router::{module_router, page},
    view::{View, view},
};

/// GET /: 危ない文字を含むタイトルを、本文と属性の両方に置く
#[page]
async fn home() -> Result<impl View> {
    // 利用者がフォームに書いた、と考える文字
    let title = r#"<script>alert("x")</script> & 'quote'"#;
    Ok(view! {
        <p>(title)</p>
        <a href="/notes" title=(title)>"一覧"</a>
    })
}

mod name {
    use topcoat::{
        Result,
        context::Cx,
        router::{page, query_params},
        view::{View, view},
    };

    #[query_params]
    struct NameQuery {
        attr: Option<String>,
    }

    /// GET /name?attr=...: 属性の「名前」を、利用者の文字から作ってしまう（してはいけない例）
    #[page]
    async fn attribute_name(cx: &Cx) -> Result<impl View> {
        let attr = query_params::<NameQuery>(cx)
            .ok()
            .and_then(|q| q.attr.clone())
            .unwrap_or_else(|| "data-x".to_string());
        Ok(view! { <p (attr)="1">"属性の名前を試す"</p> })
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
cargo run --bin u05_escape
curl.exe -s http://127.0.0.1:3000/
```

```text
### curl.exe -s "http://127.0.0.1:3000/"
<p>&lt;script&gt;alert("x")&lt;/script&gt; &amp; 'quote'</p><a href="/notes" title="<script>alert(&quot;x&quot;)</script> &amp; 'quote'">一覧</a>
```

- **本文**（`<p>` の中）では、`<` が `&lt;`、`>` が `&gt;`、`&` が `&amp;` になりました。ブラウザは、`<script>` を**ただの文字**として表示し、プログラムとしては動かしません。`"` と `'` は、本文では特別な意味が無いので、そのままです。
- **属性の値**（`title="…"` の中）では、`"` が `&quot;`、`&` が `&amp;` になりました。`"` をそのまま書くと、そこで値が終わり、うしろに好きな属性（`onclick=…` など）を書き足されてしまうからです。`"…"` の中では `<` はタグの始まりにならないので、書き換えていません。第10巻 UNIT 02 の問3で見たのと同じ動きです。

次に、属性の名前を試します。

```powershell
curl.exe -i "http://127.0.0.1:3000/name?attr=data-ok"
curl.exe -i "http://127.0.0.1:3000/name?attr=onclick%3Dalert(1)%20x"
```

```text
### curl.exe -i "http://127.0.0.1:3000/name?attr=data-ok"
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 43

<p data-ok="1">属性の名前を試す</p>

### curl.exe -i "http://127.0.0.1:3000/name?attr=onclick%3Dalert(1)%20x"
HTTP/1.1 500 Internal Server Error
content-length: 21

internal server error
```

**▼ サーバーの窓の表示**

```text

thread 'tokio-rt-worker' (210720) panicked at ...\renderer.rs:87:39:
invalid attribute key "onclick=alert(1) x": forbidden character '='
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- `data-ok` は、そのまま属性の名前になりました。`onclick=alert(1) x`（`%3D` は `=`、`%20` は空白）は、**500 Internal Server Error** で、サーバーの窓に `invalid attribute key "onclick=alert(1) x": forbidden character '='`（属性の名前が不正です。使えない文字 `=` があります）が出ました。
- 属性の名前やタグの名前は、書き換えても意味が保てないので、Topcoat は**危ない文字を見つけたら、HTML を作らずに止めます**。止めずに書いていたら、`<p onclick=alert(1) x="1">` という、押すとプログラムが動く HTML ができていました。
- そもそも、**利用者の文字から属性やタグの名前を作らない**のが正しい書き方です。名前は、プログラムの中で決めた固定の文字にします。

> **注意:** Topcoat にも、エスケープをしない書き方（`PartsWriter` の `push_str_unescaped` など）があります。利用者の文字には、決して使いません。自分で書いた、固定の HTML だけに使う道具です。

### よくあるエラーと読み方

#### エラー1: 文字を "…" で囲み忘れた

```rust
//! view! の中の文字を、"..." で囲み忘れた
use topcoat::{
    Result,
    router::{module_router, page},
    view::{View, view},
};

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <h1>こんにちは</h1>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router).await.expect("起動できません");
}
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error: expected view node
  --> src\bin\e05_noquote.rs:11:13
   |
11 |         <h1>こんにちは</h1>
   |             ^^^^^^^^^^

error: could not compile `web_lab` (bin "e05_noquote") due to 1 previous error
```

- **何が起きたか:** 「view の部品が来るはずです」というエラーで、`こんにちは` の所を指しています。`view!` の中では、`"…"` で囲んでいない言葉は、タグの名前や Rust のコードとして読もうとします。日本語の文字は、どちらにもならないので、エラーになりました。
- **直し方:** `<h1>"こんにちは"</h1>` と、`"…"` で囲みます。

#### エラー2: 閉じタグを書き忘れた

```rust
//! 閉じタグ </p> を書き忘れた
use topcoat::{
    Result,
    router::{module_router, page},
    view::{View, view},
};

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <section>
            <p>"1つ目の段落"
            <p>"2つ目の段落"</p>
        </section>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router).await.expect("起動できません");
}
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error: closing tag `section` does not match opening tag `p`
  --> src\bin\e05_noclose.rs:14:11
   |
14 |         </section>
   |           ^^^^^^^

error: could not compile `web_lab` (bin "e05_noclose") due to 1 previous error
```

- **何が起きたか:** 「閉じタグ `section` が、開きタグ `p` と合いません」というエラーです。12 行目の `<p>` を閉じていないので、`view!` は、次の `</section>` を「`<p>` を閉じるはずの所」と読みました。エラーが指すのは、書き忘れた所（12 行目）ではなく、**合わないと分かった所**（14 行目）です。
- **直し方:** 12 行目の終わりに `</p>` を足します。ブラウザの HTML は閉じタグを省いても表示されることがありますが、`view!` では、`<br>`・`<input>` のような空の要素のほかは、必ず閉じます。エラーの指す行より**上**を探すのが近道です。

#### エラー3: view! に入れた値を、あとでもう一度使った（E0382）

```rust
//! view! に入れた String を、あとでもう一度使おうとした
use topcoat::{
    Result,
    router::{module_router, page},
    view::{View, view},
};

#[page]
async fn home() -> Result<impl View> {
    let title = String::from("買い物メモ");
    let header = view! { <h1>(title)</h1> };
    Ok(view! {
        (header)
        <p>"タイトルは " (title.len()) " バイトです"</p>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router).await.expect("起動できません");
}
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error[E0382]: use of moved value: `title`
  --> src\bin\e05_moved.rs:12:8
   |
10 |       let title = String::from("買い物メモ");
   |           ----- move occurs because `title` has type `std::string::String`, which does not implement the `Copy` trait
11 |       let header = view! { <h1>(title)</h1> };
   |                    --------------------------
   |                    |            |
   |                    |            variable moved due to use in coroutine
   |                    value moved here
12 |       Ok(view! {
   |  ________^
13 | |         (header)
14 | |         <p>"タイトルは " (title.len()) " バイトです"</p>
   | |                           ----- use occurs due to use in coroutine
15 | |     })
   | |_____^ value used here after move
   |
   = note: this error originates in the macro `view` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `web_lab` (bin "e05_moved") due to 1 previous error
```

- **何が起きたか:** 「移動した値 `title` を使いました」というエラーです。`view! { <h1>(title)</h1> }` は、`async move` のブロックのように、中で使う値を**持ち主ごと受け取ります**（`variable moved due to use in coroutine`）。`title` は `header` の中に移ったので、あとの `title.len()` では使えません（第3巻 UNIT 02）。
- **直し方:** あとでも使う値は、先に必要なものを取り出しておきます。たとえば `let len = title.len();` を `header` を作る**前**に書いて、`(len)` を使います。どちらにも `String` が要るときは、`title.clone()` を `view!` に入れます。

### RustNote ではここで使う

RustNote の画面の部品は、`src/views.rs` にまとめてあります。入力欄の下にエラーを出す部品です（抜き出しています）。

```rust
/// 入力欄の下に出すエラー文（無ければ何も出さない）
#[component]
pub async fn field_error(message: Option<String>) -> Result<impl View> {
    Ok(view! {
        if let Some(message) = message {
            <p class="field-error">(message)</p>
        }
    })
}
```

- `message` が `None` なら、何も出しません。作成と編集の2つのフォームの、タイトルと本文の4か所で使っています。同じ形のエラー表示を1か所で決められるのが、コンポーネントの良さです（第10巻 UNIT 02）。

一覧の1行は、`src/routes/notes.rs` の `notes_index` の中で、`for` で並べています（抜き出しています）。

```rust
            <ul class="note-list">
                #[key(note.id.get())]
                for note in page.notes.clone() {
                    <li class=(class!("note-item", "done" if note.done))>
                        <a href=(href!(id::show, id::Id(note.id)))>(note.title)</a>
```

- 例5-1と同じく、`#[key]` で1件ずつを見分け、`class!` で完了のクラスを足しています。`key` には、`NoteId` の中の数（`note.id.get()`）を使っています。
- `(note.title)` は、利用者が書いたタイトルです。`view!` が本文の場所に合わせてエスケープするので、タイトルに `<script>` と書かれても、ただの文字として表示されます（例5-2）。

エラーの画面を作る `error_page` は、コンポーネントではない、ふつうの関数です。そのため、`view!` の最初に `cx =>` と書いて、文脈（`Cx`）を渡しています（抜き出しています）。

```rust
    // 普通の関数の中で view! を使うときは、先頭に `cx =>` を書いて文脈を渡す
    Ok(view! {
        cx =>
        (status)
```

- コンポーネント（`#[component]`）や `#[page]` の中では、`cx` は自動で渡ります。ふつうの関数の中では、`cx =>` を書かないと、`view!` が文脈を見つけられずに、``cannot find value `__cx` in this scope``（`__cx` という値が見つかりません）というエラーになります。

### 練習問題

**問1:** 例5-1の `note_card` に、タグがあるときだけ `<span class="tag">タグ</span>` を足してください。`data-tag` 属性とは別に、画面に見える形で出します。

**問2:** 次の3つのボタンは、それぞれどんな HTML になるでしょうか。`maybe_title` は `None`、`is_disabled` と `expanded` は `false` とします。

```rust
<button title=(maybe_title) disabled=(is_disabled)>"A"</button>
<button disabled="false">"B"</button>
<button aria-expanded=(if expanded { "true" } else { "false" })>"C"</button>
```

**問3:** UNIT 04 のエラー3では、自分で作った `NoteId` を `view!` に `(id)` と入れて、`E0277` になりました。`NoteId` に `NodeViewParts` を実装して、`(id)` と書けるようにしてください。

### 解答と解説

**問1の解答:**

`if let Some(tag) = tag { … }` で、タグがあるときだけ中身を取り出して表示します（`src/bin/u05_q1.rs`。変えた所だけを抜き出しています）。

```rust
            // タグがあるときだけ、中身を取り出して表示する
            if let Some(tag) = tag {
                <span class="tag">(tag)</span>
            }
```

```text
### curl.exe -s "http://127.0.0.1:3000/"
<h1>ノート（2 件）</h1><section class="panel"><h2>一覧</h2><ul><li class="note done" data-tag="買い物">牛乳を買う ✓<span class="tag">買い物</span></li><li class="note">本を返す</li></ul></section><p>まだ残っています</p><input type="checkbox" checked="">
```

- 1件目にだけ `<span class="tag">買い物</span>` が付きました。`view!` の `if` には、Rust と同じく `if let` も書けます。

**問2の解答:**

```text
### curl.exe -s "http://127.0.0.1:3000/"
<button>A</button><button disabled="false">B</button><button aria-expanded="false">C</button>
```

- A は、`title` も `disabled` も書かれていません。`None` と `false` の式は、属性ごと消えます。
- B は `disabled="false"` と書かれ、**ボタンは押せない**状態になります。`disabled` はブール属性なので、値が `"false"` でも、**あるだけで** `true` の意味になるからです。文字で書いた属性は、そのまま出るので、「押せるかどうか」を変えたいときは、必ず `disabled=(bool の式)` で書きます。
- C は `aria-expanded="false"` です。`aria-expanded` は、`"true"` と `"false"` という**文字**で意味が決まる属性なので、`bool` ではなく文字を渡します（`bool` の `false` を渡すと、属性そのものが消えて、「開いているかどうか分からない」という別の意味になってしまいます）。

**問3の解答:**

`NodeViewParts` の `into_view_parts` で、画面に出す文字を `push_string` で渡します（`src/bin/u05_q3.rs`）。

```rust
//! 問3の解答: 自分の型（NoteId）に NodeViewParts を実装して、view! に (id) と書けるようにする。
//! 実行: cargo run --bin u05_q3

use topcoat::{
    Result,
    context::Cx,
    router::{module_router, page},
    view::{NodeViewParts, PartsWriter, View, view},
};

/// ノートの ID
struct NoteId(i64);

// view! の ( ) に、子の部品として置けるようにする
impl NodeViewParts for NoteId {
    fn into_view_parts(self, _cx: &Cx, parts: &mut PartsWriter<'_>) {
        // push_string で入れた文字は、置く場所に合わせて自動でエスケープされる
        parts.push_string(format!("No.{}", self.0));
    }
}

#[page]
async fn home() -> Result<impl View> {
    let id = NoteId(7);
    Ok(view! { <h1>"ノート " (id)</h1> })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```text
### curl.exe -s "http://127.0.0.1:3000/"
<h1>ノート No.7</h1>
```

- `(id)` と書けるようになり、`No.7` と表示されました。表示の形（`No.` を付ける）を、型の側で1か所に決められます。
- `push_string` で入れた文字は、置いた場所に合わせて、Topcoat がエスケープしてくれます。名前に `_unescaped` が付いた方法（エスケープしない）は使いません。
- RustNote は、`NoteId` を画面に出すことがほとんど無く（URL の中だけ）、`NodeViewParts` は実装していません。画面に何度も出す型があるときに、この方法を使います。

### まとめ

- `view!` は HTML とほぼ同じ形で書き、決まった文字は `"…"`、変わる値は `( )`。属性に `Option`・`bool` の式を入れると、`None`・`false` のときは属性ごと消える。`if`・`for`（`#[key]` 付き）・`match`・`let` も使える。
- `#[component]` で部品を作り、名前付きの引数で呼ぶ。引数のあとに書いた部品は `child` に入る。ふつうの関数の中の `view!` には `cx =>` が要る。
- `( )` に入れた値は、置く場所に合わせてエスケープされる（本文は `<`・`>`・`&`、属性の値は `"`・`&`）。属性やタグの名前に危ない文字が来ると、HTML を作らずに panic で止まる。利用者の文字で名前を作らない。

## UNIT 06: フォーム・JSON・PRG

### この単元のゴール

- HTML のフォームから送られてきた値を、`Form<T>` で構造体として受け取り、成功したら `see_other` で 303 に移り（PRG）、失敗したら 400 で入力を残して出し直せる
- JSON を受け取る・返す API を、`#[route(POST)]` と `Json<T>` で作り、`(StatusCode::CREATED, Json(…))` のように状態コードを付けて返せる
- 形の違う本文・足りない項目・大きすぎる本文が、ページの関数より**前に** 400・413 で断られることを確かめられる
- `Deserialize` の付け忘れ・本文の引数を2つ書いたとき・303 を `Ok` で返したときのエラーを読める

### たとえ話でつかむ

役所の窓口に届く、2種類の書類を考えます。

- 1つは、**窓口で書く申込用紙**です。決まった欄（「氏名」「住所」）があり、人が手で書いて出します。これが HTML のフォームです。
- もう1つは、**ほかの役所から届く、決まった形式の電子データ**です。機械が読み書きするので、項目の名前も形も、きっちり決まっています。これが JSON です。
- 窓口の人は、書類を**受け付ける前に**、形を確かめます。欄が足りない、形式が違う、厚すぎて受け取れない、という書類は、担当者に回さずに、その場で返します。
- 申込用紙を受け付けたら、「控えは3番の窓口で」と案内して、別の窓口に移ってもらいます（UNIT 02 の PRG）。電子データには、「受け付けました。番号は 1 番です」と、**データで**返事をします。

| 役所の窓口 | Topcoat |
|---|---|
| 窓口で書く申込用紙 | HTML のフォーム（`Content-Type: application/x-www-form-urlencoded`） |
| 決まった形式の電子データ | JSON（`Content-Type: application/json`） |
| 受け付ける前に形を確かめる | `Form<T>`・`Json<T>`（形が違えば 400、大きすぎれば 413） |
| 「控えは3番の窓口で」 | `see_other(…)`（303 で GET の画面へ移す。PRG） |
| 「受け付けました。番号は 1 番」 | `(StatusCode::CREATED, Json(memo))`（201 と JSON の返事） |

> **用語:** JSON（ジェイソン：`{"title":"milk"}` のように、名前と値の組を `{}` で、並びを `[]` で書く、データの書き方。プログラムどうしのやり取りに使う）

<!-- -->

> **用語:** API（エーピーアイ：プログラムから使うための窓口。ここでは、HTML の画面ではなく、JSON を返す URL のこと）

<!-- -->

> **用語:** 201 Created（「新しく作りました」という返事。作ったものを本文で返すことが多い）

### 図で理解する

本文を受け取ってから、関数が動くまでです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0601" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="30" width="170" height="80" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="105" y="54" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">POST の本文</text>
<text x="105" y="76" font-size="11" fill="#475569" text-anchor="middle" font-family="Consolas, monospace">title=milk</text>
<text x="105" y="96" font-size="11" fill="#475569" text-anchor="middle" font-family="Consolas, monospace">{"title":"milk"}</text>
<rect x="240" y="20" width="220" height="100" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="350" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">Form&lt;MemoForm&gt; / Json&lt;NewMemo&gt;</text>
<text x="350" y="66" font-size="11" fill="#0f172a" text-anchor="middle">① Content-Type を確かめる</text>
<text x="350" y="86" font-size="11" fill="#0f172a" text-anchor="middle">② 大きさを確かめる（既定 2 MiB まで）</text>
<text x="350" y="106" font-size="11" fill="#0f172a" text-anchor="middle">③ 構造体に変える（serde）</text>
<rect x="510" y="30" width="190" height="80" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="605" y="56" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">関数が動く</text>
<text x="605" y="78" font-size="11" fill="#0f172a" text-anchor="middle">input.title を使って</text>
<text x="605" y="96" font-size="11" fill="#0f172a" text-anchor="middle">中身を確かめる・保存する</text>
<line x1="190" y1="70" x2="236" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0601)"/>
<line x1="460" y1="70" x2="506" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0601)"/>
<rect x="240" y="160" width="220" height="80" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="350" y="184" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">関数は動かない</text>
<text x="350" y="206" font-size="11" fill="#0f172a" text-anchor="middle">形が違う・項目が無い → 400</text>
<text x="350" y="226" font-size="11" fill="#0f172a" text-anchor="middle">大きすぎる → 413</text>
<line x1="350" y1="120" x2="350" y2="156" stroke="#dc2626" stroke-width="2" marker-end="url(#ar-06-0601)"/>
<text x="560" y="180" font-size="11" fill="#475569" text-anchor="middle">形は合っていても、中身の決まり</text>
<text x="560" y="198" font-size="11" fill="#475569" text-anchor="middle">（空のタイトルなど）は、</text>
<text x="560" y="216" font-size="11" fill="#475569" text-anchor="middle">関数の中で確かめる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-13: 本文の受け取り。Form・Json は、関数が動く前に、種類・大きさ・形を確かめ、合わなければ 400 や 413 で断る。空のタイトルのような中身の決まりは、関数の中で確かめる</div>
</div>

- `Form<T>`・`Json<T>` は、関数の**引数**に書きます。Topcoat は、関数を呼ぶ前に本文を読み、構造体 `T` に変えてから渡します。変えられなければ、関数は呼ばれません。
- 本文の「形」（項目があるか、数か文字か）は `Form`・`Json` が、本文の「中身の決まり」（空でないか、80 文字までか）は関数が確かめます。2段に分けて確かめるのは、第10巻 UNIT 02 の `NoteForm` と `NewNote::parse` と同じです。

画面とデータで、返し方が違います。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 200" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="20" width="330" height="165" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="185" y="44" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">人が見る画面（#[page(POST)]）</text>
<text x="40" y="72" font-size="12" fill="#0f172a">成功: Err(see_other(URL).into())</text>
<text x="56" y="92" font-size="11" fill="#475569">→ 303・location。ブラウザが GET で開き直す</text>
<text x="40" y="122" font-size="12" fill="#0f172a">失敗: Ok(view! { (StatusCode::BAD_REQUEST) … })</text>
<text x="56" y="142" font-size="11" fill="#475569">→ 400 と、入力を残したフォーム（HTML）</text>
<text x="40" y="170" font-size="11" fill="#475569">返すのは、いつも HTML か、移り先</text>
<rect x="370" y="20" width="330" height="165" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="535" y="44" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">プログラムが読む API（#[route(POST)]）</text>
<text x="390" y="72" font-size="12" fill="#0f172a">成功: Ok((StatusCode::CREATED, Json(memo)))</text>
<text x="406" y="92" font-size="11" fill="#475569">→ 201 と、作ったものの JSON</text>
<text x="390" y="122" font-size="12" fill="#0f172a">一覧: Ok(Json(memos))</text>
<text x="406" y="142" font-size="11" fill="#475569">→ 200 と、JSON の配列</text>
<text x="390" y="170" font-size="11" fill="#475569">移らずに、結果をデータで返す</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-14: 返し方の違い。画面は、成功したら 303 で GET の画面へ移し（PRG）、失敗したら 400 で出し直す。API は、状態コードと JSON をそのまま返す</div>
</div>

### まず動かす最小コード

#### 例6-1: フォームを受け取り、303 で移る

> **▼ このコードがやること（先に日本語で）:** メモの一覧と、タイトルを書くフォームの画面を、部品 `memo_page` で作ります。`GET /` はその画面を返します。`POST /` は、フォームの `title` を `Form<MemoForm>` で受け取り、前後の空白を取って空なら 400 で出し直し、空でなければメモに足して、303 で `GET /` へ移ります。メモは、プログラムが動いている間だけ、`static` の `Mutex<Vec<String>>` に覚えておきます（本物のアプリでは DB に保存します）。

```rust
//! 第6巻 UNIT 06: フォームを受け取り、成功したら 303 で移る（PRG）。失敗したら 400 で出し直す。
//! 実行: cargo run --bin u06_form

use std::sync::Mutex;

use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{StatusCode, content::Form, error::see_other, href, module_router, page},
    view::{View, component, view},
};

/// 保存したメモ（この例では、プログラムが動いている間だけ覚えておく）
static MEMOS: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// フォームから送られてくる値（name="title" の入力欄と対応する）
#[derive(Deserialize)]
struct MemoForm {
    title: String,
}

/// 一覧とフォームの画面（部品）。error があれば、入力欄の下に出す
#[component]
async fn memo_page(title: String, error: Option<&'static str>) -> Result<impl View> {
    let memos = MEMOS.lock().unwrap().clone();
    Ok(view! {
        <h1>"メモ"</h1>
        <form method="post" action="/">
            <input name="title" value=(title)>
            if let Some(error) = error {
                <p class="error">(error)</p>
            }
            <button type="submit">"保存"</button>
        </form>
        <ul>
            for memo in memos {
                <li>(memo)</li>
            }
        </ul>
    })
}

/// GET /
#[page]
async fn home() -> Result<impl View> {
    Ok(view! { memo_page(title: String::new(), error: None) })
}

/// POST /（同じモジュールなので同じ URL。メソッドが違うので取り合わない）
#[page(POST)]
async fn create(cx: &Cx, Form(input): Form<MemoForm>) -> Result<impl View> {
    let title = input.title.trim();
    if title.is_empty() {
        // 失敗: 400 で、入力を残したまま出し直す
        return Ok(view! {
            (StatusCode::BAD_REQUEST)
            memo_page(title: input.title.clone(), error: Some("タイトルを入力してください"))
        });
    }
    MEMOS.lock().unwrap().push(title.to_string());
    // 成功: 303 で一覧（GET /）へ移る
    Err(see_other(href!(home).resolve(cx)).into())
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

この例から、構造体を JSON やフォームと変換する `serde` を使うので、`web_lab` に足します。

```powershell
cargo add serde@=1.0.229 --features derive
cargo run --bin u06_form
```

2つ目の窓から、フォームと同じ形の本文を、`-d` で POST します（`%E8%B2%B7%E3%81%84%E7%89%A9` は「買い物」）。

```powershell
curl.exe -i -d "title=%E8%B2%B7%E3%81%84%E7%89%A9" http://127.0.0.1:3000/
curl.exe -s http://127.0.0.1:3000/
```

**▼ 実行結果**（date と vary の行は省いています）

```text
### curl.exe -i -d "title=%E8%B2%B7%E3%81%84%E7%89%A9" http://127.0.0.1:3000/
HTTP/1.1 303 See Other
location: /
content-length: 0


### curl.exe -s http://127.0.0.1:3000/
<h1>メモ</h1><form method="post" action="/"><input name="title" value=""><button type="submit">保存</button></form><ul><li>買い物</li></ul>
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: Form<T> は、本文を構造体にして渡す

- `curl.exe -d "title=…"` は、ブラウザのフォームと同じ形（`Content-Type: application/x-www-form-urlencoded`）で本文を送ります。`create` の引数の `Form(input): Form<MemoForm>` が、本文の `title=…` を `MemoForm { title: "買い物" }` に変えて渡しました。
- `MemoForm` に付けた `#[derive(Deserialize)]` が、「本文の `title` を、構造体の `title` に入れる」変換を作ります（エラー1）。HTML の `<input name="title">` の `name` と、構造体の項目名をそろえます（第10巻 UNIT 02 のエラー2）。

##### 解説2: 成功したら、Err(see_other(…)) で 303

- 成功したときの返事は `303 See Other` と `location: /` だけで、本文はありません。そのあとの `GET /` で、「買い物」が一覧に入っていました。
- `see_other(…)` は、Topcoat の `router::error` にある**エラーの形の返事**です。`Err(… .into())` で返すと、画面を作るのをやめて、303 の返事になります。画面（`impl View`）を返す関数の中で、「画面ではなく移り先を返す」ために、エラーの道を使うのです（エラー3）。
- 移り先の URL は、`href!(home).resolve(cx)` で、ページの関数の名前から作っています（UNIT 03）。

##### 解説3: 同じ URL の GET と POST

- `home` は `#[page]`（GET）、`create` は `#[page(POST)]` です。同じモジュールなので、どちらも `/` ですが、メソッドが違うので取り合いません（UNIT 03 のエラー1）。フォームの `action="/"` と `method="post"` が、`create` に届きます。

#### ▼ 失敗したとき（空白だけのタイトル）

```powershell
curl.exe -i -d "title=++" http://127.0.0.1:3000/
```

```text
### curl.exe -i -d "title=++" http://127.0.0.1:3000/
HTTP/1.1 400 Bad Request
content-type: text/html; charset=utf-8
content-length: 208

<h1>メモ</h1><form method="post" action="/"><input name="title" value="  "><p class="error">タイトルを入力してください</p><button type="submit">保存</button></form><ul><li>買い物</li></ul>
```

- `++` は、フォームの書き方で空白2つです。前後の空白を取ると空なので、`400 Bad Request` と、エラーの文の付いたフォームが返りました。入力欄の `value="  "` は、送られたまま残しています。
- 400 のときは、`view!` の最初に `(StatusCode::BAD_REQUEST)` を置いて、状態コードを決めています（UNIT 05）。ここでは移らずに、同じ画面を返します。

### もう一歩: 実務に近い形

#### 例6-2: JSON を受け取り、JSON で返す API

人ではなくプログラムが使う窓口（API）を作ります。

> **▼ このコードがやること（先に日本語で）:** `GET /api/memos` で、保存したメモを JSON の配列で返します。`POST /api/memos` で、`{"title":"…"}` の JSON を `Json<NewMemo>` で受け取り、番号を付けてメモに足し、`201 Created` と、足したメモの JSON を返します。

```rust
//! 第6巻 UNIT 06 の例6-2: JSON を受け取り、JSON で返す API（GET と POST /api/memos）。
//! 実行: cargo run --bin u06_api

use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use topcoat::{
    Result,
    router::{StatusCode, content::Json, module_router},
};

/// 保存したメモ
#[derive(Clone, Serialize)]
struct Memo {
    id: u32,
    title: String,
}

static MEMOS: Mutex<Vec<Memo>> = Mutex::new(Vec::new());

mod api {
    pub mod memos {
        use super::super::*;
        use topcoat::router::route;

        /// 受け取る JSON（{"title": "..."}）
        #[derive(Deserialize)]
        pub struct NewMemo {
            title: String,
        }

        /// GET /api/memos: 全部を JSON の配列で返す
        #[route(GET)]
        async fn list() -> Result<Json<Vec<Memo>>> {
            Ok(Json(MEMOS.lock().unwrap().clone()))
        }

        /// POST /api/memos: 1件足して、201 Created と、足したメモを返す
        #[route(POST)]
        async fn create(Json(input): Json<NewMemo>) -> Result<(StatusCode, Json<Memo>)> {
            let mut memos = MEMOS.lock().unwrap();
            let memo = Memo {
                id: memos.len() as u32 + 1,
                title: input.title,
            };
            memos.push(memo.clone());
            Ok((StatusCode::CREATED, Json(memo)))
        }
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

PowerShell 5.1 では、`"` を含む JSON をコマンドに直接書くと、うまく渡らないことがあります。JSON はファイルに書いて、`--data-binary "@ファイル名"` で送ります（英数字だけにして `-Encoding ascii` で書くと、文字化けもしません）。

```powershell
cargo run --bin u06_api
```

```powershell
Set-Content memo.json '{"title":"milk"}' -Encoding ascii
curl.exe -i http://127.0.0.1:3000/api/memos
curl.exe -i -H "Content-Type: application/json" --data-binary "@memo.json" http://127.0.0.1:3000/api/memos
curl.exe -i http://127.0.0.1:3000/api/memos
```

**▼ 実行結果**

```text
### curl.exe -i http://127.0.0.1:3000/api/memos
HTTP/1.1 200 OK
content-type: application/json
content-length: 2

[]


### curl.exe -i -H "Content-Type: application/json" --data-binary "@memo.json" http://127.0.0.1:3000/api/memos
HTTP/1.1 201 Created
content-type: application/json
content-length: 23

{"id":1,"title":"milk"}


### curl.exe -i http://127.0.0.1:3000/api/memos
HTTP/1.1 200 OK
content-type: application/json
content-length: 25

[{"id":1,"title":"milk"}]
```

- 最初の `GET` は `[]`（空の配列）でした。`Json(…)` で包んで返すと、Topcoat が構造体を JSON に変え、`content-type: application/json` を付けます。`Memo` に付けた `#[derive(Serialize)]` が、構造体から JSON への変換を作ります。
- `POST` は `201 Created` で、本文は `{"id":1,"title":"milk"}` でした。`Ok((StatusCode::CREATED, Json(memo)))` のように、**組（タプル）の最初に状態コード**を置くと、その状態コードで返ります（RustNote の `/health` も同じ形です）。
- `#[route(POST)]` は、画面ではなく、値をそのまま返事にします。レイアウトにも包まれません（UNIT 03）。API では、303 で移らずに、結果をデータで返します。

形の違う JSON を送ってみます（`empty.json` は `{}`、`broken.json` は `{"title":` で終わっているファイルです）。

```text
### curl.exe -i -H "Content-Type: application/json" --data-binary "@empty.json" http://127.0.0.1:3000/api/memos
HTTP/1.1 400 Bad Request
content-type: text/plain; charset=utf-8
content-length: 82

bad request: invalid JSON value: missing field `title` at line 1 column 2 (at `.`)


### curl.exe -i -H "Content-Type: application/json" --data-binary "@broken.json" http://127.0.0.1:3000/api/memos
HTTP/1.1 400 Bad Request
content-type: text/plain; charset=utf-8
content-length: 91

bad request: invalid JSON syntax: EOF while parsing a value at line 2 column 0 (at `title`)
```

- `{}` は、``missing field `title` ``（`title` の項目がありません）の 400 です。`broken.json` は、`EOF while parsing a value`（値を読んでいる途中で終わりました）の 400 です。どちらも、**関数が動く前に**、`Json<NewMemo>` が断りました。メッセージには、何行目の何文字目か（`line 1 column 2`）まで出ます。
- この 400 の本文は、送った人（プログラムを書いた人）に向けたものです。人が見る画面では、こうした英語のメッセージをそのまま出さずに、RustNote のように、日本語の画面に変えて出します（UNIT 10）。

### よくあるエラーと読み方

#### エラー1: フォームの構造体に #[derive(Deserialize)] を付け忘れた（E0277）

```rust
//! フォームの構造体に #[derive(Deserialize)] を付け忘れた
use topcoat::{
    Result,
    router::{content::Form, module_router, page},
    view::{View, view},
};

struct MemoForm {
    title: String,
}

#[page(POST)]
async fn create(Form(input): Form<MemoForm>) -> Result<impl View> {
    Ok(view! { <p>(input.title)</p> })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router).await.expect("起動できません");
}
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error[E0277]: the trait bound `MemoForm: serde::de::DeserializeOwned` is not satisfied
  --> src\bin\e06_noderive.rs:13:30
   |
13 | async fn create(Form(input): Form<MemoForm>) -> Result<impl View> {
   |                              ^^^^ unsatisfied trait bound
   |
help: the trait `for<'de> topcoat::internal::serde::Deserialize<'de>` is not implemented for `MemoForm`
  --> src\bin\e06_noderive.rs:8:1
   |
 8 | struct MemoForm {
   | ^^^^^^^^^^^^^^^
   = help: the following other types implement trait `topcoat::internal::serde::Deserialize<'de>`:
             &'a [u8]
             &'a std::path::Path
             &'a str
             ()
             (T,)
             (T0, T1)
             (T0, T1, T2)
             (T0, T1, T2, T3)
           and 185 others
   = note: required for `MemoForm` to implement `topcoat::internal::serde::de::DeserializeOwned`
   = note: required for `Form<MemoForm>` to implement `FromRequest`

```

（このあとに、同じ所を指す `E0277` がもう1つ続き、その中に ``note: for local types consider adding `#[derive(serde::Deserialize)]` to your `MemoForm` type`` という案内があります）

- **何が起きたか:** 「`MemoForm` に、`DeserializeOwned` という trait が実装されていません」というエラーです。最後の `note:` の ``required for `Form<MemoForm>` to implement `FromRequest` `` は、「`Form<MemoForm>` が本文を読めるためには、`MemoForm` が本文から作れる（`Deserialize`）必要があります」という意味です。
- **直し方:** 2つ目のエラーの `note:` のとおり、`#[derive(serde::Deserialize)]`（`use serde::Deserialize;` してあれば `#[derive(Deserialize)]`）を、`MemoForm` に付けます。JSON で返す構造体には、`Serialize` を付けます。

#### エラー2: 本文の引数を2つ書いた

フォームと JSON の両方を受け取ろうとした場合です（`errs/e06_twobody.rs`。抜き出しています）。

```rust
#[page(POST)]
async fn create(Form(form): Form<MemoForm>, Json(json): Json<MemoForm>) -> Result<impl View> {
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error: page functions cannot take more than one request body parameter
  --> src\bin\e06_twobody.rs:18:45
   |
18 | async fn create(Form(form): Form<MemoForm>, Json(json): Json<MemoForm>) -> Result<impl View> {
   |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `Form`, `Json`, `Result`, `View`, and `view`
 --> src\bin\e06_twobody.rs:4:5
  |
4 |     Result,
  |     ^^^^^^
5 |     router::{
6 |         content::{Form, Json},
  |                   ^^^^  ^^^^
...
9 |     view::{View, view},
  |            ^^^^  ^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `web_lab` (bin "e06_twobody") generated 1 warning
error: could not compile `web_lab` (bin "e06_twobody") due to 1 previous error; 1 warning emitted
```

- **何が起きたか:** 「ページの関数は、本文の引数を2つ以上受け取れません」というエラーです。本文は、届いたものを**1回しか読めない**ので、`Form` と `Json` の両方で読むことはできません。そのあとの `unused imports` の警告は、エラーで関数が作られなかったために出たもので、気にしなくて大丈夫です。
- **直し方:** 1つの URL には、1つの形の本文だけを受け取ります。フォームと JSON の両方を受け付けたいなら、URL を分けます（例6-1の `/` と、例6-2の `/api/memos` のように）。`cx: &Cx` は本文ではないので、本文の引数と一緒に書けます。

#### エラー3: 303 で移るつもりで、Ok(see_other(…)) と書いた（E0308）

```rust
//! 303 で移るつもりで、Ok(see_other(...)) と書いた
use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{content::Form, error::see_other, href, module_router, page},
    view::{View, view},
};

#[derive(Deserialize)]
struct MemoForm {
    title: String,
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <h1>"メモ"</h1> })
}

#[page(POST)]
async fn create(cx: &Cx, Form(input): Form<MemoForm>) -> Result<impl View> {
    if input.title.is_empty() {
        return Ok(view! { <p>"タイトルを入力してください"</p> });
    }
    Ok(see_other(href!(home).resolve(cx)))
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router).await.expect("起動できません");
}
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error[E0308]: mismatched types
  --> src\bin\e06_okredirect.rs:25:8
   |
23 |         return Ok(view! { <p>"タイトルを入力してください"</p> });
   |                   --------------------------------------------- the expected `async` block
24 |     }
25 |     Ok(see_other(href!(home).resolve(cx)))
   |     -- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `ScopeView<MoveView<{async block@...}>>`, found `SeeOther`
   |     |
   |     arguments to this enum variant are incorrect
   |
   = note: expected struct `topcoat::view::internal::ScopeView<topcoat::view::internal::MoveView<{async block@src\bin\e06_okredirect.rs:23:19: 23:51}>>`
              found struct `SeeOther`
help: the type constructed contains `SeeOther` due to the type of the argument passed
  --> src\bin\e06_okredirect.rs:25:5
   |
25 |     Ok(see_other(href!(home).resolve(cx)))
   |     ^^^----------------------------------^
   |        |
   |        this argument influences the type of `Ok`
note: tuple variant defined here
  --> /rustc/58cf8f9c35b355199727627d07ae34a4e2846d67/library\core\src\result.rs:562:4

For more information about this error, try `rustc --explain E0308`.
error: could not compile `web_lab` (bin "e06_okredirect") due to 1 previous error
```

- **何が起きたか:** 「型が合いません」というエラーです。この関数の戻り値は `Result<impl View>` で、23 行目で `Ok(view! { … })` を返しているので、`Ok` の中身は「その `view!` の型」に決まりました。25 行目の `Ok(see_other(…))` の中身は `SeeOther` なので、合わないと言われました。
- **直し方:** `Err(see_other(…).into())` と、**エラーの道**で返します（例6-1）。`#[route(POST)]` で、戻り値を `Result<SeeOther>` と書いた関数なら、`Ok(see_other(…))` で返せます。ページ（`#[page]`）は画面を返すものなので、移り先は `Err` で返す、と覚えておきます。

### RustNote ではここで使う

ノートの作成（`src/routes/notes.rs` の `create`）は、例6-1と同じ形です（第10巻 UNIT 02 でくわしく見ました）。

```rust
#[page(POST)]
pub async fn create(cx: &Cx, Form(input): Form<NoteForm>) -> Result<impl View> {
    let user = require_user(cx).await?;
```

- `Form<NoteForm>` で受け取り、`NewNote::parse` で中身の決まりを確かめ、成功なら `Err(see_other(…).into())` で 303、失敗なら `(StatusCode::BAD_REQUEST)` で 400 です。

ログアウト（`src/routes/logout.rs`）は、本文を受け取らない POST です（抜き出しています）。

```rust
// レイアウト（エラー画面）を通すため #[page(POST)] にしている。成功すると必ず 303 なので、
// 画面として返す中身は無い（`()`）。
#[page(POST)]
pub async fn logout(cx: &Cx) -> Result<()> {
    log_out(cx).await?;
    flash::set(cx, Flash::LoggedOut);
    Err(see_other(href!(crate::routes::login::login_page).resolve(cx)).into())
}
```

- 状態を変える操作（ログアウト）なので、GET ではなく POST にしています。戻り値は `Result<()>` で、成功しても `Ok` は返さず、いつも 303 です。
- 失敗したとき（DB に接続できないなど）に、RustNote のエラー画面（レイアウトの `error_boundary`）を通したいので、`#[route]` ではなく `#[page(POST)]` にしています。

ヘルスチェック（`src/routes/health.rs`）は、例6-2と同じく、状態コードと JSON の組を返す API です。

```rust
#[route(GET)]
pub async fn health(cx: &Cx) -> Result<(StatusCode, Json<Health>)> {
```

- DB に問い合わせて、成功なら `(StatusCode::OK, Json(Health { status: "ok" }))`、失敗なら `(StatusCode::SERVICE_UNAVAILABLE, …)` を返します。人ではなく、Fly.io が読む窓口です（第12巻 UNIT 06（準備中））。

### 練習問題

**問1:** `curl.exe -i -L -d "title=%E6%9C%AC" http://127.0.0.1:3000/`（`-L` は、移り先へ自動で移る指定）で例6-1にメモを送ると、何回の返事が表示されるでしょうか。2回目の要求は、GET と POST のどちらでしょうか。

**問2:** 例6-2の `/api/memos` に、`curl.exe -i -d "title=milk"`（フォームの形）で送ると、どうなるでしょうか。

**問3:** 3 MiB の JSON（`{"title":"aaa…"}` の `a` を 3,145,728 個）を `big.json` に作り、例6-2の `/api/memos` に送ると、どうなるでしょうか。

### 解答と解説

**問1の解答:**

```text
### curl.exe -i -L -d "title=%E6%9C%AC" http://127.0.0.1:3000/
HTTP/1.1 303 See Other
location: /
content-length: 0

HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 158

<h1>メモ</h1><form method="post" action="/"><input name="title" value=""><button type="submit">保存</button></form><ul><li>買い物</li><li>本</li></ul>
```

- 2回の返事が表示されました。1回目は POST の返事（303）、2回目は、curl が `location: /` へ移ったあとの返事（200 と、「本」が増えた一覧）です。
- 2回目は **GET** です。303 は「次は GET で取りに来て」という決まりなので、curl もブラウザも、本文を送り直さずに GET で開きます。だから、再読み込みしても、メモは二重になりません（第10巻 UNIT 02 の PRG）。

**問2の解答:**

```text
### curl.exe -i -d "title=milk" http://127.0.0.1:3000/api/memos
HTTP/1.1 400 Bad Request
content-type: text/plain; charset=utf-8
content-length: 67

bad request: expected request with `Content-Type: application/json`
```

- 400 で、`expected request with Content-Type: application/json`（`Content-Type: application/json` の要求が来るはずでした）でした。`-d` で送ると、curl は `Content-Type: application/x-www-form-urlencoded` を付けるので、`Json<NewMemo>` が、本文を読む前に断りました。
- JSON の API に送るときは、`-H "Content-Type: application/json"` を付けます。ブラウザの `fetch` などで送るときも同じです。

**問3の解答:**

PowerShell では、次のように作れます。

```powershell
Set-Content big.json ('{"title":"' + ('a' * 3145728) + '"}') -Encoding ascii
curl.exe -i -H "Content-Type: application/json" --data-binary "@big.json" http://127.0.0.1:3000/api/memos
```

```text
### curl.exe -i -H "Content-Type: application/json" --data-binary "@big.json" http://127.0.0.1:3000/api/memos
HTTP/1.1 100 Continue

HTTP/1.1 413 Payload Too Large
content-type: text/plain; charset=utf-8
content-length: 17

content too large
```

- `413 Payload Too Large` で、本文は `content too large` でした。`Json` や `Form` は、本文を全部読んでから変えるので、**既定で 2 MiB まで**しか受け付けません。とても大きな本文を送りつけて、サーバーのメモリを使い切らせる攻撃を防ぐためです。
- 最初の `HTTP/1.1 100 Continue` は、大きな本文を送る前に、curl が「送ってよいですか」と尋ね（`Expect: 100-continue`）、サーバーが「どうぞ」と答えた返事です。そのあと、大きすぎると分かった時点で 413 が返りました。
- もっと大きな本文（ファイルのアップロードなど）を受け付けたい URL だけ、`BodyLimit::max(…).at("/upload")` で上限を変えられます。RustNote は、ノートの本文が最大 10,000 文字なので、2 MiB のままで足ります。

### まとめ

- フォームは `Form<T>`、JSON は `Json<T>` で、本文を構造体（`#[derive(Deserialize)]`）として受け取る。種類・大きさ（既定 2 MiB）・形が合わなければ、関数が動く前に 400・413 で断られる。中身の決まりは関数の中で確かめる。
- 画面（`#[page(POST)]`）は、成功したら `Err(see_other(…).into())` で 303 に移り（PRG）、失敗したら `(StatusCode::BAD_REQUEST)` で 400 の画面を返す。
- API（`#[route(POST)]`）は、`Ok((StatusCode::CREATED, Json(…)))` のように、状態コードと JSON をそのまま返す。本文の引数は1つだけ。303 を `Ok` で返すと型が合わない。

## UNIT 07: app_context・memoize

### この単元のゴール

- 設定や DB のように**アプリ全体で1つ**の値を、`.app_context(値)` でルーターに登録し、`app_context::<T>(cx)` でどのページからも借りられる
- `#[memoize]` で、**1つの要求の中**で同じ問い合わせを1回にまとめ、レイアウト・ページ・部品のどこから呼んでも DB を1回しか読まない形にできる
- 「ミドルウェアではなく関数」という Topcoat の考え方（`cx` を受け取る小さな関数を、必要な所で呼ぶ）を説明できる
- 登録し忘れ・同じ型を2つ登録・スレッドをまたげない値（`Rc`）のエラーを読める

### たとえ話でつかむ

大きな事務所を考えます。

- 事務所には、**全員で使う道具**があります。コピー機や、住所録です。1台ずつ全員の机に置くのではなく、決まった場所に1つ置いて、使う人が借りに行きます。置く場所には「コピー機」「住所録」と、**種類ごとに1つ**ずつ札が付いています。
- ある日の仕事（1件の依頼）の中で、同じ人の住所を何度も調べることがあります。担当者は、最初に調べた住所を**その日の依頼のメモ用紙**に書いておき、2回目からはメモを見ます。依頼が終わったら、メモ用紙は捨てます（次の依頼では、住所が変わっているかもしれないからです）。
- 「この依頼は、受付で身分を確かめてから回すこと」と、入口に決まりを貼るのではなく、**身分を確かめる手順書**を作っておき、必要な担当者が、自分の仕事の最初にその手順書を使います。

| 事務所 | Topcoat |
|---|---|
| 全員で使う道具を、決まった場所に1つ | `.app_context(値)`（ルーターに登録。型ごとに1つ） |
| 道具を借りに行く | `app_context::<T>(cx)` |
| その日の依頼のメモ用紙 | `#[memoize]`（1つの要求の中だけ、結果を覚えておく） |
| 依頼が終わったら捨てる | 次の要求では、もう一度計算する |
| 身分を確かめる手順書 | `require_user(cx)` のような、`cx` を受け取る関数 |

> **用語:** アプリのコンテキスト（app context：アプリ全体で共有する値の置き場。DB や設定を入れる。値は型で見分ける）

<!-- -->

> **用語:** メモ化（memoize：同じ引数で呼ばれた関数の結果を覚えておき、2回目からは計算せずに覚えた結果を返すこと）

<!-- -->

> **用語:** ミドルウェア（middleware：すべての要求の前後に、決まった処理（ログインの確かめなど）をはさむ仕組み。Topcoat は、これよりも関数を勧める）

### 図で理解する

アプリ全体で1つの値と、要求ごとの覚え書きの違いです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="20" width="680" height="70" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="44" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">ルーター（起動のときに1回作る）</text>
<rect x="60" y="54" width="180" height="28" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="150" y="73" font-size="12" fill="#0f172a" text-anchor="middle">AppConfig（設定）</text>
<rect x="270" y="54" width="180" height="28" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="360" y="73" font-size="12" fill="#0f172a" text-anchor="middle">Visits（数え上げ）</text>
<rect x="480" y="54" width="180" height="28" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="570" y="73" font-size="12" fill="#0f172a" text-anchor="middle">AppState（DB など）</text>
<rect x="20" y="120" width="330" height="140" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="185" y="144" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">要求 1（GET /）</text>
<text x="40" y="170" font-size="12" fill="#0f172a">app_context で借りる（全部の要求で同じもの）</text>
<rect x="40" y="184" width="290" height="60" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="185" y="206" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">memoize の覚え書き（この要求だけ）</text>
<text x="185" y="228" font-size="11" fill="#0f172a" text-anchor="middle">load_user(1) → alice（2回目からはこれを返す）</text>
<rect x="370" y="120" width="330" height="140" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="535" y="144" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">要求 2（GET /about）</text>
<text x="390" y="170" font-size="12" fill="#0f172a">同じ AppConfig・Visits を借りる</text>
<rect x="390" y="184" width="290" height="60" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="535" y="206" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">memoize の覚え書き（空から始まる）</text>
<text x="535" y="228" font-size="11" fill="#0f172a" text-anchor="middle">要求 1 の結果は使わない</text>
<line x1="150" y1="90" x2="150" y2="118" stroke="#10b981" stroke-width="2"/>
<line x1="540" y1="90" x2="540" y2="118" stroke="#10b981" stroke-width="2"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-15: app_context と memoize。app_context の値はアプリ全体で1つで、どの要求からも同じものを借りる。memoize の覚え書きは要求ごとに空から始まり、その要求が終わると捨てられる</div>
</div>

「ミドルウェア」と「関数」の違いです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 210" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0702" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="180" y="22" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">ミドルウェア（Topcoat は勧めない）</text>
<rect x="20" y="36" width="320" height="40" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="180" y="61" font-size="12" fill="#0f172a" text-anchor="middle">入口で、ログイン中の人を調べて「置いておく」</text>
<rect x="20" y="96" width="320" height="40" rx="6" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="180" y="121" font-size="12" fill="#0f172a" text-anchor="middle">ページは「置いてあるはず」と信じて取り出す</text>
<text x="180" y="166" font-size="11" fill="#b91c1c" text-anchor="middle">入口の設定を忘れると、ページは気づけない</text>
<text x="180" y="184" font-size="11" fill="#b91c1c" text-anchor="middle">（panic・確かめ漏れ）</text>
<line x1="180" y1="76" x2="180" y2="92" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0702)"/>
<text x="540" y="22" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">関数（Topcoat の書き方）</text>
<rect x="380" y="36" width="320" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="540" y="61" font-size="12" fill="#0f172a" text-anchor="middle">ページの最初で require_user(cx) を呼ぶ</text>
<rect x="380" y="96" width="320" height="40" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="540" y="121" font-size="12" fill="#0f172a" text-anchor="middle">部品・レイアウトも current_user(cx) を呼ぶ</text>
<text x="540" y="166" font-size="11" fill="#047857" text-anchor="middle">確かめているかが、ページを読めば分かる</text>
<text x="540" y="184" font-size="11" fill="#047857" text-anchor="middle">memoize で、何回呼んでも DB は1回</text>
<line x1="540" y1="76" x2="540" y2="92" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0702)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-16: ミドルウェアと関数。Topcoat では、cx を受け取る小さな関数を、必要な所で呼ぶ。同じ計算が何度も起きないように、memoize で1つの要求の中の結果を覚えておく</div>
</div>

### まず動かす最小コード

#### 例7-1: 設定と数え上げを、どのページからも借りる

> **▼ このコードがやること（先に日本語で）:** サイトの名前を持つ `AppConfig` と、表示の回数を数える `Visits` の2つを、ルーターに `.app_context(…)` で登録します。`GET /` と `GET /about` は、どちらも `config(cx)` でサイトの名前を借り、`count_visit(cx)` で回数を1つ足して表示します。

```rust
//! 第6巻 UNIT 07: アプリ全体で共有する値（設定・数え上げ）を、app_context でどのページからも借りる。
//! 実行: cargo run --bin u07_context

use std::sync::atomic::{AtomicU64, Ordering};

use topcoat::{
    Result,
    context::{Cx, app_context},
    router::{module_router, page},
    view::{View, view},
};

/// アプリの設定（起動のときに1回だけ作る）
struct AppConfig {
    site_name: String,
}

/// 来た要求の数（どの要求からも同時に足せるように、AtomicU64 にする）
struct Visits(AtomicU64);

/// 設定を借りる小さな関数（app_context::<AppConfig> を、毎回書かなくて済む）
fn config(cx: &Cx) -> &AppConfig {
    app_context::<AppConfig>(cx)
}

/// 数を1つ足して、足したあとの数を返す
fn count_visit(cx: &Cx) -> u64 {
    app_context::<Visits>(cx).0.fetch_add(1, Ordering::Relaxed) + 1
}

/// GET /
#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let visits = count_visit(cx);
    Ok(view! {
        <h1>(config(cx).site_name.as_str())</h1>
        <p>(visits) " 回目の表示です"</p>
    })
}

mod about {
    use super::*;

    /// GET /about（別のページからも、同じ値を借りられる）
    #[page]
    async fn about(cx: &Cx) -> Result<impl View> {
        let visits = count_visit(cx);
        Ok(view! {
            <p>(config(cx).site_name.as_str()) " について（" (visits) " 回目）"</p>
        })
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        // 型ごとに1つずつ登録する
        .app_context(AppConfig {
            site_name: "UNIT 07 のメモ帳".to_string(),
        })
        .app_context(Visits(AtomicU64::new(0)))
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
cargo run --bin u07_context
```

2つ目の窓から、`/`・`/`・`/about`・`/` の順に開きます。

**▼ 実行結果**

```text
### curl.exe -s http://127.0.0.1:3000/
<h1>UNIT 07 のメモ帳</h1><p>1 回目の表示です</p>
### curl.exe -s http://127.0.0.1:3000/
<h1>UNIT 07 のメモ帳</h1><p>2 回目の表示です</p>
### curl.exe -s http://127.0.0.1:3000/about
<p>UNIT 07 のメモ帳 について（3 回目）</p>
### curl.exe -s http://127.0.0.1:3000/
<h1>UNIT 07 のメモ帳</h1><p>4 回目の表示です</p>
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 登録は、ルーターを作るときに1回

- `main` で `.app_context(AppConfig { … })` と `.app_context(Visits(…))` を書きました。値は**起動のときに1回だけ**作られ、ルーターが持ちます。
- どのページからでも `app_context::<AppConfig>(cx)` で借りられます。`/` も `/about` も、同じサイトの名前を表示しました。
- 値は**型で**見分けます。`AppConfig` を借りたいときは `app_context::<AppConfig>(cx)` です。同じ型の値を2つは登録できません（エラー2）。

##### 解説2: 要求をまたいで、同じ値を使う

- 回数は、`/`・`/`・`/about`・`/` で 1・2・3・4 と増えました。**別々の要求**（`/about` も含めて）が、同じ `Visits` を借りて足しているからです。
- `Visits` の中身は `AtomicU64`（同時に足しても数が狂わない数）です。要求は、いくつも**同時に**処理されることがあるので、共有する値は、同時に使っても安全な形にします。書き換える値なら `AtomicU64` や `Mutex`、読むだけの値（`AppConfig`）ならそのままで大丈夫です（第5巻 UNIT 05）。

##### 解説3: 借りる関数を、1つ作っておく

- `config(cx)` と `count_visit(cx)` は、`app_context` を包んだ小さな関数です。ページの中に `app_context::<AppConfig>(cx)` と何度も書かずに済み、型の名前を変えたときも、直す所が1か所になります。RustNote の `app_state(cx)` も同じ考え方です。

### もう一歩: 実務に近い形

#### 例7-2: 1つの要求の中で、同じ問い合わせを1回にする

ログイン中のユーザーは、**レイアウト**（ヘッダーの名前）でも、**ページ**でも、**部品**でも使います。そのたびに DB を読むと、1回の表示で何度も同じ問い合わせをしてしまいます。`#[memoize]` で、1つの要求の中では1回にまとめます。

> **▼ このコードがやること（先に日本語で）:** `load_user(cx, id)` は、「DB から読みました」と表示して 50 ミリ秒待ってから、id が 1 なら alice を返します。`#[memoize(as_ref)]` を付けたので、同じ要求の中の2回目からは、覚えておいた結果を返します。レイアウト・ページ・部品（`user_badge` を3回）から、合わせて5回 `load_user(cx, 1)` を、ページから1回 `load_user(cx, 2)` を呼びます。

```rust
//! 第6巻 UNIT 07 の例7-2: #[memoize] で、1つの要求の中では同じ問い合わせを1回にする。
//! 実行: cargo run --bin u07_memo

use std::time::Duration;

use topcoat::{
    Result,
    context::{Cx, memoize},
    router::{Slot, layout, module_router, page},
    view::{View, component, view},
};

/// DB から読んだ、と考えるユーザー
struct User {
    name: String,
}

/// ユーザーを「DB から」読む（ここでは 50 ミリ秒待ってから作る）。
/// #[memoize(as_ref)] で、同じ要求の中の2回目からは、覚えておいた結果を返す。
#[memoize(as_ref)]
async fn load_user(cx: &Cx, id: u32) -> Option<User> {
    println!("  DB を読みました（id = {id}）");
    tokio::time::sleep(Duration::from_millis(50)).await;
    (id == 1).then(|| User {
        name: "alice".to_string(),
    })
}

/// 見出しに名前を出す部品
#[component]
async fn user_badge(cx: &Cx) -> Result<impl View> {
    let name = load_user(cx, 1).await.map(|u| u.name.clone());
    Ok(view! { <span class="badge">(name)</span> })
}

/// レイアウトでも、同じユーザーを読む
#[layout]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    let signed_in = load_user(cx, 1).await.is_some();
    Ok(view! {
        <header>
            if signed_in {
                "ログイン中: " user_badge()
            }
        </header>
        (slot)
    })
}

/// GET /: ページでも、同じユーザーと、別のユーザー（id = 2）を読む
#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let alice = load_user(cx, 1).await.map(|u| u.name.clone());
    let other = load_user(cx, 2).await.is_some();
    Ok(view! {
        <p>"ページ: " (alice) " / id 2 は " (if other { "いる" } else { "いない" })</p>
        user_badge()
        user_badge()
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    println!("http://127.0.0.1:3000 で待っています");
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
cargo run --bin u07_memo
```

2つ目の窓から、2回開きます（`-w` で、かかった時間も表示します）。

```text
### curl.exe -s -w " （%{time_total} 秒）" http://127.0.0.1:3000/
<header>ログイン中: <span class="badge">alice</span></header><p>ページ: alice / id 2 は いない</p><span class="badge">alice</span><span class="badge">alice</span> （0.113606 秒）
### curl.exe -s -w " （%{time_total} 秒）" http://127.0.0.1:3000/
<header>ログイン中: <span class="badge">alice</span></header><p>ページ: alice / id 2 は いない</p><span class="badge">alice</span><span class="badge">alice</span> （0.116494 秒）
```

**▼ サーバーの窓の表示**

```text
http://127.0.0.1:3000 で待っています
  DB を読みました（id = 1）
  DB を読みました（id = 2）
  DB を読みました（id = 1）
  DB を読みました（id = 2）
```

- 1回の表示で、`load_user(cx, 1)` は5回呼ばれましたが、「DB を読みました（id = 1）」は**1回**だけでした。id = 2 も1回です。2回の要求で、合わせて4行です。
- 2回目の要求では、もう一度 id = 1 を読みました。覚え書きは、**要求ごと**に空から始まるからです。ほかの人がプロフィールを変えたら、次の表示では新しい名前が出ます。
- `#[memoize(as_ref)]` は、戻り値の `Option<User>` を、`Option<&User>`（覚えておいた値を借りる形）に変えます。何度呼んでも、ユーザーの値を複製しません。`as_ref` を付けない `#[memoize]` は、`&Option<User>` を返します。
- かかった時間は約 0.11 秒でした。id = 1 と id = 2 の2回ぶん（50 ミリ秒 × 2）です。同時に動く部品が同じ id を読もうとしたときも、1回の読み込みを**みんなで待つ**ので、2回読むことはありません（問1で、`#[memoize]` を外したときと比べます）。

### よくあるエラーと読み方

#### エラー1: 借りる型を、登録し忘れた

```rust
//! app_context で借りる型を、ルーターに登録し忘れた
use topcoat::{
    Result,
    context::{Cx, app_context},
    router::{module_router, page},
    view::{View, view},
};

struct AppConfig {
    site_name: String,
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let config = app_context::<AppConfig>(cx);
    Ok(view! { <h1>(config.site_name.as_str())</h1> })
}

#[tokio::main]
async fn main() {
    // .app_context(AppConfig { ... }) を書き忘れた
    let router = module_router!().build();
    topcoat::start(router).await.expect("起動できません");
}
```

コンパイルは通ります。開くと、500 になります。

```text
### curl.exe -i http://127.0.0.1:3000/
HTTP/1.1 500 Internal Server Error
content-length: 21

internal server error
```

**▼ サーバーの窓の表示**

```text

thread 'tokio-rt-worker' (215316) panicked at src\bin\e07_missing.rs:15:18:
attempted to access app context of type `"e07_missing::AppConfig"`, but this type was not registered for this context
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- **何が起きたか:** 「`AppConfig` 型のアプリのコンテキストを借りようとしましたが、この型は登録されていません」という panic です。`app_context::<T>` は、無い型を借りようとすると panic します。Topcoat は panic を 500 の返事に変えます。
- **直し方:** `main` で `.app_context(AppConfig { … })` を登録します。無くてもよい値なら、`try_app_context::<T>(cx)`（無ければ `None`）で借ります（問2）。
- **学べること:** 登録し忘れは、**そのページを開いたときに**初めて分かります。RustNote のテストは、本物と同じ `build_router` でルーターを作るので、登録し忘れがあれば、テストで見つかります（第11巻 UNIT 03（準備中））。

#### エラー2: 同じ型の値を、2つ登録しようとした

本番の DB と、控えの DB の2つを、同じ `Database` 型で登録しようとした場合です（`errs/e07_twice.rs`）。

```rust
//! 同じ型の値を、2つ登録しようとした（本番の DB と、控えの DB）
use topcoat::router::module_router;

struct Database {
    url: String,
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        .app_context(Database { url: "file:main.db".to_string() })
        .app_context(Database { url: "file:replica.db".to_string() })
        .build();
    topcoat::start(router).await.expect("起動できません");
}
```

```text

thread 'main' (210860) panicked at src\bin\e07_twice.rs:12:10:
duplicate context entry for type `"e07_twice::Database"`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- **何が起きたか:** 「`Database` 型のコンテキストが二重です」という panic で、起動した途端に止まりました。値は型で見分けるので、同じ型が2つあると、`app_context::<Database>(cx)` でどちらを返せばよいか決められないからです。
- **直し方:** 型を分けます。`struct PrimaryDb(Database);` と `struct ReplicaDb(Database);` のように、**包む型**（newtype。第4巻 UNIT 01）を作って、それぞれを登録します。

#### エラー3: スレッドをまたげない Rc を入れた（E0277）

```rust
//! スレッドをまたいで共有できない Rc を、app_context に入れようとした
use std::rc::Rc;

use topcoat::router::module_router;

struct AppConfig {
    site_name: Rc<String>,
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        .app_context(AppConfig { site_name: Rc::new("メモ帳".to_string()) })
        .build();
    topcoat::start(router).await.expect("起動できません");
}
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error[E0277]: `Rc<std::string::String>` cannot be sent between threads safely
   --> src\bin\e07_rc.rs:13:22
    |
 13 |         .app_context(AppConfig { site_name: Rc::new("メモ帳".to_string()) })
    |          ----------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Rc<std::string::String>` cannot be sent between threads safely
    |          |
    |          required by a bound introduced by this call
    |
    = help: within `AppConfig`, the trait `Send` is not implemented for `Rc<std::string::String>`
note: required because it appears within the type `AppConfig`
   --> src\bin\e07_rc.rs:6:8
    |
  6 | struct AppConfig {
    |        ^^^^^^^^^
note: required by a bound in `RouterBuilder::app_context`
   --> ...\builder.rs:357:18
    |
355 |     pub fn app_context<T>(mut self, value: T) -> Self
    |            ----------- required by a bound in this associated function
356 |     where
357 |         T: Any + Send + Sync,
    |                  ^^^^ required by this bound in `RouterBuilder::app_context`

error[E0277]: `Rc<std::string::String>` cannot be shared between threads safely
```

（このあとに、`Sync` についての同じ形のエラーが続きます）

- **何が起きたか:** 「`Rc<String>` は、スレッドの間で安全に送れません」というエラーです。最後の `note:` が、`app_context` の決まり `T: Any + Send + Sync` を指しています。要求はいくつものスレッドで同時に処理されるので、登録する値は、スレッドをまたいで送れて（`Send`）、同時に借りられる（`Sync`）必要があります。`Rc` は、どちらでもありません（第3巻 UNIT 10）。
- **直し方:** `Rc` を `Arc` に変えます。この例の `site_name` なら、包む必要は無く、ただの `String` で十分です。RustNote の `AppState` は、リポジトリを `Arc<dyn NoteRepository + Send + Sync>` で持っています。

### RustNote ではここで使う

アプリ全体で共有する値は、`src/state.rs` の `AppState` にまとめています（抜き出しています）。

```rust
/// アプリ全体で共有する状態。clone は Arc の参照を増やすだけなので安い
#[derive(Clone)]
pub struct AppState {
    /// DB そのもの（GET /health の確認に使う）
    pub db: Db,
    pub users: Arc<dyn UserRepository + Send + Sync>,
    pub notes: Arc<dyn NoteRepository + Send + Sync>,
    pub sessions: Arc<dyn SessionRepository + Send + Sync>,
}

/// リクエストの処理の中から AppState を借りる
pub fn app_state(cx: &Cx) -> &AppState {
    app_context::<AppState>(cx)
}
```

- `routes.rs` の `build_router` で `.app_context(state)` と登録し（UNIT 03）、ページからは `app_state(cx).notes.search(…)` のように使います。例7-1の `config(cx)` と同じく、借りる関数を1つ作っています。
- DB とリポジトリを1つの型にまとめたので、登録する型は `AppState` の1つだけです。テストでは、一時フォルダの DB で作った `AppState` を登録します（第9巻 UNIT 01・第11巻 UNIT 02）。

ログイン中のユーザーを読む関数は、`src/auth.rs` で `#[memoize(as_ref)]` を付けています（抜き出しています）。

```rust
/// クッキーのトークンから、ログイン中のユーザーを DB で探す。
/// `#[memoize(as_ref)]` により、1つのリクエストの中では1回しか DB に問い合わせない
/// （レイアウトとページの両方から呼ばれても、2回目は覚えておいた結果を返す）。
#[memoize(as_ref)]
async fn load_current_user(cx: &Cx) -> Result<Option<User>, AppError> {
```

- レイアウト（`root_layout` のヘッダー）と、ページ（`require_user`）の両方が、ログイン中のユーザーを使います。例7-2と同じく、どこから何回呼んでも、1つの要求の中では DB を1回しか読みません。
- 「ログインが必要なページ」は、ページの最初で `require_user(cx)` を呼びます。入口のミドルウェアではなく関数なので、ページを読めば「このページはログインを確かめている」と分かります（第8巻 UNIT 01）。

### 練習問題

**問1:** 例7-2の `load_user` から `#[memoize(as_ref)]` を外すと、1回の表示で「DB を読みました」は何回出るでしょうか。かかった時間はどう変わるでしょうか。

**問2:** 「お知らせの文」（`Banner`）を、登録してもしなくてもよい値にしたいとき、どう借りればよいでしょうか。登録したルーターとしていないルーターを、1つのプログラムで作って比べてください。

**問3:** UNIT 06 の例6-1は、メモを `static MEMOS: Mutex<Vec<String>>` に覚えていました。これを `app_context` で共有する形に直してください。直すと、何が良くなるでしょうか。

### 解答と解説

**問1の解答:**

`#[memoize(as_ref)]` と `memoize` の `use` を外し、使わなくなった `cx` を `_cx` にしました（`src/bin/u07_q1.rs`）。

```text
### curl.exe -s -w " （%{time_total} 秒）" http://127.0.0.1:3000/
<header>ログイン中: <span class="badge">alice</span></header><p>ページ: alice / id 2 は いない</p><span class="badge">alice</span><span class="badge">alice</span> （0.253671 秒）
```

**▼ サーバーの窓の表示**

```text
http://127.0.0.1:3000 で待っています
  DB を読みました（id = 1）
  DB を読みました（id = 1）
  DB を読みました（id = 1）
  DB を読みました（id = 2）
  DB を読みました（id = 1）
  DB を読みました（id = 1）
```

- 1回の表示で6回（id = 1 が5回、id = 2 が1回）読みました。呼んだ回数だけ、そのまま DB を読んでいます。
- 時間は約 0.25 秒で、`#[memoize]` があったとき（約 0.11 秒）の倍ほどです。部品はいくつか同時に動くので、6 × 50 ミリ秒（0.3 秒）よりは短くなっていますが、本物の DB なら、問い合わせの数がそのまま DB の負担になります。

**問2の解答:**

`try_app_context::<Banner>(cx)` で借ります。登録されていなければ `None` で、panic しません（`src/bin/u07_q2.rs`）。

```rust
//! 問2の解答: 無くてもよい値は try_app_context で借りる。値の違う2つのルーターを、1つのプログラムで試す。
//! 実行: cargo run --bin u07_q2

use topcoat::{
    Result,
    context::{Cx, try_app_context},
    router::{Body, Method, Router, module_router, page, request::Request, to_bytes},
    view::{View, view},
};

/// あってもなくてもよい、お知らせの文
struct Banner(String);

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    // 登録されていなければ None になる（panic しない）
    let banner = try_app_context::<Banner>(cx).map(|b| b.0.clone());
    Ok(view! {
        if let Some(banner) = banner {
            <p class="banner">(banner)</p>
        }
        <h1>"トップ"</h1>
    })
}

/// サーバーを起動せずに、ルーターに GET / を1回処理させて、本文を返す
async fn get_home(router: &Router) -> String {
    let request = Request::builder()
        .method(Method::GET)
        .uri("/")
        .body(Body::empty())
        .unwrap();
    let response = router.handle(request).await;
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[tokio::main]
async fn main() {
    let plain = module_router!().build();
    let with_banner = module_router!()
        .app_context(Banner("メンテナンスのお知らせ".to_string()))
        .build();
    println!("Banner なし: {}", get_home(&plain).await);
    println!("Banner あり: {}", get_home(&with_banner).await);
}
```

```powershell
cargo run --bin u07_q2
```

```text
Banner なし: <h1>トップ</h1>
Banner あり: <p class="banner">メンテナンスのお知らせ</p><h1>トップ</h1>
```

- 同じページの関数が、ルーターによって違う結果を返しました。`Banner` を登録したルーターだけ、お知らせが出ています。
- `get_home` は、サーバーを起動せずに、`router.handle(…)` で要求を1つ処理させています。RustNote のテスト（`tests/common/mod.rs` の `get` など）と同じやり方です。

**問3の解答:**

`Memos(Mutex<Vec<String>>)` という型を作り、`.app_context(Memos(…))` で登録して、`memos(cx)` で借ります（`src/bin/u07_q3.rs`。抜き出しています）。

```rust
/// 保存したメモ。ルーターに1つ登録して、どのページからも借りる
struct Memos(Mutex<Vec<String>>);

fn memos(cx: &Cx) -> &Memos {
    app_context::<Memos>(cx)
}
```

```text
### curl.exe -i -d "title=milk" http://127.0.0.1:3000/
HTTP/1.1 303 See Other
location: /
content-length: 0


### curl.exe -s http://127.0.0.1:3000/
<ul><li>milk</li></ul>
```

- 動きは例6-1と同じです。良くなるのは、**メモの置き場が、ルーターごとに別になる**ことです。`static` は、プログラムの中に1つしかないので、問2のようにルーターを2つ作っても、同じメモを共有してしまいます。テストを同時にいくつも動かすと、ほかのテストのメモが混ざります（第11巻 UNIT 02 のエラー2と同じ問題です）。
- `app_context` なら、テストごとに新しい `Memos`（RustNote なら、新しい DB の `AppState`）を登録したルーターを作れます。

### まとめ

- アプリ全体で1つの値（設定・DB）は `.app_context(値)` で登録し、`app_context::<T>(cx)` で借りる。値は型で見分け、同じ型は1つだけ。`Send + Sync` でなければならず、書き換えるなら `AtomicU64` や `Mutex` を使う。
- `#[memoize]` は、1つの要求の中だけ結果を覚える。レイアウト・ページ・部品のどこから何回呼んでも、同じ引数なら1回しか計算しない。`as_ref` を付けると `Option<&T>` で借りられる。
- Topcoat は、入口のミドルウェアではなく、`cx` を受け取る小さな関数（`app_state(cx)`・`require_user(cx)`）を、必要な所で呼ぶ形を勧める。無くてもよい値は `try_app_context` で借りる。

## UNIT 08: アセット・CSS・束

### この単元のゴール

- CSS や画像のような静的なファイル（アセット）を `asset!` で宣言し、`view!` の中でリンク先として使える
- `topcoat asset bundle` で「束」（ハッシュ付きの名前のファイルと目録）を作り、`AssetBundle::load()` で読み込んで配れる
- ファイル名にハッシュが付く理由と、`cache-control: …, immutable` で長く覚えてもらえる理由を、CSS を書き換えて確かめられる
- 束を作り忘れた・パスを打ち間違えた・束が古いままのときのエラーと、`topcoat dev` で自動で作り直す流れを説明できる

### たとえ話でつかむ

お店のチラシを、近所に配る場面を考えます。

- チラシの原稿（CSS）は、お店の中にあります。配る前に、**印刷所で刷って、束にします**。束には、どのチラシが何枚入っているかの**目録**を付けます。
- 刷ったチラシには、**版の番号**が印刷されています。原稿を少しでも直すと、番号が変わります。お客さんは、「この番号のチラシなら、もう持っている」と、同じ番号のチラシを取っておけます（何度も取りに来なくてよい）。
- お店の入口の案内（HTML）には、「チラシは〇〇番」と、**今の版の番号**が書いてあります。原稿を直したら、案内の番号も新しくなるので、お客さんは新しいチラシを取りに来ます。
- 印刷所に行き忘れたり、原稿の場所を間違えたりすると、束ができません。原稿を足したのに刷り直さないと、案内には書いてあるのに、束に入っていないチラシができてしまいます。

| チラシ | Topcoat のアセット |
|---|---|
| チラシの原稿 | `assets/site.css`（自分で書いた CSS） |
| 「このチラシを配る」と決める | `asset!("assets/site.css")` |
| 印刷所で刷って束にする | `topcoat asset bundle`（`target\debug\assets\` に作る） |
| 束の目録 | `manifest.toml`（どの宣言が、どのファイルか） |
| 版の番号 | ファイル名のハッシュ（`site-7becc8b6682e6054.css`） |
| 入口の案内に書いた番号 | HTML の `<link href="/_topcoat/assets/site-….css">` |

> **用語:** アセット（asset：CSS・画像・フォントなど、プログラムと一緒に配る、中身が決まったファイル）

<!-- -->

> **用語:** ハッシュ（hash：ファイルの中身から計算した、決まった長さの値。中身が1文字でも違えば、まったく違う値になる。第8巻 UNIT 02）

<!-- -->

> **用語:** キャッシュ（cache：一度受け取ったファイルを、ブラウザが手元に取っておき、次は取りに行かずに使うこと）

### 図で理解する

`asset!` で宣言してから、ブラウザが CSS を受け取るまでです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0801" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="200" height="70" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="120" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">① 宣言（コード）</text>
<text x="120" y="66" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">asset!("assets/site.css")</text>
<text x="120" y="82" font-size="10" fill="#475569" text-anchor="middle">番号（ID）だけが実行ファイルに入る</text>
<rect x="260" y="20" width="200" height="70" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="44" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">② topcoat asset bundle</text>
<text x="360" y="66" font-size="11" fill="#0f172a" text-anchor="middle">ビルドして宣言を集め、</text>
<text x="360" y="82" font-size="11" fill="#0f172a" text-anchor="middle">ハッシュ付きの名前で書き出す</text>
<rect x="500" y="20" width="200" height="70" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="600" y="44" font-size="12" font-weight="700" fill="#047857" text-anchor="middle" font-family="Consolas, monospace">target\debug\assets\</text>
<text x="600" y="66" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">manifest.toml（目録）</text>
<text x="600" y="82" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">site-7becc8b6682e6054.css</text>
<line x1="220" y1="55" x2="256" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0801)"/>
<line x1="460" y1="55" x2="496" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0801)"/>
<rect x="260" y="140" width="200" height="60" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="360" y="164" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">③ 起動のとき</text>
<text x="360" y="184" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">AssetBundle::load()</text>
<line x1="600" y1="90" x2="464" y2="160" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0801)"/>
<rect x="20" y="210" width="330" height="56" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="185" y="232" font-size="12" fill="#0f172a" text-anchor="middle">④ HTML: &lt;link href="/_topcoat/assets/</text>
<text x="185" y="252" font-size="12" fill="#0f172a" text-anchor="middle">site-7becc8b6682e6054.css"&gt;</text>
<rect x="370" y="210" width="330" height="56" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="535" y="232" font-size="12" fill="#0f172a" text-anchor="middle">⑤ GET その URL → CSS と</text>
<text x="535" y="252" font-size="11" fill="#047857" text-anchor="middle">cache-control: public, max-age=31536000, immutable</text>
<line x1="330" y1="200" x2="230" y2="208" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0801)"/>
<line x1="390" y1="200" x2="490" y2="208" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0801)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-17: アセットの流れ。コードには宣言だけを書き、topcoat asset bundle がハッシュ付きの名前のファイルと目録を作る。起動のときに目録を読み、HTML には今のファイル名の URL が入る</div>
</div>

- 実行ファイルの中に入るのは、CSS の中身ではなく、**宣言の番号（ID）**だけです。CSS のファイルは、束として、実行ファイルの隣に置きます。
- 起動のときに目録（`manifest.toml`）を読み、「この ID は、このファイル名」と分かるので、`view!` に `(SITE_CSS)` と書くと、今のファイル名の URL になります。

### まず動かす最小コード

#### 例8-1: CSS を束にして配る

`web_lab` に `assets` フォルダを作り、`assets/site.css` を置きます。

```css
/* UNIT 08 の練習用の見た目 */
body {
  font-family: sans-serif;
  margin: 2rem;
}

h1 {
  color: #1e40af;
}
```

> **▼ このコードがやること（先に日本語で）:** `asset!("assets/site.css")` で CSS を宣言し、ページの `<link rel="stylesheet">` の `href` に入れます。`main` では、`AssetBundle::load()` で、実行ファイルの隣の束を読み込み、`.assets(…)` でルーターに渡します。

```rust
//! 第6巻 UNIT 08: CSS を asset! で宣言し、topcoat asset bundle で束にして配る。
//! 実行: topcoat asset bundle --bin u08_assets のあとで cargo run --bin u08_assets

use topcoat::{
    Result,
    asset::{Asset, AssetBundle, RouterBuilderAssetExt, asset},
    router::{module_router, page},
    view::{View, view},
};

/// クレートの assets/site.css（Cargo.toml のあるフォルダからの場所）
const SITE_CSS: Asset = asset!("assets/site.css");

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"UNIT 08"</title>
                <link rel="stylesheet" href=(SITE_CSS)>
            </head>
            <body>
                <h1>"アセットの練習"</h1>
            </body>
        </html>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        // 実行ファイルの隣の assets/ から、束を読み込む
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

起動する前に、束を作ります。実行ファイルがいくつもある `web_lab` では、`--bin` で、どの実行ファイルの束かを選びます。

```powershell
topcoat asset bundle --bin u08_assets
```

```text
bundled assets into ...\web_lab\target\debug\assets
```

```powershell
Get-ChildItem target\debug\assets -Name
Get-Content target\debug\assets\manifest.toml
```

```text
manifest.toml
site-7becc8b6682e6054.css
```

```toml
version = 1

[[assets]]
id = 6866230526714045621
file = "site-7becc8b6682e6054.css"
hash = "7becc8b6682e6054d01e53b6fa67af40714b1145b9a9b5191b328ac6e4c53d2e"
content_type = "text/css"
```

起動して、ページと CSS を開きます。

```powershell
cargo run --bin u08_assets
```

```text
### curl.exe -s http://127.0.0.1:3000/
<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><title>UNIT 08</title><link rel="stylesheet" href="/_topcoat/assets/site-7becc8b6682e6054.css"></head><body><h1>アセットの練習</h1></body></html>

### curl.exe -i http://127.0.0.1:3000/_topcoat/assets/site-7becc8b6682e6054.css
HTTP/1.1 200 OK
content-type: text/css
cache-control: public, max-age=31536000, immutable
content-length: 117

/* UNIT 08 の練習用の見た目 */
body {
  font-family: sans-serif;
  margin: 2rem;
}

h1 {
  color: #1e40af;
}
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: asset! は「このファイルを配る」という宣言

- `asset!("assets/site.css")` の `assets/site.css` は、`Cargo.toml` のあるフォルダ（`web_lab`）からの場所です。`./` で始めると、そのソースファイルからの場所になります。
- `const SITE_CSS: Asset = …` と名前を付けておくと、いくつものページから同じ CSS を使えます。

##### 解説2: 束には、ハッシュ付きの名前が付く

- `topcoat asset bundle` は、プログラムを一度ビルドして、`asset!` の宣言を集め、`target\debug\assets\` に書き出しました。ファイル名は `site-7becc8b6682e6054.css` で、`-` の後ろは、CSS の中身から計算したハッシュ（`manifest.toml` の `hash` の最初の 16 文字）です。
- 目録の `id` は宣言の番号、`file` はファイル名、`content_type` は配るときの種類です。

##### 解説3: 配るときは、1年間覚えてよいと伝える

- ページの HTML には、`href="/_topcoat/assets/site-7becc8b6682e6054.css"` が入りました。`.assets(…)` を登録すると、Topcoat は `/_topcoat/assets/` の下で束のファイルを配ります。
- CSS の返事には、`cache-control: public, max-age=31536000, immutable` が付いています。「誰が覚えてもよい・31,536,000 秒（365 日）覚えてよい・中身は変わらない」という意味です。ブラウザは、2回目からは CSS を取りに来ません。
- 中身が変わったら**ファイル名（URL）が変わる**ので、長く覚えてもらっても困りません（問2）。

### もう一歩: 実務に近い形

#### 例8-2: topcoat dev で、保存するたびに作り直す

CSS を直すたびに `topcoat asset bundle` と起動をくり返すのは手間です。開発中は、`topcoat dev` で動かします。

> **▼ このコードがやること（先に日本語で）:** 例8-1と同じページに、`topcoat::dev::script()` を足します。`topcoat dev` で動かしているときだけ、ブラウザに「作り直したら読み直す」ためのスクリプトが入ります。

```rust
//! 第6巻 UNIT 08 の例8-2: topcoat dev で動かすと、保存するたびに作り直し、ブラウザも読み直す。
//! 実行: topcoat dev --bin u08_dev

use topcoat::{
    Result,
    asset::{Asset, AssetBundle, RouterBuilderAssetExt, asset},
    router::{module_router, page},
    view::{View, view},
};

const SITE_CSS: Asset = asset!("assets/site.css");

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <link rel="stylesheet" href=(SITE_CSS)>
                // topcoat dev で動かしているときだけ、自動で読み直すためのスクリプトが入る
                topcoat::dev::script()
            </head>
            <body>
                <h1>"開発サーバーの練習"</h1>
            </body>
        </html>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
topcoat dev --bin u08_dev
```

**▼ topcoat dev の窓の表示（最後に止めるまで）**

```text

  topcoat dev server
  watching for file changes...

  ready on http://127.0.0.1:3000

  ready on http://127.0.0.1:3000
```

2つ目の窓からページを開くと、`dev.js` のスクリプトが入っています（`59039` の番号は、起動するたびに変わります）。

```text
### curl.exe -s http://127.0.0.1:3000/
<!DOCTYPE html><html lang="ja"><head><meta charset="utf-8"><link rel="stylesheet" href="/_topcoat/assets/site-7becc8b6682e6054.css"><script src="http://127.0.0.1:59039/dev.js"></script></head><body><h1>開発サーバーの練習</h1></body></html>
```

`topcoat dev` を動かしたまま、`assets/site.css` の `h1` の色を変えて保存し、もう一度開きます。

```text
### （h1 の色を #047857 に変えて保存したあと）curl.exe -s http://127.0.0.1:3000/ の <link>
<link rel="stylesheet" href="/_topcoat/assets/site-86109e8897909764.css">
### （色を #1e40af に戻して保存したあと）
<link rel="stylesheet" href="/_topcoat/assets/site-7becc8b6682e6054.css">
```

- 保存しただけで、CSS のファイル名が `site-86109e8897909764.css` に変わりました。`topcoat dev` が、ファイルの変更を見つけて、ビルドと束の作り直しと、アプリの起動のし直しを、自動でしてくれたのです。ブラウザで開いていれば、`dev.js` が、ページを読み直します。
- 色を元に戻すと、ファイル名も `site-7becc8b6682e6054.css` に戻りました。**同じ中身なら、同じハッシュ**です。
- `topcoat::dev::script()` は、`topcoat dev` 以外で動かしたとき（`cargo run` や本番）には、何も出しません。RustNote のレイアウトにも、同じ1行があります。

> **注意:** 本番用の束は、本番用のビルドで作ります。`topcoat asset bundle --release --bin u08_assets` は `target\release\assets\` に作り、`target\debug\assets\` の束とは別です（第12巻 UNIT 03（準備中））。ほかのビルドで作った束を使い回すと、宣言の番号が合わないことがあるので、**実行ファイルと束は、いつも同じビルドで作ります**。

```text
### topcoat asset bundle --release --bin u08_assets
bundled assets into ...\web_lab\target\release\assets
### Get-ChildItem target\release\assets -Name
manifest.toml
site-7becc8b6682e6054.css
### Get-ChildItem target\debug\assets -Name
manifest.toml
site-7becc8b6682e6054.css
```

### よくあるエラーと読み方

#### エラー1: 束を作らずに起動した

`topcoat asset clean`（束を消すコマンド）のあとに、束を作らずに起動した場合です。

```text
### topcoat asset clean
removed ...\web_lab\target\debug\assets
### cargo run --bin u08_assets

thread 'main' (201628) panicked at src\bin\u08_assets.rs:35:37:
アセットの束を読み込めません: Custom { kind: NotFound, error: "no asset bundle at ...\\web_lab\\target\\debug\\assets" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- **何が起きたか:** `AssetBundle::load()` が、「`target\debug\assets` に束がありません」（`NotFound`）を返し、`main` の `.expect(…)` で止まりました。
- **直し方:** `topcoat asset bundle --bin u08_assets` で束を作ってから起動するか、`topcoat dev --bin u08_assets` で起動します。RustNote の `main` は、`.context("アセットの束（assets/）がありません。先に topcoat asset bundle を実行するか、topcoat dev で起動してください")` で、**何をすればよいか**を表示してから止まります。

#### エラー2: asset! のパスを打ち間違えた

`asset!("assets/sitee.css")` と、`e` を1つ多く書いた場合です（`errs/e08_typo.rs`）。

```text
### topcoat asset bundle --bin e08_typo
failed to bundle assets: io error for asset at ...\web_lab\assets\sitee.css: 指定されたファイルが見つかりません。 (os error 2)
```

- **何が起きたか:** コンパイルは通りますが、`topcoat asset bundle` が「`assets\sitee.css` のファイルが見つかりません」で失敗しました。`asset!` は、ファイルがあるかをコンパイルのときには確かめず、**束を作るとき**に確かめます。
- **直し方:** パスを `assets/site.css` に直します。`topcoat asset list --bin …` で、どのファイルが束に入る予定かを確かめられます（問1）。

#### エラー3: asset! を足したのに、束を作り直さなかった

`assets/print.css`（印刷用の CSS）を足し、`asset!("assets/print.css")` も書いて `cargo build` したのに、束を作り直さずに起動した場合です（`errs/e08_stale.rs`）。

```text
### curl.exe -i http://127.0.0.1:3000/
HTTP/1.1 500 Internal Server Error
content-length: 21

internal server error
```

**▼ サーバーの窓の表示**

```text

thread 'tokio-rt-worker' (212564) panicked at ...\renderer.rs:90:44:
failed to resolve asset Asset { id: AssetId(582545301376341735) } in the asset catalog
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- **何が起きたか:** 起動はできましたが、ページを開くと 500 でした。「`Asset { id: … }` を、アセットの目録の中で見つけられません」という panic です。新しい宣言の番号が、古い目録に無いからです。
- **直し方:** 束を作り直します（`topcoat asset bundle --bin …`）。`asset!` を足した・変えたら、**必ず束を作り直す**と覚えておきます。`topcoat dev` なら、自動で作り直されます。

### RustNote ではここで使う

RustNote の CSS は、`src/views.rs` で宣言しています（抜き出しています）。

```rust
/// 画面全体のスタイルシート（assets/style.css）。
/// `topcoat asset bundle`（または `topcoat dev`）で、ハッシュ付きのファイル名で配信される。
pub const STYLESHEET: Asset = asset!("assets/style.css");
```

- `routes.rs` のレイアウトで `<link rel="stylesheet" href=(STYLESHEET)>` と使い、`topcoat::dev::script()` も置いています（UNIT 03）。

束は、`src/main.rs` で、DB を開く**前に**読み込んでいます（抜き出しています）。

```rust
    // `topcoat asset bundle`（または `topcoat dev`）が実行ファイルの隣に作る assets/ を読む。
    // 無ければ DB に触る前に止める
    let assets = AssetBundle::load().context(
        "アセットの束（assets/）がありません。先に `topcoat asset bundle` を実行するか、`topcoat dev` で起動してください",
    )?;
```

- 束が無いと分かったら、DB を開く前に止まります。準備の足りない状態で、途中まで動いてしまわないようにする順番です（第9巻 UNIT 06）。
- テストでは、束を作らずに済むように、CSS の URL だけを決めた目録（`AssetConfig::hosted_at("/test-assets", manifest)`）を渡しています（`tests/common/mod.rs` の `test_assets`）。本番の Dockerfile では、`cargo build --release` のあとに `topcoat asset bundle --release` を実行し、`assets/` を実行ファイルの隣にコピーしています（第12巻 UNIT 02・03（準備中））。

### 練習問題

**問1:** `topcoat asset list --bin u08_assets` を実行すると、何が表示されるでしょうか。束を作る前に、何を確かめるのに使えるでしょうか。

**問2:** `assets/site.css` の `h1` の色を変えてから、`topcoat asset bundle --bin u08_assets` を実行し、起動し直して、ページの `<link>` と、前のファイル名の URL（`site-7becc8b6682e6054.css`）を開いてください。どうなるでしょうか。

**問3:** `web_lab` には、実行ファイルがいくつもあります。`topcoat asset bundle --bin u02_serve`（アセットを使っていない実行ファイル）を実行してから、`u08_assets` を起動して `/` を開くと、どうなるでしょうか。

### 解答と解説

**問1の解答:**

```text
...\web_lab\assets\site.css
```

- `asset!` で宣言された、束に入る予定のファイルが表示されました。`topcoat asset list` も、プログラムを一度ビルドして宣言を集めます。パスを打ち間違えていないか、使わなくなった宣言が残っていないかを、束を作る前に確かめられます。
- 使われていない宣言（どこからも使っていない `const`）は、コンパイルのときに取り除かれることがあり、そのときは束にも入りません。

**問2の解答:**

```text
### （h1 の色を変えてから）topcoat asset bundle --bin u08_assets
bundled assets into ...\web_lab\target\debug\assets
### Get-ChildItem target\debug\assets -Name
manifest.toml
site-7b62d7c4c2b2ded6.css
```

```text
### curl.exe -s http://127.0.0.1:3000/
<link rel="stylesheet" href="/_topcoat/assets/site-7b62d7c4c2b2ded6.css">
### curl.exe -s -o NUL -w "%{http_code}" http://127.0.0.1:3000/_topcoat/assets/site-7becc8b6682e6054.css
404
```

- ファイル名が `site-7b62d7c4c2b2ded6.css` に変わり、古い `site-7becc8b6682e6054.css` は束から消えて、開くと 404 でした。
- ブラウザは、古い CSS を 365 日覚えていますが、新しいページの `<link>` は新しい URL を指すので、新しい CSS を取りに来ます。**中身が変わったら名前が変わる**ので、「直したのに、ブラウザが古い CSS を使い続ける」ということが起きません。
- 本番では、古い URL が 404 になるので、新しい版を出したときに、古い HTML を開いたままの人が、CSS を受け取れないことがあります。束を外の置き場（CDN）に置く場合は、古いファイルをしばらく残しておく、といった工夫をします（`AssetConfig::hosted_at`）。

**問3の解答:**

```text

thread 'tokio-rt-worker' (215508) panicked at ...\renderer.rs:90:44:
failed to resolve asset Asset { id: AssetId(6866230526714045621) } in the asset catalog
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- `/` は 500 で、エラー3と同じ「目録の中で見つけられません」でした。`u02_serve` で束を作ると、`target\debug\assets\` の中身が `u02_serve` の束（アセットは無し: `assets = []`）に置き換わります。束の置き場は、**同じビルドの実行ファイルの間で1つ**だからです。
- 実行ファイルを切り替えるときは、その実行ファイルで束を作り直します。RustNote のように、アセットを使う実行ファイルが1つなら、`--bin` を付けなくても困りません。

### まとめ

- CSS や画像は `asset!("assets/…")` で宣言し、`view!` に入れると、束の中の URL（`/_topcoat/assets/名前-ハッシュ.css`）になる。束は `topcoat asset bundle --bin …` で作り、`AssetBundle::load()` で読み込んで `.assets(…)` で配る。
- ファイル名のハッシュは中身から計算されるので、中身が変わると URL も変わる。だから `cache-control: …, immutable` で1年間覚えてもらえる。
- 束が無い・パスが違う・束が古いときは、それぞれ起動時・束を作るとき・ページを開いたときに止まる。開発中は `topcoat dev` で、保存するたびに作り直す。実行ファイルと束は、いつも同じビルドで作る。

## UNIT 09: runtime・signal・procedure・shard

### この単元のゴール

- Topcoat の runtime（ブラウザの中で動く部分）を有効にし、`signal`（ブラウザが持つ状態）・`@click`（イベント）・`:hidden`（属性の結び付け）で、ページを読み直さずに画面を変えられる
- `#[procedure]` で、ブラウザのボタンからサーバーの関数を呼べる。その通信が、実際には `POST /api/…` であることを、ブラウザと curl で確かめられる
- **ブラウザから届く値は利用者の入力**であり、procedure と shard には**ページとは別の入口**があるので、中で値を確かめ、権限を確かめなければならない理由を、実際の穴で説明できる
- 構造体を `signal` に入れたとき（E0277）・`.discover()` を忘れたとき（404）・あふれる値が来たとき（500）を読める

> **注意:** Topcoat の公式の説明でも、runtime は **「とても実験的」**（highly experimental）とされています。使える Rust の型や書き方が限られ、版が上がると変わることがあります。この単元では、Topcoat 0.11.0 で実際に動かした範囲を扱います。RustNote は runtime を使っていません（後で理由を説明します）。

### たとえ話でつかむ

レストランのテーブルを考えます。

- テーブルには、**お客さんの手元のメモ**（「水のおかわり 2 回目」）があります。お客さんが自分で書き換えます。厨房に聞かなくても変えられます。これが `signal` と、ブラウザの中で動く `@click` です。
- 料理の注文は、手元では作れないので、**厨房に伝票を出します**。厨房が作って、返してくれます。これが `#[procedure]` です。
- 厨房の入口は、**お客さんの席とは別のドア**です。「この席は会員の席だから」と席の前で確かめても、厨房のドアに直接伝票を持ってくる人は、その確かめを通りません。だから厨房でも、**伝票を受け取るたびに確かめます**。
- 伝票に「パンを 1800 京個」と書かれても、そのまま作ろうとしてはいけません。書いてあることは、お客さんが書いたことです。

| レストラン | Topcoat の runtime |
|---|---|
| 手元のメモ | `signal(cx, || 1usize)`（ブラウザが持つ状態。最初の値はサーバーが決める） |
| 手元で書き換える | `@click=$(|_e| count.increment())`（ブラウザの中だけで動く） |
| 厨房に伝票を出す | `#[procedure("/api/double")]` を `.await` で呼ぶ（`POST /api/double`） |
| 厨房の別のドア | procedure の URL。ページの確かめ（`admin_page` の中の確かめ）を通らない |
| 伝票を受け取るたびに確かめる | procedure の中で、値と権限を確かめる |

> **用語:** runtime（ランタイム：ここでは、Topcoat がページに入れる、ブラウザの中で動くプログラム（JavaScript）。`$( … )` に書いた式を、ブラウザでも動かす）

<!-- -->

> **用語:** signal（シグナル：ブラウザが持つ状態。値が変わると、その値を使っている式が、ブラウザの中で計算し直される）

<!-- -->

> **用語:** procedure（プロシージャ：ブラウザから呼べる、サーバーの async 関数。1つずつに URL があり、`POST` で呼ばれる）

### 図で理解する

ページを開いてから、ボタンを押すまでです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0901" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="250" height="260" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="145" y="44" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">サーバー（Rust）</text>
<text x="40" y="74" font-size="12" fill="#0f172a">① 最初の HTML を作る</text>
<text x="56" y="94" font-size="11" fill="#475569">数: 1（signal の最初の値）</text>
<text x="56" y="112" font-size="11" fill="#475569">ボタンの処理を JavaScript にして</text>
<text x="56" y="130" font-size="11" fill="#475569">HTML の属性に入れる</text>
<rect x="40" y="200" width="210" height="60" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="145" y="224" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">#[procedure] double</text>
<text x="145" y="244" font-size="11" fill="#475569" text-anchor="middle">POST /api/double に答える</text>
<rect x="450" y="20" width="250" height="260" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="575" y="44" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">ブラウザ（runtime の JS）</text>
<text x="470" y="74" font-size="12" fill="#0f172a">② +1・説明を押す</text>
<text x="486" y="94" font-size="11" fill="#475569">signal を書き換え、</text>
<text x="486" y="112" font-size="11" fill="#475569">数や hidden をその場で直す</text>
<text x="486" y="130" font-size="11" fill="#047857">（通信なし）</text>
<text x="470" y="200" font-size="12" fill="#0f172a">③ 2倍を押す</text>
<text x="486" y="220" font-size="11" fill="#475569">サーバーに数を送り、</text>
<text x="486" y="238" font-size="11" fill="#475569">返ってきた値を signal に入れる</text>
<line x1="270" y1="100" x2="446" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0901)"/>
<text x="358" y="92" font-size="11" fill="#475569" text-anchor="middle">HTML と runtime の JS</text>
<line x1="446" y1="215" x2="254" y2="215" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0901)"/>
<text x="350" y="207" font-size="11" fill="#475569" text-anchor="middle">POST /api/double（[数]）</text>
<line x1="254" y1="245" x2="446" y2="245" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0901)"/>
<text x="350" y="265" font-size="11" fill="#475569" text-anchor="middle">200（2倍の値）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-18: runtime の流れ。サーバーは最初の HTML と、ボタンの処理（JavaScript）を送る。signal の書き換えはブラウザの中だけで済み、procedure を呼んだときだけ POST の通信が起きる</div>
</div>

procedure には、ページとは別の入口があります。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-0902" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="30" width="170" height="60" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="105" y="56" font-size="12" fill="#0f172a" text-anchor="middle">ブラウザで</text>
<text x="105" y="76" font-size="12" fill="#0f172a" text-anchor="middle">/admin を開く</text>
<rect x="20" y="140" width="170" height="60" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="105" y="166" font-size="12" fill="#0f172a" text-anchor="middle">curl などで</text>
<text x="105" y="186" font-size="12" fill="#0f172a" text-anchor="middle">直接 POST する</text>
<rect x="260" y="30" width="200" height="60" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="56" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">ページの確かめ</text>
<text x="360" y="76" font-size="11" fill="#0f172a" text-anchor="middle">is_admin(cx) → だめなら 403</text>
<rect x="520" y="85" width="180" height="70" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="610" y="112" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">POST /api/clear</text>
<text x="610" y="132" font-size="11" fill="#0f172a" text-anchor="middle">（procedure の入口）</text>
<line x1="190" y1="60" x2="256" y2="60" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0902)"/>
<line x1="460" y1="70" x2="516" y2="105" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-0902)"/>
<text x="488" y="76" font-size="10" fill="#475569">ボタン</text>
<line x1="190" y1="170" x2="516" y2="135" stroke="#dc2626" stroke-width="2" marker-end="url(#ar-06-0902)"/>
<text x="350" y="178" font-size="11" fill="#b91c1c" text-anchor="middle">ページの確かめを通らずに、直接届く</text>
<text x="610" y="190" font-size="11" fill="#047857" text-anchor="middle">→ procedure の中でも確かめる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-19: procedure の入口。ボタンはページの確かめのあとで押されるが、procedure の URL には、ページを通らずに直接 POST できる。だから、確かめは procedure の中にも書く</div>
</div>

### まず動かす最小コード

#### 例9-1: 数を足すボタンと、サーバーで計算するボタン

> **▼ このコードがやること（先に日本語で）:** `count`（最初は 1）と `open`（最初は false）の2つの signal を作ります。「+1」はブラウザの中だけで `count` に 1 を足し、「2倍（サーバーで計算）」はサーバーの `double` を呼んで、返ってきた値を `count` に入れます。「説明」は `open` を切り替え、`:hidden` で説明の文を出したり消したりします。`main` では、`.discover()` で procedure を集め、`.assets(…)` と `.runtime()` を登録します。

```rust
//! 第6巻 UNIT 09: ブラウザの中で動く部分（signal・イベント・procedure）。
//! 実行: topcoat asset bundle --bin u09_runtime のあとで cargo run --bin u09_runtime

use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    router::{RouterBuilderDiscoverExt, module_router, page},
    runtime::{RouterBuilderRuntimeExt, procedure, signal},
    view::{View, view},
};

/// サーバーで動く関数。ブラウザから POST /api/double で呼ばれる
#[procedure("/api/double")]
async fn double(value: usize) -> Result<usize> {
    Ok(value * 2)
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    // ブラウザが持つ状態。最初の値はサーバーが決めて送る
    let count = signal(cx, || 1usize);
    let open = signal(cx, || false);

    Ok(view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <title>"UNIT 09"</title>
                // ブラウザで動く部分（runtime）のスクリプト
                topcoat::runtime::script()
            </head>
            <body>
                <p>"数: " $(count.get())</p>
                // ブラウザの中だけで 1 足す（サーバーには送らない）
                <button @click=$(|_e| count.increment())>"+1"</button>
                // サーバーの double を呼んで、結果を入れる
                <button @click=$(async |_e| {
                    let doubled = double(count.get()).await;
                    count.set(doubled);
                })>
                    "2倍（サーバーで計算）"
                </button>
                <button @click=$(|_e| open.set(!open.get()))>"説明"</button>
                <p :hidden=$(!open.get())>"この文は、ブラウザの中だけで出したり消したりします"</p>
            </body>
        </html>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        // procedure（double）を集めて登録する
        .discover()
        // runtime のスクリプトは、アセットの束から配る
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .runtime()
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

runtime のスクリプトは、アセットの束から配るので、起動の前に束を作ります（UNIT 08）。

```powershell
topcoat asset bundle --bin u09_runtime
cargo run --bin u09_runtime
```

ブラウザで `http://127.0.0.1:3000/` を開き、ボタンを押しました。

**▼ ブラウザで確かめた結果**

```text
（開いた直後）                数: 1
「+1」を2回押す               数: 3
「2倍（サーバーで計算）」     数: 6
「説明」を押す                「この文は、ブラウザの中だけで出したり消したりします」が出る
--- ここから、ブラウザの通信の記録を取りながら ---
「2倍」をもう一度             数: 12   通信: POST http://127.0.0.1:3000/api/double → 200
「+1」と「説明」              数: 13・説明の文が消える   通信: 0 件
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: $( … ) は、ブラウザでも動く式

- `$(count.get())` は、**runtime の式**です。サーバーは最初の HTML を作るときにこの式を計算し（数: 1）、さらに同じ式を JavaScript にしてページに入れます。ブラウザは、`count` が変わるたびに、その JavaScript で計算し直します。
- curl でページを見ると、サーバーがどう送っているかが分かります（1行で返ってくるので、読みやすいようにタグごとに改行し、最初の部分と「+1」のボタンだけを載せています）。

```powershell
curl.exe -s http://127.0.0.1:3000/
```

```text
### curl.exe -s http://127.0.0.1:3000/
<!--::topcoat::signal({&quot;t&quot;:&quot;signal&quot;,&quot;id&quot;:&quot;b4419cc41caf26e5432cdcb030a5c804&quot;,&quot;v&quot;:{&quot;t&quot;:&quot;usize&quot;,&quot;bits&quot;:64,&quot;v&quot;:&quot;1&quot;}})-->
<!--::topcoat::signal({&quot;t&quot;:&quot;signal&quot;,&quot;id&quot;:&quot;ec97a701a0af29d73734d8de751e9b94&quot;,&quot;v&quot;:false})-->
<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="utf-8">
<title>UNIT 09</title>
<script type="module" src="/_topcoat/assets/topcoat-f8e40bad0d18978e.js" data-topcoat-usize-bits="64">
</script>
…
<button data-topcoat-on:click="(__local0) => (cx.hydrate({&quot;t&quot;:&quot;Signal&quot;,&quot;id&quot;:&quot;b4419cc41caf26e5432cdcb030a5c804&quot;})).increment()">+1</button>
…（以下、ほかのボタンと説明の文が続きます）
```

- 先頭の `<!--::topcoat::signal(…)-->` が、2つの signal の最初の値（`"v":"1"` と `false`）です。`<script type="module" src="/_topcoat/assets/topcoat-….js">` が runtime の本体で、ボタンの `data-topcoat-on:click="…"` に、`@click` の処理を JavaScript にしたものが入っています。**ブラウザに送ったものは、利用者に全部見えます**。

##### 解説2: @click と :hidden は、ブラウザの中だけで動く

- 「+1」と「説明」を押したときは、ブラウザの通信は **0 件**でした。`count.increment()` も `open.set(!open.get())` も、ブラウザの中だけで signal を書き換えています。
- `:hidden=$(!open.get())` は**結び付けの属性**です。最初は `hidden=""`（隠れている）で送られ、`open` が変わるたびに、ブラウザが `hidden` を付けたり外したりします。

##### 解説3: procedure は、POST の通信になる

- 「2倍」を押したときは、`POST http://127.0.0.1:3000/api/double` が1件あり、数は 6 → 12 になりました。`double(count.get()).await` は、Rust の関数を呼んでいるように見えますが、ブラウザから**サーバーへの HTTP の要求**です。
- だから、curl からも同じ要求を送れます。数は、`{"t":"usize","bits":64,"v":"21"}` という形の JSON で送ります（Rust の `usize` を、JavaScript でも正しく扱うための形です）。

```powershell
Set-Content args21.json '[{"t":"usize","bits":64,"v":"21"}]' -Encoding ascii
curl.exe -i -H "Content-Type: application/json" --data-binary "@args21.json" http://127.0.0.1:3000/api/double
```

```text
### curl.exe -i -H "Content-Type: application/json" --data-binary "@args21.json" http://127.0.0.1:3000/api/double
HTTP/1.1 200 OK
content-type: application/json
content-length: 32

{"t":"usize","bits":64,"v":"42"}
```

### もう一歩: 実務に近い形

#### 例9-2: ページの確かめは、procedure を守らない

管理者だけが開けるページに、「メモを全部消す」ボタンを置きます。ボタンが呼ぶ procedure を、**確かめない版**（`/api/clear`）と、**確かめる版**（`/api/clear-safe`）の2つ作って比べます。

> **▼ このコードがやること（先に日本語で）:** `is_admin(cx)` は、Cookie に `admin=yes` があれば管理者とみなします（説明を短くするための仮の仕組みです。本物はセッションで確かめます。UNIT 10）。`/admin` のページは、管理者でなければ 403 にします。`clear_unsafe` は確かめずにメモを全部消し、`clear_safe` は中で `is_admin` を確かめてから消します。どちらも、消した件数を返します。

```rust
//! 第6巻 UNIT 09 の例9-2: ページの確かめは、procedure を守らない。procedure の中でも確かめる。
//! 実行: topcoat asset bundle --bin u09_guard のあとで cargo run --bin u09_guard

use std::sync::Mutex;

use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::{Cx, app_context},
    router::{RouterBuilderDiscoverExt, error::forbidden, module_router, page, request::headers},
    runtime::{RouterBuilderRuntimeExt, procedure},
    view::{View, view},
};

/// 保存したメモ
struct Memos(Mutex<Vec<String>>);

/// 管理者か（この例では、Cookie: admin=yes が付いていれば管理者とみなす。本物はセッションで確かめる）
fn is_admin(cx: &Cx) -> bool {
    headers(cx)
        .get("cookie")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|cookie| cookie.split("; ").any(|c| c == "admin=yes"))
}

/// してはいけない例: 中で確かめずに、メモを全部消す
#[procedure("/api/clear")]
async fn clear_unsafe(cx: &Cx) -> Result<usize> {
    let mut memos = app_context::<Memos>(cx).0.lock().unwrap();
    let removed = memos.len();
    memos.clear();
    Ok(removed)
}

/// 正しい例: procedure の中でも、管理者かを確かめる
#[procedure("/api/clear-safe")]
async fn clear_safe(cx: &Cx) -> Result<usize> {
    if !is_admin(cx) {
        return Err(forbidden().into());
    }
    let mut memos = app_context::<Memos>(cx).0.lock().unwrap();
    let removed = memos.len();
    memos.clear();
    Ok(removed)
}

mod admin {
    use super::*;

    /// GET /admin: 管理者だけが開けるページ。ボタンで procedure を呼ぶ
    #[page]
    async fn admin_page(cx: &Cx) -> Result<impl View> {
        if !is_admin(cx) {
            return Err(forbidden().into());
        }
        let count = app_context::<Memos>(cx).0.lock().unwrap().len();
        Ok(view! {
            <head>topcoat::runtime::script()</head>
            <p>"メモは " (count) " 件"</p>
            <button @click=$(async |_e| {
                clear_safe().await;
            })>
                "全部消す"
            </button>
        })
    }
}

#[tokio::main]
async fn main() {
    let memos = vec!["牛乳".to_string(), "本".to_string(), "宿題".to_string()];
    let router = module_router!()
        .discover()
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .app_context(Memos(Mutex::new(memos)))
        .runtime()
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
topcoat asset bundle --bin u09_guard
cargo run --bin u09_guard
```

まず、確かめない版を試します（`noargs.json` は `[]`、引数なしの意味です）。

```text
### curl.exe -i http://127.0.0.1:3000/admin
HTTP/1.1 403 Forbidden
content-type: text/plain; charset=utf-8
content-length: 9

forbidden


### curl.exe -s -H "Cookie: admin=yes" http://127.0.0.1:3000/admin
<p>メモは 3 件</p>

### curl.exe -i -H "Content-Type: application/json" --data-binary "@noargs.json" http://127.0.0.1:3000/api/clear
HTTP/1.1 200 OK
content-type: application/json
content-length: 31

{"t":"usize","bits":64,"v":"3"}


### curl.exe -s -H "Cookie: admin=yes" http://127.0.0.1:3000/admin
<p>メモは 0 件</p>
```

- Cookie の無い要求で `/admin` を開くと 403 で、ページの確かめは働いています。
- ところが、同じ Cookie の無い要求で `/api/clear` に直接 POST すると、**200 で、メモが3件消えました**。管理者として `/admin` を開き直すと、0 件です。`/api/clear` は、`/admin` のページを通らない、**別の入口**だからです。

次に、サーバーを起動し直して（メモが3件に戻ります）、確かめる版を試します。

```text
### curl.exe -i -H "Content-Type: application/json" --data-binary "@noargs.json" http://127.0.0.1:3000/api/clear-safe
HTTP/1.1 403 Forbidden
content-type: text/plain; charset=utf-8
content-length: 9

forbidden


### curl.exe -i -H "Cookie: admin=yes" -H "Content-Type: application/json" --data-binary "@noargs.json" http://127.0.0.1:3000/api/clear-safe
HTTP/1.1 200 OK
content-type: application/json
content-length: 31

{"t":"usize","bits":64,"v":"3"}
```

- Cookie が無ければ 403、あれば 200 で3件消えました。procedure の**中で**確かめたので、どこから呼ばれても守られます。
- Topcoat の説明にも、「procedure の引数を確かめ、中で権限を確かめなさい。呼ぶ人は、自分で要求を作って送れるので」と書かれています。shard（サーバーで描き直す部品）も、同じく自分の入口を持ち、ページやレイアウトの確かめは働きません。

> **ヒント:** 「ページにボタンを出していないから、押されない」は、守りになりません。URL は、ページの HTML（解説1の `data-topcoat-on:click`）を見れば分かります。**入口ごとに確かめる**のが決まりです（第8巻 UNIT 07（準備中））。

### よくあるエラーと読み方

#### エラー1: 自分の構造体を、そのまま signal に入れた（E0277）

```rust
//! 自分で作った構造体を、そのまま signal に入れた
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    router::{module_router, page},
    runtime::{RouterBuilderRuntimeExt, signal},
    view::{View, view},
};

#[derive(Clone)]
struct Cart {
    items: usize,
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let cart = signal(cx, || Cart { items: 0 });
    Ok(view! {
        <head>topcoat::runtime::script()</head>
        <p>"かご: " $(cart.get().items)</p>
    })
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        .assets(AssetBundle::load().expect("アセットの束を読み込めません"))
        .runtime()
        .build();
    topcoat::start(router).await.expect("起動できません");
}
```

```text
   Compiling web_lab v0.1.0 (...\web_lab)
error[E0277]: the trait bound `Cart: SignalValue` is not satisfied
   --> src\bin\e09_struct.rs:18:16
    |
 18 |     let cart = signal(cx, || Cart { items: 0 });
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `Surrogated` is not implemented for `Cart`
   --> src\bin\e09_struct.rs:12:1
    |
 12 | struct Cart {
    | ^^^^^^^^^^^
    = help: the following other types implement trait `Surrogated`:
              &'__lifetime Option<T>
              &'__lifetime Result<T, E>
              &'__lifetime Signal<T>
              &'__lifetime Vec<T>
              &'__lifetime [T; N]
              &'__lifetime [T]
              &'__lifetime bool
```

（このあとに、`signal` の決まり `T: SignalValue` を指す `note:` と、同じ形のエラーが続きます）

- **何が起きたか:** 「`Cart` に `SignalValue` が実装されていません」というエラーです。signal の値は、ブラウザ（JavaScript）とサーバー（Rust）の間を行き来するので、runtime が両方で扱える型（数・文字・`bool`・`Vec`・`Option` など。`help:` に一部が並んでいます）でなければなりません。
- **直し方:** 構造体に `#[record]` を付けます（`use topcoat::runtime::record;`）。`#[record]` と `#[derive(Clone)]` を付けた `Cart` で、ビルドが通ることを確かめました。項目の型も、runtime が扱える型にします。

#### エラー2: .discover() を書き忘れた

`main` の `.discover()` を消してしまった場合です（`errs/e09_nodiscover.rs`）。コンパイルも起動も通り、ページも開けます。

```text
### curl.exe -s -o NUL -w "%{http_code}" http://127.0.0.1:3000/
200
### curl.exe -i -H "Content-Type: application/json" --data-binary "@args21.json" http://127.0.0.1:3000/api/double
HTTP/1.1 404 Not Found
content-type: text/plain; charset=utf-8
content-length: 9

not found
```

- **何が起きたか:** `/api/double` が 404 でした。`#[procedure]` は、ルーターに登録しないと、入口ができません。`module_router!()` は `#[page]` などは集めますが、procedure は集めないので、`.discover()`（または `.route(double)`）が要ります。ブラウザで「2倍」を押しても、通信が 404 で失敗し、数は変わりません。
- **直し方:** `.discover()` を足します。「押しても何も起きない」ときは、ブラウザの開発者ツールの「ネットワーク」で、procedure の要求が何番で返っているかを見ます。

#### エラー3: あふれる値が届いて、500 になった

例9-1の `double` に、`usize` の最大の値（18446744073709551615）を送った場合です（`argsbig.json`）。

```text
### curl.exe -i -H "Content-Type: application/json" --data-binary "@argsbig.json" http://127.0.0.1:3000/api/double
HTTP/1.1 500 Internal Server Error
content-length: 21

internal server error
```

**▼ サーバーの窓の表示**

```text

thread 'tokio-rt-worker' (214492) panicked at src\bin\u09_runtime.rs:16:8:
attempt to multiply with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- **何が起きたか:** `value * 2` が `usize` に入りきらず、「掛け算であふれました」（`attempt to multiply with overflow`）で panic し、500 になりました。デバッグのビルドでは、あふれると panic する決まりです。リリースのビルドでは、あふれた分が**黙って切り捨てられ**、おかしな値が返ります（第2巻 UNIT 02）。
- **学べること:** ブラウザのボタンは 1・2・4・… と数を倍にするだけですが、procedure の入口には、**どんな値でも**送れます。ブラウザから届く値は、利用者が書いた入力と同じに扱い、サーバーで確かめます（問1）。

### RustNote ではここで使う

**RustNote は、runtime を使っていません。** 画面の操作は、すべて HTML のフォームと、303 で移る PRG（UNIT 06）で作っています。理由は次の3つです。

- runtime は、まだ実験的で、版が上がると書き方が変わることがあります。教材の完成形は、長く同じ形で読めるようにしたいからです。
- フォームと PRG なら、JavaScript が動かないブラウザでも、すべての操作ができます。入口は `#[page(POST)]` だけなので、確かめる場所（`require_user`）も、ページの関数の中の1か所で済みます。
- 守りの考え方は同じです。RustNote の POST（作成・編集・完了・削除・ログアウト）は、どれもページの関数の最初で `require_user(cx)` を呼び、ほかのサイトからの POST は、Topcoat の OriginPolicy が 403 で止めます（第8巻 UNIT 01。問3）。

ノートの検索を、入力のたびに結果が変わる形にしたいとき（いまはボタンで検索）などに、shard を使う余地があります。そのときも、shard の中で `require_user(cx)` を呼び、検索の言葉を `NoteQuery::new` で確かめてから使います。

### 練習問題

**問1:** 例9-1の `double` を、2倍が `usize` に入りきらないときは 400（`bad_request`）を返すように直し、エラー3と同じ値を送って確かめてください。

**問2:** 例9-1のページの HTML を curl で見ると、`count` の最初の値や、ボタンの処理が見えました。runtime を使うページに、**ブラウザに送ってはいけないもの**は何でしょうか。

**問3:** 例9-2の確かめない版の `/api/clear` に、**ほかのサイトのページから** POST させると、どうなるでしょうか（curl で `Origin: https://evil.example` と `Sec-Fetch-Site: cross-site` を付けて試します）。それでも、確かめない版を使ってはいけないのはなぜでしょうか。

### 解答と解説

**問1の解答:**

`checked_mul` は、あふれるときに `None` を返します。`None` なら `bad_request(…)` を返します（`src/bin/u09_q1.rs`。変えた所だけを抜き出しています）。

```rust
#[procedure("/api/double")]
async fn double(value: usize) -> Result<usize> {
    // ブラウザから来た値は、利用者の入力。入りきらないなら断る
    value
        .checked_mul(2)
        .ok_or_else(|| bad_request("2倍にすると大きすぎます").into())
}
```

```text
### curl.exe -i -H "Content-Type: application/json" --data-binary "@args21.json" http://127.0.0.1:3000/api/double
HTTP/1.1 200 OK
content-type: application/json
content-length: 32

{"t":"usize","bits":64,"v":"42"}


### curl.exe -i -H "Content-Type: application/json" --data-binary "@argsbig.json" http://127.0.0.1:3000/api/double
HTTP/1.1 400 Bad Request
content-type: text/plain; charset=utf-8
content-length: 47

bad request: 2倍にすると大きすぎます
```

- 21 は 42 になり、最大の値は 400 と「2倍にすると大きすぎます」になりました。サーバーは panic せず、窓にも何も出ません。
- `bad_request` の文は、送った人に見えます。中のこと（SQL やファイルの場所など）を書かないようにします。

**問2の解答:**

- `signal` の値・`$( … )` の式・ボタンの処理は、どれも HTML に入って、ブラウザに送られます。だから、**秘密の値**（パスワードのハッシュ・API のトークン・ほかの人のメールアドレス・管理者だけが見てよい数）を、signal に入れたり、`$( … )` の式の中で使ったりしてはいけません。
- 「ブラウザで隠している（`hidden`）」も、**見えていない**だけで、HTML には入っています。見せてはいけないものは、サーバーで、そもそも HTML に入れない（`if` で出し分ける）ようにします。

**問3の解答:**

```text
### curl.exe -i -H "Origin: https://evil.example" -H "Sec-Fetch-Site: cross-site" -H "Content-Type: application/json" --data-binary "@noargs.json" http://127.0.0.1:3000/api/clear
HTTP/1.1 403 Forbidden
content-type: text/plain; charset=utf-8
content-length: 9

forbidden


### curl.exe -s -H "Cookie: admin=yes" http://127.0.0.1:3000/admin
<p>メモは 3 件</p>
```

- 403 で止まり、メモは3件のままでした。Topcoat の OriginPolicy は、**ほかのサイトのページから、ブラウザが送る**状態を変える要求（POST など）を、すべての入口で止めます（procedure も含みます。例9-1の `/api/double` でも、同じく 403 でした）。
- それでも、確かめない版を使ってはいけません。OriginPolicy が止めるのは「ほかのサイトのページから、利用者のブラウザに送らせる」攻撃（CSRF。UNIT 10）だけで、**ブラウザを使わずに、自分で要求を作る人**（例9-2の curl）は止めません。その人が管理者でなければ断る、という確かめは、procedure の中にしか書けません。

### まとめ

- runtime は、`signal` と `$( … )` の式で、ブラウザの中だけで画面を変える仕組み。`@click` はイベント、`:hidden` などは結び付けの属性。使うには `topcoat::runtime::script()`・`.runtime()`・アセットの束が要る。Topcoat 0.11 でも「とても実験的」な機能。
- `#[procedure]` は、ブラウザから `POST` で呼ばれるサーバーの関数。`.discover()` などで登録しないと 404。signal の値の型は、runtime が両方で扱える型か `#[record]` の構造体。
- ブラウザに送ったものは全部見え、ブラウザから届く値は利用者の入力。procedure と shard には別の入口があるので、**中で値と権限を確かめる**。OriginPolicy は、ほかのサイトからの POST は止めるが、直接の要求は止めない。

## UNIT 10: エラー・クッキー・セッション・CSRF

### この単元のゴール

- ページで起きた 404・400 を、`not_found!()` とレイアウトの `error_boundary` で、**状態コード付きの自分の画面**に変えられる。リダイレクトは変えずに通す理由を言える
- `cookies(cx)` でクッキーを読み書きし、`Set-Cookie` の中身（`Path`・`Max-Age` など）を読める。署名付きのクッキーで、書き換えを見つけられる
- `session::start`・`token_hash`・`stop` で、ログインの土台（トークンを発行し、ハッシュで記録し、終わらせる）を作れる
- CSRF（ほかのサイトから、ログイン中の人に操作させる攻撃）を、Topcoat の OriginPolicy がどう止めるかを確かめ、状態を変える操作を GET にしてはいけない理由を説明できる

### たとえ話でつかむ

会員制のスポーツクラブを考えます。

- 受付の人は、困ったことが起きたとき、**決まった案内の紙**を渡します。「その部屋はありません」「書き方が違います」。どの係で起きても、同じ形の紙です。ただし、「2階へどうぞ」という**道案内**は、困ったことではないので、そのまま伝えます。
- 会員には、**入館証**を渡します。入館証には、番号だけが書いてあります。クラブは、番号の**写し（ハッシュ）**を台帳に書いておき、入館証を見せられたら、台帳と照らし合わせます。退会や帰るときは、入館証を返してもらい、台帳からも消します。
- 来た人の上着に付ける**札**（「今日はロッカー 12 番」）は、本人が書き換えられます。大事な札には、**クラブの印**を押しておき、印の無い札は無視します。
- 外の誰かから「この会員の退会の手続きをして」という紙が届いても、**会員本人が窓口に来た**のでなければ、受け付けません。

| スポーツクラブ | Topcoat |
|---|---|
| 決まった案内の紙 | `error_boundary` の中で作る、エラーの画面（404・400） |
| 道案内はそのまま | リダイレクト（303・307）は、エラーの画面にせず Topcoat に返す |
| 入館証（番号だけ） | セッションのトークン（`__Host-session` のクッキー） |
| 台帳の写し | トークンのハッシュ（`TokenHash`）と、その人の記録 |
| 上着の札 | ふつうのクッキー（`theme=dark` など）。利用者が書き換えられる |
| クラブの印 | 署名付きのクッキー（`signed_cookies`）。印が合わなければ読まない |
| 本人が窓口に来たか | OriginPolicy（ほかのサイトから送られた POST を 403 で止める） |

> **用語:** クッキー（cookie：サーバーが `Set-Cookie` で預け、ブラウザが次の要求から `Cookie` で送り返してくる、小さな名前と値の組）

<!-- -->

> **用語:** セッション（session：一度ログインした人を、次の要求からも同じ人と分かるようにする仕組み。第8巻 UNIT 01）

<!-- -->

> **用語:** CSRF（シーサーフ：ほかのサイトのページから、ログイン中の人のブラウザに、勝手に要求を送らせる攻撃。クッキーは自動で付くので、本人の操作に見えてしまう）

### 図で理解する

ページで起きたエラーが、画面になるまでです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-1001" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="190" height="90" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="115" y="44" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">ページが Err を返す</text>
<text x="115" y="66" font-size="11" fill="#475569" text-anchor="middle">ok_or_not_found()?</text>
<text x="115" y="84" font-size="11" fill="#475569" text-anchor="middle">bad_request(…)・redirect(…)</text>
<text x="115" y="102" font-size="11" fill="#475569" text-anchor="middle">ほかのエラー（? で）</text>
<rect x="250" y="20" width="220" height="90" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">レイアウトの error_boundary</text>
<text x="360" y="66" font-size="11" fill="#0f172a" text-anchor="middle">fallback: error_page(cx, error)</text>
<text x="360" y="86" font-size="11" fill="#0f172a" text-anchor="middle">downcast_ref で種類を見分ける</text>
<rect x="510" y="10" width="190" height="56" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="605" y="32" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">知っている種類</text>
<text x="605" y="52" font-size="11" fill="#0f172a" text-anchor="middle">404・400 → 自分の画面</text>
<rect x="510" y="76" width="190" height="56" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="605" y="98" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">知らない種類</text>
<text x="605" y="118" font-size="11" fill="#0f172a" text-anchor="middle">Err(error) で Topcoat に返す</text>
<line x1="210" y1="65" x2="246" y2="65" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-1001)"/>
<line x1="470" y1="50" x2="506" y2="40" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-1001)"/>
<line x1="470" y1="80" x2="506" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-1001)"/>
<rect x="250" y="160" width="450" height="80" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="475" y="184" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">Topcoat が返事にする</text>
<text x="475" y="206" font-size="11" fill="#0f172a" text-anchor="middle">リダイレクト（redirect・see_other）→ 307・303 と location</text>
<text x="475" y="226" font-size="11" fill="#b91c1c" text-anchor="middle">ルーターのエラーではないもの → 500「internal server error」</text>
<line x1="605" y1="132" x2="605" y2="156" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-1001)"/>
<rect x="20" y="160" width="190" height="80" rx="8" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="115" y="184" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">どこにも当たらない URL</text>
<text x="115" y="206" font-size="11" fill="#0f172a" text-anchor="middle">not_found!() があれば</text>
<text x="115" y="224" font-size="11" fill="#0f172a" text-anchor="middle">NotFoundError としてレイアウトへ</text>
<line x1="210" y1="190" x2="300" y2="112" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-1001)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-20: エラーの流れ。ページの Err はレイアウトの error_boundary に届き、知っている種類だけを自分の画面に変える。リダイレクトなど知らない種類は Topcoat に返す</div>
</div>

ログインからログアウトまでの、セッションの一生です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 240" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-1002" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="210" height="96" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="125" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">① ログイン（POST /login）</text>
<text x="125" y="66" font-size="11" fill="#0f172a" text-anchor="middle">session::start(cx)</text>
<text x="125" y="86" font-size="11" fill="#475569" text-anchor="middle">新しいトークンをクッキーで渡し、</text>
<text x="125" y="104" font-size="11" fill="#475569" text-anchor="middle">ハッシュを記録する</text>
<rect x="255" y="20" width="210" height="96" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="44" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">② 次の要求（GET /）</text>
<text x="360" y="66" font-size="11" fill="#0f172a" text-anchor="middle">session::token_hash(cx)</text>
<text x="360" y="86" font-size="11" fill="#475569" text-anchor="middle">送られたトークンのハッシュで、</text>
<text x="360" y="104" font-size="11" fill="#475569" text-anchor="middle">記録を探す</text>
<rect x="490" y="20" width="210" height="96" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="595" y="44" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">③ ログアウト（POST /logout）</text>
<text x="595" y="66" font-size="11" fill="#0f172a" text-anchor="middle">session::stop(cx)</text>
<text x="595" y="86" font-size="11" fill="#475569" text-anchor="middle">クッキーを消させ、</text>
<text x="595" y="104" font-size="11" fill="#475569" text-anchor="middle">記録も消す</text>
<line x1="230" y1="68" x2="251" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-1002)"/>
<line x1="465" y1="68" x2="486" y2="68" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-1002)"/>
<rect x="20" y="146" width="680" height="76" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="360" y="170" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">Set-Cookie: __Host-session=（32 バイトの乱数）; HttpOnly; SameSite=Lax; Secure; Path=/; Max-Age=2592000</text>
<text x="360" y="194" font-size="11" fill="#475569" text-anchor="middle">記録に残すのはトークンのハッシュだけ。記録が盗まれても、ハッシュはトークンとして使えない</text>
<text x="360" y="212" font-size="11" fill="#475569" text-anchor="middle">トークンが1文字でも違えば、ハッシュが違うので、記録は見つからない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図6-21: セッションの一生。Topcoat はトークンの発行と受け渡しをし、アプリはハッシュと記録の保存と照合をする。ログアウトでは、クッキーと記録の両方を消す</div>
</div>

### まず動かす最小コード

#### 例10-1: エラーを、状態コード付きの自分の画面にする

> **▼ このコードがやること（先に日本語で）:** `not_found!()` で、どこにも当たらない URL も `NotFoundError` にします。レイアウトでは `error_boundary` でページを包み、エラーが起きたら `error_page` を呼びます。`error_page` は、`NotFoundError` なら 404 の画面、`BadRequestError` なら 400 の画面を作り、それ以外は `Err(error)` で Topcoat に返します。ページは、`/items/{id}`（1〜3 番だけある）・`/search?q=…`（言葉が無ければ 400）・`/old`（トップへ 307 で移す）・`/boom`（ファイルを読めずにふつうのエラー）です。

```rust
//! 第6巻 UNIT 10: エラーを、自分の画面（状態コード付き）に変える。404・400・リダイレクト・500。
//! 実行: cargo run --bin u10_errors

use topcoat::{
    Error, Result,
    context::Cx,
    router::{
        Slot, StatusCode,
        error::{BadRequestError, NotFoundError, bad_request, redirect},
        layout, module_router, not_found, page, query_params,
    },
    view::{View, error_boundary, view},
};

// どのルートにも当たらない URL も、NotFoundError としてレイアウトまで届ける
not_found!();

/// ページで起きたエラーを、状態コード付きの自分の画面に変える。
/// 知らないエラー（リダイレクトや 500 になるもの）は、そのまま Topcoat に返す
fn error_page(cx: &Cx, error: Error) -> Result<impl View + '_> {
    let (status, message) = if error.downcast_ref::<NotFoundError>().is_some() {
        (StatusCode::NOT_FOUND, "ページが見つかりません".to_string())
    } else if let Some(bad) = error.downcast_ref::<BadRequestError>() {
        (StatusCode::BAD_REQUEST, bad.description().to_string())
    } else {
        return Err(error);
    };
    Ok(view! {
        cx =>
        (status)
        <h1>(message)</h1>
        <p><a href="/">"トップへ"</a></p>
    })
}

#[layout]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <header>"UNIT 10 のサイト"</header>
        <main>
            error_boundary(fallback: |error| error_page(cx, error), (slot))
        </main>
    })
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! { <h1>"トップ"</h1> })
}

mod items {
    mod id {
        use topcoat::{
            Result,
            context::Cx,
            router::{error::RouterErrorExt, module_param, page, path_param},
            view::{View, view},
        };

        module_param!(id: u32, error = not_found);

        /// GET /items/{id}: 1〜3 番だけある
        #[page]
        async fn show(cx: &Cx) -> Result<impl View> {
            let id = *path_param::<Id>(cx)?;
            let name = ["りんご", "みかん", "ぶどう"]
                .get(id.wrapping_sub(1) as usize)
                .ok_or_not_found()?;
            Ok(view! { <h1>(id) " 番: " (*name)</h1> })
        }
    }
}

mod search {
    use super::*;

    #[query_params]
    struct SearchQuery {
        q: Option<String>,
    }

    /// GET /search?q=…: 言葉が無ければ 400
    #[page]
    async fn search(cx: &Cx) -> Result<impl View> {
        let q = query_params::<SearchQuery>(cx)
            .ok()
            .and_then(|query| query.q.clone())
            .filter(|q| !q.trim().is_empty())
            .ok_or_else(|| bad_request("検索の言葉を入れてください"))?;
        Ok(view! { <h1>"「" (q) "」の検索結果"</h1> })
    }
}

mod old {
    use super::*;

    /// GET /old: 古い URL。トップへ移す（画面は返さないので、戻り値は ()）
    #[page]
    async fn old() -> Result<()> {
        Err(redirect("/").into())
    }
}

mod boom {
    use super::*;

    /// GET /boom: ルーターのエラーではない、ふつうのエラー
    #[page]
    async fn boom() -> Result<impl View> {
        std::fs::read_to_string("no-such-file.txt")?;
        Ok(view! { <p>"読めました"</p> })
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!().build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
cargo run --bin u10_errors
```

**▼ 実行結果（404）**

```text
### curl.exe -i "http://127.0.0.1:3000/"
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 68

<header>UNIT 10 のサイト</header><main><h1>トップ</h1></main>


### curl.exe -i "http://127.0.0.1:3000/items/2"
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 75

<header>UNIT 10 のサイト</header><main><h1>2 番: みかん</h1></main>


### curl.exe -i "http://127.0.0.1:3000/items/9"
HTTP/1.1 404 Not Found
content-type: text/html; charset=utf-8
content-length: 127

<header>UNIT 10 のサイト</header><main><h1>ページが見つかりません</h1><p><a href="/">トップへ</a></p></main>


### curl.exe -i "http://127.0.0.1:3000/nothing/here"
HTTP/1.1 404 Not Found
content-type: text/html; charset=utf-8
content-length: 127

<header>UNIT 10 のサイト</header><main><h1>ページが見つかりません</h1><p><a href="/">トップへ</a></p></main>
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 404 も、レイアウトの中の自分の画面になる

- `/items/9` は、`ok_or_not_found()?` で `NotFoundError` になり、`404 Not Found` と、レイアウト（`<header>`）付きの「ページが見つかりません」が返りました。
- `/nothing/here`（どこにも当たらない URL）も、同じ画面です。`not_found!()` を書かないと、こうした URL は**レイアウトを通らずに** `not found` の文字だけになります（UNIT 02・03 の 404）。`not_found!()` は、「ほかに当たらない URL は、全部ここへ」というページを足して、`NotFoundError` をレイアウトまで届けます。
- エラーの画面の最初に `(status)` を置いています。これが無いと、エラーの画面なのに **200** で返ってしまいます（UNIT 06 の 400 と同じ考え方です）。

##### 解説2: 400 は、わけを書いて返す

```text
### curl.exe -i "http://127.0.0.1:3000/search"
HTTP/1.1 400 Bad Request
content-type: text/html; charset=utf-8
content-length: 133

<header>UNIT 10 のサイト</header><main><h1>検索の言葉を入れてください</h1><p><a href="/">トップへ</a></p></main>


### curl.exe -i "http://127.0.0.1:3000/search?q=rust"
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 84

<header>UNIT 10 のサイト</header><main><h1>「rust」の検索結果</h1></main>
```

- `bad_request("検索の言葉を入れてください")` の文は、`BadRequestError` の `description()` で取り出せます。ここで渡した文は、利用者に見えるので、直し方が分かる、短い文にします。

##### 解説3: リダイレクトと、ふつうのエラーは Topcoat に返す

```text
### curl.exe -i "http://127.0.0.1:3000/old"
HTTP/1.1 307 Temporary Redirect
location: /
content-length: 0



### curl.exe -i "http://127.0.0.1:3000/boom"
HTTP/1.1 500 Internal Server Error
content-type: text/plain; charset=utf-8
content-length: 21

internal server error
```

- `/old` は `307 Temporary Redirect` と `location: /` でした。`redirect(…)` も、`error_boundary` には「エラー」として届きますが、`error_page` は知らない種類として `Err(error)` で返したので、Topcoat が 307 の返事にしました（エラー3）。`redirect` は 307（メソッドを変えずに送り直す）、フォームのあとに使う `see_other` は 303 です（UNIT 06）。
- `/boom` は 500 で、本文は `internal server error` だけでした。ファイルが無い、という**ふつうのエラー**（`std::io::Error`）は、ルーターのエラーではないので、Topcoat は中身を見せずに 500 にします。サーバーの窓にも、**何も表示されませんでした**。何が起きたかを知るには、自分で記録します（問1）。

### もう一歩: 実務に近い形

#### 例10-2: クッキーとセッションで、「誰が見ているか」を覚える

> **▼ このコードがやること（先に日本語で）:** `GET /` は、クッキー `theme` と、セッションの名前を表示します。`POST /theme` はテーマのクッキーを書き換え、`POST /login` は `session::start` で新しいセッションを始めて、トークンのハッシュと名前を記録し、`POST /logout` は `session::stop` でセッションを終えて記録を消します。どれも、終わったら 303 でトップへ移ります。ルーターには、`.cookies()`・`.sessions(…)`・記録の置き場（`Sessions`）を登録します。

```rust
//! 第6巻 UNIT 10 の例10-2: クッキーとセッションで、「誰が見ているか」を覚える土台。
//! 実行: cargo run --bin u10_session

use std::collections::HashMap;
use std::sync::Mutex;

use serde::Deserialize;
use topcoat::{
    Result,
    context::{Cx, app_context},
    cookie::{Cookie, Cookies, RouterBuilderCookieExt, cookies},
    router::{content::Form, error::see_other, module_router, page},
    session::{self, RouterBuilderSessionExt, SessionConfig, TokenHash},
    view::{View, view},
};

/// セッションの記録（トークンのハッシュ → 名前）。本物のアプリでは DB に保存する
struct Sessions(Mutex<HashMap<TokenHash, String>>);

/// 今の要求のセッションから、名前を探す
async fn current_name(cx: &Cx) -> Result<Option<String>> {
    let Some(hash) = session::token_hash(cx).await? else {
        return Ok(None);
    };
    Ok(app_context::<Sessions>(cx)
        .0
        .lock()
        .unwrap()
        .get(&hash)
        .cloned())
}

/// GET /: クッキー（テーマ）と、セッション（名前）を読む
#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    let theme = cookies(cx)
        .get("theme")
        .map(|c| c.value().to_string())
        .unwrap_or_else(|| "light".to_string());
    let name = current_name(cx).await?;
    Ok(view! {
        <p>"テーマ: " (theme)</p>
        <p>"ログイン: " (name.unwrap_or_else(|| "（していない）".to_string()))</p>
    })
}

mod theme {
    use super::*;

    /// POST /theme: テーマのクッキーを書いて、トップへ
    #[page(POST)]
    async fn toggle(cx: &Cx) -> Result<()> {
        let jar = cookies(cx);
        let next = match jar.get("theme") {
            Some(theme) if theme.value() == "dark" => "light",
            _ => "dark",
        };
        jar.add(Cookie::build(("theme", next)).path("/").build());
        Err(see_other("/").into())
    }
}

mod login {
    use super::*;

    #[derive(Deserialize)]
    struct LoginForm {
        name: String,
    }

    /// POST /login: 新しいセッションを始めて、ハッシュと名前を記録する
    #[page(POST)]
    async fn login(cx: &Cx, Form(input): Form<LoginForm>) -> Result<()> {
        let started = session::start(cx).await?;
        app_context::<Sessions>(cx)
            .0
            .lock()
            .unwrap()
            .insert(started.token_hash, input.name);
        Err(see_other("/").into())
    }
}

mod logout {
    use super::*;

    /// POST /logout: セッションを終えて、記録も消す
    #[page(POST)]
    async fn logout(cx: &Cx) -> Result<()> {
        if let Some(hash) = session::stop(cx).await? {
            app_context::<Sessions>(cx).0.lock().unwrap().remove(&hash);
        }
        Err(see_other("/").into())
    }
}

#[tokio::main]
async fn main() {
    let router = module_router!()
        .cookies()
        .sessions(SessionConfig::default())
        .app_context(Sessions(Mutex::new(HashMap::new())))
        .build();
    topcoat::start(router)
        .await
        .expect("サーバーを起動できませんでした");
}
```

```powershell
cargo run --bin u10_session
```

まず、ふつうのクッキーです。

```text
### curl.exe -s http://127.0.0.1:3000/
<p>テーマ: light</p><p>ログイン: （していない）</p>

### curl.exe -i -X POST http://127.0.0.1:3000/theme
HTTP/1.1 303 See Other
location: /
set-cookie: theme=dark; Path=/
content-length: 0


### curl.exe -s -H "Cookie: theme=dark" http://127.0.0.1:3000/
<p>テーマ: dark</p><p>ログイン: （していない）</p>
```

- `POST /theme` の返事に、`set-cookie: theme=dark; Path=/` が付きました。ブラウザは、これを覚えて、次の要求から `Cookie: theme=dark` を付けて送ります（ここでは、curl で自分で付けました）。`jar.add(…)` で予約したクッキーは、ページが 303 を返したときにも、返事に付きます。

次に、セッションです（トークンは、実行するたびに変わります）。

```text
### curl.exe -i -d "name=alice" http://127.0.0.1:3000/login
HTTP/1.1 303 See Other
location: /
set-cookie: __Host-session=HA6WQQJ4_rfGYdRRfOj224DLJkenBzNG75aDu58OfEI%3D; HttpOnly; SameSite=Lax; Secure; Path=/; Max-Age=2592000
content-length: 0

### curl.exe -s -H "Cookie: （上の __Host-session=…）" http://127.0.0.1:3000/
<p>テーマ: light</p><p>ログイン: alice</p>

### curl.exe -s -H "Cookie: （トークンの1文字目を変えたもの）" http://127.0.0.1:3000/
<p>テーマ: light</p><p>ログイン: （していない）</p>

### curl.exe -i -X POST -H "Cookie: （__Host-session=…）" http://127.0.0.1:3000/logout
HTTP/1.1 303 See Other
location: /
set-cookie: __Host-session=; HttpOnly; SameSite=Lax; Secure; Path=/; Max-Age=0; Expires=Thu, 30 Oct 2025 12:26:22 GMT
content-length: 0


### curl.exe -s -H "Cookie: （ログアウトの前のトークン）" http://127.0.0.1:3000/
<p>テーマ: light</p><p>ログイン: （していない）</p>
```

- ログインの返事には、`__Host-session=…` のクッキーが付きました。トークンは 32 バイトの乱数で、`HttpOnly`（JavaScript から読めない）・`SameSite=Lax`（ほかのサイトからの POST には付けない）・`Secure`（HTTPS でだけ送る）・`Path=/`・`Max-Age=2592000`（30 日）が、Topcoat の既定で付きます（第8巻 UNIT 01）。
- そのトークンを送ると「alice」と分かり、**1文字変えると分かりません**でした。記録の鍵は、トークンそのものではなく、そのハッシュ（`TokenHash`）なので、少しでも違うトークンからは、別のハッシュになるからです。
- ログアウトの返事は、`Max-Age=0` と1年前の `Expires` で、クッキーを消させます。そのうえ記録も消したので、**ログアウトの前のトークンを送っても**、もう分かりません。クッキーを消すだけでは、トークンを写し取っておいた人が使えてしまうので、記録も消すのが大事です。

> **注意:** `Secure` のクッキーは、ふつうのブラウザでは HTTPS でしか送られません（`http://127.0.0.1` は、多くのブラウザが例外として扱います）。この例では、curl で自分で `Cookie:` を付けて確かめています。

#### ▼ CSRF を、OriginPolicy が止める

ほかのサイト（`https://evil.example`）のページに、`/theme` へ POST するフォームが仕込まれていた、と考えます。ブラウザは、そのフォームを送るとき、`Origin`（どのサイトのページから送ったか）と `Sec-Fetch-Site: cross-site`（ほかのサイトから）を付けます。curl で、同じヘッダーを付けて試します。

```text
### curl.exe -i -X POST -H "Origin: https://evil.example" -H "Sec-Fetch-Site: cross-site" http://127.0.0.1:3000/theme
HTTP/1.1 403 Forbidden
content-type: text/plain; charset=utf-8
content-length: 9

forbidden


### curl.exe -i -X POST -H "Origin: http://127.0.0.1:3000" -H "Sec-Fetch-Site: same-origin" http://127.0.0.1:3000/theme
HTTP/1.1 303 See Other
location: /
set-cookie: theme=dark; Path=/
content-length: 0



### curl.exe -i -H "Origin: https://evil.example" -H "Sec-Fetch-Site: cross-site" http://127.0.0.1:3000/
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 65

<p>テーマ: light</p><p>ログイン: （していない）</p>
```

- ほかのサイトからの POST は **403** で、テーマは変わりませんでした。同じサイト（`Sec-Fetch-Site: same-origin`）からの POST は通りました。Topcoat のルーターは、すべての要求の最初に OriginPolicy で確かめるので、自分では何も書かなくても、この守りが効いています（RustNote も同じです）。
- **GET は止めません**。ほかのサイトからのリンクで、ページを開くのは、ふつうのことだからです。だから、状態を変える操作（ログアウト・削除・書き換え）を GET にすると、守りが効きません（エラー2）。

### よくあるエラーと読み方

#### エラー1: .cookies() を書き忘れた

`.sessions(…)` だけ書いて、`.cookies()` を書き忘れた場合です（`errs/e10_nocookies.rs`）。

```text
### curl.exe -i http://127.0.0.1:3000/
HTTP/1.1 500 Internal Server Error
content-length: 21

internal server error
```

**▼ サーバーの窓の表示**

```text

thread 'tokio-rt-worker' (209824) panicked at src\bin\e10_nocookies.rs:30:17:
attempted to access request context of type `"topcoat_cookie::CookieJarCell"`, but this type was not registered for this context
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- **何が起きたか:** 「`CookieJarCell` 型の、要求のコンテキストを使おうとしましたが、登録されていません」という panic で、500 になりました。クッキーの入れ物（jar）は、`.cookies()` が要求ごとに用意します。セッションもクッキーでトークンを運ぶので、`.cookies()` が要ります（UNIT 07 のエラー1と同じ形の panic です）。
- **直し方:** `module_router!().cookies().sessions(SessionConfig::default())` と、両方を登録します。RustNote の `build_router` も、この順に書いています（UNIT 03）。

#### エラー2（エラーが出ない間違い）: ログアウトを GET にした

ログアウトを、`<a href="/logout">` のリンクで済ませたくて、`#[page]`（GET）にした場合です（`errs/e10_getlogout.rs`）。ほかのサイトのリンクから開かれた、と考えて試します。

```text
### curl.exe -s -H "Cookie: （alice の __Host-session=…）" http://127.0.0.1:3000/
<p>テーマ: light</p><p>ログイン: alice</p>

### （ほかのサイトのリンクから開かれた、と考える）curl.exe -i -H "Cookie: （同じトークン）" -H "Sec-Fetch-Site: cross-site" http://127.0.0.1:3000/logout
HTTP/1.1 303 See Other
location: /
set-cookie: __Host-session=; HttpOnly; SameSite=Lax; Secure; Path=/; Max-Age=0; Expires=Thu, 30 Oct 2025 12:27:27 GMT
content-length: 0


### curl.exe -s -H "Cookie: （同じトークン）" http://127.0.0.1:3000/
<p>テーマ: light</p><p>ログイン: （していない）</p>
```

- **何が起きたか:** ほかのサイトから開かれた GET なのに、ログアウトが**できてしまいました**。OriginPolicy は GET を止めないからです。`SameSite=Lax` のクッキーも、ほかのサイトのリンクで**ページを開くとき**（GET）には送られるので、本物のブラウザでも、同じことが起きます。ほかのサイトに、あなたのサイトの `/logout` へのリンクを置かれるだけで、押した人がログアウトさせられます（画像の読み込みのような、ページを開く以外の GET には、`SameSite=Lax` のクッキーは付きません）。
- **直し方:** 状態を変える操作は、POST のフォーム（ボタン）にします。RustNote のログアウトも、`#[page(POST)]` です（UNIT 06）。GET は「見るだけ」と決めておけば、OriginPolicy と `SameSite=Lax` が守ってくれます。

#### エラー3（エラーが出ない間違い）: エラーの画面で、何でもつかまえてしまった

`error_page` の最後の `return Err(error);` を、「何でも 500 の画面にする」に変えた場合です（`errs/e10_catchall.rs`）。

```text
### curl.exe -i http://127.0.0.1:3000/old
HTTP/1.1 500 Internal Server Error
content-type: text/html; charset=utf-8
content-length: 145

<header>UNIT 10 のサイト</header><main><h1>時間をおいてもう一度お試しください</h1><p><a href="/">トップへ</a></p></main>
```

- **何が起きたか:** `/old` が、307 で移らずに、500 の「時間をおいてもう一度お試しください」になりました。リダイレクトも `Err` で届くので、何でもつかまえると、**移り先の案内まで、エラーの画面に変わってしまう**のです。ログイン画面へ移す 303（RustNote の `require_user`）も、同じく壊れます。
- **直し方:** 自分の画面にするのは、**知っている種類だけ**にして、ほかは `Err(error)` で返します（例10-1）。500 の画面を出したい種類（問1の `std::io::Error` など）は、名前を挙げて足します。

### RustNote ではここで使う

RustNote のレイアウト（`src/routes.rs`）は、例10-1と同じく、`not_found!()` と `error_boundary` を使っています（抜き出しています）。

```rust
// どのルートにも当たらない URL を NotFoundError にして、レイアウトの
// error_boundary（404 の画面）まで届ける
not_found!();
```

```rust
                <main class="container">
                    // ページで起きたエラーを、ステータスコード付きのエラー画面に変える
                    error_boundary(fallback: |error| error_page(cx, error), (slot))
                </main>
```

エラーの画面を作る `error_page`（`src/views.rs`）は、知っている種類を順に見分けます（抜き出しています）。

```rust
pub fn error_page(cx: &Cx, error: Error) -> Result<impl View + '_> {
    let (status, message) = if let Some(app_error) = error.downcast_ref::<AppError>() {
        log_if_internal(app_error.status_code(), &error);
        (app_error.status_code(), app_error.public_message())
```

- 自分のエラーの型（`AppError`・`RepoError`）と、Topcoat の `NotFoundError`・`BadRequestError` だけを画面にし、ほかは `return Err(error);` で返します（例10-1・エラー3）。`AppError::Internal`（DB に接続できない など）は 500 の画面にし、`log_if_internal` で**窓に記録**してから、利用者には「時間をおいて…」だけを見せます（問1と同じ考え方です。第4巻 UNIT 05）。
- お知らせ（「ノートを作成しました」）は、例10-2のテーマと同じ、ふつうのクッキー（`flash`）で、移った先のページに受け渡しています（`src/flash.rs`。第10巻 UNIT 02）。
- ログインは、例10-2と同じく `session::start`・`token_hash`・`stop` を使い、記録は DB の `sessions` 表に、ハッシュと期限を保存しています（`src/auth.rs`。第8巻 UNIT 05（準備中））。

### 練習問題

**問1:** 例10-1の `/boom` は、窓に何も出ないまま 500 になりました。ファイルの読み込みのような `std::io::Error` を、窓に記録してから、自分の 500 の画面にしてください。`/old` の 307 は、壊さないようにします。

**問2:** 例10-2のテーマのクッキーは、利用者が `theme=dark` と自分で書き換えられます。署名付きのクッキー（`signed_cookies`）にして、書き換えられたクッキーを読まないようにしてください。

**問3:** CSRF を防ぐために、ほかに何を組み合わせているでしょうか。例10-2の `Set-Cookie` と、エラー2を見て考えてください。

### 解答と解説

**問1の解答:**

`error_page` に、`std::io::Error` の場合を足します（`src/bin/u10_q1.rs`。足した所だけを抜き出しています）。

```rust
    } else if let Some(io) = error.downcast_ref::<std::io::Error>() {
        // 中のこと（ファイルの名前など）は、画面に出さずに窓へ書く
        eprintln!("[error] {io}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "時間をおいてもう一度お試しください".to_string(),
        )
    } else {
        return Err(error);
    };
```

```text
### curl.exe -i http://127.0.0.1:3000/boom
HTTP/1.1 500 Internal Server Error
content-type: text/html; charset=utf-8
content-length: 145

<header>UNIT 10 のサイト</header><main><h1>時間をおいてもう一度お試しください</h1><p><a href="/">トップへ</a></p></main>


### curl.exe -i http://127.0.0.1:3000/old
HTTP/1.1 307 Temporary Redirect
location: /
content-length: 0
```

**▼ サーバーの窓の表示**

```text
[error] 指定されたファイルが見つかりません。 (os error 2)
```

- `/boom` は、レイアウト付きの 500 の画面になり、窓には `[error] 指定されたファイルが見つかりません。 (os error 2)` が記録されました。`/old` の 307 は、そのままです。
- 画面には「時間をおいて…」だけを出し、ファイルの名前やエラーの中身は出しません。中のことを見せると、攻撃する人の手がかりになるからです。

**問2の解答:**

鍵（`Key`）を `.app_context(Key::generate())` で登録し、`cookies(cx)` を `signed_cookies(cx)` に変えます（`src/bin/u10_q2.rs`）。

```text
### curl.exe -i -X POST http://127.0.0.1:3000/theme（の set-cookie の行）
set-cookie: theme=asxgipR70vCpIonhi5%2FngvMpGyTNGKrAGb77+CGeG4s%3Ddark; Path=/

### curl.exe -s -H "Cookie: （上の theme=…）" http://127.0.0.1:3000/
<p>テーマ: dark</p><p>ログイン: （していない）</p>

### curl.exe -s -H "Cookie: theme=dark" http://127.0.0.1:3000/（署名を付けずに書き換えた）
<p>テーマ: light</p><p>ログイン: （していない）</p>
```

- 署名付きのクッキーは、`theme=` のあとに、署名（Base64 の 44 文字）と値（`dark`）が並んだ形になりました。値は読めますが、書き換えると、署名が合わなくなります。
- 正しいクッキーを送ると dark、署名の無い `theme=dark` を送ると **light**（読まなかった）でした。`signed_cookies(cx).get(…)` は、署名が合わなければ `None` を返すからです。
- 鍵は、起動するたびに `Key::generate()` で作っているので、再起動すると前のクッキーは読めなくなります。本物のアプリでは、鍵を秘密の値として保存し（第12巻 UNIT 05（準備中））、起動のときに読み込みます。中身を見られたくもないときは、暗号化する `private_cookies` を使います。

**問3の解答:**

- **SameSite=Lax:** セッションのクッキーには `SameSite=Lax` が付いています。ブラウザは、ほかのサイトからの **POST** に、このクッキーを付けません。OriginPolicy が見落とした要求があっても、ログイン中の人として扱われません。
- **状態を変える操作は POST だけ:** エラー2のとおり、GET には `SameSite=Lax` のクッキーが付いてしまい、OriginPolicy も止めません。GET で状態を変えないことで、2つの守りが働く範囲に、すべての操作を入れています。
- **OriginPolicy:** ほかのサイトからの POST を、ルーターの最初で 403 にします。この3つを重ねて、CSRF を防いでいます。RustNote は、3つとも満たしています（第8巻 UNIT 06（準備中））。

### まとめ

- ページの `Err` は、レイアウトの `error_boundary` に届く。`not_found!()` で当たらない URL も届け、知っている種類（404・400）だけを `(status)` 付きの自分の画面にし、リダイレクトなどは `Err(error)` で Topcoat に返す。ルーターのエラーではないものは 500 になるので、記録してから画面にする。
- `cookies(cx)` でクッキーを読み書きし、大事な値は `signed_cookies`（署名）や `private_cookies`（暗号化）にする。セッションは `session::start` で発行し、ハッシュで記録して照合し、`stop` でクッキーと記録の両方を消す。
- CSRF は、OriginPolicy（ほかのサイトからの POST を 403）・`SameSite=Lax`・「状態を変える操作は POST だけ」を重ねて防ぐ。状態を変える操作を GET にすると、守りが効かない。

## この巻のまとめ

この巻の UNIT 01〜10 で、次のことができるようになったかを確かめましょう。できたものに印を付けてください。

**UNIT 01: HTTP の要求・応答・状態コード**

- ☐ HTTP の要求と応答が「1行目・ヘッダー・空行・本文」の文字の並びであることを、標準ライブラリだけのサーバーで確かめられる
- ☐ 要求の1行目（メソッド・パス・版）と、応答の1行目（版・状態コード・説明）を読める
- ☐ `Content-Length` に入れるのが文字数ではなくバイト数である理由と、まちがえたときに本文が切れることを説明できる
- ☐ 200・301・303・403・404・405 の違いと、405 のときに `Allow` を付ける理由を言える
- ☐ GET は見るだけ、POST は変えるという使い分けと、POST の後に 303 で GET へ移す PRG の理由を説明できる
- ☐ `curl.exe` の `-i`・`-d`・`-X`・`-I`・`-L`・`-v` を使い分け、`-v` の `>`・`<`・`*` の行を読み分けられる
- ☐ curl の `(7)`（たどり着けない）と、404（たどり着いたが、ページが無い）を見分けられる

**UNIT 02: Topcoat 0.11 の最小のアプリ**

- ☐ `cargo add topcoat@=0.11.0` と Tokio で、ページが1つのアプリを作って起動できる。`=` で版を固定する理由を言える
- ☐ `#[page("/")]`・`Router::builder().discover().build()`・`topcoat::start(router).await` の役割を説明できる
- ☐ 404・405 と `allow`・HEAD・`Content-Length`・圧縮を、Topcoat が代わりにしてくれることを、`curl.exe` で確かめられる
- ☐ `HOST` と `PORT` の環境変数で待ち受ける場所を変えられる。`PORT` が数でないときのエラーを読める
- ☐ `topcoat::start` が何も表示しないことと、`TcpListener::bind` で押さえてから表示して `topcoat::serve` に渡す書き方を説明できる
- ☐ `.await` の書き忘れ（E0599）、`RouterBuilderDiscoverExt` の `use` の書き忘れ（E0599）、実行ファイルが2つあるときの `--bin` のエラーを読める

**UNIT 03: ルーター・page・route・layout**

- ☐ `module_router!()` を呼んだモジュールが `/` になり、`mod about` が `/about`、`mod blog_posts` が `/blog-posts` になることを説明できる
- ☐ 画面を返す `#[page]` と、文字や JSON を返す `#[route(GET)]` を使い分け、`#[route]` がレイアウトに包まれないことを言える
- ☐ `#[layout]` の `(slot)` の所にページが入ることと、入れ子のレイアウトの順番（広い範囲が外側）を説明できる
- ☐ `#[layer]` で処理の前と後ろをはさみ、見つからない URL はモジュールのレイヤーを通らないことを確かめられる
- ☐ リンク先を `href!` で作る理由と、`is_current` で今のページのリンクに `aria-current` を付ける方法を言える
- ☐ 同じ URL のページが2つあるときの起動時の panic、`#[page]` を付け忘れたときの `E0573`、`(slot)` を書き忘れたときの警告を読める

**UNIT 04: URL の部品（path・query）と Cx**

- ☐ `module_param!(id: u64, error = not_found)` で区切りを `{id}` にし、`path_param::<Id>(cx)?` で読める
- ☐ 型を書かない `module_param!(name)` は、`%XX` を戻した文字が `&str` で読めることを説明できる
- ☐ `#[query_params]` の構造体でクエリを読み、無い・空の項目が `None` になることと、項目を `Option` にする理由を言える
- ☐ 読めなかったときに 404（`not_found`）と 400（`bad_request`）のどちらを返すかを、理由を付けて選べる
- ☐ `FromStr` を書いた自分の型（1 以上の数だけの `NoteId`）で URL を読み、決まりを型に任せられる
- ☐ `cx` からメソッド・URL・ヘッダーを読む関数を書け、`E0308`（`Result` を受け取り忘れ）と `E0277`（`view!` に入れられない型）を読める

**UNIT 05: view!・コンポーネント・HTML のエスケープ**

- ☐ `view!` の中で、決まった文字を `"…"`、変わる値を `( )` で書き、`if`・`for`（`#[key]` 付き）・`match` で画面を組み立てられる
- ☐ 属性の値に `Option` や `bool` の式を入れると、`None`・`false` のときは属性ごと消えることと、`disabled="false"` が押せないボタンになる理由を言える
- ☐ `#[component]` で部品を作り、名前付きの引数と `child` を渡して呼び出せる
- ☐ 利用者の文字が、本文では `<`・`>`・`&`、属性の値では `"`・`&` が書き換えられることを、実際の HTML で確かめられる
- ☐ 属性の名前に危ない文字が来ると panic で止まることと、利用者の文字で名前を作らない理由を説明できる
- ☐ `"…"` の付け忘れ・閉じタグの書き忘れ・`view!` に入れた値の移動（E0382）のエラーを読み、`NodeViewParts` で自分の型を表示できる

**UNIT 06: フォーム・JSON・PRG**

- ☐ `Form<T>` でフォームの本文を構造体として受け取り、`#[derive(Deserialize)]` と `name` をそろえる理由を言える
- ☐ 成功したら `Err(see_other(…).into())` で 303 に移り、失敗したら `(StatusCode::BAD_REQUEST)` で入力を残して出し直せる
- ☐ `#[route(POST)]` と `Json<T>` で JSON を受け取り、`(StatusCode::CREATED, Json(…))` で 201 と JSON を返せる
- ☐ 形の違う JSON・足りない項目・違う `Content-Type`・大きすぎる本文（既定 2 MiB）が、関数の前に 400・413 で断られることを確かめられる
- ☐ PowerShell 5.1 で JSON を送るとき、ファイルに書いて `--data-binary "@ファイル"` を使う理由を言える
- ☐ `Deserialize` の付け忘れ（E0277）・本文の引数を2つ書いたとき・303 を `Ok` で返したとき（E0308）のエラーを読める

**UNIT 07: app_context・memoize**

- ☐ 設定や DB のようにアプリ全体で1つの値を `.app_context(値)` で登録し、`app_context::<T>(cx)` で借りられる
- ☐ 値は型で見分け、同じ型は1つだけ登録できることと、2つ要るときは包む型（newtype）に分ける方法を言える
- ☐ 共有する値が `Send + Sync` でなければならない理由と、書き換える値に `AtomicU64` や `Mutex` を使う理由を説明できる
- ☐ `#[memoize]`（`as_ref`）で、1つの要求の中では同じ問い合わせを1回にまとめ、要求ごとに覚え書きが空から始まることを確かめられる
- ☐ 「ミドルウェアではなく関数」の考え方で、`require_user(cx)` のような関数を必要な所で呼ぶ理由を言える
- ☐ 無くてもよい値を `try_app_context` で借り、`static` ではなく `app_context` を使うとテストで困らない理由を説明できる

**UNIT 08: アセット・CSS・束**

- ☐ `asset!("assets/site.css")` で CSS を宣言し、`view!` の `href` に入れて、束の中の URL にできる
- ☐ `topcoat asset bundle --bin …` で束（ハッシュ付きのファイルと `manifest.toml`）を作り、`AssetBundle::load()` と `.assets(…)` で配れる
- ☐ ファイル名のハッシュが中身から決まることと、`cache-control: …, immutable` で長く覚えてもらえる理由を説明できる
- ☐ `topcoat dev` で、保存するたびにビルド・束・起動をやり直す流れと、`topcoat::dev::script()` の役目を言える
- ☐ 束が無い・パスの打ち間違い・束が古いときのエラーが、それぞれいつ起きるかを説明できる
- ☐ 実行ファイルと束を、いつも同じビルド（同じ `--bin`・同じ `--release`）で作る理由を言える

**UNIT 09: runtime・signal・procedure・shard**

- ☐ `topcoat::runtime::script()`・`.runtime()`・アセットの束で runtime を有効にし、`signal`・`@click`・`:hidden` で、読み直さずに画面を変えられる
- ☐ `$( … )` の式が、サーバーで最初の値を作り、同じ式が JavaScript になってブラウザで計算し直されることを説明できる
- ☐ `#[procedure]` の呼び出しが `POST /api/…` の通信であることを、ブラウザの通信の記録と curl で確かめられる
- ☐ ページの確かめが procedure（と shard）の入口を守らない理由と、中で値と権限を確かめる書き方を言える
- ☐ OriginPolicy が止める要求（ほかのサイトのページから）と、止めない要求（直接の要求）を区別できる
- ☐ 構造体を `signal` に入れたときの `E0277`（`#[record]` で直す）・`.discover()` 忘れの 404・あふれる値の 500 を読める

**UNIT 10: エラー・クッキー・セッション・CSRF**

- ☐ `not_found!()` とレイアウトの `error_boundary` で、404・400 を `(status)` 付きの自分の画面に変えられる
- ☐ リダイレクトなど知らない種類のエラーを `Err(error)` で返す理由と、何でもつかまえると 307・303 が壊れることを説明できる
- ☐ ルーターのエラーではないもの（`std::io::Error` など）が、何も記録されずに 500 になることと、記録してから画面にする方法を言える
- ☐ `cookies(cx)` でクッキーを読み書きし、`signed_cookies` で書き換えを見つけられる
- ☐ `session::start`・`token_hash`・`stop` で、トークンを発行し、ハッシュで記録・照合し、クッキーと記録の両方を消せる
- ☐ CSRF を、OriginPolicy・`SameSite=Lax`・「状態を変える操作は POST だけ」の3つで防ぐ理由を説明できる

## 用語ミニ辞典

この巻（UNIT 01〜10）に出てきた用語です。

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

### UNIT 03 の用語

| 用語 | 意味 |
|---|---|
| モジュールルーティング | モジュールの並びから URL を決める書き方。`module_router!()` を呼んだモジュールが `/` になる |
| `#[route(GET)]` | 文字や JSON などを、そのまま応答にする関数の印。レイアウトに包まれない |
| `#[layout]` | いくつものページを包む共通の外枠。`(slot)` の所にページの中身が入る |
| `Slot` | レイアウトが受け取る、内側のページ（または内側のレイアウト）の中身 |
| `#[layer]` | 要求の処理を前と後ろではさむ関数。`next.run(cx, body).await` で中を動かす |
| `href!` | `#[page]` などの関数の名前から、リンク先の URL を作るマクロ。無いページはコンパイルで見つかる |
| `is_current` | そのリンク先を、今開いているかどうかを返すメソッド |
| `aria-current="page"` | 画面の読み上げソフトに「これが今のページです」と伝える属性 |
| kebab-case | 単語を `-` でつなぐ書き方（`blog-posts`）。モジュール名の `_` は URL では `-` になる |
| 308 Permanent Redirect | 「ずっと、こちらに移りました。同じメソッドで送り直してください」という返事 |
| E0573 | 型が来るはずの所で、型が見つからないエラー（ここでは `#[page]` を付け忘れた） |

### UNIT 04 の用語

| 用語 | 意味 |
|---|---|
| パスパラメーター | URL のパスの中で変わる部分（`/notes/7` の `7`）。Topcoat では `{id}` と書く |
| `module_param!` | モジュールの区切りを `{id}` にし、読むための型（`Id`）を作るマクロ |
| `path_param::<Id>(cx)` | パスパラメーターを読む関数。型を付けたときは `Result` を返す |
| クエリ文字列 | URL の `?` より後ろの、`名前=中身` を `&` でつないだ部分 |
| `#[query_params]` | クエリを読み込む構造体に付ける印。`query_params::<T>(cx)` で読む |
| パーセントエンコーディング | URL に書けない文字を、`%` と2文字の 16 進数で1バイトずつ書く方法（`%E5%A4%AA` など） |
| `error = not_found` / `error = bad_request` | 読めなかったときに、404 / 400 の返事にする指定 |
| `FromStr` | 文字から値を作る trait。`module_param!` は、これで URL の部品を型に変える |
| `Cx` | 今の要求の中身と、アプリで共有する値を持つ入れ物。`cx: &Cx` で受け取る |
| `User-Agent` | 要求を送ったプログラム（ブラウザや curl）の名前を入れるヘッダー |
| E0277 | 必要な trait が実装されていないエラー（ここでは `view!` に入れられない型） |

### UNIT 05 の用語

| 用語 | 意味 |
|---|---|
| `view!` | HTML とほぼ同じ形で画面を組み立てるマクロ。決まった文字は `"…"`、変わる値は `( )` |
| コンポーネント | 何度も使える画面の部品。`#[component]` を付けた async 関数で、名前付きの引数で呼ぶ |
| `Child` | コンポーネントが受け取る中身。呼び出しの引数のあとに書いた部品が入る |
| `#[default]` | 呼ぶ側が省いた引数に、既定の値を使う印 |
| `#[key(…)]` | `for` で並べた1件ずつを見分ける印。DB の ID などを使う |
| `class!` | 条件に合うときだけクラスを足して、`class` の値を作るマクロ |
| ブール属性 | `disabled`・`checked` のように、あるかないかだけで意味が決まる属性。値が `"false"` でも「ある」になる |
| エスケープ | HTML で特別な意味を持つ文字を、ただの文字として表示される書き方に置き換えること |
| `&lt;` / `&gt;` / `&amp;` / `&quot;` | `<`・`>`・`&`・`"` を、ただの文字として HTML に書く書き方 |
| XSS | 利用者の文字に混ぜたプログラムを、ほかの人の画面で動かす攻撃 |
| `NodeViewParts` | 自分の型を、`view!` の `( )` に入れられるようにする trait |
| E0382 | 移動した（持ち主が変わった）値を、もう一度使おうとしたエラー |

### UNIT 06 の用語

| 用語 | 意味 |
|---|---|
| `Form<T>` | フォームの本文（`application/x-www-form-urlencoded`）を、構造体 `T` にして受け取る引数 |
| `Json<T>` | JSON の本文を構造体 `T` にして受け取る引数。返すときは、構造体を JSON にする |
| JSON | `{"title":"milk"}` のように、名前と値の組を書くデータの形。プログラムどうしのやり取りに使う |
| API | プログラムから使うための窓口。ここでは JSON を返す URL |
| `Deserialize` / `Serialize` | serde の trait。本文から構造体を作る／構造体から JSON を作る |
| `see_other` | 303 See Other の返事を作る関数。ページでは `Err(… .into())` で返す |
| 201 Created | 「新しく作りました」という返事 |
| 413 Payload Too Large | 本文が大きすぎて受け取れない、という返事。Topcoat の既定の上限は 2 MiB |
| `Content-Type` | 本文の種類を表すヘッダー。`Json` は `application/json` でなければ断る |
| `100 Continue` | 大きな本文を送る前の「送ってよいですか」に、サーバーが「どうぞ」と答える返事 |
| `BodyLimit` | URL ごとに、受け取る本文の大きさの上限を変えるレイヤー |

### UNIT 07 の用語

| 用語 | 意味 |
|---|---|
| アプリのコンテキスト | アプリ全体で共有する値の置き場。`.app_context(値)` で登録し、型で見分ける |
| `app_context::<T>(cx)` | 登録した値を借りる関数。登録していない型だと panic する |
| `try_app_context::<T>(cx)` | 登録した値を借りる関数。登録していなければ `None` |
| メモ化（`#[memoize]`） | 1つの要求の中で、同じ引数の関数の結果を覚えておき、2回目からは計算しない仕組み |
| `#[memoize(as_ref)]` | 覚えた `Option<T>` や `Result<T, E>` の中身を、借りる形（`Option<&T>` など）で返す指定 |
| ミドルウェア | すべての要求の前後に決まった処理をはさむ仕組み。Topcoat は、代わりに `cx` を受け取る関数を勧める |
| `AtomicU64` | 同時に足しても数が狂わない数の型 |
| `Send` / `Sync` | スレッドをまたいで送れる／同時に借りられる、という印の trait |
| `Router::handle` | サーバーを起動せずに、ルーターに要求を1つ処理させるメソッド。テストで使う |

### UNIT 08 の用語

| 用語 | 意味 |
|---|---|
| アセット | CSS・画像・フォントなど、プログラムと一緒に配る、中身が決まったファイル |
| `asset!` | 配るファイルを宣言するマクロ。実行ファイルには番号（ID）だけが入る |
| 束（バンドル） | `topcoat asset bundle` が作る、ハッシュ付きの名前のファイルと目録のまとまり |
| `manifest.toml` | 束の目録。宣言の ID と、ファイル名・ハッシュ・種類の対応を書く |
| `AssetBundle::load()` | 実行ファイルの隣の `assets/` から、束を読み込む関数 |
| キャッシュ | 受け取ったファイルをブラウザが手元に取っておき、次は取りに行かずに使うこと |
| `cache-control` | どれだけの間、覚えてよいかを伝えるヘッダー。`immutable` は「中身は変わらない」 |
| `topcoat dev` | ファイルの変更を見張り、ビルド・束・起動を自動でやり直す開発用のサーバー |
| `topcoat asset clean` | 作った束を消すコマンド |
| `topcoat asset list` | 束に入る予定のファイルを表示するコマンド |

### UNIT 09 の用語

| 用語 | 意味 |
|---|---|
| runtime | Topcoat がページに入れる、ブラウザの中で動くプログラム。`$( … )` の式をブラウザでも動かす（0.11 でも実験的） |
| signal | ブラウザが持つ状態。値が変わると、使っている式がブラウザで計算し直される |
| `$( … )` | runtime の式。サーバーで最初の値を作り、ブラウザでも同じ計算をする |
| `@click` | クリックされたときに動く、ブラウザの中の処理（イベントハンドラー） |
| `:hidden` などの結び付け属性 | signal が変わるたびに、ブラウザが属性を付け直す属性 |
| procedure | ブラウザから `POST` で呼ばれる、サーバーの async 関数。`#[procedure("/api/…")]` で URL を決められる |
| shard | 引数や signal が変わると、サーバーで描き直される部品。自分の入口（URL）を持つ |
| `#[record]` | 自分の構造体を、signal や procedure で使えるようにする印 |
| OriginPolicy | ほかのサイトのページから送られた、状態を変える要求を 403 で止める Topcoat の仕組み |
| `checked_mul` | 掛け算があふれるときに `None` を返すメソッド。あふれる値を断るのに使う |

### UNIT 10 の用語

| 用語 | 意味 |
|---|---|
| `not_found!()` | どこにも当たらない URL を `NotFoundError` にして、レイアウトまで届けるマクロ |
| `error_boundary` | 中で起きたエラーを、`fallback` の画面に変える部品 |
| `downcast_ref` | エラーが、ある型（`NotFoundError` など）かどうかを確かめて、その型として借りるメソッド |
| 307 Temporary Redirect | 「今は、こちらへ。同じメソッドで送り直して」という返事。`redirect(…)` が返す |
| クッキー | サーバーが `Set-Cookie` で預け、ブラウザが `Cookie` で送り返す、名前と値の組 |
| 署名付きのクッキー | 値の横に署名を付けたクッキー。書き換えると署名が合わず、読まれない |
| セッションのトークン | ログインした人に渡す、32 バイトの乱数。`__Host-session` のクッキーで運ぶ |
| `TokenHash` | トークンの SHA-256。記録には、トークンではなくこれを保存する |
| CSRF | ほかのサイトのページから、ログイン中の人のブラウザに勝手に要求を送らせる攻撃 |
| `Origin` / `Sec-Fetch-Site` | ブラウザが付けるヘッダー。どのサイトのページから送ったかを表す |
| `SameSite=Lax` | ほかのサイトからの POST などには付けず、リンクでページを開くときには付けるクッキーの属性 |

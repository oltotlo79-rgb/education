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
| UNIT 06 | フォーム・JSON・PRG（準備中） | 送られてきた値を型にし、303 で移る |
| UNIT 07 | app_context・memoize（準備中） | DB などを全部の処理で共有し、同じ問い合わせを1回にする |
| UNIT 08 | アセット・CSS・束（準備中） | `asset!` と `topcoat asset bundle` |
| UNIT 09 | runtime・signal・procedure・shard（準備中） | ブラウザの中で動く部分と、その守り方 |
| UNIT 10 | エラー・クッキー・セッション・CSRF（準備中） | 400・404・リダイレクトと、ログインの仕組みの土台 |

> **お知らせ:** この版には UNIT 01〜05 を収録しています。UNIT 06〜10 は準備中で、次の更新から順に追加します。以前の版（Topcoat 0.5 の版）の PDF は、この版がそろうまで残してあります。Topcoat は 0.5 から 0.11 までに書き方が大きく変わっているので、以前の版のコードは、0.11 では動かないことがあります。

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

- だから、何かを**変える**操作は POST（や PUT・DELETE）にして、GET は**見るだけ**にします。POST にしても、ほかのサイトからフォームで送らせる攻撃（CSRF）は残るので、その守りを足します（第8巻 UNIT 01、この巻の UNIT 10（準備中））。

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

- 無い URL（`/nothing`）は `404 Not Found` で、本文は `not found` だけです。ページに名札を付けていない URL は、Topcoat が自動で 404 にします。自分の画面（ヘッダーやトップへのリンクの付いたもの）で 404 を出したいときは、`not_found!()` とレイアウトを使います（UNIT 03・10（準備中））。
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
date: Fri, 30 Oct 2026 11:31:29 GMT

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

- `/api/health` は、`content-type: text/plain; charset=utf-8` で、本文は `ok` の2文字だけでした。`#[route(GET)]` の関数は、`&'static str`（文字）や `Json`（第6巻 UNIT 06（準備中））などを返し、それがそのまま応答になります。**レイアウトにも包まれません**。
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
- `/settings/nothing`（無い URL）の 404 は、**サーバーの窓に表示されていません**。モジュールに書いたレイヤーは、そのモジュールより下で**見つかったルート**を包むので、どこにも当たらない要求は通らないのです。RustNote は、`not_found!()` を書いて、見つからない URL も自分の 404 の画面とアクセスログに通しています（UNIT 10（準備中））。

> **ヒント:** レイヤーは、ログ・時間の計測・決まったヘッダーを足す、のような「どの要求にも同じことをする」仕事に使います。「ログインしているか」の確かめは、レイヤーではなく、ページの最初で関数（RustNote の `require_user`）を呼ぶのが Topcoat の考え方です（第8巻 UNIT 01・UNIT 07（準備中））。

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

- 例3-1と同じく、`module_router!()` で名簿を作っています。`.cookies()`・`.sessions(…)`・`.assets(…)`・`.app_context(…)` は、ページから使う道具を、ルーターに登録しています（UNIT 07・08・10（準備中））。
- `routes.rs` の先頭のコメントには、ファイルと URL の対応の表があります。`routes/notes/id/edit.rs` が `/notes/{id}/edit` になる、のように、**ファイルの並びが、そのまま URL の並び**です。`{id}` の部分は、UNIT 04（準備中）で扱います。

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
- `root_layout` は、`<head>` に CSS へのリンクを入れ、`<body>` の中に、ヘッダー（`site_header`）と、`error_boundary`（ページで起きたエラーを、エラーの画面に変える部品。UNIT 10（準備中））で包んだ `(slot)` を置いています。

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

- `-A` で、curl が送る `User-Agent` を変えました。ページの関数ではなく、そこから呼んだ `request_summary` が、`cx` から読んでいます。**`cx` を渡せば、どの関数でも要求の中身を読める**ので、「要求の中身を1か所で用意して、全部の関数に配る」ような仕組み（ミドルウェア）を作らなくて済みます（UNIT 07（準備中））。

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
- **学べること:** 「どう表示するか」を `view!` が勝手に決めないので、HTML に入る文字は、いつも自分で選んだものになります。エスケープ（危ない文字の書き換え）の話は UNIT 05（準備中）で扱います。

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
- 他人のノートの ID を開いても、無い ID と同じ 404 にしています（第8巻 UNIT 01・第10巻 UNIT 03（準備中））。

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
- HTML の中では `a b&amp;c` になっています。`&` は HTML で特別な意味を持つので、`view!` が `&amp;` に書き換えました（エスケープ。UNIT 05（準備中））。ブラウザには `a b&c` と表示されます。

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

- `for note in notes { note_card(…) }` で、ノートの数だけ `<li>` を作りました。`#[key(note.id)]` は、1件ずつを見分ける印です。並び順が変わっても、同じノートを同じ部品として扱えるようにします（ブラウザの中で動く部分を使うときに大事になります。UNIT 09（準備中））。
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

## この巻のまとめ

この版に収録した UNIT 01〜05 で、次のことができるようになったかを確かめましょう。できたものに印を付けてください。

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

> **お知らせ:** この版には UNIT 01〜05 を収録しています。UNIT 06（フォーム・JSON・PRG）・UNIT 07（app_context・memoize）・UNIT 08（アセット・CSS・束）・UNIT 09（runtime・signal・procedure・shard）・UNIT 10（エラー・クッキー・セッション・CSRF）は準備中で、次の更新から順に追加します。それまでの間、Topcoat を使った完成したアプリの例は、教材のフォルダの `Rust/rustnote/src/routes.rs` から読み始められます。

## 用語ミニ辞典

この巻（UNIT 01〜05）に出てきた用語です。

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

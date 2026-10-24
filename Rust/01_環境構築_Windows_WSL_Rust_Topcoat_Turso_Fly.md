<div style="page-break-after: always; text-align: center; margin: 0; padding: 0;">
<img src="assets/cover.jpg" alt="表紙" style="width: 100%; max-height: 175mm; object-fit: cover; border-radius: 10px;">
<div style="margin-top: 14px; font-size: 11px; letter-spacing: 0.18em; color: #c2410c; font-weight: 700;">VOLUME 01 / RUST BACKEND MASTERY</div>
<div style="font-size: 26px; font-weight: 800; color: #0f172a; margin-top: 6px; line-height: 1.35;">環境構築 — Windows・WSL・Rust・Topcoat・Turso・Fly</div>
<div style="font-size: 13px; color: #475569; margin-top: 8px;">PowerShell・Git・VS Code から Rust・Topcoat・Turso・Fly.io まで、教材のコードを自分の PC で動かす道具をそろえ、つまずいたときに原因を切り分けられるようになる</div>
</div>

# 第1巻: 環境構築 — Windows・WSL・Rust・Topcoat・Turso・Fly

## この巻で身に付けること

このシリーズでは、ノートアプリ **RustNote** を Rust で作り、インターネットに公開します。そのためには、文字を打つ道具（シェル）、変更を記録する道具（Git）、コードを書く道具（VS Code）、Rust そのもの、Web アプリを作る道具（Topcoat）、データベースの道具（Turso）、公開の道具（Fly.io）を、自分の PC にそろえる必要があります。この巻は、その**道具をそろえる巻**です。

道具を入れる作業は、インストーラーの「次へ」を押すだけに見えます。しかし実際には、次のような事故がよく起きます。

- 教材と同じコマンドを打ったのに「用語 'cargo' は、コマンドレット…として認識されません」と言われる
- インターネットの記事のコマンドを貼り付けたら動かない（その記事は PowerShell ではなく bash 向けだった）
- Windows で作ったスクリプトが、Linux（WSL や Docker）では動かない（**改行コード**の違い）
- パスワードを書いたファイルを、うっかり GitHub に上げてしまう
- 作業フォルダの場所が深すぎて、Rust のビルドが `LNK1104` という見慣れないエラーで止まる
- ARM64 の PC に x64 用の道具を入れてしまい、遅かったり、ビルドに失敗したりする
- Rust の版が古くて、Topcoat がビルドできない

どれも、道具の**つながり**を知っていれば防げるものばかりです。「どのシェルで打ったか」「そのコマンドはどこにあるファイルか（PATH）」「PC の CPU は何か」「どの版か」を、自分で確かめられるようになるのが、この巻のいちばんの目標です。

### 到達目標

この巻を終えると、次のことができるようになります。

- 今使っているシェル（PowerShell か bash か）・今いるフォルダ・OS・CPU の種類（ARM64 か x64 か）を、コマンドで確かめられる
- 作業フォルダを、空白・日本語・OneDrive・長すぎるパスを避けた場所に作れる。PowerShell のスクリプトを作って実行でき、実行ポリシーと文字コード（BOM）のエラーを読める
- Git をインストールして初期設定し、`.gitignore` でビルドの成果物と秘密のファイルを除外し、`.gitattributes` で改行コードを決められる
- VS Code と拡張機能 rust-analyzer を入れ、保存のたびに整形とチェックが走るように設定できる
- rustup・rustc・Cargo の役割の違いを説明し、どの版の Rust が使われているかを確かめられる
- Windows でのビルドに必要な Build Tools（リンカー）と、CPU の種類の食い違いによるエラーを切り分けられる
- `cargo fmt`・`cargo clippy`・`cargo test` を、決まった順番で使える
- WSL2 で Linux のコマンドを使い、Windows 側と Linux 側の違い（PATH・改行・ファイルの場所）を説明できる
- Topcoat 0.10 の CLI、Turso CLI、flyctl を入れ、版の表示と簡単な動作確認ができる

### 学習地図

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 400" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-001" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="6" y="30" width="708" height="90" rx="12" fill="#eff6ff"/>
<rect x="6" y="140" width="708" height="90" rx="12" fill="#fff7ed"/>
<rect x="6" y="250" width="708" height="90" rx="12" fill="#ecfdf5"/>
<text x="360" y="20" font-size="13" fill="#475569" text-anchor="middle">上の段から順に進みます（各段の中は左から右へ）</text>
<text x="65" y="70" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">Windows</text>
<text x="65" y="90" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">の土台</text>
<text x="65" y="180" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">Rust の</text>
<text x="65" y="200" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">道具</text>
<text x="65" y="290" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">Linux と</text>
<text x="65" y="310" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">クラウド</text>
<rect x="135" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="200" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 01</text>
<text x="200" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">PowerShell</text>
<text x="200" y="99" font-size="12" fill="#475569" text-anchor="middle">作業場所を決める</text>
<rect x="282" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="347" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 02</text>
<text x="347" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">Git</text>
<text x="347" y="99" font-size="12" fill="#475569" text-anchor="middle">変更と秘密を守る</text>
<rect x="429" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="494" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 03</text>
<text x="494" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">VS Code</text>
<text x="494" y="99" font-size="12" fill="#475569" text-anchor="middle">書く道具をそろえる</text>
<line x1="265" y1="75" x2="280" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-001)"/>
<line x1="412" y1="75" x2="427" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-001)"/>
<rect x="576" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-dasharray="5 4"/>
<text x="641" y="64" font-size="12" fill="#475569" text-anchor="middle">RustNote では</text>
<text x="641" y="84" font-size="12" fill="#475569" text-anchor="middle">.env を Git に</text>
<text x="641" y="102" font-size="12" fill="#475569" text-anchor="middle">入れない</text>
<path d="M494,110 V130 H200 V146" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-001)"/>
<rect x="135" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="200" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 04</text>
<text x="200" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">rustup・Cargo</text>
<text x="200" y="209" font-size="12" fill="#475569" text-anchor="middle">Rust を入れる</text>
<rect x="282" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="347" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 05</text>
<text x="347" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">Build Tools</text>
<text x="347" y="209" font-size="12" fill="#475569" text-anchor="middle">リンカーと CPU</text>
<rect x="429" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="494" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 06</text>
<text x="494" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">品質コマンド</text>
<text x="494" y="209" font-size="12" fill="#475569" text-anchor="middle">fmt・clippy・test</text>
<line x1="265" y1="185" x2="280" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-001)"/>
<line x1="412" y1="185" x2="427" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-001)"/>
<rect x="576" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-dasharray="5 4"/>
<text x="641" y="174" font-size="12" fill="#475569" text-anchor="middle">RustNote では</text>
<text x="641" y="194" font-size="12" fill="#475569" text-anchor="middle">Rust 1.99.0 を</text>
<text x="641" y="212" font-size="12" fill="#475569" text-anchor="middle">フォルダで固定</text>
<path d="M494,220 V240 H200 V256" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-001)"/>
<rect x="135" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="200" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 07</text>
<text x="200" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">WSL2・Docker</text>
<text x="200" y="319" font-size="12" fill="#475569" text-anchor="middle">Linux を動かす</text>
<rect x="282" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="347" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 08</text>
<text x="347" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">Topcoat CLI</text>
<text x="347" y="319" font-size="12" fill="#475569" text-anchor="middle">Web アプリの道具</text>
<rect x="429" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="494" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 09</text>
<text x="494" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">Turso CLI</text>
<text x="494" y="319" font-size="12" fill="#475569" text-anchor="middle">DB の道具</text>
<rect x="576" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="641" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 10</text>
<text x="641" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">flyctl</text>
<text x="641" y="319" font-size="12" fill="#475569" text-anchor="middle">公開の道具と診断</text>
<line x1="265" y1="295" x2="280" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-001)"/>
<line x1="412" y1="295" x2="427" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-001)"/>
<line x1="559" y1="295" x2="574" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-001)"/>
<text x="360" y="372" font-size="13" fill="#475569" text-anchor="middle">各単元の流れ: たとえ話 → 図 → 最小の手順 → 実務の形 → よくあるエラー → 練習問題</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-1: 第1巻の学習地図。Windows の土台（シェル・Git・エディタ）から始めて、Rust の道具、Linux とクラウドの道具へ進む</div>
</div>

| 単元 | テーマ | ひとことで言うと |
|---|---|---|
| UNIT 01 | 作業フォルダと PowerShell | どのシェルで・どこにいて・どんな PC かを確かめ、作業場所を決める |
| UNIT 02 | Git・改行コード・秘密の除外 | 変更を記録する道具を入れ、改行の方言と秘密のファイルの扱いを最初に決める |
| UNIT 03 | VS Code と rust-analyzer | コードを書く机と、横で間違いを教えてくれる先輩を用意する |
| UNIT 04 | rustup・rustc・Cargo（準備中） | Rust を入れて、どの版が使われているかを確かめる |
| UNIT 05 | MSVC Build Tools と CPU の種類（準備中） | Windows でビルドするためのリンカーを入れ、ARM64 と x64 の食い違いを見分ける |
| UNIT 06 | Cargo の品質コマンド（準備中） | `check`・`fmt`・`clippy`・`test` を目的ごとに使い分ける |
| UNIT 07 | WSL2 と Docker（準備中） | Windows の中で Linux を動かし、アプリを箱（コンテナ）に入れる |
| UNIT 08 | Topcoat 0.10 と CLI（準備中） | Web フレームワークの道具を版を固定して入れ、開発サーバーを動かす |
| UNIT 09 | Turso CLI と libSQL の確認（準備中） | データベースの道具を WSL に入れ、Rust から libSQL につながるか確かめる |
| UNIT 10 | flyctl と環境の診断（準備中） | 公開の道具を入れ、ログイン・版・つながりを順番に確かめる |

> **お知らせ:** この版には UNIT 01〜03 を収録しています。UNIT 04〜10 は準備中で、次の更新から順に追加します。本文の中の「UNIT 04 で入れます」のような予告は、これから追加する単元を指しています。

### この巻でそろえる道具の全体像

この巻で入れる道具は、積み木のように重なっています。下の段が無いと、上の段は動きません。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 360" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-002" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="680" height="54" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="360" y="42" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">インターネットのサービス</text>
<text x="360" y="62" font-size="12" fill="#475569" text-anchor="middle">crates.io（Rust の部品）・GitHub（コードの置き場）・Turso Cloud（DB）・Fly.io（公開先）</text>
<rect x="20" y="96" width="430" height="128" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="235" y="118" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">Windows 側の道具</text>
<text x="36" y="144" font-size="12" fill="#0f172a">Git（UNIT 02）</text>
<text x="36" y="166" font-size="12" fill="#0f172a">VS Code・rust-analyzer（UNIT 03）</text>
<text x="36" y="188" font-size="12" fill="#0f172a">rustup → rustc・cargo（UNIT 04）</text>
<text x="36" y="210" font-size="12" fill="#0f172a">Build Tools の link.exe（UNIT 05）</text>
<text x="262" y="144" font-size="12" fill="#0f172a">fmt・clippy・test（UNIT 06）</text>
<text x="262" y="166" font-size="12" fill="#0f172a">topcoat CLI（UNIT 08）</text>
<text x="262" y="188" font-size="12" fill="#0f172a">flyctl（UNIT 10）</text>
<rect x="470" y="96" width="230" height="128" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="585" y="118" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">WSL2 の Linux 側の道具</text>
<text x="486" y="144" font-size="12" fill="#0f172a">Ubuntu と bash（UNIT 07）</text>
<text x="486" y="166" font-size="12" fill="#0f172a">Docker（UNIT 07）</text>
<text x="486" y="188" font-size="12" fill="#0f172a">Turso CLI（UNIT 09）</text>
<rect x="20" y="246" width="430" height="44" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="235" y="273" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">シェル: PowerShell 5.1（UNIT 01）</text>
<rect x="470" y="246" width="230" height="44" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="585" y="273" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">シェル: bash（UNIT 07）</text>
<rect x="20" y="304" width="680" height="40" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="360" y="329" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">Windows 11 と CPU（ARM64 または x64）</text>
<line x1="235" y1="96" x2="235" y2="78" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-002)"/>
<line x1="585" y1="96" x2="585" y2="78" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-002)"/>
<line x1="235" y1="246" x2="235" y2="228" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-002)"/>
<line x1="585" y1="246" x2="585" y2="228" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-002)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-2: この巻でそろえる道具の重なり。どの道具も、その下にあるシェルと OS・CPU の上で動き、上にあるインターネットのサービスとつながる</div>
</div>

- いちばん下は **Windows 11 と CPU** です。CPU には **ARM64**（エイアールエム・ろくじゅうよん）と **x64**（エックス・ろくじゅうよん）の2つの種類があり、道具によっては、それぞれ専用の版を選んで入れます（UNIT 01 で確かめ方を学びます）。
- その上に、文字でお願いを打つ**シェル**があります。Windows 側では PowerShell、WSL2 の Linux 側では bash（バッシュ）です。
- シェルの上で、Git・VS Code・Rust などの道具が動きます。Turso CLI のように、**Windows 版が無く、Linux 側でしか動かない道具**もあります（UNIT 09）。
- 道具の多くは、インターネットのサービスとつながって仕事をします。

### 前提知識（先に読んでおく巻）

- **第0巻（学習ガイド）:** 特に UNIT 02（ファイル・フォルダ・パス）と UNIT 03（端末・シェル・コマンド）。`cd`・`mkdir`・`ls` などの基本のコマンド、絶対パスと相対パス、環境変数と `PATH` の意味を知っていると、この巻がずっと読みやすくなります。
- プログラミングの経験は要りません。Rust のコードは、UNIT 04 以降で少しだけ出てきます。

> **ヒント:** 第0巻を読んでいなくても、この巻は読み進められます。知らない言葉が出てきたら、各単元の「用語」の囲みと、巻末の「用語ミニ辞典」を見てください。

### この巻の進め方

道具を入れる作業は、1つ間違えると、後の道具が全部動かなくなります。各単元では、次の4つの手順を1サイクルとして、**1つずつ入れて、1つずつ確かめる**ように進めてください。

1. **入れる前に確かめる:** まず、版を表示するコマンド（`git --version` など）を打ってみます。「まだ入っていない」こと、または「もう入っている版」を知るためです。
2. **入れる・設定する:** 本文の手順どおりに入れます。インストーラーの画面で迷ったら、本文で特に書いていない項目は既定のままにします。
3. **新しい窓で確かめる:** インストールが終わったら、**PowerShell の窓を閉じて、新しく開き直して**から、もう一度、版を表示するコマンドを打ちます。インストーラーが変えた設定（`PATH` など）は、新しく開いた窓から有効になるからです。
4. **記録する:** 表示された版と、つまずいたときのエラーの最初の数行を、メモ帳などに残しておきます。あとで「何が変わったのか」を比べる手がかりになります。

> **注意:** この巻のコマンドは、**管理者ではない、ふつうの PowerShell** で打ちます。インストーラーが管理者の権限を必要とするときは、インストーラー自身が「このアプリがデバイスに変更を加えることを許可しますか？」と尋ねてきます。ふだんの作業を管理者の窓で行うと、作ったファイルの持ち主が変わったり、うっかりシステムの大事なファイルを変えてしまったりする原因になります。

### この巻の表記

- コマンドの枠は、特に断りがなければ **PowerShell** で打つものです。**WSL の bash** で打つものは、枠の前に「WSL の bash で」と書きます。同じことをするコマンドでも、シェルによって書き方が違うことがあるからです。
- コマンドの枠には、`PS C:\...>` のようなプロンプト（入力を待っている印）は書いていません。枠の中の行を、1行ずつ打って Enter キーを押してください。
- コマンドの枠の中で `#` から始まる行は、説明のための**コメント**です。打たなくてもかまいません（打っても、何も起きません）。
- **▼ 実行結果** の枠は、この本を書いた PC で実際に表示されたものです。ユーザー名などを含む部分は `...` に置き換えています。たとえば `C:\Users\...\rust-study` は、あなたの PC では `C:\Users\（あなたのユーザー名）\rust-study` になります。
- 版の数字（`2.53.0` など）や日付、番号（コミットの `d779d78` など）は、あなたの PC では違う値になることがあります。違っていても、形が同じなら問題ありません。

### 使うツールと版

この巻の手順と実行結果は、次の環境で確かめています。UNIT 04 以降の道具は、各単元で入れます。

| 道具 | 版（この本で確かめたもの） | 使う単元 |
|---|---|---|
| Windows 11 Home | ARM64 版、ビルド 26200。x64 版の Windows は補足で説明します | すべて |
| Windows PowerShell | 5.1.26100.9711（Windows に最初から入っているもの） | すべて |
| Windows Terminal | Windows 11 に最初から入っているもの | すべて |
| Git for Windows | 2.53.0.windows.2（ARM64 版） | UNIT 02〜 |
| Visual Studio Code | 1.129.1（arm64 版） | UNIT 03〜 |
| rust-analyzer（VS Code の拡張機能） | 0.3.3081 | UNIT 03〜 |
| Rust（rustup・rustc・cargo） | rustup 1.29.1。Rust は **1.98 以上**（本書シリーズの基準）、動作確認は 1.99.0 | UNIT 04〜 |
| Build Tools for Visual Studio | MSVC の C++ ビルドツール（ARM64 用）。版と選び方は UNIT 05 で説明します | UNIT 05〜 |
| WSL2 | Ubuntu 24.04.1 LTS | UNIT 07・09 |
| Topcoat / topcoat-cli | 0.10.0（`=0.10.0` で固定） | UNIT 08 |
| Turso CLI | v1.0.34（WSL の中で使う） | UNIT 09 |
| flyctl | v0.4.57（Windows 用は x64 版。ARM64 の PC ではエミュレーションで動く） | UNIT 10 |

> **注意:** このシリーズで使う Topcoat 0.10 は、**Rust 1.98 以上**でないとビルドできません。古い記事の手順で Rust を入れると、版が足りないことがあります。Rust の版の確かめ方と、プロジェクトごとに版を固定する方法は、UNIT 04 で説明します。

## UNIT 01: 作業フォルダと PowerShell

### この単元のゴール

- Windows Terminal で PowerShell を開き、**PowerShell の版・CPU の種類・今いるフォルダ**を、コマンドで確かめられる
- 練習用の作業フォルダ `rust-study` を、空白・日本語・OneDrive・深すぎる場所を避けて作れる
- PowerShell のスクリプト（`.ps1` ファイル）を作って実行できる。**実行ポリシー**と**文字コード（BOM）**のエラーを読んで直せる
- 「x64 のエミュレーションで動いている PowerShell」に気づける。空白を含むパスと、長すぎるパスの問題を避けられる

### たとえ話でつかむ

新しく工房を借りて、ものづくりを始めるところを想像してください。道具を運び込む前に、まず建物の下見をします。

- **建物の種類**を確かめます。木造か鉄筋かで、置ける機械が変わります。これが **OS**（Windows 11）です。
- **電気の規格**を確かめます。家電は、規格に合ったものを選ぶのが基本です。違う規格の家電も、変換アダプターを通せば動くことがありますが、遅くなったり、一部の機械が動かなかったりします。PC の **CPU の種類**（ARM64 か x64 か）が、この電気の規格にあたります。変換アダプターは**エミュレーション**です。
- **作業台を置く場所**を決めます。通路が狭くて物が引っかかる場所（空白や日本語を含む名前のフォルダ）、勝手に荷物を外へ運び出す倉庫（OneDrive の同期フォルダ）、奥の奥すぎて住所が配達伝票の欄に書ききれない場所（長すぎるパス）は避けます。これが**作業フォルダ**です。
- **受付の係員**と話します。あなたのお願い（コマンド）を聞いて、道具を動かしてくれる係員が**シェル**（PowerShell）です。
- 決まった作業は**手順書**に書いておきます。これが**スクリプト**です。この工房には「誰が書いた手順書なら使ってよいか」の決まり（**実行ポリシー**）と、「手順書が何語で書かれているかの印」（**BOM**）があります。

| 工房の下見 | PC での言葉 | この単元で使うもの |
|---|---|---|
| 建物の種類 | OS | Windows 11 |
| 電気の規格 | CPU の種類（アーキテクチャ） | ARM64 / x64 |
| 変換アダプター | エミュレーション | ARM64 の PC で x64 用のアプリを動かす仕組み |
| 作業台の場所 | 作業フォルダ | `C:\Users\（あなたの名前）\rust-study` |
| 受付の係員 | シェル | Windows PowerShell 5.1 |
| 手順書 | スクリプト | `whereami.ps1` |
| 手順書の決まりと言葉の印 | 実行ポリシーと BOM | `RemoteSigned`、UTF-8（BOM 付き） |

> **用語:** CPU の種類（アーキテクチャ：命令の体系のこと。Windows の PC では **x64**（Intel や AMD の CPU）と **ARM64**（Snapdragon などの CPU）が主に使われている。同じ Windows 11 でも、CPU の種類によって、動かせるプログラムのファイルが違う）

<!-- -->

> **用語:** エミュレーション（emulation：ARM64 の PC の上で、x64 用のプログラムを、通訳しながら動かす仕組み。たいていのアプリはそのまま動くが、本来の速さは出ない）

<!-- -->

> **用語:** スクリプト（script：シェルに打つコマンドを、ファイルに並べて書いたもの。PowerShell のスクリプトは `.ps1` という拡張子を付ける）

### 図で理解する

教材どおりに打ったのに結果が違うとき、最初に確かめるのは次の4つです。この単元では、この4つをコマンドで確かめる方法を身に付けます。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-011" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="16" y="16" width="336" height="108" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="32" y="42" font-size="14" font-weight="700" fill="#1e40af">① シェルとその版</text>
<text x="32" y="66" font-size="12" fill="#475569">どの係員に頼んでいるか</text>
<text x="32" y="90" font-size="12" fill="#0f172a" font-family="Consolas, monospace">$PSVersionTable.PSVersion</text>
<text x="32" y="112" font-size="12" fill="#1e40af">この本の PC: 5.1.26100.9711</text>
<rect x="368" y="16" width="336" height="108" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="384" y="42" font-size="14" font-weight="700" fill="#1e40af">② 今いるフォルダ</text>
<text x="384" y="66" font-size="12" fill="#475569">相対パスの出発点はどこか</text>
<text x="384" y="90" font-size="12" fill="#0f172a" font-family="Consolas, monospace">Get-Location</text>
<text x="384" y="112" font-size="12" fill="#1e40af">この本の PC: <tspan font-family="Consolas, monospace">C:\Users\...\rust-study</tspan></text>
<rect x="16" y="140" width="336" height="108" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="32" y="166" font-size="14" font-weight="700" fill="#c2410c">③ OS とその版</text>
<text x="32" y="190" font-size="12" fill="#475569">どんな建物か</text>
<text x="32" y="214" font-size="12" fill="#0f172a" font-family="Consolas, monospace">Get-CimInstance Win32_OperatingSystem</text>
<text x="32" y="236" font-size="12" fill="#c2410c">この本の PC: Windows 11 Home（ビルド 26200）</text>
<rect x="368" y="140" width="336" height="108" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="384" y="166" font-size="14" font-weight="700" fill="#c2410c">④ CPU の種類</text>
<text x="384" y="190" font-size="12" fill="#475569">PC と、シェルの両方を見る</text>
<text x="384" y="214" font-size="12" fill="#0f172a" font-family="Consolas, monospace">$env:PROCESSOR_ARCHITECTURE</text>
<text x="384" y="236" font-size="12" fill="#c2410c">この本の PC: ARM64</text>
<text x="360" y="276" font-size="12" fill="#475569" text-anchor="middle">結果が教材と違ったら、まずこの4つを教材の「使うツールと版」と比べる</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-3: 作業を始める前に確かめる4つのこと。シェルの版・今いるフォルダ・OS・CPU の種類を、それぞれコマンドで確かめられる</div>
</div>

次は、作業フォルダの置き場所です。Rust のビルドは、プロジェクトのフォルダの中に、長い名前のファイルをたくさん作ります。最初に置き場所を間違えると、あとで原因の分かりにくいエラーになります。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-012" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="16" y="16" width="300" height="200" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="166" y="42" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">おすすめの場所</text>
<text x="166" y="72" font-size="13" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">C:\Users\you\rust-study</text>
<text x="40" y="106" font-size="12" fill="#047857">✓ 短い（20 文字ほど）</text>
<text x="40" y="132" font-size="12" fill="#047857">✓ 半角英数字と記号だけ</text>
<text x="40" y="158" font-size="12" fill="#047857">✓ OneDrive の同期フォルダの外</text>
<text x="40" y="184" font-size="12" fill="#047857">✓ 管理者の権限なしで書き込める</text>
<text x="166" y="206" font-size="11" fill="#475569" text-anchor="middle">you の部分は、あなたのユーザー名</text>
<rect x="340" y="16" width="364" height="60" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="356" y="40" font-size="12" fill="#0f172a" font-family="Consolas, monospace">C:\Users\you\OneDrive\Rust 勉強</text>
<text x="356" y="62" font-size="12" fill="#c2410c">✗ 同期される・空白と日本語を含む</text>
<rect x="340" y="86" width="364" height="60" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="356" y="110" font-size="12" fill="#0f172a" font-family="Consolas, monospace">C:\Program Files\rust-study</text>
<text x="356" y="132" font-size="12" fill="#c2410c">✗ 書き込みに管理者の権限が要る・空白を含む</text>
<rect x="340" y="156" width="364" height="60" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="356" y="180" font-size="12" fill="#0f172a" font-family="Consolas, monospace">C:\Users\you\Desktop\work\2026\...</text>
<text x="356" y="202" font-size="12" fill="#c2410c">✗ 深すぎて、パスが長くなりやすい</text>
<text x="16" y="250" font-size="13" font-weight="700" fill="#0f172a">パスの長さ（文字数）</text>
<rect x="16" y="262" width="66" height="26" fill="#10b981"/>
<text x="49" y="280" font-size="11" fill="#ffffff" text-anchor="middle">場所</text>
<rect x="82" y="262" width="250" height="26" fill="#3b82f6"/>
<text x="207" y="280" font-size="11" fill="#ffffff" text-anchor="middle">target の中のファイル（120 文字以上）</text>
<line x1="16" y1="300" x2="700" y2="300" stroke="#475569" stroke-width="1.5" marker-end="url(#ar-01-012)"/>
<line x1="676" y1="254" x2="676" y2="306" stroke="#b91c1c" stroke-width="2" stroke-dasharray="5 4"/>
<text x="670" y="250" font-size="12" fill="#b91c1c" text-anchor="end">260 文字: 多くの道具の上限</text>
<text x="16" y="320" font-size="11" fill="#475569">0</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-4: 作業フォルダの置き場所。短く、半角英数字だけで、同期されない場所に置くと、Cargo が作る長いパスのファイルも上限の 260 文字に収まる</div>
</div>

- この本では、**ホームフォルダ**（`C:\Users\（あなたの名前）`）の直下に `rust-study` を作ります。第0巻で練習用のプロジェクトを作った場所と同じです。
- ユーザー名に日本語や空白が入っている人は、`C:\dev\rust-study` のように、C ドライブの直下に短いフォルダを作って使います。Windows 11 の C ドライブの直下には、管理者の権限が無くても新しいフォルダを作れます（この本の PC で `icacls C:\` を見て、ふつうのユーザーにフォルダを作る権限があることを確かめました）。
- 帯の図は、Rust のプロジェクトのパスの長さのイメージです。この本の PC にある4つの練習用プロジェクトでは、`target` フォルダの中のいちばん長いファイルのパスは、プロジェクトのフォルダから数えて 125〜128 文字でした。場所の部分が長いと、合計がすぐに上限に近づきます。

最後に、スクリプトを読むときの文字コードの話です。Windows PowerShell 5.1 は、スクリプトの先頭を見て、どの文字コードで読むかを決めます。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-013" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="16" y="20" width="180" height="84" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="106" y="46" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">whereami.ps1</text>
<text x="106" y="70" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">EF BB BF 23 20 …</text>
<text x="106" y="92" font-size="11" fill="#047857" text-anchor="middle">先頭に BOM がある</text>
<rect x="16" y="140" width="180" height="84" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="106" y="166" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">whereami.ps1</text>
<text x="106" y="190" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">23 20 E3 81 84 …</text>
<text x="106" y="212" font-size="11" fill="#b91c1c" text-anchor="middle">先頭に BOM がない</text>
<rect x="260" y="32" width="200" height="60" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="58" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">UTF-8 として読む</text>
<text x="360" y="80" font-size="11" fill="#475569" text-anchor="middle">日本語も正しく読める</text>
<rect x="260" y="152" width="200" height="60" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="178" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">Shift_JIS として読む</text>
<text x="360" y="200" font-size="11" fill="#475569" text-anchor="middle">日本語が化け、行がつながる</text>
<rect x="524" y="32" width="180" height="60" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="614" y="66" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">正しく動く</text>
<rect x="524" y="152" width="180" height="60" rx="10" fill="#fee2e2" stroke="#b91c1c" stroke-width="2"/>
<text x="614" y="178" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">構文のエラー</text>
<text x="614" y="200" font-size="11" fill="#475569" text-anchor="middle">トークン '{' を使用できません</text>
<line x1="196" y1="62" x2="256" y2="62" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-013)"/>
<line x1="460" y1="62" x2="520" y2="62" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-013)"/>
<line x1="196" y1="182" x2="256" y2="182" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-013)"/>
<line x1="460" y1="182" x2="520" y2="182" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-013)"/>
<text x="360" y="128" font-size="12" fill="#475569" text-anchor="middle">PowerShell 5.1 は、BOM が無いと日本語の Windows の古い文字コードで読む</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-5: Windows PowerShell 5.1 がスクリプトを読むときの分かれ道。先頭に BOM（EF BB BF）があれば UTF-8、無ければ Shift_JIS として読む</div>
</div>

> **用語:** BOM（ボム：Byte Order Mark。ファイルの先頭に置く3バイト `EF BB BF` の印で、「このファイルは UTF-8 です」と知らせる。画面には表示されない）

### まず動かす最小コード

この単元の「最小コード」は、Rust ではなく PowerShell のコマンドです。

#### 例1-1: PowerShell を開いて、シェル・CPU・今いる場所を確かめる

> **▼ この手順がやること（先に日本語で）:** Windows Terminal を開き、PowerShell に3つのお願いをします。「あなた（PowerShell）の版は？」「あなたはどの CPU 用のプログラムとして動いていますか？」「今いるフォルダはどこですか？」の3つです。

まず、Windows Terminal（ウィンドウズ ターミナル）を開きます。

1. キーボードの Windows キーを押すか、画面の下のスタートボタンを押して、スタートメニューを開きます。
2. そのまま `ターミナル` と入力し、出てきた「ターミナル」を押します。
3. 黒い（または濃い色の）窓が開き、上のタブに「Windows PowerShell」と表示され、`PS C:\Users\（あなたの名前）>` という行が出ていれば準備完了です。

開いた窓で、次の3行を1行ずつ打って Enter キーを押します。

```powershell
# PowerShell の版を表示する
$PSVersionTable.PSVersion
# このシェルが、どの CPU 用のプログラムとして動いているかを表示する
$env:PROCESSOR_ARCHITECTURE
# 今いるフォルダを表示する
Get-Location
```

**▼ 実行結果**

```text
Major  Minor  Build  Revision
-----  -----  -----  --------
5      1      26100  9711

ARM64

Path
----
C:\Users\...
```

#### ▼ 1つずつ分解して解説

##### 解説1: `$PSVersionTable.PSVersion` は PowerShell の版

```powershell
$PSVersionTable.PSVersion
```

- `$PSVersionTable` は、PowerShell が自分自身の情報をまとめて入れている変数です（`$` で始まる名前は変数です）。その中の `PSVersion` が版です。
- `Major`（大きな版）が 5、`Minor`（小さな版）が 1 なので、**Windows PowerShell 5.1** です。Windows 10・11 に最初から入っているもので、この本はこれを使います。`Build` と `Revision` は Windows の更新で増えていくので、あなたの PC では違う数字かもしれません。
- PowerShell には、別にインストールする **PowerShell 7**（コマンド名は `pwsh`）もあります。7 を使っている人は `Major` が 7 になります。多くのコマンドは同じように動きますが、ファイルを書くときの文字コードの既定などが 5.1 と違うので、この本の結果と食い違うことがあります。迷ったら 5.1 を使ってください。

##### 解説2: `$env:PROCESSOR_ARCHITECTURE` は「このシェルの」CPU の種類

```powershell
$env:PROCESSOR_ARCHITECTURE
```

- `$env:` で始まる名前は、**環境変数**（第0巻 UNIT 03）を読む書き方です。`PROCESSOR_ARCHITECTURE` には、**このプログラム（シェル）がどの CPU 用として動いているか**が入っています。
- `ARM64` なら ARM64 用、`AMD64` なら x64 用です。x64 は、AMD という会社が最初に作った仕組みなので、Intel の CPU の PC でも `AMD64` と表示されます。
- ここで大事なのは、「PC の CPU」と「シェルの CPU」が同じとは限らないことです。ARM64 の PC でも、x64 用のアプリから PowerShell を起動すると、PowerShell が x64 のエミュレーションで動き、`AMD64` と表示されます（「よくあるエラー」のエラー4で実際に見ます）。PC そのものの CPU は、例1-3のスクリプトで調べます。

##### 解説3: `Get-Location` は今いるフォルダ

```powershell
Get-Location
```

- 開いたばかりの PowerShell は、**ホームフォルダ**（`C:\Users\（あなたの名前）`）にいます。プロンプトの `PS C:\Users\...>` にも同じ場所が出ています。
- ホームフォルダは、PowerShell では `~`（チルダ）という短い書き方でも表せます。`cd ~` と打つと、どこにいてもホームフォルダに戻れます。

> **ヒント:** x64 の Windows の PC では、`$env:PROCESSOR_ARCHITECTURE` は `AMD64` と表示されます。それで正しい状態です。この本で「ARM64 の PC では」と書いてある注意は、x64 の PC の人は読み飛ばしてかまいません。

#### 例1-2: 作業フォルダ rust-study を作る

> **▼ この手順がやること（先に日本語で）:** ホームフォルダに戻ってから、練習用の作業フォルダ `rust-study` を作り、その中へ移動します。最後に、本当にそこへ移動できたかを確かめます。

```powershell
# ホームフォルダへ移動する
cd ~
# 作業フォルダ rust-study を作る
mkdir rust-study
# その中へ移動する
cd rust-study
# 今いるフォルダを確かめる
Get-Location
```

**▼ 実行結果**

```text


    ディレクトリ: C:\Users\...


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d-----        2026/10/17     11:49                rust-study


Path
----
C:\Users\...\rust-study
```

第0巻で `rust-study` をすでに作った人は、`mkdir` の行で次のエラーが出ます。フォルダはもうあるので、気にせず次の `cd rust-study` へ進んでください。

```text
mkdir : 指定された名前 C:\Users\...\rust-study の項目は既に存在します。
発生場所 行:1 文字:1
+ mkdir rust-study
+ ~~~~~~~~~~~~~~~~
    + CategoryInfo          : ResourceExists: (C:\Users\...\rust-study:String) [New-Item], IOException
    + FullyQualifiedErrorId : DirectoryExist,Microsoft.PowerShell.Commands.NewItemCommand
```

#### ▼ 1つずつ分解して解説

##### 解説1: `mkdir` はフォルダを作り、作ったものを一覧で見せる

- `mkdir rust-study` は「今いるフォルダの中に `rust-study` というフォルダを作って」というお願いです。PowerShell は、作ったフォルダを一覧の形で表示します。
- `Mode` 列の `d` は、それがフォルダ（directory）であることの印です。`LastWriteTime` は作った日時です。

##### 解説2: なぜホームフォルダの直下なのか

- ホームフォルダは、**あなたのユーザーが自由に書き込める**場所です。`C:\Program Files` のような場所は、書き込むのに管理者の権限が要ります。
- **パスが短く**なります。図で見たとおり、Rust のビルドはプロジェクトの中に長い名前のファイルを作るので、置き場所は短いほど安全です。
- **OneDrive の同期フォルダの外**です。Windows の設定によっては、「デスクトップ」や「ドキュメント」が OneDrive の同期フォルダの中に移されていることがあります。同期フォルダの中は、ファイルを書き換えるたびにインターネット上へ送られます。ビルドのたびに何千ものファイルを作る Rust のプロジェクトを置く場所には向きません。

##### 解説3: 自分のユーザー名に日本語や空白が入っているとき

- ホームフォルダのパスは、ユーザー名で決まります。たとえば `C:\Users\山田 太郎` のように日本語や空白が入っていると、一部の道具が正しく扱えないことがあります（日本語と空白が入ったフォルダでの様子は、例1-4で確かめます）。
- その場合は、次のように C ドライブの直下に短いフォルダを作り、以後の `~\rust-study` を `C:\dev\rust-study` に読み替えてください。

```powershell
# C ドライブの直下に dev フォルダを作り、その中に作業フォルダを作る
mkdir C:\dev\rust-study
cd C:\dev\rust-study
```

> **ヒント:** エクスプローラーでこのフォルダを開くには、PowerShell で `explorer .` と打ちます。`.` は「今いるフォルダ」という意味です。

### もう一歩: 実務に近い形

例1-1で確かめた3つに、OS・PC の CPU・作業フォルダの注意点を加えて、**1回で全部を表示するスクリプト**を作ります。新しい PC を用意したときや、エラーの相談をするときに、この表示を貼り付ければ、自分の環境を正確に伝えられます。

#### 例1-3: 自分の環境を1画面にまとめるスクリプト whereami.ps1

> **▼ この手順がやること（先に日本語で）:** `rust-study` フォルダに、スクリプト `whereami.ps1` を作ります。中身は、シェルの版・今いるフォルダ・OS・PC の CPU・このシェルの CPU を表示し、さらに「今いるフォルダのパスに空白や日本語が入っていないか」「OneDrive の同期フォルダの中ではないか」を調べて、OK か注意かを表示するものです。スクリプトを実行できるように設定を1つ変えてから、実行します。

**手順1: スクリプトのファイルを作る。** `rust-study` フォルダにいることを確かめてから、次のコマンドをまとめて貼り付けて Enter キーを押します。1行目の `@'` から最後の `'@` までが、ファイルに書き込む中身です。

```powershell
Set-Content -Path whereami.ps1 -Encoding utf8 -Value @'
# いま「どのシェルで・どこにいて・どんな PC か」を1画面にまとめて表示する
$os = Get-CimInstance Win32_OperatingSystem
$cpu = Get-CimInstance Win32_Processor | Select-Object -First 1

# CPU の種類の番号を名前に直す（9 = x64、12 = ARM64）
$machine = switch ($cpu.Architecture) {
    9 { "x64" }
    12 { "ARM64" }
    default { "その他 ($($cpu.Architecture))" }
}

Write-Output "シェル          : PowerShell $($PSVersionTable.PSVersion)"
Write-Output "現在のフォルダ  : $((Get-Location).Path)"
Write-Output "OS              : $($os.Caption) (ビルド $($os.BuildNumber))"
Write-Output "PC の CPU       : $machine"
Write-Output "このシェルの種類: $env:PROCESSOR_ARCHITECTURE"

# 作業フォルダのパスに空白や日本語（半角英数字と記号以外の文字）が入っていないかを調べる
$here = (Get-Location).Path
if ($here -match '[^\x21-\x7E]') {
    Write-Output "パスの文字      : 注意（空白や日本語が含まれています）"
} else {
    Write-Output "パスの文字      : OK（半角英数字と記号だけ）"
}

# OneDrive の同期フォルダの中かどうかを調べる
if ($env:OneDrive -and $here.StartsWith($env:OneDrive)) {
    Write-Output "OneDrive        : 注意（同期フォルダの中です）"
} else {
    Write-Output "OneDrive        : OK（同期フォルダの外です）"
}
'@
```

貼り付けている間、行の先頭に `>>` が表示されます。これは「まだ続きがありますね」という PowerShell の合図です。最後の `'@` の行で Enter キーを押すと、ファイルが作られます（成功しても何も表示されません）。

**手順2: 自分で作ったスクリプトを実行できるようにする。** 何も設定していない Windows では、安全のためにスクリプトの実行が止められています。次のコマンドで、「自分の PC で作ったスクリプトは実行してよい」という設定に変えます。

```powershell
# 自分のユーザーだけ、実行ポリシーを RemoteSigned にする
Set-ExecutionPolicy -Scope CurrentUser RemoteSigned
```

確認の質問が表示されたら、`Y` を打って Enter キーを押します。設定できたかは、次のコマンドで確かめます。

```powershell
# 範囲（Scope）ごとの実行ポリシーを一覧で表示する
Get-ExecutionPolicy -List
```

**▼ 実行結果**

```text

        Scope ExecutionPolicy
        ----- ---------------
MachinePolicy       Undefined
   UserPolicy       Undefined
      Process       Undefined
  CurrentUser    RemoteSigned
 LocalMachine       Undefined

```

`CurrentUser` の行が `RemoteSigned` になっていれば成功です。

**手順3: スクリプトを実行する。**

```powershell
# 今いるフォルダ（.）の whereami.ps1 を実行する
.\whereami.ps1
```

**▼ 実行結果**

```text
シェル          : PowerShell 5.1.26100.9711
現在のフォルダ  : C:\Users\...\rust-study
OS              : Microsoft Windows 11 Home (ビルド 26200)
PC の CPU       : ARM64
このシェルの種類: ARM64
パスの文字      : OK（半角英数字と記号だけ）
OneDrive        : OK（同期フォルダの外です）
```

「PC の CPU」と「このシェルの種類」が同じで、下の2行が両方 OK なら、作業を始める準備ができています。

#### ▼ 1つずつ分解して解説

##### 解説1: `Set-Content -Encoding utf8` と `@' ... '@` でファイルを作る

```powershell
Set-Content -Path whereami.ps1 -Encoding utf8 -Value @'
（中身）
'@
```

- `Set-Content` は、ファイルに文字を書き込むコマンドです（第0巻 UNIT 03）。`-Path` が書き込むファイル、`-Value` が書き込む中身です。
- `@'` と `'@` ではさんだ部分は、**何行あっても、そのまま1つの文字列**として扱われます（ヒア文字列と呼びます）。`'`（シングルクォート）を使ったヒア文字列の中では、`$os` のような変数も、書いたとおりの文字のまま残ります。スクリプトの中身を書き込むのに、ちょうどよい書き方です。
- `@'` は行の最後に、`'@` は行の先頭に書く決まりです。`'@` の前に空白があると、終わりの印として認識されません。
- `-Encoding utf8` を付けると、Windows PowerShell 5.1 は **BOM 付きの UTF-8** でファイルを書きます。図で見たとおり、日本語を含むスクリプトを 5.1 で正しく読ませるには、この BOM が必要です。

##### 解説2: 実行ポリシーと `RemoteSigned`

```powershell
Set-ExecutionPolicy -Scope CurrentUser RemoteSigned
```

- **実行ポリシー**は、「どんなスクリプトなら実行してよいか」を決める PowerShell の設定です。何も設定していない Windows では `Restricted`（制限付き）になっていて、**スクリプトは1つも実行できません**。メールなどで受け取った怪しいスクリプトを、うっかり実行しないための守りです。
- `RemoteSigned`（リモート・サインド）は、「**自分の PC で作ったスクリプトは実行してよい。インターネットから取ってきたスクリプトは、作った人の署名が付いているものだけ実行してよい**」という設定です。開発の作業では、これがよく使われます。
- `-Scope CurrentUser` は「**今のユーザーだけ**に設定する」という意味です。PC 全体の設定ではないので、管理者の権限は要りません。
- `Get-ExecutionPolicy -List` の表は、上の行ほど優先されます。`Undefined` は「この範囲では決めていない」という意味です。`CurrentUser` だけが `RemoteSigned` なので、結果として `RemoteSigned` が使われます。

> **注意:** インターネットから取ってきたスクリプトを実行するときは、中身を読んで、何をするものかを理解してからにしましょう。実行ポリシーは「うっかり」を防ぐための仕組みで、悪意のあるスクリプトを完全に防ぐものではありません。

##### 解説3: `.\whereami.ps1` の `.\` は「今いるフォルダの」

```powershell
.\whereami.ps1
```

- PowerShell は、`whereami.ps1` とだけ打っても、今いるフォルダのスクリプトを実行しません。`PATH` に書かれたフォルダだけを探すからです。同じ名前の別のコマンドと取り違えないための決まりです。
- 今いるフォルダのスクリプトを実行するときは、`.\`（今いるフォルダの）を付けて、場所をはっきり書きます。

##### 解説4: スクリプトの中身を読む

```powershell
$os = Get-CimInstance Win32_OperatingSystem
$cpu = Get-CimInstance Win32_Processor | Select-Object -First 1
```

- `Get-CimInstance` は、Windows に PC の情報を問い合わせるコマンドです。`Win32_OperatingSystem` は OS の情報、`Win32_Processor` は CPU の情報です。結果を変数 `$os` と `$cpu` に入れています。
- `$os.Caption` は OS の名前、`$os.BuildNumber` は**ビルド番号**（Windows の更新の段階を表す番号）です。
- `$cpu.Architecture` は、**PC の CPU そのもの**の種類を表す番号です。`switch` で、9 なら `x64`、12 なら `ARM64` という名前に直しています。シェルがエミュレーションで動いていても、こちらは本物の CPU を表します。

```powershell
if ($here -match '[^\x21-\x7E]') {
```

- `-match` は「左の文字列が、右の**パターン**に当てはまるか」を調べる書き方です。`[^\x21-\x7E]` は「半角の英数字と記号（文字の番号 0x21〜0x7E）**以外**の文字」というパターンで、空白（0x20）や日本語が1文字でも入っていると当てはまります。
- `$env:OneDrive` は、OneDrive を使っている PC で、同期フォルダの場所が入っている環境変数です。今いるフォルダのパスが、その場所から始まっていれば（`StartsWith`）、同期フォルダの中です。

#### 例1-4: 場所を変えて実行し、「注意」を出してみる

> **▼ この手順がやること（先に日本語で）:** 同じスクリプトを、わざと良くない場所で実行して、「注意」と表示されることを確かめます。1つ目は、空白と日本語を含む名前のフォルダ `Rust の勉強`、2つ目は OneDrive の同期フォルダです。

```powershell
# 空白と日本語を含む名前のフォルダを作って移動し、スクリプトを実行する
mkdir "Rust の勉強"
cd "Rust の勉強"
..\whereami.ps1
```

**▼ 実行結果**（`mkdir` の表示は省いています）

```text
シェル          : PowerShell 5.1.26100.9711
現在のフォルダ  : C:\Users\...\rust-study\Rust の勉強
OS              : Microsoft Windows 11 Home (ビルド 26200)
PC の CPU       : ARM64
このシェルの種類: ARM64
パスの文字      : 注意（空白や日本語が含まれています）
OneDrive        : OK（同期フォルダの外です）
```

OneDrive を使っている人は、同期フォルダへ移動して実行してみます。

```powershell
# OneDrive の同期フォルダへ移動して、rust-study のスクリプトを実行する
cd $env:OneDrive
~\rust-study\whereami.ps1
```

**▼ 実行結果**

```text
シェル          : PowerShell 5.1.26100.9711
現在のフォルダ  : C:\Users\...\OneDrive
OS              : Microsoft Windows 11 Home (ビルド 26200)
PC の CPU       : ARM64
このシェルの種類: ARM64
パスの文字      : OK（半角英数字と記号だけ）
OneDrive        : 注意（同期フォルダの中です）
```

確かめたら、作業フォルダに戻って、練習用に作ったフォルダを消しておきます。

```powershell
# 作業フォルダへ戻り、練習用のフォルダを消す
cd ~\rust-study
Remove-Item "Rust の勉強"
```

#### ▼ 1つずつ分解して解説

##### 解説1: 空白を含む名前は `"` で囲む

- `mkdir "Rust の勉強"` のように、空白を含む名前は `"` で囲みます。囲まないと、PowerShell は空白でお願いを区切って、`Rust` と `の勉強` の2つの名前だと受け取ってしまいます（「よくあるエラー」のエラー3）。
- `..\whereami.ps1` の `..\` は「1つ上のフォルダの」という意味です。`Rust の勉強` の1つ上は `rust-study` なので、さっき作ったスクリプトを指しています。

##### 解説2: スクリプトは「どこに置いてあるか」と「どこで実行したか」が別

- 2つ目の例では、スクリプトは `~\rust-study` に置いたまま、今いるフォルダだけを OneDrive に変えて実行しました。スクリプトが調べる「現在のフォルダ」は、**スクリプトの置き場所ではなく、実行したときに今いたフォルダ**です。
- Rust のプログラムでも同じことが起きます（第0巻 UNIT 02 の「実行する場所が変わると、相対パスの行き先も変わる」）。「どのフォルダで実行したか」は、いつも結果に関わる、と覚えておきましょう。

### よくあるエラーと読み方

#### エラー1: 「このシステムではスクリプトの実行が無効になっているため」（実行ポリシー）

例1-3の手順2（`Set-ExecutionPolicy`）を飛ばしてスクリプトを実行すると、次のエラーになります。

```text
.\whereami.ps1 : このシステムではスクリプトの実行が無効になっているため、ファイル C:\Users\...\rust-study\whereami.ps1 を読み込むことができません。詳細については、「about_Execution_Policies」(https://go.microsoft.com/fwlink/?LinkID=135170) を参照してください。
発生場所 行:1 文字:1
+ .\whereami.ps1
+ ~~~~~~~~~~~~~~
    + CategoryInfo          : セキュリティ エラー: (: ) []、PSSecurityException
    + FullyQualifiedErrorId : UnauthorizedAccess
```

- **何が起きたか:** 実行ポリシーが `Restricted` のままなので、PowerShell がスクリプトの実行を断りました。`CategoryInfo` の `セキュリティ エラー` と、`UnauthorizedAccess`（許可されていない操作）が、その理由を表しています。スクリプトの中身が間違っているわけではありません。
- **直し方:** 例1-3の手順2のとおり、`Set-ExecutionPolicy -Scope CurrentUser RemoteSigned` を実行してから、もう一度スクリプトを実行します。

> **ヒント:** エラーの長い1行目は、PowerShell の窓の幅で折り返して表示されます。単語の途中で折り返されることもありますが、中身は同じです。

#### エラー2: 「式またはステートメントのトークン '{' を使用できません」（BOM なしで保存したスクリプト）

メモ帳や VS Code で新しくスクリプトを書いて、ふつうに保存すると、ファイルは **BOM なしの UTF-8** になります。日本語を含むスクリプトを BOM なしで保存して実行すると、次のエラーになります。

```text
発生場所 C:\Users\...\whereami.ps1:6 文字:7
+     9 { "x64" }
+       ~
式またはステートメントのトークン '{' を使用できません。
発生場所 C:\Users\...\whereami.ps1:7 文字:8
+     12 { "ARM64" }
+        ~
式またはステートメントのトークン '{' を使用できません。
発生場所 C:\Users\...\whereami.ps1:9 文字:1
+ }
+ ~
式またはステートメントのトークン '}' を使用できません。
発生場所 C:\Users\...\whereami.ps1:28 文字:1
+ }
+ ~
式またはステートメントのトークン '}' を使用できません。
    + CategoryInfo          : ParserError: (:) [], ParseException
    + FullyQualifiedErrorId : UnexpectedToken
```

- **何が起きたか:** スクリプトを1行も実行する前に、**読み込みの段階**（`ParserError`：構文の解析のエラー）で失敗しています。BOM が無いので、PowerShell 5.1 はファイルを Shift_JIS（日本語の Windows の古い文字コード）として読みました。
- 文字化けしただけなら、コメントの中なので害はありません。困るのは、5行目のコメントの最後の文字 `）` を作るバイトの一部が、**次の改行のバイトと組になって、1つの文字として読まれてしまう**ことです。その結果、6行目の `$machine = switch ($cpu.Architecture) {` がコメントの続きになって消え、`9 { "x64" }` の `{` が「ここにあるはずのない記号」になりました。エラーの `6 文字:7` は「6行目の7文字目」という意味で、行がつながった後の数え方です（この本の PC で、ファイルを Shift_JIS として読み直して、5行目と6行目が1行につながることを確かめました）。
- **直し方:** スクリプトを **BOM 付きの UTF-8** で保存し直します。PowerShell で作るなら、例1-3のように `Set-Content -Encoding utf8` を使います。VS Code で保存し直す方法は UNIT 03 で説明します。スクリプトの中に日本語を書かない（コメントも英語にする）のも1つの方法です。

> **ヒント:** PowerShell 7 は、BOM が無いファイルを UTF-8 として読みます。このエラーは Windows PowerShell 5.1 に特有のものです。

#### エラー3: 「引数 'Files' を受け入れる位置指定パラメーターが見つかりません」（空白を含むパス）

```powershell
cd C:\Program Files
```

```text
Set-Location : 引数 'Files' を受け入れる位置指定パラメーターが見つかりません。
発生場所 行:1 文字:1
+ cd C:\Program Files
+ ~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : InvalidArgument: (:) [Set-Location]、ParameterBindingException
    + FullyQualifiedErrorId : PositionalParameterNotFound,Microsoft.PowerShell.Commands.SetLocationCommand
```

- **何が起きたか:** PowerShell はお願いを空白で区切るので、`cd` に `C:\Program` と `Files` の2つを渡したことになりました。`cd` の正式名は `Set-Location` で、行き先は1つしか受け取れません。2つ目の `Files` の置き場所（位置指定パラメーター）が無い、というエラーです。
- **直し方:** パスを `"` で囲みます。

```powershell
cd "C:\Program Files"
Get-Location
```

**▼ 実行結果**

```text
Path
----
C:\Program Files
```

> **ヒント:** `cd C:\Prog` まで打って Tab キーを押すと、PowerShell が残りを補って、空白を含むパスを自動で `'` で囲んでくれます。打ち間違いも防げるので、パスはなるべく Tab キーで補いましょう。

#### エラー4（エラーが出ない落とし穴）: 「このシェルの種類: AMD64」（x64 として動いている PowerShell）

ARM64 の PC で、x64 用のアプリから PowerShell を起動すると、PowerShell もエミュレーションで動きます。次は、この本の PC で、x64 用のプログラムである Git Bash（UNIT 02 で入る、Git に付いてくる bash）から `powershell` と打って起動し、同じスクリプトを実行した結果です。

```text
シェル          : PowerShell 5.1.26100.9711
現在のフォルダ  : C:\Users\...\rust-study
OS              : Microsoft Windows 11 Home (ビルド 26200)
PC の CPU       : ARM64
このシェルの種類: AMD64
パスの文字      : OK（半角英数字と記号だけ）
OneDrive        : OK（同期フォルダの外です）
```

- **何が起きたか:** 「PC の CPU」は `ARM64` なのに、「このシェルの種類」が `AMD64` になっています。エラーは出ませんが、PowerShell が x64 のエミュレーションで動いている状態です。この PowerShell から起動したプログラムも、x64 の世界の中で動きます。
- **なぜ困るのか:** 道具を入れたりビルドしたりするときに、PC の CPU（ARM64）とシェルの CPU（x64）が食い違うと、遅くなるだけでなく、UNIT 05 で扱う「CPU の種類の食い違い」によるエラーの入り口になります。
- **直し方:** スタートメニューから開いた Windows Terminal の PowerShell を使います。例1-1の `$env:PROCESSOR_ARCHITECTURE` で、PC と同じ `ARM64` と表示されることを確かめてから作業しましょう。

> **注意:** Git for Windows の ARM64 版でも、Git Bash の bash そのものは x64 用のプログラムでした（この本の PC で、`C:\Program Files\Git\usr\bin\bash.exe` が x64 用のファイルであることを確かめました）。この本では Git Bash は使わず、PowerShell と、UNIT 07 で入れる WSL の bash を使います。

#### エラー5（あとで出るエラーの予告）: `LNK1104: ファイル ... を開くことができません`（長すぎるパス）

このエラーは、UNIT 04 で Rust を入れた後、ビルドしたときに出るものです。原因が「作業フォルダの場所」にあるので、ここで紹介しておきます。今は読むだけで大丈夫です。

Cargo（Rust のビルドの道具）には、ビルドの結果を置くフォルダ（ふだんはプロジェクトの中の `target`）を、環境変数 `CARGO_TARGET_DIR` で別の場所に変える仕組みがあります。この本の PC で、それを使って結果の置き場所をわざと深い場所にし、小さなプログラム `hello-rust` をビルドしたところ、次のエラーになりました（パスはとても長いので、途中を `...` に置き換えています）。

```text
   Compiling hello-rust v0.1.0 (...\hello-rust)
error: linking with `link.exe` failed: exit code: 1104
  |
  = note: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2022\\BuildTools\\VC\\Tools\\MSVC\\14.44.35207\\bin\\HostARM64\\arm64\\link.exe" "/NOLOGO" ...
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: LINK : fatal error LNK1104: ファイル '...\debug\deps\hello_rust.exe' を開くことができません。


error: could not compile `hello-rust` (bin "hello-rust") due to 1 previous error
```

- **何が起きたか:** Rust のビルドの最後の段階で、**リンカー**（`link.exe`：部品をつないで実行ファイルを作る道具。UNIT 05 で入れます）が、作ろうとした実行ファイル `hello_rust.exe` を開けませんでした。`LNK1104` は、リンカーのエラーの番号です。
- **原因:** パスが長すぎました。置き場所の深さを1文字ずつ変えて試したところ、作る実行ファイルのパスの長さで、結果がはっきり分かれました。

| 実行ファイルのパスの長さ | 結果 |
|---|---|
| 255 文字 | 成功 |
| 259 文字 | 成功 |
| 260 文字 | `LNK1104` で失敗 |
| 261 文字 | `LNK1104` で失敗 |

- Windows の多くのプログラムは、昔からの決まりで、**パスを 259 文字まで**しか扱えません（終わりの印を含めて 260 文字分の場所を用意しているためです）。Windows には、この上限を外す設定（`LongPathsEnabled`）があり、この本の PC ではその設定が有効（`1`）でしたが、それでもリンカーは長いパスを開けませんでした。設定を変えても、**道具の側が対応していなければ効かない**のです。
- **直し方:** 作業フォルダを短い場所に置きます。この単元で作った `C:\Users\（あなたの名前）\rust-study` なら、プロジェクトの中の長いファイルを足しても、上限まで十分な余裕があります。

> **ヒント:** 長いパスの設定がどうなっているかは、`(Get-ItemProperty -Path HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem -Name LongPathsEnabled).LongPathsEnabled` で確かめられます。この本の PC では `1` と表示されました。ただし上のとおり、この設定に頼らず、短い場所で作業するのがいちばん確実です。

### RustNote ではここで使う

この単元で学んだ「今いるフォルダ」は、RustNote を動かすときにも、そのまま効いてきます。RustNote のフォルダ（`Rust/rustnote`）には `rust-toolchain.toml` というファイルがあり、中には次のように書かれています。

```toml
# このフォルダの中で cargo を実行すると、Rust 1.99.0 が使われる
# （既定のツールチェーンは変更しない）
[toolchain]
channel = "1.99.0"
components = ["rustfmt", "clippy"]
```

このファイルがあると、**RustNote のフォルダの中で実行したときだけ**、Rust 1.99.0 が使われます。この本の PC で、RustNote のフォルダとホームフォルダで、それぞれ「今使われる Rust の版」を表示すると、次のようになりました（Rust を入れるのは UNIT 04 です。ここでは結果だけ見てください）。

```powershell
# RustNote のフォルダ（Rust/rustnote）の中で、使われる Rust の版を表示する
rustup show active-toolchain
cargo --version
# ホームフォルダで、同じことをする
cd ~
rustup show active-toolchain
cargo --version
```

**▼ 実行結果**

```text
1.99.0-aarch64-pc-windows-msvc (overridden by 'C:\Users\...\rustnote\rust-toolchain.toml')
cargo 1.99.0 (5f94df478 2026-08-27)
stable-aarch64-pc-windows-msvc (default)
cargo 1.97.1 (c980f4866 2026-06-30)
```

- RustNote のフォルダでは、`rust-toolchain.toml` によって `1.99.0` が選ばれています（`overridden by` は「このファイルで上書きされています」という意味です）。
- ホームフォルダでは、この PC の既定の版（ここでは 1.97.1）が使われました。あなたの PC では、UNIT 04 で入れた版になります。
- **同じ `cargo` というコマンドでも、今いるフォルダによって、動く Rust の版が変わる**のです。RustNote のコマンドは、必ず RustNote のフォルダに `cd` してから打ちます。RustNote の README（説明書）にも「PowerShell で、このフォルダ（`Rust/rustnote`）に移動して実行します」と書かれています。

RustNote は、待ち受けるポートなどの設定を環境変数から読みます（第0巻 UNIT 03）。PowerShell で `$env:PORT = "3001"` のように設定すると、**その窓から起動した RustNote にだけ**効きます。窓を閉じると消え、ほかの窓には影響しません。設定がうまく効かないときは、「設定した窓と、起動した窓が同じか」を確かめましょう。

### 練習問題

**問1:** ある人が `whereami.ps1` を実行したら、次のように表示されました。このまま作業を始めてよいでしょうか。直したほうがよい点を2つ挙げて、それぞれどうすればよいか答えてください。

```text
シェル          : PowerShell 5.1.26100.9711
現在のフォルダ  : C:\Users\...\OneDrive\Rust 勉強
OS              : Microsoft Windows 11 Home (ビルド 26200)
PC の CPU       : ARM64
このシェルの種類: AMD64
パスの文字      : 注意（空白や日本語が含まれています）
OneDrive        : 注意（同期フォルダの中です）
```

**問2:** ホームフォルダの中に `My Projects` というフォルダがあります。PowerShell で、そのフォルダへ移動して、移動できたことを確かめるコマンドを書いてください。また、`cd ~\My Projects` と打つとどうなるか、理由とともに答えてください。

**問3:** 作業フォルダの候補が4つあります。いちばんよいものを選び、ほかの3つがよくない理由を、それぞれ1つずつ答えてください（ユーザー名は `taro` とします）。

1. `C:\Program Files\rust-study`
2. `C:\Users\taro\rust-study`
3. `C:\Users\taro\OneDrive\デスクトップ\rust-study`
4. `C:\Users\taro\Documents\work\2026\october\study\rust\practice\projects\rust-study`

### 解答と解説

**問1の解答:** このまま作業を始めないほうがよいです。直す点は次の2つです。

- **シェルが x64 のエミュレーションで動いている**（「PC の CPU」が `ARM64`、「このシェルの種類」が `AMD64`）。スタートメニューから Windows Terminal を開き直し、`$env:PROCESSOR_ARCHITECTURE` が `ARM64` になる PowerShell で作業します。
- **作業フォルダの場所がよくない**（OneDrive の同期フォルダの中で、名前に空白と日本語が入っている）。`cd ~` でホームフォルダに戻り、`mkdir rust-study` で作業フォルダを作り直して、そこで作業します。

直したら、もう一度 `.\whereami.ps1` を実行して、「このシェルの種類」が `ARM64` になり、下の2行が両方 OK になることを確かめましょう。

**問2の解答:**

```powershell
# 空白を含むパスは " で囲む
cd "~\My Projects"
Get-Location
```

- `cd ~\My Projects` と囲まずに打つと、PowerShell は空白で区切って `~\My` と `Projects` の2つを `cd` に渡すので、エラー3と同じ「引数 'Projects' を受け入れる位置指定パラメーターが見つかりません」というエラーになります。
- `cd ~\My` まで打って Tab キーで補うと、自動で `'` で囲んでくれるので確実です。

**問3の解答:** いちばんよいのは **2** の `C:\Users\taro\rust-study` です。

- **1** は、`C:\Program Files` の中に書き込むのに管理者の権限が要ります。名前に空白も入っています。
- **3** は、OneDrive の同期フォルダの中です。ビルドのたびに大量のファイルが同期の対象になります（`デスクトップ` という日本語も入っています）。
- **4** は、深すぎてパスが長くなっています。この場所だけで 81 文字あり、プロジェクトの中に作られる長いファイルを足すと、エラー5の上限に近づきます。

### まとめ

- 結果が教材と違ったら、まず「シェルの版・今いるフォルダ・OS・CPU の種類」の4つを確かめる。ARM64 の PC では、`$env:PROCESSOR_ARCHITECTURE` が `ARM64` になる PowerShell で作業する。
- 作業フォルダは、ホームフォルダ直下の `rust-study` のように、短く・半角英数字だけで・OneDrive の外に置く。空白を含むパスは `"` で囲み、パスが 260 文字に近づく深い場所は避ける。
- 日本語を含む `.ps1` は BOM 付きの UTF-8 で保存し、実行ポリシーは `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned` で「自分で作ったスクリプトは実行してよい」にする。

## UNIT 02: Git・改行コード・秘密の除外

### この単元のゴール

- Git for Windows をインストールし、版と、設定がどのファイルに書かれているか（`--show-origin`）を確かめられる
- 名前・メールアドレス・最初のブランチ名を設定し、最初の**コミット**ができる
- `.gitignore` で、ビルドの成果物・データベースのファイル・秘密の値を書いた `.env` を、Git に入れないようにできる
- 改行コード（**CRLF** と **LF**）の違いと、`LF will be replaced by CRLF` という警告の意味を説明し、`.gitattributes` で改行コードを決められる
- うっかり秘密をコミットすると履歴に残ることを確かめ、そのときに何をすべきかを説明できる

### たとえ話でつかむ

**Git**（ギット）は、工房の「作業日誌つきの写真アルバム」です。

- 作業台の上の、今の状態が**作業フォルダ**です。
- 作業がひと区切りしたら、記録に残したい物を選んで台紙に並べます。これが **`git add`**（ステージに載せる）です。
- 台紙をアルバムに貼り、日付と「何をしたか」の一言を書き添えます。これが **`git commit`**（コミット）です。アルバムを後からめくれば、どの日に何がどう変わったかが分かり、昔の状態に戻すこともできます。
- 撮らない物もあります。作業で出た削りくず（ビルドの成果物）、毎日中身が変わる大きな帳簿（データベースのファイル）、そして**家の鍵**（パスワードや合い鍵の番号を書いたファイル）です。「アルバムに貼らない物リスト」が **`.gitignore`** です。
- 一度アルバムに貼った鍵の写真は、次のページで「鍵は外しました」と書いても、**前のページには残ったまま**です。アルバムを人に見せれば、鍵の形は分かってしまいます。

もう1つ、**改行コード**は「文の区切り方の方言」です。Windows では行の終わりを「CR と LF」の2文字で、Linux や macOS では「LF」の1文字で書くのが流儀です。Windows の方言のまま書いた手順書を Linux の係員に渡すと、行の最後に余計な CR がくっついた、知らない言葉に聞こえてしまいます。

| アルバムのたとえ | Git の言葉 | この単元で使うもの |
|---|---|---|
| 作業台の上 | 作業フォルダ（ワーキングツリー） | `~\rust-study\hello-git` |
| 台紙に並べる | ステージ（インデックス）に載せる | `git add` |
| アルバムに貼る | コミット | `git commit -m "…"` |
| アルバムそのもの | リポジトリ | `hello-git\.git` フォルダ |
| 貼らない物リスト | 無視するファイルの一覧 | `.gitignore` |
| 文の区切りの方言 | 改行コード（CRLF / LF） | `.gitattributes` |

> **用語:** リポジトリ（repository：Git が変更の記録をしまっておく場所。プロジェクトのフォルダの中の `.git` という隠しフォルダがその正体）

<!-- -->

> **用語:** コミット（commit：ある時点のファイルの状態を、メッセージ・作った人・日時と一緒に記録したもの。1つずつに `d779d78` のような番号（ハッシュ）が付く）

<!-- -->

> **用語:** 改行コード（行の終わりを表す見えない文字。**CR**（キャリッジ・リターン、`\r`、16進数で `0D`）と **LF**（ライン・フィード、`\n`、`0A`）がある。Windows の流儀は CR+LF の2文字で **CRLF**、Linux や macOS の流儀は **LF** の1文字）

### 図で理解する

まず、Git がファイルを記録するまでの流れです。ファイルは、作業フォルダ → ステージ → リポジトリ、の順に進みます。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-021" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="16" y="20" width="220" height="210" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="126" y="44" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">作業フォルダ</text>
<text x="32" y="72" font-size="12" fill="#0f172a" font-family="Consolas, monospace">Cargo.toml</text>
<text x="32" y="92" font-size="12" fill="#0f172a" font-family="Consolas, monospace">src/main.rs</text>
<text x="32" y="112" font-size="12" fill="#0f172a" font-family="Consolas, monospace">.env.example</text>
<text x="32" y="132" font-size="12" fill="#0f172a" font-family="Consolas, monospace">.gitignore</text>
<line x1="32" y1="146" x2="220" y2="146" stroke="#cbd5e1" stroke-width="1"/>
<text x="32" y="166" font-size="12" fill="#94a3b8" font-family="Consolas, monospace">.env</text>
<text x="32" y="186" font-size="12" fill="#94a3b8" font-family="Consolas, monospace">rustnote.db</text>
<text x="32" y="206" font-size="12" fill="#94a3b8" font-family="Consolas, monospace">target/</text>
<text x="210" y="186" font-size="12" fill="#b91c1c" text-anchor="end">✗ 無視</text>
<rect x="300" y="70" width="130" height="110" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="365" y="96" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">ステージ</text>
<text x="365" y="116" font-size="11" fill="#475569" text-anchor="middle">（インデックス）</text>
<text x="365" y="142" font-size="11" fill="#475569" text-anchor="middle">次のコミットに</text>
<text x="365" y="158" font-size="11" fill="#475569" text-anchor="middle">入れる物の台紙</text>
<rect x="504" y="40" width="200" height="170" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="604" y="64" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">リポジトリ（.git）</text>
<circle cx="540" cy="110" r="10" fill="#10b981"/>
<circle cx="604" cy="110" r="10" fill="#10b981"/>
<circle cx="668" cy="110" r="10" fill="#10b981"/>
<line x1="550" y1="110" x2="594" y2="110" stroke="#047857" stroke-width="2"/>
<line x1="614" y1="110" x2="658" y2="110" stroke="#047857" stroke-width="2"/>
<text x="604" y="146" font-size="11" fill="#475569" text-anchor="middle">コミットが順に並ぶ</text>
<text x="604" y="166" font-size="11" fill="#475569" text-anchor="middle">（いつでも前に戻れる）</text>
<line x1="236" y1="110" x2="296" y2="110" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-021)"/>
<text x="266" y="100" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">git add</text>
<line x1="430" y1="125" x2="500" y2="125" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-021)"/>
<text x="465" y="115" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">git commit</text>
<text x="126" y="256" font-size="12" fill="#b91c1c" text-anchor="middle">.gitignore に書いた物は、</text>
<text x="126" y="274" font-size="12" fill="#b91c1c" text-anchor="middle">git add の対象にならない</text>
<text x="604" y="256" font-size="12" fill="#475569" text-anchor="middle">git status で、3つの場所の</text>
<text x="604" y="274" font-size="12" fill="#475569" text-anchor="middle">違いを見られる</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-6: Git がファイルを記録する流れ。作業フォルダのファイルを git add でステージに載せ、git commit でリポジトリに記録する。.gitignore に書いたファイルは載らない</div>
</div>

次に、改行コードです。同じ `pwd` という1行でも、Windows の流儀と Linux の流儀では、ファイルの中身（バイト）が違います。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-022" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="20" y="34" font-size="13" font-weight="700" fill="#0f172a">CRLF（Windows の流儀）</text>
<rect x="210" y="16" width="60" height="28" rx="4" fill="#f8fafc" stroke="#64748b"/>
<text x="240" y="35" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">p 70</text>
<rect x="274" y="16" width="60" height="28" rx="4" fill="#f8fafc" stroke="#64748b"/>
<text x="304" y="35" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">w 77</text>
<rect x="338" y="16" width="60" height="28" rx="4" fill="#f8fafc" stroke="#64748b"/>
<text x="368" y="35" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">d 64</text>
<rect x="402" y="16" width="70" height="28" rx="4" fill="#fee2e2" stroke="#b91c1c"/>
<text x="437" y="35" font-size="12" fill="#b91c1c" text-anchor="middle" font-family="Consolas, monospace">\r 0D</text>
<rect x="476" y="16" width="70" height="28" rx="4" fill="#dbeafe" stroke="#3b82f6"/>
<text x="511" y="35" font-size="12" fill="#1e40af" text-anchor="middle" font-family="Consolas, monospace">\n 0A</text>
<text x="20" y="80" font-size="13" font-weight="700" fill="#0f172a">LF（Linux・macOS の流儀）</text>
<rect x="210" y="62" width="60" height="28" rx="4" fill="#f8fafc" stroke="#64748b"/>
<text x="240" y="81" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">p 70</text>
<rect x="274" y="62" width="60" height="28" rx="4" fill="#f8fafc" stroke="#64748b"/>
<text x="304" y="81" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">w 77</text>
<rect x="338" y="62" width="60" height="28" rx="4" fill="#f8fafc" stroke="#64748b"/>
<text x="368" y="81" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">d 64</text>
<rect x="402" y="62" width="70" height="28" rx="4" fill="#dbeafe" stroke="#3b82f6"/>
<text x="437" y="81" font-size="12" fill="#1e40af" text-anchor="middle" font-family="Consolas, monospace">\n 0A</text>
<text x="560" y="54" font-size="11" fill="#b91c1c">Linux は <tspan font-family="Consolas, monospace">\r</tspan> を</text>
<text x="560" y="70" font-size="11" fill="#b91c1c">名前の一部と思う</text>
<rect x="16" y="120" width="200" height="110" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="116" y="144" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">Windows の作業フォルダ</text>
<text x="116" y="170" font-size="12" fill="#0f172a" text-anchor="middle">ふつうのファイル: LF</text>
<text x="116" y="192" font-size="12" fill="#0f172a" text-anchor="middle">*.ps1・*.bat: CRLF</text>
<text x="116" y="216" font-size="11" fill="#475569" text-anchor="middle">VS Code も LF で保存（UNIT 03）</text>
<rect x="260" y="120" width="200" height="110" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="144" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">リポジトリの中</text>
<text x="360" y="170" font-size="12" fill="#0f172a" text-anchor="middle">テキストは全部 LF</text>
<text x="360" y="192" font-size="12" fill="#0f172a" text-anchor="middle">画像・DB は変換しない</text>
<rect x="504" y="120" width="200" height="110" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="604" y="144" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">Linux の世界</text>
<text x="604" y="170" font-size="12" fill="#0f172a" text-anchor="middle">WSL・Docker・CI</text>
<text x="604" y="192" font-size="12" fill="#0f172a" text-anchor="middle">Fly.io: LF のまま動く</text>
<line x1="216" y1="175" x2="256" y2="175" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-022)" marker-start="url(#ar-01-022)"/>
<line x1="460" y1="175" x2="500" y2="175" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-022)" marker-start="url(#ar-01-022)"/>
<rect x="160" y="252" width="400" height="64" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="360" y="276" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">.gitattributes（リポジトリに入れる決まり）</text>
<text x="260" y="300" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">* text=auto eol=lf</text>
<text x="460" y="300" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">*.ps1 text eol=crlf</text>
<line x1="360" y1="252" x2="360" y2="234" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-022)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-7: 改行コードの違いと、.gitattributes で決める置き方。リポジトリの中も作業フォルダも LF にそろえ、Windows 専用のスクリプトだけ CRLF にすると、Linux の世界へそのまま持っていける</div>
</div>

- CRLF のファイルを Linux で読むと、行の最後の `\r`（`0D`）が、コマンドの名前の一部として扱われます。`pwd` は `pwd\r` という知らないコマンドになってしまいます（「よくあるエラー」のエラー2で実際に見ます）。
- RustNote は、最後は Fly.io（Linux）の上で動きます。Windows で書いたファイルを、Linux の世界へ**そのまま**持っていけるように、最初から LF にそろえておくのが安全です。

最後に、秘密の値の扱いです。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-023" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="16" y="20" width="300" height="100" rx="10" fill="#fee2e2" stroke="#b91c1c" stroke-width="2"/>
<text x="166" y="44" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">.env（本物の値）</text>
<text x="32" y="70" font-size="12" fill="#0f172a" font-family="Consolas, monospace">DATABASE_AUTH_TOKEN=eyJhbG…</text>
<text x="166" y="100" font-size="12" fill="#b91c1c" text-anchor="middle">自分の PC だけに置く・Git に入れない</text>
<rect x="16" y="140" width="300" height="100" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="166" y="164" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">.env.example（見本）</text>
<text x="32" y="190" font-size="12" fill="#0f172a" font-family="Consolas, monospace">DATABASE_AUTH_TOKEN=</text>
<text x="166" y="220" font-size="12" fill="#047857" text-anchor="middle">名前だけ書いて Git で共有する</text>
<text x="530" y="34" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">うっかり .env をコミットした場合</text>
<circle cx="400" cy="90" r="12" fill="#10b981"/>
<circle cx="520" cy="90" r="12" fill="#b91c1c"/>
<circle cx="640" cy="90" r="12" fill="#10b981"/>
<line x1="412" y1="90" x2="508" y2="90" stroke="#475569" stroke-width="2"/>
<line x1="532" y1="90" x2="628" y2="90" stroke="#475569" stroke-width="2"/>
<text x="400" y="70" font-size="11" fill="#0f172a" text-anchor="middle">最初のコミット</text>
<text x="520" y="70" font-size="11" fill="#b91c1c" text-anchor="middle">.env を入れた</text>
<text x="640" y="70" font-size="11" fill="#0f172a" text-anchor="middle">.env を外した</text>
<rect x="356" y="150" width="348" height="90" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="530" y="176" font-size="12" fill="#c2410c" text-anchor="middle">最新のコミットに .env が無くても、</text>
<text x="530" y="198" font-size="12" fill="#c2410c" text-anchor="middle">真ん中のコミットの中には残っている</text>
<text x="530" y="222" font-size="12" fill="#0f172a" text-anchor="middle">→ 上げてしまったら、鍵（トークン）を無効にする</text>
<path d="M520,104 V146" fill="none" stroke="#b91c1c" stroke-width="2" stroke-dasharray="4 3" marker-end="url(#ar-01-023)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-8: 秘密の値の置き方。本物の値は .env に書いて自分の PC だけに置き、Git では見本の .env.example を共有する。一度コミットした秘密は、後で外しても履歴に残る</div>
</div>

### まず動かす最小コード

この単元の「最小コード」も、PowerShell のコマンドです。Git を入れて、最初の設定をし、練習用のリポジトリを作るところまで進めます。

#### 例2-1: Git をインストールして、版と置き場所を確かめる

> **▼ この手順がやること（先に日本語で）:** Git for Windows（Windows 用の Git）をインストールします。新しく開いた PowerShell で、Git の版・Git の本体の場所・インストーラーが書き込んだ設定を表示して、正しく入ったことを確かめます。

**手順1: インストールする前に確かめる。** PowerShell で `git --version` と打ちます。版が表示されたら、Git はもう入っています（手順2は飛ばしてかまいません）。第0巻 UNIT 03 で見た「用語 'git' は、…認識されません」のエラーになったら、まだ入っていません。

**手順2: インストーラーで入れる。** ブラウザで Git の公式サイト（`https://git-scm.com/`）を開き、Windows 用のダウンロードのページから、インストーラーを入手します。

- インストーラーには、x64 用と **ARM64 用**があります。ARM64 の PC では ARM64 用を選びます。間違えて x64 用を入れても動きますが、エミュレーションになります（UNIT 01）。
- インストーラーを実行すると、たくさんの画面が順に出てきます。**基本は既定のまま「Next」**で進めます。途中に次の画面があります（画面の名前は、この本の PC に入っている Git 2.53.0 のインストーラーのものです）。

| 画面の名前 | 既定の選択 | この本での扱い |
|---|---|---|
| Choosing the default editor used by Git | Use Vim (the ubiquitous text editor) as Git's default editor | 既定のままでよい。VS Code に変える方法は UNIT 03 で説明します |
| Adjusting the name of the initial branch in new repositories | Let Git decide | 既定のままでよい。例2-2 で `main` に設定します |
| Adjusting your PATH environment | Git from the command line and also from 3rd-party software | 既定のまま（PowerShell から `git` を使えるようにする選択） |
| Configuring the line ending conversions | Checkout Windows-style, commit Unix-style line endings | 既定のままでよい。改行コードは例2-5 で、リポジトリごとに決めます |

> **ヒント:** winget（ウィンゲット：Windows に入っている、コマンドでアプリを入れる道具）を使う方法もあります。`winget install --id Git.Git -e --source winget` と打つと、画面を出さずに既定の設定で入ります。この本の PC では winget での入れ方は試していないので、入れた後に、次の手順3で版と置き場所を必ず確かめてください。

**手順3: 新しい窓で確かめる。** インストールが終わったら、**PowerShell の窓をいったん閉じて、新しく開き直します**。そして次のコマンドを打ちます。

```powershell
# Git の版を表示する
git --version
# git というコマンドの本体が、どこにあるかを表示する
where.exe git
# インストーラーが書き込んだ、PC 全体（system）の設定を一覧で表示する
git config --system --list
```

**▼ 実行結果**

```text
git version 2.53.0.windows.2
C:\Program Files\Git\cmd\git.exe
diff.astextplain.textconv=astextplain
filter.lfs.clean=git-lfs clean -- %f
filter.lfs.smudge=git-lfs smudge -- %f
filter.lfs.process=git-lfs filter-process
filter.lfs.required=true
http.sslbackend=schannel
core.autocrlf=true
core.fscache=true
core.symlinks=false
pull.rebase=false
credential.helper=manager
credential.https://dev.azure.com.usehttppath=true
init.defaultbranch=master
```

#### ▼ 1つずつ分解して解説

##### 解説1: `git --version` の `windows.2`

- `2.53.0` が Git そのものの版、`.windows.2` は「Git 2.53.0 をもとにした、Windows 用の2回目の版」という Git for Windows の印です。あなたの PC では、もっと新しい版かもしれません。

##### 解説2: `where.exe git` は「どこの git が動くか」

- `where.exe` は、`PATH`（第0巻 UNIT 03）に書かれたフォルダを順に探して、その名前のプログラムがどこにあるかを教えてくれます。`C:\Program Files\Git\cmd\git.exe` が、PowerShell から `git` と打ったときに動く入口です。
- PowerShell では、`where` とだけ打つと、`Where-Object`（一覧から条件に合うものを選ぶコマンド）の別名として扱われてしまいます。場所を調べるときは、必ず `.exe` まで付けて `where.exe` と打ちます。
- 「情報: 与えられたパターンのファイルが見つかりませんでした。」と表示されるときは、`PATH` に Git のフォルダが入っていません。窓を開き直したか確かめましょう。

> **ヒント:** ARM64 版の Git が入っているかは、`Test-Path "C:\Program Files\Git\clangarm64"` で確かめられます。この本の PC（ARM64 版の Git）では `True` と表示されました。

##### 解説3: インストーラーが書き込んだ設定（system）

- `git config --system --list` は、**PC の全員**に効く設定の一覧です。インストーラーを既定のまま進めると、この内容が `C:\Program Files\Git\etc\gitconfig` というファイルに書き込まれます。
- 特に知っておきたいのは次の3つです。

| 設定 | 意味 |
|---|---|
| `core.autocrlf=true` | ファイルを取り出すときに改行を CRLF に、記録するときに LF に変換する（インストーラーの「Checkout Windows-style, commit Unix-style line endings」） |
| `credential.helper=manager` | GitHub などにログインした情報を、Windows に安全に覚えておく係（Git Credential Manager）を使う |
| `init.defaultbranch=master` | 新しいリポジトリの最初のブランチの名前を `master` にする（インストーラーの「Let Git decide」） |

- 設定がどのファイルに書かれているかは、`--show-origin` を付けると分かります。

```powershell
# core.autocrlf の値と、それが書かれているファイルを表示する
git config --show-origin --get core.autocrlf
```

**▼ 実行結果**

```text
file:C:/Program Files/Git/etc/gitconfig	true
```

#### 例2-2: 名前・メールアドレス・最初のブランチ名を設定する

> **▼ この手順がやること（先に日本語で）:** コミットに記録される「作った人の名前とメールアドレス」を設定します。さらに、新しいリポジトリの最初のブランチの名前を `main` にします。どれも、あなたのユーザーだけに効く設定（global）です。

```powershell
# コミットに記録する名前（ローマ字がおすすめ）
git config --global user.name "Taro Yamada"
# コミットに記録するメールアドレス
git config --global user.email "taro@example.com"
# 新しいリポジトリの最初のブランチの名前を main にする
git config --global init.defaultBranch main
```

`Taro Yamada` と `taro@example.com` は、あなたの名前とメールアドレスに置き換えてください。3行とも、成功すると何も表示されません。設定した内容は、次のコマンドで確かめます。

```powershell
# あなたのユーザーの設定（global）を一覧で表示する
git config --global --list
```

一覧の中に、`user.name=…`・`user.email=…`・`init.defaultbranch=main` の3行があれば成功です（この本の PC には別の名前が設定されているので、ここの表示は載せていません）。

#### ▼ 1つずつ分解して解説

##### 解説1: `--system`・`--global`・`--local` の3つの段

- Git の設定には3つの段があり、**下の段ほど優先**されます。

| 段 | 効く範囲 | 書かれているファイル |
|---|---|---|
| `--system` | PC の全員 | `C:\Program Files\Git\etc\gitconfig` |
| `--global` | あなたのユーザー | `C:\Users\（あなたの名前）\.gitconfig` |
| `--local` | そのリポジトリだけ | リポジトリの中の `.git\config` |

- 例2-1で見たとおり、system には `init.defaultbranch=master` が書かれています。global に `main` を書いたので、あなたが作るリポジトリでは `main` が使われます。
- `init.defaultBranch` のように、名前の大文字・小文字は区別されません。一覧では小文字（`init.defaultbranch`）で表示されます。

##### 解説2: 名前とメールアドレスは、コミットと一緒に公開される

- ここで設定した名前とメールアドレスは、**コミットのたびに記録**されます。GitHub などにコードを上げると、誰でも見られる状態になります（例2-5の後で、実際に記録された内容を見ます）。
- 本名やふだん使いのメールアドレスを公開したくない人は、GitHub が用意している「公開用のメールアドレス（noreply のアドレス）」を使う方法があります。GitHub のアカウントの設定で確かめてください。

##### 解説3: なぜ `main` にするのか

- ブランチ（branch：枝）は、変更の流れに付ける名前です。GitHub で新しく作るリポジトリの最初のブランチは `main` なので、手元もそろえておくと、後で GitHub につなぐときに迷いません。このシリーズでも `main` を使います。

#### 例2-3: 練習用のリポジトリ hello-git を作り、状態を見る

> **▼ この手順がやること（先に日本語で）:** `rust-study` の中に練習用のフォルダ `hello-git` を作り、Git のリポジトリにします。その中に、Rust のプロジェクトによくあるファイル（設定のファイル・プログラム・環境変数の見本と本物・データベースのファイル・ビルドの成果物）の**代わりになる小さなファイル**を作り、Git から見た状態を表示します。

**手順1: フォルダを作って、リポジトリにする。**

```powershell
# 作業フォルダへ移動して、練習用のフォルダを作る
cd ~\rust-study
mkdir hello-git
cd hello-git
# このフォルダを Git のリポジトリにする
git init
```

**▼ 実行結果**（`mkdir` の表示は省いています）

```text
Initialized empty Git repository in C:/Users/.../rust-study/hello-git/.git/
```

**手順2: 練習用のファイルを作る。** Rust のプロジェクトは UNIT 04 で `cargo new` を使って作りますが、ここでは同じ形のファイルを手で作ります。まず、ファイルを作るための小さな**関数**（いくつかのコマンドをまとめて名前を付けたもの）を、PowerShell に覚えさせます。

```powershell
# 1行ずつの文字列を LF（`n）でつないで、最後にも LF を付け、UTF-8（BOM なし）で書き出す関数
function New-LfFile($Path, [string[]]$Lines) {
    $text = ($Lines -join "`n") + "`n"
    New-Item -Path $Path -Value $text -Force | Out-Null
}
```

続けて、この関数でファイルを作ります。`'…'` で囲んだ1つずつが、ファイルの1行になります。

```powershell
# Rust のプロジェクトの設定ファイルの代わり
New-LfFile Cargo.toml '[package]', 'name = "hello-git"', 'version = "0.1.0"', 'edition = "2024"', '', '[dependencies]'
# プログラムの代わり（src フォルダも一緒に作られる）
New-LfFile src\main.rs 'fn main() {', '    println!("Hello, world!");', '}'
# 環境変数の見本（値は空のまま）
New-LfFile .env.example 'HOST=127.0.0.1', 'PORT=3000', 'DATABASE_URL=rustnote.db', 'DATABASE_AUTH_TOKEN='
# 本物の値を書いた環境変数のファイル（練習用の偽物のトークン）
New-LfFile .env 'HOST=127.0.0.1', 'PORT=3000', 'DATABASE_URL=libsql://rustnote-taro.turso.io', 'DATABASE_AUTH_TOKEN=eyJhbGciOi...secret'
# データベースのファイルの代わり
New-LfFile rustnote.db 'db'
# ビルドの成果物の代わり（空のファイル。target\debug フォルダも一緒に作られる）
New-Item target\debug\hello-git.exe -Force | Out-Null
```

**手順3: Git から見た状態を表示する。**

```powershell
# 作業フォルダ・ステージ・リポジトリの違いを表示する
git status
```

**▼ 実行結果**

```text
On branch main

No commits yet

Untracked files:
  (use "git add <file>..." to include in what will be committed)
	.env
	.env.example
	Cargo.toml
	rustnote.db
	src/
	target/

nothing added to commit but untracked files present (use "git add" to track)
```

#### ▼ 1つずつ分解して解説

##### 解説1: `git init` で、フォルダがリポジトリになる

- `git init` は、今いるフォルダの中に `.git` という隠しフォルダを作ります。これがリポジトリ（アルバム）の正体です。中身を手で触る必要はありません。
- 表示のパスが `C:/Users/...` のように `/` で区切られているのは、Git が Linux 生まれの道具だからです。Windows でも `/` のパスはたいてい通じます。

##### 解説2: `New-LfFile` 関数で、LF のファイルを作る

```powershell
function New-LfFile($Path, [string[]]$Lines) {
    $text = ($Lines -join "`n") + "`n"
    New-Item -Path $Path -Value $text -Force | Out-Null
}
```

- `function 名前(引数) { … }` で、PowerShell に新しいコマンドを覚えさせます。`$Path` にファイルの名前、`$Lines` に行の並び（`[string[]]` は「文字列の並び」という意味）を受け取ります。覚えさせたコマンドは、**その窓を閉じるまで**使えます。
- `` "`n" `` は、PowerShell で **LF**（改行の1文字）を表す書き方です。`` ` ``（バッククォート）は、日本語のキーボードでは Shift キーを押しながら `@` のキーを押すと打てます。`-join` で行の並びを LF でつなぎ、最後にも LF を1つ足しています。
- `New-Item -Path … -Value …` は、受け取った文字列を**そのまま** UTF-8（BOM なし）でファイルに書きます。`-Force` は「フォルダが無ければ作り、ファイルがあれば上書きする」という指定です。`| Out-Null` は、作ったファイルの一覧の表示を消しています。
- なぜわざわざ関数を作ったのかというと、Windows PowerShell 5.1 の `Set-Content` で書くと、改行が CRLF になり、`-Encoding utf8` では BOM も付くからです。Rust のプロジェクトのファイルは、`cargo new` が作るのと同じ「LF・BOM なし」にしておきたいのです。UNIT 03 で VS Code を LF で保存する設定にした後は、エディタで作れば済むので、この関数は使いません。

> **ヒント:** この本の PC で、この関数で作ったファイルを1バイトずつ比べ、後の例の実行結果を取ったときのファイルと、中身が完全に同じであることを確かめています。

##### 解説3: `git status` の読み方

- `On branch main` は「今は `main` というブランチにいます」、`No commits yet` は「まだコミットが1つもありません」という意味です。
- `Untracked files`（追跡していないファイル）は、「作業フォルダにあるけれど、Git がまだ記録の対象にしていないファイル」の一覧です。フォルダは中身をまとめて `src/` のように表示されます。
- 今は、**`.env`（秘密）・`rustnote.db`（データベース）・`target/`（ビルドの成果物）まで並んでいます**。このまま `git add .`（全部をステージに載せる）とすると、これらも記録されてしまいます。次の「もう一歩」で、これを防ぎます。

> **注意:** `.env` のトークン `eyJhbGciOi...secret` は、練習用の偽物です。本物のトークンは、絶対にこの本の手順のように画面に出したり、人に見せたりしないでください。

### もう一歩: 実務に近い形

#### 例2-4: .gitignore で、入れてはいけないファイルを外す

> **▼ この手順がやること（先に日本語で）:** 「Git に入れないファイル」の一覧 `.gitignore` を作ります。作った後の `git status` で、秘密・データベース・ビルドの成果物が一覧から消えたことを確かめます。どの行の決まりで外されたのかも表示し、最後に `.env` を無理に追加しようとして、Git に止められることを確かめます。

```powershell
# Git に入れないファイルの一覧を作る
New-LfFile .gitignore @(
    '# ビルドの成果物（cargo build が作る）'
    '/target/'
    ''
    '# ローカルの SQLite / libSQL データベース'
    '*.db'
    '*.db-shm'
    '*.db-wal'
    ''
    '# 秘密の値を書いた環境変数ファイル（見本の .env.example だけを共有する）'
    '.env'
)
# もう一度、状態を表示する
git status
```

`@(` の後で Enter キーを押すと、行の先頭に `>>` が出て、続きを入力できます。最後の `)` の行で Enter キーを押すと、ファイルが作られます。

**▼ 実行結果**

```text
On branch main

No commits yet

Untracked files:
  (use "git add <file>..." to include in what will be committed)
	.env.example
	.gitignore
	Cargo.toml
	src/

nothing added to commit but untracked files present (use "git add" to track)
```

`.env`・`rustnote.db`・`target/` が一覧から消えました。どの決まりで外されたのかは、`git check-ignore -v` で確かめられます。

```powershell
# 3つのファイルが、.gitignore のどの行で外されたかを表示する
git check-ignore -v .env rustnote.db target/debug/hello-git.exe
```

**▼ 実行結果**

```text
.gitignore:10:.env	.env
.gitignore:5:*.db	rustnote.db
.gitignore:2:/target/	target/debug/hello-git.exe
```

最後に、`.env` を名指しで追加しようとしてみます。

```powershell
# 無視しているファイルを、名指しで追加しようとする
git add .env
```

**▼ 実行結果**

```text
The following paths are ignored by one of your .gitignore files:
.env
hint: Use -f if you really want to add them.
hint: Disable this message with "git config set advice.addIgnoredFile false"
```

#### ▼ 1つずつ分解して解説

##### 解説1: `.gitignore` の書き方

| 書き方 | 意味 | この例で外れるもの |
|---|---|---|
| `/target/` | 先頭の `/` は「リポジトリのいちばん上の」、最後の `/` は「フォルダ」 | `target` フォルダの中身全部 |
| `*.db` | `*` は「何文字でもよい」。拡張子が `.db` のファイル全部 | `rustnote.db` |
| `.env` | この名前のファイル | `.env` |
| `#` で始まる行 | コメント（説明のメモ） | なし |

- `*.db-shm` と `*.db-wal` は、SQLite や libSQL（第7巻）がデータベースのファイルの横に作る、作業用のファイルです。まだ無くても、先に書いておきます。
- `.gitignore` そのものは、**Git に入れて共有**します。一緒に作業する人の PC でも、同じファイルが外されるようにするためです。

##### 解説2: `git check-ignore -v` の読み方

- `.gitignore:10:.env	.env` は、「`.gitignore` の10行目の `.env` という決まりで、`.env` が外されています」という意味です。左から「ファイル名:行番号:決まり」、空白（タブ）をはさんで「調べたパス」です。
- 「なぜか Git に入らない」「なぜか入ってしまう」と困ったときに、どの決まりが効いているのかを確かめるのに使います。

##### 解説3: `git add .env` が止められた

- 無視しているファイルを名指しで追加しようとすると、Git は「このパスは `.gitignore` で無視されています」と教えてくれて、追加しません。
- `hint: Use -f if you really want to add them.`（本当に追加したいなら `-f` を使って）と書かれていますが、**秘密のファイルで `-f` を使ってはいけません**。使うとどうなるかは、「よくあるエラー」のエラー3で見ます。

#### 例2-5: 改行コードを .gitattributes で決めて、最初のコミットをする

> **▼ この手順がやること（先に日本語で）:** まず、残ったファイルを全部ステージに載せ、Git が改行コードについて出す警告を読みます。次に、改行コードの決まり `.gitattributes` と、Windows 専用のスクリプトの例 `check.ps1` を作って、ステージの中の改行コードがどうなっているかを比べます。最後に、最初のコミットをします。

**手順1: ステージに載せて、警告を読む。**

```powershell
# 今いるフォルダの、無視していないファイルを全部ステージに載せる
git add .
```

**▼ 実行結果**

```text
warning: in the working copy of '.env.example', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of '.gitignore', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of 'Cargo.toml', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of 'src/main.rs', LF will be replaced by CRLF the next time Git touches it
```

ステージに載ったファイルの改行コードを、`git ls-files --eol` で表示します。

```powershell
# ステージのファイルと作業フォルダのファイルの、改行コードを並べて表示する
git ls-files --eol
```

**▼ 実行結果**

```text
i/lf    w/lf    attr/                 	.env.example
i/lf    w/lf    attr/                 	.gitignore
i/lf    w/lf    attr/                 	Cargo.toml
i/lf    w/lf    attr/                 	src/main.rs
```

**手順2: 改行コードの決まりを作る。**

```powershell
# 改行コードの決まりを作る
New-LfFile .gitattributes @(
    '# テキストファイルは、リポジトリにも作業フォルダにも LF で置く'
    '* text=auto eol=lf'
    ''
    '# Windows 専用のスクリプトは CRLF で置く'
    '*.ps1 text eol=crlf'
    '*.bat text eol=crlf'
    '*.cmd text eol=crlf'
    ''
    '# 画像やデータベースは文字ではないので、改行の変換をしない'
    '*.png binary'
    '*.jpg binary'
    '*.db binary'
)
# Windows 専用のスクリプトの例を作る
New-LfFile check.ps1 'Write-Output "check"'
# もう一度、全部をステージに載せる
git add .
```

**▼ 実行結果**

```text
warning: in the working copy of 'check.ps1', LF will be replaced by CRLF the next time Git touches it
```

```powershell
git ls-files --eol
```

**▼ 実行結果**

```text
i/lf    w/lf    attr/text=auto eol=lf 	.env.example
i/lf    w/lf    attr/text=auto eol=lf 	.gitattributes
i/lf    w/lf    attr/text=auto eol=lf 	.gitignore
i/lf    w/lf    attr/text=auto eol=lf 	Cargo.toml
i/lf    w/lf    attr/text eol=crlf    	check.ps1
i/lf    w/lf    attr/text=auto eol=lf 	src/main.rs
```

**手順3: 最初のコミットをする。**

```powershell
# ステージに載せた内容を、メッセージを付けて記録する
git commit -m "最初のコミット"
```

**▼ 実行結果**

```text
[main (root-commit) d779d78] 最初のコミット
 6 files changed, 36 insertions(+)
 create mode 100644 .env.example
 create mode 100644 .gitattributes
 create mode 100644 .gitignore
 create mode 100644 Cargo.toml
 create mode 100644 check.ps1
 create mode 100644 src/main.rs
```

#### ▼ 1つずつ分解して解説

##### 解説1: 「LF will be replaced by CRLF」は何の警告か

- 警告の意味は、「作業フォルダの `.env.example` の LF は、**次に Git がこのファイルに触るとき**（取り出し直すときなど）、CRLF に置き換えられます」です。今すぐファイルが変わるわけではありません。
- 原因は、例2-1で見た system の設定 `core.autocrlf=true` です。この設定は「記録するときは LF、取り出すときは CRLF」という変換をします。私たちのファイルは LF で作ったので、「取り出すときには CRLF になりますよ」と前もって知らせてくれたのです。
- このまま別の Windows の PC でこのリポジトリを取り出すと、作業フォルダのファイルは CRLF になります。それを WSL や Docker の中で使うと、図で見た「`\r` がくっつく」問題が起きます。

##### 解説2: `git ls-files --eol` の読み方

- `i/lf` は「ステージ（index）の中は LF」、`w/lf` は「作業フォルダ（working tree）のファイルは LF」、`attr/` は「`.gitattributes` で決めた決まり」です。
- 1回目は `attr/` の後ろが空でした。決まりが無いので、取り出すときの改行は、それぞれの PC の `core.autocrlf` しだいです。
- 2回目は、`attr/text=auto eol=lf` が付きました。これで、どの PC で取り出しても、作業フォルダのファイルは LF になります。`.gitattributes` の決まりは、`core.autocrlf` より優先されるからです。
- `check.ps1` だけは `attr/text eol=crlf` です。今の作業フォルダのファイルはまだ LF（`w/lf`）ですが、次に取り出したときに CRLF になります。手順2の警告は、そのことを知らせています。

##### 解説3: `.gitattributes` の書き方

| 書き方 | 意味 |
|---|---|
| `* text=auto eol=lf` | すべてのファイルについて、文字のファイルかどうかを Git に判断させ（`text=auto`）、文字のファイルなら作業フォルダでも LF にする（`eol=lf`） |
| `*.ps1 text eol=crlf` | `.ps1` は文字のファイルとして扱い、作業フォルダでは CRLF にする。後に書いた決まりが優先される |
| `*.png binary` | 文字のファイルではない（`binary`）。改行の変換も、差分の表示もしない |

- PowerShell のスクリプトは LF でも動きますが、Windows でしか使わないファイルなので、Windows の流儀（CRLF）にそろえています。`.bat` や `.cmd`（コマンドプロンプト用のスクリプト）は、LF だと正しく動かないことがあるので、CRLF にしておきます。
- この本では、PC 全体の設定（`core.autocrlf`）は既定のまま変えず、**リポジトリごとに `.gitattributes` で決めます**。`.gitattributes` はリポジトリに入るので、一緒に作業する人の PC でも、CI（自動テストの仕組み）や Docker の中でも、同じ決まりが使われるからです。

##### 解説4: コミットの表示の読み方

- `[main (root-commit) d779d78]` は、「`main` ブランチに、最初の（root）コミット `d779d78` を作りました」という意味です。`d779d78` はコミットの番号の先頭7文字で、あなたの PC では違う番号になります。
- `6 files changed, 36 insertions(+)` は「6つのファイルが変わり、36行が増えました」、`create mode 100644` は「ふつうのファイルとして新しく記録しました」という意味です。

記録された名前とメールアドレスは、`git log` で確かめられます。

```powershell
# コミットの番号・作った人・メッセージを1行ずつ表示する
git log --format="%h %an <%ae> %s"
```

**▼ 実行結果**（例2-5のすぐ後なら、この1行だけが表示されます）

```text
d779d78 Taro Yamada <taro@example.com> 最初のコミット
```

例2-2で設定した名前とメールアドレスが、コミットと一緒に記録されています。

### よくあるエラーと読み方

#### エラー1: 「Author identity unknown」（名前とメールアドレスの未設定）

例2-2を飛ばして `git commit` すると、次のエラーになります。

```text
Author identity unknown

*** Please tell me who you are.

Run

  git config --global user.email "you@example.com"
  git config --global user.name "Your Name"

to set your account's default identity.
Omit --global to set the identity only in this repository.

fatal: unable to auto-detect email address (got '...@...(none)')
```

- **何が起きたか:** 「作った人が分かりません。あなたは誰ですか？」というエラーです。コミットには作った人の名前とメールアドレスが必要なのに、どちらも設定されていませんでした。最後の行は「メールアドレスを自動で決められませんでした」という意味で、`...` の部分には Windows のユーザー名と PC の名前が入ります。
- **直し方:** エラーの中に書かれているとおり、`git config --global user.email "…"` と `git config --global user.name "…"` を実行してから、もう一度 `git commit` します。`Omit --global`（`--global` を付けないと、このリポジトリだけの設定になります）とも書かれていますが、ふつうは `--global` で設定します。

> **ヒント:** `-m "…"` を付けずに `git commit` と打つと、メッセージを書くためのエディタが開きます。既定のエディタは Vim（ヴィム）で、画面の左に `~` が並んだ黒い画面になります。慣れないうちは、`Esc` キーを押してから `:q!` と打って Enter キーを押し、いったん抜けてください（コミットは中止されます）。この本では、いつも `-m` を付けてコミットします。

#### エラー2: 「$'\r': command not found」（CRLF のシェルスクリプトを Linux で動かす）

Windows のエディタで作ったり、`core.autocrlf=true` の Git で取り出したりして、**CRLF になったシェルスクリプト**（bash の手順書）を、WSL の bash で動かした場合です。WSL は UNIT 07 で入れるので、今は結果を見るだけで大丈夫です。

使ったスクリプト `crlf.sh` は、次の5行です（中身は LF の `lf.sh` と同じで、改行だけが CRLF です）。

```bash
#!/bin/bash
set -e
echo "hello from WSL"
cd /tmp
pwd
```

WSL の bash で、見えない文字も表示するように（`cat -v`）して実行した結果です。

```bash
# エラーも一緒に流して（2>&1）、見えない文字を ^M のように表示する（cat -v）
bash crlf.sh 2>&1 | cat -v
```

**▼ 実行結果**

```text
crlf.sh: line 2: set: -^M: invalid option
set: usage: set [-abefhkmnptuvxBCEHPT] [-o option-name] [--] [-] [arg ...]
hello from WSL^M
crlf.sh: line 4: cd: $'/tmp\r': No such file or directory
crlf.sh: line 5: $'pwd\r': command not found
```

- **何が起きたか:** `^M` と `\r` は、どちらも CR（`0D`）のことです。bash は行の終わりを LF だけで判断するので、行の最後の CR を、**コマンドの一部**として読みました。
  - 2行目: `set -e` が `set -e\r` になり、「`-\r` という知らないオプションです」と断られました。
  - 4行目: `cd /tmp` が `/tmp\r` というフォルダへの移動になり、「そんなフォルダはありません」と言われました。
  - 5行目: `pwd` が `pwd\r` という知らないコマンドになりました。
- エラーの中の `$'pwd\r'` は、bash が「見えない文字を含む名前」を表示するときの書き方です。**`\r` が見えたら、改行コードを疑う**、と覚えておきましょう。
- **直し方:** ファイルの改行を LF に直します。WSL の bash なら、次のコマンドで行の最後の CR を取り除けます。直した後に実行すると、`hello from WSL` と `/tmp` が表示されることを、この本の PC で確かめました。

```bash
# 各行の最後の \r（CR）を取り除いて、ファイルを上書きする
sed -i 's/\r$//' crlf.sh
```

- そもそも CRLF にしないためには、この単元の `.gitattributes`（`* text=auto eol=lf`）をリポジトリに入れておき、UNIT 03 で VS Code を LF で保存する設定にします。

#### エラー3（エラーが出ない事故）: `.env` をコミットしてしまった

いちばん怖いのは、エラーが出ずに**秘密が記録されてしまう**事故です。ここでは、例2-4の注意を無視して `-f` で `.env` を追加し、コミットしてしまった場面を、わざと作って確かめます。

```powershell
# わざと失敗する実験: 無視しているはずの .env を、-f で無理やり追加してコミットする
git add -f .env
git commit -m "うっかり .env を入れた"
```

気づいて、`.env` を Git の管理から外します。`--cached` は「作業フォルダのファイルは消さずに、Git の記録からだけ外す」という指定です。

```powershell
# .env を Git の記録から外す（作業フォルダの .env は残る）
git rm --cached .env
git status --short
```

**▼ 実行結果**

```text
rm '.env'
D  .env
```

`D` は「次のコミットで、このファイルを消します（Delete）」という印です。このままコミットします。

```powershell
git commit -m ".env を Git の管理から外す"
```

これで、最新のコミットには `.env` がありません。ところが、履歴を調べると、`.env` は残っています。

```powershell
# コミットの一覧を表示する
git log --oneline
# .env に関わるコミットだけを表示する
git log --oneline -- .env
# 1つ前のコミット（HEAD~1）の中の .env を表示する
git show HEAD~1:.env
```

**▼ 実行結果**

```text
50956ca .env を Git の管理から外す
45dbcd4 うっかり .env を入れた
d779d78 最初のコミット
50956ca .env を Git の管理から外す
45dbcd4 うっかり .env を入れた
HOST=127.0.0.1
PORT=3000
DATABASE_URL=libsql://rustnote-taro.turso.io
DATABASE_AUTH_TOKEN=eyJhbGciOi...secret
```

- **何が起きたか:** `git rm --cached` で外したのは、「これからのコミット」からだけです。`45dbcd4` のコミットには、`.env` の中身がそのまま残っていて、`git show コミット:ファイル名` で誰でも読めます。図で見た「前のページに残った鍵の写真」です。
- **直し方:** 手元のリポジトリだけで、まだどこにも上げていなければ、上げる前に履歴を作り直す方法があります（少し難しいので、このシリーズでは扱いません）。**GitHub などに一度でも上げてしまったら、その秘密はもう漏れたものとして扱います**。まず、その**トークンやパスワードを無効にして、新しいものに取り替えます**。Turso のトークンを無効にする方法は、第7巻で扱います。履歴の掃除は、その後です。
- **防ぎ方:** `.gitignore` に `.env` を書くのは、**最初のコミットより前**にします。そして、`-f` は使いません。

### RustNote ではここで使う

RustNote のフォルダ（`Rust/rustnote`）の `.gitignore` は、この単元で作ったものとほとんど同じです。

```text
# ビルドの成果物（cargo build / topcoat asset bundle が作る）
/target/

# ローカルの SQLite / libSQL データベース（WAL などの付属ファイルも含む）
*.db
*.db-shm
*.db-wal
*.db-journal

# 秘密の値を書いた環境変数ファイル（.env.example だけを共有する）
.env

# OS やエディタが作るファイル
.DS_Store
Thumbs.db
```

- `*.db-journal` は、データベースのもう1つの作業用ファイルです。`.DS_Store`（macOS）と `Thumbs.db`（Windows）は、OS がフォルダの中に勝手に作るファイルです。
- `.env.example` は Git に入っていて、RustNote の README では、使う人が `Copy-Item .env.example .env` でコピーして使う手順になっています（手元で動かすだけなら、中身はそのままで動きます）。本番の Turso のトークンのような**本物の値は、コピーした `.env` にだけ書く**のです。`.env.example` の最後は次のようになっていて、トークンの欄は空です。

```text
DATABASE_URL=rustnote.db
# 本番（Turso）の例:
# DATABASE_URL=libsql://rustnote-<あなたの組織名>.turso.io
# DATABASE_AUTH_TOKEN=<turso db tokens create で作ったトークン>
DATABASE_AUTH_TOKEN=
```

一方、RustNote のフォルダには `.gitattributes` がありません。この本のリポジトリで、RustNote のファイルの改行コードを表示すると、次のようになります（最初の5つだけ載せています）。

```powershell
git ls-files --eol Rust/rustnote
```

**▼ 実行結果**（抜粋）

```text
i/lf    w/lf    attr/                 	Rust/rustnote/.dockerignore
i/lf    w/lf    attr/                 	Rust/rustnote/.env.example
i/lf    w/lf    attr/                 	Rust/rustnote/.gitignore
i/lf    w/lf    attr/                 	Rust/rustnote/Cargo.lock
i/lf    w/lf    attr/                 	Rust/rustnote/Cargo.toml
```

- 今はリポジトリの中も作業フォルダも LF ですが、`attr/` が空なので、`core.autocrlf=true` の Windows の PC で取り出すと、作業フォルダでは CRLF になります。
- RustNote は、最後は Fly.io（Linux）の上で、Docker を使って動かします（第12巻）。自分のリポジトリで RustNote を管理するときは、この単元の `.gitattributes` をいちばん上のフォルダに置いておくと、`Dockerfile` や `fly.toml` のような Linux の世界で使うファイルを、LF のまま保てます。

### 練習問題

**問1:** Rust のプロジェクトのフォルダに、次のファイルがあります。Git に入れる（コミットする）ものと、入れないものに分けてください。入れないものは、`.gitignore` のどの書き方で外せるかも答えてください。

`Cargo.toml`・`Cargo.lock`・`src/main.rs`・`target/debug/rustnote.exe`・`.env`・`.env.example`・`rustnote.db`・`rustnote.db-wal`・`README.md`

**問2:** 例2-5の手順1の `git ls-files --eol` では、どのファイルも `i/lf    w/lf    attr/` でした。

1. `i/lf`・`w/lf`・`attr/` は、それぞれ何を表していますか。
2. この状態（`.gitattributes` が無い）でコミットしたリポジトリを、`core.autocrlf=true` の別の Windows の PC で取り出すと、作業フォルダのファイルの改行は何になりますか。そのファイルを WSL の bash で使うと、何が起きる可能性がありますか。

**問3:** Turso のトークンを書いた `.env` をコミットして、GitHub に上げてしまったことに気づきました。何を、どの順番でしますか。

### 解答と解説

**問1の解答:**

| 入れる | 入れない（`.gitignore` の書き方） |
|---|---|
| `Cargo.toml`・`Cargo.lock`・`src/main.rs`・`.env.example`・`README.md` | `target/debug/rustnote.exe`（`/target/`）・`.env`（`.env`）・`rustnote.db`（`*.db`）・`rustnote.db-wal`（`*.db-wal`） |

- `Cargo.lock` は、使った部品（クレート）の正確な版を記録したファイルです（UNIT 04 で出てきます）。RustNote のような**アプリ**では、誰が・いつビルドしても同じ版の部品が使われるように、Git に入れます。RustNote のフォルダにも `Cargo.lock` が入っています。
- `target` の中は、`cargo build` でいつでも作り直せるので、入れません。データベースのファイルは、中身が毎回変わり、利用者のデータが入るので入れません。

**問2の解答:**

1. `i/lf` はステージ（index）の中が LF、`w/lf` は作業フォルダ（working tree）のファイルが LF、`attr/` は `.gitattributes` の決まりが何も無いことを表します。
2. 決まりが無いので、取り出す PC の `core.autocrlf=true` に従って、作業フォルダのファイルは **CRLF** になります。例2-5の手順1の警告「LF will be replaced by CRLF the next time Git touches it」が、まさにこのことを言っていました。そのファイルがシェルスクリプトなら、WSL の bash で動かしたときに、エラー2のように `$'\r': command not found` などのエラーになる可能性があります。

**問3の解答:**

1. **トークンを無効にして、新しいトークンに取り替える。** GitHub に上げた時点で、誰かに読まれたかもしれないものとして扱います。履歴を消すより先に、鍵そのものを使えなくするのがいちばん大事です。
2. **`.env` を Git の管理から外す。** `git rm --cached .env` をして、`.gitignore` に `.env` があることを確かめ、コミットします。新しいトークンは `.env` にだけ書きます。
3. **必要なら、履歴から消す。** ただし、消しても「一度漏れた」ことは取り消せません。1のトークンの取り替えを必ず先に済ませます。

### まとめ

- Git は、作業フォルダ → `git add`（ステージ）→ `git commit`（リポジトリ）の順に記録する。最初に名前・メールアドレス・`init.defaultBranch main` を global に設定し、設定の場所は `--show-origin` で確かめる。
- `.gitignore` には、最初のコミットより前に、`/target/`・`*.db`・`.env` を書く。一度コミットした秘密は、外しても履歴に残るので、上げてしまったら、まずトークンを取り替える。
- 改行コードは、リポジトリごとに `.gitattributes`（`* text=auto eol=lf`）で LF に決める。`\r` や `^M` が見えたら、CRLF を疑う。

## UNIT 03: VS Code と rust-analyzer

### この単元のゴール

- Visual Studio Code（VS Code）の ARM64 版をインストールし、`code --version` で版と CPU の種類を確かめられる
- `code .` でフォルダを開き、VS Code の中のターミナル（統合ターミナル）で PowerShell を使える
- 拡張機能 **rust-analyzer** をコマンドで入れて、入ったことを一覧で確かめられる
- `settings.json` に「改行は LF」「保存のたびに整形」「clippy でチェック」の設定を書ける。ユーザーの設定とワークスペースの設定の違いを説明できる
- rust-analyzer が動かないときに、原因（Rust がまだ無い・開いたフォルダが違う）を切り分けられる

### たとえ話でつかむ

**VS Code** は、工房の**作業机**です。机の上には、ファイルの棚（エクスプローラー）、作業中の書類（エディタ）、受付の係員と話す窓口（ターミナル）が、1つにまとめて置かれています。

その机の横に、Rust にとても詳しい**先輩**に座ってもらいます。これが **rust-analyzer**（ラスト・アナライザー）です。

- あなたがコードを打っている間に、先輩は「その変数の型はこれだよ」「その名前のつづり、違うよ」と、横から教えてくれます。
- 書類を机に置く（保存する）たびに、先輩は書類をきれいに清書し直し（整形）、もっと良い書き方があれば赤ペンで書き込んでくれます（clippy のチェック）。
- ただし、先輩は**辞書と道具箱**（Rust の本体。UNIT 04 で入れます）を見ながら仕事をします。辞書と道具箱がまだ届いていないと、先輩は「道具が見つかりません」と言って、何もできません。
- 先輩は、**書類の束の表紙**（`Cargo.toml`）を見て、どこからどこまでが1つの仕事かを判断します。表紙の無い引き出しを渡しても、先輩は仕事を始めません。

机の使い方の決まり（「改行は LF で書く」「保存したら清書する」）は、**ルール表**（`settings.json`）に書いて、机に貼っておきます。

| 工房のたとえ | VS Code の言葉 | この単元で使うもの |
|---|---|---|
| 作業机 | VS Code の窓 | Visual Studio Code 1.129.1 |
| 机の上の窓口 | 統合ターミナル | PowerShell 5.1 |
| 横に座る先輩 | 拡張機能 rust-analyzer | rust-analyzer 0.3.3081 |
| 先輩の辞書と道具箱 | Rust の本体（cargo・rustc・rustfmt・clippy） | UNIT 04 で入れる |
| 書類の束の表紙 | `Cargo.toml` | 開いたフォルダの中にある必要がある |
| 机に貼るルール表 | 設定ファイル `settings.json` | ユーザーの設定・ワークスペースの設定 |

> **用語:** 拡張機能（extension：VS Code にあとから足す部品。言語ごとの補完やチェック、表示の日本語化などがある）

<!-- -->

> **用語:** rust-analyzer（ラスト・アナライザー：Rust のための拡張機能。打っている間の補完・型の表示・定義へのジャンプ・エラーの表示をする。中で `cargo` を呼び出して、プロジェクトの形を調べている）

### 図で理解する

まず、VS Code の画面の部分の名前です。この本では、次の名前で場所を指します。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 380" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-031" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="16" width="680" height="330" rx="8" fill="#ffffff" stroke="#475569" stroke-width="2"/>
<rect x="20" y="16" width="680" height="28" rx="8" fill="#e2e8f0"/>
<text x="360" y="35" font-size="12" fill="#0f172a" text-anchor="middle">hello-git - Visual Studio Code</text>
<rect x="20" y="44" width="40" height="276" fill="#334155"/>
<text x="40" y="74" font-size="11" fill="#ffffff" text-anchor="middle">①</text>
<text x="40" y="104" font-size="11" fill="#ffffff" text-anchor="middle">②</text>
<text x="40" y="134" font-size="11" fill="#ffffff" text-anchor="middle">③</text>
<rect x="60" y="44" width="170" height="276" fill="#f1f5f9"/>
<text x="72" y="66" font-size="12" font-weight="700" fill="#0f172a">エクスプローラー</text>
<text x="80" y="90" font-size="12" fill="#0f172a" font-family="Consolas, monospace">HELLO-GIT</text>
<text x="90" y="112" font-size="12" fill="#0f172a" font-family="Consolas, monospace">src</text>
<text x="104" y="132" font-size="12" fill="#0f172a" font-family="Consolas, monospace">main.rs</text>
<text x="90" y="152" font-size="12" fill="#0f172a" font-family="Consolas, monospace">.gitattributes</text>
<text x="90" y="172" font-size="12" fill="#0f172a" font-family="Consolas, monospace">.gitignore</text>
<text x="90" y="192" font-size="12" fill="#0f172a" font-family="Consolas, monospace">Cargo.toml</text>
<rect x="230" y="44" width="470" height="170" fill="#ffffff"/>
<text x="246" y="70" font-size="12" fill="#64748b" font-family="Consolas, monospace">main.rs</text>
<text x="246" y="98" font-size="12" fill="#1e40af" font-family="Consolas, monospace">fn main() {</text>
<text x="262" y="120" font-size="12" fill="#0f172a" font-family="Consolas, monospace">    println!("Hello, world!");</text>
<text x="246" y="142" font-size="12" fill="#1e40af" font-family="Consolas, monospace">}</text>
<rect x="230" y="214" width="470" height="106" fill="#0f172a"/>
<text x="246" y="236" font-size="12" fill="#e2e8f0">ターミナル</text>
<text x="246" y="262" font-size="12" fill="#facc15" font-family="Consolas, monospace">PS C:\Users\...\rust-study\hello-git&gt;</text>
<rect x="20" y="320" width="680" height="26" fill="#2563eb"/>
<text x="32" y="338" font-size="12" fill="#ffffff">main</text>
<text x="470" y="338" font-size="12" fill="#ffffff">UTF-8</text>
<text x="524" y="338" font-size="12" fill="#ffffff">LF</text>
<text x="554" y="338" font-size="12" fill="#ffffff">Rust</text>
<text x="600" y="338" font-size="12" fill="#ffffff">rust-analyzer</text>
<text x="40" y="370" font-size="11" fill="#475569">①エクスプローラー ②検索 ③ソース管理（Git）</text>
<text x="404" y="370" font-size="11" fill="#475569">左下: ブランチ　右下: 文字コード・改行コード・言語</text>
<line x1="465" y1="190" x2="465" y2="206" stroke="#475569" stroke-width="1.5"/>
<text x="465" y="186" font-size="11" fill="#475569" text-anchor="middle">エディタ</text>
<text x="665" y="236" font-size="11" fill="#94a3b8" text-anchor="end">パネル（統合ターミナル）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-9: VS Code の画面イメージ。左端に切り替えのアイコン、その右にエクスプローラー、中央にエディタ、下にターミナル、いちばん下のステータスバーに改行コードや文字コードが出る</div>
</div>

- いちばん下の青い帯は**ステータスバー**です。右側の `UTF-8` は今のファイルの文字コード、`LF` は改行コードです。ここを押すと、文字コードや改行コードを変えられます（例3-4で使います）。
- 左端の縦の帯のアイコンで、エクスプローラー・検索・ソース管理（Git）・拡張機能などを切り替えます。

次に、rust-analyzer がどうやって仕事をしているかです。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-032" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="16" y="40" width="160" height="150" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="96" y="66" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">VS Code</text>
<text x="96" y="92" font-size="12" fill="#0f172a" text-anchor="middle">コードを打つ</text>
<text x="96" y="114" font-size="12" fill="#0f172a" text-anchor="middle">保存する</text>
<text x="96" y="146" font-size="12" fill="#b91c1c" text-anchor="middle">赤い波線</text>
<text x="96" y="168" font-size="12" fill="#1e40af" text-anchor="middle">型の表示・補完</text>
<rect x="236" y="40" width="200" height="150" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="336" y="66" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">rust-analyzer</text>
<text x="336" y="88" font-size="11" fill="#475569" text-anchor="middle">拡張機能に入っている</text>
<text x="336" y="104" font-size="11" fill="#475569" text-anchor="middle">rust-analyzer.exe（サーバー）</text>
<text x="336" y="134" font-size="12" fill="#0f172a" text-anchor="middle">プロジェクトの形を調べる</text>
<text x="336" y="156" font-size="12" fill="#0f172a" text-anchor="middle">保存のたびにチェック</text>
<rect x="496" y="40" width="208" height="150" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="600" y="66" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">Rust の本体（UNIT 04）</text>
<text x="512" y="94" font-size="12" fill="#0f172a" font-family="Consolas, monospace">cargo metadata</text>
<text x="512" y="116" font-size="12" fill="#0f172a" font-family="Consolas, monospace">cargo check / clippy</text>
<text x="512" y="138" font-size="12" fill="#0f172a" font-family="Consolas, monospace">rustfmt</text>
<text x="600" y="172" font-size="11" fill="#475569" text-anchor="middle">Cargo.toml と src を読む</text>
<line x1="176" y1="100" x2="232" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-032)"/>
<line x1="232" y1="140" x2="180" y2="140" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-032)"/>
<line x1="436" y1="100" x2="492" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-032)"/>
<line x1="492" y1="140" x2="440" y2="140" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-032)"/>
<text x="464" y="92" font-size="11" fill="#0f172a" text-anchor="middle">呼び出す</text>
<text x="464" y="158" font-size="11" fill="#0f172a" text-anchor="middle">結果</text>
<rect x="236" y="220" width="468" height="60" rx="10" fill="#fee2e2" stroke="#b91c1c" stroke-width="2"/>
<text x="470" y="244" font-size="12" fill="#b91c1c" text-anchor="middle">Rust が無いと: cargo が見つからない（program not found）</text>
<text x="470" y="266" font-size="12" fill="#b91c1c" text-anchor="middle">Cargo.toml が無いフォルダを開くと: そもそも動き始めない</text>
<line x1="600" y1="190" x2="600" y2="216" stroke="#b91c1c" stroke-width="2" stroke-dasharray="4 3" marker-end="url(#ar-01-032)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-10: rust-analyzer の仕事の流れ。VS Code の拡張機能に入っているサーバーが、Rust の本体（cargo など）を呼び出してプロジェクトを調べ、結果を画面に返す</div>
</div>

- rust-analyzer の拡張機能には、本体のプログラム（`rust-analyzer.exe`）が入っています。ただし、プロジェクトの形を調べたり、チェックしたりするのには、**Rust の本体の `cargo` を呼び出します**。だから、Rust を入れる UNIT 04 までは、rust-analyzer は本当の力を出せません。
- この単元で拡張機能を先に入れておくと、UNIT 04 で Rust を入れた瞬間から、補完やチェックが使えるようになります。

最後に、設定の段です。UNIT 02 の Git の設定と同じように、VS Code の設定も段になっていて、**下の段ほど優先**されます。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-033" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="60" y="16" width="600" height="56" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="80" y="40" font-size="13" font-weight="700" fill="#0f172a">既定の設定</text>
<text x="80" y="60" font-size="12" fill="#475569">VS Code に最初から決まっている値（例: files.eol は "auto"）</text>
<rect x="60" y="92" width="600" height="56" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="80" y="116" font-size="13" font-weight="700" fill="#1e40af">ユーザーの設定</text>
<text x="80" y="136" font-size="12" fill="#475569">あなたが開くすべてのフォルダに効く（<tspan font-family="Consolas, monospace">%APPDATA%\Code\User\settings.json</tspan>）</text>
<rect x="60" y="168" width="600" height="56" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="80" y="192" font-size="13" font-weight="700" fill="#047857">ワークスペースの設定</text>
<text x="80" y="212" font-size="12" fill="#475569">そのフォルダだけに効く（フォルダの中の <tspan font-family="Consolas, monospace">.vscode\settings.json</tspan>）</text>
<line x1="690" y1="24" x2="690" y2="220" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-033)"/>
<text x="704" y="90" font-size="12" fill="#475569" writing-mode="tb">下ほど優先</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-11: VS Code の設定の段。既定の設定をユーザーの設定が上書きし、さらにフォルダごとのワークスペースの設定が上書きする</div>
</div>

- 自分の好み（文字の大きさなど）や、どのプロジェクトでも同じにしたいこと（改行は LF）は、**ユーザーの設定**に書きます。
- そのプロジェクトの決まり（このプロジェクトでは clippy でチェックする、など）は、**ワークスペースの設定**に書いて、Git でチームと共有します。

### まず動かす最小コード

この単元の「最小コード」も、コマンドと画面の操作です。VS Code を入れて、拡張機能を入れて、UNIT 02 の `hello-git` を開くところまで進めます。

#### 例3-1: VS Code をインストールして、版と CPU の種類を確かめる

> **▼ この手順がやること（先に日本語で）:** VS Code の公式サイトからインストーラーを入手して、VS Code を入れます。新しく開いた PowerShell で、VS Code の版と、どの CPU 用の VS Code が入ったかを表示します。

**手順1: インストーラーを入手する。** ブラウザで VS Code の公式サイト（`https://code.visualstudio.com/`）を開き、ダウンロードのページから、Windows 用の **User Installer**（ユーザー用のインストーラー）を選びます。ARM64 の PC では **Arm64** 用、x64 の PC では x64 用を選びます。

**手順2: インストールする。** インストーラーを実行すると、次の順に画面が出ます（画面の名前は、この本の PC に入っている VS Code のインストーラーのものです）。

1. 「**使用許諾契約書の同意**」: 内容を読み、「**同意する**」を選んで「次へ」を押します。
2. インストール先や、スタートメニューのフォルダの画面は、既定のまま「次へ」を押します。
3. 「**追加タスクの選択**」: 次のように選びます。

| 項目 | 選び方 |
|---|---|
| デスクトップ上にアイコンを作成する | どちらでもよい |
| エクスプローラーのファイル コンテキスト メニューに [Code で開く] アクションを追加する | 入れておくと便利 |
| エクスプローラーのディレクトリ コンテキスト メニューに [Code で開く] アクションを追加する | 入れておくと便利 |
| サポートされているファイルの種類のエディターとして、Code を登録する | どちらでもよい |
| **PATH への追加（再起動後に使用可能）** | **必ず入れる** |

4. 「インストール」を押し、終わったら「完了」を押します。

「PATH への追加」は、PowerShell から `code` というコマンドで VS Code を動かせるようにする項目です。この本では、`code` コマンドを何度も使います。

**手順3: 新しい窓で確かめる。** PowerShell の窓をいったん閉じて、新しく開き直してから、次のコマンドを打ちます。

```powershell
# VS Code の版を表示する
code --version
# code というコマンドの本体が、どこにあるかを表示する
where.exe code
```

**▼ 実行結果**

```text
1.129.1
8a7abeba6e03ea3af87bfbce9a1b7e48fed567b8
arm64
C:\Users\...\AppData\Local\Programs\Microsoft VS Code\bin\code
C:\Users\...\AppData\Local\Programs\Microsoft VS Code\bin\code.cmd
```

#### ▼ 1つずつ分解して解説

##### 解説1: `code --version` の3行

- 1行目が VS Code の版、2行目はその版を作ったときのソースコードの番号（コミットの番号）、3行目が**どの CPU 用の VS Code か**です。
- ARM64 の PC で3行目が `arm64` なら、正しい版が入っています。`x64` と表示されたら、x64 用を入れてしまっています。動きはしますが、エミュレーションになり、その VS Code の中で開くターミナルも x64 になることがあります（UNIT 01 のエラー4と同じ問題）。いったんアンインストールして、Arm64 用を入れ直しましょう。

##### 解説2: User Installer は、あなたのユーザーのフォルダに入る

- `where.exe code` の結果のとおり、User Installer は VS Code を `C:\Users\（あなたの名前）\AppData\Local\Programs\Microsoft VS Code` に入れます。あなたのユーザーのフォルダの中なので、インストールにも更新にも管理者の権限が要りません。
- `bin` フォルダの `code` と `code.cmd` が、PowerShell から `code` と打ったときに動く入口です。「PATH への追加」で、このフォルダが `PATH` に足されました。

> **ヒント:** 表示を日本語にするには、拡張機能「Japanese Language Pack for Visual Studio Code」を入れます。PowerShell で `code --install-extension MS-CEINTL.vscode-language-pack-ja` と打ってから VS Code を開き直します。それでも英語のままなら、コマンド パレット（例3-4）で `Configure Display Language` を実行して `ja` を選びます。この本の VS Code の画面の名前（「ターミナル」「拡張機能」など）は、この日本語化を入れた状態のものです。

#### 例3-2: 拡張機能 rust-analyzer を入れて、一覧で確かめる

> **▼ この手順がやること（先に日本語で）:** PowerShell から、VS Code に拡張機能 rust-analyzer を入れます。入った拡張機能の名前と版を、一覧で表示して確かめます。

```powershell
# 拡張機能 rust-analyzer を入れる
code --install-extension rust-lang.rust-analyzer
```

**▼ 実行結果**

```text
Installing extensions...
(node:55752) [DEP0169] DeprecationWarning: `url.parse()` behavior is not standardized and prone to errors that have security implications. Use the WHATWG URL API instead. CVEs are not issued for `url.parse()` vulnerabilities.
(Use `Code --trace-deprecation ...` to show where the warning was created)
Installing extension 'rust-lang.rust-analyzer'...
Extension 'rust-lang.rust-analyzer' v0.3.3081 was successfully installed.
```

```powershell
# 入っている拡張機能の名前と版を一覧で表示する
code --list-extensions --show-versions
```

**▼ 実行結果**

```text
rust-lang.rust-analyzer@0.3.3081
```

ほかの拡張機能（日本語化など）を入れている人は、それも一緒に表示されます。

#### ▼ 1つずつ分解して解説

##### 解説1: `rust-lang.rust-analyzer` は「作った人.名前」

- 拡張機能には、`作った人の ID.拡張機能の ID` という形の名前が付いています。`rust-lang` は、Rust の開発チーム（The Rust Programming Language）の ID です。
- 似た名前の拡張機能がほかにもあるので、名前を正確に打つのが大切です。VS Code の画面から入れるときは、左端の「拡張機能」（または `Ctrl + Shift + X`）を開いて `rust-analyzer` で検索し、作った人が **The Rust Programming Language** になっているものの「インストール」を押します。

##### 解説2: 途中の DeprecationWarning は気にしなくてよい

- 2〜3行目の `DeprecationWarning`（非推奨の警告）は、VS Code のコマンドの**内部**で使われている部品についての、開発者向けのお知らせです。最後の行に `was successfully installed`（正しくインストールされました）と出ていれば成功です。
- `node:55752` の数字は、実行するたびに変わります。

##### 解説3: 拡張機能も CPU の種類ごとに作られている

- rust-analyzer の拡張機能には、本体のプログラム `rust-analyzer.exe` が入っているので、CPU の種類ごとに別のものが配られています。この本の PC では、入った拡張機能のフォルダの名前が `rust-lang.rust-analyzer-0.3.3081-win32-arm64` で、Windows の ARM64 用が自動で選ばれていました。
- VS Code そのものが ARM64 用なら、拡張機能も ARM64 用が選ばれます。例3-1で `code --version` の3行目を確かめたのは、このためでもあります。
- 拡張機能は、ふつうは `C:\Users\（あなたの名前）\.vscode\extensions` の中に、`作った人.名前-版` のフォルダとして入ります。本体のプログラムを含む拡張機能は、この本の PC では、フォルダの名前の最後に `-win32-arm64` が付いていました。拡張機能の調子がおかしいときに、どの CPU 用のものが入っているかを確かめる手がかりになります。

#### 例3-3: hello-git を VS Code で開き、統合ターミナルを使う

> **▼ この手順がやること（先に日本語で）:** UNIT 02 で作った `hello-git` フォルダを、PowerShell から VS Code で開きます。VS Code の中のターミナルを開いて、UNIT 01 のスクリプトで、そのターミナルが「どのシェルで・どこにいて・どの CPU か」を確かめます。

```powershell
# hello-git フォルダへ移動して、そのフォルダを VS Code で開く
cd ~\rust-study\hello-git
code .
```

VS Code が開き、次の順に進みます。

1. 初めて開くフォルダでは、「**このフォルダー内のファイルの作成者を信頼しますか?**」と尋ねる画面が出ます。自分で作ったフォルダなので、「**はい、作成者を信頼します**」を押します。
2. 左のエクスプローラーに、`src`・`.gitattributes`・`.gitignore`・`Cargo.toml` などが並びます。`.env` や `target` のように `.gitignore` で外したものは、薄い色で表示されます。
3. 上のメニューの「**ターミナル**」→「**新しいターミナル**」を押すと、下に統合ターミナルが開きます。

統合ターミナルで、UNIT 01 のスクリプトを実行します。

```powershell
# 1つ上のフォルダ（rust-study）にある whereami.ps1 を実行する
..\whereami.ps1
```

「現在のフォルダ」が `...\rust-study\hello-git` で、「PC の CPU」と「このシェルの種類」が同じ（ARM64 の PC なら両方 `ARM64`）なら、統合ターミナルは正しく使えています。

#### ▼ 1つずつ分解して解説

##### 解説1: `code .` の `.` は「今いるフォルダ」

- `code .` は「今いるフォルダ（`.`）を VS Code で開いて」というお願いです。`code src\main.rs` のようにファイルの名前を渡すと、そのファイルを開きます。
- VS Code では、**ファイル1つではなく、プロジェクトのフォルダを開いて**作業します。rust-analyzer も、開いたフォルダの中から `Cargo.toml` を探して仕事を始めるからです（「よくあるエラー」のエラー3）。

##### 解説2: 「作成者を信頼しますか?」の意味

- フォルダの中には、VS Code の設定や、拡張機能が自動で実行するプログラムを書けます。インターネットから取ってきた知らないフォルダを開いただけで、それが勝手に動くと危険です。
- 「いいえ、作成者を信頼しません」を選ぶと、**制限モード**で開きます。制限モードでは、rust-analyzer のような拡張機能が一部しか動きません。自分で作ったフォルダや、信頼できる人のフォルダだけ「はい」を選びましょう。

##### 解説3: 統合ターミナルは、開いたフォルダから始まる

- 統合ターミナルは、VS Code で開いたフォルダ（ここでは `hello-git`）を「今いるフォルダ」として始まります。`cd` しなくても、すぐにそのプロジェクトのコマンドを打てます。
- Windows の VS Code の統合ターミナルは、ふつうは PowerShell で開きます。中身は、Windows Terminal で開く PowerShell と同じです。

> **ヒント:** よく使う操作には、キーボードの近道があります。最初は次の5つだけ覚えておけば十分です。
>
> | キー | やること |
> |---|---|
> | `Ctrl + Shift + P` | コマンド パレットを開く（操作を名前で探す） |
> | `Ctrl + P` | ファイルを名前で探して開く |
> | `Ctrl + S` | 保存する |
> | `Ctrl + Shift + E` | エクスプローラーを開く |
> | `Ctrl + Shift + X` | 拡張機能の画面を開く |

### もう一歩: 実務に近い形

#### 例3-4: settings.json に、改行・整形・チェックの決まりを書く

> **▼ この手順がやること（先に日本語で）:** VS Code の**ユーザーの設定**のファイル `settings.json` を開き、「新しいファイルの改行は LF」「ファイルの最後に改行を1つ入れる」「行の最後の空白を消す」「保存のたびに整形する」「Rust の整形は rust-analyzer に任せる」「保存のたびのチェックを clippy で行う」「PowerShell のスクリプトは BOM 付きの UTF-8 で保存する」という決まりを書きます。

**手順1: settings.json を開く。** VS Code で `Ctrl + Shift + P` を押すと、画面の上に**コマンド パレット**（VS Code のすべての操作を、名前で探して実行できる入力欄）が開きます。`ユーザー設定を開く` と入力し、「**基本設定: ユーザー設定を開く (JSON)**」を選びます。

**手順2: 次の内容にする。** 初めて開いたときは `{}` だけか、空のことがあります。そのときは、全体を次の内容に置き換えます。すでに何か書かれているときは、`{` と `}` の内側に、次の行を書き足します（行と行の間は `,` で区切ります）。

```json
{
    "files.eol": "\n",
    "files.insertFinalNewline": true,
    "files.trimTrailingWhitespace": true,
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    },
    "rust-analyzer.check.command": "clippy",
    "[powershell]": {
        "files.encoding": "utf8bom"
    }
}
```

**手順3: 保存して、確かめる。** `Ctrl + S` で保存します。確かめ方は次のとおりです。

- 「ファイル」→「新しいテキスト ファイル」で新しいファイルを作り、画面の右下のステータスバーに **`LF`** と表示されることを確かめます。
- 左のエクスプローラーの「新しいファイル...」で `test.ps1` という名前のファイルを作って開き、ステータスバーに **`UTF-8 with BOM`** と表示されることを確かめます（確かめたら、`test.ps1` は消してかまいません）。
- Rust のファイルの整形とチェックは、UNIT 04 で Rust を入れた後に効き始めます。

#### ▼ 1つずつ分解して解説

##### 解説1: 改行と空白の決まり

| 設定 | 値 | 意味（既定の値） |
|---|---|---|
| `files.eol` | `"\n"` | 新しく作るファイルの改行を LF にする（既定は `"auto"` で、Windows では CRLF になる） |
| `files.insertFinalNewline` | `true` | 保存するとき、ファイルの最後に改行を1つ入れる（既定は `false`） |
| `files.trimTrailingWhitespace` | `true` | 保存するとき、行の最後の余分な空白を消す（既定は `false`） |

- `files.eol` は、UNIT 02 の `.gitattributes` と同じ考え方を、エディタの側でも守るための設定です。既定の `"auto"` は「OS 固有の改行文字を使用します」という意味で、Windows では CRLF になってしまいます。
- この設定が効くのは**新しく作るファイル**です。すでに CRLF のファイルを開いたときは、ステータスバーの `CRLF` を押して出てくる「**改行コードの選択**」で `LF` を選び、保存し直します。
- 設定の値（`"auto"` が既定であること、`"\n"`・`"\r\n"`・`"auto"` の3つから選べること）は、この本の PC の VS Code 1.129.1 の中身で確かめました。

##### 解説2: 保存したら整形する

- `editor.formatOnSave` を `true` にすると、保存のたびに**整形**（決まった形に、字下げや空白を直すこと）が走ります。
- `"[rust]": { … }` のように、`[言語の名前]` で囲んだ中に書いた設定は、**その言語のファイルにだけ**効きます。ここでは「Rust のファイルの整形は、rust-analyzer に任せる」（`editor.defaultFormatter`）と決めています。rust-analyzer は、Rust の標準の整形の道具 **rustfmt**（ラスト・フォーマット。UNIT 04 で Rust と一緒に入ります）を使って整形します。
- 整形の道具が決まっていない言語のファイルでは、保存しても整形はされません。

##### 解説3: 保存のたびに clippy でチェックする

- rust-analyzer は、保存するたびに、裏で `cargo check`（コンパイルできるかの確認）を実行して、エラーを赤い波線で表示します。`rust-analyzer.check.command` を `"clippy"` にすると、代わりに **`cargo clippy`**（よくある書き間違いや、もったいない書き方も指摘する道具。UNIT 06）が使われます。
- この設定の名前と、既定の値が `"check"` であることは、rust-analyzer 0.3.3081 の拡張機能の設定の一覧（`package.json`）で確かめました。

##### 解説4: PowerShell のスクリプトは BOM 付きで保存する

- UNIT 01 のエラー2のとおり、Windows PowerShell 5.1 は、BOM の無い `.ps1` の日本語を正しく読めません。VS Code の既定の文字コード（`files.encoding`）は `utf8`（BOM なし）なので、VS Code で新しく作った `.ps1` に日本語を書いて保存すると、同じ問題が起きます（「よくあるエラー」のエラー4）。
- `"[powershell]": { "files.encoding": "utf8bom" }` は、「PowerShell のファイルだけ、BOM 付きの UTF-8 で保存する」という設定です。
- すでに BOM なしで保存してしまったファイルは、ステータスバーの `UTF-8` を押し、「**エンコード付きで保存**」→「**UTF-8 with BOM**」を選ぶと、BOM 付きで保存し直せます。

> **注意:** `settings.json` は、JSON（ジェイソン）という形式で書きます。文字列は `"` で囲み、項目と項目の間は `,` で区切ります。`,` の付け忘れや、`"` の閉じ忘れがあると、その部分に波線が出て、設定が読み込まれません。

#### 例3-5: プロジェクトの決まりを .vscode フォルダに入れて共有する

> **▼ この手順がやること（先に日本語で）:** `hello-git` の中に `.vscode` フォルダを作り、「このプロジェクトでは rust-analyzer を使ってください」という**おすすめの拡張機能**の一覧（`extensions.json`）と、このプロジェクトだけの設定（`settings.json`）を置きます。最後に、Git から見るとどう見えるかを確かめます。

VS Code の左のエクスプローラーで、`HELLO-GIT` の右に出るアイコンの「新しいファイル...」を押し、`.vscode/extensions.json` と打って Enter キーを押します（`/` で区切ると、`.vscode` フォルダも一緒に作られます）。中身を次のようにして保存します。

```json
{
    "recommendations": [
        "rust-lang.rust-analyzer"
    ]
}
```

同じようにして、`.vscode/settings.json` を作ります。

```json
{
    "files.eol": "\n",
    "rust-analyzer.check.command": "clippy"
}
```

統合ターミナルで、Git から見た状態を確かめます。

```powershell
git status
```

**▼ 実行結果**

```text
On branch main
Untracked files:
  (use "git add <file>..." to include in what will be committed)
	.vscode/

nothing added to commit but untracked files present (use "git add" to track)
```

#### ▼ 1つずつ分解して解説

##### 解説1: `extensions.json` は「このプロジェクトのおすすめ」

- `.vscode/extensions.json` の `recommendations` に拡張機能の名前を並べておくと、このフォルダを開いた人の VS Code の「拡張機能」の画面に、「**ワークスペースの推奨事項**」という欄ができて、そこに並びます。チームの全員が、同じ道具をそろえやすくなります。
- 勝手にインストールされるわけではありません。入れるかどうかは、開いた人が決めます。

##### 解説2: ワークスペースの設定は、Git で共有する

- `.vscode/settings.json` は、図で見た「ワークスペースの設定」です。ユーザーの設定より優先されるので、このプロジェクトでは、誰の VS Code でも「改行は LF・clippy でチェック」になります。
- `git status` に `.vscode/` が出ているとおり、このフォルダは `.gitignore` で外していないので、`git add` してコミットすれば、チームと共有できます。
- 個人の好み（文字の大きさや色の設定など）は、ここではなく、ユーザーの設定に書きます。

##### 解説3: Git のメッセージを書くエディタを VS Code にする

- UNIT 02 で、`git commit` を `-m` なしで打つと Vim が開く、という話をしました。次の設定をすると、代わりに VS Code が開くようになります。

```powershell
# Git がメッセージを書いてもらうエディタを、VS Code にする
git config --global core.editor "code --wait"
```

- `--wait` は、「VS Code でそのファイルを閉じるまで、コマンドを待たせる」という指定です。メッセージを書いて保存し、タブを閉じると、コミットが続きます。成功しても何も表示されません。

#### 例3-6: 変更した内容を、VS Code のソース管理とターミナルの両方で見る

> **▼ この手順がやること（先に日本語で）:** `src/main.rs` の表示の文字を1か所だけ変えて保存し、VS Code の「ソース管理」の画面で、どこが変わったかを確かめます。同じことを、統合ターミナルの `git status --short` と `git diff` でも確かめ、最後に変更を元に戻します。

1. エクスプローラーで `src/main.rs` を開き、`"Hello, world!"` を `"Hello, Rust!"` に書き換えて、`Ctrl + S` で保存します。
2. 左端の「**ソース管理**」のアイコンを押します（`Ctrl + Shift + G` でも開きます）。「変更」の一覧に `main.rs` が並び、右に `M`（Modified：変更あり）の印が付きます。
3. 一覧の `main.rs` を押すと、変更前と変更後が左右に並んで表示され、変わった行に色が付きます。

同じことを、統合ターミナルで確かめます。

```powershell
# 変わったファイルを短い形で表示する
git status --short
# 変わった中身（差分）を表示する
git diff
```

**▼ 実行結果**（例3-5の `.vscode` フォルダを作った後の状態です）

```text
 M src/main.rs
?? .vscode/
warning: in the working copy of 'check.ps1', LF will be replaced by CRLF the next time Git touches it
diff --git a/src/main.rs b/src/main.rs
index e7a11a9..66449d7 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,3 +1,3 @@
 fn main() {
-    println!("Hello, world!");
+    println!("Hello, Rust!");
 }
```

確かめたら、変更を元に戻しておきます。

```powershell
# src/main.rs を、最後のコミットの状態に戻す（成功しても何も表示されない）
git restore src/main.rs
```

#### ▼ 1つずつ分解して解説

##### 解説1: `git status --short` の印

- `--short` を付けると、1つのファイルを1行で表示します。左の2文字が状態の印です。
- ` M`（左が空白で右が `M`）は「作業フォルダで変更したが、まだステージに載せていない」、`??` は「Git がまだ追跡していない」という意味です。UNIT 02 のエラー3で見た `D ` は「削除をステージに載せた」でした。左の文字がステージ、右の文字が作業フォルダの状態を表します。

##### 解説2: `git diff` の読み方

- `---` の行が変更前（`a/`）、`+++` の行が変更後（`b/`）のファイルです。
- `@@ -1,3 +1,3 @@` は「変更前の1行目から3行、変更後の1行目から3行の範囲を表示します」という意味です。
- 行の先頭の `-` が消えた行、`+` が増えた行、空白が変わっていない行です。VS Code のソース管理の左右の表示は、これを見やすく並べたものです。

##### 解説3: 途中の警告は、UNIT 02 で見たものと同じ

- `git diff` の最初に出た `check.ps1` の警告は、UNIT 02 の例2-5の手順2と同じものです。`.gitattributes` で `check.ps1` を CRLF にすると決めたのに、作業フォルダの `check.ps1` がまだ LF のままなので、Git が中を調べるたびに知らせてくれています。害はありません。
- `git restore` は、作業フォルダのファイルを、最後のコミットの状態に戻すコマンドです。保存していない変更ではなく、**保存済みの変更を捨てる**操作なので、本当に戻してよいかを確かめてから打ちましょう。

### よくあるエラーと読み方

#### エラー1: 「用語 'code' は、…認識されません」（PATH に入っていない）

```text
code : 用語 'code' は、コマンドレット、関数、スクリプト ファイル、または操作可能なプログラムの名前として認識されません。名前が正しく記述されていることを確認し、パスが含まれている場合はそのパスが正しいことを確認してから、再試行してください。
発生場所 行:1 文字:1
+ code --version
+ ~~~~
    + CategoryInfo          : ObjectNotFound: (code:String) [], CommandNotFoundException
    + FullyQualifiedErrorId : CommandNotFoundException
```

- **何が起きたか:** PowerShell が `PATH` のフォルダを探しましたが、`code` が見つかりませんでした。この本の PC で、`PATH` から VS Code の `bin` フォルダを抜いた PowerShell で `code --version` を打つと、このエラーになりました。
- **直し方:** 次の順に確かめます。
  1. **窓を開き直したか。** インストーラーが足した `PATH` は、インストールの後に新しく開いた窓から有効になります（インストーラーの項目の名前が「再起動後に使用可能」になっているのは、このためです）。
  2. **「PATH への追加」を選んだか。** 選び忘れたときは、もう一度インストーラーを実行して、「追加タスクの選択」でこの項目を選び直します。
  3. 開き直しても直らないときは、PC を再起動してみます。

#### エラー2: 「Failed to load the project」「program not found」（Rust がまだ無い）

rust-analyzer を入れて、Rust を入れる前に `Cargo.toml` のあるフォルダを開くと、rust-analyzer はプロジェクトを読み込めずにエラーになります。VS Code の画面では、`rust-analyzer failed to load workspace` のような知らせが出ます。何が起きているかを、rust-analyzer の本体をコマンドで直接動かして確かめると、次のようになりました（この本の PC で、`cargo` が見つからない状態にして、`hello-git` で実行したものです。長いので、最後の部分だけ載せています）。

```text
Error: Failed to load the project at ...\hello-git\.\Cargo.toml

Caused by:
    0: Failed to read Cargo metadata from Cargo.toml file ...\hello-git\.\Cargo.toml, None
    1: Failed to run `"cargo" "metadata" "--format-version" "1" "--manifest-path" "...\\hello-git\\.\\Cargo.toml"`
    2: program not found
```

- **何が起きたか:** 下から読みます。`program not found`（プログラムが見つかりません）が根っこの原因で、その上の行が「`cargo metadata` を実行できませんでした」、さらに上が「Cargo のメタデータ（プロジェクトの情報）を読めませんでした」、いちばん上が「プロジェクトを読み込めませんでした」です。`Caused by:`（原因は）の下に、原因の原因が順に並んでいます。
- 図で見たとおり、rust-analyzer はプロジェクトを調べるのに `cargo` を呼び出します。Rust がまだ入っていないので、`cargo` が見つからなかったのです。
- **直し方:** UNIT 04 で Rust を入れます。入れた後は、VS Code をいったん閉じて開き直すか、コマンド パレットで「`rust-analyzer: Restart server`」を実行すると、rust-analyzer が読み込み直します。
- VS Code の中で詳しい様子を見るには、「表示」→「出力」でパネルを開き、右上の一覧から「**rust-analyzer Language Server**」を選びます。

> **ヒント:** このエラーの読み方（`Caused by:` の下から読む）は、Rust の多くのツールや、Rust で書いたアプリのエラーで同じです。RustNote の起動のエラーも、同じ形で表示されます（第4巻で使う anyhow というクレートの表示の形です）。

#### エラー3（エラーが出ない落とし穴）: rust-analyzer が何もしない（開いたフォルダが違う）

`Cargo.toml` の無いフォルダを開くと、rust-analyzer は**エラーも出さずに、何もしません**。

- rust-analyzer の拡張機能の設定（`package.json`）を見ると、動き始める条件は「開いたフォルダの中に `Cargo.toml` がある」か「1つ下のフォルダに `Cargo.toml` がある」ことになっています（`workspaceContains:Cargo.toml` と `workspaceContains:*/Cargo.toml`）。
- たとえば、`C:\Users\（あなたの名前）` をまるごと開くと、`Cargo.toml` は `rust-study\hello-git\Cargo.toml` のように2つ下にあるので、rust-analyzer は動き始めません。
- **直し方:** `cd` でプロジェクトのフォルダ（`Cargo.toml` があるフォルダ）へ移動してから、`code .` で開きます。「ファイル」→「フォルダーを開く...」で開くときも、`Cargo.toml` があるフォルダそのものを選びます。

#### エラー4: VS Code で新しく作った .ps1 が、実行するとエラーになる

例3-4の `[powershell]` の設定を書く**前**の VS Code で、新しく `.ps1` のファイルを作り、日本語のコメントや表示の文字を書いて保存したとします。実行すると、UNIT 01 のエラー2と同じ「式またはステートメントのトークン '{' を使用できません」のようなエラーになります。

- **何が起きたか:** VS Code の既定の文字コードは BOM なしの UTF-8 なので、BOM の無いファイルができました。Windows PowerShell 5.1 は、BOM の無いファイルを Shift_JIS として読みます。ステータスバーを見ると、`UTF-8 with BOM` ではなく `UTF-8` と表示されているはずです。
- **直し方:** ステータスバーの `UTF-8` を押し、「エンコード付きで保存」→「UTF-8 with BOM」を選んで保存し直します。例3-4の設定を入れておけば、新しく作る `.ps1` は、最初から BOM 付きで保存されます。
- 例1-3で `Set-Content -Encoding utf8` で作った `whereami.ps1` のように、もともと BOM 付きのファイルは、VS Code が開いたときに BOM に気づき（ステータスバーに `UTF-8 with BOM` と出ます）、同じ形で保存し直します。

### RustNote ではここで使う

RustNote を VS Code で開くときは、`Cargo.toml` がある RustNote のフォルダ（`Rust/rustnote`）そのものを開きます。PowerShell で、そのフォルダへ `cd` で移動してから `code .` と打つのが確実です。

- RustNote のフォルダには `rust-toolchain.toml`（UNIT 01）があるので、rust-analyzer が呼び出す `cargo` も、そのフォルダで決めた Rust 1.99.0 になります。VS Code の赤い波線と、ターミナルの `cargo build` の結果が食い違わないのは、両方が同じ版の Rust を使うからです。
- RustNote の自動チェック（`Rust/rustnote/ci.yml`）では、次のように clippy を「警告もエラーとして扱う」設定で実行しています。

```yaml
      - name: Clippy（警告もエラーとして扱う）
        run: cargo clippy --all-targets --locked -- -D warnings
```

  例3-4で `rust-analyzer.check.command` を `"clippy"` にしておくと、自動チェックで指摘されるのと同じ種類の注意を、保存するたびに VS Code の中で先に見られます。
- RustNote のフォルダには `rustfmt.toml`（整形の決まり）があり、`edition = "2024"` と書かれています。保存のたびの整形（`editor.formatOnSave`）でも、この決まりが使われます。
- RustNote のフォルダには `.vscode` フォルダはありません。自分のリポジトリで RustNote を管理するときは、例3-5の `extensions.json` と `settings.json` を置いておくと、別の PC で開いたときにも同じ環境をすぐに作れます。

### 練習問題

**問1:** 例3-4の `settings.json` を書いた VS Code で、次の3つの操作をしました。それぞれ、どうなるでしょうか。

1. 「ファイル」→「新しいテキスト ファイル」で作ったファイルの改行コード
2. UNIT 02 の `.gitattributes` を開いて、最後の行の後ろに空白を3つ打ってから保存したときの、その空白
3. エクスプローラーの「新しいファイル...」で作った `hello.ps1` に、日本語のコメントを書いて保存したときの、ファイルの先頭の BOM

**問2:** 次のフォルダを VS Code で開いたとき、rust-analyzer が動き始めるのはどれですか（`hello-git` の中に `Cargo.toml` があります）。理由も答えてください。

1. `C:\Users\taro\rust-study\hello-git`
2. `C:\Users\taro\rust-study`
3. `C:\Users\taro`
4. `C:\Users\taro\rust-study\hello-git\src`

**問3:** ユーザーの設定に `"rust-analyzer.check.command": "check"`、ワークスペースの設定（`.vscode/settings.json`）に `"rust-analyzer.check.command": "clippy"` と書かれています。このワークスペースを開いたとき、保存のたびに使われるのはどちらですか。また、ほかのフォルダを開いたときはどうなりますか。

### 解答と解説

**問1の解答:**

1. **LF** になります。`files.eol` を `"\n"` にしたので、新しく作るファイルは LF です。ステータスバーに `LF` と表示されます。
2. **消えます**。`files.trimTrailingWhitespace` を `true` にしたので、保存するときに行の最後の空白が消されます。
3. **BOM が付きます**。`"[powershell]": { "files.encoding": "utf8bom" }` を書いたので、PowerShell のファイルは BOM 付きの UTF-8 で保存されます。この設定が無ければ BOM なしで保存され、エラー4のとおり、Windows PowerShell 5.1 で実行するとエラーになります。

**問2の解答:** 動き始めるのは **1** と **2** です。

- rust-analyzer は、開いたフォルダの中か、1つ下のフォルダに `Cargo.toml` があるときに動き始めます。
- **1** は、開いたフォルダの中に `Cargo.toml` があります。いちばんおすすめの開き方です。
- **2** は、1つ下の `hello-git` に `Cargo.toml` があるので動き始めます。ただし、`rust-study` の中のほかのフォルダも一緒に開くことになるので、ふつうはプロジェクトのフォルダ（1）を開きます。
- **3** は、`Cargo.toml` が2つ下（`rust-study\hello-git`）にあるので、動き始めません。
- **4** は、`Cargo.toml` が1つ**上**にあるので、動き始めません。

**問3の解答:** このワークスペースでは **clippy** が使われます。ワークスペースの設定は、ユーザーの設定より優先されるからです。ほかのフォルダ（`.vscode/settings.json` に書いていないフォルダ）を開いたときは、ユーザーの設定の **check** が使われます。

### まとめ

- VS Code は CPU に合った版（ARM64 の PC なら Arm64 の User Installer）を、「PATH への追加」を選んで入れ、新しい窓の `code --version` の3行目で確かめる。拡張機能は `code --install-extension rust-lang.rust-analyzer` で入れる。
- プロジェクトは `cd` してから `code .` で、`Cargo.toml` のあるフォルダそのものを開く。rust-analyzer は `cargo` を呼び出して働くので、本当に動くのは UNIT 04 で Rust を入れてから。エラーは `Caused by:` の下から読む。
- 設定は「既定 → ユーザー → ワークスペース」の順に上書きされる。ユーザーの設定に `files.eol`・`editor.formatOnSave`・`rust-analyzer.check.command` などを書き、プロジェクトの決まりは `.vscode` に入れて共有する。

## この巻のまとめ

この巻では、教材のコードを自分の PC で動かすための土台として、シェル・Git・エディタの準備から始めました。次のチェックリストで、できるようになったことを確かめてください。自信がない項目は、括弧の中の単元に戻って、コマンドを打ち直しましょう。結果が教材と違ったときは、まず「シェルの版・今いるフォルダ・OS・CPU の種類」の4つを比べるのを忘れずに。

**Windows の土台**

- ☐ スタートメニューから Windows Terminal を開き、`$PSVersionTable.PSVersion` で PowerShell の版（5.1）を確かめられる（UNIT 01）
- ☐ `$env:PROCESSOR_ARCHITECTURE` で「このシェルの CPU の種類」を表示し、PC の CPU（ARM64 / x64）と食い違っていないかを確かめられる。x64 用のアプリから起動した PowerShell が `AMD64` になることを知っている（UNIT 01）
- ☐ 作業フォルダ `rust-study` を、短く・半角英数字だけで・OneDrive の外の場所（ホームフォルダの直下か `C:\dev`）に作れる（UNIT 01）
- ☐ `Set-Content -Encoding utf8` とヒア文字列（`@' … '@`）で BOM 付きのスクリプトを作り、`Set-ExecutionPolicy -Scope CurrentUser RemoteSigned` の後に `.\whereami.ps1` で実行できる（UNIT 01）
- ☐ 「スクリプトの実行が無効」（実行ポリシー）・「トークン '{' を使用できません」（BOM なし）・「位置指定パラメーターが見つかりません」（空白を含むパス）のエラーを読んで直せる（UNIT 01）
- ☐ パスが 260 文字に近づくと、`LNK1104` のようにリンカーが失敗することがあり、`LongPathsEnabled` に頼らず短い場所で作業するのが確実だと説明できる（UNIT 01）
- ☐ Git for Windows（ARM64 の PC では ARM64 版）を入れ、`git --version`・`where.exe git`・`git config --system --list` で確かめられる。`where` ではなく `where.exe` と打つ理由を知っている（UNIT 02）
- ☐ `user.name`・`user.email`・`init.defaultBranch main` を global に設定し、設定の段（system・global・local）と `--show-origin` を説明できる。名前とメールアドレスがコミットと一緒に公開されることを知っている（UNIT 02）
- ☐ 作業フォルダ → ステージ → リポジトリの流れを説明し、`git status`・`git add`・`git commit -m` で最初のコミットができる。「Author identity unknown」を直せる（UNIT 02）
- ☐ `.gitignore` に `/target/`・`*.db`・`.env` を書き、`git check-ignore -v` でどの決まりが効いているかを確かめられる（UNIT 02）
- ☐ CRLF と LF の違いと、`LF will be replaced by CRLF` の警告の意味を説明できる。`.gitattributes`（`* text=auto eol=lf`）で改行を決め、`git ls-files --eol` で確かめられる。`$'\r': command not found` を見て改行コードを疑える（UNIT 02）
- ☐ 一度コミットした `.env` が `git rm --cached` の後も履歴に残ることを `git show` で確かめ、上げてしまったら最初にトークンを取り替えると説明できる（UNIT 02）
- ☐ VS Code の Arm64 版を「PATH への追加」を選んで入れ、`code --version` の3行目で CPU の種類を確かめられる。`code --install-extension rust-lang.rust-analyzer` で拡張機能を入れられる（UNIT 03）
- ☐ `cd` してから `code .` で、`Cargo.toml` のあるフォルダそのものを開き、統合ターミナルを使える。「作成者を信頼しますか?」の意味を説明できる（UNIT 03）
- ☐ ユーザーの `settings.json` に `files.eol`・`editor.formatOnSave`・`"[rust]"` の整形の道具・`rust-analyzer.check.command`・`"[powershell]"` の `utf8bom` を書ける。既定 → ユーザー → ワークスペースの順に上書きされることを説明できる（UNIT 03）
- ☐ `.vscode/extensions.json` と `.vscode/settings.json` でプロジェクトの決まりを共有できる。rust-analyzer の「program not found」を `Caused by:` の下から読み、Rust がまだ無いことに気づける（UNIT 03）

> **お知らせ:** この版には UNIT 01〜03 を収録しています。UNIT 04（rustup・rustc・Cargo）・UNIT 05（MSVC Build Tools と CPU の種類）・UNIT 06（Cargo の品質コマンド）・UNIT 07（WSL2 と Docker）・UNIT 08（Topcoat 0.10 と CLI）・UNIT 09（Turso CLI と libSQL の確認）・UNIT 10（flyctl と環境の診断）は準備中で、次の更新から順に追加します。それまでの間、Rust のインストールは公式サイト（`https://www.rust-lang.org/`）の手順で行い、このシリーズの基準（Rust 1.98 以上、動作確認は 1.99.0）を満たしているかを `rustc --version` で確かめてください。

次の第2巻では、この巻でそろえた道具を使って、Rust の文法の土台を学びます。

## 用語ミニ辞典

この巻で出てきた主な用語です。

| 用語 | 読み・意味 | 出てくる単元 |
|---|---|---|
| シェル | 文字で打ったお願い（コマンド）を受け取って、プログラムを動かす係。Windows では PowerShell、Linux では bash | 巻頭・UNIT 01 |
| Windows PowerShell 5.1 | Windows に最初から入っている PowerShell。この本で使う版 | UNIT 01 |
| PowerShell 7 | 別にインストールする新しい PowerShell（コマンド名は `pwsh`）。文字コードの既定などが 5.1 と違う | UNIT 01 |
| Windows Terminal | シェルの窓を開くアプリ。スタートメニューで「ターミナル」と探す | UNIT 01 |
| CPU の種類（アーキテクチャ） | CPU の命令の体系。Windows の PC では x64 と ARM64 が主 | 巻頭・UNIT 01 |
| x64 / AMD64 | Intel や AMD の CPU の体系。Windows では `AMD64` と表示される | UNIT 01 |
| ARM64 | Snapdragon などの CPU の体系 | UNIT 01 |
| エミュレーション | ARM64 の PC で、x64 用のプログラムを通訳しながら動かす仕組み | UNIT 01 |
| ホームフォルダ | `C:\Users\（あなたの名前）`。PowerShell では `~` | UNIT 01 |
| 作業フォルダ | この本の練習をまとめて置くフォルダ。`~\rust-study` | UNIT 01 |
| OneDrive の同期フォルダ | 中のファイルをインターネット上に送り続けるフォルダ。作業フォルダには使わない | UNIT 01 |
| スクリプト | コマンドをファイルに並べて書いたもの。PowerShell では `.ps1` | UNIT 01 |
| ヒア文字列 | `@'` と `'@` ではさんだ、何行でもそのまま1つの文字列になる書き方 | UNIT 01 |
| 実行ポリシー | どんなスクリプトなら実行してよいかを決める PowerShell の設定。この本では `RemoteSigned` | UNIT 01 |
| BOM | ボム。ファイルの先頭の `EF BB BF` の印で、UTF-8 であることを知らせる | UNIT 01・03 |
| Shift_JIS | 日本語の Windows の古い文字コード。PowerShell 5.1 は BOM の無いスクリプトをこれで読む | UNIT 01 |
| MAX_PATH（パスの上限） | 多くの Windows のプログラムが扱えるパスの長さの上限。259 文字まで | UNIT 01 |
| リンカー | 部品をつないで実行ファイルを作る道具。Windows では `link.exe`（UNIT 05） | UNIT 01 |
| `rust-toolchain.toml` | そのフォルダの中で使う Rust の版を決めるファイル | UNIT 01・03 |
| Git | ファイルの変更の記録を残す道具 | UNIT 02 |
| リポジトリ | Git が記録をしまう場所。フォルダの中の `.git` | UNIT 02 |
| ステージ（インデックス） | 次のコミットに入れるものを並べておく台紙。`git add` で載せる | UNIT 02 |
| コミット | ある時点のファイルの状態を、メッセージ・作った人・日時と一緒に記録したもの | UNIT 02 |
| ブランチ | 変更の流れに付ける名前。このシリーズでは `main` | UNIT 02 |
| `.gitignore` | Git に入れないファイルの一覧 | UNIT 02 |
| `.gitattributes` | ファイルの種類ごとの扱い（改行コードなど）の決まり | UNIT 02 |
| 改行コード | 行の終わりを表す見えない文字。CR（`\r`・`0D`）と LF（`\n`・`0A`） | UNIT 02 |
| CRLF / LF | Windows の流儀の改行（CR+LF）と、Linux・macOS の流儀の改行（LF） | UNIT 02 |
| `core.autocrlf` | Git が改行を自動で変換する設定。Git for Windows の既定は `true` | UNIT 02 |
| `.env` / `.env.example` | 秘密の値を書く環境変数のファイルと、その見本。Git に入れるのは見本だけ | UNIT 02 |
| winget | ウィンゲット。Windows に入っている、コマンドでアプリを入れる道具 | UNIT 02 |
| VS Code | Visual Studio Code。この本で使うコードエディタ | UNIT 03 |
| 統合ターミナル | VS Code の中で開くターミナル。開いたフォルダから始まる | UNIT 03 |
| 拡張機能 | VS Code にあとから足す部品 | UNIT 03 |
| rust-analyzer | ラスト・アナライザー。Rust の補完・型の表示・チェックをする拡張機能 | UNIT 03 |
| コマンド パレット | VS Code の操作を名前で探して実行する入力欄。`Ctrl + Shift + P` | UNIT 03 |
| `settings.json` | VS Code の設定のファイル。ユーザーの設定とワークスペースの設定がある | UNIT 03 |
| ワークスペースの設定 | そのフォルダだけに効く設定。`.vscode/settings.json` | UNIT 03 |
| 制限モード | 信頼しなかったフォルダを開いたときの、拡張機能などを止めた状態 | UNIT 03 |
| 整形（フォーマット） | コードの字下げや空白を、決まった形に直すこと。Rust では rustfmt | UNIT 03 |
| clippy | クリッピー。Rust のよくある書き間違いや、もったいない書き方を指摘する道具 | UNIT 03 |

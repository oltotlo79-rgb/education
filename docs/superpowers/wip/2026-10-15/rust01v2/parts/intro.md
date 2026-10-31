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
- Topcoat 0.11 の CLI、Turso CLI、flyctl を入れ、版の表示と簡単な動作確認ができる

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
<text x="641" y="194" font-size="12" fill="#475569" text-anchor="middle">Rust 1.99.1 を</text>
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
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: 第1巻の学習地図。Windows の土台（シェル・Git・エディタ）から始めて、Rust の道具、Linux とクラウドの道具へ進む</div>
</div>

| 単元 | テーマ | ひとことで言うと |
|---|---|---|
| UNIT 01 | 作業フォルダと PowerShell | どのシェルで・どこにいて・どんな PC かを確かめ、作業場所を決める |
| UNIT 02 | Git・改行コード・秘密の除外 | 変更を記録する道具を入れ、改行の方言と秘密のファイルの扱いを最初に決める |
| UNIT 03 | VS Code と rust-analyzer | コードを書く机と、横で間違いを教えてくれる先輩を用意する |
| UNIT 04 | rustup・rustc・Cargo | Rust を入れて、どの版が使われているかを確かめる |
| UNIT 05 | MSVC Build Tools と CPU の種類 | Windows でビルドするためのリンカーを入れ、ARM64 と x64 の食い違いを見分ける |
| UNIT 06 | Cargo の品質コマンド | `check`・`fmt`・`clippy`・`test` を目的ごとに使い分ける |
| UNIT 07 | WSL2 と Docker | Windows の中で Linux を動かし、アプリを箱（コンテナ）に入れる |
| UNIT 08 | Topcoat 0.11 と CLI | Web フレームワークの道具を版を固定して入れ、開発サーバーを動かす |
| UNIT 09 | Turso CLI と libSQL の確認 | データベースの道具を WSL に入れ、Rust から libSQL につながるか確かめる |
| UNIT 10 | flyctl と環境の診断 | 公開の道具を入れ、ログイン・版・つながりを順番に確かめる |

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
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: この巻でそろえる道具の重なり。どの道具も、その下にあるシェルと OS・CPU の上で動き、上にあるインターネットのサービスとつながる</div>
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
| rust-analyzer（VS Code の拡張機能） | 0.3.3097 | UNIT 03〜 |
| Rust（rustup・rustc・cargo） | rustup 1.29.2。Rust は **1.99 以上**（本書シリーズの基準）、動作確認は 1.99.1 | UNIT 04〜 |
| Build Tools for Visual Studio | MSVC の C++ ビルドツール（ARM64 用）。版と選び方は UNIT 05 で説明します | UNIT 05〜 |
| WSL2 | Ubuntu 24.04.1 LTS | UNIT 07・09 |
| Topcoat / topcoat-cli | 0.11.0（`=0.11.0` で固定） | UNIT 08 |
| Turso CLI | v1.0.34（2026-10-31 の最新。WSL の中で使う。この巻では入れ方の説明だけ） | UNIT 09 |
| flyctl | v0.5.1（Windows 用は x64 版。ARM64 の PC ではエミュレーションで動く） | UNIT 10 |

> **ヒント:** Git for Windows と VS Code は、更新の早い道具です。2026-10-31 に `winget list` で確かめると、Git for Windows は 2.56.0.4、VS Code は 1.143.1 が出ていました。この巻の実行結果は、表の版で取ったものです。新しい版では、表示の細かい所が違うことがあります。

<!-- -->

> **注意:** このシリーズで使う Topcoat 0.11 は、**Rust 1.99 以上**でないとビルドできません。古い記事の手順で Rust を入れると、版が足りないことがあります。Rust の版の確かめ方と、プロジェクトごとに版を固定する方法は、UNIT 04 で説明します。

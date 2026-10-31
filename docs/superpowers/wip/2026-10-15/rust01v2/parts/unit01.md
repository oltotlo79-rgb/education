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
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: 作業を始める前に確かめる4つのこと。シェルの版・今いるフォルダ・OS・CPU の種類を、それぞれコマンドで確かめられる</div>
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
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: 作業フォルダの置き場所。短く、半角英数字だけで、同期されない場所に置くと、Cargo が作る長いパスのファイルも上限の 260 文字に収まる</div>
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
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: Windows PowerShell 5.1 がスクリプトを読むときの分かれ道。先頭に BOM（EF BB BF）があれば UTF-8、無ければ Shift_JIS として読む</div>
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
# このフォルダの中で cargo を実行すると、Rust 1.99.1 が使われる
# （既定のツールチェーンは変更しない）
[toolchain]
channel = "1.99.1"
components = ["rustfmt", "clippy"]
```

このファイルがあると、**RustNote のフォルダの中で実行したときだけ**、Rust 1.99.1 が使われます。この本の PC で、RustNote のフォルダとホームフォルダで、それぞれ「今使われる Rust の版」を表示すると、次のようになりました（Rust を入れるのは UNIT 04 です。ここでは結果だけ見てください）。

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
1.99.1-aarch64-pc-windows-msvc (overridden by 'C:\Users\...\rustnote\rust-toolchain.toml')
cargo 1.99.1 (5f94df478 2026-08-27)
stable-aarch64-pc-windows-msvc (default)
cargo 1.99.1 (5f94df478 2026-08-27)
```

- RustNote のフォルダでは、`rust-toolchain.toml` によって `1.99.1` が選ばれています（`overridden by` は「このファイルで上書きされています」という意味です）。
- ホームフォルダでは、この PC の既定の版（`stable`。`(default)` は「既定なので選ばれました」という意味）が使われました。今は stable も 1.99.1 なので、版の数字は同じですが、**選ばれた理由が違います**。stable は、新しい版が出たあとに `rustup update` をすると上がります（UNIT 04）。そのときも、RustNote のフォルダの中だけは 1.99.1 のままです。
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

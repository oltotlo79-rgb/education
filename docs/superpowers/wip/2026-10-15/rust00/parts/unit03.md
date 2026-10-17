## UNIT 03: 端末・シェル・コマンド

### この単元のゴール

- **端末（ターミナル）**・**シェル**・**コマンド**の違いを説明できる
- PowerShell で、今いる場所の確認・移動・フォルダ作成・一覧・表示・コピー・削除ができる
- コマンドを「コマンド名・サブコマンド・オプション・引数」の部品に分けて読める
- **コマンドライン引数**と**環境変数**を使って、プログラムに値を渡せる
- PowerShell と bash（WSL・macOS・Linux のシェル）の書き方の違いに気づける

### たとえ話でつかむ

コマンドを打つ作業は、**役所の窓口でお願いをする**ことに似ています。

- **端末（ターミナル）は窓口のカウンター**です。あなたが文字を書いて差し出し、返事の文字を受け取る場所です。Windows では「Windows Terminal（ウィンドウズ ターミナル）」というアプリがこの役目をします。
- **シェルは窓口の係員**です。あなたが書いたお願い（コマンド）を読み、「それなら `cargo` という担当に回しますね」と、OS に頼んでプログラムを起動します。Windows の **PowerShell**、Linux や macOS の **bash（バッシュ）** や **zsh（ズィーシェル）** がシェルです。
- **コマンドはお願いの文**です。決まった言い方で書かないと、係員に「そんな担当はいません」と言われてしまいます。
- PowerShell と bash は、**違う方言を話す係員**です。どちらも「フォルダの中身を見せて」というお願いを受け付けますが、細かい言い回しが違います。

マウスでアイコンをクリックする代わりに文字でお願いするのは、最初は不便に感じるかもしれません。しかし文字のお願いは、**そのままコピーして人に渡せる**、**同じ手順を何度でも正確にくり返せる**、**記録が残る**という大きな利点があります。開発の仕事でコマンドを使うのは、このためです。

> **用語:** **CLI（シーエルアイ）** … Command Line Interface の略。文字のコマンドで操作するやり方のことです。マウスで操作する画面は **GUI（ジーユーアイ）** と呼びます。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-09" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="60" width="110" height="90" rx="12" fill="#f8fafc" stroke="#475569" stroke-width="2"/>
<text x="69" y="100" font-size="15" font-weight="700" fill="#334155" text-anchor="middle">あなた</text>
<text x="69" y="122" font-size="12" fill="#64748b" text-anchor="middle">キーボード</text>
<rect x="166" y="40" width="150" height="130" rx="12" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="241" y="68" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">端末</text>
<text x="241" y="88" font-size="12" fill="#334155" text-anchor="middle">Windows Terminal</text>
<text x="241" y="116" font-size="12" fill="#334155" text-anchor="middle">文字を入力して</text>
<text x="241" y="134" font-size="12" fill="#334155" text-anchor="middle">結果を表示する窓</text>
<rect x="358" y="40" width="150" height="130" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="433" y="68" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">シェル</text>
<text x="433" y="88" font-size="12" fill="#334155" text-anchor="middle">PowerShell / bash</text>
<text x="433" y="116" font-size="12" fill="#334155" text-anchor="middle">コマンドを読んで</text>
<text x="433" y="134" font-size="12" fill="#334155" text-anchor="middle">担当を呼び出す</text>
<rect x="550" y="20" width="156" height="78" rx="12" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="628" y="50" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">プログラム</text>
<text x="628" y="72" font-size="12" fill="#334155" text-anchor="middle">cargo.exe など</text>
<rect x="550" y="112" width="156" height="78" rx="12" fill="#f8fafc" stroke="#475569" stroke-width="2"/>
<text x="628" y="142" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">OS</text>
<text x="628" y="164" font-size="12" fill="#334155" text-anchor="middle">プロセスを作る</text>
<line x1="126" y1="92" x2="162" y2="92" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-09)"/>
<line x1="318" y1="92" x2="354" y2="92" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-09)"/>
<line x1="510" y1="80" x2="546" y2="62" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-09)"/>
<line x1="510" y1="128" x2="546" y2="146" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-09)"/>
<line x1="354" y1="132" x2="318" y2="132" stroke="#94a3b8" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar-00-09)"/>
<line x1="162" y1="132" x2="126" y2="132" stroke="#94a3b8" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar-00-09)"/>
<text x="360" y="214" font-size="12" fill="#64748b" text-anchor="middle">実線 = お願い（コマンド）の流れ　／　点線 = 結果の文字が画面に戻る流れ</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図3-1: 端末・シェル・OS の関係。シェルが OS に頼んでプログラムを起動する</div>
</div>

実際の PowerShell の画面は次のようになります。`PS C:\Users\you>` の部分を**プロンプト**と呼び、「今いるフォルダはここで、お願いを待っています」という意味です。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="700" height="310" rx="10" fill="#0c1222" stroke="#334155" stroke-width="1.5"/>
<rect x="10" y="10" width="700" height="34" rx="10" fill="#1e293b"/>
<rect x="10" y="34" width="700" height="10" fill="#1e293b"/>
<rect x="22" y="16" width="200" height="28" rx="6" fill="#0c1222"/>
<text x="36" y="35" font-size="12" fill="#e2e8f0">Windows PowerShell</text>
<text x="690" y="33" font-size="13" fill="#94a3b8" text-anchor="end">－　□　×</text>
<g font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" font-size="13" xml:space="preserve" style="white-space: pre">
<text x="26" y="72" fill="#e2e8f0"><tspan fill="#facc15">PS C:\Users\you&gt;</tspan> cd rust-study\practice</text>
<text x="26" y="96" fill="#e2e8f0"><tspan fill="#facc15">PS C:\Users\you\rust-study\practice&gt;</tspan> ls src</text>
<text x="46" y="122" fill="#94a3b8">ディレクトリ: C:\Users\you\rust-study\practice\src</text>
<text x="26" y="148" fill="#e2e8f0">Mode            LastWriteTime   Length Name</text>
<text x="26" y="168" fill="#e2e8f0">----            -------------   ------ ----</text>
<text x="26" y="188" fill="#e2e8f0">-a----   2026/10/17     11:30      689 main.rs</text>
<text x="26" y="218" fill="#e2e8f0"><tspan fill="#facc15">PS C:\Users\you\rust-study\practice&gt;</tspan> cargo --version</text>
<text x="26" y="240" fill="#e2e8f0">cargo 1.99.0 (5f94df478 2026-08-27)</text>
<text x="26" y="266" fill="#e2e8f0"><tspan fill="#facc15">PS C:\Users\you\rust-study\practice&gt;</tspan> cargo run -q -- Hanako</text>
<text x="26" y="288" fill="#e2e8f0">こんにちは、Hanako さん</text>
<text x="26" y="310" fill="#e2e8f0"><tspan fill="#facc15">PS C:\Users\you\rust-study\practice&gt;</tspan> <tspan fill="#e2e8f0">_</tspan></text>
</g>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図3-2: PowerShell の画面イメージ。黄色がプロンプト（今いるフォルダ）、白が入力と結果</div>
</div>

コマンドは、決まった順番の部品でできています。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 210" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="20" width="680" height="56" rx="10" fill="#0c1222"/>
<text x="360" y="56" font-size="22" fill="#e2e8f0" text-anchor="middle" font-family="Cascadia Mono, Consolas, monospace"><tspan fill="#60a5fa">cargo</tspan> <tspan fill="#fb923c">run</tspan> <tspan fill="#34d399">--release</tspan> <tspan fill="#94a3b8">--</tspan> <tspan fill="#f472b6">Hanako</tspan></text>
<line x1="198" y1="80" x2="198" y2="104" stroke="#3b82f6" stroke-width="2"/>
<line x1="270" y1="80" x2="270" y2="104" stroke="#f97316" stroke-width="2"/>
<line x1="378" y1="80" x2="378" y2="104" stroke="#10b981" stroke-width="2"/>
<line x1="462" y1="80" x2="462" y2="104" stroke="#64748b" stroke-width="2"/>
<line x1="540" y1="80" x2="540" y2="104" stroke="#db2777" stroke-width="2"/>
<rect x="20" y="106" width="130" height="86" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="85" y="130" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">コマンド名</text>
<text x="85" y="152" font-size="12" fill="#334155" text-anchor="middle">どのプログラム</text>
<text x="85" y="170" font-size="12" fill="#334155" text-anchor="middle">に頼むか</text>
<rect x="160" y="106" width="130" height="86" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="225" y="130" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">サブコマンド</text>
<text x="225" y="152" font-size="12" fill="#334155" text-anchor="middle">その中の</text>
<text x="225" y="170" font-size="12" fill="#334155" text-anchor="middle">どの仕事か</text>
<rect x="300" y="106" width="130" height="86" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="365" y="130" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">オプション</text>
<text x="365" y="152" font-size="12" fill="#334155" text-anchor="middle">- や -- で始まる</text>
<text x="365" y="170" font-size="12" fill="#334155" text-anchor="middle">やり方の指定</text>
<rect x="440" y="106" width="110" height="86" rx="8" fill="#f8fafc" stroke="#64748b"/>
<text x="495" y="130" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">区切り</text>
<text x="495" y="152" font-size="12" fill="#334155" text-anchor="middle">ここから先は</text>
<text x="495" y="170" font-size="12" fill="#334155" text-anchor="middle">プログラムへ</text>
<rect x="560" y="106" width="140" height="86" rx="8" fill="#fdf2f8" stroke="#db2777"/>
<text x="630" y="130" font-size="13" font-weight="700" fill="#be185d" text-anchor="middle">引数</text>
<text x="630" y="152" font-size="12" fill="#334155" text-anchor="middle">プログラムに</text>
<text x="630" y="170" font-size="12" fill="#334155" text-anchor="middle">渡す値</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図3-3: コマンドの部品。「cargo に、release のやり方で run してもらい、Hanako を渡す」</div>
</div>

### まず動かす最小コード

この単元の「最小コード」は、Rust ではなく PowerShell のコマンドです。Windows のスタートメニューで「ターミナル」または「PowerShell」と検索して開き、1行ずつ打って Enter キーを押してください。

#### PowerShell の基本コマンドを打ってみる

> **▼ このコマンドがやること（先に日本語で）:** 今いるフォルダを確かめてから、練習用のフォルダ `shell-practice` を作って中に入ります。メモのファイルを作り、一覧で確かめ、中身を表示し、コピーして、コピーを消し、最後に元のフォルダへ戻ります。

```powershell
# 今いるフォルダを表示する
Get-Location
# shell-practice フォルダを作る
mkdir shell-practice
# その中へ移動する
cd shell-practice
# memo.txt に1行書き込む（文字コードは UTF-8）
Set-Content memo.txt "はじめての PowerShell" -Encoding utf8
# フォルダの中身を一覧する
ls
# ファイルの中身を表示する
cat memo.txt
# コピーを作って、一覧で確かめる
Copy-Item memo.txt memo-backup.txt
ls
# コピーを消す
Remove-Item memo-backup.txt
# 1つ上のフォルダへ戻る
cd ..
# Rust の道具の版を確かめる
cargo --version
rustc --version
```

**▼ 実行結果**（プロンプトとコマンドも含めて、画面に出たとおりに載せています）

```text
PS C:\Users\you> Get-Location

Path
----
C:\Users\you


PS C:\Users\you> mkdir shell-practice


    ディレクトリ: C:\Users\you


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d-----        2026/10/17     11:30                shell-practice


PS C:\Users\you> cd shell-practice
PS C:\Users\you\shell-practice> Set-Content memo.txt "はじめての PowerShell" -Encoding utf8
PS C:\Users\you\shell-practice> ls


    ディレクトリ: C:\Users\you\shell-practice


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----        2026/10/17     11:30             31 memo.txt


PS C:\Users\you\shell-practice> cat memo.txt
はじめての PowerShell
PS C:\Users\you\shell-practice> Copy-Item memo.txt memo-backup.txt
PS C:\Users\you\shell-practice> ls


    ディレクトリ: C:\Users\you\shell-practice


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----        2026/10/17     11:30             31 memo-backup.txt
-a----        2026/10/17     11:30             31 memo.txt


PS C:\Users\you\shell-practice> Remove-Item memo-backup.txt
PS C:\Users\you\shell-practice> cd ..
PS C:\Users\you> cargo --version
cargo 1.99.0 (5f94df478 2026-08-27)
PS C:\Users\you> rustc --version
rustc 1.99.0 (b940084d7 2026-09-28)
```

`cargo --version` と `rustc --version` は、Rust をインストールしていれば版が表示されます。まだの人は、この2行だけエラーになります（下の「よくあるエラー」を見てください）。

#### ▼ コマンドを1つずつ分解して解説

##### 解説1: 今いる場所を知る・移動する

```powershell
Get-Location
cd shell-practice
cd ..
```

- `Get-Location`（ゲット・ロケーション）は「今いるフォルダ」を表示します。プロンプトにも同じものが出ていますが、長くて省略されるときに便利です。短く `pwd` と打っても同じです。
- `cd`（change directory）は移動です。`cd shell-practice` は相対パス（UNIT 02）なので、「今いるフォルダの中の `shell-practice` へ」という意味です。
- `cd ..` は「1つ上のフォルダへ」です。

##### 解説2: 作る・見る

```powershell
mkdir shell-practice
Set-Content memo.txt "はじめての PowerShell" -Encoding utf8
ls
cat memo.txt
```

- `mkdir`（make directory）はフォルダを作ります。
- `Set-Content`（セット・コンテント）はファイルに文字を書き込みます。`-Encoding utf8` は「文字を UTF-8 という方式で保存して」というオプションです。付けないとどうなるかは「よくあるエラー」で見ます。
- `ls`（list）はフォルダの中身の一覧です。`Mode` 列の `d` はフォルダ（directory）、`a` は普通のファイルの印です。`Length` はファイルの大きさ（バイト数）です。
- `cat`（キャット）はファイルの中身を表示します。

##### 解説3: コピーする・消す

```powershell
Copy-Item memo.txt memo-backup.txt
Remove-Item memo-backup.txt
```

- `Copy-Item 元 先` でコピーします。
- `Remove-Item` で消します。**ごみ箱には入らず、すぐ消えます**。消すものを確かめてから打ちましょう。

> **ヒント:** コマンドを途中まで打って **Tab キー**を押すと、ファイル名やコマンド名を補完してくれます（`memo` まで打って Tab → `memo.txt`）。また **↑キー**で、前に打ったコマンドを呼び戻せます。打ち間違いが減るので、早めに慣れておきましょう。

##### 解説4: PowerShell と bash の対応表

PowerShell のコマンドには `Get-Location` のような正式名と、`pwd`・`ls`・`cat` のような短い別名（**エイリアス**）があります。短い別名は bash に似せてあるので、WSL や macOS で bash を使うときにも役立ちます。

| やりたいこと | PowerShell（正式名） | PowerShell（別名） | bash（WSL・macOS・Linux） |
|---|---|---|---|
| 今いるフォルダを表示 | `Get-Location` | `pwd` | `pwd` |
| 移動 | `Set-Location フォルダ` | `cd フォルダ` | `cd フォルダ` |
| 一覧 | `Get-ChildItem` | `ls`・`dir` | `ls` |
| フォルダを作る | `New-Item -ItemType Directory 名前` | `mkdir 名前` | `mkdir 名前` |
| 中身を表示 | `Get-Content ファイル` | `cat ファイル` | `cat ファイル` |
| コピー | `Copy-Item 元 先` | `cp 元 先` | `cp 元 先` |
| 削除 | `Remove-Item ファイル` | `rm ファイル` | `rm ファイル` |
| 環境変数を設定 | `$env:名前 = "値"` | ― | `export 名前=値` |
| 直前の終了コード | `$LASTEXITCODE` | ― | `$?` |

### もう一歩: 実務に近い形

コマンドからプログラムに値を渡す方法は、主に2つあります。

- **コマンドライン引数** … コマンドの後ろに並べる値（`cargo run -- Rust`）。そのとき限りの値を渡すのに使います。
- **環境変数** … シェルが覚えておいて、そこから起動したプログラムすべてに見せる「名前付きの値」。設定や秘密の鍵など、**コードに書き込みたくない値**を渡すのによく使います。

> **用語:** **環境変数** … OS やシェルが持っている「名前＝値」のメモ。たとえば `PATH` という環境変数には「コマンドのプログラムを探しに行くフォルダの一覧」が入っています。`cargo` と打つだけで動くのは、`PATH` に `C:\Users\you\.cargo\bin` が入っているからです。

#### 引数と環境変数を受け取る

> **▼ このコードがやること（先に日本語で）:** 起動するときに渡された値（引数）を、番号つきで全部表示します。そのあと、環境変数 `GREETING` の値を読み、設定されていなければ「こんにちは」を使って表示します。

```rust
use std::env;

fn main() {
    // コマンドラインから渡された値を順番に表示する（0 番目はプログラム自身）
    for (i, arg) in env::args().enumerate() {
        println!("引数 {i}: {arg}");
    }

    // 環境変数 GREETING を読む（設定されていなければ「こんにちは」を使う）
    let greeting = env::var("GREETING").unwrap_or(String::from("こんにちは"));
    println!("あいさつ: {greeting}");
}
```

PowerShell で、次の順に実行します。

```powershell
cargo run -q -- Rust "Web アプリ"
$env:GREETING = "おはよう"
cargo run -q
.\target\debug\practice.exe 1 2 3
Remove-Item Env:GREETING
cargo run -q
```

**▼ 実行結果**

```text
PS C:\Users\you\rust-study\practice> cargo run -q -- Rust "Web アプリ"
引数 0: target\debug\practice.exe
引数 1: Rust
引数 2: Web アプリ
あいさつ: こんにちは
PS C:\Users\you\rust-study\practice> $env:GREETING = "おはよう"
PS C:\Users\you\rust-study\practice> cargo run -q
引数 0: target\debug\practice.exe
あいさつ: おはよう
PS C:\Users\you\rust-study\practice> .\target\debug\practice.exe 1 2 3
引数 0: C:\Users\you\rust-study\practice\target\debug\practice.exe
引数 1: 1
引数 2: 2
引数 3: 3
あいさつ: おはよう
PS C:\Users\you\rust-study\practice> Remove-Item Env:GREETING
PS C:\Users\you\rust-study\practice> cargo run -q
引数 0: target\debug\practice.exe
あいさつ: こんにちは
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 引数を番号つきで取り出す

```rust
for (i, arg) in env::args().enumerate() {
    println!("引数 {i}: {arg}");
}
```

- `env::args()` は引数の並びです（UNIT 01 でも使いました）。
- `.enumerate()` は、並びの1つずつに `0, 1, 2, …` の番号を付けてくれます。`(i, arg)` で「番号」と「値」を同時に受け取っています。
- 0番目はプログラム自身のパスです。`cargo run` 経由だと `target\debug\practice.exe`、直接起動すると絶対パスになっています。どう起動されたかによって変わる、ということです。
- `"Web アプリ"` のように**空白を含む値は `"` で囲む**と、1つの引数として渡せます。囲まないと `Web` と `アプリ` の2つに分かれます。

##### 解説2: 環境変数を読む

```rust
let greeting = env::var("GREETING").unwrap_or(String::from("こんにちは"));
```

- `env::var("GREETING")` は環境変数 `GREETING` を読みます。設定されていないこともあるので、結果は成功か失敗のどちらかです。
- `.unwrap_or(既定値)` は「失敗だったら既定値を使う」という意味です。
- PowerShell で `$env:GREETING = "おはよう"` と設定すると、**その PowerShell の窓から起動したプログラム**にだけ見えます。窓を閉じると消え、他の窓には影響しません。
- `Remove-Item Env:GREETING` で設定を消せます。

> **補足（bash の場合）:** WSL や macOS の bash では、`GREETING=おはよう cargo run -q -- Rust` のように、コマンドの前に `名前=値` を書くと「そのコマンドにだけ」環境変数を渡せます。ずっと設定しておくなら `export GREETING=おはよう` です。

### よくあるエラーと読み方

#### コマンドの名前を打ち間違えた

```text
PS C:\Users\you> carg run
carg : 用語 'carg' は、コマンドレット、関数、スクリプト ファイル、または操作可能なプログラムの名前として認識されません。名前が正しく記述されていることを確認し、パスが含まれている場合は
そのパスが正しいことを確認してから、再試行してください。
発生場所 行:1 文字:1
+ carg run
+ ~~~~
    + CategoryInfo          : ObjectNotFound: (carg:String) [], CommandNotFoundException
    + FullyQualifiedErrorId : CommandNotFoundException
```

**何が起きたか:** シェル（係員）が `carg` という担当を探しましたが、見つかりませんでした。`発生場所 行:1 文字:1` と `~~~~` の印が、「1行目の1文字目からの `carg` の部分がおかしい」と指しています。

**直し方:** 綴りを直して `cargo run` と打ちます。綴りが正しいのに同じエラーになる場合は、**そのプログラムがインストールされていない**か、`PATH` にフォルダが入っていません。Rust をインストールした直後なら、**PowerShell の窓を開き直す**と直ることがよくあります（`PATH` の変更は、新しく開いた窓から有効になるためです）。

#### bash の書き方を PowerShell で打った

```text
PS C:\Users\you> export GREETING=hello
export : 用語 'export' は、コマンドレット、関数、スクリプト ファイル、または操作可能なプログラムの名前として認識されません。名前が正しく記述されていることを確認し、パスが含まれてい
る場合はそのパスが正しいことを確認してから、再試行してください。
発生場所 行:1 文字:1
+ export GREETING=hello
+ ~~~~~~
    + CategoryInfo          : ObjectNotFound: (export:String) [], CommandNotFoundException
    + FullyQualifiedErrorId : CommandNotFoundException
```

**何が起きたか:** `export` は bash の言葉なので、PowerShell の係員には通じません。インターネットの記事をコピーして貼ったときによく起きます。

**直し方:** PowerShell では `$env:GREETING = "hello"` と書きます。記事のコマンドを使うときは、**それが PowerShell 向けか bash 向けか**を最初に確かめましょう。この教材では、PowerShell 向けのコードブロックには `powershell`、bash 向けには `bash` の印を付けています。

#### `curl` が思った動きをしない

Web のやりとりを見る道具 `curl`（カール）は UNIT 05 で使いますが、Windows PowerShell 5.1 には落とし穴があります。

```text
PS C:\Users\you> Get-Command curl

CommandType     Name                                               Version    Source
-----------     ----                                               -------    ------
Alias           curl -> Invoke-WebRequest


PS C:\Users\you> Get-Command curl.exe

CommandType     Name                                               Version    Source
-----------     ----                                               -------    ------
Application     curl.exe                                           8.21.0.0   C:\Windows\system32\curl.exe
```

**何が起きたか:** PowerShell 5.1 では、`curl` は **`Invoke-WebRequest` という別のコマンドの別名**になっています。そのため、インターネットの記事どおりに `curl -I ...` などと打っても、オプションの意味が違って、エラーになったり入力待ちで止まったりします。

**直し方:** 本物の curl を使うときは、**`curl.exe` と拡張子まで書きます**。この教材ではいつも `curl.exe` と書きます。

#### PowerShell で作ったファイルを Rust で読めない

`-Encoding utf8` を付けずに `Set-Content` で作ったファイルを、Rust で読んでみます。

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    // memo.txt を UTF-8 の文字列として読み込む
    let text = fs::read_to_string("memo.txt")?;
    println!("memo.txt の中身: {}", text.trim_end());
    Ok(())
}
```

```powershell
Set-Content memo.txt "はじめての PowerShell"
cargo run
```

**▼ 実行結果**

```text
Error: Error { kind: InvalidData, message: "stream did not contain valid UTF-8" }
error: process didn't exit successfully: `target\debug\practice.exe` (exit code: 1)
```

**何が起きたか:** 文字をファイルに保存するときの「文字と数値の対応表」を**文字コード**と呼びます。Windows PowerShell 5.1 の `Set-Content` は、何も指定しないと日本語を **Shift_JIS（シフトジス）** という古い文字コードで保存します。一方 Rust の `read_to_string` は **UTF-8（ユーティーエフエイト）** しか受け付けないので、「中身が正しい UTF-8 ではない（InvalidData）」と失敗しました。

**直し方:** `Set-Content` に `-Encoding utf8` を付けるか、ファイルは VS Code などのエディタ（既定で UTF-8）で作ります。

> **注意:** PowerShell 5.1 の `-Encoding utf8` は、ファイルの先頭に **BOM（ボム）** という目に見えない印を付けます。Rust で読むと、その印が文字列の先頭に1文字ぶん残ります。画面上は見えませんが、「先頭の文字を比べたら一致しない」といった不思議な不具合の原因になることがあります。迷ったら、ファイルはエディタで UTF-8（BOM なし）として作るのがいちばん確実です。

### RustNote ではここで使う

- RustNote を作る間、`cargo`（ビルド・実行・テスト）、`git`（変更の記録）、`turso`（データベースの操作）、`fly`（公開）といったコマンドを使います。どれも「コマンド名・サブコマンド・オプション・引数」の形です（第1巻でインストールします）。
- Topcoat で作ったサーバーは、環境変数 `HOST` と `PORT` で待ち受ける場所を決めます。手元では何も設定せずに `127.0.0.1:3000`、Fly.io の上では `HOST=0.0.0.0`・`PORT=8080` を設定します（UNIT 06・第12巻）。
- データベースの場所やパスワード類も、コードに直接書かず**環境変数**で渡します。コードを公開しても秘密が漏れないようにするためです（第4巻・第9巻）。

### 練習問題

**問題1:** `practice` フォルダにいるとき、`src` フォルダの中身を一覧するコマンドは何でしょうか。また、今いるフォルダを確かめるコマンドを2通り答えてください。

**問題2:** 「もう一歩」のプログラムで「あいさつ: おはよう」と表示させる手順を、PowerShell と bash の両方で書いてください。

**問題3:** 引数が何個渡されたか（プログラム自身は数えない）を表示するプログラムを書いてください。`cargo run -q -- a "b c" d` で「渡された値は 3 個です」と表示されれば正解です。

### 解答と解説

**問題1の解答:** 一覧は `ls src`（正式名なら `Get-ChildItem src`）です。今いるフォルダの確認は `Get-Location` と `pwd` の2通りです。

```text
PS C:\Users\you\rust-study\practice> ls src


    ディレクトリ: C:\Users\you\rust-study\practice\src


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----        2026/10/15     20:47            689 main.rs
```

**問題2の解答:**

```powershell
# PowerShell
$env:GREETING = "おはよう"
cargo run -q
```

```bash
# bash（WSL・macOS・Linux）
GREETING=おはよう cargo run -q
```

bash で実行した結果です（引数に `Rust` も渡しています）。

```text
引数 0: target\debug\practice.exe
引数 1: Rust
あいさつ: おはよう
```

**問題3の解答:**

```rust
use std::env;

fn main() {
    // 0 番目（プログラム自身）を skip(1) で飛ばしてから数える
    let count = env::args().skip(1).count();
    println!("渡された値は {count} 個です");
}
```

```powershell
cargo run -q -- a "b c" d
```

**▼ 実行結果**

```text
渡された値は 3 個です
```

`"b c"` は `"` で囲んでいるので1個として数えられます。`skip(1)` は「先頭から1個飛ばす」、`count()` は「残りの個数を数える」という意味です。

### まとめ

- 端末は文字の窓、シェルはコマンドを読んで OS にプログラムの起動を頼む係員、コマンドは「名前・サブコマンド・オプション・引数」の形のお願いです。
- プログラムへは**コマンドライン引数**（そのときの値）と**環境変数**（設定・秘密）で値を渡せます。
- PowerShell と bash は方言が違います。記事のコマンドがどちら向けかを確かめ、Windows では `curl.exe` と書き、ファイルは UTF-8 で作りましょう。


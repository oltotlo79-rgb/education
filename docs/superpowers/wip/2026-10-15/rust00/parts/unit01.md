## UNIT 01: コンピュータ・OS・アプリ

### この単元のゴール

- **CPU・メモリ・ストレージ**の役割を、台所のたとえで説明できる
- **OS（オーエス）**が何をしているか、アプリとの関係を説明できる
- 動いているプログラムを **プロセス** と呼ぶことと、プロセスが外へ結果を伝える3つの出口（**標準出力・標準エラー出力・終了コード**）を区別できる
- RustNote のサーバーも「OS の上で動く1つのプロセス」だと分かる

### たとえ話でつかむ

コンピュータの中は、**レストランの厨房（ちゅうぼう）**にとてもよく似ています。

- **CPU（シーピーユー）は料理人**です。レシピに書かれた手順を、1つずつ、ものすごい速さでこなします。1秒間に何十億回もの計算ができますが、レシピに書いていないことは一切しません。
- **メモリは調理台**です。今まさに使っている材料や、切りかけの野菜を置いておく場所です。料理人の手がすぐ届くので速く使えますが、広さには限りがあり、**閉店（電源オフ）すると片付けられて何も残りません**。
- **ストレージ（SSD など）は冷蔵庫や食品庫**です。たくさんしまっておけて、**閉店しても中身は残ります**。ただし、取りに行くのは調理台より時間がかかります。
- **OS は店長**です。どの料理人がいつどのコンロを使うか、調理台のどこを誰が使うかを決め、他の人の皿に勝手に触らないように見張っています。Windows・macOS・Linux はどれも OS です。
- **アプリ（プログラム）はレシピ**で、**プロセスはそのレシピで「今まさに作っている最中の1皿」**です。同じレシピ（同じプログラム）から、同時に2皿（2つのプロセス）を作ることもできます。

> **用語:** **CPU** … Central Processing Unit（中央処理装置）の略。計算と命令の実行を担当する部品です。
>
> **用語:** **プロセス** … 実行中のプログラムのことです。ファイルとして保存されている「プログラム」は動いていない状態、OS が読み込んで動かし始めたものが「プロセス」です。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-05" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="180" y="26" font-size="15" font-weight="700" fill="#c2410c" text-anchor="middle">レストランの厨房</text>
<text x="540" y="26" font-size="15" font-weight="700" fill="#1e40af" text-anchor="middle">コンピュータ</text>
<rect x="40" y="40" width="280" height="50" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="180" y="62" font-size="14" font-weight="700" fill="#9a3412" text-anchor="middle">店長</text>
<text x="180" y="80" font-size="12" fill="#475569" text-anchor="middle">誰が何をいつ使うか決める</text>
<rect x="400" y="40" width="280" height="50" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="540" y="62" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">OS（Windows など）</text>
<text x="540" y="80" font-size="12" fill="#475569" text-anchor="middle">CPU・メモリ・ファイルを割り振る</text>
<rect x="40" y="104" width="280" height="50" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="180" y="126" font-size="14" font-weight="700" fill="#9a3412" text-anchor="middle">料理人</text>
<text x="180" y="144" font-size="12" fill="#475569" text-anchor="middle">レシピの手順を高速にこなす</text>
<rect x="400" y="104" width="280" height="50" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="540" y="126" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">CPU</text>
<text x="540" y="144" font-size="12" fill="#475569" text-anchor="middle">命令を1つずつ実行する</text>
<rect x="40" y="168" width="280" height="50" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="180" y="190" font-size="14" font-weight="700" fill="#9a3412" text-anchor="middle">調理台</text>
<text x="180" y="208" font-size="12" fill="#475569" text-anchor="middle">速いが狭い・閉店で片付く</text>
<rect x="400" y="168" width="280" height="50" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="540" y="190" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">メモリ（例: 16GB）</text>
<text x="540" y="208" font-size="12" fill="#475569" text-anchor="middle">電源を切ると中身が消える</text>
<rect x="40" y="232" width="280" height="50" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="180" y="254" font-size="14" font-weight="700" fill="#9a3412" text-anchor="middle">冷蔵庫・食品庫</text>
<text x="180" y="272" font-size="12" fill="#475569" text-anchor="middle">たくさん入る・閉店しても残る</text>
<rect x="400" y="232" width="280" height="50" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="540" y="254" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">ストレージ（SSD・例: 512GB）</text>
<text x="540" y="272" font-size="12" fill="#475569" text-anchor="middle">電源を切っても残る</text>
<line x1="324" y1="65" x2="396" y2="65" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-05)"/>
<line x1="324" y1="129" x2="396" y2="129" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-05)"/>
<line x1="324" y1="193" x2="396" y2="193" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-05)"/>
<line x1="324" y1="257" x2="396" y2="257" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-05)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-1: 厨房とコンピュータの対応。左のたとえが、右の部品に当たる</div>
</div>

次の図は、コンピュータを**層（レイヤー）**で見たものです。いちばん下に機械（ハードウェア）があり、その上に OS、さらにその上でたくさんのプロセスが動いています。プロセスは機械を直接さわらず、**必ず OS にお願いして**メモリやファイルやネットワークを使います。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-06" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#c2410c"/></marker>
</defs>
<text x="20" y="28" font-size="13" font-weight="700" fill="#334155">アプリ（プロセス）の層</text>
<rect x="20" y="38" width="124" height="56" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="82" y="62" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">ブラウザ</text>
<text x="82" y="82" font-size="12" fill="#64748b" text-anchor="middle">PID 5120</text>
<rect x="158" y="38" width="124" height="56" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="220" y="62" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">VS Code</text>
<text x="220" y="82" font-size="12" fill="#64748b" text-anchor="middle">PID 7344</text>
<rect x="296" y="38" width="124" height="56" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="358" y="62" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">PowerShell</text>
<text x="358" y="82" font-size="12" fill="#64748b" text-anchor="middle">PID 9012</text>
<rect x="434" y="38" width="124" height="56" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="496" y="62" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">practice.exe</text>
<text x="496" y="82" font-size="12" fill="#64748b" text-anchor="middle">PID 24076</text>
<rect x="572" y="38" width="128" height="56" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="636" y="62" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">RustNote</text>
<text x="636" y="82" font-size="12" fill="#64748b" text-anchor="middle">（サーバー）</text>
<line x1="496" y1="98" x2="496" y2="140" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-06)"/>
<text x="508" y="116" font-size="12" fill="#c2410c">「ファイルを開きたい」</text>
<text x="508" y="132" font-size="12" fill="#c2410c">「メモリを少しください」</text>
<rect x="20" y="146" width="680" height="56" rx="10" fill="#eff6ff" stroke="#1e40af" stroke-width="2"/>
<text x="360" y="170" font-size="15" font-weight="700" fill="#1e40af" text-anchor="middle">OS（Windows / Linux / macOS）</text>
<text x="360" y="190" font-size="12" fill="#334155" text-anchor="middle">プロセスに番号（PID）を付け、CPU の時間・メモリ・ファイル・ネットワークを貸し出す</text>
<rect x="20" y="222" width="160" height="56" rx="8" fill="#f8fafc" stroke="#475569"/>
<text x="100" y="255" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">CPU</text>
<rect x="194" y="222" width="160" height="56" rx="8" fill="#f8fafc" stroke="#475569"/>
<text x="274" y="255" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">メモリ</text>
<rect x="368" y="222" width="160" height="56" rx="8" fill="#f8fafc" stroke="#475569"/>
<text x="448" y="255" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">ストレージ</text>
<rect x="542" y="222" width="158" height="56" rx="8" fill="#f8fafc" stroke="#475569"/>
<text x="621" y="255" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">ネットワーク</text>
<text x="20" y="296" font-size="12" fill="#64748b">ハードウェア（機械そのもの）の層</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-2: コンピュータの層。プロセスは OS を通してだけ機械を使う（PID の数字は例）</div>
</div>

### まず動かす最小コード

最初のプログラムでは、「自分は OS の上で動く1つのプロセスである」ということを、プログラム自身に確かめさせます。

#### 自分が動いている環境とプロセス番号を表示する

> **▼ このコードがやること（先に日本語で）:** このプログラムが動いている OS の種類と CPU の種類、そして OS がこのプロセスに付けた番号（プロセス ID）を表示します。最後に CPU に1から100までの足し算をさせ、その結果も表示します。

```rust
use std::env;
use std::process;

fn main() {
    // このプログラムが動いている OS の種類（windows / linux / macos など）
    let os = env::consts::OS;
    // CPU の種類（x86_64 / aarch64 など）
    let arch = env::consts::ARCH;
    // OS がこのプロセスに付けた番号（プロセス ID）
    let pid = process::id();

    println!("OS の種類: {os}");
    println!("CPU の種類: {arch}");
    println!("プロセス ID: {pid}");

    // CPU に足し算を 100 回してもらい、途中の値はメモリ上の total に置く
    let mut total = 0;
    for n in 1..=100 {
        total += n;
    }
    println!("1 から 100 までの合計: {total}");
}
```

練習用プロジェクトの `src\main.rs` をこの内容に置き換えて保存し、実行します。

```powershell
cargo run
```

**▼ 実行結果**（2回実行した結果です）

```text
OS の種類: windows
CPU の種類: aarch64
プロセス ID: 24076
1 から 100 までの合計: 5050
```

```text
OS の種類: windows
CPU の種類: aarch64
プロセス ID: 49372
1 から 100 までの合計: 5050
```

同じプログラムなのに、**プロセス ID だけが毎回変わっている**ことに注目してください。プログラムを起動するたびに、OS が新しいプロセスを作り、新しい番号を付けているからです。`aarch64`（エーアーチ64）は ARM 系の CPU の名前です。一般的な Intel や AMD の CPU の PC なら `x86_64` と表示されます。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 道具箱から道具を取り出す（`use`）

```rust
use std::env;
use std::process;
```

- `std`（エスティーディー、standard の略）は、Rust に最初から付いてくる道具箱で、**標準ライブラリ**と呼びます。
- `use std::env;` は「`std` の中の `env`（環境に関する道具）を、短く `env` と書いて使えるようにする」という宣言です。
- 同じように `process`（プロセスに関する道具）も使えるようにしています。

> **用語:** **ライブラリ** … 誰かが作ってくれた便利な部品（関数など）の集まりです。自分で一から作らなくても、取り出して使えます。

##### 解説2: プログラムの入り口 `main`

```rust
fn main() {
    // ...
}
```

- `fn` は function（関数）の略で、「ここから処理のまとまりを書きます」という印です。
- `main`（メイン）という名前の関数は特別で、**プログラムを起動すると最初にここが実行されます**。
- `{` から `}` までが、この関数の中身です。

##### 解説3: 値に名前を付けて表示する

```rust
let os = env::consts::OS;
let pid = process::id();
println!("OS の種類: {os}");
```

- `let`（レット）は「値に名前を付けて、メモリに置いておく」という意味です。この名前付きの入れ物を **変数** と呼びます。
- `env::consts::OS` は、Rust が教えてくれる「この OS の種類」という文字です。
- `process::id()` は、**OS にこのプロセスの番号を問い合わせる**関数です。`()` は「関数を呼び出す」印です。
- `println!`（プリントライン）は、文字を画面に1行表示します。`"..."` の中の `{os}` の部分に、変数 `os` の中身が埋め込まれます。

##### 解説4: CPU に計算させる

```rust
let mut total = 0;
for n in 1..=100 {
    total += n;
}
```

- `let mut` の `mut`（ミュート、mutable の略）は「あとで中身を書き換えてよい」という印です。Rust の変数は、`mut` を付けないと書き換えられません（UNIT 09 で、付け忘れたときのエラーを見ます）。
- `for n in 1..=100` は「`n` を 1, 2, 3, …, 100 と変えながら、`{ }` の中を100回くり返す」という意味です。`1..=100` は「1以上100以下」を表します。
- `total += n;` は「`total` に `n` を足して、`total` を書き換える」という意味です。
- この100回の足し算を実行しているのが CPU、途中の `total` の値を置いているのがメモリです。

### もう一歩: 実務に近い形

プロセスは、外の世界に結果を伝える**出口を3つ**持っています。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-07" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="60" width="200" height="110" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="120" y="102" font-size="15" font-weight="700" fill="#c2410c" text-anchor="middle">プロセス</text>
<text x="120" y="126" font-size="12" fill="#475569" text-anchor="middle">practice.exe</text>
<text x="120" y="146" font-size="12" fill="#475569" text-anchor="middle">（実行中のプログラム）</text>
<line x1="222" y1="80" x2="372" y2="40" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-07)"/>
<line x1="222" y1="115" x2="372" y2="115" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-07)"/>
<line x1="222" y1="150" x2="372" y2="190" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-07)"/>
<rect x="378" y="16" width="322" height="50" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="392" y="38" font-size="14" font-weight="700" fill="#1e40af">① 標準出力（stdout）</text>
<text x="392" y="56" font-size="12" fill="#334155">ふつうの結果。println! で書く</text>
<rect x="378" y="90" width="322" height="50" rx="8" fill="#fef2f2" stroke="#dc2626"/>
<text x="392" y="112" font-size="14" font-weight="700" fill="#b91c1c">② 標準エラー出力（stderr）</text>
<text x="392" y="130" font-size="12" fill="#334155">エラーや警告の知らせ。eprintln! で書く</text>
<rect x="378" y="164" width="322" height="50" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="392" y="186" font-size="14" font-weight="700" fill="#047857">③ 終了コード（exit code）</text>
<text x="392" y="204" font-size="12" fill="#334155">終わるときに OS へ渡す数。0 = 成功</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-3: プロセスの3つの出口。画面に出る文字は①と②、成功・失敗の合図は③</div>
</div>

#### 成功と失敗を、出口を使い分けて伝える

> **▼ このコードがやること（先に日本語で）:** 起動するときに名前を1つ受け取り、「こんにちは、○○ さん」と表示します。名前が渡されなかったときは、エラーのメッセージを「エラー用の出口」に書き、**失敗の合図として終了コード 2** を OS に返して終わります。

```rust
use std::env;
use std::process;

fn main() {
    // コマンドラインで渡された 1 つ目の値を取り出す（無ければ None）
    let name = env::args().nth(1);

    match name {
        Some(name) => {
            // 標準出力へ書く（ふつうの結果）
            println!("こんにちは、{name} さん");
        }
        None => {
            // 標準エラー出力へ書く（エラーの知らせ）
            eprintln!("エラー: 名前を 1 つ渡してください（例: cargo run -- Hanako）");
            // 終了コード 2 でプロセスを終える（0 以外は「失敗」の合図）
            process::exit(2);
        }
    }
}
```

`cargo build` でコンパイルだけ行い、できあがった実行ファイル `practice.exe` を PowerShell から直接動かします。`$LASTEXITCODE` は「直前に動かしたプログラムの終了コード」を表示する PowerShell の変数です。

```powershell
cargo build
.\target\debug\practice.exe Hanako
$LASTEXITCODE
.\target\debug\practice.exe
$LASTEXITCODE
```

**▼ 実行結果**

```text
こんにちは、Hanako さん
0
エラー: 名前を 1 つ渡してください（例: cargo run -- Hanako）
2
```

さらに、`>` を使って「標準出力だけをファイルへ送る」と、2つの出口が別物であることがはっきり分かります。

```powershell
.\target\debug\practice.exe > out.txt
Get-Content out.txt
```

**▼ 実行結果**

```text
エラー: 名前を 1 つ渡してください（例: cargo run -- Hanako）
```

エラーのメッセージは画面に出ますが、`out.txt` の中身は空っぽです（`Get-Content` は何も表示しません）。`>` が横取りしたのは①の標準出力だけで、②の標準エラー出力はそのまま画面に流れたからです。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 起動時に渡された値を受け取る

```rust
let name = env::args().nth(1);
```

- `env::args()` は、プログラムを起動したときに渡された値（**コマンドライン引数**）の並びです。0番目にはプログラム自身の名前が入り、1番目から先が渡した値です。
- `.nth(1)` は「1番目を取り出す」という意味です。
- 値が渡されていないこともあるので、結果は「ある（`Some(値)`）」か「ない（`None`）」のどちらかになります。この「あるかもしれないし、ないかもしれない」を表す型を `Option`（オプション）と呼びます（第2巻で詳しく学びます）。

##### 解説2: 場合分けする `match`

```rust
match name {
    Some(name) => { /* 値があったとき */ }
    None => { /* 値がなかったとき */ }
}
```

- `match`（マッチ）は、値の形に応じて処理を分ける構文です。
- `Some(name)` の場合は、中身を `name` という名前で取り出して使えます。
- Rust では `Some` と `None` の**両方の場合を必ず書かないとコンパイルが通りません**。「値がなかったとき」を書き忘れる事故を、コンパイラが防いでくれるわけです。

##### 解説3: エラー用の出口と終了コード

```rust
eprintln!("エラー: 名前を 1 つ渡してください（例: cargo run -- Hanako）");
process::exit(2);
```

- `eprintln!`（イープリントライン）は `println!` のエラー版で、**標準エラー出力**に書きます。
- `process::exit(2)` は、終了コード 2 を OS に渡して、その場でプロセスを終わらせます。
- 終了コードは「0 なら成功、0 以外なら失敗」という世界共通の約束です。どの数を使うかはプログラムが決めます。

| 終了コード | よくある意味 |
|---|---|
| `0` | 成功 |
| `1` | 一般的な失敗（Rust で `main` がエラーを返したときもこれ） |
| `2` | 使い方の間違い（引数が足りない など）によく使われる |
| `101` | Rust のプログラムが**パニック**（次の節で説明）して止まった |

> **ヒント:** `cargo run -- Hanako` のように、`--` の後ろに書いた値がプログラムへの引数になります。`--` より前は Cargo への指示、後ろはプログラムへの値、という区切りです。

### よくあるエラーと読み方

#### 実行中に止まる「パニック」

存在しない場所のデータを読もうとすると、Rust のプログラムは**パニック**して止まります。

```rust
fn main() {
    // 3 人分の点数をメモリに並べる
    let scores = vec![80, 95, 70];
    // 11 人目（番号 10）を取り出そうとする
    let index = 10;
    println!("{}", scores[index]);
}
```

**▼ 実行結果**（`cargo run` の出力をそのまま載せています）

```text
   Compiling practice v0.1.0 (C:\Users\you\rust-study\practice)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target\debug\practice.exe`

thread 'main' (50284) panicked at src\main.rs:6:26:
index out of bounds: the len is 3 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\practice.exe` (exit code: 101)
```

**何が起きたか:** 1行ずつ読むと、次のことが書いてあります。

| 出力の部分 | 意味 |
|---|---|
| `thread 'main' (50284) panicked` | メインの処理（スレッド）がパニックした。括弧の数字はスレッドの番号 |
| `at src\main.rs:6:26` | 場所は `src\main.rs` の **6行目・26文字目** |
| `index out of bounds: the len is 3 but the index is 10` | 「範囲外の番号。長さは 3 なのに、番号 10 を指定した」 |
| `note: run with RUST_BACKTRACE=1 ...` | 「もっと詳しい経路を見たければ、この設定で実行して」という補足 |
| `(exit code: 101)` | Cargo からの報告。プロセスは終了コード 101（パニック）で終わった |

**直し方:** 番号は 0 から数えるので、3人分なら使える番号は `0`・`1`・`2` だけです。`let index = 2;` のように範囲内の番号にすれば動きます。

> **つまずきポイント:** パニックは**コンパイルは通ったのに、動かしてみて初めて起きる失敗**です（UNIT 04 で「コンパイル時の失敗」と「実行時の失敗」を区別します）。Rust は、範囲外の場所をこっそり読んでデタラメな値を返す代わりに、**その場で止めて場所を教えてくれます**。止まるのは不親切に見えますが、間違った値のまま処理が進むより、ずっと安全です。

### RustNote ではここで使う

- RustNote のサーバーも、Fly.io というクラウドの PC の上で動く**1つのプロセス**です。違いは、`practice.exe` がすぐ終わるのに対して、サーバーは**止めるまでずっと動き続ける**ことです。
- サーバーの動作記録（**ログ**）は、標準出力・標準エラー出力に書きます。Fly.io はそれを集めて、`fly logs` というコマンドで見られるようにしてくれます（第12巻）。
- 起動に必要な設定が足りないときは、RustNote は分かりやすいメッセージを出して**0 以外の終了コード**で止まります。「どこかおかしいまま動き続ける」より「すぐ止まって理由を言う」ほうが、原因を早く見つけられるからです。

### 練習問題

**問題1（予想してから実行）:** 最小コードの `for n in 1..=100` を `for n in 1..=10` に変えると、最後の行には何と表示されるでしょうか。実行する前に予想してから、確かめてください。

**問題2:** 「もう一歩」のコードで、名前が無いときの終了コードを `2` から `7` に変えてください。PowerShell で `$LASTEXITCODE` を表示して、7 になることを確かめましょう。

**問題3:** 次の a〜d は、厨房のたとえでいうと「料理人（CPU）」「調理台（メモリ）」「冷蔵庫（ストレージ）」のどれに当たるでしょうか。

- a. RustNote のサーバーが今まさに処理している、送られてきたノートのタイトル
- b. データベースに保存されたノート
- c. タイトルが 1〜80 文字かどうかを数えて判定すること
- d. `cargo build` で作られた `practice.exe` というファイル

### 解答と解説

**問題1の解答:** `1 から 100 までの合計: 55` と表示されます（文の「100」は `println!` の中の文字なので変わりません）。くり返しが1〜10になるので、1+2+…+10 = 55 です。文字の部分も直すなら、次のようにします。

```rust
fn main() {
    let mut total = 0;
    for n in 1..=10 {
        total += n;
    }
    println!("1 から 10 までの合計: {total}");
}
```

**▼ 実行結果**

```text
1 から 10 までの合計: 55
```

**問題2の解答:** `process::exit(2);` を `process::exit(7);` に書き換えて、`cargo build` してから実行します。

```powershell
cargo build
.\target\debug\practice.exe
$LASTEXITCODE
```

**▼ 実行結果**

```text
エラー: 名前を 1 つ渡してください（例: cargo run -- Hanako）
7
```

ポイントは、`cargo build` を**し直すこと**です。ソースコードを書き換えても、コンパイルし直さなければ `practice.exe` は古いままです（UNIT 04 で詳しく見ます）。

**問題3の解答:**

- a. **メモリ**（処理中の値は調理台の上にある）
- b. **ストレージ**（データベースの中身は、最終的にストレージ上のファイルに保存される）
- c. **CPU**（数えて比べるのは計算＝料理人の仕事）
- d. **ストレージ**（ファイルはストレージに保存される。実行すると、OS がその中身をメモリに読み込んでプロセスにする）

### まとめ

- CPU は計算する料理人、メモリは速いが消える調理台、ストレージは遅いが残る冷蔵庫、OS はそれらを割り振る店長です。
- 動いているプログラムを**プロセス**と呼び、OS が番号（PID）を付けて管理します。
- プロセスは**標準出力・標準エラー出力・終了コード**の3つの出口で結果を伝えます。0 は成功、Rust のパニックは 101 です。


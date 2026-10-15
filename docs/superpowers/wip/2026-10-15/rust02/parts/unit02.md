## UNIT 02: 整数・浮動小数・bool・charと変換

### この単元のゴール

- 整数の型（`i32` `u8` `u64` `usize` など）を「符号の有無」と「ビット幅」で読み分けられる
- 計算が型の範囲からはみ出す**オーバーフロー**の挙動を知り、`checked_add` などで安全に扱える
- 小数（`f64`）に誤差があること、真偽値（`bool`）と文字（`char`）の性質を説明できる
- `as` による強制変換と、`From` / `TryFrom` による安全な変換を使い分けられる

### たとえ話でつかむ

整数の型は「**桁数の決まったメーター**」です。

- 車の走行距離計が `999999` の次に `000000` へ戻ってしまうように、決まった桁数のメーターには**表せる最大の数**があります。Rust の `u8` は「0〜255 まで数えられるメーター」です。
- 型の名前の `u` は unsigned（アンサインド：符号なし ＝ マイナスなし）、`i` は integer（インテジャー：整数。マイナスもあり）、数字の `8` `32` `64` はメーターの大きさ（**ビット数**）です。
- 小数（`f64`）は「**定規の目盛り**」に似ています。目盛りの間にある値はいちばん近い目盛りに寄せて記録されるので、ほんのわずかなズレ（誤差）が出ます。
- 型の変換は「**引っ越し**」です。小さな部屋から大きな部屋へは必ず全部運べます（`From`）。大きな部屋から小さな部屋へは、入りきらないかもしれないので「入るか確かめてから運ぶ」（`TryFrom`）のが安全です。`as` は「入りきらない分は捨てて運ぶ」乱暴な引っ越しです。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="360" y="24" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">整数の型: ビット幅が大きいほど、表せる範囲が広い</text>
<text x="60" y="54" text-anchor="middle" font-size="12" fill="#64748b">型</text>
<text x="200" y="54" text-anchor="middle" font-size="12" fill="#64748b">ビット幅（長さ）</text>
<text x="480" y="54" text-anchor="middle" font-size="12" fill="#64748b">i（マイナスあり）</text>
<text x="640" y="54" text-anchor="middle" font-size="12" fill="#64748b">u（0以上）</text>
<rect x="20" y="66" width="80" height="30" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="60" y="86" text-anchor="middle" font-size="13" font-weight="700" fill="#1e40af">i8 / u8</text>
<rect x="110" y="72" width="12.5" height="18" fill="#3b82f6"/>
<text x="130" y="86" font-size="12" fill="#475569">8</text>
<text x="480" y="86" text-anchor="middle" font-size="13" fill="#0f172a">-128 〜 127</text>
<text x="640" y="86" text-anchor="middle" font-size="13" fill="#0f172a">0 〜 255</text>
<rect x="20" y="106" width="80" height="30" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="60" y="126" text-anchor="middle" font-size="13" font-weight="700" fill="#1e40af">i16 / u16</text>
<rect x="110" y="112" width="25" height="18" fill="#3b82f6"/>
<text x="143" y="126" font-size="12" fill="#475569">16</text>
<text x="480" y="126" text-anchor="middle" font-size="13" fill="#0f172a">-32,768 〜 32,767</text>
<text x="640" y="126" text-anchor="middle" font-size="13" fill="#0f172a">0 〜 65,535</text>
<rect x="20" y="146" width="80" height="30" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="60" y="166" text-anchor="middle" font-size="13" font-weight="700" fill="#c2410c">i32 / u32</text>
<rect x="110" y="152" width="50" height="18" fill="#f97316"/>
<text x="168" y="166" font-size="12" fill="#475569">32（i32 が既定）</text>
<text x="480" y="166" text-anchor="middle" font-size="13" fill="#0f172a">約 -21億 〜 21億</text>
<text x="640" y="166" text-anchor="middle" font-size="13" fill="#0f172a">0 〜 約43億</text>
<rect x="20" y="186" width="80" height="30" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="60" y="206" text-anchor="middle" font-size="13" font-weight="700" fill="#1e40af">i64 / u64</text>
<rect x="110" y="192" width="100" height="18" fill="#3b82f6"/>
<text x="218" y="206" font-size="12" fill="#475569">64</text>
<text x="480" y="206" text-anchor="middle" font-size="13" fill="#0f172a">約 ±922京</text>
<text x="640" y="206" text-anchor="middle" font-size="13" fill="#0f172a">0 〜 約1844京</text>
<rect x="20" y="226" width="80" height="30" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="60" y="246" text-anchor="middle" font-size="12" font-weight="700" fill="#1e40af">i128 / u128</text>
<rect x="110" y="232" width="200" height="18" fill="#3b82f6"/>
<text x="318" y="246" font-size="12" fill="#475569">128</text>
<text x="480" y="246" text-anchor="middle" font-size="13" fill="#0f172a">約 ±1.7×10の38乗</text>
<text x="640" y="246" text-anchor="middle" font-size="13" fill="#0f172a">0 〜 約3.4×10の38乗</text>
<rect x="20" y="264" width="80" height="30" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="60" y="284" text-anchor="middle" font-size="12" font-weight="700" fill="#047857">isize/usize</text>
<rect x="110" y="270" width="100" height="18" fill="#10b981"/>
<text x="218" y="284" font-size="12" fill="#475569">64（64ビットPC）</text>
<text x="560" y="284" text-anchor="middle" font-size="13" fill="#047857">長さ・個数・添え字に使う（PC に合わせた幅）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図2-1: 整数の型の一覧。名前の数字がビット幅で、1ビット増えるごとに表せる数は2倍になる</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-02" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="24" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">u8（0〜255）で 250 + 10 を計算すると？（本当の答え 260 は入りきらない）</text>
<rect x="20" y="105" width="150" height="70" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="95" y="135" text-anchor="middle" font-size="18" font-weight="700" fill="#c2410c">250 + 10</text>
<text x="95" y="160" text-anchor="middle" font-size="12" fill="#c2410c">型は u8</text>
<line x1="170" y1="140" x2="260" y2="62" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-02)"/>
<line x1="170" y1="140" x2="260" y2="118" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-02)"/>
<line x1="170" y1="140" x2="260" y2="174" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-02)"/>
<line x1="170" y1="140" x2="260" y2="230" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-02)"/>
<rect x="265" y="42" width="435" height="40" rx="8" fill="#fef2f2" stroke="#ef4444"/>
<text x="280" y="67" font-size="13" fill="#b91c1c">+ 演算子（cargo run の開発ビルド） → panic でプログラムが停止</text>
<rect x="265" y="98" width="435" height="40" rx="8" fill="#f8fafc" stroke="#94a3b8"/>
<text x="280" y="123" font-size="13" fill="#334155">wrapping_add(10) → 4（255 の次は 0 に戻って一周する）</text>
<rect x="265" y="154" width="435" height="40" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="280" y="179" font-size="13" fill="#1e40af">checked_add(10) → None（「計算できなかった」と知らせる）</text>
<rect x="265" y="210" width="435" height="40" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="280" y="235" font-size="13" fill="#047857">saturating_add(10) → 255（上限にはりついて止まる）</text>
<text x="95" y="215" text-anchor="middle" font-size="12" fill="#64748b">リリースビルドの + は</text>
<text x="95" y="233" text-anchor="middle" font-size="12" fill="#64748b">黙って一周（4）する</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図2-2: オーバーフローの4つの扱い方。外から来た値の計算では checked_* で「はみ出し」を検出するのが安全</div>
</div>

### まず動かす最小コード

#### 例1: 基本の型と範囲

> **▼ このコードがやること（先に日本語で）:** 整数・小数・真偽値・文字の値を作って表示し、いくつかの整数型の「最小値と最大値」と、文字型が使うバイト数を調べます。

```rust
fn main() {
    // 整数: 型を書かなければ i32 になる
    let temperature = -5;
    // 型を書いて決める。u8 は 0〜255 の整数
    let level: u8 = 200;
    // 数字の後ろに型名を付けて決める書き方もある
    let big = 3_000_000_000u64;
    // 小数: 型を書かなければ f64 になる
    let ratio = 0.75;
    // 真偽値: true か false のどちらか
    let done = false;
    // 文字: シングルクォート ' で1文字だけを囲む
    let mark = 'あ';

    println!("{temperature} {level} {big} {ratio} {done} {mark}");

    // 型ごとの最小値と最大値
    println!("i32: {} 〜 {}", i32::MIN, i32::MAX);
    println!("u8 : {} 〜 {}", u8::MIN, u8::MAX);
    println!("u64 の最大: {}", u64::MAX);
    // その型が何バイトを使うか
    println!("char は {} バイト", std::mem::size_of::<char>());
}
```

```powershell
cargo run
```

**▼ 実行結果**

```text
-5 200 3000000000 0.75 false あ
i32: -2147483648 〜 2147483647
u8 : 0 〜 255
u64 の最大: 18446744073709551615
char は 4 バイト
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 型を書かないときの「既定の型」

```rust
let temperature = -5;
let ratio = 0.75;
```

- 整数を書いて型を決めなければ `i32`、小数なら `f64` になります。迷ったらこの2つで十分です。
- `3_000_000_000u64` のように**数字の後ろに型名**を付けても型を決められます（`_` は読みやすさのための区切り）。30億は `i32` の最大（約21億）を超えるので、大きな型が必要です。

##### 解説2: `i32::MIN` と `u8::MAX`

```rust
println!("u8 : {} 〜 {}", u8::MIN, u8::MAX);
```

- `型名::MIN` `型名::MAX` で、その型が表せる最小値・最大値が分かります。`::`（コロン2つ）は「その型に属するもの」を取り出す記号です。
- 型ごとの範囲は図2-1のとおりです。**ID や件数のようにマイナスにならない値は `u`、気温や差分のようにマイナスもある値は `i`** を選びます。

| 型 | 使いどころの例 |
|---|---|
| `i32` | 特に理由がないときの整数（既定） |
| `u8` | 0〜255 に収まる小さな値（色の濃さ、年齢など） |
| `u16` | ポート番号（0〜65535） |
| `u32` / `u64` | 件数、金額（円）、大きな ID |
| `i64` | データベース（SQLite / libSQL）の整数列。マイナスも入る |
| `usize` | 長さ・個数・何番目か（`Vec` や文字列の長さはこの型） |
| `f64` | 小数（割合、平均値など） |

##### 解説3: `char` は4バイト

```rust
println!("char は {} バイト", std::mem::size_of::<char>());
```

- `char`（キャラ：character ＝ 文字）は Unicode（ユニコード：世界中の文字に番号を振った規格）の文字1つを表し、英字でも日本語でも**いつも4バイト**です。
- ただし、文字列（`String`）の中では1文字が1〜4バイトに詰めて保存されます。この違いは UNIT 05 で大事になります。

#### 例2: オーバーフローで panic する

> **▼ このコードがやること（先に日本語で）:** 0〜255 しか入らない `u8` の 255 に 1 を足して、はみ出したらどうなるかを確かめます。

```rust
fn main() {
    // 文字列から作った値（コンパイルの時点では中身が分からない値）
    let stock: u8 = "255".parse().unwrap();
    println!("在庫: {stock}");
    // u8 の最大は 255。1 を足すと入りきらない
    let next = stock + 1;
    println!("次: {next}");
}
```

**▼ 実行結果**（画面に出る全文）

```text
在庫: 255

thread 'main' (19992) panicked at src\main.rs:6:16:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\basics.exe` (exit code: 101)
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: panic（パニック）とは

- **panic** は「これ以上安全に続けられないので、プログラムを止めます」という Rust の非常停止です。`attempt to add with overflow` は「足し算がはみ出そうとした」という意味、`src\main.rs:6:16` は止まった場所（6行目16文字目）です。`(19992)` はスレッドの番号で、実行のたびに変わります。
- `cargo run` で作られる**開発用ビルド（debug）**では、はみ出しを検出して止めてくれます。
- `cargo run --release` の**リリースビルド**では検査を省くため、止まらずに**黙って一周**します（このコードなら `次: 0` と表示されました）。どちらにしても、はみ出しは「起きないように書く」か「起きたら気づけるように書く」べきものです。

##### 解説2: なぜ `"255".parse().unwrap()` と書いたのか

- `let stock: u8 = 255;` と直接書くと、Rust は**コンパイルの時点で** `stock + 1` がはみ出すことに気づき、エラーにしてくれます。実行時の panic を見せるために、わざと「実行するまで中身が分からない値」にしました。
- 実際のサーバーでも、計算に使う値の多くはフォームやデータベースから**実行中に**届きます。だから実行時の対策が必要です。

#### 例3: はみ出しを安全に扱うメソッド

> **▼ このコードがやること（先に日本語で）:** 250 に 10 を足す計算を、「一周する」「失敗を知らせる」「上限で止める」など4通りのやり方で行い、結果を見比べます。

```rust
fn main() {
    let stock: u8 = 250;
    // wrapping_add: はみ出したら 0 に戻って数え直す（時計の針のように一周する）
    println!("wrapping_add:    {}", stock.wrapping_add(10));
    // checked_add: はみ出したら None（計算できなかった）を返す
    println!("checked_add:     {:?}", stock.checked_add(10));
    println!("checked_add(5):  {:?}", stock.checked_add(5));
    // saturating_add: はみ出したら最大値で止める
    println!("saturating_add:  {}", stock.saturating_add(10));
    // overflowing_add: 一周した結果と「はみ出したかどうか」の両方を返す
    println!("overflowing_add: {:?}", stock.overflowing_add(10));
}
```

**▼ 実行結果**

```text
wrapping_add:    4
checked_add:     None
checked_add(5):  Some(255)
saturating_add:  255
overflowing_add: (4, true)
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 4つのメソッドの違い

| メソッド | はみ出したとき | 向いている場面 |
|---|---|---|
| `wrapping_add` | 一周する（260 − 256 = 4） | ハッシュ計算など、一周が仕様のとき |
| `checked_add` | `None` を返す | **外から来た値の計算**（いちばんよく使う） |
| `saturating_add` | 最大値（255）で止まる | 回数や音量など「上限で止まればよい」とき |
| `overflowing_add` | `(結果, true)` を返す | はみ出したかどうかも知りたいとき |

- 引き算・掛け算にも `checked_sub` `checked_mul` などがそろっています。

##### 解説2: `{:?}` と `Some` / `None`

```rust
println!("checked_add:     {:?}", stock.checked_add(10));
```

- `checked_add` の結果は「値がある（`Some(255)`）」か「値がない（`None`）」のどちらかです。この型を `Option` と呼びます（UNIT 08 で詳しく）。
- `{:?}` は「プログラマー向けの表示」です。`{}` では表示できない `Option` なども `{:?}` なら中身ごと表示できます。

#### 例4: as と From / TryFrom

> **▼ このコードがやること（先に日本語で）:** 300 を `u8` に、-1 を `u32` に、3.99 を整数に変換してみます。`as` だと数が化けてしまうこと、`TryFrom` なら「入らない」ことを教えてくれることを確かめます。

```rust
fn main() {
    // as: 型を強制的に変換する。入りきらない分は黙って捨てられる
    let big: i32 = 300;
    let small = big as u8;
    println!("300 as u8   = {small}");
    let minus: i32 = -1;
    println!("-1 as u32   = {}", minus as u32);
    // 小数 → 整数は、小数点以下が切り捨てられる
    println!("3.99 as i32 = {}", 3.99_f64 as i32);

    // From: 必ず成功する変換（小さい型 → 大きい型）
    let level: u8 = 200;
    let wide = u32::from(level);
    println!("u32::from(200u8) = {wide}");

    // TryFrom: 失敗するかもしれない変換。結果は Ok(値) か Err(理由)
    println!("u8::try_from(300) = {:?}", u8::try_from(big));
    println!("u8::try_from(100) = {:?}", u8::try_from(100_i32));
}
```

**▼ 実行結果**

```text
300 as u8   = 44
-1 as u32   = 4294967295
3.99 as i32 = 3
u32::from(200u8) = 200
u8::try_from(300) = Err(TryFromIntError(PosOverflow))
u8::try_from(100) = Ok(100)
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `as` は黙って数を変えてしまう

- `300 as u8` が 44 になったのは、300 − 256 = 44 と**一周した分だけが残った**からです。`-1 as u32` は u32 の最大値になりました。どちらもエラーは出ません。
- `as` は手軽ですが、外から来た値に使うと**気づかないうちに数が化ける**危険があります。

##### 解説2: `From` と `TryFrom`

- `u32::from(level)` … `u8` → `u32` のように「必ず入る」変換は `From` で書けます。必ず成功するので結果をそのまま使えます。
- `u8::try_from(big)` … 入らないかもしれない変換は `TryFrom` です。成功なら `Ok(値)`、失敗なら `Err(理由)` が返ります（`PosOverflow` は「プラス側にはみ出した」）。この `Ok` / `Err` の型 `Result` は UNIT 09 で学びます。

> **ヒント:** 変換の選び方は「**必ず入るなら `From`、入らないかもしれないなら `TryFrom`、文字列からなら `parse`**」。`as` は「小数を整数に切り捨てたい」「平均を出すために整数を小数にしたい」など、結果を理解しているときだけ使います。

#### 例5: 小数（f64）の誤差と割り算

> **▼ このコードがやること（先に日本語で）:** 0.1 + 0.2 がぴったり 0.3 にならないことを確かめ、誤差を許した比べ方と、表示の丸め方を試します。整数と小数の割り算の違いも見ます。

```rust
fn main() {
    // 型を f64 と書いておく（書かないと .abs() が使えない。下の「よくあるエラー」参照）
    let a: f64 = 0.1;
    let b: f64 = 0.2;
    // 0.1 + 0.2 は、ぴったり 0.3 にはならない
    println!("0.1 + 0.2 = {}", a + b);
    println!("0.1 + 0.2 == 0.3 ? {}", a + b == 0.3);
    // 小さな誤差を許して比べる
    let close = (a + b - 0.3).abs() < 1e-9;
    println!("ほぼ等しい ? {close}");
    // 表示だけ小数点以下2桁にそろえる
    println!("表示を丸める: {:.2}", a + b);

    // 整数どうしの割り算は、小数点以下を切り捨てる
    println!("7 / 2 = {}", 7 / 2);
    // % は割り算の余り
    println!("7 % 2 = {}", 7 % 2);
    // 小数どうしなら、小数の答えになる
    println!("7.0 / 2.0 = {}", 7.0 / 2.0);
    // 小数を 0 で割ってもエラーにならず「無限大」になる
    println!("1.0 / 0.0 = {}", 1.0_f64 / 0.0);
}
```

**▼ 実行結果**

```text
0.1 + 0.2 = 0.30000000000000004
0.1 + 0.2 == 0.3 ? false
ほぼ等しい ? true
表示を丸める: 0.30
7 / 2 = 3
7 % 2 = 1
7.0 / 2.0 = 3.5
1.0 / 0.0 = inf
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: なぜ 0.30000000000000004 になるのか

- コンピュータは小数を**2進数**（0と1）で記録します。10進数の 0.1 は2進数では「1/3 = 0.333…」のように**割り切れない数**になり、いちばん近い値に丸めて記録されます。その小さなズレが足し算で表に出ました。
- これは Rust だけでなく、JavaScript や Python でも同じです。
- 対策は2つあります。①比べるときは `(差).abs() < 1e-9`（差の絶対値がとても小さいか）で比べる。②**お金のように誤差が許されない値は、小数を使わず整数（円）で持つ**。

##### 解説2: `{:.2}` と整数の割り算

- `{:.2}` は「小数点以下2桁で表示する」書式です。値そのものは変わらず、表示だけが丸まります。
- `7 / 2` が 3 になるのは、**整数どうしの割り算は小数点以下を捨てる**からです。余りは `%` で求めます。
- 整数を 0 で割ると panic しますが、小数を 0 で割ると `inf`（infinity：無限大）になります。

#### 例6: bool と char

> **▼ このコードがやること（先に日本語で）:** ログイン状態を表す真偽値を組み合わせて判定し、文字を番号に変換したり、文字の種類を調べたりします。

```rust
fn main() {
    let logged_in = true;
    let is_owner = false;
    // && は「かつ」、|| は「または」、! は「ではない」
    println!("編集できる: {}", logged_in && is_owner);
    println!("閲覧できる: {}", logged_in || is_owner);
    println!("ログアウト中: {}", !logged_in);

    // char は Unicode の1文字。番号（コードポイント）に変換できる
    let c = 'あ';
    println!("'{c}' の番号: {} (16進数で {:X})", c as u32, c as u32);
    // 番号から char に戻す（存在しない番号もあるので、結果は Some か None）
    println!("0x41 は {:?}", char::from_u32(0x41));
    // 数字の文字を数に変換する
    println!("'7' を数に: {:?}", '7'.to_digit(10));
    // 文字の種類を調べる
    println!("'A' は英字? {}", 'A'.is_alphabetic());
    println!("'あ' は ASCII の英数字? {}", 'あ'.is_ascii_alphanumeric());
}
```

**▼ 実行結果**

```text
編集できる: false
閲覧できる: true
ログアウト中: false
'あ' の番号: 12354 (16進数で 3042)
0x41 は Some('A')
'7' を数に: Some(7)
'A' は英字? true
'あ' は ASCII の英数字? false
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 真偽値の組み合わせ

- `bool`（ブール）は `true`（真）か `false`（偽）の2つだけの型です。`&&`（かつ）・`||`（または）・`!`（ではない）で組み合わせます。
- 「ログインしていて**かつ**持ち主なら編集できる」のように、権限の判定でよく使います。

##### 解説2: 文字と番号

- `'あ'` の番号は 12354（16進数で 3042）です。Unicode の表では `U+3042` と書きます。`{:X}` は「16進数の大文字で表示する」書式です。
- `char::from_u32(0x41)` は番号 0x41（65）の文字 `'A'` を返します。番号によっては文字が存在しないので、結果は `Some` か `None` です。
- `'7'.to_digit(10)` は「10進数の数字として読むと 7」です。`'x'` のように数字でない文字なら `None` になります。

### もう一歩: 実務に近い形

お金の計算と、データベースから来た ID の変換です。どちらも「はみ出し」と「不正な値」を見逃さないことが大切です。

#### 金額の計算と ID の変換

> **▼ このコードがやること（先に日本語で）:** 単価（円）×個数を、はみ出しを検出しながら計算します。続いて、データベースから届いた `i64` の ID を、マイナスなら拒否しながら `u64` へ変換します。

```rust
fn main() {
    // 金額は小数ではなく「円（整数）」で持つ
    let unit_price: u32 = 1_250;
    // 個数はフォームから u16 で届いたとする
    let quantity: u16 = 3;

    // u16 → u32 は必ず成功するので From を使う
    let quantity = u32::from(quantity);
    // 掛け算がはみ出したら None になる checked_mul を使う
    match unit_price.checked_mul(quantity) {
        Some(total) => println!("合計: {total} 円"),
        None => println!("金額が大きすぎます"),
    }

    // 極端に大きな個数も試す
    let huge: u32 = 4_000_000;
    match unit_price.checked_mul(huge) {
        Some(total) => println!("合計: {total} 円"),
        None => println!("金額が大きすぎます"),
    }

    // データベースの ID は i64 で届くことが多い。負の値は ID として不正
    for raw_id in [42_i64, -7] {
        match u64::try_from(raw_id) {
            Ok(id) => println!("ID {id} を使います"),
            Err(e) => println!("ID {raw_id} は使えません: {e}"),
        }
    }
}
```

**▼ 実行結果**

```text
合計: 3750 円
金額が大きすぎます
ID 42 を使います
ID -7 は使えません: out of range integral type conversion attempted
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `match` で「成功」と「失敗」に分ける

```rust
match unit_price.checked_mul(quantity) {
    Some(total) => println!("合計: {total} 円"),
    None => println!("金額が大きすぎます"),
}
```

- `match`（マッチ）は「結果の形ごとに処理を分ける」構文です（UNIT 04・08 で詳しく）。`Some(total)` なら中の値を `total` という名前で取り出して表示し、`None` なら警告を表示します。
- 1,250 × 4,000,000 = 50億は `u32` の最大（約43億）を超えるので `None` になりました。panic せず、きちんと「大きすぎる」と伝えられています。

##### 解説2: `try_from` の `Err` を表示する

- `u64::try_from(-7)` は失敗し、`Err(e)` の `e` に理由が入ります。`{e}` で表示すると `out of range integral type conversion attempted`（範囲外の整数変換が試みられた）という説明になります。
- `for raw_id in [42_i64, -7]` は「42 と -7 で1回ずつ同じ処理をくり返す」書き方です（UNIT 04）。

### よくあるエラーと読み方

#### エラー1: E0308 / E0277 違う整数型どうしを足した

```rust
fn main() {
    let count: i32 = 10;
    let total: i64 = 1_000;
    let sum = count + total;
    println!("{sum}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0308]: mismatched types
 --> src\main.rs:4:23
  |
4 |     let sum = count + total;
  |                       ^^^^^ expected `i32`, found `i64`

error[E0277]: cannot add `i64` to `i32`
 --> src\main.rs:4:21
  |
4 |     let sum = count + total;
  |                     ^ no implementation for `i32 + i64`
  |
  = help: the trait `Add<i64>` is not implemented for `i32`
help: the following other types implement trait `Add<Rhs>`
 --> /rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library\core\src\ops\arith.rs:98:8
  |
  = note: `i32` implements `Add`
 ::: /rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library\core\src\ops\arith.rs:113:0
  |
  = note: in this macro invocation
 --> /rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library\core\src\internal_macros.rs:22:8
  |
  = note: `&i32` implements `Add<i32>`
 ::: /rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library\core\src\internal_macros.rs:33:8
  |
  = note: `i32` implements `Add<&i32>`
 ::: /rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library\core\src\internal_macros.rs:44:8
  |
  = note: `&i32` implements `Add`
  = note: this error originates in the macro `add_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `basics` (bin "basics") due to 2 previous errors
```

- **読み方:** 長く見えますが、大事なのは最初の2つの見出しだけです。`expected i32, found i64`（i32 が来るはずが i64 が来た）と `cannot add i64 to i32`（i32 に i64 は足せない）。後半の `help:` は「i32 に足せる型の一覧」なので、慣れるまでは読み飛ばしてかまいません。
- **何が起きたか:** Rust は `i32` と `i64` のように**型が違う数どうしの計算を自動ではしません**。他の言語のように勝手にそろえないことで、意図しないはみ出しや切り捨てを防いでいます。
- **直し方:** 小さい方を大きい方へ `From` で変換します: `let sum = i64::from(count) + total;`

#### エラー2: 型の範囲を超える数を直接書いた

```rust
fn main() {
    let level: u8 = 256;
    println!("{level}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error: literal out of range for `u8`
 --> src\main.rs:2:21
  |
2 |     let level: u8 = 256;
  |                     ^^^
  |
  = note: the literal `256` does not fit into the type `u8` whose range is `0..=255`
  = note: `#[deny(overflowing_literals)]` on by default

error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **何が起きたか:** `256` は `u8` の範囲 `0..=255`（0以上255以下）に入りません。直接書いた数（**リテラル**）なら、コンパイラが事前に見つけてくれます。
- **直し方:** 値が正しいなら型を広げます（`let level: u16 = 256;`）。

#### エラー3: E0600 符号なしの型にマイナスを入れた

```rust
fn main() {
    let stock: u32 = -1;
    println!("{stock}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0600]: cannot apply unary operator `-` to type `u32`
 --> src\main.rs:2:22
  |
2 |     let stock: u32 = -1;
  |                      ^^ cannot apply unary operator `-`
  |
  = note: unsigned values cannot be negated
help: you may have meant the maximum value of `u32`
  |
2 -     let stock: u32 = -1;
2 +     let stock: u32 = u32::MAX;
  |

For more information about this error, try `rustc --explain E0600`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **何が起きたか:** `u32` は 0 以上しか表せないので、`-`（マイナス記号）が使えません。`help` は「最大値のつもりでしたか？」と推測していますが、多くの場合の正解は**マイナスも入る型（`i32` など）にする**ことです。

#### エラー4: E0277 整数を小数で割った

```rust
fn main() {
    let total = 7;
    let half = total / 2.0;
    println!("{half}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0277]: cannot divide `{integer}` by `{float}`
 --> src\main.rs:3:22
  |
3 |     let half = total / 2.0;
  |                      ^ no implementation for `{integer} / {float}`
  |
  = help: the trait `Div<{float}>` is not implemented for `{integer}`
  = help: the following other types implement trait `Div<Rhs>`:
            `&f128` implements `Div<f128>`
            `&f128` implements `Div`
            `&f16` implements `Div<f16>`
            `&f16` implements `Div`
            `&f32` implements `Div<f32>`
            `&f32` implements `Div`
            `&f64` implements `Div<f64>`
            `&f64` implements `Div`
          and 62 others

For more information about this error, try `rustc --explain E0277`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **何が起きたか:** 整数（`{integer}`）と小数（`{float}`）は混ぜて計算できません。`{integer}` は「まだ i32 か u8 かは決まっていないが、整数のどれか」という意味です。
- **直し方:** 割る前に整数を小数へ変換します。`let half = total as f64 / 2.0;` とすると `3.5` が表示されます（整数 → 小数の `as` は意味が分かっているので安全な使い方です）。

#### エラー5: E0689 型の決まっていない小数でメソッドを呼んだ

例5で型を書かずに `.abs()` を呼ぶと、エラーになります。

```rust
fn main() {
    let a = 0.1;
    let b = 0.2;
    let diff = (a + b - 0.3).abs();
    println!("{diff}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0689]: can't call method `abs` on ambiguous numeric type `{float}`
 --> src\main.rs:4:30
  |
4 |     let diff = (a + b - 0.3).abs();
  |                              ^^^

For more information about this error, try `rustc --explain E0689`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **何が起きたか:** `{float}` は「f32 か f64 か、まだ決まっていない小数」です。型が決まらないと、どの型の `abs` を呼べばよいか分かりません。
- **直し方:** `let a: f64 = 0.1;` のように型を書きます。

### RustNote ではここで使う

RustNote のノート一覧は1ページに20件ずつ表示し、URL の `?page=3` のような**文字列**でページ番号を受け取ります。ここでは「数でない」「0」「マイナス」などのおかしな値でも、安全に1ページ目へ倒す処理を書きます。

> **▼ このコードがやること（先に日本語で）:** いろいろな `?page=` の値を `u32` に変換し、そのページで「先頭から何件を飛ばすか」を計算します。

```rust
// 1ページに表示するノートの件数
const PER_PAGE: u32 = 20;

fn main() {
    // URL の ?page= に入ってくる文字列をいくつか試す
    for raw in ["1", "3", "0", "-2", "abc"] {
        // 数（u32）にできなければ 1 ページ目とみなす
        let page: u32 = raw.parse().unwrap_or(1);
        // 0 ページ目は存在しないので、最低でも 1 にそろえる
        let page = page.max(1);
        // 先頭から何件を飛ばして表示するか
        let offset = (page - 1) * PER_PAGE;
        println!("?page={raw} → {page} ページ目, {offset} 件飛ばす");
    }
}
```

**▼ 実行結果**

```text
?page=1 → 1 ページ目, 0 件飛ばす
?page=3 → 3 ページ目, 40 件飛ばす
?page=0 → 1 ページ目, 0 件飛ばす
?page=-2 → 1 ページ目, 0 件飛ばす
?page=abc → 1 ページ目, 0 件飛ばす
```

- `u32` で受け取ると、`-2` は**数への変換の時点で失敗する**ので、マイナスのページが紛れ込みません。型を選ぶことが、そのまま入力チェックになっています。
- `page.max(1)` は「page と 1 の大きい方」です。これで 0 を 1 にそろえ、`page - 1` がマイナス（u32 のはみ出し）になるのを防いでいます。
- ほかにも RustNote では、ノートの「完了」を `done: bool`、データベースの ID を `i64` で扱います。

### 練習問題

**問題1.** `let x: u8 = 200;` のとき、`x.checked_add(100)`、`x.wrapping_add(100)`、`x.saturating_add(100)` はそれぞれ何になるでしょうか。予想してから実行して確かめてください。

**問題2.** 設定ファイルから `i64` で届いたポート番号 `8080`、`70000`、`-1` を、`u16::try_from` で `u16` に変換し、成功・失敗をそれぞれ表示してください。

**問題3.** 3人の点数 80, 95, 72（`u32`）の平均を、小数点以下1桁まで `平均: 82.3 点` のように表示してください。

### 解答と解説

**解答1.**

```rust
fn main() {
    let x: u8 = 200;
    println!("{:?}", x.checked_add(100));
    println!("{}", x.wrapping_add(100));
    println!("{}", x.saturating_add(100));
}
```

**▼ 実行結果**

```text
None
44
255
```

- 本当の答え 300 は 255 を超えます。`checked_add` は `None`、`wrapping_add` は 300 − 256 = 44、`saturating_add` は上限の 255 です。

**解答2.**

```rust
fn main() {
    for raw in [8080_i64, 70000, -1] {
        match u16::try_from(raw) {
            Ok(port) => println!("{raw}: ポート {port} を使います"),
            Err(e) => println!("{raw}: 使えません（{e}）"),
        }
    }
}
```

**▼ 実行結果**

```text
8080: ポート 8080 を使います
70000: 使えません（out of range integral type conversion attempted）
-1: 使えません（out of range integral type conversion attempted）
```

- `u16` は 0〜65535 なので、70000 も -1 も入りません。`as u16` を使っていたら 70000 は 4464 に化けて、別のポートで起動してしまうところでした。

**解答3.**

```rust
fn main() {
    let a: u32 = 80;
    let b: u32 = 95;
    let c: u32 = 72;
    // 合計は整数のまま計算する
    let sum = a + b + c;
    // 割る直前に f64 へ変換する
    let average = sum as f64 / 3.0;
    println!("平均: {average:.1} 点");
}
```

**▼ 実行結果**

```text
平均: 82.3 点
```

- `sum / 3` のまま割ると整数の割り算で 82 になってしまいます。割る直前に `as f64` で小数にするのがポイントです。`{average:.1}` は「小数点以下1桁」の書式です。

### まとめ

- 整数の型は「`i`/`u`（マイナスの有無）＋ビット幅」。範囲を超える計算は debug で panic、release で一周するので、外からの値には `checked_*` を使う。
- 小数には誤差がある。比べるときは差の大きさで比べ、お金は整数で持つ。`char` は Unicode の1文字で4バイト。
- 型の変換は「必ず入るなら `From`、入らないかもしれないなら `TryFrom`、文字列からなら `parse`」。`as` は結果を理解しているときだけ。

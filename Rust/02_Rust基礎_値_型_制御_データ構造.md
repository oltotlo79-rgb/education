<div style="page-break-after: always; text-align: center; margin: 0; padding: 0;">
<img src="assets/cover.jpg" alt="表紙" style="width: 100%; max-height: 175mm; object-fit: cover; border-radius: 10px;">
<div style="margin-top: 14px; font-size: 11px; letter-spacing: 0.18em; color: #c2410c; font-weight: 700;">VOLUME 02 / RUST BACKEND MASTERY</div>
<div style="font-size: 26px; font-weight: 800; color: #0f172a; margin-top: 6px; line-height: 1.35;">Rust基礎 — 値・型・制御・データ構造</div>
<div style="font-size: 13px; color: #475569; margin-top: 8px;">変数・型・制御構文・文字列・コレクション・エラー処理を、実行結果と一緒に1つずつ身に付ける</div>
</div>

# 第2巻: Rust基礎 — 値・型・制御・データ構造

## UNIT 01: 変数・mut・シャドーイング・定数

### この単元のゴール

- `let` で値に名前を付け、`println!` で表示できる
- Rust の変数が「既定で変更できない（不変）」理由を説明し、必要なときだけ `mut` を付けられる
- 同じ名前で値を作り直す**シャドーイング**と、`mut` による書き換えの違いを説明できる
- 「プログラム全体で変わらない値」を `const`（定数）で書ける

### たとえ話でつかむ

変数は「**箱に貼った名札**」だと考えてください。

- `let x = 5;` は、5 を入れた箱を用意して「x」という名札を貼り、**ふたをテープで閉じる**ことです。中身を見ることはできますが、入れ替えはできません。
- `let mut y = 5;` の `mut`（ミュート：mutable ＝ 変更可能 の略）は、「**ふたを開けたままにしておく**」という宣言です。あとから中身を入れ替えられます。
- **シャドーイング**（shadowing：影にする、の意味）は、同じ名札を**新しい箱に貼り替える**ことです。古い箱はそのまま残りますが、名札がなくなったので、もうその名前では呼べません。新しい箱には別の種類（型）のものを入れてもかまいません。
- `const`（コンスト：constant ＝ 定数）は、**金属のプレートに刻印した値**です。プログラムを作る（コンパイルする）時点で決まっていて、二度と変わりません。

なぜ「ふたが閉じている」のが基本なのでしょうか。大きなプログラムでは、「この値はどこかで書き換えられていないか？」を疑いながら読むのはとても疲れます。Rust は**変えない値を標準**にし、変える値にだけ `mut` という目印を付けさせることで、読む人が安心できるようにしています。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="340" height="250" rx="12" fill="#eff6ff" stroke="#3b82f6"/>
<text x="180" y="40" text-anchor="middle" font-size="16" font-weight="700" fill="#1e40af">let x = 5;（既定は不変）</text>
<rect x="120" y="96" width="140" height="84" rx="8" fill="#ffffff" stroke="#1e40af" stroke-width="2"/>
<rect x="110" y="82" width="160" height="16" rx="4" fill="#1e40af"/>
<line x1="190" y1="82" x2="190" y2="98" stroke="#f97316" stroke-width="6"/>
<text x="190" y="150" text-anchor="middle" font-size="30" font-weight="700" fill="#0f172a">5</text>
<rect x="40" y="122" width="60" height="32" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="70" y="144" text-anchor="middle" font-size="16" font-weight="700" fill="#c2410c">x</text>
<line x1="100" y1="138" x2="120" y2="138" stroke="#f97316" stroke-width="2"/>
<text x="180" y="212" text-anchor="middle" font-size="13" fill="#334155">ふたがテープで閉じている</text>
<text x="180" y="236" text-anchor="middle" font-size="13" fill="#c2410c">x = 6; → コンパイルエラー（E0384）</text>
<rect x="370" y="10" width="340" height="250" rx="12" fill="#ecfdf5" stroke="#10b981"/>
<text x="540" y="40" text-anchor="middle" font-size="16" font-weight="700" fill="#047857">let mut y = 5;（変更可能）</text>
<rect x="480" y="96" width="140" height="84" rx="8" fill="#ffffff" stroke="#047857" stroke-width="2"/>
<rect x="470" y="80" width="160" height="16" rx="4" fill="#047857" transform="rotate(-10 470 96)"/>
<text x="530" y="150" text-anchor="middle" font-size="26" font-weight="700" fill="#94a3b8" text-decoration="line-through">5</text>
<text x="575" y="150" text-anchor="middle" font-size="30" font-weight="700" fill="#0f172a">6</text>
<rect x="400" y="122" width="60" height="32" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="430" y="144" text-anchor="middle" font-size="16" font-weight="700" fill="#c2410c">y</text>
<line x1="460" y1="138" x2="480" y2="138" stroke="#f97316" stroke-width="2"/>
<text x="540" y="212" text-anchor="middle" font-size="13" fill="#334155">ふたが開いている（mut の目印）</text>
<text x="540" y="236" text-anchor="middle" font-size="13" fill="#047857">y = 6; → OK。中身を入れ替えられる</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-1: 変数は「名札を貼った箱」。let だけならふたが閉じ、mut を付けるとふたが開く</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-01" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="26" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">シャドーイング: 同じ名札を「新しい箱」に貼り替える</text>
<rect x="20" y="70" width="170" height="80" rx="8" fill="#f8fafc" stroke="#94a3b8" stroke-dasharray="5 4"/>
<text x="105" y="108" text-anchor="middle" font-size="18" fill="#94a3b8">"&#160;&#160;42&#160;&#160;"</text>
<text x="105" y="134" text-anchor="middle" font-size="12" fill="#94a3b8">型: &amp;str（文字列）</text>
<rect x="275" y="70" width="170" height="80" rx="8" fill="#f8fafc" stroke="#94a3b8" stroke-dasharray="5 4"/>
<text x="360" y="108" text-anchor="middle" font-size="18" fill="#94a3b8">"42"</text>
<text x="360" y="134" text-anchor="middle" font-size="12" fill="#94a3b8">型: &amp;str（文字列）</text>
<rect x="530" y="70" width="170" height="80" rx="8" fill="#eff6ff" stroke="#1e40af" stroke-width="2"/>
<text x="615" y="108" text-anchor="middle" font-size="22" font-weight="700" fill="#0f172a">42</text>
<text x="615" y="134" text-anchor="middle" font-size="12" fill="#1e40af">型: u32（数）</text>
<line x1="190" y1="110" x2="270" y2="110" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-01)"/>
<text x="230" y="100" text-anchor="middle" font-size="13" fill="#475569">.trim()</text>
<line x1="445" y1="110" x2="525" y2="110" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-01)"/>
<text x="485" y="100" text-anchor="middle" font-size="13" fill="#475569">.parse()</text>
<rect x="580" y="40" width="70" height="26" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="615" y="58" text-anchor="middle" font-size="14" font-weight="700" fill="#c2410c">input</text>
<text x="105" y="178" text-anchor="middle" font-size="12" fill="#64748b">1つ目の input</text>
<text x="360" y="178" text-anchor="middle" font-size="12" fill="#64748b">2つ目の input</text>
<text x="615" y="178" text-anchor="middle" font-size="12" fill="#1e40af">3つ目の input（今はこれ）</text>
<text x="360" y="212" text-anchor="middle" font-size="13" fill="#334155">古い箱は名札を失い、もう input という名前では呼べない（型が変わってもよい）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-2: let を書き直すたびに新しい箱ができ、名札だけが移っていく</div>
</div>

4つの書き方を表にまとめます。

| 書き方 | 中身の入れ替え | 型を変える | いつ使うか |
|---|---|---|---|
| `let x = 5;` | できない | できない | ほとんどの値（まずはこれ） |
| `let mut x = 5;` | できる | できない | 数を数える、少しずつ組み立てるなど、本当に変わる値 |
| `let x = x + 1;`（シャドーイング） | 新しい箱を作る | できる | 「文字列 → 空白除去 → 数」のように段階的に変換するとき |
| `const MAX: u32 = 80;` | できない | できない | プログラム全体で共有する決まった値（上限・既定値） |

### まず動かす最小コード

この巻のコードは、すべて次の手順で動かせます（詳しくは巻頭の「この巻の進め方」）。

```powershell
# 最初に1回だけ: 練習用プロジェクトを作って中に入る
cargo new basics
cd basics
# 以後は src/main.rs の中身を例のコードに置き換えて実行する
cargo run
```

#### 例1: let で名前を付けて表示する

> **▼ このコードがやること（先に日本語で）:** 文字と数にそれぞれ名前を付けて、画面に表示します。`println!` の中の `{}` が「ここに値を差し込む」印です。

```rust
fn main() {
    // title という名前に、文字列 "Rust入門" を結び付ける
    let title = "Rust入門";
    // count という名前に、数 3 を結び付ける
    let count = 3;

    // {} の位置に、後ろに書いた値が入って表示される
    println!("タイトル: {}", title);
    // {count} のように、波かっこの中へ変数名を直接書いてもよい
    println!("ノートの数: {count}");
}
```

```powershell
cargo run
```

**▼ 実行結果**

```text
タイトル: Rust入門
ノートの数: 3
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `fn main() { ... }`

```rust
fn main() {
    // ここに書いた文が上から順に実行される
}
```

- `fn`（エフエヌ：function ＝ 関数 の略）は「ひとまとまりの処理を作る」キーワードです。
- `main` という名前の関数は特別で、プログラムを起動すると**最初に呼ばれる入口**です。
- `{ }`（波かっこ）の中が関数の中身です。関数については UNIT 03 で詳しく学びます。

##### 解説2: `let 名前 = 値;`

```rust
let title = "Rust入門";
let count = 3;
```

- `let`（レット）は「この値に名前を付ける」宣言です。この「名前と値を結び付けること」を**束縛**（バインディング）とも呼びます。
- `"..."` で囲んだものは文字列、`3` は数です。どちらも型（値の種類）を書いていませんが、Rust が右側の値から自動で決めます。これを**型推論**（かたすいろん）と呼びます。
- 文の終わりには `;`（セミコロン）を付けます。

> **用語:** **型（かた）** … 値の種類のこと。「整数」「文字列」「真偽値」などがあります。Rust はすべての値の型をコンパイルの時点で決めます。

##### 解説3: `println!` と `{}`

```rust
println!("タイトル: {}", title);
println!("ノートの数: {count}");
```

- `println!`（プリントライン）は画面に1行表示する命令です。名前の最後の `!` は、これが関数ではなく**マクロ**（コードを自動で組み立てる仕組み）であることを表します。今は「表示するときは `println!`」と覚えれば十分です。
- `{}` は「ここに値を差し込む」穴です。後ろに `, title` と書いた値が入ります。
- `{count}` のように、穴の中へ変数名を直接書くこともできます。本書ではこちらを多く使います。

#### 例2: mut で中身を入れ替える

> **▼ このコードがやること（先に日本語で）:** 完了したノートの数を数える変数を作り、1を足したり2を足したりして、そのたびに表示します。数が変わる変数なので `mut` を付けます。

```rust
fn main() {
    // mut を付けると、あとから中身を入れ替えられる変数になる
    let mut done_count = 0;
    println!("はじめ: {done_count}");

    // 今の値に 1 を足して、同じ変数へ入れ直す
    done_count = done_count + 1;
    println!("1件完了: {done_count}");

    // += は「自分に足して入れ直す」の短い書き方
    done_count += 2;
    println!("さらに2件完了: {done_count}");
}
```

**▼ 実行結果**

```text
はじめ: 0
1件完了: 1
さらに2件完了: 3
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `let mut`

```rust
let mut done_count = 0;
```

- `mut` を付けた変数だけが、あとから `=` で別の値を入れ直せます。これを**再代入**（さいだいにゅう）と呼びます。
- `mut` は「この値はこの先変わります」という**読む人へのお知らせ**でもあります。

##### 解説2: 再代入と `+=`

```rust
done_count = done_count + 1;
done_count += 2;
```

- 1行目は「右側を先に計算（0 + 1 = 1）してから、その結果を `done_count` に入れる」という意味です。数学の「等しい」ではなく「入れる」です。
- `+=` は `done_count = done_count + 2` の短縮形です。`-=` `*=` `/=` も同じように使えます。
- 再代入できるのは**同じ型の値だけ**です。数の変数に文字列を入れることはできません（下の「よくあるエラー」で実際に試します）。

#### 例3: シャドーイングで段階的に変換する

> **▼ このコードがやること（先に日本語で）:** 画面から届いたような「前後に空白のある文字列」を、空白を取り除き、さらに数へ変換します。3段階とも `input` という同じ名前を使い、そのたびに新しい変数を作り直しています。

```rust
fn main() {
    // 段階1: 画面から受け取った文字列（前後に空白がある）
    let input = "  42  ";
    println!("[{input}]");

    // 段階2: 同じ名前で「前後の空白を取り除いた版」を作り直す
    let input = input.trim();
    println!("[{input}]");

    // 段階3: さらに同じ名前で「数（u32 型）に変換した版」を作り直す
    let input: u32 = input.parse().expect("数字ではありません");
    // 数になったので足し算ができる
    println!("{}", input + 1);
}
```

**▼ 実行結果**

```text
[  42  ]
[42]
43
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: もう一度 `let` を書くと「新しい変数」になる

```rust
let input = "  42  ";
let input = input.trim();
```

- 2行目の右側の `input` は「1つ目の input」、左側の `input` は「2つ目の input」です。右側を先に計算してから、新しい箱に名札を貼ります。
- `mut` を付けていないのにエラーにならないのは、**書き換えではなく新しく作っている**からです（図1-2）。
- `.trim()`（トリム）は文字列の前後の空白を取り除いた結果を返します。表示の `[` `]` は空白が見えるように付けた飾りです。

##### 解説2: 型を変えられるのがシャドーイングの強み

```rust
let input: u32 = input.parse().expect("数字ではありません");
```

- `: u32` は「この変数の型は u32（0以上の整数）です」という**型注釈**（かたちゅうしゃく）です。
- `.parse()`（パース：解析する）は文字列を別の型に変換します。どの型にするかを `: u32` から読み取ります。
- `.expect("...")` は「変換に失敗したらこのメッセージを出してプログラムを止める」という意味です。失敗の扱い方は UNIT 09 で丁寧に学びます。
- 文字列から数へと**型が変わっている**ことに注目してください。`mut` の再代入ではこれはできません。

#### 例4: ブロックの中だけのシャドーイング

> **▼ このコードがやること（先に日本語で）:** `{ }` で囲んだ範囲の中でだけ同じ名前の変数を作り直し、範囲を出たら元の値が見えることを確かめます。

```rust
fn main() {
    let level = 1;
    {
        // このブロック { } の中だけで有効な、新しい level を作る
        let level = level * 10;
        println!("ブロックの中: {level}");
    }
    // ブロックを出ると、新しい level は消え、元の level が見える
    println!("ブロックの外: {level}");
}
```

**▼ 実行結果**

```text
ブロックの中: 10
ブロックの外: 1
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 変数には「有効範囲」がある

- 変数が使える範囲を**スコープ**（scope：範囲）と呼びます。`let` で作った変数は、それを囲む `{ }` の終わりまで有効です。
- ブロックの中で作った2つ目の `level` は `}` で消えます。すると、隠れていた1つ目の `level`（値は 1）がまた見えるようになります。
- 1つ目の `level` は一度も書き換えられていません。だから `mut` は不要です。

#### 例5: const で決まった値に名前を付ける

> **▼ このコードがやること（先に日本語で）:** タイトルの最大文字数やセッションの日数など、プログラム全体で変わらない値を定数にして使います。定数同士の計算もしてみます。

```rust
// 定数はファイルのどこからでも使える。型の注釈（: usize など）が必須
const MAX_TITLE_CHARS: usize = 80;
// 定数の値は、コンパイルするときに計算できる式なら書ける
const SECONDS_PER_DAY: u64 = 24 * 60 * 60;
const SESSION_DAYS: u64 = 30;

fn main() {
    println!("タイトルは最大 {MAX_TITLE_CHARS} 文字");

    let session_seconds = SESSION_DAYS * SECONDS_PER_DAY;
    println!("セッションの有効期限: {session_seconds} 秒");

    // 数字の中の _ は読みやすくするための区切りで、値は変わらない
    let big = 1_000_000;
    println!("百万: {big}");
}
```

**▼ 実行結果**

```text
タイトルは最大 80 文字
セッションの有効期限: 2592000 秒
百万: 1000000
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `const` の決まりごと

```rust
const MAX_TITLE_CHARS: usize = 80;
```

- 名前は**すべて大文字＋アンダースコア**（`MAX_TITLE_CHARS`）で書くのが Rust の習慣です。変数は小文字（`done_count`）なので、見ただけで区別できます。
- `: usize` のような**型注釈が必須**です。`let` と違い、定数では型を省略できません。
- `fn main()` の外（ファイルの一番上）に書けるので、どの関数からでも使えます。
- `mut` は付けられません。値は**コンパイルの時点で決まり**、実行中に変わることはありません。

> **用語:** **usize / u64** … どちらも「0以上の整数」の型です。`usize` は長さや個数、`u64` は大きな数に使います。整数の型は UNIT 02 で詳しく学びます。

> **ヒント:** `const` によく似た `static`（スタティック）もあります。`static` は「プログラムの実行中ずっと同じ場所に1つだけ存在する値」です。初心者のうちは、決まった値には `const` を使えば困りません。

### もう一歩: 実務に近い形

バックエンドでは、「環境変数などから**文字列で**届いた設定値を、空白を取り除き、数に変換し、失敗したら既定値を使う」という処理がよく出てきます。段階的な変換はシャドーイング、変化する状態（試行回数）だけを `mut`、決まった値は `const` にします。

#### 設定値を段階的に整える

> **▼ このコードがやること（先に日本語で）:** 文字列のポート番号を数へ変換します。数字でなかったときは既定値 8080 を使います。最後に、接続を3回まで試すふりをして、試行回数を数えます。

```rust
// 設定値の「既定値」と「上限」は定数にしておく
const DEFAULT_PORT: u16 = 8080;
const MAX_RETRIES: u32 = 3;

fn main() {
    // 環境変数などから届いた「文字列の」ポート番号を想定する
    let port = " 3000 ";
    // 段階1: 空白を取り除く（文字列のまま）
    let port = port.trim();
    // 段階2: 数（u16）へ変換する。変換に失敗したら既定値を使う
    let port: u16 = port.parse().unwrap_or(DEFAULT_PORT);
    println!("待ち受けポート: {port}");

    // 数字ではない文字列だった場合も試す
    let broken = "abc";
    let broken: u16 = broken.trim().parse().unwrap_or(DEFAULT_PORT);
    println!("壊れた設定のときのポート: {broken}");

    // 変化する状態（試行回数）だけを mut にする
    let mut attempts = 0;
    while attempts < MAX_RETRIES {
        attempts += 1;
        println!("{attempts} 回目の接続を試みます");
    }
    println!("上限の {MAX_RETRIES} 回に達しました");
}
```

**▼ 実行結果**

```text
待ち受けポート: 3000
壊れた設定のときのポート: 8080
1 回目の接続を試みます
2 回目の接続を試みます
3 回目の接続を試みます
上限の 3 回に達しました
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `unwrap_or(既定値)`

```rust
let port: u16 = port.parse().unwrap_or(DEFAULT_PORT);
```

- `.parse()` は成功することも失敗することもあります。`.unwrap_or(DEFAULT_PORT)` は「成功したらその値、失敗したら `DEFAULT_PORT`」を選びます。
- `"abc"` は数にできないので、2つ目の結果は 8080 になりました。プログラムは止まりません。この「成功か失敗か」の仕組み（`Result`）は UNIT 09 で学びます。

##### 解説2: `mut` は狭い範囲に閉じ込める

```rust
let mut attempts = 0;
while attempts < MAX_RETRIES {
    attempts += 1;
}
```

- `while` は「条件が成り立つ間くり返す」構文です（UNIT 04 で詳しく）。
- 実際に変化するのは `attempts` だけなので、`mut` もこれだけに付けます。`port` は一度決まったら変えないので `mut` は付けません。「変わるものが少ない」ほど、プログラムは読みやすく、間違いにくくなります。

### よくあるエラーと読み方

#### エラー1: E0384 不変の変数に2回代入した

```rust
fn main() {
    let count = 1;
    println!("{count}");
    count = 2;
    println!("{count}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0384]: cannot assign twice to immutable variable `count`
 --> src\main.rs:4:5
  |
2 |     let count = 1;
  |         ----- first assignment to `count`
3 |     println!("{count}");
4 |     count = 2;
  |     ^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut count = 1;
  |         +++

For more information about this error, try `rustc --explain E0384`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** 1行目の `error[E0384]` がエラーの番号と要約です（「不変の変数 `count` に2回代入することはできません」）。`--> src\main.rs:4:5` は「src\main.rs の4行目5文字目」という場所です。`^^^^` の下線が問題の箇所、`-----` が関係する箇所（最初の代入）を指しています。
- **何が起きたか:** `mut` を付けずに作った `count` に、`count = 2;` で入れ直そうとしました。
- **直し方:** `help:` の提案どおり `let mut count = 1;` にします。または「入れ直す」のではなく「作り直す」意味でよければ `let count = 2;`（シャドーイング）にします。
- `rustc --explain E0384` を実行すると、このエラーの詳しい説明（英語）が読めます。

#### エラー2: E0308 mut の変数に別の型の値を入れた

```rust
fn main() {
    let mut input = " 42 ";
    input = input.trim();
    input = input.parse::<u32>().expect("数字ではありません");
    println!("{input}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0308]: mismatched types
 --> src\main.rs:4:13
  |
2 |     let mut input = " 42 ";
  |                     ------ expected due to this value
3 |     input = input.trim();
4 |     input = input.parse::<u32>().expect("数字ではありません");
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&str`, found `u32`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** `mismatched types` は「型が合っていません」。`expected &str, found u32` は「&str（文字列）が来るはずなのに u32（数）が来た」です。`expected due to this value` が、型が決まった場所（2行目の `" 42 "`）を指しています。
- **何が起きたか:** `mut` は「中身を入れ替えてよい」だけで、**型は変えられません**。`input` は2行目で文字列型に決まっています。
- **直し方:** 4行目を `let input: u32 = input.parse().expect("数字ではありません");` にしてシャドーイングします。そうすると `mut` も不要になります。

#### エラー3: 定数に型を書き忘れた

```rust
const MAX_TITLE_CHARS = 80;

fn main() {
    println!("{MAX_TITLE_CHARS}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error: missing type for `const` item
 --> src\main.rs:1:22
  |
1 | const MAX_TITLE_CHARS = 80;
  |                      ^ help: provide a type for the constant: `: i32`

error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **何が起きたか:** `const` には型注釈が必須です。
- **直し方:** `help` は `: i32` を提案していますが、これは「とりあえずの案」です。文字数の上限なら長さを表す `usize` が適切なので、`const MAX_TITLE_CHARS: usize = 80;` と書きます。提案をそのまま貼るのではなく、意味を考えて型を選びましょう。

#### 警告: 使っていない mut

```rust
fn main() {
    let mut name = "RustNote";
    println!("{name}");
}
```

```text
warning: variable does not need to be mutable
 --> src\main.rs:2:9
  |
2 |     let mut name = "RustNote";
  |         ----^^^^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: `basics` (bin "basics") generated 1 warning (run `cargo fix --bin "basics" -p basics` to apply 1 suggestion)
```

- `warning`（警告）はエラーではないので、プログラムは実行されます（`RustNote` と表示されます）。
- 「この変数は変更されないので `mut` は不要です」という意味です。不要な `mut` は読む人を「どこで変わるの？」と迷わせるので、`help: remove this mut` に従って消しましょう。

> **つまずきポイント:** 「とりあえず全部 `mut` にしておけばエラーが出ない」と考えがちですが、逆効果です。まず `mut` なしで書き、E0384 が出たところで「本当に変わる値か？」を考えてから付けるのが Rust 流です。

### RustNote ではここで使う

この教材で作るメモアプリ **RustNote** では、ノートのタイトルを「1〜80文字」に制限します。上限 80 は**プログラム全体で共有する決まった値**なので `const` にします。また、フォームから届いたタイトルは前後に空白が付いていることがあるので、シャドーイングで「整えた版」を作り直します。

> **▼ このコードがやること（先に日本語で）:** 空白付きのタイトルを整え、その文字数を上限と並べて表示します。

```rust
const MAX_TITLE_CHARS: usize = 80;

fn main() {
    // フォームから届いたタイトル（前後に空白がついている）
    let title = "   牛乳を買う   ";
    // 前後の空白を取り除いた版を、同じ名前で作り直す（シャドーイング）
    let title = title.trim();
    // 文字数を数える（日本語も 1 文字を 1 と数える方法。UNIT 05 で詳しく）
    let chars = title.chars().count();
    println!("タイトル「{title}」は {chars} 文字です（上限 {MAX_TITLE_CHARS} 文字）");
}
```

**▼ 実行結果**

```text
タイトル「牛乳を買う」は 5 文字です（上限 80 文字）
```

この「整える → 数える → 上限と比べる」の流れは、UNIT 05 で文字数の数え方を、UNIT 09 で「範囲外ならエラーを返す」書き方を足して、`NoteTitle::parse` という検証関数に育てていきます。

### 練習問題

**問題1.** 次のコードは E0384 になります。（a）`mut` を使う方法、（b）シャドーイングを使う方法の2通りで直してください。

```rust
fn main() {
    let count = 1;
    println!("{count}");
    count = 2;
    println!("{count}");
}
```

**問題2.** `let price = "1200";` から始めて、シャドーイングを使って数（`u32`）に変換し、10% の税を足した金額 `税込み: 1320 円` を表示してください。

**問題3.** 次のコードを実行すると何が表示されるでしょうか。実行する前に予想してください。

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("内側: {x}");
    }
    println!("外側: {x}");
}
```

### 解答と解説

**解答1.**

```rust
// (a) mut を使う方法
fn main() {
    let mut count = 1;
    println!("{count}");
    count = 2;
    println!("{count}");
}
```

```rust
// (b) シャドーイングを使う方法
fn main() {
    let count = 1;
    println!("{count}");
    let count = 2;
    println!("{count}");
}
```

**▼ 実行結果**（どちらも同じ）

```text
1
2
```

- (a) は「同じ箱の中身を入れ替える」、(b) は「新しい箱を作って名札を移す」です。結果は同じですが、「数を数えていく」なら (a)、「別の意味の値に作り直す」なら (b) が自然です。

**解答2.**

```rust
fn main() {
    let price = "1200";
    // 文字列を数に変換し、同じ名前で作り直す
    let price: u32 = price.parse().expect("数字ではありません");
    // 10% の税を足す（整数のまま計算するため、先に 110 を掛けてから 100 で割る）
    let price = price * 110 / 100;
    println!("税込み: {price} 円");
}
```

**▼ 実行結果**

```text
税込み: 1320 円
```

- 整数の割り算は小数点以下が切り捨てられるので、`price / 100 * 110` の順だと端数が失われることがあります。**先に掛けてから割る**のが整数計算のコツです（UNIT 02）。

**解答3.**

```text
内側: 12
外側: 6
```

- 2行目で `x` は 6 の新しい箱になります。ブロックの中では `x * 2` = 12 の箱が作られますが、`}` で消えるので、外側では 6 が表示されます。

### まとめ

- `let` は値に名前を付ける。既定では**変更できない**ので、変わる値にだけ `mut` を付ける。
- シャドーイングは同じ名前で**新しい変数を作り直す**こと。型も変えられるので、段階的な変換に向く。
- プログラム全体で変わらない値は、型注釈付きの `const`（大文字の名前）にする。

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

## UNIT 03: 関数・文・式・戻り値

### この単元のゴール

- `fn` で関数を定義し、引数の型と戻り値の型を書ける
- **文**（値を持たない命令）と**式**（値になるもの）の違いを説明できる
- 「最後の式にセミコロンを付けない」と戻り値になる理由を説明でき、セミコロンの付けすぎ・付け忘れのエラーを直せる
- `return` で早めに関数を抜ける書き方（早期リターン）を使える

### たとえ話でつかむ

関数は「**自動販売機**」です。

- お金を入れる口（**引数**）には形が決まっていて、硬貨の口に紙幣は入りません。Rust の関数も、引数ごとに**型**が決まっています。
- ボタンを押すと、中の仕組み（**本体**）が動いて、商品（**戻り値**）が出てきます。何が出てくるかも、型として前もって決まっています。
- 自販機の外から見えるのは「何を入れると何が出てくるか」だけです。中身を知らなくても使えるように、関数の1行目（**シグネチャ**）だけ読めば使い方が分かるようになっています。

もう1つ、「文」と「式」のたとえです。

- **式**は「答えが出る計算」です。`3 + 4` は 7 という答え（値）になります。
- **文**は「作業の指示」です。`let x = 7;` は「x を作れ」という指示で、それ自体は答えを持ちません。
- 式の後ろに `;`（セミコロン）を付けると、その式は文になり、**答えは捨てられます**。セミコロンは「答えをゴミ箱に入れる」記号だと思ってください。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-03" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="26" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">関数の1行目（シグネチャ）を部品に分ける</text>
<rect x="40" y="46" width="50" height="40" rx="6" fill="#f8fafc" stroke="#475569"/>
<text x="65" y="72" text-anchor="middle" font-size="16" font-family="Consolas, monospace" fill="#0f172a">fn</text>
<rect x="100" y="46" width="62" height="40" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="131" y="72" text-anchor="middle" font-size="16" font-family="Consolas, monospace" fill="#c2410c">add</text>
<rect x="172" y="46" width="200" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="272" y="72" text-anchor="middle" font-size="16" font-family="Consolas, monospace" fill="#1e40af">(a: i32, b: i32)</text>
<rect x="382" y="46" width="90" height="40" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="427" y="72" text-anchor="middle" font-size="16" font-family="Consolas, monospace" fill="#047857">-&gt; i32</text>
<rect x="482" y="46" width="200" height="40" rx="6" fill="#f8fafc" stroke="#475569"/>
<text x="582" y="72" text-anchor="middle" font-size="16" font-family="Consolas, monospace" fill="#0f172a">{ a + b }</text>
<text x="65" y="108" text-anchor="middle" font-size="12" fill="#475569">関数を作る</text>
<text x="131" y="108" text-anchor="middle" font-size="12" fill="#c2410c">関数名</text>
<text x="272" y="108" text-anchor="middle" font-size="12" fill="#1e40af">引数（名前: 型）を , 区切り</text>
<text x="427" y="108" text-anchor="middle" font-size="12" fill="#047857">戻り値の型</text>
<text x="582" y="108" text-anchor="middle" font-size="12" fill="#475569">本体。最後の式が戻り値</text>
<line x1="20" y1="132" x2="700" y2="132" stroke="#e2e8f0"/>
<text x="360" y="158" text-anchor="middle" font-size="14" font-weight="700" fill="#0f172a">自動販売機として見ると</text>
<rect x="60" y="180" width="150" height="56" rx="10" fill="#eff6ff" stroke="#3b82f6"/>
<text x="135" y="204" text-anchor="middle" font-size="13" fill="#1e40af">入れるもの</text>
<text x="135" y="224" text-anchor="middle" font-size="14" font-weight="700" fill="#1e40af">3 と 4（i32）</text>
<rect x="285" y="172" width="150" height="72" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="204" text-anchor="middle" font-size="16" font-weight="700" fill="#c2410c">add</text>
<text x="360" y="226" text-anchor="middle" font-size="12" fill="#c2410c">a + b を計算</text>
<rect x="510" y="180" width="150" height="56" rx="10" fill="#ecfdf5" stroke="#10b981"/>
<text x="585" y="204" text-anchor="middle" font-size="13" fill="#047857">出てくるもの</text>
<text x="585" y="224" text-anchor="middle" font-size="14" font-weight="700" fill="#047857">7（i32）</text>
<line x1="210" y1="208" x2="280" y2="208" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-03)"/>
<line x1="435" y1="208" x2="505" y2="208" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-03)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図3-1: シグネチャを読めば「何を入れると何が出てくるか」が分かる</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-04" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="180" y="26" text-anchor="middle" font-size="15" font-weight="700" fill="#047857">最後の行が「式」のブロック</text>
<rect x="20" y="40" width="320" height="120" rx="10" fill="#ecfdf5" stroke="#10b981"/>
<text x="40" y="70" font-size="15" font-family="Consolas, monospace" fill="#0f172a">{</text>
<text x="60" y="98" font-size="15" font-family="Consolas, monospace" fill="#0f172a">let x = 3;</text>
<text x="200" y="98" font-size="12" fill="#64748b">← 文（値なし）</text>
<text x="60" y="126" font-size="15" font-family="Consolas, monospace" fill="#047857" font-weight="700">x + 1</text>
<text x="200" y="126" font-size="12" fill="#047857">← 式（値は 4）</text>
<text x="40" y="150" font-size="15" font-family="Consolas, monospace" fill="#0f172a">}</text>
<line x1="180" y1="162" x2="180" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-04)"/>
<rect x="90" y="200" width="180" height="40" rx="8" fill="#ffffff" stroke="#047857" stroke-width="2"/>
<text x="180" y="226" text-anchor="middle" font-size="14" font-weight="700" fill="#047857">ブロックの値 = 4</text>
<text x="540" y="26" text-anchor="middle" font-size="15" font-weight="700" fill="#c2410c">最後の行に ; を付けたブロック</text>
<rect x="380" y="40" width="320" height="120" rx="10" fill="#fff7ed" stroke="#f97316"/>
<text x="400" y="70" font-size="15" font-family="Consolas, monospace" fill="#0f172a">{</text>
<text x="420" y="98" font-size="15" font-family="Consolas, monospace" fill="#0f172a">let x = 3;</text>
<text x="560" y="98" font-size="12" fill="#64748b">← 文（値なし）</text>
<text x="420" y="126" font-size="15" font-family="Consolas, monospace" fill="#c2410c" font-weight="700">x + 1;</text>
<text x="560" y="126" font-size="12" fill="#c2410c">← ; で値を捨てた</text>
<text x="400" y="150" font-size="15" font-family="Consolas, monospace" fill="#0f172a">}</text>
<line x1="540" y1="162" x2="540" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-04)"/>
<rect x="430" y="200" width="220" height="40" rx="8" fill="#ffffff" stroke="#c2410c" stroke-width="2"/>
<text x="540" y="226" text-anchor="middle" font-size="14" font-weight="700" fill="#c2410c">ブロックの値 = ()（空）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図3-2: セミコロン1つで、ブロック（や関数）の値が 4 から () に変わる</div>
</div>

> **用語:** **()（ユニット）** … 「意味のある値はありません」を表す、中身が空の値です。型の名前も `()` です。何も返さない関数は、実は `()` を返しています。

### まず動かす最小コード

#### 例1: 関数を定義して呼び出す

> **▼ このコードがやること（先に日本語で）:** 2つの数を足して返す関数 `add` と、あいさつを表示するだけの関数 `greet` を作り、`main` から呼び出します。

```rust
// 2つの整数を受け取り、その合計を返す関数
fn add(a: i32, b: i32) -> i32 {
    // 最後の行にセミコロンを付けないと、その値が戻り値になる
    a + b
}

// 何も返さない関数（-> 戻り値の型 を書かない）
fn greet(name: &str) {
    println!("こんにちは、{name}さん");
}

fn main() {
    let total = add(3, 4);
    println!("3 + 4 = {total}");
    greet("Rust");
    // 関数の呼び出し結果を、そのまま別の関数の引数にしてもよい
    println!("入れ子: {}", add(add(1, 2), add(3, 4)));
}
```

```powershell
cargo run
```

**▼ 実行結果**

```text
3 + 4 = 7
こんにちは、Rustさん
入れ子: 10
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 引数と戻り値の型は省略できない

```rust
fn add(a: i32, b: i32) -> i32 {
```

- `a: i32` は「`a` という名前で i32 の値を受け取る」です。`let` と違い、**関数の引数の型は必ず書きます**。
- `-> i32` は「i32 の値を返す」です。矢印 `->` の後ろが戻り値の型です。
- 型を必ず書かせるのは、**関数の1行目だけで使い方が分かるようにする**ためです。中身を読まなくても、呼ぶ側は「i32 を2つ渡せば i32 が返る」と分かります。

##### 解説2: 最後の式が戻り値

```rust
    a + b
}
```

- 本体の最後の行 `a + b` には `;` がありません。これが「この値を返す」という意味です（図3-2 の左側と同じ）。
- 他の言語のように `return a + b;` と書くこともできますが、Rust では**最後の式で返す**のがふつうの書き方です。

##### 解説3: 何も返さない関数と `&str`

```rust
fn greet(name: &str) {
    println!("こんにちは、{name}さん");
}
```

- `->` を書かない関数は、意味のある値を返しません（正確には `()` を返します）。表示だけする関数などがこの形です。
- `&str` は「文字列を借りて読む」ための型です。関数で文字列を受け取るときの基本形で、UNIT 05 で詳しく学びます。

##### 解説4: 呼び出しは式

```rust
println!("入れ子: {}", add(add(1, 2), add(3, 4)));
```

- 関数の呼び出し `add(1, 2)` は値（3）になる**式**なので、そのまま別の関数の引数に書けます。内側から順に `add(3, 7)` = 10 と計算されます。

#### 例2: ブロックも if も式

> **▼ このコードがやること（先に日本語で）:** `{ }` で囲んだブロックの計算結果や、`if` の結果を変数に入れます。最後が文で終わるブロックの値が `()` になることも確かめます。

```rust
fn main() {
    // { } のブロックも「値になる式」。最後の行の値がブロック全体の値になる
    let area = {
        let width = 3;
        let height = 4;
        width * height
    };
    println!("面積: {area}");

    // if も式なので、結果をそのまま変数に入れられる
    let size = if area >= 10 { "大" } else { "小" };
    println!("大きさ: {size}");

    // 最後が「文」（セミコロン付き）のブロックは、() という「空の値」になる
    let nothing = {
        println!("ブロックの中で表示だけする");
    };
    println!("nothing の中身: {nothing:?}");
}
```

**▼ 実行結果**

```text
面積: 12
大きさ: 大
ブロックの中で表示だけする
nothing の中身: ()
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: ブロックで「作業用の変数」を閉じ込める

```rust
let area = {
    let width = 3;
    let height = 4;
    width * height
};
```

- ブロックの中の `width` と `height` は、面積を計算するためだけの変数です。ブロックを出ると消えるので、外の名前とぶつかりません。
- 最後の `width * height` の値（12）がブロックの値になり、`area` に入ります。`};` の `;` は `let area = ...;` という**文の終わり**のセミコロンです。

##### 解説2: `if` 式

```rust
let size = if area >= 10 { "大" } else { "小" };
```

- `if` も値を返す式です。条件が成り立てば `{ "大" }` の値、そうでなければ `{ "小" }` の値になります。
- 他の言語の「三項演算子（`条件 ? A : B`）」の役目を、Rust では `if` 式が担います。

##### 解説3: `()` が返るブロック

- `println!(...);` は文なので、このブロックには最後の式がありません。値がないブロックは `()` になります。`{nothing:?}` で表示すると `()` と出ます。
- もし `x + 1;` のように**計算結果を捨てる文**を書くと、Rust は次の警告で「その値、使わなくていいの？」と教えてくれます。

```text
warning: unused arithmetic operation that must be used
 --> src\main.rs:4:9
  |
4 |         x + 1;
  |         ^^^^^ the arithmetic operation produces a value
```

#### 例3: return で早めに抜ける

> **▼ このコードがやること（先に日本語で）:** 合計点と人数から平均点を計算します。人数が 0 のときは割り算ができないので、計算する前に 0 を返して関数を終えます。

```rust
// 平均点を返す。人数が 0 のときは割り算できないので、先に 0 を返して終わる
fn average(total: u32, count: u32) -> u32 {
    if count == 0 {
        // return は「ここで関数を終えて、この値を返す」
        return 0;
    }
    total / count
}

fn main() {
    println!("{}", average(270, 3));
    println!("{}", average(0, 0));
}
```

**▼ 実行結果**

```text
90
0
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 早期リターン

- `return 0;` は、その場で関数を終えて 0 を返します。後ろの `total / count` は実行されません。だから 0 での割り算（panic）が起きません。
- このように「**おかしな場合を先に片付けて、残りで本来の処理をする**」書き方を**早期リターン**（ガード節）と呼びます。条件分岐が深く入れ子にならず、読みやすくなります。
- 早期リターンには `return` を、ふつうの戻り値には最後の式を使う、という使い分けが Rust の慣習です。

#### 例4: 何も返さない関数の戻り値

> **▼ このコードがやること（先に日本語で）:** 表示するだけの関数の戻り値をあえて変数に受け取り、中身が `()` であることを確かめます。

```rust
fn log(message: &str) {
    println!("[log] {message}");
}

fn main() {
    // 戻り値の型を書かない関数は、実は () という値を返している
    let result = log("起動しました");
    println!("log の戻り値: {result:?}");
}
```

**▼ 実行結果**

```text
[log] 起動しました
log の戻り値: ()
```

- `fn log(message: &str)` は `fn log(message: &str) -> ()` と同じ意味です。`-> ()` は省略するのが決まりです。
- Rust では「すべての関数が何かの値を返す」と考えると、文と式の仕組みがすっきり理解できます。

### もう一歩: 実務に近い形

実際のプログラムでは、1つの大きな関数に全部書くのではなく、**名前の付いた小さな関数**に分けて組み合わせます。関数名がそのまま説明になり、1つずつテストもしやすくなります。

#### 小さな関数を組み合わせて支払い額を計算する

> **▼ このコードがやること（先に日本語で）:** 「税込みにする」「送料を決める」「総額を出す」の3つの関数を作り、組み合わせて支払い総額を計算します。税込み 5,000 円以上なら送料無料です。

```rust
// 税率（%）、送料無料になる金額、送料
const TAX_RATE_PERCENT: u32 = 10;
const FREE_SHIPPING_THRESHOLD: u32 = 5_000;
const SHIPPING_FEE: u32 = 500;

// 税込みの金額を返す
fn with_tax(price: u32) -> u32 {
    price * (100 + TAX_RATE_PERCENT) / 100
}

// 合計金額に応じた送料を返す。if の結果がそのまま戻り値になる
fn shipping_fee(subtotal: u32) -> u32 {
    if subtotal >= FREE_SHIPPING_THRESHOLD {
        0
    } else {
        SHIPPING_FEE
    }
}

// 小さな関数を組み合わせて、支払い総額を計算する
fn total(price: u32, quantity: u32) -> u32 {
    let subtotal = with_tax(price * quantity);
    subtotal + shipping_fee(subtotal)
}

fn main() {
    println!("1,000円 × 2個 = {} 円", total(1_000, 2));
    println!("1,000円 × 5個 = {} 円", total(1_000, 5));
}
```

**▼ 実行結果**

```text
1,000円 × 2個 = 2700 円
1,000円 × 5個 = 5500 円
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: if 式をそのまま戻り値にする

```rust
fn shipping_fee(subtotal: u32) -> u32 {
    if subtotal >= FREE_SHIPPING_THRESHOLD {
        0
    } else {
        SHIPPING_FEE
    }
}
```

- 関数の本体の最後にある `if` 式の値が、そのまま戻り値になります。`0` にも `SHIPPING_FEE` にもセミコロンを付けないのがポイントです。
- `if` と `else` のどちらの枝も `u32` を返しているので、型がそろっています（そろっていないとエラーになります。UNIT 04）。

##### 解説2: 関数の組み合わせで読みやすくする

```rust
fn total(price: u32, quantity: u32) -> u32 {
    let subtotal = with_tax(price * quantity);
    subtotal + shipping_fee(subtotal)
}
```

- `total` の中身は「税込みにして、送料を足す」と日本語のように読めます。計算の細かい部分は、それぞれの関数の中に隠れています。
- 税率が変わったら `with_tax`（と定数）だけ、送料の規則が変わったら `shipping_fee` だけを直せば済みます。

### よくあるエラーと読み方

#### エラー1: E0308 最後の式にセミコロンを付けてしまった

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b;
}

fn main() {
    println!("{}", add(1, 2));
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0308]: mismatched types
 --> src\main.rs:1:27
  |
1 | fn add(a: i32, b: i32) -> i32 {
  |    ---                    ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     a + b;
  |          - help: remove this semicolon to return this value

For more information about this error, try `rustc --explain E0308`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** `expected i32, found ()` は「i32 を返すと約束したのに、`()` が返っている」。`implicitly returns () as its body has no tail or return expression` は「本体に最後の式も `return` もないので、`()` を返していることになる」という意味です。
- **何が起きたか:** `a + b;` のセミコロンで値が捨てられました（図3-2 の右側）。
- **直し方:** `help: remove this semicolon to return this value`（この値を返すにはセミコロンを消してください）のとおり、`a + b` にします。**初心者がいちばんよく出会うエラーの1つ**です。

#### エラー2: E0308 戻り値の型を書き忘れた

```rust
fn add(a: i32, b: i32) {
    a + b
}

fn main() {
    println!("{}", add(1, 2));
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0308]: mismatched types
 --> src\main.rs:2:5
  |
1 | fn add(a: i32, b: i32) {
  |                       - help: try adding a return type: `-> i32`
2 |     a + b
  |     ^^^^^ expected `()`, found `i32`

error[E0277]: `()` doesn't implement `std::fmt::Display`
 --> src\main.rs:6:20
  |
6 |     println!("{}", add(1, 2));
  |               --   ^^^^^^^^^ `()` cannot be formatted with the default formatter
  |               |
  |               required by this formatting parameter
  |
  = help: the trait `std::fmt::Display` is not implemented for `()`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `basics` (bin "basics") due to 2 previous errors
```

- **読み方:** エラー1の逆です。`->` がないので「`()` を返す関数」とみなされ、最後の式 `a + b`（i32）と食い違いました。2つ目の E0277 は、その `()` を `{}` で表示しようとしたために出た**連鎖のエラー**です。
- **直し方:** `help: try adding a return type: -> i32` のとおり、`fn add(a: i32, b: i32) -> i32` にします。1つ目を直せば2つ目も消えます。

> **ヒント:** エラーが複数出たときは、**いちばん上のエラーから**直しましょう。後ろのエラーは、最初のエラーの巻き添えで出ていることがよくあります。

#### エラー3: 引数の型を書き忘れた

```rust
fn double(x) -> i32 {
    x * 2
}

fn main() {
    println!("{}", double(4));
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error: expected one of `:`, `@`, or `|`, found `)`
 --> src\main.rs:1:12
  |
1 | fn double(x) -> i32 {
  |            ^ expected one of `:`, `@`, or `|`
  |
help: if this is a parameter name, give it a type
  |
1 | fn double(x: TypeName) -> i32 {
  |            ++++++++++
help: if this is a type, explicitly ignore the parameter name
  |
1 | fn double(_: x) -> i32 {
  |           ++

error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** `expected one of :, @, or |, found )` は「`x` の後ろには `:` などが来るはずなのに `)` が来た」という**文法のエラー**です。エラー番号がないのは、型を調べる前の「文の形」の段階で止まったからです。
- **直し方:** 1つ目の `help`（引数名なら型を付けてください）に従い、`fn double(x: i32) -> i32` と書きます。`TypeName` は「ここに型名を書く」という意味の仮の名前です。

#### エラー4: E0061 引数の数が合わない

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", add(1));
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0061]: this function takes 2 arguments but 1 argument was supplied
 --> src\main.rs:6:20
  |
6 |     println!("{}", add(1));
  |                    ^^^--- argument #2 of type `i32` is missing
  |
note: function defined here
 --> src\main.rs:1:4
  |
1 | fn add(a: i32, b: i32) -> i32 {
  |    ^^^         ------
help: provide the argument
  |
6 |     println!("{}", add(1, /* i32 */));
  |                         +++++++++++

For more information about this error, try `rustc --explain E0061`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** 「この関数は引数を2つ取るのに、1つしか渡されていません」。`argument #2 of type i32 is missing` は「2番目の i32 の引数がない」。`note: function defined here` が関数の定義の場所を教えてくれます。
- **直し方:** `add(1, 2)` のように2つ渡します。Rust には「引数の省略」や「既定値付きの引数」はありません。

### RustNote ではここで使う

UNIT 01 で作った「タイトルの文字数チェック」を、**名前の付いた小さな関数**に分けます。RustNote の検証処理は、最終的にこの形を `NoteTitle::parse` という1つの入口にまとめたものになります（UNIT 09）。

> **▼ このコードがやること（先に日本語で）:** タイトルの空白を取り除き、文字数を数え、1〜80文字の範囲かどうかを判定して、結果を文章で返します。空白だけのタイトルと、81文字のタイトルも試します。

```rust
const MAX_TITLE_CHARS: usize = 80;

// 前後の空白を取り除いた版を返す
fn normalize(raw: &str) -> &str {
    raw.trim()
}

// 文字数を数える（日本語も 1 文字を 1 と数える）
fn char_count(text: &str) -> usize {
    text.chars().count()
}

// 1 以上 80 以下なら true を返す
fn is_valid_length(len: usize) -> bool {
    (1..=MAX_TITLE_CHARS).contains(&len)
}

// 判定結果を、人が読める文にして返す
fn describe(raw: &str) -> String {
    let title = normalize(raw);
    let len = char_count(title);
    if is_valid_length(len) {
        format!("OK: 「{title}」（{len}文字）")
    } else {
        format!("NG: {len}文字は 1〜{MAX_TITLE_CHARS} 文字の範囲外です")
    }
}

fn main() {
    println!("{}", describe("  買い物リスト  "));
    println!("{}", describe("    "));
    // "あ" を 81 回くり返した文字列で試す
    let long = "あ".repeat(81);
    println!("{}", describe(&long));
}
```

**▼ 実行結果**

```text
OK: 「買い物リスト」（6文字）
NG: 0文字は 1〜80 文字の範囲外です
NG: 81文字は 1〜80 文字の範囲外です
```

- `(1..=MAX_TITLE_CHARS)` は「1 以上 80 以下」という**範囲**を表します（`..=` は「終わりも含む」）。`.contains(&len)` は「その範囲に len が入っているか」を `true` / `false` で返します。`&` は「値を貸す」記号で、第3巻で詳しく学びます。
- `format!` は `println!` と同じ書き方で、表示する代わりに**文字列（`String`）を作って返す**マクロです。
- `describe(&long)` の `&` も「貸す」記号です。`repeat` で作った `String` を、`&str` を受け取る関数へ渡すときに付けます（UNIT 05）。

### 練習問題

**問題1.** 次の関数はコンパイルエラーになります。何が原因かを説明して、直してください。

```rust
fn square(x: i32) -> i32 {
    x * x;
}
```

**問題2.** 摂氏（℃）を華氏（℉）に変換する関数 `fn fahrenheit(celsius: f64) -> f64` を書き、0℃、36.5℃、100℃ を変換して表示してください。式は「℉ = ℃ × 9 ÷ 5 + 32」です。

**問題3.** 次のプログラムが表示する値を予想してください。

```rust
fn main() {
    let a = 10;
    let b = {
        let t = a * 2;
        t + 1
    };
    let c = if b > 20 { b - 20 } else { 0 };
    println!("{a} {b} {c}");
}
```

### 解答と解説

**解答1.** 最後の `x * x;` のセミコロンで値が捨てられ、関数が `()` を返すことになるため E0308 になります。セミコロンを消します。

```rust
fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    println!("{}", square(7));
}
```

**▼ 実行結果**

```text
49
```

**解答2.**

```rust
// 摂氏（℃）を華氏（℉）に変換する
fn fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn main() {
    println!("0℃ = {}℉", fahrenheit(0.0));
    println!("36.5℃ = {:.1}℉", fahrenheit(36.5));
    println!("100℃ = {}℉", fahrenheit(100.0));
}
```

**▼ 実行結果**

```text
0℃ = 32℉
36.5℃ = 97.7℉
100℃ = 212℉
```

- `f64` どうしの計算なので、`9` ではなく `9.0` のように小数で書きます（`celsius * 9` だと UNIT 02 のエラー4と同じ E0277 になります）。

**解答3.**

```text
10 21 1
```

- `b` のブロックは `t = 20` を作り、最後の式 `t + 1` = 21 が値になります。`c` は `21 > 20` なので `21 - 20` = 1 です。

### まとめ

- 関数は `fn 名前(引数: 型) -> 戻り値の型 { 本体 }`。引数と戻り値の型は省略できない。
- 値になるのが**式**、値を持たない命令が**文**。最後の式（セミコロンなし）が、ブロックや関数の値になる。
- 異常な場合は `return` で早めに抜け、本来の処理は最後の式で返す。

## UNIT 04: if・loop・while・for・match

### この単元のゴール

- `if` / `else if` / `else` で条件分岐を書け、`if` を式として値を受け取れる
- `loop` / `while` / `for` の3つのくり返しを使い分けられ、`break` と `continue` を使える
- `match` で値を「形」ごとに振り分けられ、範囲・`|`・`_`・ガードを使える
- `match` が**すべての場合を網羅しているか**をコンパイラが調べる理由を説明できる

### たとえ話でつかむ

- **if** は「分かれ道の標識」です。「80点以上ならこっち、60点以上ならあっち、それ以外は向こう」と、上から順に標識を見て最初に合った道へ進みます。
- **loop** は「回転寿司のレーン」です。誰かが止める（`break`）まで、ずっと回り続けます。
- **while** は「お腹が空いている間は食べ続ける」です。毎回、食べる前に条件（まだ空いている？）を確かめます。
- **for** は「出席簿を上から順に読み上げる」です。名簿（範囲や配列）の最後まで来たら自然に終わります。
- **match** は「郵便局の仕分け棚」です。手紙（値）を宛先ごとの棚（腕）に振り分けます。**どの棚にも入らない手紙があってはいけない**ので、Rust は「全部の宛先に棚がありますか？」をコンパイル時に確かめます。棚が足りないときは「その他」の棚（`_`）を用意します。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-05" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="10" width="225" height="280" rx="12" fill="#fff7ed" stroke="#f97316"/>
<text x="122" y="38" text-anchor="middle" font-size="16" font-weight="700" fill="#c2410c">loop</text>
<rect x="47" y="70" width="150" height="40" rx="8" fill="#ffffff" stroke="#c2410c"/>
<text x="122" y="95" text-anchor="middle" font-size="13" fill="#0f172a">中身を実行</text>
<polygon points="122,140 192,170 122,200 52,170" fill="#ffffff" stroke="#c2410c"/>
<text x="122" y="175" text-anchor="middle" font-size="12" fill="#0f172a">break した？</text>
<line x1="122" y1="110" x2="122" y2="136" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-05)"/>
<path d="M52,170 L30,170 L30,90 L43,90" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-05)"/>
<text x="40" y="190" font-size="12" fill="#64748b">いいえ</text>
<line x1="122" y1="200" x2="122" y2="236" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-05)"/>
<text x="130" y="222" font-size="12" fill="#64748b">はい</text>
<text x="122" y="258" text-anchor="middle" font-size="13" fill="#c2410c">抜ける（値も返せる）</text>
<text x="122" y="280" text-anchor="middle" font-size="12" fill="#64748b">回数が決まっていないとき</text>
<rect x="247" y="10" width="225" height="280" rx="12" fill="#eff6ff" stroke="#3b82f6"/>
<text x="360" y="38" text-anchor="middle" font-size="16" font-weight="700" fill="#1e40af">while 条件</text>
<polygon points="360,60 430,90 360,120 290,90" fill="#ffffff" stroke="#1e40af"/>
<text x="360" y="95" text-anchor="middle" font-size="12" fill="#0f172a">条件は true？</text>
<line x1="360" y1="120" x2="360" y2="146" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-05)"/>
<text x="368" y="138" font-size="12" fill="#64748b">はい</text>
<rect x="285" y="150" width="150" height="40" rx="8" fill="#ffffff" stroke="#1e40af"/>
<text x="360" y="175" text-anchor="middle" font-size="13" fill="#0f172a">中身を実行</text>
<path d="M285,170 L265,170 L265,90 L286,90" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-05)"/>
<line x1="430" y1="90" x2="455" y2="90" stroke="#475569" stroke-width="2"/>
<line x1="455" y1="90" x2="455" y2="236" stroke="#475569" stroke-width="2"/>
<line x1="455" y1="236" x2="420" y2="236" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-05)"/>
<text x="440" y="80" font-size="12" fill="#64748b">いいえ</text>
<text x="360" y="242" text-anchor="middle" font-size="13" fill="#1e40af">抜ける</text>
<text x="360" y="280" text-anchor="middle" font-size="12" fill="#64748b">条件で止めたいとき</text>
<rect x="484" y="10" width="226" height="280" rx="12" fill="#ecfdf5" stroke="#10b981"/>
<text x="597" y="38" text-anchor="middle" font-size="16" font-weight="700" fill="#047857">for x in 並び</text>
<rect x="502" y="62" width="40" height="30" rx="4" fill="#ffffff" stroke="#047857"/>
<text x="522" y="82" text-anchor="middle" font-size="13" fill="#0f172a">1</text>
<rect x="552" y="62" width="40" height="30" rx="4" fill="#ffffff" stroke="#047857"/>
<text x="572" y="82" text-anchor="middle" font-size="13" fill="#0f172a">2</text>
<rect x="602" y="62" width="40" height="30" rx="4" fill="#ffffff" stroke="#047857"/>
<text x="622" y="82" text-anchor="middle" font-size="13" fill="#0f172a">3</text>
<rect x="652" y="62" width="40" height="30" rx="4" fill="#f8fafc" stroke="#94a3b8" stroke-dasharray="4 3"/>
<text x="672" y="82" text-anchor="middle" font-size="12" fill="#94a3b8">終</text>
<line x1="522" y1="92" x2="522" y2="146" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-05)"/>
<text x="530" y="124" font-size="12" fill="#64748b">1つ取り出して x に</text>
<rect x="502" y="150" width="190" height="40" rx="8" fill="#ffffff" stroke="#047857"/>
<text x="597" y="175" text-anchor="middle" font-size="13" fill="#0f172a">中身を実行 → 次の要素へ</text>
<text x="597" y="230" text-anchor="middle" font-size="13" fill="#047857">要素がなくなったら終わる</text>
<text x="597" y="280" text-anchor="middle" font-size="12" fill="#64748b">範囲・配列を順に処理するとき</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-1: 3つのくり返し。迷ったら for、条件で止めるなら while、それ以外は loop</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-06" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="24" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">match code { ... } に 404 を入れると</text>
<rect x="20" y="120" width="110" height="56" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="75" y="146" text-anchor="middle" font-size="12" fill="#c2410c">code の値</text>
<text x="75" y="166" text-anchor="middle" font-size="18" font-weight="700" fill="#c2410c">404</text>
<line x1="130" y1="148" x2="166" y2="148" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-06)"/>
<rect x="170" y="40" width="250" height="34" rx="6" fill="#f8fafc" stroke="#94a3b8"/>
<text x="185" y="62" font-size="14" font-family="Consolas, monospace" fill="#475569">200 =&gt;</text>
<text x="405" y="62" text-anchor="end" font-size="12" fill="#94a3b8">合わない ↓</text>
<rect x="170" y="80" width="250" height="34" rx="6" fill="#f8fafc" stroke="#94a3b8"/>
<text x="185" y="102" font-size="14" font-family="Consolas, monospace" fill="#475569">201 | 202 =&gt;</text>
<text x="405" y="102" text-anchor="end" font-size="12" fill="#94a3b8">合わない ↓</text>
<rect x="170" y="120" width="250" height="34" rx="6" fill="#f8fafc" stroke="#94a3b8"/>
<text x="185" y="142" font-size="14" font-family="Consolas, monospace" fill="#475569">300..=399 =&gt;</text>
<text x="405" y="142" text-anchor="end" font-size="12" fill="#94a3b8">合わない ↓</text>
<rect x="170" y="160" width="250" height="34" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="185" y="182" font-size="14" font-family="Consolas, monospace" font-weight="700" fill="#047857">404 =&gt;</text>
<text x="405" y="182" text-anchor="end" font-size="12" font-weight="700" fill="#047857">合った！</text>
<rect x="170" y="200" width="250" height="34" rx="6" fill="#ffffff" stroke="#e2e8f0" stroke-dasharray="4 3"/>
<text x="185" y="222" font-size="14" font-family="Consolas, monospace" fill="#94a3b8">400..=499 =&gt;</text>
<rect x="170" y="240" width="250" height="34" rx="6" fill="#ffffff" stroke="#e2e8f0" stroke-dasharray="4 3"/>
<text x="185" y="262" font-size="14" font-family="Consolas, monospace" fill="#94a3b8">_ =&gt;</text>
<text x="405" y="262" text-anchor="end" font-size="12" fill="#94a3b8">それ以外すべて</text>
<line x1="420" y1="177" x2="476" y2="177" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-06)"/>
<rect x="480" y="150" width="220" height="56" rx="10" fill="#ecfdf5" stroke="#047857" stroke-width="2"/>
<text x="590" y="174" text-anchor="middle" font-size="12" fill="#047857">match 全体の値</text>
<text x="590" y="195" text-anchor="middle" font-size="15" font-weight="700" fill="#047857">"見つかりません"</text>
<text x="590" y="236" text-anchor="middle" font-size="12" fill="#64748b">合った腕より下は調べない</text>
<text x="590" y="256" text-anchor="middle" font-size="12" fill="#64748b">（400..=499 にも合うが使われない）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-2: match は上から順に比べ、最初に合った腕だけを実行する。どの値もどこかの腕に合う必要がある</div>
</div>

### まず動かす最小コード

#### 例1: if / else if / else と if 式

> **▼ このコードがやること（先に日本語で）:** 点数に応じて3通りのメッセージを出し分けます。続いて、`if` の結果を変数に入れて「偶数か奇数か」を表示します。

```rust
fn main() {
    let score = 72;
    // 条件を上から順に調べ、最初に当てはまった枝だけを実行する
    if score >= 80 {
        println!("よくできました");
    } else if score >= 60 {
        println!("合格です");
    } else {
        println!("もう一歩です");
    }

    // if は式なので、結果を変数に入れられる（どの枝も同じ型を返す）
    let parity = if score % 2 == 0 { "偶数" } else { "奇数" };
    println!("{score} は{parity}");
}
```

```powershell
cargo run
```

**▼ 実行結果**

```text
合格です
72 は偶数
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 条件は上から順に、最初の1つだけ

```rust
if score >= 80 {
    println!("よくできました");
} else if score >= 60 {
    println!("合格です");
} else {
    println!("もう一歩です");
}
```

- 72 は `>= 80` に当てはまらず、`>= 60` に当てはまったので「合格です」だけが表示されました。**当てはまった時点で残りは調べません**。
- 条件のまわりに `( )` は不要です（付けると警告が出ます）。一方、`{ }` は1行でも省略できません。
- 比べる演算子は `==`（等しい）、`!=`（等しくない）、`<` `<=` `>` `>=` です。

##### 解説2: if 式の型はそろえる

```rust
let parity = if score % 2 == 0 { "偶数" } else { "奇数" };
```

- `score % 2` は2で割った余りです。0なら偶数です。
- `if` を式として使うときは、**すべての枝が同じ型の値**を返す必要があります（ここではどちらも文字列）。また、値を受け取るなら `else` も必須です（下の「よくあるエラー」で確かめます）。

#### 例2: loop と break で値を返す

> **▼ このコードがやること（先に日本語で）:** 1 から始めて2倍をくり返し、100 を超えた最初の数を見つけます。見つけた数を `break` で持ち帰ります。

```rust
fn main() {
    let mut n = 1;
    // loop は break するまで、いつまでもくり返す
    let result = loop {
        n *= 2;
        // 100 を超えたら、その値を持って loop を抜ける
        if n > 100 {
            break n;
        }
    };
    println!("100 を超えた最初の 2 の累乗: {result}");
}
```

**▼ 実行結果**

```text
100 を超えた最初の 2 の累乗: 128
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `break 値`

- `loop { ... }` は、`break` に出会うまで中身を何度でもくり返します。
- `break n;` は「ループを抜け、`n` を `loop` 式全体の値にする」という意味です。だから `let result = loop { ... };` と受け取れます。2, 4, 8, 16, 32, 64, 128 と進み、128 で抜けました。
- 「何回くり返すか前もって分からない」処理（再試行、入力待ちなど）に向いています。

#### 例3: while で条件の間くり返す

> **▼ このコードがやること（先に日本語で）:** 残り回数を表示しながら1ずつ減らし、0 になったらくり返しを終えて「発射！」と表示します。

```rust
fn main() {
    let mut remaining = 3;
    // while は「条件が true の間」くり返す
    while remaining > 0 {
        println!("残り {remaining}");
        remaining -= 1;
    }
    println!("発射！");
}
```

**▼ 実行結果**

```text
残り 3
残り 2
残り 1
発射！
```

- 毎回、中身を実行する**前に**条件 `remaining > 0` を調べます。最初から条件が `false` なら一度も実行されません。
- 条件の中の変数（`remaining`）を中で変えないと、永遠に終わらないので注意しましょう（止めるときは `Ctrl + C`）。

#### 例4: for で範囲や配列を順に処理する

> **▼ このコードがやること（先に日本語で）:** 数の範囲を順番に、逆順に、飛ばしながら取り出して表示します。最後に配列の果物を1つずつ表示します。

```rust
fn main() {
    // 0..3 は 0, 1, 2（終わりの 3 は含まない）
    for i in 0..3 {
        println!("i = {i}");
    }

    // 1..=3 は 1, 2, 3（= を付けると終わりも含む）
    for i in 1..=3 {
        // print! は改行しない表示
        print!("{i} ");
    }
    // 改行だけを出す
    println!();

    // .rev() で逆順にし、.step_by(3) で 3 つおきに取り出す
    for i in (0..10).rev().step_by(3) {
        print!("{i} ");
    }
    println!();

    // 配列の要素を先頭から1つずつ取り出す
    let fruits = ["りんご", "みかん", "ぶどう"];
    for fruit in fruits {
        println!("{fruit}");
    }
}
```

**▼ 実行結果**

```text
i = 0
i = 1
i = 2
1 2 3 
9 6 3 0 
りんご
みかん
ぶどう
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 範囲 `0..3` と `1..=3`

- `0..3` は「0 から 3 の**手前**まで」（0, 1, 2）、`1..=3` は「1 から 3 **まで**」（1, 2, 3）です。`=` の有無で終わりを含むかが変わります。
- 「0 から n 個」なら `0..n`、「1 から n まで」なら `1..=n` と覚えると迷いません。

##### 解説2: `.rev()` と `.step_by()`

- `(0..10).rev()` は 9, 8, …, 0 の逆順、`.step_by(3)` は「3つおき」です。組み合わせると 9, 6, 3, 0 になりました。
- このように、範囲の後ろに `.` で加工をつなげられます。この仕組み（イテレータ）は UNIT 10 で詳しく学びます。

##### 解説3: 配列を for で回す

- `for fruit in fruits` は、配列の要素を先頭から1つずつ `fruit` に入れてくり返します。添え字（何番目か）を自分で数える必要がないので、**数え間違いによるバグが起きません**。
- Rust では、`while` で添え字を数えるより `for` を使うのが基本です。

#### 例5: match で値を振り分ける

> **▼ このコードがやること（先に日本語で）:** HTTP のステータスコード（サーバーの返事の番号）を、意味を表す日本語に振り分けます。1つの値・複数の値・範囲・それ以外の4種類の書き方を使います。

```rust
fn main() {
    for code in [200, 202, 301, 404, 418, 503, 999] {
        // match は値を上から順に比べ、最初に合った「腕」を実行する
        let label = match code {
            // 1つの値
            200 => "成功",
            // | は「または」
            201 | 202 => "受け付けました",
            // ..= は範囲（300 以上 399 以下）
            300..=399 => "リダイレクト",
            404 => "見つかりません",
            400..=499 => "リクエストの誤り",
            500..=599 => "サーバーの誤り",
            // _ は「ここまでのどれにも当てはまらない、それ以外すべて」
            _ => "不明なコード",
        };
        println!("{code}: {label}");
    }
}
```

**▼ 実行結果**

```text
200: 成功
202: 受け付けました
301: リダイレクト
404: 見つかりません
418: リクエストの誤り
503: サーバーの誤り
999: 不明なコード
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 腕（アーム）の書き方

```rust
let label = match code {
    200 => "成功",
    201 | 202 => "受け付けました",
    300..=399 => "リダイレクト",
    _ => "不明なコード",
};
```

- `パターン => 値,` の1行を**腕**（アーム）と呼びます。`match` は上から順に比べ、最初に合った腕の値が `match` 全体の値になります（図4-2）。
- `201 | 202` は「201 または 202」、`300..=399` は「300 以上 399 以下」、`_` は「それ以外すべて」です。
- 404 は `404 =>` に合うので「見つかりません」。その下の `400..=499` にも当てはまりますが、もう調べません。だから**細かい条件を上、大まかな条件を下**に書きます。418 は `404` に合わず `400..=499` に合いました。

##### 解説2: なぜ `_` が必要なのか

- `code` は整数なので、999 のような想定外の値も入り得ます。Rust の `match` は**すべての値がどこかの腕に合うこと（網羅性）**を求めるので、最後に `_` を置いて「それ以外」を引き受けます。
- 網羅していないとコンパイルエラーになります（下の「よくあるエラー」）。これのおかげで「場合分けの書き忘れ」が実行前に見つかります。

#### 例6: ガード付きの match とラベル付き break

> **▼ このコードがやること（先に日本語で）:** 気温を「氷点下」「真夏日」などに分類します。範囲では書きにくい条件は `if` を添えて書きます。後半は、2重のループの内側から外側のループごと抜け出します。

```rust
fn main() {
    // match の腕に if（ガード）を付けて、条件を足せる
    for temp in [-3, 0, 18, 35] {
        let feeling = match temp {
            t if t < 0 => "氷点下",
            0 => "ちょうど 0 度",
            t if t >= 30 => "真夏日",
            _ => "過ごしやすい",
        };
        println!("{temp}度: {feeling}");
    }

    // 入れ子のループで、外側のループごと抜ける（ラベル付き break）
    'outer: for row in 1..=3 {
        for col in 1..=3 {
            if row * col == 4 {
                println!("{row} × {col} = 4 を見つけたので終了");
                break 'outer;
            }
            println!("{row} × {col} = {}", row * col);
        }
    }
}
```

**▼ 実行結果**

```text
-3度: 氷点下
0度: ちょうど 0 度
18度: 過ごしやすい
35度: 真夏日
1 × 1 = 1
1 × 2 = 2
1 × 3 = 3
2 × 1 = 2
2 × 2 = 4 を見つけたので終了
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: マッチガード `t if 条件`

```rust
t if t < 0 => "氷点下",
```

- `t` は「どんな値にも合い、その値に `t` という名前を付ける」パターンです。後ろの `if t < 0` が追加の条件（**ガード**）で、両方が満たされたときだけこの腕が選ばれます。

##### 解説2: ラベル付き break

```rust
'outer: for row in 1..=3 {
    for col in 1..=3 {
        if row * col == 4 {
            break 'outer;
        }
    }
}
```

- ふつうの `break` は**いちばん内側の**ループだけを抜けます。`'outer:` のように `'` で始まる名前（ラベル）を外側のループに付けておくと、`break 'outer;` で外側まで一気に抜けられます。
- 同じように `continue 'outer;` と書くと「外側のループの次の回へ進む」になります。

### もう一歩: 実務に近い形

バックエンドでは、外部のサービスへの問い合わせが一時的に失敗することがあります。そのとき「成功するまで、上限回数まで再試行する」処理を、`loop` と `match` と `continue` で書いてみます。

#### 一時的な失敗を再試行する

> **▼ このコードがやること（先に日本語で）:** にせ物の問い合わせ関数を最大5回呼びます。503（一時的に使えない）なら再試行、2xx なら成功、それ以外ならあきらめて、最終結果を表示します。

```rust
const MAX_ATTEMPTS: u32 = 5;

// サーバーに問い合わせた結果のステータスコードを返す（練習用のにせ物）
fn fake_request(attempt: u32) -> u16 {
    match attempt {
        1 | 2 => 503,
        3 => 200,
        _ => 500,
    }
}

fn main() {
    let mut attempt = 0;
    let outcome = loop {
        attempt += 1;
        let status = fake_request(attempt);
        match status {
            // 2xx なら成功。結果の文を持って loop を抜ける
            200..=299 => break format!("{attempt} 回目で成功（{status}）"),
            // 503 は一時的な失敗。回数が残っていれば、もう一度くり返す
            503 if attempt < MAX_ATTEMPTS => {
                println!("{attempt} 回目: {status} のため再試行します");
                continue;
            }
            // それ以外はあきらめる
            _ => break format!("{attempt} 回目で中止（{status}）"),
        }
    };
    println!("{outcome}");
}
```

**▼ 実行結果**

```text
1 回目: 503 のため再試行します
2 回目: 503 のため再試行します
3 回目で成功（200）
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: match の腕から break / continue する

```rust
200..=299 => break format!("{attempt} 回目で成功（{status}）"),
503 if attempt < MAX_ATTEMPTS => {
    println!("{attempt} 回目: {status} のため再試行します");
    continue;
}
_ => break format!("{attempt} 回目で中止（{status}）"),
```

- 腕の右側には `{ }` で複数の文も書けます。`continue` は「この回の残りを飛ばして、次の回へ進む」です。
- 成功・中止のどちらの腕も `break 文字列` なので、`loop` 全体の値は `String` にそろいます。
- 503 でも試行回数が上限に達していればガードが成り立たないので、最後の `_` の腕（中止）に落ちます。これで**無限ループを防いでいます**。

> **注意:** 本物の再試行では、すぐにやり直すのではなく少し待ち時間を入れます（第5巻の async で扱います）。

### よくあるエラーと読み方

#### エラー1: E0308 if と else の型が違う

```rust
fn main() {
    let n = 5;
    let label = if n > 0 { "正の数" } else { 0 };
    println!("{label}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0308]: `if` and `else` have incompatible types
 --> src\main.rs:3:43
  |
3 |     let label = if n > 0 { "正の数" } else { 0 };
  |                            --------          ^ expected `&str`, found integer
  |                            |
  |                            expected because of this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** 「`if` と `else` の型が合いません」。`expected because of this` が指す `"正の数"`（文字列）を見て「文字列が来るはずだ」と判断したのに、`else` 側は整数でした。
- **直し方:** どちらも同じ型にします。例えば `else { "0以下" }` にします。

#### エラー2: E0317 値として使う if に else がない

```rust
fn main() {
    let n = 5;
    let label = if n > 0 { "正の数" };
    println!("{label}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0317]: `if` may be missing an `else` clause
 --> src\main.rs:3:17
  |
3 |     let label = if n > 0 { "正の数" };
  |                 ^^^^^^^^^^^--------^^
  |                 |          |
  |                 |          found here
  |                 expected `&str`, found `()`
  |
  = note: `if` expressions without `else` evaluate to `()`
  = help: consider adding an `else` block that evaluates to the expected type

For more information about this error, try `rustc --explain E0317`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** 「`if` に `else` がないかもしれません」。`note` にあるとおり、`else` のない `if` は、条件が `false` のとき `()` になります。すると `label` が文字列なのか `()` なのか決まりません。
- **直し方:** `else { "0以下" }` を足して、どちらの場合も文字列になるようにします。

#### エラー3: E0004 match がすべての場合を網羅していない

```rust
fn main() {
    let code: u16 = 404;
    let label = match code {
        200..=299 => "成功",
        400..=499 => "リクエストの誤り",
        500..=599 => "サーバーの誤り",
    };
    println!("{label}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0004]: non-exhaustive patterns: `0_u16..=199_u16`, `300_u16..=399_u16` and `600_u16..=u16::MAX` not covered
 --> src\main.rs:3:23
  |
3 |     let label = match code {
  |                       ^^^^ patterns `0_u16..=199_u16`, `300_u16..=399_u16` and `600_u16..=u16::MAX` not covered
  |
  = note: the matched value is of type `u16`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
  |
6 ~         500..=599 => "サーバーの誤り",
7 ~         0_u16..=199_u16 | 300_u16..=399_u16 | 600_u16..=u16::MAX => todo!(),
  |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** `non-exhaustive patterns`（網羅されていないパターン）は「この `match` では扱われていない値があります」。具体的に `0_u16..=199_u16`（0〜199）、`300_u16..=399_u16`（300〜399）、`600_u16..=u16::MAX`（600〜最大値）が抜けていると教えてくれています。
- **直し方:** 抜けている範囲に意味があるなら腕を足し、なければ最後に `_ => "その他",` を足します。`help` が提案している `todo!()` は「あとで書く」という仮置きで、**実行してそこに来ると panic します**。そのまま残さないようにしましょう。

#### エラー4: E0308 条件に整数を書いた

```rust
fn main() {
    let unread = 3;
    if unread {
        println!("未読があります");
    }
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0308]: mismatched types
 --> src\main.rs:3:8
  |
3 |     if unread {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **何が起きたか:** JavaScript や Python では「0 以外の数は真」とみなされますが、Rust の `if` の条件は**必ず `bool`** です。
- **直し方:** 意図をそのまま比較で書きます: `if unread > 0 { ... }`。

> **つまずきポイント:** 「`if x` で済むのに面倒」と感じるかもしれません。しかし `if count` が「0 でない」なのか「存在する」なのかは読む人によって解釈が分かれます。`unread > 0` と書けば誤解の余地がありません。

### RustNote ではここで使う

RustNote のノート一覧は20件ずつページに分け、ページの下に「← 前へ」「次へ →」のリンクを出します。どちらのリンクを出すかは「前のページがあるか」「次のページがあるか」の2つの `bool` の組み合わせで決まるので、`match` で4通りすべてを書き切ります。

> **▼ このコードがやること（先に日本語で）:** ノートが45件あるときのページ数を計算し、各ページにどのページ送りリンクを出すかを表示します。

```rust
const PER_PAGE: u32 = 20;

fn main() {
    // ノートが全部で 45 件あるとする
    let total_notes: u32 = 45;
    // 必要なページ数（端数は切り上げ。45 ÷ 20 → 3）
    let total_pages = total_notes.div_ceil(PER_PAGE);

    for page in 1..=total_pages {
        let has_prev = page > 1;
        let has_next = page < total_pages;
        // 2つの bool の組み合わせで、ページ送りの表示を決める
        let nav = match (has_prev, has_next) {
            (false, false) => "（ページ送りなし）",
            (false, true) => "次へ →",
            (true, true) => "← 前へ | 次へ →",
            (true, false) => "← 前へ",
        };
        println!("{page}/{total_pages} ページ: {nav}");
    }
}
```

**▼ 実行結果**

```text
1/3 ページ: 次へ →
2/3 ページ: ← 前へ | 次へ →
3/3 ページ: ← 前へ
```

- `total_notes.div_ceil(PER_PAGE)` は「割り算して、端数を切り上げる」メソッドです。45 ÷ 20 = 2.25 → 3 ページです。
- `(has_prev, has_next)` は2つの値を組にした**タプル**です（UNIT 06）。`match` で組のまま比べると、`true` / `false` の4通りを表のように並べて書けます。1つでも書き忘れると E0004 で教えてもらえるので、**表示の出し忘れが起きません**。

### 練習問題

**問題1.** 1 から 15 までの数について、3 の倍数なら `Fizz`、5 の倍数なら `Buzz`、両方の倍数なら `FizzBuzz`、それ以外は数そのものを、空白区切りで1行に表示してください。`match` と `(n % 3, n % 5)` の組を使ってみましょう。

**問題2.** 1 + 2 + 3 + … と足していき、合計が初めて 100 以上になるのは何まで足したときかを、`loop` と `break 値` で求めてください。

**問題3.** 「よくあるエラー」のエラー3のコードを、`code` が 302 のとき `その他` と表示されるように直してください。

### 解答と解説

**解答1.**

```rust
fn main() {
    for n in 1..=15 {
        // 3 で割った余りと 5 で割った余りの組で分ける
        let text = match (n % 3, n % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => n.to_string(),
        };
        print!("{text} ");
    }
    println!();
}
```

**▼ 実行結果**

```text
1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz 
```

- `(0, 0)` を**いちばん上**に書くのがポイントです。`(0, _)` を先に書くと、15 も `Fizz` になってしまいます。`_` は「何でもよい」なので、`(0, _)` は「3 で割り切れれば、5 の余りは何でもよい」です。
- `.to_string()` は数や文字列から `String` を作るメソッドです。すべての腕を `String` にそろえるために使っています（UNIT 05）。

**解答2.**

```rust
fn main() {
    let mut n = 0;
    let mut sum = 0;
    let answer = loop {
        n += 1;
        sum += n;
        if sum >= 100 {
            break n;
        }
    };
    println!("1 から {answer} まで足すと {sum} で、初めて 100 以上になる");
}
```

**▼ 実行結果**

```text
1 から 14 まで足すと 105 で、初めて 100 以上になる
```

**解答3.** 最後に `_` の腕を足します。

```rust
fn main() {
    let code: u16 = 302;
    let label = match code {
        200..=299 => "成功",
        400..=499 => "リクエストの誤り",
        500..=599 => "サーバーの誤り",
        _ => "その他",
    };
    println!("{label}");
}
```

**▼ 実行結果**

```text
その他
```

### まとめ

- `if` は式なので値を返せる。そのときは全部の枝の型をそろえ、`else` も書く。条件は必ず `bool`。
- くり返しは、順に処理するなら `for`、条件で止めるなら `while`、それ以外は `loop`（`break 値` で結果を返せる）。
- `match` は上から順に比べ、最初に合った腕を実行する。すべての場合を網羅する必要があり、書き忘れはコンパイラが E0004 で教えてくれる。

## UNIT 05: String・str・UTF-8

### この単元のゴール

- 書き足せる文字列 `String` と、文字列を借りて読む `&str` の違いを説明できる
- 関数の引数は `&str` で受け取る、という基本形を使える
- 日本語の文字列で `len()`（バイト数）と `chars().count()`（文字数）が違う理由を UTF-8 で説明できる
- `s[0]` が書けない理由を知り、`chars()` や `get` で安全に文字を取り出せる

### たとえ話でつかむ

- **`String`** は「**自分のノート**」です。自分が持ち主なので、好きなだけページを書き足したり、消したりできます。
- **`&str`**（エスアンド・ストル、または「ストル参照」）は、ノートの一部を囲んだ「**のぞき窓**」です。窓からは中身を読めますが、窓は持ち主ではないので書き換えられません。ノート全体を囲むことも、1行だけを囲むこともできます。
- 文字列リテラル `"こんにちは"` は、「**本に印刷された文**」です。プログラムに最初から埋め込まれていて書き換えられないので、`&str`（のぞき窓）として扱います。

もう1つ、UTF-8（ユーティーエフエイト：文字をバイトの並びに変換する決まり）のたとえです。

- 文字列は、中では**バイト（0〜255 の数）の列**として保存されています。英字は1文字1バイトですが、日本語のひらがな・漢字は1文字3バイト、絵文字は4バイトを使います。
- 「1文字＝1マス」ではなく「**文字によって使うマス数が違う原稿用紙**」なので、「3マス目」が文字の途中ということが起こります。だから Rust は `s[0]` のような「○マス目の文字」という取り出し方をさせません。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-07" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="110" y="26" text-anchor="middle" font-size="14" font-weight="700" fill="#0f172a">スタック（変数そのもの）</text>
<text x="490" y="26" text-anchor="middle" font-size="14" font-weight="700" fill="#0f172a">ヒープ（文字のバイト本体）</text>
<text x="40" y="52" font-size="13" font-weight="700" fill="#1e40af">s: String</text>
<rect x="40" y="60" width="140" height="30" fill="#eff6ff" stroke="#1e40af"/>
<text x="110" y="80" text-anchor="middle" font-size="13" fill="#1e40af">ptr（本体の場所）</text>
<rect x="40" y="90" width="140" height="30" fill="#eff6ff" stroke="#1e40af"/>
<text x="110" y="110" text-anchor="middle" font-size="13" fill="#1e40af">len 10</text>
<rect x="40" y="120" width="140" height="30" fill="#eff6ff" stroke="#1e40af"/>
<text x="110" y="140" text-anchor="middle" font-size="13" fill="#1e40af">cap 10</text>
<text x="40" y="182" font-size="13" font-weight="700" fill="#c2410c">t: &amp;str = &amp;s[0..4]</text>
<rect x="40" y="190" width="140" height="30" fill="#fff7ed" stroke="#c2410c"/>
<text x="110" y="210" text-anchor="middle" font-size="13" fill="#c2410c">ptr（のぞく場所）</text>
<rect x="40" y="220" width="140" height="30" fill="#fff7ed" stroke="#c2410c"/>
<text x="110" y="240" text-anchor="middle" font-size="13" fill="#c2410c">len 4</text>
<rect x="310" y="110" width="36" height="40" fill="#ffffff" stroke="#475569"/>
<text x="328" y="136" text-anchor="middle" font-size="14" fill="#0f172a">R</text>
<rect x="346" y="110" width="36" height="40" fill="#ffffff" stroke="#475569"/>
<text x="364" y="136" text-anchor="middle" font-size="14" fill="#0f172a">u</text>
<rect x="382" y="110" width="36" height="40" fill="#ffffff" stroke="#475569"/>
<text x="400" y="136" text-anchor="middle" font-size="14" fill="#0f172a">s</text>
<rect x="418" y="110" width="36" height="40" fill="#ffffff" stroke="#475569"/>
<text x="436" y="136" text-anchor="middle" font-size="14" fill="#0f172a">t</text>
<rect x="454" y="110" width="108" height="40" fill="#f8fafc" stroke="#475569"/>
<line x1="490" y1="110" x2="490" y2="150" stroke="#cbd5e1" stroke-dasharray="3 3"/>
<line x1="526" y1="110" x2="526" y2="150" stroke="#cbd5e1" stroke-dasharray="3 3"/>
<text x="508" y="136" text-anchor="middle" font-size="15" fill="#0f172a">入</text>
<rect x="562" y="110" width="108" height="40" fill="#f8fafc" stroke="#475569"/>
<line x1="598" y1="110" x2="598" y2="150" stroke="#cbd5e1" stroke-dasharray="3 3"/>
<line x1="634" y1="110" x2="634" y2="150" stroke="#cbd5e1" stroke-dasharray="3 3"/>
<text x="616" y="136" text-anchor="middle" font-size="15" fill="#0f172a">門</text>
<path d="M310,100 L310,90 L670,90 L670,100" fill="none" stroke="#1e40af" stroke-width="2"/>
<text x="490" y="82" text-anchor="middle" font-size="12" fill="#1e40af">s が持っている 10 バイト（"Rust入門"）</text>
<path d="M310,160 L310,170 L454,170 L454,160" fill="none" stroke="#c2410c" stroke-width="2"/>
<text x="382" y="188" text-anchor="middle" font-size="12" fill="#c2410c">t がのぞく 4 バイト（"Rust"）</text>
<line x1="180" y1="75" x2="304" y2="118" stroke="#1e40af" stroke-width="2" marker-end="url(#ar-02-07)"/>
<line x1="180" y1="205" x2="304" y2="143" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-02-07)"/>
<text x="490" y="236" text-anchor="middle" font-size="13" fill="#334155">String = 持ち主。書き足すと本体を広げる（cap = 確保済みの広さ）</text>
<text x="490" y="260" text-anchor="middle" font-size="13" fill="#334155">&amp;str = のぞき窓（開始位置と長さだけ）。書き換えはできない</text>
<text x="490" y="284" text-anchor="middle" font-size="12" fill="#64748b">"Rust入門" は 4 + 3 + 3 = 10 バイト</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図5-1: String は本体（ヒープ上のバイト列）の持ち主、&amp;str はその一部をのぞく窓</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="360" y="24" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">"aあ🦀" を UTF-8 で保存すると 8 バイトになる</text>
<text x="40" y="70" text-anchor="end" font-size="12" fill="#64748b">文字</text>
<rect x="80" y="44" width="70" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="115" y="70" text-anchor="middle" font-size="16" fill="#1e40af">a</text>
<rect x="150" y="44" width="210" height="40" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="255" y="70" text-anchor="middle" font-size="16" fill="#c2410c">あ</text>
<rect x="360" y="44" width="280" height="40" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="500" y="71" text-anchor="middle" font-size="18" fill="#047857">🦀</text>
<text x="40" y="126" text-anchor="end" font-size="12" fill="#64748b">バイト</text>
<rect x="80" y="100" width="70" height="40" fill="#eff6ff" stroke="#3b82f6"/>
<text x="115" y="126" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#1e40af">61</text>
<rect x="150" y="100" width="70" height="40" fill="#fff7ed" stroke="#f97316"/>
<text x="185" y="126" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#c2410c">E3</text>
<rect x="220" y="100" width="70" height="40" fill="#fff7ed" stroke="#f97316"/>
<text x="255" y="126" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#c2410c">81</text>
<rect x="290" y="100" width="70" height="40" fill="#fff7ed" stroke="#f97316"/>
<text x="325" y="126" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#c2410c">82</text>
<rect x="360" y="100" width="70" height="40" fill="#ecfdf5" stroke="#10b981"/>
<text x="395" y="126" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#047857">F0</text>
<rect x="430" y="100" width="70" height="40" fill="#ecfdf5" stroke="#10b981"/>
<text x="465" y="126" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#047857">9F</text>
<rect x="500" y="100" width="70" height="40" fill="#ecfdf5" stroke="#10b981"/>
<text x="535" y="126" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#047857">A6</text>
<rect x="570" y="100" width="70" height="40" fill="#ecfdf5" stroke="#10b981"/>
<text x="605" y="126" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#047857">80</text>
<text x="40" y="162" text-anchor="end" font-size="12" fill="#64748b">位置</text>
<text x="115" y="162" text-anchor="middle" font-size="12" fill="#64748b">0</text>
<text x="185" y="162" text-anchor="middle" font-size="12" fill="#64748b">1</text>
<text x="255" y="162" text-anchor="middle" font-size="12" fill="#64748b">2</text>
<text x="325" y="162" text-anchor="middle" font-size="12" fill="#64748b">3</text>
<text x="395" y="162" text-anchor="middle" font-size="12" fill="#64748b">4</text>
<text x="465" y="162" text-anchor="middle" font-size="12" fill="#64748b">5</text>
<text x="535" y="162" text-anchor="middle" font-size="12" fill="#64748b">6</text>
<text x="605" y="162" text-anchor="middle" font-size="12" fill="#64748b">7</text>
<rect x="80" y="180" width="270" height="56" rx="8" fill="#f8fafc" stroke="#94a3b8"/>
<text x="215" y="204" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#0f172a">len() = 8</text>
<text x="215" y="224" text-anchor="middle" font-size="12" fill="#64748b">バイトの数</text>
<rect x="370" y="180" width="270" height="56" rx="8" fill="#f8fafc" stroke="#94a3b8"/>
<text x="505" y="204" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#0f172a">chars().count() = 3</text>
<text x="505" y="224" text-anchor="middle" font-size="12" fill="#64748b">文字の数</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図5-2: 英字は1バイト、ひらがなは3バイト、絵文字は4バイト。位置1〜3は「あ」の途中なので、そこでは切れない</div>
</div>

| | `String` | `&str` |
|---|---|---|
| 役割 | 文字列の**持ち主** | 文字列を**借りて読む窓** |
| 書き足し・変更 | できる（`mut` が必要） | できない |
| 作り方の例 | `String::from("..")`、`"..".to_string()`、`format!(..)` | `"..."`（リテラル）、`&s`、`&s[0..4]` |
| よく使う場所 | 構造体のフィールド、関数の戻り値 | **関数の引数** |

### まず動かす最小コード

#### 例1: String を作って書き足す

> **▼ このコードがやること（先に日本語で）:** 文字列リテラルから `String` を作り、後ろに文字列や1文字を書き足して表示します。`format!` と `+` で文字列をつなぐ方法も試します。

```rust
fn main() {
    // 文字列リテラルは &str 型（プログラムに埋め込まれた、書き換えられない文字列）
    let greeting: &str = "こんにちは";

    // String::from で、書き足せる String を作る
    let mut message = String::from(greeting);
    // push_str で文字列を後ろに足す
    message.push_str("、Rust");
    // push で1文字（char）を足す
    message.push('！');
    println!("{message}");

    // to_string でも String を作れる
    let name = "さくら".to_string();
    // format! は println! と同じ書き方で、表示せずに String を作る
    let line = format!("{name}さん、{message}");
    println!("{line}");

    // + でつなぐこともできる（左側の String は使い切られる）
    let a = String::from("Rust");
    let b = a + "入門";
    println!("{b}");
}
```

```powershell
cargo run
```

**▼ 実行結果**

```text
こんにちは、Rust！
さくらさん、こんにちは、Rust！
Rust入門
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: リテラルは `&str`、作ったものは `String`

```rust
let greeting: &str = "こんにちは";
let mut message = String::from(greeting);
```

- `"こんにちは"` はプログラムの中に埋め込まれた文字列で、型は `&str` です。書き換えはできません。
- `String::from(...)` で、その内容をコピーした**自分の** `String` を作ります。書き足すので `mut` を付けています。

##### 解説2: `push_str` と `push`

```rust
message.push_str("、Rust");
message.push('！');
```

- `push_str` は文字列（`&str`）を、`push` は1文字（`char`、シングルクォート）を末尾に足します。
- 足すたびに、`String` は必要なら本体の置き場所を広げます（図5-1 の `cap`）。

##### 解説3: `format!` と `+`

```rust
let line = format!("{name}さん、{message}");
let b = a + "入門";
```

- `format!` は `println!` と同じ書き方で、表示する代わりに**新しい `String` を作ります**。いくつもの値を組み合わせるときは、これがいちばん読みやすい方法です。
- `+` は「左の `String` に右の `&str` を書き足したもの」を返します。このとき左の `a` は**使い切られて**、以後は使えなくなります（理由は第3巻の「所有権」で学びます）。迷ったら `format!` を使いましょう。

#### 例2: バイト数と文字数

> **▼ このコードがやること（先に日本語で）:** 英字・ひらがな・絵文字が1つずつ入った文字列で、「バイト数」と「文字数」を比べます。1文字ずつ、何バイトを使っているかも表示します。

```rust
fn main() {
    let text = "aあ🦀";
    // len() は「バイト数」を返す
    println!("len(): {}", text.len());
    // chars().count() は「文字数」を返す
    println!("chars().count(): {}", text.chars().count());

    // 1文字ずつ取り出して、それぞれが何バイトかを調べる
    for c in text.chars() {
        println!("'{c}' は {} バイト", c.len_utf8());
    }
    // 中身のバイトを16進数で並べて表示する
    println!("bytes: {:02X?}", text.as_bytes());
}
```

**▼ 実行結果**

```text
len(): 8
chars().count(): 3
'a' は 1 バイト
'あ' は 3 バイト
'🦀' は 4 バイト
bytes: [61, E3, 81, 82, F0, 9F, A6, 80]
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `len()` はバイト数

- `len()` は**バイト数**を返します。`a`(1) + `あ`(3) + `🦀`(4) = 8 です（図5-2）。
- 「文字数」が欲しいときは `chars().count()` を使います。`chars()` は文字列を `char`（1文字）の並びとして取り出し、`count()` はその個数を数えます。

##### 解説2: `{:02X?}` でバイトを見る

- `as_bytes()` は、文字列の中身をバイトの並びとして見せます。`{:02X?}` は「16進数・大文字・2桁で、並びとして表示する」書式です。
- `E3 81 82` が「あ」、`F0 9F A6 80` が「🦀」（Rust のマスコット、カニの Ferris）です。

> **用語:** **UTF-8** … Unicode の文字番号を、1〜4バイトのバイト列に変換する決まり。英数字は ASCII（昔からある英語用の文字コード）と同じ1バイトなので、互換性が高く、Web やファイルの標準になっています。Rust の `String` と `&str` は**必ず正しい UTF-8** であることが保証されています。

#### 例3: 関数の引数は &str で受け取る

> **▼ このコードがやること（先に日本語で）:** 文字列を大文字にして「！」を付ける関数を作り、`String` と文字列リテラルの両方から呼び出します。

```rust
// &str を受け取る関数は、String からも文字列リテラルからも呼べる
fn shout(text: &str) -> String {
    format!("{}！", text.to_uppercase())
}

fn main() {
    let owned = String::from("rust");
    // &owned と書くと、String を &str として「貸す」ことになる
    println!("{}", shout(&owned));
    // 文字列リテラルは、もともと &str
    println!("{}", shout("hello"));
    // 貸しただけなので、owned はこのあとも使える
    println!("owned はまだ使える: {owned}");
}
```

**▼ 実行結果**

```text
RUST！
HELLO！
owned はまだ使える: rust
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `&str` で受け取ると、どちらからも呼べる

```rust
fn shout(text: &str) -> String {
```

- 引数を `&str` にしておくと、`String` は `&owned` と書いて、リテラルはそのまま渡せます。`String` を `&` で貸すと、自動で `&str` として扱われます。
- 逆に引数を `String` にすると、リテラルを渡すたびに `.to_string()` が必要になり、呼び出し側の `String` も手放すことになります。**読むだけの関数は `&str` で受け取る**のが基本です。
- 戻り値は新しく作った文字列なので `String` です。「受け取りは `&str`、作って返すなら `String`」と覚えましょう。

#### 例4: スライスで一部分を取り出す

> **▼ このコードがやること（先に日本語で）:** 英字と日本語の文字列から、先頭の一部分を取り出します。バイトの位置で切る方法と、文字数で切る方法、切ってよい位置かを確かめる方法を比べます。

```rust
fn main() {
    let ascii = "Rustacean";
    // &文字列[開始..終了] で一部分（スライス）を取り出す。数字は「バイトの位置」
    println!("{}", &ascii[0..4]);

    let japanese = "日本語のノート";
    // 日本語は1文字3バイトなので、0..6 は先頭の2文字
    println!("{}", &japanese[0..6]);

    // 「文字数」で切り出したいときは chars() を使う
    let first_three: String = japanese.chars().take(3).collect();
    println!("{first_three}");

    // その位置が文字の区切りかどうかを調べられる
    println!("2バイト目は文字の区切り? {}", japanese.is_char_boundary(2));
    // get は、区切りでない位置なら None を返す（panic しない）
    println!("{:?}", japanese.get(0..2));
    println!("{:?}", japanese.get(0..3));
}
```

**▼ 実行結果**

```text
Rust
日本
日本語
2バイト目は文字の区切り? false
None
Some("日")
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `&s[開始..終了]` の数字はバイト

- `&ascii[0..4]` は「0バイト目から4バイト目の手前まで」で、英字なら4文字です。
- 日本語は1文字3バイトなので、2文字欲しいなら `0..6` と書く必要があります。**文字の途中の位置を指定すると panic します**（下の「よくあるエラー」）。

##### 解説2: 文字数で切るなら `chars().take(n)`

```rust
let first_three: String = japanese.chars().take(3).collect();
```

- `chars()` で1文字ずつにして、`take(3)` で先頭3文字だけを取り、`collect()` で `String` に集めます。どんな文字でも安全です。
- `collect()` は「何に集めるか」を型注釈（`: String`）から読み取ります。

##### 解説3: `get` は失敗を `None` で返す

- `japanese.get(0..2)` は、2バイト目が文字の途中なので `None`（取り出せない）を返しました。`0..3` なら `Some("日")` です。
- 外から来た位置で切り出すときは、panic しない `get` を使うと安全です。

#### 例5: よく使う文字列メソッド

> **▼ このコードがやること（先に日本語で）:** カンマ区切りのタグの文字列を、空白の除去・小文字化・検索・置き換え・分割で加工します。最後に改行を含む文字列の行数を数えます。

```rust
fn main() {
    let raw = "  Rust, Web,  DB  ";
    // trim: 前後の空白を取り除く
    println!("[{}]", raw.trim());
    // to_lowercase: 小文字にする（to_uppercase なら大文字）
    println!("{}", raw.trim().to_lowercase());
    // contains / starts_with: 含むか・その文字で始まるか
    println!("Web を含む? {}", raw.contains("Web"));
    println!("Rust で始まる? {}", raw.trim().starts_with("Rust"));
    // replace: 置き換えた新しい String を作る
    println!("{}", raw.trim().replace(", ", " / "));
    // split: 区切り文字で分ける。各部分の空白も trim で取り除く
    for part in raw.split(',') {
        println!("- [{}]", part.trim());
    }
    // lines: 改行で分ける（\n は改行を表す）
    let memo = "1行目\n2行目\n3行目";
    println!("行数: {}", memo.lines().count());
}
```

**▼ 実行結果**

```text
[Rust, Web,  DB]
rust, web,  db
Web を含む? true
Rust で始まる? true
Rust / Web /  DB
- [Rust]
- [Web]
- [DB]
行数: 3
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 元の文字列は変わらない

- `trim` `to_lowercase` `replace` などは、**元の文字列を書き換えずに**、結果を新しく返します。`raw` は最後まで最初のままです。
- `trim` は `&str`（元の一部をのぞく窓）を、`to_lowercase` と `replace` は新しい `String` を返します。

##### 解説2: `split` と `lines`

- `split(',')` は `,` で区切った部分を順に取り出します。`"  DB  "` のように各部分に空白が残るので、`trim()` と組み合わせるのが定番です。
- `lines()` は改行で区切ります。`"\n"` は文字列の中で改行を表す書き方（エスケープ）です。

| メソッド | 何をするか | 結果の型 |
|---|---|---|
| `trim()` | 前後の空白を除く | `&str` |
| `to_lowercase()` / `to_uppercase()` | 小文字 / 大文字にする | `String` |
| `contains(x)` / `starts_with(x)` / `ends_with(x)` | 含むか / 始まるか / 終わるか | `bool` |
| `replace(a, b)` | a を b に置き換える | `String` |
| `split(x)` / `lines()` / `split_whitespace()` | 区切って順に取り出す | 部分の並び |
| `parse::<T>()` | 数などへ変換する | 成功か失敗（UNIT 09） |
| `chars()` / `chars().count()` | 1文字ずつ / 文字数 | 文字の並び / `usize` |

### もう一歩: 実務に近い形

一覧画面では、長いタイトルを省略して表示したり、入力された余分な空白をまとめたりします。どちらも「文字数」で考えるのがポイントです。

#### タイトルの省略表示と空白の整理

> **▼ このコードがやること（先に日本語で）:** タイトルが10文字を超えていたら10文字で切って「…」を付けます。また、全角スペースやタブを含む連続した空白を、半角スペース1つにまとめます。

```rust
// 長いタイトルを max 文字に切り詰め、切ったときは「…」を付ける
fn preview(title: &str, max: usize) -> String {
    if title.chars().count() <= max {
        return title.to_string();
    }
    let head: String = title.chars().take(max).collect();
    format!("{head}…")
}

// 連続した空白（全角スペースやタブも含む）を、半角スペース1つにまとめる
fn squash_spaces(text: &str) -> String {
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn main() {
    println!("{}", preview("短いタイトル", 10));
    println!("{}", preview("とても長いタイトルなので一覧では省略したい", 10));
    println!("[{}]", squash_spaces("  牛乳 と　　卵を\t買う  "));
}
```

**▼ 実行結果**

```text
短いタイトル
とても長いタイトルな…
[牛乳 と 卵を 買う]
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 文字数で比べ、文字数で切る

```rust
if title.chars().count() <= max {
    return title.to_string();
}
let head: String = title.chars().take(max).collect();
```

- `len()` で比べたり `&title[0..max]` で切ったりすると、日本語では文字の途中で切れて panic します。**比べるのも切るのも `chars()`** にそろえます。
- 短いときは早期リターンで、そのまま `String` にして返しています。

##### 解説2: `split_whitespace` と `join`

```rust
text.split_whitespace().collect::<Vec<&str>>().join(" ")
```

- `split_whitespace()` は、半角・全角スペースやタブなど**あらゆる空白**で区切り、空の部分は捨てます。
- `collect::<Vec<&str>>()` で部分を `Vec`（UNIT 06）に集め、`join(" ")` で半角スペースをはさんで1つの `String` につなぎます。`::<...>` は「何に集めるか」を直接指定する書き方です。

### よくあるエラーと読み方

#### エラー1: E0277 文字列に s[0] と書いた

```rust
fn main() {
    let s = String::from("hello");
    let first = s[0];
    println!("{first}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> src\main.rs:3:19
  |
3 |     let first = s[0];
  |                   ^ string indices are ranges of `usize`
  |
  = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
  = note: you can use `.chars().nth()` or `.bytes().nth()`
          for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
help: `usize` implements trait `SliceIndex<T>`
 --> /rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library\core\src\slice\index.rs:214:0
  |
  = note: `SliceIndex<[T]>`
 --> /rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library\core\src\bstr\traits.rs:197:0
  |
  = note: `SliceIndex<ByteStr>`
  = note: required for `String` to implement `Index<{integer}>`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** 「`str` 型は整数で添え字アクセスできません」。`string indices are ranges of usize` は「文字列の添え字は（1つの数ではなく）範囲です」、`note: you can use .chars().nth() or .bytes().nth()` は「`.chars().nth()` か `.bytes().nth()` を使えます」という提案です。
- **何が起きたか:** UTF-8 では「0番目」が1バイトなのか文字なのかがあいまいで、日本語なら文字の途中にもなり得ます。そこで Rust は `s[0]` を禁止しています。
- **直し方:** 最初の文字が欲しいなら `s.chars().next()`、n番目なら `s.chars().nth(n)` を使います。結果は「ないかもしれない」ので `Option<char>` です（UNIT 08）。

#### エラー2: 文字の途中でスライスして panic

```rust
fn main() {
    let title = String::from("日本語");
    // 先頭の2バイトを取り出そうとする
    let part = &title[0..2];
    println!("{part}");
}
```

**▼ 実行結果**

```text
thread 'main' (45508) panicked at src\main.rs:4:22:
end byte index 2 is not a char boundary; it is inside '日' (bytes 0..3 of string)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\basics.exe` (exit code: 101)
```

- **読み方:** `end byte index 2 is not a char boundary; it is inside '日' (bytes 0..3 of string)` は「終わりの位置 2 は文字の区切りではありません。'日'（0〜3バイト目）の途中です」。
- **何が起きたか:** コンパイルは通りますが、**実行したときに** panic します。位置が実行中に決まることもあるので、コンパイラには見つけられません。
- **直し方:** 文字数で切るなら `title.chars().take(2).collect::<String>()`、バイト位置が正しいか分からないなら `title.get(0..2)`（`None` が返る）を使います。

#### エラー3: E0308 String が欲しいところに &str を渡した

```rust
fn main() {
    let name: String = "Rust";
    println!("{name}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0308]: mismatched types
 --> src\main.rs:2:24
  |
2 |     let name: String = "Rust";
  |               ------   ^^^^^^ expected `String`, found `&str`
  |               |
  |               expected due to this
  |
help: try using a conversion method
  |
2 |     let name: String = "Rust".to_string();
  |                              ++++++++++++

For more information about this error, try `rustc --explain E0308`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** `expected String, found &str`。型注釈で `String` と決めたのに、リテラル（`&str`）を入れようとしました。
- **直し方:** `help` のとおり `"Rust".to_string()`（または `String::from("Rust")`）で `String` を作ります。

#### エラー4: E0369 &str どうしを + でつないだ

```rust
fn main() {
    let full = "Rust" + "入門";
    println!("{full}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0369]: cannot add `&str` to `&str`
 --> src\main.rs:2:23
  |
2 |     let full = "Rust" + "入門";
  |                ------ ^ ------ &str
  |                |      |
  |                |      `+` cannot be used to concatenate two `&str` strings
  |                &str
  |
  = note: string concatenation requires an owned `String` on the left
help: create an owned `String` from a string reference
  |
2 |     let full = "Rust".to_owned() + "入門";
  |                      +++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** `cannot add &str to &str` と `note: string concatenation requires an owned String on the left`（文字列の連結には、左側に持ち主の `String` が必要）。
- **直し方:** `help` の `.to_owned()`（`to_string()` と同じく `String` を作る）でもよいですが、`format!("{}{}", "Rust", "入門")` と書くほうが読みやすくおすすめです。

### RustNote ではここで使う

RustNote のタイトルは「1〜80**文字**」です。ここで `len()` を使うと、日本語のタイトルは1文字3バイトなので、**27文字（81バイト）になった時点で拒否されてしまいます**。必ず `chars().count()` で数えます。

> **▼ このコードがやること（先に日本語で）:** 「あ」を80文字並べたタイトル（ちょうど上限）について、`len()` と `chars().count()` で判定結果がどう違うかを確かめます。

```rust
const MAX_TITLE_CHARS: usize = 80;

// 文字数が 1〜80 の範囲なら true
fn is_valid_title(title: &str) -> bool {
    let count = title.chars().count();
    (1..=MAX_TITLE_CHARS).contains(&count)
}

fn main() {
    // 「あ」を 80 文字並べたタイトル（ちょうど上限）
    let title = "あ".repeat(80);
    println!("len()           = {} バイト", title.len());
    println!("chars().count() = {} 文字", title.chars().count());

    // もし len() で判定していたら、上限ちょうどの日本語タイトルを拒否してしまう
    let by_len = title.len() <= MAX_TITLE_CHARS;
    println!("len() で判定:           {by_len}");
    println!("chars().count() で判定: {}", is_valid_title(&title));

    // 前後の空白を取り除いてから判定する
    println!("空白だけ: {}", is_valid_title("   ".trim()));
}
```

**▼ 実行結果**

```text
len()           = 240 バイト
chars().count() = 80 文字
len() で判定:           false
chars().count() で判定: true
空白だけ: false
```

- `len()` で判定すると、規則上は正しい80文字のタイトルを「長すぎる」と誤って拒否してしまいます。
- RustNote のデータベース（libSQL / SQLite）でも `CHECK (length(title) BETWEEN 1 AND 80)` のように文字数を制限します。SQLite の `length()` は文字列の**文字数**を数えるので、Rust 側の `chars().count()` と同じ基準になります。画面・Rust・データベースで数え方をそろえることが大切です。

> **注意:** `chars().count()` が数えるのは Unicode の「文字番号」の数です。家族の絵文字のように、見た目は1文字でも複数の番号でできている文字があります。

```rust
fn main() {
    // 家族の絵文字は、3人の絵文字を「つなぎ文字」でつないだもの
    let family = "👨‍👩‍👧";
    println!("見た目は1文字: {family}");
    println!("chars().count() = {}", family.chars().count());
    println!("len() = {}", family.len());
}
```

**▼ 実行結果**

```text
見た目は1文字: 👨‍👩‍👧
chars().count() = 5
len() = 18
```

> 見た目どおりの「1文字」（書記素クラスタ）で数えるには `unicode-segmentation` という外部クレートが必要です。RustNote では、データベースの `length()` とそろえるために `chars().count()` を採用します。

### 練習問題

**問題1.** `"こんにちは"` と `"Hello"` のそれぞれについて、バイト数と文字数を表示してください。

**問題2.** 文字列の中に、指定した文字が何回出てくるかを数える関数 `fn count_char(text: &str, target: char) -> usize` を、`for` と `chars()` を使って書いてください。

**問題3.** `"Rust入門"` を逆順にした文字列 `門入tsuR` を作って表示してください（ヒント: `chars()`、`.rev()`、`collect()`）。

### 解答と解説

**解答1.**

```rust
fn main() {
    for text in ["こんにちは", "Hello"] {
        println!("{text}: {} バイト, {} 文字", text.len(), text.chars().count());
    }
}
```

**▼ 実行結果**

```text
こんにちは: 15 バイト, 5 文字
Hello: 5 バイト, 5 文字
```

- ひらがなは1文字3バイトなので 5 × 3 = 15 バイトです。英字は文字数とバイト数が一致します。

**解答2.**

```rust
// text の中に target が何回出てくるかを数える
fn count_char(text: &str, target: char) -> usize {
    let mut count = 0;
    for c in text.chars() {
        if c == target {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", count_char("あいあいあ", 'あ'));
    println!("{}", count_char("Rust", 'x'));
}
```

**▼ 実行結果**

```text
3
0
```

- `char` どうしは `==` で比べられます。数え上げる変数 `count` だけを `mut` にしています。UNIT 10 では、同じ処理を `text.chars().filter(|c| *c == target).count()` と1行で書く方法を学びます。

**解答3.**

```rust
fn main() {
    let text = "Rust入門";
    // 1文字ずつ取り出し、逆順にして、String に集める
    let reversed: String = text.chars().rev().collect();
    println!("{reversed}");
}
```

**▼ 実行結果**

```text
門入tsuR
```

- バイト単位で逆順にすると日本語が壊れますが、`chars()` で文字単位にしてから `.rev()` で逆順にすれば安全です。

### まとめ

- `String` は書き足せる文字列の持ち主、`&str` はそれを借りて読む窓。読むだけの関数の引数は `&str`、作って返すなら `String`。
- 文字列は UTF-8 のバイト列。日本語は1文字3バイトなので、`len()`（バイト数）と `chars().count()`（文字数）は違う。
- `s[0]` は書けない。文字は `chars()` で取り出し、切り出しは文字数（`chars().take(n)`）か、安全な `get` で行う。

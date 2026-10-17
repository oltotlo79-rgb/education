## UNIT 02: ファイル・フォルダ・パス

### この単元のゴール

- **ファイル**と**フォルダ（ディレクトリ）**の関係を、木の形（ツリー）で説明できる
- **絶対パス**と**相対パス**の違いを説明し、正しく書ける
- 「今いるフォルダ（**カレントディレクトリ**）」によって、相対パスの指す場所が変わることを体験する
- Rust でファイルを書いて読み戻し、「ファイルが見つからない」エラーを読める

### たとえ話でつかむ

ファイルの場所を表す**パス**は、**住所**とそっくりです。

- 「東京都 千代田区 千代田 1-1」のように、**いちばん大きな区切りから順に全部書いた住所**は、日本中どこにいる人に渡しても同じ場所を指します。これが**絶対パス**です。Windows なら `C:\Users\you\rust-study\practice\Cargo.toml` のように、ドライブ名 `C:` から書き始めます。
- 一方、「この角を曲がって2軒目」という**道案内**は、**今どこに立っているか**によって指す家が変わります。これが**相対パス**です。`src\main.rs` は「今いるフォルダの中の `src` フォルダの中の `main.rs`」という意味で、どこに立っているかで行き先が変わります。
- **フォルダ**は「棚」や「引き出し」です。引き出しの中に小さな引き出し（フォルダ）や書類（ファイル）を入れられます。

> **用語:** **ファイル** … ストレージに保存された、名前の付いたデータのかたまり。文章、画像、プログラムなど、中身は何でもかまいません。
>
> **用語:** **フォルダ（ディレクトリ）** … ファイルや別のフォルダを入れておく入れ物。Windows では「フォルダ」、コマンドの世界では「ディレクトリ」と呼ぶことが多いですが、同じものです。
>
> **用語:** **拡張子（かくちょうし）** … ファイル名の最後の `.` から後ろの部分（`main.rs` の `rs`、`Cargo.toml` の `toml`）。中身の種類の目印です。`.rs` は Rust のソースコードを表します。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="14" width="150" height="32" rx="6" fill="#f8fafc" stroke="#475569" stroke-width="1.5"/>
<text font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" x="95" y="35" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">C:\（ドライブ）</text>
<path d="M50,46 L50,72 L80,72" fill="none" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="80" y="58" width="120" height="28" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="140" y="77" font-size="13" fill="#9a3412" text-anchor="middle">Users</text>
<path d="M110,86 L110,110 L140,110" fill="none" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="140" y="96" width="120" height="28" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="200" y="115" font-size="13" fill="#9a3412" text-anchor="middle">you</text>
<path d="M170,124 L170,148 L200,148" fill="none" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="200" y="134" width="130" height="28" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="265" y="153" font-size="13" fill="#9a3412" text-anchor="middle">rust-study</text>
<path d="M230,162 L230,186 L260,186" fill="none" stroke="#94a3b8" stroke-width="1.5"/>
<path d="M230,186 L230,300 L260,300" fill="none" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="260" y="172" width="120" height="28" rx="6" fill="#eff6ff" stroke="#1e40af" stroke-width="2.5"/>
<text x="320" y="191" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">practice</text>
<rect x="260" y="286" width="120" height="28" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="320" y="305" font-size="13" fill="#9a3412" text-anchor="middle">hello</text>
<path d="M290,200 L290,224 L320,224" fill="none" stroke="#94a3b8" stroke-width="1.5"/>
<path d="M290,224 L290,262 L320,262" fill="none" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="320" y="210" width="110" height="28" rx="6" fill="#ffffff" stroke="#94a3b8"/>
<text x="375" y="229" font-size="13" fill="#334155" text-anchor="middle">Cargo.toml</text>
<rect x="320" y="248" width="110" height="28" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="375" y="267" font-size="13" fill="#9a3412" text-anchor="middle">src</text>
<path d="M430,262 L470,262" fill="none" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="470" y="248" width="110" height="28" rx="6" fill="#ffffff" stroke="#94a3b8"/>
<text x="525" y="267" font-size="13" fill="#334155" text-anchor="middle">main.rs</text>
<rect x="400" y="166" width="300" height="40" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="412" y="184" font-size="12" font-weight="700" fill="#1e40af">太枠 = 今いるフォルダ（カレントディレクトリ）</text>
<text font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" x="412" y="200" font-size="12" fill="#334155">PS C:\Users\you\rust-study\practice&gt;</text>
<rect x="430" y="22" width="270" height="110" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<rect x="446" y="40" width="16" height="16" rx="3" fill="#fff7ed" stroke="#f97316"/>
<text x="470" y="53" font-size="12" fill="#334155">フォルダ（中に入れ物を持てる）</text>
<rect x="446" y="66" width="16" height="16" rx="3" fill="#ffffff" stroke="#94a3b8"/>
<text x="470" y="79" font-size="12" fill="#334155">ファイル（データそのもの）</text>
<text x="446" y="106" font-size="12" fill="#334155">線は「この中に入っている」</text>
<text x="446" y="122" font-size="12" fill="#334155">という親子関係を表す</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図2-1: フォルダの木（ツリー）。上が「親」、線でつながった下が「子」</div>
</div>

同じ `main.rs` を、絶対パスと相対パスでどう書くかを比べます。今いるフォルダが `practice` のとき、次の図の3つの書き方は**どれも同じファイル**を指します。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-08" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="16" width="680" height="62" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="36" y="40" font-size="14" font-weight="700" fill="#c2410c">絶対パス（どこにいても同じ場所）</text>
<text x="36" y="64" font-size="14" fill="#0f172a" font-family="Cascadia Mono, Consolas, monospace">C:\Users\you\rust-study\practice\src\main.rs</text>
<rect x="20" y="92" width="680" height="62" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="36" y="116" font-size="14" font-weight="700" fill="#1e40af">相対パス（今いる practice から見た道順）</text>
<text x="36" y="140" font-size="14" fill="#0f172a" font-family="Cascadia Mono, Consolas, monospace">src\main.rs　　または　　.\src\main.rs</text>
<rect x="20" y="168" width="680" height="88" rx="10" fill="#f8fafc" stroke="#475569"/>
<text x="36" y="192" font-size="14" font-weight="700" fill="#334155">道順に使う特別な名前</text>
<text x="36" y="216" font-size="13" fill="#0f172a" font-family="Cascadia Mono, Consolas, monospace">.</text>
<text x="70" y="216" font-size="13" fill="#334155">今いるフォルダ自身（「ここ」）</text>
<text x="36" y="240" font-size="13" fill="#0f172a" font-family="Cascadia Mono, Consolas, monospace">..</text>
<text x="70" y="240" font-size="13" fill="#334155">1つ上の親フォルダ（例: practice から見た .. は rust-study）</text>
<text x="420" y="216" font-size="13" fill="#0f172a" font-family="Cascadia Mono, Consolas, monospace">..\hello</text>
<line x1="500" y1="211" x2="530" y2="211" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-08)"/>
<text x="538" y="216" font-size="13" fill="#334155">隣の hello フォルダ</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図2-2: 絶対パスと相対パス。相対パスは「今いるフォルダ」が出発点</div>
</div>

> **ヒント:** Windows ではパスの区切りに `\`（円記号またはバックスラッシュ）を使い、Linux や macOS では `/`（スラッシュ）を使います。Rust のプログラムの中では、Windows でも `/` で書いたパスがたいてい通じます。日本語キーボードでは `\` が `¥` と表示されることがありますが、同じ文字です。

### まず動かす最小コード

#### ファイルを書いて、読み戻す

> **▼ このコードがやること（先に日本語で）:** 今いるフォルダの場所を表示してから、`memo` というフォルダを作り、その中に `today.txt` というファイルを書き込みます。書いたファイルをすぐに読み戻して、相対パス・絶対パス・中身を表示します。

```rust
use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // 今いるフォルダ（カレントディレクトリ）を調べて表示する
    let here = env::current_dir()?;
    println!("今いるフォルダ: {}", here.display());

    // memo フォルダを作る（すでにあっても OK）
    fs::create_dir_all("memo")?;

    // 「memo フォルダの中の today.txt」という相対パスを組み立てる
    let relative = Path::new("memo").join("today.txt");

    // ファイルに文字を書き込む（無ければ作り、あれば上書き）
    fs::write(&relative, "Rust の勉強を始めた\n")?;

    // 同じファイルを読み戻す
    let text = fs::read_to_string(&relative)?;

    println!("相対パス: {}", relative.display());
    println!("絶対パス: {}", here.join(&relative).display());
    print!("中身: {text}");
    Ok(())
}
```

```powershell
cargo run
```

**▼ 実行結果**

```text
今いるフォルダ: C:\Users\you\rust-study\practice
相対パス: memo\today.txt
絶対パス: C:\Users\you\rust-study\practice\memo\today.txt
中身: Rust の勉強を始めた
```

実行したあと、エクスプローラーで `practice` フォルダを開くと、`memo` フォルダと `today.txt` ができているはずです。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 失敗するかもしれない処理と `?`

```rust
fn main() -> std::io::Result<()> {
    let here = env::current_dir()?;
    // ...
    Ok(())
}
```

- ファイルの操作は、**失敗する可能性がいつもある**処理です（ファイルが無い、書き込みが禁止されている、ディスクがいっぱい…）。
- `-> std::io::Result<()>` は「この `main` は、成功（`Ok`）か失敗（`Err`）のどちらかを返します」という宣言です。
- 処理の後ろに付けた `?`（はてな）は、「**失敗だったら、ここで止めて失敗を報告する。成功なら中身を取り出して先へ進む**」という意味の記号です。
- 最後の `Ok(())` は「最後まで来たので成功です」という報告です。
- この仕組みは第2巻・第4巻で詳しく学びます。今は「`?` は失敗したらそこで止まる印」と覚えておけば十分です。

##### 解説2: 今いるフォルダを調べる

```rust
let here = env::current_dir()?;
println!("今いるフォルダ: {}", here.display());
```

- `env::current_dir()` は、**このプロセスのカレントディレクトリ**を OS に問い合わせます。
- `cargo run` で起動したプログラムは、**`cargo run` を打ったときに PowerShell がいたフォルダ**をカレントディレクトリとして受け継ぎます。
- `.display()` は、パスを画面に表示できる形にする道具です。

##### 解説3: パスを組み立てる

```rust
let relative = Path::new("memo").join("today.txt");
```

- `Path::new("memo")` は「`memo` というパス」を作ります。
- `.join("today.txt")` は、その後ろに区切り文字をはさんで `today.txt` をつなぎます。
- Windows なら `memo\today.txt`、Linux なら `memo/today.txt` と、**OS に合った区切り文字**を自動で使ってくれるのが利点です。
- 区切り文字を自分で `"memo\\today.txt"` のように書くより、`join` を使うほうが安全です。

##### 解説4: 書く・読む

```rust
fs::create_dir_all("memo")?;
fs::write(&relative, "Rust の勉強を始めた\n")?;
let text = fs::read_to_string(&relative)?;
```

- `fs`（ファイルシステム）は、ファイルやフォルダを扱う道具箱です。
- `create_dir_all` はフォルダを作ります。途中のフォルダが無ければそれも作り、すでにあっても失敗しません。
- `write` はファイルに書き込みます（**上書き**なので、前の中身は消えます）。
- `read_to_string` はファイル全体を文字列として読み込みます。
- `&relative` の `&` は「パスを貸すだけで、手放さない」という印です（第3巻で詳しく学びます）。
- `"\n"` は**改行**を表す特別な書き方です。

### もう一歩: 実務に近い形

#### パスを部品に分け、「ここから見て」存在するか調べる

> **▼ このコードがやること（先に日本語で）:** `src/main.rs` というパスを「ファイル名」「拡張子」「親フォルダ」に分解して表示します。続いて、いくつかの相対パスについて「今いるフォルダから見て存在するか」を調べ、最後に今いるフォルダの中身を一覧表示します。

```rust
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // 1 つのパスを部品に分けてみる
    let path = Path::new("src/main.rs");
    println!("パス全体  : {}", path.display());
    println!("ファイル名: {:?}", path.file_name());
    println!("拡張子    : {:?}", path.extension());
    println!("親フォルダ: {:?}", path.parent());
    println!();

    // いろいろな相対パスが「今いるフォルダから見て」存在するか調べる
    let candidates = [
        "Cargo.toml",
        "src/main.rs",
        "main.rs",
        "./src/../Cargo.toml",
    ];
    for candidate in candidates {
        let exists = Path::new(candidate).exists();
        println!("{candidate:<22} -> 存在する? {exists}");
    }
    println!();

    // 今いるフォルダの中身を一覧表示する
    println!("今いるフォルダの中身:");
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let kind = if entry.path().is_dir() {
            "フォルダ"
        } else {
            "ファイル"
        };
        println!("  [{kind}] {}", entry.file_name().display());
    }
    Ok(())
}
```

**▼ 実行結果**（`practice` フォルダで `cargo run` した場合）

```text
パス全体  : src/main.rs
ファイル名: Some("main.rs")
拡張子    : Some("rs")
親フォルダ: Some("src")

Cargo.toml             -> 存在する? true
src/main.rs            -> 存在する? true
main.rs                -> 存在する? false
./src/../Cargo.toml    -> 存在する? true

今いるフォルダの中身:
  [フォルダ] .git
  [ファイル] .gitignore
  [ファイル] Cargo.lock
  [ファイル] Cargo.toml
  [フォルダ] memo
  [フォルダ] src
  [フォルダ] target
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: パスの部品

```rust
println!("ファイル名: {:?}", path.file_name());
println!("拡張子    : {:?}", path.extension());
println!("親フォルダ: {:?}", path.parent());
```

- `file_name()` は最後の部品（`main.rs`）、`extension()` は拡張子（`rs`）、`parent()` は1つ上（`src`）を返します。
- 結果に `Some(...)` が付いているのは、「部品が無い場合もある」からです（例えば `C:\` には親がありません）。UNIT 01 で見た `Option` と同じ考え方です。
- `{:?}` は「プログラマー向けの形で表示する」という指定で、`Some(...)` や `"` もそのまま見せてくれます。

##### 解説2: 相対パスは「今いるフォルダ」が出発点

```rust
let exists = Path::new(candidate).exists();
```

- `exists()` は、そのパスにファイルかフォルダが**本当にあるか**を OS に確かめます。
- `main.rs` が `false` なのは、`main.rs` が `practice` の直下ではなく `src` の中にあるからです。
- `./src/../Cargo.toml` は「ここ（`.`）→ `src` に入る → 1つ上（`..`）に戻る → `Cargo.toml`」という遠回りの道順で、結局 `Cargo.toml` を指すので `true` です。

##### 解説3: フォルダの中身を一覧する

```rust
for entry in fs::read_dir(".")? {
    let entry = entry?;
    // ...
}
```

- `fs::read_dir(".")` は、今いるフォルダ（`.`）の中身を1つずつ取り出します。
- `.git` や `.gitignore` のように `.` で始まる名前は、Git というツールが使う設定です。エクスプローラーでは隠れていることがあります（第1巻で説明します）。
- `target` フォルダは、Cargo がコンパイルした結果を置く場所です（UNIT 04）。

### よくあるエラーと読み方

#### ファイルが見つからない（NotFound）

> **▼ このコードがやること（先に日本語で）:** 今いるフォルダにある `config.txt` を読み込んで表示しようとします。ただし、このファイルはまだ作っていません。

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    // config.txt を読む（今いるフォルダに無ければ失敗する）
    let text = fs::read_to_string("config.txt")?;
    println!("設定: {text}");
    Ok(())
}
```

**▼ 実行結果**

```text
Error: Os { code: 2, kind: NotFound, message: "指定されたファイルが見つかりません。" }
error: process didn't exit successfully: `target\debug\practice.exe` (exit code: 1)
```

**何が起きたか:**

- 1行目は、プログラム自身が出したエラーです。`?` で止まった `main` が失敗を返したので、Rust がその中身を表示しました。
  - `Os { code: 2, ... }` … OS が返してきたエラーで、Windows のエラー番号は 2
  - `kind: NotFound` … 種類は「見つからない」
  - `message: "..."` … OS が用意した説明文（日本語版 Windows なので日本語）
- 2行目は Cargo の報告で、プロセスが終了コード 1（失敗）で終わったことを伝えています。

**直し方:** 「ファイルが本当にあるか」と「**今いるフォルダはどこか**」の2つを確かめます。`config.txt` を作るか、ファイルのある場所で実行するか、パスを正しく書き直します。

#### 実行する場所が変わると、相対パスの行き先も変わる

最小コード（`memo\today.txt` を書くプログラム）を、今度は `target\debug` フォルダに移動してから直接実行してみます。

```powershell
cd target\debug
.\practice.exe
```

**▼ 実行結果**

```text
今いるフォルダ: C:\Users\you\rust-study\practice\target\debug
相対パス: memo\today.txt
絶対パス: C:\Users\you\rust-study\practice\target\debug\memo\today.txt
中身: Rust の勉強を始めた
```

エラーにはなりませんが、`memo` フォルダが**予想と違う場所**（`target\debug` の中）にできてしまいました。プログラムは同じでも、カレントディレクトリが変わると相対パスの行き先が変わるからです。

> **つまずきポイント:** 「さっきは動いたのに、今は `NotFound` になる」「保存したはずのファイルが見当たらない」というトラブルの多くは、**カレントディレクトリの取り違え**が原因です。困ったら、まず PowerShell の `PS C:\...>` の部分（今いるフォルダ）を確認しましょう。プログラム側で `env::current_dir()` を表示するのも有効です。

### RustNote ではここで使う

- RustNote のプロジェクトも、`Cargo.toml`・`src` フォルダ（Rust のコード）・`migrations` フォルダ（データベースの表を作る SQL）・`tests` フォルダ（テスト）などの**決まった形のフォルダ構成**を持ちます。詳しい中身は第9巻で作ります。
- 開発中のデータベースは、**ローカルのファイル**として保存します。そのファイルの場所を相対パスで書くと、「どこで `cargo run` したか」によって別のファイルを見てしまうことがあります。RustNote では、場所を設定（環境変数）で決めるようにして、この事故を防ぎます（第9巻）。

### 練習問題

**問題1:** `memo` フォルダの中に `today.txt` と `tomorrow.txt` の2つのファイルを書き込み、`memo` フォルダの中身を一覧表示するプログラムを書いてください。

**問題2:** 次のパスは、絶対パスと相対パスのどちらでしょうか。

- a. `C:\Users\you\rust-study`
- b. `src\main.rs`
- c. `..\hello`
- d. `/home/you/rust-study`（Linux のパス）
- e. `.\Cargo.toml`

**問題3（予想してから実行）:** `practice` フォルダの中で `cd target\debug` してから `cargo run` すると、Cargo はプロジェクトを見つけられるでしょうか。見つけられるなら、最小コードの `memo` フォルダはどこにできるでしょうか。

### 解答と解説

**問題1の解答:**

```rust
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    fs::create_dir_all("memo")?;

    // 2 つのファイルを書く
    fs::write(Path::new("memo").join("today.txt"), "今日の予定\n")?;
    fs::write(Path::new("memo").join("tomorrow.txt"), "明日の予定\n")?;

    // memo フォルダの中身を一覧表示する
    for entry in fs::read_dir("memo")? {
        let entry = entry?;
        println!("{}", entry.path().display());
    }
    Ok(())
}
```

**▼ 実行結果**

```text
memo\today.txt
memo\tomorrow.txt
```

`fs::read_dir("memo")` に渡したのが相対パスなので、表示されるパスも `memo\...` という相対パスになります。

**問題2の解答:** a は**絶対**（ドライブ名 `C:` から始まる）、b は**相対**、c は**相対**（`..` は今いる場所が出発点）、d は**絶対**（Linux では `/` から始まるパスが絶対パス）、e は**相対**（`.` は今いる場所）。

**問題3の解答:** Cargo は、今いるフォルダに `Cargo.toml` が無ければ**親フォルダを順にさかのぼって**探すので、プロジェクトを見つけて実行できます。ただし、プログラムのカレントディレクトリは `target\debug` のままなので、`memo` は `target\debug` の中にできます。

```text
     Running `.\practice.exe`
今いるフォルダ: C:\Users\you\rust-study\practice\target\debug
相対パス: memo\today.txt
絶対パス: C:\Users\you\rust-study\practice\target\debug\memo\today.txt
中身: Rust の勉強を始めた
```

「Cargo がプロジェクトを探す場所」と「プログラムが相対パスを解決する場所」は別物だ、という点がポイントです。

### まとめ

- ファイルはデータ、フォルダは入れ物で、全体は `C:\` を根元とする木（ツリー）になっています。
- 絶対パスは「どこからでも同じ場所」、相対パスは「今いるフォルダ（カレントディレクトリ）からの道順」です。`.` は「ここ」、`..` は「1つ上」。
- 「ファイルが無い」と言われたら、ファイルの有無と**今いるフォルダ**の両方を確かめます。


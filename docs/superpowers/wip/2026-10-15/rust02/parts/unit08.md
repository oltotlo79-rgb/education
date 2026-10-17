## UNIT 08: enum・Option・パターン分解

### この単元のゴール

- `enum` で「いくつかの候補のうち、どれか1つ」を表す型を作れ、候補ごとに違うデータを持たせられる
- `Option<T>`（`Some(値)` か `None`）で「値がないかもしれない」を表し、`match` / `if let` / `let else` / `unwrap_or` で取り出せる
- 構造体・タプル・enum を**パターン**で分解し、ガードや `@`、`..` を使える
- `unwrap()` が panic する場面と、それを避ける書き方を説明できる

### たとえ話でつかむ

- **enum**（イーナム：enumeration ＝ 列挙）は「**信号機**」です。赤・黄・青のうち、**必ずどれか1つ**が点いています。2つ同時に点くことも、どれも点かないこともありません。
- 候補ごとに違うデータを持てるのは「**宅配便の伝票**」に似ています。「通常便」「クール便（温度の欄がある）」「着払い（金額の欄がある）」のように、種類によって記入欄が違います。Rust の enum は、種類（**バリアント**）ごとに違う形のデータを持てます。
- **Option** は「**くじ引きの封筒**」です。開けると「当たり券（`Some(値)`）」か「空っぽ（`None`）」のどちらかが入っています。**開けて確かめないと中身は使えない**のがポイントです。他の言語の `null` のように、うっかり空っぽの封筒の中身を使ってしまう事故を、Rust は型で防ぎます。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="360" y="24" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">enum Command: 5つの候補のうち、値は「どれか1つ」</text>
<rect x="20" y="44" width="124" height="110" rx="10" fill="#f8fafc" stroke="#94a3b8"/>
<text x="82" y="72" text-anchor="middle" font-size="13" font-weight="700" fill="#475569">List</text>
<text x="82" y="112" text-anchor="middle" font-size="12" fill="#94a3b8">データなし</text>
<rect x="156" y="44" width="124" height="110" rx="10" fill="#f8fafc" stroke="#94a3b8"/>
<text x="218" y="72" text-anchor="middle" font-size="13" font-weight="700" fill="#475569">Add</text>
<rect x="170" y="94" width="96" height="30" rx="4" fill="#ffffff" stroke="#cbd5e1"/>
<text x="218" y="114" text-anchor="middle" font-size="12" fill="#64748b">String</text>
<rect x="292" y="34" width="124" height="130" rx="10" fill="#ecfdf5" stroke="#047857" stroke-width="3"/>
<text x="354" y="62" text-anchor="middle" font-size="13" font-weight="700" fill="#047857">Toggle</text>
<rect x="306" y="84" width="96" height="30" rx="4" fill="#ffffff" stroke="#047857"/>
<text x="354" y="104" text-anchor="middle" font-size="14" font-weight="700" fill="#0f172a">1</text>
<text x="354" y="134" text-anchor="middle" font-size="12" fill="#047857">u64</text>
<text x="354" y="154" text-anchor="middle" font-size="12" font-weight="700" fill="#047857">← 今の値</text>
<rect x="428" y="44" width="148" height="110" rx="10" fill="#f8fafc" stroke="#94a3b8"/>
<text x="502" y="72" text-anchor="middle" font-size="13" font-weight="700" fill="#475569">Rename</text>
<rect x="440" y="86" width="124" height="26" rx="4" fill="#ffffff" stroke="#cbd5e1"/>
<text x="502" y="104" text-anchor="middle" font-size="12" fill="#64748b">id: u64</text>
<rect x="440" y="116" width="124" height="26" rx="4" fill="#ffffff" stroke="#cbd5e1"/>
<text x="502" y="134" text-anchor="middle" font-size="12" fill="#64748b">title: String</text>
<rect x="588" y="44" width="112" height="110" rx="10" fill="#f8fafc" stroke="#94a3b8"/>
<text x="644" y="72" text-anchor="middle" font-size="13" font-weight="700" fill="#475569">Quit</text>
<text x="644" y="112" text-anchor="middle" font-size="12" fill="#94a3b8">データなし</text>
<rect x="60" y="186" width="600" height="56" rx="10" fill="#eff6ff" stroke="#3b82f6"/>
<text x="360" y="210" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#1e40af">let cmd = Command::Toggle(1);</text>
<text x="360" y="230" text-anchor="middle" font-size="12" fill="#334155">match cmd { ... } で「どれなのか」を調べ、その候補のデータを取り出す</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-1: enum の値は、候補（バリアント）のうちちょうど1つ。候補ごとに持つデータの形が違ってよい</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-11" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="24" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">Option&lt;usize&gt; は「当たり券入り」か「空っぽ」の封筒</text>
<rect x="40" y="50" width="200" height="90" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="140" y="76" text-anchor="middle" font-size="14" font-weight="700" fill="#047857">Some(1)</text>
<rect x="105" y="90" width="70" height="36" rx="6" fill="#ffffff" stroke="#047857"/>
<text x="140" y="114" text-anchor="middle" font-size="16" font-weight="700" fill="#0f172a">1</text>
<rect x="40" y="160" width="200" height="80" rx="10" fill="#f8fafc" stroke="#94a3b8" stroke-width="2" stroke-dasharray="6 4"/>
<text x="140" y="196" text-anchor="middle" font-size="14" font-weight="700" fill="#475569">None</text>
<text x="140" y="220" text-anchor="middle" font-size="12" fill="#94a3b8">中身なし</text>
<line x1="240" y1="95" x2="330" y2="95" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-11)"/>
<line x1="240" y1="185" x2="330" y2="125" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-11)"/>
<line x1="240" y1="218" x2="330" y2="218" stroke="#ef4444" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-02-11)"/>
<rect x="335" y="50" width="365" height="90" rx="10" fill="#eff6ff" stroke="#3b82f6"/>
<text x="350" y="76" font-size="13" font-family="Consolas, monospace" fill="#1e40af">match / if let で開ける</text>
<text x="350" y="100" font-size="12" fill="#334155">Some(i) =&gt; 中身 i を使う</text>
<text x="350" y="122" font-size="12" fill="#334155">None =&gt; 「見つからない」ときの処理を必ず書く</text>
<rect x="335" y="160" width="365" height="80" rx="10" fill="#fef2f2" stroke="#ef4444"/>
<text x="350" y="186" font-size="13" font-family="Consolas, monospace" fill="#b91c1c">.unwrap() で無理に開ける</text>
<text x="350" y="208" font-size="12" fill="#334155">None に使うと panic でプログラムが停止</text>
<text x="350" y="228" font-size="12" fill="#334155">→ unwrap_or(代わりの値) なら止まらない</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-2: Option は開けて確かめてから使う。None の場合の処理を書かないとコンパイルが通らない</div>
</div>

### まず動かす最小コード

#### 例1: データを持たない enum と match

> **▼ このコードがやること（先に日本語で）:** 一覧の絞り込み条件「すべて・未完了・完了済み」を enum で表し、それぞれを日本語の表示名に変換します。

```rust
@@CODE u08/a_enum.rs@@
```

```powershell
cargo run
```

**▼ 実行結果**

```text
@@OUT u08/a_enum@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: enum の定義と値の作り方

```rust
enum Filter {
    All,
    Open,
    Done,
}
let current = Filter::Open;
```

- `enum 型名 { 候補, 候補, ... }` で定義します。候補を**バリアント**と呼びます。
- 値は `型名::バリアント`（`Filter::Open`）で作ります。`Filter` 型の値は、この3つ以外になりえません。
- 文字列 `"open"` で状態を持つと、`"opne"` のようなつづり間違いがそのまま動いてしまいますが、enum ならコンパイラが止めてくれます。

##### 解説2: match ですべての候補を処理する

```rust
fn label(filter: Filter) -> &'static str {
    match filter {
        Filter::All => "すべて",
        Filter::Open => "未完了",
        Filter::Done => "完了済み",
    }
}
```

- 3つの候補をすべて書いているので `_` は要りません。候補を1つ書き忘れると E0004 になります（下の「よくあるエラー」）。**あとで候補を増やしたとき、直すべき `match` をコンパイラが全部教えてくれる**のが enum の強みです。
- 戻り値の `&'static str` は「プログラムの終わりまでずっと存在する文字列（リテラル）を貸す」という型です。`'static` の意味は第3巻で詳しく学びます。

##### 解説3: derive で比較とコピーを付ける

- `#[derive(Debug, Clone, Copy, PartialEq)]` で、`{:?}` 表示（`Open` と表示された）、コピー、`==` での比較ができるようになります。`label(current)` の後も `current` を使えるのは `Copy` のおかげです。

#### 例2: データを持つ enum

> **▼ このコードがやること（先に日本語で）:** メモアプリへの操作を「一覧・追加・切り替え・改名・終了」の5種類の enum で表し、それぞれの中身を取り出して説明文を作ります。

```rust
@@CODE u08/b_enum_data.rs@@
```

**▼ 実行結果**

```text
@@OUT u08/b_enum_data@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: バリアントごとに違う形のデータ

```rust
enum Command {
    List,
    Add(String),
    Toggle(u64),
    Rename { id: u64, title: String },
    Quit,
}
```

- `Add(String)` はタプル構造体のように中身を1つ持つバリアント、`Rename { id, title }` は構造体のように名前付きのフィールドを持つバリアントです（図8-1）。
- 「追加ならタイトル、切り替えなら ID」のように、**その場合に必要なデータだけ**を持たせられます。「ID が 0 なら追加の意味」のような、まぎらわしい決まりが不要になります。

##### 解説2: match で中身を取り出す

```rust
Command::Add(title) => format!("「{title}」を追加"),
Command::Rename { id, title } => format!("#{id} を「{title}」に改名"),
```

- パターン `Command::Add(title)` に合うと、中の値が `title` という名前で使えます。これを**パターンによる分解**と呼びます。
- `cmd` は `&Command`（借り物）なので、`title` も借り物の `&String` として取り出されます。

#### 例3: Option で「見つからないかもしれない」を表す

> **▼ このコードがやること（先に日本語で）:** キーワードを含むタイトルの位置を探す関数を作ります。結果は「見つかった位置」か「見つからない」のどちらかです。その結果を5通りのやり方で扱います。

```rust
@@CODE u08/c_option.rs@@
```

**▼ 実行結果**

```text
@@OUT u08/c_option@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: Option を返す関数

```rust
fn find_title(titles: &[&str], keyword: &str) -> Option<usize> {
    for (i, title) in titles.iter().enumerate() {
        if title.contains(keyword) {
            return Some(i);
        }
    }
    None
}
```

- `Option<usize>` は「`usize` があるかもしれないし、ないかもしれない」型です。標準ライブラリで次のように定義された、ただの enum です。

```text
enum Option<T> {
    Some(T),
    None,
}
```

- `T` は「何の型でもよい」という置き場所（**ジェネリクス**。第4巻で学びます）です。見つかったら `Some(i)`、最後まで見つからなければ `None` を返します。

##### 解説2: 取り出し方の使い分け

| 書き方 | 意味 | 使う場面 |
|---|---|---|
| `match x { Some(v) => …, None => … }` | 両方の場合を書く | 両方で処理が違うとき（基本） |
| `if let Some(v) = x { … }` | Some のときだけ処理 | None なら何もしないとき |
| `x.unwrap_or(既定値)` | None なら既定値 | 代わりの値が決まっているとき |
| `x.map(変換する関数)` | Some の中身だけ変換 | 中身を加工して、Option のまま渡すとき |
| `x.is_some()` / `x.is_none()` | あるか・ないか | 中身は要らないとき |
| `x.unwrap()` | Some なら中身、None なら **panic** | 「絶対に Some」と言い切れるときだけ |

- `map(|i| i + 1)` の `|i| i + 1` は「i を受け取って i + 1 を返す」小さな関数（**クロージャ**）です。UNIT 10 で詳しく学びます。

#### 例4: let else で「なければ早期リターン」

> **▼ このコードがやること（先に日本語で）:** `page= 3` のような「名前=値」の文字列から値の部分を取り出します。`=` がなければ「取り出せない」を返します。

```rust
@@CODE u08/d_let_else.rs@@
```

**▼ 実行結果**

```text
@@OUT u08/d_let_else@@
```

- `let パターン = 値 else { ... };` は、**パターンに合えば変数を作って先へ進み、合わなければ `else` の中で関数を抜ける**書き方です（`else` の中は必ず `return` などで抜ける必要があります）。
- UNIT 03 の早期リターンと同じく、「だめな場合を先に片付ける」形になり、正常な処理の字下げが深くなりません。
- `split_once('=')` は最初の `=` で2つに分けた `Some((前, 後))` を返し、`=` がなければ `None` を返します。`_key` の `_` は「使わない変数」の印です。
- UNIT 09 で学ぶ `?` を使うと、同じことを `let (_key, value) = pair.split_once('=')?;` の1行で書けます（`cargo clippy` もそう提案します）。`let else` は、`None` のときに `return None` 以外の処理（メッセージを出す、`continue` するなど）をしたい場面で特に役立ちます。

#### 例5: いろいろなパターン

> **▼ このコードがやること（先に日本語で）:** 座標を表す構造体を分解したり、形で場合分けしたりします。範囲に合った値に名前を付ける `@` と、パターンに合うかを `true` / `false` で返す `matches!` も試します。

```rust
@@CODE u08/e_patterns.rs@@
```

**▼ 実行結果**

```text
@@OUT u08/e_patterns@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 構造体のパターン

```rust
Point { x: 0, y: 0 } => "原点",
Point { x: 0, .. } => "y 軸の上",
Point { y, .. } if y < 0 => "x 軸より下",
```

- `Point { x: 0, y: 0 }` は「x が 0 かつ y が 0」に合います。`..` は「残りのフィールドは何でもよい」です。
- `Point { y, .. }` は y の値に `y` という名前を付けます。後ろの `if y < 0` は UNIT 04 のガードです。

##### 解説2: `@` と `matches!`

- `n @ 0..=17` は「0〜17 の範囲に合ったら、その値に `n` という名前を付ける」です。範囲で調べながら値も使いたいときに便利です。
- `matches!(c, 'ぁ'..='ゖ')` は「`c` がひらがなの範囲に入るか」を調べます。`match` を短く書いたもので、合えば `true` を返すので、`if` の条件に使えます。

> **ヒント:** パターンは `match` だけでなく、`let`（例: `let (id, title) = pair;`）、`if let`、`let else`、`for`（例: `for (i, v) in ...`）、関数の引数でも使えます。どこでも「形を合わせて、中身に名前を付ける」という同じ考え方です。

### もう一歩: 実務に近い形

ユーザーが入力した1行の文字列を、enum の命令に変換する「パーサー（解析器）」を作ります。文字列のままだと処理のたびに解釈し直す必要がありますが、**入口で enum に変換しておけば、その後の処理は `match` だけで安全に書けます**。UNIT 10 で作る CLI でこの関数を使います。

#### 入力文字列を命令に変換する

> **▼ このコードがやること（先に日本語で）:** `list`、`add タイトル`、`done 番号`、`quit` の4種類の命令を解釈します。タイトルのない `add` や、数でない `done`、知らない命令は「解釈できません」とします。

```rust
@@CODE u08/f_parse_command.rs@@
```

**▼ 実行結果**

```text
@@OUT u08/f_parse_command@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 命令と残りに分ける

```rust
let (word, rest) = line.split_once(' ').unwrap_or((line, ""));
```

- `split_once(' ')` は最初の空白で `Some(("add", "牛乳を買う"))` のように分けます。`list` のように空白がないと `None` なので、`unwrap_or((line, ""))` で「全体が命令、残りは空」とみなします。

##### 解説2: 文字列で match し、ガードで条件を足す

```rust
match word {
    "list" => Some(Command::List),
    "add" if !rest.is_empty() => Some(Command::Add(rest.to_string())),
    "done" => match rest.parse::<u64>() {
        Ok(id) => Some(Command::Done(id)),
        Err(_) => None,
    },
    _ => None,
}
```

- `&str` も `"list"` のような文字列のパターンで `match` できます。
- `"add" if !rest.is_empty()` は「add で、かつタイトルが空でない」ときだけ合います。空なら下の `_` に落ちて `None` です。
- `parse` の結果は `Ok(値)` か `Err(理由)` です（UNIT 09 で学ぶ `Result`）。ここでは、失敗の理由は使わないので `Err(_)` と書いています。

##### 解説3: 入れ子のパターン

```rust
Some(Command::Add(title)) => format!("「{title}」を追加"),
```

- 「Option の中の Command の中の String」まで、1つのパターンで一度に分解しています。パターンは入れ子にできます。

### よくあるエラーと読み方

#### エラー1: E0004 enum の候補を書き忘れた

```rust
@@CODE u08/err1_e0004.rs@@
```

```text
@@ERR u08/err1_e0004@@
```

- **読み方:** `pattern Filter::Done not covered`（`Filter::Done` が扱われていません）。`note` で enum の定義の中の `Done` を指し、`help` で足すべき腕を示しています。
- **直し方:** `Filter::Done => "完了済み",` を足します。`_ =>` で済ませることもできますが、そうすると**将来候補を増やしたときに教えてもらえなくなる**ので、enum では候補をすべて書くのがおすすめです。

#### エラー2: None を unwrap して panic

```rust
@@CODE u08/err2_unwrap.rs@@
```

**▼ 実行結果**

```text
@@RUN u08/err2_unwrap@@
```

- **読み方:** `called Option::unwrap() on a None value`（`None` の値に対して `unwrap()` が呼ばれました）。場所は `src\main.rs:12:41` です。
- **何が起きたか:** 「旅行」は見つからないので `None` でした。`unwrap()` は「絶対に Some のはず」という前提で中身を取り出すので、前提が崩れると panic します。
- **直し方:** `match` や `if let` で `None` の場合を書きます。どうしても `unwrap` 相当を使うなら、`expect("…ので必ず見つかる")` と**なぜ Some と言えるのか**をメッセージに書いておくと、万一のときに原因を追いやすくなります。

> **つまずきポイント:** 練習中は `unwrap()` が手軽で多用しがちですが、サーバーで panic すると、その処理（リクエスト）は異常終了します。**外から来たデータに対する `unwrap()` は書かない**、と決めておきましょう。

#### エラー3: E0308 Some で包み忘れた

```rust
@@CODE u08/err3_some.rs@@
```

```text
@@ERR u08/err3_some@@
```

- **読み方:** `expected Option<u32>, found u32`。戻り値を `Option<u32>` と約束したのに、`n / 2`（`u32`）を返しています。
- **直し方:** `help: try wrapping the expression in Some` のとおり `Some(n / 2)` にします。`Option<u32>` と `u32` は**別の型**で、自動では変換されません。

#### エラー4: E0369 Option のまま計算した

```rust
@@CODE u08/err4_option_add.rs@@
```

```text
@@ERR u08/err4_option_add@@
```

- **読み方:** `cannot add {integer} to Option<i32>`（`Option<i32>` に整数は足せません）。封筒のまま足し算はできない、ということです。
- **直し方:** 中身を取り出してから計算します。値がないときに 0 とみなしてよいなら `stock.unwrap_or(0) + 1`、Option のまま結果を返したいなら `stock.map(|n| n + 1)` です。

### RustNote ではここで使う

RustNote の一覧ページの URL には、検索語 `?q=` が**付いていることも、付いていないこともあります**。これはまさに `Option<&str>` です。さらに「空白だけの検索語」は検索なしとみなすため、受け取った直後に `Option<String>` へ整えます。

> **▼ このコードがやること（先に日本語で）:** 検索語が「付いていない」「空白だけ」「` Rust `」の3通りについて、検索なしで全件表示するか、検索して件数を数えるかを決めます。

```rust
@@CODE u08/g_rustnote.rs@@
```

**▼ 実行結果**

```text
@@OUT u08/g_rustnote@@
```

- 入口で「検索あり（`Some(検索語)`）」か「検索なし（`None`）」に整えておけば、その後は `match` で2通りを書くだけです。空白だけの検索語がデータベースまで届いて、全件に合ってしまう、といった事故を防げます。
- 本物の RustNote では、一覧の絞り込み（すべて・未完了・完了済み）にも enum を使えます。候補が増えたときに、表示や検索条件の `match` の直し漏れをコンパイラが教えてくれます。

### 練習問題

**問題1.** 例1の `Filter` に `Archived`（表示名「アーカイブ」）を追加してください。追加しただけでは `label` がエラーになるので、それも直してください。

**問題2.** 文字列の最初の1文字を返す関数 `fn first_char(text: &str) -> Option<char>` を書き、`"Rust"`、`"日本語"`、`""` で試してください。空文字列のときは「空です」と表示します。

**問題3.** `"all"` / `"open"` / `"done"` という文字列を `Filter` に変換する関数 `fn parse_filter(text: &str) -> Option<Filter>` を書いてください。それ以外の文字列なら `None` です。

### 解答と解説

**解答1.**

```rust
@@CODE u08/ex1.rs@@
```

**▼ 実行結果**

```text
@@OUT u08/ex1@@
```

- バリアントを追加すると、`label` の `match` が E0004 で「`Filter::Archived` が扱われていない」と教えてくれます。これが enum と `match` の組み合わせの安心感です。

**解答2.**

```rust
@@CODE u08/ex2.rs@@
```

**▼ 実行結果**

```text
@@OUT u08/ex2@@
```

- `chars().next()` は「次の1文字」を返し、文字がなければ `None` です。UNIT 05 の `s[0]` の代わりになる、安全な書き方です。

**解答3.**

```rust
@@CODE u08/ex3.rs@@
```

**▼ 実行結果**

```text
@@OUT u08/ex3@@
```

- 外から来た文字列を、入口で enum に変換するパターンです。`None` が返ったら「不正な指定」として既定値（`Filter::All`）を使う、などの判断を呼ぶ側で行います。

### まとめ

- `enum` は「候補のうちどれか1つ」を表す型。候補ごとに違うデータを持て、`match` で全候補を網羅して扱う。
- `Option<T>` は `Some(値)` か `None`。`match` / `if let` / `let else` / `unwrap_or` / `map` で、`None` の場合を必ず考えて取り出す。`unwrap()` は None で panic する。
- パターンは `match` だけでなく `let` や `for` でも使え、構造体・タプル・enum を入れ子のまま分解できる。

## UNIT 10: HashMap・Iterator・closure・モジュール

### この単元のゴール

- `HashMap` でキーから値を引く表を作り、追加・取得・上書き・削除・集計（`entry`）ができる
- **クロージャ**（`|x| x * 2`）で「処理そのもの」を値として書き、関数に渡せる
- **イテレータ**の `filter` → `map` → `collect` のつながりで、並びを加工・集計できる。「イテレータは怠け者」の意味を説明できる
- `mod` / `pub` / `use` でコードをモジュールに分け、ファイルを分割できる。`pub` を付けないことで中身を守れる

### たとえ話でつかむ

- **HashMap**（ハッシュマップ）は「**駅のコインロッカー**」です。鍵の番号（**キー**）さえ分かれば、何百個ロッカーがあっても、すぐに荷物（**値**）を取り出せます。ただし、ロッカーの並び順に意味はありません。
- **クロージャ**は「**その場で書く小さなメモ付きの手順書**」です。「値段に 1.1 を掛ける」のような短い手順をその場で書いて、人（関数）に渡せます。手順書には、周りにある情報（税率など）を書き写して持たせられます。
- **イテレータ**は「**工場のベルトコンベア**」です。部品が1つずつ流れてきて、「検品（`filter`）→ 加工（`map`）→ 箱詰め（`collect`）」と各工程を通ります。コンベアは**最後の箱詰め係が「次をください」と言うまで動きません**。これが「イテレータは怠け者（lazy）」という意味です。
- **モジュール**は「**家の部屋割り**」です。台所・寝室のように用途ごとに部屋を分け、玄関（`pub`）から入れる部屋と、家族だけが入れる部屋を区別します。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-14" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="24" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">HashMap&lt;u64, String&gt;: キーからすぐに値の場所が分かる</text>
<rect x="20" y="50" width="120" height="36" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="80" y="74" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#c2410c">get(&amp;2)</text>
<rect x="20" y="150" width="120" height="36" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="80" y="174" text-anchor="middle" font-size="14" font-family="Consolas, monospace" fill="#c2410c">get(&amp;9)</text>
<rect x="190" y="90" width="120" height="56" rx="10" fill="#f8fafc" stroke="#475569"/>
<text x="250" y="114" text-anchor="middle" font-size="13" fill="#0f172a">ハッシュ関数</text>
<text x="250" y="134" text-anchor="middle" font-size="12" fill="#64748b">キー → 棚の番号</text>
<line x1="140" y1="68" x2="186" y2="104" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-14)"/>
<line x1="140" y1="168" x2="186" y2="134" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-14)"/>
<rect x="370" y="44" width="330" height="36" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="385" y="67" font-size="13" fill="#047857">[棚0] 2 → "Rust の勉強"</text>
<rect x="370" y="86" width="330" height="36" rx="6" fill="#ffffff" stroke="#cbd5e1"/>
<text x="385" y="109" font-size="13" fill="#94a3b8">[棚1] （空き）</text>
<rect x="370" y="128" width="330" height="36" rx="6" fill="#ffffff" stroke="#475569"/>
<text x="385" y="151" font-size="13" fill="#0f172a">[棚2] 1 → "牛乳を買う"</text>
<rect x="370" y="170" width="330" height="36" rx="6" fill="#ffffff" stroke="#cbd5e1"/>
<text x="385" y="193" font-size="13" fill="#94a3b8">[棚3] （空き）</text>
<line x1="310" y1="110" x2="366" y2="64" stroke="#047857" stroke-width="2" marker-end="url(#ar-02-14)"/>
<line x1="310" y1="126" x2="366" y2="186" stroke="#94a3b8" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar-02-14)"/>
<text x="250" y="200" text-anchor="middle" font-size="12" fill="#047857">2 → 棚0 に Some(&amp;値)</text>
<text x="250" y="220" text-anchor="middle" font-size="12" fill="#64748b">9 → 棚3 は空き → None</text>
<text x="535" y="236" text-anchor="middle" font-size="12" fill="#64748b">棚の順番はキーの大小と関係ない（表示は並べ替える）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図10-1: HashMap はキーから棚の番号を計算するので、全部を探さなくても値が見つかる（実際の棚の数や番号はもっと複雑）</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 210" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-15" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="24" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">numbers.iter().copied().filter(..).map(..).collect()</text>
<rect x="10" y="50" width="130" height="80" rx="10" fill="#f8fafc" stroke="#475569"/>
<text x="75" y="74" text-anchor="middle" font-size="12" font-weight="700" fill="#475569">元の Vec</text>
<text x="75" y="100" text-anchor="middle" font-size="13" font-family="Consolas, monospace" fill="#0f172a">1 2 3</text>
<text x="75" y="118" text-anchor="middle" font-size="13" font-family="Consolas, monospace" fill="#0f172a">4 5 6</text>
<rect x="190" y="50" width="150" height="80" rx="10" fill="#fff7ed" stroke="#f97316"/>
<text x="265" y="74" text-anchor="middle" font-size="12" font-weight="700" fill="#c2410c">filter（検品）</text>
<text x="265" y="96" text-anchor="middle" font-size="12" font-family="Consolas, monospace" fill="#0f172a">|n| *n % 2 == 0</text>
<text x="265" y="118" text-anchor="middle" font-size="12" fill="#64748b">偶数だけ通す</text>
<rect x="390" y="50" width="150" height="80" rx="10" fill="#eff6ff" stroke="#3b82f6"/>
<text x="465" y="74" text-anchor="middle" font-size="12" font-weight="700" fill="#1e40af">map（加工）</text>
<text x="465" y="96" text-anchor="middle" font-size="12" font-family="Consolas, monospace" fill="#0f172a">|n| n * 10</text>
<text x="465" y="118" text-anchor="middle" font-size="12" fill="#64748b">10 倍にする</text>
<rect x="590" y="50" width="120" height="80" rx="10" fill="#ecfdf5" stroke="#10b981"/>
<text x="650" y="74" text-anchor="middle" font-size="12" font-weight="700" fill="#047857">collect（箱詰め）</text>
<text x="650" y="104" text-anchor="middle" font-size="13" font-family="Consolas, monospace" fill="#0f172a">[20, 40, 60]</text>
<line x1="140" y1="90" x2="186" y2="90" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-15)"/>
<line x1="340" y1="90" x2="386" y2="90" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-15)"/>
<line x1="540" y1="90" x2="586" y2="90" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-15)"/>
<text x="165" y="150" text-anchor="middle" font-size="12" fill="#64748b">1〜6</text>
<text x="365" y="150" text-anchor="middle" font-size="12" fill="#64748b">2, 4, 6</text>
<text x="565" y="150" text-anchor="middle" font-size="12" fill="#64748b">20, 40, 60</text>
<text x="360" y="184" text-anchor="middle" font-size="13" fill="#334155">要素は1つずつ全工程を流れる。collect が「次をください」と頼むまで、どの工程も動かない</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図10-2: イテレータのパイプライン。途中の工程（アダプタ）をつなぎ、最後の消費メソッドで結果を受け取る</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-16" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="24" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">rustnote_mini のモジュールの木（crate = このプログラム全体）</text>
<rect x="260" y="40" width="200" height="56" rx="10" fill="#f8fafc" stroke="#475569" stroke-width="2"/>
<text x="360" y="64" text-anchor="middle" font-size="14" font-weight="700" fill="#0f172a">crate（src/main.rs）</text>
<text x="360" y="84" text-anchor="middle" font-size="12" font-family="Consolas, monospace" fill="#475569">mod note; mod store;</text>
<line x1="320" y1="96" x2="200" y2="130" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-16)"/>
<line x1="400" y1="96" x2="520" y2="130" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-16)"/>
<rect x="30" y="134" width="320" height="120" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="190" y="158" text-anchor="middle" font-size="14" font-weight="700" fill="#1e40af">note（src/note.rs）</text>
<text x="50" y="184" font-size="12" fill="#047857">pub NoteId / pub Note / pub TitleError</text>
<text x="50" y="206" font-size="12" fill="#047857">pub NoteTitle::parse / as_str</text>
<text x="50" y="230" font-size="12" fill="#c2410c">NoteTitle の中身の String（非公開）</text>
<rect x="370" y="134" width="320" height="120" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="530" y="158" text-anchor="middle" font-size="14" font-weight="700" fill="#047857">store（src/store.rs）</text>
<text x="390" y="184" font-size="12" fill="#047857">pub NoteStore</text>
<text x="390" y="206" font-size="12" fill="#047857">pub add / toggle / remove / list / search</text>
<text x="390" y="230" font-size="12" fill="#c2410c">notes: HashMap / last_id（非公開）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図10-3: 緑の項目（pub）は外から使える玄関、橙の項目はモジュールの中だけで触れる部屋</div>
</div>

### まず動かす最小コード

#### 例1: HashMap の基本操作

> **▼ このコードがやること（先に日本語で）:** ノートの ID からタイトルを引く表を作ります。値の取り出し・存在の確認・上書き・削除を試し、最後に ID の順に並べて表示します。

```rust
use std::collections::HashMap;

fn main() {
    // キー（ID）から値（タイトル）を引く表を作る
    let mut titles: HashMap<u64, String> = HashMap::new();
    titles.insert(1, String::from("牛乳を買う"));
    titles.insert(2, String::from("Rust の勉強"));
    titles.insert(3, String::from("部屋の掃除"));

    // get は、キーがあれば Some(&値)、なければ None
    println!("1: {:?}", titles.get(&1));
    println!("9: {:?}", titles.get(&9));
    println!("2 はある? {}", titles.contains_key(&2));

    // 同じキーで insert すると上書きされ、前の値が Some で返る
    let old = titles.insert(2, String::from("Rust の復習"));
    println!("上書き前の値: {old:?}");

    // remove は、取り除いた値を返す
    let removed = titles.remove(&3);
    println!("削除: {removed:?}, 残り {} 件", titles.len());

    // HashMap の順番は決まっていないので、表示するときはキーで並べ替える
    let mut ids: Vec<&u64> = titles.keys().collect();
    ids.sort();
    for id in ids {
        println!("#{id} {}", titles[id]);
    }
}
```

```powershell
cargo run
```

**▼ 実行結果**

```text
1: Some("牛乳を買う")
9: None
2 はある? true
上書き前の値: Some("Rust の勉強")
削除: Some("部屋の掃除"), 残り 2 件
#1 牛乳を買う
#2 Rust の復習
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `use std::collections::HashMap;`

- `HashMap` は標準ライブラリの `collections`（コレクション：データの入れ物）モジュールにあります。`Vec` や `String` と違って最初から使える状態ではないので、`use` で取り込みます（忘れたときのエラーは下の「よくあるエラー」）。

##### 解説2: `HashMap<K, V>` と主なメソッド

| メソッド | 何をするか | 戻り値 |
|---|---|---|
| `insert(k, v)` | 追加。同じキーがあれば上書き | 前の値（`Option<V>`） |
| `get(&k)` | 値を借りて取り出す | `Option<&V>` |
| `contains_key(&k)` | キーがあるか | `bool` |
| `remove(&k)` | 取り除く | 取り除いた値（`Option<V>`） |
| `len()` / `is_empty()` | 件数 / 空か | `usize` / `bool` |
| `keys()` / `values()` / `iter()` | キー / 値 / 組を順に取り出す | イテレータ |

- `get` や `remove` には `&1` のように**キーを借りて**渡します。見つからなければ `None` なので、UNIT 08 の取り出し方がそのまま使えます。
- `titles[id]` のように `[ ]` でも取り出せますが、キーがないと panic します。確実にあるときだけ使います。

##### 解説3: 順番は決まっていない

- HashMap は、キーから計算した「棚の番号」に値を置くので（図10-1）、**取り出す順番はキーの大小とも、入れた順とも関係ありません**。実行するたびに変わることもあります。
- 順番に表示したいときは、キーを `Vec` に集めて `sort` します。最初から順番が必要なら、次の例の `BTreeMap`（キーの順に並ぶ表）を使います。

#### 例2: entry で数を数える

> **▼ このコードがやること（先に日本語で）:** タグの並びから、タグごとの出現回数を数えます。最後に、タグ名の順に並べて表示します。

```rust
use std::collections::{BTreeMap, HashMap};

fn main() {
    let tags = ["rust", "web", "rust", "db", "rust", "web"];

    let mut counts: HashMap<&str, u32> = HashMap::new();
    for tag in tags {
        // entry(キー).or_insert(0) は「なければ 0 を入れて」、その値の場所を返す
        *counts.entry(tag).or_insert(0) += 1;
    }
    println!("rust の件数: {}", counts["rust"]);

    // BTreeMap に入れ直すと、キーの順に並ぶ
    let sorted: BTreeMap<&str, u32> = counts.into_iter().collect();
    println!("{sorted:?}");
}
```

**▼ 実行結果**

```text
rust の件数: 3
{"db": 1, "rust": 3, "web": 2}
```

- `counts.entry(tag).or_insert(0)` は「`tag` の欄がなければ 0 を入れて、その欄（値の場所）を返す」です。先頭の `*` で欄の中身そのものに `+= 1` しています。「あれば足す、なければ作る」が1行で書けます。
- `counts.into_iter().collect()` で、HashMap の中身を `BTreeMap` に移し替えています。`collect` は「何に集めるか」を左辺の型から読み取ります。

#### 例3: クロージャ

> **▼ このコードがやること（先に日本語で）:** 2倍にする処理、税込みにする処理をクロージャとして作って呼び出します。続いて「処理を2回くり返す関数」に、いろいろなクロージャを渡します。

```rust
// f は「i32 を受け取って i32 を返すもの」なら何でもよい
fn apply_twice(f: impl Fn(i32) -> i32, value: i32) -> i32 {
    f(f(value))
}

fn main() {
    // クロージャ: |引数| 式 の形で、その場で作る小さな関数
    let double = |x: i32| x * 2;
    println!("{}", double(21));

    // 周りの変数（tax_rate）を取り込んで使える
    let tax_rate = 10;
    let with_tax = |price: u32| price * (100 + tax_rate) / 100;
    println!("{}", with_tax(1_000));

    // 関数に「処理そのもの」を渡す
    println!("{}", apply_twice(double, 5));
    println!("{}", apply_twice(|x| x + 100, 5));
}
```

**▼ 実行結果**

```text
42
1100
20
205
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: クロージャの書き方

```rust
let double = |x: i32| x * 2;
```

- `|引数| 式` がクロージャです。`fn double(x: i32) -> i32 { x * 2 }` とほぼ同じ意味ですが、**名前を付けずにその場で書け**、変数に入れたり関数に渡したりできます。
- 引数や戻り値の型は、多くの場合省略できます（使われ方から推論されます）。本体が複数行なら `|x| { ... }` と波かっこで囲みます。

##### 解説2: 周りの変数を取り込む

```rust
let tax_rate = 10;
let with_tax = |price: u32| price * (100 + tax_rate) / 100;
```

- `with_tax` の中で、外側の `tax_rate` を使っています。ふつうの関数（`fn`）ではできないことで、これがクロージャ（closure：閉じ込める）という名前の由来です。

##### 解説3: クロージャを受け取る関数

```rust
fn apply_twice(f: impl Fn(i32) -> i32, value: i32) -> i32 {
    f(f(value))
}
```

- `impl Fn(i32) -> i32` は「i32 を受け取って i32 を返す、呼び出せるもの」という型です。クロージャでも、名前付きの関数でも渡せます。
- このように**処理を引数として受け取る**ことで、「何をするか」を呼ぶ側が決められます。イテレータの `filter` や `map` も、この仕組みでクロージャを受け取っています。

#### 例4: イテレータで加工・集計する

> **▼ このコードがやること（先に日本語で）:** 1〜6 の数から偶数だけを選んで10倍した Vec を作ります。合計・条件に合う個数・最初に合う値・「1つでも合うか」「全部合うか」も調べます。

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // 1つずつ取り出す → 偶数だけ残す → 10 倍する → Vec に集める
    let result: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|n| *n % 2 == 0)
        .map(|n| n * 10)
        .collect();
    println!("{result:?}");

    // 合計・個数
    let sum: i32 = numbers.iter().sum();
    let big = numbers.iter().copied().filter(|n| *n > 3).count();
    println!("合計 {sum}, 3 より大きいのは {big} 個");

    // find は条件に合う最初の1つ（Option）を返す
    println!("{:?}", numbers.iter().copied().find(|n| *n > 4));

    // any: 1つでも合うか / all: 全部合うか
    let any_big = numbers.iter().copied().any(|n| n > 5);
    let all_positive = numbers.iter().copied().all(|n| n > 0);
    println!("{any_big} {all_positive}");

    // 元の Vec は変わらない
    println!("{numbers:?}");
}
```

**▼ 実行結果**

```text
[20, 40, 60]
合計 21, 3 より大きいのは 3 個
Some(5)
true true
[1, 2, 3, 4, 5, 6]
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: つなげて書く（パイプライン）

```rust
let result: Vec<i32> = numbers
    .iter()
    .copied()
    .filter(|n| *n % 2 == 0)
    .map(|n| n * 10)
    .collect();
```

- `iter()` で「要素を1つずつ借りて取り出すイテレータ」を作り、`copied()` で借りた値をコピーして `i32` の値そのものにします。
- `filter` は条件が `true` の要素だけを通し、`map` は各要素を変換し、`collect` で集めます（図10-2）。
- `filter` と `find` のクロージャには、要素を**のぞき見るための参照**（`&i32`）が渡されます。そのため `*n` と書いて値に戻してから比べます。`map`・`any`・`all` には値そのもの（`i32`）が渡るので `*` は要りません。

##### 解説2: 途中の工程と、最後の工程

| 種類 | 例 | 役割 |
|---|---|---|
| アダプタ（途中の工程） | `filter` `map` `enumerate` `take(n)` `skip(n)` `rev` `copied` | 流れを加工して、新しいイテレータを返す |
| 消費メソッド（最後の工程） | `collect` `sum` `count` `find` `any` `all` `max` `min` | 流れを最後まで（または見つかるまで）動かして、結果を返す |

- `numbers` 自体は最後まで変わっていません。イテレータは元の Vec を借りて読むだけです。

#### 例5: イテレータは怠け者

> **▼ このコードがやること（先に日本語で）:** `map` の中に「処理中」と表示する文を入れておき、それがいつ表示されるかを確かめます。

```rust
fn main() {
    let titles = ["牛乳を買う", "Rust の勉強", "部屋の掃除"];

    // map を書いただけでは、まだ何も実行されない
    let lengths = titles.iter().map(|t| {
        println!("  「{t}」を処理中");
        t.chars().count()
    });
    println!("まだ何も処理していない");

    // collect で取り出し始めたときに、1つずつ処理が流れる
    let lengths: Vec<usize> = lengths.collect();
    println!("{lengths:?}");
}
```

**▼ 実行結果**

```text
まだ何も処理していない
  「牛乳を買う」を処理中
  「Rust の勉強」を処理中
  「部屋の掃除」を処理中
[5, 8, 5]
```

- `map(...)` を書いた時点では何も表示されず、「まだ何も処理していない」が先に出ました。`collect` が要素を求めたときに初めて、1つずつ `map` の中身が実行されています。
- この性質のおかげで、`take(3)` のように「最初の3つだけ」を求めれば、残りは処理されずに済みます。逆に、**消費メソッドを付け忘れると何も起きません**（下の「よくあるエラー」の警告）。

#### 例6: モジュールと pub

> **▼ このコードがやること（先に日本語で）:** タイトルの型 `NoteTitle` と検証の関数を `note` モジュールにまとめ、外から使ってよいものにだけ `pub` を付けます。`main` からは `parse` を通してしか `NoteTitle` を作れないことを確かめます。

```rust
// mod で、関係するものを1つの部屋（モジュール）にまとめる
mod note {
    // pub を付けたものだけが、部屋の外から使える
    pub struct NoteTitle(String);

    impl NoteTitle {
        pub fn parse(raw: &str) -> Option<Self> {
            let title = raw.trim();
            if is_valid_length(title) {
                Some(Self(title.to_string()))
            } else {
                None
            }
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    // pub がないので、部屋の外からは呼べない（部屋の中だけの道具）
    fn is_valid_length(title: &str) -> bool {
        (1..=80).contains(&title.chars().count())
    }
}

// use で、長い住所（note::NoteTitle）を短い名前で書けるようにする
use note::NoteTitle;

fn main() {
    match NoteTitle::parse("  牛乳を買う ") {
        Some(title) => println!("OK: {}", title.as_str()),
        None => println!("NG"),
    }
    println!("空のタイトルは作れない: {}", NoteTitle::parse("").is_none());
}
```

**▼ 実行結果**

```text
OK: 牛乳を買う
空のタイトルは作れない: true
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `mod` と `pub`

- `mod note { ... }` は `note` という名前のモジュール（部屋）を作ります。中に書いたものは、**何も付けなければ部屋の外から見えません**（非公開が既定）。
- 外から使わせたい型や関数にだけ `pub`（パブリック：公開）を付けます。`is_valid_length` は部屋の中の道具なので `pub` を付けていません。

##### 解説2: 中身を非公開にして「正しい値」を守る

```rust
pub struct NoteTitle(String);
```

- 型 `NoteTitle` は `pub` ですが、中身の `String` には `pub` がありません。そのため、部屋の外で `note::NoteTitle(String::new())` と直接作ることはできず、`NoteTitle::parse` を通るしかありません。UNIT 09 で残っていた「空のタイトルも直接作れてしまう」穴が、これでふさがりました。

##### 解説3: `use` と住所（パス）

- 部屋の中のものは `note::NoteTitle` のように `::` でつないだ**パス**（住所）で指します。`use note::NoteTitle;` と書くと、以後は `NoteTitle` だけで書けます。
- `std::collections::HashMap` も同じ仕組みで、「標準ライブラリ（`std`）の `collections` 部屋の `HashMap`」という住所です。

### もう一歩: 実務に近い形

一覧画面の上に出す「集計」を、イテレータとクロージャで書いてみます。`for` と `if` で書くこともできますが、**「何を選んで、どう変えて、何に集めるか」が1つの流れとして読める**のがイテレータの良さです。

#### ノートの一覧から集計を作る

> **▼ このコードがやること（先に日本語で）:** 5件のノートから「未完了のタイトル一覧」「タグごとの件数」「完了率」「『買』を含むタイトルの番号付き一覧」を作って表示します。

```rust
use std::collections::BTreeMap;

struct Note {
    title: String,
    tag: String,
    done: bool,
}

impl Note {
    fn new(title: &str, tag: &str, done: bool) -> Self {
        Self {
            title: title.to_string(),
            tag: tag.to_string(),
            done,
        }
    }
}

fn main() {
    let notes = vec![
        Note::new("牛乳を買う", "買い物", true),
        Note::new("Rust の勉強", "勉強", false),
        Note::new("卵を買う", "買い物", false),
        Note::new("Topcoat を調べる", "勉強", false),
        Note::new("部屋の掃除", "家事", true),
    ];

    // 未完了のノートのタイトルだけを集める
    let open: Vec<&str> = notes
        .iter()
        .filter(|n| !n.done)
        .map(|n| n.title.as_str())
        .collect();
    println!("未完了: {open:?}");

    // タグごとの件数（BTreeMap なのでタグ名の順に並ぶ）
    let mut per_tag: BTreeMap<&str, usize> = BTreeMap::new();
    for note in &notes {
        *per_tag.entry(note.tag.as_str()).or_insert(0) += 1;
    }
    println!("タグ別: {per_tag:?}");

    // 完了率（%）
    let done = notes.iter().filter(|n| n.done).count();
    let rate = done as f64 / notes.len() as f64 * 100.0;
    println!("完了率: {rate:.0}%（{done}/{} 件）", notes.len());

    // 「買」を含むタイトルを、番号付きの文字列にする
    let lines: Vec<String> = notes
        .iter()
        .filter(|n| n.title.contains("買"))
        .enumerate()
        .map(|(i, n)| format!("{}. {}", i + 1, n.title))
        .collect();
    println!("{}", lines.join(" / "));
}
```

**▼ 実行結果**

```text
未完了: ["Rust の勉強", "卵を買う", "Topcoat を調べる"]
タグ別: {"勉強": 2, "家事": 1, "買い物": 2}
完了率: 40%（2/5 件）
1. 牛乳を買う / 2. 卵を買う
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 構造体のフィールドで filter / map する

```rust
let open: Vec<&str> = notes
    .iter()
    .filter(|n| !n.done)
    .map(|n| n.title.as_str())
    .collect();
```

- `notes.iter()` は `&Note`（ノートを借りたもの）を順に流します。`filter` で未完了だけを選び、`map` でタイトルの `&str` に変えています。新しい `String` を作らず、借りたまま集めています。

##### 解説2: enumerate で番号を付けてから文字列にする

```rust
.filter(|n| n.title.contains("買"))
.enumerate()
.map(|(i, n)| format!("{}. {}", i + 1, n.title))
```

- `enumerate` は `(番号, 要素)` の組を流します。`filter` の**後**に置いているので、番号は「絞り込んだ後の何番目か」になります。順番を入れ替えると結果が変わる点に注意しましょう。
- `|(i, n)|` のように、クロージャの引数でもタプルを分解できます（UNIT 08 のパターン）。

### よくあるエラーと読み方

#### エラー1: E0433 use を書き忘れた

```rust
fn main() {
    let mut counts = HashMap::new();
    counts.insert("rust", 1);
    println!("{counts:?}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0433]: cannot find type `HashMap` in this scope
 --> src\main.rs:2:22
  |
2 |     let mut counts = HashMap::new();
  |                      ^^^^^^^ use of undeclared type `HashMap`
  |
help: consider importing this struct
  |
1 + use std::collections::HashMap;
  |

For more information about this error, try `rustc --explain E0433`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** `cannot find type HashMap in this scope`（このスコープで `HashMap` という型が見つかりません）。`help` が `use std::collections::HashMap;` を足す場所（1行目、`+` の行）を示しています。
- **直し方:** ファイルの先頭に `use std::collections::HashMap;` を書きます。エディタ（rust-analyzer）のクイックフィックスでも自動で追加できます。

#### エラー2: E0283 collect の行き先が分からない

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];
    let evens = numbers.iter().filter(|n| **n % 2 == 0).collect();
    println!("{evens:?}");
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0283]: type annotations needed
 --> src\main.rs:3:9
  |
3 |     let evens = numbers.iter().filter(|n| **n % 2 == 0).collect();
  |         ^^^^^                                           ------- type must be known at this point
  |
  = note: the type must implement `FromIterator<&i32>`
note: required by a bound in `collect`
 --> /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library\core\src\iter\traits\iterator.rs:2073:4
help: consider giving `evens` an explicit type
  |
3 |     let evens: Vec<_> = numbers.iter().filter(|n| **n % 2 == 0).collect();
  |              ++++++++

For more information about this error, try `rustc --explain E0283`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** `type annotations needed`（型の注釈が必要です）。`collect` は Vec にも HashMap にも String にも集められるので、どれに集めるのかを決められません。
- **直し方:** `help` のように `let evens: Vec<_> = ...` と書きます。`_` は「要素の型は推論に任せる」という意味です。`.collect::<Vec<_>>()` と書く方法もあります。

#### 警告: イテレータを消費していない

```rust
fn main() {
    let titles = ["牛乳を買う", "Rust の勉強"];
    titles.iter().map(|t| println!("{t}"));
}
```

```text
warning: `Iterator::map` call that discard the iterator's values
 --> src\main.rs:3:19
  |
3 |     titles.iter().map(|t| println!("{t}"));
  |                   ^^^^-------------------^
  |                   |   |
  |                   |   this function returns `()`, which is likely not what you wanted
  |                   |   called `Iterator::map` with callable that returns `()`
  |                   after this call to map, the resulting iterator is `impl Iterator<Item = ()>`, which means the only information carried by the iterator is the number of items
  |
  = note: `Iterator::map`, like many of the methods on `Iterator`, gets executed lazily, meaning that its effects won't be visible until it is iterated
  = note: `#[warn(map_unit_fn)]` (part of `#[warn(unused)]`) on by default
help: you might have meant to use `Iterator::for_each`
  |
3 -     titles.iter().map(|t| println!("{t}"));
3 +     titles.iter().for_each(|t| println!("{t}"));
  |

warning: unused `Map` that must be used
 --> src\main.rs:3:5
  |
3 |     titles.iter().map(|t| println!("{t}"));
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: iterators are lazy and do nothing unless consumed
  = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
  |
3 |     let _ = titles.iter().map(|t| println!("{t}"));
  |     +++++++

warning: `basics` (bin "basics") generated 2 warnings
```

- **読み方:** 2つの警告が出ています。1つ目は「`map` に、`()` を返す処理（表示だけ）を渡していますが、たぶん意図と違います」、2つ目は `unused Map that must be used`（使わなければならない `Map` が使われていません）と `iterators are lazy and do nothing unless consumed`（イテレータは怠け者で、消費されない限り何もしません）。
- **何が起きたか:** 実行しても**何も表示されません**。`map` を作っただけで、流す人（消費メソッド）がいないからです。
- **直し方:** 各要素に処理をするだけなら `for t in titles.iter() { println!("{t}"); }` と書くか、`help` のとおり `for_each` を使います。

#### エラー3: E0603 非公開のものを外から使った

```rust
mod note {
    pub struct NoteTitle(String);

    impl NoteTitle {
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }
}

fn main() {
    // parse を通さずに、空のタイトルを直接作ろうとする
    let title = note::NoteTitle(String::new());
    println!("{}", title.as_str());
}
```

```text
   Compiling basics v0.1.0 (.../basics)
error[E0603]: tuple struct constructor `NoteTitle` is private
  --> src\main.rs:13:23
   |
 2 |     pub struct NoteTitle(String);
   |                          ------ a constructor is private if any of the fields is private
...
13 |     let title = note::NoteTitle(String::new());
   |                       ^^^^^^^^^ private tuple struct constructor
   |
note: the tuple struct constructor `NoteTitle` is defined here
  --> src\main.rs:2:5
   |
 2 |     pub struct NoteTitle(String);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider making the field publicly accessible
   |
 2 |     pub struct NoteTitle(pub String);
   |                          +++

For more information about this error, try `rustc --explain E0603`.
error: could not compile `basics` (bin "basics") due to 1 previous error
```

- **読み方:** `tuple struct constructor NoteTitle is private`（タプル構造体 `NoteTitle` の作り方は非公開です）。`a constructor is private if any of the fields is private`（フィールドが1つでも非公開なら、外からは作れません）。
- **直し方:** ここでは `help` の「フィールドを `pub` にする」に**従ってはいけません**。従うと、検証を通らない空のタイトルが作れてしまいます。正しい直し方は、公開されている入口 `NoteTitle::parse(...)` を使うことです。エラーメッセージの提案が、いつも設計として正しいとは限りません。

> **つまずきポイント:** 「エラーが出たら `pub` を付ければ消える」と覚えると、守りたかった決まりまで壊してしまいます。`pub` は「外に約束する窓口」です。付ける前に「これを外から自由に触らせてよいか」を考えましょう。

### RustNote ではここで使う

この巻のまとめとして、RustNote の**ミニ版 CLI**（コマンドラインで動くアプリ）を作ります。ノートはデータベースではなく `HashMap` に入れておくので、プログラムを終えると消えますが、「追加・一覧・完了の切り替え・削除・検索」という RustNote の基本の流れがそろっています。第7巻からは、この `NoteStore` の役目を、データベースを使う `NoteRepository` に置き換えていきます。

#### ファイルの構成

`cargo new rustnote_mini` で新しいプロジェクトを作り、`src` の中を次の3つのファイルに分けます。

```text
rustnote_mini/
├─ Cargo.toml
└─ src/
   ├─ main.rs    入力を読んで命令を実行する（mod note; mod store; で2つを読み込む）
   ├─ note.rs    NoteId・NoteTitle・TitleError・Note（ノート1件とタイトルの検証）
   └─ store.rs   NoteStore（HashMap にノートを保持する）
```

`Cargo.toml` には、本書の基準に合わせて `rust-version` を書いておきます（外部クレートは使いません）。

```toml
[package]
name = "rustnote_mini"
version = "0.1.0"
edition = "2024"
rust-version = "1.98"

[dependencies]
```

> **▼ このコードがやること（先に日本語で）:** `src/note.rs` は、ノート1件の型と「1〜80文字のタイトルしか作れない」`NoteTitle` をまとめたモジュールです。中身は UNIT 07・09 で作ったものに `pub` を付けて整理したものです。

```rust
// src/note.rs
// ノート1件と、そのタイトルの検証をまとめたモジュール
use std::fmt;

pub const MAX_TITLE_CHARS: usize = 80;

// ノートの ID。中身の i64 は外からも読めるように pub にする
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NoteId(pub i64);

// タイトルの検証で起こりうる失敗
#[derive(Debug, PartialEq)]
pub enum TitleError {
    Empty,
    TooLong { max: usize, actual: usize },
}

impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TitleError::Empty => write!(f, "タイトルを入力してください"),
            TitleError::TooLong { max, actual } => {
                write!(
                    f,
                    "タイトルは{max}文字以内にしてください（現在{actual}文字）"
                )
            }
        }
    }
}

// 検証に通ったタイトル。中身の String には pub を付けないので、
// モジュールの外では parse を通らないと作れない
#[derive(Debug, Clone)]
pub struct NoteTitle(String);

impl NoteTitle {
    pub fn parse(raw: &str) -> Result<Self, TitleError> {
        let title = raw.trim();
        let count = title.chars().count();
        if count == 0 {
            return Err(TitleError::Empty);
        }
        if count > MAX_TITLE_CHARS {
            return Err(TitleError::TooLong {
                max: MAX_TITLE_CHARS,
                actual: count,
            });
        }
        Ok(Self(title.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ノート1件
#[derive(Debug)]
pub struct Note {
    pub id: NoteId,
    pub title: NoteTitle,
    pub done: bool,
}
```

> **▼ このコードがやること（先に日本語で）:** `src/store.rs` は、ノートを ID をキーにした `HashMap` に入れて管理するモジュールです。ID の採番と、追加・切り替え・削除・一覧・検索を担当します。

```rust
// src/store.rs
// ノートをメモリ上の HashMap に保持するモジュール（プログラムを終えると消える）
use std::collections::HashMap;

use crate::note::{Note, NoteId, NoteTitle};

#[derive(Default)]
pub struct NoteStore {
    // pub を付けないので、外からは add や toggle を通してしか変えられない
    notes: HashMap<NoteId, Note>,
    last_id: i64,
}

impl NoteStore {
    pub fn new() -> Self {
        Self::default()
    }

    // ノートを追加して、振った ID を返す
    pub fn add(&mut self, title: NoteTitle) -> NoteId {
        self.last_id += 1;
        let id = NoteId(self.last_id);
        let note = Note {
            id,
            title,
            done: false,
        };
        self.notes.insert(id, note);
        id
    }

    // 完了を切り替え、切り替えた後の状態を返す。ID がなければ None
    pub fn toggle(&mut self, id: NoteId) -> Option<bool> {
        let note = self.notes.get_mut(&id)?;
        note.done = !note.done;
        Some(note.done)
    }

    // 削除したノートを返す。ID がなければ None
    pub fn remove(&mut self, id: NoteId) -> Option<Note> {
        self.notes.remove(&id)
    }

    // ID の小さい順に並べた一覧
    pub fn list(&self) -> Vec<&Note> {
        let mut notes: Vec<&Note> = self.notes.values().collect();
        notes.sort_by_key(|note| note.id);
        notes
    }

    // タイトルに query を含むノートだけの一覧
    pub fn search(&self, query: &str) -> Vec<&Note> {
        self.list()
            .into_iter()
            .filter(|note| note.title.as_str().contains(query))
            .collect()
    }
}
```

> **▼ このコードがやること（先に日本語で）:** `src/main.rs` は、キーボードから1行ずつ命令を読み、enum に変換して、`NoteStore` の操作を呼び出します。`quit` か入力の終わりで終了します。

```rust
// src/main.rs
// src/note.rs と src/store.rs を、このクレートのモジュールとして読み込む
mod note;
mod store;

use std::io::{self, Write};

use note::{Note, NoteId, NoteTitle};
use store::NoteStore;

// 利用者が入力できる命令
enum Command {
    List,
    Add(String),
    Done(i64),
    Delete(i64),
    Search(String),
    Help,
    Quit,
}

// 1行の入力を命令に変換する。解釈できなければ None
fn parse_command(line: &str) -> Option<Command> {
    let line = line.trim();
    let (word, rest) = line.split_once(' ').unwrap_or((line, ""));
    let rest = rest.trim();
    match word {
        "list" => Some(Command::List),
        "add" => Some(Command::Add(rest.to_string())),
        "done" => rest.parse().ok().map(Command::Done),
        "del" => rest.parse().ok().map(Command::Delete),
        "find" if !rest.is_empty() => Some(Command::Search(rest.to_string())),
        "help" => Some(Command::Help),
        "quit" => Some(Command::Quit),
        _ => None,
    }
}

fn print_notes(notes: &[&Note]) {
    if notes.is_empty() {
        println!("（ノートはありません）");
        return;
    }
    for note in notes {
        let mark = if note.done { "[x]" } else { "[ ]" };
        println!("{mark} #{} {}", note.id.0, note.title.as_str());
    }
}

fn print_help() {
    println!("list          一覧を表示");
    println!("add タイトル  ノートを追加（1〜80文字）");
    println!("done 番号     完了／未完了を切り替え");
    println!("del 番号      ノートを削除");
    println!("find 語       タイトルで検索");
    println!("quit          終了");
}

fn prompt() -> io::Result<()> {
    print!("> ");
    // print! は改行しないので、すぐ画面に出すために flush する
    io::stdout().flush()
}

fn main() -> io::Result<()> {
    let mut store = NoteStore::new();
    println!("RustNote ミニ版です。help で使い方を表示します。");
    prompt()?;

    // 1行ずつ読む。入力が終わる（Ctrl+Z / Ctrl+D）か quit で抜ける
    for line in io::stdin().lines() {
        let line = line?;
        let Some(command) = parse_command(&line) else {
            println!("解釈できません（help で使い方を表示）");
            prompt()?;
            continue;
        };
        match command {
            Command::List => print_notes(&store.list()),
            Command::Add(raw) => match NoteTitle::parse(&raw) {
                Ok(title) => {
                    let id = store.add(title);
                    println!("#{} を追加しました", id.0);
                }
                Err(e) => println!("入力エラー: {e}"),
            },
            Command::Done(id) => match store.toggle(NoteId(id)) {
                Some(true) => println!("#{id} を完了にしました"),
                Some(false) => println!("#{id} を未完了に戻しました"),
                None => println!("#{id} は見つかりません"),
            },
            Command::Delete(id) => match store.remove(NoteId(id)) {
                Some(note) => println!("「{}」を削除しました", note.title.as_str()),
                None => println!("#{id} は見つかりません"),
            },
            Command::Search(query) => print_notes(&store.search(&query)),
            Command::Help => print_help(),
            Command::Quit => break,
        }
        prompt()?;
    }
    println!("終了します");
    Ok(())
}
```

```powershell
cd rustnote_mini
cargo run
```

**▼ 実行結果**（`>` の後ろが、キーボードから入力した文字です）

```text
RustNote ミニ版です。help で使い方を表示します。
> help
list          一覧を表示
add タイトル  ノートを追加（1〜80文字）
done 番号     完了／未完了を切り替え
del 番号      ノートを削除
find 語       タイトルで検索
quit          終了
> add 牛乳を買う
#1 を追加しました
> add   Rust の勉強  
#2 を追加しました
> add
入力エラー: タイトルを入力してください
> list
[ ] #1 牛乳を買う
[ ] #2 Rust の勉強
> done 1
#1 を完了にしました
> done 9
#9 は見つかりません
> find Rust
[ ] #2 Rust の勉強
> del 2
「Rust の勉強」を削除しました
> list
[x] #1 牛乳を買う
> hello
解釈できません（help で使い方を表示）
> quit
終了します
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: ファイルをモジュールにする

```rust
mod note;
mod store;
```

- `main.rs` に `mod note;`（`{ }` なし）と書くと、Rust は `src/note.rs` を探して、その中身を `note` モジュールとして読み込みます。例6の `mod note { ... }` の中身を、別のファイルに移したのと同じ意味です（図10-3）。
- `store.rs` の中の `use crate::note::{Note, NoteId, NoteTitle};` の `crate` は「このプログラム全体の根っこ（main.rs）」です。`crate::note` で、隣の部屋 `note` の中のものを指しています。

##### 解説2: HashMap で保持し、ID の採番を閉じ込める

```rust
#[derive(Default)]
pub struct NoteStore {
    notes: HashMap<NoteId, Note>,
    last_id: i64,
}
```

- `HashMap<NoteId, Note>` のキーに自分の型 `NoteId` を使うため、`note.rs` で `NoteId` に `Eq` と `Hash`（等しいかの判定と、棚の番号の計算）を derive しています。`sort_by_key` で並べ替えるための `Ord`（大小の比較）も付けています。
- `notes` と `last_id` には `pub` がないので、`main.rs` からは `add` などのメソッドを通してしか変更できません。ID が重なったり、検証していないノートが紛れ込んだりする心配がありません。
- `#[derive(Default)]` は「すべてのフィールドを既定値（空の HashMap と 0）で作る」機能を付けます。`new()` はそれを呼んでいるだけです。

##### 解説3: `?` を Option にも使う

```rust
pub fn toggle(&mut self, id: NoteId) -> Option<bool> {
    let note = self.notes.get_mut(&id)?;
    note.done = !note.done;
    Some(note.done)
}
```

- `get_mut` は値を**書き換えられる形で借りる** `get` です。見つからなければ `None` です。
- 戻り値が `Option` の関数では、`?` は「`None` ならすぐ `None` を返す」という意味になります（UNIT 09 の Result の `?` と同じ考え方です）。

##### 解説4: 入力を1行ずつ読む

```rust
fn main() -> io::Result<()> {
    ...
    for line in io::stdin().lines() {
        let line = line?;
        ...
    }
}
```

- `io::stdin().lines()` は、キーボード（標準入力）から1行ずつ `String` を流すイテレータです。読み込みに失敗することもあるので、各行は `Result` です。`main` の戻り値を `io::Result<()>` にして、`?` で失敗を返しています。
- `print!` は改行しないため、画面にすぐ出るとは限りません。`prompt` の中で `io::stdout().flush()` を呼び、`> ` を確実に表示してから入力を待ちます。
- 入力の終わりは、Windows なら `Ctrl + Z` を押してから Enter、macOS / Linux なら `Ctrl + D` です。

##### 解説5: 命令の解釈に `map` を使う

```rust
"done" => rest.parse().ok().map(Command::Done),
```

- `parse()` の `Result` を `ok()` で `Option` に変え（失敗の理由は捨てる）、`map(Command::Done)` で `Some(数)` を `Some(Command::Done(数))` に変えています。enum のバリアント `Command::Done` は、「数を受け取って `Command` を作る関数」としても使えます。
- UNIT 08 の `parse_command` と比べて、同じことを短く書いています。どちらの書き方でもかまいません。

#### よくあるエラー: E0583 モジュールのファイルが見つからない

`src/note.rs` を、うっかり `src/notes.rs` という名前で保存すると、次のエラーになります。

```text
   Compiling rustnote_mini v0.1.0 (.../rustnote_mini)
error[E0583]: file not found for module `note`
 --> src\main.rs:2:1
  |
2 | mod note;
  | ^^^^^^^^^
  |
  = help: to create the module `note`, create file "src\note.rs" or "src\note\mod.rs"
  = note: if there is a `mod note` elsewhere in the crate already, import it with `use crate::...` instead

For more information about this error, try `rustc --explain E0583`.
error: could not compile `rustnote_mini` (bin "rustnote_mini") due to 1 previous error
```

- **読み方:** `file not found for module note`（モジュール `note` のファイルが見つかりません）。`help` が `src\note.rs` か `src\note\mod.rs` を作るように教えてくれています。
- **直し方:** ファイル名をモジュール名と同じ `note.rs` にします。

### 練習問題

**問題1.** `"rust web rust db web rust"` に含まれる単語の出現回数を `HashMap` と `entry` で数え、単語の順（`BTreeMap`）に `rust: 3` のような形で表示してください。

**問題2.** 1〜10 のうち奇数だけを2乗して合計した値を、`filter` → `map` → `sum` のイテレータ1本で求めてください。

**問題3.** 次の関数を `text_utils` というモジュールの中に移し、`main` から `text_utils::shout("rust")` で呼べるようにしてください。

```rust
fn shout(text: &str) -> String {
    format!("{}！", text.to_uppercase())
}
```

### 解答と解説

**解答1.**

```rust
use std::collections::{BTreeMap, HashMap};

fn main() {
    let text = "rust web rust db web rust";
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    let sorted: BTreeMap<&str, u32> = counts.into_iter().collect();
    for (word, count) in sorted {
        println!("{word}: {count}");
    }
}
```

**▼ 実行結果**

```text
db: 1
rust: 3
web: 2
```

- `split_whitespace()` で単語に分け、`entry(word).or_insert(0)` で数えます。`BTreeMap` の `for` では `(キー, 値)` の組がキーの順に出てきます。

**解答2.**

```rust
fn main() {
    // 1〜10 のうち奇数だけを2乗して合計する
    let total: u32 = (1..=10).filter(|n| n % 2 == 1).map(|n| n * n).sum();
    println!("{total}");
}
```

**▼ 実行結果**

```text
165
```

- 1 + 9 + 25 + 49 + 81 = 165 です。範囲 `1..=10` はそれ自体がイテレータなので、`iter()` は不要です。`filter` のクロージャの `n` は `&u32` ですが、`%` は参照のままでも計算できます。

**解答3.**

```rust
mod text_utils {
    // pub を付けて、モジュールの外から呼べるようにする
    pub fn shout(text: &str) -> String {
        format!("{}！", text.to_uppercase())
    }
}

fn main() {
    println!("{}", text_utils::shout("rust"));
}
```

**▼ 実行結果**

```text
RUST！
```

- モジュールに移しただけでは非公開なので、`pub fn` にする必要があります。付け忘れると E0603（`function shout is private`）になります。

### まとめ

- `HashMap<K, V>` はキーから値を引く表（順番は決まっていない）。`get` は `Option`、集計は `entry(k).or_insert(初期値)`。順番が要るなら `BTreeMap`。
- クロージャ `|x| 式` は周りの値を取り込める小さな関数。イテレータは `filter` → `map` → `collect` のようにつなぎ、消費メソッドを呼ぶまで動かない。
- `mod` でモジュールに分け、外に見せるものだけ `pub` を付ける。中身を非公開にすれば「検証済みの値しか作れない」型を守れる。`mod note;` で `src/note.rs` を読み込む。

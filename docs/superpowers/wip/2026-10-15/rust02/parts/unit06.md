## UNIT 06: タプル・配列・スライス・Vec

### この単元のゴール

- **タプル**で型の違う値をひとまとめにし、分解して取り出せる
- **配列**（決まった個数）と **Vec**（伸び縮みする）を使い分けられる
- **スライス** `&[T]` で並びの一部を借り、配列と Vec の両方を受け取る関数を書ける
- `len`（入っている数）と `capacity`（確保済みの広さ）の違い、範囲外アクセスの危険と `get` による安全な取り出しを説明できる

### たとえ話でつかむ

- **タプル**は「**お弁当箱**」です。仕切りごとにご飯・おかず・デザートと**違う種類**のものを入れられますが、仕切りの数と並びは最初に決まっています。
- **配列**は「**卵のパック**」です。同じ種類（卵）を、決まった数（6個）だけ入れます。パックの大きさはあとから変えられません。
- **Vec**（ベック：vector ＝ 可変長の配列）は「**伸び縮みする本棚**」です。本を足していき、棚がいっぱいになったら**もっと大きな棚に引っ越し**ます。今入っている本の数が `len`、棚に入る数が `capacity` です。
- **スライス**は、卵のパックや本棚の「**一部を囲む枠**」です。「左から2番目から4番目まで」のように範囲を指して、中身を借りて読みます。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="175" y="26" text-anchor="middle" font-size="15" font-weight="700" fill="#c2410c">タプル（お弁当箱）</text>
<text x="175" y="46" text-anchor="middle" font-size="12" fill="#64748b">型が違ってよい・個数は固定</text>
<rect x="20" y="62" width="310" height="80" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<line x1="80" y1="62" x2="80" y2="142" stroke="#f97316" stroke-width="2"/>
<line x1="250" y1="62" x2="250" y2="142" stroke="#f97316" stroke-width="2"/>
<text x="50" y="100" text-anchor="middle" font-size="16" font-weight="700" fill="#0f172a">1</text>
<text x="50" y="124" text-anchor="middle" font-size="12" fill="#c2410c">u64</text>
<text x="165" y="100" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">"牛乳を買う"</text>
<text x="165" y="124" text-anchor="middle" font-size="12" fill="#c2410c">&amp;str</text>
<text x="290" y="100" text-anchor="middle" font-size="14" font-weight="700" fill="#0f172a">false</text>
<text x="290" y="124" text-anchor="middle" font-size="12" fill="#c2410c">bool</text>
<text x="50" y="162" text-anchor="middle" font-size="12" fill="#475569">.0</text>
<text x="165" y="162" text-anchor="middle" font-size="12" fill="#475569">.1</text>
<text x="290" y="162" text-anchor="middle" font-size="12" fill="#475569">.2</text>
<text x="175" y="196" text-anchor="middle" font-size="12" fill="#334155">型: (u64, &amp;str, bool)</text>
<text x="175" y="218" text-anchor="middle" font-size="12" fill="#334155">取り出し: note.1 や let (id, title, done) = note;</text>
<text x="535" y="26" text-anchor="middle" font-size="15" font-weight="700" fill="#1e40af">配列（卵のパック）とスライス</text>
<text x="535" y="46" text-anchor="middle" font-size="12" fill="#64748b">同じ型が決まった数だけ並ぶ</text>
<rect x="370" y="62" width="330" height="80" rx="12" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<circle cx="403" cy="102" r="26" fill="#ffffff" stroke="#1e40af"/>
<text x="403" y="107" text-anchor="middle" font-size="15" fill="#0f172a">72</text>
<circle cx="469" cy="102" r="26" fill="#ffffff" stroke="#1e40af"/>
<text x="469" y="107" text-anchor="middle" font-size="15" fill="#0f172a">88</text>
<circle cx="535" cy="102" r="26" fill="#ffffff" stroke="#1e40af"/>
<text x="535" y="107" text-anchor="middle" font-size="15" fill="#0f172a">95</text>
<circle cx="601" cy="102" r="26" fill="#ffffff" stroke="#1e40af"/>
<text x="601" y="107" text-anchor="middle" font-size="15" fill="#0f172a">60</text>
<circle cx="667" cy="102" r="26" fill="#ffffff" stroke="#1e40af"/>
<text x="667" y="107" text-anchor="middle" font-size="15" fill="#0f172a">81</text>
<text x="403" y="162" text-anchor="middle" font-size="12" fill="#475569">[0]</text>
<text x="469" y="162" text-anchor="middle" font-size="12" fill="#475569">[1]</text>
<text x="535" y="162" text-anchor="middle" font-size="12" fill="#475569">[2]</text>
<text x="601" y="162" text-anchor="middle" font-size="12" fill="#475569">[3]</text>
<text x="667" y="162" text-anchor="middle" font-size="12" fill="#475569">[4]</text>
<path d="M440,172 L440,184 L564,184 L564,172" fill="none" stroke="#047857" stroke-width="2"/>
<text x="502" y="204" text-anchor="middle" font-size="13" font-weight="700" fill="#047857">&amp;scores[1..3]</text>
<text x="502" y="222" text-anchor="middle" font-size="12" fill="#047857">（[1] と [2] を借りる枠）</text>
<text x="535" y="252" text-anchor="middle" font-size="12" fill="#334155">型: [u32; 5]（要素の型と個数）</text>
<text x="535" y="272" text-anchor="middle" font-size="12" fill="#334155">スライスの型: &amp;[u32]（個数は型に含まない）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図6-1: タプルは「違う型の組」、配列は「同じ型の決まった数の並び」。スライスはその一部を借りる枠</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-08" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="24" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">Vec に push していくと（i32 の場合）</text>
<text x="20" y="62" font-size="13" font-weight="700" fill="#1e40af">push 4回後</text>
<text x="20" y="80" font-size="12" fill="#475569">len 4 / cap 4</text>
<rect x="140" y="50" width="44" height="36" fill="#eff6ff" stroke="#1e40af"/>
<text x="162" y="74" text-anchor="middle" font-size="14" fill="#0f172a">1</text>
<rect x="184" y="50" width="44" height="36" fill="#eff6ff" stroke="#1e40af"/>
<text x="206" y="74" text-anchor="middle" font-size="14" fill="#0f172a">2</text>
<rect x="228" y="50" width="44" height="36" fill="#eff6ff" stroke="#1e40af"/>
<text x="250" y="74" text-anchor="middle" font-size="14" fill="#0f172a">3</text>
<rect x="272" y="50" width="44" height="36" fill="#eff6ff" stroke="#1e40af"/>
<text x="294" y="74" text-anchor="middle" font-size="14" fill="#0f172a">4</text>
<text x="340" y="74" font-size="13" fill="#c2410c">満杯。次の push で入りきらない</text>
<line x1="230" y1="92" x2="230" y2="124" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-08)"/>
<text x="245" y="114" font-size="12" fill="#475569">push(5): 2倍の広さの場所を確保し、中身をコピーして引っ越す</text>
<text x="20" y="152" font-size="13" font-weight="700" fill="#1e40af">push 5回後</text>
<text x="20" y="170" font-size="12" fill="#475569">len 5 / cap 8</text>
<rect x="140" y="140" width="44" height="36" fill="#eff6ff" stroke="#1e40af"/>
<text x="162" y="164" text-anchor="middle" font-size="14" fill="#0f172a">1</text>
<rect x="184" y="140" width="44" height="36" fill="#eff6ff" stroke="#1e40af"/>
<text x="206" y="164" text-anchor="middle" font-size="14" fill="#0f172a">2</text>
<rect x="228" y="140" width="44" height="36" fill="#eff6ff" stroke="#1e40af"/>
<text x="250" y="164" text-anchor="middle" font-size="14" fill="#0f172a">3</text>
<rect x="272" y="140" width="44" height="36" fill="#eff6ff" stroke="#1e40af"/>
<text x="294" y="164" text-anchor="middle" font-size="14" fill="#0f172a">4</text>
<rect x="316" y="140" width="44" height="36" fill="#ecfdf5" stroke="#047857" stroke-width="2"/>
<text x="338" y="164" text-anchor="middle" font-size="14" font-weight="700" fill="#047857">5</text>
<rect x="360" y="140" width="44" height="36" fill="#ffffff" stroke="#94a3b8" stroke-dasharray="4 3"/>
<rect x="404" y="140" width="44" height="36" fill="#ffffff" stroke="#94a3b8" stroke-dasharray="4 3"/>
<rect x="448" y="140" width="44" height="36" fill="#ffffff" stroke="#94a3b8" stroke-dasharray="4 3"/>
<path d="M140,186 L140,196 L360,196 L360,186" fill="none" stroke="#047857" stroke-width="2"/>
<text x="250" y="214" text-anchor="middle" font-size="12" fill="#047857">len = 5（入っている数）</text>
<path d="M140,228 L140,238 L492,238 L492,228" fill="none" stroke="#475569" stroke-width="2"/>
<text x="316" y="256" text-anchor="middle" font-size="12" fill="#475569">capacity = 8（引っ越さずに入る数）</text>
<rect x="520" y="140" width="180" height="84" rx="8" fill="#f8fafc" stroke="#94a3b8"/>
<text x="610" y="164" text-anchor="middle" font-size="12" fill="#334155">capacity の変化</text>
<text x="610" y="186" text-anchor="middle" font-size="13" font-family="Consolas, monospace" fill="#0f172a">0 → 4 → 8 → 16</text>
<text x="610" y="208" text-anchor="middle" font-size="12" fill="#64748b">足りなくなるたびに倍</text>
<text x="360" y="282" text-anchor="middle" font-size="12" fill="#64748b">入る数が分かっていれば Vec::with_capacity(n) で最初から広さを確保でき、引っ越しが減る</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図6-2: Vec の len と capacity。満杯になると、より広い場所へまとめて引っ越す</div>
</div>

| | タプル | 配列 `[T; N]` | `Vec<T>` | スライス `&[T]` |
|---|---|---|---|---|
| 要素の型 | 違ってよい | すべて同じ | すべて同じ | すべて同じ |
| 個数 | 固定 | 固定（型の一部） | 増減できる | 借りた範囲の分 |
| 取り出し | `.0` `.1` | `[i]` / `get(i)` | `[i]` / `get(i)` | `[i]` / `get(i)` |
| よく使う場面 | 関数から複数の値を返す | 曜日など数が決まった表 | 一覧・検索結果 | 関数の引数（読むだけ） |

### まず動かす最小コード

#### 例1: タプルで値をまとめる

> **▼ このコードがやること（先に日本語で）:** ID・タイトル・完了フラグという型の違う3つの値を1つのタプルにまとめ、番号で取り出したり、分解して名前を付けたりします。最後に、関数から2つの値をまとめて返します。

```rust
@@CODE u06/a_tuple.rs@@
```

```powershell
cargo run
```

**▼ 実行結果**

```text
@@OUT u06/a_tuple@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: タプルの型と取り出し方

```rust
let note: (u64, &str, bool) = (1, "牛乳を買う", false);
println!("ID: {}, タイトル: {}, 完了: {}", note.0, note.1, note.2);
```

- `( )` の中に値を `,` で並べるとタプルになります。型も `(u64, &str, bool)` のように、中身の型を順に並べて書きます。
- 中身は `.0` `.1` `.2` と**0から数えた番号**で取り出します。

##### 解説2: 分解（パターン）

```rust
let (id, title, done) = note;
```

- 左側に同じ形 `(id, title, done)` を書くと、タプルの中身がそれぞれの名前に入ります。これを**分解**（デストラクチャリング）と呼びます。`.1` より `title` のほうが、読む人に意味が伝わります。
- 使わない部分は `_` で受けて捨てられます（例: `let (id, _, _) = note;`）。

##### 解説3: 複数の値を返す

```rust
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a <= b { (a, b) } else { (b, a) }
}
```

- 戻り値の型を `(i32, i32)` にすると、2つの値を1回で返せます。呼ぶ側は `let (low, high) = min_max(9, 3);` と分解して受け取ります。
- ただし、3つ以上の値や意味のある組は、名前の付いたフィールドを持つ**構造体**（UNIT 07）にしたほうが読みやすくなります。

#### 例2: 配列

> **▼ このコードがやること（先に日本語で）:** 5人分の点数を配列に入れて、先頭・最後・個数を表示します。同じ値を並べた配列の作り方と、要素の書き換えも試します。

```rust
@@CODE u06/b_array.rs@@
```

**▼ 実行結果**

```text
@@OUT u06/b_array@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 配列の型は「要素の型と個数」

```rust
let scores: [u32; 5] = [72, 88, 95, 60, 81];
```

- `[u32; 5]` は「u32 が5個」という型です。**個数も型の一部**なので、`[u32; 5]` と `[u32; 4]` は別の型です。
- `scores[0]` のように `[ ]` に添え字（0から数える位置）を書いて取り出します。最後の要素は `scores[scores.len() - 1]` です。

##### 解説2: `[値; 個数]` と書き換え

- `[0; 4]` は「0 を4個並べた配列」です。表や計算用の入れ物を用意するときに便利です。
- `mut` を付けると `week[1] = "木";` のように**要素を書き換え**られます。ただし個数は変えられません。個数を増減したいときは Vec を使います。

#### 例3: スライスで並びを借りる

> **▼ このコードがやること（先に日本語で）:** 並びの合計を返す関数を1つ作り、配列全体・配列の一部分・Vec のどれを渡しても動くことを確かめます。

```rust
@@CODE u06/c_slice.rs@@
```

**▼ 実行結果**

```text
@@OUT u06/c_slice@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `&[u32]` は「並びを借りて読む」型

```rust
fn total(values: &[u32]) -> u32 {
```

- `&[u32]` は **u32 のスライス**です。配列 `&scores`、配列の一部 `&scores[1..3]`、Vec `&dynamic` のどれも、この型として渡せます。UNIT 05 の「関数の引数は `&str`」と同じ考え方で、**読むだけの関数は `&[T]` で受け取る**のが基本です。
- `for v in values` の `v` は、要素を借りた `&u32` です。`sum += v;` のように、足し算にはそのまま使えます。

##### 解説2: 範囲の書き方

| 書き方 | 意味 | `scores` の場合 |
|---|---|---|
| `&scores[1..3]` | 1 から 3 の手前まで | `[88, 95]` |
| `&scores[..2]` | 先頭から 2 の手前まで | `[72, 88]` |
| `&scores[3..]` | 3 から最後まで | `[60, 81]` |
| `&scores[..]` | 全体 | 5 個全部 |

- 範囲の数は、文字列のスライス（UNIT 05）と違って**要素の位置**です。

#### 例4: Vec の基本操作

> **▼ このコードがやること（先に日本語で）:** 空の Vec にタイトルを3つ足し、最後の1つを取り出します。位置を指定した安全な取り出し、並べ替え、検索、削除も試します。

```rust
@@CODE u06/d_vec.rs@@
```

**▼ 実行結果**

```text
@@OUT u06/d_vec@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `Vec<String>` と `push`

```rust
let mut titles: Vec<String> = Vec::new();
titles.push(String::from("牛乳を買う"));
```

- `Vec<String>` は「String を並べる Vec」です。`< >` の中に要素の型を書きます。
- 中身を変えるので `mut` が必要です。`push` は末尾に1つ足します。

##### 解説2: `pop` と `get` は Option を返す

- `pop()` は最後の要素を取り出して `Some(値)` を返します。空なら `None` です。
- `get(9)` は、9番目がないので `None` を返しました。`titles[9]` と書くと panic します（下の「よくあるエラー」）。**位置が外から来るときは `get`** を使います。

##### 解説3: その他の便利なメソッド

| メソッド | 何をするか |
|---|---|
| `sort()` | 小さい順に並べ替える（`mut` が必要） |
| `contains(&x)` | x が入っているか（`&` を付けて渡す） |
| `remove(i)` | i 番目を取り除き、後ろを詰める |
| `insert(i, x)` | i 番目に x を差し込む |
| `is_empty()` | 空かどうか |
| `clear()` | 全部消す |

#### 例5: len と capacity

> **▼ このコードがやること（先に日本語で）:** 空の Vec に数を1つずつ足しながら、「入っている数」と「確保済みの広さ」がどう変わるかを表示します。

```rust
@@CODE u06/e_capacity.rs@@
```

**▼ 実行結果**

```text
@@OUT u06/e_capacity@@
```

- `len` は入っている数、`capacity` は**引っ越さずに入る数**です。足りなくなると、Vec はより広い場所を確保して中身をコピーします（図6-2）。この広げ方は標準ライブラリの内部の決め方で、型や版によって変わることがあります。
- 引っ越し（コピー）は手間がかかるので、入る数が分かっているなら `Vec::with_capacity(100)` で最初に広さを確保しておくと効率的です。`len` は 0 のままである点に注意しましょう。

#### 例6: 要素を書き換えながら回す・番号付きで回す

> **▼ このコードがやること（先に日本語で）:** 3つの価格すべてに10%の税を足して書き換えます。続いて、何番目かの番号と一緒に価格を表示します。

```rust
@@CODE u06/f_iter_mut.rs@@
```

**▼ 実行結果**

```text
@@OUT u06/f_iter_mut@@
```

- `for price in &mut prices` は、要素を**書き換えられる形で借りて**回します。`*price` の `*` は「借りている先の値そのもの」を表す記号です（参照外し。第3巻で詳しく）。
- `prices.iter().enumerate()` は、要素に `0, 1, 2, …` の番号を付けて `(番号, 要素)` のタプルで取り出します。

### もう一歩: 実務に近い形

一覧画面のページ送りでは、「全件のうち、このページに表示する部分」を取り出します。スライスを使えば、コピーせずに**一部分を借りるだけ**で済みます。

#### 一覧をページごとに切り出す

> **▼ このコードがやること（先に日本語で）:** 7件のノートを3件ずつのページに分け、1〜4ページ目に表示する部分を取り出します。件数を超えたページは空になります。

```rust
@@CODE u06/g_page.rs@@
```

**▼ 実行結果**

```text
@@OUT u06/g_page@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 範囲が件数を超えないようにする

```rust
let start = (page - 1) * per_page;
if start >= items.len() {
    return &[];
}
let end = (start + per_page).min(items.len());
&items[start..end]
```

- 3ページ目は `start = 6`、`end` は `9` と件数 `7` の小さい方で `7` になります。`.min(...)` で終わりを切り詰めないと、`&items[6..9]` は範囲外で panic します。
- 4ページ目は開始位置が件数以上なので、空のスライス `&[]` を返します。
- `page` は 1 以上であることを前提にしています（0 だと `page - 1` がはみ出します）。UNIT 02 で作った「`?page=` を 1 以上にそろえる処理」と組み合わせて使います。

##### 解説2: 戻り値も借り物

- 戻り値の `&[String]` は、引数 `items` の一部を借りたものです。新しく `Vec` を作らないので、件数が多くてもコピーが発生しません。

### よくあるエラーと読み方

#### エラー1: 範囲外の添え字で panic

```rust
@@CODE u06/err1_oob.rs@@
```

**▼ 実行結果**

```text
@@RUN u06/err1_oob@@
```

- **読み方:** `index out of bounds: the len is 3 but the index is 10` は「範囲外です。長さは3なのに、添え字は10でした」。
- **何が起きたか:** Vec の中身の数は実行中に変わるので、コンパイラは事前に検出できません。Rust は範囲外の場所を読ませないよう、その場で panic して止めます（他の言語のように、でたらめなメモリを読んでしまうことはありません）。
- **直し方:** 位置が正しいと言い切れないなら `titles.get(10)` を使い、`None` の場合の処理を書きます（UNIT 08）。

#### エラー2: E0308 配列の個数が型と合わない

```rust
@@CODE u06/err2_array_len.rs@@
```

```text
@@ERR u06/err2_array_len@@
```

- **読み方:** `expected an array with a size of 3, found one with a size of 2`（3個の配列のはずが、2個だった）。配列では**個数も型の一部**なので、個数違いは型の違いとして扱われます。
- **直し方:** 値を3つ書く（`[255, 0, 0]`）か、本当に2個なら型を `[u8; 2]` にします。

#### エラー3: E0596 mut のない Vec に push した

```rust
@@CODE u06/err3_push.rs@@
```

```text
@@ERR u06/err3_push@@
```

- **読み方:** `cannot borrow titles as mutable, as it is not declared as mutable` は「`titles` は mut と宣言されていないので、書き換えのために借りることができません」。
- **何が起きたか:** `push` は Vec の中身を変える操作です。UNIT 01 の「既定は不変」は Vec の中身にも当てはまります。
- **直し方:** `help` のとおり `let mut titles = Vec::new();` にします。

#### エラー4: E0277 添え字に i32 を使った

```rust
@@CODE u06/err4_index_i32.rs@@
```

```text
@@ERR u06/err4_index_i32@@
```

- **読み方:** `the type [&str] cannot be indexed by i32` と `slice indices are of type usize or ranges of usize`（添え字の型は `usize` か、その範囲）。
- **何が起きたか:** 添え字は「0以上の、PC に合った幅の整数」である `usize` と決まっています（UNIT 02）。マイナスの添え字はありえないからです。
- **直し方:** 最初から `let i: usize = 1;` にします。ほかの型から来た値なら `usize::try_from(i)` で変換し、失敗（マイナスなど）に備えます。

> **つまずきポイント:** `titles[i as usize]` と書けばコンパイルは通りますが、`i` が -1 だと `as` で巨大な数に化けて（UNIT 02）、結局 panic します。添え字に使う値は、最初から `usize` で持つのがいちばん安全です。

### RustNote ではここで使う

本物の RustNote ではノートを構造体（UNIT 07）で表しますが、ここではタプルの Vec で「一覧表示」と「完了件数の集計」の素朴版を作ります。

> **▼ このコードがやること（先に日本語で）:** `(タイトル, 完了したか)` の組を3つ並べた Vec から、チェックボックス風の一覧を表示し、完了した件数を数えます。

```rust
@@CODE u06/h_rustnote.rs@@
```

**▼ 実行結果**

```text
@@OUT u06/h_rustnote@@
```

- `for (title, done) in notes` は、Vec の要素（タプル）を1つずつ取り出しながら分解しています。
- このループは `notes` を**使い切る**（中身を取り出し終えると、`notes` はもう使えない）ので、件数 `notes.len()` は先に数えておきます。使い切らずに借りて回す書き方（`for (title, done) in &notes`）は、第3巻の「借用」で詳しく学びます。
- 「タプルの `.0` が何だったか」を覚えておくのは大変です。次の UNIT 07 では、これを `Note { title, done }` という名前付きの構造体に置き換えます。

### 練習問題

**問題1.** `(i32, char)` のタプルを受け取り、中身を入れ替えた `(char, i32)` を返す関数 `swap` を書いて、`swap((7, 'R'))` の結果を `{:?}` で表示してください。

**問題2.** 空でない `&[i32]` を受け取り、いちばん大きい値を返す関数 `fn largest(values: &[i32]) -> i32` を書いてください。配列と Vec の両方で試しましょう。

**問題3.** 空の Vec に 1〜5 の2乗を `push` して表示し、最後の要素を `pop` で取り出して、取り出した値と残りを表示してください。

### 解答と解説

**解答1.**

```rust
@@CODE u06/ex1.rs@@
```

**▼ 実行結果**

```text
@@OUT u06/ex1@@
```

**解答2.**

```rust
@@CODE u06/ex2.rs@@
```

**▼ 実行結果**

```text
@@OUT u06/ex2@@
```

- 最初の要素を仮の最大値にして、それより大きい値が出てくるたびに入れ替えます。`max = 0` から始めると、全部マイナスの並び（2つ目の例）で 0 という誤った答えになります。
- 空の並びを渡すと `values[0]` で panic します。空にも対応するなら、戻り値を `Option<i32>` にします（UNIT 08）。

**解答3.**

```rust
@@CODE u06/ex3.rs@@
```

**▼ 実行結果**

```text
@@OUT u06/ex3@@
```

### まとめ

- タプルは違う型の組（`.0` や分解で取り出す）、配列は同じ型の決まった数の並び、Vec は増減できる並び（`mut` が必要）。
- 読むだけの関数は `&[T]`（スライス）で受け取れば、配列・配列の一部・Vec のどれでも渡せる。
- 添え字は `usize`。範囲外は panic するので、外から来た位置は `get` で取り出す。`len` は入っている数、`capacity` は確保済みの広さ。

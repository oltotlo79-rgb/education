## UNIT 03: generics・where 句・静的ディスパッチ

### この単元のゴール

- generics（ジェネリクス）で「型だけが違う同じ処理」を1回書くだけで済ませられる
- `<T: PartialOrd + Copy>` のような trait 境界と、`where` 句を読み書きできる
- `Page<T>` のような型パラメータ付きの構造体と、条件付きの `impl` を書ける
- 「単相化（たんそうか）」によって、generics が実行時の手間なしに動く仕組み（静的ディスパッチ）を説明できる

### たとえ話でつかむ

役所の申込書には「氏名：＿＿＿＿」のような**空欄**があります。書類の形は1つですが、空欄に何を書くかは申し込む人ごとに違います。generics の `T` は、この**型の空欄**です。

ただし、空欄には条件が付くことがあります。「※年齢欄には数字を記入すること」のような注意書きです。Rust では `T: PartialOrd`（T には大小を比べられる型を入れること）のように書きます。これが UNIT 02 で見た **trait 境界**です。注意書きがあるおかげで、書類を処理する側（関数の中身）は「この欄は比べられる」と安心して使えます。

そして Rust のコンパイラは、空欄つきの書類を「ひな形」として持っておき、実際に `i32` で使われたら `i32` 専用の書類を、`f64` で使われたら `f64` 専用の書類を**印刷し分けて**くれます。これが**単相化**です。実行するときには、もう空欄はありません。全部が専用の書類になっているので、速く動きます。

> **用語:** generics（ジェネリクス：型をパラメータ（あとで決まる変数のようなもの）として受け取る仕組み。`fn f<T>(x: T)` の `T` が型パラメータ）

> **用語:** 単相化（monomorphization：モノモーフィゼーション。generics の関数を、実際に使われた型ごとに専用の関数としてコンパイルし直すこと）

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-07" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="130" y="24" font-size="14" font-weight="700" fill="#475569" text-anchor="middle">あなたが書くコード</text>
<rect x="20" y="40" width="220" height="70" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="130" y="68" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">fn largest&lt;T&gt;(…)</text>
<text x="130" y="92" font-size="12" fill="#1e40af" text-anchor="middle">ひな形は1つだけ</text>
<rect x="20" y="150" width="220" height="100" rx="10" fill="#f8fafc" stroke="#64748b"/>
<text x="130" y="176" font-size="12" fill="#0f172a" text-anchor="middle">largest(&amp;[3, 9, 4])</text>
<text x="130" y="204" font-size="12" fill="#0f172a" text-anchor="middle">largest(&amp;[1.5, 0.2])</text>
<text x="130" y="232" font-size="12" fill="#0f172a" text-anchor="middle">largest(&amp;['あ', 'ん'])</text>
<text x="383" y="162" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">コンパイラが</text>
<text x="383" y="180" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">型ごとに複製</text>
<line x1="240" y1="75" x2="300" y2="75" stroke="#475569" stroke-width="2"/>
<line x1="300" y1="75" x2="300" y2="200" stroke="#475569" stroke-width="2"/>
<line x1="300" y1="75" x2="466" y2="62" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-07)"/>
<line x1="300" y1="130" x2="466" y2="130" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-07)"/>
<line x1="300" y1="200" x2="466" y2="198" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-07)"/>
<text x="590" y="24" font-size="14" font-weight="700" fill="#475569" text-anchor="middle">できあがる機械語</text>
<rect x="470" y="40" width="240" height="44" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="590" y="67" font-size="13" fill="#047857" text-anchor="middle">largest::&lt;i32&gt;（i32 専用）</text>
<rect x="470" y="108" width="240" height="44" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="590" y="135" font-size="13" fill="#047857" text-anchor="middle">largest::&lt;f64&gt;（f64 専用）</text>
<rect x="470" y="176" width="240" height="44" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="590" y="203" font-size="13" fill="#047857" text-anchor="middle">largest::&lt;char&gt;（char 専用）</text>
<text x="590" y="250" font-size="12" fill="#475569" text-anchor="middle">呼び出しは専用版へ直接つながる＝静的ディスパッチ</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-6: 単相化。1つの generics 関数が、使われた型の数だけ専用の関数になる</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="14" width="680" height="200" rx="14" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="40" y="42" font-size="14" font-weight="700" fill="#1e40af">impl&lt;T&gt; Page&lt;T&gt; … どんな T でも</text>
<text x="40" y="66" font-size="13" fill="#1e40af">使えるメソッド: has_next()</text>
<text x="40" y="100" font-size="12" fill="#475569">例: Page&lt;Note&gt;（Display なし）</text>
<text x="40" y="122" font-size="12" fill="#475569">→ has_next() だけ使える</text>
<rect x="360" y="60" width="320" height="140" rx="12" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="380" y="88" font-size="14" font-weight="700" fill="#047857">impl&lt;T: Display&gt; Page&lt;T&gt;</text>
<text x="380" y="112" font-size="13" fill="#047857">T が Display のときだけ</text>
<text x="380" y="136" font-size="13" fill="#047857">追加で使える: render()</text>
<text x="380" y="170" font-size="12" fill="#475569">例: Page&lt;&amp;str&gt;, Page&lt;i32&gt;</text>
<text x="380" y="190" font-size="12" fill="#475569">→ has_next() も render() も使える</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-7: 条件付きの impl。型パラメータが条件を満たすときだけ、メソッドが増える</div>
</div>

### まず動かす最小コード

#### 例3-1: どんな型でも「いちばん大きいもの」を探す

> **▼ このコードがやること（先に日本語で）:** 整数・小数・文字の並びから、いちばん大きいものを探す関数を**1つだけ**書き、3種類の型で使います。空の並びなら「なし（None）」を返します。最後に、関数がどの型で呼ばれたのかを表示して、型ごとに別々の関数として動いていることを確かめます。

<!--CODE:design_lab/src/bin/u03_generic.rs-->

```powershell
cargo run --bin u03_generic
```

**▼ 実行結果**

<!--OUT:u03_generic-->

#### ▼ コードを1つずつ分解して解説

##### 解説1: `<T: PartialOrd + Copy>` の読み方

```rust
fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
```

- 関数名のすぐ後ろの `<T ...>` で、「この関数は型パラメータ `T` を使います」と宣言します。名前は何でもよいのですが、1つなら `T`（Type の頭文字）が慣習です。
- `: PartialOrd + Copy` が trait 境界です。`+` は「かつ」。「`T` は大小を比べられて（`PartialOrd`）、しかもコピーできる（`Copy`）型に限る」という意味です。
- `items: &[T]` は「`T` のスライス（並び）の借用」、戻り値 `Option<T>` は「`T` が1つ、または無し」です。

> **用語:** `PartialOrd`（パーシャル・オード：`<` `>` で大小を比べられることを表す標準の trait）、`Copy`（代入や引数渡しで、所有権を移さずにビット単位でコピーされる型を表す trait。整数・小数・`char` など）

##### 解説2: 関数の中では「境界で約束されたこと」だけができる

```rust
let mut iter = items.iter().copied();
// 空なら None を返して終わり
let mut max = iter.next()?;
for item in iter {
    if item > max {
        max = item;
    }
}
```

- `item > max` が書けるのは、境界で `PartialOrd` を要求したからです。要求しなければ、`T` が比べられる型かどうかコンパイラには分からないので、比べることはできません（「よくあるエラー」の E0369）。
- `.copied()` で要素をコピーして取り出せるのは `Copy` を要求したからです。
- `iter.next()?` は、最初の要素が無ければ（空の並びなら）その場で `None` を返します。`Option` にも `?` が使えるのです。これでパニックせずに「空」を扱えます。

##### 解説3: 型ごとに別の関数ができている（単相化）

```rust
fn show_type<T>(_value: &T) {
    println!("この呼び出しの T = {}", type_name::<T>());
}
```

- `std::any::type_name::<T>()` は、その場所の `T` が実際に何の型かを文字列で返します。実行結果に `i32`・`f64`・`&str` と出たように、**呼び出しごとに `T` が具体的な型に置き換わっています**。
- コンパイラは、`largest` を `i32` 用・`f64` 用・`char` 用の3つの関数に複製して機械語にします（図4-6）。呼び出し側は、どの関数を呼ぶかがコンパイル時に決まっているので、実行時に「どれを呼ぶか」を調べる手間がありません。これを**静的ディスパッチ**と呼びます。
- 代わりに、使う型が増えるほど機械語の量（できあがる実行ファイルの大きさとコンパイル時間）が増えます。次の UNIT 04 の `dyn Trait` は、この逆の性質を持っています。

> **用語:** ディスパッチ（dispatch：呼び出すメソッドの実体を決めること）。コンパイル時に決めるのが**静的**、実行時に決めるのが**動的**。

> **ヒント:** `type_name` の出力は人間が確かめるためのものです。Rust のバージョンによって表記が変わることがあるので、プログラムの動きをこの文字列で分岐させてはいけません。

### もう一歩: 実務に近い形

RustNote の一覧画面には「ページ送り」があります。ノートの一覧でも、検索結果でも、将来のユーザー一覧でも使えるように、**中身の型を問わない**ページの入れ物を作ってみましょう。

#### 例3-2: `Page<T>` と、where 句つきの paginate

> **▼ このコードがやること（先に日本語で）:** 何かの並びを受け取り、「○ページ目」の分だけを切り出して `Page` という入れ物に入れます。入れ物には、次のページがあるかを答える機能と、中身を箇条書きにして表示する機能を持たせます。表示の機能は、中身が文字として表示できる型のときだけ使えるようにします。

<!--CODE:design_lab/src/bin/u03_page.rs-->

```powershell
cargo run --bin u03_page
```

**▼ 実行結果**

<!--OUT:u03_page-->

##### 解説4: 構造体にも型パラメータを付けられる

```rust
struct Page<T> {
    items: Vec<T>,
    page: u32,
    per_page: u32,
    total: usize,
}
```

- `Page<T>` は「`T` の一覧を1ページ分持つ入れ物」です。`Page<&str>` ならタイトルのページ、`Page<i32>` なら数字のページになります。
- `Vec<T>` や `Option<T>` も、まったく同じ仕組みで作られた標準ライブラリの generics の型です。

##### 解説5: `impl<T>` と `impl<T: Display>` の違い

```rust
impl<T> Page<T> {
    fn has_next(&self) -> bool {
```

```rust
impl<T: Display> Page<T> {
    fn render(&self) -> String {
```

- `impl<T> Page<T>` は「どんな `T` の `Page` にも、このメソッドを付ける」という意味です。`has_next` は件数の計算だけなので、中身の型を問いません。
- `impl<T: Display> Page<T>` は「`T` が `Display`（`{}` で表示できる）を実装しているときだけ、このメソッドを付ける」という意味です（図4-7）。`render` は `format!("- {x}")` で中身を表示するので、この条件が必要です。
- 表示できない型の `Page` で `render()` を呼ぶと、コンパイルエラーになります。**使えないメソッドは最初から存在しない**ので、実行してから困ることがありません。

##### 解説6: where 句で条件を見やすく書く

```rust
fn paginate<T, I>(all: I, page: u32, per_page: u32) -> Page<T>
where
    I: IntoIterator<Item = T>,
{
```

- 型パラメータが2つ（`T` と `I`）あります。`I` は「何かの並び」、`T` は「並びの中身の型」です。
- `where` の後ろに条件を書くと、関数名の行が長くならずに済みます。`<T, I: IntoIterator<Item = T>>` と同じ意味ですが、条件が増えるほど `where` のほうが読みやすくなります。
- `IntoIterator<Item = T>` は「`for` で回せるもので、取り出される要素の型が `T`」という条件です。`Vec` も配列も範囲 `1..=7` も満たすので、実行結果のように数字の範囲からでもページを切り出せました。

#### 例3-3: 型パラメータを持つサービスと、戻り値の `impl Trait`

> **▼ このコードがやること（先に日本語で）:** UNIT 02 の「保存先の約束」を、今度は generics で使います。保存先の型を `R` という型パラメータにしたサービスを作り、タイトルに単語を含むノートを検索します。検索結果は「何かの並び」として返します。

<!--CODE:design_lab/src/bin/u03_service.rs-->

```powershell
cargo run --bin u03_service
```

**▼ 実行結果**

<!--OUT:u03_service-->

##### 解説7: `NoteService<R>` と where 句

```rust
impl<R> NoteService<R>
where
    R: NoteRepository + Debug,
{
```

- `NoteService<InMemory>` を作ると、コンパイラは `InMemory` 専用の `NoteService` を作ります（単相化）。UNIT 02 の `&impl NoteRepository` と同じく静的ディスパッチなので、呼び出しの手間はありません。
- 条件が `NoteRepository + Debug` と2つになったので、`where` 句に書きました。

##### 解説8: 戻り値の `impl Iterator<Item = String>`

```rust
fn search(&self, word: &str) -> impl Iterator<Item = String> {
```

- 戻り値に `impl Trait` と書くと、「具体的な型名は言わないが、この trait を実装した何かを返す」という意味になります。`filter` などをつないだイテレータの本当の型名はとても長く、書くのが大変なので、この書き方が便利です。
- 呼び出し側は「`String` が順に出てくるもの」とだけ知っていれば、`for` で回せます。
- edition 2024 では、戻り値の `impl Trait` が引数の借用（ここでは `&self` と `word`）を使ってよいことが自動で決まります。古い edition 2021 では `+ '_` のような印を付ける必要があったので、古い記事を読むときは注意してください。

> **つまずきポイント:** 引数の `impl Trait`（例: `fn f(x: &impl Summary)`）は「呼ぶ側が型を選ぶ」、戻り値の `impl Trait` は「関数の側が型を選び、名前を隠す」です。同じ書き方でも、誰が型を決めるかが逆になります。

### よくあるエラーと読み方

#### エラー1: E0369 条件を書かずに比べようとした

`largest` の境界から `PartialOrd` を消してみます。

<!--CODE:errs/u03_err_e0369.rs-->

<!--OUT:u03_err_e0369-->

- **何が起きたか:** `binary operation > cannot be applied to type T`（`T` に `>` は使えない）。`T` は `Copy` としか約束していないので、比べられるかどうか分かりません。
- **直し方:** help のとおり、`T` に `PartialOrd` を足します（`std::cmp::PartialOrd` は `PartialOrd` の正式な名前で、`PartialOrd` とだけ書いても同じです）。generics の関数では、「中で使う操作は、すべて境界で要求する」と覚えましょう。

#### エラー2: E0277 境界を満たさない型で呼んだ

今度は関数の側は正しく、呼ぶ側が `String` の並びを渡したケースです。

<!--CODE:errs/u03_err_copy.rs-->

<!--OUT:u03_err_copy-->

- **何が起きたか:** `String` は中身をヒープに持つので `Copy` ではありません（第3巻）。`largest` の `Copy` という境界を満たさないため、`E0277` になりました。
- **直し方:** 関数の側を「コピーしないで、いちばん大きい要素への**参照**を返す」形に変えると、`String` でも使えるようになります（練習問題2）。境界は**必要最小限**にしておくと、使える型が広がります。

#### エラー3: E0282 / E0284 型を推論できない

<!--CODE:errs/u03_err_e0282.rs-->

<!--OUT:u03_err_e0282-->

- **何が起きたか:** `Vec::new()` は `Vec<T>` を作りますが、`T` が何かの手がかりがどこにもありません（何も `push` していない）。そこで `type annotations needed`（型の注釈が必要）になりました。
- **直し方:** `let notes: Vec<String> = Vec::new();` のように型を書きます。後で `notes.push(String::from("..."))` のような行があれば、コンパイラはそこから推論してくれます。

文字列を数値に変える `parse` でも、似たエラーがよく出ます。

<!--CODE:errs/u03_err_e0284.rs-->

<!--OUT:u03_err_e0284-->

- `parse` は generics の関数（`fn parse<F: FromStr>(&self)`）なので、何の型に変換するのかを教える必要があります。`let page: u32 = "3".parse().unwrap();` のように変数に型を書くか、`"3".parse::<u32>()` のように関数名の後ろに `::<型>` を付けます。この `::<>` は、見た目から**ターボフィッシュ**（turbofish）と呼ばれています。

### RustNote ではここで使う

- 一覧と検索の結果は、ここで作った `Page<T>` のような「1ページ分の入れ物」で画面に渡します（実際のページ送りは SQL の `LIMIT` / `OFFSET` で DB 側で行います。第7巻・第10巻）。
- ユースケースが保存先を受け取る形は、generics（`R: NoteRepository`）と、次の UNIT 04 の `dyn NoteRepository` の2通りがあります。RustNote でどちらを使うかは、UNIT 04 の最後で比べます。

### 練習問題

**問題1:** `==` で比べられる型なら何でも使える関数 `fn count_matching<T: PartialEq>(items: &[T], target: &T) -> usize` を書き、整数の並びと `&str` の並びで「目当ての値が何個あるか」を数えてください。

**問題2:** 「よくあるエラー」のエラー2を直すため、`Copy` を求めずに「いちばん大きい要素への参照」を返す `largest_ref` を、**where 句を使って**書いてください。`String` の並びと整数の並びの両方で動かしてください。

**問題3:** 例3-1 の `largest` を `i32`・`f64`・`char` の3種類の型で呼んでいます。さらに別の場所で `i32` の並びでもう一度呼んだとき、できあがる機械語の `largest` は何種類になるでしょうか。理由も答えてください。

### 解答と解説

**解答1:**

<!--CODE:design_lab/src/bin/u03_ex1.rs-->

**▼ 実行結果**

<!--OUT:u03_ex1-->

- `filter(|x| *x == target)` の `x` は `&&T`、`target` は `&T` です。`*x` で1段はがして `&T` 同士を比べています。`PartialEq` だけを要求しているので、`Copy` でない `String` の並びでも使えます。

**解答2:**

<!--CODE:design_lab/src/bin/u03_ex2.rs-->

**▼ 実行結果**

<!--OUT:u03_ex2-->

- 戻り値を `Option<&T>` にしたので、要素をコピーせずに済み、`Copy` の境界が不要になりました。
- `"電気代"` が最大になったのは、文字列の比較が「文字コード（Unicode の番号）の順」だからです。「電」の番号が「牛」や「本」より大きいためで、五十音順ではない点に注意してください。

**解答3:**

- **3種類**です。単相化は「使われた**型**ごとに」1つ作ります。同じ `i32` で何回呼んでも、`largest::<i32>` は1つだけで、それが使い回されます。

### まとめ

- generics の `T` は「型の空欄」で、trait 境界（`T: PartialOrd`）は空欄に入れてよい型の条件です。中で使う操作は境界で要求します。
- 条件が多いときは `where` 句、条件付きでメソッドを足すときは `impl<T: 条件> 型<T>` を使います。
- generics は単相化で型ごとの専用関数になり（静的ディスパッチ）、速い代わりに機械語が増えます。

## UNIT 04: dyn Trait とオブジェクト安全性

### この単元のゴール

- `Box<dyn Trait>` や `&dyn Trait` で、**型の違う値を1つの入れ物にまとめて**扱える
- 動的ディスパッチの仕組み（ファットポインタと vtable）を図で説明できる
- `dyn` で使える trait の条件（dyn compatible：旧称オブジェクト安全性）と、E0038 の直し方が分かる
- 実行時に実装を選んで `Arc<dyn NoteRepository + Send + Sync>` として共有し、generics との使い分けを判断できる

### たとえ話でつかむ

テレビ、エアコン、照明…と家電ごとにリモコンを持つ代わりに、**どの家電でも操作できる万能リモコン**を考えてみましょう。万能リモコンは、ボタンを押すと「いま登録されている家電の**操作表**」を見て、その家電に合った信号を送ります。

- 操作表には「電源ボタン → この信号」「音量ボタン → この信号」と書いてある
- 家電を差し替えるときは、操作表を差し替えるだけ
- ボタンを押すたびに表を1回引く手間はあるが、どんな家電でも1つのリモコンで操作できる

Rust の `dyn Trait` は、この万能リモコンです。操作表のことを **vtable**（ブイテーブル：virtual method table）と呼びます。UNIT 03 の generics が「家電ごとに専用リモコンを工場で作る（単相化）」やり方だったのに対し、`dyn Trait` は「1つのリモコンが、実行時に表を引いて動きを変える」やり方です。

> **用語:** trait オブジェクト（`dyn Trait` と書く、「その trait を実装した何かの型」の値。具体的な型は実行時まで分からない）

<!-- -->

> **用語:** 動的ディスパッチ（呼び出すメソッドの実体を、実行時に vtable を引いて決めること）

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-08" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="10" width="340" height="230" rx="10" fill="#eff6ff" stroke="#3b82f6"/>
<text x="180" y="36" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">静的ディスパッチ（generics）</text>
<rect x="40" y="56" width="120" height="44" rx="8" fill="#ffffff" stroke="#3b82f6"/>
<text x="100" y="83" font-size="12" fill="#0f172a" text-anchor="middle">largest(&amp;nums)</text>
<line x1="160" y1="78" x2="196" y2="78" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-08)"/>
<rect x="200" y="56" width="130" height="44" rx="8" fill="#ffffff" stroke="#3b82f6"/>
<text x="265" y="83" font-size="12" fill="#0f172a" text-anchor="middle">largest::&lt;i32&gt;</text>
<text x="180" y="136" font-size="12" fill="#1e40af" text-anchor="middle">行き先はコンパイル時に決まっている</text>
<text x="180" y="162" font-size="12" fill="#1e40af" text-anchor="middle">○ 呼び出しが速い（インライン化もできる）</text>
<text x="180" y="186" font-size="12" fill="#1e40af" text-anchor="middle">△ 型ごとに機械語が増える</text>
<text x="180" y="210" font-size="12" fill="#1e40af" text-anchor="middle">× 違う型を1つの Vec に入れられない</text>
<rect x="370" y="10" width="340" height="230" rx="10" fill="#fff7ed" stroke="#f97316"/>
<text x="540" y="36" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">動的ディスパッチ（dyn Trait）</text>
<rect x="385" y="56" width="92" height="44" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="431" y="83" font-size="12" fill="#0f172a" text-anchor="middle">e.export(…)</text>
<line x1="477" y1="78" x2="499" y2="78" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-08)"/>
<rect x="503" y="56" width="80" height="44" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="543" y="83" font-size="12" fill="#0f172a" text-anchor="middle">vtable</text>
<line x1="583" y1="78" x2="605" y2="78" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-08)"/>
<rect x="609" y="56" width="92" height="44" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="655" y="76" font-size="12" fill="#0f172a" text-anchor="middle">Markdown</text>
<text x="655" y="92" font-size="12" fill="#0f172a" text-anchor="middle">::export</text>
<text x="540" y="136" font-size="12" fill="#c2410c" text-anchor="middle">実行時に表を引いて行き先を決める</text>
<text x="540" y="162" font-size="12" fill="#c2410c" text-anchor="middle">○ 違う型を1つの Vec に入れられる</text>
<text x="540" y="186" font-size="12" fill="#c2410c" text-anchor="middle">○ 機械語は1つで済む</text>
<text x="540" y="210" font-size="12" fill="#c2410c" text-anchor="middle">△ 表を引く手間・Box ならヒープ確保</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-8: 静的ディスパッチと動的ディスパッチの比較</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-09" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="200" y="28" font-size="14" font-weight="700" fill="#475569" text-anchor="middle">&amp;dyn Exporter / Box&lt;dyn Exporter&gt;（16 バイト）</text>
<rect x="40" y="40" width="160" height="50" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="120" y="62" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">データの場所</text>
<text x="120" y="80" font-size="12" fill="#1e40af" text-anchor="middle">8 バイト</text>
<rect x="200" y="40" width="160" height="50" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="280" y="62" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">vtable の場所</text>
<text x="280" y="80" font-size="12" fill="#c2410c" text-anchor="middle">8 バイト</text>
<line x1="120" y1="90" x2="120" y2="164" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-09)"/>
<rect x="30" y="168" width="250" height="56" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="155" y="192" font-size="13" fill="#047857" text-anchor="middle">Markdown { heading_level: 2 }</text>
<text x="155" y="212" font-size="12" fill="#047857" text-anchor="middle">本体（Box ならヒープ上）</text>
<line x1="360" y1="65" x2="436" y2="65" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-09)"/>
<rect x="440" y="40" width="260" height="190" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="570" y="64" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">Markdown 用の vtable（操作表）</text>
<rect x="456" y="78" width="228" height="30" rx="4" fill="#ffffff" stroke="#fdba74"/>
<text x="570" y="98" font-size="12" fill="#0f172a" text-anchor="middle">片付け方（drop）</text>
<rect x="456" y="114" width="228" height="30" rx="4" fill="#ffffff" stroke="#fdba74"/>
<text x="570" y="134" font-size="12" fill="#0f172a" text-anchor="middle">大きさ・並び（size / align）</text>
<rect x="456" y="150" width="228" height="30" rx="4" fill="#ffffff" stroke="#fdba74"/>
<text x="570" y="170" font-size="12" fill="#0f172a" text-anchor="middle">name → Markdown::name</text>
<rect x="456" y="186" width="228" height="30" rx="4" fill="#ffffff" stroke="#fdba74"/>
<text x="570" y="206" font-size="12" fill="#0f172a" text-anchor="middle">export → Markdown::export</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-9: dyn Trait の正体は「データの場所」と「vtable の場所」の2つ組（ファットポインタ）</div>
</div>

図4-9 のように、`&dyn Exporter` は普通の参照（8 バイト）の2倍、**16 バイト**あります。前半が「データ本体の場所」、後半が「その型用の vtable の場所」です。`e.export(...)` と書くと、プログラムは vtable の `export` の欄を見て、`Markdown::export` を呼びます。型ごとに vtable が1つずつ用意されるので、同じ `Vec` の中に `PlainText` と `Markdown` が混ざっていても、それぞれ正しいメソッドが呼ばれます。

> **用語:** ファットポインタ（fat pointer：場所だけでなく、追加の情報（ここでは vtable の場所）も一緒に持つ太いポインタ。スライス `&[T]` も「場所＋長さ」のファットポインタ）

### まず動かす最小コード

#### 例4-1: 型の違う書き出し形式を1つの Vec にまとめる

> **▼ このコードがやること（先に日本語で）:** ノートを「テキスト」「Markdown」「CSV」の3つの形式で書き出す部品を作り、型が違うのに1つのリストへ入れて、順番に書き出します。最後に、普通の参照と `dyn` の参照の大きさを比べます。

<!--CODE:design_lab/src/bin/u04_dyn.rs-->

```powershell
cargo run --bin u04_dyn
```

**▼ 実行結果**

<!--OUT:u04_dyn-->

#### ▼ コードを1つずつ分解して解説

##### 解説1: `Vec<Box<dyn Exporter>>` の意味

```rust
let exporters: Vec<Box<dyn Exporter>> = vec![
    Box::new(PlainText),
    Box::new(Markdown { heading_level: 2 }),
    Box::new(Csv),
];
```

- `Vec<T>` は「同じ型の値」しか入れられません。`PlainText` と `Markdown` は別の型なので、そのままでは1つの `Vec` に入りません。そもそも大きさも違います（`PlainText` は 0 バイト、`Markdown` は `usize` 1つ分）。
- そこで、それぞれを `Box::new(...)` でヒープに置き、手元には「`Box<dyn Exporter>`」という**同じ大きさ（16 バイト）のファットポインタ**だけを持ちます。これなら同じ型として `Vec` に並べられます。
- `dyn Exporter` の `dyn`（ダイン：dynamic の略）は、「具体的な型は実行時まで分からないが、`Exporter` を実装している何か」という意味の印です。

##### 解説2: 呼び出す側は型を気にしない

```rust
for e in &exporters {
    println!("--- {} ---", e.name());
    println!("{}", e.export("牛乳を買う", "低脂肪のもの"));
}
```

- `e` の型は `&Box<dyn Exporter>` ですが、メソッドはそのまま呼べます。どの `export` が呼ばれるかは、`e` が持っている vtable で実行時に決まります。
- 新しい形式（たとえば HTML）を増やしても、このループは1文字も変わりません（練習問題1）。

##### 解説3: 大きさが 8 と 16 になる理由

```rust
println!("&Markdown の大きさ: {} バイト", size_of::<&Markdown>());
println!(
    "&dyn Exporter の大きさ: {} バイト",
    size_of::<&dyn Exporter>()
);
```

- `std::mem::size_of::<T>()` は、型 `T` の値が何バイトかを返します。
- 具体的な型への参照 `&Markdown` は、場所だけを持てばよいので 8 バイト（64 ビット環境）。`&dyn Exporter` は「場所＋vtable の場所」で 16 バイトです。図4-9 のとおりの結果になりました。

### もう一歩: 実務に近い形

`dyn Trait` が本当に役立つのは、「**どの実装を使うかが実行時（設定）で決まる**」ときです。RustNote でも、起動時の設定で保存先を選び、それをアプリ全体で共有します。

#### 例4-2: 設定で保存先を選び、AppState で共有する

> **▼ このコードがやること（先に日本語で）:** 「memory」「read-only」などの文字列（本物のアプリでは設定ファイルや環境変数の値）を見て、使う保存先を実行時に選びます。選んだ保存先をアプリ全体の状態 `AppState` に入れ、2つのスレッド（同時に来たリクエストの代わり）から同時に使います。知らない名前が来たら起動を失敗させます。

<!--CODE:design_lab/src/bin/u04_state.rs-->

```powershell
cargo run --bin u04_state
```

**▼ 実行結果**

<!--OUT:u04_state-->

> **注意:** `[memory] Ok(2)` と `[memory] Ok(1)` の順番は、実行するたびに入れ替わることがあります。2つのスレッドのどちらが先に `insert` するかは、OS が決めるからです。件数がいつも 2 になることのほうが大事です。

##### 解説4: `Arc<dyn NoteRepository + Send + Sync>`

```rust
type DynRepo = Arc<dyn NoteRepository + Send + Sync>;
```

- `type 名前 = 型;` は**型に別名を付ける**書き方です。長い型を何度も書かずに済みます。
- `Arc<...>`（第3巻）は、複数のスレッドで1つの値を共有するための「参照カウント付きの箱」です。`Box` と同じく、中に `dyn Trait` を入れられます。
- `+ Send + Sync` は「この trait オブジェクトの中身は、スレッドをまたいで送ってよく（`Send`）、同時に共有してよい（`Sync`）」という追加の条件です。Web サーバーはリクエストを複数のスレッドで同時に処理するので、共有する状態には必ずこの条件が要ります。付け忘れたときのエラーは後で見ます。

##### 解説5: 実行時に実装を選ぶ関数

```rust
fn build_repo(kind: &str) -> Result<DynRepo, String> {
    match kind {
        "memory" => Ok(Arc::new(InMemoryNoteRepository::default())),
        "read-only" => Ok(Arc::new(ReadOnlyRepository)),
        other => Err(format!("知らない保存先です: {other}")),
    }
}
```

- `match` の腕ごとに、**違う型**（`InMemoryNoteRepository` と `ReadOnlyRepository`）を作っています。戻り値の型が `Arc<dyn ...>` なので、どちらも同じ型として返せます。generics（`-> impl NoteRepository`）では、「1つの関数は1つの具体的な型しか返せない」ので、この書き方はできません。
- 知らない名前のときは `Err` を返し、`main` が「起動失敗」と表示しています。**設定の間違いは起動時に見つける**のが鉄則です（UNIT 09 で詳しく扱います）。

##### 解説6: `#[derive(Clone)]` の AppState は安く複製できる

```rust
#[derive(Clone)]
struct AppState {
    repo: DynRepo,
}
```

- `state.clone()` は、中の `Arc` を複製します。`Arc` の複製は「参照カウントを1増やす」だけで、保存先の中身はコピーされません。だから各スレッドが**同じ保存先**を使い、件数が合計 2 になりました。
- Web フレームワークの多くは、このような「安く複製できる共有状態」をリクエストごとに渡す設計になっています（第6巻）。

#### generics と dyn Trait の使い分け

| 比べること | generics（`T: Trait` / `impl Trait`） | `dyn Trait` |
|---|---|---|
| どの実装を使うかが決まる時 | コンパイル時 | 実行時 |
| 違う型を1つの `Vec` に混ぜる | できない | できる |
| 呼び出しの速さ | 最速（直接呼び出し・インライン化） | vtable を1回引く分だけ遅い |
| 機械語の量・コンパイル時間 | 型の数だけ増える | 1つで済む |
| trait への制約 | ほぼ無し | dyn compatible である必要がある |
| 向いている場面 | 小さな部品・何度も呼ぶ処理・ライブラリ | 設定で差し替える部品・プラグイン・型の違うものの一覧 |

迷ったら、まず generics で書き、「実行時に選びたい」「型を混ぜたい」「型パラメータが増えすぎて読みにくい」ときに `dyn` を選ぶ、で十分です。DB アクセスのように、それ自体に時間がかかる処理では、vtable を引く手間は誤差のようなものです。

### よくあるエラーと読み方

#### エラー1: E0038 dyn で使えない trait（generics のメソッド）

<!--CODE:errs/u04_err_e0038.rs-->

<!--OUT:u04_err_e0038-->

- **何が起きたか:** `the trait Exporter is not dyn compatible`（`Exporter` は dyn で使える条件を満たしていない）。理由は `method export_all has generic type parameters`（`export_all` が型パラメータを持っている）です。
- **なぜダメなのか:** vtable は「メソッド1つにつき関数1つ」の表です。しかし generics のメソッドは、単相化で `T` ごとに無数の関数になりえます。表に何を書けばよいか決められないので、vtable を作れません（`it needs to allow building a vtable`）。
- **直し方:** ①`export_all` を `&[&dyn Display]` のような dyn を使う引数に変える、②`export_all` を別の trait に移す（help の提案）、③そのメソッドに `where Self: Sized` を付けて「dyn からは呼ばない」と宣言する（エラー2の直し方と同じ）、のどれかです。

> **用語:** dyn compatible（ダイン・コンパチブル：`dyn` で使える条件を満たしていること。以前は「オブジェクト安全（object safe）」と呼ばれていて、古い資料や記事ではこちらの名前で出てきます）

#### エラー2: E0038 戻り値に `Self` を使っている

<!--CODE:errs/u04_err_self.rs-->

<!--OUT:u04_err_self-->

- **何が起きたか:** `duplicate` が戻り値に `Self`（実装する型そのもの）を使っています。`dyn Exporter` 越しに呼んだとき、`Self` が `Csv` なのか `Markdown` なのか、返ってくる値の大きさが分かりません。
- **直し方:** そのメソッドに `where Self: Sized` を付けます。「このメソッドは、大きさが分かっている具体的な型のときだけ呼べる」という意味で、dyn 越しには呼べなくなる代わりに、trait 全体は dyn で使えるようになります（練習問題2）。
- 同じ理由で、標準の `Clone` trait（`fn clone(&self) -> Self`）も dyn compatible ではありません。`Box<dyn Clone>` は作れないので覚えておきましょう。

#### エラー3: E0782 `dyn` を書き忘れた

<!--CODE:errs/u04_err_e0782.rs-->

<!--OUT:u04_err_e0782-->

- **何が起きたか:** `expected a type, found a trait`（型が来るべき場所に trait が来た）。trait の名前だけでは型になりません。
- **直し方:** help のとおり `Box<dyn Exporter>` と書きます。古い Rust（edition 2015）では `dyn` を省略できたため、古い記事のコードを写すとこのエラーに出会います。

#### エラー4: E0277 スレッドで共有するのに Send / Sync が無い

<!--CODE:errs/u04_err_send.rs-->

<!--OUT:u04_err_send-->

- **何が起きたか:** `dyn NoteRepository cannot be shared between threads safely`（スレッド間で安全に共有できない）。`dyn NoteRepository` とだけ書くと、中身が `Send` / `Sync` かどうかをコンパイラは知りません。中身が `Rc` や `RefCell`（第3巻。スレッドをまたげない型）かもしれないからです。同じ原因で `Send` と `Sync` の2つのエラーが出ています。
- `note: required by a bound in spawn` の下に `/rustc/…/library\std\src\thread\functions.rs` という、自分のものではないファイルが出ていますが、これは「標準ライブラリの `thread::spawn` の定義がこの条件を要求している」という参考情報です。読み飛ばしてかまいません。
- **直し方:** `Arc<dyn NoteRepository + Send + Sync>` と書きます。こう書くと、`Send + Sync` でない実装を入れようとしたときに、**入れる場所で**コンパイルエラーになってくれます。

### RustNote ではここで使う

- RustNote の共有状態 `AppState` は、ノートの保存先を `Arc<dyn NoteRepository + Send + Sync>` の形で持ちます（ユーザーやセッションの保存先も同じ形です）。保存先を trait オブジェクトにしておくと、テストで別の実装に**差し替え**られます。
- 一方、ユースケースの関数の引数のように「呼ぶ場所で型が決まっている」ところは、UNIT 02・03 の `impl NoteRepository` や generics でも書けます。どちらでも正しく動くので、RustNote では「状態として持つものは `dyn`、関数の引数は読みやすいほう」と考えれば十分です。
- 本物の RustNote の保存先は `async` のメソッドを持つため、`dyn` で使うには追加の工夫が要ります。その話は第5巻・第7巻で扱います。

### 練習問題

**問題1:** 例4-1 に、`<h1>タイトル</h1>` と `<p>本文</p>` を出力する `Html` 形式を追加してください（`PlainText` と `Html` の2つだけの `Vec` で構いません）。使う側の `for` ループは変えずに済むことを確かめてください。

**問題2:** エラー2のコード（`fn duplicate(&self) -> Self`）を、`where Self: Sized` を使ってコンパイルが通るように直してください。`Box<dyn Exporter>` からは `name` を、具体的な型 `Csv` からは `duplicate` を呼んでください。

**問題3:** 次の2つの場面では、generics と `dyn Trait` のどちらが向いているでしょうか。理由も答えてください。

- (a) 利用者が画面で選んだ書き出し形式（テキスト・Markdown・CSV）でノートを書き出す
- (b) ページ送りの計算をする小さな関数を、`Page<Note>` と `Page<User>` で何万回も呼ぶ

### 解答と解説

**解答1:**

<!--CODE:design_lab/src/bin/u04_ex1.rs-->

**▼ 実行結果**

<!--OUT:u04_ex1-->

- 新しい型 `Html` に `impl Exporter for Html` を書いて `Vec` に入れただけで、ループは元のままです。「新しい種類を足すときに、使う側を直さなくてよい」のが trait オブジェクトの強みです。
- 本物のアプリで HTML を書き出すときは、タイトルや本文に含まれる `<` や `&` を変換（エスケープ）しないと、表示が崩れたり、悪意のあるスクリプトが動いたりします（XSS。第8巻）。ここでは仕組みの練習として省略しています。

**解答2:**

<!--CODE:design_lab/src/bin/u04_ex2.rs-->

**▼ 実行結果**

<!--OUT:u04_ex2-->

- `where Self: Sized` は「`Self` の大きさがコンパイル時に分かるときだけ」という条件です。`dyn Exporter` は大きさが分からない型なので、このメソッドは vtable に載らず、dyn 越しには呼べません。その代わり、trait 全体は dyn で使えるようになります。

**解答3:**

- (a) は **`dyn Trait`** が向いています。どの形式を使うかが実行時（利用者の選択）で決まり、`Vec<Box<dyn Exporter>>` や `match` で選んだ結果を同じ型として扱いたいからです。
- (b) は **generics** が向いています。使う型はコンパイル時に分かっていて、何万回も呼ぶ小さな処理なので、単相化による直接呼び出し（静的ディスパッチ）の速さが活きます。

### まとめ

- `Box<dyn Trait>` / `Arc<dyn Trait + Send + Sync>` で、違う型を同じ型として扱い、実行時に実装を選べます。
- `dyn Trait` は「データの場所＋vtable の場所」のファットポインタで、vtable を引いてメソッドを呼びます（動的ディスパッチ）。
- generics のメソッドや `Self` を返すメソッドがあると dyn で使えません（E0038）。`where Self: Sized` か、trait を分けて直します。

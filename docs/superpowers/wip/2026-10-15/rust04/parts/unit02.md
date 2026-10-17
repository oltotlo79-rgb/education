## UNIT 02: trait と境界による設計

### この単元のゴール

- trait（トレイト）が「型が守る約束（できること一覧）」であることを説明できる
- `trait` を定義し、複数の型に `impl トレイト for 型` で実装できる。既定の実装も使える
- `&impl Trait` という引数で「約束を守る型なら何でも受け取る関数」を書ける
- ユースケースを具体的な保存先ではなく `NoteRepository` trait に依存させ、本物とテスト用 Fake を差し替えられる

### たとえ話でつかむ

家の壁のコンセントは、差し込み口の形と電圧が「規格」として決まっています。だから、ドライヤーでも掃除機でもスマホの充電器でも、**規格どおりのプラグを持っていれば**同じコンセントに差せます。コンセントの側は「何の家電か」を知りません。知っているのは「規格を守っている」ことだけです。

Rust の **trait** は、この「規格」にあたります。

- trait を定義する ＝ 「この規格を名乗るなら、こういうメソッドを持っていること」と決める
- `impl 規格 for 型` ＝ 「この型は規格どおりのプラグを持っています」と宣言し、中身を書く
- `&impl 規格` を受け取る関数 ＝ コンセント。規格を守る型なら何でも差せる

アプリの設計では、「ノートを保存する」部分にこの考え方を使います。ノートを作る処理（ユースケース）は「保存できるという規格（`NoteRepository`）」にだけ差し込み、実際の保存先がメモリなのか Turso の DB なのか、テスト用の偽物なのかは知らずに済むようにします。

> **用語:** trait（トレイト：型が持つべきメソッドの一覧を決めた「約束」。他の言語の interface に近い）

<!-- -->

> **用語:** ユースケース（アプリが利用者のためにする1つの仕事。「ノートを作る」「ノートを一覧する」など）

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-05" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="200" y="14" width="320" height="86" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="40" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">trait NoteRepository（約束）</text>
<text x="360" y="64" font-size="12" fill="#1e40af" text-anchor="middle">fn insert(&amp;self, owner, title) -&gt; Result</text>
<text x="360" y="84" font-size="12" fill="#1e40af" text-anchor="middle">fn list_by_owner(&amp;self, owner) -&gt; Result</text>
<rect x="20" y="170" width="210" height="80" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="125" y="196" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">InMemoryNoteRepository</text>
<text x="125" y="218" font-size="12" fill="#047857" text-anchor="middle">Vec にしまう</text>
<text x="125" y="238" font-size="12" fill="#047857" text-anchor="middle">（開発・テスト用）</text>
<rect x="255" y="170" width="210" height="80" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="196" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">BrokenRepository</text>
<text x="360" y="218" font-size="12" fill="#c2410c" text-anchor="middle">必ず失敗する Fake</text>
<text x="360" y="238" font-size="12" fill="#c2410c" text-anchor="middle">（失敗時のテスト用）</text>
<rect x="490" y="170" width="210" height="80" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2" stroke-dasharray="6 4"/>
<text x="595" y="196" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">Turso 版の実装</text>
<text x="595" y="218" font-size="12" fill="#475569" text-anchor="middle">SQL で DB にしまう</text>
<text x="595" y="238" font-size="12" fill="#475569" text-anchor="middle">（第7巻で作る）</text>
<line x1="125" y1="170" x2="270" y2="104" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-05)"/>
<line x1="360" y1="170" x2="360" y2="104" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-05)"/>
<line x1="595" y1="170" x2="450" y2="104" stroke="#475569" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-04-05)"/>
<text x="150" y="134" font-size="12" fill="#475569" text-anchor="middle">impl ... for</text>
<text x="400" y="140" font-size="12" fill="#475569" text-anchor="middle">impl ... for</text>
<text x="580" y="134" font-size="12" fill="#475569" text-anchor="middle">impl ... for</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-4: 1つの trait（約束）を、複数の型がそれぞれのやり方で実装する</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-06" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="175" y="24" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">Before: 具体的な保存先に直結</text>
<rect x="60" y="40" width="230" height="50" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="175" y="70" font-size="13" fill="#0f172a" text-anchor="middle">create_note（ユースケース）</text>
<line x1="175" y1="90" x2="175" y2="132" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-06)"/>
<rect x="60" y="136" width="230" height="50" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="175" y="166" font-size="13" fill="#0f172a" text-anchor="middle">Turso に SQL を送るコード</text>
<text x="175" y="214" font-size="12" fill="#c2410c" text-anchor="middle">テストのたびに本物の DB が必要</text>
<text x="545" y="24" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">After: trait をはさむ</text>
<rect x="430" y="40" width="230" height="44" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="545" y="67" font-size="13" fill="#0f172a" text-anchor="middle">create_note（ユースケース）</text>
<line x1="545" y1="84" x2="545" y2="106" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-06)"/>
<rect x="430" y="110" width="230" height="40" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="545" y="135" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">trait NoteRepository</text>
<rect x="400" y="176" width="130" height="40" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="465" y="201" font-size="12" fill="#047857" text-anchor="middle">メモリ / Fake</text>
<rect x="560" y="176" width="130" height="40" rx="8" fill="#f8fafc" stroke="#64748b"/>
<text x="625" y="201" font-size="12" fill="#475569" text-anchor="middle">Turso 版</text>
<line x1="465" y1="176" x2="510" y2="154" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-06)"/>
<line x1="625" y1="176" x2="580" y2="154" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-06)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-5: ユースケースも実装も trait のほうを向く。保存先を差し替えてもユースケースは変わらない</div>
</div>

図4-5 の右側では、矢印がすべて真ん中の trait に集まっています。ユースケースは「`NoteRepository` という約束を守る何か」しか知らず、実装の側も「約束を守る」ことだけを意識します。この形にしておくと、**テストでは Fake を、本番では Turso 版を**渡すだけで、ユースケースのコードは1行も変えずに済みます。

### まず動かす最小コード

#### 例2-1: Summary trait を2つの型に実装する

> **▼ このコードがやること（先に日本語で）:** 「一行の要約を作れる」という約束 `Summary` を決め、ノートとユーザーの2つの型に実装します。そのうえで「約束を守る型なら何でも受け取る」関数 `print_headline` に、ノートとユーザーの両方を渡してみます。

<!--CODE:design_lab/src/bin/u02_trait.rs-->

```powershell
cargo run --bin u02_trait
```

**▼ 実行結果**

<!--OUT:u02_trait-->

#### ▼ コードを1つずつ分解して解説

##### 解説1: trait を定義する

```rust
trait Summary {
    fn summary(&self) -> String;

    fn headline(&self) -> String {
        format!("【{}】", self.summary())
    }
}
```

- `trait 名前 { ... }` で約束を定義します。中にはメソッドの「形」（名前・引数・戻り値の型）を並べます。
- `fn summary(&self) -> String;` のように `;` で終わるメソッドは、**中身を書いていない**メソッドです。この trait を実装する型は、必ず中身を書かなければなりません。
- `headline` のように `{ ... }` で中身まで書いたものは**既定の実装**（デフォルト実装）です。実装する型が何も書かなければ、これがそのまま使われます。既定の実装の中から、まだ中身のない `self.summary()` を呼べるのがポイントです。

##### 解説2: `impl トレイト for 型` で約束を守る

```rust
impl Summary for Note {
    fn summary(&self) -> String {
        let mark = if self.done { "済" } else { "未" };
        format!("{mark} {}", self.title)
    }
}
```

- 「`Note` は `Summary` の約束を守ります」という宣言です。必須の `summary` だけを書き、`headline` は既定の実装に任せています。だから実行結果の2行目は `【未 牛乳を買う】` でした。
- `User` のほうは `headline` も自分で書いて既定の実装を**上書き**しています。だから3行目は `<<alice@example.com>>` になりました。

##### 解説3: `&impl Summary` ＝「約束を守る型なら何でも」

```rust
fn print_headline(item: &impl Summary) {
    println!("{}", item.headline());
}
```

- 引数の型 `&impl Summary` は、「`Summary` を実装している何らかの型への参照」という意味です。
- 関数の中で使えるのは、**約束に書いてあるメソッドだけ**です。`item.title` のように `Note` にしかないフィールドには触れません。触れないからこそ、`Note` でも `User` でも受け取れるのです。
- このように「型に求める条件」を **trait 境界**（trait bound）と呼びます。「この関数に渡すには `Summary` が必要」という境界線を引いているイメージです。次の UNIT 03 で、もっと正式な書き方 `<T: Summary>` を学びます。

> **用語:** trait 境界（trait bound：「この型は○○ trait を実装していること」という条件。関数や構造体が、受け取る型に求める）

### もう一歩: 実務に近い形

いよいよ RustNote の中心になる `NoteRepository` を作ります。リポジトリ（repository：倉庫）は「データのしまい方・取り出し方」を引き受ける部品の名前としてよく使われます。

#### 例2-2: NoteRepository trait と、2つの実装

> **▼ このコードがやること（先に日本語で）:** 「ノートを入れる」「持ち主ごとに一覧する」という約束 `NoteRepository` を決め、①メモリに保存する本物っぽい実装と、②必ず失敗する偽物の実装を作ります。ノートを作る処理 `create_note` は約束だけに頼って書き、2つの実装を差し替えて動かします。

<!--CODE:design_lab/src/bin/u02_repo.rs-->

```powershell
cargo run --bin u02_repo
```

**▼ 実行結果**

<!--OUT:u02_repo-->

##### 解説4: trait のメソッドは `&self` で受ける

```rust
trait NoteRepository {
    fn insert(&self, owner: UserId, title: &str) -> Result<Note, RepoError>;
    fn list_by_owner(&self, owner: UserId) -> Result<Vec<Note>, RepoError>;
}
```

- 「ノートを入れる」は中身を変える操作なので `&mut self` にしたくなりますが、ここではあえて `&self`（共有の借用）にしています。
- 理由は、Web サーバーでは**たくさんのリクエストが同時に同じリポジトリを使う**からです。`&mut self` だと同時に1人しか使えません（第3巻の借用ルール）。本物の DB 接続も、内部で順番待ちを管理してくれるので `&self` で使えます。
- そのため、メモリ版では `Mutex`（第3巻）で中身を包み、`&self` のままでも安全に書き換えられるようにしています（`self.inner.lock().unwrap()`）。
- どちらのメソッドも `Result` を返します。メモリ版は失敗しませんが、DB 版は「接続できない」などで失敗しうるので、**約束の時点で失敗の可能性を書いておく**のです。

##### 解説5: ユースケースは約束しか知らない

```rust
fn create_note(repo: &impl NoteRepository, owner: UserId, title: &str) -> Result<NoteId, String> {
```

- `create_note` は `InMemoryNoteRepository` という名前を一度も書いていません。知っているのは `NoteRepository` という約束だけです。
- だから `main` の最後で `BrokenRepository` を渡しても、そのまま動きました。実行結果の最終行 `Err("保存に失敗しました: DB に接続できません")` は、「DB が落ちていたらこう返る」という動きを、**本物の DB を落とさずに**確かめられたことを示しています。
- このように、上の層（ユースケース）が下の層（保存先の具体的な作り）ではなく、間に置いた約束に頼る設計を**依存性の逆転**（dependency inversion）と呼びます。

> **用語:** Fake（フェイク：テストのために用意する、本物と同じ約束を守る簡単な作りの実装。メモリ版や「必ず失敗する版」など）

<!-- -->

> **ヒント:** この例では、失敗をとりあえず `String` で返しています。文字列のエラーでは「入力が悪いのか、DB が悪いのか」をプログラムが見分けにくいので、UNIT 05 で専用のエラー型 `AppError` に置き換えます。

##### 解説6: 持ち主で絞り込むのは「保存先の約束」の一部

```rust
.filter(|n| n.owner == owner)
```

- `list_by_owner` は「その持ち主のノートだけ」を返します。実行結果で、bob のノート（ID 2）が alice の一覧に出てこないことを確認してください。
- 「他人のノートを見せない」は RustNote で最も大事なルールの1つです。それを**メソッドの名前と引数**（`owner: UserId` を必ず受け取る）で表しておくと、うっかり全員分を返す実装を書きにくくなります。

### よくあるエラーと読み方

#### エラー1: E0277 trait が実装されていない

`Summary` を実装していない `42`（整数）を `print_summary` に渡したときのエラーです。

<!--CODE:errs/u02_err_e0277.rs-->

<!--OUT:u02_err_e0277-->

- **何が起きたか:** `E0277` は「trait 境界を満たしていない」エラーです。`the trait Summary is not implemented for {integer}`（整数には `Summary` が実装されていない）と書かれています。`{integer}` は「まだ i32 か i64 か決まっていない整数」という意味です。
- 下の `note: required by a bound in print_summary` が、「その条件を出したのは `print_summary` の `impl Summary` の部分ですよ」と、境界の場所まで教えてくれています。`help` には「`Note` なら実装されていますよ」とヒントも出ています。
- **直し方:** 渡す値を `Summary` を実装した型にするか、その型に `impl Summary for 型` を書き足します。E0277 は Rust で最もよく見るエラーの1つなので、「`the trait X is not implemented for Y`」の X と Y を読み取る練習をしておきましょう。

#### エラー2: E0046 必須のメソッドを書き忘れた

<!--CODE:errs/u02_err_e0046.rs-->

<!--OUT:u02_err_e0046-->

- **何が起きたか:** `not all trait items implemented, missing: summary`（trait の項目が全部実装されていない。`summary` が足りない）。既定の実装がない `summary` を書き忘れています。
- **直し方:** `impl Summary for User` の中に `fn summary(&self) -> String { ... }` を書き足します。rust-analyzer を使っていれば、`impl` の行で「Implement missing members」というクイック修正を選ぶと、ひな形を自動で入れてくれます。

#### エラー3: E0117 よその trait を よその型に実装できない（孤児ルール）

<!--CODE:errs/u02_err_e0117.rs-->

<!--OUT:u02_err_e0117-->

- **何が起きたか:** `Display` も `Vec` も標準ライブラリのもの（自分の crate の外で定義されたもの）です。Rust では、**trait か型の少なくとも一方が自分の crate のもの**でないと `impl` を書けません。これを**孤児ルール**（orphan rule）と呼びます。もし誰でも `Vec` に `Display` を実装できたら、2つのライブラリが別々の実装をしたときにどちらを使うか決められなくなるからです。
- **直し方:** 最後の `note: define and implement a trait or new type instead` が答えです。UNIT 01 の newtype で `Vec<String>` を包み、自分の型にしてから実装します（練習問題3の解答を見てください）。

> **用語:** crate（クレート：Rust のコンパイルの単位。自分のプロジェクトも、`cargo add` で入れるライブラリも、それぞれ1つの crate。UNIT 06 で詳しく扱う）

### RustNote ではここで使う

- RustNote のユースケース（ノートの作成・一覧・編集・削除・完了切替）は、すべて `NoteRepository` trait を通して保存先にアクセスします。
- 本番では Turso / libSQL に SQL を送る実装を使い（第7巻）、テストではメモリ版や失敗する Fake を使います（第11巻）。
- 本物の DB 操作は「待ち時間」があるので、実際の RustNote では `async fn`（第5巻）を使った形になります。この単元では、約束と差し替えの考え方に集中するため、async を使わない形で書きました。

### 練習問題

**問題1:** タグを表す型 `Tag { name: String, count: usize }` を作り、`Summary` を実装してください。`summary` は `#買い物 (3件)` のような文字列を返すこととし、`headline` は既定の実装を使います。`print_headline` に渡して表示してください。

**問題2:** ログインのセッションには有効期限があります。「今の時刻（1970年からの秒数）を返す」trait `Clock` を作り、OS の時計を読む `SystemClock` と、決めた時刻を返し続けるテスト用の `FixedClock(u64)` を実装してください。さらに `fn is_expired(clock: &impl Clock, expires_at: u64) -> bool` を書き、期限の1秒前・ちょうどで結果を確かめてください。

**問題3:** エラー3（E0117）のコードを、newtype を使ってコンパイルが通るように直してください。

### 解答と解説

**解答1:**

<!--CODE:design_lab/src/bin/u02_ex1.rs-->

**▼ 実行結果**

<!--OUT:u02_ex1-->

- `headline` を書かなかったので、trait の既定の実装（`【...】` で囲む）が使われています。

**解答2:**

<!--CODE:design_lab/src/bin/u02_ex2.rs-->

**▼ 実行結果**

<!--OUT:u02_ex2-->

- 時刻のように「実行するたびに変わるもの」に直接頼ると、テストの結果が実行のタイミングで変わってしまいます。trait で差し替え可能にしておけば、テストでは `FixedClock` を渡して「期限ちょうど」のような境目を、何度でも同じ結果で確かめられます。
- `use std::time::{...}` を関数の中に書いているのは、その関数でしか使わないものを近くに置くためです。ファイルの先頭に書いてもかまいません。

**解答3:**

<!--CODE:design_lab/src/bin/u02_ex3.rs-->

**▼ 実行結果**

<!--OUT:u02_ex3-->

- `TitleList` は自分の crate で定義した型なので、孤児ルールに引っかかりません。UNIT 01 の newtype は「意味を分ける」だけでなく、「よその型に trait を足す」ための道具にもなります。

### まとめ

- trait は「型が守る約束」です。`impl トレイト for 型` で約束を守り、既定の実装で書く量を減らせます。
- `&impl Trait` の引数は「約束を守る型なら何でも」受け取り、関数の中では約束のメソッドだけを使います。
- ユースケースを `NoteRepository` trait に依存させると、本物の保存先とテスト用 Fake を差し替えられます。

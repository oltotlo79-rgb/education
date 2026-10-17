## UNIT 10: テスト・rustdoc（doctest）・状態機械・API レビュー

### この単元のゴール

- `#[cfg(test)]`・`#[test]`・`assert_eq!` で単体テストを書き、`cargo test` の成功と失敗の出力を読める
- `tests/` フォルダの結合テストと、ドキュメントの例を実行する **doctest** を書ける
- 「下書き → 公開 → アーカイブ」のような**状態の移り変わり（状態機械）**を enum と `match` で書き、許されない移り変わりをエラーにして、全部の組み合わせをテストできる
- 公開する API（関数・型）を、名前・公開範囲・エラー・例・`#[must_use]` の観点でレビューできる

### たとえ話でつかむ

家電の取扱説明書を思い浮かべてください。よい説明書には、①使い方の**例**（「このボタンを押すと、こうなります」）、②**やってはいけないこと**（「水に濡らさないでください」）、③**故障したときの表示**（「E3 が出たらフィルターを掃除」）が書いてあります。そして工場では、出荷前に1台ずつ**検査**をしています。

- 工場の検査 ＝ **テスト**。部品ごとの検査（単体テスト）と、組み立てた製品の検査（結合テスト）がある
- 説明書の使い方の例 ＝ **ドキュメント（rustdoc）の例**。Rust では、この例が**本当に動くかまで `cargo test` で検査されます**（doctest）。説明書と製品が食い違うことがありません
- 「この状態ではこのボタンは効かない」という決まり ＝ **状態機械**。洗濯機が「脱水中はふたが開かない」ように、ノートも「アーカイブ済みは公開できない」と決めて、守らせます

UNIT 01〜09 で作った「間違いを防ぐ設計」も、テストとドキュメントに残しておかなければ、半年後の自分やチームの誰かが、うっかり壊してしまいます。この単元は、**設計の意図を固定する**ための道具をそろえます。

> **用語:** 単体テスト（unit test：関数や型を1つずつ、ほかから切り離して確かめるテスト）／結合テスト（integration test：ライブラリを外から使う立場で、いくつかの部品を組み合わせて確かめるテスト）

<!-- -->

> **用語:** 状態機械（じょうたいきかい：state machine。「今の状態」と「操作」の組み合わせで「次の状態」が決まる仕組み。許されない組み合わせを決めておける）

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<polygon points="360,20 260,100 460,100" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<polygon points="260,100 160,180 560,180 460,100" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<polygon points="160,180 60,260 660,260 560,180" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="74" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">E2E・HTTP</text>
<text x="360" y="92" font-size="12" fill="#c2410c" text-anchor="middle">（第11巻）</text>
<text x="360" y="138" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">結合テスト</text>
<text x="360" y="160" font-size="12" fill="#1e40af" text-anchor="middle">tests/*.rs（ライブラリを外から使う）</text>
<text x="360" y="218" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">単体テスト</text>
<text x="360" y="240" font-size="12" fill="#047857" text-anchor="middle">#[cfg(test)] mod tests（関数・型ごと）</text>
<rect x="10" y="20" width="200" height="64" rx="8" fill="#f8fafc" stroke="#64748b" stroke-dasharray="5 4"/>
<text x="110" y="46" font-size="12" font-weight="700" fill="#475569" text-anchor="middle">doctest</text>
<text x="110" y="68" font-size="12" fill="#475569" text-anchor="middle">/// の例が動くか確かめる</text>
<text x="600" y="50" font-size="12" fill="#475569" text-anchor="middle">上ほど: 少ない・遅い</text>
<text x="600" y="70" font-size="12" fill="#475569" text-anchor="middle">本物の動きに近い</text>
<text x="360" y="286" font-size="12" fill="#475569" text-anchor="middle">下ほど: 数が多い・速い・失敗したとき原因の場所がすぐ分かる</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-20: テストピラミッド。速くて細かいテストを土台にたくさん置き、遅いテストは少なくする</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-21" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<circle cx="24" cy="140" r="8" fill="#475569"/>
<line x1="32" y1="140" x2="56" y2="140" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-21)"/>
<rect x="60" y="108" width="140" height="64" rx="30" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="130" y="136" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">Draft</text>
<text x="130" y="156" font-size="12" fill="#047857" text-anchor="middle">下書き（編集できる）</text>
<rect x="500" y="26" width="160" height="64" rx="30" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="580" y="54" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">Published</text>
<text x="580" y="74" font-size="12" fill="#1e40af" text-anchor="middle">公開中</text>
<rect x="500" y="200" width="160" height="64" rx="30" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="580" y="228" font-size="14" font-weight="700" fill="#475569" text-anchor="middle">Archived</text>
<text x="580" y="248" font-size="12" fill="#475569" text-anchor="middle">アーカイブ済み</text>
<line x1="200" y1="120" x2="496" y2="50" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-21)"/>
<text x="330" y="74" font-size="12" fill="#1e40af" text-anchor="middle">Publish</text>
<line x1="500" y1="76" x2="204" y2="138" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-21)"/>
<text x="380" y="122" font-size="12" fill="#1e40af" text-anchor="middle">Unpublish</text>
<line x1="200" y1="152" x2="496" y2="214" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-21)"/>
<text x="300" y="166" font-size="12" fill="#475569" text-anchor="middle">Archive</text>
<line x1="500" y1="240" x2="204" y2="166" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-21)"/>
<text x="380" y="242" font-size="12" fill="#047857" text-anchor="middle">Restore</text>
<line x1="580" y1="90" x2="580" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-21)"/>
<text x="628" y="148" font-size="12" fill="#475569" text-anchor="middle">Archive</text>
<text x="360" y="284" font-size="12" fill="#c2410c" text-anchor="middle">矢印のない組み合わせ（Archived で Publish など）は TransitionError</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-21: ノートの状態機械。許された移り変わりだけが矢印になっている</div>
</div>

### まず動かす最小コード

#### 例10-1: cargo new --lib が作るテストを動かす

> **▼ このコードがやること（先に日本語で）:** ライブラリ用のプロジェクトを作ると最初から入っている「2 + 2 が 4 になるか」のテストを、そのまま実行します。次に、わざと期待する値を間違えて、テストが失敗したときの表示を確かめます。

```powershell
cargo new --lib note_domain
cd note_domain
```

<!--OUT:u10_new-->

`src/lib.rs` には、次のコードが最初から書かれています。

<!--CODE:errs/u10_template_lib.rs-->

```powershell
cargo test
```

**▼ 実行結果**

<!--OUT:u10_template_test-->

#### ▼ コードを1つずつ分解して解説

##### 解説1: `#[cfg(test)] mod tests`

```rust
#[cfg(test)]
mod tests {
    use super::*;
```

- `#[cfg(test)]` は「テストのときだけコンパイルする」という印です（UNIT 07 の `#[cfg(feature = ...)]` の仲間）。`cargo build` で作る本番のプログラムには、テストのコードは入りません。
- テストは、テストされるコードと**同じファイルの中**の `mod tests` に書くのが Rust の習慣です。子の module なので、`use super::*;`（親の module のものを全部使う）で、private な関数もテストできます（UNIT 06）。

##### 解説2: `#[test]` と `assert_eq!`

```rust
#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
```

- `#[test]` を付けた関数が1つのテストです。`cargo test` は、この印の付いた関数を全部探して実行します。
- `assert_eq!(実際の値, 期待する値)` は、2つが等しくなければパニックを起こします。**テストの中でパニックが起きたら失敗、最後まで行けば成功**です。ほかに、`assert!(条件)`（条件が `true` か）、`assert_ne!(a, b)`（等しくないか）もあります。

##### 解説3: 実行結果の読み方

- `Running unittests src\lib.rs` の下が単体テストの結果、`Doc-tests note_domain` の下が doctest（ドキュメントの例）の結果です。まだ例を書いていないので `running 0 tests` です。
- `test result: ok. 1 passed; 0 failed; ...` の `passed` が成功、`failed` が失敗の数です。`ignored`（`#[ignore]` で飛ばしたもの）、`filtered out`（名前で絞り込んで除いたもの）もここに出ます。

#### 例10-2: テストが失敗したときの表示

`assert_eq!(result, 4);` を、わざと `assert_eq!(result, 5);` に変えて実行します。

```powershell
cargo test
```

**▼ 実行結果**

<!--OUT:u10_template_fail-->

- `test tests::it_works ... FAILED` で、どのテストが失敗したか分かります。
- その下の `panicked at src\lib.rs:12:9` が失敗した場所、`assertion left == right failed` と `left: 4` / `right: 5` が「実際の値は 4、期待した値は 5」という中身です。`assert_eq!` の**1つ目に実際の値、2つ目に期待する値**を書く習慣にしておくと、この表示が読みやすくなります。
- 最後の `error: test failed, to rerun pass --lib` は、「ライブラリのテストだけをやり直すなら `cargo test --lib`」という案内です。失敗したら、終了コードも失敗（0 以外）になるので、CI（第11巻）はここで止まってくれます。

### もう一歩: 実務に近い形

ひな形のコードを消して、RustNote のドメイン型の一部（`NoteTitle` と、ノートの状態 `NoteStatus`）を、ドキュメントとテスト付きのライブラリとして作ります。thiserror を `cargo add thiserror` で入れ、`Cargo.toml` に `rust-version = "1.98"` を足しておきます。

```text
note_domain/
├── Cargo.toml
├── src/
│   ├── lib.rs       … crate の説明（//!）と再公開
│   ├── title.rs     … NoteTitle ＋ 単体テスト ＋ doctest
│   └── status.rs    … NoteStatus（状態機械）＋ 単体テスト ＋ doctest
└── tests/
    └── note_flow.rs … 結合テスト
```

#### 例10-3: ドキュメントとテスト付きのドメイン型

> **▼ このコードがやること（先に日本語で）:** タイトルの型と、ノートの状態（下書き・公開・アーカイブ）の型を作り、それぞれに「使い方の例」付きの説明を書きます。単体テストで境界の値（80 文字と 81 文字）や、状態と操作の全部の組み合わせを確かめ、結合テストでライブラリを外から使う流れを確かめます。

**src/lib.rs**

<!--CODE:note_domain/src/lib.rs-->

**src/title.rs**

<!--CODE:note_domain/src/title.rs-->

**src/status.rs**

<!--CODE:note_domain/src/status.rs-->

**tests/note_flow.rs**

<!--CODE:note_domain/tests/note_flow.rs-->

```powershell
cargo test
```

**▼ 実行結果**

<!--OUT:u10_test-->

> **注意:** 単体テストの並び順は、実行するたびに変わることがあります。`cargo test` は、速く終わるようにテストを**同時に（並列に）**実行するためです。だから、テスト同士が順番や共有の状態（ファイル・環境変数など）に頼らないように書きます（UNIT 09 の `set_var` が unsafe な理由の1つです）。

##### 解説4: `///` と `//!` はドキュメントになるコメント

```rust
/// 前後の空白を除いて 1〜80 文字であることが保証されたタイトルです。
///
/// 作る方法は [`NoteTitle::parse`] だけです。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteTitle(String);
```

- `///` で始まるコメントは、**すぐ下の項目の説明書き**になります。`//!` は、そのファイル（module や crate）自体の説明です。中には Markdown が書けます。
- `` [`NoteTitle::parse`] `` のように角かっこで囲むと、生成されたドキュメントの中で、その項目へのリンクになります。
- `cargo doc --open` を実行すると、ブラウザでドキュメントが開きます（`--no-deps` を付けると、依存クレートの分を省いて速く作れます）。

```powershell
cargo doc --no-deps
```

<!--OUT:u10_doc-->

`Generated` の行のファイル（`target\doc\note_domain\index.html`）をブラウザで開くと、標準ライブラリのドキュメント（docs.rs）と同じ見た目の説明書が見られます。

##### 解説5: `# Errors` と `# Examples`（doctest）

```rust
/// # Errors
///
/// - 空白を除いて空なら [`TitleError::Empty`]
/// - 80 文字を超えたら [`TitleError::TooLong`]
///
/// # Examples
///
/// ```
/// use note_domain::{NoteTitle, TitleError};
///
/// let title = NoteTitle::parse("  牛乳を買う ").unwrap();
/// assert_eq!(title.as_str(), "牛乳を買う");
///
/// assert_eq!(NoteTitle::parse("   "), Err(TitleError::Empty));
/// ```
```

- `# Errors` は「どんなときに `Err` を返すか」、`# Examples` は使い方の例を書く、Rust で広く使われる見出しです。パニックする可能性がある関数には `# Panics` も書きます。
- `# Examples` の中の ```` ``` ```` で囲んだコードは、`cargo test` のときに**本当にコンパイルして実行されます**。これが **doctest** です。実行結果の `Doc-tests note_domain` の下に、`title::NoteTitle::parse (line 35) ... ok` と出ていたのがそれです。
- doctest は、ライブラリを**外から使う立場**で動きます。だから `use note_domain::...` と書き、`pub` なものしか使えません。説明書の例が、利用者の書くコードそのものになるわけです。
- 最後の `all doctests ran in ...; merged doctests compilation took ...` は、edition 2024 で doctest をまとめてコンパイルして速くする仕組みが働いた、という報告です。

##### 解説6: 境界値テストと、`Result` を返すテスト・`should_panic`

```rust
// 境界値: 80 文字ちょうどは OK、81 文字は NG
#[test]
fn boundary_80_and_81_chars() {
    let ok = "あ".repeat(MAX_TITLE_CHARS);
    let ng = "あ".repeat(MAX_TITLE_CHARS + 1);
    assert!(NoteTitle::parse(&ok).is_ok());
    assert_eq!(NoteTitle::parse(&ng), Err(TitleError::TooLong(81)));
}
```

- バグは「ちょうど境目」に潜みがちです（`>` と `>=` の書き間違いなど）。80 文字と 81 文字の両方を確かめるのが**境界値テスト**です。日本語の「あ」で作っているので、UNIT 01 の「バイト数で数えてしまう」バグもここで見つかります。
- テスト関数が `Result<(), E>` を返すようにすると、中で `?` が使えます。`Err` が返ったらテストは失敗です。
- `#[should_panic(expected = "Empty")]` は「パニックすれば成功」というテストで、`expected` にはパニックのメッセージに含まれるべき文字列を書きます。実行結果では `- should panic ... ok` と表示されていました。

##### 解説7: 状態機械を enum と `match` で書く

```rust
pub fn apply(self, action: Action) -> Result<NoteStatus, TransitionError> {
    use Action::*;
    use NoteStatus::*;
    match (self, action) {
        (Draft, Publish) => Ok(Published),
        (Published, Unpublish) => Ok(Draft),
        (Draft | Published, Archive) => Ok(Archived),
        (Archived, Restore) => Ok(Draft),
        (from, action) => Err(TransitionError { from, action }),
    }
}
```

- 「今の状態」と「操作」の**組（タプル）**で `match` すると、図4-21 の矢印1本が1行に対応します。`Draft | Published` は「どちらでも」という意味です。
- 最後の `(from, action) => Err(...)` が「それ以外は全部ダメ」です。許された移り変わりを列挙し、残りを拒否する形なので、**うっかり許してしまう組み合わせ**が生まれません。
- `apply` は `self` を受け取って新しい状態を返し、元の値は変えません。状態を `bool` を何個も組み合わせて表す（`is_published` と `is_archived` が両方 `true`…のような、ありえない状態が作れてしまう）代わりに enum にすることで、UNIT 01 と同じく**ありえない状態を表現できない**ようにしています。

##### 解説8: 全部の組み合わせを表でテストする

```rust
for (from, action, expected) in table {
    let got = from.apply(action).ok();
    assert_eq!(got, expected, "{from:?} に {action:?} を当てたとき");
}
```

- 状態3つ × 操作4つ ＝ 12 通りを、「期待する次の状態（ダメなら `None`）」と一緒に表にして、ループで確かめています。このような書き方を**表駆動テスト**（table-driven test）と呼びます。
- `assert_eq!` の3つ目以降に書いたメッセージは、失敗したときに表示されます。ループの中のテストでは、**どの組み合わせで失敗したのか**が分かるように、必ず書いておきましょう。

##### 解説9: 結合テストは「外から」使う

```rust
// tests/ フォルダのファイルは、ライブラリを「外から」使うテスト（結合テスト）
use note_domain::{Action, NoteStatus, NoteTitle, TitleError};
```

- `tests/` フォルダの中の `.rs` ファイルは、それぞれが**別の crate** としてコンパイルされます（UNIT 06 の main.rs と同じ）。`pub` で公開されたものしか使えないので、「利用者から見て、API がちゃんと使えるか」を確かめられます。
- 実行結果の `Running tests\note_flow.rs` の下が、結合テストの結果です。

##### 解説10: 名前で絞り込んで実行する

テストが増えたら、名前の一部を指定して、関係するテストだけを実行できます。

```powershell
cargo test transition
```

<!--OUT:u10_test_filter-->

- 名前に `transition` を含む `status::tests::transition_table` だけが実行され、ほかは `filtered out`（除外）になりました。直している最中のテストだけを素早く何度も回すときに便利です。

#### API レビューの観点

ライブラリやモジュールの `pub` な部分（API）は、一度使われ始めると変えにくくなります。公開する前に、次の観点で見直しましょう。この単元の `note_domain` は、すべてに答えられるように作ってあります。

| 観点 | 確かめること | note_domain での答え |
|---|---|---|
| 名前 | 何をするか名前で分かるか。標準の慣習（`parse`・`as_str`・`is_...`）に沿っているか | `NoteTitle::parse`・`as_str`・`is_editable` |
| 公開範囲 | `pub` は必要最小限か。中身を直接いじれないか（UNIT 06） | `NoteTitle` の中身は private |
| 不正な状態 | 型で防げる間違いを、型で防いでいるか（UNIT 01） | `NoteTitle` は parse でしか作れない。状態は enum |
| エラー | 失敗の種類を enum で返しているか。`# Errors` に書いたか（UNIT 05） | `TitleError`・`TransitionError` |
| パニック | 利用者の入力でパニックしないか。する場合は `# Panics` に書いたか | 入力ではパニックしない |
| 例 | `# Examples` があり、doctest で動くか | `parse`・`apply` に例 |
| 結果の捨て忘れ | 戻り値を無視したらバグになる関数に `#[must_use]` を付けたか | `as_str`・`is_editable` |
| 秘密・表示 | `Debug` に秘密が出ないか（UNIT 09） | 秘密は持たない |

`#[must_use]` は、`Result` と同じ「捨てたら警告」を自分の関数に付ける印です。たとえば、完了・未完了を入れ替えた**新しい値を返す** `toggle` に付けておくと、`progress.toggle();` のように結果を捨てたとき、次の警告が出ます（練習問題1の型を使った例）。

<!--CODE:errs/u10_forget.rs-->

<!--OUT:u10_mustuse-->

- `toggle` は値そのものを書き換えるのではなく、新しい値を返す設計なので、結果を捨てると何も起きません（最後の表示は `Open` のまま）。`#[must_use]` のおかげで、このバグにコンパイルの段階で気づけます。

### よくあるエラーと読み方

#### エラー1: テストの失敗（assert_eq! の不一致）

例10-2 で見たとおりです。読むべき場所は3つです。

1. `test 名前 ... FAILED` … **どのテスト**が失敗したか
2. `panicked at ファイル:行:列` … **どこで**失敗したか
3. `left:` / `right:`（と、自分で書いたメッセージ）… **何が違った**か

失敗したテストを直すときは、まず「テストが正しいのか、コードが正しいのか」を考えます。仕様（たとえば「タイトルは 1〜80 文字」）に照らして、間違っているほうを直します。テストを通すためだけに期待する値を書き換えるのは、検査の意味がなくなるので禁物です。

#### エラー2: doctest の失敗

`NoteTitle::parse` の例で、期待する値を `"  牛乳を買う "`（前後の空白つき）と書き間違えた場合です。

```powershell
cargo test --doc
```

<!--OUT:u10_doc_fail-->

- `src\title.rs - title::NoteTitle::parse (line 35) ... FAILED` で、**どの項目の説明の、何行目から始まる例**かが分かります。
- `panicked at ...\doctest_bundle_2024.rs:21:1` の場所は、cargo が doctest を実行するために一時的に作ったファイルなので、行番号はあてになりません。中身の `left:`（実際は `"牛乳を買う"`）と `right:`（例に書いた `"  牛乳を買う "`）を見て、**説明書の例のほうが間違っている**と分かります。
- doctest が失敗するということは、「説明書の例をそのまま写した利用者のコードが動かない」ということです。ドキュメントを直すか、コードを直すかを決めて、食い違いをなくします。`cargo test --doc` は doctest だけを実行するオプションです。

### RustNote ではここで使う

- RustNote の `NoteTitle`・`Email`・`NoteId` などのドメイン型には、この単元のように `# Errors` と `# Examples` を書き、境界値（0 文字・1 文字・80 文字・81 文字）を単体テストで確かめます。
- RustNote のノートの「完了切替（done）」は、`Open` と `Done` の2つの状態を行き来するだけの、いちばん小さな状態機械です（練習問題1）。下書き・公開・アーカイブの3状態は、この単元の練習用の例で、RustNote の画面にはありません。
- 保存先を `NoteRepository` trait にしておいたので（UNIT 02）、ユースケースのテストではメモリ版や失敗する Fake を渡せます。HTTP の画面を通したテスト、セキュリティの回帰テスト、CI での自動実行は第11巻で扱います。

### 練習問題

**問題1:** RustNote のノートの完了状態を表す enum `Progress { Open, Done }` を作り、完了と未完了を入れ替える `toggle(self) -> Progress`（`#[must_use]` 付き）と、DB の `done` 列の値から作る `from_done(done: bool) -> Progress` を書いてください。「入れ替わる」「2回入れ替えると元に戻る」「`done` 列から正しく作れる」の3つを単体テストで確かめてください。

**問題2:** 例10-3 の `NoteStatus::is_editable` に、`# Examples` 付きの説明を書いてください。`Draft` は編集できて `Published` は編集できないことを例で示し、`cargo test --doc` で doctest が3つになることを確かめてください。

**問題3:** 同僚が次の関数を `pub` で公開しようとしています。API レビューの観点で、問題点を3つ以上挙げてください。

```text
pub fn new_note(title: String, owner: i64, status: String) -> Note
```

### 解答と解説

**解答1:**

<!--CODE:ex10/toggle/src/lib.rs-->

```powershell
cargo test
```

**▼ 実行結果**

<!--OUT:u10_ex1-->

- 「2回入れ替えると元に戻る」のように、**どんな値でも成り立つ性質**をテストするのは強力です。ここでは2通りしかないので `for` で全部試しました。値の候補が多いときに、たくさんの値を自動で作って性質を確かめる方法（プロパティテスト）は第11巻で扱います。

**解答2:** `status.rs` の `is_editable` の説明を、次のように書き足します（抜粋）。

```rust
/// 編集できる状態なら `true` です。
///
/// # Examples
///
/// ```
/// use note_domain::NoteStatus;
///
/// assert!(NoteStatus::Draft.is_editable());
/// assert!(!NoteStatus::Published.is_editable());
/// ```
#[must_use]
pub fn is_editable(self) -> bool {
```

```powershell
cargo test --doc
```

**▼ 実行結果**

<!--OUT:u10_ex2-->

- `status::NoteStatus::is_editable (line 63)` が増えて、doctest が3つになりました。

**解答3:**（例）

- **タイトルが `String` のまま:** 空や 81 文字以上のタイトルでもノートが作れてしまいます。`NoteTitle` を受け取るべきです（UNIT 01）。
- **持ち主が `i64` のまま:** ノートの ID など、ほかの番号と取り違えてもコンパイルが通ります。`UserId` を受け取るべきです（UNIT 01）。
- **状態が `String`:** `"publshed"` のようなつづり間違いでも通ってしまいます。`NoteStatus` の enum にし、作成直後は `Draft` に決めておけば、引数そのものが要りません（UNIT 10）。
- **失敗を返せない:** 戻り値が `Note` なので、検証に失敗したときに `Result` で理由を返せません。`String` のまま受け取るなら、`Result<Note, ...>` にして `# Errors` を書く必要があります（UNIT 05）。
- **説明と例がない:** `///` の説明・`# Examples` がなく、使い方が分かりません。

### まとめ

- `#[cfg(test)] mod tests` に `#[test]` 関数を書き、`assert_eq!(実際, 期待)` で確かめます。失敗の表示は「どのテスト・どこで・何が違ったか」の順に読みます。
- `///` の説明に `# Errors` と `# Examples` を書くと、例は doctest として `cargo test` で実行され、説明書とコードの食い違いを防げます。`tests/` には外から使う結合テストを置きます。
- 状態の移り変わりは enum と `(状態, 操作)` の `match` で書き、許されたものだけを列挙し、全部の組み合わせを表駆動テストで確かめます。

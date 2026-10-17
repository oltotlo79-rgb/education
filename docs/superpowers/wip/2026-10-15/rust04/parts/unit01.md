## UNIT 01: newtype と不正状態を作れない型

### この単元のゴール

- 同じ `i64` でも「ユーザーの番号」と「ノートの番号」を**別の型**に分け、取り違えをコンパイルエラーにできる
- 中身を private にした型と `parse` 関数で、「**作れた時点で必ず正しい値**」を表現できる
- `Email` と `NoteTitle` を自分で書き、`Result` で「なぜダメだったか」を返せる
- コンパイラが出す「直し方の提案（help）」を、設計の意図に照らして取捨選択できる

### たとえ話でつかむ

コンサート会場を思い浮かべてください。入口のゲートでは係員がチケットを1枚ずつ確認し、本物だった人にだけ**リストバンド**を巻きます。会場の中では、売店の人もトイレの前の係員も、リストバンドを見るだけで「この人はチケット確認済み」と分かります。いちいちチケットを見せてもらう必要はありません。

プログラムも同じです。外から来た文字列（フォームの入力、URL の一部、DB から読んだ値）は「まだ確認していないチケット」です。入口で一度だけ検査し、合格したものを `NoteTitle` や `Email` という**専用の型（リストバンド）**に包んでおけば、アプリの中の関数は「`NoteTitle` 型の値を受け取った＝もう検査済み」と信じてよくなります。

もう1つのたとえは**色の違う封筒**です。社員番号も伝票番号も「ただの数字」ですが、社員番号は青い封筒、伝票番号は赤い封筒に入れるルールにすれば、「赤い封筒を入れる箱」に青い封筒は入りません。Rust では、この「封筒の色」を型で表します。

> **用語:** newtype（ニュータイプ：既存の型を1つだけ包んだ、意味の違う新しい型。`struct UserId(i64);` のように書く）

<!-- -->

> **用語:** primitive obsession（プリミティブ・オブセッション：何でも `i64` や `String` のような基本の型のまま扱ってしまう癖。取り違えや未検証の値が紛れ込む原因になる）

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-03" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="110" y="28" font-size="14" font-weight="700" fill="#475569" text-anchor="middle">外から来た文字列（未検証）</text>
<rect x="20" y="44" width="180" height="40" rx="8" fill="#f8fafc" stroke="#64748b"/>
<text x="110" y="69" font-size="13" fill="#0f172a" text-anchor="middle">"  買い物リスト  "</text>
<rect x="20" y="100" width="180" height="40" rx="8" fill="#f8fafc" stroke="#64748b"/>
<text x="110" y="125" font-size="13" fill="#0f172a" text-anchor="middle">""（空文字）</text>
<rect x="20" y="156" width="180" height="40" rx="8" fill="#f8fafc" stroke="#64748b"/>
<text x="110" y="181" font-size="13" fill="#0f172a" text-anchor="middle">"あああ…"（81文字）</text>
<line x1="200" y1="64" x2="268" y2="110" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-03)"/>
<line x1="200" y1="120" x2="268" y2="120" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-03)"/>
<line x1="200" y1="176" x2="268" y2="130" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-03)"/>
<rect x="272" y="70" width="170" height="100" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="357" y="100" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">門番</text>
<text x="357" y="124" font-size="13" fill="#1e40af" text-anchor="middle">NoteTitle::parse</text>
<text x="357" y="146" font-size="12" fill="#1e40af" text-anchor="middle">trim → 1〜80文字か？</text>
<line x1="442" y1="100" x2="508" y2="72" stroke="#047857" stroke-width="2" marker-end="url(#ar-04-03)"/>
<line x1="442" y1="140" x2="508" y2="168" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-04-03)"/>
<rect x="512" y="40" width="190" height="60" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="607" y="64" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">Ok(NoteTitle)</text>
<text x="607" y="86" font-size="12" fill="#047857" text-anchor="middle">作れた＝必ず正しい</text>
<rect x="512" y="140" width="190" height="60" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="607" y="164" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">Err(TitleError)</text>
<text x="607" y="186" font-size="12" fill="#c2410c" text-anchor="middle">理由つきで入場お断り</text>
<rect x="20" y="226" width="682" height="56" rx="8" fill="#f8fafc" stroke="#cbd5e1" stroke-dasharray="5 4"/>
<text x="361" y="250" font-size="13" fill="#475569" text-anchor="middle">門の内側（アプリの中の関数）は NoteTitle を受け取るだけ。</text>
<text x="361" y="270" font-size="13" fill="#475569" text-anchor="middle">「空かも？長すぎるかも？」と何度も検査し直す必要がない</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-2: 検証済みの型の門番。parse を通った値だけが NoteTitle になる</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-04" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="10" width="340" height="210" rx="10" fill="#fff7ed" stroke="#f97316"/>
<text x="180" y="36" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">Before: 全部 i64</text>
<rect x="30" y="52" width="300" height="36" rx="6" fill="#ffffff" stroke="#f97316"/>
<text x="180" y="75" font-size="12" fill="#0f172a" text-anchor="middle">fn find_note_raw(owner_id: i64, note_id: i64)</text>
<rect x="30" y="100" width="300" height="36" rx="6" fill="#ffffff" stroke="#f97316"/>
<text x="180" y="123" font-size="12" fill="#0f172a" text-anchor="middle">find_note_raw(note_id, owner_id)  ← 逆</text>
<line x1="180" y1="136" x2="180" y2="160" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-04)"/>
<text x="180" y="182" font-size="13" fill="#c2410c" text-anchor="middle">コンパイルは通る</text>
<text x="180" y="204" font-size="13" fill="#c2410c" text-anchor="middle">→ 他人のノートを探すバグ</text>
<rect x="370" y="10" width="340" height="210" rx="10" fill="#ecfdf5" stroke="#10b981"/>
<text x="540" y="36" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">After: UserId と NoteId</text>
<rect x="390" y="52" width="300" height="36" rx="6" fill="#ffffff" stroke="#10b981"/>
<text x="540" y="75" font-size="12" fill="#0f172a" text-anchor="middle">fn find_note(owner: UserId, id: NoteId)</text>
<rect x="390" y="100" width="300" height="36" rx="6" fill="#ffffff" stroke="#10b981"/>
<text x="540" y="123" font-size="12" fill="#0f172a" text-anchor="middle">find_note(note, owner)  ← 逆</text>
<line x1="540" y1="136" x2="540" y2="160" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-04)"/>
<text x="540" y="182" font-size="13" fill="#047857" text-anchor="middle">error[E0308] で止まる</text>
<text x="540" y="204" font-size="13" fill="#047857" text-anchor="middle">→ 実行する前にバグが消える</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-3: 同じ「数字」でも型を分けると、引数の取り違えをコンパイラが見つけてくれる</div>
</div>

図4-2 の大事な点は、**検査する場所が1か所に決まっている**ことです。`NoteTitle` の中身は private（外から触れない）なので、`parse` を通らずに `NoteTitle` を作る方法がありません。だから「`NoteTitle` 型の値がある」＝「1〜80文字の検査に合格した」が、プログラム全体で必ず成り立ちます。これを「**不正な状態を表現できない型**（make illegal states unrepresentable）」と呼びます。

### まず動かす最小コード

この巻では、練習用のプロジェクトを1つ作り、例ごとに `src/bin/` の下へファイルを足していきます。`src/bin/名前.rs` に置いたファイルは、`cargo run --bin 名前` でそれぞれ独立したプログラムとして実行できます。

```powershell
# PowerShell（WSL / macOS / Linux のターミナルでも同じコマンドです）
cargo new design_lab
cd design_lab
mkdir src/bin
```

#### 例1-1: UserId と NoteId を別の型にする

> **▼ このコードがやること（先に日本語で）:** ユーザー番号とノート番号を、①ただの数字のまま扱う関数と、②専用の型で包んで扱う関数の2通りで書きます。①は引数の順番を間違えても動いてしまうことを確かめ、②は正しい順番で呼べることを確かめます。

<!--CODE:design_lab/src/bin/u01_ids.rs-->

`src/bin/u01_ids.rs` として保存し、次のコマンドで実行します。

```powershell
cargo run --bin u01_ids
```

**▼ 実行結果**

<!--OUT:u01_ids-->

2行目に注目してください。番号が逆になった「ユーザー42のノート7」を探しています。もし本物のアプリなら、**他人のノートを開いてしまう**かもしれない重大なバグですが、コンパイラは何も言いませんでした。どちらも `i64` なので、区別のしようがないからです。

#### ▼ コードを1つずつ分解して解説

##### 解説1: タプル構造体で newtype を作る

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(i64);
```

- `struct UserId(i64);` は**タプル構造体**（フィールドに名前がなく、番号で呼ぶ構造体）です。中に `i64` を1つだけ持ちます。
- 中身は同じ `i64` でも、`UserId` と `NoteId` は**まったく別の型**として扱われます。`UserId` を受け取る場所に `NoteId` は渡せません。
- 中の値は `owner.0` のように `.0`（0番目のフィールド）で取り出します。
- 実行時の大きさや速さは、包む前の `i64` と同じです。包んだことで余計なメモリを使うことはありません（**ゼロコスト**）。

> **用語:** タプル構造体（`struct 名前(型);` の形の構造体。フィールドを `.0` `.1` と番号で読む）

##### 解説2: `#[derive(...)]` で基本の能力を付ける

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
```

- `derive`（デライブ：自動で導き出す）は、よく使う能力をコンパイラに自動で実装させる指示です。
- `Debug` は `{:?}` で表示する能力、`Clone`/`Copy` は値を複製する能力、`PartialEq`/`Eq` は `==` で比べる能力です。
- `Copy` を付けたので、`find_note(owner, note)` に渡したあとも `owner` を使い続けられます（第3巻の「コピーされる型」の話です）。ID のような小さな値には `Copy` を付けるのが普通です。
- 付けなかった能力は使えません。たとえば `+` で足す能力（`Add`）は付けていないので、`owner + note` のような意味のない計算は書けません。これも newtype の利点です。

##### 解説3: 関数の引数が「説明書」になる

```rust
fn find_note(owner: UserId, id: NoteId) -> String {
```

- 引数の型を見るだけで、「1つ目は持ち主のユーザー、2つ目はノート」と分かります。変数名に頼らなくても、**型が意味を運んでくれる**のです。
- 呼び出し側が順番を間違えると、次の「よくあるエラー」で見るとおりコンパイルエラーになります。

### もう一歩: 実務に近い形

ID の取り違えを防げたので、次は「**中身の正しさ**」を型で保証します。`NoteTitle`（1〜80文字のタイトル）、`Email`（メールアドレス）、`NoteId`（1以上の番号）を、`mod domain` という区画の中に作ります。区画の外からは中身を直接作れないようにするのがポイントです。

#### 例1-2: parse でしか作れない NoteTitle

> **▼ このコードがやること（先に日本語で）:** タイトルの文字列を受け取り、前後の空白を取り除いたうえで 1〜80 文字なら `NoteTitle` を作り、そうでなければ「空」か「長すぎ（何文字か）」という理由を返します。4種類の入力で、合格と不合格を確かめます。

<!--CODE:design_lab/src/bin/u01_title.rs-->

```powershell
cargo run --bin u01_title
```

**▼ 実行結果**

<!--OUT:u01_title-->

##### 解説4: `pub struct` だが中身は private

```rust
pub struct NoteTitle(String);
```

- `pub struct` なので、**型の名前**は `mod domain` の外からも使えます（`use domain::NoteTitle;` が書ける）。
- しかし中の `String` には `pub` が付いていません。そのため外からは `NoteTitle(String::from("..."))` のように**直接作れず**、`.0` で中身を書き換えることもできません。
- 外の世界が `NoteTitle` を手に入れる方法は `NoteTitle::parse` だけ。これが図4-2 の「門番」です。

##### 解説5: 失敗の理由を enum で返す

```rust
pub enum TitleError {
    Empty,
    TooLong { len: usize },
}
```

- `parse` の戻り値は `Result<NoteTitle, TitleError>` です。成功なら `Ok(タイトル)`、失敗なら `Err(理由)` が返ります。
- 理由を文字列ではなく enum にしておくと、呼び出し側が `match` で「空なら○○、長すぎなら△△」と**理由ごとに対応を変えられます**。`TooLong { len }` のように、役に立つ情報（実際の文字数）を一緒に運べるのも便利です。
- このエラー型は UNIT 05 で `thiserror` を使って、もっと読みやすいメッセージ付きに育てます。

##### 解説6: 「80文字」は `chars().count()` で数える

- `str::len()` は**バイト数**を返します。日本語の「あ」は UTF-8 で3バイトなので、`len()` で 80 を判定すると、日本語は約26文字で「長すぎ」になってしまいます。
- 人間が数える「文字数」に近いのは `chars().count()`（Unicode の文字単位で数える）です。RustNote の DB 側の制約（`CHECK` で 1〜80 文字）とも数え方をそろえておきます。

> **注意:** 絵文字の一部（肌の色つきの顔や国旗など）は、見た目は1文字でも `char` が複数個になります。厳密に「見た目の文字数」を数えるには `unicode-segmentation` クレートを使いますが、本教材の RustNote では `chars().count()` で十分とします。

#### 例1-3: NoteId と Email も「作れたら正しい」にする

> **▼ このコードがやること（先に日本語で）:** URL の `/notes/42` の `42` のような文字列から `NoteId` を作り、数字でないものや 0 以下を断ります。メールアドレスは前後の空白を除いて小文字にそろえ、`@` が1つで、`@` の後ろに `.` があるものだけを受け付けます。

<!--CODE:design_lab/src/bin/u01_domain.rs-->

```powershell
cargo run --bin u01_domain
```

**▼ 実行結果**

<!--OUT:u01_domain-->

##### 解説7: `FromStr` を実装すると `.parse()` が使える

```rust
impl FromStr for NoteId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: i64 = s.parse().map_err(|_| IdError::NotANumber)?;
        NoteId::new(n)
    }
}
```

- `FromStr` は「文字列から作れる型」を表す標準ライブラリの trait（UNIT 02 で詳しく学ぶ「約束ごと」）です。実装すると `"42".parse::<NoteId>()` と書けるようになります。
- `type Err = IdError;` は「失敗したときのエラー型はこれです」という宣言です。
- 中では、まず `i64` として読めるかを試し（読めなければ `NotANumber`）、読めたら `NoteId::new` に渡して 0 以下を断ります。**検査のルールは `new` の1か所だけ**に書き、`from_str` はそれを呼ぶだけにしています。ルールが2か所に分かれると、片方だけ直し忘れるからです。

##### 解説8: メールアドレスは「正規化してから」しまう

```rust
let normalized = input.trim().to_lowercase();
```

- `trim()` で前後の空白を落とし、`to_lowercase()` で小文字にそろえます。これを**正規化**（せいきか：同じ意味のものを同じ形にそろえること）と呼びます。
- `Alice@Example.COM` と `alice@example.com` を別人として登録できてしまうと、DB の `email UNIQUE`（重複禁止）が役に立ちません。だから**しまう前に形をそろえる**のです。
- ここでの検査は「最低限」です。メールアドレスの正式な規則はとても複雑で、完全に正しく判定するのは現実的ではありません。本当に届くかどうかは、確認メールを送って確かめるのが実務の定石です。

> **つまずきポイント:** `split_once('@')` は「最初の `@` で2つに分ける」メソッドです。`let Some((local, domain)) = ... else { return ...; };` は **let-else** という書き方で、「`Some` なら中身を取り出して先へ進む。`None` ならすぐ `else` の中で関数を抜ける」という意味です。`match` を書くより入れ子が浅くなります。

### よくあるエラーと読み方

#### エラー1: E0308 引数の型が違う（ID の取り違え）

例1-1 の `find_note(owner, note)` を、わざと `find_note(note, owner)` に書き換えてビルドします（`src/bin/u01_err_swap.rs`）。

<!--OUT:u01_err_swap-->

- **何が起きたか:** `E0308` は「型が合わない（mismatched types）」系のエラーです。1つ目の引数は `UserId` を期待しているのに `NoteId` が来た（`expected UserId, found NoteId`）、と引数ごとに教えてくれています。
- **直し方:** `help: swap these arguments`（引数を入れ替えて）の提案どおり、`find_note(owner, note)` にします。この提案は正しいので、そのまま従って大丈夫です。
- これこそ newtype の狙いです。`i64` のままだったら、このバグは実行して、しかも運悪く他人のノートが見えてしまうまで気づけませんでした。

#### エラー2: E0603 private なコンストラクタは使えない

`parse` を通さずに、空のタイトルを直接作ろうとしたコードです（`src/bin/u01_err_private.rs`）。

<!--CODE:errs/u01_err_private.rs-->

<!--OUT:u01_err_private-->

- **何が起きたか:** `E0603` は「private（非公開）なものを外から使おうとした」エラーです。`a constructor is private if any of the fields is private`（フィールドが1つでも private なら、`型名(...)` で作る書き方も private になる）と説明されています。
- **直し方:** `domain::NoteTitle::parse("...")` を使って作ります。
- **help の提案に従ってはいけない場面:** コンパイラは `pub String` にすれば通ると提案しています。たしかにエラーは消えますが、そうすると誰でも空のタイトルを作れるようになり、門番の意味がなくなります。**このエラーは「設計どおりに守られている」証拠**です。help は「コンパイルを通す方法」であって、「設計として正しい方法」とは限らない、と覚えておきましょう。

### RustNote ではここで使う

RustNote の `domain` には、この単元で作った型がそのまま入ります。

| 型 | 中身 | 作り方 | 守っていること |
|---|---|---|---|
| `UserId` | `i64` | DB から読んだ値 / ログイン中のセッション | ノートの持ち主との取り違え |
| `NoteId` | `i64` | URL の `{id}` を `parse` / DB から読んだ値 | 0 以下・数字でない ID、`UserId` との取り違え |
| `Email` | `String`（小文字化済み） | `Email::parse` | 形式違い・大文字小文字の違いによる二重登録 |
| `NoteTitle` | `String`（trim 済み・1〜`MAX_TITLE_CHARS`（80）文字） | `NoteTitle::parse` | 空タイトル・長すぎるタイトル |

たとえば `GET /notes/{id}` では、URL の `{id}` を受け取ったらすぐ `NoteId` に変換し、失敗したら 404 を返します。その先の処理は `NoteId` しか受け取らないので、「数字でない ID で DB を検索してしまう」心配がありません。ID の `i64` は、Turso / libSQL（SQLite 系）の `INTEGER` 型の列とそのまま対応します。

### 練習問題

**問題1:** ページ送りに使う「ページ番号」型 `PageNumber` を作ってください。中身は `u32` で、**0 は作れない**（1 から始まる）ようにします。`fn new(n: u32) -> Option<PageNumber>` と、1ページあたりの件数を受け取って「何件飛ばすか」を返す `fn offset(self, per_page: u32) -> u32` を書き、`new(0)` と、3ページ目・20件ずつの offset を表示してください。

**問題2:** `"日本語"` の `len()` と `chars().count()` をそれぞれ表示し、なぜ `NoteTitle` の文字数判定に `len()` を使ってはいけないのかを説明してください。

**問題3:** ある人が「テストで便利だから」と、`NoteTitle` に `pub fn new_unchecked(s: String) -> NoteTitle { NoteTitle(s) }`（検査なしで作れる関数）を追加しようとしています。何が問題でしょうか。どうすれば目的（テストで手軽に作る）を果たせるでしょうか。

### 解答と解説

**解答1:**

<!--CODE:design_lab/src/bin/u01_ex1.rs-->

**▼ 実行結果**

<!--OUT:u01_ex1-->

- `PageNumber` は 0 では作れないので、`offset` の中の `self.0 - 1` が**マイナスになる（u32 では桁あふれでパニックになる）ことが絶対にありません**。もし `u32` のまま受け取っていたら、`offset` の中でも 0 の検査が必要でした。検査を `new` に寄せたおかげで、使う側の関数が単純になっています。

**解答2:**

<!--CODE:design_lab/src/bin/u01_ex2.rs-->

**▼ 実行結果**

<!--OUT:u01_ex2-->

- 日本語の1文字は UTF-8 で3バイトなので、`len()` は 9 になります。`len()` で「80以下」を判定すると、日本語のタイトルは 26 文字までしか入らなくなり、利用者から見ると「80文字まで」の約束が守られません。

**解答3:**

- 検査なしで作れる `pub` な関数があると、テスト以外のコードからも呼べてしまい、「`NoteTitle` があれば必ず 1〜80文字」という約束が崩れます。門の横に裏口を作るようなものです。
- テストで手軽に作りたいなら、テストの中でも `NoteTitle::parse("買い物").unwrap()` と書けば十分です。テストでは失敗したらパニックで止まってよいので、`unwrap()` が使えます。どうしても専用の関数が欲しい場合は、`#[cfg(test)]`（テストのときだけコンパイルする印。UNIT 10 で学びます）を付けて、本番のプログラムには含まれないようにします。

### まとめ

- 意味の違う値は newtype で別の型にすると、取り違えが `E0308` のコンパイルエラーになります。
- 中身を private にして `parse` だけを入口にすると、「作れた＝正しい」が型で保証されます。
- コンパイラの help は「通す方法」です。設計を壊す提案（`pub` を付ける等）は採用しません。

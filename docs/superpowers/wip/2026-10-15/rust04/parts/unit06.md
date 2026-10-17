## UNIT 06: crate・module・可視性・workspace

### この単元のゴール

- package・crate・module・workspace の違いを、自分の言葉で説明できる
- `mod` でコードを区画に分け、ファイル（`src/domain.rs` と `src/domain/note.rs`）に分割できる
- `pub`・`pub(crate)`・`pub(super)`・何も付けない（private）の届く範囲を使い分け、`pub use` で入口を整理できる
- 複数の package を1つの workspace にまとめ、`Cargo.lock` と `target` を共有できる

### たとえ話でつかむ

会社のビルを思い浮かべてください。

- **crate** は1つの**ビル**です。ビルごとに建てられ（コンパイルされ）、外の人はビルの正面玄関から入ります。
- **module** はビルの中の**フロアや部署**です。経理部・人事部のように仕事ごとに区切り、部屋の中にさらに小部屋（子の module）を作れます。
- **`pub`** は「受付の案内板に載せる」ことです。載せたものは社外の人も使えます。`pub(crate)` は「社員証があれば入れる（ビルの中の人だけ）」、`pub(super)` は「1つ上の部署の人まで」、何も付けなければ「その部署の中だけ（社外秘）」です。
- **package** は「ビルの設計書（`Cargo.toml`）1冊と、それで建てるビル」の組です。1冊の設計書で、本館（ライブラリ）と別館（実行ファイル）を建てることもできます。
- **workspace** は、同じ敷地に建つ**複数のビル**です。倉庫（`target` フォルダ）と仕入れ台帳（`Cargo.lock`）を共有します。

案内板に何でも載せると、社外の人が奥の部屋の書類まで頼りにするようになり、模様替え（コードの変更）ができなくなります。**`pub` は必要なものだけに付ける**のが、変更に強い設計のコツです。

> **用語:** package（パッケージ：`Cargo.toml` 1つで管理される単位。0〜1個のライブラリ crate と、0個以上の実行ファイル crate を持てる）

<!-- -->

> **用語:** module（モジュール：crate の中の名前の区画。`mod 名前` で作り、中の項目に公開範囲を付けられる）

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="700" height="280" rx="14" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="30" y="36" font-size="14" font-weight="700" fill="#475569">crate の外（main.rs・ほかの crate）</text>
<text x="30" y="58" font-size="12" fill="#475569">届くのは pub だけ</text>
<rect x="30" y="72" width="660" height="206" rx="12" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="50" y="96" font-size="14" font-weight="700" fill="#1e40af">crate notes_app（lib.rs）</text>
<text x="50" y="116" font-size="12" fill="#1e40af">pub(crate) まで届く</text>
<rect x="50" y="128" width="400" height="140" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="68" y="150" font-size="13" font-weight="700" fill="#047857">mod domain::note</text>
<text x="68" y="170" font-size="12" fill="#047857">private（何も付けない）は、ここの中だけ</text>
<rect x="68" y="182" width="364" height="76" rx="8" fill="#ffffff" stroke="#10b981"/>
<text x="84" y="204" font-size="12" fill="#0f172a">pub struct Note {</text>
<text x="104" y="222" font-size="12" fill="#1e40af">pub title … どこからでも</text>
<text x="104" y="240" font-size="12" fill="#c2410c">pub(crate) owner_id … crate の中から</text>
<rect x="470" y="128" width="200" height="140" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="570" y="152" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">mod repo（private）</text>
<text x="570" y="178" font-size="12" fill="#c2410c" text-anchor="middle">module ごと非公開</text>
<text x="570" y="202" font-size="12" fill="#c2410c" text-anchor="middle">→ 中の NoteStore は</text>
<text x="570" y="222" font-size="12" fill="#c2410c" text-anchor="middle">pub use で入口に</text>
<text x="570" y="242" font-size="12" fill="#c2410c" text-anchor="middle">出したときだけ見える</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-12: pub の届く範囲。内側に書いた印ほど、外まで届くかどうかが厳しく決まる</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-13" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="10" width="700" height="250" rx="14" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="30" y="38" font-size="14" font-weight="700" fill="#475569">ws_lab/（workspace のいちばん上）</text>
<rect x="30" y="54" width="200" height="58" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="130" y="78" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">Cargo.toml</text>
<text x="130" y="98" font-size="12" fill="#1e40af" text-anchor="middle">[workspace] members</text>
<rect x="260" y="54" width="200" height="58" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="78" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">Cargo.lock（1つ）</text>
<text x="360" y="98" font-size="12" fill="#c2410c" text-anchor="middle">全員の依存の版を記録</text>
<rect x="490" y="54" width="200" height="58" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="590" y="78" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">target/（1つ）</text>
<text x="590" y="98" font-size="12" fill="#c2410c" text-anchor="middle">ビルド結果を共有</text>
<rect x="60" y="150" width="240" height="90" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="180" y="176" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">crates/app（実行ファイル）</text>
<text x="180" y="198" font-size="12" fill="#047857" text-anchor="middle">Cargo.toml / src/main.rs</text>
<text x="180" y="220" font-size="12" fill="#047857" text-anchor="middle">cargo run -p app</text>
<rect x="420" y="150" width="240" height="90" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="540" y="176" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">crates/domain（ライブラリ）</text>
<text x="540" y="198" font-size="12" fill="#047857" text-anchor="middle">Cargo.toml / src/lib.rs</text>
<text x="540" y="220" font-size="12" fill="#047857" text-anchor="middle">thiserror を使う</text>
<line x1="300" y1="195" x2="416" y2="195" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-13)"/>
<text x="358" y="185" font-size="12" fill="#475569" text-anchor="middle">path で依存</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-13: workspace の構成。複数の package が Cargo.lock と target を共有する</div>
</div>

### まず動かす最小コード

まずは1つのファイルの中で `mod { ... }` を使い、公開範囲の印の違いを確かめます。

#### 例6-1: 1つのファイルの中で module と公開範囲を試す

> **▼ このコードがやること（先に日本語で）:** `app` という区画の中に、ノートの型を置く `domain` と、ノートを操作する `service` の2つの小区画を作ります。項目ごとに「どこまで見せるか」の印を変え、`main` からどれが使えるかを確かめます。

<!--CODE:design_lab/src/bin/u06_mod.rs-->

```powershell
cargo run --bin u06_mod
```

**▼ 実行結果**

<!--OUT:u06_mod-->

#### ▼ コードを1つずつ分解して解説

##### 解説1: `mod` は名前の区画、`pub mod` で外にも見せる

```rust
mod app {
    pub mod domain {
```

- `mod app { ... }` で `app` という区画を作ります。区画の中の項目は、`app::domain::Note` のように `::` でつないだ道のり（パス）で呼びます。
- `app` 自身には `pub` が付いていませんが、`main` は同じ crate のいちばん上（`app` の親）にいるので、`app` の中を覗けます。**private は「その区画と、その子孫からだけ見える」**というルールだからです。
- `pub mod domain` の `pub` は、「`app` の外からも `domain` という区画を覗いてよい」という意味です。

##### 解説2: フィールドごとに公開範囲を変える

```rust
pub struct Note {
    // どこからでも読み書きできる
    pub title: String,
    // この crate の中からだけ読み書きできる
    pub(crate) owner_id: i64,
    // この module（domain）の中からだけ
    version: i64,
}
```

- `pub` … どこからでも（ほかの crate からも）。
- `pub(crate)` … 同じ crate の中ならどこからでも。ほかの crate からは見えません。
- 何も付けない … その module（`domain`）とその子孫の中だけ。`version` は `domain` の外からは直接読めないので、`version()` という読むだけのメソッドを用意しています。版の番号を勝手に書き換えられると困るので、増やす方法は `bump()` だけにしてあります。UNIT 01 の「門番」と同じ考え方です。

##### 解説3: `super::` と `crate::`

```rust
use super::domain::Note;
```

```rust
use crate::app::domain::Note;
```

- `super` は「1つ上の区画」です。`service` から見た `super` は `app` なので、`super::domain::Note` は `app::domain::Note` と同じものです。
- `crate` は「この crate のいちばん上」です。どこから書いても同じ場所を指すので、深い区画の中から遠くの項目を指すときに便利です。
- `use` は「この名前で呼べるように、近くに引き寄せる」命令です。`use` しておくと、`Note::new(...)` のように短く書けます。

##### 解説4: `pub(super)` は「親の区画まで」

```rust
pub(super) fn audit(note: &Note) -> String {
```

- `audit` は `service` の親である `app` の中からは呼べます（`app::run` の中で使っています）。しかし `app` の外、つまり `main` からは呼べません（練習問題1）。
- 「この補助関数は、この機能のまとまりの中だけで使う」と宣言しておくと、ほかの場所から頼られることがなく、後で自由に作り変えられます。

### もう一歩: 実務に近い形

1つのファイルに全部を書くと、すぐに何千行にもなります。実際のプロジェクトでは、module をファイルに分け、**ライブラリ crate（`src/lib.rs`）と実行ファイル crate（`src/main.rs`）を1つの package に同居**させる形がよく使われます。

#### 例6-2: lib.rs と main.rs、module をファイルに分ける

> **▼ このコードがやること（先に日本語で）:** ノートの型と保存先を「ライブラリ」として別ファイルに分けて書き、`main.rs` からはライブラリとして使います。ライブラリの入口（`lib.rs`）では、外に見せる型だけを並べ直して公開します。

新しい package を作ります。

```powershell
cargo new notes_app
cd notes_app
```

<!--OUT:u06_new-->

ファイルを次のように並べます（`src/domain` フォルダは自分で作ります）。

```text
notes_app/
├── Cargo.toml
└── src/
    ├── main.rs        … 実行ファイル crate の入口
    ├── lib.rs         … ライブラリ crate の入口
    ├── domain.rs      … mod domain の中身（子の module を宣言）
    ├── domain/
    │   ├── note.rs    … mod domain::note
    │   └── user.rs    … mod domain::user
    └── repo.rs        … mod repo
```

**src/lib.rs**

<!--CODE:notes_app/src/lib.rs-->

**src/domain.rs**

<!--CODE:notes_app/src/domain.rs-->

**src/domain/note.rs**

<!--CODE:notes_app/src/domain/note.rs-->

**src/domain/user.rs**

<!--CODE:notes_app/src/domain/user.rs-->

**src/repo.rs**

<!--CODE:notes_app/src/repo.rs-->

**src/main.rs**

<!--CODE:notes_app/src/main.rs-->

```powershell
cargo run
```

**▼ 実行結果**

<!--OUT:u06_notes_app-->

##### 解説5: `mod domain;` は「ファイルを読み込め」という意味

```rust
pub mod domain;
mod repo;
```

- `{ ... }` を書かずに `mod 名前;` と書くと、コンパイラは `src/名前.rs`（または `src/名前/mod.rs`）を探して、その中身を module として読み込みます。
- `domain.rs` の中の `pub mod note;` は、`src/domain/note.rs` を読み込みます。**フォルダの名前が親の module の名前**になります。
- `src/domain/mod.rs` という書き方は古い形（edition 2015 のころ）で、今も使えますが、`mod.rs` という同じ名前のファイルがエディタのタブに並んで紛らわしいので、本書では `domain.rs` ＋ `domain/` フォルダの形を使います。
- ファイルを作っただけでは、どこからも読み込まれません。**必ず親の側に `mod 名前;` を書く**必要があります（逆に、`mod` を書いてファイルを作り忘れると E0583 になります）。

##### 解説6: main.rs は「別の crate」

```rust
use notes_app::domain::user::count_owned;
use notes_app::{NoteStore, NoteTitle};
```

- 同じ package の中にあっても、`main.rs` から見た `lib.rs` は**別の crate**です。だから `crate::` ではなく、package の名前 `notes_app::` で呼びます。
- 別の crate なので、`pub(crate)` の項目は見えません。`Note` の `owner_id` は `pub(crate)` なので、`main.rs` からは `owner_id()` メソッドを通して読んでいます。直接 `note.owner_id` と書くとエラーになります（よくあるエラー3）。
- ロジックを `lib.rs` 側に置いておくと、テスト（UNIT 10）やほかの実行ファイルからも同じコードを使えます。**`main.rs` は薄く、中身はライブラリへ**が定石です。

##### 解説7: `pub use` で入口を整える（再公開）

```rust
pub use domain::note::{Note, NoteTitle};
pub use repo::NoteStore;
```

- `repo` module は private なので、`notes_app::repo::NoteStore` という道のりでは外から使えません。しかし `pub use repo::NoteStore;` と書いたので、`notes_app::NoteStore` という**短い入口**からは使えます。
- こうすると、ライブラリの中のフォルダ構成（`repo.rs` を `storage/memory.rs` に移す、など）を変えても、使う側の `use notes_app::NoteStore;` は変わりません。**中の配置と外への見せ方を切り離せる**のが `pub use` の価値です。

#### 例6-3: workspace で複数の package をまとめる

アプリが大きくなると、「ドメインの型だけのライブラリ」「Web サーバー」「DB に初期データを入れるツール」のように、package を分けたくなることがあります。複数の package を1か所で管理する仕組みが **workspace** です。

> **▼ このコードがやること（先に日本語で）:** `ws_lab` というフォルダを workspace にして、その中にライブラリ `domain` と実行ファイル `app` の2つの package を作ります。`app` から `domain` の `NoteTitle` を使い、依存クレートの版は workspace のいちばん上で1回だけ決めます。

まず、`ws_lab` フォルダを作り、その中に次の `Cargo.toml` を置きます（`members` は空のまま）。

```toml
[workspace]
resolver = "3"
members = []
```

続けて、`ws_lab` の中で2つの package を作ります。

```powershell
cargo new --lib crates/domain
cargo new crates/app
```

<!--OUT:u06_ws_new-->

`Adding ... as member of workspace` とあるとおり、cargo が `members` に自動で書き足してくれます。そのうえで、3つの `Cargo.toml` を次のように整えます。

**ws_lab/Cargo.toml**

<!--FILE:toml:ws_lab/Cargo.toml-->

**ws_lab/crates/domain/Cargo.toml**

<!--FILE:toml:ws_lab/crates/domain/Cargo.toml-->

**ws_lab/crates/app/Cargo.toml**

<!--FILE:toml:ws_lab/crates/app/Cargo.toml-->

**ws_lab/crates/domain/src/lib.rs**

<!--CODE:ws_lab/crates/domain/src/lib.rs-->

**ws_lab/crates/app/src/main.rs**

<!--CODE:ws_lab/crates/app/src/main.rs-->

`ws_lab` のいちばん上で、`-p`（package の指定）を付けて実行します。

```powershell
cargo run -p app
```

**▼ 実行結果**

<!--OUT:u06_ws_run-->

##### 解説8: workspace の Cargo.toml

```toml
[workspace]
resolver = "3"
members = ["crates/app", "crates/domain"]
```

- `[workspace]` があるフォルダが workspace のいちばん上です。`members` に、まとめる package のフォルダを並べます。
- `resolver = "3"` は「依存の版の決め方」の世代です。edition 2024 の既定は 3 で、`rust-version`（UNIT 07）を見て、その Rust で動く版を選んでくれます。実行結果の `Locking 6 packages to highest Rust 1.98 compatible versions`（Rust 1.98 で使える最新の版で固定した）がその働きです。workspace のいちばん上には edition がないので、明示的に書いておきます。
- `Cargo.lock` と `target` フォルダは、workspace のいちばん上に**1つだけ**できます（図4-13）。どの package からビルドしても同じ版の依存が使われ、一度ビルドした依存は使い回されます。

##### 解説9: 共有の設定を受け継ぐ `workspace = true`

```toml
[workspace.dependencies]
thiserror = "2.0.21"
```

```toml
[dependencies]
thiserror = { workspace = true }
```

- `[workspace.dependencies]` で版を1回だけ決め、各 package では `{ workspace = true }` と書いて受け継ぎます。package ごとに版を書くと、「`domain` は 2.0.21、`app` は 2.0.18」のようにずれていく原因になります。
- `edition.workspace = true` も同じで、`[workspace.package]` の値を受け継ぎます。
- 同じ workspace の package への依存は、`domain = { path = "../domain" }` のように**フォルダの場所**で書きます。crates.io（公開のクレート置き場）から取ってくるのではなく、手元のコードを使う、という意味です。

### よくあるエラーと読み方

#### エラー1: E0603 / E0616 private な関数・フィールドに触った

<!--CODE:errs/u06_err_private.rs-->

<!--OUT:u06_err_private-->

- **E0603**（`function next_version is private`）は、private な**項目**（関数・型・module など）を、その外から使おうとしたエラーです。
- **E0616**（`field version of struct Note is private`）は、private な**フィールド**を外から読もうとしたエラーです。
- **直し方:** まず「本当に外から触る必要があるのか」を考えます。読むだけなら `pub fn version(&self) -> i64` のような読み取り用のメソッドを用意し、フィールド自体は private のままにします。安易に `pub` を付けると、UNIT 01 の門番が壊れることがあります。

#### エラー2: E0603 private な module を通る道のり

`main.rs` で `use notes_app::repo::NoteStore;` と書いた場合です。

<!--CODE:errs/u06_err_crate_main.rs-->

<!--OUT:u06_err_crate-->

- **何が起きたか:** `module repo is private`。`NoteStore` 自体は `pub` でも、そこへ至る**途中の module** `repo` が private なので、この道のりは通れません。道のりの全部の区画に `pub` が必要です。
- **直し方:** `lib.rs` で `pub use repo::NoteStore;` と再公開してあるので、`use notes_app::NoteStore;` と書けば使えます。ライブラリの作者が用意した「正面玄関」を使う、ということです。

#### エラー3: E0616 `pub(crate)` は別の crate から見えない

<!--CODE:errs/u06_err_field_main.rs-->

<!--OUT:u06_err_field-->

- **何が起きたか:** `owner_id` は `pub(crate)` なので、ライブラリ crate の中（`repo.rs` や `user.rs`）からは読めますが、別の crate である `main.rs` からは private と同じ扱いです。
- **直し方:** help のとおり、用意されているメソッド `note.owner_id()` を使います。

#### エラー4: E0583 module のファイルが見つからない

`lib.rs` に `mod session;` と書いたのに、`src/session.rs` を作っていない場合です。

<!--OUT:u06_err_e0583-->

- **何が起きたか:** `file not found for module session`。help が、作るべきファイルの名前（`src\session.rs` または `src\session\mod.rs`）を教えてくれています。
- **直し方:** そのファイルを作るか、まだ要らないなら `mod session;` の行を消します。ファイル名のつづり違い（`sesion.rs` など）でも同じエラーになります。

### RustNote ではここで使う

- RustNote 程度の大きさなら、**1つの package の中を module で分ける**構成で十分です。`lib.rs` に `domain`（`Email`・`NoteTitle`・`UserId` などの型）、保存先、エラー型、設定などの module を置き、`main.rs` は起動するだけにします。具体的なフォルダ構成は第9巻で決めます。
- `NoteTitle` の中身や `Note` の `version` のように「勝手に変えられると困る」ものは private のままにし、読み取り用のメソッドだけを `pub` にします。
- workspace は、「管理用のコマンドラインツールを別に作りたい」「ドメインの型をほかのプロジェクトでも使いたい」など、package を分ける理由ができたときの選択肢です。最初から分けすぎると、かえって見通しが悪くなります。

### 練習問題

**問題1:** 例6-1 の `main` の最後に `println!("{}", app::service::audit(&note));` を足すと、どうなるでしょうか。予想してから実際に試し、理由を説明してください。

**問題2:** 例6-2 で、`main.rs` は `use notes_app::domain::user::count_owned;` と長い道のりを書いています。`lib.rs` を1行変えて、`use notes_app::{NoteStore, NoteTitle, count_owned};` と書けるようにしてください。

**問題3:** 例6-3 の workspace に、3つ目の package `crates/seed`（実行ファイル）を追加し、`domain` の `NoteTitle` を使って「正しい形のタイトルが何件あるか」を表示してください。追加の手順と `Cargo.toml` を示してください。

### 解答と解説

**解答1:** コンパイルエラーになります。

<!--OUT:u06_ex1_err-->

- `audit` は `pub(super)`、つまり「`service` の親である `app` の中まで」しか見えません。`main` は `app` の外（crate のいちばん上）にいるので、`private function` 扱いになります。`app::run` の中から呼べたのは、そこが `app` の中だからです。

**解答2:** `lib.rs` に `pub use domain::user::count_owned;` を足します。

<!--CODE:ex06/notes_app/src/lib.rs-->

`main.rs` の `use` は次の1行にまとめられます。

```rust
use notes_app::{NoteStore, NoteTitle, count_owned};
```

**▼ 実行結果**

<!--OUT:u06_ex2-->

- `pub use` は「別の場所にあるものを、ここにもあるように見せる」だけなので、`count_owned` の中身や置き場所は変わっていません。`{...}` の中の並びは、`cargo fmt` が大文字で始まる名前（型）を先に並べます。

**解答3:** `ws_lab` の中で `cargo new crates/seed` を実行すると、`members` に自動で追加されます。

<!--OUT:u06_ex3_new-->

`crates/seed/Cargo.toml` に `domain` への依存を書き、`src/main.rs` を書きます。

<!--FILE:toml:ex06/ws_lab/crates/seed/Cargo.toml-->

<!--CODE:ex06/ws_lab/crates/seed/src/main.rs-->

```powershell
cargo run -p seed
```

**▼ 実行結果**

<!--OUT:u06_ex3_run-->

- `seed` と `app` は同じ `target` フォルダを使うので、`target\debug\` に `app.exe` と `seed.exe` が並びます。依存の版も同じ `Cargo.lock` で決まります（なお、この実行結果の `Compiling` の行は、練習用に workspace を別の場所へコピーして最初からビルドしたときのものです）。

### まとめ

- crate はコンパイルの単位、module は crate の中の区画、package は `Cargo.toml` の単位、workspace は複数の package のまとまりです。
- 公開範囲は `pub` ＞ `pub(crate)` ＞ `pub(super)` ＞ private。必要なものだけを公開し、`pub use` で入口を整えます。
- `mod 名前;` はファイルを読み込む宣言です。`main.rs` は `lib.rs` を使う別の crate なので、ロジックはライブラリ側に置きます。

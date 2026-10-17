## UNIT 07: Cargo features・依存固定（Cargo.lock・バージョン指定）・最小構成

### この単元のゴール

- `Cargo.toml` の版の書き方（`"1.0.229"`・`"=0.10.0"`・`"~1.2.3"`・`"0.10"`）が、どの範囲の版を許すのかを読める
- `Cargo.lock` が何を記録し、なぜアプリでは保存（コミット）しておくのか、`--locked` が何を守るのかを説明できる
- 自分の crate に features（機能の ON/OFF）を作り、`#[cfg(feature = "...")]` と `--features` / `--no-default-features` で切り替えられる
- 依存クレートの features を必要な分だけ選び、`rust-version` で必要な Rust の版を宣言できる

### たとえ話でつかむ

車を買うときの注文書を思い浮かべてください。

- **版の指定（`Cargo.toml`）** は「**○○年式以降のモデルなら可。ただしフルモデルチェンジ前まで**」のような**条件**です。条件を満たす中で、いちばん新しいものが選ばれます。
- **`Cargo.lock`** は、実際に納車された車の**車台番号の控え**です。「条件」ではなく「実際にこれを受け取った」という記録なので、同じ控えを見せれば、別の店（別のパソコンやサーバー）でもまったく同じ車をそろえられます。
- **features** は**オプション装備**です。カーナビ・サンルーフのように、付ければ便利ですが、値段（ビルド時間）と重さ（実行ファイルの大きさ）が増え、故障する部品（攻撃される可能性のある場所）も増えます。**必要なものだけを選ぶ**のが賢い買い方です。

> **用語:** 依存クレート（dependency：自分のプロジェクトが使う、ほかの人が作った crate。`cargo add` で `Cargo.toml` の `[dependencies]` に書き込まれる）

<!-- -->

> **用語:** SemVer（セムバー：Semantic Versioning。版を `大.中.小`（major.minor.patch）の3つの数で表し、「大」が上がったら互換性のない変更、「中」は機能追加、「小」はバグ修正、という約束）

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-14" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="30" width="200" height="110" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="110" y="54" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">Cargo.toml（条件）</text>
<text x="110" y="80" font-size="12" fill="#0f172a" text-anchor="middle">serde = "1.0.229"</text>
<text x="110" y="104" font-size="12" fill="#1e40af" text-anchor="middle">1.0.229 以上</text>
<text x="110" y="124" font-size="12" fill="#1e40af" text-anchor="middle">2.0.0 未満なら可</text>
<line x1="210" y1="85" x2="266" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-14)"/>
<rect x="270" y="30" width="180" height="110" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="360" y="54" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">cargo が選ぶ</text>
<text x="360" y="80" font-size="12" fill="#475569" text-anchor="middle">条件を満たす中で</text>
<text x="360" y="100" font-size="12" fill="#475569" text-anchor="middle">いちばん新しい版</text>
<text x="360" y="120" font-size="12" fill="#475569" text-anchor="middle">（rust-version も考慮）</text>
<line x1="450" y1="85" x2="506" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-14)"/>
<rect x="510" y="30" width="200" height="110" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="610" y="54" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">Cargo.lock（記録）</text>
<text x="610" y="80" font-size="12" fill="#0f172a" text-anchor="middle">serde 1.0.229</text>
<text x="610" y="104" font-size="12" fill="#c2410c" text-anchor="middle">実際に使う版を固定</text>
<text x="610" y="124" font-size="12" fill="#c2410c" text-anchor="middle">＋チェックサム</text>
<rect x="110" y="172" width="500" height="60" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="360" y="196" font-size="13" fill="#047857" text-anchor="middle">Cargo.lock があれば、別のパソコン・CI・Docker でも同じ版がそろう</text>
<text x="360" y="218" font-size="12" fill="#047857" text-anchor="middle">cargo build --locked … Cargo.lock を書き換える必要があればエラーで止める</text>
<line x1="610" y1="140" x2="560" y2="168" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-14)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-14: 版の「条件」から、実際に使う版の「記録」へ</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-15" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="10" width="340" height="240" rx="10" fill="#f8fafc" stroke="#64748b"/>
<text x="180" y="36" font-size="14" font-weight="700" fill="#475569" text-anchor="middle">cargo run（json は OFF）</text>
<rect x="100" y="56" width="160" height="44" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="180" y="83" font-size="13" fill="#1e40af" text-anchor="middle">feat_lab</text>
<text x="180" y="136" font-size="12" fill="#475569" text-anchor="middle">依存クレート: なし</text>
<text x="180" y="162" font-size="12" fill="#475569" text-anchor="middle">export_json はコンパイルされない</text>
<text x="180" y="196" font-size="12" fill="#047857" text-anchor="middle">○ ビルドが速い・ファイルが小さい</text>
<text x="180" y="220" font-size="12" fill="#047857" text-anchor="middle">○ 使わないコードが入らない</text>
<rect x="370" y="10" width="340" height="240" rx="10" fill="#fff7ed" stroke="#f97316"/>
<text x="540" y="36" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">cargo run --features json</text>
<rect x="460" y="56" width="160" height="44" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="540" y="83" font-size="13" fill="#1e40af" text-anchor="middle">feat_lab</text>
<line x1="500" y1="100" x2="460" y2="136" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-15)"/>
<line x1="580" y1="100" x2="620" y2="136" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-15)"/>
<rect x="390" y="140" width="130" height="40" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="455" y="165" font-size="12" fill="#c2410c" text-anchor="middle">serde（+derive）</text>
<rect x="560" y="140" width="130" height="40" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="625" y="165" font-size="12" fill="#c2410c" text-anchor="middle">serde_json</text>
<text x="540" y="210" font-size="12" fill="#c2410c" text-anchor="middle">その先の依存も合わせて 11 個が加わる</text>
<text x="540" y="232" font-size="12" fill="#c2410c" text-anchor="middle">export_json が使えるようになる</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-15: features の ON/OFF で、コンパイルされるコードと依存クレートが変わる</div>
</div>

### まず動かす最小コード

#### 版の書き方を読む

「はじめに」で `cargo add serde --features derive` を実行すると、`Cargo.toml` に `serde = { version = "1.0.229", features = ["derive"] }` と書き込まれました。この `"1.0.229"` は「ちょうど 1.0.229」という意味**ではありません**。

| 書き方 | 意味（許される版の範囲） | 使いどころ |
|---|---|---|
| `"1.0.229"`（`"^1.0.229"` と同じ） | 1.0.229 以上、2.0.0 未満 | ふつうはこれ。互換性のある更新は受け入れる |
| `"0.10"`（`"^0.10"` と同じ） | 0.10.0 以上、0.11.0 未満 | 0.x の crate は「中」の数が上がると互換性が壊れる約束 |
| `"=0.10.0"` | 0.10.0 ちょうど | 変化の激しい crate を固定したいとき（本書では Topcoat を `=0.10.0` に固定） |
| `"~1.2.3"` | 1.2.3 以上、1.3.0 未満 | バグ修正（「小」）だけ受け入れたいとき |

- SemVer の約束では、「大」の数が同じなら互換性があるはずです。だから cargo は、`"1.0.229"` と書かれたら「1.x.x の中でいちばん新しい版」を選びます。
- ただし **0.x の版**は「まだ開発途中」という扱いで、`0.10` → `0.11` のように「中」が上がると壊れる変更が入ってよい約束です。Topcoat のように開発が活発な 0.x の crate は、`=` で版を固定しておくと、教材や手元のコードと食い違いません。

#### 例7-1: 自分の crate に features を作る

> **▼ このコードがやること（先に日本語で）:** ノートの書き出しを「テキスト（いつも使える）」「Markdown（既定で ON）」「JSON（指定したときだけ ON）」の3つの機能に分け、コマンドの指定によって、どの書き出しがプログラムに含まれるかを切り替えます。JSON を ON にしたときだけ、JSON 用の依存クレートが加わります。

```powershell
cargo new feat_lab
cd feat_lab
cargo add serde --features derive --optional
cargo add serde_json --optional
```

`--optional`（任意の依存）を付けると、「features で ON にしたときだけ使う依存」として追加されます。そのあと `Cargo.toml` を次のように整えます（`cargo add` が自動で作った `serde = ["dep:serde"]` などの行は、`json` の1つにまとめ直しています）。

<!--FILE:toml:feat_lab/Cargo.toml-->

**src/main.rs**

<!--CODE:feat_lab/src/main.rs-->

3通りに実行してみます。まずは何も指定しない場合です。

```powershell
cargo run
```

<!--OUT:u07_run_default-->

JSON の機能を ON にします。

```powershell
cargo run --features json
```

<!--OUT:u07_run_json-->

既定の機能（markdown）も OFF にします。

```powershell
cargo run --no-default-features
```

<!--OUT:u07_run_nodef-->

#### ▼ コードを1つずつ分解して解説

##### 解説1: `[features]` の書き方

```toml
[features]
# 何も指定しないときに ON になる機能
default = ["markdown"]
# 依存を増やさない、コードだけの機能
markdown = []
# ON にすると serde と serde_json が依存に加わる
json = ["dep:serde", "dep:serde_json"]
```

- `機能名 = [...]` で feature を定義します。`[...]` の中は「この機能を ON にしたら、一緒に ON になるもの」です。
- `markdown = []` は、ほかに何も ON にしない、コードの切り替えだけの機能です。
- `json = ["dep:serde", "dep:serde_json"]` の `dep:` は、「任意の依存（`optional = true`）の serde と serde_json を使う」という意味です。json を ON にしない限り、この2つはダウンロードもコンパイルもされません。
- `default` は特別な名前で、`cargo run` のように何も指定しないときに ON になる機能の一覧です。

##### 解説2: `#[cfg(feature = "...")]` でコードを出し入れする

```rust
#[cfg(feature = "json")]
fn export_json(note: &Note) -> String {
    serde_json::to_string(note).expect("Note は必ず JSON にできる")
}
```

- `#[cfg(...)]` を付けた項目は、条件が成り立つときだけ**コンパイルの対象になります**。json が OFF なら、`export_json` は最初から存在しないのと同じです。だから、OFF のときは serde_json が無くてもエラーになりません。
- 呼ぶ側の `println!` にも `#[cfg(feature = "json")]` を付けています。付け忘れると、OFF のときに「そんな関数は無い」というエラーになります（よくあるエラー1）。
- `#[cfg_attr(feature = "json", derive(serde::Serialize))]` は「json が ON のときだけ、`derive(serde::Serialize)` を付ける」という意味です。
- `cfg!(feature = "markdown")` は、ON なら `true`、OFF なら `false` になる式です。コードを消すのではなく、値として使いたいときに使います。

##### 解説3: `cargo tree` で依存の増え方を見る

`cargo tree` は、依存クレートを木の形で表示するコマンドです。features によって依存が変わる様子を確かめます。

```powershell
cargo tree
```

<!--OUT:u07_tree-->

```powershell
cargo tree --features json
```

<!--OUT:u07_tree_json-->

- json を OFF にしていると、依存は**ゼロ**です。ON にすると、serde・serde_json に加えて、それらが使う `syn` や `quote` など、合わせて 11 個のクレートが加わりました（図4-15）。`(*)` は「上ですでに表示したので省略」の印、`(proc-macro)` は derive などを実現するコンパイル時専用のクレートの印です。
- 依存が増えると、ビルドが遅くなり、実行ファイルが大きくなり、どれか1つに脆弱性（ぜいじゃくせい：セキュリティ上の弱点）が見つかったときの影響も受けます。**必要な機能だけを ON にする**ことは、速さだけでなく安全のためでもあります。

### もう一歩: 実務に近い形

#### Cargo.lock を読む

`cargo build` や `cargo add` をすると、`Cargo.toml` の隣に `Cargo.lock` ができます。中身の一部は次のようになっています（`feat_lab` の `Cargo.lock` からの抜粋）。

```text
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 4

[[package]]
name = "serde"
version = "1.0.229"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "4148590afebada386688f18773da617792bf2ef03ffc1e4cbd2b1d45b023e0ba"
dependencies = [
 "serde_core",
 "serde_derive",
]
```

- `Cargo.toml` が「条件」だったのに対し、`Cargo.lock` には**実際に選ばれた版**（`1.0.229`）と、ダウンロードしたファイルの**チェックサム**（中身が1バイトでも違えば変わる指紋のような値）が記録されています。
- 先頭にあるとおり、cargo が自動で作るファイルなので、**手で編集してはいけません**。
- **アプリ（実行ファイルを作る package）では、`Cargo.lock` を Git に保存します**。そうすれば、半年後に別のパソコンでビルドしても、Docker の中でビルドしても、まったく同じ版の依存が使われます。「自分のパソコンでは動くのに、サーバーでは動かない」を防ぐ基本です。

> **ヒント:** 依存を新しくしたいときは `cargo update` を実行します。`Cargo.toml` の条件の範囲内で最新の版を選び直し、`Cargo.lock` を書き換えます（`cargo update serde` のように名前を付けると、そのクレートだけを更新します）。条件の範囲を超える更新（serde 1.x → 2.x など）は、`Cargo.toml` を書き換えないと起きません。

#### `--locked` で「勝手に版を変えない」を保証する

本番用のビルド（CI や Docker）では、`cargo build --locked` のように `--locked` を付けます。`Cargo.lock` を書き換えないとビルドできない状態なら、**勝手に書き換えずにエラーで止まります**。

試しに、`Cargo.toml` の `[dependencies]` に `itoa = "1.0.18"` を手で1行足し、`cargo add` も `cargo build` もしないまま `--locked` でビルドしてみます。

```powershell
cargo build --locked
```

<!--OUT:u07_err_locked-->

- `cannot update the lock file ... because --locked was passed`（`--locked` が指定されたので Cargo.lock を更新できない）。`Cargo.toml` に新しい依存が増えたのに、`Cargo.lock` に記録がないためです。
- 手元で `cargo build`（`--locked` なし）を実行して `Cargo.lock` を更新し、それを Git に保存してから、本番用のビルドをやり直すのが正しい流れです。「`Cargo.toml` を変えたのに `Cargo.lock` の保存を忘れた」というミスを、本番に出る前に見つけられます。

#### `rust-version` で必要な Rust の版を宣言する

本書では `Cargo.toml` の `[package]` に `rust-version = "1.98"` を書きます。これは「この package をビルドするには Rust 1.98 以上が必要」という宣言です。

- 古い Rust でビルドしようとすると、分かりにくいコンパイルエラーが大量に出る代わりに、最初に1行で教えてくれます（よくあるエラー3）。
- edition 2024 の依存の決め方（resolver 3）では、cargo は依存クレートの版を選ぶときにも `rust-version` を見て、**その Rust で動く版**を優先して選びます。UNIT 06 の workspace で `Locking 6 packages to highest Rust 1.98 compatible versions` と表示されたのはこのためです。

> **注意:** 本書のシリーズは Rust 1.98 以上を基準にしています（RustNote で使う Topcoat 0.10 が Rust 1.98 以上を必要とするため）。

#### 依存の機能を必要な分だけにする（最小構成）

多くのクレートは、よく使われる機能を `default` で ON にしています。小さな道具だけ欲しいときは、`--no-default-features` で既定の機能を外し、必要なものだけを `--features` で選べます。

```powershell
cargo add serde_json --no-default-features --features alloc
```

<!--OUT:u07_add_minimal-->

`Cargo.toml` には次のように書き込まれます。

```toml
serde_json = { version = "1.0.154", default-features = false, features = ["alloc"] }
```

- `+ alloc` が ON、`- std` などが OFF になりました。`std`（OS の機能を使う部分）が要らない小さな環境向けの組み合わせです。
- どの機能が何をするかは、クレートのドキュメント（docs.rs）の「Feature flags」のページで確かめられます。**分からない機能は付けない**、**付けた理由を説明できる機能だけにする**のが安全です。
- ただし、Web アプリのようにふつうの OS の上で動くプログラムでは、既定の機能のままで困ることはほとんどありません。外すのは、ビルド時間や依存の数が問題になったとき、という順番で考えましょう。

### よくあるエラーと読み方

#### エラー1: E0425 feature を OFF にしたら関数が見つからない

例7-1 の `main` で、`export_json` を呼ぶ `println!` の上の `#[cfg(feature = "json")]` を消して、既定の機能のままビルドした場合です。

<!--OUT:u07_err_e0425-->

- **何が起きたか:** `cannot find function export_json in this scope`（`export_json` という関数が見つからない）。json が OFF なので、関数そのものがコンパイルされていません。
- うれしいことに、`note: found an item that was configured out`（設定で取り除かれた項目が見つかった）、`the item is gated behind the json feature`（json の feature の向こう側にある）と、原因まで教えてくれています。
- **直し方:** 呼ぶ側にも同じ `#[cfg(feature = "json")]` を付けるか、`cargo build --features json` で機能を ON にします。**ライブラリの API が見つからないときは、まず feature の不足を疑う**のが近道です。

#### エラー2: serde の derive 機能を付け忘れた

`cargo add serde`（`--features derive` なし）で入れた serde で、`#[derive(Serialize)]` を使った場合です。

<!--CODE:errs/u07_nodrv_main.rs-->

<!--OUT:u07_err_derive-->

- **何が起きたか:** `cannot find derive macro Serialize in this scope`（derive 用の `Serialize` が見つからない）。serde の `Serialize` には、「trait としての `Serialize`」と「derive するためのマクロとしての `Serialize`」の2つがあり、マクロのほうは **derive 機能を ON にしたときだけ**使えます。`note: Serialize is a trait, not a derive macro` は「trait のほうは見つかったけど、derive マクロではない」という意味です。
- help は「手で実装しては？」と言っていますが、ここでの正しい直し方は **`cargo add serde --features derive`** で機能を ON にすることです。このように、feature の付け忘れは、コンパイラが原因を言い当てにくいエラーになりがちです。

#### エラー3: rust-version より古い Rust でビルドした

`rust-version = "1.98"` の package を、Rust 1.90 でビルドしようとした場合です（`cargo +1.90.0 build` は「1.90.0 の Rust を使ってビルドする」という指定です）。

```powershell
cargo +1.90.0 build
```

<!--OUT:u07_err_rustver-->

- **何が起きたか:** 「rustc 1.90.0 はこの package に対応していない。feat_lab 0.1.0 は rustc 1.98 が必要」。コンパイルを始める前に止めてくれます。
- **直し方:** `rustup update` で Rust を新しくします。`rustc --version` で、今使っている版を確かめられます。

### RustNote ではここで使う

- RustNote の `Cargo.toml` には `rust-version = "1.98"` を書き、Topcoat は `=0.10.0` で版を固定します。そのほかの依存は、`cargo add` が書く `"1.0.229"` の形（互換性のある更新は受け入れる）で十分です。
- `Cargo.lock` は Git に保存し、Docker イメージを作るときや CI では `cargo build --release --locked` のように `--locked` を付けます（第12巻）。
- 依存の features は、必要なものだけを ON にします。たとえば serde は `derive` が必要なので `--features derive` を付けます。依存クレートの features の具体的な選び方は、RustNote の `Cargo.toml` を作る第9巻で扱います。

### 練習問題

**問題1:** 次の書き方は、それぞれどの範囲の版を許しますか。

- (a) `anyhow = "1.0.104"`
- (b) `topcoat = "=0.10.0"`
- (c) `foo = "0.10"`
- (d) `bar = "~1.2.3"`

**問題2:** 例7-1 に、CSV 形式で書き出す機能 `csv`（依存は増やさない）を追加してください。`"タイトル","本文"` の形で出力する `export_csv` を作り、`cargo run --no-default-features --features csv` で、テキストと CSV だけが出力されることを確かめてください。

**問題3:** 「アプリでは `Cargo.lock` を Git に保存する」理由と、本番用のビルドで `--locked` を付ける理由を、それぞれ1〜2文で説明してください。

### 解答と解説

**解答1:**

| 書き方 | 許される範囲 |
|---|---|
| (a) `"1.0.104"` | 1.0.104 以上、2.0.0 未満 |
| (b) `"=0.10.0"` | 0.10.0 ちょうど |
| (c) `"0.10"` | 0.10.0 以上、0.11.0 未満 |
| (d) `"~1.2.3"` | 1.2.3 以上、1.3.0 未満 |

- (c) が「0.10.0 以上、1.0.0 未満」ではない点に注意してください。0.x の版は、左から数えて最初の 0 でない数（ここでは「中」の 10）が上がると互換性がなくなる約束です。

**解答2:** `[features]` に1行足します。

```toml
# CSV で書き出す機能（依存は増えない）
csv = []
```

`src/main.rs` には、関数と呼び出しを足します（抜粋）。

```rust
// csv 機能が ON のときだけコンパイルされる
#[cfg(feature = "csv")]
fn export_csv(note: &Note) -> String {
    format!("\"{}\",\"{}\"", note.title, note.body)
}
```

```rust
#[cfg(feature = "csv")]
println!("[csv] {}", export_csv(&note));
```

```powershell
cargo run -q --no-default-features --features csv
```

**▼ 実行結果**

<!--OUT:u07_ex2-->

- `--no-default-features` で markdown を外し、`--features csv` で csv だけを ON にしたので、テキストと CSV だけが出力されました。複数の機能を ON にするときは `--features "csv json"` のように空白（またはカンマ）で区切ります。
- 文字列の中の `"` は、`\"` と書きます（エスケープ）。本物の CSV では、本文の中に `"` や `,` や改行があるときの扱いも必要なので、実務では `csv` クレートを使うのが安全です。

**解答3:**

- **Cargo.lock を保存する理由:** 実際に使った依存の版とチェックサムを記録しておけば、ほかのパソコンや本番サーバーでも、まったく同じ版でビルドできるからです。「いつの間にか依存が新しくなって壊れた」を防げます。
- **`--locked` を付ける理由:** `Cargo.toml` と `Cargo.lock` が食い違っている（保存し忘れ）ときに、本番用のビルドの途中で勝手に版を選び直さず、エラーで止めて知らせてくれるからです。

### まとめ

- `"1.0.229"` は「1.0.229 以上 2.0.0 未満」、`"=0.10.0"` は「ちょうど」。実際に使う版は `Cargo.lock` に記録され、アプリでは Git に保存して `--locked` でビルドします。
- features は `[features]` で定義し、`#[cfg(feature = "...")]` でコードを、`dep:` で任意の依存を出し入れします。`cargo tree` で依存の増え方を確かめられます。
- `rust-version` で必要な Rust の版を宣言し、依存の機能は必要なものだけを選びます。

<div style="page-break-after: always; text-align: center; margin: 0; padding: 0;">
<img src="assets/cover.jpg" alt="表紙" style="width: 100%; max-height: 175mm; object-fit: cover; border-radius: 10px;">
<div style="margin-top: 14px; font-size: 11px; letter-spacing: 0.18em; color: #c2410c; font-weight: 700;">VOLUME 04 / RUST BACKEND MASTERY</div>
<div style="font-size: 26px; font-weight: 800; color: #0f172a; margin-top: 6px; line-height: 1.35;">Rust設計 — 型・trait・エラー・Cargo・テスト</div>
<div style="font-size: 13px; color: #475569; margin-top: 8px;">「動けばよい」コードから、間違いを型とテストで防げる設計へ進む</div>
</div>

# 第4巻: Rust設計 — 型・trait・エラー・Cargo・テスト

## この巻で身に付けること

第2巻と第3巻で、Rust の文法と所有権のルールを学びました。この巻では、それらを組み合わせて「**間違いにくいプログラムの形**」を作る方法を学びます。

プログラムが小さいうちは、「とりあえず動く」コードでも困りません。しかし、ノートアプリ RustNote のように「登録・ログイン・ノートの作成と編集・検索」と機能が増えてくると、次のような事故が起きやすくなります。

- ユーザーの番号とノートの番号を取り違えて、**他人のノートを表示してしまう**
- 空のタイトルや 1,000 文字のタイトルが、検査をすり抜けて DB に入ってしまう
- エラーが全部「何か失敗しました」になり、利用者の入力ミスなのかサーバーの故障なのか分からない
- パスワードや DB の合言葉（トークン）が、ログにそのまま出てしまう

この巻のテーマは、こうした事故を「気をつける」ではなく、**型・trait・エラー型・Cargo の設定・テスト**という仕組みで防ぐことです。Rust のコンパイラはとても厳しい先生ですが、設計を工夫すると、その厳しさを「バグを見つけてくれる味方」として使えるようになります。

### 到達目標

この巻を終えると、次のことができるようになります。

- newtype と `parse` 関数で、**不正な値を作れない型**（`Email`・`NoteTitle`・`UserId`・`NoteId`）を設計できる
- trait で「約束」を決め、保存先などの部品を**差し替え可能**にできる。generics と `dyn Trait` の違いと使い分けを説明できる
- エラーを enum で**分類**し、`thiserror` で書きやすくし、HTTP のステータスコード（400/404/409/500）へ変換できる。`anyhow` との使い分けを説明できる
- crate・module・`pub` で公開範囲を決め、workspace・features・`Cargo.lock` で**依存を管理**できる
- Serde で JSON やフォームの入力を受け取り、**境界で検証済みの型に変換**できる。設定値を起動時に一度だけ検証し、秘密の値を表示から隠せる
- `cargo test` と doctest で設計の意図を**テストとドキュメントに固定**し、状態の移り変わり（状態機械）を enum で安全に書ける

### 学習地図

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 400" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-01" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="6" y="30" width="708" height="90" rx="12" fill="#eff6ff"/>
<rect x="6" y="140" width="708" height="90" rx="12" fill="#fff7ed"/>
<rect x="6" y="250" width="708" height="90" rx="12" fill="#ecfdf5"/>
<text x="360" y="20" font-size="13" fill="#475569" text-anchor="middle">上の段から順に進みます（各段の中は左から右へ）</text>
<text x="65" y="70" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">型で</text>
<text x="65" y="90" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">間違いを防ぐ</text>
<text x="65" y="180" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">失敗と部品を</text>
<text x="65" y="200" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">整理する</text>
<text x="65" y="290" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">外との境界と</text>
<text x="65" y="310" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">検証</text>
<rect x="135" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="200" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 01</text>
<text x="200" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">newtype</text>
<text x="200" y="99" font-size="12" fill="#475569" text-anchor="middle">取り違え防止</text>
<rect x="282" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="347" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 02</text>
<text x="347" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">trait と境界</text>
<text x="347" y="99" font-size="12" fill="#475569" text-anchor="middle">差し替え可能に</text>
<rect x="429" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="494" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 03</text>
<text x="494" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">generics</text>
<text x="494" y="99" font-size="12" fill="#475569" text-anchor="middle">型の空欄</text>
<rect x="576" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="641" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 04</text>
<text x="641" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">dyn Trait</text>
<text x="641" y="99" font-size="12" fill="#475569" text-anchor="middle">実行時に選ぶ</text>
<line x1="265" y1="75" x2="280" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-01)"/>
<line x1="412" y1="75" x2="427" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-01)"/>
<line x1="559" y1="75" x2="574" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-01)"/>
<path d="M641,110 V130 H200 V146" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-01)"/>
<rect x="135" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="200" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 05</text>
<text x="200" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">エラー型</text>
<text x="200" y="209" font-size="12" fill="#475569" text-anchor="middle">失敗を分類</text>
<rect x="282" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="347" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 06</text>
<text x="347" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">crate・module</text>
<text x="347" y="209" font-size="12" fill="#475569" text-anchor="middle">公開範囲</text>
<rect x="429" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="494" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 07</text>
<text x="494" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">features・依存</text>
<text x="494" y="209" font-size="12" fill="#475569" text-anchor="middle">依存を固定</text>
<line x1="265" y1="185" x2="280" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-01)"/>
<line x1="412" y1="185" x2="427" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-01)"/>
<rect x="576" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-dasharray="5 4"/>
<text x="641" y="174" font-size="12" fill="#475569" text-anchor="middle">RustNote では</text>
<text x="641" y="194" font-size="12" fill="#475569" text-anchor="middle">AppError</text>
<text x="641" y="212" font-size="12" fill="#475569" text-anchor="middle">Cargo.toml</text>
<path d="M494,220 V240 H200 V256" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-01)"/>
<rect x="135" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="200" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 08</text>
<text x="200" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">DTO・Serde</text>
<text x="200" y="319" font-size="12" fill="#475569" text-anchor="middle">入出力の変換</text>
<rect x="282" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="347" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 09</text>
<text x="347" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">設定と秘密</text>
<text x="347" y="319" font-size="12" fill="#475569" text-anchor="middle">起動時に検証</text>
<rect x="429" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="494" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 10</text>
<text x="494" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">テスト・doc</text>
<text x="494" y="319" font-size="12" fill="#475569" text-anchor="middle">設計を固定</text>
<line x1="265" y1="295" x2="280" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-01)"/>
<line x1="412" y1="295" x2="427" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-01)"/>
<rect x="576" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-dasharray="5 4"/>
<text x="641" y="284" font-size="12" fill="#475569" text-anchor="middle">RustNote では</text>
<text x="641" y="304" font-size="12" fill="#475569" text-anchor="middle">Config・DTO</text>
<text x="641" y="322" font-size="12" fill="#475569" text-anchor="middle">テスト</text>
<text x="360" y="372" font-size="13" fill="#475569" text-anchor="middle">各単元の流れ: たとえ話 → 図 → 最小コード → 実務の形 → よくあるエラー → 練習問題</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-1: 第4巻の学習地図。前半で型の道具をそろえ、後半でアプリの外との境界を固める</div>
</div>

| 単元 | テーマ | ひとことで言うと |
|---|---|---|
| UNIT 01 | newtype と不正状態を作れない型 | `UserId` と `NoteId` を取り違えたらコンパイルエラーにする |
| UNIT 02 | trait と境界による設計 | 「保存できる」という約束で、本物とテスト用の偽物を差し替える |
| UNIT 03 | generics・where 句・静的ディスパッチ | 型の空欄を持つ関数・構造体を1回だけ書く |
| UNIT 04 | dyn Trait とオブジェクト安全性 | 実行時に実装を選ぶ。その仕組み（vtable）と制約 |
| UNIT 05 | エラー型・thiserror・文脈 | 失敗を分類し、HTTP の 400/404/409/500 に変換する |
| UNIT 06 | crate・module・可視性・workspace | ファイルを分け、`pub` で見せる範囲を決める |
| UNIT 07 | Cargo features・依存固定・最小構成 | 依存の版をそろえ、必要な機能だけを入れる |
| UNIT 08 | DTO・Serde・境界の変換 | JSON・フォームの入力を検証済みの型に変える |
| UNIT 09 | 設定型・環境変数・秘密 | 設定を起動時に1回だけ読み、秘密を表示から隠す |
| UNIT 10 | テスト・rustdoc・状態機械・API レビュー | 設計の意図をテストとドキュメントに残す |

### 前提知識（先に読んでおく巻）

- **第1巻（環境構築）:** Rust・VS Code・rust-analyzer がインストール済みで、`cargo new` と `cargo run` ができること。
- **第2巻（Rust 基礎）:** `struct` / `impl` / `enum` / `match` / `Option` / `Result` / `?` / `Vec` / `HashMap` / クロージャ / `mod` の基本。
- **第3巻（所有権・借用）:** 所有権の移動、`&` と `&mut` の借用、`Clone` と `Copy` の違い、`Box`・`Arc`・`Mutex` の役割。

分からない言葉が出てきたら、該当の巻に戻ってかまいません。この巻の中でも、初めて出る用語には短い説明を付けています。巻末の「用語ミニ辞典」も使ってください。

### この巻の進め方

各単元では、次の順番を1サイクルとして進めてください。読むだけで終わらせず、**手を動かして、わざと壊して、エラーを読む**のが一番の近道です。

1. **予想する:** コードを実行する前に、何が表示されるか、どこでエラーになるかを予想します。
2. **そのまま動かす:** 教材のコードをそのまま実行し、実行結果が教材と同じか比べます（`Finished ... in 0.36s` のような時間の数字は、パソコンによって変わります）。
3. **1か所だけ変える:** 値や型を1つだけ変えて、結果がどう変わるかを観察します。「よくあるエラー」の節のコードを実際に書いて、エラーメッセージを自分の目で読みます。
4. **整える:** 単元の終わりに、次の3つのコマンドを実行する習慣をつけます。

```powershell
# コードの見た目を Rust の標準の形に整える
cargo fmt
# よくある書き間違い・もったいない書き方を指摘してもらう
cargo clippy
# テストを実行する（UNIT 10 で詳しく学びます）
cargo test
```

### 使うツールと版

この巻のコードは、すべて次の環境で実際にコンパイル・実行し、実行結果とエラーメッセージは実際の出力をそのまま載せています（パソコンのユーザー名などを含む部分だけ `...` に置き換えています）。

| 項目 | 版・内容 |
|---|---|
| OS | Windows 11（ARM64）。コマンドは PowerShell を主に、WSL / macOS / Linux は補足で示します |
| Rust | rustc 1.97.1 / cargo 1.97.1（`stable-aarch64-pc-windows-msvc`） |
| edition | 2024（`cargo new` の既定値） |
| thiserror | 2.0.21（UNIT 05・09。エラー型を短く書く） |
| anyhow | 1.0.104（UNIT 05。アプリの入口でエラーをまとめる） |
| serde | 1.0.229（`derive` 機能つき。UNIT 07・08） |
| serde_json | 1.0.154（UNIT 08。JSON の読み書き） |

> **ヒント:** 版の数字は「`cargo add` したときの最新版」です。あなたが試すときにはもっと新しい版が入るかもしれませんが、同じ大きな番号（thiserror なら `2.`、serde なら `1.`）の中では、この巻のコードはそのまま動くはずです。版の数字の読み方は UNIT 07 で学びます。

### 練習用プロジェクトを用意する

この巻では、練習用のプロジェクト `design_lab` を1つ作り、例ごとに `src/bin/` の下へファイルを足していきます（UNIT 06・07・10 では、別のプロジェクトも作ります）。

```powershell
cargo new design_lab
cd design_lab
mkdir src/bin
cargo add thiserror anyhow serde_json
cargo add serde --features derive
```

`cargo add` の実際の出力は次のとおりです（`cargo new` の出力は省略）。

```text
    Updating crates.io index
      Adding thiserror v2.0.21 to dependencies
             Features:
             + std
      Adding anyhow v1.0.104 to dependencies
             Features:
             + std
             - backtrace
      Adding serde_json v1.0.154 to dependencies
             Features:
             + std
             - alloc
             - arbitrary_precision
             - float_roundtrip
             - indexmap
             - preserve_order
             - raw_value
             - unbounded_depth
    Updating crates.io index
     Locking 14 packages to latest Rust 1.97.1 compatible versions
    Updating crates.io index
      Adding serde v1.0.229 to dependencies
             Features:
             + derive
             + serde_derive
             + std
             - alloc
             - rc
             - unstable
```

`Cargo.toml` は次のようになります。

```toml
[package]
name = "design_lab"
version = "0.1.0"
edition = "2024"

[dependencies]
anyhow = "1.0.104"
serde = { version = "1.0.229", features = ["derive"] }
serde_json = "1.0.154"
thiserror = "2.0.21"
```

- `src/bin/u01_ids.rs` のようにファイルを置くと、`cargo run --bin u01_ids` でそのファイルの `main` だけを実行できます。1つのプロジェクトで、たくさんの小さなプログラムを試せて便利です。
- 「よくあるエラー」の節のコードは、わざとコンパイルが失敗するコードです。`src/bin/` に置いたままにすると、プロジェクト全体をビルドする `cargo build`・`cargo test`・`cargo clippy` まで失敗するようになるので、エラーを確かめたら**ファイルを消すか、正しく直して**おいてください。

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

```rust
// 素の i64 のままだと、どちらが何の番号か型からは分からない
fn find_note_raw(owner_id: i64, note_id: i64) -> String {
    format!("ユーザー{owner_id}のノート{note_id}を探します")
}

// 意味ごとに別の型を作る（newtype パターン）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NoteId(i64);

// 引数の型そのものが「何の番号か」を表している
fn find_note(owner: UserId, id: NoteId) -> String {
    format!("ユーザー{}のノート{}を探します", owner.0, id.0)
}

fn main() {
    let owner_id: i64 = 7;
    let note_id: i64 = 42;
    // 正しい順番
    println!("{}", find_note_raw(owner_id, note_id));
    // 順番を取り違えても、コンパイルは通ってしまう
    println!("{}", find_note_raw(note_id, owner_id));

    let owner = UserId(7);
    let note = NoteId(42);
    // newtype なら、正しい順番でしか呼べない
    println!("{}", find_note(owner, note));
    // Debug 表示では型の名前も出る
    println!("{owner:?} / {note:?}");
}
```

`src/bin/u01_ids.rs` として保存し、次のコマンドで実行します。

```powershell
cargo run --bin u01_ids
```

**▼ 実行結果**

```text
   Compiling design_lab v0.1.0 (...\design_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target\debug\u01_ids.exe`
ユーザー7のノート42を探します
ユーザー42のノート7を探します
ユーザー7のノート42を探します
UserId(7) / NoteId(42)
```

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

```rust
mod domain {
    use std::fmt;

    // 中身の String は private（pub を付けない）。外から直接作れない
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NoteTitle(String);

    // タイトルが不正だった理由
    #[derive(Debug, PartialEq, Eq)]
    pub enum TitleError {
        Empty,
        TooLong { len: usize },
    }

    impl NoteTitle {
        pub const MAX_CHARS: usize = 80;

        // 文字列を検査し、合格したときだけ NoteTitle を返す
        pub fn parse(input: &str) -> Result<NoteTitle, TitleError> {
            let trimmed = input.trim();
            // 文字数は chars().count() で数える（len() はバイト数）
            let len = trimmed.chars().count();
            if len == 0 {
                return Err(TitleError::Empty);
            }
            if len > Self::MAX_CHARS {
                return Err(TitleError::TooLong { len });
            }
            Ok(NoteTitle(trimmed.to_string()))
        }

        // 中身を読むだけの窓口（書き換える窓口は用意しない）
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    // {} で表示したときの見た目
    impl fmt::Display for NoteTitle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.0)
        }
    }
}

use domain::NoteTitle;

fn main() {
    let inputs = ["  買い物リスト  ", "", "   ", &"あ".repeat(81)];
    for input in inputs {
        match NoteTitle::parse(input) {
            Ok(title) => println!("OK: [{title}] ({}文字)", title.as_str().chars().count()),
            Err(e) => println!("NG: {e:?}"),
        }
    }
}
```

```powershell
cargo run --bin u01_title
```

**▼ 実行結果**

```text
   Compiling design_lab v0.1.0 (...\design_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target\debug\u01_title.exe`
OK: [買い物リスト] (6文字)
NG: Empty
NG: Empty
NG: TooLong { len: 81 }
```

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

```rust
mod domain {
    use std::fmt;
    use std::str::FromStr;

    // ---- ID 型 ----
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NoteId(i64);

    #[derive(Debug, PartialEq, Eq)]
    pub enum IdError {
        NotANumber,
        NotPositive,
    }

    impl NoteId {
        // DB から読んだ値など、すでに正しいと分かっている数から作る
        pub fn new(value: i64) -> Result<NoteId, IdError> {
            if value <= 0 {
                return Err(IdError::NotPositive);
            }
            Ok(NoteId(value))
        }

        pub fn get(self) -> i64 {
            self.0
        }
    }

    // "42".parse::<NoteId>() と書けるようにする
    impl FromStr for NoteId {
        type Err = IdError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let n: i64 = s.parse().map_err(|_| IdError::NotANumber)?;
            NoteId::new(n)
        }
    }

    // ---- メールアドレス ----
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Email(String);

    #[derive(Debug, PartialEq, Eq)]
    pub enum EmailError {
        MissingAt,
        Malformed,
        TooLong,
    }

    impl Email {
        pub const MAX_LEN: usize = 254;

        pub fn parse(input: &str) -> Result<Email, EmailError> {
            // 前後の空白を除き、小文字にそろえる（同じ人を同じ値にするため）
            let normalized = input.trim().to_lowercase();
            if normalized.len() > Self::MAX_LEN {
                return Err(EmailError::TooLong);
            }
            let Some((local, domain)) = normalized.split_once('@') else {
                return Err(EmailError::MissingAt);
            };
            let bad = local.is_empty()
                || domain.is_empty()
                || domain.contains('@')
                || !domain.contains('.')
                || normalized.contains(char::is_whitespace);
            if bad {
                return Err(EmailError::Malformed);
            }
            Ok(Email(normalized))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    impl fmt::Display for Email {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.0)
        }
    }
}

use domain::{Email, NoteId};

fn main() {
    // URL の /notes/{id} の {id} 部分を想定
    for segment in ["42", "abc", "0", "-5"] {
        let result: Result<NoteId, _> = segment.parse();
        println!("{segment:>4} -> {result:?}");
    }

    for input in [
        "  Alice@Example.COM ",
        "alice.example.com",
        "a@b@c.com",
        "bob@localhost",
    ] {
        match Email::parse(input) {
            Ok(email) => println!("OK: {} ({}バイト)", email, email.as_str().len()),
            Err(e) => println!("NG: {input:?} -> {e:?}"),
        }
    }

    let id = NoteId::new(7).expect("7 は正の数");
    println!("DB に渡す値: {}", id.get());
}
```

```powershell
cargo run --bin u01_domain
```

**▼ 実行結果**

```text
   Compiling design_lab v0.1.0 (...\design_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
     Running `target\debug\u01_domain.exe`
  42 -> Ok(NoteId(42))
 abc -> Err(NotANumber)
   0 -> Err(NotPositive)
  -5 -> Err(NotPositive)
OK: alice@example.com (17バイト)
NG: "alice.example.com" -> MissingAt
NG: "a@b@c.com" -> Malformed
NG: "bob@localhost" -> Malformed
DB に渡す値: 7
```

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

```text
   Compiling design_lab v0.1.0 (...\design_lab)
error[E0308]: arguments to this function are incorrect
  --> src\bin\u01_err_swap.rs:15:20
   |
15 |     println!("{}", find_note(note, owner));
   |                    ^^^^^^^^^ ----  ----- expected `NoteId`, found `UserId`
   |                              |
   |                              expected `UserId`, found `NoteId`
   |
note: function defined here
  --> src\bin\u01_err_swap.rs:7:4
   |
 7 | fn find_note(owner: UserId, id: NoteId) -> String {
   |    ^^^^^^^^^
help: swap these arguments
   |
15 -     println!("{}", find_note(note, owner));
15 +     println!("{}", find_note(owner, note));
   |

For more information about this error, try `rustc --explain E0308`.
error: could not compile `design_lab` (bin "u01_err_swap") due to 1 previous error
```

- **何が起きたか:** `E0308` は「型が合わない（mismatched types）」系のエラーです。1つ目の引数は `UserId` を期待しているのに `NoteId` が来た（`expected UserId, found NoteId`）、と引数ごとに教えてくれています。
- **直し方:** `help: swap these arguments`（引数を入れ替えて）の提案どおり、`find_note(owner, note)` にします。この提案は正しいので、そのまま従って大丈夫です。
- これこそ newtype の狙いです。`i64` のままだったら、このバグは実行して、しかも運悪く他人のノートが見えてしまうまで気づけませんでした。

#### エラー2: E0603 private なコンストラクタは使えない

`parse` を通さずに、空のタイトルを直接作ろうとしたコードです（`src/bin/u01_err_private.rs`）。

```rust
mod domain {
    #[derive(Debug)]
    pub struct NoteTitle(String);

    impl NoteTitle {
        pub fn parse(input: &str) -> Option<NoteTitle> {
            let t = input.trim();
            if t.is_empty() {
                None
            } else {
                Some(NoteTitle(t.to_string()))
            }
        }
    }
}

fn main() {
    // parse を通さずに、空のタイトルを直接作ろうとする
    let title = domain::NoteTitle(String::new());
    println!("{title:?}");
}
```

```text
   Compiling design_lab v0.1.0 (...\design_lab)
error[E0603]: tuple struct constructor `NoteTitle` is private
  --> src\bin\u01_err_private.rs:19:25
   |
 3 |     pub struct NoteTitle(String);
   |                          ------ a constructor is private if any of the fields is private
...
19 |     let title = domain::NoteTitle(String::new());
   |                         ^^^^^^^^^ private tuple struct constructor
   |
note: the tuple struct constructor `NoteTitle` is defined here
  --> src\bin\u01_err_private.rs:3:5
   |
 3 |     pub struct NoteTitle(String);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider making the field publicly accessible
   |
 3 |     pub struct NoteTitle(pub String);
   |                          +++

For more information about this error, try `rustc --explain E0603`.
error: could not compile `design_lab` (bin "u01_err_private") due to 1 previous error
```

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
| `NoteTitle` | `String`（trim 済み・1〜80文字） | `NoteTitle::parse` | 空タイトル・長すぎるタイトル |

たとえば `GET /notes/{id}` では、URL の `{id}` を受け取ったらすぐ `NoteId` に変換し、失敗したら 404 を返します。その先の処理は `NoteId` しか受け取らないので、「数字でない ID で DB を検索してしまう」心配がありません。ID の `i64` は、Turso / libSQL（SQLite 系）の `INTEGER` 型の列とそのまま対応します。

### 練習問題

**問題1:** ページ送りに使う「ページ番号」型 `PageNumber` を作ってください。中身は `u32` で、**0 は作れない**（1 から始まる）ようにします。`fn new(n: u32) -> Option<PageNumber>` と、1ページあたりの件数を受け取って「何件飛ばすか」を返す `fn offset(self, per_page: u32) -> u32` を書き、`new(0)` と、3ページ目・20件ずつの offset を表示してください。

**問題2:** `"日本語"` の `len()` と `chars().count()` をそれぞれ表示し、なぜ `NoteTitle` の文字数判定に `len()` を使ってはいけないのかを説明してください。

**問題3:** ある人が「テストで便利だから」と、`NoteTitle` に `pub fn new_unchecked(s: String) -> NoteTitle { NoteTitle(s) }`（検査なしで作れる関数）を追加しようとしています。何が問題でしょうか。どうすれば目的（テストで手軽に作る）を果たせるでしょうか。

### 解答と解説

**解答1:**

```rust
mod paging {
    // 1 から始まるページ番号。0 ページは存在しない
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PageNumber(u32);

    impl PageNumber {
        pub fn new(n: u32) -> Option<PageNumber> {
            if n == 0 { None } else { Some(PageNumber(n)) }
        }

        // 1 ページ目なら 0 件飛ばす、2 ページ目なら per_page 件飛ばす
        pub fn offset(self, per_page: u32) -> u32 {
            (self.0 - 1) * per_page
        }
    }
}

use paging::PageNumber;

fn main() {
    println!("{:?}", PageNumber::new(0));
    let page = PageNumber::new(3).unwrap();
    println!("{:?} の offset = {}", page, page.offset(20));
}
```

**▼ 実行結果**

```text
None
PageNumber(3) の offset = 40
```

- `PageNumber` は 0 では作れないので、`offset` の中の `self.0 - 1` が**マイナスになる（u32 では桁あふれでパニックになる）ことが絶対にありません**。もし `u32` のまま受け取っていたら、`offset` の中でも 0 の検査が必要でした。検査を `new` に寄せたおかげで、使う側の関数が単純になっています。

**解答2:**

```rust
fn main() {
    let title = "日本語";
    // len() は UTF-8 のバイト数
    println!("len() = {}", title.len());
    // chars().count() は文字の数
    println!("chars().count() = {}", title.chars().count());
}
```

**▼ 実行結果**

```text
len() = 9
chars().count() = 3
```

- 日本語の1文字は UTF-8 で3バイトなので、`len()` は 9 になります。`len()` で「80以下」を判定すると、日本語のタイトルは 26 文字までしか入らなくなり、利用者から見ると「80文字まで」の約束が守られません。

**解答3:**

- 検査なしで作れる `pub` な関数があると、テスト以外のコードからも呼べてしまい、「`NoteTitle` があれば必ず 1〜80文字」という約束が崩れます。門の横に裏口を作るようなものです。
- テストで手軽に作りたいなら、テストの中でも `NoteTitle::parse("買い物").unwrap()` と書けば十分です。テストでは失敗したらパニックで止まってよいので、`unwrap()` が使えます。どうしても専用の関数が欲しい場合は、`#[cfg(test)]`（テストのときだけコンパイルする印。UNIT 10 で学びます）を付けて、本番のプログラムには含まれないようにします。

### まとめ

- 意味の違う値は newtype で別の型にすると、取り違えが `E0308` のコンパイルエラーになります。
- 中身を private にして `parse` だけを入口にすると、「作れた＝正しい」が型で保証されます。
- コンパイラの help は「通す方法」です。設計を壊す提案（`pub` を付ける等）は採用しません。

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

```rust
// 「一行で要約できる」という約束（trait）
trait Summary {
    // 実装する型が必ず書かなければならないメソッド
    fn summary(&self) -> String;

    // 既定の実装つきメソッド（書かなければこれが使われる）
    fn headline(&self) -> String {
        format!("【{}】", self.summary())
    }
}

struct Note {
    title: String,
    done: bool,
}

struct User {
    email: String,
}

// Note は Summary の約束を守ります、という宣言
impl Summary for Note {
    fn summary(&self) -> String {
        let mark = if self.done { "済" } else { "未" };
        format!("{mark} {}", self.title)
    }
}

impl Summary for User {
    fn summary(&self) -> String {
        format!("ユーザー {}", self.email)
    }

    // 既定の実装を上書きする
    fn headline(&self) -> String {
        format!("<<{}>>", self.email)
    }
}

// Summary を実装した型なら何でも受け取れる関数
fn print_headline(item: &impl Summary) {
    println!("{}", item.headline());
}

fn main() {
    let note = Note {
        title: "牛乳を買う".to_string(),
        done: false,
    };
    let user = User {
        email: "alice@example.com".to_string(),
    };

    println!("{}", note.summary());
    print_headline(&note);
    print_headline(&user);
}
```

```powershell
cargo run --bin u02_trait
```

**▼ 実行結果**

```text
   Compiling design_lab v0.1.0 (...\design_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.89s
     Running `target\debug\u02_trait.exe`
未 牛乳を買う
【未 牛乳を買う】
<<alice@example.com>>
```

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

```rust
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NoteId(i64);

#[derive(Debug, Clone)]
struct Note {
    id: NoteId,
    owner: UserId,
    title: String,
}

// 保存に失敗したことを表す型（DB に接続できない、など）
#[derive(Debug)]
struct RepoError(String);

// ノートの保存場所に求める「約束」
trait NoteRepository {
    fn insert(&self, owner: UserId, title: &str) -> Result<Note, RepoError>;
    fn list_by_owner(&self, owner: UserId) -> Result<Vec<Note>, RepoError>;
}

// ---- 実装1: メモリの中に保存する ----
struct Inner {
    next_id: i64,
    notes: Vec<Note>,
}

struct InMemoryNoteRepository {
    // &self のまま中身を書き換えられるよう Mutex で包む（第3巻）
    inner: Mutex<Inner>,
}

impl InMemoryNoteRepository {
    fn new() -> Self {
        let inner = Inner {
            next_id: 1,
            notes: Vec::new(),
        };
        Self {
            inner: Mutex::new(inner),
        }
    }
}

impl NoteRepository for InMemoryNoteRepository {
    fn insert(&self, owner: UserId, title: &str) -> Result<Note, RepoError> {
        let mut inner = self.inner.lock().unwrap();
        let note = Note {
            id: NoteId(inner.next_id),
            owner,
            title: title.to_string(),
        };
        inner.next_id += 1;
        inner.notes.push(note.clone());
        Ok(note)
    }

    fn list_by_owner(&self, owner: UserId) -> Result<Vec<Note>, RepoError> {
        let inner = self.inner.lock().unwrap();
        let mine = inner
            .notes
            .iter()
            .filter(|n| n.owner == owner)
            .cloned()
            .collect();
        Ok(mine)
    }
}

// ---- 実装2: いつも失敗する偽物（テスト用の Fake）----
struct BrokenRepository;

impl NoteRepository for BrokenRepository {
    fn insert(&self, _owner: UserId, _title: &str) -> Result<Note, RepoError> {
        Err(RepoError("DB に接続できません".to_string()))
    }

    fn list_by_owner(&self, _owner: UserId) -> Result<Vec<Note>, RepoError> {
        Err(RepoError("DB に接続できません".to_string()))
    }
}

// ユースケース「ノートを作る」。保存先が何なのかは知らない
fn create_note(repo: &impl NoteRepository, owner: UserId, title: &str) -> Result<NoteId, String> {
    let title = title.trim();
    if title.is_empty() {
        return Err("タイトルが空です".to_string());
    }
    let note = repo
        .insert(owner, title)
        .map_err(|e| format!("保存に失敗しました: {}", e.0))?;
    Ok(note.id)
}

fn main() {
    let repo = InMemoryNoteRepository::new();
    let alice = UserId(1);
    let bob = UserId(2);

    println!("{:?}", create_note(&repo, alice, "牛乳を買う"));
    println!("{:?}", create_note(&repo, bob, "本を返す"));
    println!("{:?}", create_note(&repo, alice, "   "));
    println!("{:?}", create_note(&repo, alice, "電気代を払う"));

    for note in repo.list_by_owner(alice).unwrap() {
        println!("alice のノート: {:?} {}", note.id, note.title);
    }

    // 同じユースケースに、壊れた保存先を渡してみる
    let broken = BrokenRepository;
    println!("{:?}", create_note(&broken, alice, "牛乳を買う"));
}
```

```powershell
cargo run --bin u02_repo
```

**▼ 実行結果**

```text
   Compiling design_lab v0.1.0 (...\design_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.64s
     Running `target\debug\u02_repo.exe`
Ok(NoteId(1))
Ok(NoteId(2))
Err("タイトルが空です")
Ok(NoteId(3))
alice のノート: NoteId(1) 牛乳を買う
alice のノート: NoteId(3) 電気代を払う
Err("保存に失敗しました: DB に接続できません")
```

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

```rust
trait Summary {
    fn summary(&self) -> String;
}

struct Note {
    title: String,
}

impl Summary for Note {
    fn summary(&self) -> String {
        self.title.clone()
    }
}

fn print_summary(item: &impl Summary) {
    println!("{}", item.summary());
}

fn main() {
    let note = Note {
        title: "牛乳を買う".to_string(),
    };
    print_summary(&note);
    // i32 は Summary を実装していない
    print_summary(&42);
}
```

```text
   Compiling design_lab v0.1.0 (...\design_lab)
error[E0277]: the trait bound `{integer}: Summary` is not satisfied
  --> src\bin\u02_err_e0277.rs:25:19
   |
25 |     print_summary(&42);
   |     ------------- ^^^ the trait `Summary` is not implemented for `{integer}`
   |     |
   |     required by a bound introduced by this call
   |
help: the trait `Summary` is implemented for `Note`
  --> src\bin\u02_err_e0277.rs:9:1
   |
 9 | impl Summary for Note {
   | ^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `print_summary`
  --> src\bin\u02_err_e0277.rs:15:30
   |
15 | fn print_summary(item: &impl Summary) {
   |                              ^^^^^^^ required by this bound in `print_summary`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `design_lab` (bin "u02_err_e0277") due to 1 previous error
```

- **何が起きたか:** `E0277` は「trait 境界を満たしていない」エラーです。`the trait Summary is not implemented for {integer}`（整数には `Summary` が実装されていない）と書かれています。`{integer}` は「まだ i32 か i64 か決まっていない整数」という意味です。
- 下の `note: required by a bound in print_summary` が、「その条件を出したのは `print_summary` の `impl Summary` の部分ですよ」と、境界の場所まで教えてくれています。`help` には「`Note` なら実装されていますよ」とヒントも出ています。
- **直し方:** 渡す値を `Summary` を実装した型にするか、その型に `impl Summary for 型` を書き足します。E0277 は Rust で最もよく見るエラーの1つなので、「`the trait X is not implemented for Y`」の X と Y を読み取る練習をしておきましょう。

#### エラー2: E0046 必須のメソッドを書き忘れた

```rust
trait Summary {
    fn summary(&self) -> String;
    fn headline(&self) -> String {
        format!("【{}】", self.summary())
    }
}

struct User {
    email: String,
}

// summary を書き忘れた
impl Summary for User {
    fn headline(&self) -> String {
        format!("<<{}>>", self.email)
    }
}

fn main() {
    let user = User {
        email: "alice@example.com".to_string(),
    };
    println!("{}", user.headline());
}
```

```text
   Compiling design_lab v0.1.0 (...\design_lab)
error[E0046]: not all trait items implemented, missing: `summary`
  --> src\bin\u02_err_e0046.rs:13:1
   |
 2 |     fn summary(&self) -> String;
   |     ---------------------------- `summary` from trait
...
13 | impl Summary for User {
   | ^^^^^^^^^^^^^^^^^^^^^ missing `summary` in implementation

For more information about this error, try `rustc --explain E0046`.
error: could not compile `design_lab` (bin "u02_err_e0046") due to 1 previous error
```

- **何が起きたか:** `not all trait items implemented, missing: summary`（trait の項目が全部実装されていない。`summary` が足りない）。既定の実装がない `summary` を書き忘れています。
- **直し方:** `impl Summary for User` の中に `fn summary(&self) -> String { ... }` を書き足します。rust-analyzer を使っていれば、`impl` の行で「Implement missing members」というクイック修正を選ぶと、ひな形を自動で入れてくれます。

#### エラー3: E0117 よその trait を よその型に実装できない（孤児ルール）

```rust
use std::fmt;

// 標準ライブラリの trait（Display）を、標準ライブラリの型（Vec<String>）に実装しようとする
impl fmt::Display for Vec<String> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.join(", "))
    }
}

fn main() {
    let titles = vec!["牛乳を買う".to_string(), "本を返す".to_string()];
    println!("{titles}");
}
```

```text
   Compiling design_lab v0.1.0 (...\design_lab)
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
 --> src\bin\u02_err_e0117.rs:4:1
  |
4 | impl fmt::Display for Vec<String> {
  | ^^^^^^^^^^^^^^^^^^^^^^-----------
  |                       |
  |                       `Vec` is not defined in the current crate
  |
  = note: impl doesn't have any local type before any uncovered type parameters
  = note: for more information see https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules
  = note: define and implement a trait or new type instead

For more information about this error, try `rustc --explain E0117`.
error: could not compile `design_lab` (bin "u02_err_e0117") due to 1 previous error
```

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

```rust
trait Summary {
    fn summary(&self) -> String;

    fn headline(&self) -> String {
        format!("【{}】", self.summary())
    }
}

struct Tag {
    name: String,
    count: usize,
}

impl Summary for Tag {
    fn summary(&self) -> String {
        format!("#{} ({}件)", self.name, self.count)
    }
}

fn print_headline(item: &impl Summary) {
    println!("{}", item.headline());
}

fn main() {
    let tag = Tag {
        name: "買い物".to_string(),
        count: 3,
    };
    print_headline(&tag);
}
```

**▼ 実行結果**

```text
【#買い物 (3件)】
```

- `headline` を書かなかったので、trait の既定の実装（`【...】` で囲む）が使われています。

**解答2:**

```rust
// 「今の時刻（秒）」を教えてくれるものの約束
trait Clock {
    fn now(&self) -> u64;
}

// 本番用: OS の時計を読む
struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

// テスト用: 決めた時刻を返し続ける
struct FixedClock(u64);

impl Clock for FixedClock {
    fn now(&self) -> u64 {
        self.0
    }
}

// セッションの期限切れ判定。時計の種類は知らない
fn is_expired(clock: &impl Clock, expires_at: u64) -> bool {
    clock.now() >= expires_at
}

fn main() {
    let before = FixedClock(999);
    let just = FixedClock(1000);
    println!("期限の1秒前: {}", is_expired(&before, 1000));
    println!("期限ちょうど: {}", is_expired(&just, 1000));
    // 本物の時計では、2000年1月1日(946684800)はとうに過ぎている
    println!("本物の時計: {}", is_expired(&SystemClock, 946_684_800));
}
```

**▼ 実行結果**

```text
期限の1秒前: false
期限ちょうど: true
本物の時計: true
```

- 時刻のように「実行するたびに変わるもの」に直接頼ると、テストの結果が実行のタイミングで変わってしまいます。trait で差し替え可能にしておけば、テストでは `FixedClock` を渡して「期限ちょうど」のような境目を、何度でも同じ結果で確かめられます。
- `use std::time::{...}` を関数の中に書いているのは、その関数でしか使わないものを近くに置くためです。ファイルの先頭に書いてもかまいません。

**解答3:**

```rust
use std::fmt;

// Vec<String> を包んだ、自分の crate の型を作る
struct TitleList(Vec<String>);

// 自分の型なら、標準ライブラリの trait を実装できる
impl fmt::Display for TitleList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}

fn main() {
    let titles = TitleList(vec!["牛乳を買う".to_string(), "本を返す".to_string()]);
    println!("{titles}");
}
```

**▼ 実行結果**

```text
牛乳を買う, 本を返す
```

- `TitleList` は自分の crate で定義した型なので、孤児ルールに引っかかりません。UNIT 01 の newtype は「意味を分ける」だけでなく、「よその型に trait を足す」ための道具にもなります。

### まとめ

- trait は「型が守る約束」です。`impl トレイト for 型` で約束を守り、既定の実装で書く量を減らせます。
- `&impl Trait` の引数は「約束を守る型なら何でも」受け取り、関数の中では約束のメソッドだけを使います。
- ユースケースを `NoteRepository` trait に依存させると、本物の保存先とテスト用 Fake を差し替えられます。

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

```rust
use std::any::type_name;

// T は「呼ばれるときに決まる型」。
// PartialOrd（大小を比べられる）と Copy（コピーできる）を T に求める
fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let mut iter = items.iter().copied();
    // 空なら None を返して終わり
    let mut max = iter.next()?;
    for item in iter {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

// どの型で呼ばれたのかを表示する
fn show_type<T>(_value: &T) {
    println!("この呼び出しの T = {}", type_name::<T>());
}

fn main() {
    println!("{:?}", largest(&[3, 9, 4]));
    println!("{:?}", largest(&[1.5, 0.2]));
    println!("{:?}", largest(&['あ', 'ん', 'か']));
    let empty: [i32; 0] = [];
    println!("{:?}", largest(&empty));

    show_type(&42);
    show_type(&1.5);
    show_type(&"文字列");
}
```

```powershell
cargo run --bin u03_generic
```

**▼ 実行結果**

```text
   Compiling design_lab v0.1.0 (...\design_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running `target\debug\u03_generic.exe`
Some(9)
Some(1.5)
Some('ん')
None
この呼び出しの T = i32
この呼び出しの T = f64
この呼び出しの T = &str
```

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

```rust
use std::fmt::Display;

// どんな型の一覧にも使える「1ページ分」の入れ物
#[derive(Debug)]
struct Page<T> {
    items: Vec<T>,
    page: u32,
    per_page: u32,
    total: usize,
}

// すべての T で使えるメソッド
impl<T> Page<T> {
    fn has_next(&self) -> bool {
        (self.page as usize) * (self.per_page as usize) < self.total
    }
}

// T が Display を実装しているときだけ使えるメソッド
impl<T: Display> Page<T> {
    fn render(&self) -> String {
        let lines: Vec<String> = self.items.iter().map(|x| format!("- {x}")).collect();
        format!(
            "{}\n({}ページ目 / 全{}件)",
            lines.join("\n"),
            self.page,
            self.total
        )
    }
}

// 何かの並び（Vec でも配列でもイテレータでもよい）から、1ページ分を切り出す
fn paginate<T, I>(all: I, page: u32, per_page: u32) -> Page<T>
where
    I: IntoIterator<Item = T>,
{
    let all: Vec<T> = all.into_iter().collect();
    let total = all.len();
    let skip = ((page - 1) * per_page) as usize;
    let items = all.into_iter().skip(skip).take(per_page as usize).collect();
    Page {
        items,
        page,
        per_page,
        total,
    }
}

fn main() {
    let titles = vec!["牛乳", "本", "電気代", "歯医者", "散髪"];
    let p1 = paginate(titles.clone(), 1, 2);
    println!("{}", p1.render());
    println!("次のページはある？ {}", p1.has_next());

    let p3 = paginate(titles, 3, 2);
    println!("{}", p3.render());
    println!("次のページはある？ {}", p3.has_next());

    // 数字の範囲（1..=7）からも同じ関数で切り出せる
    let numbers = paginate(1..=7, 2, 3);
    println!("{numbers:?}");
}
```

```powershell
cargo run --bin u03_page
```

**▼ 実行結果**

```text
   Compiling design_lab v0.1.0 (...\design_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.63s
     Running `target\debug\u03_page.exe`
- 牛乳
- 本
(1ページ目 / 全5件)
次のページはある？ true
- 散髪
(3ページ目 / 全5件)
次のページはある？ false
Page { items: [4, 5, 6], page: 2, per_page: 3, total: 7 }
```

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

```rust
use std::fmt::Debug;

trait NoteRepository {
    fn titles(&self) -> Vec<String>;
}

#[derive(Debug)]
struct InMemory(Vec<String>);

impl NoteRepository for InMemory {
    fn titles(&self) -> Vec<String> {
        self.0.clone()
    }
}

// 保存先の型 R をパラメータとして持つサービス
struct NoteService<R> {
    repo: R,
}

// where 句: 「R が NoteRepository と Debug を実装しているなら」使えるメソッド
impl<R> NoteService<R>
where
    R: NoteRepository + Debug,
{
    fn new(repo: R) -> Self {
        Self { repo }
    }

    // 戻り値の impl Iterator: 「何かのイテレータ」を返す（具体的な型名は隠す）
    fn search(&self, word: &str) -> impl Iterator<Item = String> {
        self.repo
            .titles()
            .into_iter()
            .filter(move |t| t.contains(word))
    }

    fn debug_repo(&self) -> String {
        format!("{:?}", self.repo)
    }
}

fn main() {
    let repo = InMemory(vec![
        "牛乳を買う".to_string(),
        "本を返す".to_string(),
        "牛乳パックを捨てる".to_string(),
    ]);
    let service = NoteService::new(repo);
    for title in service.search("牛乳") {
        println!("見つかった: {title}");
    }
    println!("{}", service.debug_repo());
}
```

```powershell
cargo run --bin u03_service
```

**▼ 実行結果**

```text
   Compiling design_lab v0.1.0 (...\design_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.60s
     Running `target\debug\u03_service.exe`
見つかった: 牛乳を買う
見つかった: 牛乳パックを捨てる
InMemory(["牛乳を買う", "本を返す", "牛乳パックを捨てる"])
```

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

```rust
// T に何の条件も付けずに、大小比較しようとする
fn largest<T: Copy>(items: &[T]) -> Option<T> {
    let mut iter = items.iter().copied();
    let mut max = iter.next()?;
    for item in iter {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

fn main() {
    println!("{:?}", largest(&[3, 9, 4]));
}
```

```text
   Compiling design_lab v0.1.0 (...\design_lab)
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src\bin\u03_err_e0369.rs:6:17
  |
6 |         if item > max {
  |            ---- ^ --- T
  |            |
  |            T
  |
help: consider further restricting type parameter `T` with trait `PartialOrd`
  |
2 | fn largest<T: Copy + std::cmp::PartialOrd>(items: &[T]) -> Option<T> {
  |                    ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `design_lab` (bin "u03_err_e0369") due to 1 previous error
```

- **何が起きたか:** `binary operation > cannot be applied to type T`（`T` に `>` は使えない）。`T` は `Copy` としか約束していないので、比べられるかどうか分かりません。
- **直し方:** help のとおり、`T` に `PartialOrd` を足します（`std::cmp::PartialOrd` は `PartialOrd` の正式な名前で、`PartialOrd` とだけ書いても同じです）。generics の関数では、「中で使う操作は、すべて境界で要求する」と覚えましょう。

#### エラー2: E0277 境界を満たさない型で呼んだ

今度は関数の側は正しく、呼ぶ側が `String` の並びを渡したケースです。

```rust
fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let mut iter = items.iter().copied();
    let mut max = iter.next()?;
    for item in iter {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

fn main() {
    let titles = vec!["牛乳".to_string(), "本".to_string()];
    // String は Copy ではない
    println!("{:?}", largest(&titles));
}
```

```text
   Compiling design_lab v0.1.0 (...\design_lab)
error[E0277]: the trait bound `String: Copy` is not satisfied
  --> src\bin\u03_err_copy.rs:15:30
   |
15 |     println!("{:?}", largest(&titles));
   |                      ------- ^^^^^^^ the trait `Copy` is not implemented for `String`
   |                      |
   |                      required by a bound introduced by this call
   |
note: required by a bound in `largest`
  --> src\bin\u03_err_copy.rs:1:28
   |
 1 | fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
   |                            ^^^^ required by this bound in `largest`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `design_lab` (bin "u03_err_copy") due to 1 previous error
```

- **何が起きたか:** `String` は中身をヒープに持つので `Copy` ではありません（第3巻）。`largest` の `Copy` という境界を満たさないため、`E0277` になりました。
- **直し方:** 関数の側を「コピーしないで、いちばん大きい要素への**参照**を返す」形に変えると、`String` でも使えるようになります（練習問題2）。境界は**必要最小限**にしておくと、使える型が広がります。

#### エラー3: E0282 / E0284 型を推論できない

```rust
fn main() {
    // 何の型の Vec か、どこにも手がかりがない
    let notes = Vec::new();
    println!("{}", notes.len());
}
```

```text
   Compiling design_lab v0.1.0 (...\design_lab)
error[E0282]: type annotations needed for `Vec<_>`
 --> src\bin\u03_err_e0282.rs:3:9
  |
3 |     let notes = Vec::new();
  |         ^^^^^   ---------- type must be known at this point
  |
help: consider giving `notes` an explicit type, where the type for type parameter `T` is specified
  |
3 |     let notes: Vec<T> = Vec::new();
  |              ++++++++

For more information about this error, try `rustc --explain E0282`.
error: could not compile `design_lab` (bin "u03_err_e0282") due to 1 previous error
```

- **何が起きたか:** `Vec::new()` は `Vec<T>` を作りますが、`T` が何かの手がかりがどこにもありません（何も `push` していない）。そこで `type annotations needed`（型の注釈が必要）になりました。
- **直し方:** `let notes: Vec<String> = Vec::new();` のように型を書きます。後で `notes.push(String::from("..."))` のような行があれば、コンパイラはそこから推論してくれます。

文字列を数値に変える `parse` でも、似たエラーがよく出ます。

```rust
fn main() {
    // 何の型として読むのか書いていない
    let page = "3".parse().unwrap();
    println!("{page}");
}
```

```text
   Compiling design_lab v0.1.0 (...\design_lab)
error[E0284]: type annotations needed
 --> src\bin\u03_err_e0284.rs:3:9
  |
3 |     let page = "3".parse().unwrap();
  |         ^^^^       ----- type must be known at this point
  |
  = note: cannot satisfy `<_ as FromStr>::Err == _`
help: consider giving `page` an explicit type
  |
3 |     let page: /* Type */ = "3".parse().unwrap();
  |             ++++++++++++

For more information about this error, try `rustc --explain E0284`.
error: could not compile `design_lab` (bin "u03_err_e0284") due to 1 previous error
```

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

```rust
// == で比べられる型なら何でも数えられる
fn count_matching<T: PartialEq>(items: &[T], target: &T) -> usize {
    items.iter().filter(|x| *x == target).count()
}

fn main() {
    println!("{}", count_matching(&[1, 2, 1, 3, 1], &1));
    println!("{}", count_matching(&["済", "未", "済"], &"未"));
}
```

**▼ 実行結果**

```text
3
1
```

- `filter(|x| *x == target)` の `x` は `&&T`、`target` は `&T` です。`*x` で1段はがして `&T` 同士を比べています。`PartialEq` だけを要求しているので、`Copy` でない `String` の並びでも使えます。

**解答2:**

```rust
// Copy を求めず、いちばん大きい要素への「参照」を返す
fn largest_ref<T>(items: &[T]) -> Option<&T>
where
    T: PartialOrd,
{
    let mut iter = items.iter();
    let mut max = iter.next()?;
    for item in iter {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

fn main() {
    let titles = vec!["牛乳".to_string(), "本".to_string(), "電気代".to_string()];
    println!("{:?}", largest_ref(&titles));
    println!("{:?}", largest_ref(&[3, 9, 4]));
}
```

**▼ 実行結果**

```text
Some("電気代")
Some(9)
```

- 戻り値を `Option<&T>` にしたので、要素をコピーせずに済み、`Copy` の境界が不要になりました。
- `"電気代"` が最大になったのは、文字列の比較が「文字コード（Unicode の番号）の順」だからです。「電」の番号が「牛」や「本」より大きいためで、五十音順ではない点に注意してください。

**解答3:**

- **3種類**です。単相化は「使われた**型**ごとに」1つ作ります。同じ `i32` で何回呼んでも、`largest::<i32>` は1つだけで、それが使い回されます。

### まとめ

- generics の `T` は「型の空欄」で、trait 境界（`T: PartialOrd`）は空欄に入れてよい型の条件です。中で使う操作は境界で要求します。
- 条件が多いときは `where` 句、条件付きでメソッドを足すときは `impl<T: 条件> 型<T>` を使います。
- generics は単相化で型ごとの専用関数になり（静的ディスパッチ）、速い代わりに機械語が増えます。

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

```rust
use std::mem::size_of;

// ノートを書き出す「形式」の約束
trait Exporter {
    fn name(&self) -> &str;
    fn export(&self, title: &str, body: &str) -> String;
}

struct PlainText;

struct Markdown {
    heading_level: usize,
}

struct Csv;

impl Exporter for PlainText {
    fn name(&self) -> &str {
        "テキスト"
    }

    fn export(&self, title: &str, body: &str) -> String {
        format!("{title}\n{body}")
    }
}

impl Exporter for Markdown {
    fn name(&self) -> &str {
        "Markdown"
    }

    fn export(&self, title: &str, body: &str) -> String {
        let hashes = "#".repeat(self.heading_level);
        format!("{hashes} {title}\n\n{body}")
    }
}

impl Exporter for Csv {
    fn name(&self) -> &str {
        "CSV"
    }

    fn export(&self, title: &str, body: &str) -> String {
        format!("\"{title}\",\"{body}\"")
    }
}

fn main() {
    // 型の違う 3 つの値を、1 つの Vec に入れる
    let exporters: Vec<Box<dyn Exporter>> = vec![
        Box::new(PlainText),
        Box::new(Markdown { heading_level: 2 }),
        Box::new(Csv),
    ];

    for e in &exporters {
        println!("--- {} ---", e.name());
        println!("{}", e.export("牛乳を買う", "低脂肪のもの"));
    }

    // ポインタの大きさを比べる（64 ビット環境）
    println!("&Markdown の大きさ: {} バイト", size_of::<&Markdown>());
    println!(
        "&dyn Exporter の大きさ: {} バイト",
        size_of::<&dyn Exporter>()
    );
    println!(
        "Box<dyn Exporter> の大きさ: {} バイト",
        size_of::<Box<dyn Exporter>>()
    );
}
```

```powershell
cargo run --bin u04_dyn
```

**▼ 実行結果**

```text
   Compiling design_lab v0.1.0 (...\design_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.85s
     Running `target\debug\u04_dyn.exe`
--- テキスト ---
牛乳を買う
低脂肪のもの
--- Markdown ---
## 牛乳を買う

低脂肪のもの
--- CSV ---
"牛乳を買う","低脂肪のもの"
&Markdown の大きさ: 8 バイト
&dyn Exporter の大きさ: 16 バイト
Box<dyn Exporter> の大きさ: 16 バイト
```

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

```rust
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(i64);

// &self で使え、スレッドをまたいで共有できる保存先の約束
trait NoteRepository {
    fn insert(&self, owner: UserId, title: &str) -> Result<i64, String>;
    fn count(&self) -> usize;
    fn kind(&self) -> &'static str;
}

// ---- メモリ版 ----
#[derive(Default)]
struct InMemoryNoteRepository {
    titles: Mutex<Vec<(UserId, String)>>,
}

impl NoteRepository for InMemoryNoteRepository {
    fn insert(&self, owner: UserId, title: &str) -> Result<i64, String> {
        let mut titles = self.titles.lock().unwrap();
        titles.push((owner, title.to_string()));
        Ok(titles.len() as i64)
    }

    fn count(&self) -> usize {
        self.titles.lock().unwrap().len()
    }

    fn kind(&self) -> &'static str {
        "memory"
    }
}

// ---- 読み取り専用版（メンテナンス中を想定）----
struct ReadOnlyRepository;

impl NoteRepository for ReadOnlyRepository {
    fn insert(&self, _owner: UserId, _title: &str) -> Result<i64, String> {
        Err("ただいまメンテナンス中のため保存できません".to_string())
    }

    fn count(&self) -> usize {
        0
    }

    fn kind(&self) -> &'static str {
        "read-only"
    }
}

// 共有する型に名前を付けておく
type DynRepo = Arc<dyn NoteRepository + Send + Sync>;

// アプリ全体で共有する状態
#[derive(Clone)]
struct AppState {
    repo: DynRepo,
}

// 設定の文字列を見て、実行時に実装を選ぶ
fn build_repo(kind: &str) -> Result<DynRepo, String> {
    match kind {
        "memory" => Ok(Arc::new(InMemoryNoteRepository::default())),
        "read-only" => Ok(Arc::new(ReadOnlyRepository)),
        other => Err(format!("知らない保存先です: {other}")),
    }
}

fn main() {
    for kind in ["memory", "read-only", "postgres"] {
        let repo = match build_repo(kind) {
            Ok(repo) => repo,
            Err(e) => {
                println!("起動失敗: {e}");
                continue;
            }
        };
        let state = AppState { repo };

        // 2 つのスレッドで同じ状態を使う（Web サーバーの同時リクエストの代わり）
        let mut handles = Vec::new();
        for user in 1..=2 {
            let state = state.clone();
            handles.push(thread::spawn(move || {
                state.repo.insert(UserId(user), "牛乳を買う")
            }));
        }
        for h in handles {
            let result = h.join().unwrap();
            println!("[{}] {:?}", state.repo.kind(), result);
        }
        println!("[{}] 件数 = {}", state.repo.kind(), state.repo.count());
    }
}
```

```powershell
cargo run --bin u04_state
```

**▼ 実行結果**

```text
   Compiling design_lab v0.1.0 (...\design_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.70s
     Running `target\debug\u04_state.exe`
[memory] Ok(2)
[memory] Ok(1)
[memory] 件数 = 2
[read-only] Err("ただいまメンテナンス中のため保存できません")
[read-only] Err("ただいまメンテナンス中のため保存できません")
[read-only] 件数 = 0
起動失敗: 知らない保存先です: postgres
```

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

```rust
use std::fmt::Display;

trait Exporter {
    fn name(&self) -> &str;
    // generics のメソッドを持っている
    fn export_all<T: Display>(&self, items: &[T]) -> String;
}

struct PlainText;

impl Exporter for PlainText {
    fn name(&self) -> &str {
        "テキスト"
    }

    fn export_all<T: Display>(&self, items: &[T]) -> String {
        let lines: Vec<String> = items.iter().map(|x| x.to_string()).collect();
        lines.join("\n")
    }
}

fn main() {
    let e: Box<dyn Exporter> = Box::new(PlainText);
    println!("{}", e.name());
}
```

```text
   Compiling design_lab v0.1.0 (...\design_lab)
error[E0038]: the trait `Exporter` is not dyn compatible
  --> src\bin\u04_err_e0038.rs:23:20
   |
23 |     let e: Box<dyn Exporter> = Box::new(PlainText);
   |                    ^^^^^^^^ `Exporter` is not dyn compatible
   |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
  --> src\bin\u04_err_e0038.rs:6:8
   |
 3 | trait Exporter {
   |       -------- this trait is not dyn compatible...
...
 6 |     fn export_all<T: Display>(&self, items: &[T]) -> String;
   |        ^^^^^^^^^^ ...because method `export_all` has generic type parameters
   = help: consider moving `export_all` to another trait
   = help: only type `PlainText` implements `Exporter`; consider using it directly instead.

For more information about this error, try `rustc --explain E0038`.
error: could not compile `design_lab` (bin "u04_err_e0038") due to 1 previous error
```

- **何が起きたか:** `the trait Exporter is not dyn compatible`（`Exporter` は dyn で使える条件を満たしていない）。理由は `method export_all has generic type parameters`（`export_all` が型パラメータを持っている）です。
- **なぜダメなのか:** vtable は「メソッド1つにつき関数1つ」の表です。しかし generics のメソッドは、単相化で `T` ごとに無数の関数になりえます。表に何を書けばよいか決められないので、vtable を作れません（`it needs to allow building a vtable`）。
- **直し方:** ①`export_all` を `&[&dyn Display]` のような dyn を使う引数に変える、②`export_all` を別の trait に移す（help の提案）、③そのメソッドに `where Self: Sized` を付けて「dyn からは呼ばない」と宣言する（エラー2の直し方と同じ）、のどれかです。

> **用語:** dyn compatible（ダイン・コンパチブル：`dyn` で使える条件を満たしていること。以前は「オブジェクト安全（object safe）」と呼ばれていて、古い資料や記事ではこちらの名前で出てきます）

#### エラー2: E0038 戻り値に `Self` を使っている

```rust
trait Exporter {
    fn name(&self) -> &str;
    // 自分と同じ型の新しい値を返す
    fn duplicate(&self) -> Self;
}

struct Csv;

impl Exporter for Csv {
    fn name(&self) -> &str {
        "CSV"
    }

    fn duplicate(&self) -> Self {
        Csv
    }
}

fn main() {
    let e: Box<dyn Exporter> = Box::new(Csv);
    println!("{}", e.name());
}
```

```text
   Compiling design_lab v0.1.0 (...\design_lab)
error[E0038]: the trait `Exporter` is not dyn compatible
  --> src\bin\u04_err_self.rs:20:20
   |
20 |     let e: Box<dyn Exporter> = Box::new(Csv);
   |                    ^^^^^^^^ `Exporter` is not dyn compatible
   |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
  --> src\bin\u04_err_self.rs:4:28
   |
 1 | trait Exporter {
   |       -------- this trait is not dyn compatible...
...
 4 |     fn duplicate(&self) -> Self;
   |                            ^^^^ ...because method `duplicate` references the `Self` type in its return type
   = help: consider moving `duplicate` to another trait
   = help: only type `Csv` implements `Exporter`; consider using it directly instead.

For more information about this error, try `rustc --explain E0038`.
error: could not compile `design_lab` (bin "u04_err_self") due to 1 previous error
```

- **何が起きたか:** `duplicate` が戻り値に `Self`（実装する型そのもの）を使っています。`dyn Exporter` 越しに呼んだとき、`Self` が `Csv` なのか `Markdown` なのか、返ってくる値の大きさが分かりません。
- **直し方:** そのメソッドに `where Self: Sized` を付けます。「このメソッドは、大きさが分かっている具体的な型のときだけ呼べる」という意味で、dyn 越しには呼べなくなる代わりに、trait 全体は dyn で使えるようになります（練習問題2）。
- 同じ理由で、標準の `Clone` trait（`fn clone(&self) -> Self`）も dyn compatible ではありません。`Box<dyn Clone>` は作れないので覚えておきましょう。

#### エラー3: E0782 `dyn` を書き忘れた

```rust
trait Exporter {
    fn name(&self) -> &str;
}

struct Csv;

impl Exporter for Csv {
    fn name(&self) -> &str {
        "CSV"
    }
}

fn main() {
    // dyn を書き忘れた
    let e: Box<Exporter> = Box::new(Csv);
    println!("{}", e.name());
}
```

```text
   Compiling design_lab v0.1.0 (...\design_lab)
error[E0782]: expected a type, found a trait
  --> src\bin\u04_err_e0782.rs:15:16
   |
15 |     let e: Box<Exporter> = Box::new(Csv);
   |                ^^^^^^^^
   |
help: you can add the `dyn` keyword if you want a trait object
   |
15 |     let e: Box<dyn Exporter> = Box::new(Csv);
   |                +++

For more information about this error, try `rustc --explain E0782`.
error: could not compile `design_lab` (bin "u04_err_e0782") due to 1 previous error
```

- **何が起きたか:** `expected a type, found a trait`（型が来るべき場所に trait が来た）。trait の名前だけでは型になりません。
- **直し方:** help のとおり `Box<dyn Exporter>` と書きます。古い Rust（edition 2015）では `dyn` を省略できたため、古い記事のコードを写すとこのエラーに出会います。

#### エラー4: E0277 スレッドで共有するのに Send / Sync が無い

```rust
use std::sync::Arc;
use std::thread;

trait NoteRepository {
    fn count(&self) -> usize;
}

struct InMemory;

impl NoteRepository for InMemory {
    fn count(&self) -> usize {
        0
    }
}

fn main() {
    // + Send + Sync を付け忘れた
    let repo: Arc<dyn NoteRepository> = Arc::new(InMemory);
    let r = Arc::clone(&repo);
    let handle = thread::spawn(move || r.count());
    println!("{}", handle.join().unwrap());
}
```

```text
   Compiling design_lab v0.1.0 (...\design_lab)
error[E0277]: `dyn NoteRepository` cannot be shared between threads safely
  --> src\bin\u04_err_send.rs:20:32
   |
20 |     let handle = thread::spawn(move || r.count());
   |                  ------------- ^^^^^^^^^^^^^^^^^ `dyn NoteRepository` cannot be shared between threads safely
   |                  |
   |                  required by a bound introduced by this call
   |
   = help: the trait `Sync` is not implemented for `dyn NoteRepository`
   = note: required for `Arc<dyn NoteRepository>` to implement `Send`
note: required because it's used within this closure
  --> src\bin\u04_err_send.rs:20:32
   |
20 |     let handle = thread::spawn(move || r.count());
   |                                ^^^^^^^
note: required by a bound in `spawn`
  --> /rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library\std\src\thread\functions.rs:125:0

error[E0277]: `dyn NoteRepository` cannot be sent between threads safely
  --> src\bin\u04_err_send.rs:20:32
   |
20 |     let handle = thread::spawn(move || r.count());
   |                  ------------- ^^^^^^^^^^^^^^^^^ `dyn NoteRepository` cannot be sent between threads safely
   |                  |
   |                  required by a bound introduced by this call
   |
   = help: the trait `Send` is not implemented for `dyn NoteRepository`
   = note: required for `Arc<dyn NoteRepository>` to implement `Send`
note: required because it's used within this closure
  --> src\bin\u04_err_send.rs:20:32
   |
20 |     let handle = thread::spawn(move || r.count());
   |                                ^^^^^^^
note: required by a bound in `spawn`
  --> /rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library\std\src\thread\functions.rs:125:0

For more information about this error, try `rustc --explain E0277`.
error: could not compile `design_lab` (bin "u04_err_send") due to 2 previous errors
```

- **何が起きたか:** `dyn NoteRepository cannot be shared between threads safely`（スレッド間で安全に共有できない）。`dyn NoteRepository` とだけ書くと、中身が `Send` / `Sync` かどうかをコンパイラは知りません。中身が `Rc` や `RefCell`（第3巻。スレッドをまたげない型）かもしれないからです。同じ原因で `Send` と `Sync` の2つのエラーが出ています。
- `note: required by a bound in spawn` の下に `/rustc/…/library\std\src\thread\functions.rs` という、自分のものではないファイルが出ていますが、これは「標準ライブラリの `thread::spawn` の定義がこの条件を要求している」という参考情報です。読み飛ばしてかまいません。
- **直し方:** `Arc<dyn NoteRepository + Send + Sync>` と書きます。こう書くと、`Send + Sync` でない実装を入れようとしたときに、**入れる場所で**コンパイルエラーになってくれます。

### RustNote ではここで使う

- RustNote の共有状態 `AppState` には、保存先を `Arc<dyn NoteRepository + Send + Sync>` の形で持たせる設計が使えます。本番は Turso 版、テストはメモリ版や失敗する Fake、と**起動時に差し替え**られます。
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

```rust
trait Exporter {
    fn name(&self) -> &str;
    fn export(&self, title: &str, body: &str) -> String;
}

struct PlainText;

struct Html;

impl Exporter for PlainText {
    fn name(&self) -> &str {
        "テキスト"
    }

    fn export(&self, title: &str, body: &str) -> String {
        format!("{title}\n{body}")
    }
}

// 新しい形式を足しても、使う側のループは変わらない
impl Exporter for Html {
    fn name(&self) -> &str {
        "HTML"
    }

    fn export(&self, title: &str, body: &str) -> String {
        format!("<h1>{title}</h1>\n<p>{body}</p>")
    }
}

fn main() {
    let exporters: Vec<Box<dyn Exporter>> = vec![Box::new(PlainText), Box::new(Html)];
    for e in &exporters {
        println!("--- {} ---", e.name());
        println!("{}", e.export("牛乳を買う", "低脂肪のもの"));
    }
}
```

**▼ 実行結果**

```text
--- テキスト ---
牛乳を買う
低脂肪のもの
--- HTML ---
<h1>牛乳を買う</h1>
<p>低脂肪のもの</p>
```

- 新しい型 `Html` に `impl Exporter for Html` を書いて `Vec` に入れただけで、ループは元のままです。「新しい種類を足すときに、使う側を直さなくてよい」のが trait オブジェクトの強みです。
- 本物のアプリで HTML を書き出すときは、タイトルや本文に含まれる `<` や `&` を変換（エスケープ）しないと、表示が崩れたり、悪意のあるスクリプトが動いたりします（XSS。第8巻）。ここでは仕組みの練習として省略しています。

**解答2:**

```rust
trait Exporter {
    fn name(&self) -> &str;

    // where Self: Sized を付けたメソッドは、dyn Exporter からは呼べない代わりに
    // trait 全体を dyn で使えるようにしてくれる
    fn duplicate(&self) -> Self
    where
        Self: Sized;
}

struct Csv;

impl Exporter for Csv {
    fn name(&self) -> &str {
        "CSV"
    }

    fn duplicate(&self) -> Self {
        Csv
    }
}

fn main() {
    let e: Box<dyn Exporter> = Box::new(Csv);
    println!("{}", e.name());

    // 具体的な型（Csv）のままなら duplicate も呼べる
    let original = Csv;
    let copy = original.duplicate();
    println!("{}", copy.name());
}
```

**▼ 実行結果**

```text
CSV
CSV
```

- `where Self: Sized` は「`Self` の大きさがコンパイル時に分かるときだけ」という条件です。`dyn Exporter` は大きさが分からない型なので、このメソッドは vtable に載らず、dyn 越しには呼べません。その代わり、trait 全体は dyn で使えるようになります。

**解答3:**

- (a) は **`dyn Trait`** が向いています。どの形式を使うかが実行時（利用者の選択）で決まり、`Vec<Box<dyn Exporter>>` や `match` で選んだ結果を同じ型として扱いたいからです。
- (b) は **generics** が向いています。使う型はコンパイル時に分かっていて、何万回も呼ぶ小さな処理なので、単相化による直接呼び出し（静的ディスパッチ）の速さが活きます。

### まとめ

- `Box<dyn Trait>` / `Arc<dyn Trait + Send + Sync>` で、違う型を同じ型として扱い、実行時に実装を選べます。
- `dyn Trait` は「データの場所＋vtable の場所」のファットポインタで、vtable を引いてメソッドを呼びます（動的ディスパッチ）。
- generics のメソッドや `Self` を返すメソッドがあると dyn で使えません（E0038）。`where Self: Sized` か、trait を分けて直します。

## UNIT 05: エラー型・thiserror・文脈（anyhow との使い分け）

### この単元のゴール

- エラーを「文字列」ではなく **enum で分類**し、`Display` と `std::error::Error` を実装できる
- `thiserror` 2 系で同じことを短く書き、`#[from]` と `?` で層をまたいだ変換ができる
- アプリ全体のエラー `AppError` を作り、HTTP の 400 / 404 / 409 / 500 に対応づけられる。500 のときは中身を利用者に見せない
- `anyhow` で「文脈（何をしようとして失敗したか）」を積み重ね、thiserror との使い分けを説明できる

### たとえ話でつかむ

郵便物が届けられなかったとき、郵便局は封筒に「**あて所に尋ねあたりません**」「**料金不足**」「**受取拒否**」のような理由のハンコを押して返します。理由が分類されているから、送った人は「住所を直す」「切手を足す」と、**理由ごとに次の行動を決められます**。もし全部「届きませんでした」とだけ書いてあったら、どうしようもありません。

エラーの enum は、この「返送理由のハンコ」です。そして、窓口の係員は、局の中の細かい事情（「仕分け機が故障した」）をお客さんにそのまま言わず、「申し訳ありません、時間をおいてお越しください」と**言い換えます**。一方、局内の報告書には故障の詳細を書きます。

- 中の層（ドメイン・DB）: 細かい理由を正確に分類する（`TitleError`、`RepoError`）
- 真ん中の層（アプリ）: 利用者への対応の種類で分類し直す（`AppError`）
- 外の層（HTTP）: ステータスコードと優しい文章に言い換える。詳細はログへ

> **用語:** ステータスコード（HTTP の返事に付ける3桁の番号。400 = 送られた内容が正しくない、404 = 見つからない、409 = 状態が食い違って処理できない、500 = サーバーの中の故障。第6巻で詳しく扱う）

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-10" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="110" y="24" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">ドメイン・保存先の層</text>
<text x="360" y="24" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">アプリの層</text>
<text x="610" y="24" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">HTTP の層</text>
<rect x="20" y="40" width="180" height="40" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="110" y="65" font-size="13" fill="#047857" text-anchor="middle">TitleError</text>
<rect x="20" y="95" width="180" height="40" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="110" y="120" font-size="13" fill="#047857" text-anchor="middle">EmailError</text>
<rect x="20" y="152" width="180" height="88" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="110" y="174" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">RepoError</text>
<text x="110" y="194" font-size="12" fill="#475569" text-anchor="middle">UniqueViolation</text>
<text x="110" y="212" font-size="12" fill="#475569" text-anchor="middle">VersionMismatch</text>
<text x="110" y="230" font-size="12" fill="#475569" text-anchor="middle">Connection</text>
<rect x="280" y="40" width="160" height="200" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="60" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">AppError</text>
<text x="360" y="90" font-size="12" fill="#1e40af" text-anchor="middle">InvalidTitle / Email</text>
<text x="360" y="130" font-size="12" fill="#1e40af" text-anchor="middle">NotFound</text>
<text x="360" y="170" font-size="12" fill="#1e40af" text-anchor="middle">EmailTaken / Conflict</text>
<text x="360" y="210" font-size="12" fill="#1e40af" text-anchor="middle">Internal</text>
<line x1="200" y1="60" x2="276" y2="83" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-10)"/>
<line x1="200" y1="115" x2="276" y2="90" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-10)"/>
<line x1="200" y1="190" x2="276" y2="168" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-10)"/>
<line x1="200" y1="225" x2="276" y2="208" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-10)"/>
<text x="238" y="146" font-size="12" fill="#475569" text-anchor="middle">? で</text>
<text x="238" y="162" font-size="12" fill="#475569" text-anchor="middle">変換</text>
<rect x="520" y="70" width="180" height="30" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="610" y="90" font-size="12" fill="#c2410c" text-anchor="middle">400 Bad Request</text>
<rect x="520" y="110" width="180" height="30" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="610" y="130" font-size="12" fill="#c2410c" text-anchor="middle">404 Not Found</text>
<rect x="520" y="150" width="180" height="30" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="610" y="170" font-size="12" fill="#c2410c" text-anchor="middle">409 Conflict</text>
<rect x="520" y="190" width="180" height="30" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="610" y="210" font-size="12" fill="#c2410c" text-anchor="middle">500 Internal Server Error</text>
<line x1="440" y1="85" x2="516" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-10)"/>
<line x1="440" y1="125" x2="516" y2="125" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-10)"/>
<line x1="440" y1="165" x2="516" y2="165" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-10)"/>
<line x1="440" y1="205" x2="516" y2="205" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-10)"/>
<text x="478" y="58" font-size="12" fill="#475569" text-anchor="middle">status_code()</text>
<text x="360" y="272" font-size="12" fill="#475569" text-anchor="middle">画面には言い換えた文章（500 は詳細を隠す）、ログには source() で原因の連鎖を書く</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-10: エラーの層。中の層の細かい失敗を、アプリの分類を経て HTTP の返事に変える</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-04-11" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="10" width="340" height="230" rx="10" fill="#eff6ff" stroke="#3b82f6"/>
<text x="180" y="36" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">thiserror: 種類を見分けたい</text>
<rect x="40" y="52" width="280" height="96" rx="8" fill="#ffffff" stroke="#3b82f6"/>
<text x="180" y="74" font-size="12" fill="#0f172a" text-anchor="middle">enum AppError {</text>
<text x="180" y="94" font-size="12" fill="#0f172a" text-anchor="middle">NotFound, Conflict, Internal(..)</text>
<text x="180" y="114" font-size="12" fill="#0f172a" text-anchor="middle">}</text>
<text x="180" y="136" font-size="12" fill="#1e40af" text-anchor="middle">呼ぶ側が match で対応を変える</text>
<text x="180" y="180" font-size="12" fill="#1e40af" text-anchor="middle">向く場所: ドメイン型・ライブラリ・</text>
<text x="180" y="200" font-size="12" fill="#1e40af" text-anchor="middle">HTTP に変換する AppError</text>
<rect x="370" y="10" width="340" height="230" rx="10" fill="#fff7ed" stroke="#f97316"/>
<text x="540" y="36" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">anyhow: 報告できればよい</text>
<rect x="395" y="52" width="290" height="32" rx="6" fill="#ffffff" stroke="#f97316"/>
<text x="540" y="73" font-size="12" fill="#0f172a" text-anchor="middle">シードの投入に失敗しました</text>
<rect x="395" y="96" width="290" height="32" rx="6" fill="#ffffff" stroke="#f97316"/>
<text x="540" y="117" font-size="12" fill="#0f172a" text-anchor="middle">シードファイル … を読めませんでした</text>
<rect x="395" y="140" width="290" height="32" rx="6" fill="#ffffff" stroke="#f97316"/>
<text x="540" y="161" font-size="12" fill="#0f172a" text-anchor="middle">ファイルが見つかりません (os error 2)</text>
<line x1="540" y1="140" x2="540" y2="132" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-11)"/>
<line x1="540" y1="96" x2="540" y2="88" stroke="#475569" stroke-width="2" marker-end="url(#ar-04-11)"/>
<text x="540" y="198" font-size="12" fill="#c2410c" text-anchor="middle">下から上へ「文脈」を積んで、まとめて表示</text>
<text x="540" y="220" font-size="12" fill="#c2410c" text-anchor="middle">向く場所: main・スクリプト・起動処理</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-11: thiserror は「分類するため」、anyhow は「何が起きたかを人に伝えるため」</div>
</div>

### まず動かす最小コード

まずはライブラリを使わずに、エラー型を手で書いてみます。thiserror が裏で何をしてくれるのかが分かるようになります。

#### 例5-1: TitleError に Display と Error を実装する

> **▼ このコードがやること（先に日本語で）:** タイトルの検査で起きる失敗を2種類に分けた型を作り、①利用者に見せる文章と、②開発者が調べるための表示の2通りで出力します。最後に、標準の「どんなエラーでも入る箱」にも入れられることを確かめます。

```rust
use std::error::Error;
use std::fmt;

// タイトルが不正だった理由
#[derive(Debug, PartialEq, Eq)]
enum TitleError {
    Empty,
    TooLong { len: usize, max: usize },
}

// 利用者に見せる文章（{} で表示したとき）
impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TitleError::Empty => write!(f, "タイトルを入力してください"),
            TitleError::TooLong { len, max } => {
                write!(f, "タイトルは{max}文字以内にしてください（現在{len}文字）")
            }
        }
    }
}

// 「これはエラーを表す型です」という印。中身は既定の実装で足りる
impl Error for TitleError {}

fn parse_title(input: &str) -> Result<String, TitleError> {
    let trimmed = input.trim();
    let len = trimmed.chars().count();
    if len == 0 {
        return Err(TitleError::Empty);
    }
    if len > 80 {
        return Err(TitleError::TooLong { len, max: 80 });
    }
    Ok(trimmed.to_string())
}

fn main() {
    for input in ["買い物", "", &"あ".repeat(81)] {
        match parse_title(input) {
            Ok(title) => println!("OK: {title}"),
            Err(e) => {
                println!("利用者向け(Display): {e}");
                println!("開発者向け(Debug):   {e:?}");
            }
        }
    }

    // Error を実装したので、Box<dyn Error> にも入れられる
    let boxed: Box<dyn Error> = Box::new(TitleError::Empty);
    println!("Box<dyn Error>: {boxed}");
}
```

```powershell
cargo run --bin u05_manual
```

**▼ 実行結果**

```text
   Compiling design_lab v0.1.0 (...\design_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.88s
     Running `target\debug\u05_manual.exe`
OK: 買い物
利用者向け(Display): タイトルを入力してください
開発者向け(Debug):   Empty
利用者向け(Display): タイトルは80文字以内にしてください（現在81文字）
開発者向け(Debug):   TooLong { len: 81, max: 80 }
Box<dyn Error>: タイトルを入力してください
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: Display は「人に見せる文章」、Debug は「開発者向け」

```rust
impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TitleError::Empty => write!(f, "タイトルを入力してください"),
```

- `Display` を実装すると、`{}` で表示したときの文章を決められます。エラーの `Display` は「何が悪かったか」を1文で、文末に句点を付けずに書くのが Rust の慣習です（複数のエラーをつないで表示するときに読みやすいため）。
- `#[derive(Debug)]` で作られる `{:?}` の表示（`TooLong { len: 81, max: 80 }`）は、**開発者が中身を正確に知るため**のものです。利用者の画面には出しません。

##### 解説2: `impl Error for TitleError {}`

```rust
impl Error for TitleError {}
```

- `std::error::Error` は「これはエラーを表す型です」という標準の trait です。`Debug` と `Display` を実装していることが条件で、メソッドはすべて既定の実装があるので、中身は空で構いません。
- 実装しておくと、`Box<dyn Error>`（どんなエラーでも入る箱。UNIT 04 の trait オブジェクト）に入れたり、後で見る `source()` で「原因のエラー」をたどったりできるようになります。

##### 解説3: 手で書くのは少し面倒

- エラーの種類が増えるたびに、`Display` の `match` にも1行ずつ足す必要があります。種類と文章が離れた場所に書かれるので、読みにくくもなります。
- これを `#[derive]` のように自動化してくれるのが **thiserror** です。

### もう一歩: 実務に近い形

#### 例5-2: thiserror で AppError を作り、HTTP の返事に変換する

> **▼ このコードがやること（先に日本語で）:** 入力の検査の失敗（タイトル・メール）、保存先の失敗（重複・版の食い違い・接続できない）、アプリとしての分類（見つからない・登録済み・競合・内部エラー）を、3つの層のエラー型に分けて書きます。「ノートの名前を変える」「登録する」という2つの処理でいろいろな失敗を起こし、それぞれが何番のステータスコードと、どんな文章になるかを表示します。

thiserror は「はじめに」で `cargo add thiserror` 済みです（版は 2.0.21）。

```rust
use thiserror::Error;

// ---- ドメインの層: 入力の検査で起きる失敗 ----
#[derive(Debug, Error, PartialEq, Eq)]
enum TitleError {
    #[error("タイトルを入力してください")]
    Empty,
    #[error("タイトルは{max}文字以内にしてください（現在{len}文字）")]
    TooLong { len: usize, max: usize },
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("メールアドレスの形式が正しくありません")]
struct EmailError;

// ---- 保存先（DB）の層: 保存で起きる失敗 ----
#[derive(Debug, Error)]
enum RepoError {
    #[error("一意制約に違反しました（列: {0}）")]
    UniqueViolation(String),
    #[error("version が一致しません")]
    VersionMismatch,
    #[error("DB に接続できません")]
    Connection(#[source] std::io::Error),
}

// ---- アプリの層: 利用者への返事を決めるための分類 ----
#[derive(Debug, Error)]
enum AppError {
    #[error(transparent)]
    InvalidTitle(#[from] TitleError),
    #[error(transparent)]
    InvalidEmail(#[from] EmailError),
    #[error("ノートが見つかりません")]
    NotFound,
    #[error("そのメールアドレスは既に登録されています")]
    EmailTaken,
    #[error("ほかの画面で先に更新されました。読み込み直してください")]
    Conflict,
    #[error("保存先でエラーが起きました")]
    Internal(#[source] RepoError),
}

// 保存先の失敗を、アプリの分類へ変換する
impl From<RepoError> for AppError {
    fn from(e: RepoError) -> Self {
        match e {
            RepoError::VersionMismatch => AppError::Conflict,
            other => AppError::Internal(other),
        }
    }
}

impl AppError {
    // HTTP のステータスコード
    fn status_code(&self) -> u16 {
        match self {
            AppError::InvalidTitle(_) | AppError::InvalidEmail(_) => 400,
            AppError::NotFound => 404,
            AppError::EmailTaken | AppError::Conflict => 409,
            AppError::Internal(_) => 500,
        }
    }

    // 画面に出してよい文章（500 のときは中身を隠す）
    fn public_message(&self) -> String {
        match self {
            AppError::Internal(_) => "時間をおいてもう一度お試しください".to_string(),
            other => other.to_string(),
        }
    }
}

fn parse_title(input: &str) -> Result<String, TitleError> {
    let len = input.trim().chars().count();
    if len == 0 {
        return Err(TitleError::Empty);
    }
    if len > 80 {
        return Err(TitleError::TooLong { len, max: 80 });
    }
    Ok(input.trim().to_string())
}

fn parse_email(input: &str) -> Result<String, EmailError> {
    let email = input.trim().to_lowercase();
    match email.split_once('@') {
        Some((local, domain)) if !local.is_empty() && domain.contains('.') => Ok(email),
        _ => Err(EmailError),
    }
}

// ユースケース1: ノートのタイトルを変える（db は保存の結果のつもり）
fn rename_note(input: &str, found: bool, db: Result<(), RepoError>) -> Result<String, AppError> {
    // TitleError は #[from] のおかげで ? だけで AppError になる
    let title = parse_title(input)?;
    if !found {
        return Err(AppError::NotFound);
    }
    // RepoError は impl From のおかげで ? だけで AppError になる
    db?;
    Ok(title)
}

// ユースケース2: 登録する。一意制約違反だけは「登録済み」と言い換える
fn signup(input: &str, db: Result<(), RepoError>) -> Result<String, AppError> {
    let email = parse_email(input)?;
    db.map_err(|e| match e {
        RepoError::UniqueViolation(_) => AppError::EmailTaken,
        other => AppError::from(other),
    })?;
    Ok(email)
}

// HTTP の層の代わり: 結果を「画面に返すもの」と「ログに書くもの」に分ける
fn respond(result: Result<String, AppError>) {
    match result {
        Ok(value) => println!("200 OK: {value}"),
        Err(e) => {
            println!("{} 画面: {}", e.status_code(), e.public_message());
            // ログには原因までたどって書く
            let mut cause = std::error::Error::source(&e);
            while let Some(c) = cause {
                println!("    ログ（原因）: {c}");
                cause = c.source();
            }
        }
    }
}

fn main() {
    let down = || {
        std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            "接続を拒否されました",
        )
    };

    respond(rename_note("牛乳を買う", true, Ok(())));
    respond(rename_note("   ", true, Ok(())));
    respond(rename_note("牛乳を買う", false, Ok(())));
    respond(rename_note(
        "牛乳を買う",
        true,
        Err(RepoError::VersionMismatch),
    ));
    respond(rename_note(
        "牛乳を買う",
        true,
        Err(RepoError::Connection(down())),
    ));
    respond(signup("alice@example", Ok(())));
    respond(signup(
        "alice@example.com",
        Err(RepoError::UniqueViolation("email".to_string())),
    ));
}
```

```powershell
cargo run --bin u05_app_error
```

**▼ 実行結果**

```text
   Compiling design_lab v0.1.0 (...\design_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.83s
     Running `target\debug\u05_app_error.exe`
200 OK: 牛乳を買う
400 画面: タイトルを入力してください
404 画面: ノートが見つかりません
409 画面: ほかの画面で先に更新されました。読み込み直してください
500 画面: 時間をおいてもう一度お試しください
    ログ（原因）: DB に接続できません
    ログ（原因）: 接続を拒否されました
400 画面: メールアドレスの形式が正しくありません
409 画面: そのメールアドレスは既に登録されています
```

##### 解説4: `#[derive(Error)]` と `#[error("...")]`

```rust
#[derive(Debug, Error, PartialEq, Eq)]
enum TitleError {
    #[error("タイトルを入力してください")]
    Empty,
    #[error("タイトルは{max}文字以内にしてください（現在{len}文字）")]
    TooLong { len: usize, max: usize },
}
```

- `#[derive(Error)]` が、例5-1 で手書きした `impl Display` と `impl Error` を自動で作ります。
- 各種類の上の `#[error("...")]` が `Display` の文章です。`{max}` `{len}` のように、**フィールドの名前を `{}` に書くと値が埋め込まれます**。タプルの形（`UniqueViolation(String)`）なら `{0}` で0番目のフィールドを埋め込みます。
- 種類と文章が隣り合うので、読みやすく、書き忘れも起きません。

##### 解説5: `#[from]` で `?` が変換してくれる

```rust
#[error(transparent)]
InvalidTitle(#[from] TitleError),
```

```rust
let title = parse_title(input)?;
```

- `#[from]` を付けると、thiserror が `impl From<TitleError> for AppError` を自動で作ります。
- `?` 演算子は、エラーを返すときに**自動で `From` による変換をする**ので、`Result<_, TitleError>` に `?` を付けるだけで `AppError::InvalidTitle(...)` になって返ります。
- `#[error(transparent)]` は「表示も原因も、中のエラーのものをそのまま使う」という意味です。だから 400 の画面には `TitleError` の文章「タイトルを入力してください」がそのまま出ています。

##### 解説6: 変換のルールを自分で書く `impl From`

```rust
impl From<RepoError> for AppError {
    fn from(e: RepoError) -> Self {
        match e {
            RepoError::VersionMismatch => AppError::Conflict,
            other => AppError::Internal(other),
        }
    }
}
```

- 保存先の失敗は、種類によって意味が違います。「version が一致しない」は、ほかの画面で先に更新されたという**利用者に伝えるべき競合（409）**です。それ以外の DB の失敗は、利用者にはどうしようもない**内部エラー（500）**です。
- このように、変換に判断が要るときは `#[from]` ではなく `impl From` を手で書きます。それでも `?` で使える点は同じです（`db?;`）。
- 登録の「一意制約違反」は、`signup` の中で `map_err` を使い、その場で `EmailTaken` に言い換えています。同じ `UniqueViolation` でも、どの列か・どの処理かで意味が変わるので、**意味が分かる場所（ユースケース）で変換する**のです。

> **用語:** 一意制約（UNIQUE：DB で「この列に同じ値を2つ入れてはいけない」という決まり。RustNote の `users.email` に付いている）

##### 解説7: 500 の詳細は画面に出さず、ログに出す

```rust
fn public_message(&self) -> String {
    match self {
        AppError::Internal(_) => "時間をおいてもう一度お試しください".to_string(),
        other => other.to_string(),
    }
}
```

```rust
let mut cause = std::error::Error::source(&e);
while let Some(c) = cause {
    println!("    ログ（原因）: {c}");
    cause = c.source();
}
```

- DB のエラーメッセージには、テーブル名やサーバーの場所など、**攻撃者に知られたくない情報**が含まれることがあります。だから 500 の画面には、決まった文章だけを出します。
- 一方で、開発者は原因を知る必要があります。`#[source]` を付けたフィールドは `source()` で取り出せるので、`while let` で原因の原因…とたどってログに書きます。実行結果では「DB に接続できません ← 接続を拒否されました」と2段の原因が出ています。
- この `respond` 関数は HTTP の層の代わりです。本物の Web アプリでは、`AppError` から HTTP のレスポンスを作る部分（第6巻・第10巻）でこの考え方を使います。

#### 例5-3: anyhow で文脈を積み重ねる

thiserror は「エラーの**種類**を作る」道具でした。一方、プログラムの入口（`main`）や、DB に最初のデータを入れるスクリプトのような場所では、エラーの種類で分岐することはほとんどありません。必要なのは「**何をしようとして、なぜ失敗したか**」を人間に分かるように表示して終わることです。そのための道具が **anyhow** です（版は 1.0.104）。

> **▼ このコードがやること（先に日本語で）:** 1行に1件のタイトルが書かれたテキストファイルを読み、ノートとして投入する（ふりをする）小さなツールです。ファイルが無い・中身が空、のときは、「何をしようとして失敗したか」を重ねた分かりやすいエラーを表示して終了します。

```rust
use anyhow::{Context, Result, bail};
use std::fs;

// シード（最初に入れておくノート）を、1 行 1 件のテキストファイルから読む
fn load_seed(path: &str) -> Result<Vec<String>> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("シードファイル {path} を読めませんでした"))?;

    let titles: Vec<String> = text
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    if titles.is_empty() {
        bail!("シードファイル {path} にノートが1件もありません");
    }
    Ok(titles)
}

// main が anyhow::Result を返すと、失敗したときに原因の連鎖を表示して終了する
fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "seed.txt".to_string());
    let titles = load_seed(&path).context("シードの投入に失敗しました")?;
    for title in &titles {
        println!("投入: {title}");
    }
    println!("{} 件のノートを投入しました", titles.len());
    Ok(())
}
```

`design_lab` フォルダに `seed.txt`（1行目「牛乳を買う」、2行目は空行、3行目「本を返す」）を作ってから実行します。`--` の後ろは、プログラムに渡す引数です。

```powershell
cargo run --bin u05_anyhow -- seed.txt
```

**▼ 実行結果（成功）**

```text
   Compiling design_lab v0.1.0 (...\design_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.77s
     Running `target\debug\u05_anyhow.exe seed.txt`
投入: 牛乳を買う
投入: 本を返す
2 件のノートを投入しました
```

存在しないファイルを指定すると、次のようになります。

```powershell
cargo run --bin u05_anyhow -- missing.txt
```

**▼ 実行結果（失敗）**

```text
   Compiling design_lab v0.1.0 (...\design_lab)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
     Running `target\debug\u05_anyhow.exe missing.txt`
Error: シードの投入に失敗しました

Caused by:
    0: シードファイル missing.txt を読めませんでした
    1: 指定されたファイルが見つかりません。 (os error 2)
error: process didn't exit successfully: `target\debug\u05_anyhow.exe missing.txt` (exit code: 1)
```

空白だけのファイル `empty.txt` を指定した場合（`-q` は cargo 自身の表示を省くオプション）:

```powershell
cargo run -q --bin u05_anyhow -- empty.txt
```

```text
Error: シードの投入に失敗しました

Caused by:
    シードファイル empty.txt にノートが1件もありません
```

##### 解説8: `anyhow::Result` と `?`

```rust
use anyhow::{Context, Result, bail};
```

- `anyhow::Result<T>` は `Result<T, anyhow::Error>` の別名です。`anyhow::Error` は、**`std::error::Error` を実装したどんなエラーでも**（`io::Error` でも自作の `TitleError` でも）`?` で受け取れる、何でも入る箱です。
- そのぶん、受け取った側は「どの種類のエラーか」を `match` で見分けにくくなります。だから、**種類で分岐したい場所には向きません**。

##### 解説9: `context` / `with_context` で文脈を足す

```rust
let text = fs::read_to_string(path)
    .with_context(|| format!("シードファイル {path} を読めませんでした"))?;
```

- 元のエラー「指定されたファイルが見つかりません。 (os error 2)」だけでは、**どのファイルを、何のために**読もうとしたのか分かりません。`with_context` で「シードファイル missing.txt を読めませんでした」という文脈を上に重ねます。
- `context("固定の文")` はいつも同じ文章のとき、`with_context(|| format!(...))` は文章を組み立てるときに使います。後者はクロージャなので、**失敗したときだけ** `format!` が実行され、成功したときの無駄がありません。
- `bail!("...")` は「この文章のエラーを作って、すぐ `return Err(...)` する」マクロです。

##### 解説10: `main` が `Result` を返すと何が起きるか

- `fn main() -> Result<()>` にすると、`main` の中で `?` が使えます。`Err` で終わると、Rust は `Error: ` に続けてエラーを表示し、**終了コード 1**（失敗）でプログラムを終えます。最後の行の `error: process didn't exit successfully ... (exit code: 1)` は、それを見た cargo の報告です。
- anyhow のエラーは、`Caused by:` の下に、積んだ文脈を上から順に番号付きで表示します（図4-11 の右側）。「シードの投入に失敗しました ← ファイルを読めませんでした ← ファイルが見つかりません」と、**外側の目的から内側の原因へ**読めます。

#### thiserror と anyhow の使い分け

| 場所 | 使うもの | 理由 |
|---|---|---|
| `NoteTitle::parse` などドメイン型の検査 | thiserror（または手書き） | 呼ぶ側が「空なのか長すぎるのか」で対応を変える |
| 保存先（Repository）の失敗 | thiserror | 「重複」「競合」「接続できない」を見分けて変換する |
| アプリ全体の `AppError` | thiserror | 種類ごとにステータスコードと文章を決める |
| `main`・起動処理・管理用スクリプト | anyhow | 種類で分岐せず、人に分かる文脈を付けて報告できればよい |

覚え方は「**プログラムが読むエラーは thiserror、人が読むエラーは anyhow**」です。両方を1つのプロジェクトで使うのは普通のことです。

### よくあるエラーと読み方

#### エラー1: E0277 `?` がエラーを変換できない

```rust
use thiserror::Error;

#[derive(Debug, Error)]
#[error("メールアドレスの形式が正しくありません")]
struct EmailError;

#[derive(Debug, Error)]
enum AppError {
    #[error("ノートが見つかりません")]
    NotFound,
}

fn parse_email(input: &str) -> Result<String, EmailError> {
    if input.contains('@') {
        Ok(input.to_string())
    } else {
        Err(EmailError)
    }
}

fn signup(input: &str) -> Result<String, AppError> {
    // EmailError を AppError に変換する方法が無い
    let email = parse_email(input)?;
    Ok(email)
}

fn main() {
    println!("{:?}", signup("alice@example.com"));
    println!("{:?}", AppError::NotFound);
}
```

```text
   Compiling design_lab v0.1.0 (...\design_lab)
error[E0277]: `?` couldn't convert the error to `AppError`
  --> src\bin\u05_err_from.rs:23:35
   |
21 | fn signup(input: &str) -> Result<String, AppError> {
   |                           ------------------------ expected `AppError` because of this
22 |     // EmailError を AppError に変換する方法が無い
23 |     let email = parse_email(input)?;
   |                 ------------------^ the trait `From<EmailError>` is not implemented for `AppError`
   |                 |
   |                 this can't be annotated with `?` because it has type `Result<_, EmailError>`
   |
note: `AppError` needs to implement `From<EmailError>`
  --> src\bin\u05_err_from.rs:8:1
   |
 8 | enum AppError {
   | ^^^^^^^^^^^^^
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait

For more information about this error, try `rustc --explain E0277`.
error: could not compile `design_lab` (bin "u05_err_from") due to 1 previous error
```

- **何が起きたか:** `signup` の戻り値のエラー型は `AppError` なのに、`parse_email(input)?` が返そうとしているのは `EmailError` です。`?` は `From` で変換しようとしますが、`From<EmailError> for AppError` がありません（`the trait From<EmailError> is not implemented for AppError`）。最後の `note` が「`?` は `From` trait で暗黙に変換する」と仕組みまで説明してくれています。
- **直し方:** `AppError` に `#[from] EmailError` を持つ種類を足すか（練習問題2）、その場で `.map_err(|_| AppError::...)?` と変換を書きます。

#### エラー2: 警告 unused `Result` ― 失敗を握りつぶしている

これはコンパイルエラーではなく**警告**（warning）なので、プログラムは動いてしまいます。だからこそ危険です。

```rust
fn save_note(title: &str) -> Result<(), String> {
    if title.is_empty() {
        return Err("タイトルが空です".to_string());
    }
    Ok(())
}

fn main() {
    // 結果を見ずに捨てている
    save_note("");
    println!("保存しました！");
}
```

```text
   Compiling design_lab v0.1.0 (...\design_lab)
warning: unused `Result` that must be used
  --> src\bin\u05_warn_unused.rs:10:5
   |
10 |     save_note("");
   |     ^^^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
   |
10 |     let _ = save_note("");
   |     +++++++

warning: `design_lab` (bin "u05_warn_unused") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
     Running `target\debug\u05_warn_unused.exe`
保存しました！
```

- **何が起きたか:** `save_note("")` は `Err` を返しているのに、結果を見ずに捨てたため、画面には「保存しました！」と**嘘の表示**が出ています。`Result` には「使わずに捨てたら警告する」という印（`#[must_use]`）が付いているので、コンパイラが `unused Result that must be used` と教えてくれました。
- **直し方:** `save_note("")?;` で呼び出し元に失敗を返すか、`match` / `if let Err(e) = ...` で処理します。help の `let _ = ...` は「わざと無視する」という意思表示で、**本当に無視してよいときだけ**使います。
- `cargo clippy -- -D warnings` のように実行すると、警告もエラーとして扱われ、ビルドが止まります。チーム開発や CI（第11巻・第12巻）ではこの設定がよく使われます。

### RustNote ではここで使う

RustNote の `AppError` は、この単元の `AppError` とほぼ同じ形で、次のように HTTP の返事に対応づけます。

| AppError の種類 | ステータス | RustNote で起きる場面 |
|---|---|---|
| 入力の検証エラー（タイトル・メールなど） | 400 | 空のタイトルで作成、形式の違うメールで登録 |
| `NotFound` | 404 | 存在しないノート、**他人のノート**（「ある」こと自体を隠すため 403 ではなく 404） |
| `EmailTaken` / `Conflict` | 409 | 登録済みのメール、ほかのタブで先に編集された（`notes.version` が合わない） |
| `Internal` | 500 | DB に接続できない、想定外の失敗。画面には詳細を出さず、ログに原因を書く |

フォームの検証エラーは、実際の画面では 400 を返す代わりに「入力欄の横にメッセージを出して、同じフォームを表示し直す」ことも多いです。その形は第10巻で扱います。`main` での起動時のエラー（設定が読めない・DB に接続できない）には anyhow を使います。

### 練習問題

**問題1:** `AppError` に「ログインしていない」を表す `Unauthorized`（文章「ログインしてください」、ステータス 401）を追加してください。`status_code` の `match` に書き足すまでコンパイルが通らないことも確かめてください。

**問題2:** 「よくあるエラー」のエラー1（E0277）を、thiserror の `#[from]` を使って直してください。`signup("alice")` が失敗したときの `{}` と `{:?}` の表示も確かめてください。

**問題3:** 次のエラーには thiserror と anyhow のどちらが向いているでしょうか。

- (a) `Email::parse` の失敗
- (b) RustNote の `main` で、起動時に DB へ接続できなかった
- (c) ノートの保存で起きた「version の食い違い」

### 解答と解説

**解答1:**

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("ログインしてください")]
    Unauthorized,
    #[error("ノートが見つかりません")]
    NotFound,
    #[error("ほかの画面で先に更新されました")]
    Conflict,
}

impl AppError {
    // 新しい種類を足すと、ここの match に書き足すまでコンパイルが通らない
    fn status_code(&self) -> u16 {
        match self {
            AppError::Unauthorized => 401,
            AppError::NotFound => 404,
            AppError::Conflict => 409,
        }
    }
}

fn main() {
    for e in [
        AppError::Unauthorized,
        AppError::NotFound,
        AppError::Conflict,
    ] {
        println!("{} {}", e.status_code(), e);
    }
}
```

**▼ 実行結果**

```text
401 ログインしてください
404 ノートが見つかりません
409 ほかの画面で先に更新されました
```

- `match` は、enum のすべての種類を扱わないとコンパイルエラーになります（網羅性チェック）。`Unauthorized` を足して `status_code` の `401` の行を書き忘れると、次のように教えてくれます。

```text
   Compiling design_lab v0.1.0 (...\design_lab)
error[E0004]: non-exhaustive patterns: `&AppError::Unauthorized` not covered
  --> src\bin\u05_err_e0004.rs:16:15
   |
16 |         match self {
   |               ^^^^ pattern `&AppError::Unauthorized` not covered
   |
note: `AppError` defined here
  --> src\bin\u05_err_e0004.rs:4:6
   |
 4 | enum AppError {
   |      ^^^^^^^^
 5 |     #[error("ログインしてください")]
 6 |     Unauthorized,
   |     ------------ not covered
   = note: the matched value is of type `&AppError`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
19 ~             AppError::Conflict => 409,
20 ~             &AppError::Unauthorized => todo!(),
   |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `design_lab` (bin "u05_err_e0004") due to 1 previous error
```

- `non-exhaustive patterns`（すべての場合を網羅していない）の `E0004` です。だから、エラーの種類を増やしても**ステータスコードの決め忘れが起きません**。ここで `_ => 500` のような「その他」の腕を書いてしまうと、このチェックが働かなくなるので注意しましょう。

**解答2:**

```rust
use thiserror::Error;

#[derive(Debug, Error)]
#[error("メールアドレスの形式が正しくありません")]
struct EmailError;

#[derive(Debug, Error)]
enum AppError {
    // #[from] で From<EmailError> for AppError が自動で作られる
    #[error(transparent)]
    InvalidEmail(#[from] EmailError),
    #[error("ノートが見つかりません")]
    NotFound,
}

fn parse_email(input: &str) -> Result<String, EmailError> {
    if input.contains('@') {
        Ok(input.to_string())
    } else {
        Err(EmailError)
    }
}

fn signup(input: &str) -> Result<String, AppError> {
    let email = parse_email(input)?;
    Ok(email)
}

fn main() {
    println!("{:?}", signup("alice@example.com"));
    match signup("alice") {
        Ok(_) => println!("登録しました"),
        Err(e) => println!("{e} / {e:?}"),
    }
    println!("{}", AppError::NotFound);
}
```

**▼ 実行結果**

```text
Ok("alice@example.com")
メールアドレスの形式が正しくありません / InvalidEmail(EmailError)
ノートが見つかりません
```

- `{}` では `#[error(transparent)]` により中の `EmailError` の文章が、`{:?}` では `InvalidEmail(EmailError)` という構造がそのまま表示されています。

**解答3:**

- (a) **thiserror**（または手書きの enum）。呼ぶ側が失敗の理由を見て、入力欄にメッセージを出すなど対応を変えるからです。
- (b) **anyhow**。起動時の失敗は種類で分岐せず、「DB に接続できませんでした（URL: …）」のような文脈を付けて表示し、終了すればよいからです。
- (c) **thiserror**。「version の食い違い」は 409 に変換すべき特別な失敗で、プログラムが種類を見分ける必要があるからです。

### まとめ

- エラーは enum で分類し、`Display`（人向けの文章）と `std::error::Error` を実装します。thiserror の `#[derive(Error)]` と `#[error("...")]` で短く書けます。
- `#[from]` や `impl From` があれば、`?` が層をまたいでエラーを変換します。`AppError` でステータスコードを決め、500 の詳細は画面に出さずログへ書きます。
- 種類で分岐したいエラーは thiserror、人に報告できればよいエラーは anyhow（`context` で文脈を積む）を使います。

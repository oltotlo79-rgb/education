<div style="page-break-after: always; text-align: center; margin: 0; padding: 0;">
<img src="assets/cover.jpg" alt="表紙" style="width: 100%; max-height: 175mm; object-fit: cover; border-radius: 10px;">
<div style="margin-top: 14px; font-size: 11px; letter-spacing: 0.18em; color: #c2410c; font-weight: 700;">VOLUME 02 / RUST BACKEND MASTERY</div>
<div style="font-size: 26px; font-weight: 800; color: #0f172a; margin-top: 6px; line-height: 1.35;">Rust基礎 — 値・型・制御・データ構造</div>
<div style="font-size: 13px; color: #475569; margin-top: 8px;">変数と型から文字列・コレクション・エラー処理まで、小さなコードを実際に動かして Rust の文法の土台を固める</div>
</div>

# 第2巻: Rust基礎 — 値・型・制御・データ構造

## この巻で身に付けること

第1巻で Rust の開発環境が整いました。この巻では、いよいよ Rust の**文法の土台**を学びます。値に名前を付ける方法から始めて、数や文字列の扱い、条件分岐とくり返し、データをまとめる型、そして「失敗するかもしれない処理」の書き方まで、バックエンド開発で毎日使う道具をひと通りそろえます。

各単元では、まず**数行の小さなコード**を動かし、その出力を確かめてから、1行ずつ意味を読み解きます。わざとエラーを出して、コンパイラのメッセージの読み方も練習します。Rust のコンパイラは、間違いの場所と直し方をとても丁寧に教えてくれる「先生」です。エラーを怖がらずに、たくさん出して読んでみましょう。

### 到達目標

この巻を終えると、次のことができるようになります。

- 変数・数値・文字・真偽値の型を読み分け、`mut` やシャドーイングを意図どおりに使える
- 関数・`if`・`loop`・`for`・`match` で処理の流れを組み立てられ、「式」と「文」の違いを説明できる
- 日本語を含む文字列を、UTF-8 の仕組みを踏まえて安全に扱える（`len()` と `chars().count()` の違い）
- タプル・配列・スライス・`Vec`・`HashMap` を目的に応じて使い分けられる
- `struct` と `enum` で自分の型を作り、`impl` でメソッドを付けられる
- 「値がないかもしれない」を `Option`、「失敗するかもしれない」を `Result` で表し、`?` で失敗を伝えられる
- イテレータとクロージャで並びを加工し、モジュールと `pub` でコードを整理できる
- メモアプリ **RustNote** のタイトル検証 `NoteTitle::parse` と、メモリ上で動くミニ版 CLI を自分で書ける

### 学習地図

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 360" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-02-17" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="22" text-anchor="middle" font-size="15" font-weight="700" fill="#0f172a">第2巻の学習地図（UNIT 01 → 10 の順に進む）</text>
<rect x="60" y="34" width="14" height="14" rx="3" fill="#eff6ff" stroke="#3b82f6"/>
<text x="80" y="46" font-size="12" fill="#334155">値と型</text>
<rect x="150" y="34" width="14" height="14" rx="3" fill="#fff7ed" stroke="#f97316"/>
<text x="170" y="46" font-size="12" fill="#334155">処理の流れ</text>
<rect x="260" y="34" width="14" height="14" rx="3" fill="#ecfdf5" stroke="#10b981"/>
<text x="280" y="46" font-size="12" fill="#334155">データのまとまり</text>
<rect x="400" y="34" width="14" height="14" rx="3" fill="#fef2f2" stroke="#ef4444"/>
<text x="420" y="46" font-size="12" fill="#334155">状態と失敗</text>
<rect x="510" y="34" width="14" height="14" rx="3" fill="#f5f3ff" stroke="#8b5cf6"/>
<text x="530" y="46" font-size="12" fill="#334155">道具箱と整理</text>
<rect x="20" y="66" width="124" height="80" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="82" y="88" text-anchor="middle" font-size="13" font-weight="700" fill="#1e40af">UNIT 01</text>
<text x="82" y="110" text-anchor="middle" font-size="12" fill="#0f172a">変数・mut</text>
<text x="82" y="128" text-anchor="middle" font-size="12" fill="#0f172a">シャドーイング・定数</text>
<rect x="159" y="66" width="124" height="80" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="221" y="88" text-anchor="middle" font-size="13" font-weight="700" fill="#1e40af">UNIT 02</text>
<text x="221" y="110" text-anchor="middle" font-size="12" fill="#0f172a">整数・小数・bool</text>
<text x="221" y="128" text-anchor="middle" font-size="12" fill="#0f172a">char と型の変換</text>
<rect x="298" y="66" width="124" height="80" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="88" text-anchor="middle" font-size="13" font-weight="700" fill="#c2410c">UNIT 03</text>
<text x="360" y="110" text-anchor="middle" font-size="12" fill="#0f172a">関数・文と式</text>
<text x="360" y="128" text-anchor="middle" font-size="12" fill="#0f172a">戻り値</text>
<rect x="437" y="66" width="124" height="80" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="499" y="88" text-anchor="middle" font-size="13" font-weight="700" fill="#c2410c">UNIT 04</text>
<text x="499" y="110" text-anchor="middle" font-size="12" fill="#0f172a">if・loop・while</text>
<text x="499" y="128" text-anchor="middle" font-size="12" fill="#0f172a">for・match</text>
<rect x="576" y="66" width="124" height="80" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="638" y="88" text-anchor="middle" font-size="13" font-weight="700" fill="#047857">UNIT 05</text>
<text x="638" y="110" text-anchor="middle" font-size="12" fill="#0f172a">String・str</text>
<text x="638" y="128" text-anchor="middle" font-size="12" fill="#0f172a">UTF-8</text>
<rect x="576" y="196" width="124" height="80" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="638" y="218" text-anchor="middle" font-size="13" font-weight="700" fill="#047857">UNIT 06</text>
<text x="638" y="240" text-anchor="middle" font-size="12" fill="#0f172a">タプル・配列</text>
<text x="638" y="258" text-anchor="middle" font-size="12" fill="#0f172a">スライス・Vec</text>
<rect x="437" y="196" width="124" height="80" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="499" y="218" text-anchor="middle" font-size="13" font-weight="700" fill="#047857">UNIT 07</text>
<text x="499" y="240" text-anchor="middle" font-size="12" fill="#0f172a">struct・impl</text>
<text x="499" y="258" text-anchor="middle" font-size="12" fill="#0f172a">関連関数・メソッド</text>
<rect x="298" y="196" width="124" height="80" rx="10" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="360" y="218" text-anchor="middle" font-size="13" font-weight="700" fill="#b91c1c">UNIT 08</text>
<text x="360" y="240" text-anchor="middle" font-size="12" fill="#0f172a">enum・Option</text>
<text x="360" y="258" text-anchor="middle" font-size="12" fill="#0f172a">パターン分解</text>
<rect x="159" y="196" width="124" height="80" rx="10" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="221" y="218" text-anchor="middle" font-size="13" font-weight="700" fill="#b91c1c">UNIT 09</text>
<text x="221" y="240" text-anchor="middle" font-size="12" fill="#0f172a">Result</text>
<text x="221" y="258" text-anchor="middle" font-size="12" fill="#0f172a">エラー伝播・?</text>
<rect x="20" y="196" width="124" height="80" rx="10" fill="#f5f3ff" stroke="#8b5cf6" stroke-width="2"/>
<text x="82" y="218" text-anchor="middle" font-size="13" font-weight="700" fill="#6d28d9">UNIT 10</text>
<text x="82" y="240" text-anchor="middle" font-size="12" fill="#0f172a">HashMap・Iterator</text>
<text x="82" y="258" text-anchor="middle" font-size="12" fill="#0f172a">closure・モジュール</text>
<line x1="144" y1="106" x2="155" y2="106" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-17)"/>
<line x1="283" y1="106" x2="294" y2="106" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-17)"/>
<line x1="422" y1="106" x2="433" y2="106" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-17)"/>
<line x1="561" y1="106" x2="572" y2="106" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-17)"/>
<line x1="638" y1="146" x2="638" y2="192" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-17)"/>
<line x1="576" y1="236" x2="565" y2="236" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-17)"/>
<line x1="437" y1="236" x2="426" y2="236" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-17)"/>
<line x1="298" y1="236" x2="287" y2="236" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-17)"/>
<line x1="159" y1="236" x2="148" y2="236" stroke="#475569" stroke-width="2" marker-end="url(#ar-02-17)"/>
<rect x="20" y="296" width="680" height="56" rx="10" fill="#f8fafc" stroke="#94a3b8"/>
<text x="360" y="318" text-anchor="middle" font-size="12" font-weight="700" fill="#0f172a">RustNote で育てるもの</text>
<text x="360" y="340" text-anchor="middle" font-size="12" fill="#334155">上限の定数 → 文字数の数え方 → Note 構造体 → NoteTitle::parse（検証）→ HashMap で動くミニ版 CLI</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図0-1: 10の単元は5つのまとまりに分かれる。各単元の最後で、RustNote の部品を1つずつ作っていく</div>
</div>

| 単元 | 内容 | RustNote との接続 |
|---|---|---|
| UNIT 01 | 変数・`mut`・シャドーイング・`const` | タイトルの上限 `MAX_TITLE_CHARS = 80` |
| UNIT 02 | 整数・小数・`bool`・`char` と型の変換 | `?page=` のページ番号を安全に数へ |
| UNIT 03 | 関数・文と式・戻り値 | タイトルの文字数チェックを小さな関数に |
| UNIT 04 | `if`・`loop`・`while`・`for`・`match` | 「前へ／次へ」リンクの出し分け |
| UNIT 05 | `String`・`&str`・UTF-8 | 日本語タイトルを `chars().count()` で数える |
| UNIT 06 | タプル・配列・スライス・`Vec` | 一覧の表示と完了件数の集計 |
| UNIT 07 | `struct`・`impl`・関連関数・メソッド | `Note` 構造体、`NoteId` / `UserId` |
| UNIT 08 | `enum`・`Option`・パターン | 検索語 `?q=` を `Option` で扱う |
| UNIT 09 | `Result`・`?` 演算子・エラー型 | `NoteTitle::parse` と `TitleError` |
| UNIT 10 | `HashMap`・イテレータ・クロージャ・モジュール | メモリ上で動く RustNote ミニ版 CLI |

### 前提知識

- **第0巻（学習ガイド）**: ファイルとフォルダ、ターミナル（PowerShell）でコマンドを打つこと、「ソースコードをコンパイルして実行する」流れ。
- **第1巻（環境構築）**: `rustup` と `cargo` のインストール、VS Code と rust-analyzer の準備、`cargo new` / `cargo run` / `cargo fmt` の使い方。
- プログラミングの経験はなくてもかまいません。他の言語の経験がある人は、「Rust ではここが違う」という注意（**つまずきポイント**）に特に目を通してください。

### この巻の進め方

**1. 練習用のプロジェクトを1つ作る**

この巻のコードは、すべて同じ1つのプロジェクトで試せます。最初に1回だけ、次のコマンドを実行してください。

```powershell
# 好きな作業フォルダで実行する
cargo new basics
cd basics
```

以後は、例が出てくるたびに `src/main.rs` の中身を**すべて**その例のコードに置き換えて、`cargo run` で実行します（UNIT 10 の RustNote ミニ版だけは、別のプロジェクト `rustnote_mini` を作ります）。

**2. 予想 → 実行 → 観察 → 1か所変える**

1. コードを読んで、何が表示されるかを**予想**します。
2. そのまま**実行**して、本書の「▼ 実行結果」と同じか確かめます。
3. 予想と違ったところを、「▼ コードを1つずつ分解して解説」で**観察**します。
4. 数や文字を1か所だけ**変えて**もう一度実行し、結果がどう変わるかを見ます。わざと `;` を消す、`mut` を外す、なども良い練習です。

**3. 実行結果の読み方**

- `cargo run` は、プログラムの出力の前に次の3行を表示します。本書の「▼ 実行結果」では、この3行を**省略して**プログラム自身の出力だけを載せています。

```text
   Compiling basics v0.1.0 (C:\Users\...\basics)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target\debug\basics.exe`
```

- コンパイルエラーの例では、`Compiling` の行から全文を載せています。フォルダの場所は `(.../basics)` のように伏せています。
- エラーの場所は Windows では `src\main.rs`、macOS / Linux / WSL では `src/main.rs` と表示されます。
- panic（実行時の非常停止）の表示にある `thread 'main' (12345)` の数字は、実行のたびに変わります。

**4. 各単元の組み立て**

どの単元も、「この単元のゴール → たとえ話 → 図 → まず動かす最小コード → もう一歩（実務に近い形）→ よくあるエラーと読み方 → RustNote ではここで使う → 練習問題と解答 → まとめ」の順に並んでいます。急ぐときは、ゴールと図と最小コードだけ先に読み、あとで戻ってきてもかまいません。

> **ヒント:** コードを書いたら `cargo fmt` で見た目を整え、`cargo clippy` で「もっと良い書き方」の提案を確かめる習慣を付けましょう（第1巻）。本書のコードはすべて `cargo fmt` で整えた形で載せています。

### 使うツールと版

| 項目 | この巻での前提 |
|---|---|
| Rust | **1.98 以上**（本書の動作確認は rustc 1.99.0 / cargo 1.99.0） |
| エディション | 2024（`cargo new` で作ると `Cargo.toml` に `edition = "2024"` と書かれる） |
| 外部クレート | 使わない（標準ライブラリだけ） |
| OS・端末 | Windows 11 + PowerShell を主に説明（WSL / macOS / Linux でも同じコードが動く） |
| エディタ | VS Code + rust-analyzer |

手元の版は、次のコマンドで確かめられます。

```powershell
rustc --version
cargo --version
```

1.98 より古いと表示されたら、`rustup update` で更新してください。本書と全く同じ版で試したいときは、プロジェクトのフォルダ（`Cargo.toml` と同じ場所）に `rust-toolchain.toml` というファイルを作り、次のように書きます。そのフォルダの中では、指定した版が自動で使われます（初回は自動でダウンロードされます）。

```toml
[toolchain]
channel = "1.99.0"
```

> **注意:** エラーメッセージの細かい言い回しは、Rust の版によって少し変わることがあります。本書と完全に同じでなくても、`error[E0384]` のような**エラー番号**と、`^^^` が指す場所が同じなら、内容は同じです。

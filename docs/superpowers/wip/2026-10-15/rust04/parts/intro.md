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

<!--OUT:intro_cargo_add-->

`Cargo.toml` は次のようになります。

<!--FILE:toml:design_lab/Cargo.toml-->

- `src/bin/u01_ids.rs` のようにファイルを置くと、`cargo run --bin u01_ids` でそのファイルの `main` だけを実行できます。1つのプロジェクトで、たくさんの小さなプログラムを試せて便利です。
- 「よくあるエラー」の節のコードは、わざとコンパイルが失敗するコードです。`src/bin/` に置いたままにすると、プロジェクト全体をビルドする `cargo build`・`cargo test`・`cargo clippy` まで失敗するようになるので、エラーを確かめたら**ファイルを消すか、正しく直して**おいてください。

<div style="page-break-after: always; text-align: center; margin: 0; padding: 0;">
<img src="assets/cover.jpg" alt="表紙" style="width: 100%; max-height: 175mm; object-fit: cover; border-radius: 10px;">
<div style="margin-top: 14px; font-size: 11px; letter-spacing: 0.18em; color: #c2410c; font-weight: 700;">VOLUME 07 / RUST BACKEND MASTERY</div>
<div style="font-size: 26px; font-weight: 800; color: #0f172a; margin-top: 6px; line-height: 1.35;">Turso・libSQL — SQL設計・Rust実装・運用</div>
<div style="font-size: 13px; color: #475569; margin-top: 8px;">表の設計と SQL から、transaction・migration・Rust からの接続・バックアップまで、データを安全に預かる力を身に付ける</div>
</div>

# 第7巻: Turso・libSQL — SQL設計・Rust実装・運用

## この巻で身に付けること

ノートアプリ RustNote で、利用者が書いたノートはどこに残るのでしょうか。プログラムの変数に入れておくだけでは、サーバーを再起動した瞬間に全部消えてしまいます。そこで、データを**データベース（DB）**に預けます。この巻では、RustNote が使う DB である **libSQL**（リブエスキューエル）と、それをクラウドで動かすサービス **Turso**（ターソ）を題材に、DB の使い方を基礎から学びます。

DB は「入れて、出す」だけの箱に見えますが、使い方を間違えると次のような事故が起きます。

- 一覧の SQL に「持ち主の条件」を書き忘れて、**他人のノートが見えてしまう**
- 保存の途中でエラーが起き、**半分だけ書き換わった中途半端なデータ**が残る
- ノートが1万件に増えたら、一覧の表示が急に**遅くなる**
- 表に列を足す変更を本番で流したら、**途中で失敗して元に戻せなくなる**
- フォームに入力された文字をそのまま SQL に混ぜてしまい、**表を丸ごと消される**（SQL インジェクション）
- DB の合言葉（トークン）が漏れたり、DB のファイルが消えたりしたときに、**どう戻せばよいか分からない**

この巻のテーマは、こうした事故を、表の**制約**・**transaction**（トランザクション）・**index**（インデックス）・**migration**（マイグレーション）・**パラメーターの bind**（バインド）・**Repository**（リポジトリ）・**backup**（バックアップ）という仕組みで防ぐことです。SQL は 50 年近く使われ続けている言葉で、ここで覚えたことは、PostgreSQL や MySQL など、ほかの DB でもほとんどそのまま役に立ちます。

### 到達目標

この巻を終えると、次のことができるようになります。

- 表・行・列・主キー・外部キーの意味を説明し、`NOT NULL`・`UNIQUE`・`CHECK` で**不正なデータを DB の入口で断る**表を設計できる
- `SELECT` で必要な行だけを読み出し、`WHERE`・`ORDER BY`・`LIMIT` で絞り込み・並べ替え・ページ送りができる
- `INSERT`・`UPDATE`・`DELETE` を書き、**持ち主の条件**で他人のノートを変更できないようにできる。影響を受けた行数を確かめられる
- index を作り、`EXPLAIN QUERY PLAN` で「速く探せているか」を確かめられる
- transaction で複数の SQL を「全部成功か、全部取り消しか」にまとめ、`version` 列で同時編集の食い違いを見つけられる
- migration で表の形を安全に変え、どこまで適用したかを記録できる
- Rust の libsql クレートで DB に接続し、値を**パラメーターとして bind** して SQL インジェクションを防げる
- ローカルのファイル・Turso のリモート・embedded replica の違いを説明し、目的に合わせて選べる
- SQL を Repository に集め、DB の行をドメインの型へ安全に変換できる。N+1 問題を見つけて直せる
- backup と復元の練習ができ、トークンの管理・監視・障害対応の手順を説明できる

### 学習地図

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 400" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-001" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="6" y="30" width="708" height="90" rx="12" fill="#eff6ff"/>
<rect x="6" y="140" width="708" height="90" rx="12" fill="#fff7ed"/>
<rect x="6" y="250" width="708" height="90" rx="12" fill="#ecfdf5"/>
<text x="360" y="20" font-size="13" fill="#475569" text-anchor="middle">上の段から順に進みます（各段の中は左から右へ）</text>
<text x="65" y="70" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">SQL で</text>
<text x="65" y="90" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">表を扱う</text>
<text x="65" y="180" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">速く・安全に</text>
<text x="65" y="200" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">変える</text>
<text x="65" y="290" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">Rust から</text>
<text x="65" y="310" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">使い・守る</text>
<rect x="135" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="200" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 01</text>
<text x="200" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">表を作る</text>
<text x="200" y="99" font-size="12" fill="#475569" text-anchor="middle">CREATE と制約</text>
<rect x="282" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="347" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 02</text>
<text x="347" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">読み出す</text>
<text x="347" y="99" font-size="12" fill="#475569" text-anchor="middle">SELECT・WHERE</text>
<rect x="429" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="494" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 03</text>
<text x="494" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">書き換える</text>
<text x="494" y="99" font-size="12" fill="#475569" text-anchor="middle">持ち主の条件</text>
<line x1="265" y1="75" x2="280" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-001)"/>
<line x1="412" y1="75" x2="427" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-001)"/>
<rect x="576" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-dasharray="5 4"/>
<text x="641" y="64" font-size="12" fill="#475569" text-anchor="middle">RustNote では</text>
<text x="641" y="84" font-size="12" fill="#475569" text-anchor="middle">users・notes の表</text>
<text x="641" y="102" font-size="12" fill="#475569" text-anchor="middle">一覧・検索</text>
<path d="M494,110 V130 H200 V146" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-001)"/>
<rect x="135" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="200" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 04</text>
<text x="200" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">index</text>
<text x="200" y="209" font-size="12" fill="#475569" text-anchor="middle">速く探す</text>
<rect x="282" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="347" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 05</text>
<text x="347" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">transaction</text>
<text x="347" y="209" font-size="12" fill="#475569" text-anchor="middle">全部か、無しか</text>
<rect x="429" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="494" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 06</text>
<text x="494" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">migration</text>
<text x="494" y="209" font-size="12" fill="#475569" text-anchor="middle">表の形を変える</text>
<line x1="265" y1="185" x2="280" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-001)"/>
<line x1="412" y1="185" x2="427" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-001)"/>
<rect x="576" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-dasharray="5 4"/>
<text x="641" y="174" font-size="12" fill="#475569" text-anchor="middle">RustNote では</text>
<text x="641" y="194" font-size="12" fill="#475569" text-anchor="middle">version 列</text>
<text x="641" y="212" font-size="12" fill="#475569" text-anchor="middle">schema_migrations</text>
<path d="M494,220 V240 H200 V256" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-001)"/>
<rect x="135" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="200" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 07</text>
<text x="200" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">Rust SDK</text>
<text x="200" y="319" font-size="12" fill="#475569" text-anchor="middle">値を bind する</text>
<rect x="282" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="347" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 08</text>
<text x="347" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">置き場所</text>
<text x="347" y="319" font-size="12" fill="#475569" text-anchor="middle">local・remote</text>
<rect x="429" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="494" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 09</text>
<text x="494" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">Repository</text>
<text x="494" y="319" font-size="12" fill="#475569" text-anchor="middle">SQL を集める</text>
<rect x="576" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="641" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 10</text>
<text x="641" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">運用</text>
<text x="641" y="319" font-size="12" fill="#475569" text-anchor="middle">backup・監視</text>
<line x1="265" y1="295" x2="280" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-001)"/>
<line x1="412" y1="295" x2="427" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-001)"/>
<line x1="559" y1="295" x2="574" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-001)"/>
<text x="360" y="372" font-size="13" fill="#475569" text-anchor="middle">各単元の流れ: たとえ話 → 図 → 最小コード → 実務の形 → よくあるエラー → 練習問題</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図7-?: 第7巻の学習地図。SQL の基本から始めて、安全に変える仕組み、Rust からの使い方と運用へ進む</div>
</div>

| 単元 | テーマ | ひとことで言うと |
|---|---|---|
| UNIT 01 | 関係モデルと CREATE TABLE | 表を作り、制約で不正なデータを DB の入口で断る |
| UNIT 02 | SELECT・WHERE・ORDER BY・LIMIT | 必要な行だけを、決まった順番で、少しずつ読み出す |
| UNIT 03 | INSERT・UPDATE・DELETE と持ち主の条件 | 書き換えるときは「誰のデータか」も条件に入れる |
| UNIT 04 | index と EXPLAIN QUERY PLAN | 本の索引のような仕組みで、探す速さを上げる |
| UNIT 05 | transaction・ACID・競合 | 複数の変更を「全部成功か、全部取り消しか」にまとめる |
| UNIT 06 | migration と schema version | 表の形の変更を、番号付きの手順として記録し、順に適用する |
| UNIT 07 | libsql Rust SDK の接続とパラメーターの bind | Rust から SQL を実行し、値は文字列連結せずに bind する |
| UNIT 08 | local・remote・embedded replica | DB を手元に置くか、クラウドに置くか、両方を組み合わせるか |
| UNIT 09 | Repository・行の変換・N+1 | SQL を1か所に集め、DB の行を検証済みの型へ変換する |
| UNIT 10 | backup・token・監視・障害対応 | 壊れたとき・漏れたときに、慌てず戻せるように準備する |

### SQLite・libSQL・Turso の関係

この巻には、似た名前がいくつも出てきます。最初に関係を整理しておきましょう。

- **SQLite**（エスキューライト）: 1つのファイルの中に表を保存する、とても小さな DB の仕組みです。スマートフォンのアプリやブラウザの中でも使われていて、世界でいちばん多く使われている DB だと言われます。
- **libSQL**: SQLite をもとに、Turso 社が機能を足して公開しているオープンソースの DB です。SQL の書き方は SQLite とほぼ同じで、この巻の SQL はすべて SQLite の SQL として読めます。
- **Turso**（Turso Cloud）: libSQL をインターネット上のサーバーで動かしてくれるサービスです。RustNote を公開するときの DB の置き場所になります。
- **libsql クレート**: Rust のプログラムから libSQL を使うための部品（クレート）です。ローカルのファイルにも、Turso のサーバーにも、同じ書き方でつなげます。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-002" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="16" y="100" width="180" height="100" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="106" y="128" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">あなたの Rust</text>
<text x="106" y="146" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">プログラム</text>
<rect x="36" y="158" width="140" height="30" rx="6" fill="#ffffff" stroke="#f97316"/>
<text x="106" y="178" font-size="12" fill="#c2410c" text-anchor="middle">libsql クレート</text>
<rect x="290" y="20" width="200" height="110" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="390" y="46" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">手元の DB ファイル</text>
<text x="390" y="68" font-size="12" fill="#0f172a" text-anchor="middle">notes.db（中身は libSQL の形式）</text>
<text x="390" y="90" font-size="12" fill="#475569" text-anchor="middle">開発・練習で使う</text>
<text x="390" y="110" font-size="12" fill="#475569" text-anchor="middle">（この巻のほとんど）</text>
<rect x="290" y="170" width="200" height="110" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="390" y="196" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">Turso Cloud</text>
<text x="390" y="218" font-size="12" fill="#0f172a" text-anchor="middle">ネットの向こうの libSQL サーバー</text>
<text x="390" y="240" font-size="12" fill="#475569" text-anchor="middle">公開したアプリで使う</text>
<text x="390" y="260" font-size="12" fill="#475569" text-anchor="middle">（UNIT 08・10）</text>
<line x1="196" y1="130" x2="286" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-002)"/>
<line x1="196" y1="170" x2="286" y2="215" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-002)"/>
<text x="226" y="96" font-size="11" fill="#475569" text-anchor="middle">new_local</text>
<text x="226" y="214" font-size="11" fill="#475569" text-anchor="middle">new_remote</text>
<rect x="520" y="20" width="186" height="260" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="613" y="46" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">名前の整理</text>
<text x="534" y="76" font-size="12" font-weight="700" fill="#0f172a">SQLite</text>
<text x="534" y="96" font-size="11" fill="#475569">ファイルに表を保存する</text>
<text x="534" y="112" font-size="11" fill="#475569">小さな DB の仕組み</text>
<text x="534" y="142" font-size="12" font-weight="700" fill="#0f172a">libSQL</text>
<text x="534" y="162" font-size="11" fill="#475569">SQLite をもとに機能を</text>
<text x="534" y="178" font-size="11" fill="#475569">足したオープンソースの DB</text>
<text x="534" y="208" font-size="12" font-weight="700" fill="#0f172a">Turso</text>
<text x="534" y="228" font-size="11" fill="#475569">libSQL をクラウドで</text>
<text x="534" y="244" font-size="11" fill="#475569">動かしてくれるサービス</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図7-?: SQLite・libSQL・Turso と libsql クレートの関係。同じ Rust のコードで、手元のファイルにもクラウドにもつなげる</div>
</div>

> **ヒント:** 練習では、Turso のアカウントも、インターネットへの接続もいりません。UNIT 01〜10 の例は、すべて手元の DB ファイル（またはメモリの中だけの DB）で最後まで試せます。UNIT 08 のリモートの例も、手元の何も動いていないポートにつないで、失敗のしかたを確かめます。Turso Cloud の DB の作り方とトークンの管理は、UNIT 10 でコマンドの形と手順を紹介します（この本では実行しません）。

### 前提知識（先に読んでおく巻）

- **第1巻（環境構築）:** Rust・VS Code・rust-analyzer がインストール済みで、`cargo new` と `cargo run` ができること。
- **第2巻（Rust 基礎）:** `struct` / `enum` / `match` / `Option` / `Result` / `?` / `Vec` / `String` の基本。
- **第3巻（所有権・借用）:** `&` と `&mut`、所有権の移動。`drop` で値が片付けられるタイミング（UNIT 05 で使います）。
- **第4巻（設計）:** `UserId`・`NoteId`・`NoteTitle` などの newtype、`NoteRepository` trait、thiserror で作るエラー型と `AppError`（UNIT 09 で使います）。

SQL は、この巻で初めて学ぶ前提で、いちばん最初から説明します。

> **ヒント:** この巻の Rust のコードには `async` や `.await` が出てきます。くわしい仕組みは第5巻（async・Tokio）で学びますが、この巻では「`.await` は **DB の返事が来るまで待つ**印」と読めば十分です。`#[tokio::main]` は「`main` の中で `.await` を使えるようにする」おまじないとして使います。

### この巻の進め方

各単元では、次の順番を1サイクルとして進めてください。SQL は、**自分で打って、わざと間違えて、エラーを読む**のがいちばんの近道です。

1. **予想する:** SQL を実行する前に、何行返ってくるか、どこでエラーになるかを予想します。
2. **そのまま動かす:** 教材の SQL やコードをそのまま実行し、結果が教材と同じか比べます（日時や実行時間の数字は、パソコンによって変わります）。
3. **1か所だけ変える:** 条件や値を1つだけ変えて、結果がどう変わるかを観察します。「よくあるエラー」の節の SQL も実際に流して、エラーメッセージを自分の目で読みます。
4. **整える:** Rust のコードを書いた単元の終わりには、次の3つのコマンドを実行する習慣をつけます。

```powershell
# コードの見た目を Rust の標準の形に整える
cargo fmt
# よくある書き間違い・もったいない書き方を指摘してもらう
cargo clippy
# テストを実行する
cargo test
```

### 使うツールと版

この巻の SQL とコードは、すべて次の環境で実際に実行し、実行結果とエラーメッセージは実際の出力をそのまま載せています（パソコンのユーザー名などを含む部分だけ `...` に置き換えています）。

| 項目 | 版・内容 |
|---|---|
| OS | Windows 11（ARM64）。コマンドは PowerShell を主に、WSL / macOS / Linux は補足で示します |
| Rust | **1.98 以上**（本書シリーズの基準）。動作確認は rustc 1.99.0 / cargo 1.99.0（`aarch64-pc-windows-msvc`） |
| edition | 2024（`cargo new` の既定値） |
| libsql | 0.10.0（Rust から libSQL を使うクレート。中で動く SQLite エンジンは 3.47.0） |
| tokio | 1.53.3（`rt-multi-thread` と `macros` の機能つき。`.await` を動かす土台） |
| thiserror | 2.0.21（エラー型を短く書く。第4巻 UNIT 05 と同じ） |
| Turso CLI | この巻の練習では使いません。Windows 版は無く、使うときは WSL（Windows の中の Linux）に入れます（UNIT 08・10 の例） |

### 練習用プロジェクトを用意する

この巻では、練習用のプロジェクト `sql-lab` を1つ作り、SQL のファイルを `sql/` に、Rust の小さなプログラムを `src/bin/` に足していきます。

```powershell
cargo new sql-lab
cd sql-lab
mkdir src/bin
mkdir sql
cargo add libsql@0.10.0
cargo add tokio@1.53.3 --features rt-multi-thread,macros
cargo add thiserror@2.0.21
```

`cargo new` と `cargo add` の実際の出力は次のとおりです。

```text
    Creating binary (application) `sql-lab` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
    Updating crates.io index
      Adding libsql v0.10.0 to dependencies
             Features:
             + core
             + hrana
             + libsql-sys
             + parser
             + remote
             + replication
             + serde
             + stream
             + sync
             + tls
             - cloudflare
             - encryption
             - wasm
    Updating crates.io index
     Locking 211 packages to highest Rust 1.99.0 compatible versions
      Adding generic-array v0.14.7 (available: v0.14.9)
    Updating crates.io index
      Adding tokio v1.53.3 to dependencies
             Features:
             + macros
             + rt
             + rt-multi-thread
             + tokio-macros
             - bytes
             - fs
             - full
             - io-std
             - io-uring
             - io-util
             - libc
             - mio
             - net
             - parking_lot
             - process
             - schedule-latency
             - signal
             - signal-hook-registry
             - socket2
             - sync
             - taskdump
             - test-util
             - time
             - tracing
             - windows-sys
    Updating crates.io index
      Adding thiserror v2.0.21 to dependencies
             Features:
             + std
    Updating crates.io index
     Locking 2 packages to highest Rust 1.99.0 compatible versions
      Adding thiserror v2.0.21
      Adding thiserror-impl v2.0.21
```

- `+` が付いた機能（feature）は ON、`-` は OFF です。libsql は、ローカルのファイルを扱う `core` に加えて、Turso のサーバーとやり取りする `remote`・`replication`・`sync`、暗号化した通信の `tls` が最初から ON になっています。
- 「`Locking 211 packages`」は、libsql が使っている部品まで含めて 211 個のクレートの版を `Cargo.lock` に記録した、という意味です。最初のビルドには数分かかります。

最後に、`Cargo.toml` の `[package]` の中、`edition = "2024"` の次の行に `rust-version = "1.98"` を手で1行足します（第4巻 UNIT 07 で学んだ「このプロジェクトは Rust 1.98 以上でビルドしてください」という宣言です）。できあがった `Cargo.toml` は次のとおりです。

```toml
[package]
name = "sql-lab"
version = "0.1.0"
edition = "2024"
rust-version = "1.98"

[dependencies]
libsql = "0.10.0"
thiserror = "2.0.21"
tokio = { version = "1.53.3", features = ["rt-multi-thread", "macros"] }
```

#### ARM64 版の Windows では clang が必要

libsql の `tls` の機能は、`ring` という暗号のクレートを使います。ARM64 版の Windows では、この `ring` をビルドするのに **clang**（クラン：LLVM という C 言語コンパイラの仲間）が必要です。clang が無いと、最初のビルドが次のように失敗します（実際の出力から要点を抜き出したもの）。

```text
warning: ring@0.17.14: Compiler family detection failed due to error: ToolNotFound: failed to find tool "clang": program not found (see https://docs.rs/cc/latest/cc/#compile-time-requirements for help)
error: failed to run custom build command for `ring v0.17.14`

Caused by:
  process didn't exit successfully: `target\debug\build\ring-1d47c9bd909157dd\build-script-build` (exit code: 1)
```

- **何が起きたか:** `failed to find tool "clang"` は「clang というプログラムが見つからない」という意味です。`ring` のビルド用のプログラム（build script）が、C 言語の部分をコンパイルしようとして止まりました。
- **直し方:** LLVM をインストールし、`C:\Program Files\LLVM\bin` を PATH に入れてから、PowerShell を開き直してビルドし直します。winget を使う場合は `winget install -e --id LLVM.LLVM` です。`clang --version` で版が表示されれば準備完了です。
- **x64 版の Windows・macOS・Linux:** 通常はこの手順はいりません。

> **ヒント:** どうしても clang を入れられないときは、`Cargo.toml` の libsql の行を `libsql = { version = "0.10.0", default-features = false, features = ["core"] }` に変えると、clang なしでビルドできます。ただし、この形では**ローカルのファイルしか扱えません**（Turso のサーバーにつなぐ `Builder::new_remote` などが使えなくなります）。UNIT 08 のリモートと組み込みレプリカの例以外は、この形でも最後まで試せます。

### SQL を試す道具 `sql` を用意する

SQL を試すには、SQL の文を DB に送り、返ってきた結果を見る道具が必要です。この巻では、そのための小さな Rust のプログラム `sql` を最初に作っておきます。SQL を書いたファイルを渡すと、1文ずつ実行して、結果を表の形で表示してくれます。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 190" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-07-003" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="40" width="150" height="90" rx="10" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="85" y="66" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">SQL のファイル</text>
<text x="85" y="88" font-size="12" fill="#475569" text-anchor="middle">sql/hello.sql</text>
<text x="85" y="108" font-size="11" fill="#475569" text-anchor="middle">文を ; で区切って書く</text>
<rect x="190" y="40" width="150" height="90" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="265" y="66" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">① 1文ずつに分ける</text>
<text x="265" y="88" font-size="11" fill="#475569" text-anchor="middle">; で区切る</text>
<text x="265" y="106" font-size="11" fill="#475569" text-anchor="middle">-- のコメントは捨てる</text>
<rect x="370" y="40" width="150" height="90" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="445" y="66" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">② libSQL で実行</text>
<text x="445" y="88" font-size="11" fill="#475569" text-anchor="middle">メモリの DB か</text>
<text x="445" y="106" font-size="11" fill="#475569" text-anchor="middle">--db で指定したファイル</text>
<rect x="550" y="40" width="160" height="90" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="630" y="66" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">③ 結果を表示</text>
<text x="630" y="88" font-size="11" fill="#475569" text-anchor="middle">行は表の形で</text>
<text x="630" y="106" font-size="11" fill="#475569" text-anchor="middle">エラーも表示して次の文へ</text>
<line x1="160" y1="85" x2="186" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-003)"/>
<line x1="340" y1="85" x2="366" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-003)"/>
<line x1="520" y1="85" x2="546" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#ar-07-003)"/>
<text x="360" y="165" font-size="12" fill="#475569" text-anchor="middle">--setup で渡したファイルは、結果を表示せずに先に実行する（表やデータの準備用）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図7-?: 道具 sql の仕事。SQL のファイルを1文ずつ実行し、行は表の形で、エラーはメッセージで見せる</div>
</div>

次のコードを `src/bin/sql.rs` として保存してください。長く見えますが、**今は中身を理解しなくてかまいません**。この巻の UNIT 07 まで進むと、ほとんどの行が読めるようになります。今は「SQL を試すための道具」として、写して使ってください。

#### src/bin/sql.rs（SQL を試す道具）

> **▼ このコードがやること（先に日本語で）:**
> 1. コマンドラインの引数から、DB のファイル名（`--db`）・準備用の SQL（`--setup`）・実行する SQL のファイルを受け取る
> 2. DB を開き、ファイルの SQL を `;` で1文ずつに分けて、順番に実行する
> 3. 行を返す文（`SELECT` など）は結果を表の形で、行を返さない文は「OK」と変更した行数を表示する。エラーが起きてもそこで止まらず、メッセージを表示して次の文へ進む

```rust
// SQL ファイルを libSQL で1文ずつ実行し、結果を表の形で表示する道具。
// 使い方: cargo run -q --bin sql -- [--db ファイル] [--setup 準備用.sql]... 実行する.sql...
//   --db を付けないと、メモリの中だけの DB（終わると消える）を使う
//   --setup のファイルは、結果を表示せずに先に実行する（表やデータの準備用）
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::ExitCode;

use libsql::{Builder, Connection, Value};

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), String> {
    // コマンドラインの引数を読む
    let mut db_path = String::from(":memory:");
    let mut setup_files = Vec::new();
    let mut files = Vec::new();
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--db" => db_path = args.next().ok_or("--db の後にファイル名が必要です")?,
            "--setup" => setup_files.push(args.next().ok_or("--setup の後にファイル名が必要です")?),
            _ => files.push(arg),
        }
    }
    if files.is_empty() {
        return Err("使い方: sql [--db ファイル] [--setup 準備.sql]... 実行する.sql...".into());
    }

    // DB を開いて、接続を1本だけ作る（全部の文をこの接続で実行する）
    let db = Builder::new_local(&db_path)
        .build()
        .await
        .map_err(|e| e.to_string())?;
    let conn = db.connect().map_err(|e| e.to_string())?;

    for path in &setup_files {
        let sql = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        let statements = split_statements(&sql);
        for statement in &statements {
            run_statement(&conn, statement, false).await?;
        }
        println!("（準備: {path} の {} 文を実行しました）", statements.len());
    }
    for path in &files {
        let sql = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        for statement in split_statements(&sql) {
            // 文の1行目だけを表示する（2行目以降があれば … を付ける）
            let mut lines = statement.lines();
            let first = lines.next().unwrap_or("");
            let more = if lines.next().is_some() { " …" } else { "" };
            println!("sql> {first}{more}");
            // エラーが起きても止めずに、次の文へ進む
            if let Err(message) = run_statement(&conn, &statement, true).await {
                println!("エラー: {message}");
            }
            println!();
        }
    }
    Ok(())
}

// 1つの文を実行する。show が true なら結果を表示する
async fn run_statement(conn: &Connection, sql: &str, show: bool) -> Result<(), String> {
    let stmt = conn.prepare(sql).await.map_err(|e| e.to_string())?;
    if stmt.columns().is_empty() {
        // 行を返さない文（CREATE / INSERT / UPDATE / DELETE など）
        let changed = stmt.execute(()).await.map_err(|e| e.to_string())?;
        if show {
            let word = sql.split_whitespace().next().unwrap_or("").to_uppercase();
            if ["INSERT", "UPDATE", "DELETE", "REPLACE"].contains(&word.as_str()) {
                println!("OK（変更: {changed} 行）");
            } else {
                println!("OK");
            }
        }
        return Ok(());
    }

    // 行を返す文（SELECT など）: 列名と全部の行を読み取る
    let mut rows = stmt.query(()).await.map_err(|e| e.to_string())?;
    let count = rows.column_count();
    let names: Vec<String> = (0..count)
        .map(|i| rows.column_name(i).unwrap_or("?").to_string())
        .collect();
    let mut table: Vec<Vec<String>> = Vec::new();
    while let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
        let mut cells = Vec::new();
        for i in 0..count {
            let value = row.get_value(i).map_err(|e| e.to_string())?;
            cells.push(format_value(&value));
        }
        table.push(cells);
    }
    if !show {
        return Ok(());
    }
    if sql.to_uppercase().starts_with("EXPLAIN QUERY PLAN") {
        print_query_plan(&table);
    } else {
        print_table(&names, &table);
    }
    Ok(())
}

// 値を表示用の文字列にする
fn format_value(value: &Value) -> String {
    match value {
        Value::Null => "NULL".to_string(),
        Value::Integer(n) => n.to_string(),
        Value::Real(x) => x.to_string(),
        Value::Text(s) => s.clone(),
        Value::Blob(bytes) => {
            let hex: String = bytes.iter().map(|b| format!("{b:02X}")).collect();
            format!("x'{hex}'")
        }
    }
}

// 表の形で表示する
fn print_table(names: &[String], table: &[Vec<String>]) {
    // 列ごとに、いちばん長い値の幅を求める
    let mut widths: Vec<usize> = names.iter().map(|n| width(n)).collect();
    for cells in table {
        for (i, cell) in cells.iter().enumerate() {
            widths[i] = widths[i].max(width(cell));
        }
    }
    let line: Vec<String> = widths.iter().map(|w| "-".repeat(w + 2)).collect();
    let line = format!("+{}+", line.join("+"));
    println!("{line}");
    print_cells(names, &widths);
    println!("{line}");
    for cells in table {
        print_cells(cells, &widths);
    }
    if !table.is_empty() {
        println!("{line}");
    }
    println!("({} 行)", table.len());
}

fn print_cells(cells: &[String], widths: &[usize]) {
    let padded: Vec<String> = cells
        .iter()
        .zip(widths)
        .map(|(cell, w)| format!(" {cell}{} ", " ".repeat(w - width(cell))))
        .collect();
    println!("|{}|", padded.join("|"));
}

// EXPLAIN QUERY PLAN の結果（id, parent, notused, detail）を木の形で表示する
fn print_query_plan(table: &[Vec<String>]) {
    println!("QUERY PLAN");
    let mut depth: HashMap<String, usize> = HashMap::new();
    for cells in table {
        let d = depth.get(&cells[1]).map_or(0, |p| p + 1);
        depth.insert(cells[0].clone(), d);
        println!("{}- {}", "  ".repeat(d), cells[3]);
    }
}

// 画面での幅（日本語などの全角文字は 2、それ以外は 1）
fn width(s: &str) -> usize {
    s.chars()
        .map(|c| match c as u32 {
            0x1100..=0x115F | 0x2E80..=0xA4CF | 0xAC00..=0xD7A3 => 2,
            0xF900..=0xFAFF | 0xFE30..=0xFE4F | 0xFF00..=0xFF60 | 0xFFE0..=0xFFE6 => 2,
            _ => 1,
        })
        .sum()
}

// SQL を ; ごとの文に分ける（'…' や "…" の中の ; と、-- のコメントは区切りにしない）
fn split_statements(sql: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut chars = sql.chars().peekable();
    while let Some(c) = chars.next() {
        match quote {
            Some(q) => {
                current.push(c);
                if c == q {
                    quote = None;
                }
            }
            None if c == '\'' || c == '"' => {
                quote = Some(c);
                current.push(c);
            }
            None if c == '-' && chars.peek() == Some(&'-') => {
                // 行の終わりまでコメントなので読み飛ばす
                for c in chars.by_ref() {
                    if c == '\n' {
                        current.push('\n');
                        break;
                    }
                }
            }
            None if c == ';' => {
                push_statement(&mut statements, &current);
                current.clear();
            }
            None => current.push(c),
        }
    }
    push_statement(&mut statements, &current);
    statements
}

// 空行を取り除いてから、空でなければ文の一覧に加える
fn push_statement(statements: &mut Vec<String>, text: &str) {
    let lines: Vec<&str> = text
        .lines()
        .map(str::trim_end)
        .filter(|l| !l.trim().is_empty())
        .collect();
    if !lines.is_empty() {
        statements.push(lines.join("\n").trim().to_string());
    }
}
```

#### ▼ 道具の使い方

まず、試しに次の SQL を `sql/hello.sql` として保存します。`--` から行の終わりまではコメント（メモ書き）で、DB には送られません。文の終わりには `;`（セミコロン）を付けます。

```sql
-- はじめての SQL
SELECT 1 + 1 AS answer, 'こんにちは' AS greeting;
SELECT sqlite_version() AS version;
```

`sql-lab` のフォルダで、次のように実行します。

```powershell
cargo run --bin sql -- sql/hello.sql
```

**▼ 実行結果**

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target\debug\sql.exe sql/hello.sql`
sql> SELECT 1 + 1 AS answer, 'こんにちは' AS greeting
+--------+------------+
| answer | greeting   |
+--------+------------+
| 2      | こんにちは |
+--------+------------+
(1 行)

sql> SELECT sqlite_version() AS version
+---------+
| version |
+---------+
| 3.47.0  |
+---------+
(1 行)
```

- 1回目は、この表示の前に `Compiling ...` の行が 181 行続きます（libsql とその部品のコンパイル）。この PC では `Finished ... in 1m 02s` と、約1分かかりました。2回目からは、上の結果のように `Finished ... in 0.24s` とすぐに始まります。
- `Running` の行は、cargo が作った実行ファイル `target\debug\sql.exe` を、引数 `sql/hello.sql` を付けて起動した、という意味です。
- `sql> ` の後ろに実行した文の1行目が、その下に結果が表示されます。`SELECT` が返した行は `+----+` の線で囲んだ表で、最後に行数が `(1 行)` のように出ます。
- 2つ目の文の `sqlite_version()` は、中で動いている SQLite エンジンの版を返す関数です。

使い方は、次の4つを覚えておけば十分です。

| 書き方 | 意味 |
|---|---|
| `cargo run -q --bin sql -- sql/a.sql` | `sql/a.sql` を、メモリの中だけの DB で実行する（終わると DB は消える）。`-q` は cargo の `Compiling ...` などの表示を省く |
| `cargo run -q --bin sql -- sql/a.sql sql/b.sql` | 2つのファイルを続けて、同じ DB で実行する |
| `cargo run -q --bin sql -- --setup sql/schema.sql sql/a.sql` | 先に `sql/schema.sql` を結果を表示せずに実行し（表の準備）、それから `sql/a.sql` を実行する。`--setup` は何回でも付けられる |
| `cargo run -q --bin sql -- --db u05.db sql/a.sql` | メモリではなく、ファイル `u05.db` の DB で実行する（ファイルが無ければ作られ、終わっても残る） |

`--` は「ここから先は cargo ではなく、`sql` のプログラムに渡す引数です」という区切りです。

> **ヒント:** エラーになった文は `エラー: SQLite failure: ...` と表示され、次の文へ進みます。エラーで途中の文が飛ばされると、後ろの結果も変わることがあります。教材と結果が違うときは、まず上の方にエラーが出ていないかを確かめてください。

<!-- -->

> **ヒント:** VS Code で `.sql` のファイルを開くと、SQL のキーワードに色が付きます。SQL のキーワード（`SELECT`・`FROM` など）は大文字でも小文字でも同じ意味ですが、この巻では「キーワードは大文字、表や列の名前は小文字」で書きます。どこまでが SQL の決まりで、どこからが自分で付けた名前かが、ひと目で分かるからです。


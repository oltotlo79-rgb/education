## UNIT 04: プロジェクト構成と依存の向き

### この単元のゴール

- RustNote の `src/` にある 10 個のモジュールの役割を、それぞれ1文で説明できる
- モジュールを「段」に並べ、**上の段は下の段を使ってよい・下の段は上の段を使わない**という依存の向きの決まりを説明できる。RustNote に2つだけある例外と、その理由も言える
- 1つのパッケージの中に、ライブラリ・実行ファイル・`tests/`・`examples/` の別々のクレートがあることを説明し、`crate::` と `rustnote::` を使い分けられる
- 依存の向きの決まりをテストに書き、決まりを破る変更が入ったら `cargo test` で気づけるようにできる
- trait を間に挟んだおかげで、ノートの保存先をテスト用の物に差し替えられることを、実際に確かめられる

### たとえ話でつかむ

レストランを思い浮かべてください。お店の中では、仕事が係ごとに分かれています。

- **ホールの係**は、お客さんの注文を受けて、料理を運びます。
- **盛り付けの係**は、できた料理をお皿にきれいに並べます。
- **入口の受付**は、予約と会員証を確かめます。
- **倉庫の係**は、食材を倉庫から出し入れします。
- **レシピと食材の規格書**には、「卵は M サイズ」「スープは 80 度以上で出す」のような、お店の決まりが書いてあります。

この分け方には、大事な約束が2つあります。

1つ目は、**頼む向きが決まっている**ことです。ホールの係は、倉庫に直接入って食材を持ち出しません。倉庫の係に頼みます。倉庫の係は、どのテーブルのお客さんが何を注文したかを知りません。そして規格書は、ホールのテーブルの並びも、倉庫の棚の並びも知りません。だから、お店の内装を変えても、倉庫を別の建物に移しても、規格書は書き換えなくて済みます。

2つ目は、**例外は決めて、数を少なくする**ことです。盛り付けの係は、お皿に「3番テーブル」と札を立てます。盛り付けの係がホールの席の番号を知っているのは、頼む向きの決まりからすると例外です。それでも、札が無いと料理が届かないので、この例外だけは認めています。

アプリの `src/` も同じように分けます。

| レストラン | RustNote |
|---|---|
| 係ごとの仕事 | **モジュール**（`routes`・`views`・`auth`・`repository`・`domain` など） |
| ホールの係（注文を受けて運ぶ） | `routes`（URL ごとの処理） |
| 盛り付けの係 | `views`（画面の部品） |
| 入口の受付 | `auth`（パスワード・ログイン中のユーザー） |
| 倉庫の係と倉庫 | `repository`（データのしまい方）と `db`（DB への接続） |
| レシピと食材の規格書 | `domain`（検査済みの型と決まり） |
| 頼む向きの約束 | **依存の向き**（上の段は下の段を使ってよい。下の段は上の段を使わない） |
| お皿の「3番テーブル」の札 | 例外: `views` と `auth` が、リンク先の URL を作るために `routes` を指す |

> **用語:** 依存（いぞん：あるモジュールが、ほかのモジュールやクレートを使っていること。「`routes` は `auth` に依存している」＝ `routes` の中で `auth` の関数を使っている）

<!-- -->

> **用語:** 依存の向き（いぞんのむき：どのモジュールが、どのモジュールを使ってよいかの決まり。矢印の向きで描く）

<!-- -->

> **用語:** クレート（crate：Rust がひとまとめにコンパイルする単位。ライブラリ1つ、実行ファイル1つ、テストのファイル1つが、それぞれ1つのクレートになる。第4巻 UNIT 06）

### 図で理解する

RustNote の 10 個のモジュールを、使う向きに沿って6つの段に並べると、次の図のようになります。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 470" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0401" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
<marker id="ar-09-0402" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#c2410c"/></marker>
</defs>
<rect x="10" y="10" width="650" height="56" rx="8" fill="#f8fafc"/>
<rect x="10" y="78" width="650" height="56" rx="8" fill="#eff6ff"/>
<rect x="10" y="146" width="650" height="56" rx="8" fill="#eff6ff"/>
<rect x="10" y="214" width="650" height="56" rx="8" fill="#ecfdf5"/>
<rect x="10" y="282" width="650" height="56" rx="8" fill="#fff7ed"/>
<rect x="10" y="350" width="650" height="56" rx="8" fill="#f1f5f9"/>
<text x="24" y="43" font-size="12" font-weight="700" fill="#475569">① 入口</text>
<text x="24" y="111" font-size="12" font-weight="700" fill="#1e40af">② URL ごとの処理</text>
<text x="24" y="171" font-size="12" font-weight="700" fill="#1e40af">③ 画面の部品・</text>
<text x="24" y="189" font-size="12" font-weight="700" fill="#1e40af">ログイン</text>
<text x="24" y="247" font-size="12" font-weight="700" fill="#047857">④ 共有の状態</text>
<text x="24" y="315" font-size="12" font-weight="700" fill="#c2410c">⑤ 保存（DB）</text>
<text x="24" y="383" font-size="12" font-weight="700" fill="#334155">⑥ 土台</text>
<rect x="330" y="20" width="120" height="36" rx="6" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="390" y="43" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">main.rs</text>
<rect x="330" y="88" width="120" height="36" rx="6" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="390" y="111" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">routes</text>
<rect x="200" y="156" width="120" height="36" rx="6" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="260" y="179" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">views</text>
<rect x="360" y="156" width="120" height="36" rx="6" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="420" y="179" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">auth</text>
<rect x="520" y="156" width="120" height="36" rx="6" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="580" y="179" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">flash</text>
<rect x="330" y="224" width="120" height="36" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="390" y="247" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">state</text>
<rect x="260" y="292" width="120" height="36" rx="6" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="320" y="315" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">repository</text>
<rect x="420" y="292" width="120" height="36" rx="6" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="480" y="315" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">db</text>
<rect x="200" y="360" width="120" height="36" rx="6" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="260" y="383" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">domain</text>
<rect x="360" y="360" width="120" height="36" rx="6" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="420" y="383" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">error</text>
<rect x="520" y="360" width="120" height="36" rx="6" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="580" y="383" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">config</text>
<path d="M240,156 V106 H326" fill="none" stroke="#c2410c" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-09-0402)"/>
<line x1="420" y1="156" x2="420" y2="128" stroke="#c2410c" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-09-0402)"/>
<text x="470" y="102" font-size="11" fill="#c2410c">例外: リンク先の URL を作るため</text>
<text x="470" y="119" font-size="11" fill="#c2410c">views・auth が routes を指す</text>
<line x1="690" y1="20" x2="690" y2="400" stroke="#475569" stroke-width="3" marker-end="url(#ar-09-0401)"/>
<text x="690" y="424" font-size="12" font-weight="700" fill="#475569" text-anchor="middle">使う向き</text>
<text x="335" y="432" font-size="12" fill="#0f172a" text-anchor="middle">上の段は、下の段を使ってよい（同じ段どうしも使ってよい）。下の段は、上の段を使わない</text>
<line x1="190" y1="452" x2="240" y2="452" stroke="#c2410c" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-09-0402)"/>
<text x="250" y="456" font-size="12" fill="#c2410c">下の段から上の段を指す例外（RustNote では2本だけ）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: RustNote のモジュールの段。上の段は下の段を使ってよく、下の段は上の段を知らない。例外は、リンク先の URL を作るために views と auth が routes を指す2本だけ</div>
</div>

- **① 入口**の `main.rs` は、設定を読み、DB を開き、ルーターを組み立てて待ち受けるだけです。中身の処理は書きません。
- **② URL ごとの処理**の `routes` は、リクエストを受け取って、ログインを確かめ（③ `auth`）、データを読み書きし（④ `state` のリポジトリ）、画面を組み立てて（③ `views`）返します。いちばん多くのモジュールを使う、まとめ役です。
- **③ 画面の部品・ログイン**の `views`・`auth`・`flash` は、`routes` から呼ばれる部品です。
- **④ 共有の状態**の `state` は、リポジトリを `AppState` にまとめて、どのページからも借りられるようにします。
- **⑤ 保存（DB）**の `repository` と `db` は、SQL を書き、DB に接続します。画面のことは何も知りません。
- **⑥ 土台**の `domain`・`error`・`config` は、検査済みの型、エラーの分類、設定です。中でも `domain` は、RustNote のほかのモジュールを1つも使いません。

下の段ほど、**変わりにくく、たくさんの場所から使われる**ものになっています。`NoteTitle` の決まり（1〜80 文字）は、画面のデザインを変えても、DB を Turso に移しても変わりません。変わりにくいものを、変わりやすいもの（画面や URL）に依存させないのが、段に分ける目的です。

もう1つ、知っておきたいことがあります。RustNote の「パッケージ」（`Cargo.toml` が1つのまとまり）の中には、クレートがいくつもあります。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0403" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="10" width="700" height="280" rx="10" fill="#ffffff" stroke="#94a3b8" stroke-width="2" stroke-dasharray="6 4"/>
<text x="360" y="34" font-size="13" font-weight="700" fill="#475569" text-anchor="middle">パッケージ rustnote（Cargo.toml が1つ）</text>
<rect x="250" y="96" width="220" height="118" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="122" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">ライブラリのクレート</text>
<text x="360" y="144" font-size="12" fill="#0f172a" text-anchor="middle">src/lib.rs が入口</text>
<text x="360" y="166" font-size="12" fill="#0f172a" text-anchor="middle">10 個のモジュール</text>
<text x="360" y="196" font-size="12" fill="#1e40af" text-anchor="middle">中では crate::domain::…</text>
<rect x="30" y="56" width="170" height="64" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="115" y="82" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">実行ファイル</text>
<text x="115" y="104" font-size="12" fill="#475569" text-anchor="middle">src/main.rs</text>
<rect x="30" y="196" width="170" height="64" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="115" y="222" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">examples/*.rs</text>
<text x="115" y="244" font-size="12" fill="#475569" text-anchor="middle">1ファイルが1クレート</text>
<rect x="520" y="123" width="170" height="64" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="605" y="149" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">tests/*.rs</text>
<text x="605" y="171" font-size="12" fill="#475569" text-anchor="middle">1ファイルが1クレート</text>
<line x1="200" y1="96" x2="246" y2="120" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0403)"/>
<line x1="200" y1="220" x2="246" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0403)"/>
<line x1="520" y1="155" x2="474" y2="155" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0403)"/>
<text x="360" y="246" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">外から使うときの名前</text>
<text x="360" y="268" font-size="12" fill="#0f172a" text-anchor="middle">rustnote::domain::NoteTitle（公開した物だけ）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: 1つのパッケージに、クレートがいくつもある。ライブラリの中では crate:: で、外（main.rs・tests・examples）からはパッケージの名前 rustnote:: で、同じ部品を指す</div>
</div>

- アプリの中身は、すべて**ライブラリのクレート**（`src/lib.rs` が入口）に書いてあります。その中のファイルどうしは、`crate::domain::NoteTitle` のように、`crate::`（このクレートの根元）から道のりを書きます。
- `src/main.rs` は、ライブラリとは**別のクレート**（実行ファイル）です。`tests/` と `examples/` のファイルも、1つずつが別のクレートです。これらはライブラリを外から使うので、`rustnote::domain::NoteTitle` のように、パッケージの名前から書きます。外から見えるのは、`pub` を付けて公開した物だけです。
- 中身をライブラリに置いて、`main.rs` を薄くしてあるのは、**テストから同じ部品を使うため**です。もし処理を `main.rs` に書いてしまうと、`tests/` のファイルからは使えません（`main.rs` のクレートは、外から `use` できないからです）。

### まず動かす最小コード

まず、モジュールごとの役割と、「使ってよいもの」を表にします。これが、この単元の設計書です。

| 段 | モジュール | 役割（1文で） | 使ってよいモジュール |
|---|---|---|---|
| ① | `main.rs` | 設定・DB・ルーターを準備して待ち受ける | 全部（ただし `rustnote::` の公開された物だけ） |
| ② | `routes` | URL ごとに、リクエストを受けて返事を作る | ③〜⑥ |
| ③ | `views` | 画面の部品とエラー画面を組み立てる | `flash`・⑥（例外: `routes` へのリンク） |
| ③ | `auth` | パスワードのハッシュ化とログイン中のユーザー | `flash`・④・⑥（例外: `routes` へのリンク） |
| ③ | `flash` | 「保存しました」を一度だけ出す | ⑥ |
| ④ | `state` | リポジトリを `AppState` にまとめる | ⑤・⑥ |
| ⑤ | `repository` | ノート・ユーザー・セッションの読み書き（SQL） | `db`・⑥ |
| ⑤ | `db` | DB への接続とマイグレーション | ⑥ |
| ⑥ | `error` | エラーの分類と HTTP のステータスの対応 | `domain` |
| ⑥ | `domain` | 検査済みの型と、アプリの決まり | なし |
| ⑥ | `config` | 環境変数から設定を読む | なし |

表を書いたら、本当にそのとおりになっているかを確かめます。人の目で 24 個のファイルの `use` を読み比べるのは大変なので、プログラムに数えさせましょう。

#### 例4-1: モジュールどうしの依存を一覧にする

> **▼ このコードがやること（先に日本語で）:** RustNote の `src/` の下にある `.rs` ファイルを全部読みます。ファイルの場所から、どのモジュールのファイルかを決め（`src/routes/notes.rs` なら `routes`）、行の中に `crate::auth` のような「ほかのモジュールへの道のり」と、`libsql::` のような「外のクレートの名前」が書いてあるかを調べます。最後に、モジュールごとの一覧と、互いに使い合っている組を表示します。

```rust
@@FILE:book/rustnote/examples/u04_deps.rs@@
```

コピーした `rustnote` フォルダの `examples/u04_deps.rs` として保存して実行します。

```powershell
cargo run -q --example u04_deps
```

**▼ 実行結果**

```text
@@OUT:u04_deps@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: ファイルの場所が、モジュールの場所

- Rust では、`src/lib.rs` に `pub mod routes;` と書くと、`src/routes.rs` が `routes` モジュールになります。`src/routes.rs` の中に `pub mod health;` と書くと、`src/routes/health.rs` がその子のモジュールになります（第4巻 UNIT 06）。つまり、`src/routes/` の下のファイルは、どれも `routes` の一部です。`module_of` は、このきまりを使って、`src/` から数えて最初のフォルダ名かファイル名を、モジュールの名前にしています。
- 24 個のファイルは、10 個のモジュールのファイル 22 個と、`lib.rs`・`main.rs` です。`lib.rs` は部品の一覧、`main.rs` は入口なので、この一覧には入れていません。

##### 解説2: 一覧は、図の段のとおりになっている

- `domain` は `→ （なし）` で、外のクレートも `thiserror`（エラー型を短く書く道具）だけです。RustNote のほかのモジュールも、Topcoat も libsql も使っていません。⑥の土台の中でも、いちばん下にあることが確かめられました。
- `libsql` を使っているのは、`db`・`repository`・`error` の3つだけです。`error` が `libsql` を使うのは、libsql のエラーを `RepoError` に変える `From<libsql::Error>` があるからです（UNIT 03）。画面の側（`routes`・`views`）は、SQL のクレートを使っていません。
- `repository` は `db`・`domain`・`error` だけを使い、`routes` や `views` を使っていません。データのしまい方の係は、画面のことを知らないのです。
- 外のクレートを見ると、パスワードのハッシュの `argon2` は `auth` だけ、フォームを読む `serde` は `routes` だけが使っています。1つの仕事に使う道具が、1つのモジュールに集まっています。

##### 解説3: 互いに使い合っている組が、2つある

- `auth ⇄ routes` と `routes ⇄ views` が、図の例外の2本です。`routes` は `auth` と `views` を使い、`auth` と `views` も `routes` を使っています。
- `views` が `routes` を使っている場所は、`href!(crate::routes::notes::index)` のような**リンクの URL を作る**ところです。`href!` は、ページの関数を指定して URL を作るマクロで、関数の名前を書き間違えたり、ページを消したりすると、コンパイルエラーになります。URL を文字列 `"/notes"` で書くと、この確かめが働きません。`auth` も、ログインしていない人をログイン画面へ送るところ（`require_user`）で、同じように `href!` を使っています。
- 同じクレートの中なら、モジュールどうしが互いに使い合っても、コンパイルできます。ただし、クレートを分けるとそうはいきません（エラー3で確かめます）。RustNote では「リンクの URL を作るときだけ」と理由を決めて、例外を2本にとどめています。

##### 解説4: 文字を数えるだけなので、見落とすものもある

- このプログラムは、行に書かれた**文字**を数えているだけです。コメントの行は飛ばしていますが、行の後ろに付けたコメントや、文字列の中の名前までは見分けません。
- また、型の名前を書かずに使う依存は見つけられません。たとえば `routes/health.rs` は、`app_state(cx).db.ping()` で DB を直接使っていますが、`crate::db` とは書いていないので、一覧の `routes` に `db` は出てきません。`AppState` の `db` は、「ヘルスチェックで DB の生死を確かめるため」とコメントに書いて、わざと持たせているものです（UNIT 10（準備中））。
- だから、この一覧は「おおよその地図」です。決まりを守れているかの最後の確かめは、人のレビューです。それでも、地図があると、レビューで見るべき場所がすぐ分かります。

### もう一歩: 実務に近い形

一覧を出して目で見るだけだと、誰かが決まりを破る変更を入れても、一覧を見直すまで気づけません。そこで、決まりを**テスト**にします。`cargo test` を実行するたびに、依存の向きも確かめられます。

#### 例4-2: 依存の向きの決まりを、テストにする

> **▼ このコードがやること（先に日本語で）:** 例4-1と同じ方法で、モジュールごとに「使っているモジュール」と「使っている外のクレート」を集めます。それを、次の5つの決まりと照らし合わせ、破っていたらテストを失敗させます。R-1: `domain` はほかを使わない。R-2: `libsql` は `db`・`repository`・`error` だけ。R-3: `repository` は画面とログインの部品を使わない。R-4: `routes` は `db` を直接使わない。R-5: `argon2` は `auth` だけ。

```rust
@@FILE:book/architecture_base.rs@@
```

`tests/architecture.rs` として保存して実行します。

```powershell
cargo test --test architecture
```

**▼ 実行結果**

```text
@@OUT:u04_arch@@
```

- 5つとも `ok` です。今の RustNote は、表に書いた決まりを守っています。
- 失敗したときに何が起きるかは、エラー4で、わざと決まりを破って確かめます。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 決まりに番号を付けて、テストの名前にそろえる

- テストの名前 `r1_domain_stands_alone` の `r1` は、決まりの番号 R-1 です。UNIT 01 で受入条件の番号（AC-05 など）をテストの名前に入れたのと同じ考え方です。失敗したときに、どの決まりが破られたのかが、名前だけで分かります。
- `assert!` の2つ目の引数は、失敗したときに表示する文です。「`routes が libsql を使っています。SQL は repository に書きます`」のように、**何が起きたか**と**どう直すか**を書いておくと、決まりを知らない人が失敗に出会っても、直し方が分かります。

##### 解説2: 「使ってはいけないもの」を並べる

- R-2 と R-5 は、全部のモジュールを順に調べて、許した物以外が `libsql`・`argon2` を使っていないかを確かめます。新しいモジュールを足したら、`MODULES` にも足します（足し忘れると、そのモジュールは調べられません）。
- R-3 は、`repository` が使ってはいけない5つのモジュールを並べています。「使ってよいもの」ではなく「使ってはいけないもの」を書いておくと、決まりの意図（倉庫の係は、ホールの仕事を知らない）が読み取りやすくなります。

##### 解説3: テストは、コンパイルしなくても src/ を読める

- `env!("CARGO_MANIFEST_DIR")` は、`Cargo.toml` があるフォルダの場所に置き換わるマクロです。テストを、どのフォルダから実行しても、RustNote の `src/` を見つけられます。
- このテストは、RustNote の関数を1つも呼んでいません。ファイルを文字として読んでいるだけなので、1秒もかからずに終わります。

続いて、段に分けたことで、何ができるようになったのかを確かめます。`routes` は、ノートの保存先を `app_state(cx).notes` を通して使っていました。この `notes` の型は `Arc<dyn NoteRepository + Send + Sync>` です。`routes` が知っているのは、**`NoteRepository` という約束（trait）だけ**で、その中身が libSQL かどうかは知りません（`dyn` で使う trait は第4巻 UNIT 04、リポジトリの形は第7巻 UNIT 09）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0404" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="90" width="210" height="70" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="125" y="116" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">routes/notes.rs</text>
<text x="125" y="140" font-size="11" fill="#0f172a" text-anchor="middle">app_state(cx).notes.search(…)</text>
<rect x="280" y="90" width="200" height="70" rx="8" fill="#ffffff" stroke="#475569" stroke-width="2"/>
<text x="380" y="116" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">NoteRepository</text>
<text x="380" y="140" font-size="11" fill="#475569" text-anchor="middle">約束（trait）: insert・find・search…</text>
<rect x="520" y="26" width="190" height="64" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="615" y="52" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">LibsqlNoteRepository</text>
<text x="615" y="74" font-size="11" fill="#0f172a" text-anchor="middle">本番・開発（SQL で読み書き）</text>
<rect x="520" y="160" width="190" height="64" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="615" y="186" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">BrokenNotes</text>
<text x="615" y="208" font-size="11" fill="#0f172a" text-anchor="middle">テスト用（いつも失敗する）</text>
<line x1="230" y1="125" x2="276" y2="125" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0404)"/>
<text x="255" y="80" font-size="11" fill="#475569" text-anchor="middle">約束だけを</text>
<text x="255" y="96" font-size="11" fill="#475569" text-anchor="middle">知っている</text>
<path d="M520,58 H500 V112 H484" fill="none" stroke="#10b981" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-09-0404)"/>
<path d="M520,192 H500 V138 H484" fill="none" stroke="#f97316" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-09-0404)"/>
<text x="500" y="250" font-size="11" fill="#475569" text-anchor="middle">どちらも約束を守る（impl NoteRepository for …）</text>
<text x="125" y="196" font-size="11" fill="#0f172a" text-anchor="middle">AppState の notes の型:</text>
<text x="125" y="214" font-size="11" fill="#0f172a" text-anchor="middle">Arc&lt;dyn NoteRepository + Send + Sync&gt;</text>
<text x="125" y="238" font-size="11" fill="#475569" text-anchor="middle">→ 入れ物を差し替えられる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: routes は約束（trait）だけを知っている。AppState の notes に入れる物を差し替えれば、routes を1行も変えずに、テスト用の保存先で動かせる</div>
</div>

#### 例4-3: ノートの保存先を、壊れた物に差し替える

UNIT 01 の練習問題の解答では、「障害」の枠に AC-15「DB が故障しているときに保存すると、500 で『時間をおいてもう一度お試しください』とだけ出す」を書きました。本物の DB をわざと壊すのは大変ですが、保存先を差し替えれば簡単に確かめられます。ここでは、まず一覧の画面で確かめます（保存の場面は問3で確かめます）。

> **▼ このコードがやること（先に日本語で）:** どの操作をしても `RepoError` を返す `BrokenNotes` を作り、`NoteRepository` の約束を守らせます（`impl`）。RustNote と同じ `AppState::new` で状態を作ったあと、`notes` だけを `BrokenNotes` に差し替えて、ルーターを組み立てます。ユーザーの登録とログインは本物の DB で動くので、登録してから、ノートの一覧（`GET /notes`）を開きます。500 になり、壊れた理由が画面に出ないことを確かめます。

```rust
@@FILE:book/rustnote/tests/swap.rs@@
```

`tests/swap.rs` として保存し、`println!` やサーバーのログも見えるように `--nocapture` を付けて実行します。

```powershell
cargo test --test swap -- --nocapture
```

**▼ 実行結果**

```text
@@OUT:u04_swap@@
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 差し替えたのは1行だけ

- `state.notes = Arc::new(BrokenNotes);` の1行で、ノートの保存先だけが入れ替わりました。`routes` のコードは1行も変えていません。`routes` が `LibsqlNoteRepository` という名前を知らず、約束（`NoteRepository`）だけを知っているから、こうして差し替えられます。
- ユーザー（`users`）とセッション（`sessions`）の保存先は、本物のままです。だから登録（`POST /signup -> 303`）は成功し、ログインした状態で一覧を開けました。「ノートの保存先だけが壊れた」という場面を、正確に作れています。
- `AppState` の項目は `pub` なので、テストから差し替えられます。`AppState::new` で本物を作ってから、1つだけ替えるのが、いちばん短い書き方です。

##### 解説2: 利用者に見せる文と、ログに書く文を分けている

- 画面（`html`）には「時間をおいてもう一度お試しください」だけが出て、「ディスクが壊れました」は出ていません。`assert!(!html.contains(...))` の `!` は「含まれていない」ことを確かめています。
- 一方、サーバーのログ（標準エラー出力）には、`[error] サーバーの中でエラーが起きました <- DB の値を読み取れません: ディスクが壊れました（テストのための失敗）` と、原因が `<-` でつながって書かれています。運用する人は、ログで原因を探せます。
- これは `views` の `error_page` が、500 のときだけ原因をログに書き（`log_if_internal`）、画面には `public_message()` だけを出しているからです（UNIT 03 の `AppError`）。中の事情を利用者に見せないのは、攻撃の手がかりを与えないためでもあります。

##### 解説3: 約束の中身を全部書く

- `impl NoteRepository for BrokenNotes` では、`insert`・`find`・`search`・`update`・`toggle_done`・`delete` の6つを、全部書く必要があります。1つでも足りないと、`error[E0046]: not all trait items implemented`（約束のメソッドが全部はそろっていません）というコンパイルエラーになり、続きの `missing: delete` のように、足りないメソッドの名前も教えてくれます。
- 引数の名前を `_` にしているのは、使わない引数だからです。名前を付けて使わないと、「使っていない変数」の警告が出ます。
- `POST /signup` の時間（この PC では約1秒）は、パスワードのハッシュを計算する時間です。テストは debug ビルドで動くので、本番（release ビルド）より遅くなります。時間は実行のたびに変わります。

### よくあるエラーと読み方

#### エラー1: lib の外から crate:: で読み込んだ（E0432）

`examples/` のプログラムで、`src/` の中と同じつもりで `crate::` を書いた場合です（`examples/e04_crate.rs` に保存して試します）。

```rust
@@FILE:book/errs04/e04_crate_wrong.rs@@
```

```powershell
cargo build --example e04_crate
```

```text
@@OUT:u04_err_crate@@
```

- **何が起きたか:** 「`crate::domain` を解決できません。クレートの根元に `domain` が見つかりません」というエラーです。`examples/e04_crate.rs` は、ライブラリとは別のクレートです。このクレートの根元（`crate::`）は `e04_crate.rs` そのもので、そこには `domain` というモジュールはありません。
- **直し方:** ライブラリの外からは、パッケージの名前を使って `use rustnote::domain::NoteTitle;` と書きます。直して実行すると、次のように表示されます。確かめ終わったら、`examples/e04_crate.rs` は消しておきます。

```text
@@OUT:u04_fixed_crate@@
```

- `src/main.rs` も、ライブラリとは別のクレートです。RustNote の `main.rs` が `use rustnote::{config::{self, Config}, db::{self, Db}, routes, state::AppState};` と書いているのは、このためです。

#### エラー2: ファイルの場所のとおりに道のりを書いた（E0603）

`LibsqlNoteRepository` は `src/repository/notes.rs` にあるので、その場所のとおりに道のりを書いた場合です（`examples/e04_private.rs`）。

```rust
@@FILE:book/errs04/e04_private_wrong.rs@@
```

```powershell
cargo build --example e04_private
```

```text
@@OUT:u04_err_private@@
```

- **何が起きたか:** 「モジュール `notes` は private（非公開）です」というエラーです。`note:` が `src/repository.rs` の 15 行目 `mod notes;` を指しています。`pub` が付いていないので、`repository` の外からは `notes` の中に入れません（第4巻 UNIT 06 のエラー2）。
- `struct LibsqlNoteRepository is not publicly re-exported` は、「`notes` の中の `LibsqlNoteRepository` は、この道のりでは公開されていません」という意味です。
- **直し方:** `src/repository.rs` は、`pub use notes::{LibsqlNoteRepository, NoteRepository};` で、`repository` の直下に入口を作っています。`use rustnote::repository::LibsqlNoteRepository;` と書けば使えます。直して実行すると、型の本当の場所が表示されます。確かめ終わったら、`examples/e04_private.rs` は消しておきます。

```text
@@OUT:u04_fixed_private@@
```

- 型そのものは `notes` の中にありますが、外の人には `repository` の入口から使ってもらいます。こうしておくと、あとで `notes.rs` を2つのファイルに分けても、外の `use` を書き換えずに済みます。**ファイルの並び（中の都合）と、公開する道のり（外との約束）を分ける**のが、`pub use` の役目です。

#### エラー3: クレートを分けたら、循環になった

モジュールが互いに使い合うのは、同じクレートの中なら許されます。では、`views` を別のパッケージ（クレート）に分けたらどうなるでしょう。小さな workspace で試します。`app` が画面の部品を使うために `views` に依存し、`views` がリンク先の URL を作るために `app` に依存する、という形です。

```text
cycle_ws/
├── Cargo.toml        [workspace] members = ["app", "views"]
├── app/Cargo.toml    [dependencies] views = { path = "../views" }
└── views/Cargo.toml  [dependencies] app = { path = "../app" }
```

```rust
// views/src/lib.rs: トップページへのリンクを、app の関数で作る
pub fn back_link() -> String {
    format!("<a href=\"{}\">トップページへ戻る</a>", app::home_path())
}
```

```powershell
cargo build
```

```text
@@OUT:u04_err_cycle@@
```

- **何が起きたか:** 「パッケージの依存が循環しています。`app` が自分自身に依存しています」というエラーです。その下の行が、循環の道のりです。`app` は `views` の path 依存を満たし、`views` は `app` の path 依存を満たす、つまり `app → views → app` と一周しています。
- このエラーは、コンパイルが始まる前（Cargo が依存を調べる段階）で出ます。クレートは1つずつ順番にコンパイルされるので、「`app` をコンパイルするには先に `views` が要り、`views` には先に `app` が要る」という形は、どちらからも始められないのです。
- **直し方:** 循環の1本を切ります。たとえば、リンク先の URL を `views` が自分で作るのをやめて、`app` が作った URL を引数で渡すようにします。もう1つの方法は、両方が使う物（URL の一覧）を3つ目のクレートに出して、`app` と `views` の両方がそれを使う形にすることです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0405" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
<marker id="ar-09-0406" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#dc2626"/></marker>
</defs>
<rect x="10" y="10" width="340" height="210" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="180" y="36" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">同じクレートの中（今の RustNote）</text>
<rect x="40" y="80" width="110" height="44" rx="6" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="95" y="107" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">routes</text>
<rect x="210" y="80" width="110" height="44" rx="6" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="265" y="107" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">views</text>
<line x1="150" y1="94" x2="206" y2="94" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0405)"/>
<line x1="210" y1="112" x2="154" y2="112" stroke="#475569" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-09-0405)"/>
<text x="180" y="160" font-size="12" fill="#0f172a" text-anchor="middle">モジュールは、まとめて1回で</text>
<text x="180" y="178" font-size="12" fill="#0f172a" text-anchor="middle">コンパイルされる</text>
<text x="180" y="204" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">→ コンパイルできる</text>
<rect x="370" y="10" width="340" height="210" rx="10" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="540" y="36" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">クレートを分けると</text>
<rect x="400" y="80" width="110" height="44" rx="6" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="455" y="107" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">app</text>
<rect x="570" y="80" width="110" height="44" rx="6" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<text x="625" y="107" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">views</text>
<line x1="510" y1="94" x2="566" y2="94" stroke="#dc2626" stroke-width="2" marker-end="url(#ar-09-0406)"/>
<line x1="570" y1="112" x2="514" y2="112" stroke="#dc2626" stroke-width="2" marker-end="url(#ar-09-0406)"/>
<text x="540" y="160" font-size="12" fill="#0f172a" text-anchor="middle">クレートは1つずつ順番に</text>
<text x="540" y="178" font-size="12" fill="#0f172a" text-anchor="middle">コンパイルされる</text>
<text x="540" y="204" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">→ cyclic package dependency</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: 互いに使い合う形は、同じクレートの中ならコンパイルできるが、クレートを分けると循環のエラーになる。あとでクレートに分けたくなる部品ほど、例外の矢印を少なくしておく</div>
</div>

- RustNote の `routes ⇄ views` は、今は同じクレートの中なので問題ありません。それでも、いつか画面の部品を別のクレートに分けたくなったら、この例外の矢印が最初の障害になります。例外を「リンクの URL を作るときだけ」と決めて数を少なくしているのは、そのときに切る場所を少なくするためでもあります。

#### エラー4（エラーが出ない間違い）: ルートの中に SQL を直接書いた

急いでいて、ノートの一覧の処理（`src/routes/notes.rs` の `notes_index`）に、件数を数える SQL を直接書き足してしまった場合です。試すときは、書き足して確かめたら、必ず元に戻してください（この巻では `src/` を変えません）。

```rust
    let page = app_state(cx)
        .notes
        .search(owner, &query)
        .await
        .map_err(AppError::from)?;
    // 急いで足した行: 自分のノートの件数を、SQL で直接数えてログに出す
    let conn = app_state(cx).db.connect().await.map_err(AppError::internal)?;
    let mut rows = conn
        .query(
            "SELECT COUNT(*) FROM notes WHERE user_id = ?1",
            libsql::params![owner.get()],
        )
        .await
        .map_err(AppError::internal)?;
    if let Some(row) = rows.next().await.map_err(AppError::internal)? {
        eprintln!("ノートの件数: {:?}", row.get::<i64>(0));
    }
    let keyword = query.keyword.clone();
```

まず、ビルドします。

```powershell
cargo build
```

```text
@@OUT:u04_violation_build@@
```

エラーも警告も出ません。持ち主の条件（`user_id = ?1`）も、bind（`?1` に値を別に渡す）も、ちゃんと書いてあります。コンパイラから見れば、正しいプログラムです。次に、例4-2のテストを実行します。

```powershell
cargo test --test architecture
```

```text
@@OUT:u04_violation_test@@
```

- **何が起きたか:** R-2 のテストが失敗し、`routes が libsql を使っています。SQL は repository に書きます` と表示されました。`panicked at tests\architecture.rs:122:9` は、テストのファイルの 122 行目（R-2 の `assert!`）で失敗したという意味です。`thread '…' (80420)` の数は、実行のたびに変わります。
- **なぜ困るのか:** コンパイルが通っても、SQL が `routes` と `repository` の2か所に散らばります。たとえば、あとでノートに「削除済み」の印を付ける仕組みを足して、`repository` の SQL に `AND deleted = 0` を足したとします。`routes` に書いた件数の SQL は直し忘れて、消したノートまで数えてしまいます。UNIT 03 の I-5（他人のノートは見えない）も、SQL が1か所に集まっているから、テストで守り切れるのです。
- **直し方:** 件数を数える処理は、`NoteRepository` に `count` のようなメソッドとして足し、`LibsqlNoteRepository` に SQL を書きます。`routes` からは `app_state(cx).notes.count(owner)` のように呼びます。RustNote では、`search` が返す `NotePage` の `total_count`（条件に合うノートの総数）に件数が入っているので、新しいメソッドも要りません。確かめ終わったら、`src/routes/notes.rs` を元に戻してください。
- **このテストの限界:** `libsql::params![...]` の代わりに `[owner.get()]` と書くと、`libsql::` の文字が出てこないので、R-2 は気づきません（例4-1の解説4）。決まりのテストは、レビューの代わりではなく、**レビューで見落としにくくする道具**です。

### RustNote ではここで使う

`src/lib.rs` の先頭のコメントには、モジュールの役割が一覧で書いてあります。新しく読む人は、まずここを見ます。

```rust
//! RustNote のライブラリ部分（アプリ本体）。
//!
//! 実行ファイル（src/main.rs）とテスト（tests/）の両方から使えるように、
//! アプリの中身はすべてライブラリとして書き、ここで部品（モジュール）を公開する。
```

- 「テストから使えるように、中身はライブラリに書く」という、この単元の図の理由が、そのまま書いてあります。`main.rs` は 54 行しかなく、その多くはコメントです。

`src/routes.rs` の先頭には、ファイルと URL の対応の表があります（抜き出しています）。

```rust
//! Topcoat のモジュールルーティングを使う。`module_router!()` を呼んだこのモジュールが `/` になり、
//! 子モジュールが URL の区切りを1つずつ足していく（ファイルの並び＝URL の並び）。
//!
//! | ファイル | URL |
//! |---|---|
//! | routes.rs                 | GET /（トップ）、全ページ共通のレイアウト、404 の受け皿 |
//! | routes/health.rs          | GET /health |
//! | routes/notes.rs           | GET /notes（?q= 検索・?page= ページ送り）・POST /notes |
//! | routes/notes/id.rs        | GET /notes/{id} |
//! | routes/notes/id/toggle.rs | POST /notes/{id}/toggle |
```

- `routes` の中だけは、**URL の並びのとおりに**フォルダを分けています。Topcoat のモジュールルーティングでは、`routes/notes/id/toggle.rs` が、そのまま `/notes/{id}/toggle` になります。URL を見れば、直すファイルがすぐに分かります。
- 一方、`src/` の直下は、**仕事の種類（段）で**分けています。「URL ごと」と「仕事の種類ごと」の2つの分け方を、場所によって使い分けているのです。

新しい機能を足すときは、図の段を**下から上へ**たどって、ファイルを決めます。たとえば「ノートにタグを付ける」機能なら、次のようになります（RustNote にはまだ無い機能です。置き場所の考え方の例として見てください）。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0407" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="264" width="420" height="40" rx="6" fill="#f1f5f9" stroke="#64748b" stroke-width="2"/>
<text x="34" y="289" font-size="12" fill="#0f172a">① domain.rs: TagName::parse（1〜20 文字）</text>
<rect x="20" y="214" width="420" height="40" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="34" y="239" font-size="12" fill="#0f172a">② migrations/0004_tags.sql: tags と note_tags の表</text>
<rect x="20" y="164" width="420" height="40" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="34" y="189" font-size="12" fill="#0f172a">③ repository/tags.rs: TagRepository と libSQL 版</text>
<rect x="20" y="114" width="420" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="34" y="139" font-size="12" fill="#0f172a">④ state.rs: AppState に tags を足す</text>
<rect x="20" y="64" width="420" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="34" y="89" font-size="12" fill="#0f172a">⑤ views.rs: タグの一覧の部品</text>
<rect x="20" y="14" width="420" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="34" y="39" font-size="12" fill="#0f172a">⑥ routes/notes/id/tags.rs: POST /notes/{id}/tags</text>
<line x1="460" y1="300" x2="460" y2="22" stroke="#475569" stroke-width="3" marker-end="url(#ar-09-0407)"/>
<text x="472" y="164" font-size="12" font-weight="700" fill="#475569">下から順に作る</text>
<rect x="560" y="264" width="150" height="40" rx="6" fill="#ffffff" stroke="#94a3b8" stroke-dasharray="5 4"/>
<text x="635" y="289" font-size="11" fill="#475569" text-anchor="middle">ユニットテスト</text>
<rect x="560" y="164" width="150" height="40" rx="6" fill="#ffffff" stroke="#94a3b8" stroke-dasharray="5 4"/>
<text x="635" y="189" font-size="11" fill="#475569" text-anchor="middle">tests/repository.rs</text>
<rect x="560" y="14" width="150" height="40" rx="6" fill="#ffffff" stroke="#94a3b8" stroke-dasharray="5 4"/>
<text x="635" y="39" font-size="11" fill="#475569" text-anchor="middle">tests/http.rs</text>
<text x="635" y="122" font-size="11" fill="#475569" text-anchor="middle">同じ高さの段で</text>
<text x="635" y="140" font-size="11" fill="#475569" text-anchor="middle">テストを足す</text>
<text x="230" y="324" font-size="11" fill="#475569" text-anchor="middle">下の段は、上の段が無くてもテストできる</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: 新しい機能（例: タグ）を足すときの置き場所。下の段から順に作り、段ごとにテストを足す。下の段は上の段を使わないので、画面ができる前から確かめられる</div>
</div>

- 下から作るのは、下の段が上の段を使わないからです。`TagName::parse` は、画面も DB も無くてもテストできます。`TagRepository` は、画面が無くても `tests/repository.rs` の形で確かめられます。
- 上から作る（先に画面を作る）こともできますが、そのときは、下の段の代わりに仮の物（例4-3の `BrokenNotes` のような、テスト用の実装）を置くことになります。どちらの順番でも、**どの段に何を書くか**は同じです。
- 機能を足し終えたら、例4-1の一覧を出し直し、例4-2のテストが通ることも確かめます。新しいモジュールを足したら、`MODULES` にも忘れずに足します。

### 練習問題

**問1:** ノートの詳細の画面に「複製」ボタンを付け、押すと同じタイトルと本文の新しいノートを作って、その詳細へ移る機能を足すことにしました。URL は `POST /notes/{id}/duplicate` で、他人のノートは 404 にします。

1. どのファイルに何を書けばよいかを、図の段に沿って答えてください。新しく作るファイルと、書き足すファイルを分けて答えます。
2. `NoteRepository` に新しいメソッドは要るでしょうか。理由も答えてください。

**問2:** 例4-2のテストに、決まり R-6「`views`（画面の部品）は、`db`・`repository`・`state` も `libsql` も使わない。渡された値を表示するだけ」を足してください。今の RustNote で、このテストは通るでしょうか。

**問3:** 例4-3の差し替えを使って、ノートの保存先の `insert` が失敗するとき、ノートの作成（`POST /notes`）が 500 になり、画面に「時間をおいてもう一度お試しください」だけが出ることを確かめるテストを書いてください。入力したタイトルも、壊れた理由も、画面に出ないことを確かめます。

### 解答と解説

**問1の解答:**

1. 次のとおりです。

| 段 | ファイル | 書くこと |
|---|---|---|
| ⑥ 土台 | （なし） | 複製は、今ある `NewNote` と `NoteTitle`・`NoteBody` の決まりのままで作れる |
| ⑤ 保存 | （なし） | 今ある `find` と `insert` を組み合わせる |
| ② URL | 新しく作る: `src/routes/notes/id/duplicate.rs` | `POST` のページ。自分のノートを探し、同じ内容で `insert` して、303 で新しいノートの詳細へ |
| ② URL | 書き足す: `src/routes/notes/id.rs` | `pub mod duplicate;` と、詳細の画面の「複製」ボタン（`href!(duplicate::duplicate, Id(id))` へ送るフォーム） |
| テスト | 新しく作る: `tests/` のテスト | 自分のノートは複製できて 303、他人のノートは 404 |

2. 要りません。複製は「自分のノートを1件探す（`find`）」と「ノートを1件足す（`insert`）」の組み合わせでできるからです。`find` は持ち主の条件付きなので、他人のノートは見つからず、404 になります。保存先の約束（trait）を増やさずに済むと、例4-3の `BrokenNotes` のようなテスト用の実装も、書き直さずに済みます。

この解答を、`src/` を変えてもかまわない**別のコピー**で、実際に作って確かめた結果です（この巻で使っているコピーの `src/` は変えません）。新しく作った `src/routes/notes/id/duplicate.rs` です。

```rust
@@FILE:dup_probe/rustnote/src/routes/notes/id/duplicate.rs@@
```

`src/routes/notes/id.rs` には、`pub mod delete;` の次の行に `pub mod duplicate;` を足し、詳細の画面の「削除」のフォームの前に、次のフォームを足しました。

```rust
            <form method="post" action=(href!(duplicate::duplicate, Id(id)))>
                <button type="submit" class="secondary">"複製"</button>
            </form>
```

確かめるテストです（`tests/u04_q1.rs`）。

```rust
@@FILE:dup_probe/rustnote/tests/u04_q1.rs@@
```

```powershell
cargo test --test u04_q1 -- --nocapture
```

```text
@@OUT:u04_q1@@
```

- `/notes/1` を複製すると `/notes/2` ができ、bob が同じ URL を送ると 404 になりました。`toggle.rs`・`delete.rs` と同じ形のファイルを1つ足しただけで、URL（`/notes/{id}/duplicate`）も決まりました。ファイルの並びが URL の並びになっているからです。
- 複製の処理でも、DB から読んだタイトルと本文を `NewNote::parse` に通しています。UNIT 03 で見たとおり、`insert` は検査済みの `NewNote` しか受け取らないからです。保存済みの値が検査に落ちるのは、DB に決まりを破るデータが入っているときだけなので、利用者の入力のまちがい（400）ではなく、サーバーの異常（500）として扱っています。
- この変更の後も、例4-2の依存の向きのテストは、すべて通りました。`duplicate.rs` が使っているのは `auth`・`domain`・`error`・`flash`・`state` で、②の段から下の段を使っているだけだからです。

**問2の解答:**

`tests/architecture.rs` の最後に、次のテストを足します。

```rust
@@FILE:book/r6_snippet.rs@@
```

```powershell
cargo test --test architecture r6
```

```text
@@OUT:u04_q2@@
```

- 通ります。例4-1の一覧でも、`views` が使っているのは `domain`・`error`・`flash`・`routes` だけでした。
- `cargo test --test architecture r6` のように、後ろに言葉を足すと、名前にその言葉を含むテストだけを実行します（UNIT 01）。`5 filtered out` は、ほかの5つを今回は飛ばしたという意味です。
- 画面の部品が DB を読まないと決めておくと、部品は「渡された値を表示するだけ」になり、テストで好きな値を渡して見た目を確かめられます。DB を読む処理は、`routes` の中や、`notes_index` のような async のコンポーネント（`routes/notes.rs`）に集めます。

**問3の解答:**

`tests/u04_q3.rs` です。例4-3の `BrokenNotes` と同じように、どの操作も失敗させます。そのうえで、**呼ばれたメソッドの名前を記録**しておき、最後に「`insert` だけが呼ばれた」ことも確かめます。

```rust
@@FILE:book/rustnote/tests/u04_q3.rs@@
```

```powershell
cargo test --test u04_q3 -- --nocapture
```

```text
@@OUT:u04_q3@@
```

- `POST /notes -> 500 Internal Server Error` になり、画面には決まった文だけが出ました。フォームの `title=` の後ろは、「秘密のメモ」を URL エンコードした文字です。入力した内容が、エラー画面にそのまま出てしまわないことも確かめています。
- 呼ばれたメソッドは `["insert"]` だけでした。作成の処理は、検査に合格したらすぐ `insert` を呼び、その失敗をそのまま 500 にしています。ログの `insert に失敗` も、それを裏付けています。
- `calls` は `Mutex<Vec<…>>` です。`&self` しか受け取らないメソッドの中から記録を書き足すために、`Mutex` で包んでいます（第3巻 UNIT 10・第5巻 UNIT 05）。ロックを持ったまま `.await` をしないので、標準ライブラリの `Mutex` で十分です。
- `state.notes = notes.clone();` では、`Arc<RecordingBrokenNotes>` を複製して、`Arc<dyn NoteRepository + Send + Sync>` として入れています。中身は1つなので、テストの側に残した `notes` から、同じ記録を読めます。

#### 寄り道: 呼ばれないはずのメソッドに unreachable! を書くと

記録を取る代わりに、呼ばれないはずのメソッドに `unreachable!("search は呼ばれないはず")` と書きたくなるかもしれません。ところが、ページの処理の中で起きた panic は、テストの失敗にはなりません。`search` を呼ぶ一覧のページ（`GET /notes`）を、すべてのメソッドに `unreachable!` を書いた保存先で開いてみた結果です（`tests/u04_panic.rs`。コードは例4-3と同じ形なので省きます）。

```text
@@OUT:u04_panic@@
```

- panic のメッセージは出ていますが、テストは `ok` です。Topcoat のルーター（`Router::handle`）は、ページの処理の中で起きた panic を受け止めて、500 の返事に変えるからです。1つのリクエストの panic で、サーバー全体が止まらないようにするためです。
- このときの本文は `"internal server error"` の1行だけで、RustNote のエラー画面（レイアウトや「トップページへ戻る」）は出ません。アクセスログの `GET /notes -> ...` の行も出ていません。レイアウトとアクセスログの処理ごと、途中で捨てられたからです。
- ですから、状態コードが 500 かどうかだけを確かめるテストでは、panic に気づけません。問3のテストのように、**本文と、呼ばれたメソッドの記録**まで確かめておくと、思ったとおりの道を通ったかが分かります。

### まとめ

- RustNote の `src/` は、①入口（`main.rs`）・②URL ごとの処理（`routes`）・③画面の部品とログイン（`views`・`auth`・`flash`）・④共有の状態（`state`）・⑤保存（`repository`・`db`）・⑥土台（`domain`・`error`・`config`）の6つの段に分かれている。上の段は下の段を使ってよく、下の段は上の段を使わない。下の段ほど変わりにくく、たくさんの場所から使われる。
- 例外は、リンクの URL を `href!` で作るために `views` と `auth` が `routes` を指す2本だけ。同じクレートの中なら互いに使い合ってもコンパイルできるが、クレートに分けると循環のエラーになるので、例外は理由を決めて少なくする。
- 1つのパッケージには、ライブラリ・実行ファイル・`tests/`・`examples/` の別々のクレートがある。ライブラリの中は `crate::`、外からは `rustnote::` で、公開された物だけを使う。`pub use` で、ファイルの並びと公開する道のりを分けられる。
- 依存の向きの決まりは、テストにしておくと、破る変更が入ったときに `cargo test` で気づける。ただし文字を数えるテストには限界があり、レビューと組み合わせる。
- 保存先を trait（`NoteRepository`）の後ろに隠したので、`AppState` の中身を差し替えるだけで、DB が壊れたときの画面をテストできる。

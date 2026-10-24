<div style="page-break-after: always; text-align: center; margin: 0; padding: 0;">
<img src="assets/cover.jpg" alt="表紙" style="width: 100%; max-height: 175mm; object-fit: cover; border-radius: 10px;">
<div style="margin-top: 14px; font-size: 11px; letter-spacing: 0.18em; color: #c2410c; font-weight: 700;">VOLUME 09 / RUST BACKEND MASTERY</div>
<div style="font-size: 26px; font-weight: 800; color: #0f172a; margin-top: 6px; line-height: 1.35;">RustNote — 要件設計・構成・セットアップ</div>
<div style="font-size: 13px; color: #475569; margin-top: 8px;">要件・画面・データの約束から、骨組み・起動・ヘルスチェックまでを1つの設計にそろえる</div>
</div>

# 第9巻: RustNote — 要件設計・構成・セットアップ

## この巻で身に付けること

第8巻までで、RustNote を作るための部品はそろいました。Rust の文法と所有権（第2巻・第3巻）、型と trait とエラーの設計（第4巻）、async（第5巻）、Web フレームワークの Topcoat（第6巻）、Turso・libSQL と SQL（第7巻）、ログインとセッション（第8巻）です。

それでも、部品がそろっただけでは、アプリはできあがりません。いきなりコードを書き始めると、作っている途中や公開した後に、次のような事故が起きます。

- 作り終えてから「思っていたのと違う」と言われる。何ができたら完成なのかを、誰も決めていなかった
- ログインしていない人や、**ほかの人**が、自分のノートを開けてしまう
- 空白だけのタイトルや、同じメールアドレスの二重登録が、そのまま保存されてしまう
- フォームを送った直後に再読み込みすると、**同じノートがもう1件**できてしまう
- 画面ごとに作り方がばらばらで、エラーのときに何も表示されない画面がある
- `main.rs` に何でも書き足していった結果、どこを直せばよいのか分からなくなる
- 本番で設定を1つ間違えて、空のデータベースのまま動き続ける。動いているのか壊れているのか、外から確かめる方法がない

この巻のテーマは、こうした事故を、**コードを書く前の設計**と、**最初に作る骨組み**で防ぐことです。何を作るかを「確かめられる言葉」で決め（要件・受入条件）、画面とデータの約束を決め、フォルダ構成・依存・設定・起動の順番・ルートの一覧・見た目の決まり・ヘルスチェックまでを、1つの設計としてそろえます。

この巻では、教材のフォルダに入っている**完成した RustNote のソース**（`Rust/rustnote/`）をお手本として手元にコピーし、「設計で決めたことが、コードのどこにどう現れているか」を、テストを動かしながら確かめていきます。完成形を先に見て、決めごとの理由を1つずつ確かめる、という進め方です。画面の処理を1つずつ作り込むのは、次の第10巻です。

### 到達目標

この巻を終えると、次のことができるようになります。

- RustNote の機能を「誰が・何をしたい・なぜ」のユーザーストーリーにし、正常な場合だけでなく、未ログイン・空の入力・他人の ID・壊れた DB の場合まで含めた**受入条件**を書ける。受入条件をテストの名前と1対1で対応させ、`cargo test` で確かめられる
- 画面の一覧と画面遷移図を描き、画面ごとの「いちばんの目的」と、空・エラー・成功のお知らせのような**状態**を決められる。フォームの送信が成功したら 303 で GET の画面へ移る形（PRG）を説明できる
- アプリが扱う「もの」（ユーザー・ノート・セッション）を型と表に対応させ、「必ず成り立つ決まり（不変条件）」を、**型・アプリの処理・DB の制約**のどこで守るかを表にできる
- `src/` を役割ごとのモジュールに分け、依存の向き（どのモジュールがどれを使ってよいか）を説明できる
- `Cargo.toml` で依存の版を固定し、features と `Cargo.lock` の役割を説明できる
- 設定（`Config`）を起動時に1回だけ読み、DB の初期化とマイグレーションを済ませてから、共有する状態（`AppState`）をルーターに渡す、という起動の順番を説明できる
- マイグレーション・見本のデータ（seed）・テスト用の DB を、本番と同じ SQL で、互いに混ざらないように用意できる
- ルートの一覧（メソッド・URL・ログインの要否・成功と失敗の返事）を作り、薄い縦切りの順番で実装の計画を立てられる
- 色・余白・ボタン・フォーム・エラー表示の決まりを少数にまとめ、見出し・ラベル・キーボード操作・コントラストを最初から確かめられる
- ヘルスチェックの役割を説明し、初回起動でつまずいたときに、ログと手順から原因を切り分けられる

### 学習地図

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 400" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-001" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="6" y="30" width="708" height="90" rx="12" fill="#eff6ff"/>
<rect x="6" y="140" width="708" height="90" rx="12" fill="#fff7ed"/>
<rect x="6" y="250" width="708" height="90" rx="12" fill="#ecfdf5"/>
<text x="360" y="20" font-size="13" fill="#475569" text-anchor="middle">上の段から順に進みます（各段の中は左から右へ）</text>
<text x="65" y="70" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">何を作るか</text>
<text x="65" y="90" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">決める</text>
<text x="65" y="180" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">骨組みを</text>
<text x="65" y="200" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">組み立てる</text>
<text x="65" y="290" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">つないで</text>
<text x="65" y="310" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">確かめる</text>
<rect x="135" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="200" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 01</text>
<text x="200" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">要件・受入条件</text>
<text x="200" y="99" font-size="12" fill="#475569" text-anchor="middle">完成の基準</text>
<rect x="282" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="347" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 02</text>
<text x="347" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">画面と遷移</text>
<text x="347" y="99" font-size="12" fill="#475569" text-anchor="middle">URL と状態</text>
<rect x="429" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="494" y="60" font-size="12" font-weight="700" fill="#3b82f6" text-anchor="middle">UNIT 03</text>
<text x="494" y="80" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">ドメインと表</text>
<text x="494" y="99" font-size="12" fill="#475569" text-anchor="middle">データの約束</text>
<line x1="265" y1="75" x2="280" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-001)"/>
<line x1="412" y1="75" x2="427" y2="75" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-001)"/>
<rect x="576" y="40" width="130" height="70" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-dasharray="5 4"/>
<text x="641" y="64" font-size="12" fill="#475569" text-anchor="middle">RustNote では</text>
<text x="641" y="84" font-size="12" fill="#475569" text-anchor="middle">tests/・domain.rs</text>
<text x="641" y="102" font-size="12" fill="#475569" text-anchor="middle">migrations/</text>
<path d="M494,110 V130 H200 V146" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-001)"/>
<rect x="135" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="200" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 04</text>
<text x="200" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">フォルダ構成</text>
<text x="200" y="209" font-size="12" fill="#475569" text-anchor="middle">依存の向き</text>
<rect x="282" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="347" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 05</text>
<text x="347" y="190" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">Cargo.toml</text>
<text x="347" y="209" font-size="12" fill="#475569" text-anchor="middle">版と features</text>
<rect x="429" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="494" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 06</text>
<text x="494" y="190" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">Config・AppState</text>
<text x="494" y="209" font-size="12" fill="#475569" text-anchor="middle">起動の準備</text>
<rect x="576" y="150" width="130" height="70" rx="8" fill="#ffffff" stroke="#f97316" stroke-width="2"/>
<text x="641" y="170" font-size="12" font-weight="700" fill="#f97316" text-anchor="middle">UNIT 07</text>
<text x="641" y="190" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">migration・seed</text>
<text x="641" y="209" font-size="12" fill="#475569" text-anchor="middle">表と見本のデータ</text>
<line x1="265" y1="185" x2="280" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-001)"/>
<line x1="412" y1="185" x2="427" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-001)"/>
<line x1="559" y1="185" x2="574" y2="185" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-001)"/>
<path d="M641,220 V240 H200 V256" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-001)"/>
<rect x="135" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="200" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 08</text>
<text x="200" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">ルートの一覧</text>
<text x="200" y="319" font-size="12" fill="#475569" text-anchor="middle">作る順番</text>
<rect x="282" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="347" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 09</text>
<text x="347" y="300" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">デザイン・アセット</text>
<text x="347" y="319" font-size="12" fill="#475569" text-anchor="middle">見た目と使いやすさ</text>
<rect x="429" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#10b981" stroke-width="2"/>
<text x="494" y="280" font-size="12" font-weight="700" fill="#10b981" text-anchor="middle">UNIT 10</text>
<text x="494" y="300" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">health・診断</text>
<text x="494" y="319" font-size="12" fill="#475569" text-anchor="middle">動作を確かめる</text>
<line x1="265" y1="295" x2="280" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-001)"/>
<line x1="412" y1="295" x2="427" y2="295" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-001)"/>
<rect x="576" y="260" width="130" height="70" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-dasharray="5 4"/>
<text x="641" y="284" font-size="12" fill="#475569" text-anchor="middle">RustNote では</text>
<text x="641" y="304" font-size="12" fill="#475569" text-anchor="middle">routes/・assets/</text>
<text x="641" y="322" font-size="12" fill="#475569" text-anchor="middle">GET /health</text>
<text x="360" y="372" font-size="13" fill="#475569" text-anchor="middle">各単元の流れ: たとえ話 → 図 → 最小コード → 実務の形 → よくあるエラー → 練習問題</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図9-?: 第9巻の学習地図。何を作るかを決めてから、骨組みを組み立て、ルート・見た目・ヘルスチェックでつないで確かめる</div>
</div>

| 単元 | テーマ | ひとことで言うと |
|---|---|---|
| UNIT 01 | 要件・ユーザーストーリー・受入条件 | 「何ができたら完成か」を、確かめられる言葉で決める |
| UNIT 02 | 画面遷移・情報設計・ワイヤーフレーム | 画面と URL を一覧にし、画面ごとの目的と状態を決める |
| UNIT 03 | ドメインモデル・スキーマ・不変条件 | データの決まりを、型・アプリ・DB の3か所でそろえる |
| UNIT 04 | プロジェクト構成と依存の向き | `src/` を役割で分け、どこがどこを使ってよいかを決める |
| UNIT 05 | Cargo.toml・版の固定・features（準備中） | 使うクレートと版を決めて、誰の PC でも同じ物を作る |
| UNIT 06 | Config・AppState・DB の初期化（準備中） | 設定を1回だけ読み、準備を済ませてから待ち受ける |
| UNIT 07 | migration・seed・テスト用の DB（準備中） | 表の作り方と見本のデータを、本番とテストで混ぜずに用意する |
| UNIT 08 | ルートの一覧と実装の順番（準備中） | URL ごとの約束を表にし、薄く縦に切って順に作る |
| UNIT 09 | デザインの決まり・アセット・アクセシビリティ（準備中） | 少ない決まりで見た目をそろえ、誰でも使える画面にする |
| UNIT 10 | health・初回起動・診断（準備中） | 動いているかを外から確かめ、つまずきの原因を切り分ける |

> **お知らせ:** この版には UNIT 01〜04 を収録しています。UNIT 05〜10 は準備中で、次の更新から順に追加します。本文の中の「UNIT 06（準備中）で扱います」のような予告は、これから追加する単元を指しています。

### この巻で作る「設計の成果物」と、RustNote のファイル

設計というと、分厚い書類を思い浮かべるかもしれません。この巻では、書類だけで終わらせません。決めたことは、必ずコードかテストのどこかに**形として残します**。そうすれば、あとで「決めたとおりにできているか」を、いつでも機械で確かめられるからです。

次の図は、この巻で決めることと、完成した RustNote でそれが置かれているファイルの対応です。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 340" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-002" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="160" y="24" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">この巻で決めること</text>
<text x="530" y="24" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">RustNote で置かれている場所</text>
<rect x="30" y="38" width="260" height="38" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="160" y="62" font-size="12" fill="#0f172a" text-anchor="middle">要件・受入条件（UNIT 01）</text>
<rect x="30" y="86" width="260" height="38" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="160" y="110" font-size="12" fill="#0f172a" text-anchor="middle">画面・URL・状態（UNIT 02・08）</text>
<rect x="30" y="134" width="260" height="38" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="160" y="158" font-size="12" fill="#0f172a" text-anchor="middle">ドメインの型と表（UNIT 03・07）</text>
<rect x="30" y="182" width="260" height="38" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="160" y="206" font-size="12" fill="#0f172a" text-anchor="middle">構成・依存・版（UNIT 04・05）</text>
<rect x="30" y="230" width="260" height="38" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="160" y="254" font-size="12" fill="#0f172a" text-anchor="middle">設定・起動の順番（UNIT 06）</text>
<rect x="30" y="278" width="260" height="38" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="160" y="302" font-size="12" fill="#0f172a" text-anchor="middle">見た目・確かめる入口（UNIT 09・10）</text>
<rect x="390" y="38" width="300" height="38" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="540" y="62" font-size="12" fill="#0f172a" text-anchor="middle">README.md・tests/（受入条件のテスト）</text>
<rect x="390" y="86" width="300" height="38" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="540" y="110" font-size="12" fill="#0f172a" text-anchor="middle">src/routes/・src/views.rs</text>
<rect x="390" y="134" width="300" height="38" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="540" y="158" font-size="12" fill="#0f172a" text-anchor="middle">src/domain.rs・migrations/*.sql</text>
<rect x="390" y="182" width="300" height="38" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="540" y="206" font-size="12" fill="#0f172a" text-anchor="middle">src/lib.rs・Cargo.toml・Cargo.lock</text>
<rect x="390" y="230" width="300" height="38" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="540" y="254" font-size="12" fill="#0f172a" text-anchor="middle">src/main.rs・config.rs・db.rs・state.rs</text>
<rect x="390" y="278" width="300" height="38" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="540" y="302" font-size="12" fill="#0f172a" text-anchor="middle">assets/style.css・src/routes/health.rs</text>
<line x1="290" y1="57" x2="386" y2="57" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-002)"/>
<line x1="290" y1="105" x2="386" y2="105" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-002)"/>
<line x1="290" y1="153" x2="386" y2="153" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-002)"/>
<line x1="290" y1="201" x2="386" y2="201" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-002)"/>
<line x1="290" y1="249" x2="386" y2="249" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-002)"/>
<line x1="290" y1="297" x2="386" y2="297" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-002)"/>
<text x="360" y="334" font-size="12" fill="#475569" text-anchor="middle">決めたことは、コードかテストのどこかに必ず形として残す</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図9-?: この巻で決めることと、完成した RustNote でそれが置かれているファイルの対応</div>
</div>

> **ヒント:** RustNote の `README.md` には、できること・URL の一覧・エラーのときのステータスコード・フォルダの構成が、まとめて書いてあります。この巻を読み進める間、ときどき開いて見比べると、「設計で決めたこと」と「できあがったもの」が対応していることがよく分かります。

### 前提知識（先に読んでおく巻）

- **第1巻（環境構築）:** Rust・VS Code・rust-analyzer がインストール済みで、`cargo new` と `cargo run` と `cargo test` ができること。
- **第2巻（Rust 基礎）:** `struct` / `enum` / `match` / `Option` / `Result` / `?` / `Vec` / `String`。特に、日本語の文字数を `chars().count()` で数えること（第2巻 UNIT 05）。
- **第4巻（設計）:** newtype と「不正な値を作れない型」（第4巻 UNIT 01）、エラー型（第4巻 UNIT 05）、モジュールと可視性（第4巻 UNIT 06）、テスト（第4巻 UNIT 10）。UNIT 03 は、第4巻 UNIT 01 の続きとして読めます。
- **第5巻（async）:** `async fn` と `.await`、`#[tokio::main]`（第5巻 UNIT 01）。この巻のテストは `#[tokio::test]` を使います。
- **第6巻（Topcoat）:** `#[page]` と `#[page(POST)]`、`view!`、フォームの受け取り、303 のリダイレクト（`see_other`）。
- **第7巻（Turso・libSQL）:** `CREATE TABLE` と制約（第7巻 UNIT 01）、持ち主の条件を付けた SQL（第7巻 UNIT 03）、マイグレーション（第7巻 UNIT 06）。
- **第8巻（認証認可）:** パスワードのハッシュ（Argon2id）、セッションとクッキー、他人のデータを見せない考え方。

> **注意:** この巻は、ここまでの巻の「まとめ役」です。どこかの巻の内容があいまいでも、読み進めてかまいません。本文で「第7巻 UNIT 01」のように元の場所を示すので、必要になったところで読み返してください。

### この巻の進め方

各単元では、次の順番を1サイクルとして進めてください。設計の単元は、読んで「なるほど」と思うだけで終わりがちです。**決めたことを、テストを動かして確かめる**ところまでを1セットにします。

1. **予想する:** テストを動かす前に、どのステータスコード（200・303・400・404 など）が返り、画面にどんな文が出るかを予想します。
2. **そのまま動かす:** 教材のテストをそのまま実行し、結果が教材と同じか比べます。
3. **1か所だけ変える:** 期待する値やテストの入力を1つだけ変えて、テストが失敗することを確かめます。「失敗するはずのときに、ちゃんと失敗する」ことを見ておくと、テストを信じられるようになります。
4. **整える:** 単元の終わりには、RustNote の `README.md` に書かれている次の3つのコマンドを実行する習慣をつけます。

```powershell
# コードの見た目が Rust の標準の形になっているか確かめる（直すときは --check を外す）
cargo fmt --check
# よくある書き間違い・もったいない書き方を、警告ではなくエラーとして止める
cargo clippy --all-targets -- -D warnings
# テストを全部実行する
cargo test
```

> **ヒント:** この巻の実行結果には、かかった時間（`3.05s` や `(445.6532ms)` など）や、スレッドの番号（`(68436)` など）が出てきます。時間はパソコンの速さで変わり、スレッドの番号は実行するたびに変わります。テストの表示の順番も、実行するたびに入れ替わることがあります。

### 使うツールと版

この巻のコードは、すべて次の環境で実際にコンパイル・実行し、実行結果とエラーメッセージは実際の出力をそのまま載せています（パソコンのユーザー名などを含む部分だけ `...` に置き換えています）。

| 項目 | 版・内容 |
|---|---|
| OS | Windows 11（ARM64）。コマンドは PowerShell を主に、WSL / macOS / Linux は補足で示します |
| Rust | **1.98 以上**（本書シリーズの基準）。動作確認は rustc 1.99.0 / cargo 1.99.0（`aarch64-pc-windows-msvc`） |
| edition | 2024 |
| RustNote | 教材の `Rust/rustnote/`（`rustnote` 0.1.0）。直接使うクレートの版は、すべて `Cargo.toml` で `=` を付けて固定してある |
| topcoat | 0.10.0（Web フレームワーク） |
| libsql | 0.10.0（Turso / libSQL のクライアント。ARM64 版の Windows では、ビルドに clang が必要） |
| tokio | 1.53.3（`rt-multi-thread` と `macros` の機能だけ） |
| argon2 / thiserror / anyhow | 0.6.0 / 2.0.21 / 1.0.104 |
| async-trait / serde / dotenvy | 0.1.92 / 1.0.229 / 0.15.7 |
| tempfile | 3.27.0（テスト用。テストごとに使い捨ての一時フォルダを作る） |
| topcoat-cli | 0.10.0（`topcoat dev` などのコマンド。アプリをブラウザで動かす単元で使います。UNIT 01〜04 では使いません） |

UNIT 01〜04 では、新しいクレートを足しません。RustNote にもともと入っているクレートだけで、テストと小さな確かめ用のプログラムを書きます。

### 練習用プロジェクトを用意する

この巻では、教材の `Rust/rustnote/` を**作業用のフォルダにコピー**して使います。お手本そのものを書き換えると、あとで「元はどうだったか」が分からなくなるからです。コピーしたほうには、この巻の確かめ用のテストやプログラムを足していきます。

PowerShell で作業用のフォルダ（どこでもかまいません）に移動し、次のように実行します。`"教材のフォルダ\Rust\rustnote"` の部分は、自分の PC で教材を置いた場所に読み替えてください（この PDF が入っている `pdf` フォルダの1つ上が、`Rust` フォルダです）。

```powershell
# 教材の rustnote フォルダを、中身ごと（-Recurse）作業用のフォルダへコピーする
Copy-Item -Recurse "教材のフォルダ\Rust\rustnote" .\rustnote
cd rustnote
# このフォルダで使われる Rust の版を確かめる
rustup show active-toolchain
```

```text
1.99.0-aarch64-pc-windows-msvc (overridden by '...\rustnote\rust-toolchain.toml')
```

- RustNote のフォルダには `rust-toolchain.toml` というファイルがあり、中に `channel = "1.99.0"` と書いてあります。このフォルダの中で `cargo` を実行すると、PC の既定の版に関係なく、Rust 1.99.0 が使われます。`overridden by ...`（…で上書きされています）の表示が、そのことを示しています。
- 1.99.0 がまだ入っていない PC では、次のコマンドで追加します。既定の版は変わりません。

```powershell
rustup toolchain install 1.99.0 --profile minimal --component rustfmt,clippy
```

> **注意:** ARM64 版の Windows では、RustNote をビルドするのに **LLVM（clang）** が必要です。libsql が使う暗号のライブラリ（`ring`）を、clang でコンパイルするためです。入っていないと、ビルドの途中で次のように止まります（長い出力から、大事な行だけを抜き出しています）。

```text
warning: ring@0.17.14: Compiler family detection failed due to error: ToolNotFound: failed to find tool "clang": program not found (see https://docs.rs/cc/latest/cc/#compile-time-requirements for help)
error: failed to run custom build command for `ring v0.17.14`
  error occurred in cc-rs: failed to find tool "clang": program not found (see https://docs.rs/cc/latest/cc/#compile-time-requirements for help)
```

- `failed to find tool "clang"` は「clang という道具が見つかりません」という意味です。RustNote の `README.md` のとおり、`winget install LLVM.LLVM` で LLVM を入れ、新しく開いた PowerShell で `clang --version` が動くこと（`C:\Program Files\LLVM\bin` が PATH にあること）を確かめてから、もう一度ビルドします。
- x64 の Windows・macOS・Linux では、この準備は要りません。くわしくは UNIT 05（準備中）で扱います。

準備ができたら、コピーしたフォルダで、テストを全部実行してみましょう。

```powershell
cargo test
```

最初の1回は、RustNote が使うクレート（全部で約290個）をコンパイルするので、時間がかかります（この PC では約1分でした）。次の出力は、最後の部分から、テストの区切りと結果の行を抜き出したものです。

```text
   Compiling rustnote v0.1.0 (...\rustnote)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1m 11s
     Running unittests src\lib.rs (target\debug\deps\rustnote-5cafc6c87c537344.exe)
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running unittests src\main.rs (target\debug\deps\rustnote-f46d7aa1268a13dc.exe)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests\http.rs (target\debug\deps\http-9c37f549b35c0170.exe)
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.77s
     Running tests\repository.rs (target\debug\deps\repository-411ae9b50fa8c3de.exe)
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s
   Doc-tests rustnote
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

- `Running ...` の行が、テストのまとまり（テスト用の実行ファイル）の区切りです。RustNote のテストは、`src/` の中のユニットテスト（19件）、`tests/http.rs`（HTTP のテスト 14件）、`tests/repository.rs`（DB の読み書きのテスト 8件）の、合わせて **41件**です。
- 全部が `ok` なら、コピーは正しくできています。クレートをまだダウンロードしていない PC では、最初にダウンロードの表示も加わります。
- テストは、DB を一時フォルダに作って使い、終わると消します。フォルダの中の `rustnote.db`（アプリを起動したときに作られる DB のファイル）には触りません。

#### この巻で足していくファイル

コピーした `rustnote` フォルダに、この巻では次のファイルを足していきます。RustNote の `src/` の中身は変えません。

| 置く場所 | 中身 | 実行のしかた |
|---|---|---|
| `tests/*.rs` | 受入条件・画面の流れ・依存の向きを確かめるテスト（UNIT 01・02・04） | `cargo test --test ファイル名` |
| `examples/*.rs` | ドメインの型や DB の制約を確かめ、モジュールの依存を一覧にする小さなプログラム（UNIT 03・04） | `cargo run -q --example ファイル名` |

- `tests/` に置いた `.rs` ファイルは、1つずつが別のテストのまとまりになります。`cargo test --test acceptance` と書くと、`tests/acceptance.rs` のテストだけを実行します。
- `examples/` に置いた `.rs` ファイルは、1つずつが別の小さなプログラムになり、RustNote の部品（`rustnote::domain` など）をそのまま使えます。`-q`（quiet：静かに）を付けると、`Compiling ...` のような Cargo の表示が消えて、プログラムが表示した内容だけが残ります。
- テストの中で `println!` した内容は、ふだんは隠されます（失敗したテストの分だけ表示されます）。成功したテストでも表示させたいときは、`cargo test --test screens -- --nocapture` のように、`--` の後ろに `--nocapture` を付けます。`--` より前は Cargo への指定、後ろはテストの実行ファイルへの指定です。

> **ヒント:** 練習で足したファイルも、`cargo test` を実行すると一緒にコンパイルされます。この巻の「よくあるエラー」の節では、わざとコンパイルエラーになるファイルを作ります。確かめ終わったら、そのファイルは必ず消しておきましょう。残しておくと、ほかのテストも実行できなくなります。

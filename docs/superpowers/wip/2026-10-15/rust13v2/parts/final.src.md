## 総合演習: RustNote の改善点を直す

### この演習のゴール

- この巻（と第11巻）の単元で見つけた RustNote の改善点のうち、9つを、教材のコードのコピーで実際に直せる
- 直す前に「今は失敗する」テストや実験を用意し、直したあとに通ることを確かめる、という手順で進められる
- 直したあとも、CI と同じ3つの手順（整形の確認・Clippy・テスト）が通ることを確かめられる

### 進め方

1. 教材の `Rust/rustnote` のフォルダ（`src`・`tests`・`migrations`・`assets` と設定ファイル）を、別の場所にコピーします。教材のフォルダは変えません。
2. コピーで `git init` をして、何も変えない状態を最初の記録（コミット）にします。演習を1つ終えるたびに、1つのコミットにします。こうすると、演習ごとの変更を `git show` で見られます（解答の差分は、これで作りました）。
3. それぞれの演習は、できるだけ **①今は失敗するテスト（か実験）を先に書く → ②直す → ③通ることを確かめる** の順に進めます。
4. 最後に、CI と同じ3つの手順（UNIT 10）を動かします。

| 演習 | 直すこと | 見つけた所 | 直すファイル |
|---|---|---|---|
| 1 | NUL などの制御文字を含むタイトルが 500 になる | UNIT 04・第11巻 UNIT 04 | `src/domain.rs` |
| 2 | `PORT` がおかしいと、DB を作って「起動しました」と出してから止まる | UNIT 03 | `src/main.rs`・`src/config.rs`・`Cargo.toml` |
| 3 | `NoteQuery` の項目が `pub` で、`page: 0` を作れる | UNIT 04・第11巻 UNIT 04 | `src/domain.rs` と、読む所 |
| 4 | `Flash` の種類を足して `all` に足し忘れても、だれも気づかない | UNIT 06 | `src/flash.rs` |
| 5 | 名前の無い `nav` が2つ・スマホでヘッダーがはみ出す | UNIT 07・09・第11巻 UNIT 05 | `src/views.rs`・`assets/style.css` |
| 6 | 最後より大きいページで「まだノートがありません」と出る | UNIT 08 | `src/routes/notes.rs` |
| 7 | Argon2 を同時に計算する数に上限が無い | UNIT 06・第11巻 UNIT 09 | `src/auth.rs`・`Cargo.toml` |
| 8 | 守りのヘッダー（`X-Frame-Options` など）を返していない | 第11巻 UNIT 03・06 | `src/routes.rs` |
| 9 | `RepoError` の分類の表が2か所にある | UNIT 04・07 | `src/error.rs`・`src/views.rs` |

### 図で理解する

9つの演習が、RustNote のどこを直すかです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-13-1101" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="20" width="160" height="70" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="90" y="44" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">起動（main.rs）</text>
<text x="90" y="64" font-size="11" fill="#b91c1c" text-anchor="middle">2 待ち受けを先に</text>
<rect x="190" y="20" width="160" height="70" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="270" y="44" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">入口（routes.rs）</text>
<text x="270" y="64" font-size="11" fill="#b91c1c" text-anchor="middle">8 守りのヘッダー</text>
<rect x="370" y="20" width="160" height="70" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="450" y="44" font-size="12" font-weight="700" fill="#1d4ed8" text-anchor="middle">画面（views・notes）</text>
<text x="450" y="64" font-size="11" fill="#b91c1c" text-anchor="middle">5 nav の名前</text>
<text x="450" y="80" font-size="11" fill="#b91c1c" text-anchor="middle">6 最後より大きいページ</text>
<rect x="550" y="20" width="160" height="70" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="630" y="44" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">見た目（style.css）</text>
<text x="630" y="64" font-size="11" fill="#b91c1c" text-anchor="middle">5 ヘッダーの折り返し</text>
<rect x="10" y="120" width="220" height="80" rx="6" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="120" y="144" font-size="12" font-weight="700" fill="#1d4ed8" text-anchor="middle">型（domain.rs）</text>
<text x="120" y="164" font-size="11" fill="#b91c1c" text-anchor="middle">1 タイトルの制御文字</text>
<text x="120" y="182" font-size="11" fill="#b91c1c" text-anchor="middle">3 NoteQuery を private に</text>
<rect x="250" y="120" width="220" height="80" rx="6" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="360" y="144" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">ログイン（auth・flash）</text>
<text x="360" y="164" font-size="11" fill="#b91c1c" text-anchor="middle">7 Argon2 に整理券</text>
<text x="360" y="182" font-size="11" fill="#b91c1c" text-anchor="middle">4 お知らせの一覧を1か所に</text>
<rect x="490" y="120" width="220" height="80" rx="6" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="600" y="144" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">エラー（error・views）</text>
<text x="600" y="164" font-size="11" fill="#b91c1c" text-anchor="middle">9 分類の表を1か所に</text>
<rect x="10" y="230" width="700" height="56" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="1.5"/>
<text x="24" y="254" font-size="12" fill="#047857">どの演習も: 失敗するテスト（か実験）を先に書く → 直す → 通る → 1つのコミット</text>
<text x="24" y="274" font-size="12" fill="#047857">最後に CI と同じ3つの手順: cargo fmt --check・cargo clippy -D warnings・cargo test（41 → 47 個）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図13-?: 総合演習の地図。起動・入口・画面・見た目・型・ログイン・エラーの、それぞれで見つけた改善点を、1つずつ直す</div>
</div>

### 演習の問題

**演習1:** タイトルが NUL（`%00`）1文字のノートを作ると、500 になります（UNIT 08 の⑩）。400 で「使えない文字」と知らせるように直してください。先に、HTTP のテスト（`POST /notes` に `title=%00` を送って 400 を期待する）を書き、今は失敗することを確かめてから直します。

**演習2:** `PORT=abc` で起動すると、DB のファイルを作ってマイグレーションを当て、「起動しました」と出してから止まります（UNIT 03）。ほかのプログラムがポートを使っているときも、同じです。DB に触る前に止まり、「起動しました」は受け付けられる状態になってから出るように直してください（ヒント: `tokio::net::TcpListener::bind` と `topcoat::serve`）。

**演習3:** `NoteQuery` の項目（`keyword`・`page`）を `pub` でなくし、`NoteQuery::new` を通らずには作れないようにしてください（UNIT 04 の問2）。項目を読んでいる所は、どう直せばよいでしょうか。

**演習4:** `Flash` の種類を足して、`from_code` の一覧に足し忘れると、お知らせが黙って消えます（UNIT 06 の問1）。一覧を1か所にまとめ、足し忘れたら**テストのコンパイル**で止まるようにしてください。

**演習5:** 一覧の画面の2つの `nav` に名前を付け（axe の `landmark-unique`）、幅 375px・320px でヘッダーがはみ出さないようにしてください（UNIT 09 の問1）。Chrome と axe で確かめます。

**演習6:** ノートがあるのに、最後より大きいページ（`?page=99`）で「まだノートがありません。」と出ます（UNIT 08 の問1）。先にテストを書いてから直してください。

**演習7:** 同時にたくさんのログインが来ると、Argon2 の計算が同時にいくつも走り、メモリが増えます（第11巻 UNIT 09）。同時に計算する数を、CPU の数までにしてください。同時 48 のログインで、メモリのピークを比べます。

**演習8:** 守りのヘッダー（`X-Content-Type-Options`・`X-Frame-Options`・`Referrer-Policy`・`Content-Security-Policy`）を、画面の応答に付けてください（第11巻 UNIT 03 の問2）。先にテストを書き、付けたあとも画面が壊れない（CSP の違反が出ない）ことを Chrome で確かめます。

**演習9:** `RepoError` を状態コードに分ける表が、`From<RepoError> for AppError` と `views.rs` の `error_page` の2か所にあります（UNIT 04 の改善点）。1か所にまとめ、2つの道が同じ答えになることをテストで確かめてください。

### 解答と解説

#### 演習1の解答: 制御文字を含むタイトルを 400 で断る

先に、`tests/http.rs` にテストを足して動かしました（直す前）。

```text
@@OUTGREP:ex1_before:^(\[error\]|thread|assertion|  left|  right|test result)@@
```

- 期待した 400 ではなく 500 でした。サーバーのログ（`[error]`）には、DB の `CHECK` で断られたことが出ています。

`NoteTitle::parse` で、trim のあとに制御文字（`char::is_control`）があれば、新しい種類 `TitleError::ControlChar` で断るようにしました。

```diff
@@OUT:ex1_diff@@
```

```text
@@OUTGREP:ex1_after:^(running|test result)@@
```

```text
@@OUTGREP:ex1_unit:^(running|test result)@@
```

- HTTP のテストも、タイトルのユニットテスト（4つ）も通りました。前後のタブや改行は trim で消えるので、ふつうの入力は今までどおり通ります。
- `TitleError` に種類を足しましたが、`AppError::InvalidTitle(#[from] TitleError)` は中身の種類を問わないので、ほかのファイルは変えずに済みました。画面の文は `#[error("…")]` の文です（UNIT 04）。

#### 演習2の解答: 待ち受けの場所を DB より先に取る

`main.rs` で、アセットの束を読んだすぐあと（DB を開く前）に、`TcpListener::bind` で待ち受けの場所を取り、最後に `topcoat::start` の代わりに `topcoat::serve(listener, router)` を呼ぶようにしました。`topcoat::start` は、中で `HOST`・`PORT` を読んで bind してから serve する関数なので（UNIT 03 の問3）、その前半を自分で先にするのです。`tokio::net` を使うので、`Cargo.toml` の tokio の features に `net` を足しました（自分が使う機能は、自分の `Cargo.toml` に書く。UNIT 01）。

```diff
@@OUT:ex2_diff@@
```

直す前（教材）と、直したあとの release の実行ファイルを、同じ条件で起動しました（UNIT 03 の例3-3と同じ確かめ方）。

**▼ `PORT=abc`（直す前 → 直したあと）**

```text
@@OUT:ex2_before_port@@
```

```text
@@OUT:ex2_after_port@@
```

**▼ 3014 番をほかのプログラム（python）が使っている（直す前 → 直したあと）**

```text
@@OUT:ex2_before_busy@@
```

```text
@@OUT:ex2_after_busy@@
```

**▼ ふつうに起動（直したあと）**

```text
@@OUT:ex2_after_ok@@
```

- 直す前は、どちらの場合も、DB のファイル（`port.db`・`busy.db`）を作り、3つの版を当て、「起動しました」と出してから止まりました。
- 直したあとは、どちらも、待ち受けの場所を取れずに**すぐ**止まり、フォルダの中は空のままでした。ふつうの起動では、「起動しました」のあとに `/health` が 200 でした。今の「起動しました」は、本当に受け付けられる状態で出ています。

#### 演習3の解答: NoteQuery の項目を private にする

まず、項目の `pub` だけを消して、`cargo build` をしました。

```text
@@OUT:ex3_private@@
```

- コンパイラが、項目を直接読んでいる4か所（一覧のルート・ページ送り・リポジトリの2か所）を、全部教えてくれました。項目を private にするときは、このように、まず消して、エラーの場所を直していきます。

読むためのメソッド `keyword()`（`Option<&str>`）と `page()` を足し、4か所とテストを、メソッドを呼ぶように直しました。

```diff
@@OUT:ex3_diff@@
```

項目を直接書いて作るテスト（`NoteQuery { keyword: None, page: 0 }`）を、`tests/` に置いてみました。

```text
@@OUT:ex3_construct@@
```

```text
@@OUTGREP:ex3_tests:^(running|test result)@@
```

- `domain` の外では、`page: 0` の `NoteQuery` を作れなくなりました（E0451）。第11巻 UNIT 04 の fuzz が見つけたパニックは、もう、だれにも起こせません。テストは全部通りました（ユニット 20・HTTP 15・リポジトリ 8）。

#### 演習4の解答: お知らせの種類の一覧を1か所にまとめる

`from_code` の中にあった一覧を、`Flash::ALL` という定数1か所にまとめ、テストは `ALL` の全部の種類を確かめるようにしました。さらに、テストの中に、`_` の無い `match` で種類の数を返す関数 `kinds()` を置き、`ALL` の数と比べます。

```diff
@@OUT:ex4_diff@@
```

```text
@@OUTGREP:ex4_tests:^(running|test result)@@
```

UNIT 06 の問1と同じく、`NoteArchived` を enum・`code`・`message` に足し、`ALL` には足し忘れた状態で、ビルドとテストをしました。

```text
@@OUTTAIL:ex4_add_build:1@@
```

```text
@@OUTGREP:ex4_add_test:^(error|   -->|    = note|error: could not)@@
```

- `cargo build`（アプリそのもの）は通りますが、`cargo test` は、`kinds()` の `match` に `NoteArchived` が無いので、E0004 で止まりました。直す人は、`kinds()` の腕と数（9）を足し、そのとき `ALL` にも足すことを思い出します。`ALL` に足し忘れたまま数だけ直すと、今度は `all_kinds_are_listed`（`ALL` の数と `kinds()` を比べる）が失敗します。
- CI はテストも動かすので、足し忘れたまま main に入ることはありません。

#### 演習5の解答: nav に名前を付け、ヘッダーを折り返す

ヘッダーの `nav` に `aria-label="メインメニュー"`、ページ送りの `nav` に `aria-label="ページ送り"` を付け、CSS は UNIT 09 の問1と同じく `flex-wrap: wrap` を足しました。

```diff
@@OUT:ex5_diff@@
```

直したものの release を起動し、UNIT 09 の例9-2と同じ計測（`measure.mjs all`）をしました。

```text
@@OUT:ex5_measure@@
```

- 375px・320px のはみ出しが無くなり（UNIT 09 では 392px）、axe の `landmark-unique` も消えて、一覧も詳細も「問題なし」になりました。ほかの見た目（色・打ち消し線・`pre-wrap`）は変わっていません。

#### 演習6の解答: 最後より大きいページで、正しい文を出す

先にテストを書いて動かしました（直す前）。

```text
@@OUTGREP:ex6_before:^(thread|assertion|test result)@@
```

UNIT 08 の問1と同じ直し方で、総数が 0 でなければ「このページにはノートがありません。」にしました。

```diff
@@OUT:ex6_diff@@
```

```text
@@OUTGREP:ex6_after:^(running|test result)@@
```

#### 演習7の解答: Argon2 を同時に計算する数に上限を付ける

第11巻 UNIT 09 で試した整理券（`tokio::sync::Semaphore`）を、`hash_password` と `verify_password` に足しました。券の数は CPU の数（`available_parallelism`。この PC では 12）です。`tokio::sync` を使うので、tokio の features に `sync` を足しました。

```diff
@@OUT:ex7_diff@@
```

直す前（教材）と直したあとの release を、それぞれ新しく起動し、登録してから、`oha` で同時 48・合わせて 480 回のログインを送り、メモリのピーク（`PeakWorkingSet64`）を比べました。

```text
@@OUT:ex7_before@@
```

```text
@@OUT:ex7_after@@
```

- メモリのピークは 690 MiB → **199 MiB** になり、1秒あたりのログインは 78.5 回 → **116.7 回**に増えました（480 回とも 303）。第11巻 UNIT 09 の結果（654 → 218 MiB）と同じ傾向です。券を待つ間は `await` で止まっているだけなので、メモリも CPU もほとんど使いません。

#### 演習8の解答: 守りのヘッダーを付ける

先にテストを書いて動かしました（直す前）。

```text
@@OUTGREP:ex8_before:^(thread|no entry|test result)@@
```

はじめ、`routes.rs` に、ヘッダーを付ける `#[layer]` をもう1つ足しました。すると、コンパイルは通りましたが、テストで、ルーターを組み立てるところでパニックしました。

```text
@@OUTGREP:ex8_twolayers:^(thread|multiple|test result)@@
```

- 「同じ場所（`""`、つまり `/`）に、複数の layer が登録されました」というパニックです。Topcoat では、1つのモジュールに `#[layer]` は1つだけです。そこで、今ある `access_log` の中で、応答が `Ok` のときにヘッダーを付ける形にしました。

```diff
@@OUT:ex8_diff@@
```

```text
@@OUTGREP:ex8_after:^(running|test result)@@
```

直したものの release を起動して、`curl` でヘッダーを、Chrome で CSP の違反の知らせと axe を確かめました。

```text
@@OUT:ex8_headers@@
```

```text
@@OUT:ex8_browser@@
```

- 200・404・401 の応答と `/health` に、4つのヘッダーが付きました。303 のリダイレクトには付きません（`access_log` の中では、まだ `Err` だから）。中身の無い応答なので、困りません。
- Chrome では、CSP の違反の知らせは 0 件で、CSS も効いており、axe も「問題なし」でした。RustNote は、外のサイトのスクリプトも、HTML の中に書いたスクリプトも使っていないので、`default-src 'self'` でも壊れません。

`topcoat dev` で動かしたときも確かめました（`<script>` の URL のポートは毎回変わるので、ここでは伏せています）。

```text
@@OUT:ex8_dev@@
```

- `topcoat dev` の自動の再読み込みは、別のポート（別のオリジン）の `dev.js` を読み込みます。CSP の `default-src 'self'` があると、これが止められるので、`TOPCOAT_DEV_URL` があるとき（`topcoat dev` のとき）は CSP を付けないようにしました。ほかの3つのヘッダーは付いています。

#### 演習9の解答: 分類の表を1か所にする

`AppError::classify(&RepoError)`（借りたまま分類する）に表を1つだけ置き、`From<RepoError>` はそれを使って、`Internal` のときだけ元の `RepoError` を原因として持つようにしました。`error_page` も `classify` を呼びます。

```diff
@@OUT:ex9_diff@@
```

```text
@@OUTGREP:ex9_tests:^(running|test result)@@
```

- 新しいテスト `classify_and_from_agree` は、4つの種類の `RepoError` について、借りて分類した結果と、渡して変換した結果の状態コードが同じことを確かめます。表を変えるときは、`classify` の1か所だけを直せばよくなりました。

### 仕上げ: CI と同じ3つの手順

9つの演習を終えたコピーで、UNIT 10 の例10-4と同じ3つの手順を動かしました。

```text
@@OUT:ex_final_fmt@@
```

```text
@@OUTTAIL:ex_final_clippy:1@@
```

```text
@@OUTGREP:ex_final_test:^(running|test result|     Running)@@
```

```text
@@OUT:ex_total_stat@@
```

- 整形の確認と Clippy は通り、テストは 41 個から 47 個（ユニット 22・HTTP 17・リポジトリ 8）に増えて、全部通りました。直す前の教材と比べて、13 のファイルで 231 行を足し、54 行を消しました。
- 教材のフォルダ（`Rust/rustnote`）は、この巻の説明と合わせるため、直さずにそのまま置いています。自分の RustNote を作るときは、この9つの直し方を取り入れてください。

### まとめ

- 改善点は、「今は失敗するテスト（か実験）を書く → 直す → 通る → 1つのコミット」の順で、1つずつ直す。項目を private にするときは、まず消して、コンパイラが教える場所を直す。
- 起動は、待ち受けの場所を DB より先に取る（`TcpListener::bind` と `topcoat::serve`）。型は、制御文字のタイトルを断り、`NoteQuery` を `new` でしか作れなくする。お知らせの一覧は1か所にし、足し忘れをテストのコンパイルで止める。
- 画面は、`nav` に名前を付けてヘッダーを折り返し（axe「問題なし」）、最後より大きいページで正しい文を出す。Argon2 には整理券で上限を付け（690 → 199 MiB）、守りのヘッダーは `access_log` の中で付ける（1つのモジュールに `#[layer]` は1つ。dev のときは CSP を付けない）。分類の表は `classify` の1か所にする。

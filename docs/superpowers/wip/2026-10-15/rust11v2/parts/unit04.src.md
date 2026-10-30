## UNIT 04: property test・fuzz・入力の読み取り

### この単元のゴール

- 「どんな入力でも成り立つ決まり」（property）を関数で書き、proptest に、でたらめな入力を何百個も作らせて確かめられる
- 失敗した入力を、proptest が小さく縮める（shrink）様子と、失敗の種が `.proptest-regressions` に保存される仕組みを説明できる
- でたらめな入力を HTTP の入口に送り、RustNote の本物のバグ（NUL で始まる題で 500）を見つけ、原因（SQLite の `length()`）まで追える
- cargo-fuzz（WSL の nightly）で、コードの中を見ながら入力を変えていく fuzz を動かし、proptest が見逃した入力を見つける様子を確かめられる
- でたらめな試しは「見つけないこともある」ことを、実際の回数で説明できる

### たとえ話でつかむ

新しい自動販売機の、出荷前の試験を考えます。

- **チェック表の試験（UNIT 01）:** 人が「10 円玉だけ」「お札とおつりの境目」など、間違えやすい所を考えて、表のとおりに試します。考えた所は確実に試せますが、考えなかった所は試しません。
- **でたらめに押すロボット（property test）:** 「どう押しても、入れたお金より多いおつりは出ない」という**決まり**を1つ決め、ロボットに、でたらめなボタンとお金で何百回も試させます。壊れたら、ロボットは「何をしたら壊れたか」を記録し、余分な操作を1つずつ取り除いて、**いちばん短い手順**にしてくれます（shrink）。
- **機械の中を見ながら押すロボット（fuzz）:** 機械の中のどの部品が動いたかを見ながら、「今まで動かなかった部品が動いた押し方」を少しずつ変えて、何百万回も試します。
- どちらのロボットも、**運しだい**の所があります。今日の試験で見つからなかった不具合が、明日見つかることもあります。

| 自動販売機の試験 | RustNote のテスト |
|---|---|
| チェック表の試験 | UNIT 01 の表の形のテスト（人が選んだ端の値） |
| 決まり | property（「合格・不合格が、決まりと同じ」など） |
| でたらめに押すロボット | proptest（入力の作り方 = strategy を決めて、でたらめに作る） |
| 手順を短くする | shrink（失敗した入力を、小さく縮める） |
| 機械の中を見ながら押すロボット | cargo-fuzz（libFuzzer。通ったコードの道筋を見て、入力を変える） |
| 運しだい | 見つかる回と、見つからない回がある（例4-3） |

> **用語:** property test（プロパティテスト：「どんな入力でも、この決まりが成り立つ」を、道具が作るたくさんの入力で確かめるテスト）

<!-- -->

> **用語:** strategy（ストラテジー：proptest で、入力の作り方を決めるもの。`"\\PC{0,100}"`（制御文字を除く 0〜100 文字）や `any::<u32>()` など）

<!-- -->

> **用語:** fuzz（ファズ：入力を少しずつ変えながら、とても多く試して、パニックやクラッシュを探すこと。それを行う道具を fuzzer と呼ぶ）

<!-- -->

> **用語:** NUL（ヌル：番号が 0 の文字。目には見えない。C 言語や SQLite では、文字列の終わりの印として使われることがある）

### 図で理解する

入力の選び方の、3つのやり方です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="16" width="220" height="230" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="120" y="42" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">表の形のテスト</text>
<text x="120" y="60" font-size="11" fill="#475569" text-anchor="middle">（UNIT 01）</text>
<text x="24" y="92" font-size="11" fill="#0f172a">・人が、端の値を選ぶ</text>
<text x="24" y="116" font-size="11" fill="#0f172a">・数は、10 行くらい</text>
<text x="24" y="140" font-size="11" fill="#0f172a">・毎回、同じ入力</text>
<text x="24" y="172" font-size="11" fill="#047857">◎ 考えた所は、確実に試す</text>
<text x="24" y="196" font-size="11" fill="#b91c1c">× 考えなかった所は試さない</text>
<text x="24" y="220" font-size="11" fill="#b91c1c">　（NUL の題など）</text>
<rect x="250" y="16" width="220" height="230" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="42" font-size="14" font-weight="700" fill="#1d4ed8" text-anchor="middle">property test</text>
<text x="360" y="60" font-size="11" fill="#1d4ed8" text-anchor="middle">（proptest・例4-1〜4-3）</text>
<text x="264" y="92" font-size="11" fill="#0f172a">・作り方（strategy）を決め</text>
<text x="264" y="112" font-size="11" fill="#0f172a">　でたらめに作る</text>
<text x="264" y="136" font-size="11" fill="#0f172a">・数は、256〜数百回</text>
<text x="264" y="160" font-size="11" fill="#0f172a">・失敗したら、縮めて見せる</text>
<text x="264" y="190" font-size="11" fill="#047857">◎ 考えなかった入力も試す</text>
<text x="264" y="214" font-size="11" fill="#b91c1c">× 作り方の外は試さない・</text>
<text x="264" y="232" font-size="11" fill="#b91c1c">　見つけない回もある</text>
<rect x="490" y="16" width="220" height="230" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="600" y="42" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">fuzz</text>
<text x="600" y="60" font-size="11" fill="#c2410c" text-anchor="middle">（cargo-fuzz・例4-4）</text>
<text x="504" y="92" font-size="11" fill="#0f172a">・通ったコードの道筋を見て</text>
<text x="504" y="112" font-size="11" fill="#0f172a">　入力を少しずつ変える</text>
<text x="504" y="136" font-size="11" fill="#0f172a">・数は、1 分で 100 万回以上</text>
<text x="504" y="160" font-size="11" fill="#0f172a">・コードの比べ算の値も試す</text>
<text x="504" y="190" font-size="11" fill="#047857">◎ パニックを、すばやく探す</text>
<text x="504" y="214" font-size="11" fill="#b91c1c">× nightly と Linux（WSL）が要る</text>
<text x="504" y="232" font-size="11" fill="#b91c1c">　DB の中までは見ない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: 入力の選び方。3つは置き換えではなく、組み合わせて使う。見つかった入力は、表の形のテストの1行として残す</div>
</div>

例4-3で見つかったバグの仕組みです。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-11-0401" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="70" width="130" height="60" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="75" y="96" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">フォーム</text>
<text x="75" y="116" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">title=%00</text>
<rect x="180" y="20" width="230" height="80" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="295" y="44" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">Rust: NoteTitle::parse</text>
<text x="295" y="66" font-size="11" fill="#0f172a" text-anchor="middle">chars().count() = 1 文字</text>
<text x="295" y="86" font-size="11" fill="#047857" text-anchor="middle">1〜80 なので、合格</text>
<rect x="180" y="120" width="230" height="90" rx="8" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="295" y="144" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">SQLite: CHECK</text>
<text x="295" y="164" font-size="10" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">length(title) BETWEEN 1 AND 80</text>
<text x="295" y="184" font-size="11" fill="#0f172a" text-anchor="middle">length() は NUL より前だけ数える</text>
<text x="295" y="202" font-size="11" fill="#b91c1c" text-anchor="middle">→ 0 文字なので、断る</text>
<rect x="450" y="120" width="260" height="90" rx="8" fill="#fef2f2" stroke="#ef4444" stroke-width="2"/>
<text x="580" y="148" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">500 Internal Server Error</text>
<text x="580" y="170" font-size="11" fill="#0f172a" text-anchor="middle">利用者の入力のまちがいなのに、</text>
<text x="580" y="190" font-size="11" fill="#0f172a" text-anchor="middle">サーバーの故障として扱われる</text>
<text x="580" y="50" font-size="11" fill="#047857" text-anchor="middle">直し方: parse で制御文字を断る</text>
<text x="580" y="70" font-size="11" fill="#047857" text-anchor="middle">→ 400（入力のまちがい）になる</text>
<line x1="142" y1="100" x2="176" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-0401)"/>
<line x1="295" y1="102" x2="295" y2="116" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-0401)"/>
<line x1="412" y1="165" x2="446" y2="165" stroke="#475569" stroke-width="2" marker-end="url(#ar-11-0401)"/>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図11-?: NUL で始まる題の行方。Rust と SQLite で「文字数」の数え方が違うので、Rust の検査を通った題が、DB の CHECK で断られて 500 になる</div>
</div>

### まず動かす最小コード

#### 例4-1: NoteTitle::parse の決まりを、でたらめな文字列で確かめる

proptest を、練習用のコピーの dev-dependency に足します。

```powershell
cargo add --dev proptest@=1.12.0
```

```text
@@OUTGREP:u04_add:^(    Updating|      Adding proptest|     Locking)@@
```

（`Features:` の一覧と、`proptest` が使う 14 個のクレートの `Adding` の行は、省いています。）

> **▼ このコードがやること（先に日本語で）:** 決まり「前後の空白を除いた文字数が 1〜80 なら合格」を、`rule` という関数で書きます。1つ目のテストは、proptest がどんな文字列を作るのかを、5つだけ表示します（種を決めた `TestRunner::deterministic()` なので、毎回同じものが出ます）。2つ目は「`NoteTitle::parse` の合格・不合格が、`rule` と同じ」、3つ目は「合格した題を、もう一度 `parse` しても同じ題になる」を、それぞれ 256 個のでたらめな文字列で確かめます。

```rust
@@FILE:rustnote/tests/u04_title_prop.rs@@
```

```powershell
cargo test --test u04_title_prop -- --nocapture --test-threads=1
```

**▼ 実行結果**

```text
@@OUTGREP:u04_title_prop:^(running|test |[0-9]+ 文字|ok)@@
```

（`test parse_… ...` の行のうしろに出る `proptest: FileFailurePersistence …` は、解説3で説明します。）

#### ▼ コードを1つずつ分解して解説

##### 解説1: 「答え」ではなく「決まり」を書く

- UNIT 01 の表は、「`"あ".repeat(80)` → 合格」のように、入力ごとに**答え**を書きました。property test では、入力が何かは分からないので、「合格・不合格は `rule` と同じ」のように、**どんな入力でも成り立つ決まり**を書きます。
- 決まりの書き方には、よく使う形があります。「別の方法で求めた答えと同じ」（`rule` と比べる）・「2回やっても同じ」（3つ目のテスト）・「元に戻せる」・「やってはいけないことが起きない」（パニックしない・500 にならない。例4-3）です。

##### 解説2: 入力の作り方（strategy）

- `"\\PC{0,100}"` は、「制御文字などの目に見えない文字（Unicode の分類 `C`）以外の文字を、0〜100 個」という正規表現です。Rust の文字列の中で `\` を書くので、`\\` と2つ重ねます（エラー1）。
- 表示した5つのように、proptest は、いろいろな国の文字・絵文字・記号を混ぜた文字列を作ります。人がチェック表に書かない文字です。
- `proptest!` の中の `#[test]` は、何も指定しなければ、1つのテストにつき **256 個**の入力を試します。

##### 解説3: 失敗の種を保存する場所

- 出力から省いた `FileFailurePersistence … failed to find lib.rs or main.rs` は、「失敗したときに、その入力の種（seed）を保存するファイルの場所を、`src/lib.rs` の隣で探したが、見つからなかった」という知らせです。`tests/` の中のテストでは、テストのファイルの隣（`tests/u04_title_prop.proptest-regressions`）に保存されます（例4-2）。失敗しなければ、ファイルは作られません。

### もう一歩: 実務に近い形

#### 例4-2: 失敗した入力を、小さく縮める（shrink）

UNIT 01 の「わざと間違えた版」を、property test で確かめます。

> **▼ このコードがやること（先に日本語で）:** 間違えた版を2つ用意します。`forgot_trim` は前後の空白を除き忘れた版、`counted_bytes` は文字数ではなくバイト数（`len()`）で数えた版です。それぞれが `rule` と同じになるかを、例4-1と同じ作り方の文字列で確かめます。

```rust
@@FILE:rustnote/tests/u04_shrink.rs@@
```

```powershell
cargo test --test u04_shrink -- --test-threads=1
```

**▼ 実行結果（1回目）**

```text
@@OUTGREP:u04_shrink1:^(test |proptest: Saving|cc |thread|Test failed|  left|  right| right|minimal|\tsuccesses)@@
```

- `counted_bytes` の版が失敗し、proptest は `minimal failing input`（縮めた、失敗する入力）を表示しました。まだ長く見えますが、この間違いは「80 バイトを超え、80 文字以下」の文字列でしか表に出ないので、21 文字より短くは縮められません（1文字は多くても 4 バイトなので、20 文字では 80 バイトを超えません）。`successes: 1` は、1つ通ったあとの、2つ目の入力で、もう失敗したという意味です。
- `forgot_trim` の版は、**通ってしまいました**。これは、エラー3でくわしく見ます。
- `Saving this and future failures in …\tests\u04_shrink.proptest-regressions` のとおり、失敗の種が保存されました。中身です。

```text
@@OUT:u04_shrink_regressions@@
```

同じコマンドを、もう一度動かします。

**▼ 実行結果（2回目）**

```text
@@OUTGREP:u04_shrink2:^(test |minimal|\tsuccesses)@@
```

- 2回目は、`successes: 0` のとおり、保存された種から、**同じ入力**が最初に試されて、同じように失敗しました。ファイルの先頭のコメントのとおり、このファイルを Git に入れておくと、ほかの人のパソコンや CI でも、同じ失敗を最初に試します。

#### 例4-3: でたらめな入力を、HTTP の入口に送る

今度は、関数1つではなく、RustNote の**入口**（検索・ページ番号・作成のフォーム）に、でたらめな入力を送ります。決まりは「どんな入力でも、500 にならない」です。500 は「サーバーの故障」で、利用者の入力のまちがいなら、400 で返すのが約束です。

> **▼ このコードがやること（先に日本語で）:** alice を1人だけ登録し、300 回、次の2つを送ります。①`GET /notes?q=…&page=…`（検索の言葉は、でたらめなバイトを `%XX` の形にしたもの。ページ番号は、大きな数・負の数・数字でない文字）。②`POST /notes`（題と本文も、でたらめなバイト）。①は 200 か 400、②は 303 か 400 だけが返ることを確かめ、返った状態コードの回数を数えて表示します。proptest の `TestRunner` を自分で動かし、`tokio` の `Runtime` の `block_on` で、非同期の処理を待ちます。

```rust
@@FILE:rustnote/tests/u04_http_prop.rs@@
```

```powershell
cargo test --test u04_http_prop -- --nocapture
```

**▼ 実行結果**

```text
@@OUTGREP:u04_http_prop:^(GET /notes →|POST /notes →|thread|called|test random)@@
```

- **500 が出ました。** `title=%00&body= → 500 Internal Server Error` です。題が `%00`（NUL 1文字）のノートを作ると、RustNote が 500 を返しました。
- 最後の `([], "0", [0], [])` は、縮めた入力です（検索の言葉は空・ページは `"0"`・題は NUL の1バイト・本文は空）。でたらめな長い入力から始まって、「題が NUL 1バイト」という、いちばん小さい形まで縮めてくれました。
- `POST /notes → 500` が 147 回あるのは、縮める途中で、失敗する入力を何度も試したからです。このとき、標準エラー出力には、RustNote の `[error] サーバーの中でエラーが起きました <- DB の操作に失敗しました <- SQLite failure: CHECK constraint failed: length(title) BETWEEN 1 AND 80` という行が、147 行出ていました（上の出力からは省いています）。

**原因を、1つずつ確かめます。** 題に NUL を含む4つの場合を、Rust・SQLite・HTTP の3か所で比べます（`tests/u04_nul.rs`）。

```rust
@@FILE:rustnote/tests/u04_nul.rs@@
```

```text
@@OUTGREP:u04_nul:^題@@
```

- Rust の `chars().count()` は NUL も1文字と数えるので、`NoteTitle::parse` は4つとも合格させます。ところが SQLite の `length()` は、**NUL より前の文字だけ**を数えます（SQLite の決まり）。NUL で**始まる**題は `length()` が 0 になり、DB の `CHECK (length(title) BETWEEN 1 AND 80)` に断られます（図の2つ目）。RustNote は、DB の失敗をサーバーの故障として扱うので、500 になります。
- NUL が途中にある題（`"a\0"`・`"a\0b"`）は、`length()` が 1 なので、CHECK を通って保存されました。
- おまけの行のとおり、UTF-8 として読めないバイト（`%FF`・途中で切れた「あ」）は、断られずに、`�`（U+FFFD、16 進数で `EFBFBD`）に置き換えて受け取られました。

**直した版で確かめます。** 練習用のコピーの `src/domain.rs` で、題に制御文字（NUL・タブなど）があれば断るようにします（教材の `Rust/rustnote` は変えていません）。

```diff
@@FILE:out/u04_diff_domain.txt@@
```

```text
@@OUTGREP:u04_nul_fixed:^題@@
@@OUTGREP:u04_http_prop_fixed:^(GET /notes →|POST /notes →|test random)@@
```

- NUL を含む題は、4つとも 400（入力のまちがい）になり、でたらめな入力 300 回で、500 は 0 回でした。直した版でも、`cargo test --lib`（19 個）と `cargo test --test http`（14 個）は全部通りました。
- 直した版では、`POST /notes → 400` が 255 回に増えました。でたらめなバイトの多くに、制御文字（0〜31 番など）が入っているからです。

**運しだい、ということ。** 直す前の版で、同じテスト（300 回）を 10 回くり返しました。

```text
@@OUT:u04_http_prop_runs@@
```

- 500 が見つかったのは、10 回のうち 3 回だけでした。題の最初のバイトが 0 になる入力は、1 回の試しで 256 分の 1 くらいしか作られないからです。見つからなかった7回は、`test random_inputs_never_give_500 ... ok` と、通ってしまいました。
- しかも、`TestRunner` を自分で動かしたこのテストでは、`proptest: FileFailurePersistence::SourceParallel set, but no source file known` と表示され、**失敗の種は保存されませんでした**（`proptest!` で書いたテストとは違います）。だから、見つかった入力は、`tests/u04_nul.rs` のような**ふつうのテスト**として残します。次からは、毎回、必ず確かめられます。

#### 例4-4: fuzz（cargo-fuzz）で、もっとたくさん試す

fuzz には、cargo-fuzz を使います。cargo-fuzz は、libFuzzer という fuzzer を使い、Rust の **nightly**（毎晩作られる、試しの版）が要ります。Windows（ARM64）ではなく、WSL の Ubuntu（aarch64）で動かしました。

```bash
# WSL（Ubuntu）の中で（1回だけ）
rustup toolchain install nightly --profile minimal
cargo install cargo-fuzz --locked
```

```text
@@OUT:u04_fuzz_versions@@
```

RustNote 全体は、DB（libSQL の C のコード）まで組み立てる必要があるので、ここでは、検査済みの型の `src/domain.rs` だけを取り込む、小さな練習用のクレート（`fuzz_lab`）を作ります。`domain.rs` は、標準ライブラリと `thiserror` だけを使うので、そのまま取り込めます。

```toml
@@FILE:fuzz_lab/Cargo.toml@@
```

```rust
@@FILE:fuzz_lab/src/lib.rs@@
```

```bash
cd fuzz_lab
cargo +nightly fuzz init --target parsers
cargo +nightly fuzz add query_offset
```

`fuzz/fuzz_targets/` に、fuzz の対象（fuzz target）を2つ書きます。

> **▼ このコードがやること（先に日本語で）:** `parsers` は、fuzzer が作る文字列を、RustNote の検査済みの型の `parse` に次々に渡し、パニックしないことと、「合格した値を、もう一度 `parse` しても同じ」を確かめます。`query_offset` は、`NoteQuery` を**項目を直接書いて**作り（`page` は `pub`）、`offset` がパニックしないかを試します。

```rust
@@FILE:fuzz_lab/fuzz/fuzz_targets/parsers.rs@@
```

```rust
@@FILE:fuzz_lab/fuzz/fuzz_targets/query_offset.rs@@
```

`parsers` を 60 秒だけ動かします。

```bash
cargo +nightly fuzz run parsers -- -max_total_time=60
```

```text
@@OUTGREP:u04_fuzz_parsers:^(INFO: (Seed|A corpus)|#2\t|#[0-9]+\t(pulse|DONE)|Done )@@
```

- 61 秒で、1,847,970 回試して、パニックは 0 回でした。`cov`（通ったコードの場所の数）は 289 まで増え、`corp`（「新しい場所を通った入力」として取っておいた入力）は 418 個になりました。fuzzer は、この取っておいた入力を少しずつ変えて、次の入力を作ります。
- 例4-3の NUL のバグは、この fuzz では見つかりません。`domain.rs` だけを取り込んだので、DB の CHECK は通らないからです。fuzz は、**取り込んだ範囲の中**だけを調べます。

`query_offset` を動かします（見つかったら、そこで止まります）。

```bash
cargo +nightly fuzz run query_offset
```

```text
@@OUTGREP:u04_fuzz_query:^(#2\t|thread|attempt to|SUMMARY|MS: |Failing input|Output of|Reproduce with|exit=)|^\t(0$|fuzz/artifacts|cargo fuzz run)@@
```

- `domain.rs:370:19` の `attempt to subtract with overflow`（引き算で、桁があふれた）で止まりました。370 行目は `u64::from(self.page - 1) * u64::from(PER_PAGE)` です。`Output of std::fmt::Debug: 0` のとおり、`page` が **0** のとき、`0 - 1` が `u32` で表せず、パニックします（fuzz の組み立ては、debug と同じく、あふれを確かめます）。
- `MS: 2 ChangeByte-CMP- DE: "\000\000\000\000"` は、入力の変え方の記録です。`CMP` は、「コードの中で比べている値（ここでは 0）を、入力に入れてみた」という意味です。fuzzer は、コードの中を見ているので、`0` のような値を、すぐに試せます。
- 同じことを、proptest の `any::<u32>()`（256 回）でも確かめていました（`tests/u04_query_prop.rs`）。

```text
@@OUTGREP:u04_query_prop:^test @@
```

- proptest は、`page = 0` を見つけられず、**通って**しまいました。`u32` は 42 億通りあるので、256 回のでたらめでは、0 にまず当たりません。
- 今の RustNote は、`NoteQuery` を、`page` を 1 以上にそろえる `NoteQuery::new` でしか作っていないので、このパニックは起きません（問3）。ただし、項目が `pub` なので、だれかが `NoteQuery { page: 0, .. }` と書けば起きます。**型が、約束（page は 1 以上）を守っていない**ことを、fuzz が見つけました（改善点）。

### よくあるエラーと読み方

#### エラー1: 正規表現の `\` を、1つだけ書いた

```rust
@@FILE:errs/e04_escape.rs@@
```

```text
@@OUTHEAD:u04_err_escape:12@@
@@OUTGREP:u04_err_escape:^error(\[E0277\]|: could not)@@
```

- **何が起きたか:** Rust の `"…"` の中では、`\` は「次の文字と組で、特別な文字を表す」印です（`\n` が改行、`\t` がタブ）。`\P` という組は無いので、`unknown character escape`（知らない組み合わせ）になりました。続く2つの E0277 は、最初のエラーのせいで出た、おまけのエラーです。最初のエラーから直します。
- **直し方:** `"\\PC{0,100}"` のように `\` を2つ重ねるか、`help:` のとおり `r"\PC{0,100}"`（`r` を付けた、`\` をそのまま書ける文字列）にします。

#### エラー2: `run` に渡す関数の中で、外の変数を書き換えた（E0596）

例4-3の数えるための `seen` を、`RefCell` に入れずに `let mut seen = BTreeMap::new();` と書いた場合です。

```text
@@OUT:u04_err_fnmut@@
```

- **何が起きたか:** `TestRunner::run` は、渡す関数を `Fn`（外の変数を、読むだけの関数）として受け取ります（`expects Fn instead of FnMut`）。その中で `seen` を書き換えようとしたので、「`Fn` のクロージャーの中で、`seen` を書き換えのために借りられない」と言われました（第3巻の借用の決まり）。
- **直し方:** 例4-3のように `RefCell::new(BTreeMap::new())` に入れ、使うときに `seen.borrow_mut()` で書き換えます。`RefCell` は、「書き換えのための借用を、コンパイルのときではなく、動かしているときに確かめる」入れ物です（第3巻 UNIT 09）。

#### エラー3（通ってしまう）: 前後の空白を除き忘れた版を、見つけられなかった

例4-2の `forgot_trim` は、間違えた版なのに、256 回の試しを**通りました**。

- **なぜか:** `forgot_trim` が `rule` と違う答えを出すのは、「空白だけの文字列」か、「前後に空白があって、ちょうど 80 文字の境目をまたぐ文字列」のときだけです。`"\\PC{0,100}"` は、いろいろな国の文字を、まんべんなく作るので、空白だけの文字列は、ほとんど作られません。
- **直し方:** 間違いが出やすい形に、作り方を**寄せます**。空白（半角・全角・タブ）と `a` だけで、0〜85 文字の文字列を作ります（`tests/u04_spaces.rs`）。

```rust
@@FILE:rustnote/tests/u04_spaces.rs@@
```

3回動かしました（毎回、保存された種を消してから）。

```text
@@OUTGREP:u04_spaces1:^(minimal|\tsuccesses)@@
```

- 3回とも見つかりました。`successes` は 9・0・37 で、10 個目・1 個目・38 個目の入力で失敗しました（上は1回目）。3回とも、81 文字（前後の空白を除くと 78〜80 文字）の文字列まで縮みました。`forgot_trim` は 81 文字なので不合格、`rule` は前後の空白を除くと 80 文字以下なので合格、という「80 文字の境目」での食い違いです。
- この作り方で試した別の回には、空白1つの `" "` や、タブ1つの `"\t"`（空白だけの食い違い）まで縮んだこともありました。どちらの食い違いを先に見つけたかで、縮み方が変わります。
- **学べること:** property test は、作り方の**外**の入力は試しません。「どんな入力で間違いが出るか」を考えて、作り方を決めます。そして、UNIT 01 の表（空白だけ・80 と 81）も、残しておきます。表は、毎回、必ず同じ所を試すからです。

### RustNote ではここで使う

- **見つかったバグ:** 題が NUL で始まるノートを作ると 500 になります（例4-3）。利用者は、ふつうのブラウザの入力欄からは NUL を入れにくいですが、フォームを自分で作れば、だれでも送れます。教材の `Rust/rustnote` のコードは変えていません。直すなら、例4-3の差分のように、`NoteTitle::parse` で制御文字を断ります（本文の `NoteBody` には CHECK が無いので、NUL が入っても 500 にはなりません）。
- **型が守っていない約束:** `NoteQuery` の項目は `pub` なので、`page = 0` で作るとパニックします（例4-4）。項目を `pub` でなくし、`NoteQuery::new` だけで作れるようにすると、型が約束を守れます（改善点）。
- **読めないバイト:** UTF-8 として読めないバイトは、断られずに `�` に置き換えて受け取られます。断りたいときは、`parse` で `\u{FFFD}` を断る方法があります（今は受け付けています）。
- RustNote の `tests/` には、property test も fuzz もありません。この単元のテストは、練習用のコピーにだけ足しました（`Cargo.toml` の `proptest` も同じ）。

### 練習問題

**問1:** `Email::parse` について、「合格したアドレスを、もう一度 `parse` しても同じ」と「合格したアドレスには、空白も大文字も無く、`@` はちょうど1つ」を、property test で確かめてください。でたらめな文字列では、ほとんど合格しないので、「メールアドレスらしい形」の作り方を考えてください。

**問2:** 例4-3の、直した版の結果では、`GET /notes → 400` が 227 回、`200` が 73 回でした。検索の言葉は、どんなバイトでも 200 になります。では、400 は、どの入力から来たのでしょうか。`page_text()` の作り方から考えてください。

**問3:** 例4-4の `query_offset` を、URL から作るときと同じ `NoteQuery::new(None, page)`（`page` は `Option<u32>`）で作る版にして、30 秒動かすと、どうなるでしょうか。

### 解答と解説

**問1の解答:**

```rust
@@FILE:rustnote/tests/u04_q1.rs@@
```

```text
@@OUTGREP:u04_q1:^(test |1000|ok)@@
```

- 2つの property は、どちらも 256 回で通りました。`how_many_are_accepted` のとおり、この作り方では 1000 個のうち 179 個が合格します。合格する入力が少なすぎると、`if let Ok(…)` の中の確かめが、ほとんど動かずに「通って」しまうので、割合も見ておきます。
- 作り方には、前後の空白・大文字・全角の文字（`Ａ`・`Ｂ`）・`.` や `-` を混ぜました。`parse` が、空白を除いて小文字にそろえることを、確かめるためです。

**問2の解答:**

- `page_text()` は、3つの作り方から1つを、同じくらいの割合で選びます。①`any::<u64>()` の数: ほとんどが `u32` の上限（約 42 億）より大きいので、`?page=` を `u32` として読めず 400。②`any::<i32>()` の数: 半分くらいが負の数なので 400。③`"[-+0-9a-z.]{0,8}"`: 数字だけになることは少なく、多くは 400。
- 全部を合わせると、だいたい 4 分の 3 くらいが 400 になり、300 回中 227 回と合います。ページ番号のまちがいは、500 ではなく 400 になる、ということも、同時に確かめられました。

**問3の解答:**

```rust
@@FILE:fuzz_lab/fuzz/fuzz_targets/query_new.rs@@
```

```bash
cargo +nightly fuzz run query_new -- -max_total_time=30
```

```text
@@OUTGREP:u04_fuzz_query_new:^(#[0-9]+\tDONE|Done |exit=)@@
```

- 31 秒で 10,639,929 回試して、パニックは 0 回でした。`NoteQuery::new` は、`page.unwrap_or(1).max(1)` で、0 を 1 にそろえるからです。`cov` が 45 で止まったのは、調べる場所が少ないからです。
- 同じ `offset` でも、作り方の入口（`new` か、項目を直接書くか）で、結果が変わります。入口を `new` の1つにしぼれば、約束を型が守れます。

### まとめ

- property test は、「どんな入力でも成り立つ決まり」を書き、proptest に、でたらめな入力を何百個も作らせて確かめる。失敗すると、入力を小さく縮め、`proptest!` なら種を `.proptest-regressions` に保存する。決まりは「別の方法と同じ」「2回やっても同じ」「500 にならない・パニックしない」などの形で書く。
- でたらめな入力を HTTP の入口に送ると、題が NUL で始まると 500 になる、本物のバグが見つかった。Rust は NUL も1文字と数え、SQLite の `length()` は NUL より前だけを数えるので、DB の CHECK で断られる。ただし、見つかったのは 10 回中 3 回。見つかった入力は、ふつうのテストとして残す。
- cargo-fuzz（WSL の nightly）は、コードの中を見ながら、1 分で 100 万回以上試す。`NoteQuery { page: 0 }` のパニックを、proptest の 256 回が見逃したのに、すぐに見つけた。property test は作り方の外を試さないので、間違いが出やすい形に作り方を寄せ、UNIT 01 の表も残す。

# 中断時点の引き継ぎメモ

## 2026-10-31（続き6）: 第1巻を完成（main: fc229d1 U01〜03 版合わせ・7f96140 U04・2d16bf5 U05・5e8d8ae U06・4f5b41c U07・6849718 U08・85de76e U09・5c6e582 U10）

素材は `rust01v2/`（parts の src・meta・intro・outro、out の .txt、cap01_*.sh、mask01.py・finish01.py（用語は1つの表に足す版）・doctor.ps1・u05_machine.ps1 など、u06src、u08_app_snapshot）。

- **版:** いつもの stable を 1.97.1 → 1.99.1 に上げた（`rustup update stable`。出力は UNIT 04 に使った）。rust-analyzer は別の拡張機能のフォルダ（`--extensions-dir`）で 0.3.3097 を確かめた。**Git for Windows（2.53.0 → 最新 2.56.0.4）と VS Code（1.129.1 → 1.143.1）は、作業中に使われていた（VS Code のプロセスと Git Bash）ので更新していない**（巻頭に「新しい版が出ている」と書き添えた）。
- **試し用の Rust:** PC の Rust を変えないよう、`RUSTUP_HOME`・`CARGO_HOME` を `~\rust-try`（入れた直後の状態）と `~\rsb`（実験用）に分けた。**置き場のパスが長いと、ツールチェーンの中の rlib のパスが 260 文字を超えてリンクが LNK1104**（スクラッチの中は 267 文字）。subst のドライブはシェルをまたいで見えず、ジャンクションは rustc が元のパスに直すので効かない。`~\rust-study` も作業フォルダとして作った（どれも作業のあと消す）。
- **踏んだもの:** ①Git Bash の cwd が RustNote のフォルダだと、そこから起動した PowerShell でも `rust-toolchain.toml`（1.99.1）が効き、rustup の操作がそちらに向く（toolchain の自動インストール・target remove の取り違え）。PowerShell は必ず `Set-Location ~` から始める。②rust-docs の入ったツールチェーンを消すと、Windows では 1.5 万ファイルの削除に十数分かかる。③PowerShell で `& {…} | Select-Object -First 1` は、ネイティブのコマンドの終了コードを狂わせる（doctor.ps1 は全部受け取ってから見る）。④`powershell -File ~\…` の `~` は直されない（`$HOME` を使う）。⑤Git Bash から開いた PowerShell は x64（AMD64）で PATH も違う。⑥topcoat dev はアプリを `<名前>.topcoat-dev.exe` で動かす（止めるときはその名前）。
- **Turso CLI:** 公式の `curl -sSfL https://get.tur.so/install.sh | bash` の実行は、権限の自動判定で止められた（外のコードの実行）。スクリプトを取ってきて読むだけにし、UNIT 09 は入れ方の説明＋同じ sqld（Docker の libsql-server 0.24.33）での確かめにした。doctor.ps1 では Turso CLI を任意（`--`）にした。
- **RustNote の重要な発見（コードは変えていない）:** ①**Windows の debug ビルドで、リモートの libSQL（http://）につなぐと main のスタックがあふれる**（`STATUS_STACK_OVERFLOW`。release と `RUSTFLAGS=-C link-arg=/STACK:8388608` では動く。ローカルのファイルなら debug でも動く）。②`DATABASE_URL=127.0.0.1:8080`（http:// 忘れ）はエラーにならず、`127.0.0.1` というファイルの代替データストリーム `8080` に DB を作って起動する。③RustNote は `cargo run` だけだとアセットの束が無いと起動しない（日本語のエラーで案内する）。
- **確かめた事実（抜粋）:** rustup-init 1.29.2 の対話の画面（stable 1.99.1・6 components）。受付の rustc.exe・cargo.exe は rustup.exe と同じハッシュ。`rust-toolchain.toml` の版が無いと自動で入れるが、1.29.2 では非推奨の warn が出る（`rustup install` を使う）。Build Tools 2022 17.14・MSVC 14.44.35207・Hostarm64 に arm64/x64/x86。PE の印で rustc・VS Code・Git・docker は ARM64、Git の usr/bin/bash.exe と fly.exe は x64。無い名前のリンカーで msvc の note 3行（VS Code is a different product）。WSL 2.6.3.0、小さなファイル 2000 個で /mnt/c 約 3.6 秒・Linux 側 約 0.1 秒。Docker Desktop 4.68.0（Windows 11 Home の ARM64 でも動く。公式の Arm の条件は Pro 以上と書かれている）。cargo install topcoat-cli 0.11.0 は 243 部品・107 秒。topcoat new --minimal のアプリは cargo run だけだと no asset bundle で panic。cargo +1.90.0 check は requires rustc 1.99。
- **後片付け（このメモを書いた時点で実行中か、まだ）:** `~\rsb`・`~\rust-try`・`~\rust-study` とスクラッチの rust01/sb を消す。
- **次:** 第13巻の「全巻の用語集・索引」→ finish_volume → 旧 `Rust/13_….pdf` の削除。

## 2026-10-31（続き5）: 第12巻を完成（main: 53f22ad U01 取り直し・014bcdb U02・43582ad U03・a7a8d19 U04・798d03d U05・7d8c8fe U06・3622ffd U07・45d4197 U08・dbd492f U09・281f044 U10）

素材は `rust12v2/`（parts の src・meta・intro・outro、out の .txt、cap12_*.sh、道具の .py、smoke.ps1、count_proxy.py、rustnote/examples（u01_*・u10_*）と check_fly.py、u07src）。ビルド先は `rust12/target`（UNIT 07 は `target-u07`）。

- **第12巻:** UNIT 01 を 1.99.1・trixie で取り直し、UNIT 02〜10 を書いた（117 ページ・図 17 枚）。巻頭・巻末のお知らせを外し、旧 `Rust/12_….pdf` を削除。clear_refs で第6・8・9・11・12巻の「第12巻 UNIT nn（準備中）」を外した（第11巻の「CI を置く手順は第12巻」は、実際に書いた第13巻 UNIT 10 に向け直した）。
- **Fly.io・Turso:** アカウントは使っていない。`~/.fly/config.yml` にログイン済みのトークンがあるので、API を呼ぶ flyctl のコマンド（`fly platform regions`・`fly config validate`・`fly releases` など。`--help` も含めて）は一切動かさない。動かしてよいのは `fly version` だけ。手順は公式のドキュメント（docs.fly.io・docs.turso.tech）で確かめた物を、コマンドと説明だけ載せた。flyctl は v0.5.1（`fly version upgrade` は Start-Process で失敗し、`fly.exe.old` が残った。flyctl.exe を fly.exe にコピーして直した）。
- **確かめた事実（UNIT 09・10）:** toxiproxy で DB までの片道の遅れを 0/10/50/100 ms にすると、GET /notes は 19/102/367/666 ms（約 6.5 往復）、POST /login は約 4.7、/health は約 2.2。数える係（count_proxy.py）で数えると、libsql 0.10.0 のリモートは、読む SQL（query）1つに describe と cursor の2回、書く SQL（execute）に batch の1回、接続を閉じる close は返事を待たない（GET /notes は8回送って6回待つ）。片道 3,000 ms で /health は 5 秒で打ち切り（curl 28）。ほかの接続が開いている間は -wal が残り、rustnote.db だけのコピーは 5 行（VACUUM INTO は 8 行）。強制終了のあとは -wal があれば 10 行全部見える。-wal を残して rustnote.db だけ上書きすると、件数は数えられるのに integrity_check とログインが `database disk image is malformed`。v2（ORDER BY の列名の誤り）は /health 200 のままスモークの一覧だけ 500、v1 のイメージに戻すと全部 OK、cargo test では4つ失敗して気づけた。`cargo update` で変わるのは log 0.4.34 → 0.4.35 だけ。cargo audit は第11巻と同じ8件（libsql 0.10.0 は今も最新）。
- **注意（この巻で踏んだもの）:** ①動いている bash のスクリプトを編集すると、実行が壊れる（cap12_u02.sh。止めて、残りを別のスクリプトにした）。②Git Bash の `$!` は Windows の PID ではない（`taskkill //IM` を使う）。③CARGO_TARGET_DIR を共有して、ソースのコピーを何組か作ると、cargo は mtime で「新しい」と判断して作り直さない（3つの実行ファイルが同じになった）。ビルドの前に src と migrations を touch する。④PowerShell の関数の中の Write-Output は戻り値に混ざる（smoke.ps1 は Write-Host にした。5.1 は UTF-8 の BOM と `[Console]::OutputEncoding` も要る）。⑤curl の `-w` に日本語を入れると文字化けする（日本語は echo で足す）。
- **次:** 第1巻（UNIT 01〜03 を 1.99.1・Topcoat 0.11 にそろえる → UNIT 04〜10。Turso CLI は WSL でログインせず、flyctl は `fly version` だけ）→ 第13巻の「全巻の用語集・索引」と finish_volume・旧 `Rust/13_….pdf` の削除。

## 2026-10-31（続き4）: 第13巻の UNIT 03〜10 と総合演習（main: 2f7a4a7 U03・849d9ab U04・395a338 U05・9915137 U06・0d593c2 U07・6294bf5 U08・f12e177 U09・5ecc346 U10・f9d14ba 総合演習）

素材は `rust13v2/`（parts の src・meta・intro・outro、out の .txt、errs_u03〜u09、examples（u02〜u06 の練習のプログラム）、u09lab（Playwright の measure.mjs と package.json）、fixed-patches（総合演習の9つのコミット。`git am` で当たる）、cap13_*.sh と道具の .py）。ビルド先は `rust13/target`（教材のコピー）と `target_fixed`（総合演習のコピー）。

- **残り:** 第13巻は「全巻の用語集・索引」だけ（第12巻と第1巻を仕上げてから、全巻の用語ミニ辞典を集めて作る。読みの順に並べるなら pykakasi を入れる）。そのあと finish_volume・旧 `Rust/13_….pdf` の削除。
- **RustNote を変えたところ（版合わせ）:** `Dockerfile` の FROM を `rust:1.99.1-trixie`・`debian:trixie-slim` に替えた（Docker 29.3.1・linux/arm64 で bookworm と trixie の両方をビルド・起動して確かめてから。trixie: イメージ 170MB・glibc 2.41・最初のビルド 235 秒）。README の「Docker は未検証」を直した。コードの改善点は、教材のフォルダでは直さず、総合演習のコピーでだけ直した。
- **Docker Desktop:** `docker version` が応答しなかったのは、`\\.\pipe\dockerBackendApiServer: Access is denied`（古いプロセスが残っていた）。Docker の全プロセスを止めて 10 秒待ってから起動し直すと動いた。
- **確かめた事実（抜粋）:** NoteTitle は NUL・全角のメール・😀×80 を通す。release では `NoteQuery{page:0}.offset()` が 85899345900。`#[async_trait]` 無しの dyn は E0038。持ち主の条件を消すと notes_are_isolated_per_owner だけが止める。update の find を省くと他人のノートに 409。ログイン失敗の時間は A39・B41・C36 ms、ダミー照合を外すと B は 4 ms。アクセスログに 405・403 は残らない。`href!` の {id} 忘れはコンパイルが通り、表示でパニック（500、アクセスログにも残らない）。最後より大きいページで「まだノートがありません」。日時は UTC のまま。幅 375・320px でヘッダーが 392px。CSS の閉じ括弧忘れ・変数名の誤り・asset! の名前の誤りは cargo build を通る（asset! は bundle で止まる）。1つのモジュールに #[layer] を2つ書くと、ルーターを組み立てるときにパニック。Argon2 の整理券で 690 → 199 MiB・78.5 → 116.7 rps。topcoat dev の dev.js は別のポートなので CSP の default-src 'self' だと止まる（dev のときは CSP を付けない）。
- **注意:** Git Bash の `$!` は Windows の PID ではない。`taskkill //PID $!` は別のプロセスを止めることがある（総合演習2で python の待ち受けが消えた）。`taskkill //IM rustnote.exe` を使う。
- **次:** 第12巻（UNIT 01 を 1.99.1・trixie にそろえてから UNIT 02〜10。Docker は動く。Fly.io・Turso の実アカウントは使わない）→ 第1巻 UNIT 04〜10 → 第13巻の用語集・索引。

## 2026-10-31（続き3）: 第11巻を完成（main: 75d1c40 UNIT 05・0b0e084 UNIT 06・78e4f5c UNIT 07・a9ad4be UNIT 08・c629320 UNIT 09・94e3efc UNIT 10）

素材は `rust11v2/`（parts・out（.raw を除く）・errs・variants（metrics.rs）・staging_u07・staging_u08・e2e（Playwright。node_modules を除く）・fuzz_lab・rustnote/（tests・examples・Cargo.*）・cap11_*.sh）。練習用のコピーは3つ（`rust11/rustnote`・UNIT 07 と 10 用の `rustnote_u07`（target11b）・cargo-mutants 用の `rustnote_mut`（http・u06_attacks・repository だけ））。

- **道具の版:** Playwright 1.64.1（Chrome は 155.0.8059.80 が動いた）・@axe-core/playwright 4.14.0・cargo-mutants 27.1.0（Windows ARM64 は winapi 0.2.8 で組み立て不可、WSL で動かした）・tracing 0.1.44・tracing-subscriber 0.3.23・criterion 0.8.2・oha 1.16.0・cargo-audit 0.22.2。
- **確かめた事実（抜粋）:** E2E: maxlength=80 は Chrome でも UTF-16（絵文字 40 個）、axe は一覧の2つの nav に landmark-unique。守り: CSRF は /login・/signup も 403、検索のかっこを外した版は本文が空のテストでは u06_attacks も http.rs も見逃し、本文に秘密を入れると気づく。cargo-mutants は auth.rs 9 caught・4 unviable、notes.rs 6 caught・6 unviable、見逃し 0（SQL の文字列は壊さない）。tracing の JSON ログ・/metrics（AtomicU64 の static）・SLO（NUL の題の 500 が 3 回で予算 300%）。criterion: parse a 63.7 ns・あ 79.9 ns・😀 86.6 ns。ログインは debug で 86%、release で 58% が Argon2（386.72 → 20.27 ms）。Argon2 は1回 19.00 MiB。oha: GET /login 12,710 rps・GET /notes 914 rps、ログインは同時 12 で 61.4、48 で 46.6 rps・654 MiB、Semaphore（CPU の数）で 48 が 69.5 rps・218 MiB。全部 403 でも oha は Success 100%、接続できなくても exit=0。
- **RustNote の重要な発見（コードは変えていない）:** ①題が NUL で始まると 500（UNIT 04）、②`cargo audit` で 8 件の弱点（libsql 0.10.0 の Turso 接続の h2 0.3.27・rustls 0.22.4・rustls-webpki 0.102.8。libsql は最新で直らない。replication・sync を外すと部品 367 → 330・テストは通るが弱点は残る）、③守りのヘッダー無し・NoteQuery の pub page・Argon2 の同時数に上限なし・nav の名前・maxlength の数え方。第13巻（完全コード）を書く前に、直すかを決める。
- **メモリ:** WSL の vmmem が 14 GB まで増えてバックグラウンドの処理が止められた。`wsl -u root -- bash -c "sync; echo 1 > /proc/sys/vm/drop_caches"` で空く。WSL と Windows の重い組み立てを同時にしない。cargo-mutants は、比べるテストだけのコピーで動かす（全部のテストを毎回組み立てると遅い）。
- **次:** 第13巻（UNIT 01〜02 の版合わせ → UNIT 03〜10。巻頭の「書き直し中の巻」の記述を更新）→ 第12巻（Docker）→ 第1巻。

## 2026-10-31（続き2）: 第11巻 UNIT 01〜04（main: 75ce81b 取り直し、99473a3 UNIT 03、d67b991 UNIT 04）

素材は `rust11v2/`（parts・out・errs（置き換え表と、わざと間違えたテスト）・rustnote/（tests・examples・Cargo.toml・Cargo.lock）・fuzz_lab（target と corpus を除く）・cap11*.sh）。ビルド先は `target11`。練習用のコピーの `Cargo.toml` に dev-dependency の `scraper =0.29.0`・`proptest =1.12.0` を足した（教材の RustNote には足していない）。

- **取り直し:** UNIT 01〜02 は版の表と出力だけが変わった。同じ DB を使い回すテストは 20 回で UniqueViolation 17・locked 3（前の版で出た already exists は出ず、そう書いた）。
- **UNIT 03（HTTP のテスト）:** Set-Cookie の順番は cookie 0.18.2 の HashSet で毎回変わる（1つ目を読むテストは 10 回中 1 回、別の 10 回では 6 回失敗）。`#[track_caller]` で失敗の行が道具の中（12 行目）から呼んだ行（20 行目）になる。守りのヘッダー（X-Frame-Options・CSP・nosniff・Referrer-Policy・HSTS・Cache-Control）は1つも無い（改善点）。検索を前に一致に壊すと contains と tests/http.rs の 14 個は通り、scraper と tests/repository.rs（203 行目）だけが気づく。
- **UNIT 04（property test・fuzz）:** **本物のバグ:** 題が NUL で始まると 500（Rust は1文字、SQLite の length() は NUL より前だけ数えて 0 → CHECK）。でたらめな HTTP 入力 300 回を 10 回くり返して見つかったのは 3 回。直し方（parse で制御文字を断る）はコピーでだけ試した。UTF-8 として読めないバイトは U+FFFD に置き換えて受け取られる。`NoteQuery` の項目が pub なので `page: 0` で offset が引き算のあふれ（proptest の any::<u32>() 256 回は見逃し、cargo-fuzz はすぐ見つけた）。空白を除き忘れた版は `\PC{0,100}` では見つからず、空白に寄せた作り方で見つかった。手で動かす TestRunner は失敗の種を保存しない（no source file known）。
- **WSL:** Ubuntu 24.04（aarch64）に nightly（rustc 1.101.0-nightly 2026-10-29）と cargo-fuzz 0.13.2 を入れた（rustup は 1.29.2 に自動で上がった）。Git Bash から wsl.exe に /mnt/c のパスを渡すとパス変換されるので、PowerShell から `wsl -d Ubuntu -- bash -l <パス>` で動かす。fuzz のビルド先は `~/rust11-fuzz-target`。
- **RustNote の直す候補（バグ）:** NUL で始まる題の 500。教材のコードを直すかは、第13巻（完全コード）を書く前に決める。
- **次:** 第11巻 UNIT 05（E2E・ブラウザ・アクセシビリティ）→ 06〜10 → 第13巻 → 第12巻 → 第1巻。

## 2026-10-31（続き）: 第8巻を完成（main: UNIT 03〜09 は1つずつ、UNIT 10 と巻の仕上げ）

素材は `rust08v2/`（parts・out・errs（置き換え表）・variants/audit.rs・rustnote/（tests・examples・Cargo.toml・Cargo.lock・toolchain）・cap08_uNN.sh・ba_demo（node_modules と .sqlite を除く）・mfa_lab（target を除く）・browser（site と 2 つのログ））。ビルド先は `target08`、mfa_lab は自分の target。

- **第8巻:** UNIT 01〜02 を 0.11・1.99.1 にそろえ、UNIT 03〜10 を書いた（178 ページ・図 24 枚）。巻頭・巻末のお知らせを外し、旧 `Rust/08_….pdf` を削除。Better Auth は 1.7.14（npm の latest、2026-10-26 公開）をローカルで動かした（アカウント不要）。OIDC・MFA は外のサービスを使わず、mfa_lab で RFC 7636・6238 の例と一致を確かめた。
- **UNIT 10 で確かめた事実:** 守りのテスト `u10_guards`（認可・ログイン・セッション・ログアウト・CSRF を名前付きの assert で）。ログアウトで DB を消さないと「ログアウトのあと、写し取ったクッキーは使えない」で止まり（left 200 / right 303）、CSRF は確かめられないまま終わる。`sec-fetch-site` を消すと POST /notes が 303 になって止まる（ヘッダーが無いとブラウザ以外として通る）。アクセスログには誰が・どこからが無く、OriginPolicy の 403 はどの層にも残らない（OriginLayer は一番外側、topcoat-router 0.11.0 にログを書く所なし）。監査の記録の練習版 `variants/audit.rs`（client_ip・メールを `a…@example.com` に伏せる・@ が無いと何も残さない）。テストでは `RemoteAddr` を request の extension に付けると client_ip が返る。`{input:?}` は E0277（LoginForm に Debug なし）、help の通り derive(Debug) にするとパスワードとメールの欄に打ったパスワードが記録に出る。漏れた token_hash を `Token::new(..).encode()` でクッキーの形にしても 303（サーバーはもう一度 SHA-256 にかけて探す）、password_hash をパスワードにすると 401、`DELETE FROM sessions` で 2 行・2 人とも 303。
- **RustNote の改善点（コードは変えていない）:** `tests/http.rs` に「ログインのたびにトークンが変わる」と「失敗の画面が同じ」が無い。監査の記録が無い（アクセスログだけ）。全角のメールが別のユーザーになる・連番の ID・回数の制限なし（UNIT 03・07・04）。
- **次:** 第11巻（UNIT 01〜02 をそろえてから UNIT 03〜10。UNIT 06 は第8巻 UNIT 10 の守りのテストを広げる）→ 第13巻 → 第12巻（Docker）→ 第1巻。

## 2026-10-31: 第10巻を完成（main: 85ba9ab UNIT 01〜03 … ead9551 UNIT 10）

素材は `rust10v2/`（parts・out・errs（わざと間違える置き換え表）・tests と examples・道具・ブラウザのときのサーバーのログ）。練習用コピーは今の RustNote（0.11・1.99.1）にそろえ、ビルド先は `target10`（`maskx.py` は `targetNN` を `target` に伏せる）。

- **第10巻:** UNIT 01〜02 を取り直し（変わったのは rustc のハッシュ・時間・日時だけ）、UNIT 03〜10 を書いた（188 ページ・図 23 枚）。旧 `Rust/10_….pdf` を削除。
- **道具:** `rust10/edit_src.py <ファイル> <置き換え表.py>`（1か所だけ書き換え、無ければ止まる）と `cap10_uNN.sh`（正しい版→壊した版→元に戻して src が教材と同じかを確かめる）。`build_vol.sh` に「本文・見出しの素の \（¥ に見える）」の検査を足した（第1・2・12巻の3か所も直した）。`clear_refs.py` は「UNIT 05（準備中）で」を「UNIT 05 で」にする（かな・漢字の前に空白）。
- **確かめた事実（抜粋）:** 他人と無いノートの 404 は本文まで同じ 690 バイト、403 にすると ID の数え上げで数が分かる。`/notes/01`・`+1`・`%31` も 200。楽観的ロックの 409 は入力を残し version を新しくする。お知らせがタイトルだけなので本文だけの変更は上書きで消える（改善点）。切り替えを読んでから書くと 20 回中 20 回で1回分が消える（version だけ 3）。`done + 1` は CHECK で 500、`!done` は unrecognized token。削除を GET にするとほかのサイトのリンクで消える（OriginPolicy は GET を止めない）。`#[page(DELETE)]` はコンパイルが通りフォームの POST が 405。LIKE は escape_like と ESCAPE の2つで1組、キーワードを SQL に直接入れると `' OR 1=1) OR (title LIKE '` で他人のノートが見える。ページ送りは OFFSET が見ている間の増減で重なり・抜け、cursor はずれない。20 万件の最後のページは OFFSET 約 35 ms・cursor 約 0.1 ms。範囲の外のページで「まだノートがありません」（改善点）。お知らせを消し忘れると出続け、取り出し忘れると1画面遅れて出る。
- **ブラウザ（claude-in-chrome）:** 127.0.0.1:3000 の RustNote にテスト用の alice でログイン（`__Host-session` の Secure も 127.0.0.1 で通る）。iframe の幅 320・375・768 で測ると、長い英数字の題の詳細が 320・375 ではみ出し（h1 301→500px）、一覧も 320 ではみ出す。`h1, .note-list a { overflow-wrap: anywhere; }` で直る（改善点）。Tab は押せる物だけを HTML の順で、枠は auto。Enter と Space でボタンが押せる。確かめの途中で、Enter で「削除」が押されてテスト用のノートが1件消えた（本文の問1にそのまま使った）。スクリーンショットはときどき時間切れになる（JS の結果で記録した）。
- **次:** 第8巻（UNIT 01〜02 の版と出力をそろえてから UNIT 03〜10）→ 第11巻 → 第13巻 → 第12巻（Docker）→ 第1巻。

## 2026-10-30（続き2）: 第9巻を完成（main: 1cae405 以降 UNIT 05〜09、b835b77 UNIT 10）

素材は `rust09v2/`（parts・out・book/rustnote のコピー（target と .db を除く）・errs10/health_naive.rs・道具）。

- **第9巻:** UNIT 05〜10 を 0.11・1.99.1 で書いた（204 ページ・図 26 枚）。巻頭・巻末のお知らせを外し、旧 `Rust/09_….pdf` を削除。
- **UNIT 10（health・初回起動・診断）で確かめた事実:** `/health` は届く DB で `200 {"status":"ok"}`、`http://127.0.0.1:9` で `503 {"status":"unavailable"}`＋`[error] … os error 10061`（`Db::open` はリモートでは接続しないので開くのは成功）。拒否と分かるまで約2秒（テスト 2.08 s、DB に触らない版は 0.03 s）。DB に触らない `/health` は届かない DB でも 200（`assert_eq!` で left 200 / right 503）。`/healthz` は 404 で、アクセスログに `GET /healthz -> 404` が出る（`/health` ちょうどだけ書かない。/health を5回送っても 0 行）。DB に届かないとき `GET /login` 200・`GET /notes` 303（クッキーが無ければ DB を見ない。偽のクッキーも DB を見ずに 303）・`POST /login` は約 2.04 s 後に 500。診断 `examples/u10_doctor.rs`（設定・束・開く・届く・マイグレーション数・TcpListener::bind）: 新しい DB で 0/3（ファイルは診断で作られる）、束なし＋無いフォルダでは main は束のエラーだけ・診断は2つとも ✗、動いている横では 10048 の ✗、PORT=3001 なら ✓。`AssetBundle::load()` は exe の隣の assets を見るので、examples からは `load_dir(<exe>/../../assets)`。`topcoat asset bundle`（--bin なし）でも RustNote の束を作れた。
- **Fly.io（docs.fly.io で確認、アカウントは使っていない）:** http_service.checks は 2xx で合格、301/302 は追わない。不合格のマシンはプロキシが unhealthy にして、合格するまで送らない。再起動・停止はしない。`grace_period` は起動から最初の確かめまで。
- **直したこと:** `tests/u09_a11y.rs` の doc コメントが clippy の doc_lazy_continuation に当たっていた（`//!` の空行を足した）。`clear_refs.py` が「UNIT 05（準備中）で」を「UNIT 05で」にしていたのを、かな・漢字が続くときは「UNIT 05 で」にするよう直し、既存の 22 か所を `fix_unit_space.py` でそろえた（第6・9・10・12巻の PDF も再生成）。
- **次:** 第10巻（UNIT 01〜02 の版の表記と出力を合わせてから UNIT 03〜10）→ 第8巻 → 第11巻 → 第13巻 → 第12巻（Docker）→ 第1巻。計画は `ref/tools/PLAN_ALL.md`。

## 2026-10-30（続き）: 版を最新に上げ、第6巻を完成（main: fbe63e4 RustNote、7764a49〜4837802 第6巻）

ユーザーの指示「全て完了まで進めて / バージョンはあげる / 全て最新に合わせる」。サブエージェントなし。計画は `ref/tools/PLAN_ALL.md`。

- **版:** Rust 1.99.1（toolchain をインストール。既定の stable は 1.97.1 のまま）、topcoat =0.11.0（rust-version 1.99、topcoat-cli 0.11.0 を ~/.cargo/bin に入れた）、tokio =1.54.1、cargo update で間接の依存も更新（generic-array だけ crypto-common が =0.14.7 に固定）。CI は ubuntu-26.04（runner-images にある）、checkout@v7、rust-cache@v2（v2.9.3）。Dockerfile は rust:1.99.1-bookworm のまま（trixie へは第12巻で docker build して確かめてから）。Docker Desktop はプロセスはあるが `docker version` が応答しない。
- **RustNote:** コードの変更なしで 0.11 に上がった（41 テスト・clippy・fmt・asset bundle・起動して GET /・/health・/login）。
- **第6巻:** ファイル名を `06_Web基礎_HTTPとTopcoat_0_11完全ガイド` に変え、旧 0_10 の md/pdf と旧 0_5 の PDF を削除。UNIT 01〜02 は 0.11 で取り直し（cargo add の表示が「22 deactivated features」にまとまる、初回ビルド 164 個・約 22 秒）。UNIT 03〜10 を新しく書いた（185 ページ・図 21 枚）。
- **確かめた事実（0.11）:** module_router の `_`→`-`、末尾 `/` は 308、同じ URL の page 2つは起動時 panic `duplicate route registered`、`#[page]` 忘れの href! は E0573、`(slot)` 忘れは unused variable の警告だけ。モジュールの #[layer] は当たらない URL を通らない（RustNote は not_found!() で通している）。query の空の値は None、`missing field` の 400、`%2F` は1区切りの中で `/`。view! は自作の型を入れると E0277（NodeViewParts）、本文は < > & を、属性の値は " & をエスケープ（topcoat-view の escape.rs の表どおり）、属性名に `=` は panic で 500。Json の本文の上限は既定 2 MiB（413）。app_context の登録し忘れ・二重登録は panic。memoize で5回の呼び出しが1回に（外すと6回・約 0.25 秒）。asset bundle は `target/<profile>/assets` に作り、同じ web_lab の別の --bin で bundle すると置き換わる（500）。topcoat dev は起動時にソースを同じ内容で書き直す（mtime だけ変わる）。runtime: ブラウザ（claude-in-chrome）で +1・説明は通信 0 件、2倍は POST /api/double。procedure の引数は `[{"t":"usize","bits":64,"v":"21"}]`、usize::MAX で `value * 2` が overflow panic（500）。ページの確かめは procedure を守らない（curl でメモが消える）。session: 1文字変えたトークン・ログアウト後のトークンは通らない、`redirect` は 307、`.cookies()` 忘れは CookieJarCell の panic。
- **注意:** Git Bash の `curl.exe` は Git 付属の curl 8.18（brotli あり）。本の出力は `/c/Windows/System32/curl.exe`（8.21.0）で取る（`--compressed` の結果が gzip と br で変わる）。
- **道具（ref/tools/）:** `finish_unit.py <巻> <N>`（巻頭の表・お知らせ・巻末のチェックリストと用語・assemble の範囲。meta は `parts/unitNN.meta.md`）、`finish_volume.py <巻>`（お知らせを外す）、`clear_refs.py`（完成した UNIT への「（準備中）」を全巻から外す）、`build_vol.sh <巻> <N> <md>`（fill→assemble→check→PDF→検査）。

## 2026-10-30: Rust 08・10・11 巻に UNIT 02 を main に反映（08: 2af635b、10: 8194c47、11: b2796c6）

「完成から最も遠い3つ」として、UNIT 01 だけだった巻のうち、08・10・11 を選んだ（12 も UNIT 01 だけだが、UNIT 02 は Docker が要り、デーモンを起動する副作用を避けて見送り）。統括が1人で執筆（サブエージェントなし）。素材は `rust08v2/`・`rust10v2/`・`rust11v2/`（追記）。

- **Rust 08**（1,795 行・図7枚・50 ページ）: UNIT 02 = パスワード・Argon2id・salt。`examples/u02_hash.rs`・`u02_speed.rs`、`tests/u02_stored.rs`・`u02_q2.rs`・`u02_q3.rs`、`errs/e02_old_api.rs`。確かめた事実: 同じパスワードでも PHC 文字列は毎回違い（salt 22 文字・ハッシュ 43 文字・全体 97 文字）、照合はどちらも true。release で DefaultHasher 8,721,779 回／秒、Argon2id 39 回／秒（約 22 万倍、Windows ARM64）。保存ハッシュを `broken` にすると正しいパスワードでも 500（ログ `invalid encoding`）。argon2 0.5 の書き方は E0432（SaltString・rand_core は feature の外）と E0061（hash_password は引数1つ）。失敗のログインは約 380〜410 ms でそろうが、**未登録メールの1回目だけ 804 ms**（DUMMY_HASH が LazyLock のため。改善点として本文に記載、コードは変えていない）。400 の出し直しでパスワードは返さない。
- **Rust 10**（1,907 行・図7枚・51 ページ）: UNIT 02 = 作成のフォーム・入力の検査・PRG。`tests/u02_create.rs`・`u02_reload.rs`・`u02_err_name.rs`・`u02_q1〜q3.rs`、`errs/e02_string.rs`。確かめた事実: 成功は 303・`Location: /notes/1`・`flash=note_created; HttpOnly; SameSite=Lax; Path=/; Max-Age=300`、移動先の GET でお知らせを出して `Max-Age=0` と1年前の Expires で消す。失敗は 400・Location なし・`<details … open="">`・入力が残る・両方のエラーが出る・保存 0 件。再読み込み3回で id は [1] のまま、同じ POST の再送で [1, 2]（二重送信は PRG では防げないと明記）。name が subject／body の欄なしは 400「送信された内容を読み取れませんでした」（ログはアクセスログだけ）、余分な欄は無視されて 303。`\r\n` は `\n` にそろえて保存（10 文字 → 8 文字）。エスケープは場所ごとに違い、属性の値では `"` → `&quot;`（`<` `>` はそのまま）、本文では `<` `>` → `&lt;` `&gt;`（`"` はそのまま）。
- **Rust 11**（1,932 行・図7枚・52 ページ）: UNIT 02 = リポジトリの結合テスト。`tests/u02_contract.rs`・`u02_owner_delete.rs`・`u02_shared.rs`・`u02_q1〜q3.rs`。確かめた事実（コピーの `src/repository/notes.rs` を一時的に壊して確認し、元に戻して `src/` 同一を確認）: `ESCAPE '\'` を消すと `like_special_characters_are_escaped`（ユニット）は通り、`search_filters_by_keyword_and_paginates` だけが 198 行目で `left: 0 right: 1`。DELETE の `AND user_id = ?2` を消すと `notes_are_isolated_per_owner` が 90 行目で失敗、**params が1つ余っても libsql はエラーにしない**。1つの DB ファイルを使い回すテストは、消してから8回で UNIQUE 6・`database is locked` 1・`table notes already exists` 1（毎回1つは通る）、消さずに2回目は両方 UNIQUE。LIKE は ASCII だけ大文字小文字を区別しない（`ｒｕｓｔ`・`äpfel` は 0 件）。セッションは期限ちょうどで見つからず、delete_expired で消える。users を消すと notes・sessions も CASCADE で消える。
- 図の `\` が ¥ に見える件は、SVG の tspan と説明文の span に Consolas を当てて直した（第11巻 図11-7）。heredoc で Python を書くと `\\` が `\` になり、`re.sub` の置換文字列でも `\r` が CR になるので、バックスラッシュを含む置き換えは Write で書いた .py か chr(92) で行うこと。
- 一時フォルダの `rustnote-shared-test.db*` は消した。起動したサーバーは無し。
- 決定待ち（変わらず）: Topcoat 0.11.0 へ上げるか、Build Tools 2022 と「2026」の表記、RustNote の `.gitattributes`。

## 2026-10-29: Rust 11 巻を巻頭と UNIT 01 まで、Rust 13 巻と Rust 06 巻に UNIT 02 を main に反映（11: 10ce685、13: 6b94b08、06: 39938d5）

「完成から最も遠い3つ」として、本文が無かった 11 と、UNIT 01 だけだった巻のうち、巻末の分だけ残りが多い 13 と、いちばん短かった 06 を選んだ。統括が1人で執筆（サブエージェントなし）。素材は `rust11v2/`・`rust13/`（追記）・`rust06v2/`（追記）。

- **重要: Topcoat 0.11.0 が出ている。** `cargo add topcoat@=0.10.0` の表示に `Adding topcoat v0.10.0 (available: v0.11.0)`。全巻は `=0.10.0` に固定しているのでそのまま動く。第6巻の本文と巻頭に「この本は 0.10.0 に固定」と書いた。上げるかどうかはユーザーの判断（上げるなら RustNote・全巻のコードと出力の取り直しが要る）。
- **Rust 11**（992 行・図4枚・28 ページ）: UNIT 01 = ユニットテスト・境界値・表の形のテスト。RustNote のコピー `rust11/rustnote/`（`src/` 同一を確認）に `tests/u01_title_table.rs`（10 行の表＋len() で数える間違えた版）、`examples/u01_counts.rs`、`tests/u01_q1〜q3.rs` を足した。確かめた事実: 間違えた版は 80・81・空白込み80 の3行でだけ NG（短い日本語や家族の絵文字の行は ok のまま）。家族の絵文字は 5 文字・18 バイト・UTF-16 で 8、濁点を分けた「が」は 2 文字、ゼロ幅スペースだけでも NoteTitle に合格。`#[test] async fn` は `error: async functions cannot be used for tests`。`assert_eq!` の失敗は `left: false / right: true`。Email の 254/255 バイトの境界、NoteId の `9223372036854775808` は「数字で指定してください」（大きすぎる数も NotANumber 扱い）・`"01"` は 1・`" 1"` は不合格、Password は trim せず日本語 8 文字も合格。HTML の maxlength は UTF-16 単位という記述は HTML の決まりとして書き、実際のブラウザは UNIT 05 で確かめると明記。
- **Rust 13**（2,018 行・図9枚・53 ページ）: UNIT 02 = migrations と db.rs。`examples/u02_migrate.rs`・`u02_state.rs`・`u02_pragmas.rs`・`u02_q2.rs`・`u02_q3.rs`。確かめた事実: migrate 2回目は `[]`。libsql の素の接続でも journal_mode=wal（ファイルに記録）・**foreign_keys=1**、busy_timeout は 0（RustNote は 5000）。0003 の最後の索引の列名を打ち間違えると sessions と1つ目の索引も rollback で残らず、直すと 0003 だけ流れる。適用済みの 0002 を書き換えると古い DB は変わらず新しい DB だけ pinned が増える。0004_note_pinned を正しく足すと 0004 だけ流れ、41 件のテストも通る（コピーの src を一時的に変更して確認し、元に戻した）。一覧の SQL は `SEARCH notes USING INDEX idx_notes_user_updated (user_id=?)` で並べ替え不要、持ち主の条件が無いと `SCAN notes` と `USE TEMP B-TREE FOR ORDER BY`。無いフォルダは `Unable to open connection to local database …: 14`。
- **Rust 06**（1,626 行・図6枚・44 ページ）: UNIT 02 = Topcoat 0.10 の最小のアプリ。`rust06/web_lab/`（`src/main.rs`・`src/bin/u02_serve.rs`・`u02_q1.rs`）。確かめた事実: 新しい target で 164 個・約 31 秒。`topcoat::start` は何も表示しない。GET / は `vary: Accept` と `vary: accept-encoding` 付き、無い URL は `not found`（text/plain の 404）、POST / は 405・`allow: GET, HEAD`、HEAD は本文なし、`--compressed` で `content-encoding: gzip`・`transfer-encoding: chunked`。`.await` 忘れは E0599（impl Future に expect が無い）、RouterBuilderDiscoverExt の use 忘れは E0599（help に discover_fonts も出る）、PORT=abc は `InvalidInput "PORT must be a valid port number: invalid digit found in string"`、bin が2つだと `cargo run could not determine which binary to run`。`TcpListener::bind` してから表示して `topcoat::serve` に渡すと、2つ目は `AddrInUse` で表示なしに止まる（RustNote の「起動しました」が先に出る改善点の直し方として本文に記載）。
- 起動したサーバーは、すべて PID 指定で止め、残っていないことを確認した。

## 2026-10-28: Rust 08・10・06 巻を巻頭と UNIT 01 まで main に反映（08: 05a1ae0、10: 92f3a6a、06: ddf8002）

「完成から最も遠い3つ」として、本文が無かった4巻（06・08・10・11）のうち、素材の少ない 08・10 と、06 を選んだ（11 は UNIT 01 のテストまで素材があるので後回し）。統括が1人で執筆（サブエージェントなし）。どの巻も旧 PDF は残してある。素材は `rust08v2/`・`rust10v2/`・`rust06v2/` に保存（以前の `rust08/`・`rust10/`・`rust06/` は古い素材のまま残してある）。

- 共通の道具: スクラッチの `maskx.py`（どの rustNN でも使える伏せ字。`target-shared` → `target`）と `capx.sh`（`VOL=rust08 capx.sh 名前 コマンド`）。08 と 10 は RustNote のコピーを別々に持ち、ビルド先 `target-shared` を共有した。各巻の `fill.py`・`assemble.py` は rust12 のものの書き換え。**bash の heredoc で Python や Rust を書くと `\\` が化けることがある**ので、バックスラッシュを含むファイルは Write ツールで書いた。
- **Rust 08**（1,100 行・図4枚・31 ページ）: UNIT 01 = 認証・セッション・認可と脅威モデル。確かめた事実: 未ログインの /notes は 303 → /login、他人のノートは 404。クッキーは `__Host-session` の 46 文字・`HttpOnly; SameSite=Lax; Secure; Path=/; Max-Age=2592000`、`sessions.token_hash` は 32 バイトでトークンそのものは 0 件。Origin `https://evil.example` や `Sec-Fetch-Site: cross-site` の POST は 403・本文 `forbidden`・アクセスログに出ない（OriginPolicy が層より前で動く）、ヘッダーが無い POST と同じ Origin は通る。`notes.find(NoteId)` は E0308（UserId の場所に NoteId）＋E0061。持ち主の条件の無い SQL で bob に alice のノートが見える。10 回まちがえても 10 回とも 401・11 回目に成功（回数の制限が無いことを脅威モデルの表に「守りが無い」と明記）。ログアウトで sessions の行が 1→0、`Max-Age=0` と1年前の Expires。
- **Rust 10**（978 行・図4枚・26 ページ）: UNIT 01 = 一覧・空のとき・持ち主の条件。確かめた事実: 0 件で「まだノートがありません。最初の1件を書いてみましょう。」、同じ秒の3件は 3,2,1（`updated_at DESC, id DESC`）、3件目を1日前にすると 2,1,3、bob は空で「1 / 1 ページ（全0件）」、タイトルの `<b>…</b> & <script>` はエスケープされる。**`ORDER BY updated_at DESC` だけでも、今の SQLite は同点を id の大きい順に返した**（索引を足しても同じ）。本文では「試して同じでも、SQL の決まりでは保証されないので id を足す」と正直に書いた。`for note in page.notes`（clone を外す）で E0382、help どおり `&page.notes` にすると E0507×2＋E0505（view! が値を受け取るため）。完了の切り替えで updated_at が今になり上に移る。件数の SQL にだけ持ち主の条件を忘れると bob に「全3件」。
- **Rust 06**（949 行・図4枚・25 ページ）: ファイル名を設計書どおり `06_Web基礎_HTTPとTopcoat_0_10完全ガイド.md` にした（旧 PDF は `…_0_5完全ガイド.pdf` のまま残す）。UNIT 01 = HTTP の要求・応答・状態コード。外のクレートなしの `http_lab`（`u01_raw`・`u01_status`・`u01_len_bug`・`u01_q1`）を 1.99.0 で作り直した。curl は Windows の `curl.exe` 8.21.0。Content-Length に文字数（12）を入れると本文が「こんにち」で切れる。止めたあとの接続は `curl: (7) Failed to connect … after 2029 ms`。`POST /health` は `_` に落ちて 404（本当は 405＋Allow が正しい、と解答で説明）。PowerShell 5.1 の文字化けには `[Console]::OutputEncoding = [System.Text.Encoding]::UTF8`。
- 起動したサーバー（rustnote.exe・u01_*.exe）は、すべて PID 指定で止め、残っていないことを確認した。

## 2026-10-27: 方針決定 — Fly.io・Turso のアカウントで実際には公開しない（ユーザーの決定）

ユーザーの言葉: 「Fly.io と Turso のアカウントで実際に公開しなくていい」。第12巻の巻頭のお知らせをこの方針に直して main に反映した（PDF 再生成）。

- 対象: 第12巻の UNIT 04〜10、第1巻の後半（Turso CLI・flyctl）、第7巻を直すことがあればその remote の部分も。`fly launch`・`fly deploy`・`fly secrets`・`turso auth login`・`turso db create` などのアカウントが要る操作は実行しない。
- 書き方: アカウントが要る操作は、コマンドと、公式ドキュメントで確かめた手順の説明だけを載せる。**その実行結果は載せない（作った出力を貼らない）**。実行結果を載せるのは、手元で動かせるもの（release ビルド、Docker でのビルドと起動、ローカルの libSQL ファイル、CLI の `--version`・`--help`）だけ。
- 上の「Rust 12 巻」の節にある「UNIT 04 以降は実アカウントでの実行が要る（ユーザーの許可待ち）」は、この決定で解消した。

## 2026-10-27: Rust 12 巻（Fly.io 公開）を巻頭と UNIT 01 まで main に反映（279b537）

「完成から最も遠いファイル」として、本文も素材も無かった第12巻に着手。統括が1人で執筆（サブエージェントなし）。本文 973 行・図5枚・25 ページ。旧 PDF（`Rust/12_….pdf`）は残してある。

- 構成（旧 PDF の学習地図を Topcoat 0.10 に合わせて組み直し）: 01 release ビルドと再現性 / 02 multi-stage Dockerfile / 03 アセットの束を本番用に / 04 fly launch・fly.toml・ポート / 05 secrets / 06 health check と起動の順番 / 07 migration の順番と release command / 08 deploy・ログ・動作確認・独自ドメイン / 09 台数・地域・費用・Turso / 10 rollback・backup・復旧。**UNIT 04 以降は Fly.io・Turso の実アカウントでの実行が要る（ユーザーの許可待ち）**と本文のお知らせに書いた。
- 下書き: `rust12/parts/`（intro・unit01.src.md・unit01.md・outro）。目印の埋め込みは `rust12/fill.py unit01`（rust13 と同じ形式。出力は BOM 付きでも読める）、連結は `assemble.py`（`range(1, 2)`）。練習用コピーはスクラッチの `rust12/rustnote/`（`src/` は本物と同一を確認）。足した例は `examples/u01_hash_speed.rs`・`u01_exe_report.rs`・`u01_overflow.rs`・`u01_q2.rs`（`rust12/examples/` に保存）。topcoat-cli 0.10.0 はスクラッチの `rust12/cli/` に `--root` で入れた（PC 全体には入れていない）。
- 確かめた事実（1.99.0、Windows ARM64）: release ビルド 1m25s・debug 1m02s（どちらも 289 個）。rustnote.exe は debug 21,698,048 B・release 11,820,032 B。Argon2id のハッシュ化は debug 約 380〜470 ms・release 約 20 ms。release の実行ファイルは `topcoat asset bundle --release` 無しだと `no asset bundle at <exe の隣>\assets` で DB を開く前に止まる（`--release` 無しの bundle は target\debug\assets に作られる）。2回目の起動はポート使用中で os error 10048 だが、その前に「RustNote を起動しました」と出る（main.rs が `topcoat::start` の前に表示するため。改善候補として本文に記載）。同じソースの release を2回作ると 24 バイト違う（PE の時刻 2B、デバッグ目次の時刻 2B×3、RSDS の GUID 16B）。`RUSTFLAGS=-C link-arg=/Brepro` で 0 バイト差。実行ファイルにユーザーフォルダの場所が 1,820 か所、`--remap-path-prefix=$USERPROFILE=~` で 1 か所（topcoat-asset の `asset!` が `CARGO_MANIFEST_DIR` を埋め込む）。オーバーフローは debug で panic・release で折り返し・`CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true` で panic。u32 の offset は `?page=214748366` で release だと 4（RustNote は u64 で計算しているので安全）。
- 本文で触れた RustNote の改善候補（コード未変更）: アセットが無いときのエラー文に「release のときは `--release`」が無い。ポートを押さえる前に「起動しました」と表示する（既存の候補 4 と同じ）。
- 作業中の出来事: `powershell -ExecutionPolicy Bypass` での記録スクリプトの実行は自動判定で拒否された（PowerShell ツールで同じ行を直接実行して取り直した）。起動した rustnote.exe はすべて PID 指定で止め、残っていないことを確認した。

## 2026-10-26: Rust 13 巻（完全コード）を巻頭と UNIT 01 まで main に反映（cc5094c）

「完成から最も遠いファイル」として、未着手だった第13巻に着手。統括が1人で執筆（サブエージェントなし）。本文 1,107 行・図6枚・30 ページ。旧 PDF（`Rust/13_….pdf`）は残してある。

- 構成（旧 PDF の学習地図を、今の RustNote の 42 ファイルに割り当て直した）: 01 設定ファイル（Cargo.toml・Cargo.lock・rust-toolchain.toml・rustfmt.toml・.gitignore・.env.example）/ 02 migrations・db.rs / 03 config・main・lib / 04 domain・error / 05 repository / 06 auth・flash・state / 07 routes.rs・views・signup・login・logout・health / 08 notes の CRUD / 09 style.css / 10 tests・Dockerfile・.dockerignore・fly.toml・ci.yml・README。巻末に総合演習・解答・全巻の用語集・索引（準備中）。
- 下書き: `rust13/parts/`（intro・unit01.src.md・unit01.md・outro）。`unit01.src.md` の目印（`@@FILE:`・`@@OUT:`・`@@OUTHEAD:名前:N@@`・`@@OUTTAIL:`・`@@OUTGREP:名前:正規表現@@`・`@@LOCK:パッケージ名@@`）を `rust13/fill.py unit01` で埋め、`assemble.py`（`range(1, 2)`）で連結。実出力は `rust13/out/`（`cap.sh` で取得・`mask13.py` で伏せ字）。練習用コピーはスクラッチの `rust13/rustnote/`（Git 管理ファイルだけをコピー。全ファイルが本物と同一を確認）、ビルド先は `rust13/target`。
- 確かめた事実: `cargo build --locked` で Compiling は 289 個、Cargo.lock の `[[package]]` は 361 個。テストは 41 件（unit 19・http 14・repository 8）。`cargo update -p tokio --dry-run` は `=1.53.3` だと 0 個、`1.53.3` だと `tokio v1.53.3 -> v1.54.1`（MSRV 1.98 を考慮、26 個が latest より古いまま）。**tokio の features は依存全体で足し算され、libsql_replication が `full`、topcoat が `macros`・`net`・`signal` を有効にするので、RustNote の Cargo.toml から `macros` を消してもビルドが通る**（単体の feat_demo では E0433「cannot find `main` in `tokio`」と E0752）。thiserror は 1.0.69（libsql）と 2.0.21（RustNote）が共存。ring ← rustls ← hyper-rustls ← libsql。`cargo +stable check` は `rustc 1.97.1 is not supported ... requires rustc 1.98` が 35 行。feature の打ち間違いは `help: there is a feature macros with a similar name`。checksum を1文字変えると `checksum for dotenvy v0.15.7 changed between lock files`。
- **他巻との食い違い（要修正候補）**: 第9巻の巻頭の版の表「tokio 1.53.3（`rt-multi-thread` と `macros` の機能だけ）」は、RustNote の Cargo.toml が指定する機能のことで、実際に有効な機能はもっと多い。「Cargo.toml で指定している機能は…」と言い直すとよい。
- 作業中の出来事: `cargo +1.97.1 check` を実行したら、rustup が `1.97.1-aarch64-pc-windows-msvc` を自動でインストールした（既定の stable も 1.97.1 だが、名前の違う別のツールチェーン扱い）。すぐ `rustup toolchain uninstall` で消し、`cargo +stable` で取り直した。ツールチェーンの一覧は元どおり（stable・1.90.0・1.99.0）。本文の注意書きにも反映した。

## 2026-10-25: Rust 09 巻に UNIT 04（プロジェクト構成と依存の向き）を追加して main に反映（7ddd583）

統括が1人で執筆（サブエージェントなし）。本文 4,655 行・図14枚（UNIT 04 で5枚追加）・126 ページ。旧 PDF（`Rust/09_….pdf`）は残してある。

- 下書き: `rust09/parts/unit04.src.md`（`@@FILE:…@@`・`@@OUT:…@@` の目印入り）を `rust09/fill04.py` で埋めて `parts/unit04.md` を作り、`assemble.py`（`range(1, 5)`）で連結する。intro・outro は UNIT 01〜04 収録に更新、outro に UNIT 04 のチェックリストと用語表を追加。
- 練習用コピー（`rust09/book/rustnote/`、`src/` は本物と同一を確認）に足したもの: `examples/u04_deps.rs`、`tests/architecture.rs`（R-1〜R-6）・`swap.rs`・`u04_q3.rs`・`u04_panic.rs`。テスト計 62 件が 1.99.0 で通り、fmt・clippy（all-targets、-D warnings）も通る。実出力は `rust09/out4/`（`bcap.sh` の保存先を out2 から out4 に変更）。
- 問1（ノートの複製）の解答は、別のコピー `rust09/dup_probe/`（変更したファイルだけ保存）で `src/routes/notes/id/duplicate.rs` を足して確かめた。クレートの循環は `rust09/cycle_ws/`（app ⇄ views）で取り直した。
- 確かめた事実: RustNote のモジュールの依存は `domain` が何も使わない（thiserror のみ）、libsql は db・repository・error だけ、相互依存は `auth ⇄ routes` と `routes ⇄ views`（どちらも `href!`）の2組。`routes/health.rs` は `app_state(cx).db.ping()` で DB を直接使う（文字の検査では見えない）。examples から `crate::` は E0432、`rustnote::repository::notes::…` は E0603（`pub use` の入口なら通り、`type_name` は `rustnote::repository::notes::LibsqlNoteRepository`）。trait のメソッド不足は E0046。**ページの処理の中の panic は Topcoat 0.10 の `Router::handle_with` が `catch_unwind` で受け止め、本文 `internal server error` の 500 を返す**（RustNote のエラー画面もアクセスログの行も出ず、テストは ok のまま）。
- 本文で触れた RustNote の改善候補（コード未変更）: なし（新規の指摘はなし。panic 時にアクセスログが残らない点は Topcoat 側の挙動として記載）。
- 作業中の出来事: スクラッチパッドの `main-wt`（main の worktree）が中身ごと消えて「prunable」になっていた。残っていたファイルが origin/main と同一なのを確かめてから、同じ場所に `git worktree add` で作り直した。

## 2026-10-24: Rust 09 巻・Rust 01 巻を UNIT 01〜03 まで main に反映（09: fb8b0fd、01: bfa5759）

サブエージェント2つに並行して書かせ、統括が確認（check_md・出力の照合・図の目視・RustNote 未変更の確認）してから反映した。どちらも旧 PDF は残してある。下書きと素材は `rust09/`・`rust01/` に保存（d22b785）。

- **Rust 09**（3,251行・図9枚・90ページ）: `rust09/parts/`（intro・unit01〜03・outro）、連結は `rust09/assemble.py`（`range(1, 4)`）。練習用の RustNote のコピーは `rust09/book/rustnote/`（`src/` は本物と同じ。足したのは `tests/` の acceptance・screens・flow・states・mistakes と `examples/u03_*`。53件のテストが通る）。実出力は `rust09/out2/`（28件、すべて 1.99.0 で取り直し）。
  - 確かめた事実: POST だけのルートへの GET は 405（`allow: POST`、本文 0 バイト、アクセスログ無し）。flash を消す Set-Cookie は1年前の Expires。SQLite の `trim` は半角空白だけ（全角空白1つの `length(trim(…))` は 1）。UNIQUE は大文字小文字を区別。ON DELETE なしの外部キーでは子が残る親を消すと `FOREIGN KEY constraint failed`。
  - 本文に書いた RustNote の改善点（コード未変更）: 検索欄にラベルが無い、ゼロ幅スペースだけのタイトルが型と CHECK を通る、空白だけのタイトルは DB の CHECK では止まらない（アプリは止める）、CSS に `@media` が無い、ログインの試行回数の制限が無い（スコープ外と記載）。第0巻の「空白だけのタイトルは第9巻で決める」には UNIT 01 の AC-03・UNIT 03 で「認めない」と答えた。
  - UNIT 04 以降の準備の形: 教材の `Rust/rustnote` を作業フォルダにコピーし、`tests/`・`examples/` に足す（`src/` は変えない）。
- **Rust 01**（2,548行・図11枚・68ページ）: `rust01/parts/`、連結は `rust01/assemble.py`（`range(1, 4)`）、実出力は `rust01/out2/`。巻末のお知らせで、UNIT 04 が入るまで Rust は公式サイトで入れるよう案内した。
  - 確かめた事実: `CARGO_TARGET_DIR` で深くすると、実行ファイルのパスが 259 文字までは成功・260 文字以上で LNK1104（`LongPathsEnabled=1` でも失敗）。Git Bash の `usr/bin/bash.exe` は ARM64 版の Git でも x64 で、そこから起動した PowerShell は `PROCESSOR_ARCHITECTURE=AMD64`。CRLF のスクリプトは WSL（Ubuntu 24.04.1）で `$'\r'` のエラーになり `sed -i 's/\r$//'` で直る。rust-analyzer は cargo が無いと `Failed to load the project … program not found`。PowerShell は 5.1.26100.9711。
  - **要確認（UNIT 05 を書く前に）**: この PC の Build Tools は Visual Studio Build Tools **2022**（17.14、MSVC 14.44.35207）。下の「担当が見つけた事実」の「Build Tools for Visual Studio 2026」と食い違う。
  - **統括の判断待ち**: RustNote に `.gitattributes` と `.vscode` が無い（`git ls-files --eol Rust/rustnote` の attr が空。`core.autocrlf=true` の PC では CRLF で取り出される）。本文は「自分のリポジトリでは足す」とだけ書いた。
  - 他巻との食い違い: 第2〜5・7巻の前提知識は「第1巻で rustup・cargo・fmt を準備済み」だが、第1巻の UNIT 04・06 はまだ準備中。巻頭の表の Turso CLI v1.0.34・flyctl v0.4.57 は保存済みの出力から（UNIT 07〜10 で確認し直す）。この PC には 1.97.1（既定）・1.99.0 のほかに 1.90.0 のツールチェーンもある（今回の作業より前から）。

## 2026-10-24: Rust 05 巻に UNIT 10 を追加して巻を完成（main b6d7e7e・旧 PDF 削除 b9e7a87）

- 本文は UNIT 01〜10（10,384行・図36枚・270ページ）。巻頭・巻末の「準備中」とお知らせを外し、チェックリストを学習地図の3段（async の土台／渡す・守る・止める／外とつなぎ・見守る）に分けた。**旧 `Rust/05_….pdf` は削除済み。Rust 05 巻はこれで完成。**
- UNIT 10 は素材なしから作った。`async_lab/src/job_queue.rs`（libsql の jobs 表。claim は `UPDATE … WHERE id = (SELECT … LIMIT 1) RETURNING`、fail は CASE で queued/dead）と `u10_*`・`errs/e10_*`。実出力は `rust05/out10/`。`cargo add libsql@=0.10.0` の実出力は `probe/cargo_add_libsql.txt`（Locking 114 packages）。lab に libsql を足した後の全 bin のビルドは約 65 秒。
- 確かめた事実（2026-10-24）: SELECT と UPDATE を分けた claim（間に 50ms）は2つの worker が同じ #1 を取る。1文の claim なら2 worker・10件で重複 0。lease 300ms で落ちた worker の仕事を 0.4 秒後に別 worker が attempts=2 で取り直す。CHECK 違反は `SqliteFailure(275, "CHECK constraint failed: …")`。**Windows で名前に update を含む exe（e10_select_then_update.exe）は `os error 740`（要求された操作には管理者特権が必要です）で起動できない**。名前を変えると同じ中身で動いた。
- publish.sh の注意: 旧 PDF を消すときは、先に作業ツリーからファイルを `rm` してから publish.sh にパスを渡す（ファイルが残っていると削除にならず、何も変わらない）。

## 2026-10-24: Rust 05 巻に UNIT 09 を追加して main に反映（64e4fb2）

- 本文は UNIT 01〜09（9,121行・図32枚・235ページ）。「準備中」は UNIT 10 だけ。`assemble.py` は `range(1, 10)`。実出力は `rust05/out09/`（NO_COLOR=1 で取得）。追加した例: `u09_access_log`（request_id のスパン・2リクエスト同時・JSON）・`u09_q1`（with_env_filter）・`u09_q2`（skip(token)）。`u09_metrics` は clippy の manual_is_multiple_of に合わせ `!n.is_multiple_of(5)` に変更。
- 確かめた事実: tracing-subscriber 0.3.23 の `fmt().init()`（builder）は RUST_LOG を読まない（`fmt::init()` 関数は env-filter があれば読む）、既定の書き出し先は stdout、`NO_COLOR` で色なし、2回目の `init` は「a global default trace dispatcher has already been set」で panic（`try_init` は Err）。`#[instrument]` の E0277 で rustc は `#[derive(Debug)]` を勧めるが、それだとパスワードがログに出る（本文で注意）。reqwest 0.13.5 は tracing ではなく log クレートを使う。RustNote のログは eprintln!（access_log・log_if_internal・health）で、tracing への置き換えを改善候補として本文に記載（コード未変更）。

## 2026-10-24: Rust 05 巻に UNIT 08 を追加して main に反映（ef0929e）

- 本文は UNIT 01〜08（8,128行・図29枚・209ページ）。「準備中」は UNIT 09・10。`assemble.py` は `range(1, 9)`。実出力は `rust05/out08/`（ポート番号は `PORT` に置換）。追加した例: `u08_json`（serde で受け取り・missing field）・`u08_q1`（再試行2回）・`u08_q3`（ジッター無しの backoff 表）。練習用サーバー `async_lab/src/mini_server.rs`（148行）は本文に全文を載せた。
- `cargo add reqwest@=0.13.5 --features json`・`serde@=1.0.229 --features derive` の実出力は `probe/cargo_add_reqwest.txt`・`cargo_add_serde.txt`。新しい probe でも aws-lc-sys を含めて約 33 秒でビルドでき、u08_get が同じ結果になった（この PC は LLVM/clang あり）。
- 確かめた事実: reqwest 0.13.5 の `ClientBuilder::timeout` は「Default is no timeout」、`Client` は中で Arc を使う（clone で共有）。Windows で閉じたポートへの接続は毎回約 2 秒後に os error 10061。RustNote は reqwest を使わず、`POST /notes` は毎回 insert（冪等キーなし）して 303 で詳細へ（PRG）。

## 2026-10-24: Rust 05 巻に UNIT 07 を追加して main に反映（5c76831）

- 本文は UNIT 01〜07（6,937行・図26枚・178ページ）。「準備中」は UNIT 08〜10。`assemble.py` は `range(1, 8)`。実出力は `rust05/out07/`。追加した例: `u07_not_cancelled`（timeout で待つのをやめても spawn_blocking は最後まで動き、main の後も待たれる）・`u07_q2`（count_lines_async）・`u07_q3`（Hasher 3）。`cargo add argon2@=0.6.0` の実出力は `probe/cargo_add_argon2.txt`（probe では ctutils 0.4.5、lab は 0.4.3）。
- 確かめた事実: Argon2id 既定（m=19456,t=2,p=1）で debug 0.40 秒・release 0.02 秒（この PC、論理プロセッサ 12）。tokio 1.53.3: `max_blocking_threads` 既定 512、`spawn_blocking` は abort できずランタイムの片付けでも終わりを待つ（`task/blocking.rs`）、`tokio::fs` は中で spawn_blocking。RustNote の Dockerfile は `cargo build --release --locked`。
- RustNote の改善候補として本文に書いたこと（コードは未変更）: Argon2 の同時実行数の上限が無い（Semaphore の Hasher で直せる）。`authenticate` の `&DUMMY_HASH` で LazyLock の初回計算がワーカースレッドで動く（起動時に spawn_blocking の中で force しておく関数を auth.rs に用意する案）。

## 2026-10-24: Rust 05 巻に UNIT 06 を追加して main に反映（2430d9d）

- 本文は UNIT 01〜06（6,122行・図23枚・157ページ）。「準備中」は UNIT 07〜10。旧 `Rust/05_…pdf` はまだ残してある。`assemble.py` は `range(1, 7)`。
- UNIT 06 の実出力は `rust05/out06/`。Ctrl+C の例（`u06_shutdown`・`u06_q3`）は `sendctrlc.py <exe> 1.3` で、新しいコンソールに Ctrl+C を送って取った。追加した例: `u06_fix_elapsed`・`u06_q1`〜`u06_q3`、`errs/e06_elapsed_question.rs`（E0277）・`errs/e06_tracker_not_closed.rs`（close 忘れで wait が終わらない。len は 0）。`cargo add tokio-util@=0.7.20 --features rt` の実出力は `rust05/probe/cargo_add_tokio_util.txt`（probe に rust-version 1.98 があるので「Locking 7 packages to highest Rust 1.98 compatible versions」）。
- 確かめた事実: tokio-util 0.7.20 は `sync`（CancellationToken）が機能なしで使え、`task`（TaskTracker）は `rt` が必要。Topcoat 0.10 の `serve` は Ctrl+C か Unix の SIGTERM で graceful shutdown、`DEFAULT_SHUTDOWN_TIMEOUT` は 30 秒（`topcoat-router-0.10.0/src/service.rs`）。RustNote のコードに tokio の timeout は無く、ローカル DB の `PRAGMA busy_timeout = 5000` だけ。
- 次は UNIT 07（spawn_blocking）。素材は `u07_blocking`・`u07_hash_time`・`u07_password`・`u07_limit` と `errs/e07_*`（blocking_panic・borrow_escape）。RustNote の `auth.rs` の spawn_blocking と、Argon2 の同時実行数の上限が無い点（UNIT 05 で予告済み）、DUMMY_HASH の LazyLock の初回が worker スレッドで重い計算をする点を扱う。

## 2026-10-24: Rust 05 巻に UNIT 05 を追加して main に反映（69231d7）

- 本文は UNIT 01〜05（5,263行・図19枚・133ページ）。「準備中」は UNIT 06〜10。旧 `Rust/05_…pdf` はまだ残してある。`assemble.py` は `range(1, 6)`。
- UNIT 05 の実出力は `rust05/out05/`。追加した例: `errs/e05_std_deadlock.rs`（current_thread + join! で標準 Mutex を持ったまま .await → 本当に止まる。Git Bash の `timeout 5` で止めて exit=124）・`u05_fix_deadlock.rs`（tokio Mutex で直る）・`u05_q1`（{ } で直す）・`u05_q2`（Semaphore 3枚）。問3は道具の選び方（コードなし）。
- 確かめた事実: Rust 1.99.0 でも `drop(guard)` の後の `.await` で「future cannot be sent」が消えない。spawn しないと通り、clippy の `await_holding_lock` だけが出る。tokio 1.53.3 のドキュメント: Semaphore は公平（求めた順）、RwLock は公平（書き込み優先）、mutex.rs に「標準の Mutex を async で使うのは問題なく、むしろ好ましいことが多い」。tokio の `Mutex::lock` は `Result` を返さない。RustNote は Mutex 類を使わず、`LazyLock`（DUMMY_HASH）だけ。
- 次は UNIT 06（timeout・キャンセル・graceful shutdown）。素材は `u06_timeout`・`u06_cancel`・`u06_shutdown`（tokio-util の CancellationToken・TaskTracker）・`u06_blocking_timeout` と `sendctrlc.py`（Ctrl+C を送る補助）。Topcoat の `internal_serve` の drain → cutoff の流れ（UNIT 02〜04 で引用済み）と、`serve.rs` の「Ctrl+C か SIGTERM で graceful shutdown、処理中は最大30秒」を本文でつなぐ。

## 2026-10-24: Rust 05 巻に UNIT 04 を追加して main に反映（317e15c）

- 本文は UNIT 01〜04（4,246行・図15枚・109ページ）。「準備中」は UNIT 05〜10。旧 `Rust/05_…pdf` はまだ残してある。`assemble.py` は `range(1, 5)`。
- UNIT 04 の実出力は `rust05/out04/`（エラー例 e04_* の出力は `rust05/out/`）。追加した例: `u04_try_send.rs`（Full ですぐ断る）・`u04_forgot_drop.rs`（drop(tx) 忘れで終わらない。select! で1秒だけ待つ）・`u04_q1`〜`u04_q3`（容量5・Reset の依頼と ask 関数・watch の止める合図）。
- 確かめた事実（tokio 1.53.3 のソース）: `mpsc::Sender::send` は select! で負けると値が捨てられる（cancel safety の注記）。`mpsc::Receiver::recv`・`watch::Receiver::changed` はキャンセル安全。`watch` の `borrow` は読み込みの鍵を持つ。`JoinHandle` は `Unpin`。`oneshot::Sender::send(mut self, …)`。`broadcast::channel<T: Clone>`。Topcoat の `internal_serve` は `drain`・`cutoff`・`done` の3つの watch を「送り口を drop して合図」「受け取り口の数で生きているタスクを数える（`done_tx.closed()`）」に使う（本文で引用）。
- 次は UNIT 05（Mutex・RwLock・Semaphore）。素材は `u05_std_mutex`・`u05_tokio_mutex`・`u05_rwlock`・`u05_semaphore`・`u05_sync_helper` と `errs/e05_*`（guard_await・drop_guard・double_lock・guard_no_spawn）。第3巻 UNIT 10 の「第5巻への橋渡し」（鍵を持ったまま .await しない）を受ける。

## 2026-10-23: Rust 05 巻に UNIT 03 を追加して main に反映（6b0007a）

- 本文は UNIT 01〜03（3,228行・図11枚・83ページ）。「準備中」は UNIT 04〜10。旧 `Rust/05_…pdf` はまだ残してある。`assemble.py` は `range(1, 4)`。
- UNIT 03 の実出力は `rust05/out03/`。追加した例: `u03_deadline.rs`（pin! した締め切り）・`u03_select_restart.rs` / `u03_select_resume.rs`（ループ内 select! のやり直し問題と直し方）・`u03_q1`〜`u03_q3`。`u03_limited.rs` は clippy の collapsible_if を避けるため、入れ子の `if let` を `expect` に書き換えた。`u03_joinset_fail.rs` は最後に「main が終わります（0.3 秒）」を足し、C の片付けが JoinSet の取り消しによるもの（プログラム終了ではない）と分かるようにした。
- 確かめた事実（tokio 1.53.3 のソース）: `JoinSet` の `Drop` は中のタスクを全部 `abort` する。`join_all` は子が panic すると `resume_unwind` で投げ直す。`TcpListener::accept` と `JoinSet::join_next` はキャンセル安全と明記。Topcoat の `internal_serve` は `let mut shutdown = pin!(shutdown);` を loop の外で作り、`select!` の枝で `&mut shutdown` を使う（本文で引用）。
- 作業中に気づいたこと: スクラッチ領域の一部（`rust04/render.py`・`rust_txt/`・`rust_png/` など）が消えていた（原因不明。Temp の掃除の可能性）。図の確認用スクリプトは `rust05/render.py` に作り直し、`save_wip.sh` で作業ブランチにも保存するようにした。旧 PDF のテキスト抽出（`rust_txt/`）が必要なら、旧 PDF から PyMuPDF で取り直す。

## 2026-10-22: Rust 05 巻を UNIT 01・02 まで main に反映（98bf0d1）

- `Rust/05_Rustバックエンド_async_並行処理_ジョブ_運用.md`（2,267行・図7枚）と `Rust/pdf/05_…pdf`（61ページ）を main へ。UNIT 03〜10 は本文の表とお知らせに「準備中」と明記。**旧 `Rust/05_Rustバックエンド_async_並行処理_ジョブ_運用.pdf` は残してある**（UNIT 10 まで入ったら publish.sh に削除パスとして渡す）。
- 下書きは `rust05/parts/`（intro・unit01・unit02・outro）。連結は `rust05/assemble.py`（いまは `range(1, 3)`。単元を足したら増やす）、素材の保存は `rust05/save_wip.sh`。図の id は `ar-05-<単元2桁><連番>`（巻頭は `ar-05-001`・`002`）、キャプションは `図5-?:`。
- 続きの書き方: `parts/unit03.md` から書き、intro の表の「（準備中）」とお知らせ、outro のお知らせ・チェックリスト・用語を更新する。UNIT 03〜09 の例は `async_lab/src/bin/u03_*`〜`u09_*`、エラー例は `errs/e03_*`〜`e09_*`（出力は `out/`）。**u03 以降の実行結果は保存されていないので、書くときに実行し直す**。UNIT 10（バックグラウンドジョブ）は素材なし。
- 練習プロジェクトは本文では `async_lab`（実出力の `Compiling async_lab` に合わせた）。`cargo add tokio@=1.53.3 --features full` の実出力は `rust05/probe/cargo_add.txt`。
- 確かめた事実（2026-10-22）: tokio 1.54.0 が出ているため、`cargo add tokio@1.53.3`（`=` なし）だと 1.54.0 が入る。本書は `@=1.53.3` で固定した（第7巻の巻頭は `=` なしのまま。読者の手元では 1.54.0 になる可能性があるが未修正）。Topcoat 0.10 は `topcoat-router-0.10.0/src/service.rs` の `internal_serve` で、接続ごとに `tokio::spawn` し、`tokio::select!` で accept と停止の合図を選び、`watch::channel` で処理中の接続に停止を伝えて最大 `shutdown_timeout` 待つ（UNIT 03・04・06 で使える）。`enable_all()` を忘れたランタイムで sleep すると `A Tokio 1.x context was found, but timers are disabled.` で panic。Rust 1.99 の panic 表示は `thread 'tokio-rt-worker' (OS のスレッド番号) panicked at …`。

## 2026-10-21: Rust 07 巻に UNIT 10 を追加して巻を完成（main 17a950d）

- 本文は UNIT 01〜10（8,758行・図27枚）、PDF は 222 ページ。巻頭・巻末の「準備中」とお知らせは外した。**旧 `Rust/07_Turso_libSQL_SQL設計_実装_運用.pdf` は削除済み**（`Rust/pdf/` の新しい PDF に置き換え）。Rust 07 巻はこれで完成。
- UNIT 10 の例は `rust07/sql-lab/src/bin/u10_*.rs`、実出力は `rust07/out10/`。Turso CLI のコマンド（db create・db show --url・tokens create/invalidate・db export・--from-db --timestamp・--from-file）は公式ドキュメント（`scratchpad/turso/`、2026-10-17 取得）の形だけを載せ、実行していないと本文に明記した。
- 確かめた事実（2026-10-21）: libsql 0.10 のローカル DB の journal_mode は `delete`。`VACUUM INTO ?1` はファイル名の bind ができる。読みかけの `Rows` だけでなく、受け取った `Row` も文を持つので、`drop(rows)` だけでは `cannot VACUUM - SQL statements in progress` が消えない（`local/rows.rs` で `Row` が `Statement` を持つ）。同名の写しは `output file already exists`、transaction 中は `cannot VACUUM from within a transaction`。`ATTACH` は存在しないファイルを空の DB として作る（`no such table: bk.notes`）。VACUUM/ATTACH の `execute` の戻り値は直前の DML の行数のまま（本文では使っていない）。
- RustNote について本文に書いたこと: `/health` は `SELECT 1` だけで、ローカルのパスを打ち間違えると起動時の migration が空の DB に表を作り、200 のまま空のアプリとして動く。手がかりは起動ログの「マイグレーションを適用しました: 0001_users」。トークンは起動時に1回だけ読むので、入れ替えには再起動が要る。RustNote のコードは変更していない。

## 2026-10-20: Rust 07 巻に UNIT 09 を追加して main に反映（8bd5889）

- 本文は UNIT 01〜09（7,800行・図24枚・198ページ）。「準備中」は UNIT 10 だけ。**旧 `Rust/07_Turso_libSQL_SQL設計_実装_運用.pdf` はまだ残してある**（UNIT 10 が入ったら publish.sh に削除パスとして渡す）。
- 続き: `rust07/parts/unit10.md`（未着手。backup・token・監視・障害対応）を書き、`assemble.py` を `range(1, 11)` に、intro の表の（準備中）1か所・お知らせ（2か所の「UNIT 10」）・ヒント（「準備中の UNIT 10 で扱います」）、outro のお知らせ・チェックリスト・用語を更新する。
- UNIT 10 の素材: 本物の Turso は未使用。`scratchpad/turso/` に公式ドキュメント（cli_db_tokens_create・cli_db_export/import・features_point-in-time-recovery・cloud_durability など）の保存あり。RustNote の `/health`（`src/routes/health.rs`、`SELECT 1` だけ）と `log_if_internal`（`src/views.rs`）。
- UNIT 09 の実出力は `rust07/out9/`。追加した例: `u09_err_columns.rs`・`u09_err_in_limit.rs`・`u09_q1.rs`・`u09_q2.rs`・`sql/u09_inner_join.sql`。`cargo add async-trait@0.1.92` の出力は `rust07/addprobe`（rust-version 1.98、lock から async-trait を外して取得）。
- 確かめた事実: RustNote のテストは `:memory:` ではなく一時フォルダの `test.db`（`tests/common/mod.rs`）。RustNote のテストに Repository の別実装は無い（本文は「差し替えられる形」とだけ書いた）。UNIT 08 の「第11巻で使います」は削除済み。

## 2026-10-19: Rust 07 巻に UNIT 08 を追加して main に反映（0a50245）

- 本文は UNIT 01〜08（6,600行・図22枚・168ページ）。巻頭・巻末の「準備中」は UNIT 09・10 だけ。**旧 `Rust/07_Turso_libSQL_SQL設計_実装_運用.pdf` はまだ残してある**（UNIT 10 まで入ったら publish.sh に削除パスとして渡す）。
- 続き: `rust07/parts/unit09.md`（構成メモと実出力あり）と `unit10.md`（未着手）を書き、`assemble.py` の `range(1, 9)` を `range(1, 11)` に、intro の表の（準備中）2か所・お知らせ・ヒント（「UNIT 10 で扱います」）、outro のお知らせ・チェックリスト・用語を更新する。
- UNIT 08 の実出力は `rust07/out8/`。追加した例: `sql-lab/src/bin/u08_err_stack.rs`・`u08_stack_spawn.rs`・`u08_q3.rs`・`u08_q3_leak.rs`、features が core だけの E0599 は `rust07/core-only/`（ビルドは `CARGO_TARGET_DIR=ref/libsql-core-noclang-probe/target` で clang なし）。
- 確かめた事実（2026-10-19）: remote の `build` は接続せず 4ms で成功し、最初の SQL で失敗する。replica の `build` は中で接続して約2秒後に `sync error`、コピーのファイルは作られない。debug の main で remote `execute` はスタックあふれ（終了コード -1073741571 = 0xC00000FD）、`query` では起きない。release と `tokio::spawn` では Hrana のエラーが返る。リモートの SQL エラーも `Hrana` に包まれる（libsql 0.10 のソース `hrana/mod.rs` で確認、本物の Turso では未確認）。
- RustNote の `main.rs` は起動時の migration を main の中で実行する。Windows の debug ビルドで Turso につなぐとスタックあふれの可能性あり（未確認。本文では「`--release` で試すと安全」とだけ書いた）。

## 2026-10-18: Rust 07 巻を UNIT 01〜07 まで main に反映（59eb69c）

- `Rust/07_Turso_libSQL_SQL設計_実装_運用.md`（5,700行・図19枚・147ページ）と `Rust/pdf/07_…pdf` を main へ。UNIT 08〜10 は本文に「準備中」と明記。**旧 `Rust/07_Turso_libSQL_SQL設計_実装_運用.pdf` は UNIT 08〜10 が入るまで残してある**（完成したら publish.sh に削除パスとして渡す）。
- 続きの書き方: `rust07/parts/unit08.md`〜`unit10.md` を書き、`rust07/assemble.py` の `ORDER` を `range(1, 11)` に戻し、`parts/intro.md` の「（準備中）」3か所とお知らせ・ヒント、`parts/outro.md` のお知らせを外して、チェックリストと用語に UNIT 08〜10 分を足す。
- UNIT 08 の素材（実行済み）: `sql-lab/src/bin/u08_local.rs`・`u08_open.rs`（出力は parts/unit08.md の構成メモ）、`u08_replica.rs`（コンパイルのみ確認）。debug ビルドで閉じたポートへの remote execute は `thread 'main' has overflowed its stack`（exit 0xc00000fd）、**release ビルドと tokio::spawn の中では起きず Hrana のエラーが返る**ことを 2026-10-18 に確認。core だけの features での E0599 は未作成。
- UNIT 09 の素材: `parts/unit09.md` の実出力と `src/bin/u09_*.rs`。UNIT 10 は未着手。
- `_build_pdfs.py` に `overflow-y: hidden` を追加（コード枠に縦スクロールバーが出ることがあったため）。既存の他の PDF は再生成していない。

## 3回目の停止（2026-10-18）— Rust 07 巻の途中

Rust 07 巻（Turso・libSQL）を1巻だけ仕上げる作業の途中で停止。main への反映はなし。下書きと素材は `rust07/` にある。

| 部品 | 状態 |
|---|---|
| `rust07/parts/intro.md`（表紙・巻頭・学習地図・準備・道具 sql の全文） | **完成**（673行、図3、check_md OK、実出力貼り込み済み） |
| `rust07/parts/unit01.md` | **本文完成**（約820行。目安より長いので連結時に削るか判断。check_md 未実行） |
| `unit02.md`〜`unit08.md` | 本文なし。構成メモ・図の案・使う SQL と出力の対応だけ（`<!-- TODO: 未完 -->`） |
| `unit09.md` | 本文なし。構成メモと実出力（u09_repo / u09_layers / u09_n_plus_1、E0038 など） |
| `unit10.md` | 未着手（ファイルなし） |
| `outro.md`（まとめ・用語ミニ辞典） | 未着手 |

- 例はほぼ全単元ぶん実行済み。SQL は `rust07/sql-lab/sql/`、Rust は `rust07/sql-lab/src/bin/`、出力は `rust07/out/`（UNIT 01〜03・07）、`rust07/out456/`（UNIT 04〜06 の再実行と練習問題の解答）、エラー例は `rust07/errs/`・`errs09/`。
- 連結は `rust07/assemble.py`（parts を連結し `図7-?:` を順に番号付けして `Rust/07_Turso_libSQL_SQL設計_実装_運用.md` に書き出す）。PDF は `_build_pdfs.py`、反映は `ref/publish.sh`（旧 `Rust/07_Turso_libSQL_SQL設計_実装_運用.pdf` を削除パスとして渡す）。
- 図の id は `ar-07-<単元2桁><連番>`、巻頭は `ar-07-001`〜`003`。本文から図を番号で参照しない約束。
- `sql-lab/Cargo.toml` に UNIT 09 用の `async-trait = "=0.1.92"` を追加済み。巻頭の Cargo.toml には入れていないので、UNIT 09 で `cargo add async-trait@0.1.92` と書くこと。
- 担当が見つけた事実: SQLite エンジンは 3.47.0（libsql 0.10.0）。初回ビルド約1分・181 crate。bind の値が1つ多いと余りは黙って無視され `Ok(1)`。IN の変数は 32,766 個まで。N+1 は 1,000 件で約 259ms 対 4ms。索引で 2055→2899 ページ。debug ビルドの remote 接続のスタックあふれは `tokio::spawn` 内なら起きない（release は未確認）。
- 直すこと: `sql-lab/src/bin/u05_conflict.rs` のコメント「第4巻 UNIT 07 の形」は誤りで、AppError は第4巻 UNIT 05。
- 未検証: Turso 本物・ローカル libSQL サーバーへの接続成功時、トークン不正時のエラー文（この PC に Turso CLI・sqld なし）。

## 最新の状況（2026-10-17、2回目の停止時点）— まずここを読む

### main に反映済み（完成）
- next/: **全12章（00〜11）完成**。PDF も再生成済み。
- Rust/: **00・02・03・04 巻**（Markdown で全面書き直し、PDF は `Rust/pdf/`。旧 `Rust/*.pdf` は該当巻だけ削除済み）。
- `Rust/rustnote/`: RustNote アプリの完全なソース（Rust 1.99.0 で fmt / clippy / test 41件 成功。Secret 型でトークンを隠す修正済み）。

### 未完成（本文は未執筆。素材だけこのフォルダにある）
| 巻 | 状態 | 素材（このフォルダ内） |
|---|---|---|
| Rust 01 環境構築 | 本文 0。実出力 約60件・練習プロジェクト取得済み | `rust01/`（out/, rust-study/） |
| Rust 05 async | 本文 0。UNIT 01〜09 の例31本と実出力、エラー例19本 | `rust05/`（async_lab/, errs/, out/） |
| Rust 06 HTTP/Topcoat | 本文 0。UNIT 01 と 02 の一部の実出力 | `rust06/`（web_lab/, out/） |
| Rust 07 Turso/libSQL | 本文 0。UNIT 01〜07 のコードと実出力44件 | `rust07/`（sql-lab/, out/, errs/） |
| Rust 08 認証認可 | 本文 0。UNIT 01 の例1本、Better Auth 1.7.10 のデモ結果（下記） | `rust08/` |
| Rust 09 RustNote 設計 | 本文 0。ほぼ全単元の実出力 | `rust09/`（out/, rustnote_lab/, note_layers/ ほか） |
| Rust 10 CRUD | 本文 0。RustNote のコピーと起動確認、PowerShell の補助スクリプト | `rust10/` |
| Rust 11 品質保証 | 本文 0。UNIT 01 のテストと実出力 | `rust11/`（qa_lab/, log-test-baseline.txt） |
| Rust 12 Fly.io | 本文 0。素材なし（読んだのみ） | — |
| Rust 13 完全コード | 未着手 | — |

共通の材料: `ref/rust-volume-brief.md`（執筆ブリーフ）、`ref/decisions.md`（統一の決定事項）、`ref/topcoat-0.10-reference.md`、`ref/rustnote-design-notes.md`、`ref/check_md.py`（品質チェック）、`ref/publish.sh`（main と作業ブランチへ反映するスクリプト。パスはこの PC 用）。

### 担当が見つけた事実（再開時に本文へ反映する）
- Rust 1.99: `TryFrom` 失敗の文言が「number too large/small to fit in target type」に変更。RefCell の二重借用 panic は「RefCell already borrowed」。
- `.await` の前に `drop(guard)` しても MutexGuard の Send エラーは消えない（`{}` で囲むか同期関数に分ける）。
- libsql 0.10 のローカル DB は foreign_keys が既定で ON。bind の値が足りないと NULL 扱いでエラーにならない。`row.get` の型違いは unreachable の panic。未コミットの transaction は drop でロールバック（第3巻の予告を確認済み）。Windows ARM64 の debug ビルドで remote 接続の execute が main スレッドのスタックオーバーフロー（release か tokio::spawn で回避）。
- reqwest 0.13 は Windows ARM64 で clang-cl が必要。閉じたポートへの接続は約2秒後に os error 10061。
- Topcoat: `start` は起動時に何も表示しない。既定で `vary` ヘッダーが付く。Ctrl+C で新規受付停止→処理中を最大30秒待つ。`topcoat dev` は dev.js を差し込む。1.97 でのビルドエラー文「rustc 1.97.1 is not supported ... topcoat@0.10.0 requires rustc 1.98」。
- Turso CLI は Windows 版がなく WSL が必要。トークンの `--expiration` は `Nd` か `never`。Edge replica は新規非推奨、embedded replica は継続。PITR は `--from-db --timestamp`。Turso Cloud では `PRAGMA user_version` 読み取り専用。
- flyctl の公式 install.ps1 は常に x64 版（ARM64 ではエミュレーション）。ARM64 では rustup-init が VS を自動導入できず、Build Tools for Visual Studio 2026 の「MSVC Build Tools for ARM64/ARM64EC」を手動で選ぶ。
- PowerShell 5.1: 日本語は `[Console]::OutputEncoding = UTF8` を最初の curl より前に。.ps1 は BOM 付き UTF-8 でないと Shift-JIS 扱い。Git の MinGW curl では日本語の form 値が化けるので `[uri]::EscapeDataString` で先にエンコード。
- Better Auth 1.7.10 デモ（rust08 担当）: 登録 200（cookie `better-auth.session_token`、HttpOnly、SameSite=Lax、7日）、重複 422、ログイン失敗 401（未登録と同じ）、10秒で4回目 429、Origin 不一致 403、DB の session.token は生のトークン、password は scrypt。
- OWASP Argon2id: m=19456,t=2,p=1（argon2 0.6 の既定と一致）。NIST SP 800-63B-4: 単要素パスワードは15文字以上。

### RustNote 実装の改善候補（担当が指摘。未対応）
1. 検索欄に label / aria-label がない。エラー表示が aria-describedby で入力欄と結び付いていない。入力欄の枠線 #e2e8f0 のコントラストが 1.23:1（非テキストは 3:1 必要）。
2. 登録時の 409「登録済み」でメールの存在が分かる（ログインはダミーハッシュで対策済み）。ログイン/登録のレート制限なし。Argon2 の同時実行数の上限なし。パスワード最小 8 文字（NIST は単要素なら15）。DUMMY_HASH の初回生成で1回だけ遅い。セッションは固定30日でアイドルタイムアウトなし。
3. `NoteTitle::parse` がゼロ幅スペースだけのタイトルを通す。最終ページより先で「まだノートがありません」と表示。削除に確認がない。絵文字で maxlength とサーバーの文字数が食い違う可能性。狭い画面用の CSS がない。全ページ同じ `<title>`。
4. `/health` が `SELECT 1` だけ（DB ファイル消失でも 200）。`main.rs` がポート確保前に「起動しました」と表示。
5. デプロイ: Dockerfile の既定 `DATABASE_URL=/data/rustnote.db` で secrets を忘れると一時ディスクで動く。起動時 migration は複数マシン同時起動で競合しうる・migration 単独コマンドがなく release_command を使えない。ci.yml の `actions/checkout@v7` は存在未確認、`.github/workflows/` に置かないと動かない。release-and-startup-errors.txt は古いビルドのログの可能性。
6. RustNote のログは tracing ではなく eprintln!。

### next/ の残る懸念（任意）
- 第7章の `app/page.tsx` が独自に `<header>`・`<main>` を持つため、第6章の layout と合わせるとヘッダーが二重になる。
- 第8章の BookForm は第7章と作りが違う（「更新版」として掲載）。

### 作業中に起きた副作用（ユーザーへ報告済み）
- rust08 担当が `taskkill /F /IM node.exe` を実行し、その時点で動いていた node.exe を全て強制終了した（自分の起動分以外も含む）。
- ref 調査の下位エージェントが crates.io API へのリクエストの User-Agent にユーザーのメールアドレスを入れてしまった（その後は不使用）。
- rust01 担当が誤って 1.98.0 ツールチェーンを入れ、削除済み。既定は stable 1.97.1 のまま、1.99.0 が追加インストール済み。
- `%APPDATA%\create-next-app-nodejs\Config\config.json` が更新された（無害な既定値）。空フォルダ `C:\tw-short` が残っている。

---

# 以下は 1回目の停止時（2026-10-15）の記録

## next/10-deployment.md
- 本文は未編集（差分なし）。
- 下書き: scratchpad/next10/figs/f01〜f16.html（SVG図16枚、画面イメージ6枚含む。13・16は修正後の再確認未了）、scratchpad/next10/outputs/（Next.js 16.4.1 試作アプリでの実出力: build成功/型エラー/'use client'なし/import大文字小文字/cacheComponentsビルド失敗/ESLint/prebuild環境変数チェック/git check-ignore/NEXT_PUBLIC_埋め込み/api/health/実行時エラーdigest/ポート使用中）
- 本文の修正候補: 0.1・1.2 の出力例が λ（正しくは ƒ）で /books/[id] を静的○扱い / 4.2 params: { id: string } は旧形式（Next 16 では id 空のまま表示）/ 3.2 Redirect URL https://*.vercel.app は広すぎ（Supabase 推奨は https://*-<account>.vercel.app/**）/ 2.7 Vercel 画面操作が旧UI（2026-08 から Config/Secret 種別）/ 2.8 Google Domains は終了 / 2.5 Supabase キーの場所は Settings → API Keys、publishable key（sb_publishable_...）
- 他章への重要懸念: create-next-app@latest = Next.js 16.4.1 で next.config.ts に cacheComponents: true が既定。教材のトップページ（Server Component で Supabase 読み込み）は npm run build が "uncached or runtime data during prerendering" で失敗（dev では気づかない）。cacheComponents を外すと通る。→ 第06章で版固定か注記を決める必要あり。

## next/00-introduction.md — ほぼ完了（未PDF確認一部）
- 追加: SVG 17枚（図0-1〜0-17、画面イメージ3）、サンプル8（Node 25 実行・tsc --strict 済）、つまずき12（NG→OK 9）、章冒頭ゴールと地図、## 12. 練習問題 7問。
- 既存修正: ロードマップを実際の第1〜11章に合わせた、目安時間表、エラーの読み方（〇〇Error: 行を探す）。
- 未了: 図0-9〜0-17 の PDF 目視、連続する引用囲みが1つの青枠に結合する（間に通常の文を挟む）、ロードマップのページ割れ、見出し「## 3.」が2回（既存）。

## next/01-environment-setup.md — ほぼ完了（練習問題なし・PDF未確認）
- 追加: SVG 12枚、サンプル10（実行済）、つまずき15、章冒頭ゴールと地図。
- 既存修正: nvm-windows v2.0.1 対応、nvm-sh v0.40.8、NodeSource setup_24.x、推奨 Node 24 LTS（最低22）、VS Code code コマンドの Windows 手順、3.3 の書式。
- 未了: 章末練習問題（未記入）、PDF 生成・目視、まとめの「次章でプロジェクト作成」誤り（次は TS 基礎）。

## next/04-nextjs-basics.md — 追加はまだ0（既存コメント修正のみ）
- 変更: JSX 内の // コメントを {/* */} に、行末側コメント約60を上の行へ（129行）。TS パーサで構文確認済。
- 確認済みの既存誤り（未修正）: 7.2 useActionState(createBook,null) 型エラー（ラッパー関数で修正可）/ 8.2・エラー5 非 NEXT_PUBLIC_ 変数を Client で読むと SSR HTML に実値が漏れる / エラー7 の警告は再現せず / 「force-cache が既定」は Next 15+ で誤り / Next 16: middleware.ts→proxy.ts、next/image priority→preload、error.tsx retry / エラー1・4・6 の文言が古い。
- 素材: scratchpad/next04/（bookapp 試作、ng 版、各種実出力）。
- 他章影響: create-next-app 既定 cacheComponents: true で、loading.tsx/Suspense なしの動的ルートや export const revalidate が build 失敗。06〜08・10 で方針統一が必要。

## next/07-crud-list-create.md — ほぼ完了（PDF 143p 生成済）
- 追加: SVG 13（画面イメージ3、全てPNG確認済）、サンプル9（Node 25・tsc --strict）、つまずき8、章冒頭、8. 練習問題 7問、Supabase 2026 の GRANT 変更（42501）、publishable key、PGRST204、エラーコード表。
- 既存修正: PGRST205、thenable の説明、catch に来ない件、error.tsx retry（16.3+）、useFormState→useActionState、Tailwind クラス名、リストの空行。
- 未了: NG/OK の後の説明段落が次の囲みと同じ青枠に結合（見た目のみ）。
- 他章不整合: 05 は notes 列、07 は memo 列 / 06 の BookStatus・lib/supabase.ts が 07 と名前不一致 / 05 に GRANT 手順なし。

## next/08-crud-edit-delete.md — ほぼ完了（PDF は図8-9修正前）
- 追加: SVG 11（画面イメージ3）、サンプル8（Node・tsc）、Next 参考コード3（Next 16.4.1 / supabase-js 2.117.3 で型チェック）、つまずき7、練習問題7、notFound・bind・.eq 忘れ・制約違反(23502/23514) など。
- 既存修正: JSX コメント位置15箇所（HEAD のままだと型エラー29件）、7-2 の RLS 説明。
- 要判断: status 値が 05/07（want_to_read/reading/completed）と 06/08（unread/reading/finished）で不一致 / author NOT NULL と BookForm の null 送信 / 一覧の場所が 07 は / 、08 は /books。

## next/09-styling.md — ほぼ完了（PDF は最終小修正前、187p）
- 追加: 図20（SVG17＋HTML見本3、画面イメージ6）、サンプル10（Tailwind v4.3.3 CLI 6・node 4）、つまずき9、章冒頭、練習問題8、1.1.1 Tailwind v4 の設定。
- 既存修正: 0.7 flex 抜け、6.3 コントラスト比の数値。1.1〜1.3 は v3 記述のまま注記。
- 未了: 最終 PDF 再生成と目視、本文に貼ったコードの再実行。

## Rust/00_学習ガイド — 途中（1,355行）
- 完了: 表紙・巻頭（図5）、UNIT 01、UNIT 02。未着手: UNIT 03〜10、この先の学び方、まとめ、用語ミニ辞典。
- 素材: scratchpad/rust00/（parts/、ex/ に全単元の例と解答 .rs（実行済）、study/、各種スクリプト、UNIT 03 の PowerShell 記録）。
- 横断事項: Topcoat 0.10 は rust-version 1.98 以上が必要（手元 stable 1.97.1、1.99.0 もインストール済）→ 第1巻で版を決める。Topcoat start の既定は 127.0.0.1:3000（HOST/PORT で変更）、Fly.io は 0.0.0.0:8080。

## next/02-typescript-basics.md — 途中（PDF 未生成）
- 追加: SVG 16（画面イメージ3）、サンプル9（Node 25.9 / TypeScript 7.0.2 実出力）、つまずき7、章冒頭＋サンプルを動かす準備、0.10.1・1.1・1.3・2.4・2.11・3.1・3.2・4.4.1・5.1・5.2例5・6.1.1・6.2.1・7.3.1・8 冒頭。
- 既存修正: 0.4 コメント位置、2.2 "a"*2、2.4 空配列の型（any[] / never[]）、2.9 実際のエラー文、4.2 typo 例 "actve"、1.1 node は型チェックしない注記。
- 未了: 練習問題（7問は scratchpad/next02/final/q1〜q7 に検証済み）、8.2 エラー文更新（TS18048/TS18047）、8.3 as のつまずき、enum を node で実行できない件、|| と ??、まとめ表、PDF。素材: scratchpad/next02/final/, outputs.txt

## next/05-supabase-setup.md — 途中（強制停止。PDF 未生成）
- 追加: SVG 9（画面イメージ4: 5-6 ダッシュボード、5-8 SQL Editor、5-12 API Keys ほか）、章冒頭。サンプル・つまずき・練習問題は 0。
- 既存修正: 2.1 サインアップ方法、2.2/6.2 キーの場所（Connect / Data API / API Keys、Publishable/Secret、旧 anon/service_role は 2026 年末廃止予定）、2.4 無料枠、4.4 id 既定 int8、GRANT 注記。
- **要対応（参照切れ）**: 4.3.1 GRANT 節が未作成なのに、図5-1・本文 ~390/~1663/~1798 行・目次の「練習問題」が参照している。2026-05-30 以降の新規プロジェクト（既存も 2026-10-30 から）は public の新規テーブルが API に自動公開されない → `GRANT SELECT, INSERT, UPDATE, DELETE ON public.books TO anon, authenticated, service_role;` が必要（無いと 42501）。
- 既存誤り未修正: 9.4 エラー文に for relation "books" が抜け / 7.2 .select() 無しなら data は null / 発展 RLS が開発用 USING(true) を DROP しておらず OR 結合で全員読める。図番号欠け（5-3,5-4,5-9）。
- 素材: scratchpad/next05/（supabase-playground/、PostgreSQL 17.6 + PostgREST 14.18 + supabase-js 2.117.3 の実出力: 42501, RLS で data:[], insert 違反, .select() 有無, 23514, PGRST116, 22P02, 0行update, error 無視の TypeError, キー確認スクリプト, ユーザー別 RLS, seed）

## next/06-project-setup.md — 途中（PDF 未生成）
- 追加: 章冒頭（図6-1）、図6-2〜6-6（npm install、ブレークポイント、import 関係図 4.2.1、Book 型比較、layout/page）。SVG 計6。サンプル・つまずき・練習問題 0。
- 未修正の古い記述: 1.2 質問一覧、1.4 出力、1.5 ファイル一覧、1.6 package.json、5.1 Supabase 画面、7.2 globals.css、9.2 出力、9.4 ポート。
- 検証結果（create-next-app 16.4.1）: 質問順 = recommended defaults? → TS → linter(ESLint/Biome/None) → React Compiler → Tailwind → src/ → App Router → Cache Components → import alias → AGENTS.md → フィードバック。Turbopack の質問なし。推奨既定は src/ なし・Cache Components 有効。eslint.config.mjs / AGENTS.md / @tailwindcss/turbopack、postcss/tailwind.config なし、globals.css は @import "tailwindcss"。
- 素材: scratchpad/next06/（実出力: 対話画面、npm run/ls、audit fix --force の警告、dev 出力、二重起動、ENOENT、Reload env、環境変数未設定、@/src の Module not found、use client 忘れ、Cache Components + cookies() の prerender エラー、プロジェクト名エラー、章コードの build 成功）
- 懸念: 第9章が Tailwind v3 前提 / 第7章の cookies() が Suspense 外 → 第6章で Cache Components は No を明記すべき。
- 副作用: %APPDATA%/create-next-app-nodejs/Config/config.json が更新された（無害な既定値）。

## next/11-vibe-coding-with-claude-code.md — 途中（PDF は途中版 95p）
- 追加: SVG 12（画面イメージ2）、サンプル7、つまずき9、章冒頭、0.1.2/0.4.1/0.4.2/1-4/5-4-1/7-6-1 ほか。
- 既存修正（code.claude.com で確認）: インストール方法（公式インストーラ推奨、npm は Node 22+）、権限パス Edit(.env)/Read(.env)、権限モード表、MCP の設定場所（.mcp.json / ~/.claude.json）と存在しない項目の削除、hooks は stdin JSON、SessionStart matcher compact、プランモード、AGENTS.md、skills の allowed-tools、組み込みコマンド、サブエージェントの項目。
- 未了: 9-7 sortBooks + Vitest、9-8 チェックリスト＋図＋つまずき⑩⑪、12 節の図、練習問題、目次。未修正: 3-2/3-3/14-3/17-2/18 の .claude/.mcp.json 記述、12-2 /cost→/usage、13-2 Ctrl+L、15-2 --effort は max が最上位。図11-1・11-5 の修正後未描画、11-8〜11-12 未確認。素材: scratchpad/next11/sortdemo/, render.py

## Rust/03 所有権 — 途中（3,105行）
- 完了: 表紙・巻頭・UNIT 01〜04（図12）。未着手: UNIT 05〜10 本文、まとめ、用語ミニ辞典。
- UNIT 05〜10 のコードと実出力は作成済み: scratchpad/rust03/ownership-lab/src/bin/u05_*〜u10_*（全99本）、out/。E0106/E0597/E0515/E0621/E0716/E0521/E0072/E0782/E0373/E0277/E0594、RefCell panic。
- 横断事項: rustc 1.97.1 の RefCell 二重借用 panic は "RefCell already borrowed"。libSQL のトランザクション drop 挙動は第7巻で扱うと本文に書いた。

## Rust/04 設計 — 途中（3,888行）
- 完了: 表紙・巻頭・UNIT 01〜05（図11）。UNIT 06 はコードとエラー出力（E0603/E0616/E0583）取得済みで本文未記入。未着手: UNIT 07〜10、まとめ、用語ミニ辞典。
- 版: thiserror 2.0.21 / anyhow 1.0.104 / serde 1.0.229 / serde_json 1.0.154。1単元 650〜700 行（目安超過）。本文が未執筆の UNIT 06〜10 を参照している。
- RustNote 設計: UserId/NoteId は i64、NoteRepository は &self、メモリ版は Mutex、AppState は Arc<dyn NoteRepository + Send + Sync>。
- 素材: scratchpad/rust04/（parts/, design_lab/, notes_app/, errs/, out/, assemble.py で parts を連結し目印に実コード・出力を差し込む）

## next/03-react-basics.md — ほぼ完了（PDF 未生成）
- 追加: 章冒頭、サンプル9（Node 25 / React 19.3 で確認）、つまずき10（NG→OK＋実エラー文）、練習問題7。図13枚は統括が scratchpad/next03/figs/01〜13.html から <!--FIG03-NN--> 位置へ差し込み済み（CRLF 維持、id 重複なし）。
- 既存修正: Fragment 例の構文エラー、章番号参照、useMemo の参照先、useBooks の参照先、最終行の React Router→Next.js、行末コメント2。
- 未了: PDF 生成と図ページの目視。

## Rust/02 基礎 — 途中（3,631行）
- 完了: UNIT 01〜05（図10）。書きかけ: 巻頭（表紙と # 見出しのみ。「この巻で身に付けること」未記入、表紙の題・副題は仮）、UNIT 06（コード16本は実行済み、本文なし）。未着手: UNIT 07〜10、まとめ、用語ミニ辞典。
- UNIT 01〜03 の出力は手写し（照合未了）。
- 素材: scratchpad/rust02/（parts/, out/, snips/u01〜u06（.rs/.out/.err）, basics/, run.sh, fill.py, split.py, mask.py, preview.py）
- 横断事項: タイトル検証は chars().count()（SQLite length() と同基準）。定数 MAX_TITLE_CHARS = 80（usize）、PER_PAGE = 20。

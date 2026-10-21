# 中断時点の引き継ぎメモ

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

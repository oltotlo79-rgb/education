# Rust 教材 執筆ブリーフ（全巻共通）

リポジトリ: `C:/Users/oltot/Documents/git-projects/def`
設計書: `docs/superpowers/specs/2026-10-15-next-rust教材の充実-design.md`（**3章 共通の執筆ルール** と **5章 Rust/ の構成** を必ず読む）
旧PDFのテキスト抽出: `C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust_txt/<旧PDF名>.txt`
→ 単元の並び（学習地図の01〜10）と各単元の要旨だけを参考にする。旧PDFは定型文の反復・1行に潰したコード・中身のない表が多い。**文章もコードも引き写さず、全面的に書き直す。**

## 読者
プログラミング完全初心者〜Rust初心者。です・ます調。next/ 教材（`next/02-typescript-basics.md` 等）と同じ親切さ・細かさで書く。書き方の手本として `next/02-typescript-basics.md` の冒頭〜数百行と、任意の `▼ このコードがやること` / `解説N` ブロックを1つ読んでから書き始めること。

## ファイル冒頭（表紙ブロック。この形をそのまま使い、巻番号・題・副題だけ変える）
```html
<div style="page-break-after: always; text-align: center; margin: 0; padding: 0;">
<img src="assets/cover.jpg" alt="表紙" style="width: 100%; max-height: 175mm; object-fit: cover; border-radius: 10px;">
<div style="margin-top: 14px; font-size: 11px; letter-spacing: 0.18em; color: #c2410c; font-weight: 700;">VOLUME 03 / RUST BACKEND MASTERY</div>
<div style="font-size: 26px; font-weight: 800; color: #0f172a; margin-top: 6px; line-height: 1.35;">巻タイトル</div>
<div style="font-size: 13px; color: #475569; margin-top: 8px;">副題（この巻で何ができるようになるかを1文で）</div>
</div>

# 第3巻: 巻タイトル
```

## 巻の骨組み（設計書 5.2）
- `## この巻で身に付けること`（到達目標・**学習地図の SVG 図**・前提知識（どの巻を先に読むか）・この巻の進め方・使うツールと版）
- `## UNIT 01: …` 〜 `## UNIT 10: …`
- `## この巻のまとめ`（チェックリスト）
- `## 用語ミニ辞典`（この巻で出た用語を表で）

## 単元の骨組み（設計書 5.3。見出しはこの名前で統一）
1. `### この単元のゴール`
2. `### たとえ話でつかむ`
3. `### 図で理解する`（インライン SVG を1〜2枚。設計書 3.3 の形式。marker 等の id は `ar-<巻2桁>-<連番>` で文書内一意）
4. `### まず動かす最小コード`（`#### 名前` → `> **▼ このコードがやること（先に日本語で）:**` → 完全なコード → 実行コマンド → `**▼ 実行結果**` → `#### ▼ コードを1つずつ分解して解説` → `##### 解説N`）
5. `### もう一歩: 実務に近い形`
6. `### よくあるエラーと読み方`（実際の rustc / cargo / 実行時のエラー全文 → 何が起きたか → 直し方。エラー番号 E0382 等があれば説明）
7. `### RustNote ではここで使う`（該当すれば。RustNote は教材を通して作るメモアプリ。下記「RustNote の共通設定」に合わせる）
8. `### 練習問題` と `### 解答と解説`（2〜3問。解答コードもコンパイル確認する）
9. `### まとめ`（3行程度）

分量: 1単元 250〜450 行、1巻 3,000〜4,500 行が目安。図は巻全体で 15 枚以上（学習地図＋各単元1〜2枚）。

## コードの規則
- コメントはコードの「上の行」に書く（行末コメントにしない）。rustfmt 済みの見た目にする（1行に複数文を詰めない）。
- Rust は **rustc 1.97 / edition 2024** で実際にコンパイル・実行して確認する。作業はスクラッチ領域 `C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust<巻2桁>/` で行う（`cargo new` した1プロジェクトで `src/bin/*.rs` や `examples/*.rs` を使い分けると速い）。
- 実行結果・エラーメッセージは**実際の出力**を貼る。絶対パスやユーザー名（`C:\Users\...`）は `.../` 等に伏せる。
- 外部クレートを使うときは版を明記し（`cargo add` で入った版）、Cargo.toml 断片も載せる。
- 環境は Windows 11 (ARM64, `aarch64-pc-windows-msvc`)。読者向けの説明は Windows + PowerShell を主、WSL/macOS/Linux を補足とする。
- Topcoat を扱う巻は、統括から渡される検証済みリファレンス `scratchpad/ref/topcoat-0.10-reference.md` に書かれた API だけを使う。

## RustNote の共通設定（例を書くときに合わせる。最終的なコードは別担当が `Rust/rustnote/` に実装する）
- 機能: メールアドレス+パスワードで登録/ログイン/ログアウト。ログインした人だけが自分のノートを一覧・作成・詳細・編集・完了切替（done）・削除・検索・ページ送りできる。
- テーブル: `users(id, email UNIQUE, password_hash, created_at)` / `notes(id, user_id → users.id, title CHECK 1〜80文字, body, done, version, created_at, updated_at)` / `sessions(token_hash, user_id, expires_at, created_at)` / `schema_migrations(version, applied_at)`
- 型の名前: `UserId`, `NoteId`, `Email`, `NoteTitle`（`NoteTitle::parse` で 1〜80 文字を検証）, `Note`, `NewNote`, `AppError`, `NoteRepository`, `AppState`, `Config`
- URL: `GET /health`, `GET|POST /signup`, `GET|POST /login`, `POST /logout`, `GET /notes`（`?q=` 検索・ページ送り）, `POST /notes`, `GET /notes/{id}`, `GET /notes/{id}/edit`, `POST /notes/{id}/edit`, `POST /notes/{id}/toggle`, `POST /notes/{id}/delete`。成功したフォーム送信は 303 で GET へリダイレクト（PRG）。他人のノートは 404。
- DB: Turso / libSQL（開発はローカルファイル）。パスワードは Argon2id。公開先は Fly.io（`0.0.0.0:8080`）。

## 書き方の進め方（推奨）
1. 単元ごとに `scratchpad/rust<巻>/parts/unit01.md` … のように分けて書き、コードはその都度コンパイルして結果を貼る。
2. 最後に表紙・巻頭・各単元・巻末を連結して `Rust/<ファイル名>.md` を作る（UTF-8、LF）。
3. `cd C:/Users/oltot/Documents/git-projects/def && PYTHONIOENCODING=utf-8 python _build_pdfs.py "Rust/<ファイル名>.md"` で `Rust/pdf/` に PDF を作る。PyMuPDF（`import fitz`）で表紙・学習地図・各単元の図を含むページを PNG にしてスクラッチ領域へ保存し、Read で目視確認する。崩れ・文字のはみ出し・矢印のズレ・Markdown が生で出ている箇所を直して再生成する。

## 禁止事項
- 担当ファイル以外のリポジトリ内ファイルを編集しない（`Rust/pdf/<自分の巻>.pdf` の生成は可）。旧 `Rust/*.pdf` は消さない（統括が処理する）。
- git の add/commit/push をしない。

## 報告（最後のメッセージ）
作成したファイル名、行数、図の数、コード例の数（うち実行確認済みの数）、PDF ページ数、検証できなかった箇所と理由、他の巻との整合で統括が知っておくべきこと。

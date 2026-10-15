# 設計書: next/ と Rust/ 教材の充実（図・サンプルコード・初心者向け解説の大幅追加）

作成日: 2026-10-15

## 1. 目的・背景

ユーザー要望:「next と Rust 内の教育資料をもっと親切に詳細に分かりやすく、画像やサンプルコードも増やして充実させて」。

現状調査の結果:

| 対象 | 現状 | 課題 |
|---|---|---|
| `next/` | Markdown 12章（約43,000行）+ PDF。`▼ このコードがやること` + `解説N` 形式が定着。図はインライン HTML の `<div>` 表が中心 | 「絵」としての図（概念図・流れ図・画面イメージ）が少ない。動かして確かめる小さな例、NG→OK 比較、練習問題が章によってばらつく |
| `Rust/` | PDF 14巻のみ。**ソース（Markdown/HTML）が存在しない** | 中身がテンプレート生成の定型文中心（「この行が受け取る値…を確認します」の反復、1行に潰したコード、汎用の owner/borrow 箱図）。Topcoat は 0.5.0 前提だが最新は 0.10 系で破壊的変更あり |

ユーザー決定事項（2026-10-15）:
- Rust 教材は **Markdown ソースを新規作成して全面書き直し**、PDF を再生成する。
- Topcoat は **最新 0.10 系に更新**する（0.5.0 固定はやめる）。

## 2. 成果物

### 2.1 next/
- `next/00〜11-*.md` の全12章に追記・改善（既存内容は誤り修正以外は削除しない）。
- `next/pdf/*.pdf` を再生成。

### 2.2 Rust/
- `Rust/00〜13_*.md` を新規作成（14巻）。ファイル名は既存PDF名を踏襲。ただし Topcoat の版を含む名前は更新:
  - `01_環境構築_Windows_WSL_Rust_Topcoat_Turso_Fly.md`（変更なし）
  - `06_Web基礎_HTTPとTopcoat_0_5完全ガイド` → `06_Web基礎_HTTPとTopcoat_0_10完全ガイド.md`
- `Rust/assets/cover.jpg`: 既存PDF全巻共通の表紙画像を抽出・JPEG化したもの。
- `Rust/rustnote/`: 教材で作る RustNote アプリの**実際にビルド・テストが通る完全なソース**（Cargo.toml、src、migrations、tests、Dockerfile、fly.toml、CI 定義）。第9・10・12・13巻のコードはここから引用し、教材とコードを一致させる。
- PDF の出力先は next/ と同じく `Rust/pdf/`。旧 `Rust/*.pdf`（ルート直下）は削除して置き換える。
- PDF 生成は既存の `_build_pdfs.py`（python-markdown + Chrome headless）をそのまま使う。

## 3. 共通の執筆ルール（全ファイル厳守）

### 3.1 読者と文体
- 対象: プログラミング完全初心者。です・ます調。
- 専門用語は初出で `用語（読み：意味）` と括弧書きするか、直後に `> **用語:** …` で1行定義する。
- 「なぜそうするのか」を必ず書く。手順だけを並べない。
- 抽象的な説明の前に、身近な**たとえ話**を置く。

### 3.2 サンプルコードの形式（既存 next/ の規約を踏襲）
```
#### [コード例の名前]

> **▼ このコードがやること（先に日本語で）:** [専門用語を避けて2〜3文]

​```言語
// 日本語コメントはコードの「上の行」に書く（行の右横には書かない）
コード
​```

**▼ 実行結果**（実行できる例は必ず付ける）
​```text
実際の出力
​```

#### ▼ コードを1つずつ分解して解説

##### 解説1: [塊のタイトル]
​```言語
（該当部分の抜粋）
​```
- 平易な説明（箇条書き）
> **用語:** …
```
- 見出しレベル: `##` 節 / `###` 小節 / `####` コード例 / `#####` 解説N。
- コードは省略記号（`...`）で誤魔化さず、**そのままコピーして動く完全な形**を基本とする（抜粋である場合は明記する）。
- 1行に複数文を詰め込まない。rustfmt / prettier で整形された見た目にする。

### 3.3 図（「画像」）の形式
- 図はインライン SVG で描き、キャプション付きの `<div>` で包む。PDF でも Markdown でも表示される。
- 形式（実際に PDF 生成で表示確認済み）:
```html
<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-<章>-<連番>" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
…図形…
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図X-Y: キャプション</div>
</div>
```
- 規則:
  - HTML ブロックの**前後に空行**、ブロックの**内部には空行を入れない**、開始タグは行頭（インデントしない）。
  - `marker` などの `id` は**文書内で一意**（例: `ar-04-07`）。同じ id を使い回すと矢印が別の図の定義で描かれてズレる。
  - 文字は 12px 以上。箱からはみ出さないよう短く。テキスト中の `<` `&` は `&lt;` `&amp;`。
  - 色は既存教材に合わせる（青 `#3b82f6/#1e40af`、橙 `#f97316/#c2410c`、緑 `#10b981/#047857`、灰 `#475569/#64748b`、背景の淡色 `#eff6ff/#fff7ed/#ecfdf5/#f8fafc`）。
- 図の種類: 概念図（たとえ話の絵）、処理の流れ（矢印つきフロー）、メモリ・データ構造図、Before/After 比較、**画面イメージ**（ブラウザ枠やターミナル枠の中に、実際に表示される画面を描いたもの）。
- 既存の HTML `<div>` 図はそのまま残す。

### 3.4 囲み記事
- `> **つまずきポイント:** …`（よくある勘違い。可能なら NG 例と OK 例のコードを並べる）
- `> **ヒント:** …` / `> **用語:** …` / `> **注意:** …`

### 3.5 正確性
- next/: TypeScript/JavaScript の単体サンプルは Node 25（`node file.ts` の型除去実行）等で実行して結果を貼る。React/Next.js のコードは Next.js 15+ App Router の現行仕様に合わせる。
- Rust/: すべての Rust コードは rustc 1.97 / edition 2024 で**実際にコンパイルし**、実行結果・コンパイルエラーは**実際の出力**を貼る（ユーザー名などの絶対パスは伏せる）。Topcoat は 0.10 系の検証済みリファレンス（調査結果）に書かれた API だけを使い、存在しない API を作らない。
- 外部サービス（Turso / Fly.io / Vercel / Supabase）の手順は公式ドキュメントで現行の手順を確認する。

### 3.6 PDF 確認
- 各ファイルは `python _build_pdfs.py <md>` で PDF 化し、新しく追加した図のページを画像にして目で確認する（崩れ・はみ出し・矢印ズレがないこと）。

## 4. next/ の章ごとの追加目標

各章で以下を**最低限**追加する（章の性質に応じて配分は調整してよい。既存と重複する内容は追加しない）:

| 項目 | 目標 |
|---|---|
| 新しい SVG 図 | 8枚以上（うち画面イメージを1枚以上。概念図・流れ図を中心に） |
| 新しいサンプルコード | 6個以上（小さく動かして確かめる例＋実行結果＋分解解説） |
| つまずきポイント | 5個以上（NG→OK のコード比較を含む） |
| 章冒頭「この章のゴールと地図」 | 無ければ追加（図つき） |
| 章末「練習問題（解答・解説つき）」 | 5問以上。既存の確認問題があれば解答・解説を補う |

- 新しい内容は、関連する既存の節の中や直後に挿入する（章末にまとめて押し込まない）。
- 見出し番号の体系は崩さない。新しい節には既存番号の枝番（例: `### 3.2.1`）を使うか、番号なしの見出しにする。

## 5. Rust/ の構成

### 5.1 巻の構成（14巻 × 10単元。単元は旧PDFの学習地図を踏襲し、内容は全面的に書き直す）

| 巻 | テーマ |
|---|---|
| 00 | 学習ガイド: コンピュータ・OS・ファイル・端末・ソースから実行・ブラウザとHTTP・サーバーとポート・DB・認証・コンパイラエラー・学習サイクル |
| 01 | 環境構築: PowerShell・Git・VS Code/rust-analyzer・rustup/Cargo・MSVC Build Tools・Cargo品質コマンド・WSL2/Docker・Topcoat 0.10とCLI・Turso CLI・flyctl |
| 02 | Rust基礎: 変数/mut/シャドーイング・数値/bool/char・関数/式・制御構文・String/str・タプル/配列/スライス/Vec・struct/impl・enum/Option・Result/?・HashMap/Iterator/closure/module |
| 03 | 所有権・借用・ライフタイム・Box/Rc/RefCell/Arc/Mutex・Send/Sync |
| 04 | 設計: newtype・trait・generics・dyn Trait・エラー型・crate/module/workspace・features・Serde/DTO・設定と秘密・テスト/rustdoc |
| 05 | async/Tokio・JoinSet/select!・channel・同期プリミティブ・timeout/shutdown・spawn_blocking・HTTP client/retry・tracing・background job |
| 06 | HTTP基礎と Topcoat 0.10 完全ガイド（ページ・ルーティング・パラメータ・view!/component・フォーム・共有状態・アセット・runtime/shard/procedure・エラー/cookie/session） |
| 07 | Turso/libSQL・SQL（CREATE/SELECT/INSERT/UPDATE/DELETE・index・transaction・migration・Rust SDK・embedded replica・Repository・運用） |
| 08 | 認証認可: 脅威モデル・Argon2id・登録/ログイン・セッション・cookie/CSRF・認可/IDOR・Better Auth比較・OIDC/MFA・セキュリティテスト |
| 09 | RustNote 要件・画面設計・ドメインモデル・プロジェクト構成・Cargo.toml・Config/AppState・migration/seed・ルート一覧・デザイン/アセット・health |
| 10 | RustNote CRUD 完全実装（一覧・作成・詳細・編集・toggle・削除・検索・ページング・flash・レスポンシブ/a11y） |
| 11 | 品質保証: unit/integration/HTTP テスト・property test/fuzz・E2E・セキュリティ回帰・可観測性・ベンチ・負荷・CI/供給網 |
| 12 | Fly.io 公開: release build・multi-stage Dockerfile・アセット・fly.toml・secrets・health・migration・deploy/logs・scale/cost・rollback/backup |
| 13 | 完全コード（`Rust/rustnote/` の全ファイルを解説付きで掲載）・総合演習と解答・用語集・索引 |

### 5.2 各巻の骨組み
```
（表紙ブロック: cover.jpg + VOLUME NN / RUST BACKEND MASTERY + 巻タイトル + サブタイトル）
# 第N巻: タイトル
## この巻で身に付けること（到達目標・学習地図の図・前提知識・進め方）
## UNIT 01: …
…
## UNIT 10: …
## この巻のまとめ（チェックリスト）
## 用語ミニ辞典（この巻で出た用語）
```
表紙ブロックは `<div style="page-break-after: always; …"><img src="assets/cover.jpg" …>…</div>`（PDF 表示確認済み）。

### 5.3 各単元（UNIT）の骨組み
1. `### この単元のゴール`（3〜4項目）
2. `### たとえ話でつかむ`
3. `### 図で理解する`（SVG 1〜2枚）
4. `### まず動かす最小コード`（完全なコード＋実行コマンド＋実行結果＋分解解説）
5. `### もう一歩: 実務に近い形`（コード＋解説）
6. `### よくあるエラーと読み方`（実際のコンパイラ/実行時エラー全文 → 原因 → 直し方）
7. `### RustNote ではここで使う`（該当する場合）
8. `### 練習問題`（2〜3問）と `### 解答と解説`
9. `### まとめ`（3行）

分量の目安: 1単元 250〜450 行、1巻 3,000〜4,500 行。旧PDFの定型文（「入力と前提を確認し、境界を曖昧にしない」等の反復）は引き継がない。

## 6. 作業の進め方
1. Topcoat 0.10 の API を docs.rs・GitHub・crate ソースで調べ、手元でコンパイルして検証したリファレンスを作る。libsql・argon2 が手元（Windows ARM64）でビルドできるかも確認する。
2. `Rust/rustnote/` を作り、`cargo build` / `cargo test` / `cargo clippy` を通す。
3. 章・巻ごとにサブエージェントへ並列で割り当てる（next 12章、Rust 14巻）。RustNote に依存する第9・10・12・13巻は手順2の後に着手する。
4. 全PDFを再生成し、追加した図を抜き取りで目視確認する。
5. コミットしてプッシュする（ユーザー指示）。

## 7. 範囲外
- `react_native/` と `docs/` は変更しない。
- Topcoat 0.10 に存在しない機能（ロードマップ上の認証ヘルパー等）は「未提供」と明記し、自前実装で説明する。

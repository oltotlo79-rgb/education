# 統一の決定事項（2026-10-17 再開時に統括が決定。全担当が従う）

## next/（書籍管理アプリ）
- 読書ステータスの値: `want_to_read`（読みたい）/ `reading`（読書中）/ `completed`（読了）。`unread` / `finished` は使わない。
- メモの列名: `memo`。第5章のテーブル定義も `memo` にする（`notes` は使わない）。
- 一覧ページ: `/`（`app/page.tsx`）。新規登録 `/books/new`、詳細 `/books/[id]`、編集 `/books/[id]/edit`。「一覧に戻る」は `/` へ。
- 著者 `author`: 入力必須（テーブルは NOT NULL、フォームでも必須チェック。空を null にして送らない）。
- Next.js: 2026年10月時点の最新安定版 16.x（検証は 16.4.1）。`create-next-app` では推奨既定を使わず「カスタマイズ」を選び、**Cache Components は No** を選ぶよう案内する（Yes だと教材のトップページ等が `next build` で失敗するため。理由も一言書く）。
- Supabase: 新しい API キー（Publishable key `sb_publishable_...` / Secret key）を主に説明し、旧 anon / service_role は「2026年末に廃止予定の旧方式」と注記。新規テーブルには GRANT が必要（第5章で説明）。
- Tailwind: create-next-app が入れる v4 を前提（`@import "tailwindcss";`）。v3 の書き方は注記扱い。

## Rust/
- Rust の版: Topcoat 0.10 は Rust 1.98 以上が必要。本書の基準は「Rust 1.98 以上、動作確認は 1.99.0」。`Cargo.toml` の `rust-version = "1.98"`。手元では `cargo +1.99.0 ...` または作業プロジェクトに `rust-toolchain.toml`（channel = "1.99.0"）を置いて実行する（既定ツールチェーンは変更しない）。標準ライブラリだけの例の出力は 1.97 と同じことが多いが、版の表記とエラー文は 1.99 で取り直したものに合わせる。
- Topcoat: `=0.10.0`（CLI も 0.10.0）。`topcoat::start` は環境変数 `HOST` / `PORT` を読む（既定 127.0.0.1:3000）。Fly.io では `HOST=0.0.0.0`、`PORT=8080`。
- RustNote のローカル URL: `http://127.0.0.1:3000`。
- Windows ARM64 の注意: libsql（既定 features）は clang（LLVM）が必要。Topcoat の Tailwind 機能は ARM64 用バイナリがないため **RustNote では使わず、素の CSS を asset で配信**する。
- RustNote の定数: `MAX_TITLE_CHARS = 80`、`PER_PAGE = 20`。タイトル長は `chars().count()`（SQLite の `length()` と同じ「文字数」）。`UserId` / `NoteId` は `i64` の newtype。

## 追加（2026-10-17）next/ のフォルダ構成
- **`src/` フォルダは使わない**。`app/`・`components/`・`lib/`・`types/` をプロジェクト直下に置く（create-next-app の「`src/` directory?」は No）。import エイリアスは `@/*` → `./*`（例: `@/components/BookCard`、`@/lib/supabase/server`）。
- Supabase のクライアントは第6章で `lib/supabase/server.ts`（Server Component / Server Action 用）と `lib/supabase/client.ts`（Client Component 用）の2つを作る（第7・8章はこの2つを import 済み）。第5章の動作確認用の1ファイル版は「第6章で2つに分ける」と注記。

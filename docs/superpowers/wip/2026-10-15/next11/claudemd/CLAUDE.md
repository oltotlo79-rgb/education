# 書籍管理アプリ（my-books-app）

読んだ本・読みたい本を記録する個人用の Web アプリ。

## 技術スタック
- Next.js（App Router）+ TypeScript + Tailwind CSS
- データベースと認証: Supabase（books テーブル）

## よく使うコマンド
- 開発サーバー: `npm run dev`
- Lint: `npm run lint`
- 型チェック: `npx tsc --noEmit`
- テスト: `npx vitest run`

## フォルダの役割
- `src/app/`: ページ（フォルダ名がそのまま URL になる）
- `src/components/`: 画面の部品（BookCard、StatusBadge など）
- `src/lib/`: Supabase クライアントと共通の関数
- `src/types/`: 型定義（Book 型など）

## コードの決まり
- 書籍のステータスは `reading` / `completed` / `want_to_read` の3つだけ
- Supabase からのデータ取得は Server Component で行う
- `any` 型は使わない（分からないときは `unknown` にして絞り込む）

## 作業の進め方
- 変更したら `npm run lint` と `npx tsc --noEmit` を実行し、結果を報告する
- 頼まれた範囲のファイルだけを変更する。ほかに直したい所があれば提案だけする
- `.env.local` は読まない・書かない
- `git push` はしない（人間が行う）

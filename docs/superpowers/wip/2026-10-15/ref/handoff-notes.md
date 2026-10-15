# 中断時点の引き継ぎメモ（2026-10-15）

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

## next/07-crud-list-create.md — **完成（2026-10-16 に main へ反映済み、コミット e90d823）**
- 仕上げで行ったこと: 連続する囲みの結合7か所を `<!-- -->` で分離、太字記号の閉じ忘れ1か所を修正、PDF 再生成（143p）と図13枚の目視確認。
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

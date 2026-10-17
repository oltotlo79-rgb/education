from pathlib import Path

MD = Path(r"C:/Users/oltot/Documents/git-projects/def/next/06-project-setup.md")
s = MD.read_text(encoding="utf-8")

R = [
    # 1.2 なぜ推奨設定を使わないのか
    ("> **なぜ推奨設定を使わないのか:** 推奨設定には「`src/` フォルダを作らない」「Cache Components（キャッシュ・コンポーネンツ）を有効にする」という2つが含まれています。本書のコードは `src/` フォルダがある前提で、しかも Cache Components が有効だと第7章のトップページが `next build`（本番用ビルド）で失敗します。そのため、1つずつ自分で答える「カスタマイズ」を選びます。",
     "> **なぜ推奨設定を使わないのか:** 推奨設定には「Cache Components（キャッシュ・コンポーネンツ）を有効にする」という項目が含まれています。Cache Components が有効だと、第7章で作るトップページ（サーバーで Supabase からデータを取るページ）が `next build`（本番用ビルド）で失敗します。そのため、1つずつ自分で答える「カスタマイズ」を選び、Cache Components だけは No にします。"),
    # table rows
    ("| ① | **recommended defaults**（推奨設定を使うか） | **No, customize settings** | 推奨設定は `src/` なし・Cache Components ありで、本書の手順と合いません。1つずつ自分で答えます。 |",
     "| ① | **recommended defaults**（推奨設定を使うか） | **No, customize settings** | 推奨設定は Cache Components が有効になっていて、本書の手順と合いません。1つずつ自分で答えます。 |"),
    ("| ⑥ | **`src/` directory** | **Yes** | ソースコードを `src/` ディレクトリにまとめることで、設定ファイル（`package.json`, `next.config.ts` など）とソースコードが明確に分離されます。第7章以降のファイルの場所（`components/…`）もこの前提です。**既定値は No なので、必ず Yes に切り替えてください。** |",
     "| ⑥ | **`src/` directory** | **No**（既定値のまま） | `src/` フォルダを作らず、`app/`・`components/`・`lib/`・`types/` をプロジェクトの直下（`package.json` と同じ場所）に置きます。フォルダが1段浅くなるので、どのファイルがどこにあるか見通しやすくなります。第7章以降のファイルの場所（`components/BookCard.tsx`、`app/page.tsx` など）もこの前提です。 |"),
    ("| ⑨ | **Import alias** | No（デフォルト） | デフォルトの `@/*` エイリアスをそのまま使います。`@/components/Header` のように `src/` ディレクトリ内のファイルを短いパスでインポートできます。 |",
     "| ⑨ | **Import alias** | No（デフォルト） | デフォルトの `@/*` エイリアスをそのまま使います。`@/components/Header` のように、プロジェクト直下のファイルを短いパスでインポートできます。 |"),
    # 1.2.1
    ("> 最初の質問で何も考えずに Enter を押すと、推奨設定が選ばれ、残りの質問は表示されずにすぐプロジェクトが作られます。見た目は成功しますが、`src/` フォルダが無く、Cache Components も有効になっています。",
     "> 最初の質問で何も考えずに Enter を押すと、推奨設定が選ばれ、残りの質問は表示されずにすぐプロジェクトが作られます。見た目は成功しますが、**Cache Components が有効**になっています（匿名フィードバックも有効になります）。この章のうちは何も困らないので、第7章で `npm run build` するまで気づかないのが厄介な点です。"),
    ("直し方は「作り直す」のがいちばん確実です。まだ何も書いていないので、失うものはありません。\n\nNG（推奨設定のまま作ったプロジェクトを使い続ける）:\n\n```text\nbook-management/app/page.tsx       ← src/ が無い\nbook-management/next.config.ts     ← cacheComponents: true が入っている\n```\n\nOK（フォルダを消して、カスタマイズで作り直す）:",
     "推奨設定で作ってしまったかどうかは、`next.config.ts` を開けば分かります。\n\nNG（推奨設定のまま作ったプロジェクトを使い続ける）:\n\n```text\nbook-management/next.config.ts の中に、次の2行がある\n  cacheComponents: true,\n  partialPrefetching: true,\n```\n\n直し方は2つあります。1つ目は、この2行を消すことです（詳しくは 1.7 で説明します）。2つ目は「作り直す」ことで、まだ何も書いていないので失うものはありません。\n\nOK（フォルダを消して、カスタマイズで作り直す）:"),
    # 1.2.2
    ("# 本書の答え（TypeScript / ESLint / Tailwind / src あり / App Router /", "# 本書の答え（TypeScript / ESLint / Tailwind / src なし / App Router /"),
    ("npx create-next-app@latest book-management --ts --eslint --tailwind --src-dir --app --no-react-compiler --no-cache-components --import-alias \"@/*\" --use-npm --agents-md --no-agent-feedback",
     "npx create-next-app@latest book-management --ts --eslint --tailwind --no-src-dir --app --no-react-compiler --no-cache-components --import-alias \"@/*\" --use-npm --agents-md --no-agent-feedback"),
    ("```bash\n--ts --eslint --tailwind --src-dir --app\n```", "```bash\n--ts --eslint --tailwind --app\n```"),
    ("- `--ts` は ② TypeScript に Yes、`--eslint` は ③ で ESLint、`--tailwind` は ⑤ に Yes、`--src-dir` は ⑥ に Yes、`--app` は ⑦ に Yes と答えたのと同じです。",
     "- `--ts` は ② TypeScript に Yes、`--eslint` は ③ で ESLint、`--tailwind` は ⑤ に Yes、`--app` は ⑦ に Yes と答えたのと同じです。"),
    ("```bash\n--no-react-compiler --no-cache-components --no-agent-feedback\n```", "```bash\n--no-src-dir --no-react-compiler --no-cache-components --no-agent-feedback\n```"),
    ("- `--no-` を頭に付けると「その機能は使わない」という意味になります。④ React Compiler、⑧ Cache Components、⑪ 匿名フィードバックをすべて No にしています。",
     "- `--no-` を頭に付けると「その機能は使わない」という意味になります。⑥ `src/` フォルダ、④ React Compiler、⑧ Cache Components、⑪ 匿名フィードバックをすべて No にしています。"),
    # 1.2 alias box
    ("> パスエイリアスを使うと、`@/components/Header` のように `src/` を起点とした絶対インポート（プロジェクトのルートからのパス）で書けるようになります。",
     "> パスエイリアスを使うと、`@/components/Header` のようにプロジェクトのルート（`package.json` がある場所）を起点とした絶対インポートで書けるようになります。"),
    # 1.5 tree
    ("  src/                  ... アプリのソースコードを置くフォルダ\n    app/                ... App Router のルートディレクトリ。フォルダ構成がそのままURLになる\n      favicon.ico       ... ブラウザのタブに表示されるアイコン\n      globals.css       ... グローバル CSS（Tailwind CSS を読み込む）\n      layout.tsx        ... ルートレイアウト（全ページ共通の枠）\n      page.tsx          ... トップページ（ / にアクセスしたときの表示）\n",
     "  app/                  ... App Router のルートディレクトリ。フォルダ構成がそのままURLになる\n    favicon.ico         ... ブラウザのタブに表示されるアイコン\n    globals.css         ... グローバル CSS（Tailwind CSS を読み込む）\n    layout.tsx          ... ルートレイアウト（全ページ共通の枠）\n    page.tsx            ... トップページ（ / にアクセスしたときの表示）\n"),
    # tsconfig
    ("      // @/foo は src/foo を指す\n      \"@/*\": [\"./src/*\"]", "      // @/foo はプロジェクト直下の foo を指す（./ は tsconfig.json がある場所）\n      \"@/*\": [\"./*\"]"),
    # 4.4 text
    ("1.2 の質問⑨で「`@/*` をそのまま使う」と答えたので、`@/` は **`src/` フォルダの近道**として働きます。住所にたとえると、`@/` は「本社ビル（src）」という略称です。",
     "1.2 の質問⑨で「`@/*` をそのまま使う」と答えたので、`@/` は **プロジェクト直下（`package.json` がある場所）の近道**として働きます。住所にたとえると、`@/` は「本社ビル」という略称です。"),
    ("(A) のとおり、`@/` の置き換えは `tsconfig.json` の `\"paths\"` に書かれたルール（`\"@/*\": [\"./src/*\"]`）だけで決まります。",
     "(A) のとおり、`@/` の置き換えは `tsconfig.json` の `\"paths\"` に書かれたルール（`\"@/*\": [\"./*\"]`）だけで決まります。`./` は「`tsconfig.json` があるフォルダ」、つまりプロジェクト直下のことです。"),
    # layout explanations
    ("`@/` は `src/` ディレクトリのエイリアスです。これにより `../../components/Header` のように相対パスを書かなくて済みます。",
     "`@/` はプロジェクト直下を指すエイリアスです。これにより `../components/Header` のように相対パスを書かなくて済みます。"),
    ("`@/` は `src/` の近道（パスエイリアス）なので、`@/components/Header` は `components/Header` を指します。",
     "`@/` はプロジェクト直下の近道（パスエイリアス）なので、`@/components/Header` は `components/Header` を指します。"),
    ("`src/` を起点にできるので、ファイルを移動してもインポート文を直さずに済みます。", "プロジェクト直下を起点にできるので、ファイルを移動してもインポート文を直さずに済みます。"),
    ("- どちらも `@/`（= `src/`）から始まるパスエイリアスを使い、相対パスより読みやすく書いています。", "- どちらも `@/`（= プロジェクト直下）から始まるパスエイリアスを使い、相対パスより読みやすく書いています。"),
]
for a, b in R:
    n = s.count(a)
    assert n == 1, (n, a[:60])
    s = s.replace(a, b)
MD.write_text(s, encoding="utf-8")
print("ok", len(R))

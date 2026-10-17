# 第6章: プロジェクトセットアップ — 書籍管理アプリの土台を作る

> いよいよ実際のアプリ開発が始まります！この章では、書籍管理アプリの「土台」（プロジェクトの雛形、フォルダ構成、初期設定）を作ります。

### この章で行うこと

ここまでの章で学んだ技術（TypeScript、React、Next.js、Supabase）を組み合わせて、実際のプロジェクトを作成します。料理に例えると、**材料を揃えて下ごしらえをする段階**です。包丁やまな板、調味料を準備しておくのと同じで、後でスムーズに料理（コード）が書けるように、まずは作業環境を整えます。

1. **Next.js プロジェクトの作成** — `npx create-next-app` コマンド（プロジェクトの雛形を自動生成するコマンド）を実行
2. **Tailwind CSS の理解** — Tailwind CSS（テールウィンドCSS：HTMLのクラス名でスタイルを直接指定する CSSフレームワーク）の使い方
3. **フォルダ構成の設計** — どのファイルをどこに置くか、プロジェクト全体の設計図を決める
4. **Supabase との接続設定** — アプリからデータベースに接続するための設定ファイルを作成
5. **型定義の作成** — 書籍データの「Book型」をTypeScriptで定義

この章を終えると、ブラウザで開発サーバー（Development Server：開発中にアプリを確認するためのローカルサーバー。`http://localhost:3000` でアクセスできる）にアクセスし、ヘッダー付きのトップページが表示される状態になります。

> **用語のミニ解説：**
> - **ビルド（build）**: ソースコード（人間が書いたコード）を、ブラウザや Node.js が実行できる形式に変換する作業のこと。TypeScript を JavaScript に変換したり、複数ファイルを1つにまとめたりします。
> - **devサーバ（開発サーバー）**: コードを書きながら動作確認するためのローカル（自分のPC上）で動くWebサーバー。ファイルを保存すると自動的にブラウザの表示が更新されます。
> - **依存関係（dependencies）**: あなたのアプリが動くために必要な外部のライブラリ（React や Next.js など）のこと。`package.json` に一覧が書かれます。
> - **環境変数（environment variable）**: アプリの外側からプログラムに渡す設定値。コードに直接書きたくない秘密情報（API キーなど）や、開発環境と本番環境で値を変えたい設定に使います。

> **ポイント：** この章はコードを書く量が多いですが、一つ一つのファイルの役割を理解しながら進めてください。「なぜこのファイルが必要なのか」を意識すると、後の章で迷わなくなります。

### この章のゴールと地図

家を建てるときは、いきなり壁紙を選んだりしません。まず**土地をならし（雛形づくり）**、**資材を運び込み（パッケージ追加）**、**部屋割りを決め（フォルダ構成）**、**水道や電気を引き（Supabase との接続）**、**設計図の寸法を決め（型定義）**、**玄関と看板を付けて（レイアウトとヘッダー）**、最後に**明かりをつけて確認（開発サーバー起動）**します。この章でやることは、まさにこの順番です。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-01" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="30" width="150" height="80" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="95" y="56" text-anchor="middle" font-size="14" font-weight="bold" fill="#1e40af">① 雛形を作る</text>
<text x="95" y="77" text-anchor="middle" font-size="12" fill="#1e40af">create-next-app</text>
<text x="95" y="97" text-anchor="middle" font-size="12" fill="#64748b">節1・節2</text>
<rect x="200" y="30" width="150" height="80" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="275" y="56" text-anchor="middle" font-size="14" font-weight="bold" fill="#1e40af">② 部品を足す</text>
<text x="275" y="77" text-anchor="middle" font-size="12" fill="#1e40af">npm install</text>
<text x="275" y="97" text-anchor="middle" font-size="12" fill="#64748b">節3</text>
<rect x="380" y="30" width="150" height="80" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="455" y="56" text-anchor="middle" font-size="14" font-weight="bold" fill="#1e40af">③ 部屋割り</text>
<text x="455" y="77" text-anchor="middle" font-size="12" fill="#1e40af">フォルダを作る</text>
<text x="455" y="97" text-anchor="middle" font-size="12" fill="#64748b">節4</text>
<rect x="560" y="30" width="150" height="80" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="635" y="56" text-anchor="middle" font-size="14" font-weight="bold" fill="#c2410c">④ DBとつなぐ</text>
<text x="635" y="77" text-anchor="middle" font-size="12" fill="#c2410c">.env.local ほか</text>
<text x="635" y="97" text-anchor="middle" font-size="12" fill="#64748b">節5</text>
<rect x="560" y="170" width="150" height="80" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="635" y="196" text-anchor="middle" font-size="14" font-weight="bold" fill="#c2410c">⑤ 型を決める</text>
<text x="635" y="217" text-anchor="middle" font-size="12" fill="#c2410c">types/book.ts</text>
<text x="635" y="237" text-anchor="middle" font-size="12" fill="#64748b">節6</text>
<rect x="380" y="170" width="150" height="80" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="455" y="196" text-anchor="middle" font-size="14" font-weight="bold" fill="#047857">⑥ 外枠と看板</text>
<text x="455" y="217" text-anchor="middle" font-size="12" fill="#047857">layout と Header</text>
<text x="455" y="237" text-anchor="middle" font-size="12" fill="#64748b">節7・節8</text>
<rect x="200" y="170" width="150" height="80" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="275" y="196" text-anchor="middle" font-size="14" font-weight="bold" fill="#047857">⑦ 起動して確認</text>
<text x="275" y="217" text-anchor="middle" font-size="12" fill="#047857">npm run dev</text>
<text x="275" y="237" text-anchor="middle" font-size="12" fill="#64748b">節9</text>
<rect x="20" y="170" width="150" height="80" rx="10" fill="#f8fafc" stroke="#475569" stroke-width="2" stroke-dasharray="6 4"/>
<text x="95" y="198" text-anchor="middle" font-size="14" font-weight="bold" fill="#475569">ゴール</text>
<text x="95" y="219" text-anchor="middle" font-size="12" fill="#475569">ヘッダー付きの</text>
<text x="95" y="237" text-anchor="middle" font-size="12" fill="#475569">トップページが出る</text>
<line x1="170" y1="70" x2="198" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-01)"/>
<line x1="350" y1="70" x2="378" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-01)"/>
<line x1="530" y1="70" x2="558" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-01)"/>
<line x1="635" y1="110" x2="635" y2="168" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-01)"/>
<line x1="560" y1="210" x2="532" y2="210" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-01)"/>
<line x1="380" y1="210" x2="352" y2="210" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-01)"/>
<line x1="200" y1="210" x2="172" y2="210" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-01)"/>
<rect x="20" y="268" width="14" height="14" fill="#eff6ff" stroke="#3b82f6"/>
<text x="40" y="280" font-size="12" fill="#475569">土台づくり</text>
<rect x="130" y="268" width="14" height="14" fill="#fff7ed" stroke="#f97316"/>
<text x="150" y="280" font-size="12" fill="#475569">データの準備</text>
<rect x="250" y="268" width="14" height="14" fill="#ecfdf5" stroke="#10b981"/>
<text x="270" y="280" font-size="12" fill="#475569">画面づくりと確認</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図6-1: この章の地図（①→⑦の順に進み、最後にブラウザで確認する）</div>
</div>

**この章を読み終えたときにできるようになること（ゴール）:**

- `create-next-app` の質問に、意味を理解したうえで答えられる
- 生成されたファイルのうち「自分で書くもの」「設定ファイル」「触らないもの」を区別できる
- `@/components/Header` のような `@/` 付きの import が、どのファイルを指すか説明できる
- `.env.local` に書いた値が、どうやってコード（`process.env`）に届くか説明できる
- 開発サーバーを起動し、うまくいかないとき（ポート使用中・フォルダ違い・import ミスなど）に自分で直せる

> **ヒント:** 各節の最後に「つまずきポイント」を入れてあります。エラーが出たら、まずそのエラーメッセージで本章内を検索してみてください。同じメッセージの実例と直し方が見つかるはずです。

---

## 目次

1. [Next.js プロジェクトの作成](#1-nextjs-プロジェクトの作成)
2. [Tailwind CSS の基礎](#2-tailwind-css-の基礎)
3. [必要なパッケージのインストール](#3-必要なパッケージのインストール)
4. [プロジェクト構成の作成](#4-プロジェクト構成の作成)
5. [Supabase クライアントの設定](#5-supabase-クライアントの設定)
6. [型定義ファイルの作成](#6-型定義ファイルの作成)
7. [ルートレイアウトの設定](#7-ルートレイアウトの設定)
8. [共通ヘッダーコンポーネント](#8-共通ヘッダーコンポーネント)
9. [開発サーバーの起動と動作確認](#9-開発サーバーの起動と動作確認)
10. [データフローの全体像](#10-データフローの全体像)

---

## 1. Next.js プロジェクトの作成

### 1.1 create-next-app コマンドの実行

ターミナル（コマンドプロンプト、PowerShell、または VS Code のターミナル）を開き、プロジェクトを作成したいディレクトリに移動してから、以下のコマンドを実行します。

> **▼ このコードがやること（先に日本語で）:** Next.js アプリの「雛形（ひながた：最初の土台一式）」を1コマンドで自動生成します。`npx` は「PCにインストールしないで一時的にツールを実行する」仕組みで、ここでは `create-next-app`（プロジェクト作成ツール）の最新版を呼び出しています。`book-management` の部分が作られるフォルダ名なので、好きな名前に変えてもかまいません。各単語の意味は下のコメントを見てください。

```bash
# npx: ローカルにインストールしていないコマンドも一時的に実行できるツール
# create-next-app: Next.js プロジェクトの雛形を自動生成するパッケージ
# @latest: パッケージの「最新版」を使うという指定（@を使ってバージョン指定）
# book-management: 作成するプロジェクトのフォルダ名（好きな名前に変えてもOK）
npx create-next-app@latest book-management
```

> **`npx` とは？**
> `npx` は npm 5.2 以降に同梱されているコマンドで、パッケージをグローバルインストールせずに一時的に実行できます。`create-next-app@latest` は「最新版の create-next-app を使う」という意味です。プロジェクト作成のためだけに使うコマンドを、PCに永久にインストールせずに済むので便利です。

> **`npm` と `npx` の違い:**
> `npm install` はパッケージを「インストール」して使えるようにするコマンドで、`npx` はパッケージを「実行」するコマンドです。`create-next-app` はプロジェクト作成時に一度だけ使うので、インストールせずに `npx` で実行します。

> **注意（バージョン）:** この章の画面や出力は、2026年10月時点の最新版である **create-next-app 16.4.1 / Next.js 16.4.1** で実際に確かめたものです。Next.js 16 を動かすには **Node.js 20.9 以上**が必要です（第1章で入れた Node.js のバージョンは `node --version` で確認できます）。`@latest` は「その日の最新版」なので、時期によって質問の文言や数字が少し変わることがあります。

> **つまずきポイント: プロジェクト名に大文字や空白を入れてしまう**
>
> プロジェクト名（フォルダ名）は npm のパッケージ名にもなるため、**小文字・数字・ハイフン（`-`）だけ**にするのが決まりです。

NG（大文字が入っている）:

```bash
# 大文字の B と M が入っているので作成できない
npx create-next-app@latest BookManagement
```

**▼ 実行結果**

```text
Could not create a project called "BookManagement" because of npm naming restrictions:
    * name can no longer contain capital letters
```

NG（空白が入っている）:

```bash
# 空白（スペース）が入っているので作成できない
npx create-next-app@latest "book management"
```

**▼ 実行結果**

```text
Could not create a project called "book management" because of npm naming restrictions:
    * name can only contain URL-friendly characters
```

OK（小文字とハイフンだけ）:

```bash
# 小文字とハイフンだけなら作成できる
npx create-next-app@latest book-management
```

`name can no longer contain capital letters` は「名前に大文字はもう使えません」、`name can only contain URL-friendly characters` は「名前には URL に使える文字しか使えません」という意味です。どちらもフォルダはまだ作られていないので、名前を直してもう一度実行すれば大丈夫です。

コマンドを実行すると、いくつかの質問が対話形式で表示されます。以下のように回答してください。

### 1.2 各選択肢の詳細解説

最初に出てくるのは「**おすすめの設定（recommended defaults）をそのまま使いますか？**」という質問です。ここで Enter を押すと推奨設定が選ばれてしまいますが、本書では**推奨設定を使いません**。キーボードの **↓（下矢印）キー**で「**No, customize settings**（いいえ、自分で設定する）」に合わせてから Enter を押してください。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 214" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="6" width="700" height="202" rx="10" fill="#0f172a" stroke="#475569" stroke-width="2"/>
<rect x="10" y="6" width="700" height="28" rx="10" fill="#1e293b"/>
<rect x="10" y="24" width="700" height="10" fill="#1e293b"/>
<circle cx="30" cy="20" r="5" fill="#f87171"/><circle cx="48" cy="20" r="5" fill="#fbbf24"/><circle cx="66" cy="20" r="5" fill="#34d399"/>
<text x="360" y="25" text-anchor="middle" font-size="12" fill="#94a3b8">ターミナル（npx create-next-app@latest book-management を実行した直後）</text>
<text x="28" y="58" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#22d3ee">? </tspan><tspan fill="#f8fafc" font-weight="bold">Would you like to use the recommended Next.js defaults?</tspan><tspan fill="#94a3b8"> » - Use arrow-keys. Return to submit.</tspan></text>
<text x="28" y="80" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#e2e8f0">    Yes, use recommended defaults</tspan><tspan fill="#94a3b8"> - TypeScript, ESLint, No React Compiler,</tspan></text>
<text x="28" y="102" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#94a3b8">      Tailwind CSS, No src/ directory, App Router, Cache Components, AGENTS.md,</tspan></text>
<text x="28" y="124" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#94a3b8">      Agent feedback</tspan></text>
<text x="28" y="146" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#22d3ee">&gt;   </tspan><tspan fill="#22d3ee" text-decoration="underline">No, customize settings</tspan><tspan fill="#94a3b8"> - Choose your own preferences</tspan></text>
<rect x="24" y="131" width="184" height="22" rx="4" fill="none" stroke="#f97316" stroke-width="2"/>
<text x="40" y="176" font-size="12" fill="#fdba74">↑ ↓キーで「No, customize settings」に合わせて Enter を押す</text>
<text x="40" y="198" font-size="12" fill="#94a3b8">※ 推奨（Yes）を選ぶと Cache Components が有効になる → 第7章のトップページが build で失敗する</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図X: 最初の質問の画面イメージ（矢印キーで「カスタマイズ」を選ぶ）</div>
</div>

> **なぜ推奨設定を使わないのか:** 推奨設定には「`src/` フォルダを作らない」「Cache Components（キャッシュ・コンポーネンツ）を有効にする」という2つが含まれています。本書のコードは `src/` フォルダがある前提で、しかも Cache Components が有効だと第7章のトップページが `next build`（本番用ビルド）で失敗します。そのため、1つずつ自分で答える「カスタマイズ」を選びます。

カスタマイズを選ぶと、残りの質問が1つずつ表示されます。すべて答え終わると、ターミナルは次のようになります（これは実際の画面を元に描いた画面イメージです）。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 396" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="6" width="700" height="384" rx="10" fill="#0f172a" stroke="#475569" stroke-width="2"/>
<rect x="10" y="6" width="700" height="28" rx="10" fill="#1e293b"/>
<rect x="10" y="24" width="700" height="10" fill="#1e293b"/>
<circle cx="30" cy="20" r="5" fill="#f87171"/><circle cx="48" cy="20" r="5" fill="#fbbf24"/><circle cx="66" cy="20" r="5" fill="#34d399"/>
<text x="360" y="25" text-anchor="middle" font-size="12" fill="#94a3b8">ターミナル（質問にすべて答え終わったところ）</text>
<circle cx="30" cy="54" r="9" fill="#f97316"/>
<text x="30" y="58" text-anchor="middle" font-size="12" font-weight="bold" fill="#ffffff">1</text>
<text x="46" y="58" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#4ade80">√ </tspan><tspan fill="#f8fafc" font-weight="bold">Would you like to use the recommended Next.js defaults?</tspan><tspan fill="#94a3b8"> » </tspan><tspan fill="#22d3ee">No, customize settings</tspan></text>
<circle cx="30" cy="76" r="9" fill="#f97316"/>
<text x="30" y="80" text-anchor="middle" font-size="12" font-weight="bold" fill="#ffffff">2</text>
<text x="46" y="80" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#4ade80">√ </tspan><tspan fill="#f8fafc" font-weight="bold">Would you like to use </tspan><tspan fill="#60a5fa" font-weight="bold">TypeScript</tspan><tspan fill="#f8fafc" font-weight="bold">?</tspan><tspan fill="#94a3b8"> ... </tspan><tspan fill="#e2e8f0">No</tspan><tspan fill="#94a3b8"> / </tspan><tspan fill="#22d3ee" text-decoration="underline">Yes</tspan></text>
<circle cx="30" cy="98" r="9" fill="#f97316"/>
<text x="30" y="102" text-anchor="middle" font-size="12" font-weight="bold" fill="#ffffff">3</text>
<text x="46" y="102" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#4ade80">√ </tspan><tspan fill="#f8fafc" font-weight="bold">Which linter would you like to use?</tspan><tspan fill="#94a3b8"> » </tspan><tspan fill="#22d3ee">ESLint</tspan></text>
<circle cx="30" cy="120" r="9" fill="#f97316"/>
<text x="30" y="124" text-anchor="middle" font-size="12" font-weight="bold" fill="#ffffff">4</text>
<text x="46" y="124" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#4ade80">√ </tspan><tspan fill="#f8fafc" font-weight="bold">Would you like to use </tspan><tspan fill="#60a5fa" font-weight="bold">React Compiler</tspan><tspan fill="#f8fafc" font-weight="bold">?</tspan><tspan fill="#94a3b8"> ... </tspan><tspan fill="#22d3ee" text-decoration="underline">No</tspan><tspan fill="#94a3b8"> / </tspan><tspan fill="#e2e8f0">Yes</tspan></text>
<circle cx="30" cy="142" r="9" fill="#f97316"/>
<text x="30" y="146" text-anchor="middle" font-size="12" font-weight="bold" fill="#ffffff">5</text>
<text x="46" y="146" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#4ade80">√ </tspan><tspan fill="#f8fafc" font-weight="bold">Would you like to use </tspan><tspan fill="#60a5fa" font-weight="bold">Tailwind CSS</tspan><tspan fill="#f8fafc" font-weight="bold">?</tspan><tspan fill="#94a3b8"> ... </tspan><tspan fill="#e2e8f0">No</tspan><tspan fill="#94a3b8"> / </tspan><tspan fill="#22d3ee" text-decoration="underline">Yes</tspan></text>
<circle cx="30" cy="164" r="9" fill="#f97316"/>
<text x="30" y="168" text-anchor="middle" font-size="12" font-weight="bold" fill="#ffffff">6</text>
<text x="46" y="168" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#4ade80">√ </tspan><tspan fill="#f8fafc" font-weight="bold">Would you like your code inside a </tspan><tspan fill="#60a5fa" font-weight="bold">`src/` directory</tspan><tspan fill="#f8fafc" font-weight="bold">?</tspan><tspan fill="#94a3b8"> ... </tspan><tspan fill="#22d3ee" text-decoration="underline">No</tspan><tspan fill="#94a3b8"> / </tspan><tspan fill="#e2e8f0">Yes</tspan></text>
<circle cx="30" cy="186" r="9" fill="#f97316"/>
<text x="30" y="190" text-anchor="middle" font-size="12" font-weight="bold" fill="#ffffff">7</text>
<text x="46" y="190" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#4ade80">√ </tspan><tspan fill="#f8fafc" font-weight="bold">Would you like to use </tspan><tspan fill="#60a5fa" font-weight="bold">App Router</tspan><tspan fill="#f8fafc" font-weight="bold">? (recommended)</tspan><tspan fill="#94a3b8"> ... </tspan><tspan fill="#e2e8f0">No</tspan><tspan fill="#94a3b8"> / </tspan><tspan fill="#22d3ee" text-decoration="underline">Yes</tspan></text>
<circle cx="30" cy="208" r="9" fill="#f97316"/>
<text x="30" y="212" text-anchor="middle" font-size="12" font-weight="bold" fill="#ffffff">8</text>
<text x="46" y="212" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#4ade80">√ </tspan><tspan fill="#f8fafc" font-weight="bold">Would you like to use </tspan><tspan fill="#60a5fa" font-weight="bold">Cache Components</tspan><tspan fill="#f8fafc" font-weight="bold">?</tspan><tspan fill="#94a3b8"> ... </tspan><tspan fill="#22d3ee" text-decoration="underline">No</tspan><tspan fill="#94a3b8"> / </tspan><tspan fill="#e2e8f0">Yes</tspan></text>
<circle cx="30" cy="230" r="9" fill="#f97316"/>
<text x="30" y="234" text-anchor="middle" font-size="12" font-weight="bold" fill="#ffffff">9</text>
<text x="46" y="234" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#4ade80">√ </tspan><tspan fill="#f8fafc" font-weight="bold">Would you like to customize the </tspan><tspan fill="#60a5fa" font-weight="bold">import alias</tspan><tspan fill="#f8fafc" font-weight="bold"> (`@/*` by default)?</tspan><tspan fill="#94a3b8"> ... </tspan><tspan fill="#22d3ee" text-decoration="underline">No</tspan><tspan fill="#94a3b8"> / </tspan><tspan fill="#e2e8f0">Yes</tspan></text>
<circle cx="30" cy="252" r="9" fill="#f97316"/>
<text x="30" y="256" text-anchor="middle" font-size="12" font-weight="bold" fill="#ffffff">10</text>
<text x="46" y="256" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#4ade80">√ </tspan><tspan fill="#f8fafc" font-weight="bold">Would you like to include </tspan><tspan fill="#60a5fa" font-weight="bold">AGENTS.md</tspan><tspan fill="#f8fafc" font-weight="bold"> to guide coding agents</tspan></text>
<text x="46" y="278" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#f8fafc" font-weight="bold">  to write up-to-date Next.js code?</tspan><tspan fill="#94a3b8"> ... </tspan><tspan fill="#e2e8f0">No</tspan><tspan fill="#94a3b8"> / </tspan><tspan fill="#22d3ee" text-decoration="underline">Yes</tspan></text>
<circle cx="30" cy="296" r="9" fill="#f97316"/>
<text x="30" y="300" text-anchor="middle" font-size="12" font-weight="bold" fill="#ffffff">11</text>
<text x="46" y="300" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#4ade80">√ </tspan><tspan fill="#f8fafc" font-weight="bold">Would you like to help improve Next.js by letting agents prepare</tspan></text>
<text x="46" y="322" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#f8fafc">  </tspan><tspan fill="#60a5fa" font-weight="bold">anonymized feedback</tspan><tspan fill="#f8fafc" font-weight="bold"> for your review as you code? (Disable anytime</tspan></text>
<text x="46" y="344" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#f8fafc" font-weight="bold">  with `experimental.agentFeedback: false`.)</tspan><tspan fill="#94a3b8"> ... </tspan><tspan fill="#22d3ee" text-decoration="underline">No</tspan><tspan fill="#94a3b8"> / </tspan><tspan fill="#e2e8f0">Yes</tspan></text>
<text x="46" y="366" font-size="12" font-family="Consolas, Menlo, monospace" xml:space="preserve"><tspan fill="#e2e8f0">Creating a new Next.js app in C:\Users\you\projects\book-management.</tspan></text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図X: create-next-app の質問に答え終わった画面イメージ（水色の下線が選んだ答え。番号は下の表と対応）</div>
</div>

文字だけで書くと、実際の表示は次のとおりです（Windows では先頭の印が `√`、macOS では `✔` になります）。

```text
√ Would you like to use the recommended Next.js defaults? » No, customize settings
√ Would you like to use TypeScript? ... No / Yes
√ Which linter would you like to use? » ESLint
√ Would you like to use React Compiler? ... No / Yes
√ Would you like to use Tailwind CSS? ... No / Yes
√ Would you like your code inside a `src/` directory? ... No / Yes
√ Would you like to use App Router? (recommended) ... No / Yes
√ Would you like to use Cache Components? ... No / Yes
√ Would you like to customize the import alias (`@/*` by default)? ... No / Yes
√ Would you like to include AGENTS.md to guide coding agents to write up-to-date Next.js code? ... No / Yes
√ Would you like to help improve Next.js by letting agents prepare anonymized feedback for your review as you code? (Disable anytime with `experimental.agentFeedback: false`.) ... No / Yes
```

> **ヒント（Yes / No の選び方）:** 「No / Yes」の質問は、**←→キー**（または Tab キー）で選択が切り替わり、選ばれている方に下線が付きます。Enter で確定です。文字だけの表示では「No / Yes」と両方が並ぶので、どちらを選んだかは上の画面イメージ（水色の下線）や、次の表で確認してください。

それぞれの選択肢が何を意味するのか、なぜその回答をするのかを詳しく見ていきましょう。番号は図の丸数字と対応しています。

| 番号 | 質問 | 回答 | 理由 |
|------|------|------|------|
| ① | **recommended defaults**（推奨設定を使うか） | **No, customize settings** | 推奨設定は `src/` なし・Cache Components ありで、本書の手順と合いません。1つずつ自分で答えます。 |
| ② | **TypeScript** | Yes | 型安全性により、変数や関数の引数に間違った値を渡すミスをコンパイル時に検出できます。特にチーム開発やアプリが大規模になったときに威力を発揮します。本チュートリアルでは、Book 型などを定義してデータの構造を明確にします。 |
| ③ | **linter**（どのコードチェック道具を使うか） | ESLint | JavaScript/TypeScript のコードを静的解析し、バグの原因になりやすい書き方やスタイルの不統一を自動検出します。例えば、未使用の変数がある場合に警告を出してくれます。Biome という別の道具も選べますが、情報が多い ESLint にします。 |
| ④ | **React Compiler** | No | React のコードを自動で速くしてくれる仕組みです。便利ですが、学習中は「書いたとおりに動く」ほうが理解しやすいので使いません（後から有効にできます）。 |
| ⑤ | **Tailwind CSS** | Yes | ユーティリティファーストの CSS フレームワークです。HTML（JSX）の中にクラス名を書くだけでスタイリングできるため、CSS ファイルを別途管理する必要がありません。詳しくは次のセクションで解説します。 |
| ⑥ | **`src/` directory** | **Yes** | ソースコードを `src/` ディレクトリにまとめることで、設定ファイル（`package.json`, `next.config.ts` など）とソースコードが明確に分離されます。第7章以降のファイルの場所（`src/components/…`）もこの前提です。**既定値は No なので、必ず Yes に切り替えてください。** |
| ⑦ | **App Router** | Yes | Next.js 13 以降で導入された新しいルーティング方式です。ファイルシステムベースのルーティング、Server Components、レイアウトのネストなど、モダンな機能が使えます。Pages Router（旧方式）より推奨されています。 |
| ⑧ | **Cache Components** | **No** | ページの表示結果をうまく使い回すための新しい仕組みです。有効にすると「データを取りに行く処理は `<Suspense>` で囲む」などの追加ルールが必要になり、第7章のトップページ（サーバーで Supabase からデータを取るページ）が `next build` で失敗します。**既定値は Yes なので、必ず No に切り替えてください。** |
| ⑨ | **Import alias** | No（デフォルト） | デフォルトの `@/*` エイリアスをそのまま使います。`@/components/Header` のように `src/` ディレクトリ内のファイルを短いパスでインポートできます。 |
| ⑩ | **AGENTS.md** | Yes | AI のコーディング支援（第11章の Claude Code など）に「この Next.js は新しい版なので、手元の説明書を読んでから書いてね」と伝えるメモ書きのファイルを作ります。アプリの動作には影響しません。 |
| ⑪ | **anonymized feedback**（匿名のフィードバック） | No（どちらでも可） | AI のコーディング支援が、Next.js の改善に役立つ匿名の感想を下書きしてくれる機能です。アプリの動作には関係ないので、迷ったら No で構いません。 |

> **Turbopack について:** 以前の版では「Turbopack を使うか」という質問がありましたが、Next.js 16 では Turbopack（Rust で書かれた高速なバンドラー）が**最初から標準で使われる**ため、質問自体がなくなりました。開発サーバーの起動やホットリロード（コード変更時の自動反映）が従来の Webpack より大幅に高速です。

#### 1.2.1 つまずきポイント: Enter を連打して推奨設定を選んでしまった

> **つまずきポイント: 推奨設定（Yes, use recommended defaults）のまま進めてしまう**
>
> 最初の質問で何も考えずに Enter を押すと、推奨設定が選ばれ、残りの質問は表示されずにすぐプロジェクトが作られます。見た目は成功しますが、`src/` フォルダが無く、Cache Components も有効になっています。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="340" height="310" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="180" y="34" text-anchor="middle" font-size="13" font-weight="bold" fill="#c2410c">NG: Enter 連打で「推奨設定」を選んだ</text>
<rect x="370" y="10" width="340" height="310" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="540" y="34" text-anchor="middle" font-size="13" font-weight="bold" fill="#047857">OK: 「カスタマイズ」で本書どおりに答えた</text>
<text x="26" y="64" font-size="13" fill="#1e293b" font-family="Consolas, Menlo, monospace">book-management/</text>
<text x="44" y="88" font-size="13" fill="#c2410c" font-family="Consolas, Menlo, monospace">app/</text>
<text x="176" y="88" font-size="12" fill="#c2410c">← src/ が無い</text>
<text x="62" y="112" font-size="13" fill="#1e293b" font-family="Consolas, Menlo, monospace">layout.tsx</text>
<text x="62" y="136" font-size="13" fill="#1e293b" font-family="Consolas, Menlo, monospace">page.tsx</text>
<text x="44" y="160" font-size="13" fill="#1e293b" font-family="Consolas, Menlo, monospace">public/</text>
<text x="44" y="184" font-size="13" fill="#c2410c" font-family="Consolas, Menlo, monospace">next.config.ts</text>
<text x="176" y="184" font-size="12" fill="#c2410c">← Cache Components 有効</text>
<text x="44" y="208" font-size="13" fill="#c2410c" font-family="Consolas, Menlo, monospace">tsconfig.json</text>
<text x="386" y="64" font-size="13" fill="#1e293b" font-family="Consolas, Menlo, monospace">book-management/</text>
<text x="404" y="88" font-size="13" fill="#047857" font-family="Consolas, Menlo, monospace">src/</text>
<text x="536" y="88" font-size="12" fill="#047857">← コードはここ</text>
<text x="422" y="112" font-size="13" fill="#1e293b" font-family="Consolas, Menlo, monospace">app/</text>
<text x="440" y="136" font-size="13" fill="#1e293b" font-family="Consolas, Menlo, monospace">layout.tsx</text>
<text x="440" y="160" font-size="13" fill="#1e293b" font-family="Consolas, Menlo, monospace">page.tsx</text>
<text x="404" y="184" font-size="13" fill="#1e293b" font-family="Consolas, Menlo, monospace">public/</text>
<text x="404" y="208" font-size="13" fill="#047857" font-family="Consolas, Menlo, monospace">next.config.ts</text>
<text x="536" y="208" font-size="12" fill="#047857">← 余計な設定なし</text>
<text x="404" y="232" font-size="13" fill="#047857" font-family="Consolas, Menlo, monospace">tsconfig.json</text>
<rect x="26" y="236" width="308" height="30" rx="6" fill="#ffffff" stroke="#f97316" stroke-width="1.5"/>
<text x="180" y="256" text-anchor="middle" font-size="12" fill="#c2410c" font-family="Consolas, Menlo, monospace">tsconfig: "@/*": ["./*"]</text>
<rect x="386" y="260" width="308" height="30" rx="6" fill="#ffffff" stroke="#10b981" stroke-width="1.5"/>
<text x="540" y="280" text-anchor="middle" font-size="12" fill="#047857" font-family="Consolas, Menlo, monospace">tsconfig: "@/*": ["./src/*"]</text>
<text x="180" y="290" text-anchor="middle" font-size="12" fill="#c2410c">本書のパス（src/components/…）と食い違う</text>
<text x="180" y="310" text-anchor="middle" font-size="12" fill="#c2410c">第7章のトップページが build で失敗する</text>
<text x="540" y="310" text-anchor="middle" font-size="12" fill="#047857">第7章以降のコードがそのまま動く</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図X: 推奨設定を選んだ場合（左）と、本書どおりにカスタマイズした場合（右）の違い</div>
</div>

直し方は「作り直す」のがいちばん確実です。まだ何も書いていないので、失うものはありません。

NG（推奨設定のまま作ったプロジェクトを使い続ける）:

```text
book-management/app/page.tsx       ← src/ が無い
book-management/next.config.ts     ← cacheComponents: true が入っている
```

OK（フォルダを消して、カスタマイズで作り直す）:

```powershell
# 1つ上のフォルダに戻る（book-management の中にいる場合）
cd ..
# 推奨設定で作ったフォルダを丸ごと削除する（PowerShell の場合）
Remove-Item -Recurse -Force book-management
# もう一度作成し、最初の質問で「No, customize settings」を選ぶ
npx create-next-app@latest book-management
```

> **ヒント:** macOS や Linux のターミナルでは、削除コマンドは `rm -rf book-management` です。削除は元に戻せないので、フォルダ名をよく確認してから実行してください。

#### 1.2.2 サンプル: 質問に答える代わりにオプションでまとめて指定する

> **▼ このコードがやること（先に日本語で）:** 対話の質問に1つずつ答える代わりに、答えを「オプション（`--` で始まる追加の指定）」としてコマンドに全部書いておく方法です。オプションを1つでも付けると質問は表示されず、書かなかった項目は推奨設定の値になります。本書の答えをすべて書いておけば、矢印キーの操作ミスが起きません。

```bash
# 本書の答え（TypeScript / ESLint / Tailwind / src あり / App Router /
# React Compiler なし / Cache Components なし / エイリアス @/* / npm を使う /
# AGENTS.md あり / 匿名フィードバックなし）をオプションで指定する
npx create-next-app@latest book-management --ts --eslint --tailwind --src-dir --app --no-react-compiler --no-cache-components --import-alias "@/*" --use-npm --agents-md --no-agent-feedback
```

**▼ 実行結果**（パスは例。数字は時期によって変わります）

```text
Creating a new Next.js app in C:\Users\you\projects\book-management.

Using npm.

Initializing project with template: app-tw


Installing dependencies:
- next
- react
- react-dom

Installing devDependencies:
- @tailwindcss/turbopack
- @types/node
- @types/react
- @types/react-dom
- eslint
- eslint-config-next
- tailwindcss
- typescript

npm warn deprecated eslint@9.39.5: This version is no longer supported. Please see https://eslint.org/version-support for other options.

added 363 packages, and audited 364 packages in 2m

145 packages are looking for funding
  run `npm fund` for details

5 high severity vulnerabilities

To address all issues (including breaking changes), run:
  npm audit fix --force

Run `npm audit` for details.

Generating route types...
✓ Types generated successfully

Initialized a git repository.

Success! Created book-management at C:\Users\you\projects\book-management
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `--` で始まるオプションが「質問への答え」になる

```bash
--ts --eslint --tailwind --src-dir --app
```

- `--ts` は ② TypeScript に Yes、`--eslint` は ③ で ESLint、`--tailwind` は ⑤ に Yes、`--src-dir` は ⑥ に Yes、`--app` は ⑦ に Yes と答えたのと同じです。
- 質問と同じ内容を、キー操作ではなく文字で先に伝えているだけです。

##### 解説2: `--no-` で始まるオプションは「No と答える」

```bash
--no-react-compiler --no-cache-components --no-agent-feedback
```

- `--no-` を頭に付けると「その機能は使わない」という意味になります。④ React Compiler、⑧ Cache Components、⑪ 匿名フィードバックをすべて No にしています。
- 特に `--no-cache-components` は大切です。付け忘れると推奨設定の Yes が使われてしまいます。

##### 解説3: 残りのオプション

```bash
--import-alias "@/*" --use-npm --agents-md
```

- `--import-alias "@/*"` は ⑨ のエイリアスを `@/*` にする指定、`--agents-md` は ⑩ に Yes と答える指定です。
- `--use-npm` は「パッケージのインストールに npm を使う」という指定です（pnpm や yarn という別の道具もありますが、本書は npm で統一します）。

> **用語:** 「オプション（フラグ）」とは、コマンドの後ろに付けて動きを細かく指定する追加の文字列です。`--` で始まるものが多く、`npx create-next-app@latest --help` で一覧を見られます。

> **静的解析（static analysis）とは？**
> コードを実行せずに、ソースコードを読み解いて問題を見つける手法のこと。ESLint がこの役割を担います。例えば「この変数は宣言されているけど使われていない」「`if` 文の条件が常に true になる」といった問題を、実行前にチェックしてくれます。

> **パスエイリアスとは？（重要）**
> 通常、別のファイルをインポートするときは相対パス（`../../components/Header` のように、現在のファイルからの相対位置）を使います。深い階層になると `../../../../` のように `../` が増えて読みづらくなります。
>
> パスエイリアスを使うと、`@/components/Header` のように `src/` を起点とした絶対インポート（プロジェクトのルートからのパス）で書けるようになります。ファイルを移動してもインポート文を直さなくてよいので、リファクタリング（コードの整理）も楽になります。

### 1.3 なぜ Tailwind CSS を選ぶのか

CSS のスタイリング手法にはさまざまな選択肢があります。以下の比較表で、それぞれの特徴を整理します。

| 項目 | Tailwind CSS | CSS Modules | styled-components |
|------|-------------|-------------|-------------------|
| **スタイルの書き場所** | JSX のクラス名に直接記述 | `.module.css` ファイルに記述 | JavaScript ファイル内に記述 |
| **学習コスト** | クラス名を覚える必要がある（慣れると高速） | 通常の CSS と同じ知識で書ける | CSS + JavaScript の知識が必要 |
| **ファイル数** | CSS ファイル不要（少ない） | コンポーネントごとに CSS ファイルが必要（多い） | CSS ファイル不要（少ない） |
| **名前の衝突** | なし（ユーティリティクラスのため） | なし（自動でスコープされる） | なし（自動でスコープされる） |
| **バンドルサイズ** | 未使用クラスが自動削除される（小さい） | 使った分だけ（中程度） | ランタイムで CSS を生成（やや大きい） |
| **レスポンシブ対応** | `md:`, `lg:` などのプレフィックスで簡単 | メディアクエリを自分で書く | メディアクエリを自分で書く |
| **デザインの一貫性** | デフォルトのデザインシステム内蔵 | 自分で設計する必要がある | 自分で設計する必要がある |
| **Server Components 対応** | 完全対応 | 完全対応 | 対応に制限あり（`'use client'` が必要） |
| **本チュートリアルでの採用** | **採用** | 不採用 | 不採用 |

**Tailwind CSS を選ぶ最大の理由は「開発速度」です。** CSS ファイルを作成して行き来する必要がなく、JSX を書きながらそのままスタイリングできます。最初はクラス名を覚えるのに少し時間がかかりますが、慣れると非常に高速に UI を構築できるようになります。

### 1.4 コマンド実行中のターミナル出力

質問に全て答え終わると、ターミナルに次のような表示が流れます。次は 2026年10月に実際に実行したときの表示です（パスは例。パッケージの数や秒数は時期によって変わります）。

```text
Creating a new Next.js app in C:\Users\you\projects\book-management.

Using npm.

Initializing project with template: app-tw


Installing dependencies:
- next
- react
- react-dom

Installing devDependencies:
- @tailwindcss/turbopack
- @types/node
- @types/react
- @types/react-dom
- eslint
- eslint-config-next
- tailwindcss
- typescript

npm warn deprecated eslint@9.39.5: This version is no longer supported. Please see https://eslint.org/version-support for other options.

added 363 packages, and audited 364 packages in 2m

145 packages are looking for funding
  run `npm fund` for details

5 high severity vulnerabilities

To address all issues (including breaking changes), run:
  npm audit fix --force

Run `npm audit` for details.

Generating route types...
✓ Types generated successfully

Initialized a git repository.

Success! Created book-management at C:\Users\you\projects\book-management
```

最後に `Success! Created book-management at ...` と出ていれば成功です。途中の行の意味は次のとおりです。

| 表示 | 意味 |
|------|------|
| `Initializing project with template: app-tw` | 「App Router ＋ Tailwind（tw）」用のひな形を使って作り始めた、という意味です。 |
| `Installing dependencies` / `devDependencies` | これからインストールする部品の一覧です（違いは下の囲みで説明します）。 |
| `npm warn deprecated ...` | **warn（警告）** であって、エラーではありません。「この版はサポートが終わっています」というお知らせなので、作業はそのまま続けて大丈夫です。 |
| `added 363 packages ... in 2m` | 363個の部品を入れるのに約2分かかった、という意味です。 |
| `5 high severity vulnerabilities` | 部品の中に、注意が必要な既知の問題（脆弱性）が見つかったというお知らせです。すぐ下のつまずきポイントを読んでください。 |
| `Generating route types...` | ページの URL（ルート）に合わせた TypeScript の型を自動で作っています。 |
| `Initialized a git repository.` | Git（変更履歴を記録する仕組み）の管理を始めた、という意味です。 |

> **つまずきポイント: 「vulnerabilities」と出たので `npm audit fix --force` を実行してしまう**
>
> 「脆弱性（vulnerabilities）」と出ると不安になり、表示どおり `npm audit fix --force` を実行したくなります。しかし `--force` は「互換性が壊れても構わず書き換える」という強い指示で、学習用プロジェクトでは逆に動かなくなる原因になります。

NG（表示どおりに実行してしまう）:

```bash
# 互換性を無視して部品を入れ替える（実行しないこと）
npm audit fix --force
```

OK（まず中身を確認するだけにする）:

```bash
# どの部品のどんな問題かを確認するだけ（何も書き換えない）
npm audit
```

**▼ 実行結果**（先頭部分）

```text
# npm audit report

braces  *
Severity: high
braces vulnerable to stack-exhaustion denial of service through deeply nested patterns - https://github.com/advisories/GHSA-vfj7-8cjw-p6xm
fix available via `npm audit fix --force`
Will install eslint-config-next@14.2.35, which is a breaking change
node_modules/braces
```

`Will install eslint-config-next@14.2.35, which is a breaking change` に注目してください。「直すために eslint-config-next を **14.2.35（古い版）に入れ替えます。これは互換性を壊す変更です**」と書いてあります。Next.js 16 用の設定を 14 用に戻してしまうので、`--force` を付けて実行すると ESLint の設定が合わなくなります。今回の問題は開発中だけ使うコードチェック道具（ESLint）の部品の中にあり、作るアプリ本体の動作には関係しません。部品の作者が修正版を出すのを待ち、後で `npm update`（互換性の範囲で新しくする）を実行すれば十分です。

> **「added XXX packages」とは？:** `npm install` が `node_modules/` フォルダに何個のパッケージを置いたかを示します。React/Next.js本体に加えて、依存パッケージ（依存関係でついてくる関連ライブラリ）が一緒に入るので 300〜400個程度になります（今回は 363個）。これが普通です。

> **`npm install` で何が起きているか:**
> 1. `package.json` に書かれた依存パッケージのリストを読み取る
> 2. npm レジストリ（https://www.npmjs.com）から、それぞれのパッケージをダウンロード
> 3. ダウンロードしたパッケージを `node_modules/` フォルダに展開
> 4. 各パッケージがさらに依存しているパッケージも芋づる式にダウンロード（これが「依存関係の解決」）
> 5. インストールされた正確なバージョンを `package-lock.json` に記録
>
> `node_modules/` はとても巨大になりやすいですが、`.gitignore` でGit管理から除外するのが標準的なやり方です（後述）。

この流れは「買い物」にたとえると分かりやすくなります。`package.json` は**買い物メモ**、npm レジストリは**お店**、`node_modules/` は買ってきた物をしまう**冷蔵庫**、`package-lock.json` は何をいくつ買ったかの**レシート**です。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 235" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-05" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="70" width="150" height="90" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="95" y="100" text-anchor="middle" font-size="14" font-weight="bold" fill="#1e40af">package.json</text>
<text x="95" y="122" text-anchor="middle" font-size="12" fill="#1e40af">ほしい物のリスト</text>
<text x="95" y="142" text-anchor="middle" font-size="12" fill="#64748b">（買い物メモ）</text>
<rect x="285" y="70" width="150" height="90" rx="10" fill="#f8fafc" stroke="#475569" stroke-width="2"/>
<text x="360" y="100" text-anchor="middle" font-size="14" font-weight="bold" fill="#475569">npm レジストリ</text>
<text x="360" y="122" text-anchor="middle" font-size="12" fill="#475569">インターネット上の</text>
<text x="360" y="142" text-anchor="middle" font-size="12" fill="#64748b">倉庫（お店）</text>
<rect x="550" y="20" width="150" height="90" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="625" y="50" text-anchor="middle" font-size="14" font-weight="bold" fill="#047857">node_modules/</text>
<text x="625" y="72" text-anchor="middle" font-size="12" fill="#047857">ダウンロードした実物</text>
<text x="625" y="92" text-anchor="middle" font-size="12" fill="#64748b">（冷蔵庫）</text>
<rect x="550" y="130" width="150" height="90" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="625" y="160" text-anchor="middle" font-size="14" font-weight="bold" fill="#c2410c">package-lock.json</text>
<text x="625" y="182" text-anchor="middle" font-size="12" fill="#c2410c">正確なバージョン記録</text>
<text x="625" y="202" text-anchor="middle" font-size="12" fill="#64748b">（レシート）</text>
<line x1="170" y1="115" x2="283" y2="115" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-05)"/>
<text x="227" y="105" text-anchor="middle" font-size="12" fill="#475569">① 読んで注文</text>
<line x1="435" y1="100" x2="548" y2="70" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-05)"/>
<text x="490" y="72" text-anchor="middle" font-size="12" fill="#475569">② 展開</text>
<line x1="435" y1="130" x2="548" y2="170" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-05)"/>
<text x="488" y="172" text-anchor="middle" font-size="12" fill="#475569">③ 記録</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図6-2: npm install の流れ（買い物メモ → お店 → 冷蔵庫とレシート）</div>
</div>

Git に入れる（チームで共有する）のはメモとレシート、つまり `package.json` と `package-lock.json` だけです。冷蔵庫の中身（`node_modules/`）は、メモとレシートさえあれば `npm install` でいつでも同じ状態に作り直せるので、共有しません。

> **`dependencies` と `devDependencies` の違い:**
> - **`dependencies`**: アプリが本番環境で動くために必要なパッケージ。例: React、Next.js本体。
> - **`devDependencies`**: 開発中だけ必要なパッケージ。例: TypeScript（本番ではJavaScriptに変換済み）、ESLint（本番では使わない）。
>
> 本番ビルド時には `devDependencies` を除外できるので、本番のサーバーに置くファイルを軽くできます。

完了したら、表示されたプロジェクトフォルダに `cd` で移動します。

```bash
# cd: Change Directory の略。指定したフォルダに移動するコマンド
# book-management: 移動先のフォルダ名（先ほど create-next-app で作成したフォルダ）
cd book-management
```

### 1.5 生成されるファイルの役割

`create-next-app` を実行すると、以下のファイルとディレクトリが自動生成されます（create-next-app 16.4.1 で実際に作られたもの）。

```text
book-management/
  .git/                 ... Git の履歴を保存する隠しフォルダ（自動生成。触らない）
  .next/                ... next dev / next build が作る作業用フォルダ（自動生成。.gitignoreで除外）
  node_modules/         ... インストールされたパッケージ群（数百フォルダ。.gitignoreで除外する）
  public/               ... 静的ファイル（画像など）。ここに置いたファイルは / 直下のURLでアクセス可能
    file.svg            ... ファイルアイコン（デフォルトサンプル）
    globe.svg           ... 地球アイコン（デフォルトサンプル）
    next.svg            ... Next.js ロゴ（デフォルトサンプル）
    vercel.svg          ... Vercel ロゴ（デフォルトサンプル）
    window.svg          ... ウィンドウアイコン（デフォルトサンプル）
  src/                  ... アプリのソースコードを置くフォルダ
    app/                ... App Router のルートディレクトリ。フォルダ構成がそのままURLになる
      favicon.ico       ... ブラウザのタブに表示されるアイコン
      globals.css       ... グローバル CSS（Tailwind CSS を読み込む）
      layout.tsx        ... ルートレイアウト（全ページ共通の枠）
      page.tsx          ... トップページ（ / にアクセスしたときの表示）
  .gitignore            ... Git で管理しないファイルの指定
  AGENTS.md             ... AI コーディング支援向けの説明書（質問⑩で Yes にしたので作られた）
  eslint.config.mjs     ... ESLint の設定ファイル
  next-env.d.ts         ... Next.js の TypeScript 型定義（自動生成）
  next.config.ts        ... Next.js の設定ファイル
  package-lock.json     ... パッケージの正確なバージョン記録（自動更新されるので手で編集しない）
  package.json          ... プロジェクトの設定とパッケージ一覧
  README.md             ... プロジェクトの説明
  tsconfig.json         ... TypeScript の設定ファイル
```

ファイルが多くて圧倒されるかもしれませんが、「自分で書き換えるのはどれか」で色分けすると、実はほとんどが**触らなくてよいファイル**だと分かります。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 526" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="24" y="34" font-size="13" font-weight="bold" fill="#1e293b" font-family="Consolas, Menlo, monospace">book-management/</text>
<text x="304" y="34" font-size="12" fill="#475569">プロジェクトのフォルダ（ここで npm run dev する）</text>
<rect x="38" y="42" width="57" height="22" rx="5" fill="#f8fafc" stroke="#94a3b8" stroke-width="1.5"/>
<text x="44" y="58" font-size="13" fill="#64748b" font-family="Consolas, Menlo, monospace">.next/</text>
<line x1="99" y1="53" x2="296" y2="53" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="58" font-size="12" fill="#64748b">next dev / next build が作る作業場所</text>
<rect x="38" y="66" width="108" height="22" rx="5" fill="#f8fafc" stroke="#94a3b8" stroke-width="1.5"/>
<text x="44" y="82" font-size="13" fill="#64748b" font-family="Consolas, Menlo, monospace">node_modules/</text>
<line x1="150" y1="77" x2="296" y2="77" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="82" font-size="12" fill="#64748b">npm install で入った部品（約370個）</text>
<rect x="38" y="90" width="64" height="22" rx="5" fill="#fff7ed" stroke="#f97316" stroke-width="1.5"/>
<text x="44" y="106" font-size="13" fill="#c2410c" font-family="Consolas, Menlo, monospace">public/</text>
<line x1="106" y1="101" x2="296" y2="101" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="106" font-size="12" fill="#c2410c">画像などの公開ファイル</text>
<rect x="58" y="114" width="130" height="22" rx="5" fill="#fff7ed" stroke="#f97316" stroke-width="1.5"/>
<text x="64" y="130" font-size="13" fill="#c2410c" font-family="Consolas, Menlo, monospace">next.svg ほか4つ</text>
<line x1="192" y1="125" x2="296" y2="125" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="130" font-size="12" fill="#c2410c">/next.svg のような URL でそのまま見える</text>
<rect x="38" y="138" width="43" height="22" rx="5" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="44" y="154" font-size="13" fill="#1e40af" font-family="Consolas, Menlo, monospace">app/</text>
<line x1="85" y1="149" x2="296" y2="149" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="154" font-size="12" fill="#1e40af">フォルダの形が URL になる（App Router）</text>
<rect x="58" y="162" width="93" height="22" rx="5" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="64" y="178" font-size="13" fill="#1e40af" font-family="Consolas, Menlo, monospace">favicon.ico</text>
<line x1="155" y1="173" x2="296" y2="173" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="178" font-size="12" fill="#1e40af">ブラウザのタブのアイコン</text>
<rect x="58" y="186" width="93" height="22" rx="5" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="64" y="202" font-size="13" fill="#1e40af" font-family="Consolas, Menlo, monospace">globals.css</text>
<line x1="155" y1="197" x2="296" y2="197" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="202" font-size="12" fill="#1e40af">全ページ共通の CSS（Tailwind を読み込む）</text>
<rect x="58" y="210" width="86" height="22" rx="5" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="64" y="226" font-size="13" fill="#1e40af" font-family="Consolas, Menlo, monospace">layout.tsx</text>
<line x1="148" y1="221" x2="296" y2="221" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="226" font-size="12" fill="#1e40af">全ページの外枠（節7で書き換える）</text>
<rect x="58" y="234" width="72" height="22" rx="5" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="64" y="250" font-size="13" fill="#1e40af" font-family="Consolas, Menlo, monospace">page.tsx</text>
<line x1="134" y1="245" x2="296" y2="245" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="250" font-size="12" fill="#1e40af">/ のページ（節9で書き換える）</text>
<rect x="38" y="258" width="86" height="22" rx="5" fill="#ecfdf5" stroke="#10b981" stroke-width="1.5"/>
<text x="44" y="274" font-size="13" fill="#047857" font-family="Consolas, Menlo, monospace">.gitignore</text>
<line x1="128" y1="269" x2="296" y2="269" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="274" font-size="12" fill="#047857">Git に入れないファイルの一覧</text>
<rect x="38" y="282" width="79" height="22" rx="5" fill="#fdf2f8" stroke="#db2777" stroke-width="1.5"/>
<text x="44" y="298" font-size="13" fill="#9d174d" font-family="Consolas, Menlo, monospace">AGENTS.md</text>
<line x1="121" y1="293" x2="296" y2="293" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="298" font-size="12" fill="#9d174d">AI コーディング支援向けの説明書</text>
<rect x="38" y="306" width="136" height="22" rx="5" fill="#ecfdf5" stroke="#10b981" stroke-width="1.5"/>
<text x="44" y="322" font-size="13" fill="#047857" font-family="Consolas, Menlo, monospace">eslint.config.mjs</text>
<line x1="178" y1="317" x2="296" y2="317" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="322" font-size="12" fill="#047857">ESLint（コードチェック）の設定</text>
<rect x="38" y="330" width="108" height="22" rx="5" fill="#f8fafc" stroke="#94a3b8" stroke-width="1.5"/>
<text x="44" y="346" font-size="13" fill="#64748b" font-family="Consolas, Menlo, monospace">next-env.d.ts</text>
<line x1="150" y1="341" x2="296" y2="341" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="346" font-size="12" fill="#64748b">Next.js 用の型の読み込み（自動生成）</text>
<rect x="38" y="354" width="115" height="22" rx="5" fill="#ecfdf5" stroke="#10b981" stroke-width="1.5"/>
<text x="44" y="370" font-size="13" fill="#047857" font-family="Consolas, Menlo, monospace">next.config.ts</text>
<line x1="157" y1="365" x2="296" y2="365" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="370" font-size="12" fill="#047857">Next.js の設定</text>
<rect x="38" y="378" width="100" height="22" rx="5" fill="#ecfdf5" stroke="#10b981" stroke-width="1.5"/>
<text x="44" y="394" font-size="13" fill="#047857" font-family="Consolas, Menlo, monospace">package.json</text>
<line x1="142" y1="389" x2="296" y2="389" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="394" font-size="12" fill="#047857">使う部品とスクリプト（npm run ○○）の一覧</text>
<rect x="38" y="402" width="136" height="22" rx="5" fill="#f8fafc" stroke="#94a3b8" stroke-width="1.5"/>
<text x="44" y="418" font-size="13" fill="#64748b" font-family="Consolas, Menlo, monospace">package-lock.json</text>
<line x1="178" y1="413" x2="296" y2="413" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="418" font-size="12" fill="#64748b">部品の正確なバージョン記録（自動更新）</text>
<rect x="38" y="426" width="79" height="22" rx="5" fill="#fdf2f8" stroke="#db2777" stroke-width="1.5"/>
<text x="44" y="442" font-size="13" fill="#9d174d" font-family="Consolas, Menlo, monospace">README.md</text>
<line x1="121" y1="437" x2="296" y2="437" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="442" font-size="12" fill="#9d174d">プロジェクトの説明書</text>
<rect x="38" y="450" width="108" height="22" rx="5" fill="#ecfdf5" stroke="#10b981" stroke-width="1.5"/>
<text x="44" y="466" font-size="13" fill="#047857" font-family="Consolas, Menlo, monospace">tsconfig.json</text>
<line x1="150" y1="461" x2="296" y2="461" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 3"/>
<text x="304" y="466" font-size="12" fill="#047857">TypeScript の設定（@/ の意味もここ）</text>
<rect x="24" y="493" width="14" height="14" rx="3" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="44" y="504" font-size="12" fill="#475569">自分で書く</text>
<rect x="118" y="493" width="14" height="14" rx="3" fill="#ecfdf5" stroke="#10b981" stroke-width="1.5"/>
<text x="138" y="504" font-size="12" fill="#475569">設定（ときどき編集）</text>
<rect x="272" y="493" width="14" height="14" rx="3" fill="#fff7ed" stroke="#f97316" stroke-width="1.5"/>
<text x="292" y="504" font-size="12" fill="#475569">公開ファイル</text>
<rect x="378" y="493" width="14" height="14" rx="3" fill="#f8fafc" stroke="#94a3b8" stroke-width="1.5"/>
<text x="398" y="504" font-size="12" fill="#475569">自動生成（触らない）</text>
<rect x="532" y="493" width="14" height="14" rx="3" fill="#fdf2f8" stroke="#db2777" stroke-width="1.5"/>
<text x="552" y="504" font-size="12" fill="#475569">説明書（読むだけ）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図X: create-next-app が作ったファイルと、それぞれの役割（色は「触ってよいか」の目安）</div>
</div>

> **ヒント:** 先頭が `.`（ドット）のファイルやフォルダ（`.git/`、`.next/`、`.gitignore`）は、macOS の Finder では**隠しファイル**として表示されません（Windows のエクスプローラーでも `.git/` は隠しフォルダになります）。VS Code のエクスプローラーならほぼすべて表示されます（`.git/` だけは VS Code でも既定で隠されます）。

各ファイルの役割を表にまとめます。

| ファイル | 役割 |
|----------|------|
| `package.json` | プロジェクト名、使用パッケージ、実行スクリプトなどを定義するファイルです。`npm install` はこのファイルを読んでパッケージをインストールします。 |
| `tsconfig.json` | TypeScript コンパイラの設定ファイルです。`@/*` のパスエイリアスもここで定義されています。 |
| `next.config.ts` | Next.js の動作をカスタマイズする設定ファイルです。画像の外部ドメイン許可などを設定できます。今回は Tailwind CSS を読み込むための設定（`turbopack.rules`）が最初から書かれています。 |
| `eslint.config.mjs` | ESLint のルール設定です。`eslint-config-next` の推奨ルール（`core-web-vitals` と `typescript`）が最初から有効になっています。 |
| `AGENTS.md` | AI のコーディング支援に向けた説明書です。「この Next.js は新しい版なので、`node_modules/next/dist/docs/` の説明書を読んでから書くこと」といった注意が書かれています。 |
| `src/app/layout.tsx` | すべてのページを囲むルートレイアウトです。`<html>` と `<body>` タグを含み、共通のヘッダーやフッターを配置します。 |
| `src/app/page.tsx` | トップページのコンポーネントです。URL `/` にアクセスしたときに表示されます。 |
| `src/app/globals.css` | アプリ全体に適用されるグローバル CSS です。先頭の `@import "tailwindcss";` の1行で Tailwind CSS が読み込まれます。 |

> **注意（古い記事との違い）:** 少し前の記事や本では、`tailwind.config.ts`（Tailwind の設定）、`postcss.config.mjs`（PostCSS の設定）、`.eslintrc.json`（ESLint の古い形式の設定）というファイルが登場します。現在の create-next-app が入れる **Tailwind CSS v4** は設定ファイルなしで動き（色などのカスタマイズは CSS の中に書きます）、ESLint の設定も `eslint.config.mjs` という新しい形式に変わったため、これらのファイルは作られません。**無くて正常**なので、自分で作る必要はありません。

> **`.gitignore` で何を除外するか、なぜか:**
> Git の管理から除外する（コミットしない）べきファイルの種類を `.gitignore` ファイルに書きます。`create-next-app` が自動生成する `.gitignore` には、次のようなものが含まれます。
>
> | 除外対象 | 理由 |
> |---------|------|
> | `/node_modules` | パッケージ群はとても大きい上に `package.json` から再インストールできるため。コミットすると無駄に容量を食う。 |
> | `/.next/` | Next.js のビルド結果。実行のたびに再生成されるため。 |
> | `.env*` | 秘密の環境変数（APIキー、パスワードなど）が入っているため。GitHub にアップすると漏洩する。`.env.local` も `.env` も、`.env` で始まるファイルはすべて除外される。 |
> | `npm-debug.log*` など | ログファイルは個人のPC固有のものなので共有不要。 |
> | `.DS_Store` | macOS が自動生成する隠しファイル。アプリと無関係。 |
>
> ルール: 「自動生成されるもの」「PC固有のもの」「秘密情報を含むもの」はGitに入れない、と覚えておくと良いです。

### 1.6 主な設定ファイルの中身を見てみる

#### `package.json` の構造

`create-next-app` が生成する `package.json` は、次のような構造になっています（実際の中身に説明のコメントを足したもの。本物の JSON にはコメントを書けないので、これは読むための版です）。

```json
{
  // プロジェクト名（フォルダ名と同じ）
  "name": "book-management",
  // プロジェクトのバージョン（自由に決められる）
  "version": "0.1.0",
  // true にすると npm publish で公開できなくなる（誤公開防止）
  "private": true,
  // npm run <名前> で実行できるコマンドの集まり
  "scripts": {
    // 開発サーバー起動（Next.js 16 では Turbopack が標準なのでオプション不要）
    "dev": "next dev",
    // 本番用ビルド（最適化された静的ファイルを生成）
    "build": "next build",
    // ビルド済みアプリを本番モードで起動
    "start": "next start",
    // ESLint でコード品質チェックを実行（Next.js 16 では next lint ではなく eslint を直接呼ぶ）
    "lint": "eslint"
  },
  // 本番でも必要なパッケージ
  "dependencies": {
    // Next.js フレームワーク
    "next": "16.4.1",
    // React 本体
    "react": "19.3.0",
    // React を DOM にレンダリングするためのパッケージ
    "react-dom": "19.3.0"
  },
  // 開発中だけ必要なパッケージ
  "devDependencies": {
    // Turbopack で Tailwind CSS を処理するための部品
    "@tailwindcss/turbopack": "^4",
    // Node.js の型定義
    "@types/node": "^20",
    // React の型定義
    "@types/react": "^19",
    // React DOM の型定義
    "@types/react-dom": "^19",
    // ESLint 本体
    "eslint": "^9",
    // Next.js 用 ESLint 設定
    "eslint-config-next": "16.4.1",
    // Tailwind CSS 本体（v4）
    "tailwindcss": "^4",
    // TypeScript コンパイラ
    "typescript": "^5"
  }
}
```

#### 1.6.1 サンプル: 使えるスクリプトを一覧表示する

> **▼ このコードがやること（先に日本語で）:** `package.json` の `"scripts"` に何が書いてあるかを、ファイルを開かずにターミナルで確かめます。`npm run` とだけ打つと、`npm run ○○` の ○○ に入れられる名前の一覧が表示されます。ついでに、入っている Next.js のバージョンも確認します。

```bash
# プロジェクトのフォルダ（book-management）の中で実行する
# 使えるスクリプトの一覧を表示する
npm run
# インストールされている Next.js のバージョンを表示する
npx next --version
```

**▼ 実行結果**

```text
Lifecycle scripts included in book-management@0.1.0:
  start
    next start
available via `npm run`:
  dev
    next dev
  build
    next build
  lint
    eslint
Next.js v16.4.1
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `npm run` だけで一覧が出る

```bash
npm run
```

- `npm run dev` の `dev` の部分を省略すると、「どの名前なら実行できるか」の一覧が出ます。
- `start` だけ別の見出し（Lifecycle scripts）に入っているのは、`start` が npm にとって特別な名前で、`npm start` と `run` なしでも実行できるからです。
- 一覧の下の行（`next dev` など）は、その名前で実際に実行される中身です。`npm run dev` は「`next dev` を実行して」という意味だと分かります。

##### 解説2: `npx next --version` で Next.js の版を確かめる

```bash
npx next --version
```

- `npx next` は、このプロジェクトの `node_modules/` に入っている Next.js のコマンドを実行します。`--version` を付けると版だけを表示して終わります。
- 教材や記事と動きが違うときは、まずこの版を確認する習慣をつけると、原因の切り分けが早くなります。

> **用語:** 「スクリプト（script）」とは、よく使うコマンドに短い名前を付けて `package.json` に登録したものです。長いコマンドを覚えなくても `npm run 名前` で実行できます。

#### `tsconfig.json` の主要設定

```json
{
  // TypeScript コンパイラの動作設定
  "compilerOptions": {
    // コンパイル後のJavaScriptのバージョン（ES2017相当）
    "target": "ES2017",
    // 使えるグローバルAPI（DOM、最新のES機能など）
    "lib": ["dom", "dom.iterable", "esnext"],
    // .js ファイルもTypeScriptで扱えるようにする
    "allowJs": true,
    // ライブラリの型チェックをスキップ（高速化）
    "skipLibCheck": true,
    // 厳格な型チェックを有効化（推奨）
    "strict": true,
    // .ts → .js への変換ファイルは出力しない（Next.jsが代行）
    "noEmit": true,
    // CommonJS と ES Modules の混在を許可
    "esModuleInterop": true,
    // モジュールシステムは最新の ES Modules
    "module": "esnext",
    // モジュール解決方式（バンドラー用）
    "moduleResolution": "bundler",
    // .json ファイルを import 可能にする
    "resolveJsonModule": true,
    // 各ファイルを独立したモジュールとして扱う
    "isolatedModules": true,
    // JSX を React 17 以降の新しい方式で変換する（import React が不要）
    "jsx": "react-jsx",
    // 増分コンパイルを有効化（再ビルドが速くなる）
    "incremental": true,
    "plugins": [
      // Next.js 用の TypeScript プラグイン
      { "name": "next" }
    ],
    // パスエイリアスの定義
    "paths": {
      // @/foo は src/foo を指す
      "@/*": ["./src/*"]
    }
  },
  // 型チェック対象（.next/ の中に自動生成される型も含める）
  "include": [
    "next-env.d.ts",
    "**/*.ts",
    "**/*.tsx",
    ".next/types/**/*.ts",
    ".next/dev/types/**/*.ts",
    "**/*.mts"
  ],
  // 型チェック対象から除外
  "exclude": ["node_modules"]
}
```

#### `next.config.ts` の中身

```typescript
// next.config.ts

// Next.js の設定型をインポート（type 付きで型のみ取得）
import type { NextConfig } from "next";

// 設定オブジェクトの型を NextConfig に固定
const nextConfig: NextConfig = {
  /* config options here */
  // Turbopack（開発サーバーとビルドの道具）への追加設定
  turbopack: {
    // 「どのファイルをどう変換するか」の決まり
    rules: {
      // すべての .css ファイルを対象にする
      "*.css": {
        // Tailwind CSS 用の変換部品を通してから使う
        loaders: ["@tailwindcss/turbopack"],
        // 変換後も CSS として扱う
        as: "*.css",
      },
    },
  },
};

// デフォルトエクスポート（Next.js が自動で読み込む）
export default nextConfig;
```

`/* config options here */`（ここに設定を書く）の場所に、画像の外部ドメイン許可（`images.remotePatterns`）などの設定を追加していきます。`turbopack` の部分は「CSS ファイルを読み込むときに Tailwind CSS の変換を通す」という意味で、これがあるから `globals.css` の `@import "tailwindcss";` が働きます。**消さないでください。**

> **注意:** 質問⑧で Cache Components を Yes にしてしまった場合は、この `/* config options here */` の下に `cacheComponents: true,` と `partialPrefetching: true,` の2行が入っています。直し方は 1.7 のつまずきポイントを見てください。

### 1.7 つまずきポイント: Cache Components を Yes にしてしまった

> **つまずきポイント: Cache Components が有効なまま第7章に進む**
>
> 質問⑧の既定値は Yes なので、Enter を押しただけだと有効になります。この章のページは問題なく表示されますが、第7章で「サーバーで Supabase からデータを取るトップページ」を作ると、`npm run build`（本番用ビルド）が失敗します。

NG（Cache Components が有効な `next.config.ts`）:

```typescript
// next.config.ts（質問⑧で Yes を選んだ場合）
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  // ↓ この2行が Cache Components を有効にしている
  cacheComponents: true,
  partialPrefetching: true,
  turbopack: {
    rules: {
      "*.css": {
        loaders: ["@tailwindcss/turbopack"],
        as: "*.css",
      },
    },
  },
};

export default nextConfig;
```

この状態で、第7章と同じ形のトップページ（サーバー用の Supabase クライアントで books テーブルを読むページ）を `npm run build` すると、次のエラーで止まります。

**▼ 実行結果**（実際の出力の抜粋）

```text
- Cache Components enabled
- Partial Prefetching enabled

  Creating an optimized production build ...
✓ Compiled successfully in 9.1s
  Running TypeScript ...
  Finished TypeScript in 2.3s ...
  Collecting page data using 5 workers ...
  Generating static pages using 5 workers (0/4) ...
  Generating static pages using 5 workers (1/4) 
Error: Route "/": Next.js encountered uncached or runtime data during prerendering.

`fetch(...)`, `cookies()`, `headers()`, `params`, `searchParams`, or `connection()` accessed outside of `<Suspense>` prevents the route from being prerendered, blocking the page load and leading to a slower user experience.

（直し方の候補と詳しい調べ方の案内が英語で14行ほど続くので省略）

Error occurred prerendering page "/". Read more: https://nextjs.org/docs/messages/prerender-error
Export encountered an error on /page: /, exiting the build.
⨯ Next.js build worker exited with code: 1 and signal: null
```

エラーの意味は「`/` のページで、`cookies()` などの毎回変わるデータを `<Suspense>` の外で使っているので、前もってページを作っておけません」です。Cache Components は「ページを前もって作っておく」ことを強く求める仕組みなので、データを毎回取りに行く第7章のページとは相性が悪いのです。

OK（2行を消して Cache Components を使わない）:

```typescript
// next.config.ts（本書の設定）
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  turbopack: {
    rules: {
      "*.css": {
        loaders: ["@tailwindcss/turbopack"],
        as: "*.css",
      },
    },
  },
};

export default nextConfig;
```

`cacheComponents: true,` と `partialPrefetching: true,` の2行を消して保存すれば、本書どおりの設定（質問⑧で No を選んだのと同じ状態）になります。開発サーバーを動かしている場合は、`next.config.ts` の変更後に一度止めて（Ctrl+C）起動し直してください。

---

## 2. Tailwind CSS の基礎

### 2.1 ユーティリティファーストとは

従来の CSS では、まずクラス名を考え、そのクラスに対してスタイルを定義していました。

```css
/* 従来の CSS */
/* card という名前のクラスにスタイルを定義 */
.card {
  /* 背景色を白に */
  background-color: white;
  /* 角を 8px の半径で丸める */
  border-radius: 8px;
  /* 内側の余白を 16px に */
  padding: 16px;
  /* 薄い影を下方向に */
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}
```

```html
<!-- HTML 側で class 属性に名前を指定して上記スタイルを適用 -->
<div class="card">...</div>
```

Tailwind CSS では、あらかじめ用意された小さなユーティリティクラスを組み合わせてスタイリングします。

```html
<!-- Tailwind CSS: 1つ1つが小さな役割を持つクラスを並べる -->
<!-- bg-white: 背景白 / rounded-lg: 大きめの角丸 / p-4: 内側16px余白 / shadow-sm: 控えめな影 -->
<div class="bg-white rounded-lg p-4 shadow-sm">...</div>
```

それぞれのクラスが1つの CSS プロパティに対応しています。

| クラス | 対応する CSS |
|--------|-------------|
| `bg-white` | `background-color: white;` |
| `rounded-lg` | `border-radius: 0.5rem;` |
| `p-4` | `padding: 1rem;` |
| `shadow-sm` | `box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);` |

> **ヒント（実際に出力される CSS）:** Tailwind CSS v4 が実際に作る CSS は、`.p-4 { padding: calc(var(--spacing) * 4); }` のように「変数 × 数」の形になっています。`--spacing` は `0.25rem` と決められているので、計算結果は上の表と同じ `1rem` です。表は分かりやすさのため、計算後の値で書いています。

**ユーティリティファーストのメリット:**

- CSS ファイルを作る必要がない（JSX だけで完結する）
- クラス名を考える時間がゼロになる
- デザインシステム（余白・色・サイズ）が統一される
- 使っていないスタイルが自動で削除される（バンドルサイズが小さい）

**ユーティリティファーストのデメリット:**

- JSX のクラス名が長くなることがある
- 最初はクラス名を覚えるのに時間がかかる

### 2.2 よく使うクラス一覧

本チュートリアルで頻繁に使うクラスを以下にまとめます。この表をブックマークしておくと、開発中に素早く参照できます。

#### 余白（Spacing）

| クラス | CSS | 説明 |
|--------|-----|------|
| `p-4` | `padding: 1rem;` (16px) | 内側の余白（全方向） |
| `px-4` | `padding-left: 1rem; padding-right: 1rem;` | 内側の余白（左右） |
| `py-2` | `padding-top: 0.5rem; padding-bottom: 0.5rem;` | 内側の余白（上下） |
| `pt-4` | `padding-top: 1rem;` | 内側の余白（上のみ） |
| `pb-4` | `padding-bottom: 1rem;` | 内側の余白（下のみ） |
| `m-4` | `margin: 1rem;` | 外側の余白（全方向） |
| `mx-auto` | `margin-left: auto; margin-right: auto;` | 要素を水平方向に中央揃え |
| `mt-8` | `margin-top: 2rem;` | 外側の余白（上のみ） |
| `mb-4` | `margin-bottom: 1rem;` | 外側の余白（下のみ） |
| `space-y-4` | 子要素間に `margin-top: 1rem;` | 子要素の縦方向の間隔 |
| `gap-4` | `gap: 1rem;` | Flex/Grid の子要素間の間隔 |

> **数値の法則:** Tailwind の数値は `0.25rem` (4px) 単位です。`p-1` = 4px、`p-2` = 8px、`p-4` = 16px、`p-8` = 32px のように、数値を4倍するとピクセル値になります。

#### 色（Colors）

| クラス | 説明 |
|--------|------|
| `text-gray-900` | 濃いグレーの文字色（ほぼ黒） |
| `text-gray-600` | 中間のグレーの文字色 |
| `text-gray-400` | 薄いグレーの文字色 |
| `text-blue-600` | 青い文字色（リンクなど） |
| `text-red-600` | 赤い文字色（エラーメッセージなど） |
| `text-green-600` | 緑の文字色（成功メッセージなど） |
| `text-white` | 白い文字色 |
| `bg-white` | 白い背景色 |
| `bg-gray-50` | ごく薄いグレーの背景色 |
| `bg-gray-100` | 薄いグレーの背景色 |
| `bg-blue-600` | 青い背景色（ボタンなど） |
| `bg-red-600` | 赤い背景色（削除ボタンなど） |
| `border-gray-200` | 薄いグレーのボーダー色 |
| `border-gray-300` | ボーダー色（フォーム入力欄など） |

> **色の濃さ:** `50` が最も薄く、`950` が最も濃い色です。`100` 刻みで用意されています。`gray-50`（ほぼ白）、`gray-500`（中間）、`gray-900`（ほぼ黒）のようなイメージです。

#### タイポグラフィ（Typography）

| クラス | CSS | 説明 |
|--------|-----|------|
| `text-xs` | `font-size: 0.75rem;` | 極小テキスト（12px） |
| `text-sm` | `font-size: 0.875rem;` | 小さいテキスト（14px） |
| `text-base` | `font-size: 1rem;` | 通常テキスト（16px） |
| `text-lg` | `font-size: 1.125rem;` | やや大きいテキスト（18px） |
| `text-xl` | `font-size: 1.25rem;` | 大きいテキスト（20px） |
| `text-2xl` | `font-size: 1.5rem;` | 見出し（24px） |
| `text-3xl` | `font-size: 1.875rem;` | 大見出し（30px） |
| `font-bold` | `font-weight: 700;` | 太字 |
| `font-semibold` | `font-weight: 600;` | やや太字 |
| `font-medium` | `font-weight: 500;` | やや太字（控えめ） |
| `font-normal` | `font-weight: 400;` | 通常の太さ |
| `text-center` | `text-align: center;` | テキスト中央揃え |
| `truncate` | `overflow: hidden; text-overflow: ellipsis; white-space: nowrap;` | テキストが長い場合に「...」で省略 |

#### レイアウト（Layout / Flexbox / Grid）

| クラス | CSS | 説明 |
|--------|-----|------|
| `flex` | `display: flex;` | Flexbox コンテナにする |
| `flex-col` | `flex-direction: column;` | 子要素を縦に並べる |
| `items-center` | `align-items: center;` | 交差軸方向に中央揃え |
| `justify-between` | `justify-content: space-between;` | 両端に配置 |
| `justify-center` | `justify-content: center;` | 主軸方向に中央揃え |
| `grid` | `display: grid;` | Grid コンテナにする |
| `grid-cols-1` | `grid-template-columns: repeat(1, 1fr);` | 1列のグリッド |
| `grid-cols-2` | `grid-template-columns: repeat(2, 1fr);` | 2列のグリッド |
| `grid-cols-3` | `grid-template-columns: repeat(3, 1fr);` | 3列のグリッド |
| `w-full` | `width: 100%;` | 横幅を親要素いっぱいに |
| `max-w-7xl` | `max-width: 80rem;` | 最大横幅（1280px） |
| `min-h-screen` | `min-height: 100vh;` | 最小高さを画面いっぱいに |
| `hidden` | `display: none;` | 要素を非表示にする |

#### ボーダー・角丸・影（Borders / Radius / Shadow）

| クラス | CSS | 説明 |
|--------|-----|------|
| `border` | `border-width: 1px;` | 1px のボーダーを追加 |
| `border-2` | `border-width: 2px;` | 2px のボーダーを追加 |
| `rounded` | `border-radius: 0.25rem;` | 少し角丸 |
| `rounded-md` | `border-radius: 0.375rem;` | 中程度の角丸 |
| `rounded-lg` | `border-radius: 0.5rem;` | 大きめの角丸 |
| `rounded-full` | `border-radius: 9999px;` | 完全な丸（バッジやアイコンに） |
| `shadow-sm` | 小さなシャドウ | 控えめな影 |
| `shadow` | 中程度のシャドウ | 標準的な影 |
| `shadow-lg` | 大きなシャドウ | 目立つ影 |

#### 状態変化（Hover / Focus）

| クラス | 説明 |
|--------|------|
| `hover:bg-blue-700` | マウスを載せたときに背景色を濃い青に変更 |
| `hover:text-blue-600` | マウスを載せたときに文字色を青に変更 |
| `hover:shadow-md` | マウスを載せたときに影を追加 |
| `focus:outline-none` | フォーカス時のデフォルトのアウトラインを消す |
| `focus:ring-2` | フォーカス時に2pxのリングを表示 |
| `focus:ring-blue-500` | フォーカスリングの色を青に設定 |
| `transition` | CSS トランジションを有効にする（スムーズなアニメーション） |
| `duration-200` | トランジション時間を200msに設定 |
| `cursor-pointer` | マウスカーソルをポインター（指）にする |
| `disabled:opacity-50` | 無効状態のときに半透明にする |
| `disabled:cursor-not-allowed` | 無効状態のときにカーソルを「禁止」にする |

### 2.3 レスポンシブデザイン

Tailwind CSS では、ブレークポイントのプレフィックスを付けることで、画面サイズに応じたスタイルを簡単に適用できます。

| プレフィックス | 最小幅 | 対象デバイス |
|---------------|-------|-------------|
| (なし) | 0px | すべて（モバイルファースト） |
| `sm:` | 640px | 大きめスマートフォン |
| `md:` | 768px | タブレット |
| `lg:` | 1024px | ノートPC |
| `xl:` | 1280px | デスクトップ |
| `2xl:` | 1536px | 大画面 |

**重要: Tailwind はモバイルファースト設計です。** プレフィックスなしのクラスがモバイル（最小画面）のスタイルになり、プレフィックス付きのクラスが「その画面サイズ以上」に適用されます。

#### 実例: レスポンシブなグリッドレイアウト

```tsx
{/* grid: グリッド表示 / grid-cols-1: 1列 / md:grid-cols-2: 768px以上で2列 / lg:grid-cols-3: 1024px以上で3列 / gap-4: 子要素間に16px間隔 */}
<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
  {/* 1つ目のセル */}
  <div>カード1</div>
  {/* 2つ目のセル */}
  <div>カード2</div>
  {/* 3つ目のセル */}
  <div>カード3</div>
</div>
```

この例では次のように表示が変わります。

| 画面サイズ | 適用されるクラス | 表示 |
|-----------|-----------------|------|
| 0px 〜 767px | `grid-cols-1` | 1列表示（カードが縦に並ぶ） |
| 768px 〜 1023px | `md:grid-cols-2` | 2列表示 |
| 1024px 〜 | `lg:grid-cols-3` | 3列表示 |

物差しの上に並べると、どの幅でどのクラスが効くのかがひと目で分かります。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<line x1="40" y1="56" x2="700" y2="56" stroke="#475569" stroke-width="2"/>
<line x1="40" y1="48" x2="40" y2="64" stroke="#475569" stroke-width="2"/>
<text x="40" y="40" text-anchor="middle" font-size="12" fill="#475569">0px</text>
<line x1="304" y1="48" x2="304" y2="64" stroke="#475569" stroke-width="2"/>
<text x="304" y="24" text-anchor="middle" font-size="12" font-weight="bold" fill="#1e40af">sm:</text>
<text x="304" y="40" text-anchor="middle" font-size="12" fill="#475569">640</text>
<line x1="357" y1="48" x2="357" y2="64" stroke="#475569" stroke-width="2"/>
<text x="357" y="24" text-anchor="middle" font-size="12" font-weight="bold" fill="#1e40af">md:</text>
<text x="357" y="40" text-anchor="middle" font-size="12" fill="#475569">768</text>
<line x1="462" y1="48" x2="462" y2="64" stroke="#475569" stroke-width="2"/>
<text x="462" y="24" text-anchor="middle" font-size="12" font-weight="bold" fill="#1e40af">lg:</text>
<text x="462" y="40" text-anchor="middle" font-size="12" fill="#475569">1024</text>
<line x1="568" y1="48" x2="568" y2="64" stroke="#475569" stroke-width="2"/>
<text x="568" y="24" text-anchor="middle" font-size="12" font-weight="bold" fill="#1e40af">xl:</text>
<text x="568" y="40" text-anchor="middle" font-size="12" fill="#475569">1280</text>
<line x1="674" y1="48" x2="674" y2="64" stroke="#475569" stroke-width="2"/>
<text x="674" y="24" text-anchor="middle" font-size="12" font-weight="bold" fill="#1e40af">2xl:</text>
<text x="674" y="40" text-anchor="middle" font-size="12" fill="#475569">1536</text>
<text x="700" y="82" text-anchor="end" font-size="12" fill="#64748b">画面の横幅 →</text>
<rect x="43" y="96" width="311" height="96" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<rect x="51" y="106" width="295" height="34" rx="4" fill="#ffffff" stroke="#3b82f6" stroke-width="1.5"/>
<rect x="51" y="146" width="295" height="16" rx="4" fill="#ffffff" stroke="#3b82f6" stroke-width="1.5" stroke-dasharray="4 3"/>
<text x="198" y="182" text-anchor="middle" font-size="12" font-weight="bold" fill="#1e40af">1列（スマホ）</text>
<text x="198" y="214" text-anchor="middle" font-size="12" fill="#1e40af" font-family="Consolas, Menlo, monospace">grid-cols-1</text>
<rect x="360" y="96" width="100" height="96" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<rect x="368" y="106" width="38" height="34" rx="4" fill="#ffffff" stroke="#f97316" stroke-width="1.5"/>
<rect x="414" y="106" width="38" height="34" rx="4" fill="#ffffff" stroke="#f97316" stroke-width="1.5"/>
<text x="410" y="182" text-anchor="middle" font-size="12" font-weight="bold" fill="#c2410c">2列</text>
<text x="410" y="214" text-anchor="middle" font-size="12" fill="#c2410c" font-family="Consolas, Menlo, monospace">md:grid-cols-2</text>
<rect x="465" y="96" width="232" height="96" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<rect x="473" y="106" width="67" height="34" rx="4" fill="#ffffff" stroke="#10b981" stroke-width="1.5"/>
<rect x="548" y="106" width="67" height="34" rx="4" fill="#ffffff" stroke="#10b981" stroke-width="1.5"/>
<rect x="622" y="106" width="67" height="34" rx="4" fill="#ffffff" stroke="#10b981" stroke-width="1.5"/>
<text x="581" y="182" text-anchor="middle" font-size="12" font-weight="bold" fill="#047857">3列（PC）</text>
<text x="581" y="214" text-anchor="middle" font-size="12" fill="#047857" font-family="Consolas, Menlo, monospace">lg:grid-cols-3</text>
<text x="360" y="240" text-anchor="middle" font-size="12" fill="#64748b">プレフィックス付きのクラスは「その幅以上」で効き、より広い幅のクラスが後から上書きする</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図6-3: ブレークポイントの物差しと、grid-cols-1 md:grid-cols-2 lg:grid-cols-3 の見え方</div>
</div>

### 2.4 「このクラスを付けるとこう表示される」具体例

実際にボタンを例にして、クラスを1つずつ追加していくとどう変わるか見てみましょう。

**Step 1: テキストだけ（クラスなし）**

```tsx
{/* 何もクラスを付けないボタン。ブラウザのデフォルト見た目になる */}
<button>書籍を登録する</button>
```

ブラウザデフォルトのボタンが表示されます。装飾はほぼなく、小さな灰色の枠線と白い背景のボタンです。

**Step 2: 背景色と文字色を追加**

```tsx
{/* bg-blue-600: 青い背景 / text-white: 白い文字 */}
<button className="bg-blue-600 text-white">書籍を登録する</button>
```

青い背景に白い文字のボタンになります。ただし、余白がなくテキストがギリギリです。

**Step 3: 余白を追加**

```tsx
{/* px-4: 左右の内側余白16px / py-2: 上下の内側余白8px */}
<button className="bg-blue-600 text-white px-4 py-2">書籍を登録する</button>
```

左右に16px、上下に8pxの余白が入り、見た目が改善されます。

**Step 4: 角丸を追加**

```tsx
{/* rounded-md: 中程度の角丸（border-radius: 6px） */}
<button className="bg-blue-600 text-white px-4 py-2 rounded-md">書籍を登録する</button>
```

角が丸くなり、モダンな印象になります。

**Step 5: ホバーエフェクトとトランジションを追加**

```tsx
{/* hover:bg-blue-700: マウスホバー時に少し濃い青へ */}
{/* transition: 変化をスムーズに / duration-200: 200ミリ秒かけて変化 */}
<button className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 transition duration-200">
  書籍を登録する
</button>
```

マウスを載せると背景色が少し濃くなり、200ms かけてスムーズに変化します。

**Step 6: フォーカスリングを追加**

```tsx
{/* focus:outline-none: デフォルトの黒い枠を消す */}
{/* focus:ring-2: フォーカス時に2pxのリング */}
{/* focus:ring-blue-500: リング色は青 */}
{/* focus:ring-offset-2: リングとボタンの間に2pxの隙間 */}
<button className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 transition duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2">
  書籍を登録する
</button>
```

Tab キーでフォーカスしたときに、ボタンの周りに青いリングが表示されます。これはアクセシビリティ（キーボード操作対応）のために重要です。

**完成形: 本チュートリアルで使うボタンスタイル**

```tsx
{/* font-medium: やや太字（500） */}
{/* disabled:opacity-50: disabled属性付きのときは半透明 */}
{/* disabled:cursor-not-allowed: disabled時はカーソルを「禁止」マークに */}
<button className="bg-blue-600 text-white px-4 py-2 rounded-md font-medium hover:bg-blue-700 transition duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed">
  書籍を登録する
</button>
```

---

## 3. 必要なパッケージのインストール

プロジェクトのディレクトリに移動してから、必要なパッケージをインストールします。Supabase とやり取りするための部品を**2つ**まとめて入れます。

```bash
# プロジェクトフォルダに移動
cd book-management

# npm install: package.json に依存パッケージを追加し、node_modules/ にダウンロード
# @supabase/supabase-js: Supabase 公式の JavaScript/TypeScript クライアントライブラリ
# @supabase/ssr: Next.js のようにサーバーとブラウザの両方で動くアプリ向けの補助ライブラリ
npm install @supabase/supabase-js @supabase/ssr
```

**▼ 実行結果**

```text
added 10 packages, and audited 374 packages in 18s

146 packages are looking for funding
  run `npm fund` for details

5 high severity vulnerabilities

To address all issues (including breaking changes), run:
  npm audit fix --force

Run `npm audit` for details.
```

`added 10 packages` は「2つ頼んだら、それぞれが必要とする部品も合わせて10個入った」という意味です。`vulnerabilities` の行は 1.4 で説明したものと同じ（ESLint の部品の話）なので、ここでも `npm audit fix --force` は実行しません。

### 3.1 各パッケージの役割

| パッケージ名 | 役割 | 説明 |
|-------------|------|------|
| `@supabase/supabase-js` | Supabase クライアント | Supabase（データベース、認証、ストレージ）と通信するための公式 JavaScript/TypeScript ライブラリです。`supabase.from("books").select("*")` のような書き方で、PostgreSQL データベースにデータの読み書きができます。 |
| `@supabase/ssr` | Next.js 用の補助 | `@supabase/supabase-js` を Next.js で使いやすくする公式ライブラリです。**ブラウザ用**（`createBrowserClient`）と**サーバー用**（`createServerClient`）の2種類のクライアントを作る関数が入っています。第7章以降で使う `@/lib/supabase/client` と `@/lib/supabase/server` は、この2つの関数で作ります（節5）。 |
| `next`（プリインストール済み） | Next.js フレームワーク | React ベースのフルスタックフレームワークです。ルーティング、サーバーサイドレンダリング、API ルートなどの機能を提供します。 |
| `react`（プリインストール済み） | React ライブラリ | UI コンポーネントを構築するためのライブラリです。`useState` や `useEffect` などのフック（Hook）を使って、動的な UI を作成します。 |
| `react-dom`（プリインストール済み） | React DOM | React コンポーネントをブラウザの DOM（画面）にレンダリングするためのライブラリです。 |
| `typescript`（プリインストール済み） | TypeScript | JavaScript に型システムを追加した言語です。開発時の型チェックに使われます。 |
| `tailwindcss` / `@tailwindcss/turbopack`（プリインストール済み） | Tailwind CSS | ユーティリティファーストの CSS フレームワーク（v4）と、それを Turbopack で処理するための部品です。`create-next-app` で Tailwind CSS を選択した場合、自動的にインストールされます。 |
| `eslint`（プリインストール済み） | ESLint | コードの品質チェックツールです。バグの原因になりやすいパターンを検出します。 |

> **追加で入れるのが Supabase の2つだけで十分な理由:**
> 本チュートリアルの書籍管理アプリでは、Supabase との通信がメインの外部依存です。UI は Tailwind CSS（プリインストール済み）で構築し、フォーム処理は React の標準機能で実現します。追加のUIライブラリやフォームライブラリは使用しないため、シンプルな構成を保てます。

### 3.2 パッケージがインストールされたことの確認

`package.json` を開き、`dependencies` に `@supabase/supabase-js` と `@supabase/ssr` が追加されていることを確認します。

```json
{
  "dependencies": {
    // 今インストールした Next.js 用の補助ライブラリ（^は後述）
    "@supabase/ssr": "^0.12.7",
    // 今インストールした Supabase クライアント
    "@supabase/supabase-js": "^2.118.0",
    // Next.js本体
    "next": "16.4.1",
    // React本体
    "react": "19.3.0",
    // React DOM
    "react-dom": "19.3.0"
  }
}
```

（数字は 2026年10月に実行したときのものです。あなたの環境ではもっと新しい数字になっていても問題ありません。）

#### 3.2.1 サンプル: 入った部品の版をコマンドで確かめる

> **▼ このコードがやること（先に日本語で）:** `package.json` には「この範囲の版ならOK」という希望が書かれているだけで、実際に `node_modules/` に入った版とは少し違うことがあります。`npm ls`（list の略）で、**実際に入っている版**と、部品同士の親子関係を表示します。

```bash
# 指定した部品が、実際にどの版で入っているかを木の形で表示する
npm ls @supabase/supabase-js @supabase/ssr
```

**▼ 実行結果**（1行目のパスは例）

```text
book-management@0.1.0 C:\Users\you\projects\book-management
├─┬ @supabase/ssr@0.12.7
│ └── @supabase/supabase-js@2.118.0 deduped
└── @supabase/supabase-js@2.118.0
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 木の形の読み方

```text
├─┬ @supabase/ssr@0.12.7
│ └── @supabase/supabase-js@2.118.0 deduped
```

- 1段目の `@supabase/ssr@0.12.7` は、あなたが直接入れた部品と、その版（0.12.7）です。
- その下に1段下がって `@supabase/supabase-js` が出ているのは、「`@supabase/ssr` も内部で `@supabase/supabase-js` を使っている」という親子関係を表しています。

##### 解説2: `deduped`（重複をまとめた）の意味

```text
│ └── @supabase/supabase-js@2.118.0 deduped
└── @supabase/supabase-js@2.118.0
```

- `deduped` は「同じ部品を2つ入れず、1つを共有している」という意味です。あなたが直接入れた `@supabase/supabase-js@2.118.0` を、`@supabase/ssr` も一緒に使っています。
- 何も表示されない、または `(empty)` と出る場合は、インストールに失敗しているか、別のフォルダで実行しています。`cd book-management` で正しいフォルダに入ってからもう一度試してください。

> **用語:** 「依存関係（dependency）」とは、ある部品が動くために必要な別の部品との関係のことです。`npm ls` はこの関係を木（ツリー）の形で見せてくれます。

> **`^` の意味:** `^2.49.1` は「メジャーバージョン 2 の範囲で最新版を許容する」という意味です。`2.49.1` から `2.99.99` まではインストール可能ですが、`3.0.0` はインストールされません。これにより、破壊的変更を避けつつ最新のバグ修正やパフォーマンス改善を受けられます。

> **セマンティックバージョニング（SemVer）の補足:**
> バージョン番号は `メジャー.マイナー.パッチ` の形（例: `2.49.1`）になっています。
> - **メジャー（2）**: 互換性のない変更（古い書き方が動かなくなる可能性あり）
> - **マイナー（49）**: 機能追加（後方互換あり）
> - **パッチ（1）**: バグ修正
>
> `^` は「マイナーとパッチの自動アップデートはOK、メジャーは固定」という意味のルールです。
>
> ただし、`@supabase/ssr` の `^0.12.7` のように**メジャーが 0 の場合**は特別で、「0.12.x の範囲（パッチだけ）」しか自動で上がりません。0 から始まる版は「まだ仕様が固まっていない段階」とされ、マイナーが上がるだけでも互換性が変わることがあるためです。

---

## 4. プロジェクト構成の作成

### 4.1 ディレクトリ構成

書籍管理アプリでは、以下のディレクトリ構成を採用します。

<div style="max-width:680px;margin:20px auto;font-family:'Segoe UI',sans-serif;">
  <!-- Root -->
  <div style="background:#e0f2fe;border:2px solid #0284c7;border-radius:10px;padding:12px 18px;text-align:center;font-weight:700;color:#0369a1;font-size:15px;margin-bottom:4px;">src/</div>
  <div style="display:flex;justify-content:center;gap:0;margin-bottom:4px;">
    <div style="border-left:2px solid #cbd5e1;height:18px;"></div>
  </div>
  <!-- 4 branches -->
  <div style="display:flex;gap:10px;flex-wrap:wrap;">
    <!-- app/ branch -->
    <div style="flex:1;min-width:150px;">
      <div style="background:#dbeafe;border:2px solid #2563eb;border-radius:10px;padding:10px 12px;text-align:center;font-weight:700;color:#1e40af;font-size:13px;margin-bottom:6px;">app/</div>
      <div style="margin-left:14px;border-left:2px solid #e2e8f0;">
        <div style="padding:4px 0 4px 12px;font-size:12px;color:#334155;">page.tsx <span style="color:#64748b;">(トップページ)</span></div>
        <div style="padding:4px 0 4px 12px;font-size:12px;color:#334155;">layout.tsx <span style="color:#64748b;">(ルートレイアウト)</span></div>
        <div style="padding:4px 0 4px 12px;font-size:12px;color:#334155;">globals.css</div>
        <div style="padding:6px 0 2px 12px;font-size:12px;font-weight:600;color:#1e40af;">books/</div>
        <div style="margin-left:16px;border-left:2px solid #e2e8f0;">
          <div style="padding:4px 0 2px 12px;font-size:12px;font-weight:600;color:#1e40af;">new/</div>
          <div style="padding:2px 0 4px 28px;font-size:12px;color:#334155;">page.tsx <span style="color:#64748b;">(書籍登録)</span></div>
          <div style="padding:4px 0 2px 12px;font-size:12px;font-weight:600;color:#1e40af;">[id]/</div>
          <div style="padding:2px 0 2px 28px;font-size:12px;color:#334155;">page.tsx <span style="color:#64748b;">(書籍詳細)</span></div>
          <div style="padding:2px 0 2px 28px;font-size:12px;font-weight:600;color:#1e40af;">edit/</div>
          <div style="padding:2px 0 4px 44px;font-size:12px;color:#334155;">page.tsx <span style="color:#64748b;">(書籍編集)</span></div>
        </div>
      </div>
    </div>
    <!-- components/ branch -->
    <div style="flex:1;min-width:150px;">
      <div style="background:#dcfce7;border:2px solid #16a34a;border-radius:10px;padding:10px 12px;text-align:center;font-weight:700;color:#166534;font-size:13px;margin-bottom:6px;">components/</div>
      <div style="margin-left:14px;border-left:2px solid #e2e8f0;">
        <div style="padding:4px 0 4px 12px;font-size:12px;color:#334155;">Header.tsx</div>
        <div style="padding:4px 0 4px 12px;font-size:12px;color:#334155;">BookCard.tsx</div>
        <div style="padding:4px 0 4px 12px;font-size:12px;color:#334155;">BookForm.tsx</div>
        <div style="padding:4px 0 4px 12px;font-size:12px;color:#334155;">BookList.tsx</div>
        <div style="padding:4px 0 4px 12px;font-size:12px;color:#334155;">DeleteButton.tsx</div>
        <div style="padding:4px 0 4px 12px;font-size:12px;color:#334155;">StatusBadge.tsx</div>
        <div style="padding:4px 0 4px 12px;font-size:12px;color:#334155;">RatingStars.tsx</div>
        <div style="padding:4px 0 4px 12px;font-size:12px;color:#334155;">LoadingSpinner.tsx</div>
      </div>
    </div>
    <!-- lib/ and types/ -->
    <div style="flex:0.6;min-width:130px;">
      <div style="background:#fef9c3;border:2px solid #ca8a04;border-radius:10px;padding:10px 12px;text-align:center;font-weight:700;color:#854d0e;font-size:13px;margin-bottom:6px;">lib/</div>
      <div style="margin-left:14px;border-left:2px solid #e2e8f0;">
        <div style="padding:4px 0 2px 12px;font-size:12px;font-weight:600;color:#854d0e;">supabase/</div>
        <div style="padding:2px 0 2px 28px;font-size:12px;color:#334155;">client.ts <span style="color:#64748b;">(ブラウザ用)</span></div>
        <div style="padding:2px 0 4px 28px;font-size:12px;color:#334155;">server.ts <span style="color:#64748b;">(サーバー用)</span></div>
      </div>
      <div style="margin-top:12px;background:#fce7f3;border:2px solid #db2777;border-radius:10px;padding:10px 12px;text-align:center;font-weight:700;color:#9d174d;font-size:13px;margin-bottom:6px;">types/</div>
      <div style="margin-left:14px;border-left:2px solid #e2e8f0;">
        <div style="padding:4px 0 4px 12px;font-size:12px;color:#334155;">book.ts <span style="color:#64748b;">(型定義)</span></div>
      </div>
    </div>
  </div>
</div>

テキスト形式でも示します。

```
src/
├── app/                            # App Router のルート。フォルダ名 = URL になる
│   ├── page.tsx                    # / にアクセスしたときのページ（トップ：書籍一覧）
│   ├── layout.tsx                  # 全ページ共通の枠（HTMLタグ、ヘッダーなど）
│   ├── globals.css                 # アプリ全体のグローバルCSS（Tailwindディレクティブ含む）
│   └── books/                      # /books 配下のページをまとめるフォルダ
│       ├── new/                    # /books/new ページ用のフォルダ
│       │   └── page.tsx            # 書籍登録ページ
│       └── [id]/                   # [id] は動的セグメント。/books/任意のID にマッチ
│           ├── page.tsx            # 書籍詳細ページ（/books/123 など）
│           └── edit/               # /books/[id]/edit ページ用のフォルダ
│               └── page.tsx        # 書籍編集ページ
├── components/                     # 再利用可能なUI部品を置くフォルダ
│   ├── Header.tsx                  # 共通ヘッダー（全ページ上部のナビ）
│   ├── BookCard.tsx                # 書籍カード（一覧の各アイテム）
│   ├── BookForm.tsx                # 書籍登録・編集フォーム
│   ├── BookList.tsx                # 書籍一覧表示（BookCardを並べる）
│   ├── DeleteButton.tsx            # 削除ボタン（確認ダイアログ付き）
│   ├── StatusBadge.tsx             # ステータスバッジ（読書中など）
│   ├── RatingStars.tsx             # 評価（星）表示
│   └── LoadingSpinner.tsx          # ローディング表示（くるくる回るやつ）
├── lib/                            # アプリ全体で使うユーティリティを置くフォルダ
│   └── supabase/                   # Supabase に接続するクライアントを作るファイルをまとめるフォルダ
│       ├── client.ts               # ブラウザで動く部品（Client Component）用
│       └── server.ts               # サーバーで動く部品（Server Component）用
└── types/                          # TypeScript 型定義を置くフォルダ
    └── book.ts                     # Book / BookInsert / BookUpdate などの型
```

### 4.2 各ファイルの役割

#### ページファイル（`app/` ディレクトリ）

| ファイル | URL パス | 役割 |
|----------|---------|------|
| `app/page.tsx` | `/` | トップページ。データベースから書籍の一覧を取得し、BookList コンポーネントで表示します。 |
| `app/layout.tsx` | 全ページ共通 | ルートレイアウト。`<html>` と `<body>` タグ、ヘッダーなどの全ページ共通要素を定義します。 |
| `app/globals.css` | 全ページ共通 | Tailwind CSS のディレクティブとカスタムスタイルを定義します。 |
| `app/books/new/page.tsx` | `/books/new` | 書籍登録ページ。BookForm コンポーネントを使い、新しい書籍の情報を入力・保存します。 |
| `app/books/[id]/page.tsx` | `/books/123` | 書籍詳細ページ。URL の `[id]` 部分に書籍の ID が入り、その書籍の詳細情報を表示します。 |
| `app/books/[id]/edit/page.tsx` | `/books/123/edit` | 書籍編集ページ。既存の書籍情報を読み込み、BookForm コンポーネントで編集・更新します。 |

> **`[id]` とは？（動的ルーティング）**
> フォルダ名を角括弧 `[]` で囲むと「動的ルート」になります。`/books/1`、`/books/2`、`/books/abc` のように、`[id]` の部分に任意の値が入ります。ページコンポーネントの `params` プロパティからその値を取得できます。

#### コンポーネントファイル（`components/` ディレクトリ）

| ファイル | 役割 |
|----------|------|
| `Header.tsx` | アプリ上部に表示されるナビゲーションバー。アプリ名と「書籍を登録する」リンクを含みます。全ページで共通して表示されます。 |
| `BookCard.tsx` | 書籍一覧の中の1冊分の表示カード。タイトル、著者、ステータスバッジ、評価（星）を表示します。クリックすると詳細ページに遷移します。 |
| `BookForm.tsx` | 書籍の登録・編集に使うフォーム。タイトル、著者、ステータス、評価、メモの入力欄を持ちます。新規登録と編集の両方で共通して使用します。 |
| `BookList.tsx` | BookCard を並べて一覧表示するコンポーネント。レスポンシブなグリッドレイアウトで配置します。書籍が0件の場合は「書籍が登録されていません」というメッセージを表示します。 |
| `DeleteButton.tsx` | 書籍を削除するボタン。クリック時に確認ダイアログを表示し、承諾した場合のみ削除を実行します。 |
| `StatusBadge.tsx` | 読書ステータス（「読みたい」「読書中」「読了」）を色付きバッジで表示するコンポーネント。ステータスに応じて色が変わります。 |
| `RatingStars.tsx` | 5段階の星評価を表示するコンポーネント。黄色い星と灰色の星で評価を視覚化します。 |
| `LoadingSpinner.tsx` | データ読み込み中に表示するスピナー（回転アニメーション）。API からのレスポンスを待っている間、ユーザーに読み込み中であることを伝えます。 |

#### ユーティリティファイル（`lib/` ディレクトリ）

| ファイル | 役割 |
|----------|------|
| `supabase/client.ts` | ブラウザ用の Supabase クライアントを作る `createClient()` 関数をエクスポートします。フォーム送信や削除ボタンなど、`"use client"` を付けた部品から `import { createClient } from "@/lib/supabase/client";` で使います。 |
| `supabase/server.ts` | サーバー用の Supabase クライアントを作る `createClient()` 関数をエクスポートします。トップページ（`app/page.tsx`）のような Server Component から `import { createClient } from "@/lib/supabase/server";` で使います。どちらも環境変数から Supabase の URL と API キーを読み取ります。 |

#### 型定義ファイル（`types/` ディレクトリ）

| ファイル | 役割 |
|----------|------|
| `book.ts` | 書籍データの TypeScript 型定義。`Book` 型（データベースから取得する完全なデータ）、`BookInsert` 型（新規登録時に送るデータ）、`BookUpdate` 型（更新時に送るデータ）を定義します。 |

#### 4.2.1 ファイル同士の「import 関係」を地図にする

表で役割を見ただけでは、「どのファイルがどのファイルを使うのか」がつかみにくいものです。会社にたとえると、`app/` のページは**お客さんの窓口**、`components/` は**窓口で使う道具**、`lib/supabase/` は**倉庫（データベース）への電話**、`types/book.ts` は**全員が見る伝票の書式**です。窓口は道具と電話を使い、道具も電話も伝票の書式に従います。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 410" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-07" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
<marker id="ar-06-07b" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#f97316"/></marker>
</defs>
<rect x="30" y="30" width="170" height="46" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="115" y="51" text-anchor="middle" font-size="13" font-weight="bold" fill="#1e40af">app/page.tsx</text>
<text x="115" y="68" text-anchor="middle" font-size="12" fill="#64748b">ページ（窓口）</text>
<rect x="520" y="30" width="170" height="46" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="605" y="51" text-anchor="middle" font-size="13" font-weight="bold" fill="#1e40af">app/layout.tsx</text>
<text x="605" y="68" text-anchor="middle" font-size="12" fill="#64748b">全ページの外枠</text>
<rect x="30" y="120" width="170" height="46" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="115" y="141" text-anchor="middle" font-size="13" font-weight="bold" fill="#047857">BookList.tsx</text>
<text x="115" y="158" text-anchor="middle" font-size="12" fill="#64748b">部品（一覧）</text>
<rect x="30" y="210" width="170" height="46" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="115" y="231" text-anchor="middle" font-size="13" font-weight="bold" fill="#047857">BookCard.tsx</text>
<text x="115" y="248" text-anchor="middle" font-size="12" fill="#64748b">部品（1冊分）</text>
<rect x="520" y="120" width="170" height="46" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="605" y="141" text-anchor="middle" font-size="13" font-weight="bold" fill="#047857">Header.tsx</text>
<text x="605" y="158" text-anchor="middle" font-size="12" fill="#64748b">部品（ヘッダー）</text>
<rect x="275" y="120" width="170" height="46" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="141" text-anchor="middle" font-size="13" font-weight="bold" fill="#c2410c">lib/supabase/</text>
<text x="360" y="158" text-anchor="middle" font-size="12" fill="#64748b">client.ts / server.ts</text>
<rect x="30" y="300" width="170" height="46" rx="8" fill="#fdf2f8" stroke="#db2777" stroke-width="2"/>
<text x="115" y="321" text-anchor="middle" font-size="13" font-weight="bold" fill="#9d174d">types/book.ts</text>
<text x="115" y="338" text-anchor="middle" font-size="12" fill="#64748b">データの形（型）</text>
<rect x="275" y="210" width="170" height="46" rx="8" fill="#f8fafc" stroke="#475569" stroke-width="2" stroke-dasharray="6 4"/>
<text x="360" y="231" text-anchor="middle" font-size="13" font-weight="bold" fill="#475569">@supabase/supabase-js</text>
<text x="360" y="248" text-anchor="middle" font-size="12" fill="#64748b">外部ライブラリ</text>
<rect x="520" y="210" width="170" height="46" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2" stroke-dasharray="6 4"/>
<text x="605" y="231" text-anchor="middle" font-size="13" font-weight="bold" fill="#c2410c">.env.local</text>
<text x="605" y="248" text-anchor="middle" font-size="12" fill="#64748b">process.env で読まれる</text>
<line x1="115" y1="76" x2="115" y2="118" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-07)"/>
<line x1="115" y1="166" x2="115" y2="208" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-07)"/>
<line x1="115" y1="256" x2="115" y2="298" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-07)"/>
<line x1="200" y1="62" x2="300" y2="118" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-07)"/>
<line x1="605" y1="76" x2="605" y2="118" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-07)"/>
<line x1="360" y1="166" x2="360" y2="208" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-07)"/>
<line x1="520" y1="226" x2="447" y2="160" stroke="#f97316" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar-06-07b)"/>
<line x1="250" y1="372" x2="290" y2="372" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-07)"/>
<text x="298" y="377" font-size="12" fill="#475569">A → B は「A が B を import して使う」</text>
<line x1="250" y1="392" x2="290" y2="392" stroke="#f97316" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar-06-07b)"/>
<text x="298" y="397" font-size="12" fill="#475569">設定値を読み込む（import ではない）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図6-4: ファイル同士の import 関係（矢印は上から下への一方通行）</div>
</div>

この図から、次の2つのルールが読み取れます。

- **矢印は上から下への一方通行**: ページ → 部品 → 型、の順に「使う」関係が流れます。逆向き（例: `types/book.ts` が `app/page.tsx` を import する）は作りません。逆向きの矢印ができると、どこを直すとどこが壊れるのか追いにくくなるからです。
- **データベースにつながる入口は `lib/supabase/` フォルダの1か所だけ**: ページや部品は、自分で接続を作らず、必ずこのフォルダの `createClient` を import して使います（サーバーで動くページは `server.ts`、ブラウザで動く部品は `client.ts`）。接続設定（`.env.local`）を読むのもここだけなので、接続まわりで問題が起きたら「まず `lib/supabase/` と `.env.local` を見る」と決めておけます。

### 4.3 ディレクトリとファイルの作成

以下のコマンドで必要なディレクトリとファイルを一括作成します。

> **▼ このコードがやること（先に日本語で）:** これから書いていくコードを置くための「フォルダ（ディレクトリ）」をまとめて作ります。`mkdir` はフォルダを作るコマンドで、`-p` を付けると途中の親フォルダもまとめて作ってくれます。`[id]` のように角括弧が入った特殊なフォルダ名はそのままだとシェルに誤解されるため、`\` で記号を打ち消して（エスケープして）います。うまくいかなければ、コメントにある通り VS Code から手作業で作ってもOKです。

```bash
# ディレクトリの作成
# mkdir: Make Directory の略。フォルダを作成するコマンド
# -p: 親フォルダが存在しなければ一緒に作る（既に存在してもエラーにならない）
mkdir -p src/app/books/new
# [id] は角括弧の意味を持つ特殊文字なので、シェルで誤解されないようにバックスラッシュでエスケープ
mkdir -p src/app/books/\[id\]/edit
mkdir -p src/components
mkdir -p src/lib/supabase
mkdir -p src/types
```

> **注意:** `[id]` は角括弧を含むため、シェルによってはエスケープが必要です。うまくいかない場合は、VS Code のエクスプローラーからフォルダを手動で作成してください。

> **▼ このコードがやること（先に日本語で）:** さきほど作ったフォルダの中に、これから編集していく「空っぽのファイル」を一気に用意します。`touch` は中身が空のファイルを作るコマンドで、中身は後の節で書き込んでいきます。Windows の PowerShell には `touch` がないので、その場合は下の「4.3.1」の PowerShell 版を使ってください。

```bash
# ファイルの作成（中身は後ほど記述）
# touch: 空のファイルを作成するコマンド（既に存在する場合は最終更新時刻だけ変える）
# 共通ヘッダー
touch src/components/Header.tsx
# 書籍カード
touch src/components/BookCard.tsx
# 書籍フォーム
touch src/components/BookForm.tsx
# 書籍一覧
touch src/components/BookList.tsx
# 削除ボタン
touch src/components/DeleteButton.tsx
# ステータスバッジ
touch src/components/StatusBadge.tsx
# 評価表示
touch src/components/RatingStars.tsx
# ローディング
touch src/components/LoadingSpinner.tsx
# Supabase クライアント（ブラウザ用）
touch src/lib/supabase/client.ts
# Supabase クライアント（サーバー用）
touch src/lib/supabase/server.ts
# Book型定義
touch src/types/book.ts
```

#### 4.3.1 サンプル: Windows の PowerShell で同じフォルダとファイルを作る

> **▼ このコードがやること（先に日本語で）:** 上の `mkdir -p` と `touch` は macOS / Linux（と Git Bash）向けの書き方です。Windows の PowerShell では、フォルダもファイルも `New-Item`（新しい項目を作る）というコマンドで作ります。カンマ `,` で区切ると、いくつもまとめて作れます。最後に `Get-ChildItem` で、作ったものを一覧表示して確かめます。

```powershell
# フォルダをまとめて作る（-Force: 途中のフォルダも作り、既にあってもエラーにしない）
# [id] を含むパスは "..." で囲めば、そのままの名前で作られる
New-Item -ItemType Directory -Force -Path "src/app/books/new", "src/app/books/[id]/edit", "src/components", "src/lib/supabase", "src/types" | Out-Null

# 空のファイルをまとめて作る（| Out-Null は、作るたびに出る長い表示を消すため）
New-Item -ItemType File -Path "src/components/Header.tsx", "src/components/BookCard.tsx", "src/components/BookForm.tsx", "src/components/BookList.tsx", "src/components/DeleteButton.tsx", "src/components/StatusBadge.tsx", "src/components/RatingStars.tsx", "src/components/LoadingSpinner.tsx", "src/lib/supabase/client.ts", "src/lib/supabase/server.ts", "src/types/book.ts" | Out-Null

# src の中身を、下の階層まで名前だけで一覧表示する
Get-ChildItem -Recurse -Name src
```

**▼ 実行結果**

```text
app
components
lib
types
app\books
app\globals.css
app\layout.tsx
app\page.tsx
app\books\new
app\books\[id]
app\books\[id]\edit
components\BookCard.tsx
components\BookForm.tsx
components\BookList.tsx
components\DeleteButton.tsx
components\Header.tsx
components\LoadingSpinner.tsx
components\RatingStars.tsx
components\StatusBadge.tsx
lib\supabase
lib\supabase\client.ts
lib\supabase\server.ts
types\book.ts
```

（`app\favicon.ico` も実際には表示されます。ここでは本章で作ったものが分かるよう、説明用に作った最小のフォルダで実行した結果を載せています。）

#### ▼ コードを1つずつ分解して解説

##### 解説1: `New-Item -ItemType Directory` でフォルダを作る

```powershell
New-Item -ItemType Directory -Force -Path "src/app/books/new", "src/app/books/[id]/edit"
```

- `-ItemType Directory` は「フォルダを作る」という指定です。`-Path` の後ろに作りたい場所を書きます。
- `-Force` を付けると、途中のフォルダ（`books` など）が無くても一緒に作り、すでにあってもエラーにしません。bash の `mkdir -p` と同じ働きです。
- `[id]` を含む名前も、`"..."` で囲めばそのまま作れます。

##### 解説2: ファイルを作るときは `-Force` を付けない

```powershell
New-Item -ItemType File -Path "src/types/book.ts" | Out-Null
```

- `-ItemType File` は「空のファイルを作る」という指定です。
- ここで `-Force` を付けないのはわざとです。**ファイルに `-Force` を付けると、すでにあるファイルの中身が空っぽに上書きされます**。書いたコードが消える事故を防ぐため、ファイルには付けません。
- 付けずに2回実行すると、2回目は次のエラーになります（1行目を抜粋）。ファイルは無事なので心配いりません。

```text
New-Item : The file 'C:\Users\you\projects\book-management\src\types\book.ts' already exists.
```

##### 解説3: `Get-ChildItem -Recurse -Name` で確かめる

```powershell
Get-ChildItem -Recurse -Name src
```

- `Get-ChildItem` はフォルダの中身を表示するコマンド（`ls` や `dir` と同じ）です。`-Recurse` で下の階層まで、`-Name` で名前だけを表示します。
- 区切りが `\`（Windows の書き方）になりますが、`/` で書いた場所と同じ意味です。

> **用語:** 「PowerShell（パワーシェル）」とは、Windows に最初から入っているコマンドを打つための道具です。VS Code のターミナルも、Windows では既定で PowerShell が開きます。

### 4.4 `@/` はどこを指す？（パスエイリアスの解決）

この章のコードでは、`import Header from "@/components/Header";` のように、先頭が `@/` のパスがたくさん出てきます。1.2 の質問⑨で「`@/*` をそのまま使う」と答えたので、`@/` は **`src/` フォルダの近道**として働きます。住所にたとえると、`@/` は「本社ビル（src）」という略称です。「本社ビルの3階（components）の Header さん」と書けば、どの支店から手紙を出しても同じ人に届きます。一方、相対パス（`../`）は「ここから2つ戻って右」という道案内なので、出発地点が変わるたびに書き直しが必要です。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 440" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-08" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
<marker id="ar-06-08b" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#f97316"/></marker>
<marker id="ar-06-08c" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#10b981"/></marker>
</defs>
<text x="20" y="22" font-size="13" font-weight="bold" fill="#1e293b">(A) @/ の置き換えは3ステップ</text>
<rect x="10" y="34" width="160" height="80" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="90" y="56" text-anchor="middle" font-size="12" fill="#64748b">① 書いた import</text>
<text x="90" y="78" text-anchor="middle" font-size="12" fill="#1e40af" font-family="Consolas, Menlo, monospace">import Header from</text>
<text x="90" y="96" text-anchor="middle" font-size="12" fill="#1e40af" font-family="Consolas, Menlo, monospace">"@/components/Header"</text>
<rect x="190" y="34" width="160" height="80" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="270" y="56" text-anchor="middle" font-size="12" fill="#64748b">② tsconfig.json の paths</text>
<text x="270" y="78" text-anchor="middle" font-size="12" fill="#c2410c" font-family="Consolas, Menlo, monospace">"@/*": ["./*"]</text>
<text x="270" y="98" text-anchor="middle" font-size="12" fill="#c2410c">@/ を ./ に置き換え</text>
<rect x="370" y="34" width="160" height="80" rx="8" fill="#f8fafc" stroke="#475569" stroke-width="2"/>
<text x="450" y="56" text-anchor="middle" font-size="12" fill="#64748b">③ 置き換え後のパス</text>
<text x="450" y="84" text-anchor="middle" font-size="12" fill="#475569" font-family="Consolas, Menlo, monospace">./components/Header</text>
<rect x="550" y="34" width="160" height="80" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="630" y="56" text-anchor="middle" font-size="12" fill="#64748b">④ 拡張子を補って発見</text>
<text x="630" y="84" text-anchor="middle" font-size="12" fill="#047857" font-family="Consolas, Menlo, monospace">components/</text>
<text x="630" y="102" text-anchor="middle" font-size="12" fill="#047857" font-family="Consolas, Menlo, monospace">Header.tsx</text>
<line x1="170" y1="74" x2="188" y2="74" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-08)"/>
<line x1="350" y1="74" x2="368" y2="74" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-08)"/>
<line x1="530" y1="74" x2="548" y2="74" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-08)"/>
<line x1="10" y1="136" x2="710" y2="136" stroke="#cbd5e1" stroke-width="1.5" stroke-dasharray="4 4"/>
<text x="20" y="162" font-size="13" font-weight="bold" fill="#1e293b">(B) 相対パスだと「../ で何階層のぼるか」を数える必要がある</text>
<text x="70" y="196" font-size="13" fill="#1e293b" font-family="Consolas, Menlo, monospace">./</text>
<text x="240" y="196" font-size="12" fill="#475569">← プロジェクト直下（book-management/）</text>
<text x="90" y="222" font-size="13" fill="#1e293b" font-family="Consolas, Menlo, monospace">app/</text>
<text x="110" y="248" font-size="13" fill="#1e293b" font-family="Consolas, Menlo, monospace">books/</text>
<text x="130" y="274" font-size="13" fill="#1e293b" font-family="Consolas, Menlo, monospace">[id]/</text>
<text x="150" y="300" font-size="13" fill="#1e293b" font-family="Consolas, Menlo, monospace">edit/</text>
<rect x="164" y="310" width="90" height="22" rx="4" fill="#eff6ff" stroke="#3b82f6" stroke-width="1.5"/>
<text x="170" y="326" font-size="13" fill="#1e40af" font-family="Consolas, Menlo, monospace">page.tsx</text>
<text x="262" y="326" font-size="12" fill="#1e40af">← ここで import を書く</text>
<text x="90" y="378" font-size="13" fill="#1e293b" font-family="Consolas, Menlo, monospace">components/</text>
<rect x="104" y="388" width="100" height="22" rx="4" fill="#ecfdf5" stroke="#10b981" stroke-width="1.5"/>
<text x="110" y="404" font-size="13" fill="#047857" font-family="Consolas, Menlo, monospace">Header.tsx</text>
<text x="212" y="404" font-size="12" fill="#047857">← 読み込みたいファイル</text>
<path d="M 205 296 C 225 290, 225 276, 205 270" fill="none" stroke="#f97316" stroke-width="2" marker-end="url(#ar-06-08b)"/>
<text x="230" y="288" font-size="12" fill="#c2410c">① ../</text>
<path d="M 185 270 C 205 264, 205 250, 185 244" fill="none" stroke="#f97316" stroke-width="2" marker-end="url(#ar-06-08b)"/>
<text x="210" y="262" font-size="12" fill="#c2410c">② ../</text>
<path d="M 165 244 C 185 238, 185 224, 165 218" fill="none" stroke="#f97316" stroke-width="2" marker-end="url(#ar-06-08b)"/>
<text x="190" y="236" font-size="12" fill="#c2410c">③ ../</text>
<path d="M 145 218 C 165 212, 165 198, 145 192" fill="none" stroke="#f97316" stroke-width="2" marker-end="url(#ar-06-08b)"/>
<text x="170" y="210" font-size="12" fill="#c2410c">④ ../</text>
<path d="M 64 191 L 50 191 L 50 373 L 84 373" fill="none" stroke="#10b981" stroke-width="2" marker-end="url(#ar-06-08c)"/>
<rect x="420" y="180" width="290" height="96" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="565" y="202" text-anchor="middle" font-size="12" font-weight="bold" fill="#c2410c">相対パスで書くと</text>
<text x="565" y="230" text-anchor="middle" font-size="12" fill="#c2410c" font-family="Consolas, Menlo, monospace">import Header from</text>
<text x="565" y="250" text-anchor="middle" font-size="12" fill="#c2410c" font-family="Consolas, Menlo, monospace">"../../../../components/Header";</text>
<rect x="420" y="296" width="290" height="96" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="565" y="318" text-anchor="middle" font-size="12" font-weight="bold" fill="#047857">@/ で書くと（どこからでも同じ）</text>
<text x="565" y="346" text-anchor="middle" font-size="12" fill="#047857" font-family="Consolas, Menlo, monospace">import Header from</text>
<text x="565" y="366" text-anchor="middle" font-size="12" fill="#047857" font-family="Consolas, Menlo, monospace">"@/components/Header";</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図X: パスエイリアス @/ が実際のファイルにたどり着くまで（A）と、相対パスとの比較（B）</div>
</div>

(A) のとおり、`@/` の置き換えは `tsconfig.json` の `"paths"` に書かれたルール（`"@/*": ["./src/*"]`）だけで決まります。Next.js も VS Code もこのルールを読んで、同じファイルにたどり着きます。拡張子（`.tsx` や `.ts`）は書かなくても自動で補われます。

#### 4.4.1 つまずきポイント: `@/` のあとに `src/` まで書いてしまう

> **つまずきポイント: `@/src/components/Header` と書いてしまう**
>
> 「ファイルは `src/components/Header.tsx` にあるから」と、`@/` のあとに `src/` まで書いてしまうミスはとても多いです。`@/` がすでに `src/` を意味しているので、`src/` が2回重なります。

NG（`src/` が2回になる）:

```typescript
// src/app/layout.tsx（一部）
// @/ は src/ のことなので、src/src/components/Header を探しに行ってしまう
import Header from "@/src/components/Header";
```

**▼ 実行結果**（`npm run build` の出力から抜粋）

```text
> Build error occurred
Error: Turbopack build failed with 1 error:
./src/app/layout.tsx:17:1
Error: Module not found: Can't resolve '@/src/components/Header'
  15 | // default import: Header コンポーネントを取り込む
  16 | // @/components/Header は src/components/Header（パスエイリアス @/ = src/）
> 17 | import Header from "@/src/components/Header";
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  18 |
  19 | /**
  20 |  * フォント設定

Import map: aliased to relative './src/src/components/Header' inside of [project]/


https://nextjs.org/docs/messages/module-not-found
```

`Module not found: Can't resolve '...'` は「そのファイルが見つかりません」という意味です。最後のほうの `aliased to relative './src/src/components/Header'` に、`@/` を置き換えた結果 `src/src/` になってしまったことがはっきり出ています。

OK（`@/` のあとは `src/` の中の場所から書く）:

```typescript
// src/app/layout.tsx（一部）
// @/ = src/ なので、src/ の中の components から書き始める
import Header from "@/components/Header";
```

> **ヒント（ほかの「見つからない」原因）:** 同じ `Module not found` は、次のようなときにも出ます。
>
> - **ファイル名の大文字・小文字の違い**: `@/components/header`（小文字の h）。Windows や macOS では動いてしまうことがありますが、本番（第10章の Vercel など Linux のサーバー）ではエラーになります。ファイル名と import の文字は一字一句そろえましょう。
> - **フォルダを `src/` の外に作った**: `book-management/components/Header.tsx` のように `src/` の外に置くと、`@/components/Header`（＝ `src/components/Header`）では見つかりません。VS Code のエクスプローラーで、`components` が `src` の中にあるか確認してください。


---

## 5. Supabase クライアントの設定

### 5.1 環境変数の取得

Supabase クライアントの設定には、以下の2つの情報が必要です。

1. **Project URL** -- あなたのプロジェクトの API エンドポイント（`https://xxxxxxxxxxxxxxxx.supabase.co` の形）
2. **Publishable key（パブリッシャブル・キー）** -- ブラウザに置いてもよい公開用の API キー（`sb_publishable_` で始まる文字列）

これらは Supabase のダッシュボードから取得できます。

**取得手順:**

1. [Supabase のダッシュボード](https://supabase.com/dashboard) にアクセスします
2. 書籍管理アプリ用のプロジェクト（第5章で作ったもの）を選択します
3. 画面上部の **「Connect」** ボタンをクリックします（左下の **「Project Settings」**（歯車アイコン）→ **「API Keys」** からも見られます）
4. 表示された画面から、次の2つをコピーします:
   - **Project URL**: `https://xxxxxxxxxxxxxxxx.supabase.co` の形式
   - **Publishable key**: `sb_publishable_` で始まる文字列

> **注意（ダッシュボードの表記）:** Supabase の画面は改良が続いていて、ボタンの位置や名前が変わることがあります。見つからないときは、プロジェクトの画面で「API Keys」や「Connect」という文字を探してください。

> **注意（古い anon key について）:** 以前の Supabase では、Publishable key の代わりに **anon key**（`eyJhbGciOi...` で始まる長い文字列）を使っていました。anon key は同じ「API Keys」の画面にまだあり、今も同じように使えますが、Supabase は **2026年末までに廃止する予定**です。これから作るプロジェクトでは Publishable key を使ってください。どちらを使っても、この後のコードは同じです。

> **注意（Secret key は絶対に使わない）:** 同じ画面に **Secret key**（`sb_secret_` で始まる）も表示されます。これは RLS（第5章で設定した行レベルのセキュリティ）を無視してすべてのデータを読み書きできる「合鍵」です。ブラウザに届く `NEXT_PUBLIC_` 付きの環境変数には**絶対に入れない**でください。本書のアプリでは Secret key は使いません。

### 5.2 .env.local の作成

プロジェクトのルートディレクトリ（`book-management/` の直下。`package.json` と同じ場所）に `.env.local` ファイルを作成します。

> **▼ このコードがやること（先に日本語で）:** Supabase（データベース）に接続するための「URL」と「鍵（キー）」を、コードの外側の設定ファイルに書き出します。こうした外部から渡す設定値を「環境変数」と呼び、`キー=値` の形で1行ずつ書きます。`NEXT_PUBLIC_` を頭に付けた変数だけがブラウザ側からも読めるようになる点がポイントです。`xxxxxxxx` の部分は、必ず自分の Supabase プロジェクトの値に置き換えてください。

```bash
# book-management/.env.local
# このファイルは ENV ファイルと呼ばれ、「キー=値」の形で環境変数を1行ずつ書く
# NEXT_PUBLIC_ プレフィックスを付けると Next.js がブラウザ側にも公開してくれる

# Supabase プロジェクトのURL（ダッシュボードからコピー）
NEXT_PUBLIC_SUPABASE_URL=https://xxxxxxxxxxxxxxxx.supabase.co

# Supabase の Publishable key（公開用のキー）。名前は第7章以降のコードに合わせて ANON_KEY にする
# 古い anon key（eyJ... で始まる長い文字列）を使う場合も、同じこの行に貼る
NEXT_PUBLIC_SUPABASE_ANON_KEY=sb_publishable_xxxxxxxxxxxxxxxxxxxxxxxx
```

> **なぜ変数名が `ANON_KEY` のままなのか:** 中身は Publishable key ですが、変数名は第7章以降のコード（`process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY`）に合わせて `NEXT_PUBLIC_SUPABASE_ANON_KEY` にしています。変数名は「コードとファイルで同じ名前なら何でもよい」ラベルにすぎないので、昔からの名前を使い続けても問題ありません。大切なのは、**`.env.local` の名前とコードの名前が一字一句そろっていること**です（下の 5.2.2 で、そろっていないときの症状を確かめます）。

> **`NEXT_PUBLIC_` プレフィックスの重要性:**
> Next.js では、環境変数に `NEXT_PUBLIC_` プレフィックスを付けると、ブラウザ側の JavaScript からもアクセスできるようになります。プレフィックスがない環境変数はサーバー側でのみ使用可能です。
>
> Supabase の Publishable key（や旧 anon key）はブラウザから直接 API を呼び出すために使うため、`NEXT_PUBLIC_` が必要です。このキーは Row Level Security（RLS）で守られる前提の公開用キーなので、ブラウザに届いても安全です（第5章で RLS を設定しました）。

`.env.local` に書いた値が、どうやってコードに届くのかを図にすると次のようになります。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 345" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="12" width="200" height="40" rx="6" fill="#f8fafc" stroke="#475569" stroke-width="1.5"/>
<text x="120" y="37" text-anchor="middle" font-size="13" font-weight="bold" fill="#475569">質問</text>
<rect x="235" y="12" width="210" height="40" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="1.5"/>
<text x="340" y="30" text-anchor="middle" font-size="13" font-weight="bold" fill="#c2410c">推奨設定（Enter 連打）</text>
<text x="340" y="46" text-anchor="middle" font-size="12" fill="#c2410c">Yes, use recommended defaults</text>
<rect x="460" y="12" width="240" height="40" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="1.5"/>
<text x="580" y="30" text-anchor="middle" font-size="13" font-weight="bold" fill="#047857">本書の設定（カスタマイズ）</text>
<text x="580" y="46" text-anchor="middle" font-size="12" fill="#047857">No, customize settings</text>
<rect x="20" y="60" width="680" height="26" rx="5" fill="#ffffff" stroke="#e2e8f0" stroke-width="1"/>
<text x="120" y="78" text-anchor="middle" font-size="12" fill="#1e293b" font-family="Consolas, Menlo, monospace">TypeScript</text>
<text x="340" y="78" text-anchor="middle" font-size="12" font-weight="normal" fill="#475569">Yes</text>
<text x="580" y="78" text-anchor="middle" font-size="12" font-weight="normal" fill="#475569">Yes</text>
<rect x="20" y="88" width="680" height="26" rx="5" fill="#ffffff" stroke="#e2e8f0" stroke-width="1"/>
<text x="120" y="106" text-anchor="middle" font-size="12" fill="#1e293b" font-family="Consolas, Menlo, monospace">Linter</text>
<text x="340" y="106" text-anchor="middle" font-size="12" font-weight="normal" fill="#475569">ESLint</text>
<text x="580" y="106" text-anchor="middle" font-size="12" font-weight="normal" fill="#475569">ESLint</text>
<rect x="20" y="116" width="680" height="26" rx="5" fill="#ffffff" stroke="#e2e8f0" stroke-width="1"/>
<text x="120" y="134" text-anchor="middle" font-size="12" fill="#1e293b" font-family="Consolas, Menlo, monospace">React Compiler</text>
<text x="340" y="134" text-anchor="middle" font-size="12" font-weight="normal" fill="#475569">No</text>
<text x="580" y="134" text-anchor="middle" font-size="12" font-weight="normal" fill="#475569">No</text>
<rect x="20" y="144" width="680" height="26" rx="5" fill="#ffffff" stroke="#e2e8f0" stroke-width="1"/>
<text x="120" y="162" text-anchor="middle" font-size="12" fill="#1e293b" font-family="Consolas, Menlo, monospace">Tailwind CSS</text>
<text x="340" y="162" text-anchor="middle" font-size="12" font-weight="normal" fill="#475569">Yes</text>
<text x="580" y="162" text-anchor="middle" font-size="12" font-weight="normal" fill="#475569">Yes</text>
<rect x="20" y="172" width="680" height="26" rx="5" fill="#ffffff" stroke="#e2e8f0" stroke-width="1"/>
<text x="120" y="190" text-anchor="middle" font-size="12" fill="#1e293b" font-family="Consolas, Menlo, monospace">src/ directory</text>
<text x="340" y="190" text-anchor="middle" font-size="12" font-weight="normal" fill="#475569">No</text>
<text x="580" y="190" text-anchor="middle" font-size="12" font-weight="normal" fill="#475569">No</text>
<rect x="20" y="200" width="680" height="26" rx="5" fill="#ffffff" stroke="#e2e8f0" stroke-width="1"/>
<text x="120" y="218" text-anchor="middle" font-size="12" fill="#1e293b" font-family="Consolas, Menlo, monospace">App Router</text>
<text x="340" y="218" text-anchor="middle" font-size="12" font-weight="normal" fill="#475569">Yes</text>
<text x="580" y="218" text-anchor="middle" font-size="12" font-weight="normal" fill="#475569">Yes</text>
<rect x="20" y="228" width="680" height="26" rx="5" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="120" y="246" text-anchor="middle" font-size="12" fill="#1e293b" font-family="Consolas, Menlo, monospace">Cache Components</text>
<text x="340" y="246" text-anchor="middle" font-size="12" font-weight="bold" fill="#c2410c">Yes</text>
<text x="580" y="246" text-anchor="middle" font-size="12" font-weight="bold" fill="#047857">No</text>
<rect x="20" y="256" width="680" height="26" rx="5" fill="#ffffff" stroke="#e2e8f0" stroke-width="1"/>
<text x="120" y="274" text-anchor="middle" font-size="12" fill="#1e293b" font-family="Consolas, Menlo, monospace">AGENTS.md</text>
<text x="340" y="274" text-anchor="middle" font-size="12" font-weight="normal" fill="#475569">Yes</text>
<text x="580" y="274" text-anchor="middle" font-size="12" font-weight="normal" fill="#475569">Yes</text>
<rect x="20" y="284" width="680" height="26" rx="5" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="120" y="302" text-anchor="middle" font-size="12" fill="#1e293b" font-family="Consolas, Menlo, monospace">Agent feedback</text>
<text x="340" y="302" text-anchor="middle" font-size="12" font-weight="bold" fill="#c2410c">Yes</text>
<text x="580" y="302" text-anchor="middle" font-size="12" font-weight="bold" fill="#047857">No</text>
<text x="360" y="330" text-anchor="middle" font-size="12" fill="#c2410c">違いは橙色の2行。Cache Components が Yes だと第7章で npm run build が失敗する</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図X: 推奨設定と本書の設定の違い（推奨設定のままだと Cache Components が有効になる）</div>
</div>

#### ▼ コードを1つずつ分解して解説

上の型定義を、初心者がつまずきやすい部分ごとに**1つずつ**ていねいに見ていきましょう。

---

##### 解説1: 決まった3つの文字列しか入らない「ユニオン型」

```typescript
export type BookStatus = "reading" | "completed" | "want_to_read";
```

- `type 名前 = ...` は、よく使うデータの形に**自分で名前を付ける**構文（型エイリアス）です。
- `"reading" | "completed" | "want_to_read"` の `|`（縦棒）は「または」を意味し、**この3つの文字列のどれか**しか入れられない、という制約になります。
- たとえば `"finished"` のような違う文字列を入れようとすると、コードを動かす前に TypeScript がエラーで教えてくれます。タイプミスを防げるのが利点です（6.3 で実際に試します）。
- 先頭の `export` で、他のファイルからこの `BookStatus` を使えるようにしています。

> **用語:** 「ユニオン型」とは、複数の候補のうちの**いずれか**を表す型です。決まった選択肢しか取らない値（ステータスや種別など）を安全に扱うのに向いています。

---

##### 解説2: 取得用の完全なデータ型 `Book`

```typescript
export type Book = {
  id: string;
  title: string;
  author: string;
  publisher: string | null;
  published_date: string | null;
  rating: number | null;
  status: BookStatus;
  memo: string | null;
  created_at: string;
  updated_at: string;
};
```

（コメントを省いた抜粋です。）

- `type Book = { ... }` は、**オブジェクト（複数の項目を持つデータ）の形**を定義しています。データベースの books テーブルの1行ぶんに対応します。
- 各行は `項目名: 型` の形で、「`title` は文字列」「`rating` は数値」のように、項目ごとに入る値の種類を決めています。
- `status: BookStatus` のように、解説1で作った型を**部品として再利用**できます。
- `number | null` は「数値、または null（値なし）」という意味で、未評価のときに `null` を許すための書き方です。`publisher`（出版社）・`published_date`（出版日）・`memo`（メモ）も、入力しなくてよい項目なので `| null` が付いています。
- `published_date` はデータベースでは日付（`date` 型）ですが、JavaScript には `"2024-01-15"` のような**文字列**で届くので `string` にしています。

> **用語:** 「null」とは「値が存在しない」ことを表す特別な値です。`number | null` のように書くと、「数値が入るか、まだ何も入っていないか」の両方を表現できます。

---

##### 解説3: 場面で型を分ける（Insert と Update）

```typescript
export type BookInsert = {
  title: string;
  author: string;
  publisher?: string | null;
  published_date?: string | null;
  rating?: number | null;
  status?: BookStatus;
  memo?: string | null;
};
```

（コメントを省いた抜粋です。）

- `BookInsert` は**新規登録のときに送るデータ**の型です。`id` や `created_at` はデータベースが自動で付けるので、ここには含めません。
- 項目名の後ろの `?`（クエスチョン）は「**省略してもよい**（オプショナル）」という印です。`status?` は、書かなくてもエラーにならず、DB 側のデフォルト値（`"want_to_read"`）が使われます。
- `title` と `author` には `?` が付いていません。この2つは**必ず送る**項目です。
- 一方 `BookUpdate` は更新用で、**全項目に `?` が付いています**。変更したい項目だけを送れば済むようにするためです。
- このように場面ごとに必要な項目を分けることで、「登録時に id を送ってしまう」「著者を送り忘れる」といった間違いを型で防げます。

> **用語:** 「オプショナル（`?`）」とは、そのプロパティが**あってもなくてもよい**という指定です。必須項目には付けず、省略を許したい項目にだけ付けます。

---

##### 解説4: ステータスごとの値をまとめる `Record` 型

```typescript
export const statusLabels: Record<BookStatus, string> = {
  reading: "読書中",
  completed: "読了",
  want_to_read: "読みたい",
};
```

（コメントを省いた抜粋です。）

- これは型ではなく**実際の値（定数）**です。`"reading"` のような内部用の文字列を、`"読書中"` という画面表示用の日本語に変換するための対応表です。
- `Record<BookStatus, string>` は「**キーが `BookStatus`、値が文字列**のオブジェクト」という型で、3つのステータス全部を漏れなく書くことを強制してくれます。
- 使うときは `statusLabels["reading"]` のように書くと `"読書中"` が取り出せます。
- 同じ仕組みの `statusColors` では、値が文字列ではなく `{ bg, text }`（背景色クラスと文字色クラス）のオブジェクトになっています。色は第7章の StatusBadge と同じ（読書中は青、読了は緑、読みたいは黄）です。

> **用語:** 「Record<K, V>」は TypeScript の組み込み型で、キーの型 `K` と値の型 `V` を指定して**対応表（マップ）の形**を作ります。選択肢ごとの設定値をまとめるのに便利です。

---

### 6.2 サンプル: ステータスと日本語ラベルの対応を動かしてみる

> **▼ このコードがやること（先に日本語で）:** 6.1 の `BookStatus` 型と `statusLabels` の対応表だけを取り出した小さなプログラムです。3つのステータスを順番に取り出し、それぞれの日本語ラベルを表示します。Next.js を起動しなくても、Node.js だけで動きを確かめられます。

```typescript
// scripts/status-demo.ts
// 読書ステータスの型と、日本語ラベルの対応表を小さく試すスクリプト

// 読書ステータスとして使ってよい3つの文字列だけを並べた型
type BookStatus = "reading" | "completed" | "want_to_read";

// ステータス → 日本語ラベルの対応表（3つすべて書かないと型エラーになる）
const statusLabels: Record<BookStatus, string> = {
  reading: "読書中",
  completed: "読了",
  want_to_read: "読みたい",
};

// 3つのステータスを順番に取り出してラベルを表示する
const all: BookStatus[] = ["reading", "completed", "want_to_read"];
for (const status of all) {
  console.log(`${status} → ${statusLabels[status]}`);
}
```

```bash
# scripts フォルダに保存して、Node.js でそのまま実行する
node scripts/status-demo.ts
```

**▼ 実行結果**

```text
reading → 読書中
completed → 読了
want_to_read → 読みたい
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `BookStatus[]` は「BookStatus が並んだ配列」

```typescript
const all: BookStatus[] = ["reading", "completed", "want_to_read"];
```

- 型の後ろに `[]` を付けると「その型の値がいくつも並んだ配列」という意味になります。
- 配列の中にも型のチェックが働くので、ここに `"finished"` を入れると型エラーになります。

##### 解説2: `statusLabels[status]` で対応表を引く

```typescript
for (const status of all) {
  console.log(`${status} → ${statusLabels[status]}`);
}
```

- `for (const status of all)` で、配列から1つずつ取り出して `status` に入れます。
- `statusLabels[status]` は、対応表から `status` というキーの値を取り出します。`status` が `"reading"` なら `"読書中"` です。
- `` `...${...}...` `` はテンプレートリテラルで、文字列の中に値を埋め込んでいます。

> **用語:** 「対応表（マップ）」とは、キーを渡すと値が返ってくるデータの形です。辞書で単語（キー）から意味（値）を引くのと同じです。

### 6.3 つまずきポイント: 古いステータス名や打ち間違いを入れてしまう

> **つまずきポイント: `node` で動いたから型も正しい、と思い込む**
>
> 古い記事やメモを見て、存在しないステータス（たとえば `"finished"`）を書いてしまうことがあります。第2章で学んだとおり、`node ファイル名.ts` は**型を消して動かすだけ**なので、型の間違いがあってもエラーにならず、おかしな結果のまま動いてしまいます。型の間違いを見つけるのは `tsc`（TypeScript のチェック役）や VS Code の赤線です。

NG（候補に無い `"finished"` を入れている）:

```typescript
// scripts/typo-demo.ts
// わざと型に合わない値を入れて、node と tsc の反応の違いを見るスクリプト

// 読書ステータスとして使ってよい3つの文字列
type BookStatus = "reading" | "completed" | "want_to_read";

// ステータス → 日本語ラベルの対応表
const statusLabels: Record<BookStatus, string> = {
  reading: "読書中",
  completed: "読了",
  want_to_read: "読みたい",
};

// わざと間違えた値（"finished" は BookStatus の候補に無い）
const myStatus: BookStatus = "finished";
console.log("ラベル:", statusLabels[myStatus]);
```

```bash
# まず node で実行する（型は見ない）
node scripts/typo-demo.ts
# 次に tsc で型だけをチェックする（--noEmit: JavaScript のファイルは作らない）
npx tsc --noEmit scripts/typo-demo.ts
```

**▼ 実行結果**

```text
ラベル: undefined
scripts/typo-demo.ts(15,7): error TS2322: Type '"finished"' is not assignable to type 'BookStatus'.
```

1行目は `node` の結果です。エラーにはならず、ラベルが `undefined`（値なし）になっています。画面に「undefined」と出てしまう、気づきにくいバグです。2行目は `tsc` の結果で、「15行目7文字目: `"finished"` という型は `BookStatus` に入れられません」と、原因の場所まで教えてくれています。

OK（候補にある値を使う）:

```typescript
// 読了は "completed"
const myStatus: BookStatus = "completed";
```

> **注意:** この `typo-demo.ts` は**試したら必ず削除**してください。`tsconfig.json` は `scripts/` の中の `.ts` も型チェックの対象にするので、間違いを残したままだと `npm run build`（型チェック付きの本番用ビルド）が止まります。

---

## 7. ルートレイアウトの設定

### 7.1 app/layout.tsx の作成

`src/app/layout.tsx` はすべてのページを囲む「外枠」のようなものです。HTML の `<html>` と `<body>` タグを含み、全ページ共通のヘッダーや CSS をここで設定します。

以下のコードで `src/app/layout.tsx` を書き換えます（`create-next-app` が生成したデフォルトの内容を完全に置き換えます）。

> **▼ このコードがやること（先に日本語で）:** すべてのページを包む「共通の外枠」を作ります。`<html>` や `<body>` タグ、日本語フォント、ブラウザのタブに出るタイトル、そして全ページ上部のヘッダーをここでまとめて設定します。各ページの中身は `{children}`（チルドレン：差し込み口）の場所にはめ込まれる、という仕組みがポイントです。フォントやレイアウトの細かい指定はコメントと解説表で1つずつ確認できます。

```typescript
// src/app/layout.tsx
// 全ページに適用される「ルートレイアウト」。<html> と <body> はここでしか書かない

// type import: Metadata 型だけをインポート（実体はビルド後のコードに残らない）
import type { Metadata } from "next";

// named import: next/font/google から Noto_Sans_JP 関数だけを取り出す
// next/font は Next.js が用意した、フォント最適化のための機能
import { Noto_Sans_JP } from "next/font/google";

// CSS のサイドエフェクトインポート（左辺なし）
// このファイルを読み込んだだけで CSS が適用される
import "./globals.css";

// default import: Header コンポーネントを取り込む
// @/components/Header は src/components/Header（パスエイリアス @/ = src/）
import Header from "@/components/Header";

/**
 * フォント設定
 *
 * Google Fonts から Noto Sans JP（日本語対応フォント）を読み込みます。
 * next/font/google を使うことで、フォントファイルがビルド時に
 * 自動的に最適化・セルフホスティングされます。
 * （外部の Google Fonts サーバーにリクエストが飛ばないため高速）
 */
const notoSansJP = Noto_Sans_JP({
  // 読み込むサブセット（文字種）。日本語はデフォルトで含まれる
  subsets: ["latin"],
  // 使用するフォントの太さ（通常 / やや太字 / 太字）
  weight: ["400", "500", "700"],
  // フォント読み込み中は代替フォントで表示し、完了後に切り替える
  display: "swap",
  // 重要なフォントなので、HTMLの <link rel="preload"> で先読みする
  preload: true,
});

/**
 * メタデータ設定
 *
 * ブラウザのタブに表示されるタイトルや、
 * 検索エンジンに表示される説明文を設定します。
 */
// export const metadata: Next.js が自動でこの変数を読み取って <head> に反映する
// 型は Metadata（先ほど type import したもの）
export const metadata: Metadata = {
  // <title> タグの中身（ブラウザのタブ表示）
  title: "書籍管理アプリ",
  // <meta name="description"> の中身（検索結果に出る説明文）
  description:
    "読んだ本、読んでいる本、これから読む本を管理するWebアプリケーション",
};

/**
 * ルートレイアウトコンポーネント
 *
 * すべてのページはこのレイアウトの中にレンダリングされます。
 * { children } には各ページのコンテンツが入ります。
 *
 * 構造:
 *   <html>
 *     <body>
 *       <Header />           ← 全ページ共通のヘッダー
 *       <main>{children}</main> ← 各ページの内容
 *     </body>
 *   </html>
 */
// export default function: ファイルの主要な関数として1つだけ default エクスポート
// RootLayout: コンポーネントの名前。Next.js が自動でこの関数を呼ぶ
// { children }: 分割代入で props から children を取り出している
// Readonly<{ children: React.ReactNode }>: 引数の型。children は読み取り専用、React の任意のノード
export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  // JSX を return する。1つの親要素（<html>）で全体を包む必要がある
  return (
    // lang="ja": HTML の言語属性を日本語に。スクリーンリーダーや検索エンジン用
    <html lang="ja">
      {/* body の className: notoSansJP のクラス名 + Tailwind の背景色と最小高さ */}
      {/* テンプレートリテラル `${...}` で文字列を結合 */}
      <body className={`${notoSansJP.className} bg-gray-50 min-h-screen`}>
        {/* 共通ヘッダーを最上部に配置 */}
        <Header />
        {/* main: ページのメインコンテンツ領域 */}
        {/* max-w-7xl: 最大幅1280px / mx-auto: 左右マージン自動で中央寄せ */}
        {/* px-4 sm:px-6 lg:px-8: 画面サイズに応じて左右パディングを変える（モバイル16px → タブレット24px → PC32px） */}
        {/* py-8: 上下パディング32px */}
        <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
          {/* children: 各ページの中身がここに差し込まれる */}
          {children}
        </main>
      </body>
    </html>
  );
}
```

**コードの解説:**

| 部分 | 解説 |
|------|------|
| `import type { Metadata } from "next";` | Next.js のメタデータ型をインポートします。`type` キーワードは「型のみインポート」を意味し、ランタイムには影響しません（ビルド時に消える）。 |
| `import { Noto_Sans_JP } from "next/font/google";` | Google Fonts から Noto Sans JP フォントをインポートします。`next/font` を使うと、フォントがビルド時にダウンロードされ、パフォーマンスが最適化されます。 |
| `import "./globals.css";` | Tailwind CSS のグローバルスタイルを読み込みます。この1行がないと Tailwind のクラスが機能しません。`./` は同じフォルダにあるファイルを指す相対パス。 |
| `import Header from "@/components/Header";` | 共通ヘッダーコンポーネントをインポートします。`@/` は `src/` ディレクトリのエイリアスです。これにより `../../components/Header` のように相対パスを書かなくて済みます。 |
| `subsets: ["latin"]` | ラテン文字のサブセットを読み込みます。日本語文字はデフォルトで含まれます。 |
| `weight: ["400", "500", "700"]` | 使用するフォントウェイト（太さ）を指定します。400 = 通常、500 = やや太字、700 = 太字。指定しない太さは使えなくなる代わりに、ダウンロードサイズが減ります。 |
| `display: "swap"` | フォントの読み込み方式を指定します。`"swap"` はフォント読み込み完了前にシステムフォントで表示し、読み込み後に切り替えます（FOIT: Flash Of Invisible Text を防ぐ）。 |
| `export const metadata` | ページのメタデータを定義します。`title` がブラウザのタブに表示され、`description` が検索エンジンの説明文に使われます。Next.js がこの export を自動的に検出して `<head>` タグに反映します。 |
| `lang="ja"` | HTML の言語を日本語に設定します。スクリーンリーダーや検索エンジンが正しく言語を認識できます。 |
| `bg-gray-50` | `<body>` の背景色をごく薄いグレーにします。真っ白よりも目に優しく、カードなどの白い要素が際立ちます。 |
| `min-h-screen` | `<body>` の最小高さを画面いっぱいにします。コンテンツが少ないページでも背景色が画面全体に適用されます。 |
| `max-w-7xl mx-auto` | `<main>` の最大幅を 1280px に制限し、左右の margin を auto にして中央寄せにします。大画面でもコンテンツが横に広がりすぎるのを防ぎます。 |
| `px-4 sm:px-6 lg:px-8` | 画面サイズに応じて左右の padding を変えます。モバイルでは 16px、タブレットでは 24px、PCでは 32px です。 |
| `{children}` | 各ページのコンテンツがここに挿入されます。`/` にアクセスすれば `app/page.tsx` の内容が、`/books/new` にアクセスすれば `app/books/new/page.tsx` の内容が入ります。 |

#### ▼ コードを1つずつ分解して解説

上のレイアウトを、初心者がつまずきやすい部分ごとに**1つずつ**ていねいに見ていきましょう。

---

##### 解説1: 3種類のインポートを使い分ける

```typescript
import type { Metadata } from "next";
import { Noto_Sans_JP } from "next/font/google";
import "./globals.css";
import Header from "@/components/Header";
```

- 1行目の `import type` は「**型だけ**を取り込む」インポートで、ビルド後のコードには残りません（型は実行時には不要なため）。
- 2行目の `{ Noto_Sans_JP }` は波括弧付きの「名前付きインポート」で、日本語フォントを読み込む関数を取り出しています。
- 3行目の `import "./globals.css";` は**左側に名前がない**特殊な形で、「このファイルを読み込むだけ」を意味します。これで Tailwind のスタイルが全ページに効きます。
- 4行目の `import Header` は波括弧なしの「デフォルトインポート」。`@/` は `src/` の近道（パスエイリアス）なので、`@/components/Header` は `src/components/Header` を指します。

> **用語:** 「パスエイリアス（`@/`）」とは、長い相対パス（`../../components/...`）を短く書くための別名です。`src/` を起点にできるので、ファイルを移動してもインポート文を直さずに済みます。

---

##### 解説2: フォントを最適化して読み込む

```typescript
const notoSansJP = Noto_Sans_JP({
  subsets: ["latin"],
  weight: ["400", "500", "700"],
  display: "swap",
  preload: true,
});
```

- `Noto_Sans_JP({ ... })` のように、フォント関数に**設定オブジェクト**を渡して呼び出します。戻り値を `notoSansJP` に保存し、あとで `<body>` に適用します。
- `weight: ["400", "500", "700"]` は使う太さ（通常・やや太字・太字）の指定で、使う分だけ読み込むことでファイルサイズを抑えます。
- `display: "swap"` は、フォント読み込み中はいったん代替フォントで表示し、読み込み完了後に切り替える設定です。文字が一瞬消える現象を防ぎます。
- `next/font` を使うとフォントがビルド時にダウンロードされて自前配信されるため、表示が速く安定します。

> **用語:** 「フォントの最適化」とは、必要な文字種・太さだけをまとめてダウンロードし、表示の遅延やレイアウトのガタつきを減らす仕組みのことです。

---

##### 解説3: メタデータでタブのタイトルなどを設定

```typescript
export const metadata: Metadata = {
  title: "書籍管理アプリ",
  description:
    "読んだ本、読んでいる本、これから読む本を管理するWebアプリケーション",
};
```

- `metadata` という名前で `export` した値は、**Next.js が自動で見つけて** HTML の `<head>` に反映してくれます（自分で `<title>` を書く必要がありません）。
- `title` はブラウザのタブに表示される文字、`description` は検索結果に出る説明文になります。
- `: Metadata` は「この値は `Metadata` という型に従う」という指定で、解説1で `import type` したものです。項目名を間違えると型チェックで気づけます。

> **用語:** 「メタデータ」とは、ページ自体の内容ではなく、ページに**付随する情報**（タイトル・説明・OGP など）のことです。検索エンジンや SNS が参照します。

---

##### 解説4: children に各ページが差し込まれる仕組み

```typescript
export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ja">
      <body className={`${notoSansJP.className} bg-gray-50 min-h-screen`}>
        <Header />
        <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
          {children}
        </main>
      </body>
    </html>
  );
}
```

- `RootLayout` は全ページを包む外枠コンポーネントです。引数の `{ children }` には、**表示中の各ページの中身**が自動的に入ってきます。
- `Readonly<{ children: React.ReactNode }>` は引数の型で、`React.ReactNode` は「React が表示できるあらゆる中身（要素・文字列など）」を表します。
- `className={`${notoSansJP.className} ...`}` の `` `${...}` `` はテンプレートリテラルで、フォントのクラス名と Tailwind のクラスを**つなげて1つの文字列**にしています。
- `<Header />` を常に上に置き、その下の `<main>` 内の `{children}` 部分だけがページごとに入れ替わります。これで全ページ共通の枠を1か所で管理できます。

> **用語:** 「children（チルドレン）」とは、コンポーネントの中に**差し込まれる中身**を受け取る特別な props です。レイアウトのように「枠だけ用意して中身は後から入れる」設計でよく使います。

この「枠」と「中身」の関係を図にすると次のようになります。額縁（layout.tsx）は1つだけで、飾る絵（page.tsx）だけが URL によって掛け替えられる、とイメージしてください。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 320" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-06-11" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="16" width="400" height="294" rx="10" fill="#f8fafc" stroke="#475569" stroke-width="2"/>
<text x="34" y="38" font-size="13" font-weight="bold" fill="#475569">layout.tsx が作る &lt;html&gt;（全ページで1つ）</text>
<rect x="36" y="50" width="368" height="246" rx="8" fill="#ffffff" stroke="#94a3b8" stroke-width="1.5"/>
<text x="48" y="70" font-size="12" fill="#64748b">&lt;body&gt;</text>
<rect x="52" y="80" width="336" height="48" rx="6" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="220" y="100" text-anchor="middle" font-size="13" font-weight="bold" fill="#047857">&lt;Header /&gt;（いつも同じ）</text>
<text x="220" y="119" text-anchor="middle" font-size="12" fill="#047857">書籍管理アプリ ／ 書籍一覧 ／ 書籍を登録する</text>
<rect x="52" y="140" width="336" height="144" rx="6" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="64" y="160" font-size="12" fill="#c2410c">&lt;main&gt;</text>
<rect x="80" y="172" width="280" height="96" rx="6" fill="#ffffff" stroke="#f97316" stroke-width="2" stroke-dasharray="6 4"/>
<text x="220" y="212" text-anchor="middle" font-size="16" font-weight="bold" fill="#c2410c">{children}</text>
<text x="220" y="236" text-anchor="middle" font-size="12" fill="#c2410c">ここだけが URL ごとに入れ替わる</text>
<rect x="480" y="60" width="220" height="70" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="590" y="86" text-anchor="middle" font-size="13" font-weight="bold" fill="#1e40af">URL が /</text>
<text x="590" y="110" text-anchor="middle" font-size="12" fill="#1e40af" font-family="Consolas, Menlo, monospace">app/page.tsx</text>
<rect x="480" y="200" width="220" height="70" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="590" y="226" text-anchor="middle" font-size="13" font-weight="bold" fill="#1e40af">URL が /books/new</text>
<text x="590" y="250" text-anchor="middle" font-size="12" fill="#1e40af" font-family="Consolas, Menlo, monospace">app/books/new/page.tsx</text>
<line x1="480" y1="95" x2="364" y2="200" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-11)"/>
<line x1="480" y1="235" x2="364" y2="228" stroke="#475569" stroke-width="2" marker-end="url(#ar-06-11)"/>
<text x="482" y="168" text-anchor="middle" font-size="12" fill="#475569">どちらか一方が入る</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図6-6: layout.tsx（額縁）と page.tsx（絵）の関係</div>
</div>

---

### 7.2 globals.css の確認

`src/app/globals.css` を開くと、`create-next-app` が作った次のような内容が入っています。

```css
@import "tailwindcss";

:root {
  --background: #ffffff;
  --foreground: #171717;
}

@theme inline {
  --color-background: var(--background);
  --color-foreground: var(--foreground);
  --font-sans: var(--font-geist-sans);
  --font-mono: var(--font-geist-mono);
}

@media (prefers-color-scheme: dark) {
  :root {
    --background: #0a0a0a;
    --foreground: #ededed;
  }
}

body {
  background: var(--background);
  color: var(--foreground);
  font-family: Arial, Helvetica, sans-serif;
}
```

先頭の `@import "tailwindcss";` 以外は、最初のサンプルページ用の設定（パソコンがダークモードのときに背景を黒くする、など）です。本書のアプリでは色やフォントを Tailwind のクラスと `layout.tsx` で指定するので、**ファイルの中身を次の1行（とコメント）だけに書き換えてください**。

```css
/* src/app/globals.css */

/* Tailwind CSS v4 を読み込む。この1行で、flex や bg-white などのクラスがすべて使えるようになる */
@import "tailwindcss";
```

> **なぜ残りを消すのか:** 残しておくと、パソコンがダークモードのときだけ文字色が白っぽくなり、白いカードの上の文字が読みにくくなることがあります。「自分のパソコンでは普通なのに、友達のパソコンでは見た目が違う」という原因の分かりにくい違いを防ぐため、最初に消しておきます（ダークモード対応は第9章で扱います）。

> **用語:** 「`@import`」は、CSS の中で別の CSS を読み込む命令です。`@import "tailwindcss";` は「Tailwind CSS の部品一式を読み込む」という意味で、Tailwind CSS v4 ではこの1行だけで準備が整います。

#### 7.2.1 つまずきポイント: 古い書き方（`@tailwind base;`）を書いてしまう

> **つまずきポイント: Tailwind CSS v3 時代の3行を書く**
>
> 古い記事では、`globals.css` に `@tailwind base;`、`@tailwind components;`、`@tailwind utilities;` の3行を書くよう説明されています。これは **Tailwind CSS v3** の書き方です。現在の create-next-app が入れる **v4** でこの3行を書くと、エラーは出ないのに**色や文字の大きさのクラスが効かなくなります**。

NG（v3 の書き方）:

```css
/* src/app/globals.css（v4 では NG） */
@tailwind base;
@tailwind components;
@tailwind utilities;
```

OK（v4 の書き方）:

```css
/* src/app/globals.css */
@import "tailwindcss";
```

本書の環境で、それぞれの書き方で `npm run build` して、できあがった CSS ファイルを調べた結果は次のとおりです。

| `globals.css` の書き方 | できた CSS の大きさ | `.flex` | `.bg-white` / `.bg-blue-50` / `.text-3xl` |
|------|------|------|------|
| NG: `@tailwind base;` などの3行 | 1,236 バイト | ある | **無い** |
| OK: `@import "tailwindcss";` | 12,332 バイト | ある | ある |

NG の書き方では、`flex`（横並び）のような一部のクラスだけが作られ、色（`bg-white` など）や文字サイズ（`text-3xl` など）のクラスが1つも作られませんでした。v4 では色や大きさの「見本帳（テーマ）」を `@import "tailwindcss";` で読み込む仕組みに変わったため、古い3行ではその見本帳が読み込まれないからです。「レイアウトは崩れていないのに、色も文字の大きさも付かない」ときは、まず `globals.css` の1行目を確認してください。

> **Tailwind CSS v3 の書き方について（参考）:**
> v3 では、`@tailwind base;`（リセット CSS と基本スタイル）、`@tailwind components;`（コンポーネント用クラス）、`@tailwind utilities;`（`flex` や `pt-4` などのユーティリティクラス）の3行と、`tailwind.config.ts`（設定ファイル）を組み合わせて使っていました。古い記事や、v3 で作られた既存のプロジェクトを読むときのために、「昔はこう書いていた」と知っておけば十分です。

---

## 8. 共通ヘッダーコンポーネント

### 8.1 components/Header.tsx の作成

`src/components/Header.tsx` ファイルに以下のコードを記述します。

> **▼ このコードがやること（先に日本語で）:** 全ページの上部に出る「ナビゲーションバー（共通ヘッダー）」を作ります。アプリ名のロゴと、「書籍一覧」「書籍を登録する」へのリンクを並べ、いま開いているページのリンクだけ色を変えて目立たせます。`usePathname()` という機能で現在のURLを取得するため、先頭に `"use client"` と書いてブラウザ側で動くコンポーネントにしている点がポイントです。スタイル（色や余白）の意味は各行のコメントを参照してください。

```typescript
// src/components/Header.tsx
// 全ページ上部に表示する共通ヘッダー（ナビゲーションバー）

// "use client": このファイルを Client Component として宣言する特別な文字列
// 必ずファイルの一番上（importよりも前）に書く
// これがあるとブラウザで実行され、React フックや状態管理が使える
"use client";

// default import: Next.js の Link コンポーネント
// HTML の <a> よりも高速にページ遷移できる（SPA的な切り替え）
import Link from "next/link";

// named import: usePathname フック（現在のURLパスを取得）
// 注意: import元は "next/navigation"（App Router用）。Pages Router の "next/router" とは別物
import { usePathname } from "next/navigation";

/**
 * 共通ヘッダーコンポーネント
 *
 * すべてのページの上部に表示されるナビゲーションバーです。
 * - アプリ名（クリックでトップページに遷移）
 * - 「書籍を登録する」ボタン
 * を含みます。
 *
 * "use client" を使用する理由:
 * usePathname() フックを使用するため、Client Component にする必要があります。
 * Server Component では React のフック（useState, useEffect, usePathname 等）は使えません。
 */
// export default: このファイルを import するときの主要な対象
// function Header(): 関数コンポーネントの宣言。引数なし、JSX を返す
export default function Header() {
  // 現在のURLパスを取得（アクティブなナビゲーションリンクのスタイル変更に使用）
  // usePathname() の戻り値は "/" や "/books/new" のような文字列
  const pathname = usePathname();

  // JSX を return（ヘッダー全体の構造）
  return (
    // <header>: HTML5 のセマンティックタグ。ページ上部のヘッダーを示す
    // bg-white: 白い背景 / shadow-sm: 控えめな影 / border-b: 下方向に1pxのボーダー / border-gray-200: ボーダー色
    <header className="bg-white shadow-sm border-b border-gray-200">
      {/* 内側の div: ヘッダーの中身を中央寄せ＆最大幅制限する */}
      {/* max-w-7xl: 最大幅1280px / mx-auto: 左右マージンautoで中央寄せ */}
      {/* px-4 sm:px-6 lg:px-8: 画面サイズに応じて左右パディング */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* flex 行: ロゴと右側ナビを横並びに、両端配置、高さ64px */}
        {/* flex: フレックスボックス / items-center: 垂直中央揃え */}
        {/* justify-between: 左右に均等配置（ロゴが左、ナビが右） / h-16: 高さ64px */}
        <div className="flex items-center justify-between h-16">
          {/* アプリ名（ロゴ） */}
          {/* Link: ページリロードなしで遷移する Next.js のコンポーネント */}
          {/* href: 遷移先のパス */}
          <Link
            href="/"
            // text-xl: フォントサイズ20px / font-bold: 太字 / text-gray-900: 濃いグレー
            // hover:text-blue-600: ホバー時に青 / transition duration-200: 200msで滑らかに変化
            className="text-xl font-bold text-gray-900 hover:text-blue-600 transition duration-200"
          >
            書籍管理アプリ
          </Link>

          {/* ナビゲーション */}
          {/* nav: HTML5 のナビゲーション用セマンティックタグ */}
          {/* space-x-4: 子要素間に16pxの水平方向マージン */}
          <nav className="flex items-center space-x-4">
            {/* 書籍一覧リンク */}
            <Link
              href="/"
              // テンプレートリテラル ${} で動的にクラスを切り替える
              // pathname === "/" が true なら青系（現在地ハイライト）、false ならグレー系
              className={`px-3 py-2 rounded-md text-sm font-medium transition duration-200 ${
                pathname === "/"
                  // 現在地：薄青背景＋青文字
                  ? "bg-blue-50 text-blue-700"
                  // 通常：グレー文字、ホバーで反応
                  : "text-gray-600 hover:text-gray-900 hover:bg-gray-50"
              }`}
            >
              書籍一覧
            </Link>

            {/* 書籍登録ボタン */}
            <Link
              href="/books/new"
              // px-4: 左右パディング16px / py-2: 上下パディング8px / rounded-md: 角丸
              // text-sm: 文字サイズ14px / font-medium: やや太字 / transition: 滑らか変化
              className={`px-4 py-2 rounded-md text-sm font-medium transition duration-200 ${
                pathname === "/books/new"
                  // 現在地：濃い青背景＋白文字
                  ? "bg-blue-700 text-white"
                  // 通常：青背景、ホバーで濃く
                  : "bg-blue-600 text-white hover:bg-blue-700"
              }`}
            >
              書籍を登録する
            </Link>
          </nav>
        </div>
      </div>
    </header>
  );
}
```

**コードの解説:**

| 部分 | 解説 |
|------|------|
| `"use client";` | このファイルを Client Component として宣言します。Next.js の App Router では、デフォルトですべてのコンポーネントが Server Component（サーバーで実行）です。しかし `usePathname()` などの React フックを使うには Client Component にする必要があります。ファイルの先頭にこの宣言を書くだけで切り替えられます。 |
| `import Link from "next/link";` | Next.js のリンクコンポーネントです。HTML の `<a>` タグの代わりに使います。ページ遷移時にページ全体をリロードせず、必要な部分だけを更新するため高速です。 |
| `import { usePathname } from "next/navigation";` | 現在のURLパスを取得するフックです。例えば `/books/new` にいるときは `"/books/new"` が返ります。 |
| `pathname === "/"` | 現在のページがトップページかどうかを判定し、条件に応じてスタイルを切り替えます。現在いるページのリンクには背景色を付けてハイライト表示します。 |
| `bg-white shadow-sm border-b border-gray-200` | ヘッダーの背景を白にし、下に薄いシャドウとボーダーを付けてコンテンツ部分との区切りを明確にします。 |
| `max-w-7xl mx-auto` | ヘッダー内のコンテンツの最大幅を 1280px にし、中央寄せにします。`layout.tsx` の `<main>` と同じ幅にすることで整った見た目になります。 |
| `flex items-center justify-between h-16` | Flexbox で子要素を横並びにし、垂直方向に中央揃え、左右に均等配置、高さ64px を設定します。 |
| `space-x-4` | ナビゲーションリンク間に 16px の水平方向の間隔を設けます。 |

#### ▼ コードを1つずつ分解して解説

上のヘッダーを、初心者がつまずきやすい部分ごとに**1つずつ**ていねいに見ていきましょう。

---

##### 解説1: なぜ先頭に "use client" が必要か

```typescript
"use client";
```

- この1行は、ファイルを「**Client Component（ブラウザ側で動く部品）**」として宣言する特別な合図です。必ず import より前、ファイルの一番上に書きます。
- Next.js の App Router では、何も書かないとコンポーネントは**サーバー側で動く**設定になります。サーバー側では `useState` などの React の機能や、URL を取る `usePathname()` が使えません。
- このヘッダーは現在地に応じて見た目を変えるため `usePathname()` を使います。そのために `"use client"` を付けてブラウザ側で動くようにしています。

> **用語:** 「Client Component」とは、ユーザーのブラウザ上で実行されるコンポーネントです。クリックや入力などの操作、React フックを使いたい部品はこちらにします。

---

##### 解説2: Link と usePathname を取り込む

```typescript
import Link from "next/link";
import { usePathname } from "next/navigation";
```

- `Link` は、ページを**まるごと再読み込みせずに**素早く切り替えるための Next.js 専用のリンク部品です。HTML の `<a>` の代わりに使います。
- `usePathname` は「**いま開いている URL のパス**」（例: `"/"` や `"/books/new"`）を取得するフックです。
- import 元に注意が必要で、`usePathname` は App Router 用の `"next/navigation"` から取り込みます（旧方式の `"next/router"` とは別物です）。

> **用語:** 「フック（Hook）」とは、`use` で始まる React の特別な関数です。状態や現在の URL など、コンポーネントに機能を「引っかけて」追加するために使います。

---

##### 解説3: 現在のパスを取得する

```typescript
const pathname = usePathname();
```

- `usePathname()` を呼ぶと、今表示しているページの URL パスが文字列で返ってきます。それを `pathname` に保存します。
- この値を後で `pathname === "/"` のように比較して、「いま見ているページのリンクだけ色を変える」ハイライト表示に使います。
- ページを移動すると `pathname` の値も自動で変わり、ヘッダーの見た目も更新されます。

> **用語:** 「パス（pathname）」とは、URL のうちドメイン名より後ろの部分（`/books/new` など）のことです。どのページにいるかを判別する手がかりになります。

---

##### 解説4: 現在地によってクラスを切り替える

```typescript
<Link
  href="/"
  className={`px-3 py-2 rounded-md text-sm font-medium transition duration-200 ${
    pathname === "/"
      ? "bg-blue-50 text-blue-700"
      : "text-gray-600 hover:text-gray-900 hover:bg-gray-50"
  }`}
>
  書籍一覧
</Link>
```

- `className={`...${ ... }`}` のテンプレートリテラルで、**共通のクラス**と**条件で変わるクラス**を1つの文字列に合体させています。
- `条件 ? Aの値 : Bの値` は三項演算子（条件分岐の短い書き方）で、「`pathname === "/"` が真なら A、偽なら B」を選びます。
- いまトップページにいれば薄い青背景＋青文字（`bg-blue-50 text-blue-700`）でハイライトし、そうでなければグレー文字でホバー時に反応するスタイルになります。
- `href="/"` で遷移先を指定し、クリックすると `Link` が高速にトップページへ切り替えます。

> **用語:** 「三項演算子（`条件 ? A : B`）」とは、`if/else` を1行で書ける短縮形です。値を2択で切り替えたいとき、JSX の中などで手軽に使えます。

### 8.2 つまずきポイント: `"use client"` を書き忘れる

> **つまずきポイント: `usePathname` を使うのに `"use client"` が無い**
>
> `"use client"` は1行だけの地味な文字列なので、書き忘れたり、コピーのときに抜けたりしがちです。また、`import` 文より下に書いてしまうと、宣言として認められません。

NG（`"use client"` が無い）:

```typescript
// src/components/Header.tsx（先頭部分）
// "use client" が無いので、このファイルは Server Component として扱われる
import Link from "next/link";
import { usePathname } from "next/navigation";
```

この状態で `npm run build` すると、次のエラーで止まります（開発サーバーでも、ブラウザに同じ内容のエラー画面が出ます）。

**▼ 実行結果**（`npm run build` の出力から抜粋）

```text
> Build error occurred
Error: Turbopack build failed with 1 error:
./src/components/Header.tsx:15:10
Error: You're importing a module that depends on `usePathname` into a React Server Component module. This API is only available in Client Components. To fix, mark the file (or its parent) with the `"use client"` directive.
    Learn more: https://nextjs.org/docs/app/api-reference/directives/use-client
  13 | // named import: usePathname フック（現在のURLパスを取得）
  14 | // 注意: import元は "next/navigation"（App Router用）。Pages Router の "next/router" とは別物
> 15 | import { usePathname } from "next/navigation";
     |          ^^^^^^^^^^^
  16 |
  17 | /**
  18 |  * 共通ヘッダーコンポーネント

Ecmascript file had an error

Import trace:
  Server Component:
    ./src/components/Header.tsx
    ./src/app/layout.tsx
```

英文は「`usePathname` に頼るモジュールを、React Server Component に import しています。この機能は Client Component でしか使えません。直すには、ファイル（またはその親）に `"use client"` を付けてください」という意味です。最後の `Import trace`（読み込みの道すじ）からは、`layout.tsx`（Server Component）が `Header.tsx` を読み込み、その `Header.tsx` も Server Component のまま `usePathname` を使っている、という流れが読み取れます。

OK（ファイルの一番上、import より前に書く）:

```typescript
// src/components/Header.tsx（先頭部分）
// "use client" は必ずファイルの一番上（import より前）に書く
"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
```

> **ヒント:** エラーの1行目に出るファイル名と行番号（`./src/components/Header.tsx:15:10` は「15行目の10文字目」）を見れば、どこを直せばよいかが分かります。エラーは怖がらずに、まず「どのファイルの何行目か」だけでも読む習慣をつけましょう。

---

## 9. 開発サーバーの起動と動作確認

### 9.1 仮のトップページを作成

動作確認のため、`src/app/page.tsx` を以下の内容に書き換えます（`create-next-app` のデフォルト内容を置き換えます）。

> **▼ このコードがやること（先に日本語で）:** トップページ（URL が `/` のページ）に、動作確認用の仮の画面を表示します。見出しと説明文、白いカード、3色のステータスバッジを並べて、Tailwind CSS のスタイルがちゃんと効いているかを目で確かめられるようにしています。中身はまだ仮で、本物の書籍一覧は次の章で作る、という位置づけです。クラス名（`bg-white` など）の意味は各行のコメントにあります。

```typescript
// src/app/page.tsx
// URL "/" にアクセスしたときに表示されるトップページ

// export default function: ファイルのメインコンポーネントとしてエクスポート
// HomePage: コンポーネント名（任意の名前でOK。Next.jsはdefault exportを自動的にページとして扱う）
export default function HomePage() {
  // JSX を return（このページの中身）
  return (
    // 全体を div で囲む。複数の要素を return するには親要素が必要
    <div>
      {/* h1: 大見出し / text-3xl: 30px / font-bold: 太字 / text-gray-900: 濃いグレー / mb-4: 下マージン16px */}
      <h1 className="text-3xl font-bold text-gray-900 mb-4">
        書籍一覧
      </h1>
      {/* p: 段落 / text-gray-600: 中間グレー / mb-8: 下マージン32px */}
      <p className="text-gray-600 mb-8">
        登録された書籍がここに表示されます。
      </p>

      {/* 動作確認用: Tailwind CSS のスタイルが正しく適用されるかチェック */}
      {/* bg-white: 白背景 / rounded-lg: 角丸大 / shadow-sm: 控えめな影 */}
      {/* border border-gray-200: 1pxの薄いグレーボーダー / p-6: 内側パディング24px */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        {/* h2: 中見出し / text-lg: 18px / font-semibold: やや太字（600） */}
        <h2 className="text-lg font-semibold text-gray-800 mb-2">
          セットアップ完了
        </h2>
        {/* text-sm: 14px の小さめテキスト */}
        <p className="text-gray-600 text-sm">
          Next.js + Tailwind CSS + Supabase の開発環境が正しくセットアップされました。
          次の章で、このページに実際の書籍一覧を表示します。
        </p>
        {/* バッジを横並びにする領域 */}
        {/* mt-4: 上マージン16px / flex: 横並び / space-x-2: 子要素間に8px間隔 */}
        <div className="mt-4 flex space-x-2">
          {/* 読みたいバッジ：黄系 */}
          {/* inline-block: インラインブロック表示 / bg-yellow-100: 薄い黄の背景 / text-yellow-800: 濃い黄の文字 */}
          {/* text-xs: 12px / px-2.5 py-1: パディング / rounded-full: 完全な丸（カプセル形） / font-medium: 中太字 */}
          <span className="inline-block bg-yellow-100 text-yellow-800 text-xs px-2.5 py-1 rounded-full font-medium">
            読みたい
          </span>
          {/* 読書中バッジ：青系 */}
          <span className="inline-block bg-blue-100 text-blue-800 text-xs px-2.5 py-1 rounded-full font-medium">
            読書中
          </span>
          {/* 読了バッジ：緑系 */}
          <span className="inline-block bg-green-100 text-green-800 text-xs px-2.5 py-1 rounded-full font-medium">
            読了
          </span>
        </div>
      </div>
    </div>
  );
}
```

#### ▼ コードを1つずつ分解して解説

上の仮トップページを、初心者がつまずきやすい部分ごとに**1つずつ**ていねいに見ていきましょう。

---

##### 解説1: ページは default export の関数で作る

```typescript
export default function HomePage() {
  return (
    <div>
      ...
    </div>
  );
}
```

- `app/page.tsx` の中で `export default` した関数が、その URL（ここでは `/`）に表示される**ページの本体**になります。Next.js が自動でこの関数を探して呼び出します。
- 関数名（`HomePage`）は自由に決められます。重要なのは名前ではなく「`export default` であること」です。
- `return ( ... )` の中に画面の中身（JSX）を書きます。複数の要素を返すときは、必ず1つの親要素（ここでは外側の `<div>`）で包む必要があります。

> **用語:** 「default export」とは、1ファイルにつき1つだけ指定できる「主役の書き出し」です。Next.js のページやコンポーネントは、これでファイルの代表を決めます。

---

##### 解説2: Tailwind のクラスで見出しと説明を装飾

```typescript
<h1 className="text-3xl font-bold text-gray-900 mb-4">
  書籍一覧
</h1>
<p className="text-gray-600 mb-8">
  登録された書籍がここに表示されます。
</p>
```

- JSX では HTML の `class` 属性のかわりに `className` を使います（`class` は JavaScript の予約語のため）。
- `className` に Tailwind のクラスを**スペース区切りで並べる**と、それぞれが1つのスタイルになります。`text-3xl`（文字サイズ30px）、`font-bold`（太字）、`text-gray-900`（濃いグレー）など。
- `mb-4` / `mb-8` は下方向の余白（margin-bottom）で、数字が大きいほど余白も広がります（4=16px、8=32px）。

> **用語:** 「className」とは、JSX で CSS クラスを指定するための属性名です。HTML の `class` と同じ役割ですが、React では `className` と書く決まりになっています。

---

##### 解説3: カードとステータスバッジを並べる

```typescript
<div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
  <h2 className="text-lg font-semibold text-gray-800 mb-2">
    セットアップ完了
  </h2>
  ...
  <div className="mt-4 flex space-x-2">
    <span className="inline-block bg-yellow-100 text-yellow-800 text-xs px-2.5 py-1 rounded-full font-medium">
      読みたい
    </span>
    ...
  </div>
</div>
```

- 外側の `<div>` が**白いカード**です。`bg-white`（白背景）、`rounded-lg`（角丸）、`shadow-sm`（薄い影）、`border`（枠線）、`p-6`（内側の余白24px）を組み合わせて作っています。
- バッジを囲む `<div>` の `flex space-x-2` で、3つのバッジを**横並び**にし、間に8pxの隙間を空けています。
- 各バッジ（`<span>`）の `rounded-full` でカプセル型の丸みを付け、`bg-yellow-100`/`bg-blue-100`/`bg-green-100` のように背景色を変えてステータスを色分けしています（6.1 の `statusColors` と同じ色です）。
- これは動作確認用の仮表示で、Tailwind が正しく効いているか（色・角丸・影が出るか）を目で確かめる役割です。

> **用語:** 「Flexbox（`flex`）」とは、子要素を横や縦に整列させるための CSS のレイアウト方式です。`space-x-2` のような間隔指定と組み合わせて、並びを手軽に整えられます。

---

### 9.2 開発サーバーの起動

ターミナルで、**`book-management` フォルダの中にいることを確かめてから**以下のコマンドを実行します。

```bash
# npm run dev: package.json の "scripts" にある "dev" を実行
# 中身は「next dev」で、Next.js の開発サーバーが起動する（Turbopack が標準で使われる）
# ファイルを保存するたびに自動でブラウザが更新される（ホットリロード）
# 停止するには Ctrl+C を押す
npm run dev
```

以下のような出力が表示されます（2026年10月に実際に実行したときの表示。`Network` の行の数字はパソコンごとに違います）。

```text
> book-management@0.1.0 dev
> next dev

▲ Next.js 16.4.1 (Turbopack)
- Local:         http://localhost:3000
- Network:       http://192.168.x.x:3000
- Environments: .env.local
✓ Ready in 7.0s
✓ Running next.config.ts took 80ms

○ Compiling / ...
 GET / 200 in 398ms (next.js: 201ms, application-code: 197ms)
```

最後の2行は、ブラウザで `http://localhost:3000` を開いたときに追加で出る行です。各行の意味を画面イメージに書き込むと、次のようになります。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 350" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="6" width="455" height="338" rx="10" fill="#0f172a" stroke="#475569" stroke-width="2"/>
<rect x="10" y="6" width="455" height="28" rx="10" fill="#1e293b"/><rect x="10" y="24" width="455" height="10" fill="#1e293b"/>
<circle cx="30" cy="20" r="5" fill="#f87171"/><circle cx="48" cy="20" r="5" fill="#fbbf24"/><circle cx="66" cy="20" r="5" fill="#34d399"/>
<text x="250" y="25" text-anchor="middle" font-size="12" fill="#94a3b8">ターミナル</text>
<text x="22" y="56" font-size="12" fill="#e2e8f0" font-family="Consolas, Menlo, monospace" xml:space="preserve" style="white-space: pre">PS C:\Users\you\projects\book-management&gt; npm run dev</text>
<text x="22" y="98" font-size="12" fill="#94a3b8" font-family="Consolas, Menlo, monospace" xml:space="preserve" style="white-space: pre">&gt; book-management@0.1.0 dev</text>
<text x="22" y="119" font-size="12" fill="#94a3b8" font-family="Consolas, Menlo, monospace" xml:space="preserve" style="white-space: pre">&gt; next dev</text>
<text x="22" y="161" font-size="12" fill="#f8fafc" font-family="Consolas, Menlo, monospace" xml:space="preserve" style="white-space: pre">▲ Next.js 16.4.1 (Turbopack)</text>
<line x1="468" y1="157" x2="488" y2="157" stroke="#f97316" stroke-width="1.5"/>
<text x="494" y="161" font-size="12" fill="#c2410c">使っている Next.js の版</text>
<text x="22" y="182" font-size="12" fill="#22d3ee" font-family="Consolas, Menlo, monospace" xml:space="preserve" style="white-space: pre">- Local:         http://localhost:3000</text>
<line x1="468" y1="178" x2="488" y2="178" stroke="#f97316" stroke-width="1.5"/>
<text x="494" y="182" font-size="12" fill="#c2410c">ブラウザで開く URL</text>
<text x="22" y="203" font-size="12" fill="#94a3b8" font-family="Consolas, Menlo, monospace" xml:space="preserve" style="white-space: pre">- Network:       http://192.168.x.x:3000</text>
<line x1="468" y1="199" x2="488" y2="199" stroke="#f97316" stroke-width="1.5"/>
<text x="494" y="203" font-size="12" fill="#c2410c">同じ Wi-Fi の別端末用</text>
<text x="22" y="224" font-size="12" fill="#fbbf24" font-family="Consolas, Menlo, monospace" xml:space="preserve" style="white-space: pre">- Environments: .env.local</text>
<line x1="468" y1="220" x2="488" y2="220" stroke="#f97316" stroke-width="1.5"/>
<text x="494" y="224" font-size="12" fill="#c2410c">.env.local を読めた合図</text>
<text x="22" y="245" font-size="12" fill="#4ade80" font-family="Consolas, Menlo, monospace" xml:space="preserve" style="white-space: pre">✓ Ready in 7.0s</text>
<line x1="468" y1="241" x2="488" y2="241" stroke="#f97316" stroke-width="1.5"/>
<text x="494" y="245" font-size="12" fill="#c2410c">準備完了</text>
<text x="22" y="266" font-size="12" fill="#4ade80" font-family="Consolas, Menlo, monospace" xml:space="preserve" style="white-space: pre">✓ Running next.config.ts took 80ms</text>
<text x="22" y="308" font-size="12" fill="#e2e8f0" font-family="Consolas, Menlo, monospace" xml:space="preserve" style="white-space: pre">○ Compiling / ...</text>
<line x1="468" y1="304" x2="488" y2="304" stroke="#f97316" stroke-width="1.5"/>
<text x="494" y="308" font-size="12" fill="#c2410c">初めて開いたページを変換中</text>
<text x="22" y="329" font-size="12" fill="#e2e8f0" font-family="Consolas, Menlo, monospace" xml:space="preserve" style="white-space: pre"> GET / 200 in 398ms (next.js: 201ms, …)</text>
<line x1="468" y1="325" x2="488" y2="325" stroke="#f97316" stroke-width="1.5"/>
<text x="494" y="329" font-size="12" fill="#c2410c">200 = 表示に成功</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図X: npm run dev を実行したときのターミナルの画面イメージ（右は各行の意味）</div>
</div>

> **ヒント:** 開発サーバーは、**ターミナルを開いている間ずっと動き続ける**プログラムです。このターミナルでは他のコマンドを打てなくなるので、別のコマンド（`npm install` など）を実行したいときは、VS Code のターミナルの「＋」で**新しいターミナルを開いて**そちらで実行してください。止めるときは、開発サーバーが動いているターミナルで `Ctrl+C` を押します。

### 9.3 ブラウザでの確認

ブラウザで `http://localhost:3000` を開くと、以下のように表示されます。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 400" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="6" width="700" height="388" rx="10" fill="#ffffff" stroke="#475569" stroke-width="2"/>
<rect x="10" y="6" width="700" height="58" rx="10" fill="#e2e8f0"/><rect x="10" y="50" width="700" height="14" fill="#e2e8f0"/>
<circle cx="30" cy="22" r="5" fill="#f87171"/><circle cx="48" cy="22" r="5" fill="#fbbf24"/><circle cx="66" cy="22" r="5" fill="#34d399"/>
<rect x="90" y="12" width="160" height="22" rx="6" fill="#ffffff"/>
<text x="102" y="28" font-size="12" fill="#1e293b">書籍管理アプリ</text>
<rect x="24" y="38" width="672" height="22" rx="11" fill="#ffffff" stroke="#cbd5e1"/>
<text x="40" y="53" font-size="12" fill="#475569" font-family="Consolas, Menlo, monospace">localhost:3000</text>
<rect x="11" y="64" width="698" height="329" fill="#f9fafb"/>
<rect x="11" y="64" width="698" height="44" fill="#ffffff" stroke="#e5e7eb"/>
<text x="34" y="92" font-size="16" font-weight="bold" fill="#111827">書籍管理アプリ</text>
<rect x="520" y="74" width="66" height="26" rx="5" fill="#eff6ff"/>
<text x="553" y="92" text-anchor="middle" font-size="12" font-weight="bold" fill="#1d4ed8">書籍一覧</text>
<rect x="596" y="74" width="100" height="26" rx="5" fill="#2563eb"/>
<text x="646" y="92" text-anchor="middle" font-size="12" font-weight="bold" fill="#ffffff">書籍を登録する</text>
<text x="34" y="146" font-size="22" font-weight="bold" fill="#111827">書籍一覧</text>
<text x="34" y="172" font-size="12" fill="#4b5563">登録された書籍がここに表示されます。</text>
<rect x="34" y="190" width="652" height="104" rx="8" fill="#ffffff" stroke="#e5e7eb" stroke-width="1.5"/>
<text x="52" y="218" font-size="14" font-weight="bold" fill="#1f2937">セットアップ完了</text>
<text x="52" y="242" font-size="12" fill="#4b5563">Next.js + Tailwind CSS + Supabase の開発環境が正しくセットアップされました。</text>
<rect x="52" y="256" width="58" height="22" rx="11" fill="#fef9c3"/><text x="81" y="271" text-anchor="middle" font-size="12" fill="#854d0e">読みたい</text>
<rect x="116" y="256" width="52" height="22" rx="11" fill="#dbeafe"/><text x="142" y="271" text-anchor="middle" font-size="12" fill="#1e40af">読書中</text>
<rect x="174" y="256" width="40" height="22" rx="11" fill="#dcfce7"/><text x="194" y="271" text-anchor="middle" font-size="12" fill="#166534">読了</text>
<line x1="160" y1="100" x2="288" y2="114" stroke="#f97316" stroke-width="1.5" stroke-dasharray="4 3"/>
<rect x="146" y="114" width="284" height="24" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="1.5"/>
<text x="288" y="130" text-anchor="middle" font-size="12" fill="#c2410c">① Header.tsx（layout.tsx に置いた共通部分）</text>
<line x1="400" y1="294" x2="395" y2="312" stroke="#f97316" stroke-width="1.5" stroke-dasharray="4 3"/>
<rect x="250" y="312" width="290" height="24" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="1.5"/>
<text x="395" y="328" text-anchor="middle" font-size="12" fill="#c2410c">② page.tsx の中身（{children} に入る部分）</text>
<line x1="553" y1="101" x2="568" y2="114" stroke="#f97316" stroke-width="1.5" stroke-dasharray="4 3"/>
<rect x="440" y="114" width="256" height="24" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="1.5"/>
<text x="568" y="130" text-anchor="middle" font-size="12" fill="#c2410c">③ 現在地の「書籍一覧」が薄い青になる</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図X: http://localhost:3000 を開いたときの画面イメージ（実際の表示を元に作図）</div>
</div>

**ヘッダー部分:**
- 画面最上部に白い背景のヘッダーバーが表示されます
- 左側に「書籍管理アプリ」というテキストが太字で表示されます
- 右側に「書籍一覧」というテキストリンクと「書籍を登録する」という青いボタンが表示されます
- 「書籍一覧」リンクは現在のページなので、薄い青の背景でハイライトされています

**メインコンテンツ部分:**
- ヘッダーの下に薄いグレー（`bg-gray-50`）の背景があります
- 「書籍一覧」という大きな見出し（30px, 太字）が表示されます
- その下に「登録された書籍がここに表示されます。」という説明文があります
- 白い角丸のカード内に「セットアップ完了」というメッセージが表示されます
- カードの下部に3つのステータスバッジ（「読みたい」は黄、「読書中」は青、「読了」は緑）が横並びで表示されます

**確認ポイント:**

| 確認項目 | 期待される状態 |
|---------|---------------|
| ヘッダーが表示される | 白い背景のバーが画面上部にある |
| Tailwind CSS が適用されている | テキストに色やサイズが適用され、カードに角丸やシャドウがある |
| フォント（Noto Sans JP）が適用されている | 日本語テキストが美しいフォントで表示される |
| レスポンシブ対応 | ブラウザの幅を狭めても、レイアウトが崩れない |
| ナビゲーションリンクが動作する | 「書籍を登録する」ボタンをクリックすると `/books/new` に遷移する（まだページを作成していないため 404 が表示されますが、URL が変わることを確認） |

### 9.4 よくあるトラブルと対処法

| 症状 | 原因 | 対処法 |
|------|------|--------|
| `Module not found: Can't resolve '@/components/Header'` | ファイルが存在しない、またはパスが間違っている | `src/components/Header.tsx` が正しい場所に作成されているか確認してください。ファイル名の大文字小文字も一致している必要があります。`@/src/...` と書いていないかも確認します（4.4.1）。 |
| スタイルが全く適用されない | `globals.css` が正しく読み込まれていない | `src/app/layout.tsx` で `import "./globals.css";` が記述されているか、`globals.css` の1行目が `@import "tailwindcss";` か確認してください（7.2.1）。 |
| `環境変数 NEXT_PUBLIC_SUPABASE_URL / NEXT_PUBLIC_SUPABASE_ANON_KEY が設定されていません` | `.env.local` が未作成、名前・場所が違う、変数名が違う | `.env.local` がプロジェクトのルートディレクトリ（`package.json` と同じ階層）にあり、起動時に `- Environments: .env.local` が出ているか確認してください。保存し直すと `Reload env` で読み直されますが、迷ったら `Ctrl+C` で止めて `npm run dev` で起動し直すのが確実です（5.2.2〜5.2.4）。 |
| `⚠ Port 3000 is in use ... using available port 3001 instead.` | 3000 番を別のアプリが使っている | Next.js 16 は空いている番号（3001 など）に**自動で切り替えて**起動します。表示された `Local:` の URL（`http://localhost:3001` など）をブラウザで開いてください。 |
| `⨯ Another next dev server is already running.` | 同じフォルダで開発サーバーを2つ起動しようとした | すでに動いている方を使います（9.4.1）。 |
| `npm error enoent Could not read package.json` | プロジェクトのフォルダの外で実行している | `cd book-management` でフォルダに入ってから実行します（9.4.2）。 |
| 日本語フォントが適用されていない | フォントの読み込みに失敗している | ネットワーク接続を確認してください。初回の起動・ビルド時に Google Fonts からフォントがダウンロードされます。 |

#### 9.4.1 つまずきポイント: 開発サーバーを2つ起動してしまう

> **つまずきポイント: 「動いていないのかな？」と、別のターミナルでもう一度 `npm run dev` する**
>
> 開発サーバーは、起動したターミナルの中でずっと動き続けます。別のターミナルを開いて、同じフォルダでもう一度 `npm run dev` を実行すると、Next.js 16 はそれを見つけて止まります。

NG（同じフォルダで2つ目を起動する）:

```bash
# 1つ目がまだ動いているのに、別のターミナルでもう一度起動する
npm run dev
```

**▼ 実行結果**（パスとプロセス番号は例）

```text
> book-management@0.1.0 dev
> next dev

⚠ Port 3000 is in use by process 9076, using available port 3001 instead.
▲ Next.js 16.4.1 (Turbopack)
- Local:         http://localhost:3001
- Network:       http://192.168.x.x:3001
- Environments: .env.local
✓ Ready in 832ms
✓ Running next.config.ts took 42ms
⨯ Another next dev server is already running.

- Local:        http://localhost:3000
- PID:          9076
- Dir:          C:\Users\you\projects\book-management
- Log:          .next\dev\logs\next-development.log

You can access the existing server at http://localhost:3000,
or run taskkill /PID 9076 /F to stop it and start a new one.
```

まず「3000 番は使われているので 3001 番にします」と言いかけますが、すぐに「**同じフォルダの開発サーバーがもう動いています**（Another next dev server is already running）」と気づいて止まります。最後の2行は「すでに動いているサーバーに `http://localhost:3000` でアクセスできます。止めて新しく起動したいなら `taskkill /PID 9076 /F` を実行してください」という意味です（`taskkill` は Windows でプログラムを止めるコマンド、PID はプログラムの番号です）。

OK（すでに動いている方を使う）:

```text
1. VS Code のターミナルの一覧から、npm run dev が動いているターミナルを探す
2. ブラウザで http://localhost:3000 を開く（新しく起動しなくてよい）
3. 起動し直したいときは、そのターミナルで Ctrl+C を押してから npm run dev
```

> **ヒント:** どのターミナルで動いているか分からなくなったときは、VS Code のターミナル右上のゴミ箱アイコンで**すべてのターミナルを閉じる**と、中で動いていた開発サーバーも止まります。そのあと新しいターミナルで `npm run dev` すれば、確実に1つだけになります。

#### 9.4.2 つまずきポイント: プロジェクトのフォルダの外で実行してしまう

> **つまずきポイント: `cd book-management` を忘れる**
>
> VS Code で親のフォルダ（`projects` など）を開いていると、ターミナルも親のフォルダから始まります。そのまま `npm run dev` すると、npm は `package.json` を見つけられません。

NG（`book-management` の1つ外で実行する）:

```bash
# いま C:\Users\you\projects にいる（book-management の外）
npm run dev
```

**▼ 実行結果**（パスは例）

```text
npm error code ENOENT
npm error syscall open
npm error path C:\Users\you\projects\package.json
npm error errno -4058
npm error enoent Could not read package.json: Error: ENOENT: no such file or directory, open 'C:\Users\you\projects\package.json'
npm error enoent This is related to npm not being able to find a file.
npm error enoent
npm error A complete log of this run can be found in: C:\Users\you\AppData\Local\npm-cache\_logs\2026-10-15T11_50_40_324Z-debug-0.log
```

`ENOENT` は「Error NO ENTry（そんなファイルはありません）」の略です。`path` の行を見ると、`projects\package.json` を探しに行って見つからなかったことが分かります。

OK（プロジェクトのフォルダに入ってから実行する）:

```bash
# book-management フォルダに入る
cd book-management
# package.json がある場所で実行する
npm run dev
```

> **ヒント:** いまどのフォルダにいるかは、PowerShell なら行の先頭（`PS C:\Users\you\projects\book-management>`）に表示されています。macOS / Linux では `pwd` コマンドで確認できます。`npm` のコマンドは、いつも **`package.json` がある場所**で実行する、と覚えておきましょう。

---

## 10. データフローの全体像

書籍管理アプリのデータの流れを図で確認しましょう。ユーザーの操作がどのようにデータベースに伝わり、結果がどのように画面に戻ってくるのかを理解することは、アプリ全体の設計を把握する上で非常に重要です。

### 10.1 全体アーキテクチャ

<div style="max-width:680px;margin:20px auto;font-family:'Segoe UI',sans-serif;">
  <!-- Forward flow -->
  <div style="display:flex;align-items:center;justify-content:center;gap:6px;flex-wrap:wrap;margin-bottom:16px;">
    <div style="background:#fef3c7;border:2px solid #d97706;border-radius:10px;padding:10px 14px;text-align:center;min-width:90px;">
      <div style="font-weight:700;color:#92400e;font-size:13px;">ユーザー</div>
      <div style="font-size:11px;color:#a16207;">(ブラウザ)</div>
    </div>
    <div style="display:flex;flex-direction:column;align-items:center;gap:2px;">
      <div style="color:#d97706;font-size:18px;">→</div>
      <div style="font-size:10px;color:#64748b;text-align:center;max-width:70px;">1. 操作</div>
    </div>
    <div style="background:#dbeafe;border:2px solid #2563eb;border-radius:10px;padding:10px 14px;text-align:center;min-width:90px;">
      <div style="font-weight:700;color:#1e40af;font-size:13px;">UI Components</div>
      <div style="font-size:11px;color:#3b82f6;">(React)</div>
    </div>
    <div style="display:flex;flex-direction:column;align-items:center;gap:2px;">
      <div style="color:#2563eb;font-size:18px;">→</div>
      <div style="font-size:10px;color:#64748b;text-align:center;max-width:70px;">2. リクエスト</div>
    </div>
    <div style="background:#dcfce7;border:2px solid #16a34a;border-radius:10px;padding:10px 14px;text-align:center;min-width:90px;">
      <div style="font-weight:700;color:#166534;font-size:13px;">Supabase Client</div>
      <div style="font-size:11px;color:#16a34a;">(supabase-js)</div>
    </div>
    <div style="display:flex;flex-direction:column;align-items:center;gap:2px;">
      <div style="color:#16a34a;font-size:18px;">→</div>
      <div style="font-size:10px;color:#64748b;text-align:center;max-width:70px;">3. HTTPS</div>
    </div>
    <div style="background:#e0e7ff;border:2px solid #4f46e5;border-radius:10px;padding:10px 14px;text-align:center;min-width:90px;">
      <div style="font-weight:700;color:#3730a3;font-size:13px;">Supabase API</div>
      <div style="font-size:11px;color:#4f46e5;">(REST API)</div>
    </div>
    <div style="display:flex;flex-direction:column;align-items:center;gap:2px;">
      <div style="color:#4f46e5;font-size:18px;">→</div>
      <div style="font-size:10px;color:#64748b;text-align:center;max-width:70px;">4. SQL</div>
    </div>
    <div style="background:#fce7f3;border:2px solid #db2777;border-radius:10px;padding:10px 14px;text-align:center;min-width:90px;">
      <div style="font-weight:700;color:#9d174d;font-size:13px;">PostgreSQL</div>
      <div style="font-size:11px;color:#db2777;">(データベース)</div>
    </div>
  </div>
  <!-- Return flow -->
  <div style="display:flex;align-items:center;justify-content:center;gap:6px;flex-wrap:wrap;padding:10px 16px;background:#f8fafc;border:1px solid #e2e8f0;border-radius:8px;">
    <div style="font-size:12px;color:#64748b;font-weight:600;">戻りの流れ:</div>
    <div style="font-size:12px;color:#9d174d;">5. クエリ結果</div>
    <div style="color:#94a3b8;font-size:14px;">←</div>
    <div style="font-size:12px;color:#4f46e5;">6. JSON レスポンス</div>
    <div style="color:#94a3b8;font-size:14px;">←</div>
    <div style="font-size:12px;color:#166534;">7. 型付きデータ (Book[])</div>
    <div style="color:#94a3b8;font-size:14px;">←</div>
    <div style="font-size:12px;color:#1e40af;">8. 画面更新 (再レンダリング)</div>
  </div>
</div>

### 10.2 各ステップの詳細解説

| ステップ | 場所 | 処理内容 |
|---------|------|---------|
| 1. ユーザーの操作 | ブラウザ | ユーザーがボタンをクリックしたり、フォームに入力したりします。例: 「書籍を登録する」ボタンを押す。 |
| 2. コンポーネントからの呼び出し | UI Components | React コンポーネント内で Supabase クライアントの関数を呼び出します。例: `supabase.from("books").insert(newBook)` |
| 3. HTTP リクエスト | Supabase Client | `@supabase/supabase-js` ライブラリが、JavaScript のコードを HTTPS リクエストに変換して Supabase のサーバーに送信します。URL や Anon Key は `.env.local` の環境変数から読み取られます。 |
| 4. SQL クエリの実行 | Supabase API | Supabase の REST API（PostgREST）がリクエストを受け取り、対応する SQL クエリに変換して PostgreSQL データベースに実行します。RLS（行レベルセキュリティ）によるアクセス制御もこのタイミングで適用されます。 |
| 5. クエリ結果 | PostgreSQL | データベースがクエリを実行し、結果を返します。 |
| 6. JSON レスポンス | Supabase API | クエリ結果を JSON 形式に変換し、クライアントにレスポンスとして返します。 |
| 7. 型付きデータ | Supabase Client | JSON レスポンスを受け取り、TypeScript の型（`Book[]` など）に変換します。 |
| 8. 画面更新 | UI Components → ブラウザ | 受け取ったデータで React の state を更新し、画面が再レンダリングされます。ユーザーは最新のデータを画面上で確認できます。 |

### 10.3 具体例: 書籍一覧の取得

実際のコードでデータがどう流れるか、具体例で見てみましょう（このコードは次の章で実装します）。

> **▼ このコードがやること（先に日本語で）:** データベースから書籍の一覧を取り出す処理を、具体的なコードで追いかけます。さきほど作った Supabase クライアントを使い、「books テーブルから全件を、新しい順に取得する」という指示を出しています。`await`（待つ）は通信が終わるまで結果を待つための合図で、戻り値を `data`（成功データ）と `error`（エラー）に分けて受け取り、必ずエラーの有無を確認するのが安全な書き方です。この1行の裏で、前の図の通信ステップが自動で実行されています。

```typescript
// 書籍一覧を取得するコードの流れ

// 1. Supabase クライアントをインポート
// named import で supabase 変数だけを取り込む（lib/supabase.ts で export const supabase したもの）
import { supabase } from "@/lib/supabase";
// Book 型もインポート（型情報のみ。ランタイムには影響なし）
import { Book } from "@/types/book";

// 2. データベースから書籍一覧を取得
// await: Promise（非同期処理の結果）を待つキーワード。async 関数内でのみ使用可能
// 戻り値は { data, error } のオブジェクトなので、分割代入で取り出す
const { data, error } = await supabase
  // books テーブルを指定（SQL の FROM books に相当）
  .from("books")
  // すべてのカラムを取得（SQL の SELECT * に相当）
  .select("*")
  // 作成日の降順（新しい順）に並び替え（SQL の ORDER BY に相当）
  .order("created_at", {
    // false で降順（新しい順）。true なら昇順（古い順）
    ascending: false
  });

// 3. エラーハンドリング
// if (error): error が null/undefined でなければ true（エラーが発生した）
if (error) {
  // console.error: 開発者ツールのコンソールにエラーログを赤色で出力
  console.error("書籍の取得に失敗しました:", error.message);
  // ここで関数を抜ける
  return;
}

// 4. data は Book[] 型（書籍の配列）
// data の型は自動で Book[] と推論されるが、明示的に型注釈を付けることもできる
const books: Book[] = data;
```

この1つの `supabase.from("books").select("*")` の呼び出しの裏側で、上の図の 2 → 3 → 4 → 5 → 6 → 7 のステップがすべて自動的に実行されています。Supabase クライアントがこれらの複雑な処理を抽象化してくれるため、開発者は SQL を直接書くことなく、シンプルな JavaScript/TypeScript のコードでデータベースを操作できます。

#### ▼ コードを1つずつ分解して解説

この取得処理を、初心者がつまずきやすい部分ごとに**1つずつ**ていねいに見ていきましょう。

---

##### 解説1: 共有クライアントと型をインポートする

```typescript
import { supabase } from "@/lib/supabase";
import { Book } from "@/types/book";
```

- 1行目で、さきほど `lib/supabase.ts` で作って `export` した `supabase`（データベースの窓口）を取り込んでいます。これで通信の準備は完了です。
- 2行目で `Book` 型を取り込み、取得したデータが「書籍の形」をしていることを TypeScript に伝えられるようにします。
- どちらも `@/`（= `src/`）から始まるパスエイリアスを使い、相対パスより読みやすく書いています。

> **用語:** 「型のインポート」とは、`Book` のような型定義を別ファイルから取り込むことです。データの形を共有することで、プロジェクト全体で同じ構造を保てます。

---

##### 解説2: await で取得結果を待ち、分割代入で受け取る

```typescript
const { data, error } = await supabase
  .from("books")
  .select("*")
  .order("created_at", {
    ascending: false
  });
```

- `await`（アウェイト）は、**通信が終わるまで結果を待つ**ための合図です。データベースとのやり取りには時間がかかるため、答えが返ってくるまでここで待機します。
- `.from("books")` で対象テーブルを、`.select("*")` で全カラムを、`.order("created_at", { ascending: false })` で「作成日の新しい順」を指定しています。SQL の `FROM` / `SELECT *` / `ORDER BY` に対応します。
- 戻り値は `{ data, error }` という形のオブジェクトなので、`const { data, error } = ...`（分割代入）で**成功データとエラーを同時に取り出して**います。

> **用語:** 「await / 非同期処理」とは、時間のかかる処理（通信など）の完了を待つ仕組みです。待っている間ほかの処理を止めず、結果が来たら続きを実行します。

---

##### 解説3: エラーを必ず確認してから先へ進む

```typescript
if (error) {
  console.error("書籍の取得に失敗しました:", error.message);
  return;
}

const books: Book[] = data;
```

- `if (error)` は「エラーが入っていれば（通信が失敗していれば）」という意味です。Supabase はエラーを**例外で投げず `error` に入れて返す**ため、自分でこのチェックをするのが安全な書き方です。
- 失敗時は `console.error(...)` で原因をコンソールに出し、`return` で処理を中断します。失敗したのに先へ進んでしまう事故を防げます。
- 問題がなければ、`data` を `Book[]`（書籍の配列）として受け取ります。`[]` は「その型がいくつも並んだ配列」を表します。

> **用語:** 「エラーハンドリング」とは、失敗が起きたときの対処をあらかじめ書いておくことです。通信は必ず成功するとは限らないため、`error` の確認はセットで行います。

---

## まとめ

この章で実施した内容を振り返ります。

| 項目 | 内容 |
|------|------|
| Next.js プロジェクトの作成 | `create-next-app` で TypeScript + Tailwind CSS + App Router のプロジェクトを作成しました。 |
| Tailwind CSS の基礎 | ユーティリティファーストの概念と、よく使うクラスを学びました。 |
| パッケージのインストール | `@supabase/supabase-js` をインストールしました。 |
| ディレクトリ構成の設計 | `app/`（ページ）、`components/`（UI部品）、`lib/`（ユーティリティ）、`types/`（型定義）の4つのディレクトリで整理する構成にしました。 |
| Supabase クライアントの設定 | `.env.local` に環境変数を設定し、`lib/supabase.ts` でクライアントを初期化しました。 |
| 型定義の作成 | `Book`, `BookInsert`, `BookUpdate` の3つの型と、ステータス関連の定数を定義しました。 |
| ルートレイアウトの設定 | 日本語フォント、メタデータ、全ページ共通のレイアウト構造を設定しました。 |
| ヘッダーコンポーネントの作成 | ナビゲーション付きの共通ヘッダーを作成しました。 |
| 動作確認 | 開発サーバーを起動し、ブラウザで表示を確認しました。 |

**作成したファイル一覧:**

```
src/
├── app/
│   ├── layout.tsx    ✅ 作成済み
│   ├── page.tsx      ✅ 作成済み（仮の内容）
│   └── globals.css   ✅ 確認済み
├── components/
│   └── Header.tsx    ✅ 作成済み
├── lib/
│   └── supabase.ts   ✅ 作成済み
└── types/
    └── book.ts       ✅ 作成済み
```

次の章では、書籍一覧の表示機能を実装します。`BookCard`、`BookList`、`StatusBadge`、`RatingStars`、`LoadingSpinner` の各コンポーネントを作成し、Supabase からデータを取得してトップページに表示します。

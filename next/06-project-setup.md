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

コマンドを実行すると、いくつかの質問が対話形式で表示されます。以下のように回答してください。

### 1.2 各選択肢の詳細解説

```
Would you like to use TypeScript? … Yes              # TypeScriptを使うか？ → 使う
Would you like to use ESLint? … Yes                  # ESLint（コード品質チェック）を使うか？ → 使う
Would you like to use Tailwind CSS? … Yes            # Tailwind CSS（スタイリング）を使うか？ → 使う
Would you like your code inside a `src/` directory? … Yes  # コードを src/ フォルダに入れるか？ → 入れる
Would you like to use App Router? (recommended) … Yes      # App Router（新しいルーティング）を使うか？ → 使う
Would you like to use Turbopack for next dev? … Yes        # Turbopack（高速ビルドツール）を使うか？ → 使う
Would you like to customize the import alias (@/* by default)? … No  # @/* のエイリアスをカスタムするか？ → しない
```

それぞれの選択肢が何を意味するのか、なぜその回答をするのかを詳しく見ていきましょう。

| 質問 | 回答 | 理由 |
|------|------|------|
| **TypeScript** | Yes | 型安全性により、変数や関数の引数に間違った値を渡すミスをコンパイル時に検出できます。特にチーム開発やアプリが大規模になったときに威力を発揮します。本チュートリアルでは、Book 型などを定義してデータの構造を明確にします。 |
| **ESLint** | Yes | JavaScript/TypeScript のコードを静的解析し、バグの原因になりやすい書き方やスタイルの不統一を自動検出します。例えば、未使用の変数がある場合に警告を出してくれます。 |
| **Tailwind CSS** | Yes | ユーティリティファーストの CSS フレームワークです。HTML（JSX）の中にクラス名を書くだけでスタイリングできるため、CSS ファイルを別途管理する必要がありません。詳しくは次のセクションで解説します。 |
| **`src/` directory** | Yes | ソースコードを `src/` ディレクトリにまとめることで、設定ファイル（`package.json`, `next.config.ts` など）とソースコードが明確に分離されます。プロジェクトのルートがすっきりし、見通しがよくなります。 |
| **App Router** | Yes | Next.js 13 以降で導入された新しいルーティング方式です。ファイルシステムベースのルーティング、Server Components、レイアウトのネストなど、モダンな機能が使えます。Pages Router（旧方式）より推奨されています。 |
| **Turbopack** | Yes | Rust で書かれた高速なバンドラーです。開発サーバーの起動やホットリロード（コード変更時の自動反映）が従来の Webpack より大幅に高速になります。 |
| **Import alias** | No（デフォルト） | デフォルトの `@/*` エイリアスをそのまま使います。`@/components/Header` のように `src/` ディレクトリ内のファイルを短いパスでインポートできます。 |

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

質問に全て答え終わると、ターミナルに次のような表示が流れます（実際の表示は時期によって少し変わります）。

```
Creating a new Next.js app in /Users/you/projects/book-management.

Using npm.

Initializing project with template: app

Installing dependencies:
- react
- react-dom
- next

Installing devDependencies:
- typescript
- @types/node
- @types/react
- @types/react-dom
- tailwindcss
- postcss
- eslint
- eslint-config-next

added 372 packages, and audited 373 packages in 24s

131 packages are looking for funding

found 0 vulnerabilities
Initialized a git repository.

Success! Created book-management at /Users/you/projects/book-management
```

> **「added XXX packages」とは？:** `npm install` が `node_modules/` フォルダに何個のパッケージを置いたかを示します。React/Next.js本体に加えて、依存パッケージ（依存関係でついてくる関連ライブラリ）が一緒に入るので 300〜400個程度になります。これが普通です。

> **`npm install` で何が起きているか:**
> 1. `package.json` に書かれた依存パッケージのリストを読み取る
> 2. npm レジストリ（https://www.npmjs.com）から、それぞれのパッケージをダウンロード
> 3. ダウンロードしたパッケージを `node_modules/` フォルダに展開
> 4. 各パッケージがさらに依存しているパッケージも芋づる式にダウンロード（これが「依存関係の解決」）
> 5. インストールされた正確なバージョンを `package-lock.json` に記録
>
> `node_modules/` はとても巨大になりやすいですが、`.gitignore` でGit管理から除外するのが標準的なやり方です（後述）。

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

`create-next-app` を実行すると、以下のファイルとディレクトリが自動生成されます。

```
book-management/
  node_modules/         ... インストールされたパッケージ群（数百〜数千フォルダ。.gitignoreで除外する）
  public/               ... 静的ファイル（画像など）。ここに置いたファイルは / 直下のURLでアクセス可能
    file.svg            ... ファイルアイコン（デフォルトサンプル）
    globe.svg           ... 地球アイコン（デフォルトサンプル）
    next.svg            ... Next.js ロゴ（デフォルトサンプル）
    vercel.svg          ... Vercel ロゴ（デフォルトサンプル）
    window.svg          ... ウィンドウアイコン（デフォルトサンプル）
  src/                  ... アプリのソースコードを置くフォルダ
    app/                ... App Router のルートディレクトリ。フォルダ構成がそのままURLになる
      favicon.ico       ... ブラウザのタブに表示されるアイコン
      globals.css       ... グローバル CSS（Tailwind の設定を含む）
      layout.tsx        ... ルートレイアウト（全ページ共通の枠）
      page.tsx          ... トップページ（ / にアクセスしたときの表示）
  .eslintrc.json        ... ESLint の設定ファイル
  .gitignore            ... Git で管理しないファイルの指定
  next-env.d.ts         ... Next.js の TypeScript 型定義（自動生成）
  next.config.ts        ... Next.js の設定ファイル
  package-lock.json     ... パッケージの正確なバージョン記録（自動更新されるので手で編集しない）
  package.json          ... プロジェクトの設定とパッケージ一覧
  postcss.config.mjs    ... PostCSS の設定（Tailwind CSS が使用）
  README.md             ... プロジェクトの説明
  tailwind.config.ts    ... Tailwind CSS のカスタマイズ設定
  tsconfig.json         ... TypeScript の設定ファイル
```

各ファイルの役割を表にまとめます。

| ファイル | 役割 |
|----------|------|
| `package.json` | プロジェクト名、使用パッケージ、実行スクリプトなどを定義するファイルです。`npm install` はこのファイルを読んでパッケージをインストールします。 |
| `tsconfig.json` | TypeScript コンパイラの設定ファイルです。`@/*` のパスエイリアスもここで定義されています。 |
| `next.config.ts` | Next.js の動作をカスタマイズする設定ファイルです。画像の外部ドメイン許可などを設定できます。 |
| `tailwind.config.ts` | Tailwind CSS のカスタマイズ設定ファイルです。カスタムカラーやフォントの追加ができます。 |
| `postcss.config.mjs` | PostCSS（CSS の変換ツール）の設定です。Tailwind CSS は PostCSS のプラグインとして動作します。 |
| `.eslintrc.json` | ESLint のルール設定です。`next/core-web-vitals` の推奨ルールがデフォルトで有効になっています。 |
| `src/app/layout.tsx` | すべてのページを囲むルートレイアウトです。`<html>` と `<body>` タグを含み、共通のヘッダーやフッターを配置します。 |
| `src/app/page.tsx` | トップページのコンポーネントです。URL `/` にアクセスしたときに表示されます。 |
| `src/app/globals.css` | アプリ全体に適用されるグローバル CSS です。Tailwind CSS のディレクティブ（`@tailwind base;` など）が含まれています。 |

> **`.gitignore` で何を除外するか、なぜか:**
> Git の管理から除外する（コミットしない）べきファイルの種類を `.gitignore` ファイルに書きます。`create-next-app` が自動生成する `.gitignore` には、次のようなものが含まれます。
>
> | 除外対象 | 理由 |
> |---------|------|
> | `node_modules/` | パッケージ群はとても大きい上に `package.json` から再インストールできるため。コミットすると無駄に容量を食う。 |
> | `.next/` | Next.js のビルド結果。実行のたびに再生成されるため。 |
> | `.env*.local` | 秘密の環境変数（APIキー、パスワードなど）が入っているため。GitHub にアップすると漏洩する。 |
> | `*.log`, `npm-debug.log*` | ログファイルは個人のPC固有のものなので共有不要。 |
> | `.DS_Store`, `Thumbs.db` | macOS/Windowsが自動生成する隠しファイル。アプリと無関係。 |
>
> ルール: 「自動生成されるもの」「PC固有のもの」「秘密情報を含むもの」はGitに入れない、と覚えておくと良いです。

### 1.6 主な設定ファイルの中身を見てみる

#### `package.json` の構造

`create-next-app` が生成する `package.json` は、おおよそ次のような構造になっています。

```json
{
  "name": "book-management",                  // プロジェクト名（フォルダ名と同じ）
  "version": "0.1.0",                          // プロジェクトのバージョン（自由に決められる）
  "private": true,                             // true にすると npm publish で公開できなくなる（誤公開防止）
  "scripts": {                                 // npm run <名前> で実行できるコマンドの集まり
    "dev": "next dev --turbopack",             // 開発サーバー起動（Turbopackを使用）
    "build": "next build",                     // 本番用ビルド（最適化された静的ファイルを生成）
    "start": "next start",                     // ビルド済みアプリを本番モードで起動
    "lint": "next lint"                        // ESLint でコード品質チェックを実行
  },
  "dependencies": {                            // 本番でも必要なパッケージ
    "react": "19.x.x",                         // React 本体
    "react-dom": "19.x.x",                     // React を DOM にレンダリングするためのパッケージ
    "next": "15.x.x"                           // Next.js フレームワーク
  },
  "devDependencies": {                         // 開発中だけ必要なパッケージ
    "typescript": "^5",                        // TypeScript コンパイラ
    "@types/node": "^20",                      // Node.js の型定義
    "@types/react": "^19",                     // React の型定義
    "@types/react-dom": "^19",                 // React DOM の型定義
    "postcss": "^8",                           // PostCSS（CSS変換ツール）
    "tailwindcss": "^3.x.x",                   // Tailwind CSS 本体
    "eslint": "^8",                            // ESLint 本体
    "eslint-config-next": "15.x.x"             // Next.js 用 ESLint 設定
  }
}
```

#### `tsconfig.json` の主要設定

```json
{
  "compilerOptions": {                         // TypeScript コンパイラの動作設定
    "target": "ES2017",                        // コンパイル後のJavaScriptのバージョン（ES2017相当）
    "lib": ["dom", "dom.iterable", "esnext"],  // 使えるグローバルAPI（DOM、最新のES機能など）
    "allowJs": true,                           // .js ファイルもTypeScriptで扱えるようにする
    "skipLibCheck": true,                      // ライブラリの型チェックをスキップ（高速化）
    "strict": true,                            // 厳格な型チェックを有効化（推奨）
    "noEmit": true,                            // .ts → .js への変換ファイルは出力しない（Next.jsが代行）
    "esModuleInterop": true,                   // CommonJS と ES Modules の混在を許可
    "module": "esnext",                        // モジュールシステムは最新の ES Modules
    "moduleResolution": "bundler",             // モジュール解決方式（バンドラー用）
    "resolveJsonModule": true,                 // .json ファイルを import 可能にする
    "isolatedModules": true,                   // 各ファイルを独立したモジュールとして扱う
    "jsx": "preserve",                         // JSX をそのまま残す（Next.jsが変換）
    "incremental": true,                       // 増分コンパイルを有効化（再ビルドが速くなる）
    "plugins": [
      { "name": "next" }                       // Next.js 用の TypeScript プラグイン
    ],
    "paths": {                                 // パスエイリアスの定義
      "@/*": ["./src/*"]                       // @/foo は src/foo を指す
    }
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],  // 型チェック対象
  "exclude": ["node_modules"]                  // 型チェック対象から除外
}
```

#### `next.config.ts` の例（最小構成）

```typescript
// next.config.ts

import type { NextConfig } from "next";       // Next.js の設定型をインポート（type 付きで型のみ取得）

const nextConfig: NextConfig = {               // 設定オブジェクトの型を NextConfig に固定
  /* ここに追加の設定を書く */                   // 例: images.remotePatterns で外部画像ドメインを許可など
};

export default nextConfig;                     // デフォルトエクスポート（Next.js が自動で読み込む）
```

---

## 2. Tailwind CSS の基礎

### 2.1 ユーティリティファーストとは

従来の CSS では、まずクラス名を考え、そのクラスに対してスタイルを定義していました。

```css
/* 従来の CSS */
.card {                                        /* card という名前のクラスにスタイルを定義 */
  background-color: white;                     /* 背景色を白に */
  border-radius: 8px;                          /* 角を 8px の半径で丸める */
  padding: 16px;                               /* 内側の余白を 16px に */
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);    /* 薄い影を下方向に */
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
| `shadow-sm` | `box-shadow: 0 1px 2px rgba(0,0,0,0.05);` |

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
  <div>カード1</div>          {/* 1つ目のセル */}
  <div>カード2</div>          {/* 2つ目のセル */}
  <div>カード3</div>          {/* 3つ目のセル */}
</div>
```

この例では次のように表示が変わります。

| 画面サイズ | 適用されるクラス | 表示 |
|-----------|-----------------|------|
| 0px 〜 767px | `grid-cols-1` | 1列表示（カードが縦に並ぶ） |
| 768px 〜 1023px | `md:grid-cols-2` | 2列表示 |
| 1024px 〜 | `lg:grid-cols-3` | 3列表示 |

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

プロジェクトのディレクトリに移動してから、必要なパッケージをインストールします。

```bash
# プロジェクトフォルダに移動
cd book-management

# npm install: package.json に依存パッケージを追加し、node_modules/ にダウンロード
# @supabase/supabase-js: Supabase 公式の JavaScript/TypeScript クライアントライブラリ
npm install @supabase/supabase-js
```

### 3.1 各パッケージの役割

| パッケージ名 | 役割 | 説明 |
|-------------|------|------|
| `@supabase/supabase-js` | Supabase クライアント | Supabase（データベース、認証、ストレージ）と通信するための公式 JavaScript/TypeScript ライブラリです。このパッケージを使って、ブラウザから直接 Supabase の PostgreSQL データベースにデータの読み書きができます。 |
| `next`（プリインストール済み） | Next.js フレームワーク | React ベースのフルスタックフレームワークです。ルーティング、サーバーサイドレンダリング、API ルートなどの機能を提供します。 |
| `react`（プリインストール済み） | React ライブラリ | UI コンポーネントを構築するためのライブラリです。`useState` や `useEffect` などのフック（Hook）を使って、動的な UI を作成します。 |
| `react-dom`（プリインストール済み） | React DOM | React コンポーネントをブラウザの DOM（画面）にレンダリングするためのライブラリです。 |
| `typescript`（プリインストール済み） | TypeScript | JavaScript に型システムを追加した言語です。開発時の型チェックに使われます。 |
| `tailwindcss`（プリインストール済み） | Tailwind CSS | ユーティリティファーストの CSS フレームワークです。`create-next-app` で Tailwind CSS を選択した場合、自動的にインストールされます。 |
| `eslint`（プリインストール済み） | ESLint | コードの品質チェックツールです。バグの原因になりやすいパターンを検出します。 |

> **`@supabase/supabase-js` だけインストールすれば十分な理由:**
> 本チュートリアルの書籍管理アプリでは、Supabase との通信がメインの外部依存です。UI は Tailwind CSS（プリインストール済み）で構築し、フォーム処理は React の標準機能で実現します。追加のUIライブラリやフォームライブラリは使用しないため、シンプルな構成を保てます。

### 3.2 パッケージがインストールされたことの確認

`package.json` を開き、`dependencies` に `@supabase/supabase-js` が追加されていることを確認します。

```json
{
  "dependencies": {
    "@supabase/supabase-js": "^2.x.x",   // 今インストールしたSupabaseクライアント（^は後述）
    "next": "15.x.x",                     // Next.js本体
    "react": "^19.x.x",                   // React本体
    "react-dom": "^19.x.x"                // React DOM
  }
}
```

> **`^` の意味:** `^2.49.1` は「メジャーバージョン 2 の範囲で最新版を許容する」という意味です。`2.49.1` から `2.99.99` まではインストール可能ですが、`3.0.0` はインストールされません。これにより、破壊的変更を避けつつ最新のバグ修正やパフォーマンス改善を受けられます。

> **セマンティックバージョニング（SemVer）の補足:**
> バージョン番号は `メジャー.マイナー.パッチ` の形（例: `2.49.1`）になっています。
> - **メジャー（2）**: 互換性のない変更（古い書き方が動かなくなる可能性あり）
> - **マイナー（49）**: 機能追加（後方互換あり）
> - **パッチ（1）**: バグ修正
>
> `^` は「マイナーとパッチの自動アップデートはOK、メジャーは固定」という意味のルールです。

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
        <div style="padding:4px 0 4px 12px;font-size:12px;color:#334155;">supabase.ts <span style="color:#64748b;">(Supabase client)</span></div>
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
│   └── supabase.ts                 # Supabase クライアントの初期化・エクスポート
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
| `StatusBadge.tsx` | 読書ステータス（「未読」「読書中」「読了」）を色付きバッジで表示するコンポーネント。ステータスに応じて色が変わります。 |
| `RatingStars.tsx` | 5段階の星評価を表示するコンポーネント。黄色い星と灰色の星で評価を視覚化します。 |
| `LoadingSpinner.tsx` | データ読み込み中に表示するスピナー（回転アニメーション）。API からのレスポンスを待っている間、ユーザーに読み込み中であることを伝えます。 |

#### ユーティリティファイル（`lib/` ディレクトリ）

| ファイル | 役割 |
|----------|------|
| `supabase.ts` | Supabase クライアントの初期化と設定。環境変数から Supabase の URL と API キーを読み取り、アプリ全体で使用できるクライアントインスタンスをエクスポートします。 |

#### 型定義ファイル（`types/` ディレクトリ）

| ファイル | 役割 |
|----------|------|
| `book.ts` | 書籍データの TypeScript 型定義。`Book` 型（データベースから取得する完全なデータ）、`BookInsert` 型（新規登録時に送るデータ）、`BookUpdate` 型（更新時に送るデータ）を定義します。 |

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
mkdir -p src/lib
mkdir -p src/types
```

> **注意:** `[id]` は角括弧を含むため、シェルによってはエスケープが必要です。うまくいかない場合は、VS Code のエクスプローラーからフォルダを手動で作成してください。

> **▼ このコードがやること（先に日本語で）:** さきほど作ったフォルダの中に、これから編集していく「空っぽのファイル」を一気に用意します。`touch` は中身が空のファイルを作るコマンドで、中身は後の節で書き込んでいきます。Windows の PowerShell には `touch` がないので、その場合はコメントにある `New-Item -ItemType File ファイル名` で代用してください。

```bash
# ファイルの作成（中身は後ほど記述）
# touch: 空のファイルを作成するコマンド（既に存在する場合は最終更新時刻だけ変える）
# Windows の PowerShell では touch がないので、代わりに「New-Item -ItemType File ファイル名」を使う
touch src/components/Header.tsx          # 共通ヘッダー
touch src/components/BookCard.tsx        # 書籍カード
touch src/components/BookForm.tsx        # 書籍フォーム
touch src/components/BookList.tsx        # 書籍一覧
touch src/components/DeleteButton.tsx    # 削除ボタン
touch src/components/StatusBadge.tsx     # ステータスバッジ
touch src/components/RatingStars.tsx     # 評価表示
touch src/components/LoadingSpinner.tsx  # ローディング
touch src/lib/supabase.ts                # Supabaseクライアント
touch src/types/book.ts                  # Book型定義
```

---

## 5. Supabase クライアントの設定

### 5.1 環境変数の取得

Supabase クライアントの設定には、以下の2つの情報が必要です。

1. **Supabase URL** -- あなたのプロジェクトの API エンドポイント
2. **Supabase Anon Key** -- 匿名アクセス用の API キー

これらは Supabase のダッシュボードから取得できます。

**取得手順:**

1. [Supabase のダッシュボード](https://supabase.com/dashboard) にアクセスします
2. 書籍管理アプリ用のプロジェクトを選択します
3. 左のサイドバーから **「Project Settings」**（歯車アイコン）をクリックします
4. **「API」** タブをクリックします
5. 以下の情報をコピーします:
   - **Project URL**: `https://xxxxxxxxxxxxxxxx.supabase.co` の形式
   - **Project API keys** の **anon / public** キー: `eyJhbGciOi...` で始まる長い文字列

### 5.2 .env.local の作成

プロジェクトのルートディレクトリ（`book-management/` の直下）に `.env.local` ファイルを作成します。

> **▼ このコードがやること（先に日本語で）:** Supabase（データベース）に接続するための「URL」と「鍵（キー）」を、コードの外側の設定ファイルに書き出します。こうした外部から渡す設定値を「環境変数」と呼び、`キー=値` の形で1行ずつ書きます。`NEXT_PUBLIC_` を頭に付けた変数だけがブラウザ側からも読めるようになる点がポイントです。`xxxxxxxx` の部分は、必ず自分の Supabase プロジェクトの値に置き換えてください。

```bash
# book-management/.env.local
# このファイルは ENV ファイルと呼ばれ、「キー=値」の形で環境変数を1行ずつ書く
# NEXT_PUBLIC_ プレフィックスを付けると Next.js がブラウザ側にも公開してくれる

# Supabase プロジェクトのURL（ダッシュボードからコピー）
NEXT_PUBLIC_SUPABASE_URL=https://xxxxxxxxxxxxxxxx.supabase.co

# Supabase の Anon Key（匿名アクセス用の公開鍵。長いJWTトークン形式）
NEXT_PUBLIC_SUPABASE_ANON_KEY=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

> **`NEXT_PUBLIC_` プレフィックスの重要性:**
> Next.js では、環境変数に `NEXT_PUBLIC_` プレフィックスを付けると、ブラウザ側の JavaScript からもアクセスできるようになります。プレフィックスがない環境変数はサーバー側でのみ使用可能です。
>
> Supabase の Anon Key はブラウザから直接 API を呼び出すために使うため、`NEXT_PUBLIC_` が必要です。Anon Key は Row Level Security（RLS）で保護されているため、公開しても安全です（前の章で RLS を設定しました）。

> **ENV ファイルの種類と読み込み順（Next.js）:**
> Next.js では複数の `.env` 系ファイルを使い分けられます。同じ変数が複数のファイルにある場合、上のものが優先されます。
>
> | ファイル | いつ読み込まれるか | Gitに入れる？ |
> |---------|----------------|-------------|
> | `.env.local` | 全環境（開発・本番ともに）。ただし `next test` は除く | **入れない**（.gitignoreで除外） |
> | `.env.development` | 開発時（`npm run dev`）のみ | 入れてOK（秘密情報を含まないなら） |
> | `.env.production` | 本番時（`npm run start`）のみ | 入れてOK（秘密情報を含まないなら） |
> | `.env` | すべての環境（デフォルト値） | 入れてOK（秘密情報を含まないなら） |
>
> 読み込み優先順位（高 → 低）: `.env.local` > `.env.development` / `.env.production` > `.env`
>
> **覚え方:** 「`.local` が付くファイルは個人専用＝Gitに入れない＝秘密情報を入れてOK」。`.env.local` だけは絶対にコミットしない、と覚えれば事故を防げます。

> **`.env.local` がGitにアップロードされない理由:**
> `create-next-app` が自動生成した `.gitignore` ファイルには `.env*.local` が含まれています。そのため、`.env.local` は Git にコミットされず、API キーが公開リポジトリに漏洩する心配はありません。

**必ず `xxxxxxxxxxxxxxxx` の部分を、自分のプロジェクトの値に置き換えてください。**

### 5.3 lib/supabase.ts の作成

`src/lib/supabase.ts` ファイルに以下のコードを記述します。

> **▼ このコードがやること（先に日本語で）:** データベースと話すための「窓口（Supabase クライアント）」を1か所で作り、アプリ全体で使い回せるようにします。さきほど `.env.local` に書いた URL とキーを読み込み、それを使って接続用のオブジェクトを組み立てて `export`（外に公開）します。もし設定が抜けていたら、原因がすぐ分かるエラーをわざと出すようにしているのが安心ポイントです。1行ずつの意味はコード内のコメントと、下の解説表で確認できます。

```typescript
// src/lib/supabase.ts
// このファイルは、アプリ全体で共有する Supabase クライアントを作成・エクスポートする役割

// @supabase/supabase-js パッケージから createClient 関数を「名前付きインポート」する
// {} で囲むのが名前付きインポートの構文。export されている特定の関数だけを取り出す
import { createClient } from "@supabase/supabase-js";

// 環境変数から Supabase の接続情報を取得
// process.env: Node.js 由来のグローバルオブジェクト。.env.local の中身がここに入る
// NEXT_PUBLIC_ で始まる変数は、ビルド時にブラウザ側のコードにも埋め込まれる
const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL;       // SupabaseのプロジェクトURL
const supabaseAnonKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY; // Supabase の Anon Key

// 環境変数が設定されていない場合にわかりやすいエラーメッセージを表示
// !supabaseUrl は「supabaseUrl が undefined や空文字なら true」になる条件
if (!supabaseUrl) {
  // throw: エラーを「投げる」。アプリの実行を停止し、コンソールにエラーを出力する
  // new Error(...): JavaScript標準のエラーオブジェクトを作る
  throw new Error(
    "環境変数 NEXT_PUBLIC_SUPABASE_URL が設定されていません。" +
      ".env.local ファイルを確認してください。"
  );
}

// AnonKey についても同じく未設定チェックを行う
if (!supabaseAnonKey) {
  throw new Error(
    "環境変数 NEXT_PUBLIC_SUPABASE_ANON_KEY が設定されていません。" +
      ".env.local ファイルを確認してください。"
  );
}

// Supabase クライアントを作成してエクスポート
// アプリ全体で同じインスタンスを使い回す（シングルトンパターン）
// export const: 他のファイルから import できる変数を宣言
// createClient(URL, Key): 第1引数 URL、第2引数 認証キーで Supabase に接続するクライアントを作成
export const supabase = createClient(supabaseUrl, supabaseAnonKey);
```

**コードの解説:**

| 行 | 解説 |
|----|------|
| `import { createClient } from "@supabase/supabase-js";` | Supabase の公式ライブラリから `createClient` 関数をインポートします。この関数が Supabase との接続を確立します。中括弧 `{}` で囲むのが「named import（名前付きインポート）」で、ライブラリが `export { createClient }` のように公開している特定の関数だけを取り出します。 |
| `process.env.NEXT_PUBLIC_SUPABASE_URL` | `.env.local` に定義した環境変数を読み取ります。`process.env` は Node.js が環境変数にアクセスするためのオブジェクトです。 |
| `if (!supabaseUrl)` | 環境変数が未設定（`undefined`）の場合にエラーを投げます。`.env.local` の作成忘れや typo を早期に検出できます。 |
| `createClient(supabaseUrl, supabaseAnonKey)` | Supabase クライアントを生成します。第1引数が API の URL、第2引数が認証キーです。 |
| `export const supabase` | 生成したクライアントを `supabase` という名前でエクスポートします。他のファイルから `import { supabase } from "@/lib/supabase";` でインポートして使います。 |

> **import 文の種類（重要）:**
> JavaScript/TypeScript には3種類のインポート方法があります。
> - **default import**: `import React from "react";` のように中括弧なし。各ファイルにつき1つだけ `export default` できる主要なものを取り出す。
> - **named import**: `import { useState } from "react";` のように中括弧あり。複数の名前を指定して取り出せる（例: `import { useState, useEffect } from "react";`）。
> - **type import**: `import type { Book } from "@/types/book";` のように `type` キーワード付き。型のみをインポートし、ビルド時に取り除かれる（JavaScriptには型がないため）。

> **シングルトンパターンとは？**
> アプリ全体で「ただ1つのインスタンス」を共有する設計のこと。`supabase.ts` で1回だけ `createClient` を呼び、できた `supabase` オブジェクトを全ファイルで使い回します。毎回新しく作るとリソースの無駄、設定がバラバラになる、などの問題が起きるので、共通化しておくのが定石です。

> **Server Component と Client Component で Supabase を使うときの違い（重要）:**
>
> Next.js の App Router では、コンポーネントが「Server Component」（サーバー側で実行）か「Client Component」（ブラウザ側で実行）かを区別します。それぞれで Supabase を使う際の注意点が異なります。
>
> | 観点 | Server Component | Client Component |
> |------|------------------|------------------|
> | ファイル先頭の宣言 | （宣言なし。デフォルト） | `"use client"` を書く |
> | 実行場所 | サーバー（あなたのPC or 本番サーバー） | ユーザーのブラウザ |
> | 使える React フック | 使えない（useState など不可） | 使える |
> | データ取得 | `await supabase.from(...)` を直接書ける | `useEffect` 内で呼ぶ必要あり |
> | 環境変数 | サーバー専用変数（`SUPABASE_SERVICE_ROLE_KEY` 等）も読める | `NEXT_PUBLIC_` 付きの変数のみ |
> | 認証情報 | サーバー側でCookieからセッションを取る場合がある | ブラウザのlocalStorage等にセッション保存 |
> | このチュートリアル | 主に静的な表示で使用 | フォーム入力や削除などインタラクションで使用 |
>
> 本チュートリアルでは、書籍一覧表示は Client Component（フォーム入力との連携のため `"use client"`）で実装します。より高度な用途では、Server Component から直接Supabaseを呼ぶこともできます。

---

## 6. 型定義ファイルの作成

### 6.1 types/book.ts の作成

`src/types/book.ts` ファイルに以下のコードを記述します。この型定義は、アプリ全体で書籍データの構造を統一するために使います。

> **▼ このコードがやること（先に日本語で）:** 「書籍データはどんな形をしているか」を TypeScript の型として書き出し、アプリ全体で構造を統一します。型とは「このデータには title と author が必須」といった設計図のことで、間違ったデータを使うとコードを動かす前に気づけます。場面ごとに必要な項目が違うため、取得用の `Book`・登録用の `BookInsert`・更新用の `BookUpdate` の3つに分けている点が要チェックです。後半ではステータスの日本語ラベルや色も定数としてまとめています。

```typescript
// src/types/book.ts
// 書籍データに関する型と定数をまとめて定義するファイル

/**
 * 読書ステータスの型定義
 *
 * - "unread"  : 未読（まだ読んでいない）
 * - "reading" : 読書中（今読んでいる途中）
 * - "finished": 読了（読み終わった）
 */
// type: TypeScript の型エイリアス（型に名前を付ける構文）
// "unread" | "reading" | "finished": ユニオン型。この3つの文字列リテラルのいずれかしか取れない
// export: 他のファイルからこの型をインポートできるようにする
export type BookStatus = "unread" | "reading" | "finished";

/**
 * 書籍データの型定義（データベースから取得した完全なデータ）
 *
 * Supabase の books テーブルの行データに対応します。
 * すべてのフィールドが含まれており、データベースから取得した
 * 書籍データはこの型に一致します。
 */
// type Book = { ... }: オブジェクトの形を表す型を Book という名前で定義
export type Book = {
  /** 書籍ID（UUID形式、Supabase が自動生成） */
  id: string;                  // UUID は "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx" 形式の文字列

  /** 書籍のタイトル（必須） */
  title: string;               // 例: "リーダブルコード"

  /** 著者名（必須） */
  author: string;              // 例: "Dustin Boswell"

  /** 読書ステータス（デフォルト: "unread"） */
  status: BookStatus;          // 上で定義した BookStatus 型（3つの文字列のどれか）

  /** 評価（1〜5の整数、未評価の場合は null） */
  rating: number | null;       // number | null: 数値か null のどちらか（ユニオン型）

  /** メモ・感想（任意、未入力の場合は null） */
  memo: string | null;         // 文字列または null

  /** 作成日時（ISO 8601 形式の文字列、Supabase が自動生成） */
  created_at: string;          // 例: "2024-01-15T10:30:00.000Z"

  /** 更新日時（ISO 8601 形式の文字列、Supabase が自動生成） */
  updated_at: string;          // 行が更新されるたびに Supabase のトリガーが書き換える
};

/**
 * 書籍の新規登録時に送るデータの型定義
 *
 * id, created_at, updated_at はデータベースが自動生成するため、
 * 送信データには含めません。
 * status, rating, memo はオプショナル（省略可能）です。
 */
export type BookInsert = {
  /** 書籍のタイトル（必須） */
  title: string;               // 必須項目（? が付いていない）

  /** 著者名（必須） */
  author: string;              // 必須項目

  /** 読書ステータス（省略時は "unread" がデフォルト） */
  status?: BookStatus;         // ?: オプショナル。省略可能。省略時の値はDB側のデフォルトで補う

  /** 評価（1〜5、省略時は null） */
  rating?: number | null;      // 省略してもよいし、明示的に null を送ってもよい

  /** メモ・感想（省略時は null） */
  memo?: string | null;        // 同上
};

/**
 * 書籍の更新時に送るデータの型定義
 *
 * 更新したいフィールドだけを送ればよいため、
 * すべてのフィールドがオプショナルです。
 * Partial<BookInsert> と同じ意味ですが、明示的に定義しています。
 */
export type BookUpdate = {
  /** 書籍のタイトル */
  title?: string;              // すべて ? 付き（更新時は変更したいフィールドのみ送る）

  /** 著者名 */
  author?: string;             // 同上

  /** 読書ステータス */
  status?: BookStatus;         // 同上

  /** 評価（1〜5、null で評価をクリア） */
  rating?: number | null;      // null を明示的に送ると評価をリセットできる

  /** メモ・感想（null でメモをクリア） */
  memo?: string | null;        // 同上
};

/**
 * ステータスの表示ラベル定義
 *
 * コンポーネントでステータスを日本語表示するときに使います。
 * 例: statusLabels["reading"] → "読書中"
 */
// Record<BookStatus, string>: TypeScript の組み込み型。
// 「キーが BookStatus 型、値が string 型のオブジェクト」を意味する
// 3つのキー（unread, reading, finished）すべてが必須になる
export const statusLabels: Record<BookStatus, string> = {
  unread: "未読",              // "unread" の表示用ラベル
  reading: "読書中",            // "reading" の表示用ラベル
  finished: "読了",             // "finished" の表示用ラベル
};

/**
 * ステータスに対応する色のクラス定義
 *
 * StatusBadge コンポーネントでバッジの色を切り替えるときに使います。
 * Tailwind CSS のクラス名を直接指定しています。
 */
// Record<BookStatus, { bg: string; text: string }>:
// キーが BookStatus、値が { bg: string; text: string } 型のオブジェクト
// 各ステータスごとに「背景色クラス」と「文字色クラス」を持つ
export const statusColors: Record<
  BookStatus,
  { bg: string; text: string }
> = {
  unread: {                    // 未読: グレー系（控えめ）
    bg: "bg-gray-100",         // 背景は薄いグレー
    text: "text-gray-700",     // 文字は濃いめのグレー
  },
  reading: {                   // 読書中: 青系（進行中の印象）
    bg: "bg-blue-100",         // 背景は薄い青
    text: "text-blue-700",     // 文字は濃いめの青
  },
  finished: {                  // 読了: 緑系（完了の印象）
    bg: "bg-green-100",        // 背景は薄い緑
    text: "text-green-700",    // 文字は濃いめの緑
  },
};
```

**コードの解説:**

| 型 / 定数 | 用途 |
|-----------|------|
| `BookStatus` | 読書ステータスを3つのリテラル型のユニオンで定義します。`"unread" \| "reading" \| "finished"` 以外の値を代入しようとすると、TypeScript がコンパイルエラーを出します。 |
| `Book` | データベースの `books` テーブルの1行に対応する型です。`supabase.from("books").select("*")` の返り値はこの型の配列になります。 |
| `BookInsert` | 新規登録時に必要なデータの型です。`title` と `author` が必須、それ以外はオプショナル（`?` 付き）です。`id` や `created_at` はデータベースが自動で付与するため含みません。 |
| `BookUpdate` | 更新時に送るデータの型です。すべてのフィールドがオプショナルで、変更したい項目だけを送ります。 |
| `statusLabels` | ステータスの文字列を日本語ラベルに変換するためのマッピングオブジェクトです。`Record<BookStatus, string>` は「キーが `BookStatus` 型、値が `string` 型のオブジェクト」を意味します。 |
| `statusColors` | ステータスに対応する Tailwind CSS のクラス名をマッピングしたオブジェクトです。StatusBadge コンポーネントでバッジの背景色と文字色を動的に切り替えるために使います。 |

> **Supabase の自動生成型（Database 型）について:**
> 大規模プロジェクトでは Supabase CLI で `npx supabase gen types typescript` を実行し、データベースの定義からTypeScriptの型を自動生成することがあります。生成された型は次のような形をしています。
>
> ```typescript
> export type Database = {
>   public: {
>     Tables: {
>       books: {
>         Row: { id: string; title: string; ... };       // SELECTで返ってくる行の型
>         Insert: { title: string; author: string; ... }; // INSERT時に渡す型
>         Update: { title?: string; ... };                 // UPDATE時に渡す型
>       };
>     };
>   };
> };
> ```
>
> 本チュートリアルでは学習のしやすさのため、手書きで `Book`, `BookInsert`, `BookUpdate` を定義しています。仕組みは同じです。

> **なぜ3つの型を分けるのか？**
>
> データベースとやり取りする場面によって、必要なフィールドが異なります。
>
> - **取得（SELECT）:** すべてのフィールドが返ってくる → `Book` 型
> - **登録（INSERT）:** `id` や `created_at` は不要 → `BookInsert` 型
> - **更新（UPDATE）:** 変更したいフィールドだけ送る → `BookUpdate` 型
>
> 型を分けることで、各場面で正確なデータ構造を強制でき、バグを防げます。

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
  subsets: ["latin"],         // 読み込むサブセット（文字種）。日本語はデフォルトで含まれる
  weight: ["400", "500", "700"], // 使用するフォントの太さ（通常 / やや太字 / 太字）
  display: "swap",            // フォント読み込み中は代替フォントで表示し、完了後に切り替える
  preload: true,              // 重要なフォントなので、HTMLの <link rel="preload"> で先読みする
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
  title: "書籍管理アプリ",                       // <title> タグの中身（ブラウザのタブ表示）
  description:                                  // <meta name="description"> の中身（検索結果に出る説明文）
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

### 7.2 globals.css の確認

`src/app/globals.css` は、`create-next-app` がデフォルトで生成した内容をそのまま使用します。以下の Tailwind CSS ディレクティブが含まれていることを確認してください（不要なデフォルトスタイルは削除して構いません）。

```css
/* src/app/globals.css */

/* @tailwind: Tailwind CSS 専用のディレクティブ。PostCSS が処理時に大量のクラスに展開する */
@tailwind base;        /* リセットCSSと要素のベーススタイル */
@tailwind components;  /* component レイヤー（カスタムコンポーネントクラス） */
@tailwind utilities;   /* 全ユーティリティクラス（最も多くのコードが展開される） */
```

> **Tailwind CSS ディレクティブの意味:**
>
> | ディレクティブ | 役割 |
> |---------------|------|
> | `@tailwind base;` | ブラウザ間の表示差異をなくすリセット CSS と、基本的な要素のスタイルを注入します。 |
> | `@tailwind components;` | コンポーネントクラス（`container` など）を注入します。 |
> | `@tailwind utilities;` | ユーティリティクラス（`flex`, `pt-4`, `text-center` など）をすべて注入します。未使用のクラスはビルド時に自動削除されます。 |

> **Next.js のバージョンによる違い:**
> Next.js 15 以降のバージョンでは、`globals.css` の記法が `@import "tailwindcss";` のように変わっている場合があります。`create-next-app` が生成した内容がどちらの形式であっても、Tailwind CSS は正しく動作しますのでそのまま使用してください。

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
                  ? "bg-blue-50 text-blue-700"                                 // 現在地：薄青背景＋青文字
                  : "text-gray-600 hover:text-gray-900 hover:bg-gray-50"       // 通常：グレー文字、ホバーで反応
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
                  ? "bg-blue-700 text-white"                                   // 現在地：濃い青背景＋白文字
                  : "bg-blue-600 text-white hover:bg-blue-700"                 // 通常：青背景、ホバーで濃く
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
          {/* 未読バッジ */}
          {/* inline-block: インラインブロック表示 / bg-gray-100: 薄グレー背景 / text-gray-700: 濃グレー文字 */}
          {/* text-xs: 12px / px-2.5 py-1: パディング / rounded-full: 完全な丸（カプセル形） / font-medium: 中太字 */}
          <span className="inline-block bg-gray-100 text-gray-700 text-xs px-2.5 py-1 rounded-full font-medium">
            未読
          </span>
          {/* 読書中バッジ：青系 */}
          <span className="inline-block bg-blue-100 text-blue-700 text-xs px-2.5 py-1 rounded-full font-medium">
            読書中
          </span>
          {/* 読了バッジ：緑系 */}
          <span className="inline-block bg-green-100 text-green-700 text-xs px-2.5 py-1 rounded-full font-medium">
            読了
          </span>
        </div>
      </div>
    </div>
  );
}
```

### 9.2 開発サーバーの起動

ターミナルで以下のコマンドを実行します。

```bash
# npm run dev: package.json の "scripts" にある "dev" を実行
# 中身は「next dev --turbopack」で、Next.js の開発サーバーが起動する
# ファイルを保存するたびに自動でブラウザが更新される（ホットリロード）
# 停止するには Ctrl+C を押す
npm run dev
```

以下のような出力が表示されます。

```
   ▲ Next.js 15.x.x (Turbopack)
   - Local:        http://localhost:3000
   - Network:      http://192.168.x.x:3000

 ✓ Starting...
 ✓ Ready in xxxms
```

### 9.3 ブラウザでの確認

ブラウザで `http://localhost:3000` を開くと、以下のように表示されます。

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
- カードの下部に3つのステータスバッジ（「未読」はグレー、「読書中」は青、「読了」は緑）が横並びで表示されます

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
| `Module not found: Can't resolve '@/components/Header'` | ファイルが存在しない、またはパスが間違っている | `src/components/Header.tsx` が正しい場所に作成されているか確認してください。ファイル名の大文字小文字も一致している必要があります。 |
| スタイルが全く適用されない | `globals.css` が正しく読み込まれていない | `src/app/layout.tsx` で `import "./globals.css";` が記述されているか確認してください。 |
| `環境変数 NEXT_PUBLIC_SUPABASE_URL が設定されていません` | `.env.local` が未作成または場所が間違っている | `.env.local` がプロジェクトのルートディレクトリ（`package.json` と同じ階層）に存在するか確認してください。ファイル作成後は開発サーバーの再起動（`Ctrl+C` で停止してから `npm run dev`）が必要です。 |
| ポート 3000 が既に使用されている | 他のアプリが 3000 番を使っている | `npm run dev -- --port 3001` で別のポートを指定するか、使用中のアプリを停止してください。 |
| 日本語フォントが適用されていない | フォントの読み込みに失敗している | ネットワーク接続を確認してください。初回ビルド時に Google Fonts からフォントがダウンロードされます。 |

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
  .from("books")           // books テーブルを指定（SQL の FROM books に相当）
  .select("*")             // すべてのカラムを取得（SQL の SELECT * に相当）
  .order("created_at", {   // 作成日の降順（新しい順）に並び替え（SQL の ORDER BY に相当）
    ascending: false       // false で降順（新しい順）。true なら昇順（古い順）
  });

// 3. エラーハンドリング
// if (error): error が null/undefined でなければ true（エラーが発生した）
if (error) {
  // console.error: 開発者ツールのコンソールにエラーログを赤色で出力
  console.error("書籍の取得に失敗しました:", error.message);
  return;                  // ここで関数を抜ける
}

// 4. data は Book[] 型（書籍の配列）
// data の型は自動で Book[] と推論されるが、明示的に型注釈を付けることもできる
const books: Book[] = data;
```

この1つの `supabase.from("books").select("*")` の呼び出しの裏側で、上の図の 2 → 3 → 4 → 5 → 6 → 7 のステップがすべて自動的に実行されています。Supabase クライアントがこれらの複雑な処理を抽象化してくれるため、開発者は SQL を直接書くことなく、シンプルな JavaScript/TypeScript のコードでデータベースを操作できます。

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

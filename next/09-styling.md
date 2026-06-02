# 第9章: スタイリング & UI ポリッシュ

> 機能が完成したアプリの**見た目と使い心地**を本格的に仕上げる章です。「動くだけ」のアプリを「使いたくなる」アプリに変えていきます。
>
> ここまでの章で書籍管理アプリの「動き」（データの保存・取得・編集・削除）は完成しました。本章では、その動きを包む「見た目」と「触り心地」（UI/UX、User Interface / User Experience：ユーザーが画面を見て触る部分の体験）に焦点を当てます。

### この章で学ぶこと

| テーマ | 内容 | なぜ重要か |
|--------|------|----------|
| **Tailwind CSS 実践** | CSSカスタムプロパティ（CSS Custom Properties / CSS変数：`--変数名` で色やサイズを一元管理する仕組み）の設定 | デザインの一貫性を保つため |
| **レスポンシブデザイン** | レスポンシブ（responsive：反応する）デザイン。画面サイズに応じてレイアウトを変える仕組み。スマホ/タブレット/PC全てに対応 | ユーザーは様々なデバイスでアクセスするため |
| **UIコンポーネント改善** | トースト通知（toast：パンが焼き上がる時のように下からひょいと出るメッセージ。操作結果を一時的に表示）、ページネーション（pagination：大量データを複数ページに分割表示する仕組み） | ユーザー体験の向上のため |
| **アニメーション** | ホバー（hover：マウスを要素の上に乗せた時）エフェクト、トランジション（CSS Transition：プロパティの変化を時間をかけて滑らかにつなぐ機能） | 操作のフィードバックを伝えるため |
| **ダークモード** | dark mode：背景が黒・濃灰色など暗い配色に切り替わるモード。夜間や暗い場所で目の負担を軽減 | ユーザーの好みに対応するため |
| **アクセシビリティ** | a11y（accessibility：「a」と「y」の間に11文字あるので a11y と略す。障害のある方を含む全ての人がWebを利用できるようにすること） | すべての人に使いやすいアプリにするため |

書籍管理アプリの見た目を本格的に仕上げていく章です。Tailwind CSS を活用したレスポンシブデザイン、アニメーション、ダークモード、アクセシビリティ（accessibility：Webコンテンツをすべての人が利用できるようにするための取り組み）まで網羅的に学びます。

> **デザインが重要な理由：** 同じ機能のアプリでも、見た目が整っているだけで「使いやすそう」「信頼できそう」という印象を与えます。ポートフォリオ（portfolio：作品集）に載せる際も、デザインの良さは大きなアピールポイントになります。
>
> **本章で出てくる主要な技術用語の早見表：**
>
> - **ユーティリティファースト（utility-first）**: 「`bg-red-500`（背景赤）」のような細かいパーツ単位のクラスを組み合わせてスタイルを作る考え方。Tailwind CSS が採用している。
> - **BEM（ベム、Block Element Modifier）**: 「.card__title--active」のように、ブロック名と要素名と修飾子をハイフンとアンダースコアで区切るCSSクラス命名規則。
> - **スコープドスタイル（scoped style）**: スタイルを特定のコンポーネントの中だけに閉じ込めて、他のコンポーネントに影響しないようにする仕組み。
> - **CSS-in-JS**: JavaScript の中に直接 CSS を書く方式（styled-components や Emotion などのライブラリ）。
> - **shadcn/ui（シャドシーエヌ・ユーアイ）**: コピー＆ペーストで自分のプロジェクトに取り込む形のUIコンポーネント集。Tailwind CSS と Radix UI ベース。
> - **CSS Modules**: 「`Button.module.css`」のような特別な命名のCSSファイルを使うと、クラス名が自動で一意になり、コンポーネントごとにスコープが分離される仕組み。

---

## 目次

0. [前提知識: CSSとTailwindの超基礎](#0-前提知識-cssとtailwindの超基礎)
1. [Tailwind CSS 実践](#1-tailwind-css-実践)
2. [レスポンシブデザイン](#2-レスポンシブデザイン)
3. [UI コンポーネントの改善](#3-ui-コンポーネントの改善)
4. [アニメーション](#4-アニメーション)
5. [ダークモード（発展）](#5-ダークモード発展)
6. [アクセシビリティ](#6-アクセシビリティ)
7. [最終的な画面の説明](#7-最終的な画面の説明)
8. [コンポーネント構成図（最終版）](#8-コンポーネント構成図最終版)

---

## 0. 前提知識: CSSとTailwindの超基礎

### 0.1 CSSって何？

**CSS（Cascading Style Sheets、カスケーディング・スタイル・シーツ）** はHTMLの「見た目」を指定する言語です。「カスケード」とは「滝のように上から下へ流れる」という意味で、後から書いたスタイルが前のものを上書きする性質に由来します。色・フォント・余白・配置などを記述します。

```html
<!-- HTMLファイル側 -->
<!-- button タグはクリックできるボタンを作るHTML要素 -->
<!-- class="btn-primary" はこのボタンに「btn-primary」という名札を付ける指定 -->
<!-- ↑ CSS側で .btn-primary { ... } と書くと、ここに見た目が当たる -->
<button class="btn-primary">送信</button>
```

```css
/* CSSファイル側 */
/* ピリオド「.」で始まる名前は「クラスセレクタ」と呼ばれ、 */
/* HTML の class="btn-primary" を持つ要素にスタイルを当てる */
.btn-primary {
  background-color: #3b82f6;  /* 背景色を指定。#3b82f6 は16進数で青色を表す */
  color: white;                /* 文字色を白に。color プロパティは「文字色」を指す */
  padding: 8px 16px;           /* 内側の余白。上下8px、左右16px。padding は要素内部の余白 */
  border-radius: 8px;          /* 角を丸める。8px の半径で四隅を丸くする */
  border: none;                /* 枠線を消す。none = 何も無い */
}
```

> **ボックスモデル（box model）の補足：** HTML の全ての要素は、上から順に「コンテンツ（中身） → padding（内側の余白） → border（枠線） → margin（外側の余白）」という4層構造で囲まれた箱として描画されます。これを「ボックスモデル」と呼びます。`padding` は箱の中の余白、`margin` は箱と外の他の要素との間の余白、`border` はその境界線です。この違いを理解しておくと、後で出てくる Tailwind の `p-4` / `m-4` / `border` の使い分けがスムーズになります。

**▼ 表示結果:**

```
┌──────────┐
│   送信    │  ← 青背景、白文字、角丸のボタン
└──────────┘
```

### 0.2 主要なCSSプロパティ

| プロパティ | 何を変える？ | 例 |
|-----------|------------|-----|
| `color` | 文字色 | `color: red;` |
| `background-color` | 背景色 | `background-color: #fff;` |
| `font-size` | 文字サイズ | `font-size: 16px;` |
| `font-weight` | 文字の太さ | `font-weight: bold;` |
| `padding` | 内側の余白 | `padding: 8px 16px;` |
| `margin` | 外側の余白 | `margin: 16px;` |
| `border` | 枠線 | `border: 1px solid #ccc;` |
| `border-radius` | 角丸 | `border-radius: 8px;` |
| `display` | 表示方式 | `display: flex;` |
| `width` / `height` | 幅・高さ | `width: 100%;` |

### 0.3 Tailwind CSS とは

通常のCSSは「CSSファイルを別に作って、クラスを定義して、HTMLに `class` を書く」という流れですが、これを毎回やると**クラス命名で消耗**します。「このボタン用のクラス名は何にしよう…」と毎回悩むのは意外と大変なのです。

**Tailwind CSS（テイルウィンド・シーエスエス）** は「あらかじめ大量のクラスが用意されていて、`bg-blue-500`（背景を青の500段階の色に）のようなクラス名をHTMLに直接書くだけでスタイルが当たる」という仕組みです。この考え方を**ユーティリティファースト（utility-first：小さな機能単位のクラスを組み合わせて作る方式）** と呼びます。

ちなみに従来のCSS設計では **BEM**（Block Element Modifier、`.card__title--active` のようにブロック・要素・修飾子で命名する規則）や **CSS Modules**（`Button.module.css` のようなファイルで自動的にクラス名をユニーク化する仕組み）、**CSS-in-JS**（styled-components 等、JS の中にCSSを書く方式）といった様々なアプローチがありました。Tailwind はそれらと並ぶ「もう一つの選択肢」です。

**▼ 同じボタンを Tailwind で書くと:**

```html
<!-- bg-blue-500: 背景色を青の500段階目に -->
<!-- text-white  : 文字色を白に -->
<!-- px-4        : padding（内側余白）の横方向を 1rem (16px) -->
<!-- py-2        : padding の縦方向を 0.5rem (8px) -->
<!-- rounded-lg  : 角を「大きめ」に丸める（border-radius: 0.5rem） -->
<button class="bg-blue-500 text-white px-4 py-2 rounded-lg">
  送信
</button>
```

CSSファイルを書く必要がありません。HTMLにクラスを書くだけで完結します。

### 0.4 Tailwindクラス命名のパターン

ほぼ全クラスは「**プロパティの省略** + `-` + **値**」の形をしています。

| クラス | 意味 | 通常のCSS換算 |
|--------|------|----------------|
| `bg-red-500` | 背景色を赤500 | `background-color: #ef4444;` |
| `text-white` | 文字色を白 | `color: white;` |
| `text-xl` | 文字サイズXL | `font-size: 1.25rem;` |
| `p-4` | 全方向 padding 16px | `padding: 1rem;` |
| `px-4` | 横方向 padding | `padding-left: 1rem; padding-right: 1rem;` |
| `py-2` | 縦方向 padding | `padding-top: 0.5rem; padding-bottom: 0.5rem;` |
| `m-4` | 全方向 margin | `margin: 1rem;` |
| `rounded-lg` | 角丸 大 | `border-radius: 0.5rem;` |
| `flex` | flexコンテナに | `display: flex;` |
| `items-center` | 縦方向中央揃え | `align-items: center;` |
| `gap-2` | 子要素間の隙間 | `gap: 0.5rem;` |
| `w-full` | 幅100% | `width: 100%;` |
| `hover:bg-blue-700` | マウスホバー時の色 | `:hover { background: ...; }` |

### 0.5 数値スケール

Tailwind の数値（`p-4`, `text-xl`）は**4の倍数のpx**などの規則があります。

| Tailwind | 値 | px換算 |
|----------|-----|--------|
| `p-1` | 0.25rem | 4px |
| `p-2` | 0.5rem | 8px |
| `p-4` | 1rem | 16px |
| `p-8` | 2rem | 32px |
| `text-xs` | 0.75rem | 12px |
| `text-sm` | 0.875rem | 14px |
| `text-base` | 1rem | 16px |
| `text-lg` | 1.125rem | 18px |
| `text-xl` | 1.25rem | 20px |
| `text-2xl` | 1.5rem | 24px |

### 0.6 色のスケール

色は `red-50` 〜 `red-950` の11段階で、数字が大きいほど濃くなります。

```
50  100 200 300 400 500 600 700 800 900 950
↑ 一番薄い              ↑ 標準              ↑ 一番濃い
```

> **コツ:** 「背景は薄い色（100, 200）、文字は濃い色（700, 800, 900）」という組み合わせがおすすめ。`bg-blue-100 text-blue-800` で柔らかい青のバッジになります。

### 0.7 レスポンシブの書き方

「PCでは横並び、スマホでは縦並び」のように画面サイズで変えるには **接頭辞（prefix、プレフィックス：先頭につける目印）** を付けます。

```html
<!-- ↓ Tailwind の典型的なレスポンシブ指定 -->
<!-- flex-col      : flex レイアウトで「縦並び」（column）にする -->
<!-- md:flex-row   : 「md」接頭辞は 768px 以上で適用。横並び（row）に切り替わる -->
<!-- ※ flex-col と md:flex-row の両方を書くことで、 -->
<!--   「小さい画面では縦、大きくなったら横」と動的に切り替わる -->
<div class="flex-col md:flex-row">
  <!-- スマホ: 縦並び（flex-col） -->
  <!-- 768px 以上（md）: 横並び（flex-row） -->
</div>
```

| 接頭辞 | 何以上で適用？ | 想定デバイス |
|--------|--------------|--------------|
| `sm:` | 640px 以上 | 大きめスマホ |
| `md:` | 768px 以上 | タブレット |
| `lg:` | 1024px 以上 | ノートPC |
| `xl:` | 1280px 以上 | デスクトップ |

> **モバイルファーストの原則：** Tailwind では「接頭辞なし＝最小画面（モバイル）」「接頭辞あり＝それ以上の画面で上書き」というルールになっています。つまり最初にスマホ用のレイアウトを書き、画面が大きくなるにつれて `sm:` `md:` `lg:` で部分的に上書きしていきます。これを**モバイルファースト（mobile-first）** と呼びます。逆に「PCを基準に書いて、小さい画面用に縮める」やり方は古いスタイルです。
>
> **flex と grid の使い分けの目安：**
>
> - **flex（フレックスボックス）**: 1次元（横一列、または縦一列）の並びに向く。ボタンを横に並べる、ヘッダーの中身を左右に配置する、など。
> - **grid（グリッド）**: 2次元（行と列の格子）に向く。書籍カードを「4列×何行」の格子状に並べるなど。

---

---

## 1. Tailwind CSS 実践

### 1.1 globals.css のカスタマイズ

`app/globals.css` にアプリ全体で使うカスタムスタイルを定義します。**CSS カスタムプロパティ（CSS Custom Properties / CSS variables：`--name` のように定義し `var(--name)` で参照する変数機能）** を使い、ライトモードとダークモードの両方に対応できる設計にします。

このセクションで出てくる Tailwind 独自ディレクティブも先に整理しておきます。

- `@tailwind base / components / utilities` : それぞれのレイヤーのCSSを「ここに展開してね」と Tailwind に指示する命令。
- `@layer base { ... }` : ベース層（最も優先度が低い）にCSSを追加するブロック。
- `@layer components { ... }` : 自作のコンポーネントクラス（.btn など）を入れる層。
- `@layer utilities { ... }` : 自作のユーティリティクラスを入れる層。
- `@apply クラス名1 クラス名2` : CSS内でTailwindクラスを展開する命令。長いクラス指定を1つにまとめるのに便利。

> **▼ このコードがやること（先に日本語で）:** アプリ全体で使う「色・影・角丸・アニメーション」をまとめて定義する土台のCSSファイルを作ります。ポイントは、色などを `--color-primary-500` のような「CSS変数（名前を付けた値の入れ物）」にしておくことです。こうすると、ダークモードのときは変数の中身を入れ替えるだけで画面全体の色が一気に切り替わります。各行の細かい意味はコード内コメントで丁寧に説明しています。

```css
/* app/globals.css
   ↑ このファイルはアプリ全体に適用される「グローバルCSS」。
     Next.js の App Router では app/layout.tsx で import "./globals.css" すると
     アプリのすべてのページに反映される。 */

/* @tailwind ディレクティブは Tailwind CSS が提供する特別な命令文。
   ビルド時に、それぞれ大量のCSSコードに展開される。
   この3行が無いと Tailwind のクラスは一切効かない。 */
@tailwind base;       /* base   : ブラウザ標準スタイルのリセット（normalize.css 的なもの）+ HTML要素のデフォルト */
@tailwind components; /* components: .btn など、@layer components { ... } で定義したクラス */
@tailwind utilities;  /* utilities : bg-red-500, p-4 などのユーティリティクラス本体 */

/* ========================================
   1. CSS カスタムプロパティ（カラートークン）
   ======================================== */
/* :root は「ドキュメントのルート要素（≒ <html> タグ）」を指す擬似クラスセレクタ。
   ここに変数を定義すると、ページ全体のどこからでも var(--変数名) で参照できる。
   このように共通の値を変数化したものを「デザイントークン（design tokens）」と呼ぶ。 */
:root {
  /* プライマリカラー（インディゴ系：青と紫の中間のような色） */
  /* --color- で始まる名前は「CSSカスタムプロパティ（CSS変数）」。
     値（例：#eef2ff）は16進数カラーコード。先頭の # の後の2桁ずつが
     赤・緑・青の強さを 00〜FF で表す。 */
  --color-primary-50: #eef2ff;   /* 一番薄いインディゴ（背景バッジ等に使う） */
  --color-primary-100: #e0e7ff;  /* 薄め */
  --color-primary-200: #c7d2fe;
  --color-primary-300: #a5b4fc;
  --color-primary-400: #818cf8;
  --color-primary-500: #6366f1;  /* 標準のプライマリ色 */
  --color-primary-600: #4f46e5;  /* ボタン背景でよく使う濃さ */
  --color-primary-700: #4338ca;  /* ホバー時の濃さ */
  --color-primary-800: #3730a3;
  --color-primary-900: #312e81;
  --color-primary-950: #1e1b4b;  /* 一番濃いインディゴ（ダークモード背景等） */

  /* セカンダリカラー（エメラルド系：緑系の鮮やかな色） */
  --color-secondary-50: #ecfdf5;
  --color-secondary-100: #d1fae5;
  --color-secondary-200: #a7f3d0;
  --color-secondary-300: #6ee7b7;
  --color-secondary-400: #34d399;
  --color-secondary-500: #10b981;  /* 標準のセカンダリ色 */
  --color-secondary-600: #059669;
  --color-secondary-700: #047857;
  --color-secondary-800: #065f46;
  --color-secondary-900: #064e3b;
  --color-secondary-950: #022c22;

  /* 背景・テキスト（セマンティック＝意味ベースの命名） */
  --color-background: #ffffff;          /* ページ全体の背景色（白） */
  --color-foreground: #0f172a;          /* 前景色＝メインの文字色（ほぼ黒） */
  --color-muted: #64748b;               /* muted = 弱めた色。補足テキスト用 */
  --color-muted-foreground: #94a3b8;    /* さらに薄い補足文字色 */
  --color-border: #e2e8f0;              /* 枠線の色 */
  --color-card: #ffffff;                /* カード（書籍カードなど）の背景色 */
  --color-card-foreground: #0f172a;     /* カード内の文字色 */

  /* 状態カラー（操作結果や状態を伝えるための色） */
  --color-success: #10b981;  /* 成功＝緑 */
  --color-warning: #f59e0b;  /* 警告＝黄/オレンジ */
  --color-error: #ef4444;    /* エラー＝赤 */
  --color-info: #3b82f6;     /* 情報＝青 */

  /* シャドウ（box-shadow：影） */
  /* box-shadow の値は「横ずれ 縦ずれ ぼかし 広がり 色」の順に書く */
  /* rgb(0 0 0 / 0.05) は黒で透明度5%という意味 */
  --shadow-sm: 0 1px 2px 0 rgb(0 0 0 / 0.05);
  --shadow-md: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1); /* 2つの影を重ねる */
  --shadow-lg: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);

  /* ボーダー半径（角丸の大きさ） */
  /* rem は「ルート要素のフォントサイズを基準とする単位」。普通は 1rem = 16px */
  --radius-sm: 0.375rem;  /* = 6px */
  --radius-md: 0.5rem;    /* = 8px */
  --radius-lg: 0.75rem;   /* = 12px */
  --radius-xl: 1rem;      /* = 16px */
}

/* ダークモード用のカスタムプロパティ */
/* <html class="dark"> のように dark クラスが付いたとき、
   この中の変数だけが上書きされる。CSS変数の「値だけを差し替える」
   仕組みなので、各コンポーネントで dark:bg-... を一つずつ書かなくても
   ダークモードに対応できる。 */
.dark {
  --color-background: #0f172a;        /* ほぼ黒（slate-900）に */
  --color-foreground: #f8fafc;        /* 文字は逆に白系に */
  --color-muted: #94a3b8;
  --color-muted-foreground: #64748b;
  --color-border: #334155;             /* 暗い枠線 */
  --color-card: #1e293b;               /* カード背景は背景より少し明るい暗色 */
  --color-card-foreground: #f8fafc;

  /* 影は暗い画面ではより濃く（透明度を上げる）と見える */
  --shadow-sm: 0 1px 2px 0 rgb(0 0 0 / 0.3);
  --shadow-md: 0 4px 6px -1px rgb(0 0 0 / 0.4), 0 2px 4px -2px rgb(0 0 0 / 0.3);
  --shadow-lg: 0 10px 15px -3px rgb(0 0 0 / 0.4), 0 4px 6px -4px rgb(0 0 0 / 0.3);
}

/* ========================================
   2. ベースレイヤー
   ======================================== */
/* @layer base は Tailwind の「層（layer）」のひとつ。
   ここに書くと、ユーティリティクラスより優先度が低くなるので
   個別の Tailwind クラスで上書きしやすい。 */
@layer base {
  /* HTML・Body のリセットと基本設定 */
  html {
    scroll-behavior: smooth;          /* ページ内リンクのスクロールをなめらかに */
    -webkit-font-smoothing: antialiased;  /* Mac の Chrome/Safari でフォントを滑らかに表示 */
    -moz-osx-font-smoothing: grayscale;   /* Mac の Firefox でフォントを滑らかに表示 */
  }

  body {
    /* @apply は Tailwind 独自のディレクティブ。CSS内で Tailwind クラスを使うための書き方。 */
    /* bg-[var(--color-background)] : 背景色を CSS 変数から取得（任意値構文 [ ]） */
    /* text-[var(--color-foreground)] : 文字色を CSS 変数から取得 */
    @apply bg-[var(--color-background)] text-[var(--color-foreground)];
    font-feature-settings: "rlig" 1, "calt" 1;  /* OpenType の合字機能を有効化（綺麗な文字に） */
    /* ダークモード切替時に背景・文字色を 0.3 秒かけて滑らかに変える */
    transition: background-color 0.3s ease, color 0.3s ease;
  }

  /* フォーカスリングのデフォルトスタイル */
  /* :focus-visible は「キーボードでフォーカスされたとき」のみ発火する擬似クラス。
     マウスクリック時には発火しないので、マウスユーザーに邪魔にならない。 */
  *:focus-visible {
    /* outline-2          : outline（外側の輪郭線）の太さ 2px */
    /* outline-offset-2   : outline を要素から 2px 離す */
    /* outline-primary-500: outline の色を primary-500 に */
    @apply outline-2 outline-offset-2 outline-primary-500;
  }

  /* 見出しのデフォルトスタイル */
  h1 {
    /* text-3xl       : 文字サイズ 1.875rem (30px) */
    /* font-bold      : 文字の太さを「bold（700）」に */
    /* tracking-tight : 文字間隔（letter-spacing）を狭く（-0.025em） */
    @apply text-3xl font-bold tracking-tight;
  }

  h2 {
    /* text-2xl       : 1.5rem (24px) */
    /* font-semibold  : 太字よりやや細い 600 */
    @apply text-2xl font-semibold tracking-tight;
  }

  h3 {
    /* text-xl        : 1.25rem (20px) */
    @apply text-xl font-semibold;
  }

  /* リンクのデフォルトスタイル */
  a {
    /* text-primary-600           : ライトモード時の文字色 */
    /* hover:text-primary-700     : ホバー時はより濃く */
    /* dark:text-primary-400      : ダークモード時はより明るく（dark: 接頭辞） */
    /* dark:hover:text-primary-300: ダーク × ホバーの組み合わせ */
    @apply text-primary-600 hover:text-primary-700 dark:text-primary-400 dark:hover:text-primary-300;
    transition: color 0.15s ease;   /* 色変化を 0.15 秒かけて滑らかに */
  }
}

/* ========================================
   3. コンポーネントレイヤー
   ======================================== */
/* @layer components はベースより優先度が高く、ユーティリティより低い層。
   .btn のような再利用可能なクラスをここに定義する。 */
@layer components {
  /* ボタンの共通スタイル（全ボタンの土台） */
  .btn {
    /* inline-flex                  : display: inline-flex（横並び＋インライン要素） */
    /* items-center                 : 縦方向中央揃え（align-items: center） */
    /* justify-center               : 横方向中央揃え（justify-content: center） */
    /* rounded-lg                   : 角丸 0.5rem */
    /* px-4 py-2                    : 横16px、縦8px の内側余白 */
    /* text-sm                      : フォントサイズ 0.875rem (14px) */
    /* font-medium                  : 文字太さ 500 */
    /* transition-all duration-200  : 全プロパティを 200ms かけて変化 */
    /* ease-in-out                  : 加速→減速のカーブ */
    /* focus-visible:outline-none   : キーボードフォーカス時のデフォルト輪郭線を消す */
    /* focus-visible:ring-2         : 代わりに 2px のリング（box-shadow） */
    /* focus-visible:ring-offset-2  : リングを要素から 2px 離す */
    /* disabled:pointer-events-none : 無効化時はクリックできない */
    /* disabled:opacity-50          : 無効化時は半透明（50%） */
    @apply inline-flex items-center justify-center
           rounded-lg px-4 py-2
           text-sm font-medium
           transition-all duration-200 ease-in-out
           focus-visible:outline-none focus-visible:ring-2
           focus-visible:ring-offset-2
           disabled:pointer-events-none disabled:opacity-50;
  }

  .btn-primary {
    /* btn                          : 上で定義した .btn を継承（@apply の中でカスタムクラスも使える） */
    /* bg-primary-600 text-white    : インディゴ背景に白文字 */
    /* hover:bg-primary-700         : ホバー時にやや濃く */
    /* active:bg-primary-800        : クリック中（押している瞬間）はさらに濃く */
    /* focus-visible:ring-primary-500 : フォーカスリングをインディゴで */
    @apply btn
           bg-primary-600 text-white
           hover:bg-primary-700
           active:bg-primary-800
           focus-visible:ring-primary-500;
  }

  .btn-secondary {
    /* セカンダリ（エメラルド系）のボタン。primary と同じ構造で色だけ違う */
    @apply btn
           bg-secondary-600 text-white
           hover:bg-secondary-700
           active:bg-secondary-800
           focus-visible:ring-secondary-500;
  }

  .btn-outline {
    /* アウトラインボタン：枠線だけのスタイル */
    /* border-2 border-primary-600 : 2px の枠線をプライマリ色で */
    /* text-primary-600            : 文字色もプライマリ */
    /* hover:bg-primary-50         : ホバー時にうっすら塗りつぶし */
    /* active:bg-primary-100       : 押下時はもう少し濃く */
    /* dark:border-primary-400 ... : ダークモード用は明るめのプライマリで */
    /* dark:hover:bg-primary-950   : ダークでのホバーは超暗いプライマリ */
    @apply btn
           border-2 border-primary-600 text-primary-600
           hover:bg-primary-50
           active:bg-primary-100
           dark:border-primary-400 dark:text-primary-400
           dark:hover:bg-primary-950
           focus-visible:ring-primary-500;
  }

  .btn-danger {
    /* 危険な操作（削除など）用の赤いボタン */
    @apply btn
           bg-red-600 text-white
           hover:bg-red-700
           active:bg-red-800
           focus-visible:ring-red-500;
  }

  .btn-ghost {
    /* ゴーストボタン：通常は背景なし、ホバー時だけ薄く塗る控えめなボタン */
    @apply btn
           text-gray-600 hover:bg-gray-100
           dark:text-gray-400 dark:hover:bg-gray-800
           focus-visible:ring-gray-500;
  }

  /* カードの共通スタイル */
  .card {
    /* rounded-xl                                 : 角丸 1rem */
    /* border border-[var(--color-border)]        : 枠線をCSS変数経由で */
    /* bg-[var(--color-card)]                     : 背景色 */
    /* text-[var(--color-card-foreground)]        : 文字色 */
    /* shadow-sm                                  : 小さめの影 */
    @apply rounded-xl border border-[var(--color-border)]
           bg-[var(--color-card)] text-[var(--color-card-foreground)]
           shadow-sm;
    /* 影と位置変化を 0.2 秒かけて滑らかに */
    transition: box-shadow 0.2s ease, transform 0.2s ease;
  }

  .card-hover {
    /* card に「ホバーで影を大きく＋少し上に浮く」を追加 */
    /* hover:shadow-lg     : ホバー時に大きな影 */
    /* hover:-translate-y-1: 上に -0.25rem (4px) 移動 */
    @apply card hover:shadow-lg hover:-translate-y-1;
  }

  /* 入力フィールドの共通スタイル */
  .input {
    /* w-full                              : 横幅100% */
    /* rounded-lg border ...               : 角丸＋枠線 */
    /* px-4 py-2.5                         : 内側余白（py-2.5 は 0.625rem = 10px） */
    /* text-sm                             : 14px */
    /* placeholder:text-...                : placeholder（入力前の薄い案内文）の色 */
    /* focus:border-primary-500            : フォーカス時の枠線色 */
    /* focus:outline-none                  : ブラウザ標準のアウトラインを消す */
    /* focus:ring-2                        : 代わりに2pxリング */
    /* focus:ring-primary-500/20           : リングの色＝primary-500 の透明度20% */
    /* disabled:cursor-not-allowed         : 無効化時はカーソルが「禁止マーク」に */
    /* disabled:opacity-50                 : 無効時は半透明 */
    /* dark:focus:border-primary-400       : ダーク × フォーカス時の枠線色 */
    @apply w-full rounded-lg border border-[var(--color-border)]
           bg-[var(--color-background)]
           px-4 py-2.5
           text-sm
           placeholder:text-[var(--color-muted-foreground)]
           focus:border-primary-500 focus:outline-none focus:ring-2
           focus:ring-primary-500/20
           disabled:cursor-not-allowed disabled:opacity-50
           dark:focus:border-primary-400 dark:focus:ring-primary-400/20;
    transition: border-color 0.15s ease, box-shadow 0.15s ease;
  }

  /* バッジ（badge：ラベルや状態を示す小さな印） */
  .badge {
    /* inline-flex items-center : 横並びで中央揃え（中にアイコン入れる時のため） */
    /* rounded-full             : 角丸を最大（円形/カプセル形） */
    /* px-2.5 py-0.5            : 横10px、縦2px の余白 */
    /* text-xs                  : 12px */
    /* font-medium              : 太さ500 */
    @apply inline-flex items-center rounded-full
           px-2.5 py-0.5
           text-xs font-medium;
  }

  .badge-primary {
    /* 「読書中」など、注目度を上げたいバッジ */
    /* bg-primary-100 text-primary-700 : 薄い背景に濃い文字色 */
    /* dark:bg-primary-900 dark:text-primary-300 : ダークモードは反転 */
    @apply badge bg-primary-100 text-primary-700
           dark:bg-primary-900 dark:text-primary-300;
  }

  .badge-success {
    /* 「読了」など成功状態を表す緑バッジ */
    @apply badge bg-green-100 text-green-700
           dark:bg-green-900 dark:text-green-300;
  }

  .badge-warning {
    /* 「読みたい」「注意」など警告を表す黄バッジ */
    @apply badge bg-yellow-100 text-yellow-700
           dark:bg-yellow-900 dark:text-yellow-300;
  }

  .badge-error {
    /* エラー状態を表す赤バッジ */
    @apply badge bg-red-100 text-red-700
           dark:bg-red-900 dark:text-red-300;
  }
}

/* ========================================
   4. ユーティリティレイヤー
   ======================================== */
/* @layer utilities は最も優先度の高い層。
   Tailwind が標準で提供しないユーティリティをここに自作できる。 */
@layer utilities {
  /* テキスト省略（複数行対応） */
  /* line-clamp は「N 行を超えたら省略記号「…」で打ち切る」機能 */
  .line-clamp-1 {
    display: -webkit-box;             /* WebKit 系の特殊な display 値（複数行省略に必要） */
    -webkit-line-clamp: 1;            /* 1 行で打ち切る */
    -webkit-box-orient: vertical;     /* ボックスを縦方向に並べる */
    overflow: hidden;                 /* はみ出た部分を非表示 */
  }

  .line-clamp-2 {
    display: -webkit-box;
    -webkit-line-clamp: 2;            /* 2 行で打ち切る */
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .line-clamp-3 {
    display: -webkit-box;
    -webkit-line-clamp: 3;            /* 3 行で打ち切る */
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  /* スクロールバーのカスタマイズ */
  .scrollbar-thin {
    scrollbar-width: thin;            /* Firefox 用: スクロールバーを細く */
    scrollbar-color: var(--color-muted-foreground) transparent;  /* つまみ色 トラック色 */
  }

  /* ::-webkit-scrollbar は Chrome/Safari 用のスクロールバー擬似要素 */
  .scrollbar-thin::-webkit-scrollbar {
    width: 6px;     /* 縦スクロールバーの幅 */
    height: 6px;    /* 横スクロールバーの高さ */
  }

  .scrollbar-thin::-webkit-scrollbar-track {
    background: transparent;  /* スクロールバーの「軌道（背景部分）」を透明に */
  }

  .scrollbar-thin::-webkit-scrollbar-thumb {
    background-color: var(--color-muted-foreground);  /* つまみの色 */
    border-radius: 3px;                                /* つまみを丸く */
  }

  /* グラスモーフィズム（glassmorphism：すりガラス風効果） */
  .glass {
    /* bg-white/80      : 白の透明度80%背景（数字 / 数字 で透明度指定） */
    /* backdrop-blur-md : 背景をぼかす（半透明の向こう側がモザイク状に） */
    /* dark:bg-gray-900/80 : ダーク時は暗い半透明背景 */
    @apply bg-white/80 backdrop-blur-md dark:bg-gray-900/80;
  }
}

/* ========================================
   5. アニメーション定義
   ======================================== */
/* @keyframes はアニメーションの「始まりから終わりまでのコマ」を定義する。
   from { 開始状態 } to { 終了状態 } の形が基本。
   0% { ... } 100% { ... } のようにパーセント刻みで複数指定もできる。 */

/* fadeIn: 少し下から、透明 → 不透明にフェードイン */
@keyframes fadeIn {
  from {
    opacity: 0;                       /* 透明（見えない） */
    transform: translateY(8px);       /* 8px 下にずらした位置から始まる */
  }
  to {
    opacity: 1;                       /* 不透明（見える） */
    transform: translateY(0);         /* 元の位置へ */
  }
}

/* fadeOut: 上記の逆。表示状態 → 透明＋少し下に消える */
@keyframes fadeOut {
  from {
    opacity: 1;
    transform: translateY(0);
  }
  to {
    opacity: 0;
    transform: translateY(8px);
  }
}

/* 右からスライドイン（X軸＝横方向の動き） */
@keyframes slideInRight {
  from {
    opacity: 0;
    transform: translateX(16px);      /* 16px 右にずれた位置 */
  }
  to {
    opacity: 1;
    transform: translateX(0);          /* 元の位置 */
  }
}

/* 下からスライドアップ（カードの一斉登場などに使う） */
@keyframes slideInUp {
  from {
    opacity: 0;
    transform: translateY(16px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* scaleIn: 少し小さい状態 → 通常サイズに拡大しながら出現 */
@keyframes scaleIn {
  from {
    opacity: 0;
    transform: scale(0.95);           /* 95% の大きさ */
  }
  to {
    opacity: 1;
    transform: scale(1);              /* 100%（等倍） */
  }
}

/* spin: 1周回転（ローディングスピナー用） */
@keyframes spin {
  to {
    transform: rotate(360deg);        /* 360度回す */
  }
}

/* shimmer: ローディングスケルトンの光が左から右へ流れるアニメーション */
@keyframes shimmer {
  0% {
    background-position: -200% 0;     /* 背景画像を左外側に配置 */
  }
  100% {
    background-position: 200% 0;      /* 右外側まで動かす */
  }
}

/* トーストが右から滑り込んでくる */
@keyframes toastSlideIn {
  from {
    opacity: 0;
    transform: translateX(100%);      /* 100% = 自身の幅ぶん右にいる状態 */
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

/* トーストが右に滑り出ていく（消える時） */
@keyframes toastSlideOut {
  from {
    opacity: 1;
    transform: translateX(0);
  }
  to {
    opacity: 0;
    transform: translateX(100%);
  }
}
```

#### ▼ コードを1つずつ分解して解説

このCSSはとても長く見えますが、やっていることは「①色などの値に名前を付ける」「②その名前を使ってまとめてスタイルを定義する」の2つだけです。塊ごとに見ていきましょう。

---

##### 解説1: Tailwind を読み込む3行（`@tailwind`）

```css
@tailwind base;       /* base   : ブラウザ標準スタイルのリセット（normalize.css 的なもの）+ HTML要素のデフォルト */
@tailwind components; /* components: .btn など、@layer components { ... } で定義したクラス */
@tailwind utilities;  /* utilities : bg-red-500, p-4 などのユーティリティクラス本体 */
```

- `@tailwind`（アットマーク・テイルウィンド）は Tailwind 専用の「命令文」です。ビルド時にこの1行が**数千行のCSS**に展開されます。
- `base` は「ブラウザごとに違う初期スタイルをそろえるリセット」、`components` は「`.btn` など自作のまとまったクラス」、`utilities` は「`p-4`・`bg-red-500` などの細かい1機能クラス」を入れる場所です。
- **この3行が無いと Tailwind のクラスは一切効きません。** 必ずファイルの先頭に書きます。

> **用語: ディレクティブ（directive）** … CSSやツールに対する「特別な指示」を表す命令文のこと。`@` から始まるものが多い（`@tailwind`, `@layer`, `@apply` など）。

---

##### 解説2: 色に名前を付ける（CSS変数 / カラートークン）

```css
:root {
  --color-primary-50: #eef2ff;   /* 一番薄いインディゴ（背景バッジ等に使う） */
  /* ... 中略 ... */
  --color-primary-600: #4f46e5;  /* ボタン背景でよく使う濃さ */
}
```

- `:root`（ルート）は「ページの一番外側（`<html>`）」を指します。ここに変数を書くと、ページのどこからでも `var(--変数名)` で呼び出せます。
- `--color-primary-600` のように `--` で始まる名前が **CSS変数（カスタムプロパティ）** です。値の `#4f46e5` は色を表す16進数コードで、`#` の後ろ2桁ずつが「赤・緑・青」の強さを表します。
- 色を直接 `#4f46e5` と書かず変数にしておくと、後で色を変えたいとき**1か所直すだけ**で全体に反映できます。

> **用語: カラートークン（color tokens）／デザイントークン** … 「primary-600 はこの色」のように、デザインで使う値に意味のある名前を付けて一元管理したもの。チームでデザインの一貫性を保つための定番手法。

---

##### 解説3: ダークモードで色を入れ替える（`.dark`）

```css
.dark {
  --color-background: #0f172a;        /* ほぼ黒（slate-900）に */
  --color-foreground: #f8fafc;        /* 文字は逆に白系に */
  --color-card: #1e293b;               /* カード背景は背景より少し明るい暗色 */
}
```

- `<html class="dark">` のように `dark` クラスが付いたときだけ、この中の**変数の値が上書き**されます。
- ポイントは「**新しいクラスを足すのではなく、同じ変数の中身だけを差し替える**」こと。背景の変数を黒に、文字の変数を白に入れ替えるだけで、画面全体がダークモードに変わります。
- 各コンポーネントで `dark:bg-...` を一つずつ書かなくて済むのが、CSS変数方式の最大の利点です。

> **用語: ダークモード（dark mode）** … 背景を黒・濃灰にした暗い配色モード。夜間や暗所での目の負担を減らす目的で用意する。

---

##### 解説4: ベースレイヤーで土台を整える（`@layer base`）

```css
@layer base {
  body {
    @apply bg-[var(--color-background)] text-[var(--color-foreground)];
    transition: background-color 0.3s ease, color 0.3s ease;
  }
}
```

- `@layer base`（レイヤー・ベース）は「**一番優先度の低い層**」にスタイルを置く指定です。低い層に置くと、後から個別の Tailwind クラスで簡単に上書きできます。
- `@apply`（アプライ）は「**CSSの中でTailwindクラスを使う**」ための命令です。`bg-[var(--color-background)]` は「背景色を解説2のCSS変数から取ってくる」という意味で、`[ ]`（角カッコ）はTailwindに無い任意の値を直接書く構文です。
- `transition: ... 0.3s ease` で、ダークモード切替時に背景と文字色が**0.3秒かけて滑らかに**変わるようにしています。

> **用語: レイヤー（layer）／@apply** … `@layer` はCSSの優先順位を整理するための「層」。`@apply` は長くなりがちなTailwindクラス指定をCSS側にまとめる書き方。

---

##### 解説5: ボタンの共通スタイルを作る（`@layer components` の `.btn`）

```css
@layer components {
  .btn {
    @apply inline-flex items-center justify-center
           rounded-lg px-4 py-2
           text-sm font-medium
           transition-all duration-200 ease-in-out
           disabled:pointer-events-none disabled:opacity-50;
  }
  .btn-primary {
    @apply btn
           bg-primary-600 text-white
           hover:bg-primary-700;
  }
}
```

- `.btn` は「全ボタン共通の土台」です。`inline-flex items-center justify-center` で中身を中央寄せ、`rounded-lg` で角丸、`px-4 py-2` で内側余白、`transition-all duration-200` で変化を200msかけて滑らかにしています。
- `disabled:opacity-50` は「ボタンが無効化されたときは半透明にする」指定で、`disabled:` は状態に応じてスタイルを変える接頭辞です。
- `.btn-primary` は `@apply btn ...` のように**`.btn` を継承**してから色（インディゴ背景＋白文字）だけを足しています。こうすると共通部分を何度も書かずに済みます。

> **用語: ホバー（hover）** … マウスカーソルを要素の上に乗せた状態。`hover:bg-primary-700` は「乗せたときだけ背景を濃くする」という指定。

---

##### 解説6: カードと入力欄の共通スタイル（`.card` / `.input`）

```css
.card {
  @apply rounded-xl border border-[var(--color-border)]
         bg-[var(--color-card)] text-[var(--color-card-foreground)]
         shadow-sm;
  transition: box-shadow 0.2s ease, transform 0.2s ease;
}
.input {
  @apply w-full rounded-lg border border-[var(--color-border)]
         px-4 py-2.5 text-sm
         focus:border-primary-500 focus:outline-none focus:ring-2;
}
```

- `.card` は書籍カードなどの土台です。角丸・枠線・背景・影を、すべてCSS変数経由で指定しているので**ダークモードでも自動で色が変わります**。
- `.input` は入力欄の共通スタイル。`w-full` で横幅いっぱい、`focus:ring-2` は「クリックして入力中のとき、枠の周りに2pxの光るリングを出す」指定で、`focus:` は入力欄が選択された状態を表します。
- このように共通スタイルを `@layer components` にまとめておくと、HTML側では `<div class="card">` と書くだけで統一された見た目になります。

> **用語: フォーカス（focus）** … 入力欄やボタンが「いま操作対象として選ばれている」状態。キーボード操作のユーザーにとって、どこが選択中かを示すリング表示が重要になる。

---

##### 解説7: 自作ユーティリティと省略表示（`@layer utilities` の `line-clamp`）

```css
@layer utilities {
  .line-clamp-2 {
    display: -webkit-box;
    -webkit-line-clamp: 2;            /* 2 行で打ち切る */
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
}
```

- `@layer utilities` は「**一番優先度の高い層**」で、Tailwindに無い便利クラスを自作する場所です。
- `.line-clamp-2` は「**2行を超えた文章を「…」で打ち切る**」クラスです。長いタイトルがカードからはみ出さないように使います。
- `-webkit-line-clamp` の数字を変えると打ち切る行数が変わります（`line-clamp-1` なら1行、`line-clamp-3` なら3行）。

> **用語: ユーティリティクラス（utility class）** … `p-4`（余白16px）のように「1つの機能だけを持つ小さなクラス」。これらを組み合わせてデザインするのがTailwindの考え方。

---

##### 解説8: 動きの素材を定義する（`@keyframes`）

```css
@keyframes fadeIn {
  from {
    opacity: 0;                       /* 透明（見えない） */
    transform: translateY(8px);       /* 8px 下にずらした位置から始まる */
  }
  to {
    opacity: 1;                       /* 不透明（見える） */
    transform: translateY(0);         /* 元の位置へ */
  }
}
```

- `@keyframes`（キーフレーム）は「**アニメーションの始まり（`from`）と終わり（`to`）のコマ**」を定義します。
- この `fadeIn` は「透明で8px下にいる状態」から「不透明で元の位置」へ変化します。つまり**少し下からふわっと現れる**動きです。
- `opacity`（透明度：0で透明、1で不透明）と `transform: translateY(...)`（縦方向の移動）を組み合わせるのがフェードイン演出の定番です。

> **用語: キーフレーム（keyframe）** … アニメーションの途中経過を指定する「コマ」。`from`/`to` のほか `0%`/`50%`/`100%` のようにパーセントで複数指定もできる。

---

### 1.2 カスタムカラーパレットの設定

アプリでは以下のカラーパレットを採用しています。「カラーパレット（color palette）」とは「アプリ全体で使う色の組み合わせ」のことです。色を決めて統一することで、デザインに一貫性が出ます。

| 用途 | カラー名 | ベースカラー | 説明 |
|------|----------|-------------|------|
| メインアクション | `primary` | インディゴ (#6366f1) | ボタン、リンク、フォーカスリング |
| 補助アクション | `secondary` | エメラルド (#10b981) | 成功状態、補助ボタン |
| 成功 | `success` | グリーン (#10b981) | 操作成功の通知 |
| 警告 | `warning` | アンバー (#f59e0b) | 注意喚起 |
| エラー | `error` | レッド (#ef4444) | エラーメッセージ、削除ボタン |
| 情報 | `info` | ブルー (#3b82f6) | 情報表示 |
| テキスト | `foreground` | スレート (#0f172a) | メインテキスト |
| 背景 | `background` | ホワイト (#ffffff) | ページ背景 |
| ミュート | `muted` | スレート (#64748b) | 補足テキスト |

カラーパレットの設計指針:

- **一貫性**: 同じ意味を持つ色は全画面で統一する
- **コントラスト比**: WCAG 2.1 AA 基準（4.5:1 以上）を満たす
- **ダークモード対応**: 各カラーに明暗の両バリエーションを用意する

### 1.3 tailwind.config.ts のカスタマイズ

> **▼ このコードがやること（先に日本語で）:** Tailwind CSS の動作を決める設定ファイルを作ります。「ダークモードの切り替え方法」「どのファイルからクラスを探すか」「自分で追加した色・影・アニメーション」などをここにまとめます。特に、先ほど作った CSS 変数を `var(--color-primary-500)` の形で Tailwind に登録することで、`bg-primary-500` のようなクラスが使えるようになります。設定項目ごとの意味はコメントを参照してください。

```typescript
// tailwind.config.ts
// ↑ Tailwind CSS のすべての設定をまとめたファイル。
//   Tailwind はビルド時にこのファイルを読み、どのクラスを生成するかを決める。

// "tailwindcss" パッケージから Config 型を「型としてのみ」インポート
// （import type なら実行時には消える → ビルドサイズを節約）
import type { Config } from "tailwindcss";

// config: Config と書くことで、TypeScript に「これは Tailwind の設定」と伝えられ
// 補完やタイプチェックが効くようになる
const config: Config = {
  // ダークモードの切り替え方法
  // "class"  : <html class="dark"> のように手動でクラスを付けて切り替える方式
  // "media"  : OSのカラーモード設定（prefers-color-scheme）に自動連動する方式
  // 本アプリでは「ユーザーが切り替えボタンで選べる」必要があるので "class" を選択
  darkMode: "class",

  // Tailwind がクラスを抽出する対象ファイルのパス
  // ここに書かれたファイル内で実際に使われているクラスだけを最終CSSに含めることで
  // バンドルサイズを最小化する（PurgeCSS / JIT エンジンの仕組み）
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",     // Pages Router 用（使っていなくてもOK）
    "./components/**/*.{js,ts,jsx,tsx,mdx}", // 自作コンポーネント
    "./app/**/*.{js,ts,jsx,tsx,mdx}",        // App Router 用
    // ※ ** はワイルドカード（任意の深さのフォルダ）、{ } は複数拡張子の指定
  ],

  // theme: Tailwind のデザインシステム本体。
  // ここに書いたものが Tailwind のクラスとして利用可能になる。
  theme: {
    // ========================================
    // コンテナの設定
    // ========================================
    // .container クラスの挙動をカスタマイズ。
    // <div class="container"> と書いたときの最大幅・余白・中央寄せの設定。
    container: {
      center: true,                 // コンテナを左右中央寄せ（margin: 0 auto 相当）
      padding: {                    // 画面サイズごとの左右パディング
        DEFAULT: "1rem",            // 全画面共通の最小余白 = 16px
        sm: "2rem",                 // 640px 以上 = 32px
        lg: "4rem",                 // 1024px 以上 = 64px
        xl: "5rem",                 // 1280px 以上 = 80px
        "2xl": "6rem",              // 1536px 以上 = 96px
      },
      screens: {                    // 各ブレークポイントでのコンテナ最大幅
        sm: "640px",
        md: "768px",
        lg: "1024px",
        xl: "1280px",
        "2xl": "1400px",            // デフォルトの1536pxより少し狭く（読みやすさのため）
      },
    },

    // extend: 既存のテーマを「上書き」せず「追加」する。
    // theme: { colors: {...} } と書くと標準色が全部消えるので、extend を使うのが基本。
    extend: {
      // ========================================
      // カスタムカラーパレット
      // ========================================
      colors: {
        // プライマリカラー（インディゴ系）
        // CSS変数（globals.css で定義した --color-primary-XX）を経由することで
        // ダークモード時に変数の値だけ差し替えれば自動で色が変わる
        primary: {
          50: "var(--color-primary-50)",   // bg-primary-50 のように使える
          100: "var(--color-primary-100)",
          200: "var(--color-primary-200)",
          300: "var(--color-primary-300)",
          400: "var(--color-primary-400)",
          500: "var(--color-primary-500)",
          600: "var(--color-primary-600)",
          700: "var(--color-primary-700)",
          800: "var(--color-primary-800)",
          900: "var(--color-primary-900)",
          950: "var(--color-primary-950)",
        },
        // セカンダリカラー（エメラルド系）
        secondary: {
          50: "var(--color-secondary-50)",
          100: "var(--color-secondary-100)",
          200: "var(--color-secondary-200)",
          300: "var(--color-secondary-300)",
          400: "var(--color-secondary-400)",
          500: "var(--color-secondary-500)",
          600: "var(--color-secondary-600)",
          700: "var(--color-secondary-700)",
          800: "var(--color-secondary-800)",
          900: "var(--color-secondary-900)",
          950: "var(--color-secondary-950)",
        },
        // セマンティックカラー（用途ベースの命名）
        background: "var(--color-background)",   // bg-background で使える
        foreground: "var(--color-foreground)",   // text-foreground で使える
        muted: {
          DEFAULT: "var(--color-muted)",          // bg-muted（DEFAULT は階層なしの指定で参照される）
          foreground: "var(--color-muted-foreground)", // text-muted-foreground
        },
        border: "var(--color-border)",
        card: {
          DEFAULT: "var(--color-card)",
          foreground: "var(--color-card-foreground)",
        },
        success: "var(--color-success)",
        warning: "var(--color-warning)",
        error: "var(--color-error)",
        info: "var(--color-info)",
      },

      // ========================================
      // カスタムフォント
      // ========================================
      // font-sans / font-mono クラスで使われるフォントの「フォールバック順」
      // 先頭から探して、利用可能な最初のフォントが使われる
      fontFamily: {
        sans: [
          "Inter",                  // 第一候補: 欧文用の美しいフォント
          "Noto Sans JP",           // 日本語フォント（Inter は日本語が含まれないので必須）
          "ui-sans-serif",          // OS標準のサンセリフ（macOS / Windows）
          "system-ui",              // システム標準
          "-apple-system",          // iOS / macOS の San Francisco
          "sans-serif",             // 最終フォールバック（必ず使える総称名）
        ],
        mono: [                     // 等幅フォント（コード表示用）
          "JetBrains Mono",         // プログラミング用の美しい等幅フォント
          "Fira Code",              // リガチャ（合字）対応の等幅フォント
          "ui-monospace",
          "monospace",
        ],
      },

      // ========================================
      // カスタムフォントサイズ
      // ========================================
      // text-2xs クラスを新規追加（標準には無い超小サイズ）
      // 値の形式: [フォントサイズ, { lineHeight: 行間 }]
      fontSize: {
        "2xs": ["0.625rem", { lineHeight: "0.875rem" }],  // 10px / 行間14px
      },

      // ========================================
      // カスタムシャドウ
      // ========================================
      boxShadow: {
        sm: "var(--shadow-sm)",     // shadow-sm が CSS 変数を参照するように
        md: "var(--shadow-md)",
        lg: "var(--shadow-lg)",
        // 独自の影を新規追加
        card: "0 2px 8px -2px rgb(0 0 0 / 0.08), 0 4px 12px -4px rgb(0 0 0 / 0.04)",  // カード用
        "card-hover":
          "0 8px 24px -4px rgb(0 0 0 / 0.12), 0 4px 8px -4px rgb(0 0 0 / 0.08)",       // ホバー時用
      },

      // ========================================
      // カスタムボーダー半径
      // ========================================
      // rounded-sm / rounded-md などの値をCSS変数経由に置き換え
      borderRadius: {
        sm: "var(--radius-sm)",
        md: "var(--radius-md)",
        lg: "var(--radius-lg)",
        xl: "var(--radius-xl)",
      },

      // ========================================
      // カスタムアニメーション
      // ========================================
      // keyframes: 動きの定義（CSS の @keyframes 相当）
      keyframes: {
        "fade-in": {
          from: { opacity: "0", transform: "translateY(8px)" },  // 透明＋8px下
          to: { opacity: "1", transform: "translateY(0)" },      // 不透明＋元位置
        },
        "fade-out": {
          from: { opacity: "1", transform: "translateY(0)" },
          to: { opacity: "0", transform: "translateY(8px)" },
        },
        "slide-in-right": {
          from: { opacity: "0", transform: "translateX(16px)" }, // 16px 右からスライド
          to: { opacity: "1", transform: "translateX(0)" },
        },
        "slide-in-up": {
          from: { opacity: "0", transform: "translateY(16px)" }, // 16px 下からスライド
          to: { opacity: "1", transform: "translateY(0)" },
        },
        "scale-in": {
          from: { opacity: "0", transform: "scale(0.95)" },      // 95% から
          to: { opacity: "1", transform: "scale(1)" },           // 100% へ拡大
        },
        shimmer: {
          "0%": { backgroundPosition: "-200% 0" },
          "100%": { backgroundPosition: "200% 0" },
        },
        "toast-slide-in": {
          from: { opacity: "0", transform: "translateX(100%)" },
          to: { opacity: "1", transform: "translateX(0)" },
        },
        "toast-slide-out": {
          from: { opacity: "1", transform: "translateX(0)" },
          to: { opacity: "0", transform: "translateX(100%)" },
        },
      },
      // animation: keyframes を「いつ・どんな速度で」再生するかを定義
      // 形式: "<keyframes名> <時間> <イージング> <繰り返し> <塗り潰し>"
      animation: {
        "fade-in": "fade-in 0.3s ease-out",                  // animate-fade-in クラスとして使える
        "fade-out": "fade-out 0.3s ease-out",
        "slide-in-right": "slide-in-right 0.3s ease-out",
        "slide-in-up": "slide-in-up 0.4s ease-out",
        "scale-in": "scale-in 0.2s ease-out",
        shimmer: "shimmer 2s infinite linear",               // infinite: 無限ループ、linear: 等速
        "toast-in": "toast-slide-in 0.3s ease-out",
        "toast-out": "toast-slide-out 0.3s ease-in forwards", // forwards: 終了状態をキープ
        spin: "spin 1s linear infinite",                      // animate-spin（標準）の上書き
      },

      // ========================================
      // カスタムスペーシング
      // ========================================
      // 標準にない余白/サイズの値を追加。w-18, h-88, p-128 のように使える
      spacing: {
        "18": "4.5rem",      // 72px
        "88": "22rem",       // 352px
        "128": "32rem",      // 512px
      },

      // ========================================
      // カスタムトランジション
      // ========================================
      // duration-250 / duration-350 を使えるようにする
      transitionDuration: {
        "250": "250ms",
        "350": "350ms",
      },
    },
  },

  // plugins: Tailwind の機能を拡張するプラグインを並べる配列
  // 例: @tailwindcss/forms、@tailwindcss/typography など
  // 今回は使わないので空配列
  plugins: [],
};

// ESM 形式でデフォルトエクスポート（Next.js が require できるようにする）
export default config;
```

**設定のポイント解説:**

- `darkMode: "class"` -- `<html class="dark">` でダークモードを切り替える方式。`media` にするとシステム設定に連動する
- `container` -- レスポンシブなコンテナ幅を画面サイズごとにカスタマイズ
- `colors` -- CSS カスタムプロパティ経由でカラーを定義することで、ダークモード切り替えが CSS 変数の上書きだけで済む
- `keyframes` / `animation` -- globals.css の `@keyframes` と合わせて、Tailwind のクラスとしてアニメーションを呼び出せるようにする

#### ▼ コードを1つずつ分解して解説

この設定ファイルは「Tailwind に対する取扱説明書」です。塊ごとに、どんな設定をしているかを見ていきましょう。

---

##### 解説1: ダークモードの方式を選ぶ（`darkMode`）

```typescript
darkMode: "class",
```

- `darkMode` は「ダークモードをどう切り替えるか」を決める設定です。
- `"class"` を選ぶと、`<html class="dark">` のように**クラスを手動で付け外し**して切り替えます。ユーザーが切替ボタンで選べるようにしたいので、この方式にしています。
- もう一つの `"media"` を選ぶと、OSの設定（ダーク/ライト）に自動で連動しますが、ユーザーが自分で選べなくなります。

> **用語: ダークモードの class 方式** … `<html>` に `dark` クラスが付いているかどうかでテーマを切り替える方式。JSからクラスを付け外しして制御する。

---

##### 解説2: クラスを探す対象ファイルを指定（`content`）

```typescript
content: [
  "./pages/**/*.{js,ts,jsx,tsx,mdx}",     // Pages Router 用（使っていなくてもOK）
  "./components/**/*.{js,ts,jsx,tsx,mdx}", // 自作コンポーネント
  "./app/**/*.{js,ts,jsx,tsx,mdx}",        // App Router 用
],
```

- `content` は「**どのファイルからTailwindクラスを探すか**」のリストです。
- Tailwind はここに書かれたファイルを読み、**実際に使われているクラスだけ**を最終的なCSSに含めます。使っていないクラスを除くことで、出力サイズを小さくできます。
- `**` は「どんな深さのフォルダでも」、`{js,ts,jsx,tsx,mdx}` は「これらの拡張子すべて」という意味の記号です。

> **用語: バンドルサイズ／パージ（purge）** … 最終的にユーザーへ配信されるCSS/JSの容量のこと。未使用クラスを削る（パージする）と軽くなり、表示が速くなる。

---

##### 解説3: CSS変数を Tailwind の色として登録（`extend.colors`）

```typescript
extend: {
  colors: {
    primary: {
      500: "var(--color-primary-500)",
      600: "var(--color-primary-600)",
      // ...
    },
  },
},
```

- `extend`（エクステンド＝拡張）は「標準のテーマを**消さずに追加する**」ための場所です。`extend` を使わずに書くと標準の色などが全部消えてしまうので注意します。
- `primary` の各段階を `var(--color-primary-500)` のように **globals.css のCSS変数に紐付け**ています。こうすると `bg-primary-500` のようなクラスが使えるようになります。
- さらに変数経由にしておくことで、ダークモード時は変数の値を入れ替えるだけで `bg-primary-500` の色も自動で変わります。

> **用語: extend** … Tailwind の設定で「標準設定を上書きせず、追加だけする」キーワード。色・余白・影などを安全に足せる。

---

##### 解説4: アニメーションをクラス化する（`keyframes` / `animation`）

```typescript
keyframes: {
  "fade-in": {
    from: { opacity: "0", transform: "translateY(8px)" },
    to: { opacity: "1", transform: "translateY(0)" },
  },
},
animation: {
  "fade-in": "fade-in 0.3s ease-out",
  shimmer: "shimmer 2s infinite linear",
},
```

- `keyframes` は「動きのコマ」、`animation` は「そのコマを**どんな速度・回数で再生するか**」をまとめた設定です。
- `animation` に登録すると、`animate-fade-in` や `animate-shimmer` のように**クラス名としてアニメーションを呼び出せる**ようになります。
- `"fade-in 0.3s ease-out"` は「fade-in を0.3秒、ease-out（最後ゆっくり）で再生」、`"shimmer 2s infinite linear"` は「2秒・無限ループ・等速」という意味です。

> **用語: イージング（easing）** … アニメーションの速度変化のカーブ。`ease-out` は登場向き（パッと出てフワッと止まる）、`linear` は等速、`infinite` は無限ループの指定。

---

## 2. レスポンシブデザイン

### 2.1 Tailwind のブレークポイント解説

Tailwind CSS はモバイルファースト設計です。何もプレフィックスを付けないスタイルがモバイル（最小画面）に適用され、`sm:` `md:` などのプレフィックスを付けるとそのブレークポイント**以上**の画面幅で適用されます。

| プレフィックス | 最小幅 | CSS 相当 | 想定デバイス | 用途 |
|:---:|:---:|:---|:---|:---|
| *(なし)* | 0px | `@media (min-width: 0px)` | 小型スマートフォン | デフォルト。全画面で適用 |
| `sm:` | 640px | `@media (min-width: 640px)` | 大型スマートフォン（横持ち） | テキストサイズ微調整、パディング拡大 |
| `md:` | 768px | `@media (min-width: 768px)` | タブレット | 2カラムレイアウト、サイドバー表示 |
| `lg:` | 1024px | `@media (min-width: 1024px)` | ノートPC | 3カラムレイアウト、ナビゲーション展開 |
| `xl:` | 1280px | `@media (min-width: 1280px)` | デスクトップ | 4カラムレイアウト、広いマージン |
| `2xl:` | 1536px | `@media (min-width: 1536px)` | 大型モニター | 最大幅レイアウト |

<div style="max-width:680px;margin:20px auto;font-family:'Segoe UI',sans-serif;display:flex;align-items:center;justify-content:center;gap:6px;flex-wrap:wrap;">
  <div style="background:#fef3c7;border:2px solid #f59e0b;border-radius:10px;padding:10px 14px;text-align:center;">
    <div style="font-weight:700;color:#92400e;font-size:13px;">0px</div>
    <div style="font-size:11px;color:#a16207;">モバイル</div>
    <div style="font-size:10px;color:#ca8a04;">(デフォルト)</div>
  </div>
  <div style="display:flex;flex-direction:column;align-items:center;">
    <div style="color:#f59e0b;font-size:18px;">→</div>
    <div style="font-size:10px;color:#64748b;">640px</div>
  </div>
  <div style="background:#dbeafe;border:2px solid #3b82f6;border-radius:10px;padding:10px 14px;text-align:center;">
    <div style="font-weight:700;color:#1e40af;font-size:13px;">sm:</div>
    <div style="font-size:11px;color:#3b82f6;">大型スマホ</div>
  </div>
  <div style="display:flex;flex-direction:column;align-items:center;">
    <div style="color:#3b82f6;font-size:18px;">→</div>
    <div style="font-size:10px;color:#64748b;">768px</div>
  </div>
  <div style="background:#d1fae5;border:2px solid #10b981;border-radius:10px;padding:10px 14px;text-align:center;">
    <div style="font-weight:700;color:#166534;font-size:13px;">md:</div>
    <div style="font-size:11px;color:#10b981;">タブレット</div>
  </div>
  <div style="display:flex;flex-direction:column;align-items:center;">
    <div style="color:#10b981;font-size:18px;">→</div>
    <div style="font-size:10px;color:#64748b;">1024px</div>
  </div>
  <div style="background:#e0e7ff;border:2px solid #6366f1;border-radius:10px;padding:10px 14px;text-align:center;">
    <div style="font-weight:700;color:#3730a3;font-size:13px;">lg:</div>
    <div style="font-size:11px;color:#6366f1;">ノートPC</div>
  </div>
  <div style="display:flex;flex-direction:column;align-items:center;">
    <div style="color:#6366f1;font-size:18px;">→</div>
    <div style="font-size:10px;color:#64748b;">1280px</div>
  </div>
  <div style="background:#fce7f3;border:2px solid #ec4899;border-radius:10px;padding:10px 14px;text-align:center;">
    <div style="font-weight:700;color:#9d174d;font-size:13px;">xl:</div>
    <div style="font-size:11px;color:#ec4899;">デスクトップ</div>
  </div>
  <div style="display:flex;flex-direction:column;align-items:center;">
    <div style="color:#ec4899;font-size:18px;">→</div>
    <div style="font-size:10px;color:#64748b;">1536px</div>
  </div>
  <div style="background:#f3e8ff;border:2px solid #a855f7;border-radius:10px;padding:10px 14px;text-align:center;">
    <div style="font-weight:700;color:#6b21a8;font-size:13px;">2xl:</div>
    <div style="font-size:11px;color:#a855f7;">大型モニター</div>
  </div>
</div>

**重要な考え方: モバイルファースト**

```
/* 悪い例: デスクトップから書いて、小さい画面を上書き */
/* Tailwind は「min-width」基準（その幅以上で適用）なので、 */
/* sm: より md: のほうが優先される。小さい画面向けに後から指定しても効かない。 */
className="grid-cols-4 md:grid-cols-2 sm:grid-cols-1"  ← 動かない

/* 良い例: モバイルから書いて、大きい画面を上書き */
/* デフォルト（接頭辞なし）= スマホ向け / md: = タブレット向け / lg: = PC向け の順 */
/* 画面が大きくなるほど右側の指定が優先される */
className="grid-cols-1 md:grid-cols-2 lg:grid-cols-4"  ← 正しい
```

> **モバイルファースト原則の理由：**
> 1. モバイルユーザーが多数派の時代であること。
> 2. CSSのカスケード（後優先）の仕組みと相性が良いこと。「最小画面 → 上書き」が自然な流れになる。
> 3. パフォーマンス的にも、モバイルでは多くのスタイルがスキップできて軽くなる。

### 2.2 書籍カードのグリッドをレスポンシブ対応

> **▼ このコードがやること（先に日本語で）:** 書籍カードを「画面サイズに応じて列数が変わる格子（グリッド）」に並べるコンポーネントを作ります。カギは Tailwind の「レスポンシブ接頭辞」で、`grid-cols-1`（スマホは1列）に `sm:grid-cols-2`（少し広い画面は2列）などを並べるだけで、面倒なメディアクエリを書かずに列数を切り替えられます。なお書籍が0冊のときは専用の空状態を表示する分岐も入っています。

```typescript
// ============================================================================
// ファイルパス: src/components/BookGrid.tsx
// 役割      : 書籍カードを「画面サイズに応じて列数が変わる格子状」に並べる
// ----------------------------------------------------------------------------
// Tailwind CSS のレスポンシブ接頭辞を使うことで、メディアクエリを書かずに
// 「PCでは4列、タブレットでは3列、スマホでは1列」のような切り替えができる。
// ============================================================================

"use client";

import Link from "next/link";
import { Book } from "@/types/book";
import { BookCard } from "./BookCard";
import { EmptyState } from "./EmptyState";

type BookGridProps = {
  books: Book[];   // 表示する書籍配列
};

export function BookGrid({ books }: BookGridProps) {
  // (1) 0冊なら専用の空状態コンポーネントを出して終わる（早期リターン）
  if (books.length === 0) {
    return <EmptyState />;
  }

  // (2) 1冊以上なら格子レイアウトで描画
  return (
    <div
      // ▼ Tailwind クラスの読み解き ▼
      //   grid           : display: grid（CSSグリッドレイアウト）
      //   grid-cols-1    : 列数 = 1（モバイル既定）
      //   gap-4          : マス目同士の隙間 16px
      //   sm:grid-cols-2 : 640px以上で 2 列に切替
      //   sm:gap-5       : 640px以上で 隙間 20px
      //   lg:grid-cols-3 : 1024px以上で 3 列
      //   lg:gap-6       : 1024px以上で 隙間 24px
      //   xl:grid-cols-4 : 1280px以上で 4 列
      // 「sm:」「lg:」などはメディアクエリを意味する接頭辞。
      className="
        grid
        grid-cols-1
        gap-4
        sm:grid-cols-2
        sm:gap-5
        lg:grid-cols-3
        lg:gap-6
        xl:grid-cols-4
      "
    >
      {/*
        (3) 配列を map で1件ずつ <Link> + <BookCard> に展開する
            ・key={book.id}     : Reactのリスト識別キー（必須）
            ・href={...}        : クリックで詳細ページへ
            ・className="group" : 子要素の "group-hover:..." を有効化する目印
            ・animationDelay    : i番目のカードを i*50ms 遅らせて順次フェードイン
      */}
      {books.map((book, index) => (
        <Link
          key={book.id}
          href={`/books/${book.id}`}
          className="group block"
          style={{
            animationDelay: `${index * 50}ms`,
          }}
        >
          <BookCard book={book} />
        </Link>
      ))}
    </div>
  );
}
```

#### ▼ コードを1つずつ分解して解説

---

##### 解説1: 0冊のときは早期リターン

```typescript
if (books.length === 0) {
  return <EmptyState />;
}
```

- `books.length === 0` は「書籍の配列が空（0冊）か」を判定しています。`.length` は配列の要素数です。
- 0冊なら、グリッドを描かずに `<EmptyState />`（「まだ書籍がありません」の案内）を返して**関数をここで終わらせます**。
- この「条件を満たしたら途中で `return` して抜ける」書き方を**早期リターン**と呼びます。先に例外ケースを片付けると、後の本処理がすっきりします。

> **用語: 早期リターン（early return）** … 条件を満たしたときに関数の途中で `return` して処理を打ち切る書き方。ネスト（入れ子）が浅くなり読みやすくなる。

---

##### 解説2: レスポンシブな格子レイアウト（grid 系クラス）

```typescript
className="
  grid
  grid-cols-1
  gap-4
  sm:grid-cols-2
  sm:gap-5
  lg:grid-cols-3
  lg:gap-6
  xl:grid-cols-4
"
```

- `grid` は「CSSグリッド（格子）レイアウトにする」指定です。`grid-cols-1` は列数を1にする、`gap-4` はマス目同士の隙間を16pxにする指定です。
- `sm:` `lg:` `xl:` は**レスポンシブ接頭辞**で、その画面幅以上のときだけ適用されます。`sm:grid-cols-2`（640px以上で2列）→ `lg:grid-cols-3`（1024px以上で3列）→ `xl:grid-cols-4`（1280px以上で4列）と段階的に増えます。
- 接頭辞なしの `grid-cols-1` が一番小さい画面（スマホ）向け。画面が広がるほど右側の指定で上書きされます（モバイルファースト）。

> **用語: レスポンシブ接頭辞（responsive prefix）** … `sm:` `md:` `lg:` のように、画面幅に応じてスタイルを切り替えるためにクラスの先頭に付ける目印。メディアクエリを手書きせずに済む。

---

##### 解説3: 各カードを順番にフェードインさせる（`animationDelay`）

```typescript
{books.map((book, index) => (
  <Link
    key={book.id}
    href={`/books/${book.id}`}
    className="group block"
    style={{
      animationDelay: `${index * 50}ms`,
    }}
  >
    <BookCard book={book} />
  </Link>
))}
```

- `books.map((book, index) => ...)` で配列の各書籍を `<Link>`＋`<BookCard>` に変換しています。`index` は0から始まる「何番目か」の番号です。
- `key={book.id}` はReactがリスト要素を見分けるための必須の目印。データ固有のIDを使うのが鉄則です。
- `className="group"` は「子要素の `group-hover:...` を効かせるための目印」。`animationDelay: ${index * 50}ms` は「i番目のカードを `i×50`ミリ秒遅らせて登場させる」指定で、カードが**ぱらぱらと順番に**現れる演出になります。

> **用語: animationDelay（アニメーション遅延）** … アニメーション開始までの待ち時間。各要素に少しずつ違う遅延を与えると、一斉ではなく順番に動く「スタッガード（staggered）」な演出になる。

---

> **▼ このコードがやること（先に日本語で）:** 書籍1冊分の見た目（タイトル・著者・状態バッジなど）を表示する「カード」コンポーネントを作ります。先ほど globals.css で用意した `.card` や `.badge` といった共通スタイルのクラスを当てて、見た目を整えるのがポイントです。このカードが先ほどのグリッドの中に1セルずつ並びます。各部分の役割はコメントで説明しています。

```typescript
// components/BookCard.tsx
// ↑ 書籍1冊分の見た目を表示するカードコンポーネント。
//   一覧画面 (BookGrid) の中で1セルずつ並ぶ部品。

// "use client" は Next.js App Router で「これはクライアントコンポーネント」と宣言するディレクティブ
// useState / useEffect / onClick などを使うなら必要
"use client";

// 書籍データの型を import（@/ はプロジェクトルートを指すパスエイリアス）
import { Book } from "@/types/book";

// このコンポーネントが受け取る props（外から渡される値）の型を定義
type BookCardProps = {
  book: Book;     // Book 型の書籍データを 1 件
};

// export function: 名前付きエクスポート（他ファイルから { BookCard } で取り出す）
// 引数の { book } は分割代入：props の book プロパティを直接取り出している
export function BookCard({ book }: BookCardProps) {
  // 読了状況（status）に応じてバッジのスタイルを切り替えるヘルパー関数
  // ※ 関数のなかでJSXを return して、 {statusBadge()} で呼び出す
  const statusBadge = () => {
    switch (book.status) {
      case "reading":
        // badge-primary はインディゴ色のバッジ（globals.css で定義）
        return <span className="badge-primary">読書中</span>;
      case "completed":
        // badge-success は緑色のバッジ
        return <span className="badge-success">読了</span>;
      case "want_to_read":
        // badge-warning は黄色のバッジ
        return <span className="badge-warning">読みたい</span>;
      default:
        return null;   // 該当ステータスがなければ何も表示しない
    }
  };

  return (
    // <article> は「独立した記事的なコンテンツ」を表すHTML5 タグ
    // 書籍1冊分の情報のかたまりとして意味的に正しい
    <article
      className="
        card-hover
        animate-fade-in
        flex flex-col
        overflow-hidden
        p-0
      "
      /* ▼ クラスの読み解き ▼
         card-hover     : .card（カードのベース）にホバー時の浮き上がりを追加
         animate-fade-in: 表示時に下からフェードインするアニメーション
         flex flex-col  : flex レイアウトで子要素を縦に並べる
         overflow-hidden: はみ出た要素を非表示（画像のはみ出し対策）
         p-0            : padding を 0 に（card のデフォルトpadding解除）
      */
    >
      {/* 書籍のサムネイルエリア */}
      <div
        className="
          relative
          aspect-[3/4]
          w-full
          overflow-hidden
          bg-gradient-to-br from-primary-100 to-primary-200
          dark:from-primary-900 dark:to-primary-800
        "
        /* ▼ クラスの読み解き ▼
           relative      : position: relative。子要素の absolute 配置の基準にする
           aspect-[3/4]  : 縦横比 3:4（本の縦長プロポーション）。任意値構文 [ ]
           w-full        : 幅100%
           overflow-hidden: はみ出し非表示（hover で画像がズームしてもカードからはみ出ない）
           bg-gradient-to-br: 背景を「左上 → 右下」のグラデーションに（br = bottom-right）
           from-primary-100 to-primary-200 : 開始色と終了色
           dark:from-primary-900 dark:to-primary-800: ダークモード時のグラデーション
        */
      >
        {/* book.thumbnailUrl があれば画像を表示、なければプレースホルダーを表示する分岐 */}
        {/* JSX 内の {} は JavaScript 式を埋め込む構文。三項演算子 (条件 ? A : B) で出し分け */}
        {book.thumbnailUrl ? (
          <img
            src={book.thumbnailUrl}                       // 画像URL
            alt={`「${book.title}」の表紙`}                // 代替テキスト（読み上げ・画像非表示時用）
            className="
              h-full w-full
              object-cover
              transition-transform duration-300
              group-hover:scale-105
            "
            /* ▼ クラスの読み解き ▼
               h-full w-full          : 親要素いっぱいに広げる
               object-cover           : 縦横比を保ちながら親要素を完全に覆う（はみ出る部分はカット）
               transition-transform   : transform プロパティに transition を適用
               duration-300           : 300ms かけて変化
               group-hover:scale-105  : 親要素のうち className="group" を持つもののホバー時に
                                        この要素を 105% に拡大。BookGrid 側の <Link> に group を付けている
            */
            loading="lazy"                                 // 画面外の画像は遅延読み込み（パフォーマンス向上）
          />
        ) : (
          /* プレースホルダー（画像がない場合の代替表示） */
          <div
            className="
              flex h-full w-full
              flex-col items-center justify-center
              gap-2
              p-4
              text-primary-400
              dark:text-primary-600
            "
            /* ▼ クラスの読み解き ▼
               flex h-full w-full           : flex レイアウトで親全体に広げる
               flex-col                      : 縦並び
               items-center justify-center   : 縦横とも中央寄せ
               gap-2                         : 子要素間に 0.5rem (8px) の隙間
               p-4                           : 内側余白 1rem (16px)
               text-primary-400              : アイコン色（薄めのインディゴ）
               dark:text-primary-600         : ダーク時の色
            */
          >
            {/* SVG（Scalable Vector Graphics：拡大しても綺麗な画像形式）でアイコンを描く */}
            <svg
              className="h-12 w-12"           // 48x48 サイズ
              fill="none"                      // 塗りつぶしなし
              viewBox="0 0 24 24"              // SVG の内部座標系（24x24 のキャンバス）
              stroke="currentColor"            // 線の色は親要素の文字色（text-primary-400）を継承
              strokeWidth={1.5}                // 線の太さ
              aria-hidden="true"               // スクリーンリーダーに無視させる（装飾アイコンのため）
            >
              <path
                strokeLinecap="round"          // 線の端を丸く
                strokeLinejoin="round"         // 線の折れ角を丸く
                d="M12 6.042A8.967 8.967 0 006 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 016 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 016-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0018 18a8.967 8.967 0 00-6 2.292m0-14.25v14.25"
                /* ↑ d 属性は SVG のパス（描画命令）。M=移動、A=円弧、C=曲線など */
              />
            </svg>
            <span className="text-xs font-medium">No Image</span>
            {/*  text-xs: 12px / font-medium: 500 の太さ */}
          </div>
        )}

        {/* ステータスバッジ（カード右上に絶対配置） */}
        <div className="absolute right-2 top-2">
          {/* absolute        : position: absolute。親 .relative を基準に配置
              right-2 top-2  : 右から 0.5rem、上から 0.5rem の位置 */}
          {statusBadge()}
        </div>
      </div>

      {/* 書籍情報エリア */}
      <div className="flex flex-1 flex-col gap-1.5 p-4">
        {/* ▼ クラスの読み解き ▼
            flex flex-1 : flex の子で、残り領域を全部使う（伸縮自在）
            flex-col    : 子要素を縦並びに
            gap-1.5     : 子要素間に 0.375rem (6px) の隙間
            p-4         : 内側余白 1rem (16px) */}

        {/* タイトル */}
        <h3
          className="
            line-clamp-2
            text-sm font-semibold
            leading-tight
            text-[var(--color-foreground)]
            transition-colors duration-200
            group-hover:text-primary-600
            dark:group-hover:text-primary-400
            sm:text-base
          "
          /* ▼ クラスの読み解き ▼
             line-clamp-2                    : 2 行を超えたら省略「…」（globals.css で定義）
             text-sm font-semibold           : 14px の太字
             leading-tight                   : 行間を狭め（line-height: 1.25）
             text-[var(--color-foreground)]  : 文字色をCSS変数で指定（ダーク対応のため）
             transition-colors duration-200  : 色変化を 200ms かけて滑らかに
             group-hover:text-primary-600    : 親（.group）ホバー時にインディゴへ変色
             dark:group-hover:text-primary-400: ダーク × ホバー時の色
             sm:text-base                    : 640px 以上で 16px に拡大
          */
        >
          {book.title}
        </h3>

        {/* 著者 */}
        <p
          className="
            line-clamp-1
            text-xs text-[var(--color-muted)]
            sm:text-sm
          "
          /* line-clamp-1: 1行で省略
             text-xs (12px) → sm:text-sm (14px に拡大)
             text-[var(--color-muted)]: 補足テキスト用の薄めの色 */
        >
          {book.author}
        </p>

        {/* 評価（星表示） */}
        {/* 条件付きレンダリング: rating が定義済みで 0 より大きい時のみ描画 */}
        {/* && は短絡評価。左が true なら右の JSX を返す、false なら何も返さない */}
        {book.rating !== undefined && book.rating > 0 && (
          <div className="mt-auto flex items-center gap-1 pt-2">
            {/* mt-auto: 上方向の margin を auto に → flex 子要素を一番下に押し下げる
                flex items-center: 横並びで縦中央
                gap-1: 子要素間 0.25rem (4px)
                pt-2: 上パディング 0.5rem (8px) */}

            {/* [1,2,3,4,5] の配列を map で星アイコン5つにループ展開 */}
            {[1, 2, 3, 4, 5].map((star) => (
              <svg
                key={star}                                    // React リストの一意キー（必須）
                /* テンプレートリテラル ` ` の中で ${} を使い動的にクラスを切替 */
                /* star が現在の評価以下なら金色（塗る）、それより大きいなら灰色（空き） */
                className={`h-3.5 w-3.5 ${
                  star <= book.rating!                         // ! は「null/undefined でない」と保証する非 null アサーション
                    ? "text-yellow-400"
                    : "text-gray-300 dark:text-gray-600"
                }`}
                fill="currentColor"                            // 塗り色＝親要素の文字色
                viewBox="0 0 20 20"
                aria-hidden="true"                             // スクリーンリーダーから隠す（後ろの数値で代替）
              >
                {/* 星型のパス */}
                <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
              </svg>
            ))}
            {/* 評価の数値（4.0 など）も併記 */}
            <span className="ml-1 text-xs text-[var(--color-muted)]">
              {/* ml-1: 左マージン 0.25rem (4px) */}
              {book.rating}
            </span>
          </div>
        )}
      </div>
    </article>
  );
}
```

#### ▼ コードを1つずつ分解して解説

---

##### 解説1: ステータスに応じてバッジを出し分ける（`statusBadge`）

```typescript
const statusBadge = () => {
  switch (book.status) {
    case "reading":
      return <span className="badge-primary">読書中</span>;
    case "completed":
      return <span className="badge-success">読了</span>;
    case "want_to_read":
      return <span className="badge-warning">読みたい</span>;
    default:
      return null;
  }
};
```

- `statusBadge` は「**JSXを返す小さな関数**」です。`{statusBadge()}` のように呼び出すと、状態に合ったバッジが描画されます。
- `switch (book.status)` で読了状況を分岐し、`badge-primary`（青）・`badge-success`（緑）・`badge-warning`（黄）という globals.css 定義のクラスを当て分けています。
- `default: return null` は「どれにも当てはまらなければ何も表示しない」という意味。Reactでは `null` を返すと何も描画されません。

> **用語: switch 文** … 1つの値を複数の `case` と照合して分岐する制御構文。状態（status）ごとの出し分けに向く。

---

##### 解説2: 画像があるか無いかで表示を切り替える（三項演算子）

```typescript
{book.thumbnailUrl ? (
  <img
    src={book.thumbnailUrl}
    alt={`「${book.title}」の表紙`}
    className="h-full w-full object-cover ... group-hover:scale-105"
    loading="lazy"
  />
) : (
  /* プレースホルダー（画像がない場合の代替表示） */
  <div className="flex h-full w-full ...">
    {/* No Image アイコン */}
  </div>
)}
```

- `book.thumbnailUrl ? (画像) : (代替表示)` は**三項演算子**で、「URLがあれば画像を、なければ『No Image』の代替を表示」と切り替えています。
- `object-cover` は「縦横比を保ったまま枠を覆い、はみ出る部分はカット」する指定。`group-hover:scale-105` は親（`.group`）にマウスを乗せたとき画像を105%に拡大します。
- `loading="lazy"` は「画面外の画像は後回しで読み込む」指定で、ページ表示を軽くします。`alt` は画像が出ない時や読み上げソフト用の代替文です。

> **用語: 三項演算子（`条件 ? A : B`）／object-cover** … 三項演算子は条件で値を出し分ける式。`object-cover` は画像を切り抜いて枠いっぱいに表示するCSS。

---

##### 解説3: 評価の星を5つ描く（配列の map）

```typescript
{book.rating !== undefined && book.rating > 0 && (
  <div className="mt-auto flex items-center gap-1 pt-2">
    {[1, 2, 3, 4, 5].map((star) => (
      <svg
        key={star}
        className={`h-3.5 w-3.5 ${
          star <= book.rating!
            ? "text-yellow-400"
            : "text-gray-300 dark:text-gray-600"
        }`}
        // ...
      >
        <path d="..." />
      </svg>
    ))}
  </div>
)}
```

- 先頭の `book.rating !== undefined && book.rating > 0 &&` は「評価が設定されていて0より大きいときだけ」星を描く条件です（`&&` は左が成立したときだけ右を実行）。
- `[1, 2, 3, 4, 5].map((star) => ...)` で**星アイコンを5個**作ります。`star` は1〜5の番号です。
- ``className={`... ${star <= book.rating! ? "text-yellow-400" : "text-gray-300 ..."}`}`` で、「その星の番号が評価以下なら金色、超えていれば灰色」と色を切り替えます。`book.rating!` の `!` は「ここでは null/undefined ではない」とTypeScriptに保証する記号です。

> **用語: 非nullアサーション（`!`）／mt-auto** … 値の後ろの `!` は「絶対にnull/undefinedでない」とコンパイラに伝える印。`mt-auto` は上の余白を自動で最大化し、要素を下端に押し下げるTailwindクラス。

---

### 2.3 モバイル/タブレット/デスクトップでの表示の違い

各画面サイズでのグリッドレイアウトの変化を以下にまとめます。

**モバイル（~639px）: `grid-cols-1`**

<div style="max-width: 300px; margin: 16px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #e2e8f0; border-radius: 10px; overflow: hidden; box-shadow: 0 2px 12px rgba(0,0,0,0.06);">
  <div style="background: #3b82f6; color: white; padding: 8px 14px; font-size: 11px; font-weight: 600;">📱 モバイル (&lt; 640px) — 1列</div>
  <div style="padding: 10px;">
    <div style="border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; margin-bottom: 8px; background: white; font-size: 12px;"><strong>📗 Book 1</strong><br/><span style="color:#64748b">著者名</span></div>
    <div style="border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; margin-bottom: 8px; background: white; font-size: 12px;"><strong>📘 Book 2</strong><br/><span style="color:#64748b">著者名</span></div>
    <div style="border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; margin-bottom: 8px; background: white; font-size: 12px;"><strong>📙 Book 3</strong><br/><span style="color:#64748b">著者名</span></div>
    <div style="border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; background: white; font-size: 12px;"><strong>📕 Book 4</strong><br/><span style="color:#64748b">著者名</span></div>
  </div>
</div>

- カードが1列で縦に並ぶ
- タイトルは `text-sm`（小さめ）
- パディングは最小限
- タッチ操作しやすい大きなタップ領域

**タブレット（640px~1023px）: `sm:grid-cols-2`**

<div style="max-width: 480px; margin: 16px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #e2e8f0; border-radius: 10px; overflow: hidden; box-shadow: 0 2px 12px rgba(0,0,0,0.06);">
  <div style="background: #3b82f6; color: white; padding: 8px 14px; font-size: 11px; font-weight: 600;">📱 タブレット (640px–1023px) — 2列</div>
  <div style="padding: 10px; display: flex; gap: 8px; flex-wrap: wrap;">
    <div style="flex: 1; min-width: 45%; border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; background: white; font-size: 12px;"><strong>📗 Book 1</strong><br/><span style="color:#64748b">著者名</span></div>
    <div style="flex: 1; min-width: 45%; border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; background: white; font-size: 12px;"><strong>📘 Book 2</strong><br/><span style="color:#64748b">著者名</span></div>
    <div style="flex: 1; min-width: 45%; border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; background: white; font-size: 12px;"><strong>📙 Book 3</strong><br/><span style="color:#64748b">著者名</span></div>
    <div style="flex: 1; min-width: 45%; border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; background: white; font-size: 12px;"><strong>📕 Book 4</strong><br/><span style="color:#64748b">著者名</span></div>
  </div>
</div>

- 2列のグリッド
- タイトルは `sm:text-base`（通常サイズ）
- ギャップが少し広がる（`sm:gap-5`）

**デスクトップ（1024px~1279px）: `lg:grid-cols-3`**

<div style="max-width: 620px; margin: 16px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #e2e8f0; border-radius: 10px; overflow: hidden; box-shadow: 0 2px 12px rgba(0,0,0,0.06);">
  <div style="background: #1e40af; color: white; padding: 8px 14px; font-size: 11px; font-weight: 600;">🖥️ デスクトップ (1024px–1279px) — 3列</div>
  <div style="padding: 10px; display: flex; gap: 8px; flex-wrap: wrap;">
    <div style="flex: 1; min-width: 30%; border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; background: white; font-size: 12px;"><strong>📗 Book 1</strong><br/><span style="color:#64748b">著者名</span></div>
    <div style="flex: 1; min-width: 30%; border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; background: white; font-size: 12px;"><strong>📘 Book 2</strong><br/><span style="color:#64748b">著者名</span></div>
    <div style="flex: 1; min-width: 30%; border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; background: white; font-size: 12px;"><strong>📙 Book 3</strong><br/><span style="color:#64748b">著者名</span></div>
    <div style="flex: 1; min-width: 30%; border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; background: white; font-size: 12px;"><strong>📕 Book 4</strong><br/><span style="color:#64748b">著者名</span></div>
    <div style="flex: 1; min-width: 30%; border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; background: white; font-size: 12px;"><strong>📓 Book 5</strong><br/><span style="color:#64748b">著者名</span></div>
    <div style="flex: 1; min-width: 30%; border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; background: white; font-size: 12px;"><strong>📔 Book 6</strong><br/><span style="color:#64748b">著者名</span></div>
  </div>
</div>

- 3列のグリッド
- ギャップがさらに広がる（`lg:gap-6`）

**大型デスクトップ（1280px~）: `xl:grid-cols-4`**

<div style="max-width: 760px; margin: 16px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #e2e8f0; border-radius: 10px; overflow: hidden; box-shadow: 0 2px 12px rgba(0,0,0,0.06);">
  <div style="background: #1e40af; color: white; padding: 8px 14px; font-size: 11px; font-weight: 600;">🖥️ 大型デスクトップ (1280px〜) — 4列</div>
  <div style="padding: 10px; display: flex; gap: 8px; flex-wrap: wrap;">
    <div style="flex: 1; min-width: 22%; border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; background: white; font-size: 12px;"><strong>📗 Book 1</strong><br/><span style="color:#64748b">著者名</span></div>
    <div style="flex: 1; min-width: 22%; border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; background: white; font-size: 12px;"><strong>📘 Book 2</strong><br/><span style="color:#64748b">著者名</span></div>
    <div style="flex: 1; min-width: 22%; border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; background: white; font-size: 12px;"><strong>📙 Book 3</strong><br/><span style="color:#64748b">著者名</span></div>
    <div style="flex: 1; min-width: 22%; border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px; background: white; font-size: 12px;"><strong>📕 Book 4</strong><br/><span style="color:#64748b">著者名</span></div>
  </div>
</div>

- 4列のグリッド
- 情報の一覧性が最大化

---

## 3. UI コンポーネントの改善

### 3.1 トースト通知の実装

操作結果（成功/エラー）をユーザーに伝えるトースト通知コンポーネントを作成します。

React の **Context API（コンテキスト・エーピーアイ：親から孫・ひ孫…と深い階層に値を渡すための仕組み）** を使い、アプリのどこからでもトーストを呼び出せるようにします。

> **Context API のイメージ：** 通常 React は props で値を「親 → 子」と一段ずつ渡しますが、深い階層では「props のバケツリレー」が辛くなります。Context はその回避策で、「アプリのある範囲に値を放送して、その範囲内の誰でも `useContext` で受信できる」仕組みです。

#### 3.1.1 トーストの型定義

> **▼ このコードがやること（先に日本語で）:** トースト通知（画面の隅にひょいと出る短いメッセージ）で扱うデータの「型」を先に定義します。トーストは「成功・エラー・警告・情報」の4種類だけに限定し、それ以外の文字列を間違って書けないようにします（タイポ防止）。型を別ファイルに切り出しておくと、このあと作る通知の仕組みとボタンの間でデータの形を共有しやすくなります。

```typescript
// types/toast.ts
// ↑ トースト通知に使う型をまとめて定義するファイル。
//   型を別ファイルに切り出すと、Context・コンポーネント間で共有しやすい。

// ToastType: トーストの「種類」を表す文字列リテラル型のユニオン（| で連結した複数候補）
// この4つ以外の文字列は代入できなくなる（タイポ防止）
export type ToastType = "success" | "error" | "warning" | "info";

// Toast: 1件のトーストデータを表す型（オブジェクト型）
export type Toast = {
  id: string;          // 一意な識別子（削除時に使う）
  type: ToastType;     // 上で定義した4種のどれか
  title: string;       // 通知のタイトル（必須）
  message?: string;    // 補足メッセージ。? は「省略可能（undefined を許容）」の意味
  duration?: number;   // 表示時間（ミリ秒）。省略時はデフォルト 5000ms = 5秒
};
```

#### 3.1.2 トースト Context

> **▼ このコードがやること（先に日本語で）:** アプリのどこからでもトースト通知を出せるようにする「中央の仕組み」を作ります。React の Context（コンテキスト）という機能を使うと、深い階層のコンポーネントにもデータや関数を「バケツリレー」せずに直接届けられます。ここでは「トーストを追加・削除する関数」を Context に入れ、`useToast()` という短い呼び出しで使えるようにするのが狙いです。

```typescript
// ============================================================================
// ファイルパス: src/contexts/ToastContext.tsx
// 役割      : アプリ全体で「トースト通知（一時的なメッセージ）」を扱う仕組み
// ----------------------------------------------------------------------------
// React の Context API を使うと、深い階層のコンポーネントにも
// プロパティをバケツリレーせずに値や関数を共有できる。
// ここでは「トーストを追加・削除する関数」を Context にして、
// どのコンポーネントからでも showToast(...) のように呼べるようにする。
// ============================================================================

// "use client" が必要な理由:
//   useState / useContext / setTimeout を使う＝ブラウザ側で動く必要がある。
"use client";

// React の Hook と型を取り込む
//   createContext: Context オブジェクトを作る関数
//   useContext   : Context の値を読む Hook
//   useState     : 状態管理 Hook
//   useCallback  : 関数を「依存変更時のみ再生成」するメモ化 Hook
//   ReactNode    : 「Reactで子要素として使えるあらゆる値」の型
import {
  createContext,
  useContext,
  useState,
  useCallback,
  type ReactNode,
} from "react";
import { Toast, ToastType } from "@/types/toast";

// ----------------------------------------------------------------------------
// (1) Context が提供する「価値」の型を定義
// ----------------------------------------------------------------------------
// この型に書かれた値・関数を、Context.Provider 経由で配布する。
type ToastContextType = {
  toasts: Toast[];                                                   // 現在表示中のトースト配列
  addToast: (                                                        // 任意のトーストを追加
    type: ToastType,
    title: string,
    message?: string,
    duration?: number
  ) => void;
  removeToast: (id: string) => void;                                 // 指定IDを削除
  success: (title: string, message?: string) => void;                // ↓ よく使う4種のショートカット
  error: (title: string, message?: string) => void;
  warning: (title: string, message?: string) => void;
  info: (title: string, message?: string) => void;
};

// ----------------------------------------------------------------------------
// (2) Context オブジェクトを作る
// ----------------------------------------------------------------------------
// 初期値は undefined（後で Provider が値を提供）。
// 取り出すときに undefined チェックを入れて安全にする（後述の useToast 参照）。
const ToastContext = createContext<ToastContextType | undefined>(undefined);

// ----------------------------------------------------------------------------
// (3) 一意なIDを作る簡易関数
// ----------------------------------------------------------------------------
// Date.now() (ミリ秒) と単調増加カウンタを組み合わせて衝突しないIDを作る。
// 大規模アプリでは uuid ライブラリを使うほうが安心。
let toastIdCounter = 0;
function generateToastId(): string {
  toastIdCounter += 1;
  return `toast-${Date.now()}-${toastIdCounter}`;
}

// ----------------------------------------------------------------------------
// (4) Provider コンポーネント
// ----------------------------------------------------------------------------
// アプリのルート付近で <ToastProvider> でラップすると、その内側の全ての
// コンポーネントが useToast() で値を受け取れる。
export function ToastProvider({ children }: { children: ReactNode }) {
  // (4-1) 表示中のトースト配列を state で保持
  const [toasts, setToasts] = useState<Toast[]>([]);

  // (4-2) 指定IDのトーストを削除する関数
  //       useCallback で関数の同一性を保つことで、依存配列に入れた箇所で
  //       不必要な再レンダリングが起きるのを防ぐ。
  const removeToast = useCallback((id: string) => {
    // 配列を「削除対象以外を残す」フィルタで更新
    setToasts((prev) => prev.filter((toast) => toast.id !== id));
  }, []);

  // (4-3) トーストを追加する関数
  //       duration 経過後に setTimeout で自動で削除する。
  const addToast = useCallback(
    (
      type: ToastType,
      title: string,
      message?: string,
      duration: number = 5000   // 既定で5秒で消える
    ) => {
      const id = generateToastId();
      const newToast: Toast = { id, type, title, message, duration };

      // 既存の配列に新トーストを末尾追加（破壊しないよう新配列を作る）
      setToasts((prev) => [...prev, newToast]);

      // duration が 0 より大きいときだけ自動削除のタイマーをセット
      if (duration > 0) {
        setTimeout(() => {
          removeToast(id);
        }, duration);
      }
    },
    [removeToast]   // removeToast が変わったら再生成
  );

  // ショートカットメソッド
  const success = useCallback(
    (title: string, message?: string) => addToast("success", title, message),
    [addToast]
  );

  const error = useCallback(
    (title: string, message?: string) =>
      addToast("error", title, message, 8000), // エラーは長めに表示
    [addToast]
  );

  const warning = useCallback(
    (title: string, message?: string) => addToast("warning", title, message),
    [addToast]
  );

  const info = useCallback(
    (title: string, message?: string) => addToast("info", title, message),
    [addToast]
  );

  return (
    <ToastContext.Provider
      value={{ toasts, addToast, removeToast, success, error, warning, info }}
    >
      {children}
    </ToastContext.Provider>
  );
}

// ----------------------------------------------------------------------------
// (5) Context から値を取り出すカスタム Hook
// ----------------------------------------------------------------------------
// この関数を使えば、コンポーネントから `const toast = useToast()` のように
// 呼び出すだけで Context の値を受け取れる。
// 関数名は use で始める必要がある（React の Hook ルール）。
export function useToast(): ToastContextType {
  // useContext で Context オブジェクトから値を取得
  const context = useContext(ToastContext);
  // Provider 内で使われなかった場合は、エラーで気付かせる
  // （取り出した値が undefined のままだと型エラーの温床になる）
  if (context === undefined) {
    throw new Error("useToast must be used within a ToastProvider");
  }
  return context;
}
```

#### ▼ コードを1つずつ分解して解説

---

##### 解説1: Context の「形」を型で決める（`ToastContextType`）

```typescript
type ToastContextType = {
  toasts: Toast[];
  addToast: (type: ToastType, title: string, message?: string, duration?: number) => void;
  removeToast: (id: string) => void;
  success: (title: string, message?: string) => void;
  // error / warning / info も同様
};
```

- これは「Context を通じて配る中身の一覧」を表す型です。`toasts`（表示中の配列）と、追加・削除・各種ショートカット関数が含まれます。
- `addToast: (...) => void` のように書くと「**こういう引数を取り、戻り値の無い関数**」という型になります。`message?` の `?` は省略可能の意味です。
- 先に型を決めておくと、使う側で `toast.success(...)` のように呼ぶとき、VS Codeが補完や間違いチェックをしてくれます。

> **用語: Context（コンテキスト）** … propsのバケツリレーをせずに、ある範囲のどのコンポーネントへも直接値を届けるReactの仕組み。

---

##### 解説2: Context オブジェクトを作る（`createContext`）

```typescript
const ToastContext = createContext<ToastContextType | undefined>(undefined);
```

- `createContext` は「値を配るための入れ物（Context）」を作る関数です。
- 初期値を `undefined` にしているのは、「Providerで包まずに使ったら気づけるようにする」ためです（後述の `useToast` で `undefined` を検出してエラーを出します）。
- 型 `ToastContextType | undefined` は「中身は ToastContextType か、まだ無い（undefined）かのどちらか」という意味です。

> **用語: createContext** … Contextの入れ物を生成するReact関数。`.Provider` で値を流し込み、`useContext` で受け取る。

---

##### 解説3: トーストを追加して自動で消す（`addToast`）

```typescript
const addToast = useCallback(
  (type, title, message, duration = 5000) => {
    const id = generateToastId();
    const newToast: Toast = { id, type, title, message, duration };
    setToasts((prev) => [...prev, newToast]);
    if (duration > 0) {
      setTimeout(() => { removeToast(id); }, duration);
    }
  },
  [removeToast]
);
```

- `duration = 5000` は「指定が無ければ5000ミリ秒（5秒）」というデフォルト値です。
- `setToasts((prev) => [...prev, newToast])` は「**今ある配列をコピーして末尾に新トーストを足した新しい配列**」を作ってセットします。`...prev` で元を壊さずに追加するのがReactのお作法です。
- `setTimeout(() => removeToast(id), duration)` で、`duration` 経過後に**自動でそのトーストを削除**します。`useCallback` で包むのは、関数を毎回作り直さず使い回す（メモ化）ためです。

> **用語: useCallback（メモ化）／setTimeout** … `useCallback` は依存が変わらない限り同じ関数を保つReact Hook。`setTimeout` は指定ミリ秒後に1回だけ処理を実行するブラウザ関数。

---

##### 解説4: Provider で値を配る（`ToastContext.Provider`）

```typescript
return (
  <ToastContext.Provider
    value={{ toasts, addToast, removeToast, success, error, warning, info }}
  >
    {children}
  </ToastContext.Provider>
);
```

- `<ToastContext.Provider value={...}>` で囲むと、その**内側にある全コンポーネント**が `value` の中身を受け取れます。
- `value` には解説1の型に対応する一式（配列と関数群）を渡しています。
- `{children}` は「このProviderで包まれた中身」を表し、アプリ全体をここに入れることで、どこからでもトーストを呼べるようになります。

> **用語: Provider（プロバイダー）** … Contextの値を「ここから下の範囲に配る」役割のコンポーネント。`value` に配りたいデータを渡す。

---

##### 解説5: 安全に値を取り出すカスタムフック（`useToast`）

```typescript
export function useToast(): ToastContextType {
  const context = useContext(ToastContext);
  if (context === undefined) {
    throw new Error("useToast must be used within a ToastProvider");
  }
  return context;
}
```

- `useContext(ToastContext)` で Context の現在値を取り出します。
- `if (context === undefined)` は「Providerの外で誤って使った」状況の検出です。その場合 `throw new Error(...)` で**分かりやすいエラーを出して**早期に気づかせます。
- 関数名を `use` で始めるのはReactの「Hookは use で始める」というルールに従うためです。これで使う側は `const toast = useToast()` の1行で済みます。

> **用語: カスタムフック（custom hook）** … `use` で始まる自作の関数で、Hookのロジックを再利用しやすくまとめたもの。`useContext` などの組み込みHookを内部で使える。

---

#### 3.1.3 トーストコンポーネント

> **▼ このコードがやること（先に日本語で）:** トースト通知の「実際の見た目」を作るコンポーネントです。先ほどの Context から「今表示すべきトーストの一覧」を受け取り、種類（成功・エラーなど）に応じた色やアイコンを付けて画面の隅に並べます。一定時間が経つと自動で消える点と、右からスライドして登場するアニメーションがポイントです。各処理の詳細はコメントを参照してください。

```typescript
// components/Toast.tsx
// ↑ トーストの実際の見た目（UIコンポーネント）を定義するファイル。
//   ToastContext からトースト配列を受け取って画面に描画する。

"use client";   // クライアントコンポーネント宣言（state を使うため）

// React Hooks
import { useState, useEffect, useCallback } from "react";
// 型を別名 ToastType で取り込む（同名のローカル定義と区別するため as でリネーム）
import { Toast as ToastType } from "@/types/toast";
// Context Hook
import { useToast } from "@/contexts/ToastContext";

// 各トーストタイプのアイコンとスタイルをまとめた設定オブジェクト
// 4種類（success/error/warning/info）ごとにアイコンと色を変える
// このように設定を1か所にまとめておくと、新しい種類を追加しやすい（DRY原則）
const toastConfig = {
  // ─── 成功（チェックマークアイコン+緑色） ───
  success: {
    icon: (
      <svg
        className="h-5 w-5"             /* 20x20 サイズ */
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        strokeWidth={2}
        aria-hidden="true"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          /* チェックマーク + 円形のパス */
          d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
        />
      </svg>
    ),
    /* 各色は緑系の組み合わせ。ライトモードは薄背景+濃文字、ダークモードは逆 */
    containerClass:
      "border-green-200 bg-green-50 dark:border-green-800 dark:bg-green-950",
    iconClass: "text-green-600 dark:text-green-400",
    titleClass: "text-green-800 dark:text-green-200",
    messageClass: "text-green-700 dark:text-green-300",
    progressClass: "bg-green-500",   /* プログレスバーは中間色 */
  },
  // ─── エラー（!アイコン+赤色） ───
  error: {
    icon: (
      <svg
        className="h-5 w-5"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        strokeWidth={2}
        aria-hidden="true"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          /* 円の中に「!」を描くパス */
          d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z"
        />
      </svg>
    ),
    containerClass:
      "border-red-200 bg-red-50 dark:border-red-800 dark:bg-red-950",
    iconClass: "text-red-600 dark:text-red-400",
    titleClass: "text-red-800 dark:text-red-200",
    messageClass: "text-red-700 dark:text-red-300",
    progressClass: "bg-red-500",
  },
  // ─── 警告（三角!アイコン+黄色） ───
  warning: {
    icon: (
      <svg
        className="h-5 w-5"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        strokeWidth={2}
        aria-hidden="true"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          /* 三角形の中に「!」（道路標識のような形） */
          d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z"
        />
      </svg>
    ),
    containerClass:
      "border-yellow-200 bg-yellow-50 dark:border-yellow-800 dark:bg-yellow-950",
    iconClass: "text-yellow-600 dark:text-yellow-400",
    titleClass: "text-yellow-800 dark:text-yellow-200",
    messageClass: "text-yellow-700 dark:text-yellow-300",
    progressClass: "bg-yellow-500",
  },
  // ─── 情報（iアイコン+青色） ───
  info: {
    icon: (
      <svg
        className="h-5 w-5"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        strokeWidth={2}
        aria-hidden="true"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          /* 円の中に「i」（インフォメーション） */
          d="M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z"
        />
      </svg>
    ),
    containerClass:
      "border-blue-200 bg-blue-50 dark:border-blue-800 dark:bg-blue-950",
    iconClass: "text-blue-600 dark:text-blue-400",
    titleClass: "text-blue-800 dark:text-blue-200",
    messageClass: "text-blue-700 dark:text-blue-300",
    progressClass: "bg-blue-500",
  },
};

// ----------------------------------------------------------------------------
// 個別のトーストアイテム（1件分の見た目）
// ----------------------------------------------------------------------------
function ToastItem({ toast }: { toast: ToastType }) {
  // Context から「削除関数」を取り出す
  const { removeToast } = useToast();
  // isExiting: 閉じるアニメーション中かどうか（true で出ていくアニメに切替）
  const [isExiting, setIsExiting] = useState(false);
  // 種類に応じた設定（アイコン・色）を取得
  const config = toastConfig[toast.type];

  // 閉じるボタンクリック時のハンドラ
  // useCallback で関数をメモ化（依存が変わらない限り同じ関数を使い回す）
  const handleClose = useCallback(() => {
    setIsExiting(true);   // アニメーション開始
    // 300ms（アニメーション時間）待ってから state から削除
    setTimeout(() => {
      removeToast(toast.id);
    }, 300);
  }, [removeToast, toast.id]);

  // プログレスバー用のアニメーション時間（指定がなければ5秒）
  // || 演算子: 左が falsy（undefined や 0）なら右を使う
  const duration = toast.duration || 5000;

  return (
    <div
      role="alert"             // ARIA ロール: 警告メッセージとして扱う
      aria-live="assertive"    // スクリーンリーダーに即読み上げさせる
      aria-atomic="true"       // 内容変更時に全体を読み直す（部分だけ読まない）
      className={`
        relative
        flex w-full max-w-sm items-start gap-3
        overflow-hidden
        rounded-lg border
        p-4
        shadow-lg
        ${config.containerClass}
        ${isExiting ? "animate-toast-out" : "animate-toast-in"}
      `}
      /* ▼ クラスの読み解き ▼
         relative                : 子要素の absolute（プログレスバー）の基準
         flex w-full max-w-sm    : flex 横並び、幅100%、最大幅 24rem (384px)
         items-start             : 縦方向の上揃え（アイコンとテキストを上揃え）
         gap-3                   : 子要素間 0.75rem (12px)
         overflow-hidden         : プログレスバーがはみ出ないように
         rounded-lg border       : 角丸 + 枠線
         p-4                     : 内側余白 1rem (16px)
         shadow-lg               : 大きめの影
         ${config.containerClass}: 種類ごとの色設定（緑/赤/黄/青）
         三項演算子で「閉じる時 → toast-out / 開く時 → toast-in」アニメ切替
      */
    >
      {/* アイコン */}
      <div className={`flex-shrink-0 ${config.iconClass}`}>
        {/* flex-shrink-0: flex 子要素が縮まないようにする（テキストが長くてもアイコンサイズ維持） */}
        {config.icon}
      </div>

      {/* テキスト内容 */}
      <div className="flex-1 min-w-0">
        {/* flex-1: 余り領域を全て使う
            min-w-0: flex 子要素がデフォルトで min-width: auto になり改行されない問題を回避 */}
        <p className={`text-sm font-semibold ${config.titleClass}`}>
          {toast.title}
        </p>
        {/* message があるときだけ補足行を表示 */}
        {toast.message && (
          <p className={`mt-1 text-sm ${config.messageClass}`}>
            {/* mt-1: 上マージン 0.25rem (4px) */}
            {toast.message}
          </p>
        )}
      </div>

      {/* 閉じるボタン */}
      <button
        type="button"                    // フォーム内に置かれても submit にならないよう明示
        onClick={handleClose}
        className={`
          flex-shrink-0
          rounded-md p-1
          opacity-70
          transition-opacity
          hover:opacity-100
          focus:outline-none focus:ring-2 focus:ring-offset-2
          ${config.iconClass}
        `}
        /* opacity-70 → hover:opacity-100: 普段は薄め、ホバーでくっきり */
        aria-label="通知を閉じる"        // スクリーンリーダー用のボタン説明
      >
        <svg
          className="h-4 w-4"           // 16x16 の小さい × アイコン
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
          strokeWidth={2}
          aria-hidden="true"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M6 18L18 6M6 6l12 12"    // 「×」を描く2本の斜め線
          />
        </svg>
      </button>

      {/* プログレスバー（残り時間表示） */}
      <div className="absolute bottom-0 left-0 right-0 h-1">
        {/* absolute bottom-0 left-0 right-0: 親の下端いっぱいに横一杯
            h-1: 高さ 0.25rem (4px) */}
        <div
          className={`h-full ${config.progressClass} opacity-30`}
          style={{
            // インラインアニメーション。CSS の animation プロパティを直接指定
            // shrinkWidth は別途 @keyframes で定義（幅を100%→0%に縮める）
            animation: `shrinkWidth ${duration}ms linear forwards`,
          }}
        />
      </div>
    </div>
  );
}

// ----------------------------------------------------------------------------
// トーストコンテナ（画面右上に固定配置するラッパー）
// ----------------------------------------------------------------------------
// レイアウトのルートに1つ置いて、全トーストをここに集約する
export function ToastContainer() {
  const { toasts } = useToast();   // 表示中のトースト配列を取得

  // 0件なら null を返して描画しない（DOMにも残さない）
  if (toasts.length === 0) return null;

  return (
    <div
      aria-label="通知"
      className="
        fixed
        right-4
        top-4
        z-50
        flex
        flex-col
        gap-3
        sm:right-6
        sm:top-6
      "
      /* ▼ クラスの読み解き ▼
         fixed              : 画面に固定（スクロールしても位置不変）
         right-4 top-4      : 右上から 16px 離れた位置
         z-50               : z-index: 50（他の要素より前面に）
         flex flex-col      : 縦並び
         gap-3              : トースト間 12px の隙間
         sm:right-6 sm:top-6: 640px 以上では 24px に広げる
      */
    >
      {/* トースト配列を1件ずつ ToastItem に展開 */}
      {toasts.map((toast) => (
        <ToastItem key={toast.id} toast={toast} />
      ))}
    </div>
  );
}
```

#### ▼ コードを1つずつ分解して解説

---

##### 解説1: 種類ごとのアイコン・色を1か所にまとめる（`toastConfig`）

```typescript
const toastConfig = {
  success: {
    icon: ( /* チェックマークのSVG */ ),
    containerClass: "border-green-200 bg-green-50 dark:border-green-800 dark:bg-green-950",
    iconClass: "text-green-600 dark:text-green-400",
    // ...
  },
  // error / warning / info も同じ構造
};
```

- `toastConfig` は「4種類（成功・エラー・警告・情報）それぞれの**アイコンと色設定**」をまとめたオブジェクトです。
- 後で `toastConfig[toast.type]` と書くだけで、その種類に合った設定一式を取り出せます。
- このように設定を1か所に集めておくと、**新しい種類を足すのも色を直すのも簡単**になります（同じ記述を繰り返さないDRY原則）。

> **用語: DRY原則（Don't Repeat Yourself）** … 同じ記述を何度も繰り返さず、1か所にまとめる設計の考え方。修正漏れやバグを減らせる。

---

##### 解説2: 閉じるアニメーション付きで削除する（`handleClose`）

```typescript
const [isExiting, setIsExiting] = useState(false);
const handleClose = useCallback(() => {
  setIsExiting(true);
  setTimeout(() => {
    removeToast(toast.id);
  }, 300);
}, [removeToast, toast.id]);
```

- `isExiting` は「いま閉じるアニメーション中か」を表す state です。`true` になると `animate-toast-out`（右へ滑り出る）に切り替わります。
- `handleClose` はまず `setIsExiting(true)` でアニメを始め、**300ミリ秒（アニメ時間）待ってから** `removeToast` で実際にデータから消します。
- 先に消すとアニメが見えないので、「アニメ→その後に削除」の順番が大事です。

> **用語: アニメーション中フラグ** … `isExiting` のように「いま演出の最中か」を保持する真偽値state。これでクラスを切り替えて出入りの動きを制御する。

---

##### 解説3: 読み上げソフトへ通知する（`role` / `aria-live`）

```typescript
<div
  role="alert"
  aria-live="assertive"
  aria-atomic="true"
  className={`... ${config.containerClass} ${isExiting ? "animate-toast-out" : "animate-toast-in"}`}
>
```

- `role="alert"` は「これは警告メッセージだ」とスクリーンリーダーに伝える指定です。
- `aria-live="assertive"` は「内容が出たら**すぐ読み上げて**」という指示。トーストは見逃せない情報なので即時読み上げにしています。
- `aria-atomic="true"` は「一部だけでなく**全文をまとめて読み直す**」指定です。末尾の三項演算子で、状態に応じて入場/退場アニメのクラスを切り替えています。

> **用語: aria-live** … 動的に出る内容を読み上げソフトにどう伝えるかを指定するARIA属性。`assertive`（即時）と `polite`（手が空いたら）がある。

---

##### 解説4: 残り時間を見せるプログレスバー

```typescript
<div className="absolute bottom-0 left-0 right-0 h-1">
  <div
    className={`h-full ${config.progressClass} opacity-30`}
    style={{
      animation: `shrinkWidth ${duration}ms linear forwards`,
    }}
  />
</div>
```

- 外側の `div` は `absolute bottom-0 left-0 right-0` でトーストの**下端いっぱい**に置かれ、`h-1`（高さ4px）の細い帯になります。
- 内側の `div` に `animation: shrinkWidth ${duration}ms linear forwards` を直接指定し、「**表示時間をかけて幅を100%→0%に縮める**」演出をしています。これで「あと何秒で消えるか」が視覚的に分かります。
- `linear`（等速）で時間どおりに、`forwards`（終了状態を維持）で縮みきった状態を保ちます。

> **用語: プログレスバー（progress bar）／forwards** … 進捗や残り時間を帯の長さで示すUI。`forwards` はアニメ終了後も最後のコマの見た目を保つ指定。

---

##### 解説5: 画面右上に全トーストを集約する（`ToastContainer`）

```typescript
export function ToastContainer() {
  const { toasts } = useToast();
  if (toasts.length === 0) return null;
  return (
    <div className="fixed right-4 top-4 z-50 flex flex-col gap-3 sm:right-6 sm:top-6">
      {toasts.map((toast) => (
        <ToastItem key={toast.id} toast={toast} />
      ))}
    </div>
  );
}
```

- `useToast()` で現在のトースト配列を受け取り、0件なら `return null` で何も描きません。
- `fixed right-4 top-4` で**画面の右上に固定**（スクロールしても動かない）、`z-50` で他要素より前面、`flex flex-col gap-3` で縦に間隔を空けて並べます。
- 配列を `map` で1件ずつ `<ToastItem>` に展開します。このコンテナをレイアウトに1つ置くだけで、全トーストがここに集まります。

> **用語: fixed / z-index（z-50）** … `fixed` は画面に固定する配置。`z-50` は重なり順（z-index）を50にする指定で、数字が大きいほど手前に表示される。

---

**使用例:**

> **▼ このコードがやること（先に日本語で）:** ここまでで作ったトーストの仕組みを、実際にどう呼び出すかの例です。削除ボタンを押したら書籍を消し、結果に応じて「成功（緑）」か「エラー（赤）」のトーストを出します。ポイントは `useToast()` を1行呼ぶだけで `toast.success(...)` のように通知を出せること。通信の成否は `res.ok`（HTTP の応答が正常か）で判定しています。

```typescript
// 任意のコンポーネントから呼び出す例
"use client";

// Context にアクセスするカスタムフック
import { useToast } from "@/contexts/ToastContext";

// 削除ボタンコンポーネント。受け取った bookId の書籍を削除する
export function BookDeleteButton({ bookId }: { bookId: string }) {
  // useToast() でトースト操作のためのオブジェクトを取得
  // toast.success / toast.error などのメソッドが使える
  const toast = useToast();

  // 非同期削除処理。async を付けると関数内で await が使える
  const handleDelete = async () => {
    try {
      // fetch で DELETE リクエスト送信。バッククォート ` ` の中で ${} で値を埋め込み可能
      const res = await fetch(`/api/books/${bookId}`, { method: "DELETE" });
      // res.ok は HTTP ステータスが 200-299 なら true
      if (!res.ok) throw new Error("削除に失敗しました");
      // 成功時のトースト（緑色）を表示
      toast.success("削除完了", "書籍が正常に削除されました。");
    } catch (err) {
      // エラー時のトースト（赤色）を表示
      toast.error("エラー", "書籍の削除に失敗しました。もう一度お試しください。");
    }
  };

  return (
    // btn-danger は globals.css で定義した赤い削除ボタン
    <button onClick={handleDelete} className="btn-danger">
      削除
    </button>
  );
}
```

### 3.2 ページネーションコンポーネント

**ページネーション（pagination：複数ページ分割表示）** は、データが多い時に「1 2 3 ... 10」のようにページを切り替える UI です。一度に全件読み込むよりも、サーバーへの負荷とユーザーの待ち時間を減らせます。

> **▼ このコードがやること（先に日本語で）:** 「1 2 3 … 10」のようにページを切り替えるボタン群（ページネーション）を作ります。ポイントは、ページ数がとても多いときに全部のボタンを並べず、現在ページの前後と最初・最後だけを残して途中を「…」で省略する計算をしている点です。前へ/次へボタンや、先頭・末尾ページでボタンを無効化する処理も含みます。詳しい計算はコメントを参照してください。

```typescript
// components/Pagination.tsx
// ↑ ページネーション（複数ページを切り替えるナビゲーション）コンポーネント。
//   1   2   3 ... 10  のような表示を作る。

"use client";   // クライアントコンポーネント（useSearchParams を使う）

import Link from "next/link";                                // Next.js 用リンクコンポーネント（高速遷移）
import { useSearchParams } from "next/navigation";           // URL のクエリパラメータ取得 Hook

// コンポーネントが受け取る props 型
type PaginationProps = {
  currentPage: number;   // 現在表示中のページ番号（1 始まり）
  totalPages: number;    // 全体のページ数
  basePath: string;      // ベースとなるURLパス（例: "/books"）
};

export function Pagination({
  currentPage,
  totalPages,
  basePath,
}: PaginationProps) {
  // 現在の URL のクエリパラメータ（?key=val&...）を取得
  const searchParams = useSearchParams();

  // ページ番号から URL を生成するヘルパー関数
  // 既存の検索条件（?search=xxx 等）を保ちつつ page だけ書き換える
  const createPageUrl = (page: number): string => {
    // URLSearchParams: クエリ文字列を扱う標準 API
    // 既存のパラメータを複製してから page を上書き
    const params = new URLSearchParams(searchParams.toString());
    params.set("page", String(page));                   // 数値を文字列にして set
    return `${basePath}?${params.toString()}`;          // 例: "/books?page=2&search=react"
  };

  // 表示するページ番号のリストを計算する関数
  // 例: currentPage=5, totalPages=10 → [1, "ellipsis", 4, 5, 6, "ellipsis", 10]
  // 返却型: number または "ellipsis"（省略記号）の配列
  const getPageNumbers = (): (number | "ellipsis")[] => {
    const pages: (number | "ellipsis")[] = [];
    const maxVisible = 5;   // 表示するページ番号の最大数（省略記号除く）

    if (totalPages <= maxVisible + 2) {
      // 全ページ数が少ない場合（7ページ以下なら全部表示）
      for (let i = 1; i <= totalPages; i++) {
        pages.push(i);
      }
    } else {
      // 多い場合は「先頭 ... 真ん中 ... 末尾」形式
      // 常に最初のページを表示
      pages.push(1);

      // 現在のページが最初の方（1〜3）にある場合
      // → [1, 2, 3, 4, ..., totalPages]
      if (currentPage <= 3) {
        pages.push(2, 3, 4);
        pages.push("ellipsis");
      }
      // 現在のページが最後の方（totalPages-2 以降）にある場合
      // → [1, ..., totalPages-3, totalPages-2, totalPages-1, totalPages]
      else if (currentPage >= totalPages - 2) {
        pages.push("ellipsis");
        pages.push(totalPages - 3, totalPages - 2, totalPages - 1);
      }
      // 現在のページが中間にある場合
      // → [1, ..., current-1, current, current+1, ..., totalPages]
      else {
        pages.push("ellipsis");
        pages.push(currentPage - 1, currentPage, currentPage + 1);
        pages.push("ellipsis");
      }

      // 常に最後のページを表示
      pages.push(totalPages);
    }

    return pages;
  };

  // ページが1ページしかない場合は表示しない（早期リターン）
  if (totalPages <= 1) return null;

  const pageNumbers = getPageNumbers();         // ページ番号リストを計算
  const isFirstPage = currentPage === 1;        // 最初のページか
  const isLastPage = currentPage === totalPages; // 最後のページか

  // ----- スタイル定数（複数箇所で使い回すために変数化）-----
  // 共通のページ番号ボタンスタイル
  const basePageClass = `
    inline-flex h-10 w-10 items-center justify-center
    rounded-lg text-sm font-medium
    transition-all duration-200
    focus-visible:outline-none focus-visible:ring-2
    focus-visible:ring-primary-500 focus-visible:ring-offset-2
  `;
  /* ▼ クラスの読み解き ▼
     inline-flex h-10 w-10        : 40x40 の正方形ボタン
     items-center justify-center  : 縦横中央に数字を配置
     rounded-lg                   : 角丸 8px
     text-sm font-medium          : 14px 太さ500
     transition-all duration-200  : 全プロパティ 200ms で滑らかに変化
     focus-visible:*              : キーボードフォーカス時のリング
  */

  // 現在のページ（アクティブ）用スタイル
  const activePageClass = `
    ${basePageClass}
    bg-primary-600 text-white shadow-md
    dark:bg-primary-500
  `;
  /* インディゴ背景＋白文字＋影。現在地が視覚的に分かるように */

  // 他のページ番号用スタイル
  const inactivePageClass = `
    ${basePageClass}
    text-gray-600 hover:bg-gray-100
    dark:text-gray-400 dark:hover:bg-gray-800
  `;
  /* 普段は灰色、ホバーで薄い灰色背景 */

  // 「前へ」「次へ」が無効化されているとき用
  const disabledNavClass = `
    inline-flex h-10 items-center justify-center
    rounded-lg px-3 text-sm font-medium
    text-gray-300 cursor-not-allowed
    dark:text-gray-600
  `;
  /* text-gray-300       : とても薄い灰色（押せない感じ） */
  /* cursor-not-allowed : マウスカーソルが禁止マークに */

  // 「前へ」「次へ」が有効なとき用
  const enabledNavClass = `
    inline-flex h-10 items-center justify-center
    rounded-lg px-3 text-sm font-medium
    text-gray-600 hover:bg-gray-100
    transition-all duration-200
    dark:text-gray-400 dark:hover:bg-gray-800
    focus-visible:outline-none focus-visible:ring-2
    focus-visible:ring-primary-500 focus-visible:ring-offset-2
  `;

  return (
    // <nav> 要素はナビゲーションを表すセマンティック要素
    <nav
      role="navigation"
      aria-label="ページネーション"   // スクリーンリーダー向けの説明
      className="flex items-center justify-center gap-1 py-8"
      /* flex items-center justify-center: 中央寄せ
         gap-1                            : 子要素間 4px
         py-8                             : 上下パディング 2rem (32px) */
    >
      {/* 前のページボタン */}
      {/* 最初のページなら無効化された span を、それ以外なら Link を表示 */}
      {isFirstPage ? (
        <span className={disabledNavClass} aria-disabled="true">
          <svg
            className="mr-1 h-4 w-4"   /* mr-1: 右マージン 4px */
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={2}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M15.75 19.5L8.25 12l7.5-7.5"   /* 左向きの「<」アイコン */
            />
          </svg>
          前へ
        </span>
      ) : (
        // Link は Next.js のクライアントサイドナビゲーション用
        <Link
          href={createPageUrl(currentPage - 1)}   // 一つ前のページURL
          className={enabledNavClass}
          aria-label="前のページへ"
        >
          <svg
            className="mr-1 h-4 w-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={2}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M15.75 19.5L8.25 12l7.5-7.5"
            />
          </svg>
          前へ
        </Link>
      )}

      {/* ページ番号リスト */}
      <div className="flex items-center gap-1">
        {/* pageNumbers 配列を1件ずつ展開 */}
        {pageNumbers.map((page, index) => {
          // 要素が省略記号 "ellipsis" の場合 → "..." を表示
          if (page === "ellipsis") {
            return (
              <span
                key={`ellipsis-${index}`}  // 同じキーが重複しないよう index を含める
                className="inline-flex h-10 w-10 items-center justify-center text-gray-400"
                aria-hidden="true"          // 省略記号は装飾なのでスクリーンリーダーには無視させる
              >
                ...
              </span>
            );
          }

          // 数値の場合 → 現在のページか他のページかで表示分岐
          const isActive = page === currentPage;

          return isActive ? (
            // アクティブ（現在）ページは span（リンクではない）
            <span
              key={page}
              className={activePageClass}
              aria-current="page"   // ARIA 属性: 「現在のページ」と伝える
              aria-label={`${page}ページ目（現在のページ）`}
            >
              {page}
            </span>
          ) : (
            // 他のページは Link で遷移可能に
            <Link
              key={page}
              href={createPageUrl(page)}
              className={inactivePageClass}
              aria-label={`${page}ページ目へ`}
            >
              {page}
            </Link>
          );
        })}
      </div>

      {/* 次のページボタン（最後のページなら無効化） */}
      {isLastPage ? (
        <span className={disabledNavClass} aria-disabled="true">
          次へ
          <svg
            className="ml-1 h-4 w-4"   /* ml-1: 左マージン 4px */
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={2}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M8.25 4.5l7.5 7.5-7.5 7.5"   /* 右向きの「>」アイコン */
            />
          </svg>
        </span>
      ) : (
        <Link
          href={createPageUrl(currentPage + 1)}   // 次のページURL
          className={enabledNavClass}
          aria-label="次のページへ"
        >
          次へ
          <svg
            className="ml-1 h-4 w-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={2}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M8.25 4.5l7.5 7.5-7.5 7.5"
            />
          </svg>
        </Link>
      )}
    </nav>
  );
}
```

#### ▼ コードを1つずつ分解して解説

---

##### 解説1: 検索条件を保ったままページURLを作る（`createPageUrl`）

```typescript
const createPageUrl = (page: number): string => {
  const params = new URLSearchParams(searchParams.toString());
  params.set("page", String(page));
  return `${basePath}?${params.toString()}`;
};
```

- `URLSearchParams` は「`?key=val&...` というクエリ文字列を扱う標準の道具」です。今のURLのクエリを複製してから操作します。
- `params.set("page", String(page))` で `page` の値だけを上書きします。`String(page)` は数値を文字列に変換しています（クエリは文字列のため）。
- これにより `?search=react` のような**既存の検索条件を消さずに**、ページ番号だけ差し替えたURL（例: `/books?page=2&search=react`）を作れます。

> **用語: クエリパラメータ／URLSearchParams** … URLの `?` 以降の `key=value` 部分のこと。`URLSearchParams` はそれを安全に読み書きするブラウザ標準API。

---

##### 解説2: 表示するページ番号を計算する（`getPageNumbers`）

```typescript
const getPageNumbers = (): (number | "ellipsis")[] => {
  const pages: (number | "ellipsis")[] = [];
  const maxVisible = 5;
  if (totalPages <= maxVisible + 2) {
    for (let i = 1; i <= totalPages; i++) pages.push(i);
  } else {
    pages.push(1);
    if (currentPage <= 3) { pages.push(2, 3, 4); pages.push("ellipsis"); }
    else if (currentPage >= totalPages - 2) { pages.push("ellipsis"); pages.push(totalPages - 3, totalPages - 2, totalPages - 1); }
    else { pages.push("ellipsis"); pages.push(currentPage - 1, currentPage, currentPage + 1); pages.push("ellipsis"); }
    pages.push(totalPages);
  }
  return pages;
};
```

- この関数は「画面に並べるページ番号の配列」を作ります。要素は**数値**か、省略記号を表す文字列 `"ellipsis"` のどちらかです。
- ページ数が少ない（7ページ以下）なら全部表示。多いときは「**先頭 … 真ん中 … 末尾**」の形にして、間を `"ellipsis"`（…）でまとめます。
- 現在ページが前寄り・後ろ寄り・中間のどこにあるかで、表示する番号の組み合わせを変えています。例: 5ページ目/全10なら `[1, "ellipsis", 4, 5, 6, "ellipsis", 10]`。

> **用語: ユニオン型（`number | "ellipsis"`）** … 「数値、または文字列 ellipsis のどちらか」のように複数の型を許す型。配列に番号と省略記号を混在させるのに使う。

---

##### 解説3: 先頭・末尾でボタンを無効化する

```typescript
if (totalPages <= 1) return null;
const isFirstPage = currentPage === 1;
const isLastPage = currentPage === totalPages;
```

```typescript
{isFirstPage ? (
  <span className={disabledNavClass} aria-disabled="true"> ... 前へ </span>
) : (
  <Link href={createPageUrl(currentPage - 1)} className={enabledNavClass} aria-label="前のページへ"> ... 前へ </Link>
)}
```

- `if (totalPages <= 1) return null` は「1ページしかないなら、そもそもページ送りを表示しない」早期リターンです。
- `isFirstPage` / `isLastPage` で「今が最初/最後のページか」を判定します。
- 最初のページでは「前へ」を**リンクではなく無効化された `<span>`**（`aria-disabled="true"`）にして、押せないことを見た目と読み上げソフトの両方に伝えます。それ以外は `<Link>` で前ページへ遷移できます。

> **用語: aria-disabled** … 「この要素は今は操作できない」ことをスクリーンリーダーに伝えるARIA属性。見た目の `cursor-not-allowed` と合わせて使う。

---

##### 解説4: 現在ページと他ページを描き分ける（map 内の分岐）

```typescript
{pageNumbers.map((page, index) => {
  if (page === "ellipsis") {
    return <span key={`ellipsis-${index}`} ... aria-hidden="true">...</span>;
  }
  const isActive = page === currentPage;
  return isActive ? (
    <span key={page} className={activePageClass} aria-current="page" ...>{page}</span>
  ) : (
    <Link key={page} href={createPageUrl(page)} className={inactivePageClass} ...>{page}</Link>
  );
})}
```

- 配列を `map` で展開し、要素が `"ellipsis"` なら「…」を表示します。`aria-hidden="true"` で読み上げソフトには無視させます（装飾のため）。
- 数値の場合は `isActive`（現在ページか）で分岐。現在ページは**リンクにせず `<span>`** にし、`aria-current="page"` で「これが現在地」と伝えます。
- それ以外のページは `<Link>` にして、クリックでそのページへ遷移できるようにします。

> **用語: aria-current="page"** … 同種のリンク群の中で「これが今いるページ」であることを読み上げソフトに示すARIA属性。

---

### 3.3 空状態（データがない場合）の改善

**空状態（empty state：データが0件・該当なし・初回利用などの「何もない」画面）** は、ただ真っ白にせず、ユーザーに次の行動を促す案内を出すのが現代的なUXのお作法です。

> **▼ このコードがやること（先に日本語で）:** データが0件のときに、画面を真っ白にせず「まだ何もありません」とやさしく案内するコンポーネントを作ります。ポイントは、イラストや説明文だけでなく「最初の1冊を登録する」ボタンを置いて、ユーザーが次に何をすればよいかを示すことです。これは使い心地（UX）を良くするための定番テクニックです。

```typescript
// components/EmptyState.tsx
// ↑ データが0件のときに「まだ何もありません」と分かりやすく案内するコンポーネント。
//   ただ空白にするより、ユーザーに次の行動を促すボタンを置くのが UX 的に親切。

"use client";

import Link from "next/link";

// props 型。? が付いているプロパティはすべて省略可能
type EmptyStateProps = {
  title?: string;                                 // タイトル
  message?: string;                               // 補足説明文
  actionLabel?: string;                           // ボタンの文字
  actionHref?: string;                            // ボタンのリンク先
  icon?: "book" | "search" | "error";             // 3種類から選択
};

// 引数のデフォルト値を分割代入で指定。
// 呼び出し側が省略してもこれらの値が使われる
export function EmptyState({
  title = "書籍が見つかりません",
  message = "まだ書籍が登録されていません。最初の一冊を登録してみましょう。",
  actionLabel = "書籍を登録する",
  actionHref = "/books/new",
  icon = "book",
}: EmptyStateProps) {
  // アイコンの描画
  const renderIcon = () => {
    switch (icon) {
      case "book":
        return (
          <svg
            className="h-16 w-16"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={1}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M12 6.042A8.967 8.967 0 006 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 016 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 016-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0018 18a8.967 8.967 0 00-6 2.292m0-14.25v14.25"
            />
          </svg>
        );
      case "search":
        return (
          <svg
            className="h-16 w-16"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={1}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"
            />
          </svg>
        );
      case "error":
        return (
          <svg
            className="h-16 w-16"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={1}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z"
            />
          </svg>
        );
    }
  };

  return (
    <div
      className="
        animate-fade-in
        flex flex-col items-center justify-center
        rounded-xl
        border-2 border-dashed border-gray-200
        bg-gray-50/50
        px-6 py-16
        text-center
        dark:border-gray-700
        dark:bg-gray-800/50
      "
      /* ▼ クラスの読み解き ▼
         animate-fade-in              : フェードイン
         flex flex-col items-center   : 縦並びで中央寄せ
         justify-center               : 縦方向も中央
         rounded-xl                   : 大きめの角丸
         border-2 border-dashed       : 2px の点線枠線（dashed:破線）
         bg-gray-50/50                : 灰色の透明度50%（うっすら）
         px-6 py-16                   : 横24px、縦64px の余白
         text-center                  : テキスト中央揃え
         dark:* で各色のダーク版指定
      */
    >
      {/* アイコン */}
      <div
        className="
          mb-4
          rounded-full
          bg-gray-100
          p-4
          text-gray-400
          dark:bg-gray-700
          dark:text-gray-500
        "
        /* mb-4         : 下マージン 1rem (16px)
           rounded-full : 完全な円形（円形バッジの容器）
           bg-gray-100  : 薄い灰色背景
           p-4          : padding 16px
           text-gray-400: アイコン色 */
      >
        {renderIcon()}
      </div>

      {/* タイトル */}
      <h3
        className="
          mb-2
          text-lg font-semibold
          text-gray-900
          dark:text-gray-100
        "
        /* mb-2: 下16px / text-lg: 18px / font-semibold: 600 */
      >
        {title}
      </h3>

      {/* メッセージ */}
      <p
        className="
          mb-6
          max-w-sm
          text-sm text-gray-500
          dark:text-gray-400
        "
        /* mb-6      : 下24px
           max-w-sm  : 最大幅 24rem (384px)。長い文章でも適度に折り返す
           text-sm   : 14px
           text-gray-500 : 補足色 */
      >
        {message}
      </p>

      {/* アクションボタン（actionHref があれば表示） */}
      {actionHref && (
        <Link href={actionHref} className="btn-primary gap-2">
          {/* btn-primary: globals.css のプライマリボタン
              gap-2     : flex 子要素間 8px（アイコンと文字の間） */}
          <svg
            className="h-4 w-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={2}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M12 4.5v15m7.5-7.5h-15"   /* 「+」プラスマーク */
            />
          </svg>
          {actionLabel}
        </Link>
      )}
    </div>
  );
}
```

#### ▼ コードを1つずつ分解して解説

---

##### 解説1: 省略可能なpropsにデフォルト値を持たせる

```typescript
type EmptyStateProps = {
  title?: string;
  message?: string;
  actionLabel?: string;
  actionHref?: string;
  icon?: "book" | "search" | "error";
};

export function EmptyState({
  title = "書籍が見つかりません",
  message = "まだ書籍が登録されていません。最初の一冊を登録してみましょう。",
  actionLabel = "書籍を登録する",
  actionHref = "/books/new",
  icon = "book",
}: EmptyStateProps) {
```

- 型の各プロパティに付いた `?` は「**省略してもよい**」という意味です。すべて任意なので、`<EmptyState />` だけでも使えます。
- 分割代入の `title = "..."` の形は「**呼び出し側が渡さなかったときに使うデフォルト値**」です。指定があればそちらが優先されます。
- `icon?: "book" | "search" | "error"` は3種類のうちのどれか、というユニオン型。これ以外の文字列は書けません。

> **用語: デフォルト引数（default parameter）** … 引数が渡されなかったとき自動で使われる初期値。`title = "..."` のように分割代入と組み合わせて書ける。

---

##### 解説2: iconの値に応じてSVGを返す（`renderIcon`）

```typescript
const renderIcon = () => {
  switch (icon) {
    case "book":
      return ( <svg ...>{/* 本のアイコン */}</svg> );
    case "search":
      return ( <svg ...>{/* 虫眼鏡のアイコン */}</svg> );
    case "error":
      return ( <svg ...>{/* エラーのアイコン */}</svg> );
  }
};
```

- `renderIcon` は `icon` の値（"book"/"search"/"error"）で **表示するSVGアイコンを出し分ける** 関数です。
- `{renderIcon()}` のように本体側で呼び出すと、選ばれたアイコンが描画されます。
- 用途に合わせて「検索結果0件なら虫眼鏡」「エラーなら警告アイコン」のように、同じコンポーネントで見た目を変えられます。

> **用語: SVG（Scalable Vector Graphics）** … 拡大しても劣化しないベクター形式の画像。アイコンをコードで描けるためWebでよく使われる。

---

##### 解説3: 行動を促すボタンを条件付きで出す（CTA）

```typescript
{actionHref && (
  <Link href={actionHref} className="btn-primary gap-2">
    <svg ...>{/* 「+」プラスマーク */}</svg>
    {actionLabel}
  </Link>
)}
```

- `{actionHref && (...)}` は「**リンク先が指定されているときだけ**ボタンを表示する」条件付きレンダリングです（`&&` の短絡評価）。
- `<Link>` に `btn-primary`（globals.css のプライマリボタン）を当て、`gap-2` でアイコンと文字の間に8pxの隙間を作っています。
- 空状態でただ案内するだけでなく「最初の1冊を登録する」など**次の行動への入口**を置くのが、現代的なUXの定石です。

> **用語: CTA（Call To Action）** … ユーザーに次の行動を促すボタンやリンク。「登録する」「始める」など、画面上で最も目立たせたい要素。

---

### 3.4 ローディングスケルトン

**スケルトン（skeleton：骨組み）** は、データ取得中に「これからこういう形の中身が出ます」と予告するグレーの仮表示です。スピナー（くるくる回るアイコン）よりも「コンテンツの骨組み」を見せたほうが、待ち時間の体感が短くなることが知られています。

> **▼ このコードがやること（先に日本語で）:** データ読み込み中に表示する「骨組み（スケルトン）」コンポーネントを作ります。実際のカードと同じ大きさのグレーの仮表示を並べ、光が左右に流れるアニメーション（shimmer）を付けることで「いま読み込み中です」と自然に伝えます。くるくる回るスピナーより待ち時間が短く感じられるのがねらいです。

```typescript
// components/BookCardSkeleton.tsx
// ↑ ローディング中（データ読込中）に表示する「骨組み」コンポーネント。
//   実際のコンテンツと同じ大きさのグレーのプレースホルダーを表示することで
//   「もうすぐ何かが表示される」ことをユーザーに伝える。

// 1件分のスケルトン
export function BookCardSkeleton() {
  return (
    <div className="card overflow-hidden">
      {/* サムネイルスケルトン（書籍画像の代わり） */}
      <div
        className="
          aspect-[3/4] w-full
          bg-gradient-to-r from-gray-200 via-gray-100 to-gray-200
          bg-[length:200%_100%]
          animate-shimmer
          dark:from-gray-700 dark:via-gray-600 dark:to-gray-700
        "
        /* ▼ クラスの読み解き ▼
           aspect-[3/4] w-full                : 縦横比 3:4、幅100%
           bg-gradient-to-r ... from/via/to   : 左→右の3色グラデーション（中央だけ明るい）
           bg-[length:200%_100%]              : 背景画像のサイズを横200%に
                                                 → 余分な部分が画面外にあり、shimmer で動かして光が流れる演出
           animate-shimmer                    : tailwind.config で定義したアニメーションを再生
        */
      />

      {/* テキストスケルトン */}
      <div className="flex flex-col gap-2 p-4">
        {/* タイトル行 */}
        <div
          className="
            h-4 w-3/4 rounded
            bg-gradient-to-r from-gray-200 via-gray-100 to-gray-200
            bg-[length:200%_100%]
            animate-shimmer
            dark:from-gray-700 dark:via-gray-600 dark:to-gray-700
          "
          /* h-4: 16px / w-3/4: 幅75% / rounded: 角丸 */
        />
        {/* 著者行（少し遅らせて動かす） */}
        <div
          className="
            h-3 w-1/2 rounded
            bg-gradient-to-r from-gray-200 via-gray-100 to-gray-200
            bg-[length:200%_100%]
            animate-shimmer
            dark:from-gray-700 dark:via-gray-600 dark:to-gray-700
          "
          style={{ animationDelay: "0.1s" }}   /* 0.1秒遅れて開始（複数行が同じタイミングだと不自然なため） */
        />
        {/* 評価行 */}
        <div
          className="
            mt-2 h-3 w-1/3 rounded
            bg-gradient-to-r from-gray-200 via-gray-100 to-gray-200
            bg-[length:200%_100%]
            animate-shimmer
            dark:from-gray-700 dark:via-gray-600 dark:to-gray-700
          "
          style={{ animationDelay: "0.2s" }}   /* さらに遅らせる */
        />
      </div>
    </div>
  );
}

// グリッド全体のスケルトン（複数のスケルトンを並べる）
// count はデフォルト 8、? は省略可能を意味する
export function BookGridSkeleton({ count = 8 }: { count?: number }) {
  return (
    <div
      className="
        grid grid-cols-1 gap-4
        sm:grid-cols-2 sm:gap-5
        lg:grid-cols-3 lg:gap-6
        xl:grid-cols-4
      "
      /* BookGrid と同じレスポンシブグリッド設定にすることで、
         ローディング中も実際の表示と同じ配置になる */
    >
      {/* Array.from({ length: count }) で count 個の空配列を作り、map で展開する慣用句 */}
      {/* _ は使わない引数の慣例的名前（i だけ使う） */}
      {Array.from({ length: count }).map((_, i) => (
        <BookCardSkeleton key={i} />
      ))}
    </div>
  );
}
```

#### ▼ コードを1つずつ分解して解説

---

##### 解説1: 光が流れるシマー効果（gradient + shimmer）

```typescript
<div
  className="
    aspect-[3/4] w-full
    bg-gradient-to-r from-gray-200 via-gray-100 to-gray-200
    bg-[length:200%_100%]
    animate-shimmer
    dark:from-gray-700 dark:via-gray-600 dark:to-gray-700
  "
/>
```

- `aspect-[3/4] w-full` で「実際の書籍画像と同じ縦長の枠」を作ります。`[ ]` はTailwindに無い任意の値を直接書く構文です。
- `bg-gradient-to-r from-gray-200 via-gray-100 to-gray-200` は「左→右の3色グラデーション（中央だけ少し明るい）」。これが**流れる光の素**になります。
- `bg-[length:200%_100%]` で背景を横2倍に引き伸ばして余りを画面外に置き、`animate-shimmer` でそれを左右に動かすことで「**キラッと光が流れる**」演出になります。

> **用語: スケルトン（skeleton）／シマー（shimmer）** … スケルトンは読込中に出すグレーの仮表示。シマーはそこに光を流して「読込中」を自然に伝えるアニメーション。

---

##### 解説2: 行ごとに開始をずらす（`animationDelay`）

```typescript
<div className="h-3 w-1/2 rounded ... animate-shimmer ..." style={{ animationDelay: "0.1s" }} />
<div className="mt-2 h-3 w-1/3 rounded ... animate-shimmer ..." style={{ animationDelay: "0.2s" }} />
```

- タイトル行・著者行・評価行と、テキストの仮表示を `h-4`/`h-3`、`w-3/4`/`w-1/2`/`w-1/3` のように**高さと幅を変えて**並べ、実際のレイアウトに似せています。
- `animationDelay: "0.1s"` / `"0.2s"` で各行のアニメ開始を少しずつ遅らせています。
- 全行が同じタイミングで光ると不自然なので、**ずらすことで自然な動き**に見せています。

> **用語: animationDelay（アニメーション遅延）** … アニメ開始までの待ち時間。要素ごとに変えると、一斉でなく順々に動く生き生きとした演出になる。

---

##### 解説3: 必要な数だけスケルトンを並べる（`Array.from`）

```typescript
export function BookGridSkeleton({ count = 8 }: { count?: number }) {
  return (
    <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 sm:gap-5 lg:grid-cols-3 lg:gap-6 xl:grid-cols-4">
      {Array.from({ length: count }).map((_, i) => (
        <BookCardSkeleton key={i} />
      ))}
    </div>
  );
}
```

- `count = 8` はデフォルトで8個のスケルトンを並べる指定（省略可能）。
- `Array.from({ length: count })` は「**要素が count 個の配列を作る**」慣用句で、その `.map` で `<BookCardSkeleton>` を必要数だけ生成します。`_` は「使わない引数」の慣例的な名前です。
- グリッドのクラスを `BookGrid` と**同じレスポンシブ設定**にしているので、読込中も本物と同じ配置で表示されます。

> **用語: Array.from({ length: n })** … 長さ n の配列を作る定番の書き方。`.map` と組み合わせて「同じ要素を n 個並べる」のに使う。

---

## 4. アニメーション

### 4.1 Tailwind のトランジションクラス

Tailwind CSS にはトランジション用のユーティリティクラスが組み込まれています。

「トランジション（transition）」と「アニメーション（animation）」の違いを整理しておきます。

- **トランジション**: ある状態 A から状態 B に変化するときの「途中の動き」を滑らかにする。例: `hover` で色を変える時に 0.2 秒かける。
- **アニメーション**: キーフレーム（`@keyframes`）を使って、独自の動きを定義する。例: ローディングスピナーの回転、フェードイン演出。

| クラス | 説明 | CSS 出力 |
|--------|------|----------|
| `transition` | 一般的なプロパティにトランジション適用 | `transition-property: color, background-color, border-color, ...` |
| `transition-all` | 全プロパティにトランジション適用 | `transition-property: all` |
| `transition-colors` | 色関連のみ | `transition-property: color, background-color, ...` |
| `transition-opacity` | 透明度のみ | `transition-property: opacity` |
| `transition-transform` | 変形のみ | `transition-property: transform` |
| `duration-150` | 150ms | `transition-duration: 150ms` |
| `duration-200` | 200ms | `transition-duration: 200ms` |
| `duration-300` | 300ms | `transition-duration: 300ms` |
| `duration-500` | 500ms | `transition-duration: 500ms` |
| `ease-in` | 加速カーブ（最初遅く・最後速い） | `transition-timing-function: cubic-bezier(0.4, 0, 1, 1)` |
| `ease-out` | 減速カーブ（最初速く・最後ゆっくり） | `transition-timing-function: cubic-bezier(0, 0, 0.2, 1)` |
| `ease-in-out` | 加速→減速カーブ | `transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)` |

> **イージング（easing）の選び方の目安：**
> - 要素が登場する時 → `ease-out`（パッと出てフワッと止まる、自然な動き）
> - 要素が消える時 → `ease-in`（じわっと加速して消える）
> - 行き来する時（トグル等）→ `ease-in-out`

### 4.2 カードホバーエフェクト

**ホバーエフェクト（hover effect：マウスを乗せた時に発動する視覚効果）** は、ユーザーに「ここはクリックできる」「ここに注目すべき」と伝える重要な合図になります。やりすぎると煩わしいので、微妙な変化（影が深くなる・少し浮く・色が変わる）を組み合わせるのがコツです。

> **▼ このコードがやること（先に日本語で）:** カードにマウスを乗せた時の「ふわっと浮かぶ・影が深くなる」などの演出（ホバーエフェクト）を付ける例です。ポイントは `transition`（変化を時間をかけて滑らかにつなぐ）と組み合わせて、急にパッと変わらず気持ちよく動かすこと。やりすぎると煩わしいので、影・浮き上がり・色変化を控えめに重ねるのがコツです。

```typescript
// components/BookCard.tsx 内のホバーエフェクト部分（上のシンプル版に加えて、より凝った演出を追加した例）

// カード全体のホバー（上に浮かぶ + 影が深くなる）
<article
  className="
    card
    overflow-hidden
    transition-all duration-300 ease-out
    hover:-translate-y-1.5
    hover:shadow-card-hover
  "
  /* ▼ クラスの読み解き ▼
     card                       : globals.css のカード基本スタイル
     overflow-hidden            : 中身がはみ出さない（画像ズーム時のため）
     transition-all duration-300: 全プロパティを 300ms かけて変化
     ease-out                   : 最初速く・最後ゆっくり（自然な減速カーブ）
     hover:-translate-y-1.5     : ホバーで上に 6px 移動（浮き上がる）
     hover:shadow-card-hover    : ホバーで大きな影に
  */
>
  {/* サムネイル画像のホバー（ズームイン） */}
  <div className="relative aspect-[3/4] overflow-hidden">
    <img
      src={book.thumbnailUrl}
      alt={book.title}
      className="
        h-full w-full object-cover
        transition-transform duration-500 ease-out
        group-hover:scale-110
      "
      /* group-hover:scale-110 : 親（.group）のホバー時に画像を 110% にズーム */
    />

    {/* オーバーレイ（ホバー時にフェードイン） */}
    <div
      className="
        absolute inset-0
        bg-gradient-to-t from-black/60 via-transparent to-transparent
        opacity-0
        transition-opacity duration-300
        group-hover:opacity-100
      "
      /* ▼ クラスの読み解き ▼
         absolute inset-0          : 親いっぱいに広げる（top:0 right:0 bottom:0 left:0）
         bg-gradient-to-t          : 下→上方向のグラデーション
         from-black/60 ... transparent: 下側は黒60%、上側は透明
         opacity-0                 : 通常は透明（非表示）
         group-hover:opacity-100   : 親ホバー時に表示
      */
    />

    {/* 「詳細を見る」テキスト（ホバー時に下からスライドイン） */}
    <div
      className="
        absolute bottom-0 left-0 right-0
        p-4
        text-white
        translate-y-4 opacity-0
        transition-all duration-300
        group-hover:translate-y-0 group-hover:opacity-100
      "
      /* translate-y-4 opacity-0       : 通常は 16px 下にずれて透明（見えない）
         group-hover:translate-y-0 ... : ホバー時に元位置 + 不透明（スライドアップで登場）
      */
    >
      <span className="text-sm font-medium">詳細を見る →</span>
    </div>
  </div>
</article>
```

### 4.3 ページ遷移時のアニメーション

Next.js App Router でのページ遷移時にフェードイン・スライドインのアニメーションを付けます。

> **▼ このコードがやること（先に日本語で）:** ページを移動したときに、新しい画面が「ふわっ」とフェードインしながら少し上に上がって現れる演出を付けるラッパー部品を作ります。仕組みのカギは、URL（パス）が変わったら一瞬だけ「透明・少し下」の状態を描いてから「不透明・元の位置」へ切り替えることで、アニメーションを毎回確実に発動させる点です。`requestAnimationFrame` の役割はコメントで説明しています。

```typescript
// components/PageTransition.tsx
// ↑ ページ遷移時に「ふわっ」とフェードイン＋上昇するアニメーションを付けるラッパー。
//   <PageTransition>{children}</PageTransition> のように使う。

"use client";

import { usePathname } from "next/navigation";                       // 現在の URL パス取得 Hook
import { useEffect, useState, type ReactNode } from "react";         // React の基本 Hook と型

// type ReactNode: JSXとして描画可能なあらゆる値（文字列・JSX・配列など）の型
type PageTransitionProps = {
  children: ReactNode;
};

export function PageTransition({ children }: PageTransitionProps) {
  const pathname = usePathname();                              // 現在のパス（/books など）
  const [isVisible, setIsVisible] = useState(false);           // 表示状態（アニメ用フラグ）
  const [displayChildren, setDisplayChildren] = useState(children); // 表示中の子要素

  useEffect(() => {
    // pathname または children が変わったときに実行される

    // パスが変わるたびにアニメーションをリセット
    setIsVisible(false);            // 一旦非表示状態に
    setDisplayChildren(children);   // 子要素を更新

    // requestAnimationFrame: ブラウザの次の描画タイミングで実行されるコールバック
    // → ブラウザが「非表示状態」を一度描画してから、すぐに「表示」に切り替えるため
    //   アニメーションが確実に発火する
    const timer = requestAnimationFrame(() => {
      setIsVisible(true);
    });

    // クリーンアップ: 次の useEffect 実行前 or アンマウント時にタイマーをキャンセル
    return () => cancelAnimationFrame(timer);
  }, [pathname, children]);   // この配列の値が変わるたびに再実行

  return (
    <div
      className={`
        transition-all duration-300 ease-out
        ${
          isVisible
            ? "translate-y-0 opacity-100"   // 表示中 → 元位置 / 不透明
            : "translate-y-2 opacity-0"     // 非表示 → 8px下 / 透明
        }
      `}
    >
      {displayChildren}
    </div>
  );
}
```

#### ▼ コードを1つずつ分解して解説

---

##### 解説1: パスと表示中の子要素を state で持つ

```typescript
const pathname = usePathname();
const [isVisible, setIsVisible] = useState(false);
const [displayChildren, setDisplayChildren] = useState(children);
```

- `usePathname()` は「**現在のURLパス**（`/books` など）」を取得するNext.jsのHookです。ページが変わるとこの値が変わります。
- `isVisible` は「いま表示状態（不透明・元位置）か」を表すアニメ用フラグ。最初は `false`（透明・少し下）です。
- `displayChildren` は「**今画面に出している中身**」。新しいページに切り替わる瞬間を制御するために、子要素を直接描かず一度stateに入れています。

> **用語: usePathname** … 現在のURLのパス部分を返すNext.jsのHook。これが変わったらページ遷移が起きたと判断できる。

---

##### 解説2: 遷移ごとにアニメをやり直す（`useEffect` + `requestAnimationFrame`）

```typescript
useEffect(() => {
  setIsVisible(false);
  setDisplayChildren(children);
  const timer = requestAnimationFrame(() => {
    setIsVisible(true);
  });
  return () => cancelAnimationFrame(timer);
}, [pathname, children]);
```

- `useEffect` の依存配列 `[pathname, children]` により、**ページ（パス）や中身が変わるたび**にこの中が実行されます。
- まず `setIsVisible(false)` で「透明・少し下」に戻し、`requestAnimationFrame(...)` で**次の描画タイミング**に `setIsVisible(true)` を予約します。一度「非表示」を描いてから「表示」に切り替えるので、アニメが毎回確実に発火します。
- `return () => cancelAnimationFrame(timer)` は後片付け（クリーンアップ）で、次の実行前や画面から消える時に予約を取り消します。

> **用語: requestAnimationFrame** … 「ブラウザの次の描画の直前に1回実行して」と予約するブラウザAPI。状態を確実に1コマ描かせてからアニメを始めたいときに使う。

---

##### 解説3: フラグで見た目を切り替える（transition クラス）

```typescript
<div
  className={`
    transition-all duration-300 ease-out
    ${
      isVisible
        ? "translate-y-0 opacity-100"
        : "translate-y-2 opacity-0"
    }
  `}
>
  {displayChildren}
</div>
```

- `transition-all duration-300 ease-out` は「全プロパティを300msかけて、最後ゆっくり（ease-out）で変化させる」指定です。
- `isVisible` が `true` なら `translate-y-0 opacity-100`（元位置・不透明）、`false` なら `translate-y-2 opacity-0`（8px下・透明）。この切り替わりに `transition` がかかるので**ふわっと上昇しながら現れます**。
- 中身は `children` ではなく `displayChildren` を描き、解説2のタイミング制御と組み合わせています。

> **用語: transition（トランジション）** … 状態Aから状態Bへ変わるときの途中を滑らかにつなぐCSS機能。`duration` で時間、`ease-out` で速度カーブを指定する。

---

> **▼ このコードがやること（先に日本語で）:** 先ほど作った `PageTransition`（ページ遷移アニメーション）を、アプリ全体で使うために「ルートレイアウト」に組み込む例です。ルートレイアウトは全ページ共通の `<html><body>` の枠組みで、ここでページ本文を `PageTransition` で包むと、すべてのページ移動に自動でアニメーションが付きます。トーストの仕組みもここで全体に適用します。

```typescript
// app/layout.tsx での使用例
// ↑ Next.js App Router のルートレイアウト。
//   全ページ共通の <html><body> 構造と、ヘッダー・フッターなどを定義する。

import { PageTransition } from "@/components/PageTransition";
import { ToastProvider } from "@/contexts/ToastContext";
import { ToastContainer } from "@/components/Toast";
import { Header } from "@/components/Header";
import { Footer } from "@/components/Footer";
import "./globals.css";   // グローバル CSS は必ずここで import

export default function RootLayout({
  children,                  // 各ページの内容がここに入る
}: {
  children: React.ReactNode;
}) {
  return (
    // suppressHydrationWarning: ダークモード切替時のSSR/CSR差異警告を抑制
    <html lang="ja" suppressHydrationWarning>
      {/* flex min-h-screen flex-col:
            縦並びで最低でも画面高さ100%を占める → フッターを下端に固定できる */}
      <body className="flex min-h-screen flex-col">
        {/* ToastProvider: 内側の全コンポーネントから useToast を使えるようにする */}
        <ToastProvider>
          <Header />
          {/* main: メインコンテンツのセマンティック要素
              container: tailwind.config の container 設定でレスポンシブ最大幅
              flex-1: 余り領域を全て使う（ヘッダーとフッターの間を埋める）
              py-8: 上下 32px の余白 */}
          <main className="container flex-1 py-8">
            <PageTransition>{children}</PageTransition>
          </main>
          <Footer />
          {/* ToastContainer はトーストを画面右上に固定表示する別ツリー */}
          <ToastContainer />
        </ToastProvider>
      </body>
    </html>
  );
}
```

### 4.4 ボタンのアニメーション

> **▼ このコードがやること（先に日本語で）:** 押した瞬間に少し縮む・処理中はスピナーを出す・色違い（バリアント）を選べる、といった機能を備えた「使い回せるボタン」を作ります。ポイントは、`variant`（primary や danger など）を props で受け取り、対応する見た目のクラスを切り替える設計です。ローディング中はボタンを無効化して二重クリックを防ぎます。

```typescript
// components/AnimatedButton.tsx
// ↑ 押した時のスケール変化、ローディング表示、バリアント切り替えを備えた汎用ボタン。

"use client";

// ButtonHTMLAttributes: <button> 要素が持つ全ての属性（onClick, type, name など）の型
// type を付けて import すると「型のみ」インポート（実行時に消える）
import { type ButtonHTMLAttributes, type ReactNode } from "react";

// props 型: <button> の全属性に独自プロパティを追加する（インターセクション型 &）
type AnimatedButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: "primary" | "secondary" | "danger" | "ghost";  // ボタンの種類
  size?: "sm" | "md" | "lg";                               // ボタンの大きさ
  isLoading?: boolean;                                      // 処理中フラグ（trueでスピナー表示）
  children: ReactNode;                                      // ボタン内のラベル
};

export function AnimatedButton({
  variant = "primary",       // 既定はプライマリ
  size = "md",               // 既定は中サイズ
  isLoading = false,         // 既定は処理していない
  children,
  className = "",            // 追加クラスを受け取れるように（既定は空）
  disabled,                  // 標準の disabled も受け取る
  ...props                   // 残りの<button>属性をまとめて受け取る（rest演算子）
}: AnimatedButtonProps) {
  // バリアントごとのスタイル（オブジェクトで管理→ variantStyles[variant] で参照）
  const variantStyles = {
    primary: `
      bg-primary-600 text-white
      hover:bg-primary-700
      active:bg-primary-800
      focus-visible:ring-primary-500
      shadow-md hover:shadow-lg
    `,
    secondary: `
      bg-secondary-600 text-white
      hover:bg-secondary-700
      active:bg-secondary-800
      focus-visible:ring-secondary-500
    `,
    danger: `
      bg-red-600 text-white
      hover:bg-red-700
      active:bg-red-800
      focus-visible:ring-red-500
    `,
    ghost: `
      text-gray-600 hover:bg-gray-100
      dark:text-gray-400 dark:hover:bg-gray-800
      focus-visible:ring-gray-500
    `,
  };

  // サイズごとのスタイル
  const sizeStyles = {
    sm: "px-3 py-1.5 text-xs gap-1.5",   /* 小: 横12px 縦6px 文字12px */
    md: "px-4 py-2.5 text-sm gap-2",     /* 中: 横16px 縦10px 文字14px */
    lg: "px-6 py-3 text-base gap-2.5",   /* 大: 横24px 縦12px 文字16px */
  };

  return (
    <button
      className={`
        inline-flex items-center justify-center
        rounded-lg font-medium
        transition-all duration-200 ease-out
        focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2
        disabled:pointer-events-none disabled:opacity-50
        active:scale-[0.97]
        ${variantStyles[variant]}
        ${sizeStyles[size]}
        ${className}
      `}
      /* active:scale-[0.97] : クリック中（押している瞬間）に97%サイズに縮める → 押下感の演出 */
      /* 任意値構文 [ ] でTailwindにないサイズを直接指定できる */
      disabled={disabled || isLoading}   /* 処理中も無効化 */
      {...props}                          /* onClick等の追加属性をスプレッドで渡す */
    >
      {/* isLoading が true の時だけスピナーを表示 */}
      {isLoading && (
        <svg
          className="h-4 w-4 animate-spin"   /* animate-spin: 1秒ループ回転 */
          viewBox="0 0 24 24"
          fill="none"
          aria-hidden="true"
        >
          {/* 円弧（薄い背景の円） */}
          <circle
            className="opacity-25"   /* 透明度25%（薄く） */
            cx="12"                  /* 中心 x座標 */
            cy="12"                  /* 中心 y座標 */
            r="10"                   /* 半径10 */
            stroke="currentColor"
            strokeWidth="4"
          />
          {/* 回転する濃い部分（パス） */}
          <path
            className="opacity-75"   /* 透明度75%（濃いめ） */
            fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
          />
        </svg>
      )}
      {children}
    </button>
  );
}
```

#### ▼ コードを1つずつ分解して解説

---

##### 解説1: `<button>` の全属性に独自propsを足す（インターセクション型）

```typescript
type AnimatedButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: "primary" | "secondary" | "danger" | "ghost";
  size?: "sm" | "md" | "lg";
  isLoading?: boolean;
  children: ReactNode;
};
```

- `ButtonHTMLAttributes<HTMLButtonElement>` は「`<button>` が本来持つ全属性（`onClick`・`type`・`disabled` など）の型」です。
- `&`（インターセクション型）で、それに `variant`・`size`・`isLoading` という**独自プロパティを足し合わせて**います。
- これにより、標準のボタン属性も独自のオプションも両方受け取れる、使い勝手のよいボタンになります。

> **用語: インターセクション型（`A & B`）** … 「Aの性質もBの性質も両方持つ」型。既存の型に独自プロパティを追加するときに使う。

---

##### 解説2: 残りの属性をまとめて受け取って渡す（rest と spread）

```typescript
export function AnimatedButton({
  variant = "primary",
  size = "md",
  isLoading = false,
  children,
  className = "",
  disabled,
  ...props
}: AnimatedButtonProps) {
  return (
    <button
      // ... className ...
      disabled={disabled || isLoading}
      {...props}
    >
```

- `...props`（rest演算子）は「分割代入で名前を付けなかった**残りの全属性**」をまとめて受け取ります（`onClick` など）。
- `{...props}`（spread構文）でそれらを `<button>` に**そのまま展開して渡します**。これで親が渡した `onClick` 等がボタンに反映されます。
- `disabled={disabled || isLoading}` は「明示的に無効化されている、**または処理中**なら無効化」。二重クリック（多重送信）を防ぐ工夫です。

> **用語: rest演算子 / spread構文（`...`）** … 同じ `...` 記号でも、受け取る側では「残りをまとめる（rest）」、渡す側では「展開する（spread）」働きをする。

---

##### 解説3: variant/size でスタイルを切り替える

```typescript
const variantStyles = {
  primary: `bg-primary-600 text-white hover:bg-primary-700 ...`,
  danger: `bg-red-600 text-white hover:bg-red-700 ...`,
  // ...
};
const sizeStyles = {
  sm: "px-3 py-1.5 text-xs gap-1.5",
  md: "px-4 py-2.5 text-sm gap-2",
  lg: "px-6 py-3 text-base gap-2.5",
};
// className 内で:
//   ${variantStyles[variant]}
//   ${sizeStyles[size]}
```

- `variantStyles` は色違い（primary/secondary/danger/ghost）、`sizeStyles` は大きさ（sm/md/lg）の**クラスをまとめたオブジェクト**です。
- `variantStyles[variant]` のように**propsの値をキーにして**該当スタイルを取り出し、className に差し込みます。
- `active:scale-[0.97]` も付いていて、「クリック中（押している瞬間）だけ97%に縮む」押下感の演出になります。

> **用語: ルックアップオブジェクト（lookup object）** … `スタイル[キー]` のようにキーで値を引くためのオブジェクト。長い `if`/`switch` の代わりに使えて見通しがよい。

---

##### 解説4: 処理中だけスピナーを出す

```typescript
{isLoading && (
  <svg className="h-4 w-4 animate-spin" ...>
    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
  </svg>
)}
```

- `{isLoading && (...)}` は「`isLoading` が `true` のときだけ」スピナーを描く短絡評価です。
- `animate-spin` は「1秒で1回転を無限に繰り返す」アニメ。薄い円（`opacity-25`）の上に濃い円弧（`opacity-75`）を重ねて、**回っているように見せる**定番の作りです。
- これと解説2の `disabled` を合わせて、「処理中はボタンを押せず、くるくる回って待たせる」UXを実現しています。

> **用語: スピナー（spinner）／animate-spin** … 処理中を示すくるくる回るアイコン。`animate-spin` はTailwindの回転アニメーションクラス。

---

## 5. ダークモード（発展）

### 5.1 Tailwind のダークモード設定

`tailwind.config.ts` で `darkMode: "class"` を設定済みなので、`<html>` タグに `dark` クラスを付与することでダークモードが有効になります。

ダークモード実装には大きく2つのパターンがあります。

- **「class」方式**: `<html class="dark">` のように、ユーザーの選択で手動切替する方式。本アプリで採用。
- **「media」方式**: OS の設定（`prefers-color-scheme`）に自動連動する方式。シンプルだがユーザーが選べない。

```html
<!-- ライトモード（dark クラスなし） -->
<html lang="ja">

<!-- ダークモード（dark クラスあり） -->
<!-- このクラスが付くと、すべての dark:〜 のスタイルが有効化される -->
<html lang="ja" class="dark">
```

### 5.2 dark: クラスの使い方

Tailwind の `dark:` バリアントを使うと、ダークモード時に適用されるスタイルを簡単に記述できます。

> **▼ このコードがやること（先に日本語で）:** ダークモード専用の見た目を指定する `dark:` 接頭辞の使い方を、いくつかのパターンで示します。基本は「通常のクラス」の後ろに `dark:〜` を並べるだけで、ダークモードのときだけそちらが適用されます。`dark:hover:〜` のようにホバーやフォーカスと組み合わせて書けるのもポイントです。実際の組み合わせ例を見て感覚をつかんでください。

```tsx
{/* 基本パターン: 通常のクラスの後に dark: プレフィックスを付ける */}
{/* bg-white: 通常は白背景 / dark:bg-gray-900: ダーク時は濃灰背景 */}
<div className="bg-white dark:bg-gray-900">
  {/* text-gray-900: 通常は濃灰文字 / dark:text-gray-100: ダーク時は薄灰文字 */}
  <h1 className="text-gray-900 dark:text-gray-100">タイトル</h1>
  {/* 補足テキスト: 通常は中間グレー / ダーク時は薄め */}
  <p className="text-gray-600 dark:text-gray-400">本文テキスト</p>
</div>

{/* ボーダーの例: 通常は薄い枠 / ダーク時は濃い枠 */}
<div className="border border-gray-200 dark:border-gray-700">
  ...
</div>

{/* ホバーとの組み合わせ（dark: と hover: は並べて書ける） */}
<button className="
  bg-blue-500 hover:bg-blue-600
  dark:bg-blue-600 dark:hover:bg-blue-700
">
  {/* bg-blue-500           : 通常背景 */}
  {/* hover:bg-blue-600     : ホバー時に濃く */}
  {/* dark:bg-blue-600      : ダーク時の背景 */}
  {/* dark:hover:bg-blue-700: ダーク × ホバー時 */}
  ボタン
</button>

{/* リングとフォーカスとの組み合わせ */}
<input className="
  focus:ring-blue-500
  dark:focus:ring-blue-400
" />
{/* focus:ring-blue-500       : フォーカス時のリング色 */}
{/* dark:focus:ring-blue-400  : ダーク × フォーカス時のリング色 */}
```

> **ダークモードの実装パターンまとめ：**
>
> 1. **CSS変数を切り替える方式**（本アプリで採用）: `:root` に変数を定義し、`.dark` で値を上書き。コンポーネント側は `bg-[var(--color-card)]` のように変数を参照するだけで自動切替。
> 2. **`dark:` クラスを毎回書く方式**: コンポーネントの各クラスに `dark:` 付きを並べる。直感的だが冗長になる。
> 3. **両者の併用**: 本アプリも基本色は CSS 変数、細かい補正は `dark:` で行うハイブリッド方式。

### 5.3 テーマ切り替えボタンの実装

> **▼ このコードがやること（先に日本語で）:** ライト・ダーク・システム連動を切り替えるボタンを作ります。カギは、選んだテーマに応じて `<html>` タグに `dark` クラスを付け外しすることと、その選択を `localStorage`（ブラウザに残る保存場所）に覚えさせて次回も同じ見た目で開けるようにする点です。これで globals.css の CSS 変数が一斉に切り替わり、画面全体の色が変わります。

```typescript
// components/ThemeToggle.tsx
// ↑ テーマ（ライト/ダーク/システム連動）を切り替えるボタンコンポーネント。

"use client";

import { useState, useEffect } from "react";

// Theme 型: 3つのテーマ候補
type Theme = "light" | "dark" | "system";

export function ThemeToggle() {
  // theme: 現在のユーザー選択（初期値は system）
  const [theme, setTheme] = useState<Theme>("system");
  // mounted: コンポーネントがマウント済みかどうか
  // → SSR（サーバーサイドレンダリング）とCSR（クライアント）で
  //   初期描画が異なるとReactが警告を出すので、それを避けるための仕組み
  const [mounted, setMounted] = useState(false);

  // コンポーネントがマウントされた後にテーマを読み込む
  // （SSR 時にミスマッチ＝ハイドレーションエラーが起きるのを防ぐため）
  useEffect(() => {
    setMounted(true);   // クライアントで初めて true になる

    // localStorage: ブラウザに永続的に値を保存する仕組み（タブを閉じても消えない）
    // as Theme | null: TypeScript の型アサーション。null も許容
    const savedTheme = localStorage.getItem("theme") as Theme | null;
    if (savedTheme) {
      setTheme(savedTheme);
      applyTheme(savedTheme);
    } else {
      applyTheme("system");
    }
  }, []);   // [] で初回マウント時のみ実行

  // システムのカラースキーム変更を監視
  // ユーザーが OS の設定でダーク↔ライトを切り替えたら自動連動する
  useEffect(() => {
    if (theme !== "system") return;   // system モード以外なら監視不要

    // matchMedia: メディアクエリをJSから扱う API
    // (prefers-color-scheme: dark): OSがダークモード設定かどうか
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    const handleChange = () => applyTheme("system");

    // OS設定変更時に handleChange を呼ぶ
    mediaQuery.addEventListener("change", handleChange);
    // クリーンアップ: コンポーネントがアンマウントされる時にリスナー解除
    return () => mediaQuery.removeEventListener("change", handleChange);
  }, [theme]);   // theme が変わるたびに再実行

  // テーマを実際にDOMに適用する関数
  const applyTheme = (newTheme: Theme) => {
    const root = document.documentElement;   // <html> 要素を取得
    // isDark: 実際にダークモードにすべきかの判定
    // - "dark" 選択時 → true
    // - "system" 選択時 → OS設定に応じて
    const isDark =
      newTheme === "dark" ||
      (newTheme === "system" &&
        window.matchMedia("(prefers-color-scheme: dark)").matches);

    // <html> の class に "dark" を付けたり外したり
    if (isDark) {
      root.classList.add("dark");
    } else {
      root.classList.remove("dark");
    }
  };

  // テーマを切り替えるボタンハンドラ
  // クリックするたび light → dark → system → light → ... と循環
  const toggleTheme = () => {
    const nextTheme: Theme =
      theme === "light" ? "dark" : theme === "dark" ? "system" : "light";

    setTheme(nextTheme);                            // state 更新
    localStorage.setItem("theme", nextTheme);       // 永続化
    applyTheme(nextTheme);                          // DOM 反映
  };

  // マウント前はプレースホルダーを表示（ハイドレーションエラー防止）
  if (!mounted) {
    return (
      <button
        className="
          inline-flex h-10 w-10 items-center justify-center
          rounded-lg
          text-gray-500
        "
        aria-label="テーマ切り替え"
      >
        <div className="h-5 w-5" />
        {/* 中身は空（プレースホルダー） */}
      </button>
    );
  }

  // テーマに応じたアイコン
  const themeIcon = () => {
    switch (theme) {
      case "light":
        return (
          <svg
            className="h-5 w-5"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={2}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M12 3v2.25m6.364.386l-1.591 1.591M21 12h-2.25m-.386 6.364l-1.591-1.591M12 18.75V21m-4.773-4.227l-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z"
            />
          </svg>
        );
      case "dark":
        return (
          <svg
            className="h-5 w-5"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={2}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M21.752 15.002A9.718 9.718 0 0118 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 003 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 009.002-5.998z"
            />
          </svg>
        );
      case "system":
        return (
          <svg
            className="h-5 w-5"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={2}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M9 17.25v1.007a3 3 0 01-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0115 18.257V17.25m6-12V15a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 15V5.25m18 0A2.25 2.25 0 0018.75 3H5.25A2.25 2.25 0 003 5.25m18 0V12a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 12V5.25"
            />
          </svg>
        );
    }
  };

  // テーマのラベル（画面表示と aria-label に使う）
  const themeLabel =
    theme === "light" ? "ライト" : theme === "dark" ? "ダーク" : "システム";

  return (
    <button
      onClick={toggleTheme}
      className="
        inline-flex h-10 items-center justify-center gap-2
        rounded-lg px-3
        text-gray-600
        transition-colors duration-200
        hover:bg-gray-100 hover:text-gray-900
        dark:text-gray-400
        dark:hover:bg-gray-800 dark:hover:text-gray-100
        focus-visible:outline-none focus-visible:ring-2
        focus-visible:ring-primary-500 focus-visible:ring-offset-2
      "
      /* ▼ クラスの読み解き ▼
         inline-flex h-10 ...     : 40px高さの横並びボタン
         gap-2                    : アイコンとラベルの間 8px
         rounded-lg px-3          : 角丸 + 左右パディング 12px
         transition-colors        : 色変化をトランジション
         hover/dark バリアントの組み合わせで多状態対応
      */
      aria-label={`テーマ切り替え（現在: ${themeLabel}モード）`}
      title={`${themeLabel}モード`}    /* マウスを乗せたときのツールチップ */
    >
      {themeIcon()}
      {/* ラベルは小画面で隠す（hidden）、640px以上で inline 表示 */}
      <span className="hidden text-sm font-medium sm:inline">
        {themeLabel}
      </span>
    </button>
  );
}
```

#### ▼ コードを1つずつ分解して解説

---

##### 解説1: 保存済みテーマを読み込む（`useEffect` + `localStorage`）

```typescript
const [theme, setTheme] = useState<Theme>("system");
const [mounted, setMounted] = useState(false);

useEffect(() => {
  setMounted(true);
  const savedTheme = localStorage.getItem("theme") as Theme | null;
  if (savedTheme) {
    setTheme(savedTheme);
    applyTheme(savedTheme);
  } else {
    applyTheme("system");
  }
}, []);
```

- `localStorage` は「**タブを閉じても消えないブラウザの保存場所**」です。`getItem("theme")` で前回選んだテーマを読み出します。
- `mounted` を `true` にするのは、SSR（サーバー描画）とCSR（ブラウザ描画）の食い違い（ハイドレーションエラー）を避けるためです。マウント後にだけ本来の表示をします。
- 依存配列 `[]` なので、この `useEffect` は**初回マウント時に1回だけ**実行されます。

> **用語: localStorage／ハイドレーション** … localStorage はブラウザに永続保存する仕組み。ハイドレーションはサーバー生成HTMLにReactが後から機能を結びつける処理で、両者の表示差は警告の原因になる。

---

##### 解説2: 実際にダークを適用する（`applyTheme`）

```typescript
const applyTheme = (newTheme: Theme) => {
  const root = document.documentElement;
  const isDark =
    newTheme === "dark" ||
    (newTheme === "system" &&
      window.matchMedia("(prefers-color-scheme: dark)").matches);
  if (isDark) {
    root.classList.add("dark");
  } else {
    root.classList.remove("dark");
  }
};
```

- `document.documentElement` は `<html>` 要素そのものです。ここに `dark` クラスを付け外しすると、globals.css のCSS変数が一斉に切り替わります。
- `isDark` の判定は「**`dark` が選ばれている、または `system` かつOSがダーク設定**」のとき `true`。`matchMedia("(prefers-color-scheme: dark)")` でOSの設定を読み取ります。
- `classList.add("dark")` / `remove("dark")` でダークモードのオン・オフを切り替えます。

> **用語: matchMedia / prefers-color-scheme** … `matchMedia` はメディアクエリをJSから判定するAPI。`prefers-color-scheme: dark` はOSがダーク表示を望んでいるかを表す。

---

##### 解説3: クリックでテーマを循環させる（`toggleTheme`）

```typescript
const toggleTheme = () => {
  const nextTheme: Theme =
    theme === "light" ? "dark" : theme === "dark" ? "system" : "light";
  setTheme(nextTheme);
  localStorage.setItem("theme", nextTheme);
  applyTheme(nextTheme);
};
```

- 三項演算子を2つつなげて「light → dark → system → light → …」と**順番に循環**させています。
- `setTheme` で画面表示を更新し、`localStorage.setItem` で選択を保存（次回も同じテーマで開ける）、`applyTheme` で実際の `<html>` に反映、の3つを行います。
- このように「状態更新・永続化・DOM反映」をまとめて行うのがテーマ切替の基本形です。

> **用語: 三項演算子のネスト** … `A ? x : B ? y : z` のように三項演算子を連ねた書き方。3つ以上の分岐を1行で表せるが、深くしすぎると読みにくくなる。

---

##### 解説4: マウント前はプレースホルダーを返す

```typescript
if (!mounted) {
  return (
    <button className="inline-flex h-10 w-10 ..." aria-label="テーマ切り替え">
      <div className="h-5 w-5" />
    </button>
  );
}
```

- `if (!mounted)` は「まだクライアントでマウントされていない（＝サーバー描画段階）」のときの分岐です。
- ここで**中身が空のボタン（プレースホルダー）**を返すことで、サーバーとブラウザで初期表示を一致させ、ハイドレーションエラーを防ぎます。
- マウント後は `mounted` が `true` になり、テーマに応じたアイコン入りのボタンが描画されます。

> **用語: プレースホルダー（placeholder）** … 本来の中身が決まる前に置いておく仮の表示。ここではレイアウトのずれや警告を避けるための空ボタンとして使う。

---

**ちらつき防止のスクリプト:**

ページ読み込み時にダークモードが一瞬ライトモードで表示されるのを防ぐため、`<head>` 内にインラインスクリプトを設置します。

> **▼ このコードがやること（先に日本語で）:** ページを開いた一瞬だけダークモードがライト表示でちらつく問題を防ぐ小さなスクリプトを、`<head>` の中に仕込みます。ポイントは、画面が描かれる「前」に保存済みのテーマ設定を読み取り、すぐ `<html>` に `dark` クラスを付けてしまうこと。React の描画を待たずに先回りするため、あえて `<head>` 内の即時実行スクリプトとして書きます。

```typescript
// app/layout.tsx の <head> 内に追加

// dangerouslySetInnerHTML: React の機能。普通は危険なので使うべきではないが、
// React のハイドレーション前に実行するインラインスクリプトを書きたい時に使う
<script
  dangerouslySetInnerHTML={{
    __html: `
      /* 即時実行関数（IIFE）: 定義と同時に呼び出す関数。グローバル変数を汚さない */
      (function() {
        try {
          /* localStorage から保存済みテーマを取得 */
          var theme = localStorage.getItem('theme');
          /* isDark の判定:
             - 明示的に 'dark' が保存されている
             - または theme 未保存 かつ OS がダーク設定 */
          var isDark = theme === 'dark' ||
            (!theme && window.matchMedia('(prefers-color-scheme: dark)').matches);
          if (isDark) {
            /* <html> に dark クラスを付与（React の描画前に間に合う） */
            document.documentElement.classList.add('dark');
          }
        } catch (e) {
          /* localStorage が使えないブラウザ等のためのフォールバック（何もしない） */
        }
      })();
    `,
  }}
/>
```

---

## 6. アクセシビリティ

### 6.1 aria 属性の追加

アクセシビリティを確保するために、各コンポーネントに適切な ARIA 属性を追加します。

**ARIA（Accessible Rich Internet Applications、エイリア）** とは、HTMLだけでは表現しきれない要素の意味や状態をスクリーンリーダー（screen reader：画面の内容を音声で読み上げるソフト）に伝えるための仕様です。`role`、`aria-label`、`aria-expanded` などの属性を使います。

> **▼ このコードがやること（先に日本語で）:** サイト上部のヘッダーに、目の見えない方が使う読み上げソフト（スクリーンリーダー）向けの情報を足します。カギは `aria-label`（要素の意味を言葉で補う）や `aria-expanded`（メニューが開いているか）などの ARIA 属性で、見た目を変えずに「ここはナビゲーション」「このボタンで開閉する」と伝えられます。どの属性が何を伝えるかはコメントを参照してください。

```typescript
// components/Header.tsx
// ↑ サイト全体のヘッダー（上部の固定ナビゲーション）。

"use client";

import { useState } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";   // 現在のURLパス取得
import { ThemeToggle } from "./ThemeToggle";

export function Header() {
  const pathname = usePathname();
  // モバイルメニューの開閉状態
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  // ナビゲーションアイテムを配列で管理。後でmapで展開する
  const navItems = [
    { href: "/", label: "ホーム" },
    { href: "/books", label: "書籍一覧" },
    { href: "/books/new", label: "新規登録" },
  ];

  return (
    <header
      className="
        sticky top-0 z-40
        border-b border-[var(--color-border)]
        glass
      "
      /* ▼ クラスの読み解き ▼
         sticky top-0    : スクロールしても上端に貼り付く
         z-40            : z-index 40（他要素より前面、トーストの50よりは後ろ）
         border-b        : 下側だけ枠線
         glass           : すりガラス効果（globals.css 定義）
      */
      role="banner"   /* ARIA: サイトのバナー領域（ヘッダー）であることを伝える */
    >
      <div className="container flex h-16 items-center justify-between">
        {/* container: tailwind.config の container 設定
            flex h-16 items-center justify-between:
              横並び、高さ64px、縦中央、両端寄せ */}

        {/* ロゴ */}
        <Link
          href="/"
          className="
            flex items-center gap-2
            text-xl font-bold
            text-[var(--color-foreground)]
            transition-colors hover:text-primary-600
            dark:hover:text-primary-400
          "
          aria-label="BookShelf ホームへ"   /* スクリーンリーダー用 */
        >
          <svg
            className="h-7 w-7 text-primary-600 dark:text-primary-400"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            strokeWidth={2}
            aria-hidden="true"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M12 6.042A8.967 8.967 0 006 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 016 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 016-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0018 18a8.967 8.967 0 00-6 2.292m0-14.25v14.25"
            />
          </svg>
          BookShelf
        </Link>

        {/* デスクトップナビゲーション */}
        <nav
          className="hidden items-center gap-1 md:flex"
          /* hidden    : 通常は非表示（モバイル時） */
          /* md:flex   : 768px以上で flex 表示に切替 */
          role="navigation"
          aria-label="メインナビゲーション"
        >
          {navItems.map((item) => {
            // 現在のパスと一致するかを判定
            const isActive = pathname === item.href;
            return (
              <Link
                key={item.href}
                href={item.href}
                className={`
                  rounded-lg px-3 py-2 text-sm font-medium
                  transition-colors duration-200
                  ${
                    isActive
                      ? "bg-primary-50 text-primary-700 dark:bg-primary-950 dark:text-primary-300"
                      : "text-gray-600 hover:bg-gray-100 hover:text-gray-900 dark:text-gray-400 dark:hover:bg-gray-800 dark:hover:text-gray-100"
                  }
                `}
                /* isActive ? 強調表示 : 通常表示 で切り替え */
                aria-current={isActive ? "page" : undefined}
                /* aria-current="page": スクリーンリーダーに「これが現在のページ」と伝える */
              >
                {item.label}
              </Link>
            );
          })}
        </nav>

        {/* 右側のアクション */}
        <div className="flex items-center gap-2">
          <ThemeToggle />

          {/* モバイルメニューボタン（ハンバーガー） */}
          <button
            className="
              inline-flex h-10 w-10 items-center justify-center
              rounded-lg
              text-gray-600
              transition-colors
              hover:bg-gray-100
              dark:text-gray-400 dark:hover:bg-gray-800
              md:hidden
            "
            /* md:hidden: 768px以上で非表示（モバイル専用ボタン） */
            onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
            aria-expanded={isMobileMenuOpen}      /* メニューが展開中かをスクリーンリーダーに通知 */
            aria-controls="mobile-menu"           /* このボタンが操作する対象要素のid */
            aria-label={isMobileMenuOpen ? "メニューを閉じる" : "メニューを開く"}
          >
            {isMobileMenuOpen ? (
              <svg
                className="h-6 w-6"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                strokeWidth={2}
                aria-hidden="true"
              >
                <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
              </svg>
            ) : (
              <svg
                className="h-6 w-6"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                strokeWidth={2}
                aria-hidden="true"
              >
                <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
              </svg>
            )}
          </button>
        </div>
      </div>

      {/* モバイルメニュー */}
      {isMobileMenuOpen && (
        <nav
          id="mobile-menu"
          className="
            border-t border-[var(--color-border)]
            bg-[var(--color-background)]
            p-4
            md:hidden
          "
          role="navigation"
          aria-label="モバイルナビゲーション"
        >
          <ul className="flex flex-col gap-1" role="list">
            {navItems.map((item) => {
              const isActive = pathname === item.href;
              return (
                <li key={item.href}>
                  <Link
                    href={item.href}
                    className={`
                      block rounded-lg px-4 py-3 text-sm font-medium
                      transition-colors duration-200
                      ${
                        isActive
                          ? "bg-primary-50 text-primary-700 dark:bg-primary-950 dark:text-primary-300"
                          : "text-gray-600 hover:bg-gray-50 dark:text-gray-400 dark:hover:bg-gray-800"
                      }
                    `}
                    aria-current={isActive ? "page" : undefined}
                    onClick={() => setIsMobileMenuOpen(false)}
                  >
                    {item.label}
                  </Link>
                </li>
              );
            })}
          </ul>
        </nav>
      )}
    </header>
  );
}
```

#### ▼ コードを1つずつ分解して解説

---

##### 解説1: ヘッダーの領域を読み上げソフトに伝える（`role="banner"`）

```typescript
<header
  className="sticky top-0 z-40 border-b border-[var(--color-border)] glass"
  role="banner"
>
```

- `sticky top-0` は「スクロールしても**画面上端に貼り付く**」配置。`z-40` で他の要素より前面（ただしトーストの `z-50` よりは後ろ）に置きます。
- `glass` は globals.css のすりガラス効果（半透明＋背景ぼかし）です。
- `role="banner"` は「ここはサイトのヘッダー領域だ」とスクリーンリーダーに伝えるARIAロール。見た目は変わらず、意味だけを補います。

> **用語: ARIA role（banner）** … 要素の役割を読み上げソフトに伝える属性。`banner` はページ上部のヘッダー（ロゴやナビ）領域を表す。

---

##### 解説2: ナビ配列を map し、現在地を強調（`aria-current`）

```typescript
{navItems.map((item) => {
  const isActive = pathname === item.href;
  return (
    <Link
      key={item.href}
      href={item.href}
      className={`rounded-lg px-3 py-2 ... ${
        isActive
          ? "bg-primary-50 text-primary-700 dark:bg-primary-950 dark:text-primary-300"
          : "text-gray-600 hover:bg-gray-100 ..."
      }`}
      aria-current={isActive ? "page" : undefined}
    >
      {item.label}
    </Link>
  );
})}
```

- `navItems` 配列を `map` で展開し、各リンクを作ります。`isActive = pathname === item.href` で「今いるページか」を判定します。
- 三項演算子で、現在ページのリンクだけ**背景色付きで強調**し、他は通常色＋ホバー時の薄い背景にしています。
- `aria-current={isActive ? "page" : undefined}` は、現在ページにだけ「これが現在地」と読み上げソフトに伝えます。該当しなければ `undefined`（属性を付けない）にします。

> **用語: aria-current="page"** … 同種リンク群の中で「今いるページ」を示すARIA属性。視覚的な強調と合わせて使うとアクセシブルになる。

---

##### 解説3: ハンバーガーボタンの開閉状態を伝える（`aria-expanded` / `aria-controls`）

```typescript
<button
  onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
  aria-expanded={isMobileMenuOpen}
  aria-controls="mobile-menu"
  aria-label={isMobileMenuOpen ? "メニューを閉じる" : "メニューを開く"}
>
```

- `onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}` は「**現在の開閉状態を反転**」させる処理。`!` で true↔false を切り替えます。
- `aria-expanded={isMobileMenuOpen}` は「メニューが今**開いているか閉じているか**」を読み上げソフトに伝えます。`aria-controls="mobile-menu"` は「このボタンが操作する対象のid」を示します。
- `aria-label` も開閉状態で文言を変え、ボタンの目的を明確にしています。

> **用語: aria-expanded / aria-controls** … `aria-expanded` は開閉式の要素が今開いているかを示す。`aria-controls` はそのボタンが制御する要素のidを結びつける。

---

##### 解説4: モバイルメニューを条件付きで表示

```typescript
{isMobileMenuOpen && (
  <nav id="mobile-menu" className="border-t ... md:hidden" role="navigation" aria-label="モバイルナビゲーション">
    <ul className="flex flex-col gap-1" role="list">
      {navItems.map((item) => (
        // ... 各リンク。onClick で setIsMobileMenuOpen(false)
      ))}
    </ul>
  </nav>
)}
```

- `{isMobileMenuOpen && (...)}` で「開いているときだけ」メニューを描画します（短絡評価）。
- `id="mobile-menu"` は解説3の `aria-controls` と対応し、`md:hidden` で「768px以上（PC）では非表示」にします。モバイル専用のメニューです。
- 各リンクの `onClick` で `setIsMobileMenuOpen(false)` を呼び、**リンクを押したらメニューが閉じる**ようにしています。

> **用語: 条件付きレンダリング（`条件 && JSX`）** … 条件が真のときだけ要素を描く書き方。開いているときだけメニューを出すなどの出し分けに使う。

---

### 6.2 キーボードナビゲーション

**キーボードナビゲーション（keyboard navigation：マウスを使わずキーボードだけで操作できるようにすること）** は、視覚障害のあるユーザーや、マウスが使いづらい状況のユーザーにとって必須の機能です。

主な操作:

- **Tab キー**: 次のフォーカス可能要素へ移動
- **Shift + Tab**: 前のフォーカス可能要素へ移動
- **Enter / Space**: 選択中の要素を実行（クリック相当）
- **Escape**: モーダルやポップアップを閉じる
- **矢印キー**: メニュー内の項目選択（必要に応じて自分で実装）

**フォーカストラップ（focus trap：フォーカスを特定範囲内に閉じ込める仕組み）** は、モーダル表示中に Tab キーで背景の要素にフォーカスが飛んでしまうのを防ぐテクニックです。

> **▼ このコードがやること（先に日本語で）:** マウスを使わずキーボードだけで操作できるようにする対応を、カードに足します。ポイントは、Tab キーで要素を順にフォーカスでき、Enter/Space で実行、Escape で閉じる、という基本操作に対応すること。さらにモーダル表示中はフォーカスを枠内に閉じ込める「フォーカストラップ」で、背景に迷子にならないようにします。

```typescript
// components/BookCard.tsx に追加するキーボード対応

// カードが Link で囲まれている場合、Enter/Space で遷移が可能（Link のデフォルト動作）
// それ以外のインタラクティブ要素にも対応する

// 例: 削除確認ダイアログのキーボード対応
// components/ConfirmDialog.tsx
// ↑ モーダル（modal：背景を暗くして他操作を遮るダイアログ）型の確認ボックス。
//   削除など重要操作の前にユーザーに最終確認を求める用途。

"use client";

import { useEffect, useRef, type ReactNode } from "react";

type ConfirmDialogProps = {
  isOpen: boolean;              // ダイアログを表示するかどうか
  title: string;                // タイトル
  message: string;              // 本文メッセージ
  confirmLabel?: string;        // 確認ボタンの文字（既定: "確認"）
  cancelLabel?: string;         // キャンセルボタンの文字（既定: "キャンセル"）
  variant?: "danger" | "primary"; // 確認ボタンの色
  onConfirm: () => void;        // 確認時のコールバック
  onCancel: () => void;         // キャンセル時のコールバック
};

export function ConfirmDialog({
  isOpen,
  title,
  message,
  confirmLabel = "確認",
  cancelLabel = "キャンセル",
  variant = "primary",
  onConfirm,
  onCancel,
}: ConfirmDialogProps) {
  // useRef: DOM要素への参照を保持する Hook
  // .current で実際の DOM ノードにアクセスできる
  const dialogRef = useRef<HTMLDivElement>(null);            // ダイアログ本体への参照
  const cancelButtonRef = useRef<HTMLButtonElement>(null);   // キャンセルボタンへの参照

  // ダイアログが開いたらキャンセルボタンにフォーカス
  // ※ 確認ボタンではなくキャンセルに当てるのは「誤操作で確定しない」ための配慮
  useEffect(() => {
    if (isOpen) {
      cancelButtonRef.current?.focus();   // ?. は null/undefined チェック付きアクセス
    }
  }, [isOpen]);

  // Escape キーで閉じる + Tab キーでのフォーカストラップ（ダイアログ内に閉じ込める）
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!isOpen) return;

      // Escape キーでキャンセル
      if (e.key === "Escape") {
        onCancel();
      }

      // Tab キーのフォーカストラップ
      // → Tab/Shift+Tab を押してもダイアログの外に出ないようにする
      if (e.key === "Tab" && dialogRef.current) {
        // ダイアログ内でフォーカス可能な要素を全て取得
        // querySelectorAll: CSS セレクタで子要素を検索する API
        const focusableElements = dialogRef.current.querySelectorAll<HTMLElement>(
          'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
        );
        const firstElement = focusableElements[0];                          // 最初
        const lastElement = focusableElements[focusableElements.length - 1]; // 最後

        if (e.shiftKey) {
          // Shift+Tab: 逆方向に移動
          // 最初の要素でさらに戻ろうとしたら、最後の要素にラップ
          if (document.activeElement === firstElement) {
            e.preventDefault();
            lastElement.focus();
          }
        } else {
          // Tab: 順方向
          // 最後の要素でさらに進もうとしたら、最初の要素にラップ
          if (document.activeElement === lastElement) {
            e.preventDefault();
            firstElement.focus();
          }
        }
      }
    };

    // document 全体で keydown を監視
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [isOpen, onCancel]);

  // body のスクロールを無効化
  // モーダル表示中に背景をスクロールできてしまうと混乱するため
  useEffect(() => {
    if (isOpen) {
      document.body.style.overflow = "hidden";   // スクロール禁止
    } else {
      document.body.style.overflow = "";          // 元に戻す
    }
    return () => {
      document.body.style.overflow = "";          // クリーンアップでも復元
    };
  }, [isOpen]);

  // 閉じている時は何も描画しない
  if (!isOpen) return null;

  return (
    <div
      className="
        fixed inset-0 z-50
        flex items-center justify-center
        p-4
      "
      /* fixed inset-0 : 画面全体に固定（top:0 right:0 bottom:0 left:0）
         z-50         : 最前面
         flex ... center: 中身を縦横中央に
         p-4          : 画面端からの余白 */
      role="dialog"
      aria-modal="true"                            /* 背景操作を遮るモーダルであると伝える */
      aria-labelledby="dialog-title"               /* タイトルとなる要素のid */
      aria-describedby="dialog-description"        /* 説明文となる要素のid */
    >
      {/* オーバーレイ（背景を暗くする半透明の幕） */}
      <div
        className="
          absolute inset-0
          bg-black/50
          animate-fade-in
          backdrop-blur-sm
        "
        /* bg-black/50      : 黒の透明度50%
           backdrop-blur-sm : 背景を少しぼかす */
        onClick={onCancel}    /* オーバーレイクリックでキャンセル */
        aria-hidden="true"    /* スクリーンリーダーから隠す（装飾要素） */
      />

      {/* ダイアログ本体 */}
      <div
        ref={dialogRef}                            /* DOMへの参照を取得 */
        className="
          relative
          w-full max-w-md
          animate-scale-in
          rounded-xl
          bg-[var(--color-card)]
          p-6
          shadow-xl
        "
        /* relative                  : オーバーレイより手前に重ねる
           w-full max-w-md           : 幅100%（ただし最大448px）
           animate-scale-in          : 拡大しながら表示
           rounded-xl                : 大きめ角丸
           bg-[var(--color-card)]    : カード背景色（ダーク対応） */
      >
        <h2
          id="dialog-title"                        /* aria-labelledby で参照される */
          className="text-lg font-semibold text-[var(--color-foreground)]"
        >
          {title}
        </h2>

        <p
          id="dialog-description"                  /* aria-describedby で参照される */
          className="mt-2 text-sm text-[var(--color-muted)]"
        >
          {message}
        </p>

        <div className="mt-6 flex justify-end gap-3">
          {/* mt-6: 上マージン 24px / justify-end: 右寄せ / gap-3: ボタン間 12px */}
          <button
            ref={cancelButtonRef}                  /* ボタンへの参照（初期フォーカス用） */
            onClick={onCancel}
            className="btn-ghost"                  /* 控えめなボタン */
          >
            {cancelLabel}
          </button>

          <button
            onClick={onConfirm}
            /* variant に応じて btn-danger（赤）または btn-primary（インディゴ） */
            className={variant === "danger" ? "btn-danger" : "btn-primary"}
          >
            {confirmLabel}
          </button>
        </div>
      </div>
    </div>
  );
}
```

#### ▼ コードを1つずつ分解して解説

---

##### 解説1: DOM要素への参照を持つ（`useRef`）

```typescript
const dialogRef = useRef<HTMLDivElement>(null);
const cancelButtonRef = useRef<HTMLButtonElement>(null);

useEffect(() => {
  if (isOpen) {
    cancelButtonRef.current?.focus();
  }
}, [isOpen]);
```

- `useRef` は「**実際のDOM要素への参照**」を保持するHookです。`.current` でその要素にアクセスできます。`<div ref={dialogRef}>` のように要素に紐付けます。
- ダイアログが開いたら `cancelButtonRef.current?.focus()` で**キャンセルボタンに自動でフォーカス**を当てます。`?.` は「nullでなければ実行」の安全アクセスです。
- 確認ボタンではなくキャンセルに当てるのは、Enter連打などで**誤って確定させない**ための配慮です。

> **用語: useRef** … 再描画を起こさずに値やDOM参照を保持するReact Hook。`.current` で中身を読み書きする。

---

##### 解説2: Escapeで閉じ、Tabを閉じ込める（フォーカストラップ）

```typescript
const handleKeyDown = (e: KeyboardEvent) => {
  if (!isOpen) return;
  if (e.key === "Escape") { onCancel(); }
  if (e.key === "Tab" && dialogRef.current) {
    const focusableElements = dialogRef.current.querySelectorAll<HTMLElement>(
      'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
    );
    const firstElement = focusableElements[0];
    const lastElement = focusableElements[focusableElements.length - 1];
    if (e.shiftKey) {
      if (document.activeElement === firstElement) { e.preventDefault(); lastElement.focus(); }
    } else {
      if (document.activeElement === lastElement) { e.preventDefault(); firstElement.focus(); }
    }
  }
};
```

- `e.key === "Escape"` のとき `onCancel()` を呼び、**Escapeキーで閉じられる**ようにします。
- Tabキーのときは、`querySelectorAll` でダイアログ内の**フォーカス可能な要素を全部集め**、最初と最後を取り出します。
- 最後の要素でさらにTabを押したら最初へ、最初の要素でShift+Tabを押したら最後へ「ラップ」させます。これで**フォーカスがダイアログの外に逃げない**＝フォーカストラップが完成します。

> **用語: フォーカストラップ（focus trap）** … モーダル表示中、Tab移動の対象を枠内に閉じ込める仕組み。背景の要素に誤ってフォーカスが移るのを防ぐ。

---

##### 解説3: 背景のスクロールを止める（`document.body.style.overflow`）

```typescript
useEffect(() => {
  if (isOpen) {
    document.body.style.overflow = "hidden";
  } else {
    document.body.style.overflow = "";
  }
  return () => {
    document.body.style.overflow = "";
  };
}, [isOpen]);
```

- ダイアログが開いている間 `document.body.style.overflow = "hidden"` で**ページ全体のスクロールを禁止**します。モーダルの後ろが動くと混乱するためです。
- 閉じたら `""`（空文字）に戻して、元のスクロール可能な状態に復帰させます。
- `return () => { ... }` のクリーンアップでも復元しておくことで、コンポーネントが消えてもスクロールが固まったままにならないようにします。

> **用語: クリーンアップ関数** … `useEffect` が返す関数。次の実行前やアンマウント時に呼ばれ、イベント解除やスタイル復元などの後片付けを担う。

---

##### 解説4: モーダルの意味づけ（`role="dialog"` / `aria-modal`）

```typescript
<div
  role="dialog"
  aria-modal="true"
  aria-labelledby="dialog-title"
  aria-describedby="dialog-description"
>
  <div className="... bg-black/50 ..." onClick={onCancel} aria-hidden="true" />
  <div ref={dialogRef} className="relative ...">
    <h2 id="dialog-title">{title}</h2>
    <p id="dialog-description">{message}</p>
    {/* ... ボタン ... */}
  </div>
</div>
```

- `role="dialog"` と `aria-modal="true"` で「これは**背景操作を遮るモーダルダイアログ**だ」とスクリーンリーダーに伝えます。
- `aria-labelledby="dialog-title"` / `aria-describedby="dialog-description"` は、見出し（`id="dialog-title"`）と説明文（`id="dialog-description"`）を**ダイアログの名前・説明として結びつけ**ます。
- 背景の幕（`bg-black/50`）は `onClick={onCancel}` で「外側クリックで閉じる」、`aria-hidden="true"` で読み上げ対象から除外しています。

> **用語: role="dialog" / aria-modal** … ダイアログであることと、背景が操作不可のモーダルであることを伝えるARIA。`aria-labelledby`/`describedby` で見出し・説明を関連付ける。

---

### 6.3 色のコントラスト

**WCAG（Web Content Accessibility Guidelines、ダブリュー・シー・エー・ジー：W3C が策定するWebアクセシビリティ指針）** 2.1 のコントラスト比（色の明暗差）基準を満たすための指針です。コントラストが弱いと、視覚障害のあるユーザーや明るい場所でスマホを見るユーザーが文字を読み取れなくなります。

| レベル | コントラスト比 | 対象 |
|:---:|:---:|:---|
| AA（通常テキスト） | 4.5:1 以上 | 本文、ラベル、プレースホルダ |
| AA（大きいテキスト） | 3:1 以上 | 見出し（18px以上、または14px太字以上） |
| AAA（通常テキスト） | 7:1 以上 | 最高レベルの可読性が求められる場面 |

本アプリで採用しているカラーのコントラスト比:

| 組み合わせ | ライトモード | ダークモード | 基準 |
|:---|:---:|:---:|:---:|
| メインテキスト / 背景 | 15.4:1 (#0f172a / #ffffff) | 16.0:1 (#f8fafc / #0f172a) | AA |
| ミュートテキスト / 背景 | 5.5:1 (#64748b / #ffffff) | 5.7:1 (#94a3b8 / #0f172a) | AA |
| プライマリボタン文字 / ボタン背景 | 8.6:1 (#ffffff / #4f46e5) | -- | AA |
| エラーテキスト / エラー背景 | 4.6:1 (#991b1b / #fef2f2) | 5.1:1 | AA |

**確認ツール:**
- Chrome DevTools: Elements パネルの Color Contrast 表示
- [WebAIM Contrast Checker](https://webaim.org/resources/contrastchecker/)
- axe DevTools ブラウザ拡張

---

## 7. 最終的な画面の説明

### 7.1 トップページ（書籍一覧）

<div style="max-width: 700px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #cbd5e1; border-radius: 12px; overflow: hidden; box-shadow: 0 4px 20px rgba(0,0,0,0.08); background: #f8fafc;">
  <!-- Header -->
  <div style="background: linear-gradient(135deg, #1e40af 0%, #3b82f6 100%); color: white; padding: 12px 20px; display: flex; justify-content: space-between; align-items: center;">
    <div style="font-weight: 700; font-size: 15px;">📖 BookShelf</div>
    <div style="display: flex; gap: 16px; font-size: 12px;">
      <span style="opacity: 0.9;">ホーム</span>
      <span style="border-bottom: 2px solid white; padding-bottom: 2px;">書籍一覧</span>
      <span style="opacity: 0.9;">新規登録</span>
      <span style="background: rgba(255,255,255,0.2); border-radius: 50%; width: 24px; height: 24px; display: inline-flex; align-items: center; justify-content: center; font-size: 11px;">🌙</span>
    </div>
  </div>
  <div style="font-size: 9px; color: #64748b; text-align: right; padding: 2px 12px; background: #f1f5f9;">← ヘッダー (sticky, glass効果)</div>
  <!-- Title Section -->
  <div style="padding: 16px 20px 8px; display: flex; justify-content: space-between; align-items: center;">
    <div>
      <div style="font-size: 18px; font-weight: 700; color: #1e293b;">書籍一覧</div>
      <div style="font-size: 11px; color: #64748b; margin-top: 2px;">登録されている書籍: 24冊</div>
    </div>
    <div style="background: #3b82f6; color: white; padding: 6px 14px; border-radius: 8px; font-size: 12px; font-weight: 600; cursor: pointer;">+ 新規登録</div>
  </div>
  <!-- Search & Filter -->
  <div style="padding: 8px 20px 12px;">
    <div style="background: white; border: 1px solid #e2e8f0; border-radius: 8px; padding: 8px 12px; font-size: 12px; color: #94a3b8; margin-bottom: 8px;">🔍 タイトルまたは著者名で検索...</div>
    <div style="display: flex; gap: 10px; font-size: 11px;">
      <span style="background: white; border: 1px solid #e2e8f0; border-radius: 6px; padding: 4px 10px;">ステータス: 全て ▼</span>
      <span style="background: white; border: 1px solid #e2e8f0; border-radius: 6px; padding: 4px 10px;">並び順: 新しい順 ▼</span>
    </div>
  </div>
  <div style="font-size: 9px; color: #64748b; text-align: right; padding: 0 12px 4px;">← フィルター・検索バー</div>
  <!-- Book Card Grid -->
  <div style="padding: 4px 20px 12px; display: flex; gap: 10px; flex-wrap: wrap;">
    <div style="flex: 1; min-width: 22%; background: white; border: 1px solid #e2e8f0; border-radius: 10px; overflow: hidden; box-shadow: 0 1px 4px rgba(0,0,0,0.04);">
      <div style="background: linear-gradient(135deg, #dbeafe, #bfdbfe); height: 60px; display: flex; align-items: center; justify-content: center; font-size: 24px;">📗</div>
      <div style="padding: 8px; font-size: 11px;"><strong>React入門</strong><br/><span style="color:#64748b">山田太郎</span><br/><span style="color:#f59e0b;">★★★★☆</span></div>
    </div>
    <div style="flex: 1; min-width: 22%; background: white; border: 1px solid #e2e8f0; border-radius: 10px; overflow: hidden; box-shadow: 0 1px 4px rgba(0,0,0,0.04);">
      <div style="background: linear-gradient(135deg, #dbeafe, #bfdbfe); height: 60px; display: flex; align-items: center; justify-content: center; font-size: 24px;">📘</div>
      <div style="padding: 8px; font-size: 11px;"><strong>TypeScript実践</strong><br/><span style="color:#64748b">鈴木花子</span><br/><span style="color:#f59e0b;">★★★☆☆</span></div>
    </div>
    <div style="flex: 1; min-width: 22%; background: white; border: 1px solid #e2e8f0; border-radius: 10px; overflow: hidden; box-shadow: 0 1px 4px rgba(0,0,0,0.04);">
      <div style="background: linear-gradient(135deg, #dbeafe, #bfdbfe); height: 60px; display: flex; align-items: center; justify-content: center; font-size: 24px;">📙</div>
      <div style="padding: 8px; font-size: 11px;"><strong>Next.js入門</strong><br/><span style="color:#64748b">佐藤次郎</span><br/><span style="color:#f59e0b;">★★★★★</span></div>
    </div>
  </div>
  <div style="padding: 0 20px 12px; display: flex; gap: 10px; flex-wrap: wrap;">
    <div style="flex: 1; min-width: 22%; background: white; border: 1px solid #e2e8f0; border-radius: 10px; overflow: hidden; box-shadow: 0 1px 4px rgba(0,0,0,0.04);">
      <div style="background: linear-gradient(135deg, #e0e7ff, #c7d2fe); height: 60px; display: flex; align-items: center; justify-content: center; font-size: 24px;">📕</div>
      <div style="padding: 8px; font-size: 11px;"><strong>CSS設計</strong><br/><span style="color:#64748b">田中一郎</span><br/><span style="color:#f59e0b;">★★☆☆☆</span></div>
    </div>
    <div style="flex: 1; min-width: 22%; background: white; border: 1px solid #e2e8f0; border-radius: 10px; overflow: hidden; box-shadow: 0 1px 4px rgba(0,0,0,0.04);">
      <div style="background: linear-gradient(135deg, #e0e7ff, #c7d2fe); height: 60px; display: flex; align-items: center; justify-content: center; font-size: 24px;">📓</div>
      <div style="padding: 8px; font-size: 11px;"><strong>Node.js実践</strong><br/><span style="color:#64748b">高橋三郎</span><br/><span style="color:#f59e0b;">★★★★☆</span></div>
    </div>
    <div style="flex: 1; min-width: 22%; background: white; border: 1px solid #e2e8f0; border-radius: 10px; overflow: hidden; box-shadow: 0 1px 4px rgba(0,0,0,0.04);">
      <div style="background: linear-gradient(135deg, #e0e7ff, #c7d2fe); height: 60px; display: flex; align-items: center; justify-content: center; font-size: 24px;">📔</div>
      <div style="padding: 8px; font-size: 11px;"><strong>Git入門</strong><br/><span style="color:#64748b">伊藤四郎</span><br/><span style="color:#f59e0b;">★★★☆☆</span></div>
    </div>
  </div>
  <div style="font-size: 9px; color: #64748b; text-align: right; padding: 0 12px 8px;">← 書籍カードグリッド (xl:4列, lg:3列, sm:2列, 1列)</div>
  <!-- Pagination -->
  <div style="text-align: center; padding: 8px 20px 12px; display: flex; justify-content: center; gap: 4px; align-items: center;">
    <span style="font-size: 11px; color: #3b82f6; cursor: pointer;">← 前へ</span>
    <span style="background: #3b82f6; color: white; border-radius: 6px; padding: 3px 9px; font-size: 11px; font-weight: 600;">1</span>
    <span style="background: white; border: 1px solid #e2e8f0; border-radius: 6px; padding: 3px 9px; font-size: 11px;">2</span>
    <span style="background: white; border: 1px solid #e2e8f0; border-radius: 6px; padding: 3px 9px; font-size: 11px;">3</span>
    <span style="font-size: 11px; color: #94a3b8;">...</span>
    <span style="background: white; border: 1px solid #e2e8f0; border-radius: 6px; padding: 3px 9px; font-size: 11px;">10</span>
    <span style="font-size: 11px; color: #3b82f6; cursor: pointer;">次へ →</span>
  </div>
  <div style="font-size: 9px; color: #64748b; text-align: right; padding: 0 12px 8px;">← ページネーション</div>
  <!-- Footer -->
  <div style="background: #1e293b; color: #94a3b8; padding: 12px 20px; display: flex; justify-content: space-between; font-size: 11px;">
    <span>BookShelf &copy; 2026</span>
    <div style="display: flex; gap: 12px;">
      <span style="color: #60a5fa;">GitHub</span>
      <span style="color: #60a5fa;">Twitter</span>
    </div>
  </div>
  <div style="font-size: 9px; color: #64748b; text-align: right; padding: 2px 12px; background: #f1f5f9;">← フッター</div>
</div>

**ヘッダー:**
- 画面上部に `sticky` で固定。スクロールしても常に表示される
- `glass` クラスによる半透明のすりガラス効果（`backdrop-blur-md`）
- 左にロゴ（本のアイコン + "BookShelf"）、中央にナビゲーションリンク、右にテーマ切り替えボタン
- モバイルではハンバーガーメニューに切り替わる

**メインコンテンツ:**
- ページタイトルと書籍数を表示。右端に「新規登録」ボタン
- 検索バーとフィルター（ステータス、並び順）
- 書籍カードがレスポンシブグリッドで並ぶ。各カードはホバーで浮き上がるアニメーション付き
- カードにはサムネイル、タイトル（最大2行で省略）、著者名、星評価を表示

**フッター:**
- 著作権表示、外部リンク

### 7.2 新規登録ページ

<div style="max-width: 620px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #cbd5e1; border-radius: 12px; overflow: hidden; box-shadow: 0 4px 20px rgba(0,0,0,0.08); background: #f8fafc;">
  <!-- Header -->
  <div style="background: linear-gradient(135deg, #1e40af 0%, #3b82f6 100%); color: white; padding: 12px 20px; display: flex; justify-content: space-between; align-items: center;">
    <div style="font-weight: 700; font-size: 15px;">📖 BookShelf</div>
    <div style="display: flex; gap: 16px; font-size: 12px;">
      <span style="opacity: 0.9;">ホーム</span>
      <span style="opacity: 0.9;">書籍一覧</span>
      <span style="border-bottom: 2px solid white; padding-bottom: 2px;">新規登録</span>
    </div>
  </div>
  <!-- Breadcrumb -->
  <div style="padding: 12px 20px 4px;">
    <span style="font-size: 12px; color: #3b82f6; cursor: pointer;">← 書籍一覧に戻る</span>
  </div>
  <!-- Form Card -->
  <div style="margin: 12px 20px 16px; background: white; border: 1px solid #e2e8f0; border-radius: 12px; padding: 24px; box-shadow: 0 1px 6px rgba(0,0,0,0.04);">
    <div style="text-align: center; font-size: 17px; font-weight: 700; color: #1e293b; margin-bottom: 20px;">新しい書籍を登録</div>
    <!-- Title Field -->
    <div style="margin-bottom: 14px;">
      <div style="font-size: 12px; font-weight: 600; color: #374151; margin-bottom: 4px;">タイトル <span style="color: #ef4444;">*</span></div>
      <div style="border: 1px solid #d1d5db; border-radius: 8px; padding: 8px 12px; font-size: 12px; color: #9ca3af; background: #f9fafb;">書籍のタイトルを入力</div>
    </div>
    <!-- Author Field -->
    <div style="margin-bottom: 14px;">
      <div style="font-size: 12px; font-weight: 600; color: #374151; margin-bottom: 4px;">著者名 <span style="color: #ef4444;">*</span></div>
      <div style="border: 1px solid #d1d5db; border-radius: 8px; padding: 8px 12px; font-size: 12px; color: #9ca3af; background: #f9fafb;">著者名を入力</div>
    </div>
    <!-- ISBN Field -->
    <div style="margin-bottom: 14px;">
      <div style="font-size: 12px; font-weight: 600; color: #374151; margin-bottom: 4px;">ISBN</div>
      <div style="border: 1px solid #d1d5db; border-radius: 8px; padding: 8px 12px; font-size: 12px; color: #9ca3af; background: #f9fafb;">978-...</div>
    </div>
    <!-- Status Field -->
    <div style="margin-bottom: 14px;">
      <div style="font-size: 12px; font-weight: 600; color: #374151; margin-bottom: 4px;">ステータス <span style="color: #ef4444;">*</span></div>
      <div style="border: 1px solid #d1d5db; border-radius: 8px; padding: 8px 12px; font-size: 12px; color: #374151; background: #f9fafb; display: flex; justify-content: space-between;">
        <span>読みたい</span><span style="color: #9ca3af;">▼</span>
      </div>
    </div>
    <!-- Rating Field -->
    <div style="margin-bottom: 14px;">
      <div style="font-size: 12px; font-weight: 600; color: #374151; margin-bottom: 4px;">評価</div>
      <div style="font-size: 20px; letter-spacing: 4px; color: #d1d5db;">★ ★ ★ ★ ★ <span style="font-size: 11px; color: #9ca3af; letter-spacing: normal;">(クリックで選択)</span></div>
    </div>
    <!-- Memo Field -->
    <div style="margin-bottom: 18px;">
      <div style="font-size: 12px; font-weight: 600; color: #374151; margin-bottom: 4px;">メモ</div>
      <div style="border: 1px solid #d1d5db; border-radius: 8px; padding: 8px 12px; font-size: 12px; color: #9ca3af; background: #f9fafb; min-height: 60px;"></div>
    </div>
    <!-- Buttons -->
    <div style="display: flex; gap: 10px; justify-content: flex-end;">
      <div style="border: 1px solid #d1d5db; border-radius: 8px; padding: 8px 18px; font-size: 12px; font-weight: 600; color: #374151; cursor: pointer; background: white;">キャンセル</div>
      <div style="background: #3b82f6; color: white; border-radius: 8px; padding: 8px 18px; font-size: 12px; font-weight: 600; cursor: pointer;">+ 登録する</div>
    </div>
  </div>
  <!-- Footer -->
  <div style="background: #1e293b; color: #94a3b8; padding: 12px 20px; display: flex; justify-content: space-between; font-size: 11px;">
    <span>BookShelf &copy; 2026</span>
    <div style="display: flex; gap: 12px;">
      <span style="color: #60a5fa;">GitHub</span>
      <span style="color: #60a5fa;">Twitter</span>
    </div>
  </div>
</div>

**レイアウト:**
- フォームは `max-w-2xl` で中央配置。カードスタイルで背景と分離
- 「← 書籍一覧に戻る」のパンくずリンクをフォーム上部に配置
- 必須フィールドには `*` マーク。バリデーションエラー時はフィールド下に赤文字でメッセージ表示
- 評価は星アイコンをクリックして選択するインタラクティブ UI
- 「登録する」ボタンは送信中にローディングスピナーを表示
- 送信成功時にトースト通知が右上に表示される

### 7.3 詳細ページ

<div style="max-width: 660px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #cbd5e1; border-radius: 12px; overflow: hidden; box-shadow: 0 4px 20px rgba(0,0,0,0.08); background: #f8fafc;">
  <!-- Header -->
  <div style="background: linear-gradient(135deg, #1e40af 0%, #3b82f6 100%); color: white; padding: 12px 20px; display: flex; justify-content: space-between; align-items: center;">
    <div style="font-weight: 700; font-size: 15px;">📖 BookShelf</div>
    <div style="display: flex; gap: 16px; font-size: 12px;">
      <span style="opacity: 0.9;">ホーム</span>
      <span style="opacity: 0.9;">書籍一覧</span>
      <span style="opacity: 0.9;">新規登録</span>
    </div>
  </div>
  <!-- Breadcrumb -->
  <div style="padding: 12px 20px 4px;">
    <span style="font-size: 12px; color: #3b82f6; cursor: pointer;">← 書籍一覧に戻る</span>
  </div>
  <!-- Book Info Card -->
  <div style="margin: 12px 20px; background: white; border: 1px solid #e2e8f0; border-radius: 12px; padding: 20px; box-shadow: 0 1px 6px rgba(0,0,0,0.04); display: flex; gap: 20px; flex-wrap: wrap;">
    <!-- Cover Image -->
    <div style="width: 140px; min-height: 187px; background: linear-gradient(135deg, #dbeafe 0%, #bfdbfe 50%, #93c5fd 100%); border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 48px; flex-shrink: 0;">📘</div>
    <!-- Book Details -->
    <div style="flex: 1; min-width: 200px;">
      <div style="display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 8px;">
        <div></div>
        <span style="background: #dbeafe; color: #1e40af; font-size: 11px; font-weight: 600; padding: 3px 10px; border-radius: 20px;">📖 読書中</span>
      </div>
      <div style="font-size: 18px; font-weight: 700; color: #1e293b; margin-bottom: 4px;">React実践ガイド</div>
      <div style="font-size: 13px; color: #64748b; margin-bottom: 10px;">山田太郎</div>
      <div style="font-size: 16px; color: #f59e0b; margin-bottom: 12px;">★★★★☆ <span style="font-size: 12px; color: #64748b;">4.0</span></div>
      <!-- Properties Table -->
      <table style="font-size: 12px; color: #475569; border-collapse: collapse; width: 100%; margin-bottom: 14px;">
        <tr><td style="padding: 4px 0; color: #94a3b8; width: 80px;">ISBN</td><td style="padding: 4px 0;">978-4-xxx-xxxxx</td></tr>
        <tr><td style="padding: 4px 0; color: #94a3b8;">登録日</td><td style="padding: 4px 0;">2026/01/15</td></tr>
      </table>
      <!-- Action Buttons -->
      <div style="display: flex; gap: 8px;">
        <div style="border: 1px solid #3b82f6; color: #3b82f6; border-radius: 8px; padding: 6px 16px; font-size: 12px; font-weight: 600; cursor: pointer;">✏️ 編集</div>
        <div style="border: 1px solid #ef4444; color: #ef4444; border-radius: 8px; padding: 6px 16px; font-size: 12px; font-weight: 600; cursor: pointer;">🗑️ 削除</div>
      </div>
    </div>
  </div>
  <!-- Memo Section -->
  <div style="margin: 0 20px 16px; background: white; border: 1px solid #e2e8f0; border-radius: 12px; padding: 16px 20px; box-shadow: 0 1px 6px rgba(0,0,0,0.04);">
    <div style="font-size: 14px; font-weight: 700; color: #1e293b; margin-bottom: 8px;">📝 メモ</div>
    <div style="border-top: 1px solid #e2e8f0; padding-top: 10px; font-size: 13px; color: #475569; line-height: 1.6;">
      この本は非常に参考になった。特に第3章の<br/>アーキテクチャの解説が素晴らしい。
    </div>
  </div>
  <!-- Footer -->
  <div style="background: #1e293b; color: #94a3b8; padding: 12px 20px; display: flex; justify-content: space-between; font-size: 11px;">
    <span>BookShelf &copy; 2026</span>
    <div style="display: flex; gap: 12px;">
      <span style="color: #60a5fa;">GitHub</span>
      <span style="color: #60a5fa;">Twitter</span>
    </div>
  </div>
</div>

**レイアウト:**
- 上段は2カラム: 左に表紙画像（`aspect-[3/4]`）、右に書籍情報
- モバイルでは1カラムに切り替わり、画像が上、情報が下に配置される
- ステータスバッジが右上に表示される
- 「編集」ボタン（`btn-outline`）と「削除」ボタン（`btn-danger`）を横に並べる
- 削除ボタン押下時は `ConfirmDialog` が表示される
- メモは別のカードセクションとして表示

### 7.4 編集ページ

<div style="max-width: 620px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #cbd5e1; border-radius: 12px; overflow: hidden; box-shadow: 0 4px 20px rgba(0,0,0,0.08); background: #f8fafc;">
  <!-- Header -->
  <div style="background: linear-gradient(135deg, #1e40af 0%, #3b82f6 100%); color: white; padding: 12px 20px; display: flex; justify-content: space-between; align-items: center;">
    <div style="font-weight: 700; font-size: 15px;">📖 BookShelf</div>
    <div style="display: flex; gap: 16px; font-size: 12px;">
      <span style="opacity: 0.9;">ホーム</span>
      <span style="opacity: 0.9;">書籍一覧</span>
      <span style="opacity: 0.9;">新規登録</span>
    </div>
  </div>
  <!-- Breadcrumb -->
  <div style="padding: 12px 20px 4px;">
    <span style="font-size: 12px; color: #3b82f6; cursor: pointer;">← 書籍詳細に戻る</span>
  </div>
  <!-- Edit Form Card -->
  <div style="margin: 12px 20px 16px; background: white; border: 1px solid #e2e8f0; border-radius: 12px; padding: 24px; box-shadow: 0 1px 6px rgba(0,0,0,0.04);">
    <div style="text-align: center; font-size: 17px; font-weight: 700; color: #1e293b; margin-bottom: 20px;">書籍情報を編集</div>
    <!-- Title Field (pre-filled) -->
    <div style="margin-bottom: 14px;">
      <div style="font-size: 12px; font-weight: 600; color: #374151; margin-bottom: 4px;">タイトル <span style="color: #ef4444;">*</span></div>
      <div style="border: 1px solid #d1d5db; border-radius: 8px; padding: 8px 12px; font-size: 12px; color: #1e293b; background: #f9fafb;">React実践ガイド</div>
    </div>
    <!-- Author Field (pre-filled) -->
    <div style="margin-bottom: 14px;">
      <div style="font-size: 12px; font-weight: 600; color: #374151; margin-bottom: 4px;">著者名 <span style="color: #ef4444;">*</span></div>
      <div style="border: 1px solid #d1d5db; border-radius: 8px; padding: 8px 12px; font-size: 12px; color: #1e293b; background: #f9fafb;">山田太郎</div>
    </div>
    <!-- ISBN Field (pre-filled) -->
    <div style="margin-bottom: 14px;">
      <div style="font-size: 12px; font-weight: 600; color: #374151; margin-bottom: 4px;">ISBN</div>
      <div style="border: 1px solid #d1d5db; border-radius: 8px; padding: 8px 12px; font-size: 12px; color: #1e293b; background: #f9fafb;">978-4-xxx-xxxxx</div>
    </div>
    <!-- Status Field (pre-filled) -->
    <div style="margin-bottom: 14px;">
      <div style="font-size: 12px; font-weight: 600; color: #374151; margin-bottom: 4px;">ステータス <span style="color: #ef4444;">*</span></div>
      <div style="border: 1px solid #d1d5db; border-radius: 8px; padding: 8px 12px; font-size: 12px; color: #374151; background: #f9fafb; display: flex; justify-content: space-between;">
        <span>読書中</span><span style="color: #9ca3af;">▼</span>
      </div>
    </div>
    <!-- Rating Field (pre-filled) -->
    <div style="margin-bottom: 14px;">
      <div style="font-size: 12px; font-weight: 600; color: #374151; margin-bottom: 4px;">評価</div>
      <div style="font-size: 20px; letter-spacing: 4px;"><span style="color: #f59e0b;">★ ★ ★ ★</span> <span style="color: #d1d5db;">★</span></div>
    </div>
    <!-- Memo Field (pre-filled) -->
    <div style="margin-bottom: 18px;">
      <div style="font-size: 12px; font-weight: 600; color: #374151; margin-bottom: 4px;">メモ</div>
      <div style="border: 1px solid #d1d5db; border-radius: 8px; padding: 8px 12px; font-size: 12px; color: #1e293b; background: #f9fafb; min-height: 60px;">この本は非常に参考になった。特に第3章のアーキテクチャの解説が素晴らしい。</div>
    </div>
    <!-- Buttons -->
    <div style="display: flex; gap: 10px; justify-content: flex-end;">
      <div style="border: 1px solid #d1d5db; border-radius: 8px; padding: 8px 18px; font-size: 12px; font-weight: 600; color: #374151; cursor: pointer; background: white;">キャンセル</div>
      <div style="background: #3b82f6; color: white; border-radius: 8px; padding: 8px 18px; font-size: 12px; font-weight: 600; cursor: pointer;">✓ 更新する</div>
    </div>
  </div>
  <!-- Footer -->
  <div style="background: #1e293b; color: #94a3b8; padding: 12px 20px; display: flex; justify-content: space-between; font-size: 11px;">
    <span>BookShelf &copy; 2026</span>
    <div style="display: flex; gap: 12px;">
      <span style="color: #60a5fa;">GitHub</span>
      <span style="color: #60a5fa;">Twitter</span>
    </div>
  </div>
</div>

**レイアウト:**
- 新規登録ページとほぼ同じフォームレイアウトを再利用
- 違いは: タイトルが「書籍情報を編集」、ボタンラベルが「更新する」、パンくずが「← 書籍詳細に戻る」
- フォームの各フィールドには既存データが pre-fill される
- 更新成功時にトースト通知が表示され、詳細ページにリダイレクト

---

## 8. コンポーネント構成図（最終版）

<div style="max-width:680px;margin:20px auto;font-family:'Segoe UI',sans-serif;">
  <!-- Legend -->
  <div style="display:flex;flex-wrap:wrap;gap:8px;margin-bottom:14px;font-size:11px;">
    <span style="background:#e0e7ff;border:1px solid #6366f1;border-radius:4px;padding:2px 8px;color:#3730a3;">Layout</span>
    <span style="background:#fce7f3;border:1px solid #ec4899;border-radius:4px;padding:2px 8px;color:#9d174d;">Provider</span>
    <span style="background:#dbeafe;border:1px solid #3b82f6;border-radius:4px;padding:2px 8px;color:#1e40af;">Page</span>
    <span style="background:#d1fae5;border:1px solid #10b981;border-radius:4px;padding:2px 8px;color:#166534;">Feature</span>
    <span style="background:#fef3c7;border:1px solid #f59e0b;border-radius:4px;padding:2px 8px;color:#92400e;">UI</span>
    <span style="background:#f3e8ff;border:1px solid #a855f7;border-radius:4px;padding:2px 8px;color:#6b21a8;">Utility</span>
  </div>
  <!-- Root -->
  <div style="background:#e0e7ff;border:2px solid #6366f1;border-radius:10px;padding:8px 14px;text-align:center;font-weight:700;color:#3730a3;font-size:13px;margin-bottom:4px;box-shadow:0 2px 12px rgba(0,0,0,0.08);">RootLayout <span style="font-weight:400;font-size:11px;">(html, body)</span></div>
  <div style="display:flex;justify-content:center;"><div style="border-left:2px solid #cbd5e1;height:14px;"></div></div>
  <!-- ToastProvider -->
  <div style="background:#fce7f3;border:2px solid #ec4899;border-radius:10px;padding:8px 14px;text-align:center;font-weight:700;color:#9d174d;font-size:13px;margin-bottom:4px;box-shadow:0 2px 12px rgba(0,0,0,0.08);">ToastProvider</div>
  <div style="display:flex;justify-content:center;"><div style="border-left:2px solid #cbd5e1;height:10px;"></div></div>
  <!-- Layout children row -->
  <div style="display:flex;gap:8px;justify-content:center;flex-wrap:wrap;margin-bottom:4px;">
    <div style="background:#e0e7ff;border:2px solid #6366f1;border-radius:8px;padding:6px 12px;text-align:center;font-size:12px;">
      <div style="font-weight:700;color:#3730a3;">Header</div>
      <div style="font-size:10px;color:#6366f1;">sticky, glass効果</div>
      <div style="margin-top:4px;background:#f3e8ff;border:1px solid #a855f7;border-radius:4px;padding:2px 6px;font-size:10px;color:#6b21a8;">ThemeToggle</div>
    </div>
    <div style="background:#e0e7ff;border:2px solid #6366f1;border-radius:8px;padding:6px 12px;text-align:center;min-width:200px;">
      <div style="font-weight:700;color:#3730a3;font-size:12px;">PageTransition</div>
      <div style="font-size:10px;color:#6366f1;">フェードアニメーション</div>
    </div>
    <div style="background:#e0e7ff;border:2px solid #6366f1;border-radius:8px;padding:6px 12px;text-align:center;font-size:12px;">
      <div style="font-weight:700;color:#3730a3;">Footer</div>
    </div>
    <div style="background:#f3e8ff;border:2px solid #a855f7;border-radius:8px;padding:6px 12px;text-align:center;font-size:12px;">
      <div style="font-weight:700;color:#6b21a8;">ToastContainer</div>
      <div style="font-size:10px;color:#a855f7;">→ ToastItem</div>
    </div>
  </div>
  <div style="display:flex;justify-content:center;"><div style="border-left:2px solid #cbd5e1;height:10px;"></div></div>
  <!-- Page Components -->
  <div style="display:grid;grid-template-columns:1fr 1fr;gap:8px;margin-bottom:4px;">
    <div style="background:#dbeafe;border:2px solid #3b82f6;border-radius:8px;padding:8px 12px;box-shadow:0 2px 12px rgba(0,0,0,0.08);">
      <div style="font-weight:700;color:#1e40af;font-size:12px;">書籍一覧ページ</div>
      <div style="font-size:10px;color:#3b82f6;margin-bottom:6px;">app/books/page.tsx</div>
      <div style="display:flex;flex-wrap:wrap;gap:4px;">
        <span style="background:#d1fae5;border:1px solid #10b981;border-radius:4px;padding:2px 6px;font-size:10px;color:#166534;">BookGrid</span>
        <span style="background:#fef3c7;border:1px solid #f59e0b;border-radius:4px;padding:2px 6px;font-size:10px;color:#92400e;">Pagination</span>
      </div>
      <div style="margin-top:4px;margin-left:8px;font-size:10px;color:#64748b;">
        └ <span style="color:#166534;">BookCard</span> → <span style="color:#92400e;">Badge</span><br/>
        └ <span style="color:#92400e;">EmptyState</span> / <span style="color:#92400e;">BookCardSkeleton</span>
      </div>
    </div>
    <div style="background:#dbeafe;border:2px solid #3b82f6;border-radius:8px;padding:8px 12px;box-shadow:0 2px 12px rgba(0,0,0,0.08);">
      <div style="font-weight:700;color:#1e40af;font-size:12px;">書籍詳細ページ</div>
      <div style="font-size:10px;color:#3b82f6;margin-bottom:6px;">app/books/[id]/page.tsx</div>
      <div style="display:flex;flex-wrap:wrap;gap:4px;">
        <span style="background:#d1fae5;border:1px solid #10b981;border-radius:4px;padding:2px 6px;font-size:10px;color:#166534;">BookDetail</span>
      </div>
      <div style="margin-top:4px;margin-left:8px;font-size:10px;color:#64748b;">
        └ <span style="color:#92400e;">Badge</span> / <span style="color:#92400e;">AnimatedButton</span><br/>
        └ <span style="color:#92400e;">ConfirmDialog</span>
      </div>
    </div>
    <div style="background:#dbeafe;border:2px solid #3b82f6;border-radius:8px;padding:8px 12px;box-shadow:0 2px 12px rgba(0,0,0,0.08);">
      <div style="font-weight:700;color:#1e40af;font-size:12px;">新規登録ページ</div>
      <div style="font-size:10px;color:#3b82f6;margin-bottom:6px;">app/books/new/page.tsx</div>
      <div style="display:flex;flex-wrap:wrap;gap:4px;">
        <span style="background:#d1fae5;border:1px solid #10b981;border-radius:4px;padding:2px 6px;font-size:10px;color:#166534;">BookForm</span>
      </div>
      <div style="margin-top:4px;margin-left:8px;font-size:10px;color:#64748b;">└ <span style="color:#92400e;">Input</span> / <span style="color:#92400e;">AnimatedButton</span></div>
    </div>
    <div style="background:#dbeafe;border:2px solid #3b82f6;border-radius:8px;padding:8px 12px;box-shadow:0 2px 12px rgba(0,0,0,0.08);">
      <div style="font-weight:700;color:#1e40af;font-size:12px;">書籍編集ページ</div>
      <div style="font-size:10px;color:#3b82f6;margin-bottom:6px;">app/books/[id]/edit/page.tsx</div>
      <div style="display:flex;flex-wrap:wrap;gap:4px;">
        <span style="background:#d1fae5;border:1px solid #10b981;border-radius:4px;padding:2px 6px;font-size:10px;color:#166534;">BookForm</span>
      </div>
      <div style="margin-top:4px;margin-left:8px;font-size:10px;color:#64748b;">└ <span style="color:#92400e;">Input</span> / <span style="color:#92400e;">AnimatedButton</span></div>
    </div>
  </div>
</div>

**構成図の読み方:**

| 色 | カテゴリ | 説明 |
|:---|:---------|:-----|
| 青 | Page Components | Next.js のルートに対応するページコンポーネント |
| 緑 | Feature Components | 特定の機能を担うコンポーネント（書籍グリッド、フォーム等） |
| 黄 | UI Components | 再利用可能な汎用 UI 部品 |
| 紫 | Utility Components | トースト通知やテーマ切替などの横断的機能 |
| インディゴ | Layout Components | ヘッダー、フッター、ページ遷移など画面の骨格 |
| ピンク | Providers | React Context による状態管理 |

---

## まとめ

この章で実装した内容を振り返ります。

ここまでで、書籍管理アプリは「動くだけのアプリ」から「整ったデザイン・滑らかなアニメーション・ダークモード対応・誰でも使えるアクセシビリティ」を備えた本格的なWebアプリに進化しました。Tailwind CSS のユーティリティファースト思想に慣れると、CSSを書く時間より「何を作りたいか」を考える時間に集中できるようになります。

| 項目 | 実装内容 | ファイル |
|:-----|:---------|:---------|
| グローバルスタイル | CSS カスタムプロパティ、ベースレイヤー、コンポーネントレイヤー | `app/globals.css` |
| Tailwind 設定 | カスタムカラー、アニメーション、フォント、シャドウ | `tailwind.config.ts` |
| レスポンシブグリッド | 1列→2列→3列→4列のブレークポイント対応 | `components/BookGrid.tsx` |
| トースト通知 | Context API ベースの成功/エラー通知 | `contexts/ToastContext.tsx`, `components/Toast.tsx` |
| ページネーション | ページ番号計算ロジック付きナビゲーション | `components/Pagination.tsx` |
| 空状態 | アイコン + メッセージ + CTA ボタン | `components/EmptyState.tsx` |
| スケルトン | シマーアニメーション付きローディング | `components/BookCardSkeleton.tsx` |
| カードアニメーション | ホバーで浮き上がり + 画像ズーム + オーバーレイ | `components/BookCard.tsx` |
| ページ遷移 | フェードイン・スライドアップ | `components/PageTransition.tsx` |
| ダークモード | class ベース切替 + ちらつき防止スクリプト | `components/ThemeToggle.tsx` |
| アクセシビリティ | aria属性、キーボードナビ、フォーカストラップ、コントラスト比 | 各コンポーネント |
| 確認ダイアログ | モーダル + フォーカストラップ + Escape 対応 | `components/ConfirmDialog.tsx` |

次の章では、テストの書き方とデプロイについて学びます。

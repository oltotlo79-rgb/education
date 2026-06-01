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

#### 3.1.3 トーストコンポーネント

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

**使用例:**

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

### 3.3 空状態（データがない場合）の改善

**空状態（empty state：データが0件・該当なし・初回利用などの「何もない」画面）** は、ただ真っ白にせず、ユーザーに次の行動を促す案内を出すのが現代的なUXのお作法です。

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

### 3.4 ローディングスケルトン

**スケルトン（skeleton：骨組み）** は、データ取得中に「これからこういう形の中身が出ます」と予告するグレーの仮表示です。スピナー（くるくる回るアイコン）よりも「コンテンツの骨組み」を見せたほうが、待ち時間の体感が短くなることが知られています。

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

**ちらつき防止のスクリプト:**

ページ読み込み時にダークモードが一瞬ライトモードで表示されるのを防ぐため、`<head>` 内にインラインスクリプトを設置します。

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

### 6.2 キーボードナビゲーション

**キーボードナビゲーション（keyboard navigation：マウスを使わずキーボードだけで操作できるようにすること）** は、視覚障害のあるユーザーや、マウスが使いづらい状況のユーザーにとって必須の機能です。

主な操作:

- **Tab キー**: 次のフォーカス可能要素へ移動
- **Shift + Tab**: 前のフォーカス可能要素へ移動
- **Enter / Space**: 選択中の要素を実行（クリック相当）
- **Escape**: モーダルやポップアップを閉じる
- **矢印キー**: メニュー内の項目選択（必要に応じて自分で実装）

**フォーカストラップ（focus trap：フォーカスを特定範囲内に閉じ込める仕組み）** は、モーダル表示中に Tab キーで背景の要素にフォーカスが飛んでしまうのを防ぐテクニックです。

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

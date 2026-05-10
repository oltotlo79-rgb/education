# 第4章: Next.js の基礎

## 0. 前提知識: URL・ルーティング・サーバーレンダリング

Next.js を理解するには「URL」「ルーティング」「レンダリング」の3つの基本概念を押さえる必要があります。ここから始めましょう。

### 0.1 URL の構造

```
https://example.com:443/products/123?sort=price&page=2#reviews
─┬───   ─┬─────────  ─┬─ ─┬──────────  ─┬───────────────  ─┬──────
 │       │            │   │              │                  │
 │       │            │   │              │                  └─ フラグメント（ページ内位置）
 │       │            │   │              └─ クエリパラメータ（?以降）
 │       │            │   └─ パス（path: ページの場所）
 │       │            └─ ポート番号（省略時はhttpsなら443）
 │       └─ ホスト名（ドメイン）
 └─ プロトコル（通信方式）
```

Next.js で書く各ページは、この**パス**（`/products/123` の部分）に対応します。

### 0.2 ルーティングって何？

「URL を受け取って、どのページを表示するか決める仕組み」が **ルーティング（routing）** です。Next.js の **App Router** では、`app/` フォルダの中のフォルダ構成がそのまま URL になります。

```
app/
 ├─ page.tsx            → /        （トップページ）
 ├─ about/
 │   └─ page.tsx        → /about
 └─ books/
     ├─ page.tsx        → /books
     └─ [id]/
         └─ page.tsx    → /books/123 （[id] は動的な値）
```

> **ファイル＝ページ:** 「`app/about/page.tsx` を作っただけで `/about` というURLでそのページが見られるようになる」というのが Next.js のキモです。第3章までの React だけでは、自分でルーターを書く必要がありました。

### 0.3 レンダリング3兄弟（CSR / SSR / SSG）

Webページを「画面に出るHTMLにする」処理を **レンダリング** と呼びます。これが**いつ・どこで**行われるかで3種類あります。

| 種類 | フルネーム | いつHTMLができる？ | どこで？ | メリット |
|------|-----------|--------------------|---------|----------|
| **CSR** | Client Side Rendering | アクセス時 | ブラウザ | 動的、軽量サーバー |
| **SSR** | Server Side Rendering | アクセス時 | サーバー | SEO良、初回表示速い |
| **SSG** | Static Site Generation | ビルド時に1回 | サーバー | 最速、安価 |

> **本書での使い分け:** Next.js の App Router では、`page.tsx` をシンプルに書くと**サーバーで実行される（SSR/SSG相当）** のがデフォルトです。インタラクティブな部品だけ `"use client"` を付けて CSR にします。

---

## はじめに

この章では、React ベースの**フルスタックフレームワーク**（Full-stack Framework：フロントエンド=画面もバックエンド=サーバー処理も両方カバーするフレームワーク）である **Next.js**（ネクストジェイエス） の基礎を学びます。

Next.js を使うことで、React 単体では実現が難しい**サーバーサイドレンダリング**（SSR：Server Side Rendering = サーバー側でHTMLを生成してからブラウザに送る方式。ページの初回表示が速くなる）、**静的サイト生成**（SSG：Static Site Generation = ビルド時にHTMLを事前生成する方式。最も高速）、**ファイルベースルーティング**（ファイルを配置するだけでURLとページが自動的に対応する仕組み）などを簡単に実装できます。

### なぜ React だけではなく Next.js を使うのか

React は「UIの部品を作るライブラリ」であり、それ以外の機能（ページのURL管理、サーバーとの通信、SEO対策など）は自分で用意する必要があります。Next.js はこれらを**最初から備えている**ため、開発者はアプリのロジックに集中できます。

| 機能 | React のみ | Next.js |
|------|-----------|---------|
| ページのURL管理（ルーティング） | 別途ライブラリ（react-router等）が必要 | ファイルを置くだけで自動対応 |
| サーバーでのデータ取得 | 自分でAPIサーバーを構築 | Server Components で直接取得 |
| SEO（検索エンジン対策） | 追加設定が必要 | 標準で対応 |
| ページ読み込み速度 | 初回が遅い（CSR） | 高速（SSR/SSG） |
| デプロイ（公開） | 別途設定が必要 | Vercel で数クリック |

> **この章で学ぶこと:**
> - Next.js の概念と React との関係
> - **App Router**（アップルーター：Next.js 13以降の新しいルーティング方式）によるルーティング
> - **Server Components**（サーバー上で実行されるコンポーネント）と **Client Components**（ブラウザ上で実行されるコンポーネント）の使い分け
> - レイアウト、ナビゲーション、**データフェッチ**（Data Fetch：サーバーやAPIからデータを取得すること）の基本
> - **Server Actions**（サーバーアクション：フォーム送信などのサーバー処理をシンプルに書ける仕組み）によるサーバー処理
> - プロジェクト構成の**ベストプラクティス**（Best Practice：最も良いとされるやり方・慣習）

---

## 1. Next.js とは

### 1.1 React との関係

Next.js は **Vercel 社** が開発・メンテナンスしている React ベースのフレームワークです。React はあくまで「UIライブラリ」であり、ルーティングやサーバーサイド処理などは含まれていません。Next.js はその React の上に、Web アプリケーション開発に必要な機能を包括的に提供します。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif;">
  <div style="border: 2px solid #1e40af; border-radius: 12px; padding: 20px; background: #eff6ff; box-shadow: 0 4px 16px rgba(30,64,175,0.10);">
    <div style="font-weight: 700; color: #1e40af; font-size: 15px; margin-bottom: 14px; text-align: center;">Next.js フレームワーク</div>
    <div style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin-bottom: 16px;">
      <div style="background: #1e40af; color: #fff; border-radius: 8px; padding: 8px 14px; font-size: 12px; font-weight: 600; text-align: center;">ルーティング<br/>（App Router）</div>
      <div style="background: #1e40af; color: #fff; border-radius: 8px; padding: 8px 14px; font-size: 12px; font-weight: 600; text-align: center;">SSR / SSG / ISR</div>
      <div style="background: #1e40af; color: #fff; border-radius: 8px; padding: 8px 14px; font-size: 12px; font-weight: 600; text-align: center;">API Routes /<br/>Server Actions</div>
      <div style="background: #1e40af; color: #fff; border-radius: 8px; padding: 8px 14px; font-size: 12px; font-weight: 600; text-align: center;">画像最適化<br/>（next/image）</div>
      <div style="background: #1e40af; color: #fff; border-radius: 8px; padding: 8px 14px; font-size: 12px; font-weight: 600; text-align: center;">フォント最適化<br/>（next/font）</div>
      <div style="background: #1e40af; color: #fff; border-radius: 8px; padding: 8px 14px; font-size: 12px; font-weight: 600; text-align: center;">ミドルウェア</div>
    </div>
    <div style="text-align: center; color: #3b82f6; font-size: 18px; margin-bottom: 8px;">&#x2193; &#x2193; &#x2193;</div>
    <div style="border: 2px solid #38bdf8; border-radius: 10px; padding: 16px; background: #e0f7fa;">
      <div style="font-weight: 700; color: #0e7490; font-size: 14px; margin-bottom: 10px; text-align: center;">React ライブラリ</div>
      <div style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center;">
        <div style="background: #38bdf8; color: #000; border-radius: 8px; padding: 7px 14px; font-size: 12px; font-weight: 600;">コンポーネント</div>
        <div style="background: #38bdf8; color: #000; border-radius: 8px; padding: 7px 14px; font-size: 12px; font-weight: 600;">JSX</div>
        <div style="background: #38bdf8; color: #000; border-radius: 8px; padding: 7px 14px; font-size: 12px; font-weight: 600;">State 管理</div>
        <div style="background: #38bdf8; color: #000; border-radius: 8px; padding: 7px 14px; font-size: 12px; font-weight: 600;">Hooks</div>
        <div style="background: #38bdf8; color: #000; border-radius: 8px; padding: 7px 14px; font-size: 12px; font-weight: 600;">仮想 DOM</div>
      </div>
    </div>
  </div>
</div>

**たとえ話で理解する:**

- **React** = エンジン（車を動かす核心部分）
- **Next.js** = 完成した車（エンジンに加え、ハンドル、ブレーキ、ナビなど全部入り）

React 単体でアプリを作ることもできますが、ルーティングやデータ取得の仕組みを自分で選定・構築する必要があります。Next.js を使えば、それらが最初から統合されています。

### 1.2 SSR, SSG, CSR の違い

Web ページのレンダリング方法は大きく3つあります。Next.js はこれらすべてに対応しています。

#### CSR（Client-Side Rendering）- クライアントサイドレンダリング

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #e2e8f0; border-radius: 12px; overflow: hidden; box-shadow: 0 2px 12px rgba(0,0,0,0.06);">
  <div style="background: #1e40af; color: white; padding: 10px 20px; font-weight: 700; font-size: 13px; text-align: center;">CSR（Client-Side Rendering）の流れ</div>
  <div style="padding: 16px 20px;">
    <div style="display: flex; justify-content: space-around; margin-bottom: 12px; font-weight: 700; font-size: 13px; color: #1e40af;">
      <span style="flex: 1; text-align: center;">ユーザー（ブラウザ）</span><span style="flex: 1; text-align: center;">サーバー</span>
    </div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 8px;">
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;">ページをリクエスト</div>
      <div style="flex: 1; height: 2px; background: linear-gradient(to right, #3b82f6, #3b82f6); position: relative;"><span style="position: absolute; right: -4px; top: -4px; color: #3b82f6;">&#x25B6;</span></div>
      <div style="flex: 1; font-size: 12px; color: #334155;"></div>
    </div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 8px;">
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;"></div>
      <div style="flex: 1; height: 2px; background: #94a3b8; border-top: 2px dashed #94a3b8; position: relative;"><span style="position: absolute; left: -4px; top: -4px; color: #94a3b8;">&#x25C0;</span></div>
      <div style="flex: 1; font-size: 12px; color: #334155;">空の HTML + JS バンドル</div>
    </div>
    <div style="background: #fef3c7; border-left: 3px solid #f59e0b; padding: 8px 12px; margin: 8px 0; border-radius: 0 6px 6px 0;">
      <div style="font-size: 12px; color: #92400e; line-height: 1.8;">
        &#x23F3; JavaScript をダウンロード<br/>
        &#x23F3; JavaScript を実行<br/>
        &#x23F3; React が DOM を構築<br/>
        &#x23F3; データを API から取得<br/>
        &#x23F3; ようやくページが表示される
      </div>
    </div>
    <div style="background: #fee2e2; border-radius: 8px; padding: 10px 14px; margin-top: 10px; font-size: 12px; line-height: 1.8;">
      <span style="color: #dc2626;">&#x274C; 初回表示が遅い</span><br/>
      <span style="color: #dc2626;">&#x274C; SEO に不利</span><br/>
      <span style="color: #16a34a;">&#x2705; ページ遷移は高速</span>
    </div>
  </div>
</div>

**画面にはこう表示される:** 最初に一瞬だけ白い画面やローディングスピナーが表示され、JavaScript の処理が完了してからコンテンツが表示されます。素の React（Create React App）はデフォルトでこの方式です。

#### SSR（Server-Side Rendering）- サーバーサイドレンダリング

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #e2e8f0; border-radius: 12px; overflow: hidden; box-shadow: 0 2px 12px rgba(0,0,0,0.06);">
  <div style="background: #1e40af; color: white; padding: 10px 20px; font-weight: 700; font-size: 13px; text-align: center;">SSR（Server-Side Rendering）の流れ</div>
  <div style="padding: 16px 20px;">
    <div style="display: flex; justify-content: space-around; margin-bottom: 12px; font-weight: 700; font-size: 13px; color: #1e40af;">
      <span style="flex: 1; text-align: center;">ユーザー</span><span style="flex: 1; text-align: center;">サーバー</span><span style="flex: 1; text-align: center;">データベース</span>
    </div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 6px;">
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;">リクエスト</div>
      <div style="flex: 1; height: 2px; background: #3b82f6;"></div>
      <div style="flex: 1; font-size: 12px; color: #334155;">&#x25B6;</div>
      <div style="flex: 1;"></div>
    </div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 6px;">
      <div style="flex: 1;"></div>
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;">データ取得</div>
      <div style="flex: 1; height: 2px; background: #3b82f6;"></div>
      <div style="flex: 1; font-size: 12px; color: #334155;">&#x25B6;</div>
    </div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 6px;">
      <div style="flex: 1;"></div>
      <div style="flex: 1; font-size: 12px; color: #334155;">&#x25C0;</div>
      <div style="flex: 1; height: 2px; background: #94a3b8; border-top: 2px dashed #94a3b8;"></div>
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;">データ返却</div>
    </div>
    <div style="background: #dbeafe; border-left: 3px solid #3b82f6; padding: 8px 12px; margin: 8px 40px 8px 33%; border-radius: 0 6px 6px 0; font-size: 12px; color: #1e3a5f;">
      サーバー上で React を実行し HTML を生成
    </div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 6px;">
      <div style="flex: 1; font-size: 12px; color: #334155;">&#x25C0;</div>
      <div style="flex: 1; height: 2px; background: #94a3b8; border-top: 2px dashed #94a3b8;"></div>
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;">完成 HTML</div>
      <div style="flex: 1;"></div>
    </div>
    <div style="background: #dcfce7; border-left: 3px solid #22c55e; padding: 8px 12px; margin: 8px 60% 8px 0; border-radius: 0 6px 6px 0; font-size: 12px; color: #14532d;">
      すぐにページが表示 &#x2192; Hydration でインタラクティブに
    </div>
    <div style="background: #f0fdf4; border-radius: 8px; padding: 10px 14px; margin-top: 10px; font-size: 12px; line-height: 1.8;">
      <span style="color: #16a34a;">&#x2705; 初回表示が速い</span><br/>
      <span style="color: #16a34a;">&#x2705; SEO に有利</span><br/>
      <span style="color: #d97706;">&#x26A0;&#xFE0F; サーバー負荷あり</span>
    </div>
  </div>
</div>

**画面にはこう表示される:** リクエストのたびにサーバーで HTML が生成されるため、最新のデータが反映された完成形のページがすぐに表示されます。ブラウザが JavaScript を読み込むと、ボタンクリックなどのインタラクションが可能になります（この過程を **Hydration** と呼びます）。

#### SSG（Static Site Generation）- 静的サイト生成

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #e2e8f0; border-radius: 12px; overflow: hidden; box-shadow: 0 2px 12px rgba(0,0,0,0.06);">
  <div style="background: #1e40af; color: white; padding: 10px 20px; font-weight: 700; font-size: 13px; text-align: center;">SSG（Static Site Generation）の流れ</div>
  <div style="padding: 16px 20px;">
    <div style="display: flex; justify-content: space-around; margin-bottom: 12px; font-weight: 700; font-size: 13px; color: #1e40af;">
      <span style="flex: 1; text-align: center;">ビルド時</span><span style="flex: 1; text-align: center;">サーバー / CDN</span><span style="flex: 1; text-align: center;">ユーザー</span>
    </div>
    <div style="background: #dbeafe; border-left: 3px solid #3b82f6; padding: 8px 12px; margin: 8px 60% 8px 0; border-radius: 0 6px 6px 0; font-size: 12px; color: #1e3a5f;">
      ビルド時に全ページの HTML を事前生成
    </div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 8px;">
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;">生成済み HTML を配置</div>
      <div style="flex: 1; height: 2px; background: #3b82f6;"></div>
      <div style="flex: 1; font-size: 12px; color: #334155;">&#x25B6;</div>
    </div>
    <div style="border-top: 1px dashed #cbd5e1; margin: 12px 0; padding-top: 12px;">
      <div style="font-size: 11px; color: #64748b; text-align: center; margin-bottom: 8px;">&#x2015;&#x2015; ユーザーアクセス時 &#x2015;&#x2015;</div>
    </div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 6px;">
      <div style="flex: 1;"></div>
      <div style="flex: 1; font-size: 12px; color: #334155;">&#x25C0;</div>
      <div style="flex: 1; height: 2px; background: #3b82f6;"></div>
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;">リクエスト</div>
    </div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 6px;">
      <div style="flex: 1;"></div>
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;">事前生成 HTML</div>
      <div style="flex: 1; height: 2px; background: #94a3b8; border-top: 2px dashed #94a3b8;"></div>
      <div style="flex: 1; font-size: 12px; color: #334155;">&#x25B6;</div>
    </div>
    <div style="background: #dcfce7; border-left: 3px solid #22c55e; padding: 8px 12px; margin: 8px 0 8px 66%; border-radius: 0 6px 6px 0; font-size: 12px; color: #14532d;">
      即座にページが表示される
    </div>
    <div style="background: #f0fdf4; border-radius: 8px; padding: 10px 14px; margin-top: 10px; font-size: 12px; line-height: 1.8;">
      <span style="color: #16a34a;">&#x2705; 最速の表示</span><br/>
      <span style="color: #16a34a;">&#x2705; SEO に最適</span><br/>
      <span style="color: #16a34a;">&#x2705; サーバー負荷なし</span><br/>
      <span style="color: #d97706;">&#x26A0;&#xFE0F; データが古い可能性</span>
    </div>
  </div>
</div>

**画面にはこう表示される:** ビルド時に生成済みの HTML がそのまま返されるため、表示速度は最速です。ただし、ビルド後にデータが変わっても、再ビルドするまで古いデータが表示され続けます。ブログや企業サイトなど、更新頻度の低いページに最適です。

#### 3つのレンダリング方式の比較

| 特性 | CSR | SSR | SSG |
|---|---|---|---|
| **初回表示速度** | 遅い | 速い | 最速 |
| **SEO** | 不利 | 有利 | 最も有利 |
| **データの鮮度** | 常に最新 | 常に最新 | ビルド時点 |
| **サーバー負荷** | 低い | 高い | 最も低い |
| **使用例** | ダッシュボード | EC サイト商品ページ | ブログ記事 |

### 1.3 なぜ Next.js を使うのか（素の React との比較）

| 機能 | 素の React | Next.js |
|---|---|---|
| **ルーティング** | react-router 等を別途インストール | App Router が組み込み |
| **SSR / SSG** | 自分で構築が必要（非常に複雑） | 設定なしで利用可能 |
| **コード分割** | 手動で React.lazy 等を使う | 自動で最適化 |
| **画像最適化** | 自分で実装 | `next/image` が自動最適化 |
| **フォント最適化** | 自分で実装 | `next/font` が自動最適化 |
| **API エンドポイント** | Express 等の別サーバーが必要 | Route Handlers / Server Actions |
| **TypeScript** | 設定が必要 | 初期設定済み |
| **ESLint** | 設定が必要 | 初期設定済み |
| **環境変数** | dotenv 等を別途設定 | `.env.local` が組み込み |
| **デプロイ** | ホスティング先の選定・設定が必要 | Vercel にワンクリックデプロイ |

> **結論:** 2024年以降、新規の React プロジェクトを始める場合は Next.js（App Router）を使うのがスタンダードです。React の公式ドキュメントでも Next.js が推奨フレームワークの1つとして紹介されています。

---

## 2. App Router

### 2.1 App Router vs Pages Router

Next.js には2つのルーティングシステムがあります。

| 特性 | Pages Router（従来） | App Router（推奨） |
|---|---|---|
| **ディレクトリ** | `pages/` | `app/` |
| **導入バージョン** | Next.js 初期〜 | Next.js 13.4〜（安定版） |
| **デフォルトコンポーネント** | Client Component | **Server Component** |
| **レイアウト** | `_app.tsx`, `_document.tsx` | `layout.tsx`（ネスト可能） |
| **データ取得** | `getServerSideProps`, `getStaticProps` | **async/await で直接** |
| **ローディング UI** | 自分で実装 | `loading.tsx` |
| **エラー UI** | `_error.tsx`（グローバル） | `error.tsx`（ルートごと） |
| **Server Actions** | 非対応 | **対応** |
| **Streaming** | 非対応 | **対応** |

> **本チュートリアルでは App Router のみを使用します。** Pages Router は旧バージョンとの互換性のために残されていますが、新規プロジェクトでは App Router を選択してください。

### 2.2 ファイルベースルーティングの仕組み

Next.js の App Router では、`app/` ディレクトリ内のフォルダ構造がそのまま URL のパスに対応します。これが **ファイルベースルーティング** です。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; background: #1e293b; border-radius: 12px; padding: 20px; color: #e2e8f0; font-size: 13px; line-height: 2; box-shadow: 0 4px 16px rgba(0,0,0,0.15);">
  <div style="font-weight: 700; color: #93c5fd; margin-bottom: 8px; font-size: 14px;">ファイル構造 &#x2192; URL パス</div>
  <div>&#x1F4C1; app/</div>
  <div style="padding-left: 24px;">&#x1F4C4; page.tsx <span style="color: #94a3b8;">&#x2192;</span> <span style="background: #3b82f6; color: white; padding: 2px 8px; border-radius: 4px; font-size: 11px;">/</span> <span style="color: #64748b; font-size: 11px;">（トップ）</span></div>
  <div style="padding-left: 24px;">&#x1F4C1; about/</div>
  <div style="padding-left: 48px;">&#x1F4C4; page.tsx <span style="color: #94a3b8;">&#x2192;</span> <span style="background: #3b82f6; color: white; padding: 2px 8px; border-radius: 4px; font-size: 11px;">/about</span></div>
  <div style="padding-left: 24px;">&#x1F4C1; books/</div>
  <div style="padding-left: 48px;">&#x1F4C4; page.tsx <span style="color: #94a3b8;">&#x2192;</span> <span style="background: #3b82f6; color: white; padding: 2px 8px; border-radius: 4px; font-size: 11px;">/books</span></div>
  <div style="padding-left: 48px;">&#x1F4C1; <span style="background: #f59e0b; color: #000; padding: 1px 6px; border-radius: 4px; font-size: 11px;">[id]</span>/</div>
  <div style="padding-left: 72px;">&#x1F4C4; page.tsx <span style="color: #94a3b8;">&#x2192;</span> <span style="background: #f59e0b; color: #000; padding: 2px 8px; border-radius: 4px; font-size: 11px;">/books/123</span></div>
  <div style="padding-left: 48px;">&#x1F4C1; new/</div>
  <div style="padding-left: 72px;">&#x1F4C4; page.tsx <span style="color: #94a3b8;">&#x2192;</span> <span style="background: #3b82f6; color: white; padding: 2px 8px; border-radius: 4px; font-size: 11px;">/books/new</span></div>
</div>

**重要なルール:**
- **フォルダ** がURLのセグメント（パスの一部）になる
- **`page.tsx`** があるフォルダだけがアクセス可能なルートになる
- `page.tsx` がないフォルダは URL として公開されない（コンポーネントの整理用に使える）

### 2.3 page.tsx, layout.tsx, loading.tsx, error.tsx の役割

App Router では、特別な名前を持つファイルにそれぞれ役割があります。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif;">
  <div style="display: flex; gap: 16px; flex-wrap: wrap;">
    <div style="flex: 1; min-width: 200px; border: 1px solid #e2e8f0; border-radius: 12px; padding: 16px; background: #f8fafc;">
      <div style="font-weight: 700; color: #1e40af; font-size: 13px; margin-bottom: 10px;">app/books/ ディレクトリ</div>
      <div style="display: flex; flex-direction: column; gap: 6px;">
        <div style="background: #22c55e; color: #fff; border-radius: 8px; padding: 6px 12px; font-size: 12px; font-weight: 600;">layout.tsx（共通レイアウト）</div>
        <div style="background: #3b82f6; color: #fff; border-radius: 8px; padding: 6px 12px; font-size: 12px; font-weight: 600;">page.tsx（ページ本体）</div>
        <div style="background: #f59e0b; color: #fff; border-radius: 8px; padding: 6px 12px; font-size: 12px; font-weight: 600;">loading.tsx（読み込み中の表示）</div>
        <div style="background: #ef4444; color: #fff; border-radius: 8px; padding: 6px 12px; font-size: 12px; font-weight: 600;">error.tsx（エラー時の表示）</div>
        <div style="background: #8b5cf6; color: #fff; border-radius: 8px; padding: 6px 12px; font-size: 12px; font-weight: 600;">not-found.tsx（404の表示）</div>
      </div>
    </div>
    <div style="flex: 1; min-width: 280px; border: 1px solid #e2e8f0; border-radius: 12px; padding: 16px; background: #f8fafc;">
      <div style="font-weight: 700; color: #1e40af; font-size: 13px; margin-bottom: 10px;">レンダリングの流れ</div>
      <div style="text-align: center;">
        <div style="background: #1e293b; color: #fff; border-radius: 8px; padding: 8px 14px; font-size: 12px; font-weight: 600; display: inline-block;">リクエスト受信</div>
        <div style="color: #64748b; font-size: 16px; margin: 4px 0;">&#x2193;</div>
        <div style="background: #fef3c7; border: 2px solid #f59e0b; border-radius: 8px; padding: 8px 14px; font-size: 12px; font-weight: 600; display: inline-block;">データ読み込み中？</div>
        <div style="display: flex; justify-content: center; gap: 40px; margin-top: 4px;">
          <div>
            <div style="color: #f59e0b; font-size: 11px; font-weight: 700;">Yes &#x2193;</div>
            <div style="background: #fef3c7; border: 1px solid #f59e0b; border-radius: 6px; padding: 6px 10px; font-size: 11px; margin-top: 2px;">loading.tsx を表示</div>
          </div>
          <div>
            <div style="color: #64748b; font-size: 11px; font-weight: 700;">No &#x2193;</div>
          </div>
        </div>
        <div style="color: #64748b; font-size: 16px; margin: 4px 0;">&#x2193;</div>
        <div style="background: #fee2e2; border: 2px solid #ef4444; border-radius: 8px; padding: 8px 14px; font-size: 12px; font-weight: 600; display: inline-block;">エラー発生？</div>
        <div style="display: flex; justify-content: center; gap: 40px; margin-top: 4px;">
          <div>
            <div style="color: #ef4444; font-size: 11px; font-weight: 700;">Yes &#x2193;</div>
            <div style="background: #fee2e2; border: 1px solid #ef4444; border-radius: 6px; padding: 6px 10px; font-size: 11px; margin-top: 2px;">error.tsx を表示</div>
          </div>
          <div>
            <div style="color: #64748b; font-size: 11px; font-weight: 700;">No &#x2193;</div>
          </div>
        </div>
        <div style="color: #64748b; font-size: 16px; margin: 4px 0;">&#x2193;</div>
        <div style="background: #f3e8ff; border: 2px solid #8b5cf6; border-radius: 8px; padding: 8px 14px; font-size: 12px; font-weight: 600; display: inline-block;">ページ存在？</div>
        <div style="display: flex; justify-content: center; gap: 40px; margin-top: 4px;">
          <div>
            <div style="color: #8b5cf6; font-size: 11px; font-weight: 700;">No &#x2193;</div>
            <div style="background: #f3e8ff; border: 1px solid #8b5cf6; border-radius: 6px; padding: 6px 10px; font-size: 11px; margin-top: 2px;">not-found.tsx</div>
          </div>
          <div>
            <div style="color: #22c55e; font-size: 11px; font-weight: 700;">Yes &#x2193;</div>
            <div style="background: #dcfce7; border: 1px solid #22c55e; border-radius: 6px; padding: 6px 10px; font-size: 11px; margin-top: 2px;">layout.tsx + page.tsx</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

#### page.tsx - ページ本体

そのルート（URL）にアクセスしたときに表示されるメインコンテンツです。

```typescript
// app/page.tsx
// URL: /

export default function HomePage() {
  return (
    <div>
      <h1>書籍管理アプリへようこそ</h1>
      <p>あなたの読書記録を管理しましょう。</p>
    </div>
  );
}
```

**画面にはこう表示される:** ブラウザで `http://localhost:3000/` にアクセスすると、「書籍管理アプリへようこそ」という見出しと「あなたの読書記録を管理しましょう。」という段落が表示されます。

#### layout.tsx - 共通レイアウト

複数のページで共有される UI（ヘッダー、サイドバーなど）を定義します。ページ遷移してもレイアウトは再レンダリングされず、状態が保持されます。

```typescript
// ============================================================================
// ファイルパス: app/layout.tsx
// 役割      : すべてのページを「ヘッダー＋本文＋フッター」の枠で包む
//             ＝ Next.jsアプリの **一番外側の共通レイアウト**
// ----------------------------------------------------------------------------
// このファイルは Next.js が自動的に認識する「予約名」のひとつ。
// app/ フォルダ直下に layout.tsx が必ず1つ必要。
// ============================================================================

// `Metadata` 型を import する（Next.js 標準の型）。
// `import type` は「型だけ取り込む」記法。実行時のJSには残らないので軽量。
import type { Metadata } from "next";

// ─────────────────────────────────────────────────────────────────────
// (1) メタデータの定義
// ─────────────────────────────────────────────────────────────────────
// metadata という名前で export すると、Next.js が <head> 内の <title> や
// <meta name="description" ...> を自動生成してくれる。
// 自分で <head> を書く必要がない。
export const metadata: Metadata = {
  title: "書籍管理アプリ",                                       // ブラウザタブの文字
  description: "あなたの読書記録を管理するアプリケーション",   // SEO・SNS共有時の説明文
};

// ─────────────────────────────────────────────────────────────────────
// (2) ルートレイアウトのコンポーネント本体
// ─────────────────────────────────────────────────────────────────────
// `RootLayout` は予約名ではないが、慣習的にこの名前を付ける。
//
// 引数（Props）:
//   - children: このレイアウトの中に差し込まれる「各ページの中身」
//     React.ReactNode 型は「JSX/文字列/数値/null/...などReactで扱える何でも」を表す。
export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    // ── 必須: <html> と <body> はルートレイアウトでだけ書く ──
    // lang="ja" は「このページは日本語ですよ」という指定。
    // スクリーンリーダーや翻訳ツールがこれを参照する。
    <html lang="ja">
      <body>

        {/* ─── 共通ヘッダー（全ページに表示される） ─── */}
        <header>
          <nav>
            <h1>📚 書籍管理アプリ</h1>
          </nav>
        </header>

        {/*
          ─── ページ本体が差し込まれる場所 ───
          各 page.tsx の return 値が、ここに自動で入る。
          /books にアクセスしたら books/page.tsx の内容が、
          /books/123 にアクセスしたら books/[id]/page.tsx の内容がここに入る。
        */}
        <main>{children}</main>

        {/* ─── 共通フッター（全ページに表示される） ─── */}
        <footer>
          <p>&copy; 2024 書籍管理アプリ</p>
        </footer>

      </body>
    </html>
  );
}
```

**▼ ブラウザでの見た目（どのページでも上下にこの枠が出る）:**

```
┌─────────────────────────────────────────┐
│ 📚 書籍管理アプリ                         │  ← <header> 内の <h1>
├─────────────────────────────────────────┤
│                                          │
│   ← ここに各ページ ({children}) の         │
│      中身が差し込まれる                    │
│                                          │
├─────────────────────────────────────────┤
│ © 2024 書籍管理アプリ                     │  ← <footer>
└─────────────────────────────────────────┘
```

**▼ 動作の仕組み:**

| URL | layout.tsx | 中央 ({children}) |
|------|------------|-------------------|
| `/`           | 表示される | `app/page.tsx` の中身 |
| `/books`      | 表示される | `app/books/page.tsx` の中身 |
| `/books/123`  | 表示される | `app/books/[id]/page.tsx` の中身 |

ページ遷移しても**ヘッダーとフッターは再描画されない**（=スクロール位置や状態が保たれる）のが Next.js のレイアウト機能のメリットです。

#### loading.tsx - 読み込み中の表示

データの読み込み中に自動的に表示される UI です。React の `<Suspense>` を内部的に使用しています。

```typescript
// app/books/loading.tsx

export default function BooksLoading() {
  return (
    <div className="loading-container">
      <div className="spinner" />
      <p>書籍データを読み込んでいます...</p>
    </div>
  );
}
```

**画面にはこう表示される:** `/books` にアクセスした直後、データベースからデータを取得している間、スピナーアニメーションと「書籍データを読み込んでいます...」というメッセージが表示されます。データの取得が完了すると、自動的に `page.tsx` の内容に切り替わります。

#### error.tsx - エラー時の表示

ページの描画中にエラーが発生した場合に表示される UI です。**`"use client"` が必須** です（エラーバウンダリは Client Component でなければならないため）。

```typescript
// app/books/error.tsx
"use client";

export default function BooksError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <div className="error-container">
      <h2>エラーが発生しました</h2>
      <p>{error.message}</p>
      <button onClick={() => reset()}>もう一度試す</button>
    </div>
  );
}
```

**画面にはこう表示される:** 書籍データの取得中にエラーが発生すると、「エラーが発生しました」という見出しとエラーメッセージ、そして「もう一度試す」ボタンが表示されます。ボタンを押すと、そのセグメントの再レンダリングが試みられます。

### 2.4 動的ルーティング（[id]）

URL の一部をパラメータとして受け取りたい場合、フォルダ名を角括弧で囲みます。

```
app/
  books/
    [id]/
      page.tsx    → /books/1, /books/2, /books/abc などにマッチ
```

```typescript
// app/books/[id]/page.tsx

// params はオブジェクトとして渡される
// Next.js 15 では params は Promise として渡されるため await が必要
export default async function BookDetailPage({
  params,
}: {
  params: Promise<{ id: string }>;
}) {
  const { id } = await params;

  return (
    <div>
      <h1>書籍詳細</h1>
      <p>書籍 ID: {id}</p>
      {/* 実際にはここで id を使ってデータベースから書籍情報を取得する */}
    </div>
  );
}
```

**画面にはこう表示される:**
- `/books/1` にアクセスすると「書籍 ID: 1」と表示されます。
- `/books/42` にアクセスすると「書籍 ID: 42」と表示されます。
- `/books/abc` にアクセスすると「書籍 ID: abc」と表示されます。

URL のその部分がそのまま `id` パラメータとして使えるわけです。

#### 複数の動的セグメント

```
app/
  users/
    [userId]/
      books/
        [bookId]/
          page.tsx   → /users/5/books/12 にマッチ
```

```typescript
// ==========================================================================
// ファイルパス: app/users/[userId]/books/[bookId]/page.tsx
// 役割      : URL の中の userId と bookId を取り出して表示するページ
// ----------------------------------------------------------------------------
// このファイルは Next.js が「動的セグメントを含むページ」と認識する。
// /users/5/books/12 にアクセスされると userId="5", bookId="12" になる。
// ============================================================================

// (1) 関数本体に async を付けると、内部で await が使える。
//     コンポーネント関数を async にできるのは Server Component だけの特権。
//
// (2) 引数の Props 型は { params: Promise<{ ... }> }
//     Next.js 15 以降では params が Promise でラップされて渡される。
//     なので await で「中身の値」を取り出してから使う。
export default async function UserBookPage({
  params,
}: {
  params: Promise<{ userId: string; bookId: string }>;
}) {
  // (3) Promise を await してオブジェクトを取り出し、
  //     さらに分割代入で userId と bookId に展開する。
  //     URL が /users/5/books/12 なら userId = "5", bookId = "12" となる。
  //     ※ 文字列で来ることに注意。数値が必要なら Number(userId) で変換する。
  const { userId, bookId } = await params;

  // (4) 取り出した値を JSX 内に埋め込んで表示
  return (
    <div>
      <p>ユーザー ID: {userId}</p>
      <p>書籍 ID: {bookId}</p>
    </div>
  );
}
// ▼ /users/5/books/12 にアクセスしたときの画面表示:
//   ユーザー ID: 5
//   書籍 ID: 12
```

#### キャッチオールセグメント

```
app/
  docs/
    [...slug]/
      page.tsx   → /docs/a, /docs/a/b, /docs/a/b/c などにマッチ
```

```typescript
// ==========================================================================
// ファイルパス: app/docs/[...slug]/page.tsx
// 役割      : スラッシュ区切りで何階層でもマッチする「キャッチオール」ページ
// ----------------------------------------------------------------------------
// [...slug] のように ... を付けると、それ以降のすべてのパスが配列として渡る。
// /docs/react           → slug = ["react"]
// /docs/react/hooks     → slug = ["react", "hooks"]
// /docs/react/hooks/useEffect → slug = ["react", "hooks", "useEffect"]
// ============================================================================

export default async function DocsPage({
  params,
}: {
  // (1) slug は文字列の配列として渡る
  params: Promise<{ slug: string[] }>;
}) {
  // (2) Promise から slug 配列を取り出す
  const { slug } = await params;
  // 例: /docs/react/hooks/useEffect → slug = ["react", "hooks", "useEffect"]

  // (3) Array.prototype.join(" / ") で配列を文字列にする
  //     ["react", "hooks", "useEffect"].join(" / ") → "react / hooks / useEffect"
  return (
    <div>
      <p>パス: {slug.join(" / ")}</p>
    </div>
  );
}
// ▼ /docs/react/hooks/useEffect にアクセスしたときの画面表示:
//   パス: react / hooks / useEffect
```

---

## 3. Server Components vs Client Components

### 3.1 概念の説明

Next.js App Router の最大の特徴は、**デフォルトですべてのコンポーネントが Server Component** であるということです。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; display: flex; gap: 12px; flex-wrap: wrap;">
  <div style="flex: 1; min-width: 280px; border: 2px solid #22c55e; border-radius: 12px; overflow: hidden; box-shadow: 0 2px 12px rgba(0,0,0,0.06);">
    <div style="background: #22c55e; color: #fff; padding: 10px 16px; font-weight: 700; font-size: 13px; text-align: center;">Server Components（デフォルト）</div>
    <div style="padding: 12px; display: flex; flex-direction: column; gap: 6px;">
      <div style="background: #dcfce7; border-radius: 6px; padding: 6px 10px; font-size: 12px; color: #14532d;">&#x2705; サーバー上で実行される</div>
      <div style="background: #dcfce7; border-radius: 6px; padding: 6px 10px; font-size: 12px; color: #14532d;">&#x2705; データベースに直接アクセス可能</div>
      <div style="background: #dcfce7; border-radius: 6px; padding: 6px 10px; font-size: 12px; color: #14532d;">&#x2705; JS バンドルに含まれない</div>
      <div style="background: #fee2e2; border-radius: 6px; padding: 6px 10px; font-size: 12px; color: #7f1d1d;">&#x274C; useState, useEffect 使用不可</div>
      <div style="background: #fee2e2; border-radius: 6px; padding: 6px 10px; font-size: 12px; color: #7f1d1d;">&#x274C; onClick 等のイベント処理不可</div>
      <div style="background: #fee2e2; border-radius: 6px; padding: 6px 10px; font-size: 12px; color: #7f1d1d;">&#x274C; ブラウザ API 使用不可</div>
    </div>
  </div>
  <div style="flex: 1; min-width: 280px; border: 2px solid #3b82f6; border-radius: 12px; overflow: hidden; box-shadow: 0 2px 12px rgba(0,0,0,0.06);">
    <div style="background: #3b82f6; color: #fff; padding: 10px 16px; font-weight: 700; font-size: 13px; text-align: center;">Client Components（'use client'）</div>
    <div style="padding: 12px; display: flex; flex-direction: column; gap: 6px;">
      <div style="background: #dbeafe; border-radius: 6px; padding: 6px 10px; font-size: 12px; color: #1e3a5f;">&#x1F310; ブラウザ上で実行される</div>
      <div style="background: #dbeafe; border-radius: 6px; padding: 6px 10px; font-size: 12px; color: #1e3a5f;">&#x1F310; API 経由でデータ取得</div>
      <div style="background: #dbeafe; border-radius: 6px; padding: 6px 10px; font-size: 12px; color: #1e3a5f;">&#x1F310; JS バンドルに含まれる</div>
      <div style="background: #dcfce7; border-radius: 6px; padding: 6px 10px; font-size: 12px; color: #14532d;">&#x2705; useState, useEffect 使用可能</div>
      <div style="background: #dcfce7; border-radius: 6px; padding: 6px 10px; font-size: 12px; color: #14532d;">&#x2705; onClick 等のイベント処理可能</div>
      <div style="background: #dcfce7; border-radius: 6px; padding: 6px 10px; font-size: 12px; color: #14532d;">&#x2705; ブラウザ API 使用可能</div>
    </div>
  </div>
</div>

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #e2e8f0; border-radius: 12px; overflow: hidden; box-shadow: 0 2px 12px rgba(0,0,0,0.06);">
  <div style="background: #1e40af; color: white; padding: 10px 20px; font-weight: 700; font-size: 13px; text-align: center;">Server Component の実行フロー</div>
  <div style="padding: 16px 20px;">
    <div style="display: flex; justify-content: space-around; margin-bottom: 12px; font-weight: 700; font-size: 13px; color: #1e40af;">
      <span style="flex: 1; text-align: center;">ブラウザ</span><span style="flex: 1; text-align: center;">サーバー</span><span style="flex: 1; text-align: center;">データベース</span>
    </div>
    <div style="background: #dbeafe; border-left: 3px solid #3b82f6; padding: 6px 12px; margin: 6px 20% 6px 33%; border-radius: 0 6px 6px 0; font-size: 12px; color: #1e3a5f;">Server Component が実行される</div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 6px;">
      <div style="flex: 1;"></div>
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;">データを直接取得</div>
      <div style="flex: 1; height: 2px; background: #3b82f6;"></div>
      <div style="flex: 1; font-size: 12px;">&#x25B6;</div>
    </div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 6px;">
      <div style="flex: 1;"></div>
      <div style="flex: 1; font-size: 12px;">&#x25C0;</div>
      <div style="flex: 1; height: 2px; background: #94a3b8; border-top: 2px dashed #94a3b8;"></div>
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;">データ返却</div>
    </div>
    <div style="background: #dbeafe; border-left: 3px solid #3b82f6; padding: 6px 12px; margin: 6px 20% 6px 33%; border-radius: 0 6px 6px 0; font-size: 12px; color: #1e3a5f;">HTML を生成</div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 6px;">
      <div style="flex: 1; font-size: 12px;">&#x25C0;</div>
      <div style="flex: 1; height: 2px; background: #94a3b8; border-top: 2px dashed #94a3b8;"></div>
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;">HTML を送信</div>
      <div style="flex: 1;"></div>
    </div>
    <div style="background: #dcfce7; border-left: 3px solid #22c55e; padding: 8px 12px; margin: 6px 66% 6px 0; border-radius: 0 6px 6px 0; font-size: 12px; color: #14532d; line-height: 1.7;">
      HTML を即座に表示<br/>
      Client Component の JS を読み込み<br/>
      Hydration（インタラクティブに）<br/>
      ボタンクリックなどが可能に
    </div>
  </div>
</div>

### 3.2 "use client" ディレクティブ

Client Component にするには、ファイルの **先頭** に `"use client"` と記述します。

```typescript
// ==========================================================================
// Server Component の例（"use client" を書かない＝デフォルト）
// ==========================================================================
// このファイルはサーバーでだけ実行される。
//   ✅ 関数を async にして await でDBやAPIにアクセスできる
//   ✅ 機密情報（APIキーなど）を扱える（ブラウザに送られない）
//   ❌ useState / useEffect / onClick などのインタラクティブ機能は使えない
//   ❌ window / document などブラウザ専用のAPIは使えない

export default function BookList() {
  // (1) 通常はここで await fetch(...) や DBクライアントを呼んでデータを取得する
  //     例: const books = await db.query("SELECT * FROM books");
  return <div>書籍一覧（サーバーで描画）</div>;
}
```

```typescript
// ==========================================================================
// Client Component の例（"use client" をファイル先頭に書く）
// ==========================================================================
// このディレクティブが書かれた瞬間から、このファイルとそこから import される
// すべてのコンポーネントはブラウザで動く扱いになる。
"use client";

// (1) Client Component ではブラウザ側で動く React のフックが使える
import { useState } from "react";

export default function SearchBar() {
  // (2) useState で「検索クエリ文字列」を状態として管理する
  //     [現在の値, 値を変える関数] = useState(初期値)
  //     初期値が "" なので最初の query は空文字。
  const [query, setQuery] = useState("");

  // (3) JSX で <input> を描画
  //     ・value={query}     : 入力欄に現在の状態を反映する（制御コンポーネント）
  //     ・onChange={(e)=>...}: ユーザーが文字を打つたびに呼ばれる関数
  //     ・e.target.value     : 入力欄の現在の値（input要素のテキスト）
  //     ・setQuery(...)      : 状態を更新 → React が再描画 → value も更新される
  return (
    <input
      type="text"
      value={query}
      onChange={(e) => setQuery(e.target.value)}
      placeholder="書籍を検索..."
    />
  );
}
```

> **▼ 動作:**
>
> - 検索バー（テキスト入力欄）が表示される
> - キーボードで「Re」と打つと query が "" → "R" → "Re" に更新され、画面の入力欄もそれに追従する
> - これは `useState` がブラウザのメモリに値を保持し、変更ごとに React が再描画するため
> - サーバーはこの入力プロセスにまったく関与しない

> **重要:** `"use client"` はそのファイルとそこから import されるすべてのモジュールを Client Component の境界として宣言します。つまり、Client Component の子コンポーネントも自動的に Client Component になります。

### 3.3 いつどちらを使うか（判断フローチャート）

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; text-align: center;">
  <div style="background: #1e293b; color: #fff; border-radius: 10px; padding: 10px 20px; font-size: 13px; font-weight: 700; display: inline-block;">コンポーネントを作成</div>
  <div style="color: #64748b; font-size: 18px; margin: 6px 0;">&#x2193;</div>
  <div style="background: #fef3c7; border: 2px solid #f59e0b; border-radius: 8px; padding: 10px 16px; font-size: 13px; font-weight: 600; display: inline-block;">useState / useEffect を使う？</div>
  <div style="display: flex; justify-content: center; gap: 80px; margin-top: 6px;">
    <div>
      <div style="color: #3b82f6; font-size: 12px; font-weight: 700;">はい &#x2193;</div>
      <div style="background: #3b82f6; color: #fff; border-radius: 8px; padding: 8px 16px; font-size: 12px; font-weight: 600; margin-top: 4px;">Client Component<br/><span style="font-weight: 400; font-size: 11px;">（'use client' を追加）</span></div>
    </div>
    <div>
      <div style="color: #64748b; font-size: 12px; font-weight: 700;">いいえ &#x2193;</div>
    </div>
  </div>
  <div style="color: #64748b; font-size: 18px; margin: 6px 0;">&#x2193;</div>
  <div style="background: #fef3c7; border: 2px solid #f59e0b; border-radius: 8px; padding: 10px 16px; font-size: 13px; font-weight: 600; display: inline-block;">onClick / onChange 等のイベントハンドラを使う？</div>
  <div style="display: flex; justify-content: center; gap: 80px; margin-top: 6px;">
    <div>
      <div style="color: #3b82f6; font-size: 12px; font-weight: 700;">はい &#x2192; Client Component</div>
    </div>
    <div>
      <div style="color: #64748b; font-size: 12px; font-weight: 700;">いいえ &#x2193;</div>
    </div>
  </div>
  <div style="color: #64748b; font-size: 18px; margin: 6px 0;">&#x2193;</div>
  <div style="background: #fef3c7; border: 2px solid #f59e0b; border-radius: 8px; padding: 10px 16px; font-size: 13px; font-weight: 600; display: inline-block;">ブラウザ API を使う？<br/><span style="font-weight: 400; font-size: 11px;">（window, document, localStorage など）</span></div>
  <div style="display: flex; justify-content: center; gap: 80px; margin-top: 6px;">
    <div>
      <div style="color: #3b82f6; font-size: 12px; font-weight: 700;">はい &#x2192; Client Component</div>
    </div>
    <div>
      <div style="color: #64748b; font-size: 12px; font-weight: 700;">いいえ &#x2193;</div>
    </div>
  </div>
  <div style="color: #64748b; font-size: 18px; margin: 6px 0;">&#x2193;</div>
  <div style="background: #fef3c7; border: 2px solid #f59e0b; border-radius: 8px; padding: 10px 16px; font-size: 13px; font-weight: 600; display: inline-block;">サードパーティライブラリで<br/>クライアント機能を使う？</div>
  <div style="display: flex; justify-content: center; gap: 80px; margin-top: 6px;">
    <div>
      <div style="color: #3b82f6; font-size: 12px; font-weight: 700;">はい &#x2192; Client Component</div>
    </div>
    <div>
      <div style="color: #64748b; font-size: 12px; font-weight: 700;">いいえ &#x2193;</div>
    </div>
  </div>
  <div style="color: #64748b; font-size: 18px; margin: 6px 0;">&#x2193;</div>
  <div style="background: #22c55e; color: #fff; border-radius: 10px; padding: 10px 20px; font-size: 13px; font-weight: 700; display: inline-block;">Server Component（デフォルトのまま）</div>
</div>

**原則:** できるだけ Server Component を使い、インタラクティブ性が必要な部分だけを Client Component にします。

| 用途 | Server Component | Client Component |
|---|:---:|:---:|
| データの取得・表示 | **推奨** | |
| データベースへの直接アクセス | **推奨** | 不可 |
| 機密情報（APIキーなど）へのアクセス | **推奨** | 不可 |
| 静的な UI の描画 | **推奨** | |
| フォーム入力 | | **必須** |
| クリックイベント | | **必須** |
| useState / useEffect | | **必須** |
| ブラウザ API（localStorage 等） | 不可 | **必須** |

### 3.4 書籍管理アプリでの使い分け例

書籍管理アプリの具体的なページで、どのようにコンポーネントを分割するか見てみましょう。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #e2e8f0; border-radius: 12px; padding: 20px; background: #f8fafc; box-shadow: 0 2px 12px rgba(0,0,0,0.06);">
  <div style="font-weight: 700; color: #1e40af; font-size: 14px; margin-bottom: 14px; text-align: center;">書籍一覧ページ /books &#x2015; コンポーネント構成</div>
  <div style="text-align: center;">
    <div style="background: #22c55e; color: #fff; border-radius: 8px; padding: 8px 16px; font-size: 12px; font-weight: 600; display: inline-block;">layout.tsx<br/><span style="font-weight: 400; font-size: 11px;">【Server】共通ヘッダー・フッター</span></div>
    <div style="color: #64748b; font-size: 16px; margin: 4px 0;">&#x2193;</div>
    <div style="background: #22c55e; color: #fff; border-radius: 8px; padding: 8px 16px; font-size: 12px; font-weight: 600; display: inline-block;">page.tsx<br/><span style="font-weight: 400; font-size: 11px;">【Server】DB から書籍一覧を取得</span></div>
    <div style="display: flex; justify-content: center; gap: 24px; margin-top: 8px;">
      <div style="text-align: center;">
        <div style="color: #64748b; font-size: 16px;">&#x2193;</div>
        <div style="background: #3b82f6; color: #fff; border-radius: 8px; padding: 8px 16px; font-size: 12px; font-weight: 600;">SearchBar.tsx<br/><span style="font-weight: 400; font-size: 11px;">【Client】検索入力欄</span></div>
      </div>
      <div style="text-align: center;">
        <div style="color: #64748b; font-size: 16px;">&#x2193;</div>
        <div style="background: #22c55e; color: #fff; border-radius: 8px; padding: 8px 16px; font-size: 12px; font-weight: 600;">BookList.tsx<br/><span style="font-weight: 400; font-size: 11px;">【Server】書籍カード一覧</span></div>
        <div style="color: #64748b; font-size: 16px; margin: 4px 0;">&#x2193;</div>
        <div style="background: #22c55e; color: #fff; border-radius: 8px; padding: 8px 16px; font-size: 12px; font-weight: 600;">BookCard.tsx<br/><span style="font-weight: 400; font-size: 11px;">【Server】書籍カード1枚</span></div>
        <div style="color: #64748b; font-size: 16px; margin: 4px 0;">&#x2193;</div>
        <div style="background: #3b82f6; color: #fff; border-radius: 8px; padding: 8px 16px; font-size: 12px; font-weight: 600;">FavoriteButton.tsx<br/><span style="font-weight: 400; font-size: 11px;">【Client】お気に入りボタン</span></div>
      </div>
    </div>
  </div>
  <div style="display: flex; gap: 12px; justify-content: center; margin-top: 14px; font-size: 11px;">
    <span><span style="display: inline-block; width: 10px; height: 10px; background: #22c55e; border-radius: 3px; margin-right: 4px;"></span>Server Component</span>
    <span><span style="display: inline-block; width: 10px; height: 10px; background: #3b82f6; border-radius: 3px; margin-right: 4px;"></span>Client Component</span>
  </div>
</div>

```typescript
// app/books/page.tsx（Server Component）
// データベースから直接書籍一覧を取得する

import { SearchBar } from "@/components/SearchBar";
import { BookList } from "@/components/BookList";

export default async function BooksPage() {
  // Server Component なので、サーバー上で直接データ取得ができる
  // この処理はブラウザには送信されない
  const response = await fetch("https://api.example.com/books", {
    cache: "no-store", // 常に最新データを取得（SSR）
  });
  const books = await response.json();

  return (
    <div>
      <h1>書籍一覧</h1>
      {/* Client Component: ユーザーの入力を受け付ける */}
      <SearchBar />
      {/* Server Component: 取得したデータを表示する */}
      <BookList books={books} />
    </div>
  );
}
```

```typescript
// components/SearchBar.tsx（Client Component）
"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";

export function SearchBar() {
  const [query, setQuery] = useState("");
  const router = useRouter();

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    // 検索クエリ付きで遷移
    router.push(`/books?q=${encodeURIComponent(query)}`);
  };

  return (
    <form onSubmit={handleSearch}>
      <input
        type="text"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        placeholder="タイトルで検索..."
      />
      <button type="submit">検索</button>
    </form>
  );
}
```

```typescript
// components/BookCard.tsx（Server Component）

import { FavoriteButton } from "./FavoriteButton";

type Book = {
  id: string;
  title: string;
  author: string;
};

export function BookCard({ book }: { book: Book }) {
  return (
    <div className="book-card">
      <h3>{book.title}</h3>
      <p>{book.author}</p>
      {/* インタラクティブな部分だけ Client Component */}
      <FavoriteButton bookId={book.id} />
    </div>
  );
}
```

```typescript
// components/FavoriteButton.tsx（Client Component）
"use client";

import { useState } from "react";

export function FavoriteButton({ bookId }: { bookId: string }) {
  const [isFavorite, setIsFavorite] = useState(false);

  const handleClick = async () => {
    setIsFavorite(!isFavorite);
    // API を呼び出してお気に入り状態を保存
    await fetch(`/api/books/${bookId}/favorite`, {
      method: "POST",
      body: JSON.stringify({ favorite: !isFavorite }),
    });
  };

  return (
    <button onClick={handleClick}>
      {isFavorite ? "★ お気に入り済み" : "☆ お気に入り"}
    </button>
  );
}
```

**画面にはこう表示される:** ページ上部に検索バー、その下に書籍カードが並んで表示されます。各カードには書籍タイトル、著者名、お気に入りボタンがあります。検索バーに文字を入力するとリアルタイムに反映され、お気に入りボタンをクリックすると「☆ お気に入り」が「★ お気に入り済み」に切り替わります。

---

## 4. レイアウトとテンプレート

### 4.1 ルートレイアウト

`app/layout.tsx` は **ルートレイアウト** と呼ばれ、アプリケーション全体に適用されます。`<html>` タグと `<body>` タグを含む **唯一の** レイアウトです。

```typescript
// app/layout.tsx

import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";

// Google Fonts の Inter フォントを最適化して読み込む
const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: {
    template: "%s | 書籍管理アプリ",
    default: "書籍管理アプリ",
  },
  description: "あなたの読書記録を管理するアプリケーション",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="ja">
      <body className={inter.className}>
        {/* ここにヘッダーを配置 */}
        <header className="site-header">
          <nav>
            <a href="/">書籍管理アプリ</a>
          </nav>
        </header>

        {/* children に各ページの内容が入る */}
        <main className="site-main">{children}</main>

        {/* ここにフッターを配置 */}
        <footer className="site-footer">
          <p>&copy; 2024 書籍管理アプリ</p>
        </footer>
      </body>
    </html>
  );
}
```

> **ポイント:**
> - ルートレイアウトは **必須** です。削除するとエラーになります。
> - `<html>` と `<body>` タグはルートレイアウトにのみ記述します。
> - `metadata` オブジェクトで SEO 用のタイトル・説明を設定できます。
> - `template: "%s | 書籍管理アプリ"` により、子ページのタイトルが「ページ名 | 書籍管理アプリ」の形式になります。

### 4.2 ネストされたレイアウト

各ルートセグメント（フォルダ）にも `layout.tsx` を置くことができ、そのセグメント以下のページに適用されます。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; text-align: center; border: 1px solid #e2e8f0; border-radius: 12px; padding: 20px; background: #f8fafc; box-shadow: 0 2px 12px rgba(0,0,0,0.06);">
  <div style="font-weight: 700; color: #1e40af; font-size: 14px; margin-bottom: 14px;">レイアウトのネスト構造</div>
  <div style="background: #ef4444; color: #fff; border-radius: 10px; padding: 10px 20px; font-size: 12px; font-weight: 600; display: inline-block;">app/layout.tsx<br/><span style="font-weight: 400; font-size: 11px;">（ルートレイアウト）ヘッダー + フッター</span></div>
  <div style="display: flex; justify-content: center; gap: 40px; margin-top: 8px;">
    <div style="text-align: center;">
      <div style="color: #64748b; font-size: 16px;">&#x2193;</div>
      <div style="background: #3b82f6; color: #fff; border-radius: 8px; padding: 8px 16px; font-size: 12px; font-weight: 600;">app/page.tsx<br/><span style="font-weight: 400; font-size: 11px;">トップページ</span></div>
    </div>
    <div style="text-align: center;">
      <div style="color: #64748b; font-size: 16px;">&#x2193;</div>
      <div style="background: #f59e0b; color: #fff; border-radius: 8px; padding: 8px 16px; font-size: 12px; font-weight: 600;">app/books/layout.tsx<br/><span style="font-weight: 400; font-size: 11px;">（書籍セクション）サイドバー</span></div>
      <div style="display: flex; justify-content: center; gap: 16px; margin-top: 8px;">
        <div style="text-align: center;">
          <div style="color: #64748b; font-size: 14px;">&#x2193;</div>
          <div style="background: #3b82f6; color: #fff; border-radius: 8px; padding: 7px 12px; font-size: 11px; font-weight: 600;">books/page.tsx<br/><span style="font-weight: 400;">書籍一覧</span></div>
        </div>
        <div style="text-align: center;">
          <div style="color: #64748b; font-size: 14px;">&#x2193;</div>
          <div style="background: #3b82f6; color: #fff; border-radius: 8px; padding: 7px 12px; font-size: 11px; font-weight: 600;">books/[id]/page.tsx<br/><span style="font-weight: 400;">書籍詳細</span></div>
        </div>
      </div>
    </div>
  </div>
</div>

```typescript
// app/books/layout.tsx
// /books 以下のすべてのページに適用されるレイアウト

import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "書籍管理",
};

export default function BooksLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="books-layout">
      {/* サイドバー */}
      <aside className="sidebar">
        <nav>
          <ul>
            <li><a href="/books">書籍一覧</a></li>
            <li><a href="/books/new">書籍を追加</a></li>
            <li><a href="/books/favorites">お気に入り</a></li>
          </ul>
        </nav>
      </aside>

      {/* メインコンテンツ */}
      <section className="content">
        {children}
      </section>
    </div>
  );
}
```

**画面にはこう表示される:**

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #e2e8f0; border-radius: 12px; overflow: hidden; box-shadow: 0 4px 20px rgba(0,0,0,0.08);">
  <div style="background: linear-gradient(135deg, #1e40af, #3b82f6); padding: 14px 24px; color: white; font-weight: 700; font-size: 15px;">📚 ヘッダー（ルートレイアウト）</div>
  <div style="display: flex; min-height: 200px;">
    <div style="width: 180px; background: #f1f5f9; padding: 16px; border-right: 1px solid #e2e8f0;">
      <div style="font-size: 13px; font-weight: 600; color: #475569; margin-bottom: 12px;">メニュー</div>
      <div style="font-size: 13px; color: #3b82f6; padding: 6px 0; cursor: pointer;">📖 書籍一覧</div>
      <div style="font-size: 13px; color: #64748b; padding: 6px 0; cursor: pointer;">➕ 書籍追加</div>
      <div style="font-size: 13px; color: #64748b; padding: 6px 0; cursor: pointer;">⭐ お気に入り</div>
    </div>
    <div style="flex: 1; padding: 24px; background: white;">
      <div style="border: 2px dashed #93c5fd; border-radius: 8px; padding: 32px; text-align: center; color: #3b82f6; font-size: 14px; font-weight: 600;">
        メインコンテンツ<br/><span style="font-size: 12px; color: #94a3b8; font-weight: 400;">（page.tsx の内容がここに表示される）</span>
      </div>
    </div>
  </div>
  <div style="background: #1e293b; padding: 12px 24px; color: #94a3b8; font-size: 12px; text-align: center;">フッター（ルートレイアウト）</div>
</div>

`/books` や `/books/123` にアクセスすると、ルートレイアウトのヘッダー・フッターに加えて、書籍セクション専用のサイドバーが表示されます。トップページ（`/`）にはサイドバーは表示されません。

### 4.3 書籍管理アプリのレイアウト構成

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; text-align: center; border: 1px solid #e2e8f0; border-radius: 12px; padding: 20px; background: #f8fafc; box-shadow: 0 2px 12px rgba(0,0,0,0.06);">
  <div style="font-weight: 700; color: #1e40af; font-size: 14px; margin-bottom: 14px;">書籍管理アプリ &#x2015; レイアウト構成</div>
  <div style="background: #ef4444; color: #fff; border-radius: 10px; padding: 10px 20px; font-size: 12px; font-weight: 600; display: inline-block;">app/layout.tsx<br/><span style="font-weight: 400; font-size: 11px;">ヘッダー（ロゴ + ナビ）/ フッター</span></div>
  <div style="display: flex; justify-content: center; gap: 32px; margin-top: 8px;">
    <div style="text-align: center;">
      <div style="color: #64748b; font-size: 16px;">&#x2193;</div>
      <div style="background: #22c55e; color: #fff; border-radius: 8px; padding: 8px 14px; font-size: 12px; font-weight: 600;">app/page.tsx<br/><span style="font-weight: 400; font-size: 11px;">ヒーロー画像 / 紹介テキスト</span></div>
    </div>
    <div style="text-align: center;">
      <div style="color: #64748b; font-size: 16px;">&#x2193;</div>
      <div style="background: #f59e0b; color: #fff; border-radius: 8px; padding: 8px 14px; font-size: 12px; font-weight: 600;">app/books/layout.tsx<br/><span style="font-weight: 400; font-size: 11px;">サイドバー（カテゴリ一覧）</span></div>
      <div style="display: flex; flex-wrap: wrap; justify-content: center; gap: 8px; margin-top: 8px;">
        <div>
          <div style="color: #64748b; font-size: 14px;">&#x2193;</div>
          <div style="background: #3b82f6; color: #fff; border-radius: 6px; padding: 6px 10px; font-size: 11px; font-weight: 600;">books/page.tsx<br/><span style="font-weight: 400;">検索 + カード一覧</span></div>
        </div>
        <div>
          <div style="color: #64748b; font-size: 14px;">&#x2193;</div>
          <div style="background: #3b82f6; color: #fff; border-radius: 6px; padding: 6px 10px; font-size: 11px; font-weight: 600;">books/new/page.tsx<br/><span style="font-weight: 400;">書籍追加フォーム</span></div>
        </div>
        <div>
          <div style="color: #64748b; font-size: 14px;">&#x2193;</div>
          <div style="background: #3b82f6; color: #fff; border-radius: 6px; padding: 6px 10px; font-size: 11px; font-weight: 600;">books/[id]/page.tsx<br/><span style="font-weight: 400;">書籍詳細</span></div>
        </div>
        <div>
          <div style="color: #64748b; font-size: 14px;">&#x2193;</div>
          <div style="background: #3b82f6; color: #fff; border-radius: 6px; padding: 6px 10px; font-size: 11px; font-weight: 600;">books/[id]/edit/page.tsx<br/><span style="font-weight: 400;">書籍編集フォーム</span></div>
        </div>
      </div>
    </div>
  </div>
</div>

---

## 5. ナビゲーション

### 5.1 Link コンポーネント

Next.js では、ページ間の遷移に HTML の `<a>` タグではなく、`next/link` の `<Link>` コンポーネントを使います。

```typescript
// components/Navigation.tsx
import Link from "next/link";

export function Navigation() {
  return (
    <nav>
      <ul>
        <li>
          <Link href="/">ホーム</Link>
        </li>
        <li>
          <Link href="/books">書籍一覧</Link>
        </li>
        <li>
          <Link href="/books/new">書籍を追加</Link>
        </li>
      </ul>
    </nav>
  );
}
```

**`<Link>` と `<a>` の違い:**

| 特性 | `<a>` タグ | `<Link>` コンポーネント |
|---|---|---|
| ページ遷移 | ページ全体をリロード | **クライアント側で遷移（高速）** |
| プリフェッチ | なし | **ビューポート内のリンクを自動プリフェッチ** |
| 状態の保持 | リセットされる | **レイアウトの状態が保持される** |
| JavaScript | 不要 | 必要 |

```typescript
// 動的ルートへのリンク
import Link from "next/link";

type Book = {
  id: string;
  title: string;
};

export function BookLink({ book }: { book: Book }) {
  return (
    <Link href={`/books/${book.id}`}>
      {book.title} の詳細を見る
    </Link>
  );
}
```

**画面にはこう表示される:** 「○○ の詳細を見る」というリンクテキストが表示されます。クリックすると、ページ全体がリロードされることなく、スムーズに書籍詳細ページに遷移します。ヘッダーやサイドバーのレイアウトはそのまま残り、メインコンテンツ部分だけが切り替わります。

> **プリフェッチについて:** `<Link>` コンポーネントはデフォルトで、ユーザーの画面（ビューポート）に表示されているリンク先のページを裏で先読みします。これにより、リンクをクリックした瞬間にページが表示されるような高速な体験が実現します。

### 5.2 useRouter フック

プログラムからページ遷移を行いたい場合（ボタンクリック後やフォーム送信後など）は、`useRouter` フックを使います。**Client Component でのみ使用可能です。**

```typescript
// components/BookForm.tsx
"use client";

import { useRouter } from "next/navigation"; // ⚠️ next/router ではない！

export function BookForm() {
  const router = useRouter();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    // 書籍を保存する処理（省略）

    // 保存後、書籍一覧ページに遷移
    router.push("/books");
  };

  return (
    <form onSubmit={handleSubmit}>
      {/* フォームの内容（省略） */}
      <button type="submit">保存</button>
    </form>
  );
}
```

**useRouter の主要メソッド:**

```typescript
"use client";

import { useRouter } from "next/navigation";

export function NavigationExample() {
  const router = useRouter();

  return (
    <div>
      {/* 指定したパスに遷移（履歴に追加） */}
      <button onClick={() => router.push("/books")}>
        書籍一覧へ
      </button>

      {/* 指定したパスに遷移（履歴を置換） */}
      <button onClick={() => router.replace("/books")}>
        書籍一覧へ（履歴を置換）
      </button>

      {/* ブラウザの「戻る」と同じ */}
      <button onClick={() => router.back()}>
        戻る
      </button>

      {/* ページのデータを再取得（再レンダリング） */}
      <button onClick={() => router.refresh()}>
        更新
      </button>
    </div>
  );
}
```

| メソッド | 説明 | 使用例 |
|---|---|---|
| `router.push(url)` | 指定した URL に遷移 | フォーム送信後のリダイレクト |
| `router.replace(url)` | 現在の履歴エントリを置換して遷移 | ログイン後のリダイレクト |
| `router.back()` | ブラウザの戻るボタンと同じ | 詳細ページから一覧に戻る |
| `router.refresh()` | 現在のページを再取得 | データ更新後の再読み込み |
| `router.prefetch(url)` | 指定した URL を先読み | 近く遷移しそうなページの準備 |

### 5.3 リダイレクト

#### Server Component でのリダイレクト

```typescript
// app/books/[id]/page.tsx
import { redirect, notFound } from "next/navigation";

export default async function BookDetailPage({
  params,
}: {
  params: Promise<{ id: string }>;
}) {
  const { id } = await params;
  const book = await fetchBook(id);

  // 書籍が見つからない場合は 404 ページを表示
  if (!book) {
    notFound();
  }

  // 非公開の書籍は一覧ページにリダイレクト
  if (book.isPrivate) {
    redirect("/books");
  }

  return (
    <div>
      <h1>{book.title}</h1>
    </div>
  );
}

async function fetchBook(id: string) {
  // データベースから書籍を取得する処理（仮）
  return { title: "サンプル書籍", isPrivate: false };
}
```

#### middleware.ts でのリダイレクト

`middleware.ts` をプロジェクトルートに置くことで、リクエストの段階でリダイレクトを行えます。認証チェックなどに便利です。

```typescript
// middleware.ts（プロジェクトルート）

import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";

export function middleware(request: NextRequest) {
  // 例: 未ログインユーザーを /books ページからリダイレクト
  const isLoggedIn = request.cookies.get("session");

  if (!isLoggedIn && request.nextUrl.pathname.startsWith("/books")) {
    return NextResponse.redirect(new URL("/login", request.url));
  }

  return NextResponse.next();
}

// ミドルウェアを適用するパスを指定
export const config = {
  matcher: ["/books/:path*"],
};
```

---

## 6. データフェッチ

### 6.1 Server Component でのデータ取得

Server Component ではコンポーネント関数を `async` にして、直接 `await` でデータを取得できます。これが Next.js App Router の最も強力な機能の1つです。

```typescript
// ============================================================================
// ファイルパス: app/books/page.tsx
// 役割      : 書籍一覧ページ。サーバーで全書籍を取ってきてHTMLにする。
// 種類      : Server Component（"use client" を書いていないので自動でこちら）
// ----------------------------------------------------------------------------
// Server Component の最大の魅力は「コンポーネント関数自体を async にできて、
// 直接 await でデータを取ってこられる」こと。
// そのデータはサーバーで埋め込まれた状態でブラウザに届くので、
// 初回表示が速く、SEOにも有利。
// ============================================================================


// (1) 書籍データ1件分の「形」を定義
//     Book 型を1度ここで決めておくと、map のコールバック内などで
//     book.〇〇 と書いたとき VS Code が補完してくれる。
type Book = {
  id: string;             // 一意なID
  title: string;          // 書籍タイトル
  author: string;         // 著者名
  publishedYear: number;  // 出版年
};


// (2) APIから書籍配列を取ってくる関数
//     async を付けると、関数の中で await が使えるようになる。
//     戻り値は「Book型の配列が将来届く」を意味する Promise<Book[]>。
async function getBooks(): Promise<Book[]> {

  // fetch は Web標準のHTTP通信API。
  // 第2引数の cache オプションで Next.js のキャッシュ動作を指定する。
  const response = await fetch("https://api.example.com/books", {
    cache: "no-store",   // ← 毎リクエストで最新データを取得する（=SSR動作）
    // cache: "force-cache",      // ← ビルド時に1度だけ取得（=SSG動作）
    // next: { revalidate: 60 },  // ← 60秒ごとに再取得（=ISR動作）
  });

  // (2-1) 通信は成功したけどステータスが 4xx/5xx の場合は失敗扱いにする。
  // response.ok は status が 200〜299 のとき true、それ以外で false。
  if (!response.ok) {
    throw new Error("書籍データの取得に失敗しました");
  }

  // (2-2) JSON 文字列を JS オブジェクトに変換して返す
  return response.json();
}


// (3) ページコンポーネント本体
//     関数自体に async を付けられるのが Server Component の特権。
//     Client Component（"use client"あり）では関数本体を async にできない。
export default async function BooksPage() {

  // この行はサーバーで実行される。
  // 取得処理が終わるまでブラウザにはHTMLが送られない（その間 loading.tsx が表示される）。
  const books = await getBooks();

  // (4) 取れたデータを使ってJSXを組み立てる
  //     books.length は配列の要素数（書籍が3件なら3）。
  //     books.map((book) => ...) で1件ずつJSXに変換する。
  //     key={book.id} は React がリストの差分検出に使う必須の属性。
  return (
    <div>
      <h1>書籍一覧（{books.length}冊）</h1>
      <ul>
        {books.map((book) => (
          <li key={book.id}>
            <h3>{book.title}</h3>
            <p>著者: {book.author}（{book.publishedYear}年）</p>
          </li>
        ))}
      </ul>
    </div>
  );
}
```

**▼ APIが3件のデータを返した場合のブラウザ表示:**

```
書籍一覧（3冊）
 ・ リーダブルコード
   著者: Dustin Boswell（2012年）
 ・ プロを目指す人のためのTypeScript入門
   著者: 鈴木 僚太（2022年）
 ・ 達人プログラマー
   著者: David Thomas（2016年）
```

**▼ 通信失敗時:**

`throw new Error(...)` が走ると、Next.js は同じセグメント内の `error.tsx` を自動で表示します。`error.tsx` を作っていない場合は親のエラーバウンダリにバブリングします。

**▼ ブラウザに届くHTMLの様子（要点だけ抜粋）:**

```html
<div>
  <h1>書籍一覧（3冊）</h1>
  <ul>
    <li><h3>リーダブルコード</h3><p>著者: Dustin Boswell（2012年）</p></li>
    <li><h3>プロを目指す人のためのTypeScript入門</h3><p>著者: 鈴木 僚太（2022年）</p></li>
    <li><h3>達人プログラマー</h3><p>著者: David Thomas（2016年）</p></li>
  </ul>
</div>
```

**▼ 普通のReactとの最大の違い:**
クライアントサイドの useState/useEffect でデータを取りに行くのと違って、**最初からデータが入ったHTMLが返ってくる**ので、表示がカクカクしない・SEOに強い・JSバンドルも小さく済む。

#### fetch のキャッシュ戦略

```typescript
// SSR - リクエストのたびに最新データを取得
fetch(url, { cache: "no-store" });

// SSG - ビルド時にデータを取得し、キャッシュ（デフォルト）
fetch(url, { cache: "force-cache" });

// ISR - 60秒ごとにキャッシュを再検証
fetch(url, { next: { revalidate: 60 } });
```

| 戦略 | 説明 | 使用例 |
|---|---|---|
| `cache: "no-store"` | 毎回取得（SSR） | ユーザー固有のデータ |
| `cache: "force-cache"` | ビルド時に取得（SSG） | 変更の少ないデータ |
| `next: { revalidate: N }` | N秒ごとに再検証（ISR） | ニュース記事など |

### 6.2 Client Component でのデータ取得

Client Component では、従来の React と同じように `useEffect` と `useState` を使うか、サードパーティのデータ取得ライブラリ（SWR や TanStack Query）を使います。

```typescript
// components/BookSearch.tsx
"use client";

import { useState, useEffect } from "react";

type Book = {
  id: string;
  title: string;
  author: string;
};

export function BookSearch() {
  const [query, setQuery] = useState("");
  const [books, setBooks] = useState<Book[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // 検索クエリが空なら何もしない
    if (!query.trim()) {
      setBooks([]);
      return;
    }

    // デバウンス: 入力が止まって300ms後に検索を実行
    const timer = setTimeout(async () => {
      setIsLoading(true);
      setError(null);

      try {
        const response = await fetch(
          `/api/books/search?q=${encodeURIComponent(query)}`
        );
        if (!response.ok) {
          throw new Error("検索に失敗しました");
        }
        const data = await response.json();
        setBooks(data);
      } catch (err) {
        setError(err instanceof Error ? err.message : "エラーが発生しました");
      } finally {
        setIsLoading(false);
      }
    }, 300);

    // クリーンアップ: 次の入力が来たらタイマーをキャンセル
    return () => clearTimeout(timer);
  }, [query]);

  return (
    <div>
      <input
        type="text"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        placeholder="書籍を検索..."
      />

      {isLoading && <p>検索中...</p>}
      {error && <p className="error">{error}</p>}

      <ul>
        {books.map((book) => (
          <li key={book.id}>
            {book.title} - {book.author}
          </li>
        ))}
      </ul>
    </div>
  );
}
```

**画面にはこう表示される:** 検索入力欄が表示されます。ユーザーが「村上」と入力すると、入力が止まって300ミリ秒後に「検索中...」というメッセージが一瞬表示され、その後「村上春樹」の著書一覧がリスト形式で表示されます。

> **Server Component と Client Component のデータ取得の比較:**
>
> | 特性 | Server Component | Client Component |
> |---|---|---|
> | **コード量** | 少ない（async/await のみ） | 多い（useState + useEffect） |
> | **ローディング状態** | loading.tsx が自動管理 | 自分で管理が必要 |
> | **エラー処理** | error.tsx が自動管理 | 自分で管理が必要 |
> | **データの安全性** | APIキー等が露出しない | APIキーは使えない |
> | **SEO** | HTML にデータが含まれる | 初回は空 |
> | **リアルタイム更新** | ページ再読み込みが必要 | 可能 |

### 6.3 loading.tsx による読み込み状態

`loading.tsx` を配置するだけで、Server Component のデータ取得中に自動的にローディング UI が表示されます。

```typescript
// app/books/loading.tsx

export default function BooksLoading() {
  return (
    <div className="loading-container">
      {/* スケルトンUI: 実際のコンテンツと同じ形状の灰色の枠 */}
      <h1 className="skeleton" style={{ width: "200px", height: "32px" }} />

      <div className="book-grid">
        {/* 6個のスケルトンカード */}
        {Array.from({ length: 6 }).map((_, i) => (
          <div key={i} className="skeleton-card">
            <div
              className="skeleton"
              style={{ width: "100%", height: "200px" }}
            />
            <div
              className="skeleton"
              style={{ width: "80%", height: "20px", marginTop: "8px" }}
            />
            <div
              className="skeleton"
              style={{ width: "60%", height: "16px", marginTop: "4px" }}
            />
          </div>
        ))}
      </div>
    </div>
  );
}
```

**画面にはこう表示される:** データの読み込み中、見出しの位置に灰色のバーが1つ、その下に灰色のカード状のブロックが6個並びます。これらは実際のコンテンツと同じ大きさ・位置で表示されるため、読み込みが完了するとスムーズに実際のコンテンツに切り替わり、画面がガタつきません（この手法を **スケルトンUI** と呼びます）。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; border: 1px solid #e2e8f0; border-radius: 12px; overflow: hidden; box-shadow: 0 2px 12px rgba(0,0,0,0.06);">
  <div style="background: #1e40af; color: white; padding: 10px 20px; font-weight: 700; font-size: 13px; text-align: center;">loading.tsx による Streaming の流れ</div>
  <div style="padding: 16px 20px;">
    <div style="display: flex; justify-content: space-around; margin-bottom: 12px; font-weight: 700; font-size: 13px; color: #1e40af;">
      <span style="flex: 1; text-align: center;">ユーザー</span><span style="flex: 1; text-align: center;">Next.js</span><span style="flex: 1; text-align: center;">データベース</span>
    </div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 6px;">
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;">/books にアクセス</div>
      <div style="flex: 1; height: 2px; background: #3b82f6;"></div>
      <div style="flex: 1; font-size: 12px;">&#x25B6;</div>
      <div style="flex: 1;"></div>
    </div>
    <div style="background: #dbeafe; border-left: 3px solid #3b82f6; padding: 6px 12px; margin: 6px 20% 6px 33%; border-radius: 0 6px 6px 0; font-size: 12px; color: #1e3a5f;">loading.tsx を即座に返す</div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 6px;">
      <div style="flex: 1; font-size: 12px;">&#x25C0;</div>
      <div style="flex: 1; height: 2px; background: #94a3b8; border-top: 2px dashed #94a3b8;"></div>
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;">スケルトン UI</div>
      <div style="flex: 1;"></div>
    </div>
    <div style="background: #fef3c7; border-left: 3px solid #f59e0b; padding: 6px 12px; margin: 6px 60% 6px 0; border-radius: 0 6px 6px 0; font-size: 12px; color: #92400e;">スケルトン UI を表示中...</div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 6px;">
      <div style="flex: 1;"></div>
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;">データ取得</div>
      <div style="flex: 1; height: 2px; background: #3b82f6;"></div>
      <div style="flex: 1; font-size: 12px;">&#x25B6;</div>
    </div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 6px;">
      <div style="flex: 1;"></div>
      <div style="flex: 1; font-size: 12px;">&#x25C0;</div>
      <div style="flex: 1; height: 2px; background: #94a3b8; border-top: 2px dashed #94a3b8;"></div>
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;">データ返却</div>
    </div>
    <div style="background: #dbeafe; border-left: 3px solid #3b82f6; padding: 6px 12px; margin: 6px 20% 6px 33%; border-radius: 0 6px 6px 0; font-size: 12px; color: #1e3a5f;">page.tsx をレンダリング</div>
    <div style="display: flex; align-items: center; gap: 4px; margin-bottom: 6px;">
      <div style="flex: 1; font-size: 12px;">&#x25C0;</div>
      <div style="flex: 1; height: 2px; background: #94a3b8; border-top: 2px dashed #94a3b8;"></div>
      <div style="flex: 1; text-align: right; font-size: 12px; color: #334155;">実コンテンツ</div>
      <div style="flex: 1;"></div>
    </div>
    <div style="background: #dcfce7; border-left: 3px solid #22c55e; padding: 6px 12px; margin: 6px 60% 6px 0; border-radius: 0 6px 6px 0; font-size: 12px; color: #14532d;">スケルトン &#x2192; 実際のコンテンツに切替</div>
  </div>
</div>

---

## 7. Server Actions

### 7.1 Server Actions とは

Server Actions は、**Client Component から直接サーバー上の関数を呼び出せる** 仕組みです。API ルートを別途作成する必要がなく、フォーム処理やデータの変更（作成・更新・削除）をシンプルに書けます。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif;">
  <div style="border: 1px solid #e2e8f0; border-radius: 12px; padding: 16px; margin-bottom: 12px; background: #f8fafc;">
    <div style="font-weight: 700; color: #64748b; font-size: 12px; margin-bottom: 10px;">従来の方法</div>
    <div style="display: flex; align-items: center; justify-content: center; gap: 8px; flex-wrap: wrap;">
      <div style="background: #3b82f6; color: #fff; border-radius: 8px; padding: 8px 14px; text-align: center; font-size: 12px; font-weight: 600;">Client Component<br/><span style="font-weight: 400; font-size: 11px;">（フォーム）</span></div>
      <div style="color: #94a3b8; font-size: 11px; text-align: center;">fetch('/api/books')<br/>&#x2192;</div>
      <div style="background: #f59e0b; color: #fff; border-radius: 8px; padding: 8px 14px; text-align: center; font-size: 12px; font-weight: 600;">API Route<br/><span style="font-weight: 400; font-size: 11px;">（route.ts）</span></div>
      <div style="color: #94a3b8; font-size: 11px; text-align: center;">DB操作<br/>&#x2192;</div>
      <div style="background: #22c55e; color: #fff; border-radius: 8px; padding: 8px 14px; text-align: center; font-size: 12px; font-weight: 600;">データベース</div>
    </div>
  </div>
  <div style="border: 2px solid #8b5cf6; border-radius: 12px; padding: 16px; background: #faf5ff;">
    <div style="font-weight: 700; color: #7c3aed; font-size: 12px; margin-bottom: 10px;">Server Actions（新しい方法）</div>
    <div style="display: flex; align-items: center; justify-content: center; gap: 8px; flex-wrap: wrap;">
      <div style="background: #3b82f6; color: #fff; border-radius: 8px; padding: 8px 14px; text-align: center; font-size: 12px; font-weight: 600;">Client Component<br/><span style="font-weight: 400; font-size: 11px;">（フォーム）</span></div>
      <div style="color: #7c3aed; font-size: 11px; text-align: center; font-weight: 600;">関数を直接呼び出し<br/>&#x2192;</div>
      <div style="background: #8b5cf6; color: #fff; border-radius: 8px; padding: 8px 14px; text-align: center; font-size: 12px; font-weight: 600;">Server Action<br/><span style="font-weight: 400; font-size: 11px;">（サーバー上で実行）</span></div>
      <div style="color: #7c3aed; font-size: 11px; text-align: center; font-weight: 600;">DB操作<br/>&#x2192;</div>
      <div style="background: #22c55e; color: #fff; border-radius: 8px; padding: 8px 14px; text-align: center; font-size: 12px; font-weight: 600;">データベース</div>
    </div>
  </div>
</div>

Server Actions は `"use server"` ディレクティブで宣言します。

### 7.2 フォーム処理での利用

#### 基本的な Server Action

```typescript
// app/books/new/actions.ts
"use server";

// この関数はサーバー上でのみ実行される
// クライアントには関数の中身は送信されない
export async function createBook(formData: FormData) {
  const title = formData.get("title") as string;
  const author = formData.get("author") as string;
  const publishedYear = Number(formData.get("publishedYear"));

  // バリデーション
  if (!title || !author) {
    return {
      error: "タイトルと著者は必須です",
    };
  }

  // データベースに保存（例）
  // const book = await db.book.create({
  //   data: { title, author, publishedYear },
  // });

  console.log("書籍を追加:", { title, author, publishedYear });

  // 成功したら書籍一覧にリダイレクト
  // redirect("/books");
  return { success: true };
}
```

#### Server Component のフォーム（JavaScript 不要）

```typescript
// app/books/new/page.tsx（Server Component）

import { createBook } from "./actions";

export default function NewBookPage() {
  return (
    <div>
      <h1>書籍を追加</h1>

      {/* action 属性に Server Action を渡す */}
      {/* JavaScript が無効でも動作する（Progressive Enhancement） */}
      <form action={createBook}>
        <div>
          <label htmlFor="title">タイトル</label>
          <input
            type="text"
            id="title"
            name="title"
            required
          />
        </div>

        <div>
          <label htmlFor="author">著者</label>
          <input
            type="text"
            id="author"
            name="author"
            required
          />
        </div>

        <div>
          <label htmlFor="publishedYear">出版年</label>
          <input
            type="number"
            id="publishedYear"
            name="publishedYear"
          />
        </div>

        <button type="submit">追加する</button>
      </form>
    </div>
  );
}
```

**画面にはこう表示される:** 「書籍を追加」という見出しの下に、タイトル、著者、出版年の入力フィールドと「追加する」ボタンが表示されます。各フィールドに入力して「追加する」ボタンを押すと、フォームのデータがサーバーに送信され、`createBook` 関数がサーバー上で実行されます。

#### Client Component のフォーム（ローディング状態の管理）

```typescript
// components/BookFormClient.tsx
"use client";

import { useActionState } from "react";
import { createBook } from "@/app/books/new/actions";

export function BookFormClient() {
  const [state, formAction, isPending] = useActionState(createBook, null);

  return (
    <form action={formAction}>
      <div>
        <label htmlFor="title">タイトル</label>
        <input type="text" id="title" name="title" required />
      </div>

      <div>
        <label htmlFor="author">著者</label>
        <input type="text" id="author" name="author" required />
      </div>

      <div>
        <label htmlFor="publishedYear">出版年</label>
        <input type="number" id="publishedYear" name="publishedYear" />
      </div>

      {/* エラーメッセージの表示 */}
      {state?.error && (
        <p className="error-message">{state.error}</p>
      )}

      {/* 送信中はボタンを無効化 */}
      <button type="submit" disabled={isPending}>
        {isPending ? "追加中..." : "追加する"}
      </button>
    </form>
  );
}
```

**画面にはこう表示される:** フォームは前の例と同じ見た目ですが、「追加する」ボタンを押すと、ボタンのテキストが「追加中...」に変わり、ボタンがグレーアウトして再クリックできなくなります。バリデーションエラーがあれば、ボタンの上にエラーメッセージが赤字で表示されます。

### 7.3 書籍の追加/編集での利用予告

次章以降で、Server Actions を使って以下の機能を実装します:

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif;">
  <div style="border: 1px solid #e2e8f0; border-radius: 12px; overflow: hidden; box-shadow: 0 2px 12px rgba(0,0,0,0.06);">
    <div style="background: #1e40af; color: white; padding: 10px 20px; font-weight: 700; font-size: 13px; text-align: center;">書籍管理アプリの Server Actions</div>
    <div style="padding: 16px;">
      <div style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin-bottom: 16px;">
        <div style="background: #22c55e; color: #fff; border-radius: 8px; padding: 7px 14px; font-size: 12px; font-weight: 600;">createBook<br/><span style="font-weight: 400; font-size: 11px;">書籍の新規追加</span></div>
        <div style="background: #f59e0b; color: #fff; border-radius: 8px; padding: 7px 14px; font-size: 12px; font-weight: 600;">updateBook<br/><span style="font-weight: 400; font-size: 11px;">書籍情報の編集</span></div>
        <div style="background: #ef4444; color: #fff; border-radius: 8px; padding: 7px 14px; font-size: 12px; font-weight: 600;">deleteBook<br/><span style="font-weight: 400; font-size: 11px;">書籍の削除</span></div>
        <div style="background: #3b82f6; color: #fff; border-radius: 8px; padding: 7px 14px; font-size: 12px; font-weight: 600;">toggleFavorite<br/><span style="font-weight: 400; font-size: 11px;">お気に入り切替</span></div>
        <div style="background: #8b5cf6; color: #fff; border-radius: 8px; padding: 7px 14px; font-size: 12px; font-weight: 600;">searchBooks<br/><span style="font-weight: 400; font-size: 11px;">書籍の検索</span></div>
      </div>
      <div style="display: flex; gap: 8px; flex-wrap: wrap;">
        <div style="flex: 1; min-width: 180px; background: #f0fdf4; border: 1px solid #bbf7d0; border-radius: 8px; padding: 10px; font-size: 11px; line-height: 1.7; color: #14532d;">
          <div style="font-weight: 700; color: #22c55e; margin-bottom: 4px;">createBook</div>
          1. フォームデータの取得<br/>2. バリデーション<br/>3. Supabase に INSERT<br/>4. /books にリダイレクト
        </div>
        <div style="flex: 1; min-width: 180px; background: #fffbeb; border: 1px solid #fde68a; border-radius: 8px; padding: 10px; font-size: 11px; line-height: 1.7; color: #78350f;">
          <div style="font-weight: 700; color: #f59e0b; margin-bottom: 4px;">updateBook</div>
          1. フォームデータの取得<br/>2. バリデーション<br/>3. Supabase を UPDATE<br/>4. /books/[id] にリダイレクト
        </div>
        <div style="flex: 1; min-width: 180px; background: #fef2f2; border: 1px solid #fecaca; border-radius: 8px; padding: 10px; font-size: 11px; line-height: 1.7; color: #7f1d1d;">
          <div style="font-weight: 700; color: #ef4444; margin-bottom: 4px;">deleteBook</div>
          1. 確認チェック<br/>2. Supabase から DELETE<br/>3. /books にリダイレクト
        </div>
      </div>
    </div>
  </div>
</div>

> **予告:** 第6章「Supabase との連携」で、実際のデータベースと接続してこれらの Server Actions を完成させます。

---

## 8. 環境変数

### 8.1 .env.local の設定方法

Next.js では、プロジェクトルートに `.env.local` ファイルを作成して環境変数を設定します。

```bash
# .env.local（プロジェクトルートに作成）

# データベース接続情報（サーバーサイドのみ）
DATABASE_URL="postgresql://user:password@localhost:5432/bookapp"

# Supabase 接続情報（サーバーサイドのみ）
SUPABASE_URL="https://xxxxx.supabase.co"
SUPABASE_SERVICE_ROLE_KEY="eyJhbGciOiJIUzI1NiIsInR5cCI6..."

# Supabase 接続情報（クライアントサイドでも使用可能）
NEXT_PUBLIC_SUPABASE_URL="https://xxxxx.supabase.co"
NEXT_PUBLIC_SUPABASE_ANON_KEY="eyJhbGciOiJIUzI1NiIsInR5cCI6..."
```

> **重要:** `.env.local` は `.gitignore` に含まれているため、Git にコミットされません。APIキーやパスワードなどの機密情報を安全に管理できます。

### 8.2 NEXT_PUBLIC_ プレフィックス

環境変数のアクセス範囲は、変数名のプレフィックスによって決まります。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; display: flex; gap: 12px; flex-wrap: wrap;">
  <div style="flex: 1; min-width: 280px; border: 2px solid #f59e0b; border-radius: 12px; overflow: hidden;">
    <div style="background: #f59e0b; color: #fff; padding: 10px 16px; font-weight: 700; font-size: 13px; text-align: center;">NEXT_PUBLIC_ あり</div>
    <div style="padding: 12px;">
      <div style="background: #fffbeb; border-radius: 6px; padding: 6px 10px; font-size: 12px; font-weight: 600; color: #92400e; margin-bottom: 8px; text-align: center;">NEXT_PUBLIC_SUPABASE_URL</div>
      <div style="display: flex; flex-direction: column; gap: 4px;">
        <div style="background: #dcfce7; border-radius: 6px; padding: 5px 10px; font-size: 12px; color: #14532d;">&#x2705; Server Component</div>
        <div style="background: #dcfce7; border-radius: 6px; padding: 5px 10px; font-size: 12px; color: #14532d;">&#x2705; Client Component</div>
        <div style="background: #fee2e2; border-radius: 6px; padding: 5px 10px; font-size: 12px; color: #7f1d1d;">&#x26A0;&#xFE0F; ブラウザの JS に含まれる</div>
      </div>
    </div>
  </div>
  <div style="flex: 1; min-width: 280px; border: 2px solid #22c55e; border-radius: 12px; overflow: hidden;">
    <div style="background: #22c55e; color: #fff; padding: 10px 16px; font-weight: 700; font-size: 13px; text-align: center;">NEXT_PUBLIC_ なし</div>
    <div style="padding: 12px;">
      <div style="background: #f0fdf4; border-radius: 6px; padding: 6px 10px; font-size: 12px; font-weight: 600; color: #14532d; margin-bottom: 8px; text-align: center;">SUPABASE_SERVICE_ROLE_KEY</div>
      <div style="display: flex; flex-direction: column; gap: 4px;">
        <div style="background: #dcfce7; border-radius: 6px; padding: 5px 10px; font-size: 12px; color: #14532d;">&#x2705; Server Component</div>
        <div style="background: #fee2e2; border-radius: 6px; padding: 5px 10px; font-size: 12px; color: #7f1d1d;">&#x274C; Client Component（undefined）</div>
        <div style="background: #dcfce7; border-radius: 6px; padding: 5px 10px; font-size: 12px; color: #14532d;">&#x2705; ブラウザには絶対に送信されない</div>
      </div>
    </div>
  </div>
</div>

```typescript
// Server Component での使用
// app/books/page.tsx

export default async function BooksPage() {
  // ✅ どちらもアクセス可能
  const url = process.env.SUPABASE_URL;
  const serviceKey = process.env.SUPABASE_SERVICE_ROLE_KEY;
  const publicUrl = process.env.NEXT_PUBLIC_SUPABASE_URL;

  console.log(url);        // "https://xxxxx.supabase.co"
  console.log(serviceKey);  // "eyJhbGciOi..."
  console.log(publicUrl);   // "https://xxxxx.supabase.co"

  return <div>...</div>;
}
```

```typescript
// Client Component での使用
// components/SomeClient.tsx
"use client";

export function SomeClient() {
  // ✅ NEXT_PUBLIC_ 付きはアクセス可能
  const publicUrl = process.env.NEXT_PUBLIC_SUPABASE_URL;
  console.log(publicUrl); // "https://xxxxx.supabase.co"

  // ❌ NEXT_PUBLIC_ なしは undefined
  const serviceKey = process.env.SUPABASE_SERVICE_ROLE_KEY;
  console.log(serviceKey); // undefined（安全！）

  return <div>...</div>;
}
```

**使い分けの原則:**

| 環境変数の種類 | プレフィックス | 使用例 |
|---|---|---|
| **機密情報** | なし | `SUPABASE_SERVICE_ROLE_KEY`、`DATABASE_URL` |
| **公開しても安全な情報** | `NEXT_PUBLIC_` | `NEXT_PUBLIC_SUPABASE_URL`、`NEXT_PUBLIC_SUPABASE_ANON_KEY` |

### 8.3 Supabase の接続情報の管理

書籍管理アプリでは、Supabase を使います。環境変数の設定例は以下の通りです。

```bash
# .env.local

# Supabase プロジェクト URL（公開OK）
NEXT_PUBLIC_SUPABASE_URL="https://abcdefghijklm.supabase.co"

# Supabase 匿名キー（公開OK - Row Level Security で保護される）
NEXT_PUBLIC_SUPABASE_ANON_KEY="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."

# Supabase サービスロールキー（絶対に公開してはいけない）
SUPABASE_SERVICE_ROLE_KEY="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

```typescript
// lib/supabase/client.ts
// Client Component 用の Supabase クライアント

import { createBrowserClient } from "@supabase/ssr";

export function createClient() {
  return createBrowserClient(
    process.env.NEXT_PUBLIC_SUPABASE_URL!,
    process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!
  );
}
```

```typescript
// lib/supabase/server.ts
// Server Component 用の Supabase クライアント

import { createServerClient } from "@supabase/ssr";
import { cookies } from "next/headers";

export async function createClient() {
  const cookieStore = await cookies();

  return createServerClient(
    process.env.NEXT_PUBLIC_SUPABASE_URL!,
    process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!,
    {
      cookies: {
        getAll() {
          return cookieStore.getAll();
        },
        setAll(cookiesToSet) {
          try {
            cookiesToSet.forEach(({ name, value, options }) =>
              cookieStore.set(name, value, options)
            );
          } catch {
            // Server Component からの呼び出し時は
            // cookie のセットができないが、問題ない
          }
        },
      },
    }
  );
}
```

> **次章で詳しく解説:** Supabase のセットアップと実際の接続方法は、第5章「Supabase 入門」で詳しく扱います。ここでは環境変数の管理方法だけ理解しておいてください。

---

## 9. プロジェクト構成のベストプラクティス

### 9.1 ディレクトリ構成例

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; background: #1e293b; border-radius: 12px; padding: 20px; color: #e2e8f0; font-size: 13px; line-height: 1.9; box-shadow: 0 4px 16px rgba(0,0,0,0.15);">
  <div style="font-weight: 700; color: #f8fafc; font-size: 15px; margin-bottom: 10px;">&#x1F4C2; bookshelf-app/</div>
  <div style="padding-left: 20px;">
    <div><span style="background: #3b82f6; color: white; padding: 1px 8px; border-radius: 4px; font-size: 11px; font-weight: 700;">app</span> &#x1F4C1; app/ <span style="color: #64748b; font-size: 11px;">（ルーティング・ページ）</span></div>
    <div style="padding-left: 24px;">&#x1F4C4; layout.tsx</div>
    <div style="padding-left: 24px;">&#x1F4C4; page.tsx</div>
    <div style="padding-left: 24px;">&#x1F4C4; globals.css</div>
    <div style="padding-left: 24px;">&#x1F4C1; books/</div>
    <div style="padding-left: 48px;">&#x1F4C4; page.tsx</div>
    <div style="padding-left: 48px;">&#x1F4C4; loading.tsx</div>
    <div style="padding-left: 48px;">&#x1F4C4; error.tsx</div>
    <div style="padding-left: 48px;">&#x1F4C4; layout.tsx</div>
    <div style="padding-left: 48px;">&#x1F4C4; actions.ts</div>
    <div style="padding-left: 48px;">&#x1F4C1; new/</div>
    <div style="padding-left: 72px;">&#x1F4C4; page.tsx</div>
    <div style="padding-left: 48px;">&#x1F4C1; <span style="color: #fbbf24;">[id]</span>/</div>
    <div style="padding-left: 72px;">&#x1F4C4; page.tsx</div>
    <div style="padding-left: 72px;">&#x1F4C1; edit/</div>
    <div style="padding-left: 96px;">&#x1F4C4; page.tsx</div>
  </div>
  <div style="padding-left: 20px; margin-top: 4px;">
    <div><span style="background: #22c55e; color: white; padding: 1px 8px; border-radius: 4px; font-size: 11px; font-weight: 700;">components</span> &#x1F4C1; components/ <span style="color: #64748b; font-size: 11px;">（再利用可能なUI）</span></div>
    <div style="padding-left: 24px;">&#x1F4C1; ui/ <span style="color: #64748b; font-size: 11px;">（汎用UIパーツ）</span></div>
    <div style="padding-left: 48px;">&#x1F4C4; Button.tsx &#x2502; &#x1F4C4; Input.tsx</div>
    <div style="padding-left: 24px;">&#x1F4C1; books/ <span style="color: #64748b; font-size: 11px;">（書籍関連）</span></div>
    <div style="padding-left: 48px;">&#x1F4C4; BookCard.tsx &#x2502; &#x1F4C4; BookForm.tsx &#x2502; &#x1F4C4; SearchBar.tsx</div>
    <div style="padding-left: 24px;">&#x1F4C1; layout/ <span style="color: #64748b; font-size: 11px;">（レイアウト関連）</span></div>
    <div style="padding-left: 48px;">&#x1F4C4; Header.tsx &#x2502; &#x1F4C4; Footer.tsx &#x2502; &#x1F4C4; Sidebar.tsx</div>
  </div>
  <div style="padding-left: 20px; margin-top: 4px;">
    <div><span style="background: #f59e0b; color: white; padding: 1px 8px; border-radius: 4px; font-size: 11px; font-weight: 700;">lib</span> &#x1F4C1; lib/ <span style="color: #64748b; font-size: 11px;">（ユーティリティ）</span></div>
    <div style="padding-left: 24px;">&#x1F4C1; supabase/</div>
    <div style="padding-left: 48px;">&#x1F4C4; client.ts &#x2502; &#x1F4C4; server.ts</div>
    <div style="padding-left: 24px;">&#x1F4C4; utils.ts</div>
  </div>
  <div style="padding-left: 20px; margin-top: 4px;">
    <div><span style="background: #8b5cf6; color: white; padding: 1px 8px; border-radius: 4px; font-size: 11px; font-weight: 700;">types</span> &#x1F4C1; types/ <span style="color: #64748b; font-size: 11px;">（型定義）</span></div>
    <div style="padding-left: 24px;">&#x1F4C4; book.ts &#x2502; &#x1F4C4; user.ts</div>
  </div>
  <div style="padding-left: 20px; margin-top: 4px;">
    <div><span style="background: #ef4444; color: white; padding: 1px 8px; border-radius: 4px; font-size: 11px; font-weight: 700;">public</span> &#x1F4C1; public/ <span style="color: #64748b; font-size: 11px;">（静的ファイル）</span></div>
    <div style="padding-left: 24px;">&#x1F4C1; images/</div>
  </div>
  <div style="padding-left: 20px; margin-top: 8px; border-top: 1px solid #334155; padding-top: 8px; color: #94a3b8;">
    &#x1F4C4; .env.local &#x2502; &#x1F4C4; next.config.ts &#x2502; &#x1F4C4; tsconfig.json &#x2502; &#x1F4C4; package.json
  </div>
</div>

### 9.2 各ディレクトリの役割

| ディレクトリ | 役割 | 含まれるもの |
|---|---|---|
| `app/` | **ルーティングとページ** | page.tsx, layout.tsx, loading.tsx, error.tsx, Server Actions |
| `components/` | **再利用可能な UI コンポーネント** | ボタン、カード、フォーム、ヘッダー等 |
| `lib/` | **ビジネスロジックとユーティリティ** | DB接続、ヘルパー関数、外部API呼び出し |
| `types/` | **TypeScript 型定義** | アプリ全体で使う型（Book, User 等） |
| `public/` | **静的ファイル** | 画像、フォント、favicon 等 |

### 9.3 書籍管理アプリのファイル構成

以下は、書籍管理アプリの最終的なファイル構成の全体像です。

```typescript
// types/book.ts
// アプリ全体で使う書籍の型定義

export type Book = {
  id: string;
  title: string;
  author: string;
  published_year: number;
  description: string | null;
  cover_image_url: string | null;
  is_favorite: boolean;
  created_at: string;
  updated_at: string;
};

// フォーム送信時に使う型（id や日時は不要）
export type BookFormData = {
  title: string;
  author: string;
  published_year: number;
  description?: string;
};
```

```typescript
// lib/utils.ts
// 汎用ユーティリティ関数

/**
 * 日付文字列を日本語のフォーマットに変換する
 * @example formatDate("2024-01-15") → "2024年1月15日"
 */
export function formatDate(dateString: string): string {
  const date = new Date(dateString);
  return date.toLocaleDateString("ja-JP", {
    year: "numeric",
    month: "long",
    day: "numeric",
  });
}

/**
 * クラス名を結合する（falsy な値は除外）
 * @example cn("base", isActive && "active", "extra") → "base active extra"
 */
export function cn(...classes: (string | false | undefined | null)[]): string {
  return classes.filter(Boolean).join(" ");
}
```

```typescript
// components/ui/Button.tsx
// 汎用ボタンコンポーネント

type ButtonProps = {
  children: React.ReactNode;
  variant?: "primary" | "secondary" | "danger";
  disabled?: boolean;
  type?: "button" | "submit";
  onClick?: () => void;
};

export function Button({
  children,
  variant = "primary",
  disabled = false,
  type = "button",
  onClick,
}: ButtonProps) {
  const baseClass = "btn";
  const variantClass = `btn-${variant}`;

  return (
    <button
      type={type}
      className={`${baseClass} ${variantClass}`}
      disabled={disabled}
      onClick={onClick}
    >
      {children}
    </button>
  );
}
```

### パスエイリアスの設定

`@/` というパスエイリアスを使うと、ディレクトリの深さに関係なく、プロジェクトルートからの絶対パスで import できます。Next.js のプロジェクト作成時に自動で設定されます。

```typescript
// ❌ 相対パスでの import（ディレクトリが深いと地獄）
import { BookCard } from "../../../components/books/BookCard";
import { formatDate } from "../../../lib/utils";

// ✅ パスエイリアスでの import（常にわかりやすい）
import { BookCard } from "@/components/books/BookCard";
import { formatDate } from "@/lib/utils";
```

この設定は `tsconfig.json` に記述されています:

```json
{
  "compilerOptions": {
    "paths": {
      "@/*": ["./*"]
    }
  }
}
```

---

## 10. よくあるエラーと対処法

### エラー1: "useState" は Server Component で使用できない

```
Error: useState only works in Client Components. Add the "use client" directive
at the top of the file to use it.
```

**原因:** Server Component（デフォルト）で `useState` や `useEffect` を使おうとしている。

**対処法:** ファイルの先頭に `"use client"` を追加する。

```typescript
// ❌ エラーになる
import { useState } from "react";

export default function Counter() {
  const [count, setCount] = useState(0); // ← ここでエラー
  return <button onClick={() => setCount(count + 1)}>{count}</button>;
}

// ✅ 修正版
"use client"; // ← これを追加

import { useState } from "react";

export default function Counter() {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount(count + 1)}>{count}</button>;
}
```

---

### エラー2: "async/await" は Client Component で使用できない

```
Error: async/await is not yet supported in Client Components, only Server
Components.
```

**原因:** `"use client"` を付けたコンポーネントで `async` 関数としてエクスポートしている。

**対処法:** データ取得を Server Component に移動するか、`useEffect` を使う。

```typescript
// ❌ エラーになる
"use client";

export default async function BooksPage() {
  const books = await fetch("/api/books"); // ← async は Client Component で不可
  return <div>...</div>;
}

// ✅ 修正版 1: Server Component にする（"use client" を削除）
export default async function BooksPage() {
  const books = await fetch("https://api.example.com/books");
  return <div>...</div>;
}

// ✅ 修正版 2: Client Component のまま useEffect を使う
"use client";

import { useState, useEffect } from "react";

export default function BooksPage() {
  const [books, setBooks] = useState([]);

  useEffect(() => {
    fetch("/api/books")
      .then((res) => res.json())
      .then(setBooks);
  }, []);

  return <div>...</div>;
}
```

---

### エラー3: "next/router" と "next/navigation" の混同

```
Error: NextRouter was not mounted.
```

**原因:** App Router で `next/router` を import している。App Router では `next/navigation` を使う。

```typescript
// ❌ App Router では使えない
import { useRouter } from "next/router";

// ✅ App Router ではこちらを使う
import { useRouter } from "next/navigation";
```

---

### エラー4: metadata と "use client" の共存

```
Error: You are attempting to export "metadata" from a component marked with
"use client", which is unsupported.
```

**原因:** `"use client"` のファイルで `metadata` をエクスポートしている。メタデータは Server Component でのみエクスポート可能。

```typescript
// ❌ エラーになる
"use client";

export const metadata = { title: "書籍一覧" }; // ← Client Component では不可

// ✅ 修正版: metadata は Server Component（page.tsx や layout.tsx）に置く
// app/books/page.tsx（Server Component）
export const metadata = { title: "書籍一覧" };

export default function BooksPage() {
  return <div>...</div>;
}
```

---

### エラー5: 環境変数が undefined になる

**原因:** Client Component で `NEXT_PUBLIC_` プレフィックスのない環境変数を使っている。

```typescript
// ❌ Client Component では undefined になる
"use client";

const apiKey = process.env.API_SECRET_KEY; // undefined

// ✅ NEXT_PUBLIC_ を付ける（ただし機密情報には使わない！）
const publicUrl = process.env.NEXT_PUBLIC_API_URL; // 値が取得できる
```

> **注意:** 機密情報に `NEXT_PUBLIC_` を付けてはいけません。Server Action や API Route 経由でアクセスしてください。

---

### エラー6: "Text content does not match server-rendered HTML"（Hydration エラー）

```
Warning: Text content did not match. Server: "2024年3月15日" Client: "3/15/2024"
```

**原因:** サーバーとクライアントで異なる出力が生成されている。日時のフォーマットやランダム値でよく発生する。

```typescript
// ❌ サーバーとクライアントで結果が異なる可能性がある
export default function DateDisplay() {
  return <p>{new Date().toLocaleString()}</p>;
}

// ✅ 修正版 1: suppressHydrationWarning を使う
export default function DateDisplay() {
  return <p suppressHydrationWarning>{new Date().toLocaleString()}</p>;
}

// ✅ 修正版 2: Client Component にして useEffect で設定
"use client";

import { useState, useEffect } from "react";

export default function DateDisplay() {
  const [dateStr, setDateStr] = useState("");

  useEffect(() => {
    setDateStr(new Date().toLocaleString("ja-JP"));
  }, []);

  return <p>{dateStr}</p>;
}
```

---

### エラー7: fetch でのキャッシュに関する警告

```
Warning: fetch for "https://..." on "/" used "no-store" and
should not be called inside a layout or template.
```

**対処法:** `layout.tsx` 内では `cache: "no-store"` を避け、`page.tsx` で使う。

---

### エラーの調査方法（一般的なアドバイス）

1. **ターミナルのエラーメッセージを読む:** Next.js のエラーメッセージは非常に詳しく、多くの場合、解決策まで提示してくれます。
2. **ブラウザのコンソールを確認する:** 開発者ツール（F12）のConsoleタブにもエラーが表示されます。
3. **Server Component と Client Component の境界を確認する:** 多くのエラーは、この2つの境界での誤りが原因です。
4. **Next.js の公式ドキュメントを参照する:** [https://nextjs.org/docs](https://nextjs.org/docs) には詳細な説明とトラブルシューティングがあります。

---

## まとめ

この章では Next.js の基礎を広く学びました。

| トピック | 学んだこと |
|---|---|
| **Next.js とは** | React のフルスタックフレームワーク。SSR/SSG/CSR に対応 |
| **App Router** | ファイルベースルーティング。page.tsx, layout.tsx 等の特殊ファイル |
| **Server/Client Components** | デフォルトは Server。インタラクティブ性が必要なら "use client" |
| **レイアウト** | ルートレイアウトとネストされたレイアウトで共通UIを効率的に管理 |
| **ナビゲーション** | Link コンポーネント（宣言的）と useRouter フック（命令的） |
| **データフェッチ** | Server Component なら async/await で直接。Client は useEffect |
| **Server Actions** | "use server" でサーバー処理を直接呼び出し。フォーム処理に最適 |
| **環境変数** | NEXT_PUBLIC_ の有無でアクセス範囲が変わる |
| **プロジェクト構成** | app/, components/, lib/, types/ の4層構造 |

> **次の章では:** Supabase のセットアップを行い、実際のデータベースと接続します。書籍管理アプリのバックエンドを構築し、この章で学んだ Server Components や Server Actions を使ってデータの読み書きを実装していきます。

---

### 確認問題

以下の問いに答えて、理解度を確認してみましょう。

1. **Server Component と Client Component の違い** を3つ挙げてください。
2. 以下のコンポーネントは Server / Client のどちらにすべきですか？
   - データベースから書籍一覧を取得して表示するコンポーネント
   - 検索バー（ユーザーの入力をリアルタイムに反映）
   - 書籍カードの静的な表示（クリックイベントなし）
3. `NEXT_PUBLIC_API_KEY` と `API_SECRET_KEY` はそれぞれどこで使用できますか？
4. `layout.tsx` と `page.tsx` の違いは何ですか？
5. Server Actions を使う利点を2つ挙げてください。

<details>
<summary>解答を見る</summary>

1. **違い3つ:**
   - Server Component はサーバーで実行、Client Component はブラウザで実行
   - Server Component はデータベースに直接アクセスできる、Client Component はできない
   - Client Component は useState/useEffect が使える、Server Component は使えない

2. **Server / Client の判断:**
   - データベースから書籍一覧を取得 → **Server Component**（DBに直接アクセス可能）
   - 検索バー → **Client Component**（useState でリアルタイム入力を管理）
   - 書籍カードの静的表示 → **Server Component**（インタラクティブ性不要）

3. **環境変数のアクセス範囲:**
   - `NEXT_PUBLIC_API_KEY` → Server Component と Client Component の両方で使用可能
   - `API_SECRET_KEY` → Server Component のみ（Client Component では `undefined`）

4. **layout.tsx と page.tsx の違い:**
   - `layout.tsx` は複数のページで共有される外枠（ヘッダー、サイドバーなど）。ページ遷移しても状態が保持される
   - `page.tsx` はそのルート固有のメインコンテンツ。遷移のたびに再レンダリングされる

5. **Server Actions の利点:**
   - API ルートを別途作成する必要がなく、コードがシンプルになる
   - サーバー上で直接実行されるため、機密情報（DBの接続情報など）がクライアントに漏れない

</details>

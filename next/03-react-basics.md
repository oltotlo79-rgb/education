# 第3章: React の基礎

> この章では、React（リアクト：Meta社が開発したUI構築用JavaScriptライブラリ）の基本的な概念をゼロから学びます。React は**ユーザーインターフェース（UI：User Interface。ユーザーが見て操作する画面部分の総称）** を効率的に作るためのライブラリ（Library：特定の機能を提供するプログラム部品の集まり。自分のコードから呼び出して使う）です。
>
> **この章を読む前に頭に入れておきたいこと:**
>
> - **JSX**（ジェイエスエックス：JavaScript XMLの略）は、JavaScript の中に HTML 風の構文を書ける「拡張構文」です。ブラウザは JSX を直接理解できないため、ビルドツール（Vite, Babel など）が裏で `React.createElement(...)` という関数呼び出しに変換しています。つまり JSX は「見た目が HTML だけど、実体は JavaScript の関数呼び出し」と覚えてください。
> - **コンポーネント**（Component：UIの部品）は、画面の一部を表す関数です。「JSX を返す関数」を作って、それを別の関数から `<MyComponent />` のように呼び出すだけで、UI を組み立てられます。
> - **state**（ステート：状態）の値は **直接書き換えてはいけません**。React は state の更新を「参照が変わったかどうか」で判定するため、`obj.name = "..."` のような直接代入では再レンダリング（再描画）が起こりません。必ず `setXxx(...)` という更新関数を呼びます。
> - **Strict Mode**（ストリクトモード：開発時の不具合検出モード）が有効だと、開発中はコンポーネントの関数が**わざと2回呼ばれます**。「あれ、console.log が2回出る？」と驚くかもしれませんが、これは仕様です。本番ビルドでは1回しか呼ばれません。

### この章で学ぶこと

| 概念 | 一言で言うと | 身近な例え |
|------|-----------|-----------|
| **コンポーネント** | 画面の部品 | レゴブロック。小さな部品を組み合わせてページを作る |
| **JSX / TSX** | HTMLのようにUIを書く構文 | HTMLとJavaScriptを混ぜて書ける魔法の書き方 |
| **Props** | 親から子へのデータ受け渡し | 上司から部下への指示書。「このデータを使って表示して」 |
| **State** | コンポーネントの内部データ | 変化する黒板。書き換えると画面が自動で更新される |
| **イベント** | ユーザー操作への反応 | ボタンを押す→何かが起きる、という仕組み |
| **useEffect** | 画面描画以外の処理 | 画面が表示された後に行う「裏方の仕事」（API通信など） |
| **カスタムフック** | 共通ロジックの切り出し | よく使う手順をマニュアルにまとめておくこと |

> **Reactを学ぶ前に：** React は「宣言的UI」（Declarative UI：宣言的ユーアイ。「最終的にこう見えてほしい」を書くスタイル）という考え方を採用しています。これは「画面がどうあるべきか」を記述するスタイルで、jQuery（ジェイクエリー：古くからあるDOM操作用JavaScriptライブラリ）のように「画面のどこをどう操作するか」を1ステップずつ記述する「命令的UI」（Imperative UI：めいれいてきユーアイ）とは大きく異なります。最初は戸惑うかもしれませんが、慣れると非常に直感的に感じるようになります。
>
> 例えるなら、命令的UIは「友達に道を教えるとき、最初の交差点を右、次の信号を左、3軒目の家…」と1手ずつ指示する方法。宣言的UIは「○○駅前の郵便局」と最終目的地だけ伝え、行き方はカーナビ（=React）に任せる方法です。

---

## 目次

0. [前提知識: HTMLとDOMの超基礎](#0-前提知識-htmlとdomの超基礎)
1. [React とは](#1-react-とは)
2. [JSX / TSX](#2-jsx--tsx)
3. [コンポーネント](#3-コンポーネント)
4. [State (useState)](#4-state-usestate)
5. [イベントハンドリング](#5-イベントハンドリング)
6. [useEffect](#6-useeffect)
7. [カスタムフック](#7-カスタムフック)
8. [よくあるミスと対処法](#8-よくあるミスと対処法)

---

## 0. 前提知識: HTMLとDOMの超基礎

React は「画面を作る」ための仕組みです。「画面」とは内部的には **HTML** で記述された文書です。React のコードを読む前に、HTML と DOM について最低限おさえておきましょう。

### 0.1 HTMLって何？

HTML（HyperText Markup Language）は、**Webページの構造を記述する言語**です。`<タグ名>...</タグ名>` という形で、文章のどの部分が「見出し」で、どの部分が「段落」で、どこに「リンク」があるか、をコンピュータに教えます。

```html
<!DOCTYPE html>                                  <!-- ブラウザに「これはHTML5の文書です」と宣言する一行。HTMLファイルの先頭に必ず書く -->
<html>                                           <!-- ページ全体の最も外側のタグ。<html>...</html> の中にすべてが入る -->
  <head>                                         <!-- 文書の「メタ情報（タイトル・文字コード・読み込むCSS等）」を入れる場所。画面には表示されない -->
    <title>はじめてのページ</title>              <!-- ブラウザのタブ部分に表示される文字 -->
  </head>
  <body>                                         <!-- 画面に実際に表示される「本文」を入れる場所 -->
    <h1>こんにちは</h1>                          <!-- 一番大きな見出し（heading 1）。1ページに1つだけ書くのが原則 -->
    <p>これは段落（paragraph）です。</p>          <!-- <p> は paragraph（段落）の略。通常の文章ブロック -->
    <a href="https://example.com">リンク</a>     <!-- <a> はアンカー（anchor）。href 属性で飛び先URLを指定するクリック可能なリンク -->
  </body>
</html>
```

**▼ ブラウザで表示すると：**

```
┌──────────────────────────────────┐
│  こんにちは                       │ ← <h1> の部分が大きな見出し
│                                   │
│  これは段落（paragraph）です。    │ ← <p> の部分は普通の段落
│                                   │
│  リンク                           │ ← <a> の部分は青い下線付きリンク
└──────────────────────────────────┘
```

### 0.2 よく出てくるHTMLタグ

| タグ | 意味 | サンプル |
|------|------|----------|
| `<div>` | 汎用ブロック（箱） | `<div>...</div>` |
| `<span>` | 汎用インライン（行内） | `<span>太字</span>` |
| `<h1>`〜`<h6>` | 見出し（h1が一番大きい） | `<h1>タイトル</h1>` |
| `<p>` | 段落 | `<p>本文</p>` |
| `<a>` | リンク | `<a href="...">テキスト</a>` |
| `<ul>` `<li>` | 順序なしリスト | `<ul><li>項目</li></ul>` |
| `<button>` | ボタン | `<button>OK</button>` |
| `<input>` | 入力欄 | `<input type="text" />` |
| `<form>` | フォーム | `<form>...</form>` |
| `<img>` | 画像 | `<img src="..." alt="..." />` |

タグには **属性（attribute）** を付けられます。`<a href="https://example.com">` の `href="..."` の部分が属性です。

### 0.3 DOM（Document Object Model）

ブラウザは HTML を読み込むと、それを**ツリー構造のオブジェクト**として記憶します。これが **DOM** です。

```
html
 └─ body
     ├─ h1 ("こんにちは")
     ├─ p  ("これは段落です。")
     └─ a  ("リンク") [href="https://example.com"]
```

「DOM を操作する」とは、JavaScript からこのツリーを読み書きして画面を変更することです（例: `element.textContent = "新しい文字"` で文字を書き換える、`element.style.color = "red"` で色を変える、など）。React は **「DOM 操作を直接書かなくて済むようにする」** ためのライブラリ、と言い換えてもOKです。React に「画面はこういう状態であってほしい」と JSX で宣言すれば、内部で必要な DOM 操作を裏で勝手にやってくれます。

### 0.4 ブラウザの開発者ツールで触ってみる

Chrome や Edge で右クリック →「検証（Inspect）」を選ぶと、開発者ツール（DevTools）が開きます。「Elements」タブが今そのページの DOM ツリーです。**Console** タブで JavaScript を実行できます。

```javascript
// Consoleタブで実行
// document はブラウザが用意している「ページ全体を表すオブジェクト」。常にグローバルに存在する。
document.title            // ▶ ページタイトル（<title>タグの中身）を取得。文字列が返る
// querySelector はCSSセレクタで要素を1つ取得するメソッド。"h1" は「h1タグを探す」という意味。
// .textContent は「その要素のテキスト部分だけ」を取り出すプロパティ。
document.querySelector("h1").textContent  // ▶ h1の文字を取得
```

> **これだけ覚えれば次に進める:** HTMLは「タグで文書の構造を書くもの」、DOMは「ブラウザがそれを読み込んだ後のツリー状のデータ」、React は「DOMを直接いじらず、JSX という書き方で画面を宣言するライブラリ」。これだけ頭に入れて先へ進みましょう。

---

## 1. React とは

### 1.1 コンポーネントベースの考え方

React は **Facebook（現 Meta）** が開発した、ユーザーインターフェース（UI：画面）を構築するための JavaScript ライブラリです。2013年にオープンソース（誰でも無料で使える形）として公開され、現在では世界で最も使われているUIライブラリです。

React の最大の特徴は **「コンポーネント」**（Component：コンポーネント。UIの部品。ボタン、カード、ヘッダーなど、画面を構成する一つひとつの要素。React では「JSX を返す関数」として書く）という考え方です。

> **なぜReactが人気なの？** 従来のWeb開発では、HTMLファイル全体を書き換える必要がありました。React では、変化した部分だけを効率的に更新できるため、**高速で滑らかな画面更新**が可能です。また、一度作った部品（コンポーネント）を使い回せるため、開発効率も大幅に向上します。

コンポーネントとは、UI の一部分を独立した再利用可能なパーツとして定義したものです。レゴブロックのように、小さなパーツを組み合わせて画面全体を構築します。

例えば、書籍管理アプリを考えてみましょう。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; background: #f8fafc; border-radius: 12px; padding: 24px; border: 1px solid #e2e8f0; box-shadow: 0 2px 8px rgba(0,0,0,0.06);">
  <!-- Level 0: App -->
  <div style="text-align: center; margin-bottom: 12px;">
    <span style="display: inline-block; background: #1e40af; color: white; padding: 10px 24px; border-radius: 8px; font-weight: 700; font-size: 14px;">App（アプリ全体）</span>
  </div>
  <div style="text-align: center; color: #94a3b8; font-size: 14px; line-height: 1;">│</div>
  <div style="text-align: center; color: #94a3b8; font-size: 14px; margin-bottom: 8px;">├────────────────┼────────────────┤</div>
  <!-- Level 1: Header, Main, Footer -->
  <div style="display: flex; justify-content: center; gap: 20px; margin-bottom: 12px;">
    <div style="text-align: center;">
      <span style="display: inline-block; background: #10b981; color: white; padding: 8px 14px; border-radius: 8px; font-size: 12px; font-weight: 600;">Header（ヘッダー）</span>
      <div style="color: #94a3b8; font-size: 13px; margin-top: 4px;">├─────┤</div>
      <div style="display: flex; gap: 6px; margin-top: 4px; justify-content: center;">
        <span style="background: #e0f2fe; color: #1e40af; padding: 4px 10px; border-radius: 6px; font-size: 11px;">Logo</span>
        <span style="background: #e0f2fe; color: #1e40af; padding: 4px 10px; border-radius: 6px; font-size: 11px;">Navigation</span>
      </div>
    </div>
    <div style="text-align: center;">
      <span style="display: inline-block; background: #10b981; color: white; padding: 8px 14px; border-radius: 8px; font-size: 12px; font-weight: 600;">Main（メイン）</span>
      <div style="color: #94a3b8; font-size: 13px; margin-top: 4px;">├──────────┼──────────┤</div>
      <div style="display: flex; gap: 6px; margin-top: 4px; justify-content: center;">
        <span style="background: #e0f2fe; color: #1e40af; padding: 4px 10px; border-radius: 6px; font-size: 11px;">SearchBar</span>
        <span style="display: inline-block; background: #f59e0b; color: white; padding: 4px 10px; border-radius: 6px; font-size: 11px; font-weight: 600;">BookList</span>
        <span style="background: #e0f2fe; color: #1e40af; padding: 4px 10px; border-radius: 6px; font-size: 11px;">AddBookForm</span>
      </div>
      <!-- BookList children -->
      <div style="color: #94a3b8; font-size: 13px; margin-top: 6px;">↓ BookList の子要素</div>
      <div style="display: flex; gap: 6px; margin-top: 4px; justify-content: center; flex-wrap: wrap;">
        <span style="background: #ef4444; color: white; padding: 4px 10px; border-radius: 6px; font-size: 11px; font-weight: 600;">BookCard 1</span>
        <span style="background: #ef4444; color: white; padding: 4px 10px; border-radius: 6px; font-size: 11px; font-weight: 600;">BookCard 2</span>
        <span style="background: #ef4444; color: white; padding: 4px 10px; border-radius: 6px; font-size: 11px; font-weight: 600;">BookCard 3</span>
      </div>
      <!-- BookCard1 children -->
      <div style="color: #94a3b8; font-size: 13px; margin-top: 6px;">↓ BookCard の子要素</div>
      <div style="display: flex; gap: 6px; margin-top: 4px; justify-content: center;">
        <span style="background: #fecaca; color: #991b1b; padding: 4px 10px; border-radius: 6px; font-size: 11px;">BookImage</span>
        <span style="background: #fecaca; color: #991b1b; padding: 4px 10px; border-radius: 6px; font-size: 11px;">BookInfo</span>
        <span style="background: #fecaca; color: #991b1b; padding: 4px 10px; border-radius: 6px; font-size: 11px;">BookActions</span>
      </div>
    </div>
    <div style="text-align: center;">
      <span style="display: inline-block; background: #10b981; color: white; padding: 8px 14px; border-radius: 8px; font-size: 12px; font-weight: 600;">Footer（フッター）</span>
    </div>
  </div>
</div>

このように、画面を「コンポーネントの木構造（ツリー）」として捉えます。それぞれのコンポーネントは：

- **独立している**: 自分の状態（state）とロジックを持つ
- **再利用できる**: `BookCard` は何度でも使い回せる
- **組み合わせできる**: 小さなコンポーネントを組み合わせて大きなコンポーネントを作る

### 1.2 仮想DOM の仕組み

ブラウザが持つ **DOM（Document Object Model）** を直接操作すると、非常に遅くなります。React は「仮想DOM（Virtual DOM）」という仕組みを使って、この問題を解決します。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; display: flex; gap: 16px; flex-wrap: wrap; justify-content: center;">
  <!-- 従来の方法 -->
  <div style="flex: 1; min-width: 280px; background: #fef2f2; border-radius: 12px; padding: 20px; border: 1px solid #fecaca;">
    <div style="text-align: center; font-weight: 700; font-size: 14px; color: #991b1b; margin-bottom: 16px; padding-bottom: 8px; border-bottom: 2px solid #fecaca;">従来の方法</div>
    <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
      <div style="background: white; border: 1px solid #e5e7eb; padding: 8px 16px; border-radius: 8px; font-size: 12px; text-align: center; width: 85%;">データ変更</div>
      <div style="color: #94a3b8; font-size: 16px;">↓</div>
      <div style="background: white; border: 1px solid #e5e7eb; padding: 8px 16px; border-radius: 8px; font-size: 12px; text-align: center; width: 85%;">DOM全体を再描画</div>
      <div style="color: #94a3b8; font-size: 16px;">↓</div>
      <div style="background: white; border: 1px solid #e5e7eb; padding: 8px 16px; border-radius: 8px; font-size: 12px; text-align: center; width: 85%;">ブラウザがレイアウト再計算</div>
      <div style="color: #94a3b8; font-size: 16px;">↓</div>
      <div style="background: #ef4444; color: white; padding: 8px 16px; border-radius: 8px; font-size: 12px; font-weight: 600; text-align: center; width: 85%;">画面全体がちらつく・遅い</div>
    </div>
  </div>
  <!-- React の方法 -->
  <div style="flex: 1; min-width: 280px; background: #f0fdf4; border-radius: 12px; padding: 20px; border: 1px solid #bbf7d0;">
    <div style="text-align: center; font-weight: 700; font-size: 14px; color: #166534; margin-bottom: 16px; padding-bottom: 8px; border-bottom: 2px solid #bbf7d0;">React の方法</div>
    <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
      <div style="background: white; border: 1px solid #e5e7eb; padding: 8px 16px; border-radius: 8px; font-size: 12px; text-align: center; width: 85%;">データ変更（state更新）</div>
      <div style="color: #94a3b8; font-size: 16px;">↓</div>
      <div style="background: white; border: 1px solid #e5e7eb; padding: 8px 16px; border-radius: 8px; font-size: 12px; text-align: center; width: 85%;">新しい仮想DOMを作成</div>
      <div style="color: #94a3b8; font-size: 16px;">↓</div>
      <div style="background: white; border: 1px solid #e5e7eb; padding: 8px 16px; border-radius: 8px; font-size: 12px; text-align: center; width: 85%;">前回の仮想DOMと比較<br/><span style="color: #6b7280; font-size: 11px;">（差分検出 = Reconciliation）</span></div>
      <div style="color: #94a3b8; font-size: 16px;">↓</div>
      <div style="background: white; border: 1px solid #e5e7eb; padding: 8px 16px; border-radius: 8px; font-size: 12px; text-align: center; width: 85%;">変更箇所だけ実際のDOMに反映</div>
      <div style="color: #94a3b8; font-size: 16px;">↓</div>
      <div style="background: #10b981; color: white; padding: 8px 16px; border-radius: 8px; font-size: 12px; font-weight: 600; text-align: center; width: 85%;">最小限の更新で高速に描画</div>
    </div>
  </div>
</div>

もう少し詳しく見てみましょう。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; background: #f8fafc; border-radius: 12px; padding: 24px; border: 1px solid #e2e8f0; box-shadow: 0 2px 8px rgba(0,0,0,0.06);">
  <!-- Participants -->
  <div style="display: flex; justify-content: space-between; margin-bottom: 20px; gap: 4px;">
    <div style="background: #1e40af; color: white; padding: 6px 10px; border-radius: 8px; font-size: 11px; font-weight: 600; text-align: center; flex: 1;">ユーザー</div>
    <div style="background: #3b82f6; color: white; padding: 6px 10px; border-radius: 8px; font-size: 11px; font-weight: 600; text-align: center; flex: 1;">React<br/>コンポーネント</div>
    <div style="background: #3b82f6; color: white; padding: 6px 10px; border-radius: 8px; font-size: 11px; font-weight: 600; text-align: center; flex: 1;">仮想DOM</div>
    <div style="background: #3b82f6; color: white; padding: 6px 10px; border-radius: 8px; font-size: 11px; font-weight: 600; text-align: center; flex: 1;">差分検出<br/>エンジン</div>
    <div style="background: #10b981; color: white; padding: 6px 10px; border-radius: 8px; font-size: 11px; font-weight: 600; text-align: center; flex: 1;">実際のDOM</div>
    <div style="background: #10b981; color: white; padding: 6px 10px; border-radius: 8px; font-size: 11px; font-weight: 600; text-align: center; flex: 1;">ブラウザ</div>
  </div>
  <!-- Steps -->
  <div style="display: flex; flex-direction: column; gap: 6px;">
    <div style="display: flex; align-items: center; gap: 8px; padding: 8px 12px; background: #eff6ff; border-radius: 8px; border-left: 4px solid #3b82f6;">
      <span style="background: #1e40af; color: white; padding: 2px 8px; border-radius: 10px; font-size: 11px; font-weight: 700; flex-shrink: 0;">1</span>
      <span style="font-size: 12px; color: #1e3a5f;"><strong>ユーザー → コンポーネント</strong>：ボタンをクリック</span>
    </div>
    <div style="display: flex; align-items: center; gap: 8px; padding: 8px 12px; background: #eff6ff; border-radius: 8px; border-left: 4px solid #3b82f6;">
      <span style="background: #1e40af; color: white; padding: 2px 8px; border-radius: 10px; font-size: 11px; font-weight: 700; flex-shrink: 0;">2</span>
      <span style="font-size: 12px; color: #1e3a5f;"><strong>コンポーネント</strong>：state を更新</span>
    </div>
    <div style="display: flex; align-items: center; gap: 8px; padding: 8px 12px; background: #eff6ff; border-radius: 8px; border-left: 4px solid #3b82f6;">
      <span style="background: #1e40af; color: white; padding: 2px 8px; border-radius: 10px; font-size: 11px; font-weight: 700; flex-shrink: 0;">3</span>
      <span style="font-size: 12px; color: #1e3a5f;"><strong>コンポーネント → 仮想DOM</strong>：新しい仮想DOMツリーを生成</span>
    </div>
    <div style="display: flex; align-items: center; gap: 8px; padding: 8px 12px; background: #f0fdf4; border-radius: 8px; border-left: 4px solid #10b981;">
      <span style="background: #166534; color: white; padding: 2px 8px; border-radius: 10px; font-size: 11px; font-weight: 700; flex-shrink: 0;">4</span>
      <span style="font-size: 12px; color: #1e3a5f;"><strong>仮想DOM → 差分検出エンジン</strong>：前回の仮想DOMと比較</span>
    </div>
    <div style="display: flex; align-items: center; gap: 8px; padding: 8px 12px; background: #f0fdf4; border-radius: 8px; border-left: 4px solid #10b981;">
      <span style="background: #166534; color: white; padding: 2px 8px; border-radius: 10px; font-size: 11px; font-weight: 700; flex-shrink: 0;">5</span>
      <span style="font-size: 12px; color: #1e3a5f;"><strong>差分検出エンジン</strong>：変更された箇所を特定</span>
    </div>
    <div style="display: flex; align-items: center; gap: 8px; padding: 8px 12px; background: #f0fdf4; border-radius: 8px; border-left: 4px solid #10b981;">
      <span style="background: #166534; color: white; padding: 2px 8px; border-radius: 10px; font-size: 11px; font-weight: 700; flex-shrink: 0;">6</span>
      <span style="font-size: 12px; color: #1e3a5f;"><strong>差分検出エンジン → 実際のDOM</strong>：変更箇所のみ更新</span>
    </div>
    <div style="display: flex; align-items: center; gap: 8px; padding: 8px 12px; background: #fefce8; border-radius: 8px; border-left: 4px solid #eab308;">
      <span style="background: #a16207; color: white; padding: 2px 8px; border-radius: 10px; font-size: 11px; font-weight: 700; flex-shrink: 0;">7</span>
      <span style="font-size: 12px; color: #1e3a5f;"><strong>実際のDOM → ブラウザ</strong>：最小限の再描画</span>
    </div>
    <div style="display: flex; align-items: center; gap: 8px; padding: 8px 12px; background: #fefce8; border-radius: 8px; border-left: 4px solid #eab308;">
      <span style="background: #a16207; color: white; padding: 2px 8px; border-radius: 10px; font-size: 11px; font-weight: 700; flex-shrink: 0;">8</span>
      <span style="font-size: 12px; color: #1e3a5f;"><strong>ブラウザ → ユーザー</strong>：画面が更新される</span>
    </div>
  </div>
</div>

**ポイント**: 仮想DOM は JavaScript のオブジェクトに過ぎません。JavaScript オブジェクトの比較は、実際の DOM 操作よりもはるかに高速です。React がこの差分を計算し、必要最小限の DOM 操作だけを行うことで、パフォーマンスが向上します。

### 1.3 なぜ React を選んだのか

フロントエンドのフレームワーク/ライブラリには多くの選択肢があります。

| 特徴 | React | Vue | Angular | Svelte |
|------|-------|-----|---------|--------|
| 学習コスト | 中 | 低 | 高 | 低 |
| エコシステム | 非常に大きい | 大きい | 大きい | 成長中 |
| 求人数 | 非常に多い | 多い | 多い | 少なめ |
| TypeScript対応 | 良好 | 良好 | 標準搭載 | 良好 |
| コミュニティ | 最大級 | 大きい | 大きい | 成長中 |

**React を選ぶ理由:**

1. **巨大なエコシステム**: npm パッケージ、ツール、ライブラリが圧倒的に多い
2. **求人需要**: 日本でもグローバルでも、React エンジニアの需要は非常に高い
3. **学んだ知識の応用範囲が広い**: React Native（モバイルアプリ）、Next.js（フルスタック）など
4. **TypeScript との親和性**: 型安全な開発が自然にできる
5. **豊富な学習リソース**: 公式ドキュメント、書籍、動画、コミュニティが充実

---

## 2. JSX / TSX

### 2.1 JSX とは何か

**JSX（JavaScript XML：ジェイエスエックス）** は、JavaScript の中に HTML のようなコードを書ける構文拡張（言語の文法を拡張したもの）です。TypeScript で使う場合は **TSX**（TypeScript XML）と呼びます（ファイル拡張子が `.tsx`）。

JSX は実際にはブラウザが直接理解できるものではなく、ビルドツール（Vite, Webpack, Babel など、ソースコードを実行可能な形式に変換するツール）によって通常の JavaScript に変換されます。変換結果は「`React.createElement()` という関数を呼ぶだけのコード」になります。つまり JSX の `<h1>...</h1>` は内部的には「React 用のオブジェクトを作る関数呼び出し」に過ぎません。

```tsx
// JSX で書いたコード
// const = 再代入できない変数を宣言するキーワード（JavaScript ES2015以降）
// element = 変数名（自分でつけた名前）
// <h1>こんにちは、React！</h1> = JSX。HTML そっくりだが、これは JS 式（オブジェクト）。
const element = <h1>こんにちは、React！</h1>;

// ↓ ビルドツールによって変換される ↓

// 実際に実行される JavaScript コード
// React.createElement(タグ名, 属性オブジェクト, 子要素...) の形で React 要素を作る。
//   第1引数 "h1": どのタグを作るか
//   第2引数 null: 属性（className や onClick など）。今回はないので null
//   第3引数 "こんにちは、React！": 子要素（タグの中身）
const element = React.createElement("h1", null, "こんにちは、React！");
```

JSX のおかげで、UI の構造を直感的に記述できます。なお、ブラウザがコードを動かす前に「JSX → JavaScript」への変換が必ず走っている、というのは重要なポイントです。エラーメッセージや devtools の表示で `createElement` が出てきても驚かないようにしましょう。

### 2.2 基本的な構文

#### 最小の React コンポーネント

> **▼ このコードがやること（先に日本語で）:** 「画面に "Hello, World!" という見出しを1つ表示するだけ」の、**最小のコンポーネント**を作ります。ポイントは2つだけ——①コンポーネントは「**JSX（見た目）を `return` で返す関数**」であること、②その関数名は **大文字で始める**こと（Reactが「これは部品だ」と判断する目印）。この2つさえ守れば、どんなに小さくてもコンポーネントになります。

```tsx
// ==========================================================================
// 一番小さい React コンポーネントの例
// ==========================================================================
// function = JS の関数宣言キーワード
// App     = 関数（コンポーネント）の名前。React では「大文字で始まる関数」が
//           自動的にコンポーネントとして扱われる。小文字始まりだと普通の関数。
// ()      = 引数の括弧（このコンポーネントは引数を受け取らないので空）
// { ... } = 関数の本体ブロック
function App() {
  // return = この関数が返す値を指定するキーワード
  // <h1>...</h1> は JSX。HTMLそっくりだが、内部では React.createElement に変換される。
  return <h1>Hello, World!</h1>;
}
```

> **▼ ブラウザでの表示:**
>
> 画面には **「Hello, World!」** という大きな見出しが1行表示されます。<h1>はHTMLで一番大きな見出しを意味するため、文字サイズが大きく、太字になります。

#### 複数の要素を返す場合

JSX では、**必ず1つのルート要素** で囲む必要があります。

```tsx
// ==========================================================================
// ❌ NG例 — 複数の要素を「並べただけ」では返せない
// ==========================================================================
// JSX は内部的には1個のオブジェクトを返す JavaScript 式に変換される。
// 並列に複数の要素を書くと「どれが戻り値？」と JS が判断できずエラーになる。
function App() {
  return (
    <h1>タイトル</h1>     {/* ← 1つ目のルート要素 */}
    <p>本文</p>           {/* ← 2つ目のルート要素 → ❌ エラー */}
  );
}
// ▼ エラー
// JSX expressions must have one parent element.
```

```tsx
// ==========================================================================
// ✅ 解決策1: <div> で1つに包む
// ==========================================================================
function App() {
  return (
    // (1) ルート要素は1つ。<div> でまとめる。
    //     JSX のインデントは見やすさのためで、必須ではない。
    <div>
      <h1>タイトル</h1>   {/* (2) <div> の中の子要素1つ目 */}
      <p>本文</p>         {/* (3) <div> の中の子要素2つ目 */}
    </div>
  );
}
```

> **▼ ブラウザでの表示:**
>
> ```
> タイトル              ← <h1> 大きな見出し
> 本文                  ← <p>  通常の段落
> ```
> ただし HTML 出力に余計な `<div>` タグが1つ増える点に注意。

```tsx
// ==========================================================================
// ✅ 解決策2: Fragment（<>...</>）で包む【こちらが推奨】
// ==========================================================================
// Fragment は「論理的に1つにまとめるが、DOMには何も出さない」特殊なラッパー。
function App() {
  return (
    <>                    {/* 開始タグ: 名前のないタグ */}
      <h1>タイトル</h1>
      <p>本文</p>
    </>                   {/* 終了タグ */}
  );
}
```

> **▼ ブラウザでの表示:**
>
> 上の `<div>` 版と見た目は同じだが、実際の HTML には `<div>` が出力されない。レイアウト崩れを起こしたくないときに便利。

#### HTML と JSX の違い

```tsx
// ==========================================================================
// HTML との細かい違い — JSX で書き換えが必要なポイント
// ==========================================================================
function App() {
  return (
    <div>
      {/*
        (1) class は予約語のため className を使う
        　  → JS の class（クラス定義）と区別するため。
      */}
      <div className="container">

        {/*
          (2) <label for=""> は htmlFor に書き換える
          　  → for も JS の予約語（for ループ）だから。
        */}
        <label htmlFor="name">名前:</label>
        <input id="name" />

        {/*
          (3) style 属性はオブジェクトで渡す（文字列ではない！）
          - 外側の {} は「JSXに JS の式を埋め込む」記号
          - 内側の {} は「JS のオブジェクトリテラル」
          - キーはキャメルケース: font-size → fontSize, margin-top → marginTop
          - 値は文字列（"16px"）でもOK。数値だけ渡すと px が自動で付くものもある
        */}
        <p style={{ color: "red", fontSize: "16px", marginTop: "10px" }}>
          赤いテキスト
        </p>

        {/*
          (4) コメントは {/* ... */} 形式
          普通の HTML コメント <!-- ... --> は JSX では書けない。
        */}

        {/*
          (5) 自己閉じタグは「/」で必ず閉じる必要がある
          HTML では <br>, <img src="..."> でも OK だが、JSX は厳密。
        */}
        <br />
        <img src="/logo.png" alt="ロゴ" />
        <input type="text" />
      </div>
    </div>
  );
}
```

> **▼ ブラウザでの表示:**
>
> ```
> 名前: [               ] ← input ボックス
>
> 赤いテキスト              ← color:red, font-size:16px で表示
>
> [ロゴ画像]
>
> [               ]        ← 別の input ボックス
> ```

| HTML | JSX/TSX |
|------|---------|
| `class` | `className` |
| `for` | `htmlFor` |
| `style="color: red"` | `style={{ color: "red" }}` |
| `tabindex` | `tabIndex` |
| `onclick` | `onClick` |
| `<!-- コメント -->` | `{/* コメント */}` |

### 2.3 JavaScript 式の埋め込み

JSX の中で `{}` を使うと、JavaScript の式を埋め込めます。

> **▼ このコードがやること（先に日本語で）:** 見た目（JSX）の中に、**変数の値や計算結果を埋め込んで**表示します。やり方は「埋め込みたい場所を `{ }`（中カッコ）で囲む」だけ。`{ }` の中は"JavaScriptの世界"になり、変数・計算・関数の呼び出しなどを書けます。ただし**書けるのは「式（値になるもの）」だけ**で、`if` 文や `for` 文のような「文（命令）」は書けない、という1点に注意します（理由はコード内のコメントで説明します）。

```tsx
// ==========================================================================
// JSX に JavaScript の式を埋め込むサンプル
// ==========================================================================
// JSX 中で { ... } を使うと、その中だけ「JavaScriptの式」を書ける。
// 注意: 文（statement）は書けない。式（expression）のみ。
//   ・OK : 変数、計算式、関数呼び出し、三項演算子、配列メソッド
//   ・NG : if 文、for 文、変数宣言（const x = ...）

function App() {
  // (1) コンポーネント関数の中（return より前）は普通の JS。
  //     ここで変数宣言や計算をしておく。
  const userName: string = "田中太郎";   // 文字列の変数
  const age: number = 25;                // 数値の変数
  const today: Date = new Date();        // 現在時刻の Date オブジェクト

  // (2) return 内の JSX に { } で値を差し込む
  return (
    <div>
      {/* (a) 変数をそのまま埋め込む。
              <h1> の中身は「ようこそ、田中太郎さん！」になる。 */}
      <h1>ようこそ、{userName}さん！</h1>

      {/* (b) 計算式の結果を埋め込む。
              {age + 1} は 25+1 を計算してから 26 を表示する。 */}
      <p>年齢: {age}歳（来年は{age + 1}歳）</p>

      {/* (c) メソッド呼び出しも埋め込める。
              today.toLocaleDateString("ja-JP") は日付を「2026/3/16」形式に変換する。 */}
      <p>今日の日付: {today.toLocaleDateString("ja-JP")}</p>

      {/* (d) テンプレートリテラル（バッククォート）も式なのでOK。 */}
      <p>{`${userName}さんは${age}歳です`}</p>
    </div>
  );
}
```

> **▼ ブラウザでの表示（2026年5月10日に開いた場合の例）:**
>
> ```
> ようこそ、田中太郎さん！      ← <h1> 大きな見出し
>
> 年齢: 25歳（来年は26歳）
>
> 今日の日付: 2026/5/10
>
> 田中太郎さんは25歳です
> ```

**重要**: `{}` の中に書けるのは **式（expression）** だけです。`if` 文や `for` 文などの **文（statement）** は書けません。

```tsx
// NG: if 文は式ではないので書けない
// 文（statement）は「値を持たない命令」。値を持たないものを { } の中に書くことはできない。
<p>{if (age >= 20) { "成人" }}</p>

// OK: 三項演算子は式なので書ける
// 「条件 ? 真の値 : 偽の値」は値を返す式。だから JSX 内に埋め込める。
<p>{age >= 20 ? "成人" : "未成年"}</p>
```

### 2.4 条件付きレンダリング

#### 三項演算子

```tsx
// ==========================================================================
// 三項演算子 ?: で表示を切り替える
// ==========================================================================
// 条件 ? 真の時の値 : 偽の時の値  という JS の式。
// JSX の中で「片方を表示・もう片方を表示」を切り替えるのに最適。

function UserStatus() {
  // (1) ユーザーがログイン済みかどうかを表す状態（ここでは固定値）
  const isLoggedIn: boolean = true;
  const userName: string = "田中太郎";

  return (
    <div>
      {/*
        (2) JSX 内の { ... } は JS の式。三項演算子は式なので使える。
            isLoggedIn が true なら <p>ようこそ...</p> を、
            false なら <p>ログインしてください。</p> を返す。
      */}
      {isLoggedIn ? (
        <p>ようこそ、{userName}さん！</p>
      ) : (
        <p>ログインしてください。</p>
      )}
    </div>
  );
}
```

> **▼ ブラウザでの表示:**
>
> - `isLoggedIn = true` のとき：`ようこそ、田中太郎さん！`
> - `isLoggedIn = false` のとき：`ログインしてください。`

#### &&（論理AND）演算子

条件を満たしたときだけ表示したい場合に便利です。

```tsx
// ==========================================================================
// && 演算子で「条件を満たしたときだけ」表示する
// ==========================================================================
// JS では: A && B
//   A が truthy（真っぽい値）なら B が評価され、その結果が返る。
//   A が falsy（false, 0, "", null, undefined）なら A 自身が返る。
// JSX では false / null / undefined は「何も表示しない」扱いになる。
// よって「条件 && JSX」と書けば「条件が true のときだけ JSX を表示」になる。

function Notification() {
  // 数値の状態（未読件数）と真偽値の状態（エラー有無）
  const unreadCount: number = 5;
  const hasError: boolean = false;

  return (
    <div>
      <h1>ダッシュボード</h1>

      {/*
        (a) unreadCount > 0 の結果は boolean。
            - 5 > 0 → true なので <p>...</p> が表示される
            - 0 > 0 → false なので何も表示されない
      */}
      {unreadCount > 0 && (
        <p className="notification">
          未読メッセージが{unreadCount}件あります。
        </p>
      )}

      {/*
        (b) hasError は false なので右辺は評価されず、何も表示されない
      */}
      {hasError && (
        <p className="error">エラーが発生しました。</p>
      )}
    </div>
  );
}
```

> **▼ ブラウザでの表示:**
>
> ```
> ダッシュボード                          ← <h1>
> 未読メッセージが5件あります。           ← (a) は表示される
>                                         ← (b) hasError=false なので非表示
> ```

**注意**: `&&` 演算子を使う際、左辺が数値の `0` だと `0` がそのまま画面に表示されてしまいます。

```tsx
// NG: count が 0 のとき、画面に「0」が表示されてしまう
// JSX は false/null/undefined は描画しないが、0 は数値として描画してしまう。
{count && <p>{count}件</p>}

// OK: 比較演算子を使えば安全
// count > 0 は必ず true/false の真偽値になるので、0件のときは何も表示されない。
{count > 0 && <p>{count}件</p>}
```

#### 複雑な条件分岐

```tsx
// type 文は TypeScript の型定義。Status は "loading" / "success" / "error" のいずれか
// の文字列しか入らない「ユニオン型」になる。タイポしただけで VS Code が赤線で教えてくれる。
type Status = "loading" | "success" | "error";

// 関数コンポーネント DataDisplay の定義
function DataDisplay() {
  // 現在の状態を表す変数。: Status は型注釈で「Status 型しか入らない」と宣言。
  const status: Status = "success";
  // 表示するデータ本体（成功時に出す文字列）
  const data: string = "データの内容";
  // エラー時のメッセージ。今回はエラーがないので空文字
  const errorMessage: string = "";

  // 関数で条件分岐をまとめる
  // renderContent は「画面に出す中身を返す関数」。
  // 戻り値の型 JSX.Element は「JSXの要素1つ」を意味する。
  const renderContent = (): JSX.Element => {
    // switch 文: 1つの値を複数の case と比較して分岐する制御構造
    switch (status) {
      case "loading":
        // 状態が "loading" のとき表示するJSXを返す
        return <p>読み込み中...</p>;
      case "error":
        // 状態が "error" のとき。className="error" でCSSクラスを指定（赤字スタイル等）
        // { } の中は JS 式。errorMessage 変数の値が埋め込まれる。
        return <p className="error">エラー: {errorMessage}</p>;
      case "success":
        // 状態が "success" のとき。{data} で変数の中身を表示。
        return <p>{data}</p>;
    }
  };

  // 親JSXで <h2> と renderContent() の戻り値を並べる。
  // {renderContent()} の () は「関数を呼び出す」記号。
  // 関数の戻り値（=JSX）がここに展開される。
  return (
    <div>
      <h2>データ表示</h2>
      {renderContent()}
    </div>
  );
}
```

> `status` が `"success"` の場合、画面には **「データ表示」** という見出しと **「データの内容」** というテキストが表示されます。

### 2.5 リストのレンダリング（map）

配列データを画面に表示するには、`map` メソッドを使います。

> **▼ このコードがやること（先に日本語で）:** 「りんご・バナナ…」という**文字列の配列**を、画面上の**箇条書きリスト（`<li>` の並び）**に変換して表示します。カギになるのが第2章で学んだ `map`——「**配列の各要素を1つずつ別のものに作り替えて、新しい配列を作る**」メソッドです。ここでは「各フルーツ名（文字列）」を「`<li>フルーツ名</li>`（リスト項目のJSX）」に作り替えています。さらに、各項目には `key`（要素を見分けるための目印）を必ず付ける、という決まりも出てきます（理由は後述の 8.3 で詳しく解説します）。

```tsx
// 関数コンポーネント FruitList を定義
function FruitList() {
  // fruits: 4つの文字列が入った配列。型 string[] は「文字列の配列」を意味する。
  const fruits: string[] = ["りんご", "バナナ", "みかん", "ぶどう"];

  return (
    <div>
      <h2>フルーツ一覧</h2>
      {/* <ul> は順序なしリスト（unordered list）。中に <li>（list item）を並べる */}
      <ul>
        {/*
          {fruits.map(...)} = JSX 中に「配列の各要素を JSX に変換した結果の配列」を埋め込む。
          map((要素, インデックス) => 戻り値) で全要素に対して関数を呼び、新しい配列を作る。
            fruit  = 配列の各要素（"りんご" など）
            index  = 配列内での位置（0, 1, 2, 3）
          アロー関数の () => (...) は「JSXを返すアロー関数」。
          { } の中で () で囲んでいるのは「return できる単一の式」にするため。
        */}
        {fruits.map((fruit, index) => (
          // key={index} は React がリスト要素を識別するための特別な属性。
          // 後述のとおり、本来は配列の index ではなく一意なIDを使うのが望ましい。
          // ここでは「並び替えがない静的なリスト」なので index でも問題ない。
          <li key={index}>{fruit}</li>
        ))}
      </ul>
    </div>
  );
}
```

> この結果、画面には **「フルーツ一覧」** という見出しと、箇条書きで以下のリストが表示されます:
>
> - りんご
> - バナナ
> - みかん
> - ぶどう

#### オブジェクト配列のレンダリング

```tsx
// ==========================================================================
// オブジェクト配列を map で展開して表示するサンプル
// ==========================================================================

// (1) Book 型を定義（書籍データ1件分の形）
//     type を使うのが React/TS では一般的。interface でも同じことができる。
type Book = {
  id: number;          // 一意なID（key に使う）
  title: string;       // 書籍タイトル
  author: string;      // 著者名
  price: number;       // 税込価格
  isAvailable: boolean;// 在庫があるか（true=あり, false=なし）
};

function BookList() {
  // (2) Book 型の配列を作る。
  //     型注釈 Book[] で「Book型のオブジェクトしか入らない配列」と宣言。
  //     1件でもプロパティが欠けていると VS Code が赤線で教えてくれる。
  const books: Book[] = [
    { id: 1, title: "React入門",       author: "田中太郎", price: 2800, isAvailable: true },
    { id: 2, title: "TypeScript実践",  author: "鈴木花子", price: 3200, isAvailable: false },
    { id: 3, title: "Next.js徹底解説", author: "佐藤一郎", price: 3500, isAvailable: true },
  ];

  return (
    <div>
      <h2>書籍一覧</h2>

      {/*
        (3) books.map((book) => JSX) で「配列の各要素を JSX に変換」する。
            戻り値は「JSX要素の配列」になり、それを React がそのまま並べて描画する。

            (4) key={book.id} は「リスト項目を識別するため」の特別な属性。
                React がリストの差分検出に使う。配列内で一意である必要がある。
                添え字 i ではなく、データのID（DBのprimary key）を使うのが鉄則。
      */}
      {books.map((book) => (
        <div key={book.id} className="book-card">
          <h3>{book.title}</h3>
          <p>著者: {book.author}</p>

          {/*
            (5) book.price.toLocaleString() は数値を「3桁ごとにカンマで区切った
                文字列」に変換するメソッド。2800 → "2,800"
          */}
          <p>価格: ¥{book.price.toLocaleString()}</p>

          {/*
            (6) 三項演算子で表示文字列と色を切り替える
                {" "} は意図的に空白を1文字入れたいとき使う書き方
          */}
          <p>
            状態:{" "}
            <span style={{ color: book.isAvailable ? "green" : "red" }}>
              {book.isAvailable ? "在庫あり" : "在庫なし"}
            </span>
          </p>
        </div>
      ))}
    </div>
  );
}
```

> **▼ ブラウザでの表示:**
>
> ```
> 書籍一覧
> ──────────────────────────────────
> React入門
> 著者: 田中太郎
> 価格: ¥2,800
> 状態: 在庫あり          ← 緑色
> ──────────────────────────────────
> TypeScript実践
> 著者: 鈴木花子
> 価格: ¥3,200
> 状態: 在庫なし          ← 赤色
> ──────────────────────────────────
> Next.js徹底解説
> 著者: 佐藤一郎
> 価格: ¥3,500
> 状態: 在庫あり          ← 緑色
> ```

**重要**: `key` プロパティ（key prop：キー プロップ。Reactがリスト要素を識別するための特別な属性）には、配列内で一意（ユニーク）な値を指定します。データベースから取得した `id` がベストです。`index` は最後の手段です（要素の並び替え・追加・削除で不具合の原因になります）。`key` は React 内部の差分計算（reconciliation）専用で、子コンポーネントから `props.key` として読むことはできません。

#### フィルタリングと map の組み合わせ

```tsx
// 「在庫があるものだけ」を表示する関数コンポーネント
function AvailableBookList() {
  // Book型の配列を用意（前のサンプルと同じ Book 型を使用）
  const books: Book[] = [
    { id: 1, title: "React入門", author: "田中太郎", price: 2800, isAvailable: true },
    { id: 2, title: "TypeScript実践", author: "鈴木花子", price: 3200, isAvailable: false },
    { id: 3, title: "Next.js徹底解説", author: "佐藤一郎", price: 3500, isAvailable: true },
  ];

  return (
    <div>
      <h2>在庫のある書籍</h2>
      {/*
        メソッドチェーン: 「.filter() の戻り値（新しい配列）」に「.map() を呼ぶ」と続けて書く形。
        - filter((要素) => 条件) は「条件が true の要素だけ残した新しい配列」を返す。
          ここでは book.isAvailable が true の書籍だけを残す。
        - 続く map((要素) => JSX) で「残った書籍を JSX に変換」。
        - filter/map とも元の配列を変更しない。新しい配列を返す（破壊的ではない）。
      */}
      {books
        .filter((book) => book.isAvailable)
        .map((book) => (
          // key={book.id} は必須。データのID（一意な値）を使うのがベスト。
          <div key={book.id}>
            <p>
              {/* {book.title} は文字列。- は普通の文字。
                  ¥ は通貨記号（普通の文字）。
                  {book.price.toLocaleString()} は「数値→3桁カンマ区切り文字列」変換。 */}
              {book.title} - ¥{book.price.toLocaleString()}
            </p>
          </div>
        ))}
    </div>
  );
}
```

> この結果、画面には **「在庫のある書籍」** という見出しと、在庫のある2冊だけが表示されます:
>
> React入門 - ¥2,800
>
> Next.js徹底解説 - ¥3,500

---

## 3. コンポーネント

### 3.1 関数コンポーネントの書き方

React では、**関数コンポーネント**（Function Component：関数として書かれたコンポーネント）が標準的な書き方です（**クラスコンポーネント**（Class Component：class 構文で書かれた古い書き方）は現在の React では推奨されていません。本書では扱いません）。

> **▼ このコードがやること（先に日本語で）:** 同じ「こんにちは！と表示する部品」を、**3通りの書き方**で示します。中身はどれも同じで、書き方（見た目）が違うだけです——①`function` を使う書き方、②`const 名前 = () => {...}` のアロー関数で書く書き方、③1行で返せるときに `return` と `{ }` を省く短い書き方。どれを使っても結果は同じなので、「いろんな書き方があるが、やっていることは同じ」と分かれば十分です。実務では②③のアロー関数の形をよく見かけます。

```tsx
// 最もシンプルな関数コンポーネント
// function 文で「JSXを返す関数」を作るだけでコンポーネントになる。
// 関数名 Greeting は大文字始まり（PascalCase）にする。これが React の決まり。
function Greeting() {
  // return = この関数の戻り値を指定。<h1>...</h1> の JSX をそのまま返す。
  return <h1>こんにちは！</h1>;
}

// アロー関数でも書ける
// const Greeting = ... で「Greeting という変数にアロー関数を入れる」書き方。
// () は引数（このコンポーネントは props を取らないので空）。
// => の右側 { ... } が関数本体。
const Greeting = () => {
  return <h1>こんにちは！</h1>;
};

// 1行で返せる場合は return を省略できる
// アロー関数で「=> 式」と書くと、その式が自動的に戻り値になる（即時 return）。
// 中括弧 { } を書かないことで「式を直接返す」モードになる。
const Greeting = () => <h1>こんにちは！</h1>;
```

> いずれの書き方でも、画面には **「こんにちは！」** という見出しが表示されます。

#### コンポーネントの使い方

```tsx
// 親コンポーネント App から子コンポーネント Greeting を呼び出す例
function App() {
  return (
    // <div> で全体を1つにまとめる（JSXの「ルート要素は1つ」ルール）
    <div>
      {/* <Greeting /> は自己閉じタグ。HTMLの <br /> と同じく / で閉じる。
          中身がない子コンポーネントは自己閉じタグで書くのが慣習。
          同じコンポーネントは何度でも書けて、それぞれが独立したインスタンスになる。 */}
      <Greeting />
      <Greeting />
      <Greeting />
    </div>
  );
}
```

> この結果、画面には **「こんにちは！」** が3回表示されます。同じコンポーネントを何度でも再利用できます。**1つのコンポーネント定義（設計図）から、いくつでもインスタンス（実際の表示）を作れる**のがコンポーネント指向のメリットです。

**命名規則**: コンポーネント名は必ず **大文字始まり（PascalCase：パスカルケース。単語の先頭をすべて大文字にする命名法）** にします。小文字で始まると、HTML タグとして認識されてしまいます。

```tsx
// OK: 大文字始まり → React コンポーネント
// JSX → JS変換時に React.createElement(Greeting, ...) と「変数 Greeting」として扱われる
<Greeting />
<BookCard />
<UserProfile />

// NG: 小文字始まり → HTML タグとして扱われる
// React.createElement("greeting", ...) になり、不明なHTMLタグとしてDOMに出力される
<greeting />  // <greeting> という存在しない HTML タグになる
```

### 3.2 Props の受け渡し（TypeScript での型定義含む）

**Props**（プロパティ：Properties。「親が子に渡す入力データ」のこと。HTMLタグの属性に似ている）は、親コンポーネントから子コンポーネントにデータを渡す仕組みです。値は **読み取り専用**で、子の中で書き換えてはいけません（書き換えても親には反映されないし、Reactの想定外の動作になります）。

> **propsドリリング**（Props Drilling：プロップスドリリング）について: 親 → 子 → 孫… と何階層も同じ props を「素通し」で渡し続ける現象です。階層が深くなるとコード保守が辛くなるため、後の章で扱う **Context**（コンテキスト）や状態管理ライブラリで解決します。
>
> **リフトアップ**（Lifting State Up：状態を持ち上げる）について: 兄弟コンポーネント間で同じ state を共有したいとき、共通の親に state を「持ち上げ」、props と更新関数を子に配るパターンです。例えば「親が `items` を持ち、子A が読み出し、子B が更新」という構図にします。

#### 基本的な Props

```tsx
// ==========================================================================
// Props の最小サンプル — 親が「name」という値を子に渡す
// ==========================================================================

// (1) 子コンポーネントが受け取る Props の「形」を type で定義する。
//     ここでは「name という string が必須で1つ」と宣言。
type GreetingProps = {
  name: string;
};

// (2) Greeting 関数コンポーネント
//     関数の引数で「props オブジェクト」を受け取る。
//     ここでは ({ name }: GreetingProps) と分割代入 + 型注釈で書いている。
//     等価なのは:
//       function Greeting(props: GreetingProps) {
//         const name = props.name;
//         ...
//       }
//     props.name と書かずに済むので分割代入のほうが好まれる。
function Greeting({ name }: GreetingProps) {
  // (3) 受け取った name を JSX 内に { } で埋め込んで使う
  return <h1>こんにちは、{name}さん！</h1>;
}

// (4) 親コンポーネント（App）が Greeting を3回使う
//     <Greeting name="田中" /> の name="田中" の部分が Props として子に渡る。
//     文字列リテラルは "..." または '...' で書く（{} はJS式埋め込み用）。
function App() {
  return (
    <div>
      <Greeting name="田中" />
      <Greeting name="鈴木" />
      <Greeting name="佐藤" />
    </div>
  );
}
```

> **▼ ブラウザでの表示:**
>
> ```
> こんにちは、田中さん！        ← <h1>
> こんにちは、鈴木さん！        ← <h1>
> こんにちは、佐藤さん！        ← <h1>
> ```
>
> 同じ Greeting コンポーネントが3回呼ばれ、それぞれ異なる name を受け取って描画されている。コンポーネントの再利用性がこれで実現できる。

#### さまざまな型の Props

```tsx
// Props の型を定義。1つのオブジェクトの「形」を表す type を作る。
type UserCardProps = {
  name: string;                    // 必須の文字列
  age: number;                     // 必須の数値
  email?: string;                  // ? はオプショナル（省略可能）。値が無いと型は string | undefined になる
  isAdmin: boolean;                // 必須の真偽値（true / false）
  hobbies: string[];               // 文字列の配列。[] は「配列」を表す
  onClickProfile: () => void;      // 関数型。「引数なし、戻り値なし（void）の関数」を表す
};

// 分割代入で props のプロパティを直接取り出す。
// 元の書き方: function UserCard(props: UserCardProps) { const { name, ... } = props; }
function UserCard({
  name,
  age,
  email,
  isAdmin,
  hobbies,
  onClickProfile,
}: UserCardProps) {
  return (
    // className は HTML の class 属性の JSX 版（class は JS の予約語のため）
    <div className="user-card">
      <h2>{name}</h2>           {/* { } で JS の式を JSX に埋め込む。name 変数の値が表示される */}
      <p>年齢: {age}歳</p>      {/* 「年齢: 25歳」のように文字列と変数が混ざる */}

      {/* オプショナルな props は存在チェック
          email が undefined（=falsy）なら && の右辺が評価されず、何も表示されない。
          email が文字列（=truthy）なら <p>...</p> が表示される。 */}
      {email && <p>メール: {email}</p>}

      {/* 三項演算子 条件 ? 真の値 : 偽の値。式なので JSX 内に書ける */}
      <p>権限: {isAdmin ? "管理者" : "一般ユーザー"}</p>

      <div>
        趣味:
        <ul>
          {/* hobbies.map で配列を <li> の配列に変換。
              key には今回 index を使っているが、本来はデータ固有のIDが望ましい。 */}
          {hobbies.map((hobby, index) => (
            <li key={index}>{hobby}</li>
          ))}
        </ul>
      </div>

      {/* onClick に「関数自身」を渡す。()を付けると即実行になりバグの元なので注意。
          onClickProfile は親から受け取った関数で、ボタンクリック時に呼び出される。 */}
      <button onClick={onClickProfile}>プロフィールを見る</button>
    </div>
  );
}

// 使い方
function App() {
  // ボタンクリック時の処理を関数として用意。alert はブラウザの組み込み関数。
  const handleClick = () => {
    alert("プロフィールページへ移動します");
  };

  return (
    // 子コンポーネントに props を渡す。
    // - 文字列は "..." または '...' で書く（ダブルクオート/シングルクオート）
    // - 数値・真偽値・配列・関数は { } で囲んでJS式として渡す
    <UserCard
      name="田中太郎"
      age={25}
      email="tanaka@example.com"
      isAdmin={false}
      hobbies={["読書", "プログラミング", "映画鑑賞"]}
      onClickProfile={handleClick}
    />
  );
}
```

> この結果、画面には以下のように表示されます:
>
> **田中太郎**
>
> 年齢: 25歳
>
> メール: tanaka@example.com
>
> 権限: 一般ユーザー
>
> 趣味:
> - 読書
> - プログラミング
> - 映画鑑賞
>
> 【プロフィールを見る】ボタン

#### デフォルト値を持つ Props

```tsx
// Button が受け取る Props の型定義
type ButtonProps = {
  label: string;                          // ボタンに表示する文字（必須）
  color?: string;                         // ? を付けると省略可能（オプショナル）
  size?: "small" | "medium" | "large";    // ユニオン型: 3つの文字列のどれかしか入らない
  disabled?: boolean;                     // クリック不可にするかどうか（省略可能）
};

// 分割代入で props を取り出し、同時に「= 値」でデフォルト値を設定。
// 親が color を指定しなかった場合は "blue" が使われる。
function Button({
  label,
  color = "blue",
  size = "medium",
  disabled = false,
}: ButtonProps) {
  // Record<キーの型, 値の型> は「オブジェクトの型」。
  // ここでは「キー: string, 値: string のオブジェクト」を表す。
  // ボタンサイズごとに padding（内側余白）の値を持たせている。
  const sizeStyles: Record<string, string> = {
    small: "8px 16px",
    medium: "12px 24px",
    large: "16px 32px",
  };

  return (
    <button
      // style はオブジェクトで指定。外側の {} が JSX、内側の {} が JSオブジェクトリテラル。
      // CSSプロパティ名はキャメルケース（background-color → backgroundColor）。
      style={{
        backgroundColor: color,                 // 背景色（props 由来）
        padding: sizeStyles[size],              // size に対応する余白文字列
        color: "white",                         // 文字色
        border: "none",                         // 枠線なし
        borderRadius: "4px",                    // 角を丸める
        opacity: disabled ? 0.5 : 1,            // 無効化されてたら半透明
        cursor: disabled ? "not-allowed" : "pointer", // マウスカーソルの形
      }}
      // disabled={disabled} は「JS変数の値をHTML属性にセット」する書き方。
      // boolean を渡せば true のとき disabled 属性が付き、false なら付かない。
      disabled={disabled}
    >
      {/* {label} は props の文字列をボタンの中身として表示 */}
      {label}
    </button>
  );
}

function App() {
  return (
    <div>
      {/* color, size, disabled を省略 → デフォルト値が使われる */}
      <Button label="デフォルト" />
      {/* color="red" size="large" を指定 */}
      <Button label="赤い大きなボタン" color="red" size="large" />
      {/* disabled だけ書くと自動的に disabled={true} と同じ意味になる（JSXのショートハンド） */}
      <Button label="無効化" disabled />
      <Button label="緑の小さなボタン" color="green" size="small" />
    </div>
  );
}
```

> この結果、画面には4つのボタンが表示されます:
>
> 1. **「デフォルト」** - 青色・中サイズのボタン
> 2. **「赤い大きなボタン」** - 赤色・大サイズのボタン
> 3. **「無効化」** - 青色・中サイズだが半透明でクリック不可のボタン
> 4. **「緑の小さなボタン」** - 緑色・小サイズのボタン

#### children Props

コンポーネントのタグで囲んだ中身は、`children`（子要素）として渡されます。これにより「外側の枠だけ用意して、中身は呼び出し側が自由に決められる」コンポーネントが作れます。

```tsx
// CardProps の型定義
type CardProps = {
  title: string;
  // React.ReactNode = 「React で描画できるもの全部」を表す特別な型。
  // 文字列、数値、JSX要素、その配列、null、undefined… すべて受け入れる。
  // children という名前は React の特別な予約名。タグで囲んだ中身が自動的にここに入る。
  children: React.ReactNode;
};

function Card({ title, children }: CardProps) {
  return (
    // インラインスタイルでカードの見た目を作る
    <div
      style={{
        border: "1px solid #ddd",        // 1px、実線、薄いグレーの枠線
        borderRadius: "8px",             // 角丸
        padding: "16px",                 // 内側の余白
        margin: "8px",                   // 外側の余白
      }}
    >
      {/* タイトル部分。borderBottom で下線、paddingBottom で下線との余白を確保 */}
      <h2 style={{ borderBottom: "1px solid #eee", paddingBottom: "8px" }}>
        {title}
      </h2>
      {/* {children} と書くと、親で <Card>...</Card> の間に書いた要素がここに展開される */}
      <div>{children}</div>
    </div>
  );
}

function App() {
  return (
    <div>
      {/* 開始タグと終了タグで囲んだ中身（<p>2つ）が children として渡される */}
      <Card title="お知らせ">
        <p>新機能がリリースされました！</p>
        <p>詳しくはこちらをご覧ください。</p>
      </Card>

      {/* 別の呼び出し。中身に画像と段落を入れている。 */}
      <Card title="プロフィール">
        {/* <img> は自己閉じタグ。src は画像URL、alt は代替テキスト（読み上げ・画像未表示時用） */}
        <img src="/avatar.png" alt="アバター" />
        <p>田中太郎</p>
      </Card>
    </div>
  );
}
```

> この結果、画面には2つのカードが表示されます:
>
> **【お知らせカード】** 枠線で囲まれた領域に「お知らせ」という見出しと、2行のテキスト
>
> **【プロフィールカード】** 枠線で囲まれた領域に「プロフィール」という見出しと、画像・名前

### 3.3 コンポーネントの分割の考え方

コンポーネントを分割する判断基準:

1. **再利用されるか**: 複数箇所で使う UI は別コンポーネントにする
2. **責務が明確か**: 1つのコンポーネントが1つの役割を持つ
3. **複雑すぎないか**: 1ファイルが 100行を超えたら分割を検討する
4. **独立してテストできるか**: テストしやすい単位に分ける

```
src/                              # ソースコードのルートフォルダ
├── components/                   # 再利用可能なUI部品を置く
│   ├── common/                   # 汎用コンポーネント（プロジェクト横断で使う）
│   │   ├── Button.tsx            # ボタン
│   │   ├── Card.tsx              # 枠付きカード
│   │   ├── Input.tsx             # 入力欄
│   │   └── Modal.tsx             # モーダル（ポップアップ）
│   ├── layout/                   # 画面レイアウト系（ページ全体の骨組み）
│   │   ├── Header.tsx            # 上部ヘッダー
│   │   ├── Footer.tsx            # 下部フッター
│   │   └── Sidebar.tsx           # サイドバー
│   └── book/                     # 書籍機能専用のコンポーネント群
│       ├── BookCard.tsx          # 書籍1件分のカード
│       ├── BookList.tsx          # 書籍リスト全体
│       ├── BookDetail.tsx        # 詳細表示
│       └── BookForm.tsx          # 登録・編集フォーム
├── pages/                        # 1ページ＝1ファイル（ルーティング先）
│   ├── HomePage.tsx              # トップページ
│   ├── BookListPage.tsx          # 書籍一覧ページ
│   └── BookDetailPage.tsx        # 書籍詳細ページ
└── App.tsx                       # アプリ全体のルート（最上位コンポーネント）
```

### 3.4 書籍カードコンポーネントの例

実際のアプリで使うような、少し本格的なコンポーネントを作ってみましょう。

> **▼ このコードがやること（先に日本語で）:** これまで学んだ要素を全部使って、**1冊分の「書籍カード」部品**を作ります。流れは——①本1冊の「形」を `Book` 型で決める、②この部品が親から受け取る材料（props）の「形」を `BookCardProps` 型で決める（表示する本のデータ＋「カートに追加」「お気に入り切替」されたときに呼ぶ関数など）、③受け取った props を**分割代入**で取り出し、④星評価などを計算して、⑤カードの見た目（JSX）を返す。少し長いですが、**型 → props → 表示** という、これまでの章の集大成です。コードの後ろで、初めて見る書き方を分解して解説します。

```tsx
// types.ts - 型定義
// 書籍データ1件分の「形」を表す型
type Book = {
  id: number;                  // 一意なID（DBの主キーに相当）
  title: string;               // 書名
  author: string;              // 著者名
  price: number;               // 価格（円）
  rating: number;              // 評価1〜5
  isAvailable: boolean;        // 在庫があるか
  coverImage?: string;         // ? でオプショナル: 画像URLが無い書籍もある
  publishedDate: string;       // 出版日（ISO形式の文字列 "2025-01-15"）
  tags: string[];              // タグの配列（複数の文字列）
};

// BookCard.tsx - 書籍カードコンポーネント
// このコンポーネントが親から受け取る props の型
type BookCardProps = {
  book: Book;                                       // 表示する書籍データ
  onAddToCart: (bookId: number) => void;            // カート追加時のコールバック関数
  onToggleFavorite: (bookId: number) => void;       // お気に入り切替時のコールバック関数
  isFavorite: boolean;                              // 現在お気に入り状態か
};

// 分割代入で props を取り出す
function BookCard({ book, onAddToCart, onToggleFavorite, isFavorite }: BookCardProps) {
  // 星の表示を作る関数
  // ★を rating 個、☆を (5 - rating) 個並べて文字列を作る。
  // "★".repeat(3) は "★★★"（3回繰り返した文字列）になる JS の文字列メソッド。
  const renderStars = (rating: number): string => {
    return "★".repeat(rating) + "☆".repeat(5 - rating);
  };

  return (
    // カード全体の枠
    <div
      style={{
        border: "1px solid #ddd",              // 薄い枠線
        borderRadius: "12px",                  // 大きめ角丸
        padding: "16px",                       // 内側の余白
        maxWidth: "300px",                     // 最大幅
        boxShadow: "0 2px 8px rgba(0,0,0,0.1)", // ふんわり影
      }}
    >
      {/* 表紙画像
          三項演算子 条件 ? A : B で、coverImage の有無により表示を切り替え。
          coverImage が "string" → truthy → 画像を表示
          coverImage が undefined → falsy → No Image プレースホルダーを表示 */}
      {book.coverImage ? (
        <img
          src={book.coverImage}                                    // 画像URL
          alt={`${book.title}の表紙`}                              // 代替テキスト（テンプレートリテラル）
          style={{ width: "100%", borderRadius: "8px" }}            // 横幅100%・角丸
        />
      ) : (
        <div
          style={{
            width: "100%",
            height: "200px",
            backgroundColor: "#f0f0f0",         // 薄いグレー
            borderRadius: "8px",
            display: "flex",                     // 中身を flex レイアウトに
            alignItems: "center",                // 縦方向中央寄せ
            justifyContent: "center",            // 横方向中央寄せ
            color: "#999",
          }}
        >
          No Image
        </div>
      )}

      {/* 書籍情報。h3 のマージンを上12px・下4pxに調整 */}
      <h3 style={{ margin: "12px 0 4px" }}>{book.title}</h3>
      <p style={{ color: "#666", margin: "0 0 8px" }}>{book.author}</p>

      {/* 評価。色は山吹色（#f39c12）。星と数値を並べて表示 */}
      <p style={{ color: "#f39c12", margin: "0 0 8px" }}>
        {renderStars(book.rating)} ({book.rating}/5)
      </p>

      {/* タグ一覧
          display:flex でタグを横並びに、flexWrap:wrap で幅が足りなければ折り返す */}
      <div style={{ display: "flex", gap: "4px", flexWrap: "wrap", marginBottom: "8px" }}>
        {/* タグ配列を map で <span> に変換。key にタグ文字列を使う（タグは重複しない前提） */}
        {book.tags.map((tag) => (
          <span
            key={tag}
            style={{
              backgroundColor: "#e8f4fd",      // 淡い青背景
              color: "#1a73e8",                 // 青文字
              padding: "2px 8px",
              borderRadius: "12px",             // 強めの角丸でピル状に
              fontSize: "12px",
            }}
          >
            {tag}
          </span>
        ))}
      </div>

      {/* 価格と在庫状態を左右に配置（space-between） */}
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <span style={{ fontSize: "20px", fontWeight: "bold" }}>
          ¥{book.price.toLocaleString()}        {/* 3桁カンマ区切り */}
        </span>
        {/* 在庫がある=緑、ない=赤 */}
        <span style={{ color: book.isAvailable ? "green" : "red", fontSize: "14px" }}>
          {book.isAvailable ? "在庫あり" : "在庫なし"}
        </span>
      </div>

      {/* アクションボタン */}
      <div style={{ display: "flex", gap: "8px", marginTop: "12px" }}>
        <button
          // アロー関数で包む理由: onAddToCart に book.id を引数として渡したいから。
          // onClick={onAddToCart(book.id)} と書くとレンダー中に即実行されてしまう。
          onClick={() => onAddToCart(book.id)}
          // 在庫なし → disabled=true（クリック不可）
          disabled={!book.isAvailable}
          style={{
            flex: 1,                                                 // 残りスペースを埋める
            padding: "8px",
            backgroundColor: book.isAvailable ? "#1a73e8" : "#ccc",  // 状態で色変化
            color: "white",
            border: "none",
            borderRadius: "6px",
            cursor: book.isAvailable ? "pointer" : "not-allowed",
          }}
        >
          カートに追加
        </button>
        <button
          // クリックでお気に入り切り替え。コールバックに book.id を渡す。
          onClick={() => onToggleFavorite(book.id)}
          style={{
            padding: "8px 12px",
            backgroundColor: "transparent",     // 背景なし
            border: "1px solid #ddd",
            borderRadius: "6px",
            cursor: "pointer",
            fontSize: "18px",
          }}
        >
          {/* お気に入り中は塗りつぶしハート、未登録は中抜きハート */}
          {isFavorite ? "❤" : "♡"}
        </button>
      </div>
    </div>
  );
}

// 使い方
function App() {
  // サンプルデータ。: Book で型を明示してプロパティ漏れを防ぐ。
  const sampleBook: Book = {
    id: 1,
    title: "React入門ガイド",
    author: "田中太郎",
    price: 2800,
    rating: 4,
    isAvailable: true,
    publishedDate: "2025-01-15",
    tags: ["React", "TypeScript", "フロントエンド"],
  };

  return (
    // 親 → 子へ props を渡す
    // book={sampleBook} はオブジェクトを渡している（{} の中の {} ではない、変数を渡しているだけ）
    <BookCard
      book={sampleBook}
      // インラインでコールバック関数を渡す。alert はブラウザの組み込み関数。
      // テンプレートリテラル `...${id}...` は文字列内に変数を埋め込む書き方。
      onAddToCart={(id) => alert(`書籍ID: ${id} をカートに追加しました`)}
      onToggleFavorite={(id) => alert(`書籍ID: ${id} のお気に入りを切り替えました`)}
      isFavorite={false}
    />
  );
}
```

> この結果、画面にはカード型のUIが表示されます:
>
> 【No Image】（灰色のプレースホルダー）
>
> **React入門ガイド**
>
> 田中太郎
>
> ★★★★☆ (4/5)
>
> `React` `TypeScript` `フロントエンド` （タグバッジ）
>
> **¥2,800** 在庫あり（緑色）
>
> 【カートに追加】ボタン 【♡】ボタン

---

## 4. State (useState)

### 4.1 state とは何か

**state**（ステート：状態。コンポーネントが内部で保持する「変化するデータ」）は、コンポーネントが持つ「変化するデータ」です。state が変わると、React は自動的にそのコンポーネントを**再レンダリング**（さいレンダリング：Re-render。state や props が変わったときに、コンポーネント関数が再度実行されて新しい JSX が作られ、画面が更新されること）します。

> **なぜ普通の変数ではダメか:** React は「state が変わったかどうか」だけを監視しています。普通のローカル変数（`let count = 0;` のようなもの）は React の管理外なので、いくら値を変えても画面は更新されません。値を「画面に反映される形で覚える」には `useState` という **フック**（Hook：フック。`use` で始まる React の特別な関数。コンポーネントに機能を追加する）を使う必要があります。

普通の変数と state の違いを見てみましょう。

```tsx
// NG: 普通の変数は変更しても再レンダリングされない
function Counter() {
  // let は再代入できる変数を作るキーワード。
  // ただしこの変数は React の管理外なので、変更しても画面更新は起きない。
  // しかも関数が再実行されるたびに 0 に戻されてしまう。
  let count = 0;

  // ボタンが押されたときに呼ぶ関数
  const handleClick = () => {
    count += 1; // 値は変わるが、画面は更新されない！
    console.log(count); // コンソールには 1, 2, 3... と出る
  };

  return (
    <div>
      <p>カウント: {count}</p> {/* ずっと 0 のまま */}
      <button onClick={handleClick}>+1</button>
    </div>
  );
}
```

> この結果、ボタンをクリックしても画面には **「カウント: 0」** のまま変わりません。コンソールには値が増えていきますが、React は変数の変化を検知できないのです。

### 4.2 useState の使い方

> **▼ このコードがやること（先に日本語で）:** 「ボタンを押すと数字が増えたり減ったりする**カウンター**」を作ります。これはReactで最も基本的な「動く部品」です。カギは `useState`——「**コンポーネントが値を覚えておく**ための道具」です。`const [count, setCount] = useState(0)` の一行で「今の値（`count`）」と「値を変える関数（`setCount`）」をセットで受け取り、ボタンが押されたら `setCount` を呼んで値を変えます。**`setCount` を呼ぶと、Reactが自動で画面を描き直してくれる**——この「値を変えたら画面も自動更新」が `useState` の最大のポイントです。

```tsx
// ============================================================================
// シンプルなカウンターコンポーネント（詳細コメント版）
// ----------------------------------------------------------------------------
// 「ボタンを押すと数値が +1 / -1 され、画面が自動更新される」
// React の最も基本的なインタラクティブ部品。
// ============================================================================

// React の useState フックを使うので import で取り込む。
// `useState` は「コンポーネントが値を覚えておく」ための関数。
import { useState } from "react";

// 関数コンポーネント = 大文字始まりの関数で、JSXを返すもの。
function Counter() {
  // ──────────────────────────────────────────────────────────────────────
  // (1) state（状態）を作る
  // ──────────────────────────────────────────────────────────────────────
  // useState<number>(0) は「数値型の状態を初期値0で作って」という意味。
  //
  // 戻り値は配列で2つの要素が入っている:
  //   [現在の値, 値を更新するための関数]
  //
  // この配列を「分割代入」で count, setCount に取り出している。
  //   → const count = 戻り値[0];  ← 現在の数値
  //   → const setCount = 戻り値[1]; ← 数値を変える関数
  //
  // 重要: count の値を直接変えることはできない（const なので再代入不可）。
  //       必ず setCount(...) を呼ぶ。setCount を呼ぶと、Reactが
  //       「state が変わったぞ」と認識し、自動で画面を再描画してくれる。
  const [count, setCount] = useState<number>(0);

  // ──────────────────────────────────────────────────────────────────────
  // (2) ボタンが押されたときの処理（イベントハンドラ）
  // ──────────────────────────────────────────────────────────────────────
  // アロー関数で「+1する処理」を定義。
  // setCount を呼ぶことで、count が 0 → 1 → 2 ... と更新される。
  const handleIncrement = () => {
    setCount(count + 1);  // 「今の count + 1」を新しい値として設定
  };

  // 同じく「-1する処理」
  const handleDecrement = () => {
    setCount(count - 1);
  };

  // 「0 に戻す処理」
  const handleReset = () => {
    setCount(0);
  };

  // ──────────────────────────────────────────────────────────────────────
  // (3) 画面に出すJSXを返す
  // ──────────────────────────────────────────────────────────────────────
  // {count} の波カッコはJSX中にJavaScriptの値を埋め込む書き方。
  // count が 5 なら、「カウント: 5」と表示される。
  //
  // onClick={handleDecrement} は「-1ボタンが押されたら handleDecrement を呼んで」の意味。
  // 注意: onClick={handleDecrement()} と () を付けると「即実行」になりバグの元。
  //       関数自体を渡す（= 関数名のみ書く）のが正解。
  return (
    <div>
      <h2>カウンター</h2>
      <p>カウント: {count}</p>
      <button onClick={handleDecrement}>-1</button>
      <button onClick={handleReset}>リセット</button>
      <button onClick={handleIncrement}>+1</button>
    </div>
  );
}
```

**▼ ブラウザでの見た目（初期表示）:**

```
┌────────────────────────────────┐
│  カウンター                     │ ← <h2>
│  カウント: 0                    │ ← {count} に 0 が入る
│ [ -1 ] [ リセット ] [ +1 ]      │ ← 3つの <button>
└────────────────────────────────┘
```

**▼ 動作の流れ:**

| 操作 | count の変化 | 画面表示 |
|------|--------------|----------|
| 初期表示 | 0 | カウント: 0 |
| 「+1」を押す | 0 → 1 | カウント: 1 |
| 「+1」をもう1回 | 1 → 2 | カウント: 2 |
| 「-1」を押す | 2 → 1 | カウント: 1 |
| 「リセット」を押す | 1 → 0 | カウント: 0 |

**▼ 重要ポイント:**
1. `count` は**読み取り専用**。直接 `count = 5;` とは書けない（書いても画面に反映されない）
2. 値を変えたいときは必ず `setCount(...)` を呼ぶ
3. `setCount` を呼ぶと、React が裏で関数 `Counter` をもう一度実行し、新しい count で画面を再描画する

#### useState の型推論

```tsx
// 型を明示的に指定するパターン（<>はジェネリクス: 型を1つ渡す）
// useState<number>(0) は「number型の状態を初期値0で作る」と書いている。
const [count, setCount] = useState<number>(0);
const [name, setName] = useState<string>("");
const [isVisible, setIsVisible] = useState<boolean>(false);

// 初期値から型推論される（明示しなくてもOK）
// 初期値 0 を見て TypeScript が「number 型だな」と自動で判断する。
const [count, setCount] = useState(0);          // number と推論
const [name, setName] = useState("");            // string と推論
const [isVisible, setIsVisible] = useState(false); // boolean と推論

// null を使う場合は明示的な型指定が必要
// 理由: 初期値が null だけだと「null 型」と推論されてしまい、
//        後から User オブジェクトを入れられなくなる。
// User | null は「User型 または null」のユニオン型。
const [user, setUser] = useState<User | null>(null);
```

### 4.3 state の更新とリレンダリング

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; background: #f8fafc; border-radius: 12px; padding: 24px; border: 1px solid #e2e8f0; box-shadow: 0 2px 8px rgba(0,0,0,0.06);">
  <!-- Participants -->
  <div style="display: flex; justify-content: space-between; gap: 8px; margin-bottom: 16px;">
    <div style="background: #1e40af; color: white; padding: 8px 12px; border-radius: 8px; font-size: 12px; font-weight: 600; text-align: center; flex: 1;">ユーザー</div>
    <div style="background: #3b82f6; color: white; padding: 8px 12px; border-radius: 8px; font-size: 12px; font-weight: 600; text-align: center; flex: 1;">画面（DOM）</div>
    <div style="background: #3b82f6; color: white; padding: 8px 12px; border-radius: 8px; font-size: 12px; font-weight: 600; text-align: center; flex: 1;">Counter<br/>コンポーネント</div>
    <div style="background: #10b981; color: white; padding: 8px 12px; border-radius: 8px; font-size: 12px; font-weight: 600; text-align: center; flex: 1;">React State</div>
  </div>
  <!-- Initial state -->
  <div style="text-align: center; background: #dbeafe; color: #1e40af; padding: 6px 12px; border-radius: 6px; font-size: 12px; font-weight: 600; margin-bottom: 12px;">初期表示: カウント: 0</div>
  <!-- First click sequence -->
  <div style="background: #eff6ff; border-radius: 10px; padding: 14px; margin-bottom: 12px; border: 1px solid #bfdbfe;">
    <div style="font-size: 11px; font-weight: 700; color: #1e40af; margin-bottom: 8px; text-transform: uppercase; letter-spacing: 0.5px;">1回目のクリック</div>
    <div style="display: flex; flex-direction: column; gap: 5px;">
      <div style="display: flex; align-items: center; gap: 8px;">
        <span style="background: #1e40af; color: white; padding: 1px 7px; border-radius: 10px; font-size: 10px; font-weight: 700;">1</span>
        <span style="font-size: 12px; color: #334155;"><strong>ユーザー → 画面</strong>：+1ボタンをクリック</span>
      </div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <span style="background: #1e40af; color: white; padding: 1px 7px; border-radius: 10px; font-size: 10px; font-weight: 700;">2</span>
        <span style="font-size: 12px; color: #334155;"><strong>画面 → コンポーネント</strong>：onClickイベント発火</span>
      </div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <span style="background: #1e40af; color: white; padding: 1px 7px; border-radius: 10px; font-size: 10px; font-weight: 700;">3</span>
        <span style="font-size: 12px; color: #334155;"><strong>コンポーネント → State</strong>：setCount(1) を呼び出し</span>
      </div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <span style="background: #10b981; color: white; padding: 1px 7px; border-radius: 10px; font-size: 10px; font-weight: 700;">4</span>
        <span style="font-size: 12px; color: #334155;"><strong>State</strong>：count: 0 → 1 に更新予約</span>
      </div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <span style="background: #10b981; color: white; padding: 1px 7px; border-radius: 10px; font-size: 10px; font-weight: 700;">5</span>
        <span style="font-size: 12px; color: #334155;"><strong>State → コンポーネント</strong>：再レンダリングをスケジュール</span>
      </div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <span style="background: #8b5cf6; color: white; padding: 1px 7px; border-radius: 10px; font-size: 10px; font-weight: 700;">6</span>
        <span style="font-size: 12px; color: #334155;"><strong>コンポーネント</strong>：関数が再実行される（count = 1 で）</span>
      </div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <span style="background: #8b5cf6; color: white; padding: 1px 7px; border-radius: 10px; font-size: 10px; font-weight: 700;">7</span>
        <span style="font-size: 12px; color: #334155;"><strong>コンポーネント → 画面</strong>：新しいJSXを返す</span>
      </div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <span style="background: #8b5cf6; color: white; padding: 1px 7px; border-radius: 10px; font-size: 10px; font-weight: 700;">8</span>
        <span style="font-size: 12px; color: #334155;"><strong>画面</strong>：差分を検出して更新</span>
      </div>
    </div>
  </div>
  <div style="text-align: center; background: #dbeafe; color: #1e40af; padding: 6px 12px; border-radius: 6px; font-size: 12px; font-weight: 600; margin-bottom: 12px;">表示: カウント: 1</div>
  <!-- Second click sequence -->
  <div style="background: #f0fdf4; border-radius: 10px; padding: 14px; margin-bottom: 12px; border: 1px solid #bbf7d0;">
    <div style="font-size: 11px; font-weight: 700; color: #166534; margin-bottom: 8px; text-transform: uppercase; letter-spacing: 0.5px;">2回目のクリック</div>
    <div style="display: flex; flex-direction: column; gap: 5px;">
      <div style="display: flex; align-items: center; gap: 8px;">
        <span style="background: #166534; color: white; padding: 1px 7px; border-radius: 10px; font-size: 10px; font-weight: 700;">1</span>
        <span style="font-size: 12px; color: #334155;"><strong>ユーザー → 画面</strong>：+1ボタンを再度クリック</span>
      </div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <span style="background: #166534; color: white; padding: 1px 7px; border-radius: 10px; font-size: 10px; font-weight: 700;">2</span>
        <span style="font-size: 12px; color: #334155;"><strong>画面 → コンポーネント</strong>：onClickイベント発火</span>
      </div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <span style="background: #166534; color: white; padding: 1px 7px; border-radius: 10px; font-size: 10px; font-weight: 700;">3</span>
        <span style="font-size: 12px; color: #334155;"><strong>コンポーネント → State</strong>：setCount(2) を呼び出し</span>
      </div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <span style="background: #166534; color: white; padding: 1px 7px; border-radius: 10px; font-size: 10px; font-weight: 700;">4</span>
        <span style="font-size: 12px; color: #334155;"><strong>State → コンポーネント</strong>：再レンダリング</span>
      </div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <span style="background: #166534; color: white; padding: 1px 7px; border-radius: 10px; font-size: 10px; font-weight: 700;">5</span>
        <span style="font-size: 12px; color: #334155;"><strong>コンポーネント → 画面</strong>：新しいJSXを返す</span>
      </div>
    </div>
  </div>
  <div style="text-align: center; background: #dbeafe; color: #1e40af; padding: 6px 12px; border-radius: 6px; font-size: 12px; font-weight: 600;">表示: カウント: 2</div>
</div>

**重要なポイント**: `setCount` を呼んでも、その場では `count` の値は変わりません。次の再レンダリング時に新しい値になります。これは React の「state は1回の関数実行中は同じ値で固定される」というルールによるものです（**state は再レンダリング時に最新化される**）。

```tsx
function Counter() {
  // count, setCount を useState で作る
  const [count, setCount] = useState<number>(0);

  const handleClick = () => {
    // 1回目: 「次のレンダリングで count を (現在の count=0) + 1 = 1 にして」と予約
    setCount(count + 1);
    // この時点で count はまだ 0（更新は予約されただけ）
    console.log(count); // まだ 0 のまま！（次のレンダリングで 1 になる）
    // 2回目: count はまだ 0 → setCount(0 + 1) になる → 結局 1 を予約しているだけ
    setCount(count + 1); // count はまだ 0 なので、0 + 1 = 1 になる（2 にはならない！）
  };

  return (
    <div>
      <p>カウント: {count}</p>
      <button onClick={handleClick}>+2 のつもり（実際は+1）</button>
    </div>
  );
}
```

> この結果、ボタンをクリックすると **「カウント: 1」** になります。+2 になると期待しますが、同じレンダリング内では `count` は古い値（0）のままなので、両方とも `setCount(0 + 1)` = `setCount(1)` になります。

**解決策: 関数型更新を使う**

`setXxx(値)` の代わりに `setXxx((prev) => 新しい値)` と書くと、React は「予約された変更を順番に適用」してくれます。`prev` には「これまでに予約された変更を全部反映した最新値」が渡されます。

```tsx
function Counter() {
  const [count, setCount] = useState<number>(0);

  const handleClick = () => {
    // prev には常に「最新の値」が渡される
    // 関数型更新: setCount に「現在値 → 次の値」という関数を渡す
    setCount((prev) => prev + 1); // 0 → 1
    // ↑の予約が反映された結果が prev に渡る → 1 + 1 = 2
    setCount((prev) => prev + 1); // 1 → 2
  };

  return (
    <div>
      <p>カウント: {count}</p>
      <button onClick={handleClick}>+2</button>
    </div>
  );
}
```

> この結果、ボタンをクリックすると **「カウント: 2」** → **「カウント: 4」** → ... と2ずつ増えます。関数型更新なら、直前の最新値に基づいて計算されるため、正しく動作します。

### 4.4 オブジェクトの state

> **▼ このコードがやること（先に日本語で）:** 1つの値（数字など）ではなく、「名前・メール・年齢…」を**まとめて持つオブジェクト**を state にする例です。ここで一番大事なのは更新のしかた——オブジェクトの一部だけを変えたいときも、**古いオブジェクトを直接書き換えてはいけません**。第3章で学んだ**スプレッド構文 `...`** で「今の全項目をコピー」してから、変えたい項目だけを上書きした**新しいオブジェクトを作って** `setXxx` に渡します。「直接いじらず、新しく作り替えて差し替える」がReactのstate更新の鉄則です（理由はコード内で説明します）。

```tsx
// useState フックを取り込む
import { useState } from "react";

// プロフィール情報の「形」を表す型
type UserProfile = {
  name: string;
  email: string;
  age: number;
  bio: string;       // 自己紹介
};

function ProfileEditor() {
  // オブジェクト state を作る。複数の関連プロパティをまとめて管理できる。
  const [profile, setProfile] = useState<UserProfile>({
    name: "田中太郎",
    email: "tanaka@example.com",
    age: 25,
    bio: "React を勉強中です",
  });

  // 名前を変更する関数
  // 引数 e の型 React.ChangeEvent<HTMLInputElement> は「<input>のChangeイベント」を表す。
  // e.target.value で入力欄の現在の文字列を取得できる。
  const handleNameChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    // スプレッド構文 ...profile で「現在のprofileの全プロパティを展開」してコピーし、
    // 続けて name: e.target.value で name だけ新しい値に上書きする。
    // この結果、新しい profile オブジェクトが作られる（=参照が変わる=Reactが変化を検知）。
    setProfile({
      ...profile,
      name: e.target.value,
    });
  };

  // メールを変更する関数（パターンは同じ）
  const handleEmailChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setProfile({
      ...profile,
      email: e.target.value,
    });
  };

  // 年齢を変更する関数
  // <input type="number"> でも e.target.value は文字列なので Number() で数値に変換する。
  const handleAgeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setProfile({
      ...profile,
      age: Number(e.target.value),
    });
  };

  // 自己紹介を変更する関数
  // <textarea> の場合は型が HTMLTextAreaElement に変わる点に注意。
  const handleBioChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setProfile({
      ...profile,
      bio: e.target.value,
    });
  };

  return (
    <div>
      <h2>プロフィール編集</h2>

      {/* 制御コンポーネント（Controlled Component）パターン:
          value と onChange を両方セットで指定し、表示値を state で完全に制御する。
          これにより state が「唯一の正解」となり、画面と data が常に同期する。 */}
      <div>
        <label>名前: </label>
        <input value={profile.name} onChange={handleNameChange} />
      </div>

      <div>
        <label>メール: </label>
        <input value={profile.email} onChange={handleEmailChange} />
      </div>

      <div>
        <label>年齢: </label>
        <input
          type="number"                          // 数値入力モード
          value={profile.age}
          onChange={handleAgeChange}
        />
      </div>

      <div>
        <label>自己紹介: </label>
        {/* textarea は HTML では <textarea>中身</textarea> だが、
            React では value 属性で中身を制御する。 */}
        <textarea value={profile.bio} onChange={handleBioChange} />
      </div>

      <h3>プレビュー</h3>
      <p>名前: {profile.name}</p>
      <p>メール: {profile.email}</p>
      <p>年齢: {profile.age}歳</p>
      <p>自己紹介: {profile.bio}</p>
    </div>
  );
}
```

> **制御コンポーネント / 非制御コンポーネント:** value と onChange の両方で state とつなぐ書き方が**制御コンポーネント**（Controlled Component：Reactのstateが入力値の真実の出どころ）。逆に value を指定せず DOM 任せにする書き方は**非制御コンポーネント**（Uncontrolled Component：DOM自身が値を保持し、必要なときだけ ref で読み取る）。本書では基本的に制御コンポーネントを使います。

> この結果、画面には4つの入力欄と、その下にプレビューが表示されます。入力欄に文字を入力すると、**リアルタイムにプレビュー部分が更新**されます。例えば名前の入力欄を「鈴木花子」に変更すると、プレビューの名前も即座に **「名前: 鈴木花子」** に変わります。

**重要**: オブジェクトの state を更新するときは、必ず**新しいオブジェクトを作成**してください。プロパティを直接変更してはいけません。

```tsx
// NG: 直接変更（React が変化を検知できない）
profile.name = "新しい名前";
setProfile(profile); // 同じオブジェクト参照なので再レンダリングされない！

// OK: 新しいオブジェクトを作成
setProfile({ ...profile, name: "新しい名前" });
```

#### ネストしたオブジェクトの更新

```tsx
// 住所の型（小さな部品）
type Address = {
  prefecture: string;       // 都道府県
  city: string;             // 市区町村
  street: string;           // 番地
};

// ユーザー型（Address を中に含む = ネストした構造）
type UserWithAddress = {
  name: string;
  address: Address;
};

function AddressEditor() {
  // ネストしたオブジェクト state
  const [user, setUser] = useState<UserWithAddress>({
    name: "田中太郎",
    address: {
      prefecture: "東京都",
      city: "渋谷区",
      street: "道玄坂1-1-1",
    },
  });

  const handleCityChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    // ネストしたオブジェクトもスプレッド構文で展開する
    // 外側の {...user} で「user の全プロパティ」を展開してコピー、
    // address プロパティだけは別の新しいオブジェクトに置き換える。
    // その内側でも {...user.address} で展開し、city だけ上書きする。
    // → user も user.address も新しい参照になるので React がきちんと変化を検知する。
    setUser({
      ...user,
      address: {
        ...user.address,
        city: e.target.value,
      },
    });
  };

  return (
    <div>
      <p>
        {/* {} で JS式を埋め込み、文字列を連結 */}
        住所: {user.address.prefecture}
        {user.address.city}
        {user.address.street}
      </p>
      <label>市区町村: </label>
      <input value={user.address.city} onChange={handleCityChange} />
    </div>
  );
}
```

> この結果、画面には **「住所: 東京都渋谷区道玄坂1-1-1」** と表示され、入力欄で市区町村を変更すると住所表示がリアルタイムに更新されます。例えば「新宿区」と入力すると、**「住所: 東京都新宿区道玄坂1-1-1」** に変わります。

### 4.5 配列の state

配列の state も、直接変更せず、常に**新しい配列を作成**して更新します。

> **▼ このコードがやること（先に日本語で）:** 「TODOリスト（やることの一覧）」を題材に、**配列の state を追加・削除・変更する**基本パターンを学びます。ここでもオブジェクトのときと同じ鉄則——**元の配列を直接いじらない**——が効いてきます。具体的には、追加は**スプレッド構文 `[...todos, 新しい項目]`**（今の全部＋末尾に1つ）、削除は **`filter`**（消したいもの以外を残す）、変更は **`map`**（対象だけ作り替える）を使い、いずれも「**新しい配列を作って差し替える**」形にします。第2章で学んだ `map`／`filter` が、ここで実戦投入されます。

```tsx
import { useState } from "react";

// TODO項目1件の型
type Todo = {
  id: number;            // 一意なID（key に使う）
  text: string;          // タスク本文
  completed: boolean;    // 完了済みか
};

function TodoApp() {
  // 配列 state。初期値は2件のサンプル。
  const [todos, setTodos] = useState<Todo[]>([
    { id: 1, text: "React を学ぶ", completed: false },
    { id: 2, text: "TypeScript を学ぶ", completed: true },
  ]);
  // 入力欄の現在値（制御コンポーネント用）
  const [inputValue, setInputValue] = useState<string>("");
  // 次に使う一意なID。新規追加するたびに1増やす。
  const [nextId, setNextId] = useState<number>(3);

  // ── 追加 ──
  const handleAdd = () => {
    // .trim() は前後の空白を取り除いた新しい文字列を返す。
    // 空文字なら何もせず即 return（早期リターン）。
    if (inputValue.trim() === "") return;

    // 新しい Todo オブジェクトを作る
    const newTodo: Todo = {
      id: nextId,
      text: inputValue,
      completed: false,
    };
    // [...todos, newTodo] で「既存の全要素＋末尾に新項目」の新しい配列を作って渡す。
    // 元の todos 配列は変更しない（イミュータブル＝不変な更新）。
    setTodos([...todos, newTodo]); // 既存の配列を展開して新しい要素を追加
    setInputValue("");                // 入力欄をクリア
    setNextId(nextId + 1);            // 次回ID用に+1
  };

  // ── 削除 ──
  const handleDelete = (id: number) => {
    // filter は「条件が true の要素だけ残した新しい配列」を返す。
    // ここでは「ID が一致しないもの = 削除対象でないもの」だけを残す。
    setTodos(todos.filter((todo) => todo.id !== id)); // id が一致しない要素だけ残す
  };

  // ── 完了/未完了の切り替え ──
  const handleToggle = (id: number) => {
    setTodos(
      // map は「各要素を変換して新しい配列を作る」メソッド。
      // 一致する Todo だけ「completed を反転した新しいオブジェクト」に置き換える。
      // 一致しない Todo はそのまま返す。
      todos.map((todo) =>
        todo.id === id ? { ...todo, completed: !todo.completed } : todo
      )
    );
  };

  // ── テキスト更新 ──
  const handleUpdate = (id: number, newText: string) => {
    setTodos(
      // 一致する Todo の text プロパティだけを上書き
      todos.map((todo) =>
        todo.id === id ? { ...todo, text: newText } : todo
      )
    );
  };

  return (
    <div>
      <h2>TODOリスト</h2>

      {/* 入力欄 */}
      <div>
        <input
          value={inputValue}                                           // 表示値を state に紐づけ
          onChange={(e) => setInputValue(e.target.value)}              // 入力ごとに state を更新
          placeholder="新しいタスクを入力"                              // 未入力時の薄いガイド文字
        />
        <button onClick={handleAdd}>追加</button>
      </div>

      {/* TODOリスト */}
      <ul>
        {todos.map((todo) => (
          // key には DBの主キーに当たる一意なIDを使う
          <li key={todo.id}>
            <input
              type="checkbox"                                          // チェックボックス入力
              checked={todo.completed}                                 // 制御コンポーネント
              onChange={() => handleToggle(todo.id)}                   // クリックで切替
            />
            <span
              style={{
                // 完了済みなら取り消し線、未完了ならなし
                textDecoration: todo.completed ? "line-through" : "none",
                // 完了済みは薄い灰色、未完了は黒
                color: todo.completed ? "#999" : "#000",
              }}
            >
              {todo.text}
            </span>
            {/* アロー関数で包んで id を引数に渡す */}
            <button onClick={() => handleDelete(todo.id)}>削除</button>
          </li>
        ))}
      </ul>

      {/* 統計
          todos.length は配列の要素数。
          filter(...).length で「条件を満たす要素の数」を計算できる。 */}
      <p>
        合計: {todos.length}件 / 完了: {todos.filter((t) => t.completed).length}件
        / 未完了: {todos.filter((t) => !t.completed).length}件
      </p>
    </div>
  );
}
```

> この結果、画面には以下のように表示されます:
>
> **TODOリスト**
>
> 【入力欄】【追加ボタン】
>
> - [ ] React を学ぶ 【削除】
> - [x] ~~TypeScript を学ぶ~~ 【削除】
>
> 合計: 2件 / 完了: 1件 / 未完了: 1件
>
> 「テストを書く」と入力して【追加】を押すと:
>
> - [ ] React を学ぶ 【削除】
> - [x] ~~TypeScript を学ぶ~~ 【削除】
> - [ ] テストを書く 【削除】
>
> 合計: 3件 / 完了: 1件 / 未完了: 2件
>
> 「React を学ぶ」のチェックボックスをクリックすると:
>
> - [x] ~~React を学ぶ~~ 【削除】
>
> 合計: 3件 / 完了: 2件 / 未完了: 1件

#### 配列操作の早見表

| 操作 | 使わない（直接変更） | 使う（新しい配列を作成） |
|------|---------------------|------------------------|
| 追加 | `push`, `unshift` | `[...arr, newItem]`, `[newItem, ...arr]` |
| 削除 | `splice` | `filter` |
| 置換 | `arr[i] = value` | `map` |
| 並び替え | `sort`, `reverse` | `[...arr].sort()`, `[...arr].reverse()` |

---

## 5. イベントハンドリング

### 5.1 onClick

> **▼ このコードがやること（先に日本語で）:** ボタンを押したときに処理を動かす **`onClick`** の使い方を、いくつかのパターンで示します。一番大事なのは「**関数を"渡す"のであって、"実行"してはいけない**」という点です。`onClick={handleClick}`（関数そのものを渡す＝押したとき呼ばれる）は正しく、`onClick={handleClick()}`（その場で実行してしまう）は誤り。引数を渡したいときは `onClick={() => handleClick(値)}` のように**アロー関数で包んで「押されたら実行する」形にする**——この違いがこの節のヤマです（コード内とこの後で詳しく解説します）。

```tsx
function ClickExamples() {
  // 基本的なクリックハンドラ
  // イベントハンドラ（Event Handler）= イベント発生時に呼ばれる関数。
  const handleClick = () => {
    alert("ボタンがクリックされました！");
  };

  // 引数を受け取るクリックハンドラ
  // 通常のJS関数なので、引数は自由に定義できる。
  const handleItemClick = (itemName: string) => {
    // テンプレートリテラル: ` ` で囲み、${} の中に式を埋め込める文字列。
    alert(`${itemName}がクリックされました`);
  };

  // イベントオブジェクトを受け取るクリックハンドラ
  // React.MouseEvent<HTMLButtonElement> は「<button>に対する Mouseイベント」の型。
  //   e.clientX / e.clientY = クリックされた画面座標
  //   e.currentTarget = ハンドラを付けた要素そのもの
  //   e.target = 実際にクリックされた要素（子要素の場合がある）
  const handleButtonClick = (e: React.MouseEvent<HTMLButtonElement>) => {
    console.log("クリック位置:", e.clientX, e.clientY);
    console.log("クリックされた要素:", e.currentTarget.textContent);
  };

  return (
    <div>
      {/* 基本的な使い方
          onClick={関数名} で「関数の参照」を渡す。React がクリック時に呼んでくれる。 */}
      <button onClick={handleClick}>クリック</button>

      {/* 引数を渡す場合はアロー関数で包む
          () => handleItemClick("Apple") は「引数なしの関数」で、その中で
          handleItemClick("Apple") を呼ぶ。React はこの匿名関数をクリック時に呼ぶ。 */}
      <button onClick={() => handleItemClick("Apple")}>Apple</button>
      <button onClick={() => handleItemClick("Banana")}>Banana</button>

      {/* イベントオブジェクトを使う
          関数参照を直接渡すと、React がクリック時に第1引数として event を渡してくれる。 */}
      <button onClick={handleButtonClick}>位置を表示</button>

      {/* インラインで直接書く
          わざわざ関数を別に定義しなくても、その場でアロー関数を書ける。 */}
      <button onClick={() => console.log("インラインハンドラ")}>
        インライン
      </button>
    </div>
  );
}
```

> この結果、画面には4つのボタンが表示されます。
>
> 【クリック】を押すと、アラートで **「ボタンがクリックされました！」** と表示されます。
>
> 【Apple】を押すと、**「Appleがクリックされました」** と表示されます。
>
> 【位置を表示】を押すと、コンソールにクリック座標が出力されます。

**注意**: `onClick` に関数を渡すときは、**関数を実行してはいけません**。

```tsx
// NG: 関数を実行してしまっている（レンダリング時に即座に実行される）
// () を付けると「今すぐ呼び出す」意味になり、戻り値（多くの場合 undefined）が onClick に渡る。
// 結果として、画面表示の瞬間に handleClick が動いてしまい、無限ループや誤動作になる。
<button onClick={handleClick()}>クリック</button>

// OK: 関数の参照を渡す
// () を付けないことで「関数そのもの」を渡せる。React がクリック時に呼ぶ。
<button onClick={handleClick}>クリック</button>

// OK: アロー関数で包む
// 「クリック時に handleClick() を呼ぶ」新しい関数を作って渡している。
// 引数を渡したい場合はこの書き方が必要。
<button onClick={() => handleClick()}>クリック</button>
```

### 5.2 onChange

> **▼ このコードがやること（先に日本語で）:** 入力欄（テキスト・チェックボックス・ラジオボタン・セレクト）に何か入力・選択されたときに反応する **`onChange`** の使い方です。基本の流れは「**入力された値を state に保存し、その state を入力欄の表示に反映する**」というワンセット。これにより「**画面の見た目（入力欄）と、プログラムが持つ値（state）が常に一致する**」状態を作れます（これを"制御された入力"と呼びます）。入力の種類ごとに「値の取り出し方」が少し違う点も、コード内で1つずつ確認します。

```tsx
import { useState } from "react";

function InputExamples() {
  // 4つの独立した state を用意
  const [text, setText] = useState<string>("");
  const [selectedColor, setSelectedColor] = useState<string>("red");
  const [isChecked, setIsChecked] = useState<boolean>(false);
  const [selectedSize, setSelectedSize] = useState<string>("M");

  return (
    <div>
      {/* テキスト入力 */}
      <div>
        <label>名前: </label>
        <input
          type="text"                                                    // 1行のテキスト入力
          value={text}                                                   // 表示値は state と同期
          onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
            // e.target = 入力欄のDOM要素。.value で現在の文字列を取得
            setText(e.target.value)
          }
          placeholder="名前を入力してください"                            // 未入力時のガイド文字
        />
        <p>入力値: 「{text}」</p>
      </div>

      {/* セレクトボックス（プルダウンメニュー） */}
      <div>
        <label>色: </label>
        <select
          value={selectedColor}                                          // 現在の選択値
          onChange={(e: React.ChangeEvent<HTMLSelectElement>) =>
            setSelectedColor(e.target.value)                             // <select>専用の型
          }
        >
          {/* <option> の value 属性が選択時に e.target.value として返る */}
          <option value="red">赤</option>
          <option value="blue">青</option>
          <option value="green">緑</option>
        </select>
        {/* 選択中の色名で文字色を変える */}
        <p style={{ color: selectedColor }}>
          選択した色: {selectedColor}
        </p>
      </div>

      {/* チェックボックス */}
      <div>
        <label>
          <input
            type="checkbox"
            // チェックボックスでは value ではなく checked プロパティを使う
            checked={isChecked}
            onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
              // e.target.checked は真偽値（true/false）。文字列ではない！
              setIsChecked(e.target.checked)
            }
          />
          利用規約に同意する
        </label>
        <p>同意状態: {isChecked ? "同意済み" : "未同意"}</p>
      </div>

      {/* ラジオボタン
          複数のラジオボタンを「同じグループ」として扱うには、name 属性を揃える。 */}
      <div>
        <p>サイズ:</p>
        {/* ["S","M","L","XL"] の各要素を <label> に展開 */}
        {["S", "M", "L", "XL"].map((size) => (
          <label key={size} style={{ marginRight: "12px" }}>
            <input
              type="radio"
              name="size"                                                // 同じ name でグループ化
              value={size}
              checked={selectedSize === size}                            // この選択肢が現在の値と同じか
              onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                setSelectedSize(e.target.value)
              }
            />
            {size}
          </label>
        ))}
        <p>選択サイズ: {selectedSize}</p>
      </div>
    </div>
  );
}
```

> この結果、画面には4つの入力セクションが表示されます:
>
> 1. テキスト入力欄に「こんにちは」と入力すると、下に **「入力値: 「こんにちは」」** とリアルタイム表示
> 2. セレクトボックスで「青」を選ぶと、**「選択した色: blue」** が青色で表示
> 3. チェックボックスをオンにすると、**「同意状態: 同意済み」** に変化
> 4. ラジオボタンで「L」を選ぶと、**「選択サイズ: L」** に変化

### 5.3 onSubmit（フォーム送信）

ここでは「書籍登録フォーム」を作ります。少しコードが長く、初めて見る書き方も多いので、**最初に完成形を載せたあと、難しい部分を1つずつ分解して、初心者向けに徹底的に解説**します。コードを今すぐ全部理解できなくても大丈夫です。「あとで解説があるんだな」と思って、まずは雰囲気を眺めてください。

> **このフォームがやることの全体像（先に日本語で）:**
> 1. ユーザーが「タイトル・著者・価格・カテゴリ・説明」を入力する。
> 2. 入力した内容は `formData` という1つの箱（state）にまとめて保存する。
> 3. 入力するたびに、その内容を `formData` に反映する（`handleChange`）。
> 4. 「登録する」ボタンを押したら（`handleSubmit`）、まず**入力チェック**（`validate`）をする。
> 5. 必須項目が空なら、赤字でエラーメッセージを出す。
> 6. すべてOKなら「登録完了！」画面に切り替える。
>
> この「入力 → チェック → 送信 → 完了画面」という流れは、ほぼ全てのアプリのフォームに共通する基本パターンです。

#### ▼ まずは完成形（このあと部品ごとに分解します）

```tsx
import { useState } from "react";

// フォーム入力データの型
// 注意: price は文字列にしている。<input type="number"> でも DOM 上の値は文字列なので、
//       入力中の "" や "1." のような中間状態を表現しやすい。
type BookFormData = {
  title: string;
  author: string;
  price: string;
  category: string;
  description: string;
};

function BookForm() {
  // フォーム全体のデータを1つのオブジェクト state で持つ
  const [formData, setFormData] = useState<BookFormData>({
    title: "",
    author: "",
    price: "",
    category: "programming",         // デフォルトカテゴリ
    description: "",
  });

  // エラー情報を「フィールド名 → エラーメッセージ」の形で持つ。
  // Partial<X> は X の全プロパティをオプショナル化した型。
  // Record<K, V> は「キーKのオブジェクトに値Vが入る」型。
  // keyof BookFormData は "title" | "author" | ... の文字列リテラルユニオン。
  const [errors, setErrors] = useState<Partial<Record<keyof BookFormData, string>>>({});
  // 送信完了フラグ
  const [isSubmitted, setIsSubmitted] = useState<boolean>(false);

  // バリデーション関数
  // 戻り値が true なら「全項目OK」。
  const validate = (): boolean => {
    const newErrors: Partial<Record<keyof BookFormData, string>> = {};

    // .trim() で前後空白を取り除いた文字列が "" なら未入力。
    if (formData.title.trim() === "") {
      newErrors.title = "タイトルは必須です";
    }
    if (formData.author.trim() === "") {
      newErrors.author = "著者は必須です";
    }
    // Number("") は NaN、Number("100") は 100。<= 0 で「未入力 or 0以下」を弾く。
    if (formData.price === "" || Number(formData.price) <= 0) {
      newErrors.price = "価格は0より大きい数値を入力してください";
    }

    // エラーオブジェクトを state にセット
    setErrors(newErrors);
    // Object.keys でプロパティ名の配列を取得。長さ0なら「エラーなし」=true。
    return Object.keys(newErrors).length === 0;
  };

  // 汎用的な入力ハンドラ
  // ChangeEvent の型を3種類のユニオンにして、input/select/textarea を1関数で処理。
  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>
  ) => {
    // 分割代入で name と value を取り出す。
    // name は <input name="..."> 属性、value は現在値。
    const { name, value } = e.target;
    setFormData({
      ...formData,
      // [name]: value は「動的なキー」で、name の値そのものをプロパティ名として使う。
      // 例えば name="title" なら { ...formData, title: value } と書くのと同じ。
      [name]: value,
    });

    // 入力時にエラーをクリア
    // as keyof BookFormData は「string を BookFormData のキー型として扱う」型アサーション。
    if (errors[name as keyof BookFormData]) {
      setErrors({
        ...errors,
        [name]: undefined,
      });
    }
  };

  // フォーム送信
  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    // <form> のデフォルト動作（ページリロード）を抑止
    e.preventDefault(); // ページ遷移（デフォルト動作）を防ぐ

    if (validate()) {
      console.log("送信データ:", formData);
      setIsSubmitted(true);
      // 実際のアプリではここで API を呼ぶ
    }
  };

  // 早期 return で「送信完了画面」と「フォーム画面」を切り替える
  if (isSubmitted) {
    return (
      <div>
        <h2>登録完了！</h2>
        <p>「{formData.title}」を登録しました。</p>
        {/* 完了画面のボタン: フラグを false に戻すとフォーム画面に戻る */}
        <button onClick={() => setIsSubmitted(false)}>もう1冊登録する</button>
      </div>
    );
  }

  return (
    // <form> の onSubmit に handleSubmit を指定。
    // フォーム内の submit ボタンクリック または Enter キーで発火する。
    <form onSubmit={handleSubmit}>
      <h2>書籍登録</h2>

      <div>
        {/* htmlFor は HTML の for 属性のJSX版。クリックで対応する input にフォーカス */}
        <label htmlFor="title">タイトル *</label>
        <input
          id="title"
          name="title"                      // handleChange の中で動的キーとして使う
          type="text"
          value={formData.title}
          onChange={handleChange}
        />
        {/* エラーがある（truthy）ときだけ赤字でメッセージを表示 */}
        {errors.title && <p style={{ color: "red" }}>{errors.title}</p>}
      </div>

      <div>
        <label htmlFor="author">著者 *</label>
        <input
          id="author"
          name="author"
          type="text"
          value={formData.author}
          onChange={handleChange}
        />
        {errors.author && <p style={{ color: "red" }}>{errors.author}</p>}
      </div>

      <div>
        <label htmlFor="price">価格 *</label>
        <input
          id="price"
          name="price"
          type="number"                     // 数値入力（スマホで数字キーボードが出る）
          value={formData.price}
          onChange={handleChange}
        />
        {errors.price && <p style={{ color: "red" }}>{errors.price}</p>}
      </div>

      <div>
        <label htmlFor="category">カテゴリ</label>
        <select
          id="category"
          name="category"
          value={formData.category}
          onChange={handleChange}
        >
          <option value="programming">プログラミング</option>
          <option value="design">デザイン</option>
          <option value="business">ビジネス</option>
          <option value="other">その他</option>
        </select>
      </div>

      <div>
        <label htmlFor="description">説明</label>
        <textarea
          id="description"
          name="description"
          value={formData.description}
          onChange={handleChange}
          rows={4}                           // 表示行数の目安
        />
      </div>

      {/* type="submit" のボタンを押すと <form> の onSubmit が走る */}
      <button type="submit">登録する</button>
    </form>
  );
}
```

#### ▼ ここからが本題：難しい部分を1つずつ分解して解説

上のコードには、初心者がつまずきやすい書き方がいくつも入っています。順番に、**1つずつ**ていねいに見ていきましょう。

---

##### 解説1: フォームの入力内容を「1つの箱」にまとめて持つ

```tsx
const [formData, setFormData] = useState<BookFormData>({
  title: "",
  author: "",
  price: "",
  category: "programming",
  description: "",
});
```

- `useState`（ユーズ・ステート）は、第4章で学んだ「**変化する値を持つための道具**」です。`const [今の値, 値を変える関数] = useState(初期値)` の形で使います。
- ここでは「今の値」を `formData`、「値を変える関数」を `setFormData` という名前にしています。
- 入力欄は5つ（タイトル・著者・価格・カテゴリ・説明）ありますが、**5個バラバラにstateを作るのではなく、1つのオブジェクト（`{ }` でまとめたもの）にまとめて**持っています。こうすると管理が楽になります。
- 初期値はすべて空文字 `""`（からの文字列）。ただしカテゴリだけは最初から `"programming"` を選んだ状態にしています。
- `<BookFormData>` の部分は「この箱の中身は `BookFormData` という型ですよ」とTypeScriptに教えている部分です（`BookFormData` 型はコードの上のほうで定義しています）。

> **なぜ価格(`price`)も文字列 `""` なの？数字じゃないの？** Webの入力欄（`<input>`）は、たとえ「数値入力欄」でも、**中身は必ず文字列として扱われる**という決まりがあるためです。入力途中の `"1."`（まだ打ちかけ）のような状態も文字列なら素直に表せます。数値が必要になった時だけ、あとで数値に変換します（解説4で出てきます）。

---

##### 解説2: エラーメッセージを入れる箱（ここが一番むずかしく見える部分）

```tsx
const [errors, setErrors] = useState<Partial<Record<keyof BookFormData, string>>>({});
```

`Partial<Record<keyof BookFormData, string>>` という型が、**初見だと一番こわく見える部分**です。でも分解すれば難しくありません。内側から順に読み解きましょう。

1. **`keyof BookFormData`** … `keyof`（キーオブ）は「その型が持っているキー（項目名）を全部集める」という意味です。`BookFormData` は `title / author / price / category / description` という項目を持つので、`keyof BookFormData` は **「`"title"` または `"author"` または … `"description"`」** という「項目名の一覧」を表します。
2. **`Record<キー, 値>`** … `Record`（レコード）は「**こういうキーに、こういう値が入るオブジェクト**」を表す型です。`Record<keyof BookFormData, string>` は「`title` や `author` などの各項目名をキーにして、値が**文字列（エラーメッセージ）**のオブジェクト」という意味になります。つまり `{ title: "タイトルは必須です", price: "価格を…" }` のような形です。
3. **`Partial<...>`** … `Partial`（パーシャル＝部分的）は「**全部の項目を"あってもなくてもよい"扱いにする**」型です。エラーは「ある項目だけ」発生することが多い（タイトルだけエラー、など）ので、全項目そろっていなくてOKにしたいのです。
4. 最後に `useState<...>({})` の **`{}`** は「初期値は**空っぽのオブジェクト**（エラーが1つもない状態）」という意味です。

> **まとめると:** `errors` は「**エラーが出た項目だけ、その項目名→エラーメッセージ という形で入る箱**」です。最初は空 `{}`。たとえばタイトルが未入力なら `{ title: "タイトルは必須です" }` のようになります。むずかしい型に見えますが、やりたいことは「エラーメッセージを項目ごとに覚えておく箱」というだけです。

```tsx
const [isSubmitted, setIsSubmitted] = useState<boolean>(false);
```

- `isSubmitted`（イズ・サブミテッド＝送信済みか）は「**送信が完了したかどうか**」を表す `true`/`false` の値です。最初は `false`（まだ送信していない）。送信が成功したら `true` にして、画面を「完了画面」に切り替えるのに使います（解説6）。

---

##### 解説3: 入力するたびに内容を保存する `handleChange`

入力欄に文字を打つたびに呼ばれる関数です。ここに**スプレッド構文**と**動的なキー**という、2つの大事な書き方が出てきます。

```tsx
const handleChange = (
  e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>
) => {
  const { name, value } = e.target;
  setFormData({
    ...formData,
    [name]: value,
  });
  // （エラーを消す処理は後述）
};
```

- **`e`（イベントオブジェクト）** … 入力が変化したとき、Reactが「何が起きたか」の情報を `e` という箱に入れて渡してくれます。`e: React.ChangeEvent<...>` はその `e` の型で、「`<input>` か `<select>` か `<textarea>` の変化イベント」という意味です（`|` は「または」）。**この型指定はおまじない**と思って大丈夫です。1つの関数で3種類の入力欄すべてを処理できるようにしています。
- **`const { name, value } = e.target;`** … `e.target`（イー・ターゲット）は「**変化が起きた入力欄そのもの**」を指します。そこから `name`（その欄の名前）と `value`（今入力されている値）を取り出しています。`const { name, value } = ...` は第3章で学んだ「**分割代入**」（オブジェクトから必要な項目だけ取り出す書き方）です。
  - `name` は、下の `<input name="title" ... />` の `name="title"` の部分から来ます。つまり「どの欄が変化したか」が文字列で分かります。

- **`...formData`（スプレッド構文）** … `...`（ドット3つ）は「**今ある `formData` の中身を、まるごとコピーして展開する**」という意味です。これを「スプレッド構文（spread＝展開）」と呼びます。

> **なぜわざわざ全部コピーするの？** Reactでは、stateを更新するとき「**古いものを直接書き換えず、新しいオブジェクトを作って丸ごと差し替える**」のがルールだからです。`...formData` で「今の全項目をコピー」してから、変化した1項目だけを上書きすることで、「他の項目はそのまま・変わった欄だけ新しい値」という新しいオブジェクトを作っています。

- **`[name]: value`（動的なキー）** … キー名を `[ ]`（角カッコ）で囲むと、「**変数 `name` の中身を、そのままキー（項目名）として使う**」という意味になります。これを「動的なキー（computed property name）」と呼びます。
  - 例えばタイトル欄が変化したなら `name` は `"title"` なので、`[name]: value` は `title: value` と書いたのと同じになります。著者欄なら `author: value` になります。
  - **これがこの関数のキモです。** たった1つの `handleChange` で、5つのどの入力欄が変わっても、その欄だけを正しく更新できるのです。「もし `[name]: value` と書かず `title: value` と固定で書いてしまうと、どの欄を触ってもタイトルだけが書き換わってしまう」と考えると、ありがたみが分かります。

つまり `setFormData({ ...formData, [name]: value })` は、日本語にすると「**今の全項目はそのままに、今変化した欄（name）だけを新しい値（value）にした、新しいformDataを作ってセットして**」という意味です。

続いて、同じ `handleChange` の中の後半部分です。

```tsx
if (errors[name as keyof BookFormData]) {
  setErrors({
    ...errors,
    [name]: undefined,
  });
}
```

- これは「**入力し直したら、その欄のエラー表示を消す**」ための処理です。一度エラーが出ても、ユーザーが入力を直したら赤字を消してあげる、という親切機能です。
- **`errors[name as keyof BookFormData]`** … 「`errors` という箱の中に、今の欄（`name`）のエラーが入っているか？」を確認しています。
  - `name as keyof BookFormData` の **`as`** は「**型アサーション**」と呼ばれ、「TypeScriptさん、この `name`（ただの文字列）を、`BookFormData` の項目名のどれかとして扱ってね」と**お願い（言い換え）する**書き方です。`name` はそのままだと「ただの文字列」扱いで、`errors` のキーとして使うとTypeScriptが警告するため、`as` で「ちゃんと項目名ですよ」と伝えています。
- **`if ( ... )`** … カッコの中が「**中身がある（エラーが存在する）**」と判断できるときだけ、`{ }` の中を実行します。エラーが無いなら何もしません。
- **`[name]: undefined`** … その欄のエラーを `undefined`（＝値が無い状態）にして、実質的に消しています。ここでも `...errors` で他のエラーは残しつつ、その欄だけ消しています（スプレッド構文と動的キーの再利用です）。

---

##### 解説4: 入力チェック（バリデーション）`validate`

「送信してよい内容か」をチェックする関数です。`validate`（バリデート）は「検証する」という意味の英単語です。

```tsx
const validate = (): boolean => {
  const newErrors: Partial<Record<keyof BookFormData, string>> = {};

  if (formData.title.trim() === "") {
    newErrors.title = "タイトルは必須です";
  }
  if (formData.author.trim() === "") {
    newErrors.author = "著者は必須です";
  }
  if (formData.price === "" || Number(formData.price) <= 0) {
    newErrors.price = "価格は0より大きい数値を入力してください";
  }

  setErrors(newErrors);
  return Object.keys(newErrors).length === 0;
};
```

- **`(): boolean =>`** … この関数は最後に `true` か `false`（＝`boolean` 型）を返します、という宣言です。チェックの結果「OKだったか(true)／ダメだったか(false)」を呼び出し元に伝えるためです。
- **`const newErrors = {}`** … まず「**今回のエラーを集める、空っぽの箱**」を用意します。これからチェックして、エラーがあればこの箱に入れていきます。
- **`formData.title.trim() === ""`** … `.trim()`（トリム）は「文字列の**前後の空白を取り除く**」働きをします。`=== ""` は「**取り除いた結果がからっぽか**」の判定です。
  - つまり「スペースだけ入力した」場合も「未入力」とみなせます。`=== ""` の `===`（イコール3つ）は第2章で学んだ「**厳密に等しいか**」の比較です（`=` 1つは代入なので別物）。
  - 未入力なら `newErrors.title = "タイトルは必須です";` で、箱にエラーメッセージを入れます。
- **`Number(formData.price)`** … `Number(...)` は「**文字列を数値に変換する**」関数です。`Number("100")` は数値の `100` になります。`Number("")`（空文字）は **`NaN`**（ナン＝Not a Number、数値ではない、を表す特別な値）になります。
  - 条件 `formData.price === "" || Number(formData.price) <= 0` は、`||`（または）で2つをつないで「**価格が空 または 0以下**」のときにエラー、という意味です。マイナスや0、未入力の価格をはじいています。
- **`setErrors(newErrors)`** … 集めたエラーを `errors`（解説2の箱）にセットします。これで画面の赤字メッセージが更新されます。
- **`Object.keys(newErrors).length === 0`** … ここが「OKだったか」を返す部分です。
  - `Object.keys(オブジェクト)` は「そのオブジェクトの**キー（項目名）を配列にして取り出す**」関数です。`{ title: "...", price: "..." }` なら `["title", "price"]` を返します。
  - `.length` はその配列の**個数**。`=== 0` は「**エラーが1個もない**」という意味です。
  - つまりこの関数は「エラーが0個なら `true`（全部OK）、1個でもあれば `false`」を返します。

---

##### 解説5: 送信ボタンを押したときの `handleSubmit`

```tsx
const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
  e.preventDefault();

  if (validate()) {
    console.log("送信データ:", formData);
    setIsSubmitted(true);
  }
};
```

- **`e.preventDefault();`** … これは**フォームでとても重要なおまじない**です。
  - HTMLの `<form>` は、送信ボタンを押すと**もともと「ページ全体を再読み込みして別ページへ移動する」という昔ながらの動作**を持っています。
  - `e.preventDefault()`（プリベント・デフォルト＝デフォルト動作を防ぐ）は、その「**勝手なページ再読み込みを止めて**」と指示する命令です。これを書かないと、ボタンを押した瞬間に画面がリロードされ、Reactの処理が動く前にすべてリセットされてしまいます。
  - **フォームの `onSubmit` では、ほぼ必ず最初に `e.preventDefault()` を書く**と覚えてください。
- **`if (validate())`** … 解説4の `validate()` を実行し、戻り値が `true`（＝全項目OK）のときだけ `{ }` の中を実行します。エラーがあれば（`false`）何もせず、赤字メッセージが表示されたままになります。
- **`console.log("送信データ:", formData);`** … 開発者向けに、送信内容をコンソール（開発者ツールの画面）に表示しています。実際のアプリでは、ここで**サーバーにデータを送る処理**を書きます（このチュートリアルでは第7章のSupabase連携で実装します）。
- **`setIsSubmitted(true);`** … 「送信済み」の印を `true` にします。これで次の解説6の画面切り替えが起きます。

---

##### 解説6: 画面の出し分け（早期 return と 条件付き表示）

```tsx
if (isSubmitted) {
  return (
    <div>
      <h2>登録完了！</h2>
      <p>「{formData.title}」を登録しました。</p>
      <button onClick={() => setIsSubmitted(false)}>もう1冊登録する</button>
    </div>
  );
}

return (
  <form onSubmit={handleSubmit}>
    {/* ...フォーム本体... */}
  </form>
);
```

- これは「**送信が済んでいるかどうかで、表示する画面を切り替える**」しくみです。
- **`if (isSubmitted) { return (...) }`** … もし送信済み（`isSubmitted` が `true`）なら、ここで「登録完了！」画面を `return`（返す）して、関数を**ここで終わらせます**。この「条件を満たしたら途中で `return` して抜ける」書き方を「**早期 return（early return）**」と呼びます。
- 早期 return で抜けなかった場合（まだ送信していない場合）だけ、下の `return (<form ...>)` まで進み、フォーム画面が表示されます。
- **`<button onClick={() => setIsSubmitted(false)}>`** … 完了画面のボタンを押すと `isSubmitted` を `false` に戻すので、もう一度フォーム画面に戻れます。

最後に、フォーム本体の中にあるエラー表示も見ておきましょう。

```tsx
{errors.title && <p style={{ color: "red" }}>{errors.title}</p>}
```

- これは「**`errors.title` に中身があるときだけ、赤字のエラーメッセージを表示する**」という書き方です。
- **`A && B`** … `&&`（アンド）を使った `A && B` は「**Aに中身があれば B を表示、Aがからっぽなら何も表示しない**」という、Reactでとてもよく使う「条件付き表示」のテクニックです。
  - `errors.title` にエラーメッセージ（文字列）が入っていれば、その右の `<p style={{ color: "red" }}>...</p>`（赤い文字の段落）が表示されます。
  - エラーが無ければ（`undefined`）何も表示されません。
- `style={{ color: "red" }}` の `{{ }}`（中カッコ2つ）は、第3章で説明した「**JSXの中にスタイルのオブジェクトを書く**」書き方です（外側は「JSの世界に入る」印、内側は「スタイルのオブジェクト」）。

> **ここまでのまとめ:** 難しく見えたコードも、分解すれば「①入力を1つの箱に保存し（スプレッド構文＋動的キー）、②送信時にページ再読み込みを止めて（preventDefault）、③入力チェックし（validate）、④OKなら完了画面に切り替える（早期return）」という流れでした。`Partial<Record<keyof ...>>` のような型も、「エラーを項目ごとに覚える箱」という目的さえ分かれば怖くありません。**一度で全部覚えなくて大丈夫**です。フォームを作るたびにこの節へ戻ってくれば、少しずつ手に馴染んでいきます。

> この結果、画面には書籍登録フォームが表示されます:
>
> **書籍登録**
>
> タイトル *: 【入力欄】
> 著者 *: 【入力欄】
> 価格 *: 【入力欄】
> カテゴリ: 【セレクトボックス（プログラミング）】
> 説明: 【テキストエリア】
>
> 【登録する】ボタン
>
> 何も入力せずに【登録する】を押すと、各必須項目の下に赤字で **「タイトルは必須です」** 等のエラーメッセージが表示されます。
>
> すべての必須項目を入力して送信すると、**「登録完了！」** 画面に切り替わり、**「「React入門」を登録しました。」** と表示されます。

### 5.4 イベントオブジェクトの型 一覧

| イベント | 型 |
|---------|-----|
| `onClick`（ボタン） | `React.MouseEvent<HTMLButtonElement>` |
| `onClick`（div） | `React.MouseEvent<HTMLDivElement>` |
| `onChange`（input） | `React.ChangeEvent<HTMLInputElement>` |
| `onChange`（select） | `React.ChangeEvent<HTMLSelectElement>` |
| `onChange`（textarea） | `React.ChangeEvent<HTMLTextAreaElement>` |
| `onSubmit`（form） | `React.FormEvent<HTMLFormElement>` |
| `onKeyDown` | `React.KeyboardEvent<HTMLInputElement>` |
| `onFocus` / `onBlur` | `React.FocusEvent<HTMLInputElement>` |

---

## 6. useEffect

### 6.1 副作用とは

React コンポーネントの主な仕事は「UI を描画すること（レンダリング：Rendering。state や props からJSXを作る一連の処理）」です。それ以外の処理を**副作用**（Side Effect：サイドエフェクト。レンダリング以外の、外部世界へ影響を及ぼす処理）と呼びます。「画面を描く」関数の中で副作用を直接実行すると、レンダリングのたびに発生してしまったり、Strict Mode で関数が2回呼ばれるために2回実行されてしまったりするため、`useEffect` で隔離します。

例:
- API からデータを取得する
- DOM を直接操作する
- タイマーを設定する
- ログを出力する
- ローカルストレージにアクセスする
- 外部サービスに接続する

これらの処理は **`useEffect`** フックの中で行います。

### 6.2 基本的な使い方

> **▼ このコードがやること（先に日本語で）:** 「カウントが変わるたびに、ブラウザのタブに出る文字も一緒に変える」という例で、**`useEffect`** の基本を学びます。`useEffect` は「**画面が描かれた"あと"に、何か追加の処理（副作用）をする**」ための道具です。書き方は `useEffect(やりたい処理, 依存配列)` の2点セット。**依存配列**（2つ目の `[count]` の部分）は「**この中の値が変わったときだけ、処理をやり直す**」という指定で、ここが `useEffect` の一番のキモです。`[count]` なら「count が変わるたびに実行」、空の `[]` なら「最初の1回だけ実行」になります。

```tsx
// ============================================================================
// useEffect の超基本サンプル（詳細コメント版）
// ----------------------------------------------------------------------------
// 「count が変わるたびに、ブラウザタブのタイトルも追従して変わる」コンポーネント
// ============================================================================

// useState（状態を持つフック）と useEffect（副作用を扱うフック）を取り込む
import { useState, useEffect } from "react";

function PageTitle() {
  // count: 数値の状態。初期値は 0。
  const [count, setCount] = useState<number>(0);

  // ──────────────────────────────────────────────────────────────────────
  // useEffect: 「画面が描かれた *あと* に何かをする」ためのフック
  // ──────────────────────────────────────────────────────────────────────
  // 第1引数: 実行する関数（副作用本体）
  // 第2引数: 依存配列（この配列の中身が変わったときだけ第1引数が実行される）
  //
  // ここでは [count] を指定しているので「count が変わったら関数を実行」となる。
  // 初回マウント時にも1回実行される（このときも count = 0 → 0 という変化扱い）。
  useEffect(() => {
    // document.title はブラウザタブ上部の文字を変える命令（DOM操作 = 副作用）。
    // 副作用を JSX の中で直接書くと不安定なので、useEffect の中に隔離する。
    document.title = `カウント: ${count}`;

    // 開発者ツールの Console タブに出力する。動作確認に便利。
    console.log(`useEffect が実行されました（count = ${count}）`);
  }, [count]); // ← count が変わるたびに上の関数が再実行される

  // 画面は「現在のカウント」と「+1ボタン」だけ。
  return (
    <div>
      <p>カウント: {count}</p>
      <button onClick={() => setCount(count + 1)}>+1</button>
    </div>
  );
}
```

**▼ ブラウザ画面（初期表示）:**

```
タブタイトル: 「カウント: 0」          ← document.title が変更された
┌─────────────────────────┐
│  カウント: 0              │
│  [ +1 ]                   │
└─────────────────────────┘
```

**▼ Console タブ（開発者ツール）の出力:**

```
useEffect が実行されました（count = 0）
```

**▼ 「+1」ボタンを3回押した後:**

```
タブタイトル: 「カウント: 3」
画面: カウント: 3
Console:
  useEffect が実行されました（count = 0）  ← 初回マウント時
  useEffect が実行されました（count = 1）  ← 1回目クリック後
  useEffect が実行されました（count = 2）  ← 2回目クリック後
  useEffect が実行されました（count = 3）  ← 3回目クリック後
```

> **依存配列のキモ:** もし `[count]` を `[]`（空配列）にすると、初回しか実行されないため、タブタイトルが「カウント: 0」のまま固まります。逆に第2引数を**完全に省略**すると毎回再描画ごとに実行され、無限ループになりがちです。「何が変わったらこの副作用を再実行したいか？」を意識して書くのが鉄則です。
>
> なお開発モードの **Strict Mode** が有効だと、初回の useEffect は **わざと2回呼ばれます**（マウント → アンマウント → 再マウントをシミュレートして、クリーンアップ忘れを検出するため）。「console.log が2回出る？」と驚かないようにしましょう。本番ビルドでは1回しか呼ばれません。

### 6.3 依存配列

依存配列の指定方法によって、`useEffect` の実行タイミングが変わります。

> **▼ このコードがやること（先に日本語で）:** `useEffect` の2つ目の引数「**依存配列**」を、書き方ごとに比べます。覚えることは3パターンだけ——①**省略**すると「毎回（再描画のたび）実行」（基本使わない／無限ループの危険）、②**空の `[]`** にすると「**最初の1回だけ**実行」（データ取得の初期化に最適）、③**`[値]`** にすると「**その値が変わったときだけ**実行」。この「いつ実行されるか」を依存配列でコントロールするのが `useEffect` の使いこなしの中心です。

```tsx
import { useState, useEffect } from "react";

function EffectExamples() {
  const [count, setCount] = useState<number>(0);
  const [name, setName] = useState<string>("");

  // パターン1: 毎回実行（依存配列なし）
  // 第2引数を完全に省略すると、コンポーネントが再描画されるたびに毎回実行される。
  // 注意: 中で state を更新するとすぐ無限ループに陥る。ほぼ使うべきでない。
  useEffect(() => {
    console.log("毎回のレンダリング後に実行");
  }); // ← 第2引数を省略

  // パターン2: 初回のみ実行（空の依存配列）
  // [] は「依存する値が無い」という意味なので、初回マウント時1回だけ実行される。
  // ※ Strict Mode（開発時のみ）では2回呼ばれるが、本番では1回。
  useEffect(() => {
    console.log("コンポーネントのマウント時に1回だけ実行");
    // API からデータを取得する処理などをここに書く
  }, []); // ← 空の配列

  // パターン3: 特定の値が変わったときに実行
  // [count] と書くと「count が前回と異なる場合だけ」関数が再実行される。
  // 初回マウント時にも1回実行される（前回値がない状態を「変化あり」とみなすため）。
  useEffect(() => {
    console.log(`count が変わりました: ${count}`);
  }, [count]); // ← count が変わるたびに実行

  // パターン4: 複数の依存値
  // 配列内の値のうち1つでも変わったら実行される。
  useEffect(() => {
    console.log(`count または name が変わりました: ${count}, ${name}`);
  }, [count, name]); // ← count または name が変わるたびに実行

  return (
    <div>
      <p>カウント: {count}</p>
      <button onClick={() => setCount(count + 1)}>+1</button>
      <input value={name} onChange={(e) => setName(e.target.value)} />
    </div>
  );
}
```

> **依存配列の3パターンまとめ:**
>
> - **省略する**: 毎レンダリング後に実行（ほぼ使わない）。
> - **`[]`（空配列）**: マウント時に1回だけ実行（API初期取得などに最適）。
> - **`[a, b]`（値を指定）**: 配列の中身が前回と異なるときだけ実行。
>
> 依存配列に含めるべき値を抜かす（**stale closure**：古い変数を参照したまま動くバグ）と、想定通りに動かなくなります。基本的には ESLint の `react-hooks/exhaustive-deps` ルールに従って、effect 内で使う変数は全部入れるのが安全です。

| 依存配列 | 実行タイミング | 用途 |
|---------|--------------|------|
| 省略 | 毎回のレンダリング後 | ほとんど使わない |
| `[]`（空配列） | 初回マウント時のみ | API 初期データ取得 |
| `[count]` | `count` が変わった時 | 値の変化に応じた処理 |
| `[count, name]` | いずれかが変わった時 | 複数の値に応じた処理 |

#### 実践例: API からデータを取得

```tsx
import { useState, useEffect } from "react";

// 受け取るユーザーデータの型
type User = {
  id: number;
  name: string;
  email: string;
};

function UserList() {
  // 取得したユーザー配列。初期値は空配列。
  const [users, setUsers] = useState<User[]>([]);
  // 読み込み中フラグ
  const [loading, setLoading] = useState<boolean>(true);
  // エラー情報。エラー無しなら null。string | null のユニオン型を明示。
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // API からユーザーデータを取得
    // async/await は「非同期処理を同期っぽく書く」JS の構文。
    // useEffect の第1引数の関数自体は async にできないため、内側に async 関数を定義する。
    const fetchUsers = async () => {
      try {
        setLoading(true);
        // fetch はブラウザ組み込みのHTTPクライアント関数。Promise を返す。
        // await でレスポンスが返ってくるまで待つ。
        const response = await fetch("https://jsonplaceholder.typicode.com/users");

        // response.ok は HTTPステータスが200-299のとき true。
        // それ以外（404, 500 等）は手動でエラーを投げる必要がある（fetch は HTTP エラーで reject しない）。
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }

        // .json() はレスポンスボディを JSON としてパースする非同期メソッド。
        // 型注釈 User[] で「返ってくるのは User の配列」と宣言。
        const data: User[] = await response.json();
        setUsers(data);
      } catch (err) {
        // err が Error インスタンスならその message を、そうでなければ汎用文言を使う。
        // err instanceof Error は TypeScript の型ガード。
        setError(err instanceof Error ? err.message : "不明なエラー");
      } finally {
        // finally ブロックは成功・失敗どちらでも必ず実行される。
        setLoading(false);
      }
    };

    fetchUsers();
  }, []); // 空配列 → 初回マウント時に1回だけ実行

  // 早期 return パターンで状態別の画面を切り替え
  if (loading) {
    return <p>読み込み中...</p>;
  }

  if (error) {
    return <p style={{ color: "red" }}>エラー: {error}</p>;
  }

  // 正常時の表示
  return (
    <div>
      <h2>ユーザー一覧</h2>
      <ul>
        {users.map((user) => (
          // key には DB の id を使う
          <li key={user.id}>
            {user.name} ({user.email})
          </li>
        ))}
      </ul>
    </div>
  );
}
```

> この結果、画面にはまず **「読み込み中...」** と表示されます。
>
> API からデータ取得が完了すると、**「ユーザー一覧」** という見出しと、ユーザーのリストが表示されます:
>
> - Leanne Graham (Sincere@april.biz)
> - Ervin Howell (Shanna@melissa.tv)
> - ... (以下続く)
>
> ネットワークエラーが発生した場合は、赤字で **「エラー: Failed to fetch」** 等と表示されます。

### 6.4 クリーンアップ

`useEffect` の中で `return` した関数は、**クリーンアップ関数**として実行されます。コンポーネントがアンマウント（画面から消える）されるとき、または次の effect が実行される前に呼ばれます。

```tsx
import { useState, useEffect } from "react";

function Timer() {
  // 経過秒数
  const [seconds, setSeconds] = useState<number>(0);
  // 動作中フラグ（true=動作中, false=停止）
  const [isRunning, setIsRunning] = useState<boolean>(false);

  useEffect(() => {
    // 早期 return: 動作中でなければ何もしない（後続のクリーンアップも登録されない）
    if (!isRunning) return; // タイマーが停止中なら何もしない

    // 1秒ごとにカウントアップ
    // setInterval(関数, ミリ秒) はブラウザ組み込み関数。
    // 指定ミリ秒ごとに関数を実行し続け、識別ID（intervalId）を返す。
    const intervalId = setInterval(() => {
      // 関数型更新を使うと、依存配列に seconds を入れなくて済む。
      setSeconds((prev) => prev + 1);
    }, 1000);

    // クリーンアップ: タイマーを解除する
    // useEffect の中で return した関数は「次の effect 実行直前」または「アンマウント時」に呼ばれる。
    // タイマーを止めないと、コンポーネントが消えても動き続けてメモリリークになる。
    return () => {
      clearInterval(intervalId);                  // タイマー停止
      console.log("タイマーをクリーンアップしました");
    };
  }, [isRunning]); // isRunning が変わるたびに再設定

  // リセット処理: 停止＋秒数0
  const handleReset = () => {
    setIsRunning(false);
    setSeconds(0);
  };

  return (
    <div>
      <h2>タイマー</h2>
      <p style={{ fontSize: "48px", fontFamily: "monospace" }}>
        {/* 分: 秒数を60で割って小数点以下を切り捨て、2桁0埋め */}
        {Math.floor(seconds / 60)
          .toString()                       // 数値 → 文字列に変換
          .padStart(2, "0")}                // 2桁にして足りなければ "0" を前に追加 (1 → "01")
        {/* 区切り文字 ":" 続いて 秒（60で割った余り、2桁0埋め） */}
        :{(seconds % 60).toString().padStart(2, "0")}
      </p>
      {/* 動作中なら開始ボタンを無効化 */}
      <button onClick={() => setIsRunning(true)} disabled={isRunning}>
        開始
      </button>
      {/* 停止中なら停止ボタンを無効化 */}
      <button onClick={() => setIsRunning(false)} disabled={!isRunning}>
        停止
      </button>
      <button onClick={handleReset}>リセット</button>
    </div>
  );
}
```

> この結果、画面には大きなフォントで **「00:00」** と表示され、3つのボタンがあります。
>
> 【開始】を押すと、1秒ごとにカウントアップ: **「00:01」** → **「00:02」** → ...
>
> 【停止】を押すと、カウントが止まります。
>
> 【リセット】を押すと、**「00:00」** に戻り、停止します。

#### ウィンドウサイズの監視（クリーンアップの実用例）

```tsx
import { useState, useEffect } from "react";

// ウィンドウサイズの型
type WindowSize = {
  width: number;
  height: number;
};

function WindowSizeDisplay() {
  // window.innerWidth / innerHeight はブラウザの「ビューポート」サイズ。
  // useState の初期値として使うことで、初回描画から正しい値を表示できる。
  const [windowSize, setWindowSize] = useState<WindowSize>({
    width: window.innerWidth,
    height: window.innerHeight,
  });

  useEffect(() => {
    // リサイズイベントのハンドラ
    const handleResize = () => {
      setWindowSize({
        width: window.innerWidth,
        height: window.innerHeight,
      });
    };

    // イベントリスナーを登録
    // window.addEventListener("resize", 関数) でウィンドウサイズ変化時に関数が呼ばれる。
    window.addEventListener("resize", handleResize);

    // クリーンアップ: イベントリスナーを解除
    // 解除し忘れると、コンポーネントが何度もマウント/アンマウントするたびに
    // リスナーが増殖してメモリリーク＆パフォーマンス低下を招く。
    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, []); // 初回マウント時にのみ設定

  return (
    <div>
      <h2>ウィンドウサイズ</h2>
      <p>幅: {windowSize.width}px</p>
      <p>高さ: {windowSize.height}px</p>
    </div>
  );
}
```

> この結果、画面には **「幅: 1920px」** **「高さ: 1080px」** のように現在のウィンドウサイズが表示されます。ブラウザのウィンドウをリサイズすると、数値がリアルタイムに変化します。

### 6.5 コンポーネントライフサイクルと useEffect

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; display: flex; flex-direction: column; gap: 16px;">
  <!-- Mount Phase -->
  <div style="background: #eff6ff; border-radius: 12px; padding: 20px; border: 1px solid #bfdbfe;">
    <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 14px;">
      <span style="background: #1e40af; color: white; padding: 4px 12px; border-radius: 8px; font-size: 13px; font-weight: 700;">1</span>
      <span style="font-weight: 700; font-size: 14px; color: #1e40af;">マウント（初回表示）</span>
    </div>
    <div style="display: flex; flex-direction: column; gap: 6px; padding-left: 12px;">
      <div style="display: flex; align-items: center; gap: 8px;">
        <div style="background: white; border: 1px solid #cbd5e1; padding: 6px 14px; border-radius: 8px; font-size: 12px; flex: 1;">コンポーネント関数が実行される</div>
      </div>
      <div style="color: #94a3b8; font-size: 14px; padding-left: 20px;">↓</div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <div style="background: white; border: 1px solid #cbd5e1; padding: 6px 14px; border-radius: 8px; font-size: 12px; flex: 1;">JSX が返される</div>
      </div>
      <div style="color: #94a3b8; font-size: 14px; padding-left: 20px;">↓</div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <div style="background: white; border: 1px solid #cbd5e1; padding: 6px 14px; border-radius: 8px; font-size: 12px; flex: 1;">DOM に反映</div>
      </div>
      <div style="color: #94a3b8; font-size: 14px; padding-left: 20px;">↓</div>
      <div style="display: flex; align-items: center; gap: 8px;">
        <div style="background: #3b82f6; color: white; padding: 6px 14px; border-radius: 8px; font-size: 12px; font-weight: 600; flex: 1;">useEffect が実行される（依存配列: []）</div>
      </div>
    </div>
  </div>
  <!-- Arrow down -->
  <div style="text-align: center; color: #1e40af; font-size: 20px; font-weight: bold;">↓</div>
  <!-- Update Phase -->
  <div style="background: #fefce8; border-radius: 12px; padding: 20px; border: 1px solid #fde68a; position: relative;">
    <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 14px;">
      <span style="background: #a16207; color: white; padding: 4px 12px; border-radius: 8px; font-size: 13px; font-weight: 700;">2</span>
      <span style="font-weight: 700; font-size: 14px; color: #a16207;">更新（state/props 変更時）</span>
      <span style="background: #fbbf24; color: #78350f; padding: 2px 8px; border-radius: 6px; font-size: 10px; margin-left: auto;">繰り返し</span>
    </div>
    <div style="display: flex; flex-direction: column; gap: 6px; padding-left: 12px;">
      <div style="background: white; border: 1px solid #e5e7eb; padding: 6px 14px; border-radius: 8px; font-size: 12px;">state または props が変更</div>
      <div style="color: #94a3b8; font-size: 14px; padding-left: 20px;">↓</div>
      <div style="background: white; border: 1px solid #e5e7eb; padding: 6px 14px; border-radius: 8px; font-size: 12px;">コンポーネント関数が再実行</div>
      <div style="color: #94a3b8; font-size: 14px; padding-left: 20px;">↓</div>
      <div style="background: white; border: 1px solid #e5e7eb; padding: 6px 14px; border-radius: 8px; font-size: 12px;">新しい JSX が返される</div>
      <div style="color: #94a3b8; font-size: 14px; padding-left: 20px;">↓</div>
      <div style="background: white; border: 1px solid #e5e7eb; padding: 6px 14px; border-radius: 8px; font-size: 12px;">差分を検出して DOM を更新</div>
      <div style="color: #94a3b8; font-size: 14px; padding-left: 20px;">↓</div>
      <div style="background: #ef4444; color: white; padding: 6px 14px; border-radius: 8px; font-size: 12px; font-weight: 600;">前回の useEffect のクリーンアップ関数を実行</div>
      <div style="color: #94a3b8; font-size: 14px; padding-left: 20px;">↓</div>
      <div style="background: #3b82f6; color: white; padding: 6px 14px; border-radius: 8px; font-size: 12px; font-weight: 600;">新しい useEffect が実行される（依存配列の値が変わった場合）</div>
    </div>
  </div>
  <!-- Arrow down -->
  <div style="text-align: center; color: #a16207; font-size: 20px; font-weight: bold;">↓</div>
  <!-- Unmount Phase -->
  <div style="background: #fef2f2; border-radius: 12px; padding: 20px; border: 1px solid #fecaca;">
    <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 14px;">
      <span style="background: #991b1b; color: white; padding: 4px 12px; border-radius: 8px; font-size: 13px; font-weight: 700;">3</span>
      <span style="font-weight: 700; font-size: 14px; color: #991b1b;">アンマウント（画面から消える）</span>
    </div>
    <div style="display: flex; flex-direction: column; gap: 6px; padding-left: 12px;">
      <div style="background: white; border: 1px solid #e5e7eb; padding: 6px 14px; border-radius: 8px; font-size: 12px;">コンポーネントが DOM から削除</div>
      <div style="color: #94a3b8; font-size: 14px; padding-left: 20px;">↓</div>
      <div style="background: #ef4444; color: white; padding: 6px 14px; border-radius: 8px; font-size: 12px; font-weight: 600;">useEffect のクリーンアップ関数を実行</div>
    </div>
  </div>
  <!-- Legend -->
  <div style="display: flex; gap: 16px; justify-content: center; margin-top: 4px;">
    <div style="display: flex; align-items: center; gap: 4px;">
      <span style="display: inline-block; width: 12px; height: 12px; background: #3b82f6; border-radius: 3px;"></span>
      <span style="font-size: 11px; color: #64748b;">useEffect 実行</span>
    </div>
    <div style="display: flex; align-items: center; gap: 4px;">
      <span style="display: inline-block; width: 12px; height: 12px; background: #ef4444; border-radius: 3px;"></span>
      <span style="font-size: 11px; color: #64748b;">クリーンアップ実行</span>
    </div>
  </div>
</div>

---

## 7. カスタムフック

### 7.1 なぜカスタムフックを作るのか

複数のコンポーネントで**同じロジック**を使い回したい場合、**カスタムフック**（Custom Hook：ユーザー定義のフック。`use` で始まる関数として作る再利用可能なロジック）を作ります。

**カスタムフックなしの場合:**

```tsx
// ComponentA.tsx
function ComponentA() {
  // ウィンドウサイズの state
  const [windowSize, setWindowSize] = useState({ width: 0, height: 0 });

  useEffect(() => {
    // リサイズ時のハンドラ
    const handleResize = () => {
      setWindowSize({ width: window.innerWidth, height: window.innerHeight });
    };
    handleResize();                                                       // 初回値を反映
    window.addEventListener("resize", handleResize);                      // イベント登録
    return () => window.removeEventListener("resize", handleResize);      // クリーンアップ
  }, []);

  return <p>幅: {windowSize.width}</p>;
}

// ComponentB.tsx — 全く同じロジックをコピペ...
function ComponentB() {
  // 同じ state と同じ effect をコピペで持っている
  const [windowSize, setWindowSize] = useState({ width: 0, height: 0 });

  useEffect(() => {
    const handleResize = () => {
      setWindowSize({ width: window.innerWidth, height: window.innerHeight });
    };
    handleResize();
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  return <p>高さ: {windowSize.height}</p>;
}
```

このように同じコードを何度も書くのは **DRY 原則（Don't Repeat Yourself）** に反します。

### 7.2 基本的な作り方

カスタムフックのルール:
- 関数名は **`use`** で始める（必須。React がフックとして認識する）
- 中で useState、useEffect などのフックを使える
- 値や関数を返す

#### useWindowSize フック

> **▼ このコードがやること（先に日本語で）:** 「今のブラウザ画面の横幅・高さを教えてくれる」**自作の道具（カスタムフック）** を作ります。カスタムフックとは「**`useState` や `useEffect` を使う処理を、`use〇〇` という名前の関数にまとめて、複数のコンポーネントで使い回せるようにしたもの**」です。ここでは「画面サイズを state で覚え、`useEffect` で"画面サイズが変わったら更新する"仕掛けをセットする」処理を `useWindowSize` という1つの関数に閉じ込めています。こうしておくと、どの画面からでも `const size = useWindowSize()` の1行で画面サイズを使えます。

```tsx
import { useState, useEffect } from "react";

type WindowSize = {
  width: number;
  height: number;
};

// カスタムフック: ウィンドウサイズを返す
// 関数名は必ず use で始める（React がフックとして認識する条件）。
// 戻り値の型を WindowSize と明示しておくと利用側の補完が効く。
function useWindowSize(): WindowSize {
  // 内部で他のフック（useState, useEffect）を呼んで状態管理。
  const [windowSize, setWindowSize] = useState<WindowSize>({
    width: window.innerWidth,
    height: window.innerHeight,
  });

  useEffect(() => {
    const handleResize = () => {
      setWindowSize({
        width: window.innerWidth,
        height: window.innerHeight,
      });
    };

    window.addEventListener("resize", handleResize);
    // クリーンアップでリスナー解除
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  // 最終的に値を返す。返した値が呼び出し元コンポーネントで使える。
  return windowSize;
}

// 使い方: どのコンポーネントでも簡単に使える！
function Header() {
  // 分割代入で width だけ取り出す
  const { width } = useWindowSize();
  // 三項演算子で表示テキストを切替
  return <header>{width > 768 ? "デスクトップメニュー" : "モバイルメニュー"}</header>;
}

function Footer() {
  // 同じカスタムフックを別コンポーネントから呼んでも、それぞれ独立に state を持つ。
  // 「フックの共有」はロジックの共有であり、state そのものは共有されない点に注意。
  const { width, height } = useWindowSize();
  return (
    <footer>
      画面サイズ: {width} x {height}
    </footer>
  );
}
```

> `Header` コンポーネントでは、画面幅が768pxより大きい場合 **「デスクトップメニュー」**、小さい場合 **「モバイルメニュー」** と表示されます。ウィンドウをリサイズすると自動的に切り替わります。

#### useLocalStorage フック

```tsx
import { useState, useEffect } from "react";

// ローカルストレージと同期する state を提供するカスタムフック
// <T> は「型を引数で受け取る」ジェネリクス。呼び出し側が string でも number でも指定可能。
// 戻り値の型は「[現在値, 更新関数]」のタプル型（順序固定の配列型）。
function useLocalStorage<T>(key: string, initialValue: T): [T, (value: T) => void] {
  // 初期値: ローカルストレージに値があればそれを使う
  // useState の第1引数に「関数」を渡すと、その関数の戻り値が初期値になる（遅延初期化）。
  // 重い処理を初回マウント時だけ実行したいときに使うパターン。
  const [storedValue, setStoredValue] = useState<T>(() => {
    try {
      // localStorage は文字列でしか保存できないので、保存時は JSON.stringify、
      // 読み出し時は JSON.parse する。
      const item = window.localStorage.getItem(key);
      // item が null（未保存）なら初期値、あればJSONパース。
      // as T は型アサーション（「これは T 型ですよ」と TS に教える）。
      return item ? (JSON.parse(item) as T) : initialValue;
    } catch {
      // パース失敗やストレージ無効時は初期値にフォールバック
      return initialValue;
    }
  });

  // state が変わったらローカルストレージにも保存
  useEffect(() => {
    try {
      window.localStorage.setItem(key, JSON.stringify(storedValue));
    } catch (error) {
      // プライベートブラウジング等で書き込み失敗する可能性がある
      console.error("ローカルストレージへの保存に失敗:", error);
    }
  }, [key, storedValue]); // key と storedValue のどちらか変わったら再実行

  // 配列で「現在値」と「更新関数」を返す → useState とほぼ同じインターフェース
  return [storedValue, setStoredValue];
}

// 使い方
function Settings() {
  // ジェネリクスで型を指定。<string> は省略しても初期値から推論される。
  const [theme, setTheme] = useLocalStorage<string>("theme", "light");
  const [fontSize, setFontSize] = useLocalStorage<number>("fontSize", 16);

  return (
    <div>
      <h2>設定</h2>

      <div>
        <label>テーマ: </label>
        <select value={theme} onChange={(e) => setTheme(e.target.value)}>
          <option value="light">ライト</option>
          <option value="dark">ダーク</option>
        </select>
      </div>

      <div>
        <label>フォントサイズ: {fontSize}px</label>
        <input
          type="range"              // スライダー入力
          min={12}                  // 最小値
          max={24}                  // 最大値
          value={fontSize}
          onChange={(e) => setFontSize(Number(e.target.value))}    // 文字列→数値変換
        />
      </div>

      {/* テンプレートリテラルで "16px" のようなCSS値を作る */}
      <p style={{ fontSize: `${fontSize}px` }}>
        このテキストのフォントサイズが変わります。
      </p>
    </div>
  );
}
```

> この結果、画面にはテーマ切り替えのセレクトボックスとフォントサイズのスライダーが表示されます。
>
> フォントサイズを20pxに変更すると、プレビューテキストのサイズが大きくなります。
>
> **ページを再読み込みしても設定が保持されます**（ローカルストレージに保存されているため）。

#### useToggle フック

```tsx
import { useState, useCallback } from "react";

// true/false を切り替えるシンプルなカスタムフック
// useCallback は「関数をメモ化（記憶）して、依存配列が変わるまで同じ参照を返す」フック。
// 子コンポーネントに props として関数を渡すときに、不要な再レンダリングを防ぐ目的で使う。
function useToggle(initialValue: boolean = false): [boolean, () => void] {
  // 真偽値の state
  const [value, setValue] = useState<boolean>(initialValue);

  // useCallback(関数, 依存配列) で関数をメモ化。
  // 依存配列 [] なので、この toggle 関数の参照はコンポーネントの寿命を通じて変わらない。
  // setValue(prev => !prev) は関数型更新で、前の値を反転する。
  const toggle = useCallback(() => {
    setValue((prev) => !prev);
  }, []);

  // [現在値, 切替関数] のタプルを返す
  return [value, toggle];
}

// 使い方
function App() {
  // 3つの独立した toggle 状態
  const [isMenuOpen, toggleMenu] = useToggle(false);
  const [isDarkMode, toggleDarkMode] = useToggle(false);
  const [isModalOpen, toggleModal] = useToggle(false);

  return (
    // 背景・文字色を isDarkMode で切替
    <div style={{ backgroundColor: isDarkMode ? "#333" : "#fff", color: isDarkMode ? "#fff" : "#000" }}>
      <button onClick={toggleDarkMode}>
        {isDarkMode ? "ライトモード" : "ダークモード"}に切り替え
      </button>

      <button onClick={toggleMenu}>
        メニュー{isMenuOpen ? "を閉じる" : "を開く"}
      </button>

      {/* && 演算子: isMenuOpen が true のときだけ <nav>...</nav> を表示 */}
      {isMenuOpen && (
        <nav>
          <ul>
            <li>ホーム</li>
            <li>書籍一覧</li>
            <li>設定</li>
          </ul>
        </nav>
      )}

      <button onClick={toggleModal}>モーダルを開く</button>

      {isModalOpen && (
        <div style={{ border: "2px solid #ccc", padding: "16px", margin: "16px" }}>
          <h3>モーダルの内容</h3>
          <p>これはモーダルウィンドウです。</p>
          {/* モーダル内の閉じるボタンも同じ toggleModal を共有 */}
          <button onClick={toggleModal}>閉じる</button>
        </div>
      )}
    </div>
  );
}
```

> この結果、画面には3つのボタンが表示されます。
>
> 【ダークモードに切り替え】を押すと、背景が黒・文字が白に変わり、ボタンのテキストが **「ライトモードに切り替え」** に変わります。
>
> 【メニューを開く】を押すと、ナビゲーションリストが表示され、ボタンが **「メニューを閉じる」** に変わります。

### 7.3 useBooks フックの予告

後の章（第5章以降）で、書籍管理アプリのために以下のような `useBooks` カスタムフックを実装します。

```tsx
// 予告: 後の章で実装するカスタムフック
// このフックが返す値の型
type UseBooksReturn = {
  books: Book[];                                                       // 全書籍データ
  loading: boolean;                                                    // 読み込み中フラグ
  error: string | null;                                                // エラーメッセージ または null
  // Omit<Book, "id"> は「Book 型から id プロパティを除外した型」（新規追加時はIDは未確定）
  // Promise<void> は「非同期処理だが戻り値なし」を表す
  addBook: (book: Omit<Book, "id">) => Promise<void>;
  // Partial<Book> は「Book の全プロパティをオプショナル化」した型（一部だけ更新する用）
  updateBook: (id: number, updates: Partial<Book>) => Promise<void>;
  deleteBook: (id: number) => Promise<void>;
  searchBooks: (query: string) => void;                                // 検索クエリ設定
  filteredBooks: Book[];                                               // 検索結果
};

function useBooks(): UseBooksReturn {
  // API との通信、state 管理、検索ロジックなどを
  // このフックに集約する予定
  // → 第5章「API連携」で詳しく実装します
}

// 使い方（完成イメージ）
function BookListPage() {
  // 必要な値だけを分割代入で取り出す
  const { books, loading, error, deleteBook, searchBooks, filteredBooks } = useBooks();

  // 状態別の早期 return
  if (loading) return <p>読み込み中...</p>;
  if (error) return <p>エラー: {error}</p>;

  return (
    <div>
      {/* 子コンポーネントに「検索処理」を関数として渡す */}
      <SearchBar onSearch={searchBooks} />
      {/* リストデータと削除コールバックを渡す */}
      <BookList books={filteredBooks} onDelete={deleteBook} />
    </div>
  );
}
```

> このカスタムフックを使うことで、書籍データの取得・追加・更新・削除・検索といったすべてのロジックが1箇所にまとまり、コンポーネントは UI の描画に集中できるようになります。

---

## 8. よくあるミスと対処法

### 8.1 state の直接変更

**最も多い間違い**: state のオブジェクトや配列を直接変更してしまうこと。

> **▼ このコードがやること（先に日本語で）:** 初心者が**最もやりがちな失敗**——「stateを直接書き換えてしまう」——の **NG例とOK例を並べて**示します。`user.name = "佐藤"` のように直接いじると、**Reactが変化に気づけず、画面が更新されません**。正しくは、スプレッド構文 `...` で**新しいオブジェクト／配列を作って** `setUser(...)` / `setItems(...)` に渡します。「直接いじらず、作り替えて差し替える」——4.4・4.5でも出たこの鉄則を、失敗例とセットで体に染み込ませる節です。

```tsx
import { useState } from "react";

type User = {
  name: string;
  age: number;
};

function UserEditor() {
  // オブジェクト state と配列 state
  const [user, setUser] = useState<User>({ name: "田中", age: 25 });
  const [items, setItems] = useState<string[]>(["A", "B", "C"]);

  // ========== NG な例 ==========

  const badUpdateName = () => {
    // NG: オブジェクトのプロパティを直接変更
    // user オブジェクトは React 内部にも参照されている。
    // .name を直接書き換えてしまうと、React が「同じ参照だから変化なし」と判断して再描画しない。
    user.name = "鈴木";
    setUser(user); // 同じ参照のオブジェクトなので React は変化を検知できない！
  };

  const badAddItem = () => {
    // NG: 配列を直接変更
    // push は配列を変更する破壊的メソッド。
    items.push("D");
    setItems(items); // 同じ参照の配列なので React は変化を検知できない！
  };

  const badSortItems = () => {
    // NG: sort は元の配列を変更する破壊的メソッド
    items.sort();
    setItems(items);
  };

  // ========== OK な例 ==========

  const goodUpdateName = () => {
    // OK: 新しいオブジェクトを作成
    // {...user, name: "鈴木"} は「user のコピー + name 上書き」の新しいオブジェクト。
    setUser({ ...user, name: "鈴木" });
  };

  const goodAddItem = () => {
    // OK: 新しい配列を作成
    // [...items, "D"] は「items の全要素 + "D"」の新しい配列。
    setItems([...items, "D"]);
  };

  const goodSortItems = () => {
    // OK: コピーしてからソート
    // [...items] でまずコピーを作り、その上で .sort()。元の items は変わらない。
    setItems([...items].sort());
  };

  return (
    <div>
      <p>名前: {user.name}</p>
      {/* join(", ") は配列要素を区切り文字で連結した文字列を作る */}
      <p>アイテム: {items.join(", ")}</p>
      <button onClick={goodUpdateName}>名前を変更</button>
      <button onClick={goodAddItem}>アイテム追加</button>
      <button onClick={goodSortItems}>ソート</button>
    </div>
  );
}
```

> **なぜ直接変更がダメなのか**: React は state の更新を**参照の比較（`===`：Object.is 相当）** で検出します。同じオブジェクト/配列への参照のままだと、中身が変わっていても「変化なし」と判断され、再レンダリングが発生しません。これを **イミュータブル更新**（Immutable Update：不変更新。常に新しいオブジェクトを作って更新する考え方）と呼びます。

### 8.2 useEffect の無限ループ

> **▼ このコードがやること（先に日本語で）:** `useEffect` で起こりがちな**「無限ループ」**——処理が止まらずアプリが固まる失敗——の**典型パターンと、その直し方**を示します。原因はだいたい同じで、「**`useEffect` の中で state を更新しているのに、依存配列の指定を間違えている**」こと。すると『state更新 → 再描画 → またuseEffect実行 → state更新 →…』が永遠に続きます。この節で「やってはいけない書き方」を知っておくと、実際に画面が固まったとき、すぐ原因に気づけます。

```tsx
import { useState, useEffect } from "react";

function InfiniteLoopExample() {
  const [count, setCount] = useState<number>(0);
  const [data, setData] = useState<string[]>([]);

  // ========== NG: 無限ループ ==========

  // パターン1: 依存配列を省略して state を更新
  // 依存配列なしの useEffect は毎回のレンダリング後に走る。
  // 中で setCount を呼ぶと state が変わり → 再レンダリング → またこの useEffect が走る → ...
  useEffect(() => {
    setCount(count + 1); // state 更新 → 再レンダリング → useEffect 再実行 → state 更新 → ...
  }); // 依存配列がない！

  // パターン2: useEffect 内で毎回新しいオブジェクト/配列を state に設定
  // ["A","B","C"] は毎回「新しい配列オブジェクト」として作られるため、
  // 参照比較では常に「変化あり」となり、再描画が止まらない。
  useEffect(() => {
    setData(["A", "B", "C"]); // 毎回新しい配列オブジェクトが作られる → 再レンダリング → ...
  }); // 依存配列がない！

  // パターン3: 依存配列に毎回変わる値を入れる
  // { key: "value" } は実行のたびに新しいオブジェクト参照になるため、
  // 毎レンダリングで「依存値が変わった」とみなされてしまう。
  useEffect(() => {
    console.log("実行");
  }, [{ key: "value" }]); // オブジェクトリテラルは毎回新しい参照 → 毎回実行

  // ========== OK: 正しい使い方 ==========

  // 修正1: 適切な依存配列を指定
  // [] で初回マウント時のみ実行。関数型更新で外部の count に依存しない。
  useEffect(() => {
    setCount((prev) => prev + 1); // 初回のみ実行
  }, []); // 空配列 → 初回マウント時のみ

  // 修正2: 条件付きで実行
  // 「data が空のときだけ」更新するので無限ループにならない。
  useEffect(() => {
    if (data.length === 0) {
      setData(["A", "B", "C"]); // data が空のときだけ設定
    }
  }, [data.length]);

  // 修正3: useMemo でオブジェクトの参照を安定させる
  // useMemo は「依存配列が変わるまで同じ値（参照）を返す」フック。
  // （useMemo については次章以降で詳しく解説します）

  return <p>カウント: {count}</p>;
}
```

**無限ループを防ぐチェックリスト:**

1. `useEffect` の依存配列は省略していないか?
2. `useEffect` 内で依存配列に含まれる state を無条件に更新していないか?
3. 依存配列にオブジェクトリテラルや配列リテラルを直接書いていないか?
4. `useEffect` 内の関数が毎回再生成されていないか?

### 8.3 key の重要性

`key` は React がリスト内の各要素を識別するために使います。正しい `key` を指定しないと、予期しないバグが発生します。

> **▼ このコードがやること（先に日本語で）:** 2.5 の `map` で出てきた **`key`（各要素の目印）** を、なぜ正しく付けないといけないのかを、**バグが起きる例**で見せます。Reactはリストを更新するとき、`key` を手がかりに「どの項目が同じで、どれが増減したか」を見分けます。ここで**配列の並び順（index）を key に使う**と、項目を削除・並べ替えしたときにReactが取り違え、**入力中の文字が別の行に移る**などの不可解なバグが起きます。だから `key` には「**順番に左右されない固有のID（本やTODOの `id`）**」を使う——これがこの節の結論です。

```tsx
import { useState } from "react";

type Item = {
  id: number;
  text: string;
};

function KeyExample() {
  const [items, setItems] = useState<Item[]>([
    { id: 1, text: "アイテム1" },
    { id: 2, text: "アイテム2" },
    { id: 3, text: "アイテム3" },
  ]);

  // 先頭に追加する
  const addToTop = () => {
    // Date.now() は現在時刻のミリ秒値。簡易的な一意ID生成に使う。
    const newItem: Item = {
      id: Date.now(),
      text: `アイテム${items.length + 1}`,
    };
    // [新規, ...既存] で先頭に挿入した新しい配列を作る
    setItems([newItem, ...items]);
  };

  return (
    <div>
      <button onClick={addToTop}>先頭に追加</button>

      <h3>NG: index を key に使用</h3>
      {/* 先頭に追加すると全要素の index がずれてしまい、React は
          「key=0 の中身が変わった」と解釈する。input の入力値(=非制御の DOM 状態)が
          別の要素に紐づいてしまう。 */}
      {items.map((item, index) => (
        <div key={index}>
          <span>{item.text}</span>
          {/* input の値が正しい要素に紐づかない！ */}
          <input type="text" placeholder="メモを入力" />
        </div>
      ))}

      <h3>OK: 一意な id を key に使用</h3>
      {/* item.id は要素固有なので、配列の中で順序が変わっても
          「この id の要素はこの DOM ノード」という対応が保たれる。 */}
      {items.map((item) => (
        <div key={item.id}>
          <span>{item.text}</span>
          <input type="text" placeholder="メモを入力" />
        </div>
      ))}
    </div>
  );
}
```

> この結果を確認するには、以下の操作をしてみてください:
>
> 1. 「NG」セクションの各入力欄に「メモA」「メモB」「メモC」と入力する
> 2. 【先頭に追加】ボタンをクリックする
> 3. **NG の方**: 入力したメモの位置がずれる! 「メモA」が新しいアイテムの横に表示される
> 4. **OK の方**: 入力したメモは元のアイテムに正しく紐づいたまま
>
> これは、`index` を key にすると、先頭に要素を挿入したとき全要素の index が変わり、React が要素の対応を正しく把握できなくなるためです。

**key のベストプラクティス:**

```tsx
// ベスト: データベースの ID を使う
// 配列内で一意かつ、再レンダリングしても変わらない値が理想。
{items.map((item) => <Item key={item.id} />)}

// OK: ユニークな文字列を使う
// slug 等の人間が読めるユニークIDでもOK。
{items.map((item) => <Item key={item.slug} />)}

// 最終手段: index を使う（並び替え・追加・削除がない静的リストのみ）
// 並びが固定で、要素が増減しないリストでは index でも問題ない。
{staticItems.map((item, index) => <Item key={index} />)}

// NG: ランダム値を使う（毎回変わるので意味がない）
// Math.random() は描画のたびに違う値を返す。key の意味（=同一性の判別）が成立しない。
// 全要素が毎回「別物」と判定され、入力欄の状態が失われる等の不具合の元。
{items.map((item) => <Item key={Math.random()} />)}
```

---

## 書籍管理アプリのコンポーネント設計

この章で学んだ知識を活かして、書籍管理アプリの全体像を確認しましょう。

<div style="max-width: 680px; margin: 20px auto; font-family: 'Segoe UI', sans-serif; background: #f8fafc; border-radius: 12px; padding: 24px; border: 1px solid #e2e8f0; box-shadow: 0 2px 8px rgba(0,0,0,0.06);">
  <!-- App -->
  <div style="text-align: center; margin-bottom: 6px;">
    <span style="display: inline-block; background: #1e293b; color: white; padding: 8px 20px; border-radius: 8px; font-weight: 700; font-size: 14px;">App</span>
  </div>
  <div style="text-align: center; color: #94a3b8; font-size: 14px;">│</div>
  <!-- Layout -->
  <div style="text-align: center; margin-bottom: 6px;">
    <span style="display: inline-block; background: #334155; color: white; padding: 7px 18px; border-radius: 8px; font-weight: 600; font-size: 13px;">Layout</span>
  </div>
  <div style="text-align: center; color: #94a3b8; font-size: 13px;">├──────────────────┼──────────────────┤</div>
  <!-- Header / MainContent / Footer -->
  <div style="display: flex; justify-content: space-between; gap: 8px; margin-top: 8px; margin-bottom: 16px;">
    <!-- Header branch -->
    <div style="flex: 1; text-align: center;">
      <div style="background: #10b981; color: white; padding: 6px 8px; border-radius: 8px; font-size: 11px; font-weight: 600; margin-bottom: 6px;">Header</div>
      <div style="color: #94a3b8; font-size: 11px;">├───┼───┤</div>
      <div style="display: flex; gap: 3px; margin-top: 4px; justify-content: center; flex-wrap: wrap;">
        <span style="background: #d1fae5; color: #065f46; padding: 3px 6px; border-radius: 5px; font-size: 10px;">Logo</span>
        <span style="background: #d1fae5; color: #065f46; padding: 3px 6px; border-radius: 5px; font-size: 10px;">Navigation</span>
        <span style="background: #d1fae5; color: #065f46; padding: 3px 6px; border-radius: 5px; font-size: 10px;">ThemeToggle<br/><span style="font-size: 9px; color: #6b7280;">state: isDark</span></span>
      </div>
    </div>
    <!-- MainContent branch -->
    <div style="flex: 2.5; text-align: center;">
      <div style="background: #10b981; color: white; padding: 6px 8px; border-radius: 8px; font-size: 11px; font-weight: 600; margin-bottom: 6px;">MainContent<br/><span style="font-weight: 400; font-size: 10px;">（ルーティングで切り替え）</span></div>
      <div style="color: #94a3b8; font-size: 11px;">├──────┼──────┼──────┤</div>
      <div style="display: flex; gap: 4px; margin-top: 4px; justify-content: center; flex-wrap: wrap;">
        <span style="background: #3b82f6; color: white; padding: 4px 8px; border-radius: 6px; font-size: 10px; font-weight: 600;">HomePage</span>
        <span style="background: #3b82f6; color: white; padding: 4px 8px; border-radius: 6px; font-size: 10px; font-weight: 600;">BookListPage<br/><span style="font-weight: 400; font-size: 9px;">state: books, searchQuery</span></span>
        <span style="background: #3b82f6; color: white; padding: 4px 8px; border-radius: 6px; font-size: 10px; font-weight: 600;">BookDetailPage<br/><span style="font-weight: 400; font-size: 9px;">state: book, loading</span></span>
        <span style="background: #3b82f6; color: white; padding: 4px 8px; border-radius: 6px; font-size: 10px; font-weight: 600;">AddBookPage<br/><span style="font-weight: 400; font-size: 9px;">state: formData, errors</span></span>
      </div>
    </div>
    <!-- Footer -->
    <div style="flex: 0.7; text-align: center;">
      <div style="background: #10b981; color: white; padding: 6px 8px; border-radius: 8px; font-size: 11px; font-weight: 600;">Footer</div>
    </div>
  </div>
  <!-- Divider -->
  <div style="border-top: 2px dashed #cbd5e1; margin: 12px 0; position: relative;">
    <span style="position: absolute; top: -10px; left: 50%; transform: translateX(-50%); background: #f8fafc; padding: 0 8px; font-size: 11px; color: #64748b; font-weight: 600;">ページ別の子コンポーネント</span>
  </div>
  <!-- BookListPage details -->
  <div style="background: #eff6ff; border-radius: 10px; padding: 14px; margin-bottom: 10px; border: 1px solid #bfdbfe;">
    <div style="font-size: 12px; font-weight: 700; color: #1e40af; margin-bottom: 8px;">BookListPage の子コンポーネント</div>
    <div style="display: flex; gap: 6px; flex-wrap: wrap; margin-bottom: 8px;">
      <span style="background: white; border: 1px solid #93c5fd; color: #1e40af; padding: 4px 10px; border-radius: 6px; font-size: 11px;">SearchBar<br/><span style="font-size: 9px; color: #6b7280;">props: onSearch / state: query</span></span>
      <span style="background: white; border: 1px solid #93c5fd; color: #1e40af; padding: 4px 10px; border-radius: 6px; font-size: 11px;">FilterPanel<br/><span style="font-size: 9px; color: #6b7280;">props: onFilter / state: selectedCategory</span></span>
      <span style="background: white; border: 1px solid #93c5fd; color: #1e40af; padding: 4px 10px; border-radius: 6px; font-size: 11px;">BookGrid<br/><span style="font-size: 9px; color: #6b7280;">props: books</span></span>
    </div>
    <div style="padding-left: 12px;">
      <div style="font-size: 11px; color: #64748b; margin-bottom: 6px;">↳ BookGrid の子:</div>
      <div style="display: flex; gap: 6px; flex-wrap: wrap; margin-bottom: 8px;">
        <span style="background: #ef4444; color: white; padding: 4px 10px; border-radius: 6px; font-size: 11px; font-weight: 600;">BookCard<br/><span style="font-weight: 400; font-size: 9px;">props: book, onDelete, onToggleFavorite</span></span>
        <span style="background: #ef4444; color: white; padding: 4px 10px; border-radius: 6px; font-size: 11px;">BookCard</span>
        <span style="background: #ef4444; color: white; padding: 4px 10px; border-radius: 6px; font-size: 11px;">BookCard...</span>
      </div>
      <div style="padding-left: 12px;">
        <div style="font-size: 11px; color: #64748b; margin-bottom: 6px;">↳ BookCard の子:</div>
        <div style="display: flex; gap: 4px; flex-wrap: wrap;">
          <span style="background: #fecaca; color: #991b1b; padding: 3px 8px; border-radius: 5px; font-size: 10px;">BookImage<br/><span style="font-size: 9px;">props: src, alt</span></span>
          <span style="background: #fecaca; color: #991b1b; padding: 3px 8px; border-radius: 5px; font-size: 10px;">BookInfo<br/><span style="font-size: 9px;">props: title, author, price</span></span>
          <span style="background: #fecaca; color: #991b1b; padding: 3px 8px; border-radius: 5px; font-size: 10px;">BookActions<br/><span style="font-size: 9px;">props: onEdit, onDelete</span></span>
          <span style="background: #fecaca; color: #991b1b; padding: 3px 8px; border-radius: 5px; font-size: 10px;">RatingStars<br/><span style="font-size: 9px;">props: rating</span></span>
        </div>
      </div>
    </div>
  </div>
  <!-- AddBookPage details -->
  <div style="background: #fffbeb; border-radius: 10px; padding: 14px; margin-bottom: 10px; border: 1px solid #fde68a;">
    <div style="font-size: 12px; font-weight: 700; color: #a16207; margin-bottom: 8px;">AddBookPage の子コンポーネント</div>
    <div style="display: flex; gap: 6px; flex-wrap: wrap; margin-bottom: 8px;">
      <span style="background: #f59e0b; color: white; padding: 4px 10px; border-radius: 6px; font-size: 11px; font-weight: 600;">BookForm<br/><span style="font-weight: 400; font-size: 9px;">props: onSubmit / state: formData, errors</span></span>
    </div>
    <div style="padding-left: 12px;">
      <div style="font-size: 11px; color: #64748b; margin-bottom: 6px;">↳ BookForm の子:</div>
      <div style="display: flex; gap: 4px; flex-wrap: wrap;">
        <span style="background: #fef3c7; color: #92400e; padding: 3px 8px; border-radius: 5px; font-size: 10px;">FormInput<br/><span style="font-size: 9px;">props: label, value, onChange, error</span></span>
        <span style="background: #fef3c7; color: #92400e; padding: 3px 8px; border-radius: 5px; font-size: 10px;">FormSelect<br/><span style="font-size: 9px;">props: options, value, onChange</span></span>
        <span style="background: #fef3c7; color: #92400e; padding: 3px 8px; border-radius: 5px; font-size: 10px;">Button<br/><span style="font-size: 9px;">props: label, onClick, variant</span></span>
      </div>
    </div>
  </div>
  <!-- BookDetailPage details -->
  <div style="background: #f5f3ff; border-radius: 10px; padding: 14px; border: 1px solid #ddd6fe;">
    <div style="font-size: 12px; font-weight: 700; color: #5b21b6; margin-bottom: 8px;">BookDetailPage の子コンポーネント</div>
    <div style="display: flex; gap: 6px; flex-wrap: wrap; margin-bottom: 8px;">
      <span style="background: white; border: 1px solid #c4b5fd; color: #5b21b6; padding: 4px 10px; border-radius: 6px; font-size: 11px;">BookDetail<br/><span style="font-size: 9px; color: #6b7280;">props: book</span></span>
      <span style="background: white; border: 1px solid #c4b5fd; color: #5b21b6; padding: 4px 10px; border-radius: 6px; font-size: 11px;">ReviewList<br/><span style="font-size: 9px; color: #6b7280;">props: reviews</span></span>
    </div>
    <div style="padding-left: 12px;">
      <div style="font-size: 11px; color: #64748b; margin-bottom: 6px;">↳ ReviewList の子:</div>
      <span style="background: #ede9fe; color: #5b21b6; padding: 3px 8px; border-radius: 5px; font-size: 10px;">ReviewCard<br/><span style="font-size: 9px;">props: review</span></span>
    </div>
  </div>
</div>

各コンポーネントにどの `state` が必要で、どの `props` を受け取るかが一目でわかります。後の章でこの設計に基づいて実装を進めていきます。

---

## まとめ

この章で学んだ React の基礎をまとめます。

| 概念 | 要点 |
|------|------|
| **コンポーネント** | UIの再利用可能な部品。関数として定義し、JSX を返す |
| **JSX/TSX** | JavaScript の中に HTML 風のコードを書ける構文 |
| **Props** | 親から子へデータを渡す仕組み。TypeScript で型を定義 |
| **State (useState)** | コンポーネントが持つ変化するデータ。更新すると再レンダリング |
| **イベントハンドリング** | onClick, onChange, onSubmit でユーザー操作を処理 |
| **useEffect** | 副作用（API取得、DOM操作等）を実行するフック |
| **カスタムフック** | ロジックを再利用可能な関数に切り出す仕組み |

**次の章（第4章）** では、React Router を使ったルーティングと、より実践的な画面遷移の実装に進みます。

import re

p = r'C:/Users/oltot/Documents/git-projects/def/next/09-styling.md'
s = open(p, encoding='utf-8').read()

reps = [
    # 0.3 解説1
    ("- これは **Tailwind CSS v4** の書き方です。この章の 1.1 節に出てくる `@tailwind base;` などの3行は **v3** の書き方で、v4 ではこの1行に置き換わりました（詳しくは 1.1.1 節）。",
     "- これは **Tailwind CSS v4** の書き方で、1.1 節のアプリの `globals.css` も同じ1行から始まります。古い記事で見かける `@tailwind base;` などの3行は **v3** の書き方で、v4 ではこの1行に置き換わりました（詳しくは 1.1.1 節）。"),
    # 1.1.1 heading and intro
    ("### 1.1.1 Tailwind CSS v4 での書き方（2026年時点の最新版）",
     "### 1.1.1 v3 と v4 の違い（古い記事・古いプロジェクトを読むときのために）"),
    ("ここまでの `globals.css` と、次の 1.3 節の `tailwind.config.ts` は **v3** の書き方です。2026年に `create-next-app` で作ったプロジェクトは **v4** なので、同じことを v4 ではどう書くかを整理しておきます。いちばん大きな違いは、**設定が JavaScript のファイル（`tailwind.config.ts`）から CSS ファイルの中へ引っ越した**ことです。",
     "1.1 節の `globals.css` は **v4** の書き方です。一方、ネットの記事や 2024年ごろまでに作られたプロジェクトでは、**v3** の書き方（`@tailwind` の3行 ＋ `tailwind.config.ts`）がまだよく見つかります。両者の違いを整理しておきましょう。いちばん大きな違いは、**設定が JavaScript のファイル（`tailwind.config.ts`）から CSS ファイルの中へ引っ越した**ことです。"),
    ('font-weight="bold" fill="#475569">v3 まで（この章の 1.1〜1.3 節）</text>',
     'font-weight="bold" fill="#475569">v3 まで（1.3 節・古い記事）</text>'),
    ('font-weight="bold" fill="#047857">v4（2026年の create-next-app）</text>',
     'font-weight="bold" fill="#047857">v4（この章の 1.1 節）</text>'),
    ("この章の v3 のコードと、v4 での書き方の対応表です。",
     "v3 と v4 の書き方の対応表です。"),
    ("| やりたいこと | v3（この章の本文） | v4 |",
     "| やりたいこと | v3（古い書き方） | v4（この章） |"),
    ("| 自作クラス（`.btn` など） | `@layer components { .btn { … } }` | `@utility btn { … }` |",
     "| 自作クラス（`.btn` など） | `@layer components { .btn { … } }`（`@apply btn` で取り込める） | `@layer components` はそのまま使える（ただし `@apply btn` は不可）。接頭辞を付けたい小さなクラスは `@utility glass { … }` |"),
    ("ほかに、**名前や初期値が変わったクラス**もあります。この章のコードを v4 で使うときは、次の点に気を付けてください。",
     "ほかに、**名前や初期値が変わったクラス**もあります。古い記事のコードを v4 のプロジェクトで使うときは、次の点に気を付けてください。"),
    ("上の表の `@import` と `@theme inline` がもう使われていることが分かります。",
     "上の表の `@import` と `@theme inline` がもう使われていることが分かります。1.1 節の `globals.css` は、このファイルを丸ごと置き換えたものです。"),
    ("#### v4 版の globals.css を小さく作って動かす",
     "#### v4 の globals.css を小さく作って動かす"),
    ("> **▼ このコードがやること（先に日本語で）:** この章の `globals.css` と `tailwind.config.ts` でやっていたことのうち、代表的な4つ「ダークモードの切り替え方式」「切り替わる色の登録」「ブランド色とアニメーションの追加」「自作のボタンクラス」を、v4 の書き方で **CSS ファイル1枚**にまとめます。",
     "> **▼ このコードがやること（先に日本語で）:** 1.1 節の `globals.css` は長いので、その中から代表的な4つ「ダークモードの切り替え方式」「切り替わる色の登録」「ブランド色とアニメーションの追加」「自作のボタンクラス」だけを取り出した**最小版**を作ります。"),
    # 1.3
    ("### 1.3 tailwind.config.ts のカスタマイズ",
     "### 1.3 （参考）v3 の tailwind.config.ts"),
    ("> **注意:** この `tailwind.config.ts` は **v3 用**の設定ファイルです。v4 のプロジェクトでは自動では読まれないので、1.1.1 節の対応表を見ながら `globals.css` の `@theme` などに書き換えるか、`@config` で読み込ませてください。設定項目ごとの「何のための設定か」という考え方は v4 でも同じなので、解説はそのまま役に立ちます。",
     "> **注意:** この節は **v3 のプロジェクト向けの参考**です。v4（この章のアプリ）では、ここに書いてある設定はすべて 1.1 節の `globals.css`（`@custom-variant`・`@theme`・`@utility`）に書き済みなので、**このファイルは作りません**。v4 のプロジェクトに置いても自動では読まれません（1.1.1 節のつまずきポイント）。古い記事や v3 のプロジェクトで `tailwind.config.ts` を見かけたときに読めるよう、設定項目ごとの意味を解説しておきます。"),
    ("それでは、v3 の設定ファイルを見ていきましょう。",
     "それでは、v3 の設定ファイルを見ていきましょう（1.1 節の v4 版と見比べると、同じ値が並んでいることが分かります）。"),
    # comments in components
    ("        // CSS変数（globals.css で定義した --color-primary-XX）を経由することで",
     "        // CSS変数（globals.css の :root で定義した --color-primary-XX）を経由することで"),
    ("             line-clamp-2                    : 2 行を超えたら省略「…」（globals.css で定義）",
     "             line-clamp-2                    : 2 行を超えたら省略「…」（Tailwind 標準のクラス）"),
    ("              container: tailwind.config の container 設定でレスポンシブ最大幅",
     "              container: globals.css の @utility container で中央寄せ＋レスポンシブな余白"),
    ("        {/* container: tailwind.config の container 設定",
     "        {/* container: globals.css の @utility container（中央寄せ＋余白）"),
    # 5.1
    ("`tailwind.config.ts` で `darkMode: \"class\"` を設定済みなので、`<html>` タグに `dark` クラスを付与することでダークモードが有効になります。",
     "1.1 節の `globals.css` で `@custom-variant dark (&:where(.dark, .dark *));` を設定済みなので、`<html>` タグに `dark` クラスを付与することでダークモードが有効になります（v3 では `tailwind.config.ts` の `darkMode: \"class\"` で同じことを指定します）。"),
    # まとめ
    ("| グローバルスタイル | CSS カスタムプロパティ、ベースレイヤー、コンポーネントレイヤー | `app/globals.css` |",
     "| グローバルスタイル | `@theme` のカラートークン・影・アニメーション、`.dark` の上書き、ベース／コンポーネントレイヤー、`@utility` | `app/globals.css` |"),
    ("| Tailwind 設定 | カスタムカラー、アニメーション、フォント、シャドウ | `tailwind.config.ts` |",
     "| Tailwind 設定 | ダークモードの方式（`@custom-variant`）、カスタムカラー、アニメーション、フォント、シャドウ | `app/globals.css`（v3 では `tailwind.config.ts`） |"),
    ("| Tailwind v4 の書き方 | `@import` / `@theme` / `@custom-variant` / `@utility` で設定を CSS に集約 | `app/globals.css`（1.1.1 節） |",
     "| v3 との違い | 設定ファイルの引っ越し、名前が変わったクラスの対応表 | 1.1.1 節 / 1.3 節（参考） |"),
]
for a, b in reps:
    n = s.count(a)
    assert n == 1, (n, a[:60])
    s = s.replace(a, b)

# mini sample: 解説2 and 解説4 extra bullets
a = "- `inline` を付けるのは「変数の中身（`var(--background)`）をそのままクラスに書き込んで」という意味です。切り替わる値を参照するときは `@theme inline` を使うのが公式のおすすめです。"
b = a + "\n- 1.1 節では別のやり方として、`@theme` に**ライトの値を直接**書き、`.dark` で同じ名前の変数を上書きしました。どちらも「変数の中身が入れ替わるとクラスの色が変わる」点は同じです。`create-next-app` の初期ファイルはこちらの `@theme inline` 方式なので、両方読めるようにしておきましょう。"
assert s.count(a) == 1
s = s.replace(a, b)

a = "- `@utility` で作ったクラスは普通のユーティリティと同じ扱いになるので、`hover:btn-primary` や `md:btn-primary` のように接頭辞も付けられます。"
b = a + "\n- 1.1 節のアプリでは、ボタンを `@layer components` に書きました。components の層のクラスは、あとから `p-0` などのユーティリティで**部分的に上書きできる「部品」**です（図9-9）。一方 `@utility` で作ったクラスは utilities の層に入ります。「部品（ボタン・カード）は `@layer components`、小さな道具（`glass` など）は `@utility`」と使い分けるのがおすすめです。"
assert s.count(a) == 1
s = s.replace(a, b)

# outline-none -> outline-hidden in code (v4 name), keep the rename-table row
before = s.count('outline-none')
s = s.replace('focus-visible:outline-none', 'focus-visible:outline-hidden')
s = s.replace('focus:outline-none', 'focus:outline-hidden')
print('outline-none before', before, 'after', s.count('outline-none'))

open(p, 'w', encoding='utf-8').write(s)
print('done')

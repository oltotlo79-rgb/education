##### 解説1: Tailwind を読み込み、ダークモードの合図を決める（`@import` / `@custom-variant`）

```css
@import "tailwindcss";

@custom-variant dark (&:where(.dark, .dark *));
```

- `@import "tailwindcss";` は Tailwind CSS 本体を読み込む1行です。ビルド時にこの1行が、ブラウザごとの違いをそろえるリセット CSS（base）と、`p-4`・`bg-red-500` などのクラス（utilities）に**展開**されます。
- **この1行が無いと Tailwind のクラスは一切効きません。** 必ずファイルの先頭に書きます。
- `@custom-variant dark (…)` は「`dark:` の付いたクラスを**いつ**効かせるか」を決める宣言です。`&:where(.dark, .dark *)` は「`dark` クラスが付いた要素自身か、その中にある要素」という意味で、`<html class="dark">` にすればページ全体が当てはまります。書き忘れると、v4 の `dark:` は OS の設定にしか反応しません（5.1 節のつまずきポイント）。

> **用語: ディレクティブ（directive）** … CSSやツールに対する「特別な指示」を表す命令文のこと。`@` から始まるものが多い（`@import`, `@theme`, `@layer`, `@apply` など）。

`@import "tailwindcss"` で読み込まれる CSS は、base（土台）・components（部品）・utilities（便利クラス）という**層（レイヤー）**に分けて並べられます。

{{LAYER}}

v3 では、この部分を `@tailwind base;` `@tailwind components;` `@tailwind utilities;` の3行で書き、ダークモードの方式は `tailwind.config.ts` の `darkMode: "class"` で指定していました（1.3 節）。古い記事でこの3行を見かけたら「v3 の書き方だな」と読み替えてください。

---

##### 解説2: 色に名前を付けてクラスにする（`@theme` / カラートークン）

```css
@theme {
  /* 一番薄いインディゴ（背景バッジ等に使う） */
  --color-primary-50: #eef2ff;
  /* ボタン背景でよく使う濃さ */
  --color-primary-600: #4f46e5;
  /* ページ全体の背景色（白）→ bg-background */
  --color-background: #ffffff;
}
```

- （上はコードの抜粋です）`@theme { … }` の中に書いた `--color-primary-600` のような **CSS変数（カスタムプロパティ）** は、Tailwind の**クラスとして使える**ようになります。名前が `--color-` で始まるので「色」と判断され、`bg-primary-600`（背景）・`text-primary-600`（文字）・`border-primary-600`（枠線）などが一度に使えるようになります。
- 同時に `:root`（ページの一番外側＝`<html>`）の CSS 変数としても出力されるので、`var(--color-primary-600)` と書けばページのどこからでも呼び出せます。本章のコンポーネントで使っている `bg-[var(--color-card)]` は、こちらの使い方です。
- 値の `#4f46e5` は色を表す16進数コードで、`#` の後ろ2桁ずつが「赤・緑・青」の強さを表します。色を直接書かず変数にしておくと、後で色を変えたいとき**1か所直すだけ**で全体に反映できます。
- 影（`--shadow-`）・角丸（`--radius-`）・フォント（`--font-`）・文字サイズ（`--text-`）も同じしくみで、**名前の前半で何のクラスになるかが決まります**（1.1.1 節の「名前空間」）。

> **用語: カラートークン（color tokens）／デザイントークン** … 「primary-600 はこの色」のように、デザインで使う値に意味のある名前を付けて一元管理したもの。チームでデザインの一貫性を保つための定番手法。

---

##### 解説3: ダークモードで色を入れ替える（`.dark`）

```css
.dark {
  /* ほぼ黒（slate-900）に */
  --color-background: #0f172a;
  /* 文字は逆に白系に */
  --color-foreground: #f8fafc;
  /* カード背景は背景より少し明るい暗色 */
  --color-card: #1e293b;
}
```

- `<html class="dark">` のように `dark` クラスが付いたときだけ、この中の**変数の値が上書き**されます。
- ポイントは「**新しいクラスを足すのではなく、同じ変数の中身だけを差し替える**」こと。`bg-background` などのクラスは中で `var(--color-background)` を使っているので、変数を黒に入れ替えるだけで画面全体がダークモードに変わります。
- 各コンポーネントで `dark:bg-...` を一つずつ書かなくて済むのが、CSS変数方式の最大の利点です。
- ただし影（`--shadow-sm` など）は、ここで上書きしても変わりません。v4 の `shadow-sm` などのクラスは、`@theme` の値がクラスの中に直接書き込まれるためです。ダークモードで影を変えたいときは `dark:shadow-lg` のように `dark:` を使います。

> **用語: ダークモード（dark mode）** … 背景を黒・濃灰にした暗い配色モード。夜間や暗所での目の負担を減らす目的で用意する。

---

##### 解説4: ベースレイヤーで土台を整える（`@layer base`）

```css
@layer base {
  body {
    @apply bg-background text-foreground;
    transition: background-color 0.3s ease, color 0.3s ease;
  }
}
```

- `@layer base`（レイヤー・ベース）は「**一番優先度の低い層**」にスタイルを置く指定です。低い層に置くと、後から個別の Tailwind クラスで簡単に上書きできます。
- `@apply`（アプライ）は「**CSSの中でTailwindクラスを使う**」ための命令です。`bg-background` は解説2で `@theme` に登録した `--color-background` を背景色にするクラスです。
- `transition: ... 0.3s ease` で、ダークモード切替時に背景と文字色が**0.3秒かけて滑らかに**変わるようにしています。

> **用語: レイヤー（layer）／@apply** … `@layer` はCSSの優先順位を整理するための「層」。`@apply` は長くなりがちなTailwindクラス指定をCSS側にまとめる書き方。

---

##### 解説5: ボタンの共通スタイルを作る（`@layer components` の `.btn`）

```css
@layer components {
  .btn,
  .btn-primary,
  .btn-secondary,
  .btn-outline,
  .btn-danger,
  .btn-ghost {
    @apply inline-flex items-center justify-center
           rounded-lg px-4 py-2
           text-sm font-medium
           transition-all duration-200 ease-in-out
           disabled:pointer-events-none disabled:opacity-50;
  }

  .btn-primary {
    @apply bg-primary-600 text-white
           hover:bg-primary-700;
  }
}
```

- （上はコードの抜粋です）`.btn, .btn-primary, …` のように**カンマで区切ってクラスを並べる**と、並べたすべてのクラスに同じスタイルが当たります。これが「全ボタン共通の土台」です。
- 中身は、`inline-flex items-center justify-center` で中身を中央寄せ、`rounded-lg` で角丸、`px-4 py-2` で内側余白、`transition-all duration-200` で変化を200msかけて滑らかにする指定です。
- `disabled:opacity-50` は「ボタンが無効化されたときは半透明にする」指定で、`disabled:` は状態に応じてスタイルを変える接頭辞です。
- `.btn-primary` には、土台に足りない**色だけ**（インディゴ背景＋白文字）を書いています。こうすると共通部分を何度も書かずに済みます。
- v3 では `.btn-primary { @apply btn …; }` のように**自作クラスを `@apply` で取り込む**書き方もできましたが、v4 の `@apply` で取り込めるのは Tailwind のクラス（と `@utility` で作ったクラス）だけです。そのため本章では「セレクタを並べる」方法を使っています。

> **用語: ホバー（hover）** … マウスカーソルを要素の上に乗せた状態。`hover:bg-primary-700` は「乗せたときだけ背景を濃くする」という指定。

{{STATES}}

---

##### 解説6: カードと入力欄の共通スタイル（`.card` / `.input`）

```css
.card,
.card-hover {
  @apply rounded-xl border border-border
         bg-card text-card-foreground
         shadow-sm;
  transition: box-shadow 0.2s ease, transform 0.2s ease;
}
.input {
  @apply w-full rounded-lg border border-border
         px-4 py-2.5 text-sm
         focus:border-primary-500 focus:outline-hidden focus:ring-2;
}
```

- `.card` は書籍カードなどの土台です。`border-border` は「枠線の色を `--color-border` に」、`bg-card` は「背景を `--color-card` に」という指定で、どれも解説2の変数経由なので**ダークモードでも自動で色が変わります**。
- `.input` は入力欄の共通スタイル。`w-full` で横幅いっぱい、`focus:outline-hidden` はブラウザ標準の輪郭線を消す指定（v3 の `outline-none` に当たる）、`focus:ring-2` は「クリックして入力中のとき、枠の周りに2pxの光るリングを出す」指定です。`focus:` は入力欄が選択された状態を表します。
- このように共通スタイルを `@layer components` にまとめておくと、HTML側では `<div class="card">` と書くだけで統一された見た目になります。

> **用語: フォーカス（focus）** … 入力欄やボタンが「いま操作対象として選ばれている」状態。キーボード操作のユーザーにとって、どこが選択中かを示すリング表示が重要になる。

---

##### 解説7: 自作ユーティリティを作る（`@utility`）

```css
@utility glass {
  @apply bg-white/80 backdrop-blur-md dark:bg-gray-900/80;
}
```

- `@utility 名前 { … }` は、Tailwind に無い便利クラスを**自作**する書き方です（名前の前に `.` は付けません）。作ったクラスは `p-4` などと同じユーティリティとして扱われ、`md:glass` のように接頭辞も付けられます。
- `glass` は「半透明の白＋向こう側をぼかす」で、すりガラスのように見せるクラスです。同じやり方で、`container`（中央寄せと画面幅に応じた余白）と `scrollbar-thin`（細いスクロールバー）も作っています。`@variant sm { … }` は「`sm:` の条件（640px 以上）のときだけ」という意味です。
- 長いタイトルを「…」で打ち切る `line-clamp-1`〜`line-clamp-3` は、Tailwind に**最初から入っている**ので自作は不要です（以前の版では自作していました）。Tailwind が作る CSS は次のとおりで、`-webkit-line-clamp` の数字が打ち切る行数です。

```css
.line-clamp-2 {
  overflow: hidden;
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 2;
}
```

> **用語: ユーティリティクラス（utility class）** … `p-4`（余白16px）のように「1つの機能だけを持つ小さなクラス」。これらを組み合わせてデザインするのがTailwindの考え方。

---

##### 解説8: 動きの素材を定義する（`--animate-*` と `@keyframes`）

```css
@theme {
  --animate-fade-in: fade-in 0.3s ease-out;

  @keyframes fade-in {
    from {
      /* 透明（見えない） */
      opacity: 0;
      /* 8px 下にずらした位置から始まる */
      transform: translateY(8px);
    }
    to {
      /* 不透明（見える） */
      opacity: 1;
      /* 元の位置へ */
      transform: translateY(0);
    }
  }
}
```

- `@keyframes`（キーフレーム）は「**アニメーションの始まり（`from`）と終わり（`to`）のコマ**」を定義します。
- この `fade-in` は「透明で8px下にいる状態」から「不透明で元の位置」へ変化します。つまり**少し下からふわっと現れる**動きです。
- `--animate-fade-in: fade-in 0.3s ease-out;` は「`fade-in` のコマを 0.3 秒、最後ゆっくり（ease-out）で再生する」という登録で、これで `animate-fade-in` というクラスが使えるようになります。`@keyframes` を `@theme` の中に書いておくと、そのアニメーションを実際に使ったときだけ CSS に出力されます。
- `opacity`（透明度：0で透明、1で不透明）と `transform: translateY(...)`（縦方向の移動）を組み合わせるのがフェードイン演出の定番です。

> **用語: キーフレーム（keyframe）** … アニメーションの途中経過を指定する「コマ」。`from`/`to` のほか `0%`/`50%`/`100%` のようにパーセントで複数指定もできる。

---


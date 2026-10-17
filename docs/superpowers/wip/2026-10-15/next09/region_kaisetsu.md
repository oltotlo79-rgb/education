##### 解説1: Tailwind を読み込む3行（`@tailwind`）

```css
/* base   : ブラウザ標準スタイルのリセット（normalize.css 的なもの）+ HTML要素のデフォルト */
@tailwind base;
/* components: .btn など、@layer components { ... } で定義したクラス */
@tailwind components;
/* utilities : bg-red-500, p-4 などのユーティリティクラス本体 */
@tailwind utilities;
```

- `@tailwind`（アットマーク・テイルウィンド）は Tailwind 専用の「命令文」です。ビルド時にこの1行が**数千行のCSS**に展開されます。
- `base` は「ブラウザごとに違う初期スタイルをそろえるリセット」、`components` は「`.btn` など自作のまとまったクラス」、`utilities` は「`p-4`・`bg-red-500` などの細かい1機能クラス」を入れる場所です。
- **この3行が無いと Tailwind のクラスは一切効きません。** 必ずファイルの先頭に書きます。

> **用語: ディレクティブ（directive）** … CSSやツールに対する「特別な指示」を表す命令文のこと。`@` から始まるものが多い（`@tailwind`, `@layer`, `@apply` など）。

3つの層は、**積み重なった棚**のような関係です。上の段ほど強く、同じ要素に別々の段からスタイルが当たってぶつかったときは、上の段のものが勝ちます。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 252" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-09" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="30" y="22" text-anchor="middle" font-size="12" font-weight="bold" fill="#475569">強い</text>
<line x1="30" y1="226" x2="30" y2="34" stroke="#475569" stroke-width="1.5" marker-end="url(#ar-09-09)"/>
<text x="30" y="244" text-anchor="middle" font-size="12" font-weight="bold" fill="#475569">弱い</text>
<rect x="60" y="30" width="330" height="58" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="76" y="54" font-size="14" font-weight="bold" fill="#1e40af">utilities（ユーティリティ）</text>
<text x="76" y="76" font-size="12" fill="#1e40af">p-4 / bg-red-500 / hover:… / 自作の便利クラス</text>
<rect x="60" y="99" width="330" height="58" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="76" y="123" font-size="14" font-weight="bold" fill="#c2410c">components（コンポーネント）</text>
<text x="76" y="145" font-size="12" fill="#c2410c">.btn / .card / .badge / .input</text>
<rect x="60" y="168" width="330" height="58" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="76" y="192" font-size="14" font-weight="bold" fill="#475569">base（ベース）</text>
<text x="76" y="214" font-size="12" fill="#475569">html / body / h1 / a の土台とリセット</text>
<rect x="420" y="30" width="290" height="196" rx="10" fill="#ffffff" stroke="#cbd5e1"/>
<text x="436" y="56" font-size="13" font-weight="bold" fill="#334155">例: ぶつかったら上の段が勝つ</text>
<text x="436" y="84" font-size="12" font-family="Consolas, monospace" fill="#1e293b">&lt;button class="btn p-0"&gt;</text>
<text x="436" y="112" font-size="12" fill="#c2410c">.btn の padding（components）</text>
<text x="452" y="130" font-size="12" fill="#c2410c">→ 上下 8px・左右 16px</text>
<text x="436" y="156" font-size="12" fill="#1e40af">p-0 の padding（utilities）</text>
<text x="452" y="174" font-size="12" fill="#1e40af">→ 0</text>
<text x="436" y="206" font-size="13" font-weight="bold" fill="#047857">結果: padding は 0（p-0 の勝ち）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図9-9: CSS の3つの層（レイヤー）と強さの順番。v4 でも theme → base → components → utilities の順は同じ</div>
</div>

この仕組みのおかげで、「`.btn` で共通の見た目を作っておき、特定のボタンだけ `p-0` や `w-full` などのユーティリティで**部分的に上書きする**」ことが安心してできます。

---

##### 解説2: 色に名前を付ける（CSS変数 / カラートークン）

```css
:root {
  /* 一番薄いインディゴ（背景バッジ等に使う） */
  --color-primary-50: #eef2ff;
  /* ... 中略 ... */
  /* ボタン背景でよく使う濃さ */
  --color-primary-600: #4f46e5;
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
  /* ほぼ黒（slate-900）に */
  --color-background: #0f172a;
  /* 文字は逆に白系に */
  --color-foreground: #f8fafc;
  /* カード背景は背景より少し明るい暗色 */
  --color-card: #1e293b;
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

完成した `.btn-primary`（本文のコード全体）は、操作の状態によって次の5通りの見た目に切り替わります。PDF や紙ではマウスを乗せられないので、それぞれの状態を並べて描きました。

<div style="max-width: 720px; margin: 18px auto; font-family: 'Yu Gothic UI', 'Yu Gothic', Meiryo, sans-serif;">
<div style="display: flex; gap: 10px; justify-content: center;">
<div style="flex: 1; border: 1px solid #e2e8f0; border-radius: 10px; padding: 14px 6px 10px; text-align: center; background: #f8fafc;"><span style="display: inline-block; padding: 8px 16px; border-radius: 8px; background: #4f46e5; color: #ffffff; font-size: 14px;">保存</span><div style="font-size: 13px; font-weight: bold; color: #334155; margin-top: 12px;">通常</div><div style="font-family: Consolas, monospace; font-size: 12px; color: #64748b; word-break: break-all;">bg-primary-600</div></div>
<div style="flex: 1; border: 1px solid #e2e8f0; border-radius: 10px; padding: 14px 6px 10px; text-align: center; background: #f8fafc;"><span style="display: inline-block; padding: 8px 16px; border-radius: 8px; background: #4338ca; color: #ffffff; font-size: 14px;">保存</span><div style="font-size: 13px; font-weight: bold; color: #334155; margin-top: 12px;">マウスを乗せた</div><div style="font-family: Consolas, monospace; font-size: 12px; color: #64748b; word-break: break-all;">hover:bg-primary-700</div></div>
<div style="flex: 1; border: 1px solid #e2e8f0; border-radius: 10px; padding: 14px 6px 10px; text-align: center; background: #f8fafc;"><span style="display: inline-block; padding: 8px 16px; border-radius: 8px; background: #3730a3; color: #ffffff; font-size: 14px;">保存</span><div style="font-size: 13px; font-weight: bold; color: #334155; margin-top: 12px;">押している瞬間</div><div style="font-family: Consolas, monospace; font-size: 12px; color: #64748b; word-break: break-all;">active:bg-primary-800</div></div>
<div style="flex: 1; border: 1px solid #e2e8f0; border-radius: 10px; padding: 14px 6px 10px; text-align: center; background: #f8fafc;"><span style="display: inline-block; padding: 8px 16px; border-radius: 8px; background: #4f46e5; color: #ffffff; font-size: 14px; box-shadow: 0 0 0 2px #ffffff, 0 0 0 4px #6366f1;">保存</span><div style="font-size: 13px; font-weight: bold; color: #334155; margin-top: 12px;">Tab キーで選択</div><div style="font-family: Consolas, monospace; font-size: 12px; color: #64748b; word-break: break-all;">focus-visible:ring-2</div></div>
<div style="flex: 1; border: 1px solid #e2e8f0; border-radius: 10px; padding: 14px 6px 10px; text-align: center; background: #f8fafc;"><span style="display: inline-block; padding: 8px 16px; border-radius: 8px; background: #4f46e5; color: #ffffff; font-size: 14px; opacity: 0.5;">保存</span><div style="font-size: 13px; font-weight: bold; color: #334155; margin-top: 12px;">押せない</div><div style="font-family: Consolas, monospace; font-size: 12px; color: #64748b; word-break: break-all;">disabled:opacity-50</div></div>
</div>
<div style="font-size: 12px; color: #64748b; margin-top: 6px; text-align: center;">図9-10: .btn-primary の状態ごとの見た目（色が少しずつ濃くなる・輪が出る・薄くなる、の違いに注目）</div>
</div>

「押せる」「今押している」「キーボードでここを選んでいる」「今は押せない」が見た目だけで伝わることで、ユーザーは迷わずに操作できます。特に **Tab キーで選択したときの輪（フォーカスリング）** は、マウスを使わない人にとって「今どこにいるか」を知る唯一の手がかりなので、消さないようにしましょう。

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
    /* 2 行で打ち切る */
    -webkit-line-clamp: 2;
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
```

- `@keyframes`（キーフレーム）は「**アニメーションの始まり（`from`）と終わり（`to`）のコマ**」を定義します。
- この `fadeIn` は「透明で8px下にいる状態」から「不透明で元の位置」へ変化します。つまり**少し下からふわっと現れる**動きです。
- `opacity`（透明度：0で透明、1で不透明）と `transform: translateY(...)`（縦方向の移動）を組み合わせるのがフェードイン演出の定番です。

> **用語: キーフレーム（keyframe）** … アニメーションの途中経過を指定する「コマ」。`from`/`to` のほか `0%`/`50%`/`100%` のようにパーセントで複数指定もできる。

---


@@START: ### 5.1 環境変数の取得
@@END: ### 5.3 lib/supabase.ts の作成
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

FIG:figE

ポイントは②です。ブラウザで動くコードには `process.env` という入れ物がそもそもありません。そこで Next.js は、ブラウザ用の JavaScript を作るときに `process.env.NEXT_PUBLIC_SUPABASE_URL` という部分を見つけて、**値そのもの（`"https://....supabase.co"` という文字列）に書き換えて埋め込みます**。実際に開発サーバーが作ったブラウザ用のファイルを調べると、`NEXT_PUBLIC_` 付きの値は文字列として埋め込まれ、付いていない変数は `process.env.SECRET_TOKEN` のまま（ブラウザでは中身がないので `undefined`）になっていました。

> **ENV ファイルの種類と読み込み順（Next.js）:**
> Next.js では複数の `.env` 系ファイルを使い分けられます。同じ変数が複数のファイルにある場合、上のものが優先されます。
>
> | ファイル | いつ読み込まれるか | Gitに入れる？ |
> |---------|----------------|-------------|
> | `.env.local` | 全環境（開発・本番ともに）。ただしテスト環境（`NODE_ENV=test`）では読まれない | **入れない**（.gitignoreで除外） |
> | `.env.development` | 開発時（`npm run dev`）のみ | 既定では除外される（秘密を含まないなら共有してもよい） |
> | `.env.production` | 本番時（`npm run build` / `npm run start`）のみ | 既定では除外される（秘密を含まないなら共有してもよい） |
> | `.env` | すべての環境（デフォルト値） | 既定では除外される（秘密を含まないなら共有してもよい） |
>
> 読み込み優先順位（高 → 低）: `.env.local` > `.env.development` / `.env.production` > `.env`
>
> **覚え方:** 「`.local` が付くファイルは個人専用＝Gitに入れない＝秘密情報を入れてOK」。`.env.local` だけは絶対にコミットしない、と覚えれば事故を防げます。

> **`.env.local` がGitにアップロードされない理由:**
> `create-next-app` が自動生成した `.gitignore` ファイルには `.env*` が含まれています（`.env` で始まるファイルはすべて除外、という意味）。そのため、`.env.local` は Git にコミットされず、API キーが公開リポジトリに漏洩する心配はありません。

**必ず `xxxxxxxxxxxxxxxx` の部分を、自分のプロジェクトの値に置き換えてください。**

#### 5.2.1 サンプル: `.env.local` が読めているかを確かめるスクリプト

> **▼ このコードがやること（先に日本語で）:** アプリを起動する前に、`.env.local` に必要な2つの値が正しい名前で書かれているかを確かめる小さなスクリプトです。Node.js の `--env-file` オプションで `.env.local` を読み込ませてから実行し、値があれば `OK`、無ければ `NG` と表示します。秘密を画面に出しすぎないよう、値は先頭20文字だけ見せます。

```typescript
// scripts/check-env.ts
// .env.local に必要な2つの値が書かれているかを、アプリを起動せずに確かめるスクリプト

// 確かめたい環境変数の名前を配列にまとめる
const names = ["NEXT_PUBLIC_SUPABASE_URL", "NEXT_PUBLIC_SUPABASE_ANON_KEY"];

// 1つずつ取り出して、値があるかどうかを表示する
for (const name of names) {
  // process.env[name] で、その名前の環境変数の値を読む（無ければ undefined）
  const value = process.env[name];
  if (value) {
    // 秘密を画面に出しすぎないよう、先頭20文字だけ見せる
    console.log(`OK  ${name} = ${value.slice(0, 20)}...`);
  } else {
    console.log(`NG  ${name} が見つかりません`);
  }
}
```

`book-management/` の中に `scripts` フォルダを作ってこのファイルを保存し、プロジェクトのフォルダで次のように実行します。

```bash
# --env-file=.env.local: このファイルの中身を環境変数として読み込んでから実行する
node --env-file=.env.local scripts/check-env.ts
```

**▼ 実行結果**（説明用のダミーの値で実行したもの）

```text
OK  NEXT_PUBLIC_SUPABASE_URL = https://abcdefghijkl...
OK  NEXT_PUBLIC_SUPABASE_ANON_KEY = sb_publishable_dummy...
```

ファイル名を打ち間違えると（`.env.locl`）、スクリプトが動く前に Node.js が止まります。

```bash
# わざとファイル名を打ち間違える
node --env-file=.env.locl scripts/check-env.ts
```

**▼ 実行結果**

```text
C:\Program Files\nodejs\node.exe: .env.locl: not found
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 名前の一覧を配列にして、順番に調べる

```typescript
const names = ["NEXT_PUBLIC_SUPABASE_URL", "NEXT_PUBLIC_SUPABASE_ANON_KEY"];

for (const name of names) {
  const value = process.env[name];
```

- 調べたい名前を配列（`[ ]` で囲んだ並び）にまとめ、`for (const name of names)` で1つずつ取り出しています。
- `process.env[name]` は、`process.env.NEXT_PUBLIC_SUPABASE_URL` と書くのと同じ意味です。名前を変数で指定したいときは `[ ]` を使います。

##### 解説2: 値があれば OK、無ければ NG

```typescript
if (value) {
  console.log(`OK  ${name} = ${value.slice(0, 20)}...`);
} else {
  console.log(`NG  ${name} が見つかりません`);
}
```

- 値が無いとき `process.env[name]` は `undefined` になり、`if (value)` は偽になります。
- `value.slice(0, 20)` は「先頭から20文字だけ切り出す」という意味です。キーを丸ごと画面に出さないための工夫です。

##### 解説3: `--env-file` は Node.js の機能

```bash
node --env-file=.env.local scripts/check-env.ts
```

- `--env-file=ファイル名` を付けると、Node.js がそのファイルを読んで `process.env` に入れてから実行してくれます。Next.js が `.env.local` を読む仕組みとは別ですが、「書いた名前と値がちゃんと読めるか」を確かめるには十分です。
- `.ts` ファイルをそのまま実行できるのは、第2章で学んだとおり Node.js が型の部分を消して動かしてくれるからです（Node.js 24 以降なら追加の設定なしで動きます）。

> **用語:** 「スクリプト」とは、手作業の確認や準備を自動でやってくれる小さなプログラムのことです。アプリ本体とは別に `scripts/` のようなフォルダにまとめておくのが一般的です。

#### 5.2.2 つまずきポイント: 変数名がコードとそろっていない

> **つまずきポイント: Supabase の画面に出ている変数名をそのまま使ってしまう**
>
> Supabase の「Connect」画面には、Next.js 用の設定例として `NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY` という名前が表示されることがあります。これをそのまま貼ると、本書のコードが読む `NEXT_PUBLIC_SUPABASE_ANON_KEY` と名前が違うため、キーが「無い」ことになります。

NG（変数名がコードと違う）:

```bash
# book-management/.env.local
NEXT_PUBLIC_SUPABASE_URL=https://xxxxxxxxxxxxxxxx.supabase.co
# 名前が PUBLISHABLE_KEY になっている（コードは ANON_KEY を読む）
NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY=sb_publishable_xxxxxxxxxxxxxxxxxxxxxxxx
```

この状態で 5.2.1 のスクリプトを実行すると、2つ目が NG になります。

**▼ 実行結果**

```text
OK  NEXT_PUBLIC_SUPABASE_URL = https://abcdefghijkl...
NG  NEXT_PUBLIC_SUPABASE_ANON_KEY が見つかりません
```

OK（値はそのまま、名前だけ本書のコードに合わせる）:

```bash
# book-management/.env.local
NEXT_PUBLIC_SUPABASE_URL=https://xxxxxxxxxxxxxxxx.supabase.co
# 名前を ANON_KEY にする（値は Publishable key のまま）
NEXT_PUBLIC_SUPABASE_ANON_KEY=sb_publishable_xxxxxxxxxxxxxxxxxxxxxxxx
```

#### 5.2.3 つまずきポイント: `.env.local` の名前や場所が違う

> **つまずきポイント: `.env.local.txt` になっている／`app/` の中に置いている**
>
> Windows のメモ帳で保存すると、`.env.local.txt` のように `.txt` が勝手に付くことがあります（エクスプローラーの設定によっては `.txt` が見えません）。また、`app/` などのフォルダの中に置いてしまうと Next.js は読みません。`.env.local` は **`package.json` と同じ場所**に、**拡張子なしの名前**で置きます。

NG（名前や場所が違う）:

```text
book-management/.env.local.txt      ← .txt が付いている
book-management/app/.env.local      ← app/ の中にある
```

OK（`package.json` と同じ場所に、正しい名前で置く）:

```text
book-management/.env.local
book-management/package.json
```

見分け方は簡単です。`npm run dev` の起動時の表示に **`- Environments: .env.local`** の行があるかを見てください。この行が無ければ、Next.js は `.env.local` を見つけられていません。次は `.env.local.txt` しか無い状態で起動し、Supabase を使うページ（第7章で作るトップページと同じ形のもの）を開いたときの実際の表示です。

**▼ 実行結果**

```text
▲ Next.js 16.4.1 (Turbopack)
- Local:         http://localhost:3000
- Network:       http://192.168.x.x:3000
✓ Ready in 677ms
✓ Running next.config.ts took 40ms

○ Compiling / ...
⨯ Error: 環境変数 NEXT_PUBLIC_SUPABASE_URL / NEXT_PUBLIC_SUPABASE_ANON_KEY が設定されていません。.env.local ファイルを確認してください。
    at createClient (src\lib\supabase\server.ts:18:11)
    at Page (src\app\page.tsx:4:38)
  16 |   // どちらかが空なら、原因と直し方が分かるメッセージでわざと止める
  17 |   if (!supabaseUrl || !supabaseKey) {
> 18 |     throw new Error(
     |           ^
  19 |       "環境変数 NEXT_PUBLIC_SUPABASE_URL / NEXT_PUBLIC_SUPABASE_ANON_KEY が設定されていません。" +
  20 |         ".env.local ファイルを確認してください。"
  21 |     ); {
  digest: '366980926'
}
 GET / 500 in 725ms (next.js: 524ms, application-code: 202ms)
```

（ポート番号は説明に合わせて 3000 にしています。）`- Environments: .env.local` の行が無く、次の 5.3 で作る `server.ts` のエラーチェックが「設定されていません」と知らせています。`GET / 500` の 500 は「サーバー側でエラーが起きた」という意味です。

#### 5.2.4 つまずきポイント: `.env.local` を直したのに反映されない

> **つまずきポイント: 直したつもりが、開発サーバーに伝わっていない**
>
> Next.js 16 の開発サーバーは、`.env.local` を**保存し直すと自動で読み直して**くれます。そのときターミナルに `Reload env: .env.local` と表示されます。

上の 5.2.3 の状態のまま、エディタで `.env.local` を正しい名前で保存したときの実際の表示です。

**▼ 実行結果**

```text
  Reload env: .env.local
✓ Compiled in 22ms
 GET / 200 in 7.6s (next.js: 54ms, application-code: 7.6s)
```

`GET / 200`（200 は「成功」）に変わりました。ただし、**ファイル名を変更しただけ**（`.env.local.txt` → `.env.local`）のときは、この自動読み直しが起きないことがありました。筆者の実験でも、名前を変えただけでは `500` のままで、もう一度エディタで保存したところで `Reload env` が出て直りました。

NG（`Reload env` が出ていないのに、ブラウザだけ再読み込みし続ける）:

```text
（ターミナルに Reload env: .env.local が出ていない）
```

OK（確実な直し方: 開発サーバーを起動し直す）:

```bash
# ターミナルで Ctrl+C を押して開発サーバーを止めてから、もう一度起動する
npm run dev
```

起動し直したら、`- Environments: .env.local` の行が出ていることを確認してください。「迷ったら再起動」と覚えておけば、`.env.local` まわりの「直したのに直らない」はほぼ防げます。


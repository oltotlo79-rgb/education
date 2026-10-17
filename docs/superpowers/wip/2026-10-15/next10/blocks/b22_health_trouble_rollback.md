### 2.9.1 ヘルスチェック用の API で「どの版が、どの設定で動いているか」を確かめる

本番で何かおかしいとき、最初に知りたいのは「いま動いているのは、どのコミットから作られた版か」「環境変数はちゃんと入っているか」です。これを1回のアクセスで確かめられる、小さな API（ヘルスチェック：健康診断）を作っておきましょう。

> **▼ このコードがやること（先に日本語で）:** `/api/health` という URL を開くと、「アプリが動いているか」「本番かプレビューか」「どのコミットの版か」「Supabase の環境変数が登録されているか（true / false だけ）」を JSON で返します。値そのものは返さないので、本番に置いたままでも安全です。

```ts
// src/app/api/health/route.ts
// /api/health にアクセスすると「いま動いているアプリの状態」を JSON で返す
export function GET() {
  return Response.json({
    // ここまで処理が来れば、アプリ自体は動いている
    status: "ok",
    // Vercel 上なら production / preview、手元なら local
    environment: process.env.VERCEL_ENV ?? "local",
    // どのコミットから作られたか（先頭7文字）。手元なら local
    commit: process.env.VERCEL_GIT_COMMIT_SHA?.slice(0, 7) ?? "local",
    // 環境変数が「登録されているか」だけを true / false で返す（値そのものは返さない）
    supabaseUrlConfigured: Boolean(process.env.NEXT_PUBLIC_SUPABASE_URL),
    supabaseKeyConfigured: Boolean(process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY),
    // このレスポンスを作った時刻
    checkedAt: new Date().toISOString(),
  });
}
```

```powershell
# ビルドして本番モードで起動したあと、別の PowerShell で実行する
curl.exe -s http://localhost:3001/api/health
```

**▼ 実行結果**（手元。`.env.local` に2つの値が入っている場合）

```text
{"status":"ok","environment":"local","commit":"local","supabaseUrlConfigured":true,"supabaseKeyConfigured":true,"checkedAt":"2026-10-17T02:30:08.709Z"}
```

Vercel にデプロイしたあとは、ブラウザで `https://book-management-app.vercel.app/api/health` を開きます。たとえば次のように、`environment` が `production` に、`commit` がそのデプロイのコミット番号に変わります（値は一例です）。

```text
{"status":"ok","environment":"production","commit":"a0b5a74","supabaseUrlConfigured":true,"supabaseKeyConfigured":true,"checkedAt":"..."}
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `route.ts` と `GET` 関数

```ts
export function GET() {
  return Response.json({ ... });
}
```

- `app/api/health/route.ts` という場所に置いたファイルは、画面（ページ）ではなく **API**（データだけを返す窓口）になります。URL は `/api/health` です。
- `GET` という名前で関数を export すると、ブラウザでその URL を開いたとき（GET リクエスト）に実行されます。`Response.json(...)` は「この中身を JSON 形式で返す」という意味です。

> **用語:** JSON（ジェイソン）… `{"名前": 値}` の形でデータを書く、Web でいちばんよく使われるデータの書き方。

##### 解説2: Vercel が自動で入れてくれる環境変数

```ts
environment: process.env.VERCEL_ENV ?? "local",
commit: process.env.VERCEL_GIT_COMMIT_SHA?.slice(0, 7) ?? "local",
```

- `VERCEL_ENV` と `VERCEL_GIT_COMMIT_SHA` は、自分で登録しなくても Vercel が自動で用意する「システム環境変数」です。`VERCEL_ENV` は `production` / `preview` / `development` のどれか、`VERCEL_GIT_COMMIT_SHA` はデプロイのもとになったコミットの番号（40文字）が入ります。Environment Variables 画面の「Enable access to System Environment Variables」にチェックが入っていると使えます（通常は最初から入っています）。
- 手元にはこれらが無いので、`??`（左が無ければ右を使う）で `"local"` を返しています。`?.slice(0, 7)` は「値があれば先頭7文字だけ取り出す」という意味で、`git log --oneline` に出る短い番号と同じ長さです。

##### 解説3: 値ではなく「あるかないか」だけを返す

```ts
supabaseUrlConfigured: Boolean(process.env.NEXT_PUBLIC_SUPABASE_URL),
```

- `Boolean(...)` は、値が入っていれば `true`、無ければ `false` に変換します。誰でも開ける URL なので、**値そのものは返さない**のがポイントです。
- これで、「本番で `false` になっている → Vercel に登録し忘れている」とすぐに分かります。

### 2.9.2 本番だけ動かないときの切り分け

「手元では動くのに、本番では動かない」ときは、やみくもに直さず、**どのログを見ればよいか**を先に決めます。次の3つの質問で、見る場所が決まります。

{{FIG:f13}}

いちばん多い「環境変数の登録し忘れ」を、手元でわざと再現してみましょう。`.env.local` を一時的に別の名前に変え（`prebuild` のチェックも外した状態で）、ビルドして本番モードで動かします。

```powershell
# .env.local が無い状態でビルドする（この教材のアプリはビルド自体は成功してしまう）
npm run build
npm run start -- -p 3001
# 別の PowerShell で、トップページの HTTP ステータスコードだけを表示する
curl.exe -s -o NUL -w "%{http_code}" http://localhost:3001/
# ヘルスチェックも確かめる
curl.exe -s http://localhost:3001/api/health
```

**▼ 実行結果**（`curl.exe` の結果）

```text
PS> curl.exe -s -o NUL -w "%{http_code}" http://localhost:3001/
500
PS> curl.exe -s http://localhost:3001/api/health
{"status":"ok","environment":"local","commit":"local","supabaseUrlConfigured":false,"supabaseKeyConfigured":false,"checkedAt":"2026-10-17T02:37:18.617Z"}
```

**▼ 実行結果**（`npm run start` を動かしているターミナル。ファイルの場所は一部を伏せています）

```text
✓ Ready in 287ms
✓ Running next.config.ts took 75ms
⨯ Error: Your project's URL and Key are required to create a Supabase client!

Check your Supabase project's API settings to find these values

https://supabase.com/dashboard/project/_/settings/api
    at <unknown> (https://supabase.com/dashboard/project/_/settings/api)
    at <unknown> (C:\...\book-management\.next\server\chunks\ssr\_0ayfz82odguq8._.js:352:25)
    at du (C:\...\book-management\.next\server\chunks\ssr\_0ayfz82odguq8._.js:356:6060)
    at async f (C:\...\book-management\.next\server\chunks\ssr\_06ce-r9tf-zzn._.js:1:2218) {
  digest: '2080180400'
}
```

- `500` は「サーバーの中でエラーが起きた」という HTTP のステータスコードです（正常なら `200`）。
- ヘルスチェックは動いていて、`supabaseUrlConfigured` と `supabaseKeyConfigured` が `false` です。原因が「環境変数が無いこと」だと一目で分かります。
- ターミナル（Vercel なら「Logs」）には、エラー文と `digest`（ダイジェスト：エラーの識別番号）が出ています。

このとき、利用者のブラウザには次のような画面が表示されます（Next.js 16.4.1 の実際のエラー画面です。パソコンの設定によっては白い背景で表示されます）。画面の一番下の番号と、ログの `digest` が同じ番号になっているのがポイントです。

{{FIG:f14}}

> **ヒント:** 本番のエラー画面には、セキュリティのためにエラーの詳しい中身は表示されません。利用者から「ERROR 2080180400 と出た」と教えてもらえれば、Vercel の「Logs」でその番号を検索して、原因のログにたどり着けます。

もう1つ、「本番だけ動かない」原因としてよくあるのが、URL の書き方です。

> **つまずきポイント:** コードの中に `http://localhost:3000` と書いてしまうと、本番では動きません。`localhost` は「**このコンピュータ自身**」という意味なので、本番のサーバーや利用者のスマホから見ると、あなたのPCではなく自分自身を指してしまうからです。

**NG: 自分のPCの住所を書いている**

```ts
// NG: 本番では「利用者自身のコンピュータ」を見に行ってしまう
const res = await fetch("http://localhost:3000/api/health");
```

**OK: ブラウザ側のコードなら、`/` から始まる書き方にする**

```ts
// OK: 今開いているサイトと同じ住所（ドメイン）に問い合わせる
const res = await fetch("/api/health");
```

### 2.10 ロールバック（前のバージョンに戻す）

どれだけ気をつけても、本番にバグを出してしまうことはあります。そんなときに慌てて直そうとすると、焦りから別のバグを生みがちです。まずは**正常に動いていた前のバージョンに戻して**利用者への影響を止め、落ち着いてから原因を直す——これがロールバックの考え方です。

Vercel のデプロイは、新しく作っても古いものが消えずに残っています。本番 URL は「どのデプロイを表示するか」を指しているだけなので、**指す先を前のデプロイに付け替えれば一瞬で戻せます**。これを **Instant Rollback（インスタント・ロールバック）**と呼びます。

{{FIG:f15}}

#### Instant Rollback の手順（ダッシュボード）

1. Vercel ダッシュボードでプロジェクトを開く
2. 概要（Overview）ページの「Production Deployment」の欄にある **「Instant Rollback」** を押す（サイドバーの「Deployments」で、戻したいデプロイの「⋯」から選ぶこともできます）
3. 現在の本番と、戻せるデプロイが表示されるので、戻したいデプロイを選んで **「Continue」**
4. 対象のドメインなどの確認内容を読んで **「Confirm Rollback」**
5. すぐに本番 URL が前のバージョンに切り替わる

- 無料の **Hobby プラン**では、戻せるのは「**直前の**本番デプロイ」だけです（Pro プラン以上なら、過去に本番だった好きなデプロイに戻せます）。
- ロールバックはビルドし直さないので、**環境変数は戻した先のデプロイを作ったときの値のまま**です。設定画面で値を変えていても反映されません。
- データベース（Supabase）の中身は戻りません。戻るのはアプリのプログラムだけです。

> **注意:** ロールバックすると、Vercel は「本番ドメインの自動の付け替え」を止めます。そのため、**ロールバックのあとに `main` へ push しても、自動では本番に出ません**。バグを直した版が Ready になったら、Production Deployment の欄に出る **「Undo Rollback」** を押して、本番にしたいデプロイを選んで「Confirm」します（Deployments の「⋯」→「Promote（本番に昇格させる）」でも同じです）。これで、push すると自動で本番に出る、いつもの状態に戻ります。

#### もう1つの戻し方: `git revert` で「取り消しのコミット」を作る

ダッシュボードを使わず、Git の履歴そのものを戻す方法もあります。`git revert` は、指定したコミットの変更を**打ち消す新しいコミット**を作るコマンドです。それを push すれば、Vercel が普通にビルドしてデプロイします（ビルドする分、Instant Rollback より時間はかかります）。

> **▼ このコードがやること（先に日本語で）:** 直前のコミット（バグを入れてしまったコミット）を取り消すコミットを作り、履歴を確認します。過去のコミットを消すのではなく「取り消しました」という記録を足すので、GitHub にすでに push した履歴でも安全に使えます。

```powershell
# 直近3件のコミットを1行ずつ表示する
git log --oneline -3
# 直前のコミット（HEAD）を打ち消すコミットを作る（--no-edit はメッセージを自動で付ける）
git revert --no-edit HEAD
# もう一度、履歴を確かめる
git log --oneline -3
```

**▼ 実行結果**（実験用のリポジトリでの例）

```text
PS> git log --oneline -3
649d244 トップページの見出しを変更（バグ入り）
f8386de 書籍詳細にメタデータを追加
8cb7ccb health + check-env
PS> git revert --no-edit HEAD
[main a0b5a74] Revert "トップページの見出しを変更（バグ入り）"
 Date: Sat Oct 17 11:31:49 2026 +0900
 1 file changed, 1 insertion(+), 1 deletion(-)
PS> git log --oneline -3
a0b5a74 Revert "トップページの見出しを変更（バグ入り）"
649d244 トップページの見出しを変更（バグ入り）
f8386de 書籍詳細にメタデータを追加
```

- `Revert "…"` という新しいコミット（`a0b5a74`）ができました。バグ入りのコミット（`649d244`）は履歴に残ったままですが、その変更内容はちょうど打ち消されています。
- あとは `git push` すれば、取り消し済みの版が Production デプロイとして公開されます。

| 戻し方 | 速さ | 向いている場面 |
|---|---|---|
| Instant Rollback | 一瞬（ビルドしない） | 今すぐ被害を止めたいとき。まずはこちら |
| `git revert` → push | 1〜3分（ビルドする） | Git の履歴も「正しい状態」にそろえたいとき |

> **ヒント:** ターミナル派の人は、Vercel CLI の `vercel rollback`（ロールバック）と `vercel promote`（本番に昇格）でも同じ操作ができます。

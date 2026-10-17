#### 実験1: `NEXT_PUBLIC_` の値は本当に JS に書き込まれるのか

「ブラウザに送られる」「JS に埋め込まれる」と言われても、目で見ないと実感がわきません。小さな実験で確かめてみましょう。書籍管理アプリのプロジェクトで試してかまいません（実験が終わったら、作ったファイルと追加した2行は削除してください）。

> **▼ このコードがやること（先に日本語で）:** 「公開してよい値」と「秘密の値」を1つずつ `.env.local` に書き足し、その2つを画面に表示しようとする小さな部品とページを作ります。ビルドしたあと、ブラウザに配られる JS ファイルの中を検索して、どちらの値が入っているかを確かめます。

```bash
# .env.local の末尾に、実験用の2行を書き足す（本物の秘密は書かないこと）
NEXT_PUBLIC_APP_NAME=MyBookShelf-PUBLIC-123
SECRET_MESSAGE=TopSecret-XYZ-789
```

```tsx
// src/components/EnvViewer.tsx（実験用の部品）
"use client";

export default function EnvViewer() {
  // (1) NEXT_PUBLIC_ 付き: ビルド時に値が埋め込まれる
  const publicName = process.env.NEXT_PUBLIC_APP_NAME;
  // (2) NEXT_PUBLIC_ なし: ブラウザには届かない
  const secret = process.env.SECRET_MESSAGE;

  return (
    <ul>
      <li>公開してよい値: {publicName ?? "(undefined)"}</li>
      <li>秘密の値: {secret ?? "(undefined)"}</li>
    </ul>
  );
}
```

```tsx
// src/app/env-check/page.tsx（実験用のページ。/env-check で開ける）
import EnvViewer from "@/components/EnvViewer";

export default function EnvCheckPage() {
  return (
    <main className="p-6">
      <h1>環境変数チェック</h1>
      <EnvViewer />
    </main>
  );
}
```

```powershell
# 1) ビルドする
npm run build
# 2) ブラウザに配られる JS（.next\static\chunks）の中から、公開してよい値を探す
Select-String -Path .next\static\chunks\*.js -Pattern "MyBookShelf-PUBLIC-123" -List | Select-Object Filename
# 3) 同じように、秘密の値を探す
Select-String -Path .next\static\chunks\*.js -Pattern "TopSecret-XYZ-789" -List | Select-Object Filename
```

**▼ 実行結果**（Windows の PowerShell。ファイル名の英数字は環境ごとに変わります）

```text
PS> Select-String -Path .next\static\chunks\*.js -Pattern "MyBookShelf-PUBLIC-123" -List | Select-Object Filename

Filename
--------
0vu48qaqft2br.js

PS> Select-String -Path .next\static\chunks\*.js -Pattern "TopSecret-XYZ-789" -List | Select-Object Filename
PS>
```

公開してよい値は見つかり、秘密の値は何も表示されませんでした（＝どのファイルにも入っていない）。

> **ヒント:** Mac のターミナルでは `grep -rl "MyBookShelf-PUBLIC-123" .next/static` で同じことを確かめられます（見つかったファイルの場所が表示されます）。

#### ▼ コードを1つずつ分解して解説

##### 解説1: Client Component の中で環境変数を読む

```tsx
const publicName = process.env.NEXT_PUBLIC_APP_NAME;
const secret = process.env.SECRET_MESSAGE;
```

- この部品は `"use client"` 付きの Client Component（ブラウザでも動く部品）なので、ここに書いた `process.env.〜` は、ビルドのときに「ブラウザ用の JS」へ変換されます。
- `NEXT_PUBLIC_` 付きの方は、ビルドの瞬間に `"MyBookShelf-PUBLIC-123"` という**値そのもの**に書き換えられます。付いていない方は書き換えられず、ブラウザでは `undefined`（値が無い）になります。

##### 解説2: `Select-String` で JS ファイルの中を検索する

```powershell
Select-String -Path .next\static\chunks\*.js -Pattern "MyBookShelf-PUBLIC-123" -List | Select-Object Filename
```

- `Select-String` は PowerShell の「文字列を探す」コマンドです。`-Path` で探す場所（`.next\static\chunks` の中の `.js` ファイル全部）を、`-Pattern` で探す文字列を指定します。
- `-List` は「1ファイルにつき最初の1件だけ」、`| Select-Object Filename` は「見つかったファイル名だけを表示する」という意味です。圧縮された JS は1行がとても長いので、中身をそのまま表示すると画面が埋まってしまいます。

> **用語:** `.next\static\chunks` … ビルドで作られた「ブラウザに配る JS」が置かれるフォルダ。ここにあるファイルは、本番ではサイトを開いた人なら誰でもダウンロードできます。

##### 解説3: 結果の読み方

- 公開してよい値は `0vu48qaqft2br.js` の中から見つかりました。つまり、**本番サイトを開いた人は誰でも、この値を読める**ということです。Supabase の URL や公開用キーが `NEXT_PUBLIC_` 付きで問題ないのは、「誰に見られても困らない値」だからです。
- 秘密の値はどこからも見つかりませんでした。`NEXT_PUBLIC_` を付けなければ、ブラウザ用の JS には入りません。

{{FIG:f05}}

> **つまずきポイント:** 「ブラウザ用の JS に入らないなら、Client Component で秘密の値を読んでも安全」……ではありません。上の実験のページを本番モードで動かし、サーバーから届く HTML を調べると、秘密の値がそのまま入っています。

```powershell
# 本番モードで起動する（-p 3001 はポート番号の指定。1.2.1 参照）
npm run start -- -p 3001
# 別の PowerShell を開き、サーバーが返す HTML の中から秘密の値を探す
(curl.exe -s http://localhost:3001/env-check | Select-String "TopSecret-XYZ-789").Matches.Value
```

**▼ 実行結果**

```text
PS> (curl.exe -s http://localhost:3001/env-check | Select-String "TopSecret-XYZ-789").Matches.Value
TopSecret-XYZ-789
```

ブラウザの画面には次のように表示されているのに、です。

```text
公開してよい値: MyBookShelf-PUBLIC-123
秘密の値: (undefined)
```

Next.js は Client Component も、まずサーバー側（このページではビルドのとき）で HTML を作ってから送ります。サーバーでは秘密の環境変数も読めるので、そのときの値が HTML に書き込まれてしまうのです。ブラウザで JS が動くと `(undefined)` に置き換わるため画面からは消えますが、右クリック →「ページのソースを表示」を開けば誰でも読めます。

**NG:** 上の `EnvViewer.tsx` のように、`"use client"` の部品の中で `NEXT_PUBLIC_` の付いていない秘密の値を読む。

**OK:** 秘密の値は Server Component（`"use client"` を書かないファイル）や API の中だけで使い、画面には「結果」だけを出す。

```tsx
// src/app/env-check-ok/page.tsx（OK: "use client" を書かない＝ Server Component）
export default function EnvCheckOkPage() {
  // Server Component の中なら、秘密の値を読んでもブラウザ用の JS には入らない
  const hasSecret = Boolean(process.env.SECRET_MESSAGE);

  return (
    <main className="p-6">
      {/* 値そのものではなく「設定されているか」だけを表示する */}
      <p>秘密の値は設定済み？ {hasSecret ? "はい" : "いいえ"}</p>
    </main>
  );
}
```

**▼ 実行結果**（HTML の中に秘密の値は見つからない。画面には「秘密の値は設定済み？ はい」と表示される）

```text
PS> (curl.exe -s http://localhost:3001/env-check-ok | Select-String "TopSecret-XYZ-789").Matches.Value
PS>
```

> **つまずきポイント:** `NEXT_PUBLIC_` 付きでも、コードに **`process.env.NEXT_PUBLIC_〜` と「そのまま」書かないと値が埋め込まれません**。環境変数の名前を引数で受け取る「便利関数」を作ると、ブラウザでは `undefined` になります。

**NG: 名前を引数で渡して `process.env[name]` で読む**

```tsx
"use client";

// 環境変数の名前を受け取って値を返す「便利関数」（のつもり）
function readEnv(name: string) {
  return process.env[name];
}

export default function EnvViewer() {
  // OK: ドット記法でそのまま書く（ビルド時に値へ置き換わる）
  const direct = process.env.NEXT_PUBLIC_APP_NAME;
  // NG: 名前を引数で渡して [ ] で読む（置き換えが起きない）
  const viaFunction = readEnv("NEXT_PUBLIC_APP_NAME");

  return (
    <ul>
      <li>ドット記法: {direct ?? "(undefined)"}</li>
      <li>関数経由: {viaFunction ?? "(undefined)"}</li>
    </ul>
  );
}
```

**▼ 実行結果**（ビルドして本番モードで `/env-check` を開いたときの画面）

```text
ドット記法: MyBookShelf-PUBLIC-123
関数経由: (undefined)
```

Next.js がビルドのときに値へ置き換えるのは、ソースコードに `process.env.NEXT_PUBLIC_APP_NAME` と**文字どおり書かれている場所だけ**です。関数の中の `process.env[name]` は「実行してみないと何の名前か分からない」ので置き換えられず、ブラウザには `process.env` の中身が無いため `undefined` になります。

**OK: 使う変数ごとに `process.env.NEXT_PUBLIC_〜` と書き、1か所にまとめておく**

```ts
// src/lib/publicEnv.ts（OK: 名前をそのまま書いておけば、ビルド時に値へ置き換わる）
export const publicEnv = {
  // アプリ名（実験用）
  appName: process.env.NEXT_PUBLIC_APP_NAME,
  // Supabase の URL
  supabaseUrl: process.env.NEXT_PUBLIC_SUPABASE_URL,
};
```

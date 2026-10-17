登録する値の例です。`yourname` の部分は、あなたの Vercel のアカウント名（チーム名）に置き換えます。プレビュー URL の最後の `-yourname.vercel.app` の部分を見ると分かります。

```text
Site URL:
https://book-management-app.vercel.app

Redirect URLs:
http://localhost:3000/**
https://book-management-app.vercel.app/**
https://*-yourname.vercel.app/**
```

- `*` は「`.` と `/` 以外の何文字でも」、`**` は「`/` も含めて何文字でも」に当てはまる記号です（Supabase 公式ドキュメントの説明）。
- 3行目の `https://*-yourname.vercel.app/**` は、`book-management-app-git-feature-review-yourname.vercel.app` のようなプレビュー URL に当てはまります。
- Site URL は、戻り先を指定しなかったときに使われる「基本の戻り先」です。初期値は `http://localhost:3000` なので、公開したら本番の URL に変えておきます。

> **つまずきポイント:** プレビュー用だからといって `https://*.vercel.app/**` のように広く登録してはいけません。これだと**他人が Vercel で作ったアプリ**まで「戻ってきてよい場所」に含まれてしまい、ログインの情報を横取りされる危険があります。必ず自分のアカウント名（`-yourname`）まで含めて登録しましょう。

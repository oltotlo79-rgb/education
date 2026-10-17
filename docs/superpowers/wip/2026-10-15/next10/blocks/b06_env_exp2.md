#### 実験2: 値を変えても、ビルドし直すまでブラウザ用の JS は古いまま

> **▼ このコードがやること（先に日本語で）:** `.env.local` の Supabase の URL を書き換えて、**ビルドし直す前と後**で、ブラウザ用の JS に新しい値が入っているかを検索して比べます。「値を変えたのに反映されない」の正体を、目で確かめる実験です。

```powershell
# 1) .env.local の NEXT_PUBLIC_SUPABASE_URL を新しい値に書き換えて保存しておく
#    （例: https://xyzbookdemo123.supabase.co → https://newbookdemo456.supabase.co）
# 2) ビルドし直す「前」に、新しい値を探す
Select-String -Path .next\static\chunks\*.js -Pattern "newbookdemo456" -List | Select-Object Filename
# 3) ビルドし直す
npm run build
# 4) ビルドし直した「後」に、もう一度探す
Select-String -Path .next\static\chunks\*.js -Pattern "newbookdemo456" -List | Select-Object Filename
```

**▼ 実行結果**（ビルドの出力は省略）

```text
PS> Select-String -Path .next\static\chunks\*.js -Pattern "newbookdemo456" -List | Select-Object Filename
PS> npm run build
PS> Select-String -Path .next\static\chunks\*.js -Pattern "newbookdemo456" -List | Select-Object Filename

Filename
--------
2z5twj0x3ydak.js
```

- ビルドし直す**前**は、新しい値はどこにもありません。ブラウザ用の JS には、前回ビルドしたときの古い URL が書き込まれたままです。
- ビルドし直した**後**に、やっと新しい値が入りました。Vercel の「Redeploy」は、この「ビルドし直し」を Vercel のサーバー上で行うボタンです。

> **補足:** Vercel には、もう1つ「デプロイごとに環境変数の値を固定する」仕組みがあります。公式ドキュメントにも「環境変数の変更は、それ以前のデプロイには反映されず、新しいデプロイにだけ反映される」と書かれています。サーバーだけで使う変数でも同じなので、**「Vercel で環境変数を変えたら Redeploy」**と覚えておけば間違いありません。

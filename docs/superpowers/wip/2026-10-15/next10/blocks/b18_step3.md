3. 1つ目を入力したら「Add More」で入力欄を増やし、2つとも入力する

> **つまずきポイント:** Value 欄には「値だけ」を入れます。`.env.local` の1行をまるごと貼ったり、前後に `"`（ダブルクォート）や最後に `/` を付けたりすると、Vercel はそれも含めて「値」として保存します。手元では動くのに、本番でだけ Supabase への接続に失敗する原因になります。

**NG: Value に「変数名=」まで入っている／余計な記号が付いている**

```text
Key:   NEXT_PUBLIC_SUPABASE_URL
Value: NEXT_PUBLIC_SUPABASE_URL=https://xxxx.supabase.co

Key:   NEXT_PUBLIC_SUPABASE_URL
Value: "https://xxxx.supabase.co/"
```

**OK: Value には値だけを入れる**

```text
Key:   NEXT_PUBLIC_SUPABASE_URL
Value: https://xxxx.supabase.co
```

1.4.1 のチェックスクリプト（`prebuild`）を入れておけば、上の NG のような値は `https://` で始まらない、または `/` で終わるため、Vercel のビルドの時点で止まって気づけます。

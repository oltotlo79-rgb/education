たとえば、`.env.local` や Vercel に次のように登録したとします。名前の付け方1つで、安全かどうかが決まります。

**NG: 秘密のキーに `NEXT_PUBLIC_` を付けている**

```bash
# NG: ブラウザ用の JS に値が埋め込まれ、世界中に公開される（0.3 の実験1 を参照）
NEXT_PUBLIC_SUPABASE_SECRET_KEY=sb_secret_xxxxxxxxxxxxxxxx
```

**OK: 秘密のキーには `NEXT_PUBLIC_` を付けない**

```bash
# OK: サーバーで動くコードだけが読める（この教材のアプリでは、そもそも使わない）
SUPABASE_SECRET_KEY=sb_secret_xxxxxxxxxxxxxxxx
```

> **つまずきポイント:** 「Secret key もブラウザから使えたら便利そう」と考えて `NEXT_PUBLIC_` を付けるのは、家の合鍵を玄関に貼り出すようなものです。もし付けたまま公開してしまったら、名前を直すだけでは足りません。Supabase の **Project Settings → API Keys** でそのキーを無効にして新しいキーを作り、新しい値を登録し直してください。

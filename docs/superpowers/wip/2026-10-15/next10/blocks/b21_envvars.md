1. Vercel ダッシュボードでプロジェクトを選択
2. 「Settings」を開き、左のサイドバーから「Environment Variables」を選ぶ（2026年の画面では、上のタブではなくサイドバーから選びます）
3. **Type** で「Config」か「Secret」を選び、Key・Value・Environments を入力して「Save」を押す。登録済みの変数を直すときは、その行の右端の「⋯」→「Edit」
4. **重要**: 環境変数を変更した後は、サイドバーの「Deployments」で最新のデプロイの「⋯」→「Redeploy」を選び、確認画面でもう一度「Redeploy」を押す必要があります

{{FIG:f12}}

2026年8月から、環境変数を登録するときに **Type（種類）** を選ぶようになりました（それまでの「Sensitive」というスイッチの代わりです）。

| Type | 保存したあとに値を画面で見られるか | 向いている値 |
|------|------|------|
| Config（コンフィグ：設定値） | 見られる | `NEXT_PUBLIC_SUPABASE_URL`、`NEXT_PUBLIC_SUPABASE_ANON_KEY` のような、公開してよい値 |
| Secret（シークレット：秘密） | 見られない（上書きはできる） | Secret key（`sb_secret_...`）など、本当に秘密の値 |

- `NEXT_PUBLIC_` 付きの値は、どのみちブラウザに配られる公開情報なので **Config** で問題ありません。あとで「何を登録したっけ？」と確認できて便利です。
- Redeploy の確認画面には「Use existing Build Cache（前回のビルド結果の一部を使い回す）」というチェックがあります。環境変数を変えたときは、念のためチェックを外して、最初からビルドし直すと確実です。

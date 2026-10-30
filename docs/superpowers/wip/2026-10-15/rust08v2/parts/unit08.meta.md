title: Better Auth との比較と、おすすめの構成
---checklist
- ☐ Better Auth を本物の Node.js で動かし、RustNote と同じ順番（登録・重複・失敗・回数の制限・Origin・DB・ログアウト）で確かめられる
- ☐ 同じ所（失敗の文と時間・SameSite=Lax・Origin・ログアウトで行を消す）と、違う所（DB のトークン・署名付きのクッキー・scrypt・期限・回数の制限）を説明できる
- ☐ DB が漏れたときの守りが、トークンのハッシュを入れる RustNote と、トークンを入れて署名する Better Auth で、どう違うかを言える
- ☐ secret の設定忘れ・開発のときの回数の制限・送り元の IP が分からないとき、の3つの設定の間違いを説明できる
- ☐ Rust のアプリで、自分で作る・Better Auth・外の認証サービスのどれを選ぶかを、理由とともに言える
---glossary
| Better Auth | TypeScript の認証のライブラリ。JavaScript の実行環境で動き、プラグインで機能を足せる |
| scrypt | メモリを多く使う、パスワード用のハッシュの方法。Better Auth の既定 |
| 署名付きのクッキー | 秘密の鍵で作った署名を、値に付けたクッキー。書き換えると署名が合わなくなる |
| `BETTER_AUTH_SECRET` | Better Auth の秘密の鍵を渡す環境変数。本番で無いと止まる |
| 429 Too Many Requests | 短い時間に送りすぎた、という返事。回数の制限に引っかかったときに返る |
| `X-Forwarded-For` | 中継のサーバーが、元の送り元の IP を伝えるためのヘッダー |
| リバースプロキシ | アプリの手前で、リクエストを受けて渡す中継のサーバー |
| NFKC | 全角と半角などをそろえる Unicode の正規化。Better Auth はパスワードに使う |

title: クッキー・CSRF・Origin・SameSite
---checklist
- ☐ OriginPolicy の判断の順番（見るだけのメソッド → 信じる送り元 → Sec-Fetch-Site → Origin と Host → どちらも無い）を、表で確かめて説明できる
- ☐ 本物のブラウザで、HttpOnly のクッキーが document.cookie に出ないことと、SameSite=Lax のクッキーが、ほかのサイトからの fetch と POST には付かず、リンクの GET には付くことを確かめられる
- ☐ CSRF の守りが SameSite と OriginPolicy の2つの層で、OriginPolicy を外すと同じサイトの別の名前からの POST が通る理由を言える
- ☐ ほかのサービスからの POST を `trust_origins` で信じるときの、値の書き方（最後に / を付けない）を説明できる
- ☐ 状態を変える操作を、ログアウトも含めて全部 POST にする理由を言える
---glossary
| CSRF | ほかのサイトのページが、ログイン中の利用者のブラウザに、勝手にリクエストを送らせる攻撃 |
| OriginPolicy | GET 以外の、ほかのサイトからのリクエストを 403 にする Topcoat の守り。決まりで必ず付く |
| `Sec-Fetch-Site` | ブラウザが付ける「どこから来たか」の印。same-origin・same-site・cross-site・none |
| `Origin` | ブラウザが付ける、送り元のページの「方式・名前・ポート」。最後に / は付かない |
| same-site | 方式と登録した名前（example.com の部分）が同じこと。別の名前（サブドメイン）でも same-site |
| `SameSite=Lax` | ほかのサイトからの POST や裏の読み込みには付けず、画面ごと移る GET には付けるクッキーの設定 |
| `HttpOnly` | ページの JavaScript から読めないクッキーの設定 |
| `__Host-` | Secure・Path=/・Domain なしのときだけ受け付けられる、クッキーの名前の始まり |
| `trust_origins` | 決まった送り元からの、変えるリクエストを通す OriginPolicy の設定 |

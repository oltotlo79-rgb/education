title: E2E・ブラウザ・アクセシビリティ
---checklist
- ☐ Playwright で本物の Chrome を動かし、いちばん大事な流れ（登録 → 作成 → 一覧 → ログアウト）を1本のテストで確かめられる
- ☐ webServer で、テストのたびに RustNote を新しい DB で起動する設定を書ける
- ☐ getByLabel・getByRole で、画面に書かれた名前で探す理由を、アクセシビリティと結び付けて言える
- ☐ maxlength が UTF-16 で数えることを本物のブラウザで確かめ、守りはサーバーの parse がする理由を言える
- ☐ axe の指摘（landmark-unique）を読み、aria-label で直せる
---glossary
| E2E | 利用者と同じ入口（ブラウザ）から、サーバー・DB まで本物でつないで確かめるテスト |
| Playwright | ブラウザをプログラムから動かす道具。`channel: "chrome"` でパソコンの Chrome を使える |
| `webServer` | Playwright の設定。テストの前にサーバーを起動し、URL が答えるまで待つ |
| `getByLabel`・`getByRole` | ラベルや、ボタン・リンクの名前で要素を探す、Playwright の探し方 |
| strict mode | 探した要素が1つに決まらないときに止まる、Playwright の決まり |
| アクセシビリティ（a11y） | 障害のある人や、キーボード・読み上げを使う人も、同じように使えること |
| ランドマーク | `header`・`nav`・`main` など、画面の大きな区切り。同じ種類が2つあるときは名前を付ける |
| axe | 画面のアクセシビリティを、決まった点検の項目で自動に調べる道具 |

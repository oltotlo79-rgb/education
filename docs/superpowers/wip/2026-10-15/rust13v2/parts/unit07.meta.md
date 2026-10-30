title: 画面の枠・部品・登録とログインの画面
---checklist
- ☐ module_router!() のファイルの並びと URL の対応、build_router で足す4つ（クッキー・セッション・アセット・共有の状態）を言える
- ☐ #[layer] の access_log と #[layout] の root_layout の役目と、405・403 がアクセスログに残らない理由を説明できる
- ☐ view! の文字の書き方（囲まないと expected view node）と、(式) の自動のエスケープを、本物の HTML で示せる
- ☐ #[component] の名前付きの引数（忘れると E0277）と、href! がページの関数だけを行き先にできることを説明できる
- ☐ 登録・ログイン・ログアウト・ヘルスチェックの状態コード（400・409・303・401・405・403・200）を、curl の結果で言える
---glossary
| モジュールルーティング | ファイル（モジュール）の並びを、そのまま URL の並びにするしくみ（`module_router!()`） |
| `#[layer]` | ページの処理の前と後に処理を足す関数。RustNote のアクセスログ |
| `#[layout]` | どのページも包む外枠。RustNote では HTML の外枠・ヘッダー・エラー画面への差し替え |
| `error_boundary` | 枠の中で、ページのエラーを受け止め、別の画面（error_page）に替える部品 |
| `#[component]` | 画面の部品にする印。`view!` の中で、名前付きの引数で使う |
| `href!` | ページの関数から URL を作るマクロ。打ち間違いがコンパイルで見つかる |
| PRG | Post/Redirect/Get。POST の結果を 303 で GET の画面に移し、再読み込みで二重に送らないようにする形 |
| OriginPolicy | Topcoat の守り。ほかのサイトからの POST を 403 で止める |

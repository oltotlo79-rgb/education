title: HTTP のテスト
---checklist
- ☐ レスポンスの4つの部分（状態コード・ヘッダー・Set-Cookie・本文）を全部表示して読み、それぞれ何を確かめればよいかを言える
- ☐ assert_redirect や cookie_attributes のような確かめの道具を作り、#[track_caller] で、失敗の場所をテストの行にできる
- ☐ 本文を contains ではなく、scraper の CSS セレクターで要素を決めて確かめる理由を、空の一覧でも通ってしまう例で説明できる
- ☐ 同じ名前のヘッダーを get_all で全部読み、名前で探す理由（Set-Cookie の順番は決まっていない）を言える
- ☐ body_text のあとでレスポンスを使えない E0382 を読み、状態コードを先に取っておく形に直せる
---glossary
| ヘッダー | レスポンスの本文の前に付く「名前: 値」の行。同じ名前の行が、いくつあってもよい |
| `Set-Cookie` | クッキーを渡すヘッダー。`HttpOnly`・`Secure`・`SameSite`・`Path`・`Max-Age` などの属性が付く |
| CSS セレクター | `ul.note-list li a` のように、HTML の要素を場所で指定する書き方 |
| `scraper` | HTML を読み、CSS セレクターで要素を探すクレート |
| `#[track_caller]` | 関数の中で失敗したとき、その関数を呼んだ行を、失敗の場所として表示させる印 |
| `get_all` | 同じ名前のヘッダーを、全部取り出すメソッド（`get` は1つ目だけ） |
| 守りのヘッダー | `X-Frame-Options`・`Content-Security-Policy` など、ブラウザに守り方を伝えるヘッダー |

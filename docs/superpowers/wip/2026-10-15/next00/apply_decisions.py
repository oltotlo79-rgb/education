import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(old, new, count=1):
    global s
    n = s.count(old)
    if n != count:
        raise SystemExit(f"count mismatch ({n} != {count}): {old[:80]!r}")
    s = s.replace(old, new)

# ---- status values (crud.ts) ----
rep('books.push({ id: nextId, title: title, status: "unread" });', 'books.push({ id: nextId, title: title, status: "want_to_read" });', 2)
rep('updateStatus(1, "finished");\nconsole.log("② 1番を読了(finished)にした後");', 'updateStatus(1, "completed");\nconsole.log("② 1番を読了(completed)にした後");')
rep("""① 2冊を登録した直後
  1: リーダブルコード [unread]
  2: 達人プログラマー [unread]
② 1番を読了(finished)にした後
  1: リーダブルコード [finished]
  2: 達人プログラマー [unread]
③ 2番を削除した後
  1: リーダブルコード [finished]""", """① 2冊を登録した直後
  1: リーダブルコード [want_to_read]
  2: 達人プログラマー [want_to_read]
② 1番を読了(completed)にした後
  1: リーダブルコード [completed]
  2: 達人プログラマー [want_to_read]
③ 2番を削除した後
  1: リーダブルコード [completed]""")
rep('登録したばかりの本の状態は `"unread"`（未読）です。', '登録したばかりの本の状態は `"want_to_read"`（読みたい）です。本書のアプリでも、読書ステータスは `want_to_read`（読みたい）・`reading`（読書中）・`completed`（読了）の3つの値を使います。')
# ---- status values (json) ----
rep('  status: "finished",\n};', '  status: "completed",\n};')
rep('{"title":"こころ","author":"夏目漱石","status":"finished"}', '{"title":"こころ","author":"夏目漱石","status":"completed"}')
rep('>status: "finished"</text>', '>status: "completed"</text>', 2)
rep('>"status":"finished"}</text>', '>"status":"completed"}</text>')
# ---- status labels in mockups / figure ----
rep('font-weight: 600;">📕 未読</span>', 'font-weight: 600;">📕 読みたい</span>')
rep('background: white;"></span> 未読\n', 'background: white;"></span> 読みたい\n')
rep('<rect x="256" y="126" width="46" height="16" rx="8" fill="#fef3c7"/>\n<text x="279" y="138.5" text-anchor="middle" font-size="12" fill="#92400e">未読</text>',
    '<rect x="250" y="126" width="56" height="16" rx="8" fill="#fef3c7"/>\n<text x="278" y="138.5" text-anchor="middle" font-size="12" fill="#92400e">読みたい</text>')
rep('> - `status` … 読書状態。「未読／読書中／読了」のいずれか。', '> - `status` … 読書状態。`want_to_read`（読みたい）／`reading`（読書中）／`completed`（読了）のいずれか。')

# ---- published_year -> rating (the app uses rating; no later chapter uses published_year) ----
rep('<label style="display: block; font-size: 13px; font-weight: 600; color: #334155; margin-bottom: 6px;">出版年</label>\n      <div style="background: white; border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px 14px; font-size: 14px; color: #94a3b8;">例: 2024</div>',
    '<label style="display: block; font-size: 13px; font-weight: 600; color: #334155; margin-bottom: 6px;">評価（1〜5）</label>\n      <div style="background: white; border: 1px solid #e2e8f0; border-radius: 8px; padding: 10px 14px; font-size: 14px; color: #94a3b8;">例: 4</div>')
rep('<!-- 出版年 -->', '<!-- 評価 -->')
rep('- **新規登録**: タイトル・著者・出版年・ステータス・メモを入力して書籍を登録', '- **新規登録**: タイトル・著者・評価・ステータス・メモを入力して書籍を登録')
rep('<code style="background: #ede7f6; padding: 1px 5px; border-radius: 4px; font-size: 11px;">published_year</code><br>',
    '<code style="background: #ede7f6; padding: 1px 5px; border-radius: 4px; font-size: 11px;">rating</code><br>')
rep('<code style="background: #ede7f6; padding: 1px 5px; border-radius: 4px; font-size: 11px;">created_at</code>\n',
    '<code style="background: #ede7f6; padding: 1px 5px; border-radius: 4px; font-size: 11px;">created_at</code>\n        <code style="background: #ede7f6; padding: 1px 5px; border-radius: 4px; font-size: 11px;">updated_at</code>\n')
rep('> - `published_year` … 出版年（数値）。', '> - `rating` … 評価（1〜5 の整数。まだ評価していなければ空＝null）。')
rep('> - `created_at` … レコードが作成された日時。自動で入る。', '> - `created_at` … レコードが作成された日時。自動で入る。\n> - `updated_at` … レコードが最後に更新された日時。自動で入る。\n>\n> ほかにも出版社・出版日・表紙画像の URL などの列があります。テーブルの設計は第5章でくわしく行います。')

# ---- list page is "/" ----
rep('> - **NG:** `http://localhost/books` … ポート番号を省略したので 80 番の窓口に行ってしまい、開発中のアプリにつながらない\n> - **OK:** `http://localhost:3000/books` … `npm run dev` が待っている 3000 番の窓口に届く',
    '> - **NG:** `http://localhost/books/new` … ポート番号を省略したので 80 番の窓口に行ってしまい、開発中のアプリにつながらない\n> - **OK:** `http://localhost:3000/books/new` … `npm run dev` が待っている 3000 番の窓口に届く')
rep('その間にブラウザで `http://localhost:3000/books` を開くと「ようこそ、本の店へ」と表示され、ターミナルには `[サーバー] 注文を受けました: GET /books` と記録されます',
    'その間にブラウザで `http://localhost:3000/hello` を開くと「ようこそ、本の店へ」と表示され、ターミナルには `[サーバー] 注文を受けました: GET /hello` と記録されます')
rep('<text x="80" y="37.5" font-family="Consolas, monospace" font-size="12" fill="#334155">localhost:3000/books</text>',
    '<text x="80" y="37.5" font-family="Consolas, monospace" font-size="12" fill="#334155">localhost:3000/</text>')
rep('`http://localhost:3000/books?status=reading` という URL', '`http://localhost:3000/?status=reading` という URL')
rep("(1) `http:`（http）、(2) `localhost`、(3) `3000`、(4) `/books`、(5) `?status=reading`", "(1) `http:`（http）、(2) `localhost`、(3) `3000`、(4) `/`、(5) `?status=reading`")
rep('const url = new URL("http://localhost:3000/books?status=reading");', 'const url = new URL("http://localhost:3000/?status=reading");')
rep("http:\nlocalhost\n3000\n/books\n?status=reading\nreading", "http:\nlocalhost\n3000\n/\n?status=reading\nreading")
rep('- `?status=reading` のようなクエリ文字列は、「読書中の本だけ」のような絞り込みの条件を URL に載せるときによく使われる形です。',
    '- パスの `/` は「サイトのいちばん上（トップページ）」を表します。本書のアプリでは、書籍の一覧ページが `/` です。\n- `?status=reading` のようなクエリ文字列は、「読書中の本だけ」のような絞り込みの条件を URL に載せるときによく使われる形です。')

# ---- Next.js 16 ----
rep("""```
> next dev

   ▲ Next.js 15.0.0
   - Local:        http://localhost:3000

 ✓ Ready in 2.1s
```""", """```
> book-management@0.1.0 dev
> next dev

▲ Next.js 16.4.1 (Turbopack)
- Local:         http://localhost:3000
- Network:       http://192.168.x.x:3000
✓ Ready in 7.0s
```

（第6章で作るプロジェクトで、Next.js 16.4.1 を使って実際に起動したときの出力です。`Network:` の数字や `Ready in` の秒数は環境によって変わります）""")
rep('| **Next.js** | 15.x |', '| **Next.js** | 16.x |')
rep('| **Node.js** | 20.x 以上 |', '| **Node.js** | 24（LTS）推奨・22 以上 |')
rep('| **npm** | 10.x |', '| **npm** | 11.x（Node.js 24 に付属） |')
rep('- Next.js の開発サーバーは、3000 番がふさがっていると自動で 3001 番などに切り替え、ターミナルに「Port 3000 is in use」という警告を出します。その場合はブラウザでも `http://localhost:3001` を開く必要があります。',
    '- Next.js の開発サーバーは、3000 番がふさがっていると自動で 3001 番などに切り替え、ターミナルに `⚠ Port 3000 is in use by process 9076, using available port 3001 instead.`（3000 番は使用中なので、空いている 3001 番を使います。数字は場合によって変わります）のような警告を出します。その場合はブラウザでも `http://localhost:3001` を開く必要があります。')
open(p, "w", encoding="utf-8", newline="\n").write(s)
print("done")

本物の出力で練習してみましょう。次の図は、Next.js 16.4.1 で `BookCard.tsx` の `book.title` を、わざと `book.titel` と打ち間違えてビルドしたときの出力です。

{{FIG:f06}}

> **ヒント:** Next.js 15 では `./src/components/BookCard.tsx:7:39` と `Type error: ...` の形、16 では `src/components/BookCard.tsx(7,39): error TS2551: ...` の形で表示されます。見た目は違っても「ファイル名・行・列・種類・内容」が入っている点は同じです。

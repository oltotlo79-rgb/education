// Server Action 用の専用ファイル
// app/books/new/actions.ts
// ファイル先頭の `"use server"` で「この中の関数は全部サーバー実行」と宣言
"use server";

// この関数はサーバー上でのみ実行される
// クライアントには関数の中身は送信されない
// 引数 `FormData` は Web標準のフォーム値コンテナ
// <form action={createBook}> の送信内容が自動で渡される
export async function createBook(formData: FormData) {
  // `formData.get("title")` で name="title" の値を取り出す
  // 戻り値は `FormDataEntryValue | null` なので `as string` で型を絞り込む
  const title = formData.get("title") as string;
  // 同じく著者
  const author = formData.get("author") as string;
  // 数値に変換（formData の値は基本文字列）
  const publishedYear = Number(formData.get("publishedYear"));

  // バリデーション
  // 必須項目が空ならエラーを返す
  if (!title || !author) {
    // オブジェクトを返すと呼び出し側で state として扱える
    return {
      error: "タイトルと著者は必須です",
    };
  }

  // データベースに保存（例）
  // 本番ではここで Supabase や Prisma で INSERT
  // const book = await db.book.create({
  //   data: { title, author, publishedYear },
  // });

  // サーバーのコンソールに出力（ブラウザには出ない）
  console.log("書籍を追加:", { title, author, publishedYear });

  // 成功したら書籍一覧にリダイレクト
  // redirect("/books");
  // `redirect()` を呼べば一覧ページに自動遷移
  // 成功フラグを返す
  return { success: true };
}
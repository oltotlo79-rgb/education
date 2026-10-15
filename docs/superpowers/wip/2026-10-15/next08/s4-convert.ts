// 編集ページと BookForm がやっている「値の変換」だけを取り出して
// 動かしてみる練習用コード

// Supabase から返ってくる books テーブルの1行の形
type BookRow = {
  id: string;
  title: string;
  author: string | null;
  publisher: string | null;
  published_date: string | null;
  rating: number | null;
  status: string | null;
  memo: string | null;
};

// フォームが扱う値の形（文字の欄は必ず string）
type BookFormData = {
  title: string;
  author: string;
  publisher: string;
  published_date: string;
  rating: number | null;
  status: string;
  memo: string;
};

// 編集ページの initialData 作りと同じ変換（null → ""）
function toFormData(book: BookRow): BookFormData {
  return {
    title: book.title ?? "",
    author: book.author ?? "",
    publisher: book.publisher ?? "",
    published_date: book.published_date ?? "",
    rating: book.rating ?? null,
    status: book.status ?? "unread",
    memo: book.memo ?? "",
  };
}

// BookForm が .update() に渡す中身と同じ変換（"" → null、空白を除去）
function toUpdatePayload(form: BookFormData) {
  return {
    title: form.title.trim(),
    author: form.author.trim() || null,
    publisher: form.publisher.trim() || null,
    published_date: form.published_date || null,
    rating: form.rating,
    status: form.status,
    memo: form.memo.trim() || null,
  };
}

// DB にこんな行が入っていたとする（出版社・出版日・メモは未入力）
const row: BookRow = {
  id: "3f2a8c1e-5b7d-4e9a-9c01-2b3d4e5f6a7b",
  title: "React実践ガイド",
  author: "山田太郎",
  publisher: null,
  published_date: null,
  rating: 4,
  status: "reading",
  memo: null,
};

// ① DB の行 → フォームの初期値
const form = toFormData(row);
console.log("① フォームの初期値:", form);

// ② ユーザーが出版社を入力し、メモに空白だけ打ったとする
const edited: BookFormData = {
  ...form,
  publisher: "  技術評論社  ",
  memo: "   ",
};

// ③ フォームの値 → UPDATE で送る中身
console.log("③ UPDATE で送る中身:", toUpdatePayload(edited));

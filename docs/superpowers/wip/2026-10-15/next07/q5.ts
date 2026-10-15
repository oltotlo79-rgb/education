// BookForm.tsx と同じ型（Node で単体実行するためにここへ写している）
type BookStatus = "reading" | "completed" | "want_to_read";

type BookFormData = {
  title: string;
  author: string;
  publisher: string;
  published_date: string;
  rating: number | null;
  status: BookStatus;
  memo: string;
};

// 「エラーが出た項目だけ、項目名 → メッセージ」で入る箱の型
type FormErrors = Partial<Record<keyof BookFormData, string>>;

// BookForm の validate と同じルールを、画面と切り離した「ただの関数」にしたもの
function validateBookForm(data: BookFormData): FormErrors {
  const errors: FormErrors = {};

  // タイトル: 必須、100文字以内
  if (!data.title.trim()) {
    errors.title = "タイトルは必須です";
  } else if (data.title.trim().length > 100) {
    errors.title = "タイトルは100文字以内で入力してください";
  }

  // 著者: 必須、50文字以内
  if (!data.author.trim()) {
    errors.author = "著者は必須です";
  } else if (data.author.trim().length > 50) {
    errors.author = "著者は50文字以内で入力してください";
  }

  // 出版社: 任意、50文字以内
  if (data.publisher.trim().length > 50) {
    errors.publisher = "出版社は50文字以内で入力してください";
  }

  // メモ: 任意、1000文字以内
  if (data.memo.trim().length > 1000) {
    errors.memo = "メモは1000文字以内で入力してください";
  }

  return errors;
}

// 新規登録フォームの初期値（BookForm の defaultFormData と同じ）
const empty: BookFormData = {
  title: "",
  author: "",
  publisher: "",
  published_date: "",
  rating: null,
  status: "want_to_read",
  memo: "",
};

console.log(
  validateBookForm({
    ...empty,
    title: "　　",
    author: "A".repeat(51),
    memo: "   ",
  }),
);

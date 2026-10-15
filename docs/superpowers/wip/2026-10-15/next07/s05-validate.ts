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

// 試したい入力のパターンを並べる
const cases: { label: string; data: BookFormData }[] = [
  { label: "1. 何も入力しない", data: empty },
  { label: "2. 空白だけ入力", data: { ...empty, title: "   ", author: "　" } },
  {
    label: "3. タイトルが101文字",
    data: { ...empty, title: "あ".repeat(101), author: "山田" },
  },
  {
    label: "4. 正しい入力",
    data: { ...empty, title: "リーダブルコード", author: "Dustin Boswell" },
  },
];

for (const { label, data } of cases) {
  const errors = validateBookForm(data);
  const canSubmit = Object.keys(errors).length === 0;
  console.log(label, "→", canSubmit ? "送信OK" : "送信しない", errors);
}

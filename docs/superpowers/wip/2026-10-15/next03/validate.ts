// ファイル名: validate.ts
// 5.3 のフォームと同じ形の入力データ
type BookFormData = {
  title: string;
  author: string;
  price: string;
  category: string;
  description: string;
};

// 「エラーが出た項目だけ、項目名 → メッセージ」が入る型
type FormErrors = Partial<Record<keyof BookFormData, string>>;

// 5.3 の validate から「チェックする部分」だけを取り出した関数
// （setErrors を呼ぶ代わりに、見つけたエラーを return する）
function findErrors(formData: BookFormData): FormErrors {
  const newErrors: FormErrors = {};
  if (formData.title.trim() === "") {
    newErrors.title = "タイトルは必須です";
  }
  if (formData.author.trim() === "") {
    newErrors.author = "著者は必須です";
  }
  if (formData.price === "" || Number(formData.price) <= 0) {
    newErrors.price = "価格は0より大きい数値を入力してください";
  }
  return newErrors;
}

// ラベルと入力データを受け取り、チェック結果を表示する
function check(label: string, formData: BookFormData): void {
  const errors = findErrors(formData);
  // エラーの数が 0 なら送信してよい
  const canSubmit = Object.keys(errors).length === 0;
  console.log(`【${label}】 送信できる: ${canSubmit}`);
  console.log(errors);
}

// 何も入力していないフォーム（カテゴリだけ最初から選ばれている）
const emptyForm: BookFormData = {
  title: "",
  author: "",
  price: "",
  category: "programming",
  description: "",
};

check("全部空", emptyForm);
check("空白だけのタイトルと負の価格", {
  ...emptyForm,
  title: "   ",
  author: "田中",
  price: "-100",
});
check("正しい入力", {
  ...emptyForm,
  title: "React入門",
  author: "田中",
  price: "2800",
});

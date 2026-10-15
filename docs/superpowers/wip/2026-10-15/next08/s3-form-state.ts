// BookForm の handleChange が state をどう作り替えるのかを、
// React を使わずに確かめる練習用コード

type BookFormData = {
  title: string;
  author: string;
  publisher: string;
  published_date: string;
  rating: number | null;
  status: string;
  memo: string;
};

// 編集ページから渡ってくる既存データ（initialData）の例
const initialData: BookFormData = {
  title: "React実践ガイド",
  author: "山田太郎",
  publisher: "技術評論社",
  published_date: "2024-01-15",
  rating: 4,
  status: "reading",
  memo: "",
};

// handleChange と同じく「1項目だけ差し替えた新しいオブジェクト」を作る
function applyChange(
  prev: BookFormData,
  name: string,
  value: string,
): BookFormData {
  return {
    ...prev,
    [name]: value,
  };
}

// 「ステータス欄を読了に変えた」という入力を再現する
const next = applyChange(initialData, "status", "finished");
console.log("変更前の status:", initialData.status);
console.log("変更後の status:", next.status);
console.log("title はそのまま:", next.title);
// React は「前と別のオブジェクトか」で変化を見分ける
console.log("新しいオブジェクトになった?:", !Object.is(initialData, next));

// NG: 元のオブジェクトを直接書き換えると「同じオブジェクト」のまま
const sameObject = initialData;
sameObject.status = "finished";
console.log(
  "直接書き換えたら別物になった?:",
  !Object.is(initialData, sameObject),
);

// ?? と || の違い（左がどんな値のときに右へ置き換わるか）
const samples = [null, undefined, "", 0];
for (const v of samples) {
  const label = String(JSON.stringify(v)).padEnd(9);
  const withNullish = JSON.stringify(v ?? "既定値");
  const withOr = JSON.stringify(v || "既定値");
  console.log(`${label} ?? → ${withNullish} / || → ${withOr}`);
}

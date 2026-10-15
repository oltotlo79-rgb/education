interface Book {
  title: string;
  price: number;
}

// 「本当に Book の形をしているか」を実行時に調べる型ガード関数
function isBook(value: unknown): value is Book {
  if (typeof value !== "object" || value === null) {
    return false;
  }
  // in で「そのプロパティを持っているか」を確かめてから、型を調べる
  return (
    "title" in value &&
    typeof value.title === "string" &&
    "price" in value &&
    typeof value.price === "number"
  );
}

const json = '{"title":"TypeScript入門"}';

// JSON.parse の結果は「まだ何か分からない」ので unknown として受け取る
const data: unknown = JSON.parse(json);

if (isBook(data)) {
  console.log(data.price.toLocaleString());
} else {
  console.log("本のデータの形が正しくありません:", data);
}

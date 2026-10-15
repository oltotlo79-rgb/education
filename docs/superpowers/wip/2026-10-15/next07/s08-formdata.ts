type BookStatus = "reading" | "completed" | "want_to_read";

// データベースに INSERT する直前の形（books テーブルの列名に合わせる）
type BookInsert = {
  title: string;
  author: string;
  publisher: string | null;
  published_date: string | null;
  rating: number | null;
  status: BookStatus;
  memo: string | null;
};

// 成功なら value（保存してよい値）、失敗なら errors（項目名 → メッセージ）を返す
type ParseResult =
  | { ok: true; value: BookInsert }
  | { ok: false; errors: Record<string, string> };

// 許可するステータスの一覧
const STATUSES: BookStatus[] = ["reading", "completed", "want_to_read"];

// FormData から1項目を「必ず文字列」として取り出す小さな道具
function getText(formData: FormData, name: string): string {
  // 戻り値は string か File か null（FormDataEntryValue | null）
  const raw = formData.get(name);
  // 文字列以外（File や null）が来たら空文字として扱う
  if (typeof raw !== "string") {
    return "";
  }
  return raw.trim();
}

function parseBookForm(formData: FormData): ParseResult {
  const errors: Record<string, string> = {};

  // (1) 必須項目
  const title = getText(formData, "title");
  if (title === "") {
    errors.title = "タイトルは必須です";
  }
  const author = getText(formData, "author");
  if (author === "") {
    errors.author = "著者は必須です";
  }

  // (2) 評価: 空なら null、それ以外は「1〜5 の整数」かを確かめてから数値にする
  const ratingText = getText(formData, "rating");
  let rating: number | null = null;
  if (ratingText !== "") {
    const n = Number(ratingText);
    if (Number.isInteger(n) && n >= 1 && n <= 5) {
      rating = n;
    } else {
      errors.rating = "評価は1〜5から選んでください";
    }
  }

  // (3) ステータス: 決められた3つのどれかと一致するものを探す
  const statusText = getText(formData, "status");
  const status = STATUSES.find((s) => s === statusText);
  if (status === undefined) {
    errors.status = "ステータスの値が正しくありません";
  }

  // (4) エラーが1つでもあれば失敗として返す
  if (Object.keys(errors).length > 0 || status === undefined) {
    return { ok: false, errors };
  }

  // (5) 任意項目は「空なら null」にそろえて返す
  return {
    ok: true,
    value: {
      title,
      author,
      publisher: getText(formData, "publisher") || null,
      published_date: getText(formData, "published_date") || null,
      rating,
      status,
      memo: getText(formData, "memo") || null,
    },
  };
}

// 動作確認1: 画面から普通に送られてきた入力
const good = new FormData();
good.append("title", "  リーダブルコード  ");
good.append("author", "Dustin Boswell");
good.append("publisher", "");
good.append("rating", "4");
good.append("status", "completed");
console.log("確認1:", parseBookForm(good));

// 動作確認2: 開発者ツールなどで書き換えられた入力（著者欄なし・評価99・存在しないステータス）
const bad = new FormData();
bad.append("title", "テスト");
bad.append("rating", "99");
bad.append("status", "deleted");
console.log("確認2:", parseBookForm(bad));

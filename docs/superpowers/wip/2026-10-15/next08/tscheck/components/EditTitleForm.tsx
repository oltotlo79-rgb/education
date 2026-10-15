// 上で作った Server Action を読み込む
import {
  updateBookTitle,
  updateBookTitleFromHidden,
} from "@/app/books/actions";

type Props = {
  id: string;
  title: string;
};

// 方法A: bind で id を固定する
export function EditTitleFormWithBind({ id, title }: Props) {
  // id を1番目の引数に固定した新しい関数を作る（まだ実行しない）
  const updateBookTitleWithId = updateBookTitle.bind(null, id);

  return (
    <form action={updateBookTitleWithId}>
      <input name="title" defaultValue={title} />
      <button type="submit">更新する</button>
    </form>
  );
}

// 方法B: hidden input で id も一緒に送る
export function EditTitleFormWithHidden({ id, title }: Props) {
  return (
    <form action={updateBookTitleFromHidden}>
      {/* 画面には出ないが、送信時に id も一緒に送られる */}
      <input type="hidden" name="id" value={id} />
      <input name="title" defaultValue={title} />
      <button type="submit">更新する</button>
    </form>
  );
}

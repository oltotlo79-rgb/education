// app/ng-func/page.tsx（Server Component）
import { DeleteButton } from "./DeleteButton";

export default function NgFuncPage() {
  return (
    <DeleteButton
      title="リーダブルコード"
      onDelete={() => console.log("削除しました")}
    />
  );
}

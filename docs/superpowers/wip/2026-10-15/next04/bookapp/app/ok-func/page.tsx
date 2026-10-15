// app/ok-func/page.tsx（Server Component）
import { DeleteButton } from "./DeleteButton";

export default function OkFuncPage() {
  // 渡すのは「文字列」だけ。関数は渡さない
  return <DeleteButton title="リーダブルコード" />;
}

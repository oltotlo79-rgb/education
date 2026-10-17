// app/env-ok/page.tsx（Server Component）
import { EnvClient } from "./EnvClient";

export default function EnvOkPage() {
  // 秘密のキーはサーバーの中で使うだけにする
  // Client Component には「設定されているかどうか」という結果だけを渡す
  const hasKey = Boolean(process.env.SECRET_API_KEY);

  return (
    <EnvClient
      siteName={process.env.NEXT_PUBLIC_SITE_NAME ?? ""}
      hasKey={hasKey}
    />
  );
}

// app/env-check/page.tsx（Server Component）
import { EnvClient } from "./EnvClient";

export default function EnvCheckPage() {
  // サーバーでは NEXT_PUBLIC_ の有無に関係なく読める
  console.log("[server] SECRET_API_KEY =", process.env.SECRET_API_KEY);
  console.log("[server] NEXT_PUBLIC_SITE_NAME =", process.env.NEXT_PUBLIC_SITE_NAME);

  return (
    <div>
      <h1>環境変数チェック</h1>
      <EnvClient />
    </div>
  );
}

// app/where/page.tsx（"use client" が無いので Server Component）
import { ClientLogger } from "./ClientLogger";

export default function WherePage() {
  // このログはどこに出る？
  console.log("[WherePage] Server Component を実行しました");

  return (
    <div>
      <h1>どこで動く？実験</h1>
      {/* ここに Client Component を埋め込む */}
      <ClientLogger />
    </div>
  );
}

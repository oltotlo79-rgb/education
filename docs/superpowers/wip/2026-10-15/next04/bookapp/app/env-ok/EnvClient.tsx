// app/env-ok/EnvClient.tsx（Client Component）
"use client";

export function EnvClient({
  siteName,
  hasKey,
}: {
  siteName: string;
  hasKey: boolean;
}) {
  return (
    <ul>
      <li>サイト名: {siteName}</li>
      <li>秘密のキー: {hasKey ? "設定済み" : "未設定"}</li>
    </ul>
  );
}

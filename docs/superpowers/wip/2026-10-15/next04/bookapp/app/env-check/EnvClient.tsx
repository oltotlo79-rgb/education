// app/env-check/EnvClient.tsx（Client Component）
"use client";

export function EnvClient() {
  return (
    <ul>
      <li>サイト名: {process.env.NEXT_PUBLIC_SITE_NAME}</li>
      <li>秘密のキー: {String(process.env.SECRET_API_KEY)}</li>
    </ul>
  );
}

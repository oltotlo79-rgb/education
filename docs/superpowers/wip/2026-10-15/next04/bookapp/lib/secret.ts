// lib/secret.ts（サーバー専用の処理をまとめるファイル）
import "server-only";

export function getSecretKey() {
  return process.env.SECRET_API_KEY ?? "";
}

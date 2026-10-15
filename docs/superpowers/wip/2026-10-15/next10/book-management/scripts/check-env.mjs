// scripts/check-env.mjs
// ビルドの前に「必要な環境変数がそろっているか」を確かめるスクリプト

// アプリが動くために必要な環境変数の名前
const required = [
  "NEXT_PUBLIC_SUPABASE_URL",
  "NEXT_PUBLIC_SUPABASE_ANON_KEY",
];

// 手元では .env.local を読み込む（Vercel 上にはこのファイルが無い）
try {
  process.loadEnvFile(".env.local");
} catch {
  // ファイルが無ければ何もしない（Vercel では値が最初から入っている）
}

// 値が入っていない名前だけを集める
const missing = required.filter((name) => !process.env[name]);

if (missing.length > 0) {
  console.error("✖ 次の環境変数が設定されていません:");
  for (const name of missing) {
    console.error(`  - ${name}`);
  }
  console.error("→ 手元なら .env.local、Vercel なら Environment Variables を確認してください");
  // 終了コード 1 は「失敗」の合図。npm はここで止まり、ビルドに進まない
  process.exit(1);
}

// URL のよくある書き間違い（末尾の / や http:）もついでに確かめる
const url = process.env.NEXT_PUBLIC_SUPABASE_URL;
if (!url.startsWith("https://") || url.endsWith("/")) {
  console.error(`✖ NEXT_PUBLIC_SUPABASE_URL の形が正しくありません: ${url}`);
  console.error("→ https://xxxx.supabase.co の形（最後に / を付けない）にしてください");
  process.exit(1);
}

console.log(`✔ 必要な環境変数 ${required.length} 個はすべて設定されています`);

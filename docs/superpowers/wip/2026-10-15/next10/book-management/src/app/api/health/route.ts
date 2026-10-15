// /api/health にアクセスすると「いま動いているアプリの状態」を JSON で返す
export function GET() {
  return Response.json({
    // ここまで処理が来れば、アプリ自体は動いている
    status: "ok",
    // Vercel 上なら production / preview、手元なら local
    environment: process.env.VERCEL_ENV ?? "local",
    // どのコミットから作られたか（先頭7文字）。手元なら local
    commit: process.env.VERCEL_GIT_COMMIT_SHA?.slice(0, 7) ?? "local",
    // 環境変数が「登録されているか」だけを true / false で返す（値そのものは返さない）
    supabaseUrlConfigured: Boolean(process.env.NEXT_PUBLIC_SUPABASE_URL),
    supabaseKeyConfigured: Boolean(process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY),
    // このレスポンスを作った時刻
    checkedAt: new Date().toISOString(),
  });
}

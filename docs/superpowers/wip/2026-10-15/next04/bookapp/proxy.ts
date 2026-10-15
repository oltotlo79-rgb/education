// proxy.ts（プロジェクト直下。app/ と同じ階層に1つだけ置く）
import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";

export function proxy(request: NextRequest) {
  // Cookie から「ログインの証（トークン）」を読む
  const token = request.cookies.get("token");

  // ログインの証が無い＝未ログインなら、ログイン画面へ送り返す
  if (!token) {
    const loginUrl = new URL("/login", request.url);
    return NextResponse.redirect(loginUrl);
  }

  // ログイン済みなら、そのまま目的のページへ通す
  return NextResponse.next();
}

export const config = {
  // /mypage と、その下のすべてのページにだけ適用する
  matcher: ["/mypage/:path*"],
};

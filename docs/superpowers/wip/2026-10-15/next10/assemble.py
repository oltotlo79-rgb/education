"""Assemble next/10-deployment.md from the original + blocks/ + figs/.

Usage: python assemble.py <src.md> <dst.md>
Every anchor must occur exactly once; otherwise the script aborts.
"""
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
BLOCKS = HERE / "blocks"
FIGS = HERE / "figs"

src, dst = Path(sys.argv[1]), Path(sys.argv[2])
raw = src.read_bytes().decode("utf-8")
text = raw.replace("\r\n", "\n")


def fig(match: re.Match) -> str:
    return (FIGS / f"{match.group(1)}.html").read_text(encoding="utf-8").replace("\r\n", "\n").strip("\n")


def block(name: str) -> str:
    body = (BLOCKS / f"{name}.md").read_text(encoding="utf-8").replace("\r\n", "\n")
    body = re.sub(r"\{\{FIG:(f\d\d)\}\}", fig, body)
    return body.strip("\n")


def once(sub: str) -> int:
    n = text.count(sub)
    if n != 1:
        raise SystemExit(f"anchor count {n} != 1: {sub[:70]!r}")
    return text.index(sub)


def replace(old: str, new: str) -> None:
    global text
    once(old)
    text = text.replace(old, new)


def after_line(sub: str, body: str) -> None:
    global text
    pos = once(sub)
    end = text.index("\n", pos)
    tail = text[end:]
    if tail.startswith("\n\n"):
        text = text[:end] + "\n\n" + body + tail
    else:
        text = text[:end] + "\n\n" + body + "\n" + tail


def before_line(sub: str, body: str) -> None:
    global text
    pos = once(sub)
    start = text.rindex("\n", 0, pos) + 1
    text = text[:start] + body + "\n\n" + text[start:]


def after_fence(sub: str, body: str) -> None:
    global text
    pos = once(sub)
    fence = text.index("```\n", pos)
    end = fence + len("```")
    text = text[:end] + "\n\n" + body + text[end:]


# --- 章冒頭・目次 ---
after_line("> **Edge Function（エッジ関数）とは？**", block("b01_goal"))
after_line("9. [おわりに](#9-おわりに)",
           "> **ヒント:** 目次の前に「この章のゴールと地図」があります。また、「8. 付録」と「9. おわりに」の間に「練習問題（解答・解説つき）」があります。")

# --- 0.1 ---
replace("**▼ 期待される出力（抜粋）:**", "**▼ 期待される出力（抜粋。Next.js 15 での表示例）:**")
replace(
    "┌ ○ /                                    1.32 kB        92.5 kB\n"
    "├ ○ /books                               2.45 kB        93.6 kB\n"
    "├ λ /books/[id]                          1.78 kB        92.9 kB\n"
    "└ λ /books/[id]/edit                     2.10 kB        93.2 kB",
    "┌ ƒ /                                    1.32 kB        92.5 kB\n"
    "├ ○ /books/new                           2.45 kB        93.6 kB\n"
    "├ ƒ /books/[id]                          1.78 kB        92.9 kB\n"
    "└ ƒ /books/[id]/edit                     2.10 kB        93.2 kB",
)
replace("λ  (Dynamic)  server-rendered on demand", "ƒ  (Dynamic)  server-rendered on demand")
replace("`λ` はリクエストごとにサーバーで生成されるページ",
        "`ƒ` はリクエストごとにサーバーで生成されるページ（古い Next.js では `λ` と表示されていました）")
after_line("- `First Load JS` は「最初にユーザーのブラウザがダウンロードする JavaScript の量」", block("b02_build_fig"))

# --- 0.2 ---
after_fence("あなたのアプリ: https://your-app.vercel.app", block("b03_flow_fig"))

# --- 0.3 ---
replace("| `NEXT_PUBLIC_SUPABASE_ANON_KEY` | Supabase の匿名キー | 公開可 |",
        "| `NEXT_PUBLIC_SUPABASE_ANON_KEY` | Supabase の公開用キー（値は Publishable key `sb_publishable_...`。旧 anon key でも動く） | 公開可 |")
replace("| `SUPABASE_SERVICE_ROLE_KEY` | Supabaseの管理者キー | **絶対秘密**（サーバー専用） |",
        "| `SUPABASE_SERVICE_ROLE_KEY` | Supabase の管理者キー（Secret key `sb_secret_...`。旧 service_role key） | **絶対秘密**（サーバー専用。本書のアプリでは使わない） |")
replace(
    "NEXT_PUBLIC_SUPABASE_URL=https://xxxxx.supabase.co\n"
    "# ↑ NEXT_PUBLIC_ プレフィックス付きなのでブラウザにも値が送られる。URL は公開しても問題ない情報\n"
    "\n"
    "NEXT_PUBLIC_SUPABASE_ANON_KEY=eyJhbGc...\n"
    "# ↑ 匿名キー。これも公開前提のキー。ただし RLS（Row Level Security）でデータを保護することが必須",
    "# NEXT_PUBLIC_ プレフィックス付きなのでブラウザにも値が送られる。URL は公開しても問題ない情報\n"
    "NEXT_PUBLIC_SUPABASE_URL=https://xxxxx.supabase.co\n"
    "\n"
    "# 公開用キー（Publishable key）。変数名は ANON_KEY のまま、値に sb_publishable_ で始まるキーを入れる（第5章）\n"
    "# 公開前提のキーなので、RLS（Row Level Security）でデータを守ることが必須\n"
    "NEXT_PUBLIC_SUPABASE_ANON_KEY=sb_publishable_xxxxxxxxxxxxxxxx",
)
replace("**本番（Vercel）**: Vercelダッシュボードの **Settings → Environment Variables** で同じ値を設定する。",
        "**本番（Vercel）**: Vercel ダッシュボードでプロジェクトを開き、**Settings → Environment Variables** で同じ値を設定する（手順は 2.5 と 2.7）。\n\n"
        + block("b04_env_fig"))
after_line("> **`NEXT_PUBLIC_` の意味:**", block("b05_env_exp1"))
after_line("> **環境変数を変更したら必ず「再デプロイ」が必要な理由:**", block("b06_env_exp2"))

# --- 1.2 ---
start = once("ビルドが成功すると、以下のような出力が表示されます。")
endmark = "prerendered as static content\n```"
end = text.index(endmark, start) + len(endmark)
assert end - start < 1200, end - start
text = text[:start] + block("b07_build_output") + text[end:]
after_line("> 開発中のコード（TypeScript、JSX など）をブラウザが直接理解できる形式", block("b08_build_read"))

# --- 1.3 ---
after_line("3. **エラーの詳細メッセージ**: 何が問題なのか具体的に書かれている部分", block("b09_read_fig"))
before_line("#### エラー2: import エラー", block("b10_err1"))
before_line("#### エラー3: 環境変数が undefined", block("b11_err2"))
replace("# NEXT_PUBLIC_SUPABASE_ANON_KEY=eyJhbGciOiJIUzI1NiIs...\n",
        "# NEXT_PUBLIC_SUPABASE_ANON_KEY=sb_publishable_...（旧 anon key なら eyJhbGciOi... で始まる長い文字列）\n")
before_line("#### エラー4: 'use client' ディレクティブの不足", block("b12_err3"))
before_line("#### エラー5: ESLint エラー", block("b13_err4"))
before_line("> **その他、本番でハマる典型エラー:**", block("b14_err5_6"))

# --- 1.4 / 1.5 ---
replace("| `NEXT_PUBLIC_SUPABASE_ANON_KEY` | Supabase の匿名キー | `eyJhbGciOiJIUzI1NiIs...` |",
        "| `NEXT_PUBLIC_SUPABASE_ANON_KEY` | Supabase の公開用キー（Publishable key） | `sb_publishable_...`（旧 anon key なら `eyJhbGciOi...`） |")
before_line("### 1.5 ビルド前チェックリスト", block("b15_env_check"))
replace("- [ ] `npm run lint` がエラーなく成功する",
        "- [ ] `npm run lint` がエラーなく成功する（Next.js 16 では、ビルドとは別に実行が必要）")
replace("- [ ] コンソールにエラーが出ていない",
        "- [ ] コンソールにエラーが出ていない\n"
        "- [ ] `npm run start` の本番モードで、全ページを開いて確かめた（1.2.1）\n"
        "- [ ] `next.config.ts` に `cacheComponents: true` が入っていない（1.2.2）\n"
        "- [ ] `git status` や `git ls-files .env.local` に `.env.local` が出てこない（1.4.2）")

# --- 2.5 ---
replace("2. 「Add New...」→「Project」をクリック",
        "2. 「Add New...」→「Project」をクリック（画面によっては、ダッシュボード右上の「New Project」ボタン）")
after_line("4. 「Import」をクリック", block("b16_step1_fig"))
after_line("> - **Output Directory**（出力ディレクトリ）:", block("b17_step2_fig"))
replace("> **値の確認方法**: Supabase ダッシュボード → Settings → API で確認できます。",
        "> **値の確認方法**: Supabase ダッシュボードでプロジェクトを開き、画面上部の **Connect** ボタンから Project URL と公開用キー（Publishable key）をコピーするのがいちばん簡単です。設定画面から探す場合は、**Project Settings** の **Data API**（Project URL）と **API Keys**（Publishable key）にあります（第5章）。")
replace("| `NEXT_PUBLIC_SUPABASE_ANON_KEY` | `eyJhbGciOiJIUzI1NiIs...（あなたの匿名キー）` |",
        "| `NEXT_PUBLIC_SUPABASE_ANON_KEY` | `sb_publishable_...（あなたの Publishable key）` |")
replace("3. 「Add」ボタンで各変数を追加", block("b18_step3"))
after_line("4. 表示された URL（例: `https://book-management-app.vercel.app`）をクリックしてアプリを確認", block("b19_step4_fig"))

# --- 2.6 / 2.7 / 2.8 ---
before_line("### 2.7 環境変数の追加・変更方法（デプロイ後）", block("b20_timeline_preview"))
replace("1. Vercel ダッシュボードでプロジェクトを選択\n"
        "2. 「Settings」タブ → 「Environment Variables」\n"
        "3. 変数を追加・編集・削除\n"
        "4. **重要**: 環境変数を変更した後は、「Deployments」タブから最新のデプロイを「Redeploy」する必要があります",
        block("b21_envvars"))
replace("お名前.com、Google Domains など", "お名前.com、Cloudflare、Squarespace Domains（旧 Google Domains）など")

# --- 2.9 / 2.10 ---
replace("- [ ] HTTPS（鍵アイコン）で接続されている",
        "- [ ] HTTPS（鍵アイコン）で接続されている\n"
        "- [ ] `/api/health` が `\"environment\":\"production\"` と `true` を返す（2.9.1）")
replace("> **トラブルシューティング**: 問題が発生した場合は、Vercel ダッシュボードの「Deployments」→ 該当デプロイ →「Logs」でビルドログとランタイムログを確認できます。",
        "> **トラブルシューティング**: 問題が発生した場合は、Vercel ダッシュボードのサイドバーの「Deployments」から該当のデプロイを開くと、Deployment Details の「Building」の欄でビルドログを確認できます。ランタイムログ（アクセスされたときのログ）は、サイドバーの「Logs」で見られます。")
after_line("> **トラブルシューティング**:", block("b22_health_trouble_rollback"))

# --- 3 ---
replace("Supabase には2種類のキーがあります。",
        "Supabase には2種類のキーがあります。2025年から新しい形式の **Publishable key**（`sb_publishable_...`）と **Secret key**（`sb_secret_...`）が導入され、従来の anon キー／service_role キーは2026年末に廃止予定の旧方式になりました（第5章）。役割は同じです。")
replace("| `anon` キー（匿名キー） | クライアントサイドからのアクセス | はい（RLS で保護） |",
        "| Publishable key（旧 `anon` キー） | クライアントサイドからのアクセス | はい（RLS で保護） |")
replace("| `service_role` キー | サーバーサイドからの管理アクセス | **絶対に公開してはいけない** |",
        "| Secret key（旧 `service_role` キー） | サーバーサイドからの管理アクセス | **絶対に公開してはいけない** |")
after_fence("SUPABASE_SERVICE_ROLE_KEY       → サーバーサイドのみで使用（絶対に公開しない）", block("b23_supabase"))
replace("`service_role` キーには絶対に `NEXT_PUBLIC_` を付けないでください。",
        "Secret key（旧 `service_role` キー）には絶対に `NEXT_PUBLIC_` を付けないでください。")
after_line("> **重要**: `NEXT_PUBLIC_` プレフィックスが付いた環境変数は", block("b24_secret_ngok"))
replace("3. **Redirect URLs** にログイン後にリダイレクトされる URL を追加する（プレビュー環境用に `https://*.vercel.app` のようなワイルドカードも登録可能）",
        "3. **Redirect URLs** に、ログイン後に戻ってきてよい URL を追加する（下の例を参照）")
after_line("> **なぜ必要？:** Supabase Auth は", block("b25_redirect"))
replace("\n---\n\n## 4. パフォーマンス最適化", "\n" + block("b26_pause") + "\n\n---\n\n## 4. パフォーマンス最適化")

# --- 4.2 ---
start = once("```typescript\n// src/app/books/[id]/page.tsx\nimport type { Metadata } from 'next';")
endmark = "description: `${book?.title}（${book?.author}）の詳細情報`,\n  };\n}\n```"
end = text.index(endmark, start) + len(endmark)
assert end - start < 2000, end - start
text = text[:start] + block("b27_metadata") + text[end:]

# --- 5.1 / 練習問題 ---
replace("- [x] デプロイ後の動作確認",
        "- [x] デプロイ後の動作確認\n"
        "- [x] ビルドエラーの読み方と直し方\n"
        "- [x] Preview と Production の違い\n"
        "- [x] ロールバック（前のバージョンに戻す）")
replace("\n---\n\n## 9. おわりに", "\n" + block("b28_practice") + "\n\n---\n\n## 9. おわりに")

if "{{FIG" in text:
    raise SystemExit("unreplaced figure placeholder")
out = text.replace("\n", "\r\n") if "\r\n" in raw else text
dst.write_bytes(out.encode("utf-8"))
print("written", dst, "lines", text.count("\n") + 1)

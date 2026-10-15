import sys, re
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
start = s.index("  <!-- 第1章 -->")
end_marker = "</div>\n\n各章の目安学習時間は"
end = s.index(end_marker)
BLUE = ("#e3f2fd", "#2196f3", "#0d47a1", "#1565c0")
ORANGE = ("#fff3e0", "#ff9800", "#e65100", "#ef6c00")
PINK = ("#fce4ec", "#e91e63", "#880e4f", "#ad1457")
PURPLE = ("#f3e5f5", "#9c27b0", "#4a148c", "#6a1b9a")
YELLOW = ("#fffde7", "#fdd835", "#f57f17", "#f9a825")
GRAY = ("#eceff1", "#607d8b", "#263238", "#455a64")
items = [
    ("第1章", "🔧", BLUE, "第1章: 開発環境のセットアップ", "Node.js / VS Code / Git / ターミナルを準備する"),
    ("第2章", "📝", BLUE, "第2章: TypeScript の基礎", "型の基本を学ぶ"),
    ("第3章", "⚛️", BLUE, "第3章: React の基礎", "コンポーネント・State・イベントの考え方"),
    ("第4章", "🚀", BLUE, "第4章: Next.js の基礎", "App Router・Server / Client Components"),
    ("第5章", "🗄️", ORANGE, "第5章: Supabase セットアップとデータベース設計", "データベースを準備する"),
    ("第6章", "🏗️", ORANGE, "第6章: プロジェクトセットアップ", "書籍管理アプリの土台を作る"),
    ("第7章", "📋", PINK, "第7章: 一覧表示と登録機能", "CRUD の Read と Create"),
    ("第8章", "✏️", PINK, "第8章: 編集・削除・詳細表示", "CRUD の Update と Delete、検索・並び替え"),
    ("第9章", "🎨", PURPLE, "第9章: スタイリング & UI ポリッシュ", "見た目と使いやすさを仕上げる"),
    ("第10章", "🌐", PURPLE, "第10章: デプロイと総まとめ", "アプリをインターネットに公開する"),
    ("完成", "🎉", YELLOW, "完成！", "書籍管理アプリの完成です。おめでとうございます！"),
    ("第11章", "🤖", GRAY, "第11章: Claude Code × Next.js バイブコーディング（発展）", "AI と一緒に、さらに機能を足していく"),
]
out = []
for i, (cmt, emoji, (bg, border, tc, sc), title, sub) in enumerate(items):
    last = i == len(items) - 1
    mb = "" if last else " margin-bottom: 4px;"
    out.append(f"  <!-- {cmt} -->\n")
    out.append(f'  <div style="display: flex; align-items: center;{mb}">\n')
    out.append(f'    <div style="width: 40px; text-align: center; font-size: 20px;">{emoji}</div>\n')
    out.append(f'    <div style="flex: 1; background: {bg}; border-left: 4px solid {border}; border-radius: 0 8px 8px 0; padding: 12px 16px; box-shadow: 0 2px 12px rgba(0,0,0,0.08);">\n')
    out.append(f'      <div style="font-weight: 700; color: {tc}; font-size: 14px;">{title}</div>\n')
    out.append(f'      <div style="font-size: 12px; color: {sc};">{sub}</div>\n')
    out.append("    </div>\n")
    out.append("  </div>\n")
    if not last:
        out.append('  <div style="text-align: center; color: #94a3b8; font-size: 18px; margin: 2px 0;">↓</div>\n')
new = s[:start] + "".join(out) + s[end:]
open(p, "w", encoding="utf-8", newline="\n").write(new)
print("ok", len(s), len(new))

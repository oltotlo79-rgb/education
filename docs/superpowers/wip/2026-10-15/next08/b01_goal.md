### この章のゴールと地図

この章を読み終えると、次の4つができるようになります。

1. **詳細表示**: URL の一部（`/books/ここ`）から本の id を取り出し、その本 **1冊だけ** をデータベースから取ってきて表示できる
2. **編集**: 今の内容が最初から入っている編集フォームを作り、「更新する」で保存して詳細ページに戻れる
3. **削除**: 確認ダイアログを出してから、その本 **1冊だけ** を安全に消せる
4. **失敗の見分け**: 「存在しない id」「条件の付け忘れ」「エラーは出ないのに反映されない」といった失敗の原因を、自分で切り分けられる

> **たとえ話:** 一覧ページは「本棚」、詳細ページは本に挟まった「貸出カード」だと考えてください。URL の id は **カードの番号** です。番号を伝えれば、司書さん（Supabase）がそのカードを1枚だけ出してくれます。編集は「カードの書き直し」、削除は「カードの廃棄」です。廃棄はやり直せないので、司書さんは「本当に捨てますか？」と必ず確認します。この章で作るのは、まさにこの3つの窓口です。

下の図は、この章で作る画面のつながりと、それぞれを説明している節の番号です。迷ったらこの地図に戻ってきてください。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-01" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="22" text-anchor="middle" font-size="14" font-weight="700" fill="#475569">この章で作る画面のつながり</text>
<rect x="20" y="40" width="170" height="74" rx="10" fill="#f8fafc" stroke="#94a3b8" stroke-width="2" stroke-dasharray="6 4"/>
<text x="105" y="66" text-anchor="middle" font-size="15" font-weight="700" fill="#475569">一覧ページ</text>
<text x="105" y="86" text-anchor="middle" font-size="12" fill="#64748b" font-family="Consolas, monospace">/books</text>
<text x="105" y="104" text-anchor="middle" font-size="12" fill="#64748b">一覧表示は第7章</text>
<rect x="275" y="40" width="170" height="74" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="66" text-anchor="middle" font-size="15" font-weight="700" fill="#1e40af">詳細ページ</text>
<text x="360" y="86" text-anchor="middle" font-size="12" fill="#1e40af" font-family="Consolas, monospace">/books/[id]</text>
<text x="360" y="104" text-anchor="middle" font-size="12" fill="#1e40af">§1 詳細表示（Read）</text>
<rect x="530" y="40" width="170" height="74" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="615" y="66" text-anchor="middle" font-size="15" font-weight="700" fill="#047857">編集ページ</text>
<text x="615" y="86" text-anchor="middle" font-size="12" fill="#047857" font-family="Consolas, monospace">/books/[id]/edit</text>
<text x="615" y="104" text-anchor="middle" font-size="12" fill="#047857">§2 編集（Update）</text>
<rect x="275" y="180" width="170" height="74" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="206" text-anchor="middle" font-size="15" font-weight="700" fill="#c2410c">削除ボタン</text>
<text x="360" y="226" text-anchor="middle" font-size="12" fill="#c2410c">確認ダイアログ → 削除</text>
<text x="360" y="244" text-anchor="middle" font-size="12" fill="#c2410c">§3 削除（Delete）</text>
<line x1="190" y1="66" x2="271" y2="66" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-01)"/>
<text x="232" y="58" text-anchor="middle" font-size="12" fill="#475569">クリック</text>
<line x1="445" y1="66" x2="526" y2="66" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-01)"/>
<text x="487" y="58" text-anchor="middle" font-size="12" fill="#475569">編集する</text>
<line x1="530" y1="98" x2="449" y2="98" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-01)"/>
<text x="489" y="116" text-anchor="middle" font-size="12" fill="#475569">保存</text>
<line x1="360" y1="114" x2="360" y2="176" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-01)"/>
<text x="368" y="150" font-size="12" fill="#475569">削除する</text>
<path d="M275,217 L105,217 L105,118" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-01)"/>
<text x="190" y="209" text-anchor="middle" font-size="12" fill="#475569">削除後は一覧へ</text>
<rect x="20" y="266" width="160" height="28" rx="6" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="100" y="285" text-anchor="middle" font-size="12" fill="#475569">§4・5 検索とソート</text>
<rect x="195" y="266" width="160" height="28" rx="6" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="275" y="285" text-anchor="middle" font-size="12" fill="#475569">§6 動作確認</text>
<rect x="370" y="266" width="160" height="28" rx="6" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="450" y="285" text-anchor="middle" font-size="12" fill="#475569">§7 トラブル対応</text>
<rect x="545" y="266" width="155" height="28" rx="6" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="622" y="285" text-anchor="middle" font-size="12" fill="#475569">§9 練習問題</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-1: この章の地図（点線の一覧ページは第7章で作った一覧表示。§ は節の番号）</div>
</div>

**前提として必要なもの:** 第7章までの書籍管理アプリ（書籍の一覧表示と、新規登録ページ `/books/new` が動いていること）と、Supabase の `books` テーブルに本が数冊登録されていること。

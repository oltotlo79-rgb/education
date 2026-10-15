実際の画面では、詳細ページの上に次のようなダイアログが重なって表示されます。ダイアログが出ている間は、後ろのページを操作できません。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 340" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="700" height="320" rx="10" fill="#ffffff" stroke="#cbd5e1" stroke-width="2"/>
<path d="M10,46 L10,20 Q10,10 20,10 L700,10 Q710,10 710,20 L710,46 Z" fill="#f1f5f9"/>
<line x1="10" y1="46" x2="710" y2="46" stroke="#cbd5e1"/>
<circle cx="32" cy="28" r="5" fill="#f87171"/>
<circle cx="50" cy="28" r="5" fill="#fbbf24"/>
<circle cx="68" cy="28" r="5" fill="#34d399"/>
<rect x="90" y="18" width="600" height="20" rx="10" fill="#ffffff" stroke="#e2e8f0"/>
<text x="104" y="33" font-size="12" fill="#475569" font-family="Consolas, monospace">localhost:3000/books/7c9e6679-7425-40de-944b-e07fc1f90ae7</text>
<rect x="40" y="66" width="640" height="56" rx="6" fill="#1e40af"/>
<text x="60" y="101" font-size="18" font-weight="700" fill="#ffffff">TypeScript入門</text>
<text x="60" y="152" font-size="13" fill="#475569">出版社 未設定</text>
<text x="60" y="180" font-size="13" fill="#475569">評価 ★★★☆☆</text>
<text x="60" y="208" font-size="13" fill="#475569">ステータス 未読</text>
<rect x="556" y="222" width="60" height="30" rx="5" fill="#3b82f6"/>
<text x="586" y="242" text-anchor="middle" font-size="12" fill="#ffffff">編集する</text>
<rect x="622" y="222" width="60" height="30" rx="5" fill="#dc2626"/>
<text x="652" y="242" text-anchor="middle" font-size="12" fill="#ffffff">削除する</text>
<rect x="11" y="47" width="698" height="282" fill="#0f172a" fill-opacity="0.35"/>
<rect x="180" y="96" width="360" height="160" rx="10" fill="#ffffff" stroke="#94a3b8"/>
<text x="202" y="126" font-size="13" font-weight="700" fill="#1e293b">localhost:3000 の内容</text>
<text x="202" y="160" font-size="13" fill="#1e293b">本当に「TypeScript入門」を削除しますか？</text>
<text x="202" y="182" font-size="13" fill="#1e293b">この操作は取り消せません。</text>
<rect x="340" y="208" width="84" height="30" rx="15" fill="#2563eb"/>
<text x="382" y="228" text-anchor="middle" font-size="13" font-weight="700" fill="#ffffff">OK</text>
<rect x="434" y="208" width="90" height="30" rx="15" fill="#ffffff" stroke="#94a3b8"/>
<text x="479" y="228" text-anchor="middle" font-size="13" fill="#1e40af">キャンセル</text>
<rect x="130" y="268" width="460" height="34" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="360" y="290" text-anchor="middle" font-size="12" fill="#c2410c">OK → confirm() が true（削除へ進む）／ キャンセル → false（何もしない）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-10: 画面イメージ ― 削除ボタンを押した直後の確認ダイアログ（見た目はブラウザによって少し異なる）</div>
</div>

- 1行目の「localhost:3000 の内容」はブラウザが自動で付ける見出しで、プログラムからは変えられません。
- 2〜3行目が、`window.confirm()` に渡した文字列です。`\n` を入れた位置で改行されています。
- 「OK」を押すと `confirm()` は `true` を、「キャンセル」を押すと `false` を返します。この戻り値で、削除に進むかどうかを決めます。

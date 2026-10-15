同じ流れを **ユーザーが見る画面** で並べると、「詳細 → 編集 → 保存 → 詳細」とぐるっと一周する形になります。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-05" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="105" y="20" text-anchor="middle" font-size="13" font-weight="700" fill="#1e40af">① 詳細ページ</text>
<text x="360" y="20" text-anchor="middle" font-size="13" font-weight="700" fill="#047857">② 編集ページ（既存値入り）</text>
<text x="615" y="20" text-anchor="middle" font-size="13" font-weight="700" fill="#1e40af">③ 詳細ページ（更新後）</text>
<rect x="10" y="30" width="190" height="180" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-width="2"/>
<rect x="11" y="31" width="188" height="22" rx="7" fill="#f1f5f9"/>
<text x="20" y="47" font-size="12" fill="#475569" font-family="Consolas, monospace">/books/abc</text>
<rect x="20" y="62" width="170" height="30" rx="4" fill="#1e40af"/>
<text x="28" y="82" font-size="13" font-weight="700" fill="#ffffff">React実践ガイド</text>
<text x="24" y="114" font-size="12" fill="#475569">評価 ★★★★☆</text>
<text x="24" y="134" font-size="12" fill="#475569">状態 読書中</text>
<rect x="22" y="170" width="76" height="26" rx="4" fill="#3b82f6"/>
<text x="60" y="188" text-anchor="middle" font-size="12" fill="#ffffff">編集する</text>
<rect x="110" y="170" width="76" height="26" rx="4" fill="#fee2e2"/>
<text x="148" y="188" text-anchor="middle" font-size="12" fill="#dc2626">削除する</text>
<rect x="265" y="30" width="190" height="180" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-width="2"/>
<rect x="266" y="31" width="188" height="22" rx="7" fill="#f1f5f9"/>
<text x="275" y="47" font-size="12" fill="#475569" font-family="Consolas, monospace">/books/abc/edit</text>
<text x="277" y="76" font-size="13" font-weight="700" fill="#1e293b">書籍を編集</text>
<rect x="277" y="86" width="166" height="22" rx="3" fill="#ffffff" stroke="#cbd5e1"/>
<text x="284" y="102" font-size="12" fill="#1e293b">React実践ガイド</text>
<rect x="277" y="114" width="166" height="22" rx="3" fill="#fff7ed" stroke="#f97316"/>
<text x="284" y="130" font-size="12" fill="#c2410c">★★★★★（5）</text>
<rect x="277" y="142" width="166" height="22" rx="3" fill="#fff7ed" stroke="#f97316"/>
<text x="284" y="158" font-size="12" fill="#c2410c">読了</text>
<rect x="277" y="174" width="80" height="26" rx="4" fill="#ffffff" stroke="#cbd5e1"/>
<text x="317" y="192" text-anchor="middle" font-size="12" fill="#475569">キャンセル</text>
<rect x="367" y="174" width="76" height="26" rx="4" fill="#3b82f6"/>
<text x="405" y="192" text-anchor="middle" font-size="12" fill="#ffffff">更新する</text>
<rect x="520" y="30" width="190" height="180" rx="8" fill="#ffffff" stroke="#cbd5e1" stroke-width="2"/>
<rect x="521" y="31" width="188" height="22" rx="7" fill="#f1f5f9"/>
<text x="530" y="47" font-size="12" fill="#475569" font-family="Consolas, monospace">/books/abc</text>
<rect x="530" y="62" width="170" height="30" rx="4" fill="#1e40af"/>
<text x="538" y="82" font-size="13" font-weight="700" fill="#ffffff">React実践ガイド</text>
<rect x="530" y="100" width="170" height="44" rx="4" fill="#ecfdf5"/>
<text x="534" y="118" font-size="12" font-weight="700" fill="#047857">評価 ★★★★★</text>
<text x="534" y="138" font-size="12" font-weight="700" fill="#047857">状態 読了</text>
<rect x="532" y="170" width="76" height="26" rx="4" fill="#3b82f6"/>
<text x="570" y="188" text-anchor="middle" font-size="12" fill="#ffffff">編集する</text>
<rect x="620" y="170" width="76" height="26" rx="4" fill="#fee2e2"/>
<text x="658" y="188" text-anchor="middle" font-size="12" fill="#dc2626">削除する</text>
<line x1="200" y1="110" x2="261" y2="110" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-05)"/>
<text x="232" y="100" text-anchor="middle" font-size="12" fill="#475569">編集する</text>
<line x1="455" y1="110" x2="516" y2="110" stroke="#475569" stroke-width="2" marker-end="url(#ar-08-05)"/>
<text x="487" y="100" text-anchor="middle" font-size="12" fill="#475569">更新する</text>
<text x="487" y="128" text-anchor="middle" font-size="12" fill="#475569" font-family="Consolas, monospace">UPDATE</text>
<path d="M317,212 C317,252 105,252 105,214" fill="none" stroke="#94a3b8" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-08-05)"/>
<text x="211" y="268" text-anchor="middle" font-size="12" fill="#64748b">キャンセル（router.back）</text>
<text x="615" y="232" text-anchor="middle" font-size="12" fill="#475569" font-family="Consolas, monospace">router.push → refresh</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-5: 詳細 → 編集 → 保存 → 詳細 の画面遷移（オレンジは書き換えた欄、緑は更新後に変わった表示）</div>
</div>

- **① → ②**: 詳細ページの「編集する」は、ただのリンク（``<Link href={`/books/${book.id}/edit`}>``）です。URL が `/books/abc/edit` に変わります。
- **②**: 編集ページはサーバー側で本を1冊取ってきて、**今の値を入れた状態のフォーム** を表示します。ユーザーは変えたい欄（図のオレンジ）だけを書き換えます。
- **② → ③**: 「更新する」を押すと UPDATE が実行され、成功したら ``router.push(`/books/${bookId}`)`` で詳細ページに戻ります。続けて `router.refresh()` を呼ぶので、詳細ページは最新の内容（図の緑）で表示されます。
- **キャンセル**: `router.back()` でブラウザの「戻る」と同じ動きをするので、何も保存せずに①へ戻ります。

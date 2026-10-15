完成した編集ページを開くと、次のように **今の値が最初から入った状態** のフォームが表示されます（6-3 のテストで登録した「React実践ガイド」を開いた例）。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 420" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-08-06" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#f97316"/></marker>
</defs>
<rect x="10" y="10" width="700" height="400" rx="10" fill="#ffffff" stroke="#cbd5e1" stroke-width="2"/>
<path d="M10,46 L10,20 Q10,10 20,10 L700,10 Q710,10 710,20 L710,46 Z" fill="#f1f5f9"/>
<line x1="10" y1="46" x2="710" y2="46" stroke="#cbd5e1"/>
<circle cx="32" cy="28" r="5" fill="#f87171"/>
<circle cx="50" cy="28" r="5" fill="#fbbf24"/>
<circle cx="68" cy="28" r="5" fill="#34d399"/>
<rect x="90" y="18" width="600" height="20" rx="10" fill="#ffffff" stroke="#e2e8f0"/>
<text x="104" y="33" font-size="12" fill="#475569" font-family="Consolas, monospace">localhost:3000/books/3f2a8c1e-5b7d-4e9a-9c01-2b3d4e5f6a7b/edit</text>
<text x="40" y="72" font-size="12" fill="#2563eb">← 詳細に戻る</text>
<text x="40" y="102" font-size="20" font-weight="700" fill="#1e293b">書籍を編集</text>
<text x="40" y="124" font-size="12" fill="#64748b">「React実践ガイド」の情報を編集します。</text>
<rect x="30" y="136" width="500" height="262" rx="8" fill="#ffffff" stroke="#e2e8f0" stroke-width="2"/>
<text x="46" y="158" font-size="12" fill="#374151">タイトル <tspan fill="#ef4444">*</tspan></text>
<rect x="46" y="164" width="230" height="26" rx="4" fill="#ffffff" stroke="#cbd5e1"/>
<text x="54" y="182" font-size="12" fill="#1e293b">React実践ガイド</text>
<text x="290" y="158" font-size="12" fill="#374151">著者</text>
<rect x="290" y="164" width="226" height="26" rx="4" fill="#ffffff" stroke="#cbd5e1"/>
<text x="298" y="182" font-size="12" fill="#1e293b">山田太郎</text>
<text x="46" y="210" font-size="12" fill="#374151">出版社</text>
<rect x="46" y="216" width="230" height="26" rx="4" fill="#ffffff" stroke="#cbd5e1"/>
<text x="54" y="234" font-size="12" fill="#1e293b">技術評論社</text>
<text x="290" y="210" font-size="12" fill="#374151">出版日</text>
<rect x="290" y="216" width="226" height="26" rx="4" fill="#ffffff" stroke="#cbd5e1"/>
<text x="298" y="234" font-size="12" fill="#1e293b">2024/01/15</text>
<text x="46" y="262" font-size="12" fill="#374151">評価</text>
<rect x="46" y="268" width="230" height="26" rx="4" fill="#ffffff" stroke="#cbd5e1"/>
<text x="54" y="286" font-size="12" fill="#1e293b">★★★★☆（4）</text>
<text x="264" y="286" text-anchor="end" font-size="12" fill="#64748b">▼</text>
<text x="290" y="262" font-size="12" fill="#374151">ステータス</text>
<rect x="290" y="268" width="226" height="26" rx="4" fill="#ffffff" stroke="#cbd5e1"/>
<text x="298" y="286" font-size="12" fill="#1e293b">読書中</text>
<text x="504" y="286" text-anchor="end" font-size="12" fill="#64748b">▼</text>
<text x="46" y="314" font-size="12" fill="#374151">メモ</text>
<rect x="46" y="320" width="470" height="30" rx="4" fill="#ffffff" stroke="#cbd5e1"/>
<text x="54" y="340" font-size="12" fill="#1e293b">コンポーネント設計の章が特に参考になった</text>
<rect x="330" y="360" width="88" height="28" rx="6" fill="#ffffff" stroke="#cbd5e1"/>
<text x="374" y="379" text-anchor="middle" font-size="12" fill="#374151">キャンセル</text>
<rect x="428" y="360" width="88" height="28" rx="6" fill="#2563eb"/>
<text x="472" y="379" text-anchor="middle" font-size="12" font-weight="700" fill="#ffffff">更新する</text>
<rect x="548" y="150" width="156" height="62" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="558" y="170" font-size="12" fill="#c2410c">DB の今の値が</text>
<text x="558" y="187" font-size="12" fill="#c2410c">最初から入っている</text>
<text x="558" y="204" font-size="12" fill="#c2410c">（initialData）</text>
<line x1="548" y1="179" x2="521" y2="178" stroke="#f97316" stroke-width="2" marker-end="url(#ar-08-06)"/>
<rect x="548" y="250" width="156" height="48" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="558" y="270" font-size="12" fill="#c2410c">評価やステータスも</text>
<text x="558" y="287" font-size="12" fill="#c2410c">今の値が選ばれている</text>
<line x1="548" y1="279" x2="521" y2="281" stroke="#f97316" stroke-width="2" marker-end="url(#ar-08-06)"/>
<rect x="548" y="330" width="156" height="48" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="558" y="350" font-size="12" fill="#c2410c">ボタンは「更新する」</text>
<text x="558" y="367" font-size="12" fill="#c2410c">（isEdit が true）</text>
<line x1="548" y1="360" x2="521" y2="371" stroke="#f97316" stroke-width="2" marker-end="url(#ar-08-06)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-7: 画面イメージ ― 編集ページ（/books/[id]/edit）を開いた直後。新規登録と同じ BookForm だが、今の値が入っている</div>
</div>

新規登録ページ（`/books/new`）と見比べると、違いは「欄に最初から値が入っている」「ボタンの文字が『更新する』」「タイトルの上に『〇〇の情報を編集します』と出る」の3点だけです。同じ `BookForm` を props で2役に切り替えていることが、画面からも分かります。

> **ヒント:** 出版日の欄の見た目（`2024/01/15` のような表示）はブラウザや OS の設定で変わりますが、プログラムの中の値はいつも `"2024-01-15"`（YYYY-MM-DD 形式の文字列）です。`<input type="date">` は、この形式の文字列でないと値を表示できません。

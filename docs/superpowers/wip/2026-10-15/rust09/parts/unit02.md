## UNIT 02: 画面遷移・情報設計・ワイヤーフレーム

### この単元のゴール

- RustNote の画面を一覧にし、それぞれの URL・メソッドと「いちばんの目的」を1つずつ決められる
- 画面遷移図を描き、フォームの送信が成功したら 303 で GET の画面へ移る **PRG**（Post/Redirect/Get）の流れと、その理由を説明できる
- 1つの画面が持つ**状態**（空・入力エラー・成功のお知らせ・ページ送り・検索で0件・エラー画面）を書き出し、テストで確かめられる
- 状態を変える操作を GET（リンク）にしてはいけない理由を、405 の実例で説明できる

### たとえ話でつかむ

役所の窓口を思い浮かべてください。入口に案内図があり、「1番 受付」「2番 書類の記入」「3番 証明書の受け取り」のように、部屋ごとに**1つの目的**が決まっています。

- 書類に書き間違いがあると、窓口の人は「ここが空いています」と赤ペンで印を付けて、**書いた書類を返してくれます**。最初から全部書き直しにはなりません。
- 手続きが終わると、窓口の人は「終わりました。3番の窓口で受け取ってください」と、**次の部屋へ案内**します。同じ窓口の前に立ったまま「もう一度お願いします」と言うと、同じ手続きをもう1回してしまいます。
- 3番の窓口では、「先ほどの手続きは完了しています」と**一度だけ**声をかけてもらえます。

Web アプリの画面の設計も、この案内図づくりと同じです。

| 役所 | Web アプリ |
|---|---|
| 部屋と、その目的 | **画面**（ページ）と、画面のいちばんの目的 |
| 部屋の番号札 | **URL**（`/notes`・`/notes/1/edit` など） |
| 案内図 | **画面遷移図** |
| 窓口に出す書類 | **フォーム**（POST で送る） |
| 書類を返してもらう | 入力エラーのとき、**400** で入力を残したまま画面を出し直す |
| 「次の部屋へどうぞ」 | **303 See Other**（別の URL を GET で開き直させる） |
| 一度だけの声かけ | **フラッシュメッセージ**（「ノートを作成しました」） |

> **用語:** 画面遷移図（がめんせんいず：画面と画面のつながりを、矢印で描いた図。どの操作で、どの画面へ移るかを表す）

<!-- -->

> **用語:** 情報設計（じょうほうせっけい：画面に何を、どの順番で、どうまとめて置くかを決めること。information architecture）

<!-- -->

> **用語:** ワイヤーフレーム（wireframe：色や飾りを付けずに、画面の部品の配置だけを線で描いた設計図）

<!-- -->

> **用語:** PRG（Post/Redirect/Get：フォームの POST が成功したら、画面を直接返さずに、303 で別の URL へ移し、ブラウザに GET で開き直させる形）

### 図で理解する

RustNote の画面は、トップ・登録・ログイン・一覧・詳細・編集の6つと、どの URL でも出る可能性のあるエラー画面です。次の図は、その画面遷移図です。四角が画面、矢印が「どの操作で、どこへ移るか」です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 380" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0201" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="24" y="30" width="140" height="48" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="94" y="50" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">トップ</text>
<text x="94" y="68" font-size="11" fill="#475569" text-anchor="middle">GET /</text>
<rect x="24" y="140" width="140" height="48" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="94" y="160" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">登録</text>
<text x="94" y="178" font-size="11" fill="#475569" text-anchor="middle">GET /signup</text>
<rect x="24" y="250" width="140" height="48" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="94" y="270" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">ログイン</text>
<text x="94" y="288" font-size="11" fill="#475569" text-anchor="middle">GET /login</text>
<rect x="290" y="140" width="150" height="48" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="365" y="160" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">一覧</text>
<text x="365" y="178" font-size="11" fill="#475569" text-anchor="middle">GET /notes</text>
<rect x="556" y="140" width="150" height="48" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="631" y="160" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">詳細</text>
<text x="631" y="178" font-size="11" fill="#475569" text-anchor="middle">GET /notes/{id}</text>
<rect x="556" y="270" width="150" height="48" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="631" y="290" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">編集</text>
<text x="631" y="308" font-size="11" fill="#475569" text-anchor="middle">GET /notes/{id}/edit</text>
<line x1="94" y1="78" x2="94" y2="136" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0201)"/>
<text x="102" y="112" font-size="11" fill="#475569">ボタン</text>
<line x1="94" y1="190" x2="94" y2="246" stroke="#475569" stroke-width="2" marker-start="url(#ar-09-0201)" marker-end="url(#ar-09-0201)"/>
<text x="102" y="222" font-size="11" fill="#475569">リンク</text>
<path d="M24,54 H12 V274 H20" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0201)"/>
<path d="M164,54 H365 V136" fill="none" stroke="#475569" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar-09-0201)"/>
<text x="265" y="46" font-size="11" fill="#475569" text-anchor="middle">ログイン中なら 303</text>
<line x1="164" y1="164" x2="286" y2="164" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0201)"/>
<text x="225" y="154" font-size="11" fill="#0f172a" text-anchor="middle">POST /signup</text>
<text x="225" y="180" font-size="11" fill="#0f172a" text-anchor="middle">→ 303</text>
<path d="M164,274 H330 V192" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0201)"/>
<text x="247" y="266" font-size="11" fill="#0f172a" text-anchor="middle">POST /login → 303</text>
<text x="395" y="212" font-size="11" fill="#c2410c" text-anchor="middle">入力の誤りは</text>
<text x="395" y="228" font-size="11" fill="#c2410c" text-anchor="middle">400 で出し直す</text>
<line x1="440" y1="154" x2="552" y2="154" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0201)"/>
<text x="496" y="146" font-size="11" fill="#0f172a" text-anchor="middle">POST /notes → 303</text>
<line x1="556" y1="178" x2="444" y2="178" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0201)"/>
<text x="500" y="196" font-size="11" fill="#0f172a" text-anchor="middle">…/delete → 303</text>
<path d="M606,140 V114 H656 V136" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0201)"/>
<text x="631" y="104" font-size="11" fill="#0f172a" text-anchor="middle">…/toggle → 303</text>
<line x1="596" y1="188" x2="596" y2="266" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0201)"/>
<text x="588" y="232" font-size="11" fill="#475569" text-anchor="end">編集ボタン</text>
<line x1="666" y1="270" x2="666" y2="192" stroke="#475569" stroke-width="2" marker-end="url(#ar-09-0201)"/>
<text x="674" y="226" font-size="11" fill="#0f172a">保存</text>
<text x="674" y="242" font-size="11" fill="#0f172a">→ 303</text>
<text x="360" y="346" font-size="12" fill="#475569" text-anchor="middle">ログイン中のヘッダー「ログアウト」→ POST /logout → 303 でログイン画面へ</text>
<text x="360" y="368" font-size="12" fill="#475569" text-anchor="middle">存在しない URL・他人のノート・読めない入力は、どこでもエラー画面（404・400 など）</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: RustNote の画面遷移図。画面を開くのは GET、ノートを変えるのは POST で、POST が成功したら必ず 303 で GET の画面へ移る</div>
</div>

- **見るのは GET、変えるのは POST:** 画面を開く矢印（ボタン・リンク）はすべて GET です。登録・ログイン・作成・保存・完了の切り替え・削除・ログアウトのように、DB の中身やログインの状態を**変える**操作は、すべてフォームの POST です。
- **POST の行き先は、必ず 303:** POST が成功したら、画面を直接返さずに、303 で GET の画面へ移します。作成と保存と完了の切り替えは、そのノートの詳細へ。削除は、もうノートが無いので一覧へ。登録とログインは一覧へ。ログアウトはログイン画面へ戻ります。
- **失敗は、その場で出し直す:** 入力に誤りがあるときは、移動せずに 400（ログインの失敗は 401、版の食い違いは 409）で同じ画面を出し直します。入力した内容は消しません。

次の図は、PRG の流れを、ブラウザとサーバーのやり取りとして描いたものです。ノートを作成する場合です。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 380" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-09-0202" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="50" y="16" width="140" height="36" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="120" y="39" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">ブラウザ</text>
<rect x="530" y="16" width="140" height="36" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="600" y="39" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">RustNote</text>
<line x1="120" y1="52" x2="120" y2="360" stroke="#94a3b8" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="600" y1="52" x2="600" y2="360" stroke="#94a3b8" stroke-width="2" stroke-dasharray="4 4"/>
<text x="360" y="78" font-size="12" fill="#0f172a" text-anchor="middle">① POST /notes（title=牛乳を買う）</text>
<line x1="120" y1="88" x2="596" y2="88" stroke="#3b82f6" stroke-width="2" marker-end="url(#ar-09-0202)"/>
<text x="360" y="118" font-size="12" fill="#0f172a" text-anchor="middle">② 303 See Other　Location: /notes/1</text>
<text x="360" y="134" font-size="12" fill="#0f172a" text-anchor="middle">Set-Cookie: flash=note_created</text>
<line x1="600" y1="144" x2="124" y2="144" stroke="#10b981" stroke-width="2" marker-end="url(#ar-09-0202)"/>
<text x="360" y="178" font-size="12" fill="#0f172a" text-anchor="middle">③ GET /notes/1（Cookie: flash=note_created）</text>
<line x1="120" y1="188" x2="596" y2="188" stroke="#3b82f6" stroke-width="2" marker-end="url(#ar-09-0202)"/>
<text x="360" y="218" font-size="12" fill="#0f172a" text-anchor="middle">④ 200 OK　画面に「ノートを作成しました」</text>
<text x="360" y="234" font-size="12" fill="#0f172a" text-anchor="middle">Set-Cookie: flash=（消す指示）</text>
<line x1="600" y1="244" x2="124" y2="244" stroke="#10b981" stroke-width="2" marker-end="url(#ar-09-0202)"/>
<rect x="140" y="262" width="440" height="88" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="1.5"/>
<text x="360" y="286" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">ここで再読み込み（F5）しても</text>
<text x="360" y="308" font-size="12" fill="#0f172a" text-anchor="middle">繰り返されるのは ③ の GET だけ。ノートは増えない</text>
<text x="360" y="330" font-size="12" fill="#0f172a" text-anchor="middle">flash は消えたので、お知らせも二度は出ない</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: PRG の流れ。POST の返事は 303 と移動先だけで、ブラウザが改めて GET する。再読み込みで繰り返されるのは GET なので、ノートが二重にできない</div>
</div>

- ① フォームの POST を受け取ったサーバーは、ノートを保存します。② その返事として画面を返さず、「`/notes/1` を開き直してください」という 303 を返します。同時に、「次の画面で『作成しました』と出す」という予約を、`flash` という名前のクッキーに入れます。
- ③ ブラウザは、言われた URL を自分で GET します。④ サーバーは詳細の画面を返し、`flash` を読んで「ノートを作成しました」を表示し、そのクッキーを消す指示を出します。
- もし ② で画面をそのまま返していたら（PRG にしていなかったら）、ブラウザの「今の画面」は POST の結果です。そこで再読み込みすると、ブラウザは POST をもう一度送ろうとし、同じノートがもう1件できてしまいます。

> **用語:** フラッシュメッセージ（flash message：「保存しました」のように、次の画面で一度だけ出すお知らせ。RustNote では、決まった短いコードをクッキーに入れて運ぶ）

### まず動かす最小コード

画面の一覧（情報設計）を表にします。画面ごとに、**いちばんの目的を1つだけ**決めるのがコツです。目的が2つあると、どちらの部品を目立たせるかで迷い、画面がごちゃごちゃします。

| 画面 | URL（GET） | いちばんの目的 | 主な部品 | ここから送る POST |
|---|---|---|---|---|
| トップ | `/` | 何のアプリかを伝え、登録・ログインへ案内する | 見出し・説明の文・「新規登録」「ログイン」のボタン | なし |
| 登録 | `/signup` | アカウントを作る | メールアドレス・パスワード・パスワード（確認） | `POST /signup` |
| ログイン | `/login` | ログインする | メールアドレス・パスワード | `POST /login` |
| 一覧 | `/notes` | 自分のノートを見つける | 検索欄・「新しいノートを書く」（折りたたみ）・ノートの一覧・ページ送り | `POST /notes` |
| 詳細 | `/notes/{id}` | 1件のノートを読む | タイトル・完了の印・作成と更新の日時・本文・編集／完了／削除のボタン | `…/toggle`・`…/delete` |
| 編集 | `/notes/{id}/edit` | 1件のノートを直す | タイトル・本文（見えない版の番号つき） | `…/edit` |
| エラー | （どの URL でも） | 何が起きたかと、戻り道を伝える | 番号つきの見出し（`404 Not Found` など）・説明・「トップページへ戻る」 | なし |

- どの画面にも、上に共通の帯（ヘッダー）があります。ログインしていなければ「ログイン」「新規登録」、ログイン中なら「ノート一覧」・自分のメールアドレス・「ログアウト」のボタンが並びます。
- 作成のフォームは、独立した画面にせず、一覧の中の折りたたみにしました。ノートを書くのは「一覧を見ながら」が多いからです。

いちばん部品の多い一覧の画面を、ワイヤーフレームにすると次のようになります。色や飾りは付けず、部品の並びだけを描きます。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 400" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="16" y="16" width="470" height="368" rx="10" fill="#ffffff" stroke="#64748b" stroke-width="2"/>
<rect x="17" y="17" width="468" height="29" rx="9" fill="#f1f5f9"/>
<circle cx="34" cy="31" r="4" fill="#cbd5e1"/>
<circle cx="48" cy="31" r="4" fill="#cbd5e1"/>
<circle cx="62" cy="31" r="4" fill="#cbd5e1"/>
<rect x="80" y="22" width="320" height="18" rx="9" fill="#ffffff" stroke="#cbd5e1"/>
<text x="92" y="35" font-size="11" fill="#475569">127.0.0.1:3000/notes</text>
<rect x="17" y="46" width="468" height="34" fill="#eff6ff"/>
<text x="30" y="68" font-size="13" font-weight="700" fill="#1e40af">RustNote</text>
<text x="236" y="68" font-size="11" fill="#1d4ed8">ノート一覧</text>
<text x="300" y="68" font-size="11" fill="#64748b">alice@example.com</text>
<rect x="414" y="55" width="62" height="20" rx="4" fill="#ffffff" stroke="#94a3b8"/>
<text x="445" y="69" font-size="11" fill="#0f172a" text-anchor="middle">ログアウト</text>
<text x="32" y="111" font-size="18" font-weight="700" fill="#0f172a">ノート</text>
<circle cx="104" cy="104" r="9" fill="#f97316"/>
<text x="104" y="108" font-size="11" font-weight="700" fill="#ffffff" text-anchor="middle">1</text>
<rect x="32" y="124" width="300" height="26" rx="4" fill="#ffffff" stroke="#94a3b8"/>
<text x="42" y="142" font-size="12" fill="#94a3b8">キーワードで検索</text>
<rect x="340" y="124" width="56" height="26" rx="4" fill="#3b82f6"/>
<text x="368" y="142" font-size="12" fill="#ffffff" text-anchor="middle">検索</text>
<circle cx="412" cy="137" r="9" fill="#f97316"/>
<text x="412" y="141" font-size="11" font-weight="700" fill="#ffffff" text-anchor="middle">2</text>
<text x="32" y="178" font-size="12" font-weight="700" fill="#0f172a">▶ 新しいノートを書く</text>
<circle cx="182" cy="174" r="9" fill="#f97316"/>
<text x="182" y="178" font-size="11" font-weight="700" fill="#ffffff" text-anchor="middle">3</text>
<rect x="32" y="192" width="430" height="34" rx="6" fill="#ffffff" stroke="#cbd5e1"/>
<text x="44" y="214" font-size="12" fill="#1d4ed8">牛乳を買う</text>
<text x="452" y="214" font-size="11" fill="#64748b" text-anchor="end">更新: 2026-10-24 04:28:10</text>
<rect x="32" y="232" width="430" height="34" rx="6" fill="#ffffff" stroke="#cbd5e1"/>
<text x="44" y="254" font-size="12" fill="#94a3b8">宿題</text>
<rect x="76" y="241" width="36" height="18" rx="9" fill="#ecfdf5" stroke="#10b981"/>
<text x="94" y="254" font-size="11" fill="#047857" text-anchor="middle">完了</text>
<text x="452" y="254" font-size="11" fill="#64748b" text-anchor="end">更新: 2026-10-23 12:00:00</text>
<rect x="32" y="272" width="430" height="34" rx="6" fill="#ffffff" stroke="#cbd5e1"/>
<text x="44" y="294" font-size="12" fill="#1d4ed8">Rust の本</text>
<text x="452" y="294" font-size="11" fill="#64748b" text-anchor="end">更新: 2026-10-22 09:15:00</text>
<circle cx="474" cy="249" r="9" fill="#f97316"/>
<text x="474" y="253" font-size="11" font-weight="700" fill="#ffffff" text-anchor="middle">4</text>
<text x="200" y="344" font-size="12" fill="#0f172a" text-anchor="middle">1 / 2 ページ（全21件）</text>
<text x="360" y="344" font-size="12" fill="#1d4ed8">次へ →</text>
<circle cx="424" cy="340" r="9" fill="#f97316"/>
<text x="424" y="344" font-size="11" font-weight="700" fill="#ffffff" text-anchor="middle">5</text>
<text x="504" y="104" font-size="12" font-weight="700" fill="#c2410c">1 見出し</text>
<text x="504" y="121" font-size="12" fill="#0f172a">画面の目的を1つ表す</text>
<text x="504" y="148" font-size="12" font-weight="700" fill="#c2410c">2 検索欄（?q=）</text>
<text x="504" y="165" font-size="12" fill="#0f172a">空なら全部を表示</text>
<text x="504" y="192" font-size="12" font-weight="700" fill="#c2410c">3 新しいノートを書く</text>
<text x="504" y="209" font-size="12" fill="#0f172a">ふだんは折りたたみ。</text>
<text x="504" y="226" font-size="12" fill="#0f172a">入力の誤りは開いて 400</text>
<text x="504" y="253" font-size="12" font-weight="700" fill="#c2410c">4 ノートの一覧</text>
<text x="504" y="270" font-size="12" fill="#0f172a">新しく更新した順。</text>
<text x="504" y="287" font-size="12" fill="#0f172a">0件なら「まだノートが</text>
<text x="504" y="304" font-size="12" fill="#0f172a">ありません。…」</text>
<text x="504" y="336" font-size="12" font-weight="700" fill="#c2410c">5 ページ送り</text>
<text x="504" y="353" font-size="12" fill="#0f172a">1ページ 20 件まで</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図9-?: 一覧の画面のワイヤーフレーム。上から見出し・検索欄・作成の折りたたみ・ノートの一覧・ページ送りの順に並べ、それぞれの部品が持つ状態を書き添える</div>
</div>

- 部品は、利用者がよく使う順に、上から並べています。一覧の画面に来た人は、まず「探す」か「書く」かのどちらかをするので、その2つを一覧より上に置きました。
- 完了したノート（図の「宿題」）は、タイトルをうすい色にし、「完了」の印を付けます。日時は UTC（世界の標準時）の `年-月-日 時:分:秒` の形で出します。

この表が、実際の画面と合っているかを、テストで確かめます。

#### 例2-1: 画面を順番に開いて、ステータスと見出しを確かめる

> **▼ このコードがやること（先に日本語で）:** 画面の一覧の表のとおりに、トップ・登録・ログイン・一覧・詳細・編集・エラーの画面を順番に開きます。それぞれのステータスコードと、画面のいちばん大きな見出し（`<h1>`）を1行ずつ表示します。見出しは、その画面の目的を表す言葉のはずです。

```rust
//! 画面を順番に開き、ステータスコードと見出し（h1）を表示する。
//! 実行: cargo test --test screens -- --nocapture

mod common;

use common::{body_text, test_app};
use topcoat::router::response::Response;

const PASSWORD: &str = "correct horse battery";

// HTML から最初の <h1>…</h1> の中身を取り出す
fn h1(html: &str) -> String {
    let start = html.find("<h1>").map(|i| i + "<h1>".len());
    let end = html.find("</h1>");
    match (start, end) {
        (Some(start), Some(end)) if start <= end => html[start..end].to_string(),
        _ => "（h1 なし）".to_string(),
    }
}

// 1 画面分を「[画面の名前] 送ったリクエスト -> ステータス 見出し」の形で表示する
async fn show(screen: &str, request: &str, response: Response) -> String {
    let status = response.status().as_u16();
    let html = body_text(response).await;
    println!("[{screen}] {request} -> {status} 見出し「{}」", h1(&html));
    html
}

#[tokio::test]
async fn walk_all_screens() {
    let app = test_app().await;

    // ログインしていない人が見る画面
    show("トップ", "GET /", app.get("/", None).await).await;
    show("登録", "GET /signup", app.get("/signup", None).await).await;
    show("ログイン", "GET /login", app.get("/login", None).await).await;

    // 登録すると、ログインした状態になる
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;
    let html = show(
        "一覧（空）",
        "GET /notes",
        app.get("/notes", Some(&cookie)).await,
    )
    .await;
    assert!(html.contains("まだノートがありません"));

    // タイトルを空にして作成を送ると、一覧の画面が 400 で出直す
    let response = app
        .post_form("/notes", "title=&body=memo", Some(&cookie))
        .await;
    show("作成（失敗）", "POST /notes", response).await;

    let url = app.create_note(&cookie, "牛乳を買う", "2本").await;
    show("詳細", "GET /notes/1", app.get(&url, Some(&cookie)).await).await;
    let edit = format!("{url}/edit");
    show(
        "編集",
        "GET /notes/1/edit",
        app.get(&edit, Some(&cookie)).await,
    )
    .await;
    show("エラー", "GET /nope", app.get("/nope", Some(&cookie)).await).await;
}
```

`tests/screens.rs` として保存して実行します。`println!` の表示を見たいので、`-- --nocapture` を付けます。

```powershell
cargo test --test screens -- --nocapture
```

**▼ 実行結果**

```text
   Compiling rustnote v0.1.0 (...\rustnote)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.67s
     Running tests\screens.rs (target\debug\deps\screens-a0a0ab1b6a58860c.exe)

running 1 test
GET / -> 200 OK (1.377ms)
[トップ] GET / -> 200 見出し「RustNote」
GET /signup -> 200 OK (432.8µs)
[登録] GET /signup -> 200 見出し「新規登録」
GET /login -> 200 OK (206µs)
[ログイン] GET /login -> 200 見出し「ログイン」
POST /signup -> 303 See Other (445.6532ms)
GET /notes -> 200 OK (3.8955ms)
[一覧（空）] GET /notes -> 200 見出し「ノート」
POST /notes -> 400 Bad Request (2.5227ms)
[作成（失敗）] POST /notes -> 400 見出し「ノート」
POST /notes -> 303 See Other (4.1806ms)
GET /notes/1 -> 200 OK (3.1275ms)
[詳細] GET /notes/1 -> 200 見出し「牛乳を買う」
GET /notes/1/edit -> 200 OK (1.8396ms)
[編集] GET /notes/1/edit -> 200 見出し「ノートを編集」
GET /nope -> 404 Not Found (1.2528ms)
[エラー] GET /nope -> 404 見出し「404 Not Found」
test walk_all_screens ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.49s
```

`[ ]` で始まる行が、このテストの表示です。その1行上の `GET / -> 200 OK (1.377ms)` のような行は、RustNote 自身のアクセスログです。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 見出しが、画面の目的を表しているか

- 表示された見出しは「RustNote」「新規登録」「ログイン」「ノート」「牛乳を買う」「ノートを編集」「404 Not Found」でした。情報設計の表の「いちばんの目的」と見比べると、どの画面も、見出しを読めば何の画面かが分かります。
- 詳細の画面の見出しは、固定の言葉ではなく、ノートのタイトル（「牛乳を買う」）です。詳細の目的は「1件を読む」なので、何のノートかがいちばん大事だからです。
- 目の不自由な人が使う読み上げソフトでは、見出しを手がかりに、画面の中を移動したり、何の画面かを知ったりします。見出しを画面の目的に合わせるのは、見た目のためだけではありません（UNIT 09（準備中）でくわしく扱います）。

##### 解説2: 失敗しても、画面は「一覧」のまま

- `[作成（失敗）] POST /notes -> 400 見出し「ノート」` に注目してください。タイトルが空の作成を送ると、別のエラー画面ではなく、**一覧の画面が 400 で出し直されます**。図の「入力の誤りは 400 で出し直す」です。
- たとえ話の「書類を返してもらう」と同じで、利用者はその場で直して、もう一度送れます。

##### 解説3: アクセスログで、移動のしかたが分かる

- `POST /signup -> 303 See Other` は、登録が成功して 303 で移動したことを示しています。`app.sign_up` は登録のリクエストだけを送っていますが、アクセスログを見ると、登録が PRG の形になっていることが分かります。
- `GET / -> 200 OK (1.377ms)` の最後のかっこは、そのリクエストの処理にかかった時間です。登録だけ `445.6532ms` と長いのは、パスワードを Argon2id でハッシュにしているからです（わざと時間のかかる計算です。第8巻）。

##### 解説4: h1 関数は、テスト用の簡単な取り出し方

```rust
fn h1(html: &str) -> String {
    let start = html.find("<h1>").map(|i| i + "<h1>".len());
    let end = html.find("</h1>");
```

- HTML の中から、最初の `<h1>` と `</h1>` の位置を探して、その間を取り出しています。`find` は見つかった位置（バイトの番号）を `Some(位置)` で返し、無ければ `None` を返します。
- `<h1 class="...">` のように属性が付いた見出しには使えない、簡単な作りです。RustNote の見出しはどれも属性なしの `<h1>` なので、これで足ります。本格的に HTML を調べるなら、HTML を解析するクレートを使います。

### もう一歩: 実務に近い形

#### 例2-2: PRG の流れを、1歩ずつ表示する

> **▼ このコードがやること（先に日本語で）:** ノートの作成を送り、返事のステータス・移動先（`Location`）・フラッシュのクッキー（`Set-Cookie`）を表示します。次に、ブラウザがするのと同じように、移動先を GET で開き直し、「ノートを作成しました」が出ることを確かめます。最後に、再読み込みしたときと同じように、もう一度 GET して、お知らせが二度は出ないことを確かめます。

```rust
//! PRG（Post/Redirect/Get）の流れを、1歩ずつ表示する。
//! 実行: cargo test --test flow -- --nocapture

mod common;

use common::{body_text, encode, location, test_app};
use topcoat::router::{header, response::Response};

const PASSWORD: &str = "correct horse battery";

// Set-Cookie のうち、flash= で始まるものをそのまま返す
fn flash_set_cookie(response: &Response) -> Option<String> {
    response
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .find(|value| value.starts_with("flash="))
        .map(str::to_string)
}

// 「はい」「いいえ」で表示する
fn yes_no(value: bool) -> &'static str {
    if value { "はい" } else { "いいえ" }
}

#[tokio::test]
async fn create_note_with_prg() {
    let app = test_app().await;
    let session = app.sign_up("alice@example.com", PASSWORD).await;

    // 1) 作成フォームを送る（POST）
    let form = format!("title={}&body=", encode("牛乳を買う"));
    let response = app.post_form("/notes", &form, Some(&session)).await;
    println!("1) POST /notes -> {}", response.status());
    println!("   Location: {}", location(&response));
    let set_cookie = flash_set_cookie(&response).expect("flash がありません");
    println!("   Set-Cookie: {set_cookie}");
    let url = location(&response).to_string();

    // 2) ブラウザは Location の URL を GET で開き直す（受け取った flash のクッキーも送る）
    let flash = set_cookie.split(';').next().unwrap();
    let cookies = format!("{session}; {flash}");
    let response = app.get(&url, Some(&cookies)).await;
    println!("2) GET {url} -> {}", response.status());
    if let Some(set_cookie) = flash_set_cookie(&response) {
        println!("   Set-Cookie: {set_cookie}");
    }
    let html = body_text(response).await;
    println!(
        "   「ノートを作成しました」が出た: {}",
        yes_no(html.contains("ノートを作成しました"))
    );

    // 3) 再読み込み（F5）。flash のクッキーは消えたので、セッションだけを送る
    let response = app.get(&url, Some(&session)).await;
    println!("3) GET {url} -> {}", response.status());
    let html = body_text(response).await;
    println!(
        "   「ノートを作成しました」が出た: {}",
        yes_no(html.contains("ノートを作成しました"))
    );
}
```

`tests/flow.rs` として保存して実行します。

```powershell
cargo test --test flow -- --nocapture
```

**▼ 実行結果**

```text
   Compiling rustnote v0.1.0 (...\rustnote)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.05s
     Running tests\flow.rs (target\debug\deps\flow-4a51733f40431b48.exe)

running 1 test
POST /signup -> 303 See Other (576.4574ms)
POST /notes -> 303 See Other (5.8182ms)
1) POST /notes -> 303 See Other
   Location: /notes/1
   Set-Cookie: flash=note_created; HttpOnly; SameSite=Lax; Path=/; Max-Age=300
GET /notes/1 -> 200 OK (3.6119ms)
2) GET /notes/1 -> 200 OK
   Set-Cookie: flash=; Path=/; Max-Age=0; Expires=Fri, 24 Oct 2025 04:28:10 GMT
   「ノートを作成しました」が出た: はい
GET /notes/1 -> 200 OK (2.8042ms)
3) GET /notes/1 -> 200 OK
   「ノートを作成しました」が出た: いいえ
test create_note_with_prg ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.62s
```

PRG の図の ①〜④ と、再読み込みの場面が、そのまま表示されています。

#### ▼ コードを1つずつ分解して解説

##### 解説1: POST の返事は「303 と移動先」だけ

```text
1) POST /notes -> 303 See Other
   Location: /notes/1
```

- 作成が成功した返事は、画面ではなく、`303 See Other` と `Location: /notes/1`（移動先）です。ブラウザは、この2つを受け取ると、自動で `/notes/1` を GET します。テストには自動で移動する仕組みが無いので、2) で自分で GET しています。
- 移動先が一覧ではなく**できたノートの詳細**なのは、利用者が「ちゃんと保存されたか」をすぐ確かめられるようにするためです。

> **用語:** 303 See Other（シー・アザー：「別の URL を GET で見てください」という返事。POST の後のリダイレクトに使う。似た 307 や 308 は、元のメソッド（POST）のまま送り直させるので、PRG には使わない）

##### 解説2: フラッシュのクッキーは、決まったコードと短い期限

```text
   Set-Cookie: flash=note_created; HttpOnly; SameSite=Lax; Path=/; Max-Age=300
```

- クッキーの中身は、文章そのものではなく `note_created` という**決まったコード**です。画面に出す文章（「ノートを作成しました」）は、サーバーがコードから選びます（`src/flash.rs`）。利用者がクッキーを書き換えても、知らないコードは無視されるので、好きな文字を画面に出させることはできません。
- `Max-Age=300` は「300 秒（5分）で消える」、`HttpOnly` は「ページの JavaScript からは読めない」、`SameSite=Lax` は「ほかのサイトからの POST には付けて送らない」、`Path=/` は「サイトのどの URL にも送る」という意味です。

##### 解説3: 一度出したら、クッキーを消す

```text
   Set-Cookie: flash=; Path=/; Max-Age=0; Expires=Fri, 24 Oct 2025 04:28:10 GMT
```

- 2) の返事には、`flash` の値を空にして `Max-Age=0`（今すぐ消す）にした `Set-Cookie` が付いています。`Expires` に1年前の日時が入っているのも、「もう期限切れです」と伝えて、確実に消してもらうためです。
- ブラウザはこのクッキーを消すので、3) の再読み込みでは、もう `flash` を送りません。だから「ノートを作成しました」は二度と出ません。テストでは、3) でセッションのクッキーだけを送って、この動きを再現しています。

##### 解説4: クッキーを2つ送るときは「; 」でつなぐ

```rust
    let flash = set_cookie.split(';').next().unwrap();
    let cookies = format!("{session}; {flash}");
```

- `Set-Cookie` の値は `flash=note_created; HttpOnly; ...` のように、後ろに属性が続きます。ブラウザが送り返すのは、最初の `名前=値` の部分だけです。`split(';').next()` で、そこだけを取り出しています。
- リクエストの `Cookie` ヘッダーには、複数のクッキーを `; `（セミコロンと空白）でつないで入れます。ログインのクッキー（`__Host-session=...`）と `flash=note_created` の両方を送っています。

#### 例2-3: 一覧画面の「状態」を1つずつ作る

画面は、1つの URL でも、いくつもの**状態**を持ちます。たとえば一覧の画面は、ノートが1件も無いとき・入力に誤りがあったとき・ノートが多くてページが分かれるとき・検索して何も当たらなかったときで、表示が変わります。設計の段階で状態を書き出しておかないと、「ノートが0件のときに何も表示されない、真っ白な一覧」ができてしまいます。

| 状態 | ステータス | 画面に出すもの |
|---|---|---|
| 空（1件も無い） | 200 | 「まだノートがありません。最初の1件を書いてみましょう。」 |
| 入力エラー | 400 | 項目の下に「タイトルを入力してください」。入力した本文は残す。作成の折りたたみは開いておく |
| ページが分かれる | 200 | 「1 / 2 ページ（全21件）」と「次へ →」 |
| 検索で0件 | 200 | 「条件に合うノートはありません。」（「空」とは違う文にする） |

> **▼ このコードがやること（先に日本語で）:** 表の4つの状態を、順番に作ります。それぞれの状態で一覧を開き、ステータスコードと、表の「画面に出すもの」の文が本当に出ているかを表示します。21件のノートは、画面を通さずに、RustNote のリポジトリ（DB に保存する部品）で直接入れます。

```rust
//! 一覧画面（GET /notes）の「状態」を1つずつ作り、画面に出る文を確かめる。
//! 実行: cargo test --test states -- --nocapture

mod common;

use common::{body_text, test_app};
use rustnote::domain::{NewNote, UserId};
use rustnote::repository::{LibsqlNoteRepository, NoteRepository};
use topcoat::router::response::Response;

const PASSWORD: &str = "correct horse battery";

// 画面の状態の名前・ステータスと、期待する文が見つかったかを表示する
async fn check(state: &str, response: Response, expected: &[&str]) {
    let status = response.status().as_u16();
    let html = body_text(response).await;
    println!("[{state}] {status}");
    for text in expected {
        let found = if html.contains(text) {
            "あり"
        } else {
            "なし"
        };
        println!("    {found}: {text}");
    }
}

#[tokio::test]
async fn list_page_states() {
    let app = test_app().await;
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;

    // 1) 空の状態: まだ1件もない
    let response = app.get("/notes", Some(&cookie)).await;
    check("空", response, &["まだノートがありません。"]).await;

    // 2) 入力エラーの状態: タイトルを空で送った（入力した本文は残る）
    let response = app
        .post_form("/notes", "title=&body=memo", Some(&cookie))
        .await;
    check(
        "入力エラー",
        response,
        &["タイトルを入力してください", ">memo</textarea>"],
    )
    .await;

    // 3) 件数が多い状態: 21 件あると、2 ページに分かれる
    // （画面を通さず、リポジトリで直接 21 件を入れる。最初に登録した alice の ID は 1）
    let notes = LibsqlNoteRepository::new(app.db.db.clone());
    let alice = UserId::new(1).unwrap();
    for i in 1..=21 {
        let note = NewNote::parse(&format!("メモ{i:02}"), "").unwrap();
        notes.insert(alice, &note).await.unwrap();
    }
    let response = app.get("/notes", Some(&cookie)).await;
    check(
        "2ページある",
        response,
        &["1 / 2 ページ（全21件）", "次へ →"],
    )
    .await;

    // 4) 検索して1件も当たらない状態（「空」とは違う文を出す）
    let response = app.get("/notes?q=zzz", Some(&cookie)).await;
    check("検索0件", response, &["条件に合うノートはありません。"]).await;
}
```

`tests/states.rs` として保存して実行します。

```powershell
cargo test --test states -- --nocapture
```

**▼ 実行結果**

```text
   Compiling rustnote v0.1.0 (...\rustnote)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.33s
     Running tests\states.rs (target\debug\deps\states-a9e2e08f16d176d0.exe)

running 1 test
POST /signup -> 303 See Other (418.421ms)
GET /notes -> 200 OK (2.5087ms)
[空] 200
    あり: まだノートがありません。
POST /notes -> 400 Bad Request (2.5551ms)
[入力エラー] 400
    あり: タイトルを入力してください
    あり: >memo</textarea>
GET /notes -> 200 OK (2.8296ms)
[2ページある] 200
    あり: 1 / 2 ページ（全21件）
    あり: 次へ →
GET /notes -> 200 OK (1.9618ms)
[検索0件] 200
    あり: 条件に合うノートはありません。
test list_page_states ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.52s
```

4つの状態のすべてで、表に書いた文が「あり」になりました。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 「空」と「検索で0件」は、違う文にする

- どちらも「表示するノートが0件」ですが、利用者の気持ちは違います。「空」の人は、まだ1件も書いていないので、「書いてみましょう」と背中を押す文がよい。「検索で0件」の人は、ノートを持っているのに見つからないので、「条件に合うノートはありません」と、条件のせいだと分かる文がよい。
- RustNote の一覧（`src/routes/notes.rs`）は、検索の言葉があるかどうかで、2つの文を出し分けています。

##### 解説2: 入力エラーでは、入力した内容を消さない

```rust
        &["タイトルを入力してください", ">memo</textarea>"],
```

- `>memo</textarea>` は、本文の入力欄（`<textarea>`）の中に、送った `memo` がそのまま入っていることを確かめています。せっかく書いた本文が、タイトルを忘れただけで消えたら、利用者は困ります。
- RustNote は、入力エラーのとき、作成の折りたたみ（`<details>`）を開いた状態で出します。閉じたままだと、エラーの文がどこに出ているのか分かりにくいからです（`open=(...)` の部分）。

##### 解説3: 前提のデータは、画面を通さずに入れてもよい

```rust
    let notes = LibsqlNoteRepository::new(app.db.db.clone());
    let alice = UserId::new(1).unwrap();
```

- 21件のノートを `app.create_note` で作ると、POST が21回送られ、アクセスログが21行並びます。確かめたいのは「21件あるときの一覧」であって、作成の画面ではありません。そこで、RustNote のリポジトリ（`LibsqlNoteRepository`）に、DB へ直接入れてもらいます。
- `app.db.db` は、テスト用の道具 `test_app()` が作った、一時フォルダの DB です。`UserId::new(1)` の 1 は、最初に登録した alice の ID です（`users.id` は 1 から順に振られます）。
- `NewNote::parse` は、タイトルと本文を検査して、保存してよい形にする関数です。リポジトリは検査済みの `NewNote` しか受け取らないので、画面を通さなくても、決まりを破るノートは入れられません（UNIT 03 のテーマです）。

### よくあるエラーと読み方

画面の設計でよくある3つの間違いを、わざと起こしてみます。次のファイルを `tests/mistakes.rs` として保存し、テストの名前で1つずつ実行します。

```rust
//! 画面の設計でよくある間違いを、わざと起こして返事を確かめる。
//! 実行: cargo test --test mistakes 名前 -- --nocapture

mod common;

use common::{body_text, test_app};
use topcoat::router::{header, response::Response};

const PASSWORD: &str = "correct horse battery";

// ステータス・Allow ヘッダー・本文の長さと、エラー画面の見出しと説明を表示する
async fn show(request: &str, response: Response) {
    println!("{request} -> {}", response.status());
    if let Some(allow) = response.headers().get(header::ALLOW) {
        println!("    allow: {}", allow.to_str().unwrap());
    }
    let html = body_text(response).await;
    println!("    本文: {} バイト", html.len());
    // エラー画面は <section class="error-page"><h1>…</h1><p>…</p> の形
    if let Some(start) = html.find(r#"<section class="error-page">"#) {
        let section = &html[start..];
        let h1 = between(section, "<h1>", "</h1>");
        let message = between(section, "<p>", "</p>");
        println!("    見出し: {h1}");
        println!("    説明: {message}");
    }
}

// text の中で、最初の begin と、その後の end にはさまれた部分を返す
fn between<'a>(text: &'a str, begin: &str, end: &str) -> &'a str {
    let Some(start) = text.find(begin).map(|i| i + begin.len()) else {
        return "";
    };
    let rest = &text[start..];
    let stop = rest.find(end).unwrap_or(rest.len());
    &rest[..stop]
}

// 間違い1: 「完了にする」をリンク（GET）にしてしまった
#[tokio::test]
async fn toggle_by_link() {
    let app = test_app().await;
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;
    let url = app.create_note(&cookie, "牛乳を買う", "").await;

    let path = format!("{url}/toggle");
    show(&format!("GET {path}"), app.get(&path, Some(&cookie)).await).await;
}

// 間違い2: フォームの送り先（action）を /note と書き間違えた
#[tokio::test]
async fn wrong_action() {
    let app = test_app().await;
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;

    let response = app.post_form("/note", "title=a&body=", Some(&cookie)).await;
    show("POST /note", response).await;
}

// 間違い3: フォームの項目名を間違えて、body が届かなかった／ページ番号が数字でない
#[tokio::test]
async fn unreadable_input() {
    let app = test_app().await;
    let cookie = app.sign_up("alice@example.com", PASSWORD).await;

    let response = app
        .post_form("/notes", "title=a&text=memo", Some(&cookie))
        .await;
    show("POST /notes（body なし）", response).await;
    let response = app.get("/notes?page=abc", Some(&cookie)).await;
    show("GET /notes?page=abc", response).await;
}
```

#### エラー1: 状態を変える操作を、リンク（GET）にした（405 Method Not Allowed）

詳細の画面の「完了にする」を、フォームのボタンではなく `<a href="/notes/1/toggle">完了にする</a>` のようなリンクにしてしまった場合です。リンクをクリックすると、ブラウザは GET を送ります。

```powershell
cargo test --test mistakes toggle_by_link -- --nocapture
```

```text
   Compiling rustnote v0.1.0 (...\rustnote)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.38s
     Running tests\mistakes.rs (target\debug\deps\mistakes-4b9f6b7878cdffc1.exe)

running 1 test
POST /signup -> 303 See Other (424.5596ms)
POST /notes -> 303 See Other (5.5717ms)
GET /notes/1/toggle -> 405 Method Not Allowed
    allow: POST
    本文: 0 バイト
test toggle_by_link ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.45s
```

- **何が起きたか:** 405 Method Not Allowed は「その URL は、そのメソッドでは受け付けません」という返事です。`allow: POST` が「この URL は POST だけです」と教えています。本文は 0 バイトなので、ブラウザには真っ白な画面が出ます。RustNote のアクセスログ（`GET /notes/1/toggle -> ...` の行）も出ていません。RustNote のページの処理まで届く前に、Topcoat が断っているからです。
- **なぜ GET にしてはいけないか:** GET は「見るだけ」の約束です。見るだけのはずの URL でノートが変わると、困ったことが起きます。たとえば Topcoat が標準で入れている、ほかのサイトからの送信を止める仕組み（OriginPolicy）は、GET・HEAD・OPTIONS 以外のリクエストだけを調べます。もし削除が GET だったら、悪いサイトに置かれたリンクや画像を開いただけで、ノートが消えるかもしれません。
- **直し方:** 状態を変える操作は、`<form method="post" action="...">` の中のボタンにします。RustNote の詳細の画面も、完了の切り替えと削除をフォームのボタンにしています（「RustNote ではここで使う」で見ます）。

#### エラー2: フォームの送り先の書き間違い（404 Not Found）

作成のフォームの `action` を、`/notes` ではなく `/note` と書き間違えた場合です。

```powershell
cargo test --test mistakes wrong_action -- --nocapture
```

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running tests\mistakes.rs (target\debug\deps\mistakes-4b9f6b7878cdffc1.exe)

running 1 test
POST /signup -> 303 See Other (396.6173ms)
POST /note -> 404 Not Found (2.0998ms)
POST /note -> 404 Not Found
    本文: 692 バイト
    見出し: 404 Not Found
    説明: ページが見つかりません
test wrong_action ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.41s
```

- **何が起きたか:** `/note` という URL は、どの画面にもありません。RustNote は、どこにも当てはまらない URL を、GET でも POST でも 404 のエラー画面にします（`src/routes.rs` の `not_found!();`）。こちらはアクセスログも出ていて、エラー画面にはヘッダー・見出し・説明・「トップページへ戻る」が入っています（本文 692 バイト）。
- **直し方:** `action` を `/notes` に直します。RustNote のコードでは、URL を文字列で手書きせず、`href!(create)` のように**処理の名前から URL を作る**ので、この書き間違いは起きにくくなっています（「RustNote ではここで使う」）。

#### エラー3: 読み取れない入力（400 Bad Request）

フォームの項目名を `body` ではなく `text` にしてしまい、本文が届かなかった場合と、ページ番号に数字でないもの（`?page=abc`）が来た場合です。

```powershell
cargo test --test mistakes unreadable_input -- --nocapture
```

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running tests\mistakes.rs (target\debug\deps\mistakes-4b9f6b7878cdffc1.exe)

running 1 test
POST /signup -> 303 See Other (481.16ms)
POST /notes -> 400 Bad Request (2.411ms)
POST /notes（body なし） -> 400 Bad Request
    本文: 715 バイト
    見出し: 400 Bad Request
    説明: 送信された内容を読み取れませんでした
GET /notes -> 400 Bad Request (2.0836ms)
GET /notes?page=abc -> 400 Bad Request
    本文: 715 バイト
    見出し: 400 Bad Request
    説明: 送信された内容を読み取れませんでした
test unreadable_input ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.53s
```

- **何が起きたか:** どちらも、「送られてきた形が、決めた形と違う」ので、RustNote は入力を受け取る段階で 400 で断り、「送信された内容を読み取れませんでした」というエラー画面を出しています。空のタイトル（入力の誤り）のときの 400 とは違い、フォームを出し直していません。正しい画面から送ればまず起きない、**作りの間違い**だからです。
- **直し方:** フォームの `name` 属性を、受け取る側の構造体の項目名（`title`・`body`）と同じにします。ページ送りのリンクも、手で組み立てずに RustNote の部品（`pagination`）に任せます。
- **設計で決めること:** 「利用者の入力の誤り」と「形の間違い」は、出し方を分けます。前者は利用者が直せるように、項目ごとのエラーを出してフォームを出し直します。後者は利用者には直しようがないので、短い説明とトップへの戻り道だけを出します。

### RustNote ではここで使う

RustNote の `src/routes.rs` の先頭には、ファイルと URL の対応表が、コメントで書いてあります（抜き出しています）。画面遷移図の画面と URL が、そのままファイルの並びになっています。

```rust
//! | ファイル | URL |
//! |---|---|
//! | routes.rs                 | GET /（トップ）、全ページ共通のレイアウト、404 の受け皿 |
//! | routes/health.rs          | GET /health |
//! | routes/signup.rs          | GET /signup・POST /signup |
//! | routes/login.rs           | GET /login・POST /login |
//! | routes/logout.rs          | POST /logout |
//! | routes/notes.rs           | GET /notes（?q= 検索・?page= ページ送り）・POST /notes |
//! | routes/notes/id.rs        | GET /notes/{id} |
//! | routes/notes/id/edit.rs   | GET /notes/{id}/edit・POST /notes/{id}/edit |
//! | routes/notes/id/toggle.rs | POST /notes/{id}/toggle |
//! | routes/notes/id/delete.rs | POST /notes/{id}/delete |
```

ノートの作成（`src/routes/notes.rs` の `create`）は、PRG と「400 で出し直す」の形そのものです。

```rust
#[page(POST)]
pub async fn create(cx: &Cx, Form(input): Form<NoteForm>) -> Result<impl View> {
    let user = require_user(cx).await?;

    let errors = match NewNote::parse(&input.title, &input.body) {
        Ok(new_note) => {
            let note = app_state(cx)
                .notes
                .insert(user.id, &new_note)
                .await
                .map_err(AppError::from)?;
            flash::set(cx, Flash::NoteCreated);
            // PRG: 作成できたら 303 で詳細ページ（GET）へ
            return Err(see_other(href!(id::show, id::Id(note.id)).resolve(cx)).into());
        }
        Err(errors) => errors,
    };

    // 入力に問題があった: 400 で一覧と入力済みのフォームを再表示する
    let form = NoteFormState::from_input(&input.title, &input.body).with_errors(&errors);
    Ok(view! {
        (StatusCode::BAD_REQUEST)
        notes_index(owner: user.id, query: NoteQuery::new(None, None), form: form)
    })
}
```

- 最初の `require_user(cx)` が、ログインしていなければ 303 でログイン画面へ送ります（UNIT 01 の AC-02）。
- 検査に合格したら保存し、`flash::set` でお知らせを予約して、`see_other(...)` で 303 を返します。移動先の URL は、`href!(id::show, id::Id(note.id))` のように、**詳細の画面の処理（`show`）の名前**から作っています。URL を文字列で手書きしないので、エラー2のような書き間違いが起きにくく、URL を変えたときもリンクがずれません。
- 検査に失敗したら、`NoteFormState::from_input` で入力した内容を残し、`(StatusCode::BAD_REQUEST)` を付けて、一覧の画面を出し直しています。
- Topcoat のページの処理は、成功の値（`Ok`）が「画面」なので、リダイレクトは `Err(...)` の側から返します。少し不思議に見えますが、「画面を返さずに、ここで処理を打ち切って移動させる」という意味だと読むと分かりやすいでしょう。

詳細の画面（`src/routes/notes/id.rs` の `show`）の操作のボタンは、どれも `method="post"` のフォームです（抜き出しています）。

```rust
        <div class="note-actions">
            <a class="button" href=(href!(edit::edit_page, Id(id)))>"編集"</a>
            <form method="post" action=(href!(toggle::toggle, Id(id)))>
                <button type="submit" class="secondary">
                    if note.done {
                        "未完了に戻す"
                    } else {
                        "完了にする"
                    }
                </button>
            </form>
            <form method="post" action=(href!(delete::delete, Id(id)))>
                <button type="submit" class="danger">"削除"</button>
            </form>
        </div>
```

- 「編集」は、編集の画面を**開くだけ**なのでリンク（GET）です。「完了にする」と「削除」は、ノートを**変える**のでフォームの POST です。エラー1の約束どおりです。
- ボタンの文は、状態に合わせて「完了にする」と「未完了に戻す」を出し分けています。これも画面の「状態」の1つです。

最後に、狭い画面（スマートフォンなど）についてです。RustNote のレイアウトは `<meta name="viewport" content="width=device-width, initial-scale=1">` を入れています。CSS（`assets/style.css`）では、本文の枠を `max-width: 760px` と左右 `16px` の余白で作り、一覧の1行は `flex-wrap: wrap`（入りきらなければ折り返す）にしています。画面の幅で見た目を切り替える `@media` の指定はありません。実際の見え方を確かめて直すのは、UNIT 09（準備中）で扱います。

### 練習問題

**問1:** 詳細の画面に「複製する」ボタン（同じタイトルと本文で、新しいノートをもう1件作る）を足すことになりました。RustNote の決まりに合わせて、次の4つを設計してください。

1. メソッドと URL
2. 成功したときの返事と移動先
3. 他人のノート・存在しないノートで押されたときの返事
4. ボタンを置く場所と、HTML の形（リンクかフォームか）

**問2:** 削除の PRG を、例2-2と同じ形のテストで確かめてください。`tests/flow.rs` に `delete_note_with_prg` というテストを足し、削除の POST の返事（ステータス・移動先・フラッシュのクッキー）と、移動先の画面に「ノートを削除しました」が出て、消したノートのタイトルが無いことを表示します。

**問3:** 一覧の画面の検索欄は、次の HTML になっています（`src/routes/notes.rs` の `view!` から作られる形）。

```html
<form method="get" action="/notes" class="search-form">
  <input name="q" type="search" placeholder="キーワードで検索">
  <button type="submit">検索</button>
</form>
```

登録やログインの画面の入力欄と比べて、足りないものがあります。それは何で、なぜ必要でしょうか。直すなら、どう書きますか。

### 解答と解説

**問1の解答（例）:**

1. `POST /notes/{id}/duplicate`。ノートを1件**増やす**操作なので、GET ではなく POST にします。完了の切り替え（`…/toggle`）や削除（`…/delete`）と同じく、`/notes/{id}` の下に置きます。RustNote のファイルの並びなら `src/routes/notes/id/duplicate.rs` です。
2. 成功したら、303 で**新しくできたノート**の詳細（`/notes/{新しい id}`）へ移し、「ノートを複製しました」のようなフラッシュを出します。作成と同じく、「できたものをすぐ確かめられる」ようにするためです。
3. 404。他人のノートも、存在しないノートも、AC-05・AC-06 と同じく区別せずに 404 にします。
4. 詳細の画面の操作のボタンの並び（`note-actions`）に、`<form method="post" action=...>` のボタンとして置きます。リンクにはしません（エラー1）。

- 新しい機能を足すときも、まず受入条件（UNIT 01）と画面遷移図に書き足してから作ります。そうすると、「複製したら、どの画面へ移るか」のような細かいことを、作っている途中で迷わずに済みます。

**問2の解答:**

```rust

#[tokio::test]
async fn delete_note_with_prg() {
    let app = test_app().await;
    let session = app.sign_up("alice@example.com", PASSWORD).await;
    let url = app.create_note(&session, "牛乳を買う", "").await;

    // 1) 削除のフォームを送る（POST）
    let path = format!("{url}/delete");
    let response = app.post_form(&path, "", Some(&session)).await;
    println!("1) POST {path} -> {}", response.status());
    println!("   Location: {}", location(&response));
    let set_cookie = flash_set_cookie(&response).expect("flash がありません");
    println!("   Set-Cookie: {set_cookie}");
    let next = location(&response).to_string();

    // 2) 移動先の一覧を開く。「ノートを削除しました」が出て、ノートは消えている
    let flash = set_cookie.split(';').next().unwrap();
    let cookies = format!("{session}; {flash}");
    let response = app.get(&next, Some(&cookies)).await;
    println!("2) GET {next} -> {}", response.status());
    let html = body_text(response).await;
    println!(
        "   「ノートを削除しました」が出た: {}",
        yes_no(html.contains("ノートを削除しました"))
    );
    println!(
        "   「牛乳を買う」が残っている: {}",
        yes_no(html.contains("牛乳を買う"))
    );
}
```

`tests/flow.rs` の最後に足して、このテストだけを実行した結果です。

```powershell
cargo test --test flow delete_note_with_prg -- --nocapture
```

```text
   Compiling rustnote v0.1.0 (...\rustnote)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.34s
     Running tests\flow.rs (target\debug\deps\flow-4a51733f40431b48.exe)

running 1 test
POST /signup -> 303 See Other (468.4094ms)
POST /notes -> 303 See Other (8.3214ms)
POST /notes/1/delete -> 303 See Other (6.691ms)
1) POST /notes/1/delete -> 303 See Other
   Location: /notes
   Set-Cookie: flash=note_deleted; HttpOnly; SameSite=Lax; Path=/; Max-Age=300
GET /notes -> 200 OK (5.2852ms)
2) GET /notes -> 200 OK
   「ノートを削除しました」が出た: はい
   「牛乳を買う」が残っている: いいえ
test delete_note_with_prg ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.50s
```

- 削除の移動先は、詳細ではなく一覧（`Location: /notes`）です。消したノートの詳細へ移っても、404 になるだけだからです。画面遷移図の「…/delete → 303」の矢印のとおりです。
- `flash_set_cookie` と `yes_no` は、例2-2で作った関数を、同じファイルの中でそのまま使っています。

**問3の解答:**

- 足りないのは、入力欄の**ラベル**（`<label>`）です。登録やログインの画面では、`<label for="email">メールアドレス</label>` のように、入力欄ごとに名前が付いています。検索欄には `placeholder`（入力欄の中にうすく出る見本の文）しかありません。
- `placeholder` は、文字を入力し始めると消えてしまいます。また、読み上げソフトや音声入力では、ラベルが入力欄の名前として使われます。ラベルが無いと、「何を入れる欄なのか」が伝わらないことがあります。
- 直し方の例です。見た目のラベルを置きたくないときは、`aria-label` で、目には見えない名前を付ける方法もあります。

```html
<form method="get" action="/notes" class="search-form">
  <label for="q">ノートを検索</label>
  <input id="q" name="q" type="search" placeholder="キーワードで検索">
  <button type="submit">検索</button>
</form>
```

- これは、完成した RustNote にも残っている改善点です。完成したアプリでも、画面の設計を見直すと、こうした抜けが見つかります。見た目と使いやすさの決まりは、UNIT 09（準備中）でまとめて確かめます。

### まとめ

- 画面は「いちばんの目的」を1つ決めて一覧にし、画面遷移図で、見る操作（GET）と変える操作（POST）を分ける。POST が成功したら、303 で GET の画面へ移す（PRG）。再読み込みで二重に保存されず、お知らせも一度だけ出せる。
- 1つの画面が持つ状態（空・入力エラー・ページ送り・検索で0件・エラー画面）を表にし、それぞれの文をテストで確かめる。「空」と「検索で0件」のように、似ていても利用者の気持ちが違う状態は、文を分ける。
- 状態を変える操作をリンク（GET）にすると、405 になるうえ、ほかのサイトからの送信を止める仕組みも働かない。変える操作は、必ずフォームの POST にする。

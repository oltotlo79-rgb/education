<div style="page-break-after: always; text-align: center; margin: 0; padding: 0;">
<img src="assets/cover.jpg" alt="表紙" style="width: 100%; max-height: 175mm; object-fit: cover; border-radius: 10px;">
<div style="margin-top: 14px; font-size: 11px; letter-spacing: 0.18em; color: #c2410c; font-weight: 700;">VOLUME 00 / RUST BACKEND MASTERY</div>
<div style="font-size: 26px; font-weight: 800; color: #0f172a; margin-top: 6px; line-height: 1.35;">学習ガイド — コンピュータとWebから完成像まで</div>
<div style="font-size: 13px; color: #475569; margin-top: 8px;">プログラミング未経験から、Rust で作る Web アプリ「RustNote」の全体像と、全14巻の歩き方をつかむ</div>
</div>

# 第0巻: 学習ガイド — コンピュータとWebから完成像まで

> この巻は、全14巻からなる「Rust バックエンド教材」の**入口**です。プログラミングをしたことがない人を想定して、「コンピュータの中で何が起きているのか」「Web アプリはどうやって動いているのか」を、たとえ話と図で1つずつ説明します。そのうえで、このシリーズを通して作るメモアプリ **RustNote（ラストノート）** の完成像と、14巻をどの順番で読めばよいかを示します。
>
> 難しい文法はまだ覚えなくて大丈夫です。この巻のコードは「動かして、出力を見て、何が起きたかを言葉で説明する」ためのものです。

## この巻で身に付けること

### 到達目標

この巻を読み終えると、次のことができるようになります。

- **コンピュータの部品と OS の役割**を、たとえ話を使って自分の言葉で説明できる
- **ファイル・フォルダ・パス**を区別し、「今いるフォルダ」を意識してコマンドを打てる
- **PowerShell（パワーシェル）**で基本のコマンドを打ち、Rust のプログラムをコンパイルして実行できる
- ブラウザに URL を入れてから画面が出るまでに起きる **HTTP のやりとり**を、順番に説明できる
- **サーバー・ポート・localhost** の意味が分かり、Rust だけで小さな Web サーバーを動かせる
- **データベース**・**認証**・**認可**が、なぜ Web アプリに必要なのかを説明できる
- Rust コンパイラの**エラーメッセージを部品ごとに読んで**、自分で直せる
- RustNote の完成像と、全14巻の読み進め方を把握して、学習の計画を立てられる

### このシリーズで作るもの: RustNote

このシリーズでは、最終的に **RustNote** という「ログインして使うメモ帳」の Web アプリを作り、インターネットに公開します。最初に完成した姿を見ておくと、各巻で学ぶことが「全体のどこの部品なのか」が分かりやすくなります。

RustNote でできることは次のとおりです。

- メールアドレスとパスワードで**アカウントを作る（登録）**、**ログイン**、**ログアウト**する
- ログインした人だけが、**自分のノート**を一覧・作成・表示・編集・削除できる
- ノートに「済み（done）」の印を付けたり外したりできる
- キーワードで**検索**でき、ノートが多いときは**ページ送り**で少しずつ表示する
- 他の人のノートは、URL を直接打ち込んでも**絶対に見られない**

> **用語:** **Web アプリ（ウェブアプリ）** … ブラウザ（Chrome や Edge など）で開いて使うアプリのことです。スマホのアプリのようにインストールしなくても、URL を開くだけで使えます。

#### 完成画面のイメージ

次の2枚は、RustNote の完成画面のイメージです（実際の色や配置は第9巻・第10巻で一緒に作りながら決めます）。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 400" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-01" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#c2410c"/></marker>
</defs>
<rect x="10" y="10" width="700" height="380" rx="12" fill="#ffffff" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="10" y="10" width="700" height="44" rx="12" fill="#e2e8f0"/>
<rect x="10" y="40" width="700" height="14" fill="#e2e8f0"/>
<circle cx="34" cy="32" r="6" fill="#f87171"/>
<circle cx="54" cy="32" r="6" fill="#fbbf24"/>
<circle cx="74" cy="32" r="6" fill="#34d399"/>
<rect x="100" y="19" width="520" height="26" rx="13" fill="#ffffff" stroke="#cbd5e1"/>
<text x="118" y="37" font-size="13" fill="#334155">https://your-app.fly.dev/login</text>
<rect x="11" y="54" width="698" height="46" fill="#1e40af"/>
<text x="34" y="84" font-size="19" font-weight="700" fill="#ffffff">RustNote</text>
<text x="686" y="82" font-size="13" fill="#dbeafe" text-anchor="end">ログイン ｜ 新規登録</text>
<rect x="210" y="118" width="300" height="252" rx="12" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="360" y="152" font-size="18" font-weight="700" fill="#0f172a" text-anchor="middle">ログイン</text>
<text x="234" y="182" font-size="13" fill="#475569">メールアドレス</text>
<rect x="234" y="190" width="252" height="30" rx="6" fill="#ffffff" stroke="#94a3b8"/>
<text x="244" y="210" font-size="13" fill="#0f172a">hanako@example.com</text>
<text x="234" y="242" font-size="13" fill="#475569">パスワード</text>
<rect x="234" y="250" width="252" height="30" rx="6" fill="#ffffff" stroke="#94a3b8"/>
<text x="244" y="270" font-size="12" fill="#0f172a">●●●●●●●●</text>
<rect x="234" y="294" width="252" height="34" rx="8" fill="#3b82f6"/>
<text x="360" y="316" font-size="14" font-weight="700" fill="#ffffff" text-anchor="middle">ログイン</text>
<text x="360" y="352" font-size="12" fill="#1d4ed8" text-anchor="middle">アカウントがない方は「新規登録」へ</text>
<text x="30" y="190" font-size="13" font-weight="700" fill="#c2410c">① GET /login</text>
<text x="30" y="208" font-size="12" fill="#c2410c">でこの画面を表示</text>
<line x1="150" y1="200" x2="200" y2="200" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-01)"/>
<text x="540" y="306" font-size="13" font-weight="700" fill="#c2410c">② POST /login</text>
<text x="540" y="324" font-size="12" fill="#c2410c">へ入力内容を送る</text>
<line x1="534" y1="311" x2="494" y2="311" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-01)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図0-1: RustNote のログイン画面（完成イメージ）。表示は GET、送信は POST という別々のリクエストになる</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 470" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="700" height="450" rx="12" fill="#ffffff" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="10" y="10" width="700" height="44" rx="12" fill="#e2e8f0"/>
<rect x="10" y="40" width="700" height="14" fill="#e2e8f0"/>
<circle cx="34" cy="32" r="6" fill="#f87171"/>
<circle cx="54" cy="32" r="6" fill="#fbbf24"/>
<circle cx="74" cy="32" r="6" fill="#34d399"/>
<rect x="100" y="19" width="520" height="26" rx="13" fill="#ffffff" stroke="#cbd5e1"/>
<text x="118" y="37" font-size="13" fill="#334155">https://your-app.fly.dev/notes</text>
<rect x="11" y="54" width="698" height="46" fill="#1e40af"/>
<text x="34" y="84" font-size="19" font-weight="700" fill="#ffffff">RustNote</text>
<text x="560" y="82" font-size="13" fill="#dbeafe" text-anchor="end">hanako@example.com</text>
<rect x="576" y="64" width="110" height="28" rx="6" fill="#ffffff" fill-opacity="0.15" stroke="#dbeafe"/>
<text x="631" y="83" font-size="13" fill="#ffffff" text-anchor="middle">ログアウト</text>
<rect x="36" y="116" width="520" height="32" rx="6" fill="#ffffff" stroke="#94a3b8"/>
<text x="48" y="137" font-size="13" fill="#94a3b8">キーワードで検索（例: 買い物）</text>
<rect x="566" y="116" width="118" height="32" rx="6" fill="#475569"/>
<text x="625" y="137" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">検索</text>
<rect x="36" y="160" width="648" height="86" rx="10" fill="#ecfdf5" stroke="#10b981"/>
<text x="52" y="182" font-size="13" font-weight="700" fill="#047857">新しいノート</text>
<rect x="52" y="192" width="230" height="30" rx="6" fill="#ffffff" stroke="#94a3b8"/>
<text x="62" y="212" font-size="12" fill="#94a3b8">タイトル（1〜80文字）</text>
<rect x="292" y="192" width="270" height="30" rx="6" fill="#ffffff" stroke="#94a3b8"/>
<text x="302" y="212" font-size="12" fill="#94a3b8">本文（なくてもよい）</text>
<rect x="572" y="192" width="96" height="30" rx="6" fill="#10b981"/>
<text x="620" y="212" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">作成</text>
<text x="52" y="238" font-size="12" fill="#047857">送信すると POST /notes → 303 で一覧へ戻る</text>
<rect x="36" y="258" width="648" height="46" rx="8" fill="#ffffff" stroke="#e2e8f0"/>
<rect x="52" y="272" width="18" height="18" rx="3" fill="#ffffff" stroke="#475569" stroke-width="1.5"/>
<text x="84" y="287" font-size="14" fill="#0f172a">牛乳を買う</text>
<text x="420" y="287" font-size="12" fill="#64748b">10/15 09:12 更新</text>
<text x="560" y="287" font-size="13" fill="#1d4ed8">編集</text>
<text x="620" y="287" font-size="13" fill="#dc2626">削除</text>
<rect x="36" y="310" width="648" height="46" rx="8" fill="#f8fafc" stroke="#e2e8f0"/>
<rect x="52" y="324" width="18" height="18" rx="3" fill="#10b981" stroke="#047857" stroke-width="1.5"/>
<path d="M55,333 L60,338 L67,328" fill="none" stroke="#ffffff" stroke-width="2.2"/>
<text x="84" y="339" font-size="14" fill="#94a3b8">Rust の本を読む</text>
<line x1="84" y1="334" x2="190" y2="334" stroke="#94a3b8" stroke-width="1.2"/>
<text x="420" y="339" font-size="12" fill="#64748b">10/14 21:40 更新</text>
<text x="560" y="339" font-size="13" fill="#1d4ed8">編集</text>
<text x="620" y="339" font-size="13" fill="#dc2626">削除</text>
<rect x="36" y="362" width="648" height="46" rx="8" fill="#ffffff" stroke="#e2e8f0"/>
<rect x="52" y="376" width="18" height="18" rx="3" fill="#ffffff" stroke="#475569" stroke-width="1.5"/>
<text x="84" y="391" font-size="14" fill="#0f172a">週末の買い物リスト</text>
<text x="420" y="391" font-size="12" fill="#64748b">10/13 18:05 更新</text>
<text x="560" y="391" font-size="13" fill="#1d4ed8">編集</text>
<text x="620" y="391" font-size="13" fill="#dc2626">削除</text>
<text x="360" y="440" font-size="13" fill="#475569" text-anchor="middle">« 前へ　　1 / 3 ページ　　次へ »</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図0-2: ログイン後のノート一覧画面（完成イメージ）。チェック済みのノートは「済み（done）」</div>
</div>

画面の部品と、そのとき裏でブラウザが送るリクエスト（お願い）の対応は次のとおりです。今は「部品ごとに決まった URL がある」とだけ分かれば十分です。URL やメソッド（GET・POST）は UNIT 05 で説明します。

| 画面の部品 | ブラウザが送るリクエスト | サーバーがすること |
|---|---|---|
| 一覧を開く・検索・ページ送り | `GET /notes`（`?q=買い物` で検索） | ログイン中の人のノートだけを探して HTML で返す |
| 「作成」ボタン | `POST /notes` | タイトルを検査して保存し、一覧へ移動させる（303） |
| タイトルをクリック | `GET /notes/{id}` | そのノートの詳細を返す（他人のなら 404） |
| 「編集」 | `GET /notes/{id}/edit` → `POST /notes/{id}/edit` | 編集フォームを返す → 変更を保存する |
| チェックボックス | `POST /notes/{id}/toggle` | 済み／未済みを切り替える |
| 「削除」 | `POST /notes/{id}/delete` | ノートを削除する |
| 「ログアウト」 | `POST /logout` | ログイン状態を終わらせる |

#### 全体の構成

RustNote は、次の3つの登場人物でできています。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 340" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-02" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="12" y="86" width="146" height="160" rx="12" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="85" y="116" font-size="16" font-weight="700" fill="#1e40af" text-anchor="middle">ブラウザ</text>
<text x="85" y="142" font-size="12" fill="#334155" text-anchor="middle">あなたの PC・スマホ</text>
<text x="85" y="162" font-size="12" fill="#334155" text-anchor="middle">Chrome / Edge など</text>
<rect x="30" y="180" width="110" height="48" rx="6" fill="#ffffff" stroke="#93c5fd"/>
<text x="85" y="200" font-size="12" fill="#1e40af" text-anchor="middle">受け取った HTML</text>
<text x="85" y="218" font-size="12" fill="#1e40af" text-anchor="middle">を画面に描く</text>
<rect x="236" y="30" width="250" height="276" rx="14" fill="#fff7ed" stroke="#f97316" stroke-width="2" stroke-dasharray="6 4"/>
<text x="361" y="54" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">Fly.io（クラウド上の PC）</text>
<rect x="254" y="68" width="214" height="222" rx="10" fill="#ffffff" stroke="#c2410c" stroke-width="2"/>
<text x="361" y="94" font-size="15" font-weight="700" fill="#0f172a" text-anchor="middle">RustNote サーバー</text>
<text x="361" y="114" font-size="12" fill="#475569" text-anchor="middle">Rust + Topcoat 0.10</text>
<text x="361" y="132" font-size="12" fill="#475569" text-anchor="middle">0.0.0.0:8080 で待ち受け</text>
<rect x="270" y="144" width="182" height="30" rx="6" fill="#fff7ed" stroke="#fdba74"/>
<text x="361" y="164" font-size="12" fill="#9a3412" text-anchor="middle">ルーティング（URL→処理）</text>
<rect x="270" y="180" width="182" height="30" rx="6" fill="#fff7ed" stroke="#fdba74"/>
<text x="361" y="200" font-size="12" fill="#9a3412" text-anchor="middle">認証・セッション・認可</text>
<rect x="270" y="216" width="182" height="30" rx="6" fill="#fff7ed" stroke="#fdba74"/>
<text x="361" y="236" font-size="12" fill="#9a3412" text-anchor="middle">ノートの作成・更新・削除</text>
<rect x="270" y="252" width="182" height="30" rx="6" fill="#fff7ed" stroke="#fdba74"/>
<text x="361" y="272" font-size="12" fill="#9a3412" text-anchor="middle">HTML を組み立てて返す</text>
<rect x="562" y="86" width="146" height="160" rx="12" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="635" y="114" font-size="15" font-weight="700" fill="#047857" text-anchor="middle">Turso</text>
<text x="635" y="134" font-size="12" fill="#334155" text-anchor="middle">（libSQL の DB）</text>
<text x="635" y="160" font-size="12" fill="#047857" text-anchor="middle">users</text>
<text x="635" y="180" font-size="12" fill="#047857" text-anchor="middle">notes</text>
<text x="635" y="200" font-size="12" fill="#047857" text-anchor="middle">sessions</text>
<text x="635" y="220" font-size="12" fill="#047857" text-anchor="middle">schema_migrations</text>
<line x1="160" y1="140" x2="250" y2="140" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-02)"/>
<text x="198" y="112" font-size="12" fill="#1e40af" text-anchor="middle">リクエスト</text>
<text x="198" y="130" font-size="12" fill="#1e40af" text-anchor="middle">GET /notes</text>
<line x1="250" y1="196" x2="160" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-02)"/>
<text x="198" y="216" font-size="12" fill="#1e40af" text-anchor="middle">レスポンス</text>
<text x="198" y="234" font-size="12" fill="#1e40af" text-anchor="middle">HTML</text>
<line x1="470" y1="140" x2="558" y2="140" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-02)"/>
<text x="514" y="112" font-size="12" fill="#047857" text-anchor="middle">SQL で</text>
<text x="514" y="130" font-size="12" fill="#047857" text-anchor="middle">問い合わせ</text>
<line x1="558" y1="196" x2="470" y2="196" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-02)"/>
<text x="514" y="216" font-size="12" fill="#047857" text-anchor="middle">結果の行</text>
<text x="360" y="330" font-size="12" fill="#64748b" text-anchor="middle">開発中は、同じ組み立てを自分の PC の中で動かします（DB はローカルのファイル）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図0-3: RustNote のシステム構成。ブラウザ ⇔ Fly.io 上の Rust サーバー ⇔ Turso データベース</div>
</div>

| 登場人物 | 役割 | たとえると | 主に学ぶ巻 |
|---|---|---|---|
| ブラウザ | URL を開き、サーバーから届いた HTML を画面に描く | お店に来たお客さん | 第0巻・第6巻 |
| RustNote サーバー | リクエストを受け取り、ログインの確認・ノートの処理をして HTML を返す。**Rust** で書き、Web アプリ用の部品集 **Topcoat（トップコート）** を使う | お店の店員 | 第2〜6巻・第8〜10巻 |
| Turso（ターソ） | ユーザー・ノート・ログイン状態を、消えないように保存しておく**データベース** | お店の帳簿・倉庫 | 第7巻 |
| Fly.io（フライアイオー） | サーバーのプログラムを、インターネットから届く場所で動かしてくれる**クラウドサービス** | お店を出す貸しテナント | 第12巻 |

> **用語:** **フレームワーク** … アプリを作るときに毎回必要になる「よくある仕組み」（URL ごとに処理を振り分ける、フォームの値を受け取る、など）をまとめて用意してくれる部品集です。Topcoat は Rust 用の Web フレームワークの1つで、このシリーズでは 0.10 系を使います。まだ若いフレームワークなので、版が変わると書き方が変わることがあります（第1巻・第6巻で詳しく扱います）。

### シリーズ全14巻の地図

14巻は、大きく4つのまとまりに分かれています。上から順に読むのが基本です。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 452" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-03" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="10" y="14" width="96" height="84" rx="10" fill="#f8fafc" stroke="#475569"/>
<text x="58" y="52" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">入門編</text>
<text x="58" y="72" font-size="12" fill="#64748b" text-anchor="middle">準備する</text>
<rect x="10" y="126" width="96" height="84" rx="10" fill="#eff6ff" stroke="#3b82f6"/>
<text x="58" y="164" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">Rust 言語編</text>
<text x="58" y="184" font-size="12" fill="#1e40af" text-anchor="middle">書けるようになる</text>
<rect x="10" y="238" width="96" height="84" rx="10" fill="#fff7ed" stroke="#f97316"/>
<text x="58" y="276" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">Web 土台編</text>
<text x="58" y="296" font-size="12" fill="#c2410c" text-anchor="middle">部品を知る</text>
<rect x="10" y="350" width="96" height="84" rx="10" fill="#ecfdf5" stroke="#10b981"/>
<text x="58" y="388" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">RustNote 編</text>
<text x="58" y="408" font-size="12" fill="#047857" text-anchor="middle">作って公開する</text>
<line x1="58" y1="100" x2="58" y2="122" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-03)"/>
<line x1="58" y1="212" x2="58" y2="234" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-03)"/>
<line x1="58" y1="324" x2="58" y2="346" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-03)"/>
<rect x="120" y="20" width="108" height="72" rx="8" fill="#ffedd5" stroke="#c2410c" stroke-width="3"/>
<text x="174" y="44" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">第0巻（この巻）</text>
<text x="174" y="64" font-size="12" fill="#0f172a" text-anchor="middle">学習ガイド</text>
<text x="174" y="82" font-size="12" fill="#0f172a" text-anchor="middle">PC と Web の基本</text>
<rect x="240" y="20" width="108" height="72" rx="8" fill="#ffffff" stroke="#475569"/>
<text x="294" y="44" font-size="12" font-weight="700" fill="#334155" text-anchor="middle">第1巻</text>
<text x="294" y="64" font-size="12" fill="#0f172a" text-anchor="middle">環境構築</text>
<text x="294" y="82" font-size="12" fill="#0f172a" text-anchor="middle">道具をそろえる</text>
<line x1="230" y1="56" x2="236" y2="56" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-03)"/>
<rect x="120" y="132" width="108" height="72" rx="8" fill="#ffffff" stroke="#3b82f6"/>
<text x="174" y="156" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">第2巻</text>
<text x="174" y="176" font-size="12" fill="#0f172a" text-anchor="middle">Rust 基礎</text>
<text x="174" y="194" font-size="12" fill="#0f172a" text-anchor="middle">値・型・制御</text>
<rect x="240" y="132" width="108" height="72" rx="8" fill="#ffffff" stroke="#3b82f6"/>
<text x="294" y="156" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">第3巻</text>
<text x="294" y="176" font-size="12" fill="#0f172a" text-anchor="middle">所有権・借用</text>
<text x="294" y="194" font-size="12" fill="#0f172a" text-anchor="middle">ライフタイム</text>
<rect x="360" y="132" width="108" height="72" rx="8" fill="#ffffff" stroke="#3b82f6"/>
<text x="414" y="156" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">第4巻</text>
<text x="414" y="176" font-size="12" fill="#0f172a" text-anchor="middle">型・trait</text>
<text x="414" y="194" font-size="12" fill="#0f172a" text-anchor="middle">エラー・テスト</text>
<rect x="480" y="132" width="108" height="72" rx="8" fill="#ffffff" stroke="#3b82f6"/>
<text x="534" y="156" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">第5巻</text>
<text x="534" y="176" font-size="12" fill="#0f172a" text-anchor="middle">async</text>
<text x="534" y="194" font-size="12" fill="#0f172a" text-anchor="middle">並行処理・運用</text>
<line x1="230" y1="168" x2="236" y2="168" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-03)"/>
<line x1="350" y1="168" x2="356" y2="168" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-03)"/>
<line x1="470" y1="168" x2="476" y2="168" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-03)"/>
<rect x="120" y="244" width="108" height="72" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="174" y="268" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">第6巻</text>
<text x="174" y="288" font-size="12" fill="#0f172a" text-anchor="middle">HTTP と</text>
<text x="174" y="306" font-size="12" fill="#0f172a" text-anchor="middle">Topcoat 0.10</text>
<rect x="240" y="244" width="108" height="72" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="294" y="268" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">第7巻</text>
<text x="294" y="288" font-size="12" fill="#0f172a" text-anchor="middle">Turso・SQL</text>
<text x="294" y="306" font-size="12" fill="#0f172a" text-anchor="middle">データの保存</text>
<rect x="360" y="244" width="108" height="72" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="414" y="268" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">第8巻</text>
<text x="414" y="288" font-size="12" fill="#0f172a" text-anchor="middle">認証・認可</text>
<text x="414" y="306" font-size="12" fill="#0f172a" text-anchor="middle">Argon2id</text>
<line x1="230" y1="280" x2="236" y2="280" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-03)"/>
<line x1="350" y1="280" x2="356" y2="280" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-03)"/>
<rect x="120" y="356" width="108" height="72" rx="8" fill="#ffffff" stroke="#10b981"/>
<text x="174" y="380" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">第9巻</text>
<text x="174" y="400" font-size="12" fill="#0f172a" text-anchor="middle">要件・設計</text>
<text x="174" y="418" font-size="12" fill="#0f172a" text-anchor="middle">構成・準備</text>
<rect x="240" y="356" width="108" height="72" rx="8" fill="#ffffff" stroke="#10b981"/>
<text x="294" y="380" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">第10巻</text>
<text x="294" y="400" font-size="12" fill="#0f172a" text-anchor="middle">CRUD と画面</text>
<text x="294" y="418" font-size="12" fill="#0f172a" text-anchor="middle">完全実装</text>
<rect x="360" y="356" width="108" height="72" rx="8" fill="#ffffff" stroke="#10b981"/>
<text x="414" y="380" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">第11巻</text>
<text x="414" y="400" font-size="12" fill="#0f172a" text-anchor="middle">テスト・安全</text>
<text x="414" y="418" font-size="12" fill="#0f172a" text-anchor="middle">品質保証</text>
<rect x="480" y="356" width="108" height="72" rx="8" fill="#ffffff" stroke="#10b981"/>
<text x="534" y="380" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">第12巻</text>
<text x="534" y="400" font-size="12" fill="#0f172a" text-anchor="middle">Fly.io に公開</text>
<text x="534" y="418" font-size="12" fill="#0f172a" text-anchor="middle">運用・復旧</text>
<rect x="600" y="356" width="108" height="72" rx="8" fill="#ffffff" stroke="#10b981"/>
<text x="654" y="380" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">第13巻</text>
<text x="654" y="400" font-size="12" fill="#0f172a" text-anchor="middle">完全コード</text>
<text x="654" y="418" font-size="12" fill="#0f172a" text-anchor="middle">演習・用語集</text>
<line x1="230" y1="392" x2="236" y2="392" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-03)"/>
<line x1="350" y1="392" x2="356" y2="392" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-03)"/>
<line x1="470" y1="392" x2="476" y2="392" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-03)"/>
<line x1="590" y1="392" x2="596" y2="392" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-03)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図0-4: シリーズ全14巻の地図。左の4段を上から下へ、各段の中は左から右へ読む</div>
</div>

| 巻 | テーマ | 読み終えるとできること |
|---|---|---|
| 第0巻 | 学習ガイド（この巻） | コンピュータと Web の基本が分かり、全体の地図を持てる |
| 第1巻 | 環境構築 | PowerShell・Git・VS Code・Rust・Topcoat・Turso・Fly.io の道具を自分の PC にそろえられる |
| 第2巻 | Rust 基礎 | 変数・型・関数・条件分岐・くり返し・構造体などで、小さなプログラムを書ける |
| 第3巻 | 所有権・借用・ライフタイム | Rust 独特の「値の持ち主」のルールを理解し、コンパイラと仲良くなれる |
| 第4巻 | 型・trait・エラー・テスト | 大きめのプログラムを、型とエラー処理とテストで安全に組み立てられる |
| 第5巻 | async・並行処理 | たくさんのリクエストを同時にさばくための非同期処理が分かる |
| 第6巻 | HTTP と Topcoat 0.10 | Topcoat で Web ページを返し、フォームを受け取れる |
| 第7巻 | Turso・SQL | テーブルを設計し、SQL でデータを保存・検索できる |
| 第8巻 | 認証・認可 | 安全な登録・ログイン・ログアウトと、「自分のデータだけ触れる」仕組みを作れる |
| 第9巻 | RustNote 設計・準備 | 要件と画面を決め、プロジェクトの骨組みを作れる |
| 第10巻 | RustNote 完全実装 | ノートの一覧・作成・編集・削除・検索・ページ送りを完成させられる |
| 第11巻 | 品質保証 | テスト・セキュリティ確認・ログ・性能測定で品質を確かめられる |
| 第12巻 | Fly.io 公開・運用 | Docker で包んでインターネットに公開し、障害から復旧できる |
| 第13巻 | 完全コード・演習・用語集 | 完成したコードの全体を読み通し、総合演習で力を確かめられる |

### この巻の学習地図

この巻の10の単元（UNIT）は、「コンピュータの基本」→「Web の基本」→「学び方」の順に並んでいます。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-04" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="20" width="122" height="78" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="75" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">UNIT 01</text>
<text x="75" y="66" font-size="13" fill="#0f172a" text-anchor="middle">コンピュータ</text>
<text x="75" y="84" font-size="13" fill="#0f172a" text-anchor="middle">OS・アプリ</text>
<rect x="156" y="20" width="122" height="78" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="217" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">UNIT 02</text>
<text x="217" y="66" font-size="13" fill="#0f172a" text-anchor="middle">ファイル</text>
<text x="217" y="84" font-size="13" fill="#0f172a" text-anchor="middle">フォルダ・パス</text>
<rect x="298" y="20" width="122" height="78" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="359" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">UNIT 03</text>
<text x="359" y="66" font-size="13" fill="#0f172a" text-anchor="middle">端末・シェル</text>
<text x="359" y="84" font-size="13" fill="#0f172a" text-anchor="middle">コマンド</text>
<rect x="440" y="20" width="122" height="78" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="501" y="44" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">UNIT 04</text>
<text x="501" y="66" font-size="13" fill="#0f172a" text-anchor="middle">ソースコード</text>
<text x="501" y="84" font-size="13" fill="#0f172a" text-anchor="middle">から実行まで</text>
<rect x="582" y="20" width="122" height="78" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="643" y="44" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">UNIT 05</text>
<text x="643" y="66" font-size="13" fill="#0f172a" text-anchor="middle">ブラウザ</text>
<text x="643" y="84" font-size="13" fill="#0f172a" text-anchor="middle">URL・HTTP</text>
<line x1="138" y1="59" x2="152" y2="59" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-04)"/>
<line x1="280" y1="59" x2="294" y2="59" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-04)"/>
<line x1="422" y1="59" x2="436" y2="59" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-04)"/>
<line x1="564" y1="59" x2="578" y2="59" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-04)"/>
<path d="M643,100 L643,124 L75,124 L75,146" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-04)"/>
<rect x="14" y="150" width="122" height="78" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="75" y="174" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">UNIT 06</text>
<text x="75" y="196" font-size="13" fill="#0f172a" text-anchor="middle">サーバー・ポート</text>
<text x="75" y="214" font-size="13" fill="#0f172a" text-anchor="middle">localhost</text>
<rect x="156" y="150" width="122" height="78" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="217" y="174" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">UNIT 07</text>
<text x="217" y="196" font-size="13" fill="#0f172a" text-anchor="middle">データベース</text>
<text x="217" y="214" font-size="13" fill="#0f172a" text-anchor="middle">と永続化</text>
<rect x="298" y="150" width="122" height="78" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="359" y="174" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">UNIT 08</text>
<text x="359" y="196" font-size="13" fill="#0f172a" text-anchor="middle">認証と認可</text>
<text x="359" y="214" font-size="13" fill="#0f172a" text-anchor="middle">（ログイン）</text>
<rect x="440" y="150" width="122" height="78" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="501" y="174" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">UNIT 09</text>
<text x="501" y="196" font-size="13" fill="#0f172a" text-anchor="middle">コンパイラ</text>
<text x="501" y="214" font-size="13" fill="#0f172a" text-anchor="middle">エラーの読み方</text>
<rect x="582" y="150" width="122" height="78" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="643" y="174" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">UNIT 10</text>
<text x="643" y="196" font-size="13" fill="#0f172a" text-anchor="middle">学習サイクル</text>
<text x="643" y="214" font-size="13" fill="#0f172a" text-anchor="middle">と完成像</text>
<line x1="138" y1="189" x2="152" y2="189" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-04)"/>
<line x1="280" y1="189" x2="294" y2="189" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-04)"/>
<line x1="422" y1="189" x2="436" y2="189" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-04)"/>
<line x1="564" y1="189" x2="578" y2="189" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-04)"/>
<rect x="120" y="256" width="16" height="16" rx="3" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="144" y="269" font-size="12" fill="#334155">コンピュータの基本</text>
<rect x="300" y="256" width="16" height="16" rx="3" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="324" y="269" font-size="12" fill="#334155">Web の基本</text>
<rect x="450" y="256" width="16" height="16" rx="3" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="474" y="269" font-size="12" fill="#334155">学び方</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図0-5: この巻の学習地図。青はコンピュータ、橙は Web、緑は学び方のまとまり</div>
</div>

| 単元 | 一言でいうと | 動かすもの |
|---|---|---|
| UNIT 01 | コンピュータは「料理人・調理台・冷蔵庫・店長」 | 自分のプロセス番号を表示するプログラム |
| UNIT 02 | ファイルの場所は「住所」で表す | ファイルを書いて読み戻すプログラム |
| UNIT 03 | 文字で OS にお願いする窓 | PowerShell の基本コマンド |
| UNIT 04 | 人の言葉を機械の言葉に翻訳する | `cargo new` → `cargo run` |
| UNIT 05 | Web は「決まった形の手紙」のやりとり | `curl.exe` と、HTTP を手で送るプログラム |
| UNIT 06 | ポートは「部屋番号」 | Rust だけで作る小さな Web サーバー |
| UNIT 07 | 消えない場所にしまう | ファイルに保存するカウンター |
| UNIT 08 | 「誰か」と「何をしてよいか」は別の話 | ログインと持ち主チェックのミニ版 |
| UNIT 09 | エラーは「直し方つきの診断書」 | わざと間違えたコード |
| UNIT 10 | 予想→実行→観察→変更→説明 | テストで間違いを見つけて直す |

### 前提知識

- **プログラミングの経験は不要**です。
- パソコンの基本操作（ファイルを開く、文字を入力する、コピーと貼り付け）ができれば十分です。
- 英語が苦手でも大丈夫です。コマンドやエラーメッセージに出てくる英単語は、そのつど意味を説明します。

この巻は全14巻の最初に読む巻です。先に読んでおくべき巻はありません。

### この巻の進め方

この巻は「読むだけ」でも理解できるように書いていますが、**手を動かすと理解の深さがまったく違います**。おすすめの進め方は次のとおりです。

1. まず UNIT 01 から UNIT 10 まで、コードは眺めるだけで一度通して読む
2. 第1巻で Rust などの道具をインストールする
3. この巻に戻り、コードを実際に動かしながら2回目を読む

コードを動かすときは、練習用のプロジェクト（プログラムをまとめておくフォルダ）を1つだけ作り、例ごとに中身を入れ替えて使います。UNIT 04 で詳しく説明しますが、手順だけ先に書いておきます。

```powershell
# ホームフォルダに rust-study フォルダを作って移動する
mkdir rust-study
cd rust-study
# 練習用プロジェクト practice を作って、その中へ移動する
cargo new practice
cd practice
```

あとは、各例のコードを `src\main.rs` というファイルに**まるごと貼り付けて保存**し、`cargo run` を実行します。

> **ヒント:** `cargo run` は、プログラム自身が表示する文字より先に、次のような3行を表示します。
>
> `Compiling practice v0.1.0 (...)` ／ `Finished ...` ／ `Running ...`
>
> これは Cargo（カーゴ）という道具の「今コンパイルしています」「終わりました」「実行します」という報告です（UNIT 04 で説明します）。この巻の実行結果では、説明に必要なとき以外はこの3行を省き、**プログラム自身の出力だけ**を載せます。`cargo run -q` と打つと、この3行を出さずに実行できます。

> **ヒント:** この巻に出てくる `C:\Users\you\...` の `you` の部分は、あなたの Windows のユーザー名に読み替えてください。プロセス番号や時刻など、実行するたびに変わる値もあります。

### 使うツールと版

この巻のコードと実行結果は、次の環境で実際に動かして確認しています。

| 道具 | 版 | この巻での使い道 |
|---|---|---|
| Windows 11 | ARM64 版（x64 版でも同じ手順） | すべての作業 |
| PowerShell | 5.1（Windows に最初から入っているもの） | コマンドを打つ（UNIT 03 以降） |
| Rust（rustc・cargo） | 1.97.1、edition 2024 | すべての Rust コード |
| curl.exe | 8.x（Windows に最初から入っているもの） | HTTP のやりとりを見る（UNIT 05・06） |
| ブラウザ | Chrome または Edge | 作ったサーバーにアクセスする（UNIT 06） |
| SQLite のコマンドラインツール | 3.50（参考） | SQL の例の確認だけに使用（UNIT 07。読者は見るだけでよい） |

> **注意:** この巻ではまだ Topcoat・Turso・Fly.io は使いません。Rust に最初から付いてくる機能（**標準ライブラリ**）だけで、Web の仕組みを体験します。Topcoat を使うための Rust の版などは、第1巻で改めて確認します。
>
> WSL（Windows の中で Linux を動かす仕組み）や macOS・Linux を使う人は、コマンドの違いを各単元の「補足」で示します。

## UNIT 01: コンピュータ・OS・アプリ

### この単元のゴール

- **CPU・メモリ・ストレージ**の役割を、台所のたとえで説明できる
- **OS（オーエス）**が何をしているか、アプリとの関係を説明できる
- 動いているプログラムを **プロセス** と呼ぶことと、プロセスが外へ結果を伝える3つの出口（**標準出力・標準エラー出力・終了コード**）を区別できる
- RustNote のサーバーも「OS の上で動く1つのプロセス」だと分かる

### たとえ話でつかむ

コンピュータの中は、**レストランの厨房（ちゅうぼう）**にとてもよく似ています。

- **CPU（シーピーユー）は料理人**です。レシピに書かれた手順を、1つずつ、ものすごい速さでこなします。1秒間に何十億回もの計算ができますが、レシピに書いていないことは一切しません。
- **メモリは調理台**です。今まさに使っている材料や、切りかけの野菜を置いておく場所です。料理人の手がすぐ届くので速く使えますが、広さには限りがあり、**閉店（電源オフ）すると片付けられて何も残りません**。
- **ストレージ（SSD など）は冷蔵庫や食品庫**です。たくさんしまっておけて、**閉店しても中身は残ります**。ただし、取りに行くのは調理台より時間がかかります。
- **OS は店長**です。どの料理人がいつどのコンロを使うか、調理台のどこを誰が使うかを決め、他の人の皿に勝手に触らないように見張っています。Windows・macOS・Linux はどれも OS です。
- **アプリ（プログラム）はレシピ**で、**プロセスはそのレシピで「今まさに作っている最中の1皿」**です。同じレシピ（同じプログラム）から、同時に2皿（2つのプロセス）を作ることもできます。

> **用語:** **CPU** … Central Processing Unit（中央処理装置）の略。計算と命令の実行を担当する部品です。
>
> **用語:** **プロセス** … 実行中のプログラムのことです。ファイルとして保存されている「プログラム」は動いていない状態、OS が読み込んで動かし始めたものが「プロセス」です。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-05" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="180" y="26" font-size="15" font-weight="700" fill="#c2410c" text-anchor="middle">レストランの厨房</text>
<text x="540" y="26" font-size="15" font-weight="700" fill="#1e40af" text-anchor="middle">コンピュータ</text>
<rect x="40" y="40" width="280" height="50" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="180" y="62" font-size="14" font-weight="700" fill="#9a3412" text-anchor="middle">店長</text>
<text x="180" y="80" font-size="12" fill="#475569" text-anchor="middle">誰が何をいつ使うか決める</text>
<rect x="400" y="40" width="280" height="50" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="540" y="62" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">OS（Windows など）</text>
<text x="540" y="80" font-size="12" fill="#475569" text-anchor="middle">CPU・メモリ・ファイルを割り振る</text>
<rect x="40" y="104" width="280" height="50" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="180" y="126" font-size="14" font-weight="700" fill="#9a3412" text-anchor="middle">料理人</text>
<text x="180" y="144" font-size="12" fill="#475569" text-anchor="middle">レシピの手順を高速にこなす</text>
<rect x="400" y="104" width="280" height="50" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="540" y="126" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">CPU</text>
<text x="540" y="144" font-size="12" fill="#475569" text-anchor="middle">命令を1つずつ実行する</text>
<rect x="40" y="168" width="280" height="50" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="180" y="190" font-size="14" font-weight="700" fill="#9a3412" text-anchor="middle">調理台</text>
<text x="180" y="208" font-size="12" fill="#475569" text-anchor="middle">速いが狭い・閉店で片付く</text>
<rect x="400" y="168" width="280" height="50" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="540" y="190" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">メモリ（例: 16GB）</text>
<text x="540" y="208" font-size="12" fill="#475569" text-anchor="middle">電源を切ると中身が消える</text>
<rect x="40" y="232" width="280" height="50" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="180" y="254" font-size="14" font-weight="700" fill="#9a3412" text-anchor="middle">冷蔵庫・食品庫</text>
<text x="180" y="272" font-size="12" fill="#475569" text-anchor="middle">たくさん入る・閉店しても残る</text>
<rect x="400" y="232" width="280" height="50" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="540" y="254" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">ストレージ（SSD・例: 512GB）</text>
<text x="540" y="272" font-size="12" fill="#475569" text-anchor="middle">電源を切っても残る</text>
<line x1="324" y1="65" x2="396" y2="65" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-05)"/>
<line x1="324" y1="129" x2="396" y2="129" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-05)"/>
<line x1="324" y1="193" x2="396" y2="193" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-05)"/>
<line x1="324" y1="257" x2="396" y2="257" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-05)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-1: 厨房とコンピュータの対応。左のたとえが、右の部品に当たる</div>
</div>

次の図は、コンピュータを**層（レイヤー）**で見たものです。いちばん下に機械（ハードウェア）があり、その上に OS、さらにその上でたくさんのプロセスが動いています。プロセスは機械を直接さわらず、**必ず OS にお願いして**メモリやファイルやネットワークを使います。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-06" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#c2410c"/></marker>
</defs>
<text x="20" y="28" font-size="13" font-weight="700" fill="#334155">アプリ（プロセス）の層</text>
<rect x="20" y="38" width="124" height="56" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="82" y="62" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">ブラウザ</text>
<text x="82" y="82" font-size="12" fill="#64748b" text-anchor="middle">PID 5120</text>
<rect x="158" y="38" width="124" height="56" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="220" y="62" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">VS Code</text>
<text x="220" y="82" font-size="12" fill="#64748b" text-anchor="middle">PID 7344</text>
<rect x="296" y="38" width="124" height="56" rx="8" fill="#ffffff" stroke="#3b82f6" stroke-width="2"/>
<text x="358" y="62" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">PowerShell</text>
<text x="358" y="82" font-size="12" fill="#64748b" text-anchor="middle">PID 9012</text>
<rect x="434" y="38" width="124" height="56" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="496" y="62" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">practice.exe</text>
<text x="496" y="82" font-size="12" fill="#64748b" text-anchor="middle">PID 24076</text>
<rect x="572" y="38" width="128" height="56" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="636" y="62" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">RustNote</text>
<text x="636" y="82" font-size="12" fill="#64748b" text-anchor="middle">（サーバー）</text>
<line x1="496" y1="98" x2="496" y2="140" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-06)"/>
<text x="508" y="116" font-size="12" fill="#c2410c">「ファイルを開きたい」</text>
<text x="508" y="132" font-size="12" fill="#c2410c">「メモリを少しください」</text>
<rect x="20" y="146" width="680" height="56" rx="10" fill="#eff6ff" stroke="#1e40af" stroke-width="2"/>
<text x="360" y="170" font-size="15" font-weight="700" fill="#1e40af" text-anchor="middle">OS（Windows / Linux / macOS）</text>
<text x="360" y="190" font-size="12" fill="#334155" text-anchor="middle">プロセスに番号（PID）を付け、CPU の時間・メモリ・ファイル・ネットワークを貸し出す</text>
<rect x="20" y="222" width="160" height="56" rx="8" fill="#f8fafc" stroke="#475569"/>
<text x="100" y="255" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">CPU</text>
<rect x="194" y="222" width="160" height="56" rx="8" fill="#f8fafc" stroke="#475569"/>
<text x="274" y="255" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">メモリ</text>
<rect x="368" y="222" width="160" height="56" rx="8" fill="#f8fafc" stroke="#475569"/>
<text x="448" y="255" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">ストレージ</text>
<rect x="542" y="222" width="158" height="56" rx="8" fill="#f8fafc" stroke="#475569"/>
<text x="621" y="255" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">ネットワーク</text>
<text x="20" y="296" font-size="12" fill="#64748b">ハードウェア（機械そのもの）の層</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-2: コンピュータの層。プロセスは OS を通してだけ機械を使う（PID の数字は例）</div>
</div>

### まず動かす最小コード

最初のプログラムでは、「自分は OS の上で動く1つのプロセスである」ということを、プログラム自身に確かめさせます。

#### 自分が動いている環境とプロセス番号を表示する

> **▼ このコードがやること（先に日本語で）:** このプログラムが動いている OS の種類と CPU の種類、そして OS がこのプロセスに付けた番号（プロセス ID）を表示します。最後に CPU に1から100までの足し算をさせ、その結果も表示します。

```rust
use std::env;
use std::process;

fn main() {
    // このプログラムが動いている OS の種類（windows / linux / macos など）
    let os = env::consts::OS;
    // CPU の種類（x86_64 / aarch64 など）
    let arch = env::consts::ARCH;
    // OS がこのプロセスに付けた番号（プロセス ID）
    let pid = process::id();

    println!("OS の種類: {os}");
    println!("CPU の種類: {arch}");
    println!("プロセス ID: {pid}");

    // CPU に足し算を 100 回してもらい、途中の値はメモリ上の total に置く
    let mut total = 0;
    for n in 1..=100 {
        total += n;
    }
    println!("1 から 100 までの合計: {total}");
}
```

練習用プロジェクトの `src\main.rs` をこの内容に置き換えて保存し、実行します。

```powershell
cargo run
```

**▼ 実行結果**（2回実行した結果です）

```text
OS の種類: windows
CPU の種類: aarch64
プロセス ID: 24076
1 から 100 までの合計: 5050
```

```text
OS の種類: windows
CPU の種類: aarch64
プロセス ID: 49372
1 から 100 までの合計: 5050
```

同じプログラムなのに、**プロセス ID だけが毎回変わっている**ことに注目してください。プログラムを起動するたびに、OS が新しいプロセスを作り、新しい番号を付けているからです。`aarch64`（エーアーチ64）は ARM 系の CPU の名前です。一般的な Intel や AMD の CPU の PC なら `x86_64` と表示されます。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 道具箱から道具を取り出す（`use`）

```rust
use std::env;
use std::process;
```

- `std`（エスティーディー、standard の略）は、Rust に最初から付いてくる道具箱で、**標準ライブラリ**と呼びます。
- `use std::env;` は「`std` の中の `env`（環境に関する道具）を、短く `env` と書いて使えるようにする」という宣言です。
- 同じように `process`（プロセスに関する道具）も使えるようにしています。

> **用語:** **ライブラリ** … 誰かが作ってくれた便利な部品（関数など）の集まりです。自分で一から作らなくても、取り出して使えます。

##### 解説2: プログラムの入り口 `main`

```rust
fn main() {
    // ...
}
```

- `fn` は function（関数）の略で、「ここから処理のまとまりを書きます」という印です。
- `main`（メイン）という名前の関数は特別で、**プログラムを起動すると最初にここが実行されます**。
- `{` から `}` までが、この関数の中身です。

##### 解説3: 値に名前を付けて表示する

```rust
let os = env::consts::OS;
let pid = process::id();
println!("OS の種類: {os}");
```

- `let`（レット）は「値に名前を付けて、メモリに置いておく」という意味です。この名前付きの入れ物を **変数** と呼びます。
- `env::consts::OS` は、Rust が教えてくれる「この OS の種類」という文字です。
- `process::id()` は、**OS にこのプロセスの番号を問い合わせる**関数です。`()` は「関数を呼び出す」印です。
- `println!`（プリントライン）は、文字を画面に1行表示します。`"..."` の中の `{os}` の部分に、変数 `os` の中身が埋め込まれます。

##### 解説4: CPU に計算させる

```rust
let mut total = 0;
for n in 1..=100 {
    total += n;
}
```

- `let mut` の `mut`（ミュート、mutable の略）は「あとで中身を書き換えてよい」という印です。Rust の変数は、`mut` を付けないと書き換えられません（UNIT 09 で、付け忘れたときのエラーを見ます）。
- `for n in 1..=100` は「`n` を 1, 2, 3, …, 100 と変えながら、`{ }` の中を100回くり返す」という意味です。`1..=100` は「1以上100以下」を表します。
- `total += n;` は「`total` に `n` を足して、`total` を書き換える」という意味です。
- この100回の足し算を実行しているのが CPU、途中の `total` の値を置いているのがメモリです。

### もう一歩: 実務に近い形

プロセスは、外の世界に結果を伝える**出口を3つ**持っています。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-07" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="60" width="200" height="110" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="120" y="102" font-size="15" font-weight="700" fill="#c2410c" text-anchor="middle">プロセス</text>
<text x="120" y="126" font-size="12" fill="#475569" text-anchor="middle">practice.exe</text>
<text x="120" y="146" font-size="12" fill="#475569" text-anchor="middle">（実行中のプログラム）</text>
<line x1="222" y1="80" x2="372" y2="40" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-07)"/>
<line x1="222" y1="115" x2="372" y2="115" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-07)"/>
<line x1="222" y1="150" x2="372" y2="190" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-07)"/>
<rect x="378" y="16" width="322" height="50" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="392" y="38" font-size="14" font-weight="700" fill="#1e40af">① 標準出力（stdout）</text>
<text x="392" y="56" font-size="12" fill="#334155">ふつうの結果。println! で書く</text>
<rect x="378" y="90" width="322" height="50" rx="8" fill="#fef2f2" stroke="#dc2626"/>
<text x="392" y="112" font-size="14" font-weight="700" fill="#b91c1c">② 標準エラー出力（stderr）</text>
<text x="392" y="130" font-size="12" fill="#334155">エラーや警告の知らせ。eprintln! で書く</text>
<rect x="378" y="164" width="322" height="50" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="392" y="186" font-size="14" font-weight="700" fill="#047857">③ 終了コード（exit code）</text>
<text x="392" y="204" font-size="12" fill="#334155">終わるときに OS へ渡す数。0 = 成功</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-3: プロセスの3つの出口。画面に出る文字は①と②、成功・失敗の合図は③</div>
</div>

#### 成功と失敗を、出口を使い分けて伝える

> **▼ このコードがやること（先に日本語で）:** 起動するときに名前を1つ受け取り、「こんにちは、○○ さん」と表示します。名前が渡されなかったときは、エラーのメッセージを「エラー用の出口」に書き、**失敗の合図として終了コード 2** を OS に返して終わります。

```rust
use std::env;
use std::process;

fn main() {
    // コマンドラインで渡された 1 つ目の値を取り出す（無ければ None）
    let name = env::args().nth(1);

    match name {
        Some(name) => {
            // 標準出力へ書く（ふつうの結果）
            println!("こんにちは、{name} さん");
        }
        None => {
            // 標準エラー出力へ書く（エラーの知らせ）
            eprintln!("エラー: 名前を 1 つ渡してください（例: cargo run -- Hanako）");
            // 終了コード 2 でプロセスを終える（0 以外は「失敗」の合図）
            process::exit(2);
        }
    }
}
```

`cargo build` でコンパイルだけ行い、できあがった実行ファイル `practice.exe` を PowerShell から直接動かします。`$LASTEXITCODE` は「直前に動かしたプログラムの終了コード」を表示する PowerShell の変数です。

```powershell
cargo build
.\target\debug\practice.exe Hanako
$LASTEXITCODE
.\target\debug\practice.exe
$LASTEXITCODE
```

**▼ 実行結果**

```text
こんにちは、Hanako さん
0
エラー: 名前を 1 つ渡してください（例: cargo run -- Hanako）
2
```

さらに、`>` を使って「標準出力だけをファイルへ送る」と、2つの出口が別物であることがはっきり分かります。

```powershell
.\target\debug\practice.exe > out.txt
Get-Content out.txt
```

**▼ 実行結果**

```text
エラー: 名前を 1 つ渡してください（例: cargo run -- Hanako）
```

エラーのメッセージは画面に出ますが、`out.txt` の中身は空っぽです（`Get-Content` は何も表示しません）。`>` が横取りしたのは①の標準出力だけで、②の標準エラー出力はそのまま画面に流れたからです。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 起動時に渡された値を受け取る

```rust
let name = env::args().nth(1);
```

- `env::args()` は、プログラムを起動したときに渡された値（**コマンドライン引数**）の並びです。0番目にはプログラム自身の名前が入り、1番目から先が渡した値です。
- `.nth(1)` は「1番目を取り出す」という意味です。
- 値が渡されていないこともあるので、結果は「ある（`Some(値)`）」か「ない（`None`）」のどちらかになります。この「あるかもしれないし、ないかもしれない」を表す型を `Option`（オプション）と呼びます（第2巻で詳しく学びます）。

##### 解説2: 場合分けする `match`

```rust
match name {
    Some(name) => { /* 値があったとき */ }
    None => { /* 値がなかったとき */ }
}
```

- `match`（マッチ）は、値の形に応じて処理を分ける構文です。
- `Some(name)` の場合は、中身を `name` という名前で取り出して使えます。
- Rust では `Some` と `None` の**両方の場合を必ず書かないとコンパイルが通りません**。「値がなかったとき」を書き忘れる事故を、コンパイラが防いでくれるわけです。

##### 解説3: エラー用の出口と終了コード

```rust
eprintln!("エラー: 名前を 1 つ渡してください（例: cargo run -- Hanako）");
process::exit(2);
```

- `eprintln!`（イープリントライン）は `println!` のエラー版で、**標準エラー出力**に書きます。
- `process::exit(2)` は、終了コード 2 を OS に渡して、その場でプロセスを終わらせます。
- 終了コードは「0 なら成功、0 以外なら失敗」という世界共通の約束です。どの数を使うかはプログラムが決めます。

| 終了コード | よくある意味 |
|---|---|
| `0` | 成功 |
| `1` | 一般的な失敗（Rust で `main` がエラーを返したときもこれ） |
| `2` | 使い方の間違い（引数が足りない など）によく使われる |
| `101` | Rust のプログラムが**パニック**（次の節で説明）して止まった |

> **ヒント:** `cargo run -- Hanako` のように、`--` の後ろに書いた値がプログラムへの引数になります。`--` より前は Cargo への指示、後ろはプログラムへの値、という区切りです。

### よくあるエラーと読み方

#### 実行中に止まる「パニック」

存在しない場所のデータを読もうとすると、Rust のプログラムは**パニック**して止まります。

```rust
fn main() {
    // 3 人分の点数をメモリに並べる
    let scores = vec![80, 95, 70];
    // 11 人目（番号 10）を取り出そうとする
    let index = 10;
    println!("{}", scores[index]);
}
```

**▼ 実行結果**（`cargo run` の出力をそのまま載せています）

```text
   Compiling practice v0.1.0 (C:\Users\you\rust-study\practice)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target\debug\practice.exe`

thread 'main' (43864) panicked at src\main.rs:6:26:
index out of bounds: the len is 3 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\practice.exe` (exit code: 101)
```

**何が起きたか:** 1行ずつ読むと、次のことが書いてあります。

| 出力の部分 | 意味 |
|---|---|
| `thread 'main' (43864) panicked` | メインの処理（スレッド）がパニックした。括弧の数字はスレッドの番号 |
| `at src\main.rs:6:26` | 場所は `src\main.rs` の **6行目・26文字目** |
| `index out of bounds: the len is 3 but the index is 10` | 「範囲外の番号。長さは 3 なのに、番号 10 を指定した」 |
| `note: run with RUST_BACKTRACE=1 ...` | 「もっと詳しい経路を見たければ、この設定で実行して」という補足 |
| `(exit code: 101)` | Cargo からの報告。プロセスは終了コード 101（パニック）で終わった |

**直し方:** 番号は 0 から数えるので、3人分なら使える番号は `0`・`1`・`2` だけです。`let index = 2;` のように範囲内の番号にすれば動きます。

> **つまずきポイント:** パニックは**コンパイルは通ったのに、動かしてみて初めて起きる失敗**です（UNIT 04 で「コンパイル時の失敗」と「実行時の失敗」を区別します）。Rust は、範囲外の場所をこっそり読んでデタラメな値を返す代わりに、**その場で止めて場所を教えてくれます**。止まるのは不親切に見えますが、間違った値のまま処理が進むより、ずっと安全です。

### RustNote ではここで使う

- RustNote のサーバーも、Fly.io というクラウドの PC の上で動く**1つのプロセス**です。違いは、`practice.exe` がすぐ終わるのに対して、サーバーは**止めるまでずっと動き続ける**ことです。
- サーバーの動作記録（**ログ**）は、標準出力・標準エラー出力に書きます。Fly.io はそれを集めて、`fly logs` というコマンドで見られるようにしてくれます（第12巻）。
- 起動に必要な設定が足りないときは、RustNote は分かりやすいメッセージを出して**0 以外の終了コード**で止まります。「どこかおかしいまま動き続ける」より「すぐ止まって理由を言う」ほうが、原因を早く見つけられるからです。

### 練習問題

**問題1（予想してから実行）:** 最小コードの `for n in 1..=100` を `for n in 1..=10` に変えると、最後の行には何と表示されるでしょうか。実行する前に予想してから、確かめてください。

**問題2:** 「もう一歩」のコードで、名前が無いときの終了コードを `2` から `7` に変えてください。PowerShell で `$LASTEXITCODE` を表示して、7 になることを確かめましょう。

**問題3:** 次の a〜d は、厨房のたとえでいうと「料理人（CPU）」「調理台（メモリ）」「冷蔵庫（ストレージ）」のどれに当たるでしょうか。

- a. RustNote のサーバーが今まさに処理している、送られてきたノートのタイトル
- b. データベースに保存されたノート
- c. タイトルが 1〜80 文字かどうかを数えて判定すること
- d. `cargo build` で作られた `practice.exe` というファイル

### 解答と解説

**問題1の解答:** `1 から 100 までの合計: 55` と表示されます（文の「100」は `println!` の中の文字なので変わりません）。くり返しが1〜10になるので、1+2+…+10 = 55 です。文字の部分も直すなら、次のようにします。

```rust
fn main() {
    let mut total = 0;
    for n in 1..=10 {
        total += n;
    }
    println!("1 から 10 までの合計: {total}");
}
```

**▼ 実行結果**

```text
1 から 10 までの合計: 55
```

**問題2の解答:** `process::exit(2);` を `process::exit(7);` に書き換えて、`cargo build` してから実行します。

```powershell
cargo build
.\target\debug\practice.exe
$LASTEXITCODE
```

**▼ 実行結果**

```text
エラー: 名前を 1 つ渡してください（例: cargo run -- Hanako）
7
```

ポイントは、`cargo build` を**し直すこと**です。ソースコードを書き換えても、コンパイルし直さなければ `practice.exe` は古いままです（UNIT 04 で詳しく見ます）。

**問題3の解答:**

- a. **メモリ**（処理中の値は調理台の上にある）
- b. **ストレージ**（データベースの中身は、最終的にストレージ上のファイルに保存される）
- c. **CPU**（数えて比べるのは計算＝料理人の仕事）
- d. **ストレージ**（ファイルはストレージに保存される。実行すると、OS がその中身をメモリに読み込んでプロセスにする）

### まとめ

- CPU は計算する料理人、メモリは速いが消える調理台、ストレージは遅いが残る冷蔵庫、OS はそれらを割り振る店長です。
- 動いているプログラムを**プロセス**と呼び、OS が番号（PID）を付けて管理します。
- プロセスは**標準出力・標準エラー出力・終了コード**の3つの出口で結果を伝えます。0 は成功、Rust のパニックは 101 です。

## UNIT 02: ファイル・フォルダ・パス

### この単元のゴール

- **ファイル**と**フォルダ（ディレクトリ）**の関係を、木の形（ツリー）で説明できる
- **絶対パス**と**相対パス**の違いを説明し、正しく書ける
- 「今いるフォルダ（**カレントディレクトリ**）」によって、相対パスの指す場所が変わることを体験する
- Rust でファイルを書いて読み戻し、「ファイルが見つからない」エラーを読める

### たとえ話でつかむ

ファイルの場所を表す**パス**は、**住所**とそっくりです。

- 「東京都 千代田区 千代田 1-1」のように、**いちばん大きな区切りから順に全部書いた住所**は、日本中どこにいる人に渡しても同じ場所を指します。これが**絶対パス**です。Windows なら `C:\Users\you\rust-study\practice\Cargo.toml` のように、ドライブ名 `C:` から書き始めます。
- 一方、「この角を曲がって2軒目」という**道案内**は、**今どこに立っているか**によって指す家が変わります。これが**相対パス**です。`src\main.rs` は「今いるフォルダの中の `src` フォルダの中の `main.rs`」という意味で、どこに立っているかで行き先が変わります。
- **フォルダ**は「棚」や「引き出し」です。引き出しの中に小さな引き出し（フォルダ）や書類（ファイル）を入れられます。

> **用語:** **ファイル** … ストレージに保存された、名前の付いたデータのかたまり。文章、画像、プログラムなど、中身は何でもかまいません。
>
> **用語:** **フォルダ（ディレクトリ）** … ファイルや別のフォルダを入れておく入れ物。Windows では「フォルダ」、コマンドの世界では「ディレクトリ」と呼ぶことが多いですが、同じものです。
>
> **用語:** **拡張子（かくちょうし）** … ファイル名の最後の `.` から後ろの部分（`main.rs` の `rs`、`Cargo.toml` の `toml`）。中身の種類の目印です。`.rs` は Rust のソースコードを表します。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="14" width="150" height="32" rx="6" fill="#f8fafc" stroke="#475569" stroke-width="1.5"/>
<text x="95" y="35" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">C:\（ドライブ）</text>
<path d="M50,46 L50,72 L80,72" fill="none" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="80" y="58" width="120" height="28" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="140" y="77" font-size="13" fill="#9a3412" text-anchor="middle">Users</text>
<path d="M110,86 L110,110 L140,110" fill="none" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="140" y="96" width="120" height="28" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="200" y="115" font-size="13" fill="#9a3412" text-anchor="middle">you</text>
<path d="M170,124 L170,148 L200,148" fill="none" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="200" y="134" width="130" height="28" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="265" y="153" font-size="13" fill="#9a3412" text-anchor="middle">rust-study</text>
<path d="M230,162 L230,186 L260,186" fill="none" stroke="#94a3b8" stroke-width="1.5"/>
<path d="M230,186 L230,300 L260,300" fill="none" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="260" y="172" width="120" height="28" rx="6" fill="#eff6ff" stroke="#1e40af" stroke-width="2.5"/>
<text x="320" y="191" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">practice</text>
<rect x="260" y="286" width="120" height="28" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="320" y="305" font-size="13" fill="#9a3412" text-anchor="middle">hello</text>
<path d="M290,200 L290,224 L320,224" fill="none" stroke="#94a3b8" stroke-width="1.5"/>
<path d="M290,224 L290,262 L320,262" fill="none" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="320" y="210" width="110" height="28" rx="6" fill="#ffffff" stroke="#94a3b8"/>
<text x="375" y="229" font-size="13" fill="#334155" text-anchor="middle">Cargo.toml</text>
<rect x="320" y="248" width="110" height="28" rx="6" fill="#fff7ed" stroke="#f97316"/>
<text x="375" y="267" font-size="13" fill="#9a3412" text-anchor="middle">src</text>
<path d="M430,262 L470,262" fill="none" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="470" y="248" width="110" height="28" rx="6" fill="#ffffff" stroke="#94a3b8"/>
<text x="525" y="267" font-size="13" fill="#334155" text-anchor="middle">main.rs</text>
<rect x="400" y="166" width="300" height="40" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="412" y="184" font-size="12" font-weight="700" fill="#1e40af">太枠 = 今いるフォルダ（カレントディレクトリ）</text>
<text x="412" y="200" font-size="12" fill="#334155">PowerShell の表示: PS C:\Users\you\rust-study\practice&gt;</text>
<rect x="430" y="22" width="270" height="110" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<rect x="446" y="40" width="16" height="16" rx="3" fill="#fff7ed" stroke="#f97316"/>
<text x="470" y="53" font-size="12" fill="#334155">フォルダ（中に入れ物を持てる）</text>
<rect x="446" y="66" width="16" height="16" rx="3" fill="#ffffff" stroke="#94a3b8"/>
<text x="470" y="79" font-size="12" fill="#334155">ファイル（データそのもの）</text>
<text x="446" y="106" font-size="12" fill="#334155">線は「この中に入っている」</text>
<text x="446" y="122" font-size="12" fill="#334155">という親子関係を表す</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図2-1: フォルダの木（ツリー）。上が「親」、線でつながった下が「子」</div>
</div>

同じ `main.rs` を、絶対パスと相対パスでどう書くかを比べます。今いるフォルダが `practice` のとき、次の図の3つの書き方は**どれも同じファイル**を指します。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-08" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="16" width="680" height="62" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="36" y="40" font-size="14" font-weight="700" fill="#c2410c">絶対パス（どこにいても同じ場所）</text>
<text x="36" y="64" font-size="14" fill="#0f172a" font-family="Cascadia Mono, Consolas, monospace">C:\Users\you\rust-study\practice\src\main.rs</text>
<rect x="20" y="92" width="680" height="62" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="36" y="116" font-size="14" font-weight="700" fill="#1e40af">相対パス（今いる practice から見た道順）</text>
<text x="36" y="140" font-size="14" fill="#0f172a" font-family="Cascadia Mono, Consolas, monospace">src\main.rs　　または　　.\src\main.rs</text>
<rect x="20" y="168" width="680" height="88" rx="10" fill="#f8fafc" stroke="#475569"/>
<text x="36" y="192" font-size="14" font-weight="700" fill="#334155">道順に使う特別な名前</text>
<text x="36" y="216" font-size="13" fill="#0f172a" font-family="Cascadia Mono, Consolas, monospace">.</text>
<text x="70" y="216" font-size="13" fill="#334155">今いるフォルダ自身（「ここ」）</text>
<text x="36" y="240" font-size="13" fill="#0f172a" font-family="Cascadia Mono, Consolas, monospace">..</text>
<text x="70" y="240" font-size="13" fill="#334155">1つ上の親フォルダ（例: practice から見た .. は rust-study）</text>
<text x="420" y="216" font-size="13" fill="#0f172a" font-family="Cascadia Mono, Consolas, monospace">..\hello</text>
<line x1="500" y1="211" x2="530" y2="211" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-08)"/>
<text x="538" y="216" font-size="13" fill="#334155">隣の hello フォルダ</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図2-2: 絶対パスと相対パス。相対パスは「今いるフォルダ」が出発点</div>
</div>

> **ヒント:** Windows ではパスの区切りに `\`（円記号またはバックスラッシュ）を使い、Linux や macOS では `/`（スラッシュ）を使います。Rust のプログラムの中では、Windows でも `/` で書いたパスがたいてい通じます。日本語キーボードでは `\` が `¥` と表示されることがありますが、同じ文字です。

### まず動かす最小コード

#### ファイルを書いて、読み戻す

> **▼ このコードがやること（先に日本語で）:** 今いるフォルダの場所を表示してから、`memo` というフォルダを作り、その中に `today.txt` というファイルを書き込みます。書いたファイルをすぐに読み戻して、相対パス・絶対パス・中身を表示します。

```rust
use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // 今いるフォルダ（カレントディレクトリ）を調べて表示する
    let here = env::current_dir()?;
    println!("今いるフォルダ: {}", here.display());

    // memo フォルダを作る（すでにあっても OK）
    fs::create_dir_all("memo")?;

    // 「memo フォルダの中の today.txt」という相対パスを組み立てる
    let relative = Path::new("memo").join("today.txt");

    // ファイルに文字を書き込む（無ければ作り、あれば上書き）
    fs::write(&relative, "Rust の勉強を始めた\n")?;

    // 同じファイルを読み戻す
    let text = fs::read_to_string(&relative)?;

    println!("相対パス: {}", relative.display());
    println!("絶対パス: {}", here.join(&relative).display());
    print!("中身: {text}");
    Ok(())
}
```

```powershell
cargo run
```

**▼ 実行結果**

```text
今いるフォルダ: C:\Users\you\rust-study\practice
相対パス: memo\today.txt
絶対パス: C:\Users\you\rust-study\practice\memo\today.txt
中身: Rust の勉強を始めた
```

実行したあと、エクスプローラーで `practice` フォルダを開くと、`memo` フォルダと `today.txt` ができているはずです。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 失敗するかもしれない処理と `?`

```rust
fn main() -> std::io::Result<()> {
    let here = env::current_dir()?;
    // ...
    Ok(())
}
```

- ファイルの操作は、**失敗する可能性がいつもある**処理です（ファイルが無い、書き込みが禁止されている、ディスクがいっぱい…）。
- `-> std::io::Result<()>` は「この `main` は、成功（`Ok`）か失敗（`Err`）のどちらかを返します」という宣言です。
- 処理の後ろに付けた `?`（はてな）は、「**失敗だったら、ここで止めて失敗を報告する。成功なら中身を取り出して先へ進む**」という意味の記号です。
- 最後の `Ok(())` は「最後まで来たので成功です」という報告です。
- この仕組みは第2巻・第4巻で詳しく学びます。今は「`?` は失敗したらそこで止まる印」と覚えておけば十分です。

##### 解説2: 今いるフォルダを調べる

```rust
let here = env::current_dir()?;
println!("今いるフォルダ: {}", here.display());
```

- `env::current_dir()` は、**このプロセスのカレントディレクトリ**を OS に問い合わせます。
- `cargo run` で起動したプログラムは、**`cargo run` を打ったときに PowerShell がいたフォルダ**をカレントディレクトリとして受け継ぎます。
- `.display()` は、パスを画面に表示できる形にする道具です。

##### 解説3: パスを組み立てる

```rust
let relative = Path::new("memo").join("today.txt");
```

- `Path::new("memo")` は「`memo` というパス」を作ります。
- `.join("today.txt")` は、その後ろに区切り文字をはさんで `today.txt` をつなぎます。
- Windows なら `memo\today.txt`、Linux なら `memo/today.txt` と、**OS に合った区切り文字**を自動で使ってくれるのが利点です。
- 区切り文字を自分で `"memo\\today.txt"` のように書くより、`join` を使うほうが安全です。

##### 解説4: 書く・読む

```rust
fs::create_dir_all("memo")?;
fs::write(&relative, "Rust の勉強を始めた\n")?;
let text = fs::read_to_string(&relative)?;
```

- `fs`（ファイルシステム）は、ファイルやフォルダを扱う道具箱です。
- `create_dir_all` はフォルダを作ります。途中のフォルダが無ければそれも作り、すでにあっても失敗しません。
- `write` はファイルに書き込みます（**上書き**なので、前の中身は消えます）。
- `read_to_string` はファイル全体を文字列として読み込みます。
- `&relative` の `&` は「パスを貸すだけで、手放さない」という印です（第3巻で詳しく学びます）。
- `"\n"` は**改行**を表す特別な書き方です。

### もう一歩: 実務に近い形

#### パスを部品に分け、「ここから見て」存在するか調べる

> **▼ このコードがやること（先に日本語で）:** `src/main.rs` というパスを「ファイル名」「拡張子」「親フォルダ」に分解して表示します。続いて、いくつかの相対パスについて「今いるフォルダから見て存在するか」を調べ、最後に今いるフォルダの中身を一覧表示します。

```rust
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // 1 つのパスを部品に分けてみる
    let path = Path::new("src/main.rs");
    println!("パス全体  : {}", path.display());
    println!("ファイル名: {:?}", path.file_name());
    println!("拡張子    : {:?}", path.extension());
    println!("親フォルダ: {:?}", path.parent());
    println!();

    // いろいろな相対パスが「今いるフォルダから見て」存在するか調べる
    let candidates = [
        "Cargo.toml",
        "src/main.rs",
        "main.rs",
        "./src/../Cargo.toml",
    ];
    for candidate in candidates {
        let exists = Path::new(candidate).exists();
        println!("{candidate:<22} -> 存在する? {exists}");
    }
    println!();

    // 今いるフォルダの中身を一覧表示する
    println!("今いるフォルダの中身:");
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let kind = if entry.path().is_dir() {
            "フォルダ"
        } else {
            "ファイル"
        };
        println!("  [{kind}] {}", entry.file_name().display());
    }
    Ok(())
}
```

**▼ 実行結果**（`practice` フォルダで `cargo run` した場合）

```text
パス全体  : src/main.rs
ファイル名: Some("main.rs")
拡張子    : Some("rs")
親フォルダ: Some("src")

Cargo.toml             -> 存在する? true
src/main.rs            -> 存在する? true
main.rs                -> 存在する? false
./src/../Cargo.toml    -> 存在する? true

今いるフォルダの中身:
  [フォルダ] .git
  [ファイル] .gitignore
  [ファイル] Cargo.lock
  [ファイル] Cargo.toml
  [フォルダ] memo
  [フォルダ] src
  [フォルダ] target
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: パスの部品

```rust
println!("ファイル名: {:?}", path.file_name());
println!("拡張子    : {:?}", path.extension());
println!("親フォルダ: {:?}", path.parent());
```

- `file_name()` は最後の部品（`main.rs`）、`extension()` は拡張子（`rs`）、`parent()` は1つ上（`src`）を返します。
- 結果に `Some(...)` が付いているのは、「部品が無い場合もある」からです（例えば `C:\` には親がありません）。UNIT 01 で見た `Option` と同じ考え方です。
- `{:?}` は「プログラマー向けの形で表示する」という指定で、`Some(...)` や `"` もそのまま見せてくれます。

##### 解説2: 相対パスは「今いるフォルダ」が出発点

```rust
let exists = Path::new(candidate).exists();
```

- `exists()` は、そのパスにファイルかフォルダが**本当にあるか**を OS に確かめます。
- `main.rs` が `false` なのは、`main.rs` が `practice` の直下ではなく `src` の中にあるからです。
- `./src/../Cargo.toml` は「ここ（`.`）→ `src` に入る → 1つ上（`..`）に戻る → `Cargo.toml`」という遠回りの道順で、結局 `Cargo.toml` を指すので `true` です。

##### 解説3: フォルダの中身を一覧する

```rust
for entry in fs::read_dir(".")? {
    let entry = entry?;
    // ...
}
```

- `fs::read_dir(".")` は、今いるフォルダ（`.`）の中身を1つずつ取り出します。
- `.git` や `.gitignore` のように `.` で始まる名前は、Git というツールが使う設定です。エクスプローラーでは隠れていることがあります（第1巻で説明します）。
- `target` フォルダは、Cargo がコンパイルした結果を置く場所です（UNIT 04）。

### よくあるエラーと読み方

#### ファイルが見つからない（NotFound）

> **▼ このコードがやること（先に日本語で）:** 今いるフォルダにある `config.txt` を読み込んで表示しようとします。ただし、このファイルはまだ作っていません。

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    // config.txt を読む（今いるフォルダに無ければ失敗する）
    let text = fs::read_to_string("config.txt")?;
    println!("設定: {text}");
    Ok(())
}
```

**▼ 実行結果**

```text
Error: Os { code: 2, kind: NotFound, message: "指定されたファイルが見つかりません。" }
error: process didn't exit successfully: `target\debug\practice.exe` (exit code: 1)
```

**何が起きたか:**

- 1行目は、プログラム自身が出したエラーです。`?` で止まった `main` が失敗を返したので、Rust がその中身を表示しました。
  - `Os { code: 2, ... }` … OS が返してきたエラーで、Windows のエラー番号は 2
  - `kind: NotFound` … 種類は「見つからない」
  - `message: "..."` … OS が用意した説明文（日本語版 Windows なので日本語）
- 2行目は Cargo の報告で、プロセスが終了コード 1（失敗）で終わったことを伝えています。

**直し方:** 「ファイルが本当にあるか」と「**今いるフォルダはどこか**」の2つを確かめます。`config.txt` を作るか、ファイルのある場所で実行するか、パスを正しく書き直します。

#### 実行する場所が変わると、相対パスの行き先も変わる

最小コード（`memo\today.txt` を書くプログラム）を、今度は `target\debug` フォルダに移動してから直接実行してみます。

```powershell
cd target\debug
.\practice.exe
```

**▼ 実行結果**

```text
今いるフォルダ: C:\Users\you\rust-study\practice\target\debug
相対パス: memo\today.txt
絶対パス: C:\Users\you\rust-study\practice\target\debug\memo\today.txt
中身: Rust の勉強を始めた
```

エラーにはなりませんが、`memo` フォルダが**予想と違う場所**（`target\debug` の中）にできてしまいました。プログラムは同じでも、カレントディレクトリが変わると相対パスの行き先が変わるからです。

> **つまずきポイント:** 「さっきは動いたのに、今は `NotFound` になる」「保存したはずのファイルが見当たらない」というトラブルの多くは、**カレントディレクトリの取り違え**が原因です。困ったら、まず PowerShell の `PS C:\...>` の部分（今いるフォルダ）を確認しましょう。プログラム側で `env::current_dir()` を表示するのも有効です。

### RustNote ではここで使う

- RustNote のプロジェクトも、`Cargo.toml`・`src` フォルダ（Rust のコード）・`migrations` フォルダ（データベースの表を作る SQL）・`tests` フォルダ（テスト）などの**決まった形のフォルダ構成**を持ちます。詳しい中身は第9巻で作ります。
- 開発中のデータベースは、**ローカルのファイル**として保存します。そのファイルの場所を相対パスで書くと、「どこで `cargo run` したか」によって別のファイルを見てしまうことがあります。RustNote では、場所を設定（環境変数）で決めるようにして、この事故を防ぎます（第9巻）。

### 練習問題

**問題1:** `memo` フォルダの中に `today.txt` と `tomorrow.txt` の2つのファイルを書き込み、`memo` フォルダの中身を一覧表示するプログラムを書いてください。

**問題2:** 次のパスは、絶対パスと相対パスのどちらでしょうか。

- a. `C:\Users\you\rust-study`
- b. `src\main.rs`
- c. `..\hello`
- d. `/home/you/rust-study`（Linux のパス）
- e. `.\Cargo.toml`

**問題3（予想してから実行）:** `practice` フォルダの中で `cd target\debug` してから `cargo run` すると、Cargo はプロジェクトを見つけられるでしょうか。見つけられるなら、最小コードの `memo` フォルダはどこにできるでしょうか。

### 解答と解説

**問題1の解答:**

```rust
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    fs::create_dir_all("memo")?;

    // 2 つのファイルを書く
    fs::write(Path::new("memo").join("today.txt"), "今日の予定\n")?;
    fs::write(Path::new("memo").join("tomorrow.txt"), "明日の予定\n")?;

    // memo フォルダの中身を一覧表示する
    for entry in fs::read_dir("memo")? {
        let entry = entry?;
        println!("{}", entry.path().display());
    }
    Ok(())
}
```

**▼ 実行結果**

```text
memo\today.txt
memo\tomorrow.txt
```

`fs::read_dir("memo")` に渡したのが相対パスなので、表示されるパスも `memo\...` という相対パスになります。

**問題2の解答:** a は**絶対**（ドライブ名 `C:` から始まる）、b は**相対**、c は**相対**（`..` は今いる場所が出発点）、d は**絶対**（Linux では `/` から始まるパスが絶対パス）、e は**相対**（`.` は今いる場所）。

**問題3の解答:** Cargo は、今いるフォルダに `Cargo.toml` が無ければ**親フォルダを順にさかのぼって**探すので、プロジェクトを見つけて実行できます。ただし、プログラムのカレントディレクトリは `target\debug` のままなので、`memo` は `target\debug` の中にできます。

```text
     Running `.\practice.exe`
今いるフォルダ: C:\Users\you\rust-study\practice\target\debug
相対パス: memo\today.txt
絶対パス: C:\Users\you\rust-study\practice\target\debug\memo\today.txt
中身: Rust の勉強を始めた
```

「Cargo がプロジェクトを探す場所」と「プログラムが相対パスを解決する場所」は別物だ、という点がポイントです。

### まとめ

- ファイルはデータ、フォルダは入れ物で、全体は `C:\` を根元とする木（ツリー）になっています。
- 絶対パスは「どこからでも同じ場所」、相対パスは「今いるフォルダ（カレントディレクトリ）からの道順」です。`.` は「ここ」、`..` は「1つ上」。
- 「ファイルが無い」と言われたら、ファイルの有無と**今いるフォルダ**の両方を確かめます。


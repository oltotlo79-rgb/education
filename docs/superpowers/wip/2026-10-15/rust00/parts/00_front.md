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


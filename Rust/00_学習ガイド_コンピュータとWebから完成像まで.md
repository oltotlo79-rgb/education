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
<text x="360" y="330" font-size="12" fill="#64748b" text-anchor="middle">開発中は同じ組み立てを自分の PC で動かします（http://127.0.0.1:3000、DB はローカルのファイル）</text>
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

<!-- -->

> **ヒント:** この巻に出てくる `C:\Users\you\...` の `you` の部分は、あなたの Windows のユーザー名に読み替えてください。プロセス番号や時刻など、実行するたびに変わる値もあります。

### 使うツールと版

この巻のコードと実行結果は、次の環境で実際に動かして確認しています。

| 道具 | 版 | この巻での使い道 |
|---|---|---|
| Windows 11 | ARM64 版（x64 版でも同じ手順） | すべての作業 |
| PowerShell | 5.1（Windows に最初から入っているもの） | コマンドを打つ（UNIT 03 以降） |
| Rust（rustc・cargo） | 1.99.0、edition 2024（1.98 以上なら同じ手順） | すべての Rust コード |
| curl.exe | 8.x（Windows に最初から入っているもの） | HTTP のやりとりを見る（UNIT 05・06） |
| ブラウザ | Chrome または Edge | 作ったサーバーにアクセスする（UNIT 06） |
| SQLite のコマンドラインツール | 3.50（参考） | SQL の例の確認だけに使用（UNIT 07。読者は見るだけでよい） |

> **注意:** この巻ではまだ Topcoat・Turso・Fly.io は使いません。Rust に最初から付いてくる機能（**標準ライブラリ**）だけで、Web の仕組みを体験します。このシリーズで使う Topcoat 0.10 は **Rust 1.98 以上**が必要なので、シリーズ全体で「Rust 1.98 以上、動作確認は 1.99.0」を基準にしています。インストールと版の確かめ方は第1巻で説明します。
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

thread 'main' (50284) panicked at src\main.rs:6:26:
index out of bounds: the len is 3 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\practice.exe` (exit code: 101)
```

**何が起きたか:** 1行ずつ読むと、次のことが書いてあります。

| 出力の部分 | 意味 |
|---|---|
| `thread 'main' (50284) panicked` | メインの処理（スレッド）がパニックした。括弧の数字はスレッドの番号 |
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
<text font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" x="95" y="35" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">C:\（ドライブ）</text>
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
<text font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" x="412" y="200" font-size="12" fill="#334155">PS C:\Users\you\rust-study\practice&gt;</text>
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

## UNIT 03: 端末・シェル・コマンド

### この単元のゴール

- **端末（ターミナル）**・**シェル**・**コマンド**の違いを説明できる
- PowerShell で、今いる場所の確認・移動・フォルダ作成・一覧・表示・コピー・削除ができる
- コマンドを「コマンド名・サブコマンド・オプション・引数」の部品に分けて読める
- **コマンドライン引数**と**環境変数**を使って、プログラムに値を渡せる
- PowerShell と bash（WSL・macOS・Linux のシェル）の書き方の違いに気づける

### たとえ話でつかむ

コマンドを打つ作業は、**役所の窓口でお願いをする**ことに似ています。

- **端末（ターミナル）は窓口のカウンター**です。あなたが文字を書いて差し出し、返事の文字を受け取る場所です。Windows では「Windows Terminal（ウィンドウズ ターミナル）」というアプリがこの役目をします。
- **シェルは窓口の係員**です。あなたが書いたお願い（コマンド）を読み、「それなら `cargo` という担当に回しますね」と、OS に頼んでプログラムを起動します。Windows の **PowerShell**、Linux や macOS の **bash（バッシュ）** や **zsh（ズィーシェル）** がシェルです。
- **コマンドはお願いの文**です。決まった言い方で書かないと、係員に「そんな担当はいません」と言われてしまいます。
- PowerShell と bash は、**違う方言を話す係員**です。どちらも「フォルダの中身を見せて」というお願いを受け付けますが、細かい言い回しが違います。

マウスでアイコンをクリックする代わりに文字でお願いするのは、最初は不便に感じるかもしれません。しかし文字のお願いは、**そのままコピーして人に渡せる**、**同じ手順を何度でも正確にくり返せる**、**記録が残る**という大きな利点があります。開発の仕事でコマンドを使うのは、このためです。

> **用語:** **CLI（シーエルアイ）** … Command Line Interface の略。文字のコマンドで操作するやり方のことです。マウスで操作する画面は **GUI（ジーユーアイ）** と呼びます。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-09" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="60" width="110" height="90" rx="12" fill="#f8fafc" stroke="#475569" stroke-width="2"/>
<text x="69" y="100" font-size="15" font-weight="700" fill="#334155" text-anchor="middle">あなた</text>
<text x="69" y="122" font-size="12" fill="#64748b" text-anchor="middle">キーボード</text>
<rect x="166" y="40" width="150" height="130" rx="12" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="241" y="68" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">端末</text>
<text x="241" y="88" font-size="12" fill="#334155" text-anchor="middle">Windows Terminal</text>
<text x="241" y="116" font-size="12" fill="#334155" text-anchor="middle">文字を入力して</text>
<text x="241" y="134" font-size="12" fill="#334155" text-anchor="middle">結果を表示する窓</text>
<rect x="358" y="40" width="150" height="130" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="433" y="68" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">シェル</text>
<text x="433" y="88" font-size="12" fill="#334155" text-anchor="middle">PowerShell / bash</text>
<text x="433" y="116" font-size="12" fill="#334155" text-anchor="middle">コマンドを読んで</text>
<text x="433" y="134" font-size="12" fill="#334155" text-anchor="middle">担当を呼び出す</text>
<rect x="550" y="20" width="156" height="78" rx="12" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="628" y="50" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">プログラム</text>
<text x="628" y="72" font-size="12" fill="#334155" text-anchor="middle">cargo.exe など</text>
<rect x="550" y="112" width="156" height="78" rx="12" fill="#f8fafc" stroke="#475569" stroke-width="2"/>
<text x="628" y="142" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">OS</text>
<text x="628" y="164" font-size="12" fill="#334155" text-anchor="middle">プロセスを作る</text>
<line x1="126" y1="92" x2="162" y2="92" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-09)"/>
<line x1="318" y1="92" x2="354" y2="92" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-09)"/>
<line x1="510" y1="80" x2="546" y2="62" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-09)"/>
<line x1="510" y1="128" x2="546" y2="146" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-09)"/>
<line x1="354" y1="132" x2="318" y2="132" stroke="#94a3b8" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar-00-09)"/>
<line x1="162" y1="132" x2="126" y2="132" stroke="#94a3b8" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar-00-09)"/>
<text x="360" y="214" font-size="12" fill="#64748b" text-anchor="middle">実線 = お願い（コマンド）の流れ　／　点線 = 結果の文字が画面に戻る流れ</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図3-1: 端末・シェル・OS の関係。シェルが OS に頼んでプログラムを起動する</div>
</div>

実際の PowerShell の画面は次のようになります。`PS C:\Users\you>` の部分を**プロンプト**と呼び、「今いるフォルダはここで、お願いを待っています」という意味です。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="700" height="310" rx="10" fill="#0c1222" stroke="#334155" stroke-width="1.5"/>
<rect x="10" y="10" width="700" height="34" rx="10" fill="#1e293b"/>
<rect x="10" y="34" width="700" height="10" fill="#1e293b"/>
<rect x="22" y="16" width="200" height="28" rx="6" fill="#0c1222"/>
<text x="36" y="35" font-size="12" fill="#e2e8f0">Windows PowerShell</text>
<text x="690" y="33" font-size="13" fill="#94a3b8" text-anchor="end">－　□　×</text>
<g font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" font-size="13" xml:space="preserve" style="white-space: pre">
<text x="26" y="72" fill="#e2e8f0"><tspan fill="#facc15">PS C:\Users\you&gt;</tspan> cd rust-study\practice</text>
<text x="26" y="96" fill="#e2e8f0"><tspan fill="#facc15">PS C:\Users\you\rust-study\practice&gt;</tspan> ls src</text>
<text x="46" y="122" fill="#94a3b8">ディレクトリ: C:\Users\you\rust-study\practice\src</text>
<text x="26" y="148" fill="#e2e8f0">Mode            LastWriteTime   Length Name</text>
<text x="26" y="168" fill="#e2e8f0">----            -------------   ------ ----</text>
<text x="26" y="188" fill="#e2e8f0">-a----   2026/10/17     11:30      689 main.rs</text>
<text x="26" y="218" fill="#e2e8f0"><tspan fill="#facc15">PS C:\Users\you\rust-study\practice&gt;</tspan> cargo --version</text>
<text x="26" y="240" fill="#e2e8f0">cargo 1.99.0 (5f94df478 2026-08-27)</text>
<text x="26" y="266" fill="#e2e8f0"><tspan fill="#facc15">PS C:\Users\you\rust-study\practice&gt;</tspan> cargo run -q -- Hanako</text>
<text x="26" y="288" fill="#e2e8f0">こんにちは、Hanako さん</text>
<text x="26" y="310" fill="#e2e8f0"><tspan fill="#facc15">PS C:\Users\you\rust-study\practice&gt;</tspan> <tspan fill="#e2e8f0">_</tspan></text>
</g>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図3-2: PowerShell の画面イメージ。黄色がプロンプト（今いるフォルダ）、白が入力と結果</div>
</div>

コマンドは、決まった順番の部品でできています。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 210" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="20" width="680" height="56" rx="10" fill="#0c1222"/>
<text x="360" y="56" font-size="22" fill="#e2e8f0" text-anchor="middle" font-family="Cascadia Mono, Consolas, monospace"><tspan fill="#60a5fa">cargo</tspan> <tspan fill="#fb923c">run</tspan> <tspan fill="#34d399">--release</tspan> <tspan fill="#94a3b8">--</tspan> <tspan fill="#f472b6">Hanako</tspan></text>
<line x1="198" y1="80" x2="198" y2="104" stroke="#3b82f6" stroke-width="2"/>
<line x1="270" y1="80" x2="270" y2="104" stroke="#f97316" stroke-width="2"/>
<line x1="378" y1="80" x2="378" y2="104" stroke="#10b981" stroke-width="2"/>
<line x1="462" y1="80" x2="462" y2="104" stroke="#64748b" stroke-width="2"/>
<line x1="540" y1="80" x2="540" y2="104" stroke="#db2777" stroke-width="2"/>
<rect x="20" y="106" width="130" height="86" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="85" y="130" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">コマンド名</text>
<text x="85" y="152" font-size="12" fill="#334155" text-anchor="middle">どのプログラム</text>
<text x="85" y="170" font-size="12" fill="#334155" text-anchor="middle">に頼むか</text>
<rect x="160" y="106" width="130" height="86" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="225" y="130" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">サブコマンド</text>
<text x="225" y="152" font-size="12" fill="#334155" text-anchor="middle">その中の</text>
<text x="225" y="170" font-size="12" fill="#334155" text-anchor="middle">どの仕事か</text>
<rect x="300" y="106" width="130" height="86" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="365" y="130" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">オプション</text>
<text x="365" y="152" font-size="12" fill="#334155" text-anchor="middle">- や -- で始まる</text>
<text x="365" y="170" font-size="12" fill="#334155" text-anchor="middle">やり方の指定</text>
<rect x="440" y="106" width="110" height="86" rx="8" fill="#f8fafc" stroke="#64748b"/>
<text x="495" y="130" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">区切り</text>
<text x="495" y="152" font-size="12" fill="#334155" text-anchor="middle">ここから先は</text>
<text x="495" y="170" font-size="12" fill="#334155" text-anchor="middle">プログラムへ</text>
<rect x="560" y="106" width="140" height="86" rx="8" fill="#fdf2f8" stroke="#db2777"/>
<text x="630" y="130" font-size="13" font-weight="700" fill="#be185d" text-anchor="middle">引数</text>
<text x="630" y="152" font-size="12" fill="#334155" text-anchor="middle">プログラムに</text>
<text x="630" y="170" font-size="12" fill="#334155" text-anchor="middle">渡す値</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図3-3: コマンドの部品。「cargo に、release のやり方で run してもらい、Hanako を渡す」</div>
</div>

### まず動かす最小コード

この単元の「最小コード」は、Rust ではなく PowerShell のコマンドです。Windows のスタートメニューで「ターミナル」または「PowerShell」と検索して開き、1行ずつ打って Enter キーを押してください。

#### PowerShell の基本コマンドを打ってみる

> **▼ このコマンドがやること（先に日本語で）:** 今いるフォルダを確かめてから、練習用のフォルダ `shell-practice` を作って中に入ります。メモのファイルを作り、一覧で確かめ、中身を表示し、コピーして、コピーを消し、最後に元のフォルダへ戻ります。

```powershell
# 今いるフォルダを表示する
Get-Location
# shell-practice フォルダを作る
mkdir shell-practice
# その中へ移動する
cd shell-practice
# memo.txt に1行書き込む（文字コードは UTF-8）
Set-Content memo.txt "はじめての PowerShell" -Encoding utf8
# フォルダの中身を一覧する
ls
# ファイルの中身を表示する
cat memo.txt
# コピーを作って、一覧で確かめる
Copy-Item memo.txt memo-backup.txt
ls
# コピーを消す
Remove-Item memo-backup.txt
# 1つ上のフォルダへ戻る
cd ..
# Rust の道具の版を確かめる
cargo --version
rustc --version
```

**▼ 実行結果**（プロンプトとコマンドも含めて、画面に出たとおりに載せています）

```text
PS C:\Users\you> Get-Location

Path
----
C:\Users\you


PS C:\Users\you> mkdir shell-practice


    ディレクトリ: C:\Users\you


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d-----        2026/10/17     11:30                shell-practice


PS C:\Users\you> cd shell-practice
PS C:\Users\you\shell-practice> Set-Content memo.txt "はじめての PowerShell" -Encoding utf8
PS C:\Users\you\shell-practice> ls


    ディレクトリ: C:\Users\you\shell-practice


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----        2026/10/17     11:30             31 memo.txt


PS C:\Users\you\shell-practice> cat memo.txt
はじめての PowerShell
PS C:\Users\you\shell-practice> Copy-Item memo.txt memo-backup.txt
PS C:\Users\you\shell-practice> ls


    ディレクトリ: C:\Users\you\shell-practice


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----        2026/10/17     11:30             31 memo-backup.txt
-a----        2026/10/17     11:30             31 memo.txt


PS C:\Users\you\shell-practice> Remove-Item memo-backup.txt
PS C:\Users\you\shell-practice> cd ..
PS C:\Users\you> cargo --version
cargo 1.99.0 (5f94df478 2026-08-27)
PS C:\Users\you> rustc --version
rustc 1.99.0 (b940084d7 2026-09-28)
```

`cargo --version` と `rustc --version` は、Rust をインストールしていれば版が表示されます。まだの人は、この2行だけエラーになります（下の「よくあるエラー」を見てください）。

#### ▼ コマンドを1つずつ分解して解説

##### 解説1: 今いる場所を知る・移動する

```powershell
Get-Location
cd shell-practice
cd ..
```

- `Get-Location`（ゲット・ロケーション）は「今いるフォルダ」を表示します。プロンプトにも同じものが出ていますが、長くて省略されるときに便利です。短く `pwd` と打っても同じです。
- `cd`（change directory）は移動です。`cd shell-practice` は相対パス（UNIT 02）なので、「今いるフォルダの中の `shell-practice` へ」という意味です。
- `cd ..` は「1つ上のフォルダへ」です。

##### 解説2: 作る・見る

```powershell
mkdir shell-practice
Set-Content memo.txt "はじめての PowerShell" -Encoding utf8
ls
cat memo.txt
```

- `mkdir`（make directory）はフォルダを作ります。
- `Set-Content`（セット・コンテント）はファイルに文字を書き込みます。`-Encoding utf8` は「文字を UTF-8 という方式で保存して」というオプションです。付けないとどうなるかは「よくあるエラー」で見ます。
- `ls`（list）はフォルダの中身の一覧です。`Mode` 列の `d` はフォルダ（directory）、`a` は普通のファイルの印です。`Length` はファイルの大きさ（バイト数）です。
- `cat`（キャット）はファイルの中身を表示します。

##### 解説3: コピーする・消す

```powershell
Copy-Item memo.txt memo-backup.txt
Remove-Item memo-backup.txt
```

- `Copy-Item 元 先` でコピーします。
- `Remove-Item` で消します。**ごみ箱には入らず、すぐ消えます**。消すものを確かめてから打ちましょう。

> **ヒント:** コマンドを途中まで打って **Tab キー**を押すと、ファイル名やコマンド名を補完してくれます（`memo` まで打って Tab → `memo.txt`）。また **↑キー**で、前に打ったコマンドを呼び戻せます。打ち間違いが減るので、早めに慣れておきましょう。

##### 解説4: PowerShell と bash の対応表

PowerShell のコマンドには `Get-Location` のような正式名と、`pwd`・`ls`・`cat` のような短い別名（**エイリアス**）があります。短い別名は bash に似せてあるので、WSL や macOS で bash を使うときにも役立ちます。

| やりたいこと | PowerShell（正式名） | PowerShell（別名） | bash（WSL・macOS・Linux） |
|---|---|---|---|
| 今いるフォルダを表示 | `Get-Location` | `pwd` | `pwd` |
| 移動 | `Set-Location フォルダ` | `cd フォルダ` | `cd フォルダ` |
| 一覧 | `Get-ChildItem` | `ls`・`dir` | `ls` |
| フォルダを作る | `New-Item -ItemType Directory 名前` | `mkdir 名前` | `mkdir 名前` |
| 中身を表示 | `Get-Content ファイル` | `cat ファイル` | `cat ファイル` |
| コピー | `Copy-Item 元 先` | `cp 元 先` | `cp 元 先` |
| 削除 | `Remove-Item ファイル` | `rm ファイル` | `rm ファイル` |
| 環境変数を設定 | `$env:名前 = "値"` | ― | `export 名前=値` |
| 直前の終了コード | `$LASTEXITCODE` | ― | `$?` |

### もう一歩: 実務に近い形

コマンドからプログラムに値を渡す方法は、主に2つあります。

- **コマンドライン引数** … コマンドの後ろに並べる値（`cargo run -- Rust`）。そのとき限りの値を渡すのに使います。
- **環境変数** … シェルが覚えておいて、そこから起動したプログラムすべてに見せる「名前付きの値」。設定や秘密の鍵など、**コードに書き込みたくない値**を渡すのによく使います。

> **用語:** **環境変数** … OS やシェルが持っている「名前＝値」のメモ。たとえば `PATH` という環境変数には「コマンドのプログラムを探しに行くフォルダの一覧」が入っています。`cargo` と打つだけで動くのは、`PATH` に `C:\Users\you\.cargo\bin` が入っているからです。

#### 引数と環境変数を受け取る

> **▼ このコードがやること（先に日本語で）:** 起動するときに渡された値（引数）を、番号つきで全部表示します。そのあと、環境変数 `GREETING` の値を読み、設定されていなければ「こんにちは」を使って表示します。

```rust
use std::env;

fn main() {
    // コマンドラインから渡された値を順番に表示する（0 番目はプログラム自身）
    for (i, arg) in env::args().enumerate() {
        println!("引数 {i}: {arg}");
    }

    // 環境変数 GREETING を読む（設定されていなければ「こんにちは」を使う）
    let greeting = env::var("GREETING").unwrap_or(String::from("こんにちは"));
    println!("あいさつ: {greeting}");
}
```

PowerShell で、次の順に実行します。

```powershell
cargo run -q -- Rust "Web アプリ"
$env:GREETING = "おはよう"
cargo run -q
.\target\debug\practice.exe 1 2 3
Remove-Item Env:GREETING
cargo run -q
```

**▼ 実行結果**

```text
PS C:\Users\you\rust-study\practice> cargo run -q -- Rust "Web アプリ"
引数 0: target\debug\practice.exe
引数 1: Rust
引数 2: Web アプリ
あいさつ: こんにちは
PS C:\Users\you\rust-study\practice> $env:GREETING = "おはよう"
PS C:\Users\you\rust-study\practice> cargo run -q
引数 0: target\debug\practice.exe
あいさつ: おはよう
PS C:\Users\you\rust-study\practice> .\target\debug\practice.exe 1 2 3
引数 0: C:\Users\you\rust-study\practice\target\debug\practice.exe
引数 1: 1
引数 2: 2
引数 3: 3
あいさつ: おはよう
PS C:\Users\you\rust-study\practice> Remove-Item Env:GREETING
PS C:\Users\you\rust-study\practice> cargo run -q
引数 0: target\debug\practice.exe
あいさつ: こんにちは
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 引数を番号つきで取り出す

```rust
for (i, arg) in env::args().enumerate() {
    println!("引数 {i}: {arg}");
}
```

- `env::args()` は引数の並びです（UNIT 01 でも使いました）。
- `.enumerate()` は、並びの1つずつに `0, 1, 2, …` の番号を付けてくれます。`(i, arg)` で「番号」と「値」を同時に受け取っています。
- 0番目はプログラム自身のパスです。`cargo run` 経由だと `target\debug\practice.exe`、直接起動すると絶対パスになっています。どう起動されたかによって変わる、ということです。
- `"Web アプリ"` のように**空白を含む値は `"` で囲む**と、1つの引数として渡せます。囲まないと `Web` と `アプリ` の2つに分かれます。

##### 解説2: 環境変数を読む

```rust
let greeting = env::var("GREETING").unwrap_or(String::from("こんにちは"));
```

- `env::var("GREETING")` は環境変数 `GREETING` を読みます。設定されていないこともあるので、結果は成功か失敗のどちらかです。
- `.unwrap_or(既定値)` は「失敗だったら既定値を使う」という意味です。
- PowerShell で `$env:GREETING = "おはよう"` と設定すると、**その PowerShell の窓から起動したプログラム**にだけ見えます。窓を閉じると消え、他の窓には影響しません。
- `Remove-Item Env:GREETING` で設定を消せます。

> **補足（bash の場合）:** WSL や macOS の bash では、`GREETING=おはよう cargo run -q -- Rust` のように、コマンドの前に `名前=値` を書くと「そのコマンドにだけ」環境変数を渡せます。ずっと設定しておくなら `export GREETING=おはよう` です。

### よくあるエラーと読み方

#### コマンドの名前を打ち間違えた

```text
PS C:\Users\you> carg run
carg : 用語 'carg' は、コマンドレット、関数、スクリプト ファイル、または操作可能なプログラムの名前として認識されません。名前が正しく記述されていることを確認し、パスが含まれている場合は
そのパスが正しいことを確認してから、再試行してください。
発生場所 行:1 文字:1
+ carg run
+ ~~~~
    + CategoryInfo          : ObjectNotFound: (carg:String) [], CommandNotFoundException
    + FullyQualifiedErrorId : CommandNotFoundException
```

**何が起きたか:** シェル（係員）が `carg` という担当を探しましたが、見つかりませんでした。`発生場所 行:1 文字:1` と `~~~~` の印が、「1行目の1文字目からの `carg` の部分がおかしい」と指しています。

**直し方:** 綴りを直して `cargo run` と打ちます。綴りが正しいのに同じエラーになる場合は、**そのプログラムがインストールされていない**か、`PATH` にフォルダが入っていません。Rust をインストールした直後なら、**PowerShell の窓を開き直す**と直ることがよくあります（`PATH` の変更は、新しく開いた窓から有効になるためです）。

#### bash の書き方を PowerShell で打った

```text
PS C:\Users\you> export GREETING=hello
export : 用語 'export' は、コマンドレット、関数、スクリプト ファイル、または操作可能なプログラムの名前として認識されません。名前が正しく記述されていることを確認し、パスが含まれてい
る場合はそのパスが正しいことを確認してから、再試行してください。
発生場所 行:1 文字:1
+ export GREETING=hello
+ ~~~~~~
    + CategoryInfo          : ObjectNotFound: (export:String) [], CommandNotFoundException
    + FullyQualifiedErrorId : CommandNotFoundException
```

**何が起きたか:** `export` は bash の言葉なので、PowerShell の係員には通じません。インターネットの記事をコピーして貼ったときによく起きます。

**直し方:** PowerShell では `$env:GREETING = "hello"` と書きます。記事のコマンドを使うときは、**それが PowerShell 向けか bash 向けか**を最初に確かめましょう。この教材では、PowerShell 向けのコードブロックには `powershell`、bash 向けには `bash` の印を付けています。

#### `curl` が思った動きをしない

Web のやりとりを見る道具 `curl`（カール）は UNIT 05 で使いますが、Windows PowerShell 5.1 には落とし穴があります。

```text
PS C:\Users\you> Get-Command curl

CommandType     Name                                               Version    Source
-----------     ----                                               -------    ------
Alias           curl -> Invoke-WebRequest


PS C:\Users\you> Get-Command curl.exe

CommandType     Name                                               Version    Source
-----------     ----                                               -------    ------
Application     curl.exe                                           8.21.0.0   C:\Windows\system32\curl.exe
```

**何が起きたか:** PowerShell 5.1 では、`curl` は **`Invoke-WebRequest` という別のコマンドの別名**になっています。そのため、インターネットの記事どおりに `curl -I ...` などと打っても、オプションの意味が違って、エラーになったり入力待ちで止まったりします。

**直し方:** 本物の curl を使うときは、**`curl.exe` と拡張子まで書きます**。この教材ではいつも `curl.exe` と書きます。

#### PowerShell で作ったファイルを Rust で読めない

`-Encoding utf8` を付けずに `Set-Content` で作ったファイルを、Rust で読んでみます。

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    // memo.txt を UTF-8 の文字列として読み込む
    let text = fs::read_to_string("memo.txt")?;
    println!("memo.txt の中身: {}", text.trim_end());
    Ok(())
}
```

```powershell
Set-Content memo.txt "はじめての PowerShell"
cargo run
```

**▼ 実行結果**

```text
Error: Error { kind: InvalidData, message: "stream did not contain valid UTF-8" }
error: process didn't exit successfully: `target\debug\practice.exe` (exit code: 1)
```

**何が起きたか:** 文字をファイルに保存するときの「文字と数値の対応表」を**文字コード**と呼びます。Windows PowerShell 5.1 の `Set-Content` は、何も指定しないと日本語を **Shift_JIS（シフトジス）** という古い文字コードで保存します。一方 Rust の `read_to_string` は **UTF-8（ユーティーエフエイト）** しか受け付けないので、「中身が正しい UTF-8 ではない（InvalidData）」と失敗しました。

**直し方:** `Set-Content` に `-Encoding utf8` を付けるか、ファイルは VS Code などのエディタ（既定で UTF-8）で作ります。

> **注意:** PowerShell 5.1 の `-Encoding utf8` は、ファイルの先頭に **BOM（ボム）** という目に見えない印を付けます。Rust で読むと、その印が文字列の先頭に1文字ぶん残ります。画面上は見えませんが、「先頭の文字を比べたら一致しない」といった不思議な不具合の原因になることがあります。迷ったら、ファイルはエディタで UTF-8（BOM なし）として作るのがいちばん確実です。

### RustNote ではここで使う

- RustNote を作る間、`cargo`（ビルド・実行・テスト）、`git`（変更の記録）、`turso`（データベースの操作）、`fly`（公開）といったコマンドを使います。どれも「コマンド名・サブコマンド・オプション・引数」の形です（第1巻でインストールします）。
- Topcoat で作ったサーバーは、環境変数 `HOST` と `PORT` で待ち受ける場所を決めます。手元では何も設定せずに `127.0.0.1:3000`、Fly.io の上では `HOST=0.0.0.0`・`PORT=8080` を設定します（UNIT 06・第12巻）。
- データベースの場所やパスワード類も、コードに直接書かず**環境変数**で渡します。コードを公開しても秘密が漏れないようにするためです（第4巻・第9巻）。

### 練習問題

**問題1:** `practice` フォルダにいるとき、`src` フォルダの中身を一覧するコマンドは何でしょうか。また、今いるフォルダを確かめるコマンドを2通り答えてください。

**問題2:** 「もう一歩」のプログラムで「あいさつ: おはよう」と表示させる手順を、PowerShell と bash の両方で書いてください。

**問題3:** 引数が何個渡されたか（プログラム自身は数えない）を表示するプログラムを書いてください。`cargo run -q -- a "b c" d` で「渡された値は 3 個です」と表示されれば正解です。

### 解答と解説

**問題1の解答:** 一覧は `ls src`（正式名なら `Get-ChildItem src`）です。今いるフォルダの確認は `Get-Location` と `pwd` の2通りです。

```text
PS C:\Users\you\rust-study\practice> ls src


    ディレクトリ: C:\Users\you\rust-study\practice\src


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----        2026/10/15     20:47            689 main.rs
```

**問題2の解答:**

```powershell
# PowerShell
$env:GREETING = "おはよう"
cargo run -q
```

```bash
# bash（WSL・macOS・Linux）
GREETING=おはよう cargo run -q
```

bash で実行した結果です（引数に `Rust` も渡しています）。

```text
引数 0: target\debug\practice.exe
引数 1: Rust
あいさつ: おはよう
```

**問題3の解答:**

```rust
use std::env;

fn main() {
    // 0 番目（プログラム自身）を skip(1) で飛ばしてから数える
    let count = env::args().skip(1).count();
    println!("渡された値は {count} 個です");
}
```

```powershell
cargo run -q -- a "b c" d
```

**▼ 実行結果**

```text
渡された値は 3 個です
```

`"b c"` は `"` で囲んでいるので1個として数えられます。`skip(1)` は「先頭から1個飛ばす」、`count()` は「残りの個数を数える」という意味です。

### まとめ

- 端末は文字の窓、シェルはコマンドを読んで OS にプログラムの起動を頼む係員、コマンドは「名前・サブコマンド・オプション・引数」の形のお願いです。
- プログラムへは**コマンドライン引数**（そのときの値）と**環境変数**（設定・秘密）で値を渡せます。
- PowerShell と bash は方言が違います。記事のコマンドがどちら向けかを確かめ、Windows では `curl.exe` と書き、ファイルは UTF-8 で作りましょう。

## UNIT 04: ソースコードから実行まで

### この単元のゴール

- **ソースコード**・**コンパイラ**・**実行ファイル**の関係を、流れ図で説明できる
- `rustc`（コンパイラ）と `cargo`（プロジェクト管理の道具）の役割の違いが分かる
- `cargo new`・`cargo run`・`cargo build`・`cargo check`・`--release` を使い分けられる
- **コンパイル時の失敗**と**実行時の失敗**を区別できる

### たとえ話でつかむ

プログラムが動くまでの流れは、**本の翻訳と出版**にたとえられます。

- **ソースコードは日本語の原稿**です。人間が読み書きしやすい言葉（Rust）で書かれていますが、コンピュータ（CPU）はこのままでは読めません。CPU が読めるのは **機械語**（0 と 1 の並び）だけです。
- **コンパイラ（rustc）は翻訳者**です。原稿を機械語に翻訳します。ただし Rust の翻訳者はとても厳しく、**翻訳する前に原稿を隅々まで校閲**します。「ここで数字と文字を足していますよ」「この変数は書き換え禁止ですよ」と矛盾を1つでも見つけると、翻訳を断って赤ペンの指摘（**コンパイルエラー**）を返します。
- **実行ファイル（.exe）は翻訳済みの本**です。CPU がそのまま読めるので、翻訳者がいなくても何度でも読めます（実行できます）。
- **Cargo（カーゴ）は編集者**です。原稿の置き場所を決め、翻訳者に依頼し、必要な参考資料（他の人が作った部品＝**クレート**）を取り寄せ、できた本を決まった棚にしまってくれます。私たちはふだん編集者の Cargo に頼み、翻訳者の rustc を直接呼ぶことはあまりありません。

> **用語:** **コンパイル** … ソースコードを、CPU が実行できる形（機械語）に翻訳すること。翻訳する道具を **コンパイラ** と呼びます。Rust のコンパイラの名前は `rustc`（ラストシー）です。
>
> **用語:** **クレート（crate）** … Rust のプログラムや部品のひとまとまり。「木箱」という意味です。他の人が作ったクレートは crates.io というサイトで公開されていて、Cargo が自動で取り寄せてくれます（Topcoat もクレートの1つです）。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-10" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
<marker id="ar-00-11" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#dc2626"/></marker>
</defs>
<rect x="12" y="40" width="140" height="90" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="82" y="66" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">ソースコード</text>
<text font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" x="82" y="88" font-size="12" fill="#334155" text-anchor="middle">src\main.rs</text>
<text x="82" y="108" font-size="12" fill="#334155" text-anchor="middle">人が読める文字</text>
<rect x="196" y="40" width="140" height="90" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="266" y="66" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">コンパイル</text>
<text x="266" y="88" font-size="12" fill="#334155" text-anchor="middle">rustc が校閲して</text>
<text x="266" y="108" font-size="12" fill="#334155" text-anchor="middle">機械語に翻訳</text>
<rect x="380" y="40" width="140" height="90" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="450" y="66" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">実行ファイル</text>
<text x="450" y="88" font-size="12" fill="#334155" text-anchor="middle">practice.exe</text>
<text x="450" y="108" font-size="12" fill="#334155" text-anchor="middle">CPU が読める</text>
<rect x="564" y="40" width="144" height="90" rx="10" fill="#f8fafc" stroke="#475569" stroke-width="2"/>
<text x="636" y="66" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">実行（プロセス）</text>
<text x="636" y="88" font-size="12" fill="#334155" text-anchor="middle">OS が読み込んで</text>
<text x="636" y="108" font-size="12" fill="#334155" text-anchor="middle">動かす</text>
<line x1="154" y1="85" x2="192" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-10)"/>
<line x1="338" y1="85" x2="376" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-10)"/>
<line x1="522" y1="85" x2="560" y2="85" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-10)"/>
<line x1="266" y1="132" x2="266" y2="176" stroke="#dc2626" stroke-width="2" marker-end="url(#ar-00-11)"/>
<rect x="166" y="180" width="200" height="80" rx="10" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="266" y="204" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">コンパイルエラー</text>
<text x="266" y="226" font-size="12" fill="#334155" text-anchor="middle">矛盾があると .exe は作られない</text>
<text x="266" y="246" font-size="12" fill="#334155" text-anchor="middle">例: 型の不一致（UNIT 09）</text>
<line x1="636" y1="132" x2="636" y2="176" stroke="#dc2626" stroke-width="2" marker-end="url(#ar-00-11)"/>
<rect x="506" y="180" width="202" height="80" rx="10" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="607" y="204" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">実行時エラー（パニック）</text>
<text x="607" y="226" font-size="12" fill="#334155" text-anchor="middle">動かして初めて分かる失敗</text>
<text x="607" y="246" font-size="12" fill="#334155" text-anchor="middle">例: 範囲外の番号・変換失敗</text>
<text x="20" y="24" font-size="12" fill="#64748b">cargo build ＝ 左の2つの矢印まで　／　cargo run ＝ 右の矢印（実行）まで全部</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-1: ソースコードが実行されるまで。失敗には「コンパイル時」と「実行時」の2種類がある</div>
</div>

Cargo のプロジェクトは、次のような決まった形のフォルダになっています。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-12" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="14" width="330" height="272" rx="12" fill="#f8fafc" stroke="#475569" stroke-width="1.5"/>
<text font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" x="30" y="40" font-size="14" font-weight="700" fill="#334155">hello\（プロジェクトのフォルダ）</text>
<rect x="34" y="54" width="290" height="44" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="46" y="73" font-size="13" font-weight="700" fill="#9a3412">Cargo.toml</text>
<text x="46" y="90" font-size="12" fill="#334155">名前・版・使うクレートの一覧（設定）</text>
<rect x="34" y="106" width="290" height="44" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" x="46" y="125" font-size="13" font-weight="700" fill="#1e40af">src\main.rs</text>
<text x="46" y="142" font-size="12" fill="#334155">ソースコード（あなたが書く）</text>
<rect x="34" y="158" width="290" height="44" rx="8" fill="#ffffff" stroke="#94a3b8"/>
<text x="46" y="177" font-size="13" font-weight="700" fill="#334155">Cargo.lock</text>
<text x="46" y="194" font-size="12" fill="#334155">実際に使ったクレートの版の記録（自動）</text>
<rect x="34" y="210" width="290" height="62" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" x="46" y="229" font-size="13" font-weight="700" fill="#047857">target\（自動で作られる）</text>
<text font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" x="46" y="247" font-size="12" fill="#334155">debug\hello.exe　…　cargo build</text>
<text font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" x="46" y="264" font-size="12" fill="#334155">release\hello.exe … --release 付き</text>
<rect x="420" y="40" width="286" height="66" rx="10" fill="#ffffff" stroke="#475569"/>
<text x="436" y="64" font-size="13" font-weight="700" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">cargo new hello</text>
<text x="436" y="88" font-size="12" fill="#334155">フォルダ・Cargo.toml・main.rs を作る</text>
<rect x="420" y="124" width="286" height="66" rx="10" fill="#ffffff" stroke="#475569"/>
<text x="436" y="148" font-size="13" font-weight="700" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">cargo build</text>
<text x="436" y="172" font-size="12" fill="#334155">src を読んで target に .exe を作る</text>
<rect x="420" y="208" width="286" height="66" rx="10" fill="#ffffff" stroke="#475569"/>
<text x="436" y="232" font-size="13" font-weight="700" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">cargo run</text>
<text x="436" y="256" font-size="12" fill="#334155">必要なら build してから .exe を実行</text>
<line x1="416" y1="73" x2="328" y2="76" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-12)"/>
<line x1="416" y1="157" x2="328" y2="232" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-12)"/>
<line x1="416" y1="241" x2="328" y2="246" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-12)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図4-2: Cargo のプロジェクトの形と、主なコマンドが触る場所</div>
</div>

### まず動かす最小コード

#### `cargo new` でプロジェクトを作り、`cargo run` で動かす

> **▼ このコマンドがやること（先に日本語で）:** `hello` という名前の Rust プロジェクトを作り、中に入って、最初から用意されているプログラムをコンパイルして実行します。もう一度実行して、2回目は翻訳（コンパイル）が省かれることも確かめます。

UNIT 03 で作った `rust-study` フォルダの中で、次のコマンドを打ちます。

```powershell
cargo new hello
cd hello
cat Cargo.toml
cat src\main.rs
cargo run
cargo run
```

**▼ 実行結果**

```text
PS C:\Users\you\rust-study> cargo new hello
    Creating binary (application) `hello` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
PS C:\Users\you\rust-study> cd hello
PS C:\Users\you\rust-study\hello> cat Cargo.toml
[package]
name = "hello"
version = "0.1.0"
edition = "2024"

[dependencies]
PS C:\Users\you\rust-study\hello> cat src\main.rs
fn main() {
    println!("Hello, world!");
}
PS C:\Users\you\rust-study\hello> cargo run
   Compiling hello v0.1.0 (C:\Users\you\rust-study\hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target\debug\hello.exe`
Hello, world!
PS C:\Users\you\rust-study\hello> cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\hello.exe`
Hello, world!
```

#### ▼ 出力を1つずつ分解して解説

##### 解説1: `cargo new` が作ったもの

```text
    Creating binary (application) `hello` package
```

- 「`hello` という名前の、**実行できるアプリ（binary）**のパッケージを作りました」という報告です。
- 2行目の `note:` は「`Cargo.toml` に書ける設定の一覧はこのページにあります」という案内で、エラーではありません。
- 作られたのは `Cargo.toml`・`src\main.rs`・`.gitignore`（Git 用の設定）と、隠しフォルダの `.git` です。

##### 解説2: `Cargo.toml` はプロジェクトの「表紙」

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2024"

[dependencies]
```

- `[package]` の下はこのプロジェクト自身の情報です。`name` は名前、`version` はこのプログラムの版です。
- `edition = "2024"` は「Rust 2024 年版の文法で書いています」という宣言です。Rust は数年ごとに書き方の細かい決まりを見直していて、このシリーズは最新の 2024 年版を使います。
- `[dependencies]`（依存関係）の下には、使いたいクレートを書きます。今は空です。第6巻では、ここに Topcoat が加わります。

> **用語:** **TOML（トムル）** … 設定ファイルの書き方の1つ。`名前 = 値` を並べ、`[見出し]` でまとまりを作ります。

##### 解説3: `cargo run` の3行の報告

```text
   Compiling hello v0.1.0 (C:\Users\you\rust-study\hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target\debug\hello.exe`
```

- `Compiling` … 「`hello` の 0.1.0 版をコンパイルしています」。
- `Finished` … 「終わりました」。`dev` プロファイルは**開発用の設定**で、`unoptimized`（最適化なし）・`debuginfo`（調査用の情報つき）という意味です。かかった時間は 0.30 秒でした。
- `Running` … 「できた `target\debug\hello.exe` を実行します」。この後の `Hello, world!` が、プログラム自身の出力です。
- 2回目は `Compiling` の行がありません。ソースコードが変わっていないので、Cargo が「翻訳し直す必要なし」と判断し、前回の `.exe` をそのまま使ったからです。

##### 解説4: できた実行ファイルを確かめる

`cargo run` の後に一覧を見ると、`target` フォルダと `Cargo.lock` が増えています。

```text
PS C:\Users\you\rust-study\hello> ls


    ディレクトリ: C:\Users\you\rust-study\hello


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d-----        2026/10/17     11:30                src
d-----        2026/10/17     11:30                target
-a----        2026/10/17     11:30              8 .gitignore
-a----        2026/10/17     11:30            149 Cargo.lock
-a----        2026/10/17     11:30             76 Cargo.toml


PS C:\Users\you\rust-study\hello> .\target\debug\hello.exe
Hello, world!
```

- `target\debug\hello.exe` が翻訳済みの本です。**Cargo を通さずに直接実行しても動きます**。
- 実行ファイルは、そのまま別の Windows PC にコピーしても動きます（同じ種類の CPU なら）。Rust を入れていない PC でも動くのが、コンパイルする言語の特徴です。
- `target` フォルダは何度でも作り直せるので、Git での記録の対象外です（`.gitignore` にそう書かれています）。

### もう一歩: 実務に近い形

#### 開発用（debug）と本番用（release）の速さを比べる

> **▼ このコードがやること（先に日本語で）:** 1 から 2 億までの数について「7 で割った余り」を足し合わせ、かかった時間を表示します。同じコードを、開発用の設定と本番用の設定でコンパイルして、速さを比べます。

```rust
use std::hint::black_box;
use std::time::Instant;

fn main() {
    // 計算を始めた時刻を覚えておく
    let start = Instant::now();

    // 2 億回のくり返しで、1 から順に「7 で割った余り」を足していく
    let mut total: u64 = 0;
    for n in 1..=200_000_000u64 {
        // black_box は「この計算を省略しないで」とコンパイラに伝えるための道具
        total += black_box(n) % 7;
    }

    // かかった時間を表示する
    println!("合計: {total}");
    println!("かかった時間: {:?}", start.elapsed());
}
```

```powershell
cargo run
cargo run --release
```

**▼ 実行結果**（`Compiling` などの行も載せています）

```text
   Compiling practice v0.1.0 (C:\Users\you\rust-study\practice)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target\debug\practice.exe`
合計: 599999998
かかった時間: 1.3268229s
   Compiling practice v0.1.0 (C:\Users\you\rust-study\practice)
    Finished `release` profile [optimized] target(s) in 0.71s
     Running `target\release\practice.exe`
合計: 599999998
かかった時間: 545.5482ms
```

答えは同じですが、`--release` 付きは2倍以上速くなりました（時間は PC や、そのとき他に動いているアプリによって変わります）。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 時間を計る

```rust
let start = Instant::now();
// ...
println!("かかった時間: {:?}", start.elapsed());
```

- `Instant::now()` は「今この瞬間」を記録します。`start.elapsed()` は「`start` から何秒たったか」です。
- `{:?}` で表示すると、`1.3268229s`（秒）や `545.5482ms`（ミリ秒＝1000分の1秒）のように単位つきで出ます。

##### 解説2: 大きな数と型

```rust
let mut total: u64 = 0;
for n in 1..=200_000_000u64 {
```

- `u64` は「0 以上の整数で、とても大きな数まで入る型」です。合計が約 6 億になるので、大きめの型を選んでいます。型は第2巻で学びます。
- `200_000_000` の `_` は読みやすくするための区切りで、`200000000` と同じです。

##### 解説3: debug と release の違い

| | `cargo build`（debug） | `cargo build --release` |
|---|---|---|
| 置き場所 | `target\debug\` | `target\release\` |
| コンパイルの速さ | 速い | 遅め（最適化に時間をかける） |
| 実行の速さ | 遅め | 速い |
| 使う場面 | 開発中、何度も直して試すとき | 本番に出すとき、速さを測るとき |

- **最適化**とは、コンパイラが「同じ結果になる、もっと速いやり方」に機械語を作り変えることです。
- `black_box` は、計算を丸ごと省略される（最適化されすぎる）のを防ぐための道具です。速さ比べの実験でだけ使います。

#### コンパイラを直接呼ぶ: `rustc`

Cargo は裏で `rustc` を呼んでいます。1ファイルだけなら、`rustc` を直接使うこともできます。

```rust
fn main() {
    println!("rustc だけでコンパイルしました");
}
```

この内容で `main.rs` を作ったフォルダで、次を実行します。

```powershell
rustc main.rs
ls
.\main.exe
```

**▼ 実行結果**

```text
PS C:\Users\you\rust-study\rustc-only> rustc main.rs
PS C:\Users\you\rust-study\rustc-only> ls


    ディレクトリ: C:\Users\you\rust-study\rustc-only


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----        2026/10/17     11:30         131072 main.exe
-a----        2026/10/17     11:30        1396736 main.pdb
-a----        2026/10/15     20:38             74 main.rs


PS C:\Users\you\rust-study\rustc-only> .\main.exe
rustc だけでコンパイルしました
```

`rustc` は**何も表示せずに**終わり、`main.exe` ができました。成功したときに黙っているのは、多くのコマンドに共通する習慣です。`main.pdb` は、問題を調べるときに使う補助のファイルです。実際の開発では、クレートの取り寄せや設定の管理をしてくれる Cargo を使います。

#### よく使う Cargo のコマンド

| コマンド | すること | いつ使うか |
|---|---|---|
| `cargo new 名前` | 新しいプロジェクトを作る | 始めるとき |
| `cargo run` | コンパイルして実行する | 動かして確かめるとき |
| `cargo build` | コンパイルだけする | 実行ファイルだけ欲しいとき |
| `cargo check` | 校閲（エラー探し）だけして、`.exe` は作らない | 書きながら何度も確かめるとき（いちばん速い） |
| `cargo test` | テストを実行する | 直したら毎回（UNIT 10） |
| `cargo fmt` | コードの見た目（字下げなど）を整える | 保存する前 |
| `cargo clippy` | 「もっと良い書き方」を助言してもらう | 区切りのいいところで |

### よくあるエラーと読み方

#### プロジェクトの外で `cargo run` した

```text
PS C:\Users\you\rust-study> cargo run
error: could not find `Cargo.toml` in `C:\Users\you\rust-study` or any parent directory
```

**何が起きたか:** 「`C:\Users\you\rust-study` にも、その親のフォルダにも `Cargo.toml` が見つからない」というエラーです。Cargo は `Cargo.toml` のある場所をプロジェクトとみなすので、プロジェクトの**外**にいると何をコンパイルすればよいか分かりません。

**直し方:** `cd hello` のように、`Cargo.toml` のあるフォルダへ移動してから実行します。プロンプトで今いる場所を確かめましょう（UNIT 02・03）。

#### コンパイルは通ったのに、実行して止まった

> **▼ このコードがやること（先に日本語で）:** 文字の `"42"` と `"4x2"` を、それぞれ数値に変換しようとします。1つ目は成功しますが、2つ目は数値ではないので、実行中に失敗します。

```rust
fn main() {
    // 文字列を数値に変換する。どちらもコンパイルは通る
    let ok: i32 = "42".parse().unwrap();
    println!("1 つ目: {ok}");

    // "4x2" は数値ではないので、実行してみて初めて失敗が分かる
    let ng: i32 = "4x2".parse().unwrap();
    println!("2 つ目: {ng}");
}
```

**▼ 実行結果**

```text
1 つ目: 42

thread 'main' (53536) panicked at src\main.rs:7:33:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\practice.exe` (exit code: 101)
```

**何が起きたか:**

- コンパイルは成功し、1つ目の `println!` までは実行されました（`1 つ目: 42`）。
- 7行目で、`"4x2"` を数値にしようとして失敗しました。`.parse()` は「成功（`Ok`）か失敗（`Err`）」を返し、`.unwrap()` は「成功なら中身を取り出す、**失敗ならパニックする**」という道具です。
- `ParseIntError { kind: InvalidDigit }` は「整数への変換エラー、種類は『数字でない文字がある』」という意味です。

**直し方:** 「失敗するかもしれない」と分かっている処理では、`unwrap()` を使わず、成功と失敗の両方の場合を書きます（練習問題3）。フォームに入力された値などは、利用者が何を打つか分からないので、必ずこの書き方をします。

> **つまずきポイント:** 「コンパイルが通った ＝ 正しく動く」ではありません。コンパイラが見つけるのは**書き方の矛盾**（型が合わない、変数が無い など）で、**実行してみないと分からない値の問題**（ファイルが無い、入力が数字でない など）は、実行時の失敗になります。Rust はコンパイル時に見つけられる問題を最大限見つけてくれるので、実行時の失敗はかなり少なくなります。

### RustNote ではここで使う

- 開発中は `cargo run` で RustNote のサーバーを起動し、ブラウザで `http://127.0.0.1:3000` を開いて確かめます。Topcoat には、ファイルを保存するたびに自動でビルドし直してくれる `topcoat dev` というコマンドもあります（第6巻）。
- `Cargo.toml` の `[dependencies]` に、Topcoat・データベース用のクレート・パスワード処理用のクレートなどを書きます。どのクレートのどの版を使うかは第9巻で決めます。
- 公開するときは `cargo build --release` で本番用の実行ファイルを作り、Docker という箱に入れて Fly.io に送ります（第12巻）。

### 練習問題

**問題1:** `cargo new greet` を実行すると、どんなファイルとフォルダが作られるでしょうか。また、`cargo run` の後に増えるものは何でしょうか。

**問題2:** 次の場面では、`cargo build`・`cargo run`・`cargo check` のどれを使うのがよいでしょうか。

- a. コードを書いている途中で、エラーが無いかだけを素早く確かめたい
- b. 書き終えたので、動かして結果を見たい
- c. 実行ファイルだけを作って、あとで別の PC で動かしたい

**問題3:** 「よくあるエラー」の `"4x2"` の例を、パニックせずに「変換できません」と表示して最後まで動くように書き直してください。

### 解答と解説

**問題1の解答:** `greet` フォルダの中に `Cargo.toml`・`src\main.rs`・`.gitignore`（と隠しフォルダ `.git`）が作られます。`cargo run` の後には、コンパイル結果の入った `target` フォルダと、使ったクレートの版を記録する `Cargo.lock` が増えます。

**問題2の解答:** a は `cargo check`（`.exe` を作らないぶん最も速い）、b は `cargo run`、c は `cargo build`（本番用なら `cargo build --release`）。

**問題3の解答:**

```rust
fn main() {
    for text in ["42", "4x2"] {
        // parse の結果は「成功(Ok)」か「失敗(Err)」のどちらか
        match text.parse::<i32>() {
            Ok(n) => println!("{text} -> 数値 {n}"),
            Err(e) => println!("{text} -> 変換できません（{e}）"),
        }
    }
}
```

**▼ 実行結果**

```text
42 -> 数値 42
4x2 -> 変換できません（invalid digit found in string）
```

`match` で `Ok` と `Err` の両方の場合を書いたので、失敗しても止まらずに最後まで進みます。`{e}` には、エラーの説明文「invalid digit found in string（文字列の中に数字でない文字があった）」が入ります。`parse::<i32>()` の `::<i32>` は「`i32`（整数）に変換して」という指定です。

### まとめ

- ソースコードは rustc が**校閲してから**機械語に翻訳し、実行ファイルができます。Cargo はその作業全体を取り仕切る編集者です。
- `cargo run` は「必要ならコンパイル → 実行」、`cargo check` は校閲だけ、`--release` は速い本番用です。
- 失敗には**コンパイル時**（書き方の矛盾、`.exe` が作られない）と**実行時**（パニックなど、動かして初めて分かる）の2種類があります。

## UNIT 05: ブラウザ・URL・HTTP

### この単元のゴール

- **URL** を「スキーム・ホスト・ポート・パス・クエリ」の部品に分けて読める
- ブラウザに URL を入れてから画面が出るまでに起きることを、順番に説明できる
- **HTTP** の**リクエスト**と**レスポンス**の中身（メソッド・パス・ヘッダー・本文・ステータスコード）を読める
- `curl.exe` を使って、ブラウザの裏で行われているやりとりを自分の目で見られる

### たとえ話でつかむ

Web のしくみは、**決まった書式の注文書を郵送して、返事をもらう**ことにそっくりです。

- **URL は宛先**です。「どの会社（ホスト）の、どの窓口（ポート）の、どの担当（パス）に、どんな条件（クエリ）で」お願いするかが全部書いてあります。
- **DNS（ディーエヌエス）は電話帳**です。`example.com` のような会社名を、郵便で実際に使う番地（**IP アドレス**、例 `172.66.147.243`）に引き直してくれます。
- **HTTP リクエストは注文書**です。1行目に「何をしてほしいか（例: `GET /notes` ＝ ノートの一覧を見せて）」、続けて添え状（**ヘッダー**）、必要なら中身（**本文**）を書きます。書式が決まっているので、世界中のどのサーバーにも通じます。
- **HTTP レスポンスは返事**です。1行目に結果のスタンプ（**ステータスコード**、例 `200 OK` ＝ 成功、`404 Not Found` ＝ 見つからない）、続けて添え状、最後に中身（多くは **HTML**）が入っています。
- **ブラウザは秘書**です。注文書を書いて送り、届いた HTML を読んで、文字や色や画像をきれいに並べた画面にしてくれます。

> **用語:** **HTTP（エイチティーティーピー）** … HyperText Transfer Protocol の略。ブラウザとサーバーがやりとりするときの「注文書と返事の書き方の決まり（**プロトコル**）」です。**HTTPS** は、そのやりとりを暗号化して、途中で盗み見や書き換えをされないようにしたものです。
>
> **用語:** **HTML（エイチティーエムエル）** … Web ページの中身を書く言葉。`<h1>見出し</h1>` のように、タグで文章の役割を示します。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="14" y="20" width="692" height="54" rx="10" fill="#0c1222"/>
<text x="360" y="55" font-size="19" text-anchor="middle" font-family="Cascadia Mono, Consolas, monospace"><tspan fill="#60a5fa">https</tspan><tspan fill="#94a3b8">://</tspan><tspan fill="#fb923c">your-app.fly.dev</tspan><tspan fill="#94a3b8">:</tspan><tspan fill="#facc15">443</tspan><tspan fill="#34d399">/notes</tspan><tspan fill="#94a3b8">?</tspan><tspan fill="#f472b6">q=rust&amp;page=2</tspan></text>
<rect x="14" y="100" width="120" height="104" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="74" y="124" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">スキーム</text>
<text x="74" y="146" font-size="12" fill="#334155" text-anchor="middle">通信のやり方</text>
<text x="74" y="166" font-size="12" fill="#334155" text-anchor="middle">https = 暗号化</text>
<text x="74" y="186" font-size="12" fill="#334155" text-anchor="middle">http = そのまま</text>
<rect x="144" y="100" width="150" height="104" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="219" y="124" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">ホスト</text>
<text x="219" y="146" font-size="12" fill="#334155" text-anchor="middle">どのコンピュータか</text>
<text x="219" y="166" font-size="12" fill="#334155" text-anchor="middle">DNS で IP アドレス</text>
<text x="219" y="186" font-size="12" fill="#334155" text-anchor="middle">に引き直される</text>
<rect x="304" y="100" width="120" height="104" rx="8" fill="#fefce8" stroke="#ca8a04"/>
<text x="364" y="124" font-size="13" font-weight="700" fill="#a16207" text-anchor="middle">ポート</text>
<text x="364" y="146" font-size="12" fill="#334155" text-anchor="middle">どの窓口か</text>
<text x="364" y="166" font-size="12" fill="#334155" text-anchor="middle">https は 443</text>
<text x="364" y="186" font-size="12" fill="#334155" text-anchor="middle">（ふつう省略）</text>
<rect x="434" y="100" width="120" height="104" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="494" y="124" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">パス</text>
<text x="494" y="146" font-size="12" fill="#334155" text-anchor="middle">どのページ・</text>
<text x="494" y="166" font-size="12" fill="#334155" text-anchor="middle">どの担当か</text>
<rect x="564" y="100" width="142" height="104" rx="8" fill="#fdf2f8" stroke="#db2777"/>
<text x="635" y="124" font-size="13" font-weight="700" fill="#be185d" text-anchor="middle">クエリ</text>
<text x="635" y="146" font-size="12" fill="#334155" text-anchor="middle">追加の条件</text>
<text x="635" y="166" font-size="12" fill="#334155" text-anchor="middle">名前=値 を</text>
<text x="635" y="186" font-size="12" fill="#334155" text-anchor="middle">&amp; でつなぐ</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図5-1: URL の分解。「rust を含むノートの2ページ目を見せて」というお願いになっている</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-13" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="20" width="130" height="250" rx="12" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="79" y="48" font-size="15" font-weight="700" fill="#1e40af" text-anchor="middle">ブラウザ</text>
<rect x="576" y="20" width="130" height="250" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="641" y="48" font-size="15" font-weight="700" fill="#c2410c" text-anchor="middle">サーバー</text>
<rect x="290" y="20" width="140" height="44" rx="10" fill="#f8fafc" stroke="#475569"/>
<text x="360" y="47" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">DNS（電話帳）</text>
<line x1="146" y1="40" x2="286" y2="40" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-13)"/>
<text x="216" y="32" font-size="12" fill="#334155" text-anchor="middle">① 名前は？</text>
<line x1="286" y1="56" x2="146" y2="56" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-13)"/>
<text x="216" y="74" font-size="12" fill="#334155" text-anchor="middle">IP アドレス</text>
<line x1="146" y1="104" x2="572" y2="104" stroke="#475569" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-00-13)"/>
<text x="360" y="96" font-size="12" fill="#334155" text-anchor="middle">② 接続する（https なら暗号化の準備も）</text>
<line x1="146" y1="148" x2="572" y2="148" stroke="#1e40af" stroke-width="2.5" marker-end="url(#ar-00-13)"/>
<text x="360" y="140" font-size="12" fill="#1e40af" text-anchor="middle">③ リクエスト: GET /notes HTTP/1.1 ＋ ヘッダー</text>
<rect x="592" y="164" width="98" height="40" rx="8" fill="#ffffff" stroke="#fdba74"/>
<text x="641" y="181" font-size="12" fill="#9a3412" text-anchor="middle">④ 処理して</text>
<text x="641" y="197" font-size="12" fill="#9a3412" text-anchor="middle">HTML を作る</text>
<line x1="572" y1="222" x2="146" y2="222" stroke="#c2410c" stroke-width="2.5" marker-end="url(#ar-00-13)"/>
<text x="360" y="214" font-size="12" fill="#c2410c" text-anchor="middle">⑤ レスポンス: 200 OK ＋ ヘッダー ＋ HTML</text>
<rect x="30" y="234" width="98" height="28" rx="6" fill="#ffffff" stroke="#93c5fd"/>
<text x="79" y="253" font-size="12" fill="#1e40af" text-anchor="middle">⑥ 画面に描く</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図5-2: URL を入れてから画面が出るまで。不具合はこの①〜⑥のどこで起きたかで切り分ける</div>
</div>

HTTP のリクエストとレスポンスは、どちらも「1行目・ヘッダー・空行・本文」という同じ形をしています。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="180" y="22" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">リクエスト（ブラウザ → サーバー）</text>
<rect x="14" y="32" width="332" height="250" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<rect x="24" y="44" width="312" height="30" rx="4" fill="#dbeafe"/>
<text x="32" y="64" font-size="13" font-weight="700" fill="#1e3a8a" font-family="Cascadia Mono, Consolas, monospace">POST /notes HTTP/1.1</text>
<text x="32" y="96" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">Host: your-app.fly.dev</text>
<text x="32" y="116" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">Cookie: session=...</text>
<text x="32" y="136" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">Content-Type: application/</text>
<text x="32" y="152" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">  x-www-form-urlencoded</text>
<text x="32" y="176" font-size="12" fill="#94a3b8">（空行 ＝ ヘッダーの終わり）</text>
<rect x="24" y="188" width="312" height="30" rx="4" fill="#ffffff" stroke="#93c5fd"/>
<text x="32" y="208" font-size="12" fill="#0f172a" font-family="Cascadia Mono, Consolas, monospace">title=牛乳を買う&amp;body=</text>
<text x="32" y="242" font-size="12" fill="#1e40af">1行目: メソッド・パス・HTTP の版</text>
<text x="32" y="262" font-size="12" fill="#1e40af">本文: フォームに入力した内容</text>
<text x="540" y="22" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">レスポンス（サーバー → ブラウザ）</text>
<rect x="374" y="32" width="332" height="250" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<rect x="384" y="44" width="312" height="30" rx="4" fill="#ffedd5"/>
<text x="392" y="64" font-size="13" font-weight="700" fill="#7c2d12" font-family="Cascadia Mono, Consolas, monospace">HTTP/1.1 200 OK</text>
<text x="392" y="96" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">Content-Type: text/html;</text>
<text x="392" y="112" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">  charset=utf-8</text>
<text x="392" y="132" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">Content-Length: 1834</text>
<text x="392" y="176" font-size="12" fill="#94a3b8">（空行 ＝ ヘッダーの終わり）</text>
<rect x="384" y="188" width="312" height="30" rx="4" fill="#ffffff" stroke="#fdba74"/>
<text x="392" y="208" font-size="12" fill="#0f172a" font-family="Cascadia Mono, Consolas, monospace">&lt;!doctype html&gt;&lt;html&gt;...</text>
<text x="392" y="242" font-size="12" fill="#c2410c">1行目: HTTP の版・ステータスコード</text>
<text x="392" y="262" font-size="12" fill="#c2410c">本文: 画面に描く HTML</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図5-3: HTTP のリクエストとレスポンスの形。どちらも「1行目・ヘッダー・空行・本文」（値は例）</div>
</div>

**メソッド**は「何をしてほしいか」を表す動詞です。RustNote で使うのは主に次の2つです。

| メソッド | 意味 | 例 |
|---|---|---|
| `GET` | 見せて（データを変えない） | `GET /notes` ノート一覧を見せて |
| `POST` | 送るので処理して（データが変わる） | `POST /notes` このノートを作って |

**ステータスコード**は返事の結果を表す3桁の数字で、最初の桁でおおまかな意味が分かります。

| コード | 名前 | 意味 | RustNote での例 |
|---|---|---|---|
| `200` | OK | 成功 | 一覧ページを返した |
| `303` | See Other | 「処理したので、こちらの URL を見に行って」 | ノート作成後に一覧へ移動させる |
| `400` | Bad Request | リクエストの形がおかしい | 送られてきた値が読めない |
| `401` / `403` | Unauthorized / Forbidden | ログインが必要／権限がない | （UNIT 08 で説明） |
| `404` | Not Found | 見つからない | 存在しない、または**他人の**ノート |
| `500` | Internal Server Error | サーバー側の失敗 | プログラムの不具合 |

2xx は成功、3xx は「別の場所へ」、4xx は**頼んだ側**の問題、5xx は**サーバー側**の問題、と覚えておくと便利です。

### まず動かす最小コード

#### `curl.exe` でブラウザの裏側をのぞく

**curl（カール）** は、ブラウザの代わりに HTTP のリクエストを送り、返ってきたものを**そのまま文字で見せてくれる**道具です。Windows 10 以降には最初から入っています（UNIT 03 で見たとおり、PowerShell では必ず `curl.exe` と書きます）。

> **▼ このコマンドがやること（先に日本語で）:** 説明用に用意されている公式のサイト `example.com` に HTTP のリクエストを送り、送った注文書（リクエスト）と、返ってきた返事（レスポンス）のヘッダーを表示します。本文の HTML は表示せずに捨てます。

```powershell
curl.exe -4 -sv http://example.com/ -o NUL
```

**▼ 実行結果**（あなたの PC のアドレスの部分は伏せています）

```text
* Host example.com:80 was resolved.
* IPv6: (none)
* IPv4: 172.66.147.243, 104.20.23.154
*   Trying 172.66.147.243:80...
* Established connection to example.com (172.66.147.243 port 80) from 192.168.x.x port 56035
* using HTTP/1.x
> GET / HTTP/1.1
> Host: example.com
> User-Agent: curl/8.21.0
> Accept: */*
>
* Request completely sent off
< HTTP/1.1 200 OK
< Date: Thu, 15 Oct 2026 11:38:59 GMT
< Content-Type: text/html; charset=utf-8
< Transfer-Encoding: chunked
< Connection: keep-alive
< Age: 9
< Server: cloudflare
< Last-Modified: Tue, 13 Oct 2026 22:05:48 GMT
< Allow: GET, HEAD
< Accept-Ranges: bytes
< cf-cache-status: HIT
< CF-RAY: a4ae8a66ff859649-KIX
< alt-svc: h3=":443"; ma=86400
<
{ [589 bytes data]
* Connection #0 to host example.com:80 left intact
```

#### ▼ 出力を1つずつ分解して解説

##### 解説1: コマンドのオプション

```powershell
curl.exe -4 -sv http://example.com/ -o NUL
```

- `-v`（verbose: 詳しく）は、やりとりの中身を表示するオプションです。`-s`（silent）は進み具合の表示を消します。2つまとめて `-sv` と書けます。
- `-4` は「IPv4 という形式のアドレスで接続して」という指定です。表示を短くするために付けています。
- `-o NUL` は「本文は `NUL`（何も残らない特別な場所）に捨てて」という意味です。bash では `-o /dev/null` と書きます。

##### 解説2: `*` の行 — 名前を引いて接続する（図5-2 の①②）

```text
* Host example.com:80 was resolved.
* IPv4: 172.66.147.243, 104.20.23.154
* Established connection to example.com (172.66.147.243 port 80) from 192.168.x.x port 56035
```

- `*` の行は curl 自身の報告です。
- `was resolved` は「DNS で名前を引けた」という意味で、`example.com` の IP アドレスが2つ見つかりました。
- `Established connection` は「接続できた」。`port 80` は、`http` の URL でポートを省略したときに使われる番号です（`https` なら 443）。

##### 解説3: `>` の行 — 送った注文書（図5-2 の③）

```text
> GET / HTTP/1.1
> Host: example.com
> User-Agent: curl/8.21.0
> Accept: */*
>
```

- `>` の行は、**curl がサーバーへ送ったリクエスト**そのものです。
- 1行目 `GET / HTTP/1.1` は「HTTP/1.1 の決まりで、パス `/`（トップページ）を GET（見せて）」。
- `Host:` はどのサイト宛てか、`User-Agent:` は送り主のソフトの名前、`Accept: */*` は「どんな種類の中身でも受け取ります」というヘッダーです。
- 最後の `>` だけの行が**空行**で、「ヘッダーはここまで」の合図です。GET なので本文はありません。

##### 解説4: `<` の行 — 返ってきた返事（図5-2 の⑤）

```text
< HTTP/1.1 200 OK
< Content-Type: text/html; charset=utf-8
< Server: cloudflare
<
{ [589 bytes data]
```

- `<` の行は、**サーバーから返ってきたレスポンス**です。
- 1行目の `200 OK` が結果のスタンプで、「成功」です。
- `Content-Type: text/html; charset=utf-8` は「本文は HTML で、文字コードは UTF-8 です」という添え状です。ブラウザはこれを見て、本文を HTML として描きます。
- 空行の後の `{ [589 bytes data]` は、「本文を 589 バイト受け取った（が `-o NUL` で捨てた）」という報告です。
- `-v` の代わりに `-i` を付けると、ヘッダーと本文の HTML をそのまま表示できます。

#### URL を部品に分けるプログラム

> **▼ このコードがやること（先に日本語で）:** 図5-1 の URL を、文字の区切り（`://`・`/`・`?`・`&`・`=`）を手がかりにして、スキーム・ホスト・パス・クエリに分けて表示します。URL が「決まった区切りを持つ、ただの文字」であることを確かめます。

```rust
fn main() {
    let url = "https://your-app.fly.dev/notes?q=rust&page=2";
    println!("URL 全体: {url}");

    // 「://」の前がスキーム（通信のやり方）
    let (scheme, rest) = url.split_once("://").unwrap();
    println!("スキーム: {scheme}");

    // 最初の「/」の前がホスト（どのコンピュータか）
    let (host, path_and_query) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };
    println!("ホスト  : {host}");

    // 「?」の前がパス（どのページか）、後ろがクエリ（追加の条件）
    let (path, query) = path_and_query
        .split_once('?')
        .unwrap_or((path_and_query, ""));
    println!("パス    : {path}");

    // クエリは「名前=値」が & でつながっている
    for pair in query.split('&') {
        if let Some((name, value)) = pair.split_once('=') {
            println!("クエリ  : {name} = {value}");
        }
    }
}
```

**▼ 実行結果**

```text
URL 全体: https://your-app.fly.dev/notes?q=rust&page=2
スキーム: https
ホスト  : your-app.fly.dev
パス    : /notes
クエリ  : q = rust
クエリ  : page = 2
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 区切りで2つに分ける `split_once`

```rust
let (scheme, rest) = url.split_once("://").unwrap();
```

- `split_once("://")` は、文字列を**最初に見つかった** `://` の前後の2つに分けます。結果は `("https", "your-app.fly.dev/notes?q=rust&page=2")` です。
- 区切りが見つからないこともあるので結果は `Option` で、`.unwrap()` で中身を取り出しています（見つからなければパニックします。練習用の割り切りです）。
- `let (scheme, rest) = ...` は、2つ組の値を2つの変数に一度に受け取る書き方です。

##### 解説2: 位置を探して切り出す

```rust
let (host, path_and_query) = match rest.find('/') {
    Some(i) => (&rest[..i], &rest[i..]),
    None => (rest, "/"),
};
```

- `rest.find('/')` は、最初の `/` が**何文字目（何バイト目）にあるか**を探します。
- `&rest[..i]` は「先頭から i の手前まで」、`&rest[i..]` は「i から最後まで」の切り出しです。
- `/` が無い URL（`https://example.com` など）のときは、パスを `/` とみなしています。

##### 解説3: クエリを1組ずつ読む

```rust
for pair in query.split('&') {
    if let Some((name, value)) = pair.split_once('=') {
        println!("クエリ  : {name} = {value}");
    }
}
```

- `query.split('&')` は `&` で区切った部品を1つずつ取り出します（`q=rust`、`page=2`）。
- `if let Some(...) = ...` は「区切りが見つかったときだけ中を実行する」という書き方です。

> **注意:** 本物の URL には、日本語や記号を `%E7%89%9B` のような形に置き換える決まり（**URL エンコード**）などもあります。実際のアプリでは、こうした細かい決まりまで正しく扱ってくれる部品（Topcoat など）に任せます。ここでは「URL は区切りの決まった文字列だ」と分かれば十分です。

### もう一歩: 実務に近い形

#### HTTP の注文書を、自分の手で書いて送る

ブラウザや curl がしていることは、結局「決まった形の文字を送って、文字を受け取る」だけです。それを Rust で直接やってみます。

> **▼ このコードがやること（先に日本語で）:** `example.com` の 80 番ポートに接続し、HTTP のリクエストを**文字として**組み立てて送ります。返ってきた文字を全部受け取り、空行を境にヘッダーと本文に分けて表示します。

```rust
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // example.com の 80 番ポート（http の窓口）に接続する
    let mut stream = TcpStream::connect("example.com:80")?;

    // HTTP リクエストは「決まった形の文字」。1 行ずつ \r\n で区切り、空行で終わる
    let request = "GET / HTTP/1.1\r\n\
                   Host: example.com\r\n\
                   User-Agent: rust-study\r\n\
                   Connection: close\r\n\
                   \r\n";
    stream.write_all(request.as_bytes())?;

    // サーバーからの返事（レスポンス）を最後まで受け取る
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // 空行（\r\n\r\n）の前がヘッダー、後ろが本文
    let (head, body) = response.split_once("\r\n\r\n").unwrap_or((&response, ""));
    println!("----- ヘッダー部分 -----");
    println!("{head}");
    println!("----- 本文の先頭 60 文字 -----");
    let preview: String = body.chars().take(60).collect();
    println!("{preview}");
    Ok(())
}
```

**▼ 実行結果**

```text
----- ヘッダー部分 -----
HTTP/1.1 200 OK
Date: Sat, 17 Oct 2026 02:30:02 GMT
Content-Type: text/html; charset=utf-8
Transfer-Encoding: chunked
Connection: close
Server: cloudflare
Last-Modified: Tue, 13 Oct 2026 20:08:28 GMT
Allow: GET, HEAD
Accept-Ranges: bytes
Age: 11679
cf-cache-status: HIT
CF-RAY: a4bbe10c7ef7440f-KIX
alt-svc: h3=":443"; ma=86400
----- 本文の先頭 60 文字 -----
241
<!doctype html><html lang=en><head><meta charset=utf-8>
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 接続する `TcpStream::connect`

```rust
let mut stream = TcpStream::connect("example.com:80")?;
```

- **TCP（ティーシーピー）** は、2台のコンピュータの間に「文字を順番どおり確実に届ける管」をつなぐしくみです。HTTP はこの管の上を流れます。
- `connect("example.com:80")` は、名前を引いて（DNS）、80 番ポートに接続します。図5-2 の①②を1行でやっています。
- 接続できたら、`stream` に書けば相手に届き、`stream` から読めば相手からの文字が受け取れます。

##### 解説2: 注文書を文字で組み立てる

```rust
let request = "GET / HTTP/1.1\r\n\
               Host: example.com\r\n\
               User-Agent: rust-study\r\n\
               Connection: close\r\n\
               \r\n";
```

- HTTP の決まりでは、行の区切りは `\r\n`（**CR LF**：復帰と改行の2文字）です。
- 最後の `\r\n` だけの行が空行で、「ヘッダーはここまで」の合図です。これを忘れると、サーバーは続きを待ち続けます。
- 行末の `\` は「ソースコード上の改行と、次の行の先頭の空白を無視して1つの文字列につなげて」という Rust の書き方です。
- `Connection: close` は「返事を送ったら接続を切ってください」というお願いです。これで `read_to_string` が「最後まで読んだ」と分かります。

##### 解説3: 返事を受け取って分ける

```rust
stream.read_to_string(&mut response)?;
let (head, body) = response.split_once("\r\n\r\n").unwrap_or((&response, ""));
```

- `read_to_string` で、接続が切れるまでの全部を `response` に読み込みます。
- 最初の空行（`\r\n\r\n`）でヘッダーと本文を分けています。
- 本文の先頭の `241` は HTML ではありません。ヘッダーに `Transfer-Encoding: chunked` とあるとおり、このサーバーは本文を「塊（chunk）」に分けて送っていて、`241` は「次の塊は 16 進数で 241（＝577）バイト」という目印です。ブラウザや curl は、こうした細かい決まりも自動で処理してくれています。

### よくあるエラーと読み方

#### 名前が引けない（curl のエラー 6）

```text
PS C:\Users\you> curl.exe -sS http://exampel-typo.invalid/
curl: (6) Could not resolve host: exampel-typo.invalid
```

**何が起きたか:** 図5-2 の①で失敗しています。DNS（電話帳）に `exampel-typo.invalid` という名前が載っていませんでした。`(6)` は curl のエラー番号です。

**直し方:** URL の綴りを確かめます。綴りが正しいなら、インターネットにつながっているかを確かめます。

#### 接続できない（curl のエラー 7）

```text
PS C:\Users\you> curl.exe -sS http://127.0.0.1:8080/
curl: (7) Failed to connect to 127.0.0.1:8080 after 2030 ms: Could not connect to server
```

**何が起きたか:** 図5-2 の②で失敗しています。名前は分かった（ここでは `127.0.0.1` を直接指定）けれど、その番地の 8080 番の窓口で**誰も待っていなかった**のです。サーバーを起動し忘れている、ポート番号を間違えている、のどちらかがほとんどです（UNIT 06 で、このポートで待つサーバーを作ります）。

**直し方:** サーバーが起動しているか、ポート番号が合っているかを確かめます。

#### 返事は来たが 404

```text
PS C:\Users\you> curl.exe -s -i http://example.com/notes-not-here
HTTP/1.1 404 Not Found
Date: Thu, 15 Oct 2026 11:39:00 GMT
Content-Type: text/html; charset=utf-8
Transfer-Encoding: chunked
Connection: keep-alive
Server: cloudflare
Age: 9487
cf-cache-status: HIT
```

（本文の HTML は省略しています）

**何が起きたか:** これは**通信としては成功**しています。サーバーまで届き、サーバーが「そのパスのページはありません（404）」と返事をくれました。①〜⑤のうち④の結果です。

**直し方:** パスの綴りを確かめます。自分のサーバーなら、そのパスを受け付ける処理（**ルート**）を作ったかを確かめます。

> **つまずきポイント:** 「ページが表示されない」とひとことで言っても、①名前が引けない、②接続できない、④サーバーが 404 や 500 を返した、⑥ HTML の書き方がおかしい、では原因も直し方もまったく違います。`curl.exe -v` で**どこまで進んだか**を確かめるのが、Web の不具合を調べる第一歩です。

### RustNote ではここで使う

- RustNote のページは、すべて「メソッド＋パス」の組で決まっています（巻頭の表）。たとえば一覧は `GET /notes`、作成は `POST /notes`、検索は `GET /notes?q=買い物` です。
- フォームの送信（POST）に成功したら、RustNote は HTML を直接返さずに **303 See Other** で「`GET /notes` を見に行って」と返します。こうしておくと、ブラウザの再読み込みボタンで同じノートが二重に作られる事故を防げます。この作法を **PRG（Post/Redirect/Get）** と呼びます（第6巻・第10巻）。
- 他の人のノートの URL を開かれたときは、**404** を返します。理由は UNIT 08 で説明します。

### 練習問題

**問題1:** 次の URL を、スキーム・ホスト・ポート・パス・クエリに分けてください。

```text
http://127.0.0.1:8080/notes/42/edit?from=list
```

**問題2:** URL を分けるプログラムを改造して、ホストとポートを分けて表示してください。ポートが書かれていない URL のときは、`http` なら `80`、`https` なら `443` と表示します。

**問題3:** 次の場面で返すステータスコードとして、最もふさわしいものを選んでください（200・303・404・500）。

- a. ノート一覧のページを表示する
- b. ノートを作成したので、一覧のページへ移動してもらう
- c. `/notes/9999` が開かれたが、そんな番号のノートは無い
- d. プログラムの不具合で、処理の途中で失敗した

### 解答と解説

**問題1の解答:** スキームは `http`、ホストは `127.0.0.1`、ポートは `8080`、パスは `/notes/42/edit`、クエリは `from=list` です。ホストに IP アドレスを直接書くこともできます。

**問題2の解答:**

```rust
fn main() {
    let url = "http://127.0.0.1:8080/notes/42/edit?from=list";

    let (scheme, rest) = url.split_once("://").unwrap();
    let (host_and_port, path_and_query) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };

    // 「ホスト:ポート」を「:」で分ける。ポートが書かれていなければ既定値を使う
    let (host, port) = match host_and_port.split_once(':') {
        Some((host, port)) => (host, port),
        None if scheme == "https" => (host_and_port, "443"),
        None => (host_and_port, "80"),
    };

    let (path, query) = path_and_query
        .split_once('?')
        .unwrap_or((path_and_query, ""));

    println!("スキーム: {scheme}");
    println!("ホスト  : {host}");
    println!("ポート  : {port}");
    println!("パス    : {path}");
    println!("クエリ  : {query}");
}
```

**▼ 実行結果**

```text
スキーム: http
ホスト  : 127.0.0.1
ポート  : 8080
パス    : /notes/42/edit
クエリ  : from=list
```

`None if scheme == "https" =>` は「`None` で、しかも条件が成り立つとき」という書き方です。`match` は上から順に調べるので、https でなければ最後の `None =>` に進みます。

**問題3の解答:** a は `200`、b は `303`、c は `404`、d は `500`。4xx は「頼み方（URL）の問題」、5xx は「サーバー側の問題」です。

### まとめ

- URL は「スキーム・ホスト・ポート・パス・クエリ」からなる宛先で、DNS がホスト名を IP アドレスに引き直します。
- HTTP のリクエストとレスポンスは「1行目・ヘッダー・空行・本文」の決まった形の**文字**で、メソッド（GET・POST）とステータスコード（200・303・404・500 など）が要です。
- 表示されないときは `curl.exe -v` で、名前解決・接続・返事のどこまで進んだかを確かめます。

## UNIT 06: サーバー・ポート・localhost

### この単元のゴール

- **サーバー**とは「リクエストを待ち続けるプロセス」だと説明できる
- **IP アドレス**・**ポート**・**localhost（127.0.0.1）**・**0.0.0.0** の違いを説明できる
- Rust の標準ライブラリだけで小さな Web サーバーを作り、`curl.exe` とブラウザからアクセスできる
- 「ポートが使用中」「実行ファイルを上書きできない」というエラーに対処できる

### たとえ話でつかむ

ネットワークの住所は、**大きなマンション**で考えると分かりやすくなります。

- **IP アドレスはマンションの住所**です。インターネットにつながったコンピュータ1台1台に付いています（例 `172.66.147.243`）。
- **ポートは部屋番号**です。1つのマンション（1台の PC）には 1〜65535 番までの部屋があり、それぞれの部屋に別々の住人（プロセス）が住めます。同じ住所に届いた荷物も、部屋番号で届け先のプロセスが決まります。
- **サーバーは、部屋で来客を待ち続けている住人**です。「8080 号室で待っています」と管理人（OS）に届け出ておくと、8080 号室宛ての訪問者が来るたびに取り次いでもらえます。この届け出を **bind（バインド）**、待つことを **listen（リッスン）** と呼びます。1つの部屋に住めるのは1人だけです。
- **localhost（ローカルホスト）＝ 127.0.0.1 は「このマンションの中」**を表す特別な住所です。内線電話のようなもので、外の人はこの番号にかけられません。開発中は、自分の PC の中だけでサーバーとブラウザをやりとりさせるのに使います。
- **0.0.0.0 で待つのは「どの入口から来た客も受け付ける」**という届け出です。127.0.0.1 で待つと内線（同じ PC の中）からしか来客がありませんが、0.0.0.0 で待つと、ネットワークの外からの来客も受け付けます。

> **用語:** **サーバー** … 頼まれごと（リクエスト）を待ち受けて、返事（レスポンス）を返す側のプログラム、またはそれが動いているコンピュータ。頼む側（ブラウザなど）は **クライアント** と呼びます。
>
> **用語:** **IP アドレス** … ネットワーク上のコンピュータの住所。`127.0.0.1` のような4つの数字の形（IPv4）と、`::1` のような長い形（IPv6）があります。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-14" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="250" y="14" width="456" height="262" rx="14" fill="#f8fafc" stroke="#475569" stroke-width="2"/>
<text x="478" y="40" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">あなたの PC（IP アドレス = 住所）</text>
<rect x="270" y="56" width="132" height="96" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="336" y="80" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">3000 番</text>
<text x="336" y="104" font-size="12" fill="#334155" text-anchor="middle">RustNote</text>
<text x="336" y="122" font-size="12" fill="#334155" text-anchor="middle">（開発中）</text>
<rect x="414" y="56" width="132" height="96" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="3"/>
<text x="480" y="80" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">8080 番</text>
<text x="480" y="104" font-size="12" fill="#334155" text-anchor="middle">練習サーバー</text>
<text x="480" y="122" font-size="12" fill="#334155" text-anchor="middle">practice.exe</text>
<rect x="558" y="56" width="132" height="96" rx="10" fill="#ffffff" stroke="#cbd5e1" stroke-dasharray="5 4"/>
<text x="624" y="80" font-size="13" font-weight="700" fill="#94a3b8" text-anchor="middle">5000 番</text>
<text x="624" y="104" font-size="12" fill="#94a3b8" text-anchor="middle">空き部屋</text>
<text x="624" y="122" font-size="12" fill="#94a3b8" text-anchor="middle">（誰も待っていない）</text>
<rect x="270" y="170" width="420" height="92" rx="10" fill="#eff6ff" stroke="#3b82f6"/>
<text x="286" y="194" font-size="13" font-weight="700" fill="#1e40af">ブラウザ・curl（同じ PC の中のクライアント）</text>
<text x="286" y="218" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">http://127.0.0.1:8080/  → 8080 番へ</text>
<text x="286" y="238" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">http://localhost:8080/  → 同じ意味</text>
<text x="286" y="256" font-size="12" fill="#334155" font-family="Cascadia Mono, Consolas, monospace">http://127.0.0.1:5000/  → 誰もいない</text>
<line x1="480" y1="168" x2="480" y2="156" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-14)"/>
<rect x="14" y="56" width="216" height="206" rx="12" fill="#ffffff" stroke="#94a3b8"/>
<text x="122" y="82" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">覚えておきたい番号</text>
<text x="30" y="110" font-size="12" fill="#334155">80　 http の既定</text>
<text x="30" y="134" font-size="12" fill="#334155">443　https の既定</text>
<text x="30" y="158" font-size="12" fill="#334155">3000 Topcoat の既定</text>
<text x="30" y="182" font-size="12" fill="#334155">8080 Fly.io の RustNote</text>
<text x="30" y="206" font-size="12" fill="#334155">　　 ／ この巻の練習</text>
<text x="30" y="236" font-size="12" fill="#64748b">1つのポートで待てるのは</text>
<text x="30" y="254" font-size="12" fill="#64748b">1つのプロセスだけ</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図6-1: ポートは部屋番号。URL の「:8080」で、どのプロセスに届けるかが決まる</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-15" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
<marker id="ar-00-16" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#dc2626"/></marker>
</defs>
<text x="180" y="22" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">127.0.0.1:8080 で待つ（開発中）</text>
<rect x="14" y="34" width="332" height="170" rx="12" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="30" y="58" font-size="12" font-weight="700" fill="#1e40af">あなたの PC</text>
<rect x="190" y="74" width="140" height="50" rx="8" fill="#ffffff" stroke="#3b82f6"/>
<text x="260" y="104" font-size="12" fill="#1e40af" text-anchor="middle">サーバー</text>
<rect x="30" y="74" width="110" height="50" rx="8" fill="#ffffff" stroke="#3b82f6"/>
<text x="85" y="104" font-size="12" fill="#1e40af" text-anchor="middle">ブラウザ</text>
<line x1="142" y1="99" x2="186" y2="99" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-15)"/>
<text x="30" y="150" font-size="12" fill="#047857">同じ PC の中からは届く</text>
<line x1="180" y1="236" x2="250" y2="132" stroke="#dc2626" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar-00-16)"/>
<text x="30" y="178" font-size="12" fill="#b91c1c">外の PC からは</text>
<text x="30" y="196" font-size="12" fill="#b91c1c">届かない ×</text>
<rect x="110" y="236" width="140" height="40" rx="8" fill="#ffffff" stroke="#94a3b8"/>
<text x="180" y="261" font-size="12" fill="#334155" text-anchor="middle">別の PC・スマホ</text>
<text x="540" y="22" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">0.0.0.0:8080 で待つ（Fly.io）</text>
<rect x="374" y="34" width="332" height="170" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="390" y="58" font-size="12" font-weight="700" fill="#c2410c">Fly.io の上のマシン</text>
<rect x="550" y="74" width="140" height="50" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="620" y="96" font-size="12" fill="#c2410c" text-anchor="middle">RustNote</text>
<text x="620" y="114" font-size="12" fill="#c2410c" text-anchor="middle">HOST=0.0.0.0</text>
<rect x="390" y="74" width="120" height="50" rx="8" fill="#ffffff" stroke="#f97316"/>
<text x="450" y="96" font-size="12" fill="#c2410c" text-anchor="middle">Fly.io の</text>
<text x="450" y="114" font-size="12" fill="#c2410c" text-anchor="middle">受付（443）</text>
<line x1="512" y1="99" x2="546" y2="99" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-15)"/>
<text x="620" y="150" font-size="12" fill="#047857" text-anchor="middle">外からの https を</text>
<text x="620" y="168" font-size="12" fill="#047857" text-anchor="middle">8080 番へ取り次ぐ</text>
<line x1="540" y1="236" x2="460" y2="130" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-15)"/>
<rect x="470" y="236" width="140" height="40" rx="8" fill="#ffffff" stroke="#94a3b8"/>
<text x="540" y="261" font-size="12" fill="#334155" text-anchor="middle">世界中のブラウザ</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図6-2: 待ち受ける住所の違い。127.0.0.1 は自分の PC の中だけ、0.0.0.0 は外からも受け付ける</div>
</div>

> **つまずきポイント:** `0.0.0.0` は「**どの入口でも待つ**」という待ち受け側の指定で、ブラウザに打つ宛先ではありません。開発中のサーバーにアクセスするときは `http://127.0.0.1:3000` や `http://localhost:3000` を使います。

### まず動かす最小コード

#### Rust だけで作る、いちばん小さな Web サーバー

> **▼ このコードがやること（先に日本語で）:** 自分の PC（127.0.0.1）の 8080 番ポートで来客を待ちます。リクエストが来るたびに、その1行目を画面に記録し、「Hello from Rust!」という見出しの HTML を HTTP の決まった形で返します。止めるまでずっと待ち続けます。

```rust
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    // 自分の PC（127.0.0.1）の 8080 番ポートで待ち受けを始める
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("http://127.0.0.1:8080 で待っています（止めるには Ctrl+C）");

    // 接続が来るたびに、1 つずつ順番に処理する
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut reader = BufReader::new(&stream);

        // リクエストの 1 行目（例: GET / HTTP/1.1）を読む
        let mut first_line = String::new();
        reader.read_line(&mut first_line)?;
        println!("受け取った: {}", first_line.trim_end());

        // 残りのヘッダー行を、空行が来るまで読み飛ばす
        loop {
            let mut line = String::new();
            let n = reader.read_line(&mut line)?;
            if n == 0 || line == "\r\n" {
                break;
            }
        }

        // 返事（レスポンス）を HTTP の決まった形で組み立てて送る
        let body = "<h1>Hello from Rust!</h1>\n";
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(response.as_bytes())?;
    }
    Ok(())
}
```

PowerShell の窓を**2つ**開きます。1つ目の窓でサーバーを起動します。

```powershell
cargo run
```

サーバーは止まらずに待ち続けます。2つ目の窓から、curl でアクセスします。

```powershell
curl.exe -s -i http://127.0.0.1:8080/
curl.exe -s http://localhost:8080/hello
```

**▼ 実行結果**（2つ目の窓・curl 側）

```text
HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8
Content-Length: 26

<h1>Hello from Rust!</h1>
<h1>Hello from Rust!</h1>
```

**▼ 実行結果**（1つ目の窓・サーバー側の記録）

```text
http://127.0.0.1:8080 で待っています（止めるには Ctrl+C）
受け取った: GET / HTTP/1.1
受け取った: GET /hello HTTP/1.1
```

ブラウザで `http://127.0.0.1:8080/` を開いても、同じ見出しが表示されます。止めるときは、サーバーの窓で **Ctrl キーを押しながら C** を押します。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 部屋を借りて待ち始める（bind）

```rust
let listener = TcpListener::bind("127.0.0.1:8080")?;
```

- `TcpListener::bind` は、OS に「127.0.0.1 の 8080 番で待たせてください」と届け出ます。
- 届け出が通ると `listener`（待ち受け係）が手に入ります。他のプロセスがすでに 8080 番を使っていると、ここで失敗します（「よくあるエラー」で見ます）。

##### 解説2: 来客を1人ずつ迎える

```rust
for stream in listener.incoming() {
    let mut stream = stream?;
    // ...
}
```

- `listener.incoming()` は「来客（接続）の列」です。`for` で1人ずつ取り出し、来客がいないときは**次の来客まで待ちます**。だからこのプログラムは終わりません。
- `stream` は UNIT 05 の `TcpStream` と同じで、読めば相手からの文字、書けば相手への文字になります。今回はサーバー側なので、まず読んでから書きます。

##### 解説3: 注文書を読む

```rust
let mut first_line = String::new();
reader.read_line(&mut first_line)?;
// ...
loop {
    let mut line = String::new();
    let n = reader.read_line(&mut line)?;
    if n == 0 || line == "\r\n" {
        break;
    }
}
```

- `BufReader` は、届いた文字を1行ずつ読みやすくする道具です。
- 1行目（`GET / HTTP/1.1`）を読んで記録したあと、空行（`"\r\n"`）が来るまでヘッダーを読み飛ばします。
- ヘッダーを読まずに返事をして接続を閉じると、相手側で「接続が途中で切られた」というエラーになることがあるため、最後まで読んでいます。
- `n == 0` は「相手が何も送らずに接続を閉じた」場合です。

##### 解説4: 返事を書く

```rust
let response = format!(
    "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
    body.len(),
    body
);
stream.write_all(response.as_bytes())?;
```

- `format!` は、`{}` の場所に値を埋め込んだ文字列を作ります（`println!` の「表示せずに文字列を作る」版）。
- UNIT 05 の図5-3 と同じ「1行目・ヘッダー・空行・本文」の形です。
- `Content-Length` は本文の**バイト数**です。`body.len()` はバイト数を返すので、ここに使えます（文字数とバイト数の違いは UNIT 10 で見ます）。
- `write_all` で相手に送ります。`stream` の処理が終わると接続は自動で閉じられます。

### もう一歩: 実務に近い形

本物の Web アプリは、URL のパスによって返す内容を変えます。この振り分けを **ルーティング** と呼びます。

#### パスで返事を変える（ルーティング）

> **▼ このコードがやること（先に日本語で）:** リクエストの1行目からメソッドとパスを取り出し、`GET /` ならトップページ、`GET /health` なら `ok`、それ以外なら「404 ページが見つかりません」を返します。どのリクエストに何を返したかを、サーバーの画面に記録します。

```rust
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("http://127.0.0.1:8080 で待っています（止めるには Ctrl+C）");

    for stream in listener.incoming() {
        handle(stream?)?;
    }
    Ok(())
}

// 1 つの接続（1 回のリクエスト）を処理する
fn handle(mut stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(&stream);

    // 1 行目「GET /health HTTP/1.1」を、空白で 3 つに分ける
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");

    // 残りのヘッダー行は空行まで読み飛ばす
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        if n == 0 || line == "\r\n" {
            break;
        }
    }

    // メソッドとパスの組み合わせで、返す内容を決める（ルーティング）
    let (status, body) = match (method, path) {
        ("GET", "/") => ("200 OK", "<h1>トップページ</h1>\n"),
        ("GET", "/health") => ("200 OK", "ok\n"),
        _ => ("404 Not Found", "<h1>ページが見つかりません</h1>\n"),
    };
    println!("{method} {path} -> {status}");

    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(response.as_bytes())?;
    Ok(())
}
```

サーバーを起動したまま、別の窓から curl で3回アクセスし、最後にブラウザで `http://127.0.0.1:8080/` を開きます。

```powershell
curl.exe -s http://127.0.0.1:8080/health
curl.exe -s -i http://127.0.0.1:8080/notes
curl.exe -s -i -X POST http://127.0.0.1:8080/health
```

**▼ 実行結果**（curl 側）

```text
ok
HTTP/1.1 404 Not Found
Content-Type: text/html; charset=utf-8
Content-Length: 43

<h1>ページが見つかりません</h1>

HTTP/1.1 404 Not Found
Content-Type: text/html; charset=utf-8
Content-Length: 43

<h1>ページが見つかりません</h1>
```

**▼ 実行結果**（サーバー側の記録）

```text
http://127.0.0.1:8080 で待っています（止めるには Ctrl+C）
GET /health -> 200 OK
GET /notes -> 404 Not Found
POST /health -> 404 Not Found
GET / -> 200 OK
GET /favicon.ico -> 404 Not Found
  -> 404 Not Found
```

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="340" height="230" rx="10" fill="#ffffff" stroke="#94a3b8" stroke-width="1.5"/>
<rect x="10" y="10" width="340" height="36" rx="10" fill="#e2e8f0"/>
<rect x="10" y="36" width="340" height="10" fill="#e2e8f0"/>
<circle cx="30" cy="28" r="5" fill="#f87171"/>
<circle cx="46" cy="28" r="5" fill="#fbbf24"/>
<circle cx="62" cy="28" r="5" fill="#34d399"/>
<rect x="78" y="17" width="262" height="22" rx="11" fill="#ffffff" stroke="#cbd5e1"/>
<text x="90" y="33" font-size="12" fill="#334155">127.0.0.1:8080</text>
<text x="26" y="86" font-size="24" font-weight="700" fill="#0f172a">トップページ</text>
<text x="26" y="222" font-size="12" fill="#94a3b8">（CSS が無いので、飾りのない見た目）</text>
<rect x="370" y="10" width="340" height="230" rx="10" fill="#0c1222" stroke="#334155"/>
<text x="386" y="34" font-size="12" fill="#94a3b8">PowerShell（サーバー側）</text>
<g font-family="Cascadia Mono, Consolas, Yu Gothic UI, monospace" font-size="12" fill="#e2e8f0" xml:space="preserve" style="white-space: pre">
<text x="386" y="62">http://127.0.0.1:8080 で待っています</text>
<text x="386" y="84">GET /health -&gt; 200 OK</text>
<text x="386" y="106">GET /notes -&gt; 404 Not Found</text>
<text x="386" y="128">POST /health -&gt; 404 Not Found</text>
<text x="386" y="150" fill="#facc15">GET / -&gt; 200 OK</text>
<text x="386" y="172" fill="#facc15">GET /favicon.ico -&gt; 404 Not Found</text>
<text x="386" y="194" fill="#facc15">  -&gt; 404 Not Found</text>
</g>
<text x="386" y="226" font-size="12" fill="#facc15">黄色 = ブラウザで1回開いたときの記録</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図6-3: ブラウザで開いた画面（左）と、そのときのサーバーの記録（右）</div>
</div>

#### ▼ コードを1つずつ分解して解説

##### 解説1: 処理を関数に分ける

```rust
for stream in listener.incoming() {
    handle(stream?)?;
}
```

- 1回分のリクエストの処理を `handle` という関数に切り出しました。`main` は「待って、来たら `handle` に任せる」だけになり、読みやすくなります。
- `fn handle(mut stream: TcpStream) -> std::io::Result<()>` は、「`TcpStream` を1つ受け取り、成功か失敗を返す関数」という意味です。

##### 解説2: メソッドとパスを取り出す

```rust
let mut parts = first_line.split_whitespace();
let method = parts.next().unwrap_or("");
let path = parts.next().unwrap_or("");
```

- `split_whitespace()` は空白で区切って、部品を1つずつ取り出せるようにします。`GET /health HTTP/1.1` なら `GET`、`/health`、`HTTP/1.1` の順です。
- `.next()` で次の部品を1つ取り出します。無ければ `unwrap_or("")` で空文字にします。

##### 解説3: `match` で振り分ける

```rust
let (status, body) = match (method, path) {
    ("GET", "/") => ("200 OK", "<h1>トップページ</h1>\n"),
    ("GET", "/health") => ("200 OK", "ok\n"),
    _ => ("404 Not Found", "<h1>ページが見つかりません</h1>\n"),
};
```

- `(method, path)` の組を、上から順に比べます。`_` は「それ以外ぜんぶ」です。
- `POST /health` が 404 になったのは、`("GET", "/health")` とメソッドが違うからです。**同じパスでもメソッドが違えば別の注文**として扱われます。
- これが **ルーティング** の正体です。Topcoat などのフレームワークは、これをもっと便利で安全な形で用意してくれます。

##### 解説4: ブラウザは頼んでいないものも頼む

- ブラウザで1回開いただけなのに、記録が3行増えました。
- `GET /favicon.ico` は、ブラウザがタブに表示する小さなアイコン（**ファビコン**）を自動で取りに来たものです。
- 最後の `  -> 404 Not Found`（メソッドもパスも空）は、ブラウザが「次に使うかもしれない」と先回りして開いた予備の接続が、何も送らずに閉じられたものです（解説3 の `n == 0` の場合）。
- サーバーの記録には、こうした「人が頼んでいないリクエスト」も混ざります。記録を読むときに慌てないようにしましょう。

### よくあるエラーと読み方

#### ポートがすでに使われている

サーバーを動かしたまま、同じプログラムをもう1つ起動すると、次のエラーになります。

```text
Error: Os { code: 10048, kind: AddrInUse, message: "通常、各ソケット アドレスに対してプロトコル、ネットワーク アドレス、またはポートのどれか 1 つのみを使用できます。" }
```

**何が起きたか:** `bind` で 8080 番を借りようとしましたが、すでに別のプロセスが住んでいました。`AddrInUse`（アドレス使用中）、Windows のエラー番号は 10048 です。

**直し方:** 先に動いているサーバーを止めます（その窓で Ctrl+C）。どのプロセスが使っているか分からないときは、次のコマンドで調べられます。

```powershell
Get-NetTCPConnection -LocalPort 8080 -State Listen | Select-Object LocalAddress, LocalPort, State, OwningProcess
Get-Process -Id (Get-NetTCPConnection -LocalPort 8080 -State Listen).OwningProcess
netstat -ano | findstr :8080
```

**▼ 実行結果**

```text
LocalAddress LocalPort  State OwningProcess
------------ ---------  ----- -------------
127.0.0.1         8080 Listen         54588


Handles  NPM(K)    PM(K)      WS(K)     CPU(s)     Id  SI ProcessName
-------  ------    -----      -----     ------     --  -- -----------
     67       6      864       6396       0.00  54588   1 practice


  TCP    127.0.0.1:8080         0.0.0.0:0              LISTENING       54588
```

- 1つ目のコマンドは「8080 番で待ち受けている（`Listen`）接続」を探し、持ち主のプロセス番号（`OwningProcess`）を表示します。
- 2つ目は、その番号のプロセスの情報を表示します。`ProcessName` が `practice` なので、さっき起動した自分のサーバーだと分かります。
- 3つ目の `netstat` は昔からあるコマンドで、最後の数字が同じプロセス番号です。

自分のサーバーなら、`Stop-Process -Id 54588` で止めることもできます。

#### サーバーを動かしたまま `cargo build` した

```text
   Compiling practice v0.1.0 (C:\Users\you\rust-study\practice)
error: failed to remove file `C:\Users\you\rust-study\practice\target\debug\practice.exe`

Caused by:
  アクセスが拒否されました。 (os error 5)
```

**何が起きたか:** Cargo は新しい `practice.exe` を作る前に古いファイルを消そうとしますが、Windows では**実行中のプログラムのファイルは消せません**。そのため「アクセスが拒否されました（OS のエラー 5）」になりました。`Caused by:`（原因は）の下に、OS からの本当の理由が書かれています。

**直し方:** サーバーを Ctrl+C で止めてから、もう一度 `cargo build`（または `cargo run`）します。サーバーを作っているときは「直す → 止める → 起動し直す」の順番を習慣にしましょう。

### RustNote ではここで使う

- RustNote は Topcoat の `topcoat::start` でサーバーを起動します。待ち受ける場所は環境変数 `HOST` と `PORT` で決まり、何も設定しなければ **`127.0.0.1:3000`** です。開発中はブラウザで **`http://127.0.0.1:3000`** を開きます。この巻の練習サーバーが 8080 番なのは、RustNote とぶつからないようにするためです。
- Fly.io の上では、`HOST=0.0.0.0`・`PORT=8080` を設定して起動します。0.0.0.0 にしないと、Fly.io の受付からの取り次ぎを受けられません（第12巻）。
- RustNote にも、`ok` と返すだけの `GET /health` を作ります。Fly.io はこの URL に定期的にアクセスして、「サーバーが生きているか」を確かめます（**ヘルスチェック**、第9巻・第12巻）。

### 練習問題

**問題1:** 「もう一歩」のサーバーに、`GET /about` で「このサーバーについて」という見出しを返すルートを追加し、curl で確かめてください。

**問題2:** 次の説明のうち正しいものをすべて選んでください。

- a. `127.0.0.1:8080` で待っているサーバーには、同じ Wi-Fi の別の PC からアクセスできる
- b. `localhost` と `127.0.0.1` は、どちらも自分の PC を指す
- c. 1つのポートでは、1つのプロセスしか待ち受けられない
- d. ブラウザで `http://0.0.0.0:3000` を開くのが、Fly.io にアクセスする正しい方法である

**問題3:** `cargo run` したら `AddrInUse` のエラーが出ました。調べて解決する手順を書いてください。

### 解答と解説

**問題1の解答:** `match` に1行追加します（他の部分は「もう一歩」のコードと同じです）。

```rust
    let (status, body) = match (method, path) {
        ("GET", "/") => ("200 OK", "<h1>トップページ</h1>\n"),
        ("GET", "/health") => ("200 OK", "ok\n"),
        ("GET", "/about") => ("200 OK", "<h1>このサーバーについて</h1>\n"),
        _ => ("404 Not Found", "<h1>ページが見つかりません</h1>\n"),
    };
```

```powershell
curl.exe -s -i http://127.0.0.1:8080/about
```

**▼ 実行結果**

```text
HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8
Content-Length: 40

<h1>このサーバーについて</h1>
```

サーバー側には `GET /about -> 200 OK` と記録されます。追加する場所は `_ =>` の行より**上**です。`_` は「それ以外ぜんぶ」なので、その下に書いた行には決して届きません。

**問題2の解答:** 正しいのは **b と c** です。a は誤りで、127.0.0.1 は自分の PC の中だけです。d も誤りで、0.0.0.0 は待ち受け側の指定です。Fly.io のアプリには `https://アプリ名.fly.dev` の URL でアクセスします。

**問題3の解答:**

1. `Get-NetTCPConnection -LocalPort ポート番号 -State Listen` で、そのポートを使っているプロセスの番号（OwningProcess）を調べる
2. `Get-Process -Id 番号` で、それが何のプログラムかを確かめる
3. 自分が前に起動したサーバーなら、その窓で Ctrl+C するか `Stop-Process -Id 番号` で止める。別の大事なアプリなら、自分のサーバーのポート番号のほうを変える

### まとめ

- サーバーは、OS に届け出たポート（部屋番号）で来客を待ち続けるプロセスです。1つのポートで待てるのは1つのプロセスだけです。
- `127.0.0.1`（localhost）は自分の PC の中だけ、`0.0.0.0` で待つと外からも受け付けます。RustNote は手元で `127.0.0.1:3000`、Fly.io で `0.0.0.0:8080` です。
- パスとメソッドで処理を振り分けるのがルーティングです。ポートの使用中・実行ファイルのロックは「先に動いているものを止める」で解決します。

## UNIT 07: データベースと永続化

### この単元のゴール

- 変数の値はプロセスが終わると消え、ファイルやデータベースの中身は残ること（**永続化**）を体験する
- データベースの **テーブル（表）・行・列・主キー・制約** を説明できる
- 「ファイルに書くだけ」では足りなくなる理由と、データベースが引き受けてくれる仕事が分かる
- SQL の文と、制約違反のエラーを読める

### たとえ話でつかむ

データの置き場所を、**会社の事務所**にたとえます。

- **変数（メモリ）はホワイトボード**です。書くのも消すのも速いですが、退社するとき（プロセスの終了）に消されてしまい、停電（電源オフ）でも消えます。
- **ファイルはノートのメモ**です。閉じても残りますが、書き方は自分しだいです。何人もが同時に書き込むと字が重なり、目当ての1行を探すにも最初から読むしかありません。
- **データベース（DB）は台帳を管理する経理係**です。データを決まった形の**表**で整理し、「この列は必ず埋める」「メールアドレスは同じものを2つ登録しない」といった**ルール（制約）**を守らない記入は受け付けません。何人から同時に頼まれても順番に正しく処理し、「Hanako さんのノートだけ」のような探し物もすばやく見つけてくれます。

> **用語:** **永続化（えいぞくか）** … プログラムが終わっても、PC を再起動しても、データが残るように保存すること。
>
> **用語:** **データベース（DB）** … データを整理して保存し、探したり書き換えたりする仕事を引き受ける専用のプログラム（とそのデータ）。RustNote は **Turso（ターソ）** というサービスの **libSQL（リブエスキューエル）** というデータベースを使います。libSQL は、世界中で使われている **SQLite（エスキューライト）** の仲間です。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 270" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-17" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#047857"/></marker>
</defs>
<text x="20" y="24" font-size="13" font-weight="700" fill="#334155">プログラムを3回起動すると…</text>
<rect x="150" y="38" width="160" height="196" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="230" y="60" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">1回目</text>
<rect x="340" y="38" width="160" height="196" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="420" y="60" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">2回目</text>
<rect x="530" y="38" width="160" height="196" rx="10" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="610" y="60" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">3回目</text>
<text x="20" y="104" font-size="13" font-weight="700" fill="#1e40af">メモリ</text>
<text x="20" y="122" font-size="12" fill="#64748b">（変数）</text>
<rect x="170" y="80" width="120" height="50" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="230" y="111" font-size="16" font-weight="700" fill="#1e40af" text-anchor="middle">0 → 1</text>
<rect x="360" y="80" width="120" height="50" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="420" y="111" font-size="16" font-weight="700" fill="#1e40af" text-anchor="middle">0 → 1</text>
<rect x="550" y="80" width="120" height="50" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="610" y="111" font-size="16" font-weight="700" fill="#1e40af" text-anchor="middle">0 → 1</text>
<text x="20" y="184" font-size="13" font-weight="700" fill="#047857">ストレージ</text>
<text x="20" y="202" font-size="12" fill="#64748b">（count.txt）</text>
<rect x="170" y="160" width="120" height="50" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="230" y="191" font-size="16" font-weight="700" fill="#047857" text-anchor="middle">0 → 1</text>
<rect x="360" y="160" width="120" height="50" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="420" y="191" font-size="16" font-weight="700" fill="#047857" text-anchor="middle">1 → 2</text>
<rect x="550" y="160" width="120" height="50" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="610" y="191" font-size="16" font-weight="700" fill="#047857" text-anchor="middle">2 → 3</text>
<line x1="292" y1="185" x2="356" y2="185" stroke="#047857" stroke-width="2" marker-end="url(#ar-00-17)"/>
<line x1="482" y1="185" x2="546" y2="185" stroke="#047857" stroke-width="2" marker-end="url(#ar-00-17)"/>
<text x="20" y="140" font-size="12" fill="#b91c1c">毎回 0 から</text>
<text x="360" y="258" font-size="12" fill="#334155" text-anchor="middle">メモリの値はプロセスの終わりに消える　／　緑の矢印 = ファイルの値を次の起動が引き継ぐ</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図7-1: メモリの値はプロセスが終わると消え、ストレージに書いた値は次の起動に引き継がれる</div>
</div>

データベースは、データを **テーブル（表）** の形でしまいます。表計算ソフトのシートによく似ています。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="24" y="26" font-size="14" font-weight="700" fill="#047857">notes テーブル（一部の列だけ）</text>
<rect x="24" y="40" width="80" height="34" fill="#047857"/>
<rect x="104" y="40" width="90" height="34" fill="#047857"/>
<rect x="194" y="40" width="230" height="34" fill="#047857"/>
<rect x="424" y="40" width="70" height="34" fill="#047857"/>
<rect x="494" y="40" width="200" height="34" fill="#047857"/>
<text x="64" y="62" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">id</text>
<text x="149" y="62" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">user_id</text>
<text x="309" y="62" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">title</text>
<text x="459" y="62" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">done</text>
<text x="594" y="62" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">updated_at</text>
<rect x="24" y="74" width="670" height="34" fill="#ffffff" stroke="#cbd5e1"/>
<rect x="24" y="108" width="670" height="34" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<rect x="24" y="142" width="670" height="34" fill="#ffffff" stroke="#cbd5e1"/>
<rect x="194" y="74" width="230" height="102" fill="none" stroke="#3b82f6" stroke-width="3"/>
<text x="64" y="96" font-size="13" fill="#0f172a" text-anchor="middle">1</text>
<text x="149" y="96" font-size="13" fill="#0f172a" text-anchor="middle">1</text>
<text x="206" y="96" font-size="13" fill="#0f172a">牛乳を買う</text>
<text x="459" y="96" font-size="13" fill="#0f172a" text-anchor="middle">0</text>
<text x="594" y="96" font-size="13" fill="#0f172a" text-anchor="middle">2026-10-15 09:12</text>
<text x="64" y="130" font-size="13" fill="#0f172a" text-anchor="middle">2</text>
<text x="149" y="130" font-size="13" fill="#0f172a" text-anchor="middle">1</text>
<text x="206" y="130" font-size="13" fill="#0f172a">Rust の本を読む</text>
<text x="459" y="130" font-size="13" fill="#0f172a" text-anchor="middle">1</text>
<text x="594" y="130" font-size="13" fill="#0f172a" text-anchor="middle">2026-10-14 21:40</text>
<text x="64" y="164" font-size="13" fill="#0f172a" text-anchor="middle">3</text>
<text x="149" y="164" font-size="13" fill="#0f172a" text-anchor="middle">2</text>
<text x="206" y="164" font-size="13" fill="#0f172a">歯医者の予約</text>
<text x="459" y="164" font-size="13" fill="#0f172a" text-anchor="middle">0</text>
<text x="594" y="164" font-size="13" fill="#0f172a" text-anchor="middle">2026-10-13 18:05</text>
<rect x="24" y="196" width="210" height="70" rx="8" fill="#fff7ed" stroke="#f97316"/>
<text x="36" y="218" font-size="13" font-weight="700" fill="#c2410c">行（レコード）</text>
<text x="36" y="238" font-size="12" fill="#334155">ノート1件ぶんのデータ</text>
<text x="36" y="256" font-size="12" fill="#334155">（橙の枠がその1件）</text>
<rect x="250" y="196" width="210" height="70" rx="8" fill="#eff6ff" stroke="#3b82f6"/>
<text x="262" y="218" font-size="13" font-weight="700" fill="#1e40af">列（カラム）</text>
<text x="262" y="238" font-size="12" fill="#334155">項目。型とルールを持つ</text>
<text x="262" y="256" font-size="12" fill="#334155">（青の枠が title 列）</text>
<rect x="476" y="196" width="218" height="70" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="488" y="218" font-size="13" font-weight="700" fill="#047857">主キー（id）</text>
<text x="488" y="238" font-size="12" fill="#334155">1件を特定する番号</text>
<text x="488" y="256" font-size="12" fill="#334155">同じ値は2つとない</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図7-2: テーブルの行と列。done は 0 = 未済み、1 = 済み（値は例）</div>
</div>

テーブルどうしは、列の値でつながります。`notes` の `user_id` 列には「そのノートの持ち主の `users.id`」が入っています。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 220" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-18" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#c2410c"/></marker>
</defs>
<text x="24" y="24" font-size="14" font-weight="700" fill="#1e40af">users</text>
<rect x="24" y="34" width="60" height="30" fill="#1e40af"/>
<rect x="84" y="34" width="200" height="30" fill="#1e40af"/>
<text x="54" y="54" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">id</text>
<text x="184" y="54" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">email（UNIQUE）</text>
<rect x="24" y="64" width="260" height="30" fill="#ffffff" stroke="#cbd5e1"/>
<rect x="24" y="94" width="260" height="30" fill="#ffffff" stroke="#cbd5e1"/>
<text x="54" y="84" font-size="13" fill="#0f172a" text-anchor="middle">1</text>
<text x="96" y="84" font-size="13" fill="#0f172a">hanako@example.com</text>
<text x="54" y="114" font-size="13" fill="#0f172a" text-anchor="middle">2</text>
<text x="96" y="114" font-size="13" fill="#0f172a">taro@example.com</text>
<text x="404" y="24" font-size="14" font-weight="700" fill="#047857">notes</text>
<rect x="404" y="34" width="50" height="30" fill="#047857"/>
<rect x="454" y="34" width="80" height="30" fill="#047857"/>
<rect x="534" y="34" width="166" height="30" fill="#047857"/>
<text x="429" y="54" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">id</text>
<text x="494" y="54" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">user_id</text>
<text x="617" y="54" font-size="13" font-weight="700" fill="#ffffff" text-anchor="middle">title</text>
<rect x="404" y="64" width="296" height="30" fill="#ffffff" stroke="#cbd5e1"/>
<rect x="404" y="94" width="296" height="30" fill="#ffffff" stroke="#cbd5e1"/>
<rect x="404" y="124" width="296" height="30" fill="#ffffff" stroke="#cbd5e1"/>
<text x="429" y="84" font-size="13" fill="#0f172a" text-anchor="middle">1</text>
<text x="494" y="84" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">1</text>
<text x="546" y="84" font-size="13" fill="#0f172a">牛乳を買う</text>
<text x="429" y="114" font-size="13" fill="#0f172a" text-anchor="middle">2</text>
<text x="494" y="114" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">1</text>
<text x="546" y="114" font-size="13" fill="#0f172a">Rust の本を読む</text>
<text x="429" y="144" font-size="13" fill="#0f172a" text-anchor="middle">3</text>
<text x="494" y="144" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">2</text>
<text x="546" y="144" font-size="13" fill="#0f172a">歯医者の予約</text>
<path d="M400,79 C360,79 330,77 290,77" fill="none" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-18)"/>
<path d="M400,109 C350,109 340,83 290,83" fill="none" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-18)"/>
<path d="M400,139 C350,139 340,109 290,109" fill="none" stroke="#c2410c" stroke-width="2" marker-end="url(#ar-00-18)"/>
<text x="344" y="168" font-size="12" fill="#c2410c" text-anchor="middle">user_id の値で</text>
<text x="344" y="184" font-size="12" fill="#c2410c" text-anchor="middle">持ち主を指す</text>
<text x="24" y="148" font-size="12" fill="#334155">UNIQUE = 同じメールアドレスは</text>
<text x="24" y="164" font-size="12" fill="#334155">2回登録できない</text>
<text x="24" y="210" font-size="12" fill="#334155">Hanako（1番）のノートは 1・2、Taro（2番）のノートは 3</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図7-3: users と notes のつながり。user_id が「持ち主」を表す</div>
</div>

### まず動かす最小コード

#### 消えるカウンターと、消えないカウンター

> **▼ このコードがやること（先に日本語で）:** 2つのカウンター（数を数える仕組み）を同時に1増やします。1つはメモリの中だけ、もう1つは `count.txt` というファイルに保存します。プログラムを何回か起動して、どちらが前回の続きから数えるかを比べます。

```rust
use std::fs;

fn main() {
    // (1) メモリの中だけのカウンター。プログラムが始まるたびに 0 から
    let mut in_memory = 0;
    in_memory += 1;

    // (2) ファイルに保存するカウンター。前回の値を読み、1 足して書き戻す
    //     count.txt が無い（初回）ときは "0" として扱う
    let saved = fs::read_to_string("count.txt").unwrap_or(String::from("0"));
    let mut on_disk: u32 = saved.trim().parse().unwrap();
    on_disk += 1;
    fs::write("count.txt", on_disk.to_string()).unwrap();

    println!("メモリのカウンター  : {in_memory}");
    println!("ファイルのカウンター: {on_disk}");
}
```

```powershell
cargo run -q
cargo run -q
cargo run -q
cat count.txt
```

**▼ 実行結果**

```text
メモリのカウンター  : 1
ファイルのカウンター: 1
メモリのカウンター  : 1
ファイルのカウンター: 2
メモリのカウンター  : 1
ファイルのカウンター: 3
3
```

メモリのカウンターは毎回 1 に戻り、ファイルのカウンターは 1→2→3 と増えました。PC を再起動してからもう一度実行しても、ファイルのカウンターは 4 になります。

#### ▼ コードを1つずつ分解して解説

##### 解説1: メモリのカウンター

```rust
let mut in_memory = 0;
in_memory += 1;
```

- 変数 `in_memory` はメモリ上にあり、**プロセスが終わると OS に片付けられます**。次に起動したときは、また `0` から始まります。

##### 解説2: 前回の値を読む

```rust
let saved = fs::read_to_string("count.txt").unwrap_or(String::from("0"));
let mut on_disk: u32 = saved.trim().parse().unwrap();
```

- `count.txt` を読みます。初回はファイルが無くて失敗するので、`unwrap_or` で `"0"` を使います。
- ファイルの中身は**文字**の `"3"` なので、`.parse()` で**数値**の `3` に変換します。`.trim()` は前後の空白や改行を取り除きます。
- `: u32` は「0 以上の整数として扱う」という型の指定です。これがあるので、`parse` は何に変換すればよいか分かります。

##### 解説3: 新しい値を書き戻す

```rust
on_disk += 1;
fs::write("count.txt", on_disk.to_string()).unwrap();
```

- 1 増やした数を `.to_string()` で文字に戻して、ファイルに上書きします。
- 「読む → 変える → 書き戻す」は、データを保存するプログラムの基本の形です。データベースを使っても、この流れは変わりません。

### もう一歩: 実務に近い形

#### ファイルで「表」を作ってみる

ノートの一覧を、1行1件・タブ区切りのファイル（**TSV**）で保存してみます。データベースの表を、手作りで真似したものです。

> **▼ このコードがやること（先に日本語で）:** 起動時にタイトルが渡されたら、`notes.tsv` の最後に「id・持ち主の番号・タイトル・済みかどうか」を1行追加します。そのあと、ファイルの全行を表の形で表示します。

```rust
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let file = "notes.tsv";

    // 引数が渡されたら、それをタイトルとして 1 行（1 件）追加する
    if let Some(title) = env::args().nth(1) {
        // 今ある行数 + 1 を、新しい行の id にする
        let existing = fs::read_to_string(file).unwrap_or_default();
        let next_id = existing.lines().count() + 1;

        // ファイルの末尾に追記する（無ければ作る）
        let mut f = OpenOptions::new().create(true).append(true).open(file)?;
        // 列の順番: id, user_id, title, done（タブ文字 \t で区切る）
        writeln!(f, "{next_id}\t1\t{title}\tfalse")?;
    }

    // ファイルの全行を、表（テーブル）の形で表示する
    let text = fs::read_to_string(file).unwrap_or_default();
    println!("id | user_id | done  | title");
    for line in text.lines() {
        let cols: Vec<&str> = line.split('\t').collect();
        println!(
            "{:<2} | {:<7} | {:<5} | {}",
            cols[0], cols[1], cols[3], cols[2]
        );
    }
    Ok(())
}
```

```powershell
cargo run -q -- 牛乳を買う
cargo run -q -- "Rust の本を読む"
cat notes.tsv
```

**▼ 実行結果**

```text
id | user_id | done  | title
1  | 1       | false | 牛乳を買う
id | user_id | done  | title
1  | 1       | false | 牛乳を買う
2  | 1       | false | Rust の本を読む
1	1	牛乳を買う	false
2	1	Rust の本を読む	false
```

最後の2行が `notes.tsv` の中身そのものです。列の間の空白に見える部分は、タブ文字です。

#### ▼ コードを1つずつ分解して解説

##### 解説1: 追記モードで開く

```rust
let mut f = OpenOptions::new().create(true).append(true).open(file)?;
writeln!(f, "{next_id}\t1\t{title}\tfalse")?;
```

- `fs::write` は上書きでしたが、`OpenOptions` で `append(true)`（末尾に追記）を指定すると、前の行を残したまま書き足せます。`create(true)` は「無ければ作る」です。
- `writeln!` は、`println!` の「画面の代わりにファイルへ書く」版です。`\t` はタブ文字です。

##### 解説2: 1行を列に分ける

```rust
let cols: Vec<&str> = line.split('\t').collect();
```

- `split('\t')` でタブごとに区切り、`.collect()` で部品を並べた **`Vec`（ベクタ：伸び縮みする並び）** にまとめます。
- `cols[0]` が id、`cols[2]` がタイトルのように、番号で列を取り出せます。

##### 解説3: このやり方の限界

手作りの表は動きますが、本物のアプリにするには次の問題があります。

| 困りごと | 手作りファイルでは | データベースなら |
|---|---|---|
| 2人が同時に追加した | id が重なったり、行が混ざったりする | 順番に処理して、id も自動で重ならない |
| タイトルにタブが含まれていた | 列がずれて表が壊れる | 値の区切りを DB が管理するので壊れない |
| 「Hanako のノートだけ」を探す | 全行を読んで1行ずつ調べる | 条件を書くだけで、索引を使って速く探す |
| 空のタイトル、同じメールの二重登録 | 自分で全部チェックする | **制約**として書いておけば DB が断る |
| 途中で電源が切れた | 書きかけの半端な行が残る | 「全部やるか、何もしないか」を保証できる（**トランザクション**） |

#### 本物のデータベースなら: SQL の雰囲気

データベースには、**SQL（エスキューエル）** という言葉でお願いします。同じことを SQL で書くと次のようになります。ここは**見るだけ**で大丈夫です（第7巻で、Turso の道具を使って実際に動かします）。

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL UNIQUE
);
CREATE TABLE notes (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    title TEXT NOT NULL CHECK (length(title) BETWEEN 1 AND 80),
    done INTEGER NOT NULL DEFAULT 0
);
INSERT INTO users (email) VALUES ('hanako@example.com');
INSERT INTO users (email) VALUES ('taro@example.com');
INSERT INTO notes (user_id, title) VALUES (1, '牛乳を買う');
INSERT INTO notes (user_id, title) VALUES (1, 'Rust の本を読む');
INSERT INTO notes (user_id, title) VALUES (2, '歯医者の予約');
SELECT id, title, done FROM notes WHERE user_id = 1;
```

**▼ 実行結果**（SQLite 3.50 のコマンドラインツールで実行し、列を ` | ` で区切って表示したもの）

```text
id | title | done
1 | 牛乳を買う | 0
2 | Rust の本を読む | 0
```

- `CREATE TABLE` は表を作る文です。列の名前・型（`INTEGER` は整数、`TEXT` は文字）・ルールを書きます。
- `PRIMARY KEY` は主キー、`NOT NULL` は「必ず値を入れる」、`UNIQUE` は「同じ値を2つ入れない」、`CHECK (...)` は「この条件を満たす値だけ入れる」、`REFERENCES users(id)` は「`users` の `id` のどれかを指す」というルールです。
- `INSERT INTO ... VALUES ...` は行を追加する文、`SELECT ... FROM ... WHERE ...` は「条件に合う行を探して見せて」という文です。`WHERE user_id = 1` と書くだけで、Hanako のノートだけが返ってきます。

### よくあるエラーと読み方

#### 保存されたデータが壊れていた

最小コードの `count.txt` の中身を、手で `abc` に書き換えてから実行します。

```powershell
Set-Content count.txt "abc"
cargo run
```

**▼ 実行結果**

```text
thread 'main' (9276) panicked at src\main.rs:11:49:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\practice.exe` (exit code: 101)
```

**何が起きたか:** 11行目の `parse().unwrap()` で、`"abc"` を数値に変換できずにパニックしました（UNIT 04 と同じ `ParseIntError`）。保存されたデータは、**いつも自分の思ったとおりの形とは限らない**のです。

**直し方:** 今回は `count.txt` を消すか、数字に直せば動きます。本物のアプリでは、読み込んだデータが壊れていても止まらずにエラーとして扱う書き方をします。データベースを使うと、そもそも型や制約に合わないデータが入りにくくなります。

#### データベースが記入を断った（制約違反）

上の SQL の続きで、ルールに違反する記入を試します。

```sql
INSERT INTO users (email) VALUES ('hanako@example.com');
```

```text
Runtime error near line 1: UNIQUE constraint failed: users.email (19)
```

```sql
INSERT INTO notes (user_id, title) VALUES (1, '');
```

```text
Runtime error near line 1: CHECK constraint failed: length(title) BETWEEN 1 AND 80 (19)
```

**何が起きたか:**

- 1つ目は「`users.email` の **UNIQUE 制約**に失敗」。同じメールアドレスがすでに登録されているので断られました。
- 2つ目は「**CHECK 制約** `length(title) BETWEEN 1 AND 80` に失敗」。空のタイトル（0文字）は 1〜80 文字の範囲外です。
- 末尾の `(19)` は、SQLite の「制約違反」を表すエラー番号です。

**直し方:** これは「DB がちゃんと仕事をした」結果です。アプリ側では、この失敗を受け取って「そのメールアドレスはすでに登録されています」「タイトルは1〜80文字で入力してください」のような、利用者に分かる言葉に直して画面に表示します。

> **つまずきポイント:** 「アプリ側でチェックしているから、DB の制約はいらない」と考えがちですが、両方に書くのが基本です。アプリのチェックは**利用者に親切なメッセージを出すため**、DB の制約は**どんな経路からでも間違ったデータを入れさせない最後の砦**です。

### RustNote ではここで使う

- RustNote のデータは、次の4つのテーブルにしまいます（第7巻・第9巻で作ります）。

| テーブル | 中身 | 主な列 |
|---|---|---|
| `users` | 登録した人 | `id`、`email`（UNIQUE）、`password_hash`、`created_at` |
| `notes` | ノート | `id`、`user_id`（→ `users.id`）、`title`（1〜80文字）、`body`、`done`、`version`、`created_at`、`updated_at` |
| `sessions` | ログイン中の状態 | `token_hash`、`user_id`、`expires_at`、`created_at` |
| `schema_migrations` | 表の作り変えの記録 | `version`、`applied_at` |

- 開発中はデータベースを**自分の PC のファイル**として持ち、公開後はインターネット上の **Turso** を使います。プログラムからの使い方はほぼ同じです。
- テーブルの形を後から変えるときは、変更の手順を SQL のファイル（**マイグレーション**）として残し、`schema_migrations` に「どこまで適用したか」を記録します（第7巻）。

### 練習問題

**問題1:** 最小コードを改造して、`cargo run -q -- reset` と実行したときは `count.txt` を消して「カウンターを 0 に戻しました」と表示するようにしてください。

**問題2:** 「本物のデータベースなら」の SQL の表に対して、次の SQL を実行すると何が返ってくるでしょうか。

```sql
SELECT title FROM notes WHERE user_id = 2;
```

**問題3:** RustNote で次のデータを保存するとき、「メモリだけでよい」か「データベースに保存すべき」かを答えてください。

- a. ユーザーのメールアドレス
- b. 今まさに処理しているリクエストで送られてきたタイトルの文字列
- c. ノートの「済み（done）」の状態
- d. ログイン中であることの記録（セッション）

### 解答と解説

**問題1の解答:**

```rust
use std::env;
use std::fs;

fn main() {
    // 1 つ目の引数が "reset" なら、ファイルを消して 0 に戻す
    if env::args().nth(1).as_deref() == Some("reset") {
        // ファイルが無い場合のエラーは気にしないので、結果を捨てる
        let _ = fs::remove_file("count.txt");
        println!("カウンターを 0 に戻しました");
        return;
    }

    let saved = fs::read_to_string("count.txt").unwrap_or(String::from("0"));
    let mut on_disk: u32 = saved.trim().parse().unwrap();
    on_disk += 1;
    fs::write("count.txt", on_disk.to_string()).unwrap();
    println!("ファイルのカウンター: {on_disk}");
}
```

```powershell
cargo run -q
cargo run -q
cargo run -q -- reset
cargo run -q
```

**▼ 実行結果**

```text
ファイルのカウンター: 1
ファイルのカウンター: 2
カウンターを 0 に戻しました
ファイルのカウンター: 1
```

`.as_deref()` は、「`Option<String>`（あるかもしれない文字列）」を、`"reset"` と比べられる形（`Option<&str>`）に変える道具です。`return` で `main` を途中で終えています。`let _ = ...` は「結果を使わずに捨てる」という意味です。

**問題2の解答:** `user_id` が 2 の行は Taro の「歯医者の予約」だけなので、次の1行が返ってきます。

```text
歯医者の予約
```

**問題3の解答:** a・c・d は**データベース**です。サーバーが再起動しても残っていないと困るからです（d が消えると、全員がログアウトしてしまいます）。b は**メモリだけでよい**です。検査してデータベースに書き込んだら、その文字列はもう不要です。

### まとめ

- 変数（メモリ）の値はプロセスの終了で消えます。残したいデータはストレージに**永続化**します。
- データベースはデータを**テーブル（行と列）**で整理し、主キー・UNIQUE・CHECK などの**制約**で間違ったデータを断ってくれます。
- 手作りのファイルでは、同時書き込み・検索・壊れたデータに弱いので、Web アプリではデータベースを使います。

## UNIT 08: 認証と認可

### この単元のゴール

- **認証**（あなたは誰？）と**認可**（それをしてよい？）の違いを説明できる
- ログインしてからページを見るまでの流れ（パスワードの確認 → **セッション** → **Cookie**）を説明できる
- パスワードをそのまま保存してはいけない理由と、**ハッシュ**の考え方が分かる
- 「他人のデータが見えてしまう」不具合と、それを防ぐ書き方が分かる

### たとえ話でつかむ

ログインのしくみは、**ホテル**とそっくりです。

- **認証はフロントでの本人確認**です。名前と身分証で「あなたが予約した Hanako さんですね」と確かめます。Web アプリでは、メールアドレスとパスワードで確かめます。
- **セッションとカードキー**。本人確認が済むと、フロントは**カードキー**を渡し、台帳に「このカードキーは Hanako さんのもの、明日の 11 時まで有効」と記録します。それ以降、毎回身分証を見せる必要はありません。Web アプリでは、このカードキーにあたる**合言葉（トークン）**をブラウザの **Cookie（クッキー）** にしまい、ブラウザはリクエストのたびにそれを添えて送ります。
- **認可は「どの部屋を開けられるか」**です。カードキーはホテルの客だという証明にはなりますが、**開けられるのは自分の部屋だけ**です。Web アプリでも、ログインしている人が**他の人のノート**を見ようとしたら断らなければなりません。
- **ログアウトはチェックアウト**です。台帳からカードキーの記録を消すので、そのカードキーではもう扉が開きません。

> **用語:** **認証（にんしょう、Authentication）** … 相手が誰なのかを確かめること。ログインがその代表です。
>
> **用語:** **認可（にんか、Authorization）** … 確かめた相手に、その操作をしてよいかを判断すること。
>
> **用語:** **Cookie（クッキー）** … サーバーがブラウザに預けておく小さなメモ。ブラウザは、同じサイトへのリクエストのたびに自動でそれを送り返します。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 260" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-19" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="14" width="332" height="232" rx="12" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="180" y="42" font-size="16" font-weight="700" fill="#1e40af" text-anchor="middle">認証: あなたは誰？</text>
<rect x="34" y="62" width="130" height="64" rx="8" fill="#ffffff" stroke="#93c5fd"/>
<text x="99" y="88" font-size="12" fill="#334155" text-anchor="middle">メールアドレス</text>
<text x="99" y="108" font-size="12" fill="#334155" text-anchor="middle">＋ パスワード</text>
<line x1="166" y1="94" x2="194" y2="94" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-19)"/>
<rect x="198" y="62" width="130" height="64" rx="8" fill="#ffffff" stroke="#93c5fd"/>
<text x="263" y="88" font-size="12" fill="#334155" text-anchor="middle">本人なら</text>
<text x="263" y="108" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">「Hanako です」</text>
<text x="34" y="156" font-size="12" fill="#334155">・ログイン画面で1回だけ行う</text>
<text x="34" y="178" font-size="12" fill="#334155">・失敗 → ログインできない</text>
<text x="34" y="200" font-size="12" fill="#334155">・ホテルならフロントの本人確認</text>
<text x="34" y="222" font-size="12" fill="#334155">・HTTP では 401（ログインが必要）</text>
<rect x="374" y="14" width="332" height="232" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="540" y="42" font-size="16" font-weight="700" fill="#c2410c" text-anchor="middle">認可: それをしてよい？</text>
<rect x="394" y="62" width="130" height="64" rx="8" fill="#ffffff" stroke="#fdba74"/>
<text x="459" y="88" font-size="12" fill="#334155" text-anchor="middle">Hanako が</text>
<text x="459" y="108" font-size="12" fill="#334155" text-anchor="middle">ノート 11 を見たい</text>
<line x1="526" y1="94" x2="554" y2="94" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-19)"/>
<rect x="558" y="62" width="130" height="64" rx="8" fill="#ffffff" stroke="#fdba74"/>
<text x="623" y="88" font-size="12" fill="#334155" text-anchor="middle">持ち主は Taro</text>
<text x="623" y="108" font-size="12" font-weight="700" fill="#b91c1c" text-anchor="middle">→ 断る</text>
<text x="394" y="156" font-size="12" fill="#334155">・操作のたびに毎回行う</text>
<text x="394" y="178" font-size="12" fill="#334155">・ログイン済みでも断ることがある</text>
<text x="394" y="200" font-size="12" fill="#334155">・ホテルならカードキーで開く部屋</text>
<text x="394" y="222" font-size="12" fill="#334155">・HTTP では 403 か 404</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-1: 認証と認可は別の質問。ログインできても、他人のノートは見せない</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 400" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-20" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="10" width="140" height="36" rx="8" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="84" y="34" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">ブラウザ</text>
<rect x="290" y="10" width="140" height="36" rx="8" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="34" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">サーバー</text>
<rect x="566" y="10" width="140" height="36" rx="8" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="636" y="34" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">データベース</text>
<line x1="84" y1="48" x2="84" y2="390" stroke="#cbd5e1" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="360" y1="48" x2="360" y2="390" stroke="#cbd5e1" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="636" y1="48" x2="636" y2="390" stroke="#cbd5e1" stroke-width="2" stroke-dasharray="4 4"/>
<line x1="86" y1="76" x2="356" y2="76" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="220" y="70" font-size="12" fill="#1e40af" text-anchor="middle">① POST /login（メール・パスワード）</text>
<line x1="362" y1="102" x2="632" y2="102" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="498" y="96" font-size="12" fill="#047857" text-anchor="middle">② users からメールで探す</text>
<line x1="632" y1="124" x2="362" y2="124" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="498" y="140" font-size="12" fill="#047857" text-anchor="middle">password_hash を返す</text>
<rect x="250" y="150" width="220" height="30" rx="6" fill="#ffffff" stroke="#fdba74"/>
<text x="360" y="170" font-size="12" fill="#9a3412" text-anchor="middle">③ Argon2id で照合 → 本人と確認</text>
<line x1="362" y1="198" x2="632" y2="198" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="498" y="192" font-size="12" fill="#047857" text-anchor="middle">④ sessions に記録</text>
<line x1="356" y1="226" x2="86" y2="226" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="220" y="220" font-size="12" fill="#c2410c" text-anchor="middle">⑤ 303 → /notes ＋ Cookie を預ける</text>
<line x1="86" y1="270" x2="356" y2="270" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="220" y="264" font-size="12" fill="#1e40af" text-anchor="middle">⑥ GET /notes ＋ Cookie</text>
<line x1="362" y1="296" x2="632" y2="296" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="498" y="290" font-size="12" fill="#047857" text-anchor="middle">⑦ sessions で誰かを調べる</text>
<line x1="362" y1="324" x2="632" y2="324" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="498" y="318" font-size="12" fill="#047857" text-anchor="middle">⑧ その人のノートだけ探す</text>
<line x1="356" y1="364" x2="86" y2="364" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-20)"/>
<text x="220" y="358" font-size="12" fill="#c2410c" text-anchor="middle">⑨ 200 OK ＋ 一覧の HTML</text>
<text x="20" y="150" font-size="12" font-weight="700" fill="#1e40af">認証</text>
<text x="20" y="320" font-size="12" font-weight="700" fill="#c2410c">認可</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-2: ログインしてから一覧を見るまで。①〜⑤が認証、⑥〜⑨で毎回「誰か」を調べて認可する</div>
</div>

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 222" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-21" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
<marker id="ar-00-22" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#dc2626"/></marker>
</defs>
<rect x="14" y="40" width="170" height="64" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="99" y="66" font-size="12" fill="#334155" text-anchor="middle">パスワード</text>
<text x="99" y="88" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle" font-family="Cascadia Mono, Consolas, monospace">correct-horse</text>
<rect x="256" y="30" width="196" height="84" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="354" y="58" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">ハッシュ関数</text>
<text x="354" y="80" font-size="12" fill="#334155" text-anchor="middle">Argon2id（わざと遅い）</text>
<text x="354" y="100" font-size="12" fill="#334155" text-anchor="middle">＋ ソルト（人ごとの乱数）</text>
<rect x="524" y="40" width="182" height="64" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="615" y="66" font-size="12" fill="#334155" text-anchor="middle">DB に保存するもの</text>
<text x="615" y="88" font-size="13" font-weight="700" fill="#047857" text-anchor="middle" font-family="Cascadia Mono, Consolas, monospace">$argon2id$v=19$…</text>
<line x1="186" y1="72" x2="252" y2="72" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-21)"/>
<line x1="454" y1="72" x2="520" y2="72" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-21)"/>
<path d="M615,108 C615,170 99,170 99,110" fill="none" stroke="#dc2626" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#ar-00-22)"/>
<text x="360" y="184" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">× 逆向きには戻せない（一方向）</text>
<text x="360" y="208" font-size="12" fill="#334155" text-anchor="middle">ログインのときは、入力されたパスワードを同じ手順で変換して、保存された値と照らし合わせる</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図8-3: パスワードはハッシュにして保存する。DB が盗まれても、元のパスワードは分からない</div>
</div>

### まず動かす最小コード

#### ログインと「持ち主チェック」のミニ版

本物のログイン機能は第8巻で作ります。ここでは、認証と認可の**考え方だけ**を、メモリの中のデータで確かめます。

> **▼ このコードがやること（先に日本語で）:** 2人のユーザー（Hanako と Taro）と、それぞれのノートを1つずつ用意します。まず Hanako としてログインを試し（間違ったパスワードと正しいパスワード）、次にログインした Hanako が「自分のノート」と「Taro のノート」を見ようとしたときの結果を表示します。

```rust
// 登録した人（ユーザー）
struct User {
    id: u32,
    email: String,
    // 注意: 学習用にパスワードをそのまま持っている。本物はハッシュだけを保存する
    password: String,
}

// ノート（誰のものかを user_id で覚えている）
struct Note {
    id: u32,
    user_id: u32,
    title: String,
}

// 認証: メールアドレスとパスワードが合っていれば、その人の id を返す
fn login(users: &[User], email: &str, password: &str) -> Option<u32> {
    for user in users {
        if user.email == email && user.password == password {
            return Some(user.id);
        }
    }
    None
}

// 認可: ログイン中の人（current_user）が持ち主であるノートだけを返す
fn find_note(notes: &[Note], current_user: u32, note_id: u32) -> Option<&Note> {
    notes
        .iter()
        .find(|note| note.id == note_id && note.user_id == current_user)
}

fn main() {
    let users = vec![
        User {
            id: 1,
            email: String::from("hanako@example.com"),
            password: String::from("correct-horse"),
        },
        User {
            id: 2,
            email: String::from("taro@example.com"),
            password: String::from("battery-staple"),
        },
    ];
    let notes = vec![
        Note {
            id: 10,
            user_id: 1,
            title: String::from("牛乳を買う"),
        },
        Note {
            id: 11,
            user_id: 2,
            title: String::from("歯医者の予約"),
        },
    ];

    // 1. 認証（あなたは誰？）
    let wrong = login(&users, "hanako@example.com", "wrong-password");
    println!("[認証] 間違ったパスワード -> {wrong:?}");
    let hanako = login(&users, "hanako@example.com", "correct-horse");
    println!("[認証] 正しいパスワード   -> {hanako:?}");

    // 2. 認可（そのノートを見てよい？）
    let Some(current_user) = hanako else {
        println!("ログインできなかったので、ここで終わり");
        return;
    };
    for note_id in [10, 11] {
        match find_note(&notes, current_user, note_id) {
            Some(note) => println!("[認可] ノート {note_id} -> 200 OK「{}」", note.title),
            None => println!("[認可] ノート {note_id} -> 404 Not Found"),
        }
    }
}
```

**▼ 実行結果**

```text
[認証] 間違ったパスワード -> None
[認証] 正しいパスワード   -> Some(1)
[認可] ノート 10 -> 200 OK「牛乳を買う」
[認可] ノート 11 -> 404 Not Found
```

ログインに成功した Hanako でも、Taro のノート 11 は「見つからない（404）」になりました。

#### ▼ コードを1つずつ分解して解説

##### 解説1: データの形を決める `struct`

```rust
struct Note {
    id: u32,
    user_id: u32,
    title: String,
}
```

- `struct`（ストラクト、**構造体**）は、関係のある値をひとまとめにした「自分だけの型」を作る書き方です。データベースのテーブルの1行と同じ形にしています。
- `String` は、あとから長さが変わってもよい文字列の型です（第2巻）。
- `user_id` が「このノートの持ち主」です。認可のかなめになる列です。

##### 解説2: 認証する関数 `login`

```rust
fn login(users: &[User], email: &str, password: &str) -> Option<u32> {
    for user in users {
        if user.email == email && user.password == password {
            return Some(user.id);
        }
    }
    None
}
```

- `users: &[User]` は「ユーザーの並びを借りて受け取る」、`-> Option<u32>` は「見つかれば `Some(id)`、見つからなければ `None` を返す」という意味です。
- `&&` は「かつ」です。メールアドレスとパスワードの**両方**が合ったときだけ、その人の `id` を返します。
- 最後の `None` は「全員調べても合わなかった」ときの返り値です。Rust では、最後に書いた値（`;` を付けない）がそのまま関数の返り値になります。

> **注意:** この例はパスワードをそのまま比べていますが、**本物のアプリでは絶対にこう書きません**。パスワードをそのまま保存すると、データベースが盗まれたとき、全員のパスワードがそのまま漏れてしまいます。正しいやり方は「もう一歩」と第8巻で扱います。

##### 解説3: 認可する関数 `find_note`

```rust
fn find_note(notes: &[Note], current_user: u32, note_id: u32) -> Option<&Note> {
    notes
        .iter()
        .find(|note| note.id == note_id && note.user_id == current_user)
}
```

- `.iter().find(...)` は、並びの中から条件に合う最初の1つを探します。見つからなければ `None` です。
- `|note| ...` は **クロージャ**（その場で作る小さな関数）で、「1件ずつ `note` として受け取り、条件を満たすか」を書いています。
- 条件は「番号が一致する **かつ** 持ち主がログイン中の人」です。他人のノートは「存在しない」のと同じ扱いになります。これが認可です。

##### 解説4: ログインできなかったら先へ進まない

```rust
let Some(current_user) = hanako else {
    println!("ログインできなかったので、ここで終わり");
    return;
};
```

- `let Some(x) = 値 else { ... };` は、「値が `Some` なら中身を `x` に取り出して先へ進む。`None` なら `else` の中を実行する」という書き方です。
- `else` の中は必ず `return` などで抜けなければなりません。「ログインしていないのに、ノートを探す処理へ進んでしまう」ことが、文法の上で起こりえない形になっています。

### もう一歩: 実務に近い形

#### ハッシュの性質を確かめる（しくみを見るためのおもちゃ）

パスワードの代わりに保存する**ハッシュ**の性質を、Rust の標準ライブラリにある `DefaultHasher` で確かめます。

> **▼ このコードがやること（先に日本語で）:** いくつかの文字列から、それぞれの「指紋」のような数値（ハッシュ値）を作って表示します。同じ文字列なら同じ値になること、1文字違うだけでまったく違う値になることを確かめます。

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// 文字列から「指紋」のような数値を作る（しくみを見るためのおもちゃ）
fn toy_hash(text: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    let inputs = ["correct-horse", "correct-horse", "correct-horsf", "a"];
    for input in inputs {
        // {:016x} は「16 けたの 16 進数で表示する」という指定
        println!("{input:<14} -> {:016x}", toy_hash(input));
    }
}
```

**▼ 実行結果**

```text
correct-horse  -> f96723216a3e141a
correct-horse  -> f96723216a3e141a
correct-horsf  -> db8dd628cfe17851
a              -> 719b50b9a4f0e9f3
```

（数値そのものは、Rust の版によって変わることがあります。）

#### ▼ コードを1つずつ分解して解説

##### 解説1: ハッシュ値の作り方

```rust
let mut hasher = DefaultHasher::new();
text.hash(&mut hasher);
hasher.finish()
```

- `DefaultHasher` は、データから決まった長さの数値（ハッシュ値）を作る道具です。
- `text.hash(&mut hasher)` で文字列を流し込み、`finish()` で結果の数値を取り出します。

##### 解説2: 結果から分かる性質

- **同じ入力からは、いつも同じ値**ができます（1行目と2行目）。だから、ログインのときに入力されたパスワードを同じ手順で変換すれば、保存しておいた値と照らし合わせられます。
- **1文字違うだけで、まったく違う値**になります（`horse` と `horsf`）。似ているかどうかも分かりません。
- 入力が1文字でも長い文でも、**結果は同じ長さ**です。結果から元の文字列を計算で戻すことはできません（**一方向**）。

##### 解説3: なぜ本物には Argon2id を使うのか

> **注意:** `DefaultHasher` は、データを素早く仕分けるための道具で、**パスワードの保存には絶対に使ってはいけません**。速すぎるので、盗んだ人が「よくあるパスワード」を1秒に何億通りも試して、元のパスワードを当ててしまえるからです。

パスワード専用のハッシュ関数 **Argon2id（アルゴンツー・アイディー）** は、次の工夫で当てられにくくしてあります。

- **わざと遅く、わざとメモリをたくさん使う**ので、総当たりで試すのにとても時間がかかる
- 人ごとに違う乱数（**ソルト**）を混ぜるので、同じパスワードの人がいても保存される値は別々になる

RustNote は `users.password_hash` 列に Argon2id の結果だけを保存し、元のパスワードはどこにも保存しません（第8巻）。

### よくあるエラーと読み方

#### 持ち主チェックを書き忘れた（コンパイラの警告がヒント）

最小コードの `find_note` で、持ち主の条件を消してしまったとします（抜粋。他の部分は同じです）。

```rust
// 認可: ログイン中の人（current_user）が持ち主であるノートだけを返す
fn find_note(notes: &[Note], current_user: u32, note_id: u32) -> Option<&Note> {
    notes.iter().find(|note| note.id == note_id)
}
```

**▼ 実行結果**（コンパイル時の警告と、実行結果）

```text
   Compiling practice v0.1.0 (C:\Users\you\rust-study\practice)
warning: unused variable: `current_user`
  --> src\main.rs:27:30
   |
27 | fn find_note(notes: &[Note], current_user: u32, note_id: u32) -> Option<&Note> {
   |                              ^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_current_user`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: field `user_id` is never read
  --> src\main.rs:12:5
   |
10 | struct Note {
   |        ---- field in this struct
11 |     id: u32,
12 |     user_id: u32,
   |     ^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `practice` (bin "practice") generated 2 warnings (run `cargo fix --bin "practice" -p practice` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target\debug\practice.exe`
[認証] 間違ったパスワード -> None
[認証] 正しいパスワード   -> Some(1)
[認可] ノート 10 -> 200 OK「牛乳を買う」
[認可] ノート 11 -> 200 OK「歯医者の予約」
```

**何が起きたか:**

- 実行結果の最後の行で、Hanako が **Taro のノート「歯医者の予約」を見られてしまいました**。URL の番号を書き換えるだけで他人のデータが見えてしまうこの種の不具合は **IDOR（アイドア）** と呼ばれ、実際の Web サービスでもよく起きる、深刻な問題です。
- コンパイルは成功しましたが、コンパイラは **warning（警告）** を2つ出しています。
  - `unused variable: current_user` …「引数 `current_user` を受け取ったのに、一度も使っていない」
  - `field user_id is never read` …「`Note` の `user_id` を、どこからも読んでいない」
- どちらも「持ち主を調べる処理が無い」ことを、遠回しに教えてくれています。

**直し方:** 条件に `&& note.user_id == current_user` を戻します。警告は「エラーではないから無視してよいもの」ではなく、**バグの手がかり**です。警告が出たら、必ず理由を確かめましょう。

> **ヒント:** `help: if this is intentional, prefix it with an underscore` は「わざと使わないのなら、名前の先頭に `_` を付けてください（そうすれば警告を消せます）」という助言です。今回は**わざとではない**ので、`_` を付けて黙らせるのは間違った直し方です。

#### 401・403・404 の使い分け

| 状況 | 返すもの | 理由 |
|---|---|---|
| ログインしていない人が `/notes` を開いた | ログイン画面へ移動させる（または 401） | まず認証が必要 |
| ログインしているが、権限のない操作をした | 403 Forbidden | 認可で断った |
| ログインしているが、**他人のノート**を開こうとした | **404 Not Found** | 「そのノートは存在する」ことすら教えないため |

他人のノートに 403 を返すと、「その番号のノートは存在する（が、あなたのではない）」と相手に分かってしまいます。404 なら、存在しないノートと区別がつきません。RustNote はこの理由で 404 を返します。

### RustNote ではここで使う

- 登録・ログイン・ログアウトは `GET|POST /signup`・`GET|POST /login`・`POST /logout` です。パスワードは **Argon2id** のハッシュにして `users.password_hash` に保存します。
- ログインに成功すると、推測できない長い乱数（**セッショントークン**）を作り、ブラウザには Cookie として渡し、データベースの `sessions` テーブルには**トークンのハッシュ**（`token_hash`）と有効期限を保存します。トークンそのものを保存しないのは、パスワードと同じく、DB が漏れても悪用されないようにするためです。
- Topcoat のセッション機能が使う Cookie には、`Secure`（https のときだけ送る）・`HttpOnly`（ページ内のプログラムから読めない）・`SameSite=Lax`（他のサイトからの送信に勝手に付かない）といった安全のための印が付きます（第8巻）。
- ノートを探す SQL には、必ず `WHERE user_id = ?`（ログイン中の人）を付けます。他人のノートは 404 です。

### 練習問題

**問題1:** 最小コードの `main` を書き換えて、Taro（`taro@example.com`・`battery-staple`）でログインし、ノート 10 と 11 を見ようとした結果を表示してください。

**問題2:** 次の a〜d は、認証と認可のどちらでしょうか。

- a. ログイン画面で、入力されたパスワードが正しいか確かめる
- b. 編集ボタンが押されたとき、そのノートの持ち主がログイン中の人か確かめる
- c. Cookie のセッショントークンから、ログイン中の人が誰かを調べる
- d. 管理者だけが、全ユーザーの一覧ページを開ける

**問題3:** パスワードを「そのまま」ではなく「ハッシュにして」保存する理由を、1〜2文で説明してください。

### 解答と解説

**問題1の解答:** `main` 関数の「1. 認証」から後ろを、次のように置き換えます（`struct` と関数、`users`・`notes` の用意は同じです）。

```rust
    // Taro（taro@example.com）でログインする
    let taro = login(&users, "taro@example.com", "battery-staple");
    println!("[認証] Taro -> {taro:?}");

    let Some(current_user) = taro else {
        println!("ログインできなかったので、ここで終わり");
        return;
    };
    for note_id in [10, 11] {
        match find_note(&notes, current_user, note_id) {
            Some(note) => println!("[認可] ノート {note_id} -> 200 OK「{}」", note.title),
            None => println!("[認可] ノート {note_id} -> 404 Not Found"),
        }
    }
}
```

**▼ 実行結果**

```text
[認証] Taro -> Some(2)
[認可] ノート 10 -> 404 Not Found
[認可] ノート 11 -> 200 OK「歯医者の予約」
```

Hanako のときとちょうど逆になりました。同じ `find_note` でも、ログイン中の人によって結果が変わるのが認可です。

**問題2の解答:** a は**認証**、b は**認可**、c は**認証**（セッションで「誰か」を確かめている）、d は**認可**（誰かは分かったうえで、してよいかを判断している）。

**問題3の解答:** データベースが盗まれたり、内部の人がのぞいたりしても、元のパスワードが分からないようにするためです。多くの人は同じパスワードを他のサービスでも使っているので、漏れると被害がそのサービスだけでは済みません。

### まとめ

- **認証**は「あなたは誰？」（ログイン）、**認可**は「それをしてよい？」（持ち主チェック）で、別々に毎回考えます。
- ログイン後はセッショントークンを Cookie で受け渡し、サーバーは毎回それで「誰か」を調べます。パスワードもトークンも、DB には**ハッシュ**で保存します。
- 他人のデータには 404 を返します。コンパイラの「使っていない」という警告は、持ち主チェック漏れの手がかりになります。

## UNIT 09: コンパイラエラーとの付き合い方

### この単元のゴール

- Rust のエラーメッセージを、**エラー番号・場所・印（`^^^`）・`note:`・`help:`** の部品に分けて読める
- エラーが複数出たときに、**上から1つずつ**直す理由が分かる
- `rustc --explain` で、エラー番号の詳しい説明を読める
- **エラー（error）**と**警告（warning）**の違いが分かり、`cargo clippy` の助言を活かせる

### たとえ話でつかむ

Rust のコンパイラのエラーメッセージは、**とても親切な先生の添削（赤ペン）**です。

- 「×」だけ付けて返す先生ではありません。**どのページの何行目の何文字目か**、**何が問題か**、**なぜそう判断したか**、そして多くの場合**どう直せばよいか**まで書いてくれます。
- 添削が10か所あっても、最初の1つの書き間違いが原因で、後ろがまとめて×になっていることがあります。**最初の赤ペンから順に**直すのが近道です。
- 先生の説明が短すぎて分からないときは、**教科書の該当ページ**（`rustc --explain`）を開けば、例つきの長い説明が読めます。

最初はエラーが出るたびに「怒られた」と感じるかもしれません。でも Rust のコンパイラは、**動かしてから壊れるはずだった問題を、動かす前に見つけてくれる**相棒です。エラーをたくさん見た人ほど、Rust が上手になります。

> **用語:** **エラー（error）** … これを直さないとコンパイルが終わらない（実行ファイルが作られない）問題。
>
> **用語:** **警告（warning）** … コンパイルは終わるが、「たぶん間違いでは？」「もっと良い書き方がある」という指摘。UNIT 08 で見たとおり、バグの手がかりになることがあります。

### 図で理解する

次のエラーメッセージは「もう一歩」で実際に出すものです。部品ごとに意味があります。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 420" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="10" y="10" width="420" height="400" rx="10" fill="#0c1222"/>
<g font-family="Cascadia Mono, Consolas, monospace" font-size="12.5" fill="#e2e8f0" xml:space="preserve" style="white-space: pre">
<text x="22" y="36"><tspan fill="#f87171" font-weight="700">error[E0308]</tspan>: mismatched types</text>
<text x="22" y="58"><tspan fill="#60a5fa"> --&gt;</tspan> src\main.rs:8:11</text>
<text x="22" y="80" fill="#60a5fa">  |</text>
<text x="22" y="102"><tspan fill="#60a5fa">8 |</tspan>     greet(name);</text>
<text x="22" y="124"><tspan fill="#60a5fa">  |</tspan>     <tspan fill="#60a5fa">-----</tspan> <tspan fill="#f87171">^^^^ expected `&amp;str`,</tspan></text>
<text x="22" y="146"><tspan fill="#60a5fa">  |</tspan>           <tspan fill="#f87171">found `String`</tspan></text>
<text x="22" y="168"><tspan fill="#60a5fa">  |     |</tspan></text>
<text x="22" y="190"><tspan fill="#60a5fa">  |     arguments to this function</tspan></text>
<text x="22" y="212"><tspan fill="#60a5fa">  |     are incorrect</tspan></text>
<text x="22" y="234"><tspan fill="#4ade80" font-weight="700">note</tspan>: function defined here</text>
<text x="22" y="256"><tspan fill="#60a5fa"> --&gt;</tspan> src\main.rs:2:4</text>
<text x="22" y="278"><tspan fill="#60a5fa">2 |</tspan> fn greet(name: &amp;str) {</text>
<text x="22" y="300"><tspan fill="#60a5fa">  |</tspan>    <tspan fill="#4ade80">^^^^^</tspan> <tspan fill="#60a5fa">----------</tspan></text>
<text x="22" y="322"><tspan fill="#38bdf8" font-weight="700">help</tspan>: consider borrowing here</text>
<text x="22" y="344"><tspan fill="#60a5fa">8 |</tspan>     greet(<tspan fill="#4ade80">&amp;</tspan>name);</text>
<text x="22" y="366"><tspan fill="#60a5fa">  |</tspan>           <tspan fill="#4ade80">+</tspan></text>
<text x="22" y="396">For more information ... --explain E0308</text>
</g>
<circle cx="420" cy="32" r="3" fill="#dc2626"/>
<line x1="420" y1="32" x2="452" y2="32" stroke="#dc2626" stroke-width="1.5"/>
<circle cx="420" cy="54" r="3" fill="#3b82f6"/>
<line x1="420" y1="54" x2="452" y2="80" stroke="#3b82f6" stroke-width="1.5"/>
<circle cx="420" cy="120" r="3" fill="#dc2626"/>
<line x1="420" y1="120" x2="452" y2="134" stroke="#dc2626" stroke-width="1.5"/>
<circle cx="420" cy="186" r="3" fill="#3b82f6"/>
<line x1="420" y1="186" x2="452" y2="188" stroke="#3b82f6" stroke-width="1.5"/>
<circle cx="420" cy="230" r="3" fill="#10b981"/>
<line x1="420" y1="230" x2="452" y2="246" stroke="#10b981" stroke-width="1.5"/>
<circle cx="420" cy="318" r="3" fill="#0284c7"/>
<line x1="420" y1="318" x2="452" y2="306" stroke="#0284c7" stroke-width="1.5"/>
<circle cx="420" cy="392" r="3" fill="#475569"/>
<line x1="420" y1="392" x2="452" y2="372" stroke="#475569" stroke-width="1.5"/>
<rect x="454" y="14" width="256" height="40" rx="6" fill="#fef2f2" stroke="#dc2626"/>
<text x="464" y="31" font-size="12" font-weight="700" fill="#b91c1c">① エラー番号と要約</text>
<text x="464" y="47" font-size="12" fill="#334155">E0308 = 型が合わない</text>
<rect x="454" y="62" width="256" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="464" y="79" font-size="12" font-weight="700" fill="#1e40af">② 場所（ファイル:行:列）</text>
<text x="464" y="95" font-size="12" fill="#334155">main.rs の 8 行目 11 文字目</text>
<rect x="454" y="114" width="256" height="40" rx="6" fill="#fef2f2" stroke="#dc2626"/>
<text x="464" y="131" font-size="12" font-weight="700" fill="#b91c1c">③ ^^^ = 問題の箇所と理由</text>
<text x="464" y="147" font-size="12" fill="#334155">&amp;str が欲しいのに String</text>
<rect x="454" y="168" width="256" height="40" rx="6" fill="#eff6ff" stroke="#3b82f6"/>
<text x="464" y="185" font-size="12" font-weight="700" fill="#1e40af">④ --- = 関係する箇所</text>
<text x="464" y="201" font-size="12" fill="#334155">この関数への引数が原因</text>
<rect x="454" y="226" width="256" height="40" rx="6" fill="#ecfdf5" stroke="#10b981"/>
<text x="464" y="243" font-size="12" font-weight="700" fill="#047857">⑤ note: 補足情報</text>
<text x="464" y="259" font-size="12" fill="#334155">関数はここで定義されている</text>
<rect x="454" y="286" width="256" height="40" rx="6" fill="#f0f9ff" stroke="#0284c7"/>
<text x="464" y="303" font-size="12" font-weight="700" fill="#0369a1">⑥ help: 直し方の提案</text>
<text x="464" y="319" font-size="12" fill="#334155">+ の位置に &amp; を足すとよい</text>
<rect x="454" y="352" width="256" height="40" rx="6" fill="#f8fafc" stroke="#475569"/>
<text x="464" y="369" font-size="12" font-weight="700" fill="#334155">⑦ もっと詳しい説明の読み方</text>
<text x="464" y="385" font-size="12" fill="#334155">rustc --explain E0308</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図9-1: エラーメッセージの部品（紙面に合わせて長い行は折り返している。右端の点がその行）。①→②→③の順に読み、⑥の提案を検討する</div>
</div>

エラーが出たときの進め方は、次の流れです。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-23" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="20" width="124" height="70" rx="10" fill="#fef2f2" stroke="#dc2626" stroke-width="2"/>
<text x="76" y="48" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">① いちばん上の</text>
<text x="76" y="68" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">エラーだけ読む</text>
<rect x="156" y="20" width="124" height="70" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="218" y="48" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">② --&gt; の場所を</text>
<text x="218" y="68" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">エディタで開く</text>
<rect x="298" y="20" width="124" height="70" rx="10" fill="#f0f9ff" stroke="#0284c7" stroke-width="2"/>
<text x="360" y="48" font-size="13" font-weight="700" fill="#0369a1" text-anchor="middle">③ help: を読み</text>
<text x="360" y="68" font-size="13" font-weight="700" fill="#0369a1" text-anchor="middle">意味を考える</text>
<rect x="440" y="20" width="124" height="70" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="502" y="48" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">④ 1か所だけ</text>
<text x="502" y="68" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">直す</text>
<rect x="582" y="20" width="124" height="70" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="644" y="48" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">⑤ cargo check</text>
<text x="644" y="68" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">でもう一度</text>
<line x1="140" y1="55" x2="152" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-23)"/>
<line x1="282" y1="55" x2="294" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-23)"/>
<line x1="424" y1="55" x2="436" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-23)"/>
<line x1="566" y1="55" x2="578" y2="55" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-23)"/>
<path d="M644,92 L644,120 L76,120 L76,94" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-23)"/>
<text x="360" y="114" font-size="12" fill="#334155" text-anchor="middle">まだエラーが残っていれば、また①へ（消えたエラーは同じ原因だった）</text>
<rect x="156" y="150" width="408" height="84" rx="10" fill="#f8fafc" stroke="#475569" stroke-dasharray="6 4"/>
<text x="360" y="174" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">③で意味が分からないとき</text>
<text x="360" y="198" font-size="12" fill="#334155" text-anchor="middle">rustc --explain エラー番号 → 例つきの説明を読む</text>
<text x="360" y="220" font-size="12" fill="#334155" text-anchor="middle">それでも分からなければ、エラー全文で検索する・AI に聞く</text>
<line x1="360" y1="92" x2="360" y2="100" stroke="#475569" stroke-width="2"/>
<line x1="330" y1="128" x2="330" y2="146" stroke="#94a3b8" stroke-width="2" stroke-dasharray="4 3" marker-end="url(#ar-00-23)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図9-2: エラーとの付き合い方。上から1つずつ、1か所直したらすぐ確かめる</div>
</div>

### まず動かす最小コード

この単元の最小コードは、**わざと間違えたコード**です。1つずつ `src\main.rs` に貼り付けて `cargo run` し、エラーを自分の目で見てから直してください。

#### 例1: 型が合わない（E0308）

> **▼ このコードがやること（先に日本語で）:** 年齢を数値（`u32`）の変数に入れたいのに、うっかり `"20"` と引用符を付けて**文字**として書いてしまったコードです。

```rust
fn main() {
    // 年齢を数値（u32）として持ちたい
    let age: u32 = "20";
    println!("年齢: {age}");
}
```

**▼ 実行結果**

```text
   Compiling practice v0.1.0 (C:\Users\you\rust-study\practice)
error[E0308]: mismatched types
 --> src\main.rs:3:20
  |
3 |     let age: u32 = "20";
  |              ---   ^^^^ expected `u32`, found `&str`
  |              |
  |              expected due to this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `practice` (bin "practice") due to 1 previous error
```

#### ▼ 出力を1つずつ分解して解説

##### 解説1: 1行目は「番号」と「要約」

```text
error[E0308]: mismatched types
```

- `error` … 直さないとコンパイルが終わらない問題です。
- `[E0308]` … **エラー番号**です。同じ番号のエラーは同じ種類の問題なので、番号で検索すると説明が見つかります。
- `mismatched types` … 要約で、「型が食い違っている」という意味です。

##### 解説2: `-->` は場所

```text
 --> src\main.rs:3:20
```

- `src\main.rs` ファイルの **3行目・20文字目**です。VS Code では、`Ctrl + G` で行番号を指定して移動できます。ターミナルのこの部分を Ctrl を押しながらクリックすると、その場所が開くこともあります。

##### 解説3: `^^^` と `---` の印

```text
3 |     let age: u32 = "20";
  |              ---   ^^^^ expected `u32`, found `&str`
  |              |
  |              expected due to this
```

- `^^^^` は**問題の箇所**です。`"20"` の下に付いていて、「`u32` を期待していたのに、`&str`（文字列）が見つかった」と書かれています。
- `---` は**関係する箇所**です。`u32` の下に付いていて、「このせいで `u32` を期待した（expected due to this）」と、判断の根拠を示しています。
- 直し方は、引用符を外して `let age: u32 = 20;` とすることです。

##### 解説4: 最後の2行

```text
For more information about this error, try `rustc --explain E0308`.
error: could not compile `practice` (bin "practice") due to 1 previous error
```

- 1行目は「詳しい説明は `rustc --explain E0308` で読めます」という案内です（「もう一歩」で使います）。
- 2行目は Cargo のまとめで、「前に出たエラー1個のせいで、`practice` をコンパイルできなかった」という意味です。エラーの中身はこの行ではなく、**上のほう**にあります。

#### 例2: 名前の打ち間違い（E0425）

ここから先のエラーの出力では、最初の `Compiling ...` の行を省きます。例2・例3では、最後の2行（例1の解説4と同じ形の案内とまとめ）も省いています。

```rust
fn main() {
    let message = "こんにちは";
    println!("{}", mesage);
}
```

**▼ 実行結果**

```text
error[E0425]: cannot find value `mesage` in this scope
 --> src\main.rs:3:20
  |
3 |     println!("{}", mesage);
  |                    ^^^^^^
  |
help: a local variable with a similar name exists
  |
3 |     println!("{}", message);
  |                       +
```

- 「この範囲（scope）に `mesage` という値は見つからない」というエラーです。
- `help:` で「似た名前のローカル変数があります」と、`message` を提案してくれています。下の `+` は「ここに1文字足す」という印です。
- 打ち間違いは、Rust で最もよく出るエラーの1つです。`help:` の提案どおりに直せば動きます。

#### 例3: 書き換え禁止の変数を書き換えた（E0384）

```rust
fn main() {
    let count = 0;
    count = count + 1;
    println!("count = {count}");
}
```

**▼ 実行結果**

```text
error[E0384]: cannot assign twice to immutable variable `count`
 --> src\main.rs:3:5
  |
2 |     let count = 0;
  |         ----- first assignment to `count`
3 |     count = count + 1;
  |     ^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut count = 0;
  |         +++
```

- 「書き換え禁止（immutable）の変数 `count` に、2回目の代入はできない」というエラーです。
- `---` が「最初の代入はここ（2行目）」、`^^^` が「2回目の代入はここ（3行目）」を指しています。**問題の行と原因の行が別**であることに注意してください。
- `help:` は「この変数を書き換え可能（mutable）にしては？」と、2行目に `mut` を足す（`+++`）提案をしています。UNIT 01 で学んだ `let mut` です。

3つとも、`help:` や印のとおりに直すと動きます。

```text
年齢: 20
こんにちは
count = 1
```

（例1〜3を直したものを、それぞれ実行した結果です。）

### もう一歩: 実務に近い形

#### `note:` と `help:` の両方が出るエラー

> **▼ このコードがやること（先に日本語で）:** 名前を受け取ってあいさつする関数 `greet` を作り、`String` 型の変数を渡して呼び出そうとしたコードです。関数が受け取る型と、渡した型が少しだけ違っています。

```rust
// name さんにあいさつする関数
fn greet(name: &str) {
    println!("こんにちは、{name} さん");
}

fn main() {
    let name = String::from("Hanako");
    greet(name);
}
```

**▼ 実行結果**

```text
error[E0308]: mismatched types
 --> src\main.rs:8:11
  |
8 |     greet(name);
  |     ----- ^^^^ expected `&str`, found `String`
  |     |
  |     arguments to this function are incorrect
  |
note: function defined here
 --> src\main.rs:2:4
  |
2 | fn greet(name: &str) {
  |    ^^^^^ ----------
help: consider borrowing here
  |
8 |     greet(&name);
  |           +

For more information about this error, try `rustc --explain E0308`.
error: could not compile `practice` (bin "practice") due to 1 previous error
```

#### ▼ 出力を1つずつ分解して解説

##### 解説1: 問題の箇所と、関係する箇所

- `^^^^` は8行目の `name` を指し、「`&str` が欲しいのに `String` が来た」と言っています。
- `-----` は `greet` を指し、「この関数への引数が正しくない（arguments to this function are incorrect）」と言っています。

##### 解説2: `note:` は別の場所の補足

```text
note: function defined here
 --> src\main.rs:2:4
```

- `note:` は「参考までに」という補足です。ここでは「その関数はここ（2行目）で定義されていて、`name: &str` という引数を受け取ると書いてある」と、**関数の側の場所**まで見せてくれています。
- エラーの原因が**2か所にまたがる**とき（呼び出す側と、定義する側）に、両方を見比べられるようにしてくれているのです。

##### 解説3: `help:` は直し方の提案

```text
help: consider borrowing here
  |
8 |     greet(&name);
  |           +
```

- 「ここで借りる（borrow）ことを検討してください」という提案で、`name` の前に `&` を足した形を見せてくれています。
- 提案どおり `greet(&name);` に直すと、`こんにちは、Hanako さん` と表示されます。
- `&` の意味（借用）は第3巻のテーマです。今は「help が具体的な直し方まで見せてくれる」ことが分かれば十分です。

> **注意:** `help:` は**多くの場合正しい**提案ですが、いつもあなたの意図どおりとは限りません。UNIT 08 の「`_` を付けて警告を消す」提案のように、意味を考えずに従うと問題を隠してしまうこともあります。「なぜその直し方で直るのか」を一言で説明できるか、自分に聞いてから直しましょう。

#### 同じ原因のエラーは、まとめて消える

```rust
fn main() {
    // 在庫の数
    let mut stock = HashMap::new();
    stock.insert("りんご", 3);

    // 値段
    let mut prices = HashMap::new();
    prices.insert("りんご", 120);

    println!("{stock:?} {prices:?}");
}
```

**▼ 実行結果**

```text
error[E0433]: cannot find type `HashMap` in this scope
 --> src\main.rs:3:21
  |
3 |     let mut stock = HashMap::new();
  |                     ^^^^^^^ use of undeclared type `HashMap`
  |
help: consider importing this struct
  |
1 + use std::collections::HashMap;
  |

error[E0433]: cannot find type `HashMap` in this scope
 --> src\main.rs:7:22
  |
7 |     let mut prices = HashMap::new();
  |                      ^^^^^^^ use of undeclared type `HashMap`
  |
help: consider importing this struct
  |
1 + use std::collections::HashMap;
  |

For more information about this error, try `rustc --explain E0433`.
error: could not compile `practice` (bin "practice") due to 2 previous errors
```

- エラーは2つですが、原因は「`HashMap` を `use` していない」の1つだけです。
- `help:` の `1 + use std::collections::HashMap;` は、「1行目にこの行を**追加**して」という意味です（`+` は追加の印）。1行足せば、**2つのエラーが同時に消えます**。
- エラーが20個出ても慌てないでください。**最初の1つを直して、もう一度コンパイル**すると、半分以下に減ることがよくあります。

#### `rustc --explain` で教科書を開く

```powershell
rustc --explain E0384
```

**▼ 実行結果**（先頭の部分）

````text
An immutable variable was reassigned.

Erroneous code example:

```
fn main() {
    let x = 3;
    x = 5; // error, reassignment of immutable variable
}
```

By default, variables in Rust are immutable. To fix this error, add the keyword
`mut` after the keyword `let` when declaring the variable. For example:

```
fn main() {
    let mut x = 3;
    x = 5;
}
```
````

- 1行目が要約（「書き換え禁止の変数に再代入した」）、続いて**間違った例**（Erroneous code example）と、**直し方の説明と正しい例**が載っています。
- 英語ですが、コードの例は英語が読めなくても比べられます。翻訳ツールにかけるのも良い方法です。

#### エラーではないが、より良い書き方: `cargo clippy`

例3を `let mut count = 0;` に直したコードに `cargo clippy` をかけると、次の助言が出ます。

```powershell
cargo clippy
```

**▼ 実行結果**（助言の部分）

```text
warning: manual implementation of an assign operation
 --> src\main.rs:3:5
  |
3 |     count = count + 1;
  |     ^^^^^^^^^^^^^^^^^ help: replace it with: `count += 1`
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#assign_op_pattern
  = note: `#[warn(clippy::assign_op_pattern)]` on by default
```

**Clippy（クリッピー）** は、コンパイラより一歩踏み込んで「動くけれど、こう書くほうが Rust らしい」という助言をくれる道具です。ここでは「`count = count + 1` は `count += 1` と書けます」と教えてくれています。エラーと同じ読み方で読めます。

### よくあるエラーと読み方

#### 使い終わった値を、また使おうとした（E0382）

```rust
fn main() {
    let title = String::from("牛乳を買う");
    let saved = title;
    println!("保存したタイトル: {saved}");
    println!("元のタイトル: {title}");
}
```

**▼ 実行結果**

```text
error[E0382]: borrow of moved value: `title`
 --> src\main.rs:5:24
  |
2 |     let title = String::from("牛乳を買う");
  |         ----- move occurs because `title` has type `String`, which does not implement the `Copy` trait
3 |     let saved = title;
  |                 ----- value moved here
4 |     println!("保存したタイトル: {saved}");
5 |     println!("元のタイトル: {title}");
  |                              ^^^^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let saved = title.clone();
  |                      ++++++++
```

**何が起きたか:** ラベルを上から読むと、話の筋が分かります。

1. 2行目: `title` は `String` 型で、コピーされない（`Copy` ではない）型なので、渡すと**移動（move）**する
2. 3行目: `title` の中身が `saved` に**移動した**（value moved here）
3. 5行目: 移動した後の `title` を使おうとした（borrowed here after move）

Rust では、文字列などの値には**持ち主がちょうど1人**いて、`let saved = title;` で持ち主が `saved` に引っ越します。引っ越した後の `title` は空き家なので、使えません。これが Rust の中心にある**所有権**というルールで、第3巻で詳しく学びます。

**直し方:** `help:` の提案どおり `title.clone()` で複製を作って渡すか、`saved` だけを使うように書き直します（練習問題2）。

> **ヒント:** 日本語の文字が入った行では、`^^^^^` の印が少しずれて見えることがあります。日本語の文字が画面上で2文字分の幅を取るためです。印の位置より、`-->` の行番号と列番号を信じましょう。

#### `;` を忘れた

```rust
fn main() {
    let price = 120
    let count = 3;
    println!("合計: {}", price * count);
}
```

**▼ 実行結果**

```text
error: expected `;`, found keyword `let`
 --> src\main.rs:2:20
  |
2 |     let price = 120
  |                    ^ help: add `;` here
3 |     let count = 3;
  |     --- unexpected token

error: could not compile `practice` (bin "practice") due to 1 previous error
```

**何が起きたか:** 「`;` があるはずの所で、`let` というキーワードが見つかった」というエラーです。このように、文法の間違いにはエラー番号が付かないこともあります。

**直し方:** `help: add ; here` のとおり、2行目の終わり（`^` の位置）に `;` を足します。コンパイラは3行目の `let` を見て初めて「おかしい」と気づくので、**エラーの印が、本当に直すべき場所の次の行に付く**ことがある、と覚えておきましょう。

### RustNote ではここで使う

- RustNote では Topcoat の `view!` や `#[page]` のような**マクロ**（コードを自動で書き足してくれる仕組み）を使います。マクロの中で間違えると、少し長めのエラーが出ることがありますが、読み方は同じです。まず `-->` の場所と最初の `^^^` を見ます。
- 作業の区切りごとに `cargo check`・`cargo clippy`・`cargo test` を通す習慣を付けます。第11巻では、警告が1つでもあれば失敗にする設定（`-D warnings`）で、品質を保つしくみも作ります。

### 練習問題

**問題1:** 次のコードのエラーを読み、直してください。税込み価格（120 円の 1.08 倍）を表示したいコードです。

```rust
fn main() {
    let price: u32 = 120;
    // 税込み価格（1.08 倍）を計算したい
    let with_tax = price * 1.08;
    println!("税込み: {with_tax}");
}
```

```text
error[E0277]: cannot multiply `u32` by `{float}`
 --> src\main.rs:4:26
  |
4 |     let with_tax = price * 1.08;
  |                          ^ no implementation for `u32 * {float}`
  |
  = help: the trait `Mul<{float}>` is not implemented for `u32`
  = help: the following other types implement trait `Mul<Rhs>`:
            `&u32` implements `Mul<u32>`
            `&u32` implements `Mul`
            `u32` implements `Mul<&u32>`
            `u32` implements `Mul<Duration>`
            `u32` implements `Mul`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `practice` (bin "practice") due to 1 previous error
```

**問題2:** 「よくあるエラー」の E0382 のコードを、`help:` の提案を使って直してください。

**問題3:** `rustc --explain E0425` を実行して、最初の1行に書かれている要約を日本語にしてください。

### 解答と解説

**問題1の解答:** エラーの要約は「`u32`（整数）に `{float}`（小数）を掛けることはできない」です。Rust は、整数と小数を**勝手に変換して計算しません**。どちらかに型をそろえます。

```rust
fn main() {
    let price: u32 = 120;

    // 方法 1: 整数のまま「108 を掛けて 100 で割る」（小数点以下は切り捨て）
    let with_tax = price * 108 / 100;
    println!("税込み（整数で計算）: {with_tax}");

    // 方法 2: f64（小数の型）に変換してから 1.08 倍する
    let with_tax_f64 = f64::from(price) * 1.08;
    println!("税込み（小数で計算）: {with_tax_f64}");
}
```

**▼ 実行結果**

```text
税込み（整数で計算）: 129
税込み（小数で計算）: 129.60000000000002
```

小数で計算すると `129.60000000000002` という半端な結果になりました。コンピュータは小数を2進数で近似して持つため、こうした小さな誤差が出ます。お金の計算は、方法1のように**整数（円）で扱う**のが基本です。

`= help:` のあとの長い一覧（`Mul` を持っている型の一覧）は、今は読み飛ばして大丈夫です。エラーメッセージは、**分かる部分から読めばよい**のです。

**問題2の解答:**

```rust
fn main() {
    let title = String::from("牛乳を買う");
    // clone() で「複製」を作って saved に渡す。title は手元に残る
    let saved = title.clone();
    println!("保存したタイトル: {saved}");
    println!("元のタイトル: {title}");
}
```

**▼ 実行結果**

```text
保存したタイトル: 牛乳を買う
元のタイトル: 牛乳を買う
```

**問題3の解答:** 最初の行は `An unresolved name was used.` で、「解決できない（見つからない）名前が使われた」という意味です。例2の `mesage` がまさにこれでした。

```text
An unresolved name was used.

Erroneous code examples:
```

### まとめ

- エラーメッセージは「① 番号と要約 → ② `-->` の場所 → ③ `^^^` の理由 → `note:` の補足 → `help:` の提案」の順に読みます。
- 複数のエラーは**上から1つずつ**。1か所直したらすぐ `cargo check` します。分からなければ `rustc --explain 番号`。
- 警告や Clippy の助言も読み方は同じです。`help:` は「なぜ直るのか」を考えてから使いましょう。

## UNIT 10: 学習サイクルと完成像

### この単元のゴール

- 「**予想 → 実行 → 観察 → 1か所変える → 説明・記録**」の学習サイクルを、自分で回せる
- `cargo test` で**テスト**を動かし、失敗したテストの出力（`left` と `right`）を読める
- 文字列の「**バイト数**」と「**文字数**」の違いが分かる（RustNote のタイトル検査につながる）
- この巻の10単元が、RustNote の1回のリクエストのどこにつながっているかを説明できる

### たとえ話でつかむ

プログラミングの上達は、**料理の練習**に似ています。

- レシピ（教材）を何冊読んでも、料理は上手になりません。上達する人は、作る前に「**たぶんこんな味になる**」と予想し（予想）、作って（実行）、味見をして（観察）、「**塩をひとつまみだけ**増やしたらどうなるか」を試し（1か所変える）、違いを言葉にして**レシピノートに書き留めます**（説明・記録）。
- 一度に塩と砂糖と火加減を全部変えると、何が効いたのか分かりません。**変えるのは1か所ずつ**です。
- 書き留めたレシピノートが、プログラミングでは**テスト**です。「このタイトルは正しい」「空のタイトルはだめ」を自動で確かめるコードを書いておくと、後でコードを変えたときに、前にできていたことが壊れていないかを**一瞬で確かめられます**。

> **用語:** **テスト** … プログラムが期待どおりに動くかを確かめるための、別の小さなプログラム。Rust では `#[test]` という印を付けた関数がテストになり、`cargo test` で全部まとめて実行できます。

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 320" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-24" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<circle cx="360" cy="160" r="62" fill="#f8fafc" stroke="#cbd5e1" stroke-width="2"/>
<text x="360" y="154" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">1単元ごとに</text>
<text x="360" y="176" font-size="14" font-weight="700" fill="#334155" text-anchor="middle">何周も回す</text>
<rect x="290" y="12" width="140" height="58" rx="12" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="36" font-size="14" font-weight="700" fill="#1e40af" text-anchor="middle">① 予想する</text>
<text x="360" y="56" font-size="12" fill="#334155" text-anchor="middle">何が表示される？</text>
<rect x="520" y="86" width="150" height="58" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="595" y="110" font-size="14" font-weight="700" fill="#c2410c" text-anchor="middle">② 実行する</text>
<text x="595" y="130" font-size="12" fill="#334155" text-anchor="middle">そのまま動かす</text>
<rect x="470" y="232" width="160" height="58" rx="12" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="550" y="256" font-size="14" font-weight="700" fill="#047857" text-anchor="middle">③ 観察する</text>
<text x="550" y="276" font-size="12" fill="#334155" text-anchor="middle">予想と同じ？違う？</text>
<rect x="90" y="232" width="160" height="58" rx="12" fill="#fdf2f8" stroke="#db2777" stroke-width="2"/>
<text x="170" y="256" font-size="14" font-weight="700" fill="#be185d" text-anchor="middle">④ 1か所変える</text>
<text x="170" y="276" font-size="12" fill="#334155" text-anchor="middle">値や型を1つだけ</text>
<rect x="40" y="86" width="170" height="58" rx="12" fill="#fefce8" stroke="#ca8a04" stroke-width="2"/>
<text x="125" y="110" font-size="14" font-weight="700" fill="#a16207" text-anchor="middle">⑤ 説明・記録する</text>
<text x="125" y="130" font-size="12" fill="#334155" text-anchor="middle">言葉にしてテストに</text>
<path d="M432,40 C500,44 560,60 585,82" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-24)"/>
<path d="M600,146 C610,180 595,210 575,228" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-24)"/>
<path d="M468,262 L254,262" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-24)"/>
<path d="M140,230 C120,200 115,175 120,148" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-24)"/>
<path d="M150,84 C180,50 230,40 286,40" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-24)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図10-1: 学習サイクル。読んだ量ではなく、このサイクルを何周回したかで進み具合を測る</div>
</div>

この単元のコードで扱う「文字数」の落とし穴を、先に図で見ておきます。Rust の文字列は **UTF-8** という形でメモリに並んでいて、日本語の文字は1文字が **3バイト** になります。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 210" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="20" y="28" font-size="13" font-weight="700" fill="#334155">"Rust"</text>
<rect x="120" y="12" width="40" height="28" rx="4" fill="#eff6ff" stroke="#3b82f6"/>
<rect x="164" y="12" width="40" height="28" rx="4" fill="#eff6ff" stroke="#3b82f6"/>
<rect x="208" y="12" width="40" height="28" rx="4" fill="#eff6ff" stroke="#3b82f6"/>
<rect x="252" y="12" width="40" height="28" rx="4" fill="#eff6ff" stroke="#3b82f6"/>
<text x="140" y="31" font-size="13" fill="#1e40af" text-anchor="middle">R</text>
<text x="184" y="31" font-size="13" fill="#1e40af" text-anchor="middle">u</text>
<text x="228" y="31" font-size="13" fill="#1e40af" text-anchor="middle">s</text>
<text x="272" y="31" font-size="13" fill="#1e40af" text-anchor="middle">t</text>
<text x="310" y="31" font-size="12" fill="#334155">4 文字 ＝ 4 バイト（英数字は1文字1バイト）</text>
<text x="20" y="88" font-size="13" font-weight="700" fill="#334155">"牛乳を買う"</text>
<rect x="120" y="64" width="114" height="34" rx="4" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<rect x="238" y="64" width="114" height="34" rx="4" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<rect x="356" y="64" width="114" height="34" rx="4" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<rect x="474" y="64" width="114" height="34" rx="4" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<rect x="592" y="64" width="114" height="34" rx="4" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="177" y="87" font-size="15" font-weight="700" fill="#c2410c" text-anchor="middle">牛</text>
<text x="295" y="87" font-size="15" font-weight="700" fill="#c2410c" text-anchor="middle">乳</text>
<text x="413" y="87" font-size="15" font-weight="700" fill="#c2410c" text-anchor="middle">を</text>
<text x="531" y="87" font-size="15" font-weight="700" fill="#c2410c" text-anchor="middle">買</text>
<text x="649" y="87" font-size="15" font-weight="700" fill="#c2410c" text-anchor="middle">う</text>
<g fill="#ffffff" stroke="#94a3b8">
<rect x="120" y="104" width="36" height="22" rx="3"/><rect x="159" y="104" width="36" height="22" rx="3"/><rect x="198" y="104" width="36" height="22" rx="3"/>
<rect x="238" y="104" width="36" height="22" rx="3"/><rect x="277" y="104" width="36" height="22" rx="3"/><rect x="316" y="104" width="36" height="22" rx="3"/>
<rect x="356" y="104" width="36" height="22" rx="3"/><rect x="395" y="104" width="36" height="22" rx="3"/><rect x="434" y="104" width="36" height="22" rx="3"/>
<rect x="474" y="104" width="36" height="22" rx="3"/><rect x="513" y="104" width="36" height="22" rx="3"/><rect x="552" y="104" width="36" height="22" rx="3"/>
<rect x="592" y="104" width="36" height="22" rx="3"/><rect x="631" y="104" width="36" height="22" rx="3"/><rect x="670" y="104" width="36" height="22" rx="3"/>
</g>
<text x="120" y="146" font-size="12" fill="#334155">↑ 各文字の下の小さな箱が1バイト。5 文字 ＝ 15 バイト</text>
<rect x="20" y="160" width="330" height="40" rx="8" fill="#fef2f2" stroke="#dc2626"/>
<text x="34" y="185" font-size="13" fill="#b91c1c" font-family="Cascadia Mono, Consolas, monospace">title.len()           → 15（バイト数）</text>
<rect x="370" y="160" width="336" height="40" rx="8" fill="#ecfdf5" stroke="#10b981"/>
<text x="384" y="185" font-size="13" fill="#047857" font-family="Cascadia Mono, Consolas, monospace">title.chars().count() → 5（文字数）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図10-2: バイト数と文字数。「80文字まで」を len() で数えると、日本語は 27 文字目で上限を超えてしまう</div>
</div>

### まず動かす最小コード

#### 予想してから実行する: タイトルの文字数チェック（バグあり）

RustNote のノートのタイトルは「1文字以上80文字以下」と決めています。その検査をする関数を書いてみました。ただし、この版には**バグ**が1つ入っています。

> **▼ このコードがやること（先に日本語で）:** タイトルの文字数を数える関数と、「1〜80文字なら正しい」と判定する関数を作り、3つのタイトルで試して結果を表示します。さらに、正しく動いているかを自動で確かめる**テスト**を4つ用意します。

```rust
// タイトルの文字数を数える（この版にはバグがある）
fn title_length(title: &str) -> usize {
    title.len()
}

// 1 文字以上 80 文字以下なら true（正しいタイトル）
fn is_valid_title(title: &str) -> bool {
    let length = title_length(title);
    (1..=80).contains(&length)
}

fn main() {
    for title in ["Rust", "牛乳を買う", ""] {
        let length = title_length(title);
        let valid = is_valid_title(title);
        println!("{title:?}: 文字数 {length}, 正しい? {valid}");
    }
}

// ここから下は cargo test のときだけ使われる「テスト」
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_japanese_characters() {
        assert_eq!(title_length("牛乳を買う"), 5);
    }

    #[test]
    fn empty_title_is_invalid() {
        assert!(!is_valid_title(""));
    }

    #[test]
    fn eighty_characters_is_valid() {
        // 「あ」を 80 個並べた文字列を作る
        let title = "あ".repeat(80);
        assert!(is_valid_title(&title));
    }

    #[test]
    fn eighty_one_characters_is_invalid() {
        let title = "あ".repeat(81);
        assert!(!is_valid_title(&title));
    }
}
```

**① 予想:** `cargo run` すると、3行それぞれに何と表示されるでしょうか。特に `"牛乳を買う"` の文字数を予想してから、次に進んでください。

**② 実行:**

```powershell
cargo run -q
```

**▼ 実行結果**

```text
"Rust": 文字数 4, 正しい? true
"牛乳を買う": 文字数 15, 正しい? true
"": 文字数 0, 正しい? false
```

**③ 観察:** `"牛乳を買う"` は5文字なのに、**15** と表示されました。予想と違ったところが、学びのチャンスです。続けてテストも実行してみます。

```powershell
cargo test
```

**▼ 実行結果**

```text
   Compiling practice v0.1.0 (C:\Users\you\rust-study\practice)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.46s
     Running unittests src\main.rs (target\debug\deps\practice-7d4a8a329333a0ec.exe)

running 4 tests
test tests::eighty_one_characters_is_invalid ... ok
test tests::empty_title_is_invalid ... ok
test tests::eighty_characters_is_valid ... FAILED
test tests::counts_japanese_characters ... FAILED

failures:

---- tests::eighty_characters_is_valid stdout ----

thread 'tests::eighty_characters_is_valid' (54260) panicked at src\main.rs:39:9:
assertion failed: is_valid_title(&title)

---- tests::counts_japanese_characters stdout ----

thread 'tests::counts_japanese_characters' (3680) panicked at src\main.rs:27:9:
assertion `left == right` failed
  left: 15
 right: 5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::counts_japanese_characters
    tests::eighty_characters_is_valid

test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin practice`
```

#### ▼ コードと出力を1つずつ分解して解説

##### 解説1: バグの正体は `len()`

```rust
fn title_length(title: &str) -> usize {
    title.len()
}
```

- `len()` は文字列の **バイト数** を返します（図10-2）。英数字だけなら文字数と同じなので、`"Rust"` では正しく見えます。
- 日本語は1文字3バイトなので、`"牛乳を買う"` は 15 になりました。この関数のままだと、日本語のタイトルは **26文字（78バイト）までしか**付けられません。
- `usize` は「0 以上の整数で、長さや個数を表すのに使う型」です。

##### 解説2: テストの書き方

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_japanese_characters() {
        assert_eq!(title_length("牛乳を買う"), 5);
    }
    // ...
}
```

- `#[cfg(test)]` は「`cargo test` のときだけコンパイルする」という印です。ふだんの `cargo run` では、テストは実行ファイルに含まれません。
- `mod tests { ... }` はテストをまとめる入れ物（**モジュール**）、`use super::*;` は「外側で作った関数をここで使えるようにする」という宣言です。
- `#[test]` を付けた関数が、テスト1つ分です。
- `assert_eq!(実際の値, 期待する値)` は「2つが等しいはず。違ったらテスト失敗」、`assert!(条件)` は「条件が true のはず」という確認です。`!` は「〜でない」です。

##### 解説3: テストの結果の読み方

```text
test tests::eighty_characters_is_valid ... FAILED
test tests::counts_japanese_characters ... FAILED
```

- テストごとに `ok`（成功）か `FAILED`（失敗）が並びます。実行の順番は毎回変わることがあります（いくつかのテストを同時に動かしているため）。
- 最後の `test result: FAILED. 2 passed; 2 failed;` が集計です。

```text
thread 'tests::counts_japanese_characters' (3680) panicked at src\main.rs:27:9:
assertion `left == right` failed
  left: 15
 right: 5
```

- 失敗したテストの詳しい理由です。`src\main.rs:27:9` が失敗した `assert_eq!` の場所です。
- `left` は `assert_eq!` の**左側（実際の値）**、`right` は**右側（期待した値）**です。「15 になったが、5 を期待していた」と読みます。
- `eighty_characters_is_valid` は `assertion failed: is_valid_title(&title)`、つまり「80文字のタイトルが正しいと判定されなかった」です（240バイトなので 80 を超えた）。

##### 解説4: うまくいったテストにも意味がある

- `eighty_one_characters_is_invalid`（81文字はだめ）は、バグがあっても**たまたま**成功しています（243バイトも 80 を超えるので）。
- テストは「成功した＝正しい」とは限りません。**失敗するはずのことが、ちゃんと失敗するか**も確かめるために、境目（0文字・1文字・80文字・81文字）をいくつも用意しておくのがコツです。

### もう一歩: 実務に近い形

#### ④ 1か所だけ変えて、⑤ テストで確かめる

直すのは `title_length` の**1行だけ**です。

```rust
// タイトルの文字数を数える（文字の個数で数える）
fn title_length(title: &str) -> usize {
    title.chars().count()
}
```

ほかの部分（`is_valid_title`・`main`・テスト）は、そのままにします。

```powershell
cargo run -q
cargo test
```

**▼ 実行結果**

```text
"Rust": 文字数 4, 正しい? true
"牛乳を買う": 文字数 5, 正しい? true
"": 文字数 0, 正しい? false
```

```text
   Compiling practice v0.1.0 (C:\Users\you\rust-study\practice)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running unittests src\main.rs (target\debug\deps\practice-7d4a8a329333a0ec.exe)

running 4 tests
test tests::counts_japanese_characters ... ok
test tests::empty_title_is_invalid ... ok
test tests::eighty_one_characters_is_invalid ... ok
test tests::eighty_characters_is_valid ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: `chars().count()` で文字を数える

```rust
title.chars().count()
```

- `.chars()` は文字列を**1文字ずつ**取り出します（バイトではなく文字単位）。
- `.count()` は、取り出した個数を数えます。`"牛乳を買う"` なら 5 です。
- データベースの SQLite（libSQL）の `length()` も文字数を数えるので、RustNote ではアプリ側も DB 側も「文字数で 1〜80」にそろえています。

##### 解説2: `contains` で範囲を確かめる

```rust
(1..=80).contains(&length)
```

- `1..=80` は「1 以上 80 以下」の範囲です（UNIT 01 の `for` でも使いました）。
- `.contains(&length)` は「`length` がこの範囲に入っているか」を true / false で返します。`length >= 1 && length <= 80` と同じ意味ですが、範囲が一目で分かります。

##### 解説3: テストは「直したこと」を固定する

- 今回は、テストのおかげで「直ったこと」を**4つの角度から一度に**確かめられました。
- もっと大事なのはこれからです。後で誰かが「`len()` のほうが速いのでは」と書き換えたら、`cargo test` がすぐに `left: 15, right: 5` で知らせてくれます。**一度直したバグが二度と戻らない**ようにするのがテストの役目です。
- 学習でも同じです。分かったことをテストの形で残すと、自分の理解の「記録」にもなります。

### よくあるエラーと読み方

#### テストの失敗を読む

ここまでに見た、テストの失敗の出力をまとめます。

| 出力 | 意味 | 次にすること |
|---|---|---|
| `test 名前 ... FAILED` | そのテストが失敗した | 下の `---- 名前 stdout ----` を読む |
| `panicked at src\main.rs:27:9:` | 失敗した確認の場所 | その行の `assert` を見る |
| ``assertion `left == right` failed`` と `left:` / `right:` | `assert_eq!` で値が違った | 左（実際）と右（期待）を比べる |
| `assertion failed: 式` | `assert!` の条件が false だった | その式がなぜ false か考える |
| `test result: FAILED. 2 passed; 2 failed;` | 集計 | 全部 ok になるまで直す |
| ``error: test failed, to rerun pass `--bin practice` `` | Cargo のまとめ | 気にしなくてよい（再実行の方法の案内） |

> **つまずきポイント:** テストが失敗したとき、**テストのほうを書き換えて**通そうとしたくなることがあります。たとえば `assert_eq!(title_length("牛乳を買う"), 15);` に変えれば通りますが、それは「バグを正しいことにしてしまう」直し方です。失敗したら、まず「**期待した値（right）のほうが正しいか**」を考え、正しければ**コードのほう**を直します。

### RustNote ではここで使う

- RustNote では、タイトルを `String` のままではなく **`NoteTitle`** という専用の型にして、作るときに必ず `NoteTitle::parse` で「1〜80文字か」を検査します（上限は `MAX_TITLE_CHARS = 80` という定数）。文字数は、この単元と同じ `chars().count()` で数えます。検査を通らないタイトルは、そもそも `NoteTitle` として存在できないので、「検査を忘れる」事故が起こりません（第4巻・第9巻）。
- この単元のようなテストを、第11巻ではたくさん書きます。関数のテスト、HTTP のリクエストを送ってレスポンスを確かめるテスト、「他人のノートは 404」を確かめるセキュリティのテストまで用意します。

この巻の10単元は、RustNote の**1回のリクエスト**の中に全部登場します。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 360" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-25" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="360" y="22" font-size="14" font-weight="700" fill="#0f172a" text-anchor="middle">「牛乳を買う」というノートを作成ボタンで送ったとき</text>
<rect x="14" y="36" width="216" height="62" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="122" y="60" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">① POST /notes を送る</text>
<text x="122" y="82" font-size="12" fill="#334155" text-anchor="middle">URL・HTTP・Cookie（U05）</text>
<rect x="252" y="36" width="216" height="62" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="60" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">② 8080 番に届く</text>
<text x="360" y="82" font-size="12" fill="#334155" text-anchor="middle">Fly.io・ポート（U06）</text>
<rect x="490" y="36" width="216" height="62" rx="10" fill="#f8fafc" stroke="#475569" stroke-width="2"/>
<text x="598" y="60" font-size="13" font-weight="700" fill="#334155" text-anchor="middle">③ Rust のプロセスが処理</text>
<text x="598" y="82" font-size="12" fill="#334155" text-anchor="middle">OS・コンパイル済み（U01・U04）</text>
<rect x="490" y="134" width="216" height="62" rx="10" fill="#fdf2f8" stroke="#db2777" stroke-width="2"/>
<text x="598" y="158" font-size="13" font-weight="700" fill="#be185d" text-anchor="middle">④ 誰か調べる</text>
<text x="598" y="180" font-size="12" fill="#334155" text-anchor="middle">セッション・認証（U08）</text>
<rect x="252" y="134" width="216" height="62" rx="10" fill="#fefce8" stroke="#ca8a04" stroke-width="2"/>
<text x="360" y="158" font-size="13" font-weight="700" fill="#a16207" text-anchor="middle">⑤ タイトルを検査</text>
<text x="360" y="180" font-size="12" fill="#334155" text-anchor="middle">1〜80文字・テスト（U10）</text>
<rect x="14" y="134" width="216" height="62" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="122" y="158" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">⑥ DB に INSERT</text>
<text x="122" y="180" font-size="12" fill="#334155" text-anchor="middle">テーブル・制約（U07）</text>
<rect x="14" y="232" width="216" height="62" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="122" y="256" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">⑦ 303 で一覧へ</text>
<text x="122" y="278" font-size="12" fill="#334155" text-anchor="middle">ステータスコード（U05）</text>
<rect x="252" y="232" width="216" height="62" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="360" y="256" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">⑧ GET /notes</text>
<text x="360" y="278" font-size="12" fill="#334155" text-anchor="middle">自分のノートだけ（U08）</text>
<rect x="490" y="232" width="216" height="62" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="598" y="256" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">⑨ 一覧が表示される</text>
<text x="598" y="278" font-size="12" fill="#334155" text-anchor="middle">HTML を描く（U05）</text>
<line x1="232" y1="67" x2="248" y2="67" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-25)"/>
<line x1="470" y1="67" x2="486" y2="67" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-25)"/>
<line x1="598" y1="100" x2="598" y2="130" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-25)"/>
<line x1="488" y1="165" x2="472" y2="165" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-25)"/>
<line x1="250" y1="165" x2="234" y2="165" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-25)"/>
<line x1="122" y1="198" x2="122" y2="228" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-25)"/>
<line x1="232" y1="263" x2="248" y2="263" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-25)"/>
<line x1="470" y1="263" x2="486" y2="263" stroke="#475569" stroke-width="2" marker-end="url(#ar-00-25)"/>
<text x="360" y="326" font-size="12" fill="#64748b" text-anchor="middle">途中でエラーが出たら U09 の読み方で、ファイルやコマンドで迷ったら U02・U03 に戻る</text>
<text x="360" y="348" font-size="12" fill="#64748b" text-anchor="middle">（U は UNIT の略）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図10-3: RustNote の1回のノート作成と、この巻の単元の対応</div>
</div>

### 練習問題

**問題1（予想してから実行）:** 「もう一歩」で直した版に、次のテストを追加して `cargo test` すると、成功するでしょうか、失敗するでしょうか。予想してから実行してください。

```rust
    #[test]
    fn spaces_only_title_is_invalid() {
        assert!(!is_valid_title("   "));
    }
```

**問題2:** 問題1で失敗した場合は、`title_length` の1行だけを変えて、5つのテストがすべて成功するようにしてください。

**問題3:** 図10-1 の学習サイクルを、この単元のどの作業が①〜⑤のどれに当たるか、対応づけて説明してください。

### 解答と解説

**問題1の解答:** **失敗します**。空白3つの `"   "` は、`chars().count()` で数えると3文字なので、「1〜80文字」の範囲に入り、正しいタイトルと判定されてしまうからです（出力の最初の3行と最後の1行は省略しています）。

```text
running 5 tests
test tests::counts_japanese_characters ... ok
test tests::eighty_characters_is_valid ... ok
test tests::eighty_one_characters_is_invalid ... ok
test tests::empty_title_is_invalid ... ok
test tests::spaces_only_title_is_invalid ... FAILED

failures:

---- tests::spaces_only_title_is_invalid stdout ----

thread 'tests::spaces_only_title_is_invalid' (53800) panicked at src\main.rs:50:9:
assertion failed: !is_valid_title("   ")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::spaces_only_title_is_invalid

test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

このように、**先に「こうあるべき」というテストを書いて、失敗するのを確かめてから直す**やり方は、実務でもよく使われます。

**問題2の解答:** 数える前に、前後の空白を `trim()` で取り除きます。

```rust
// タイトルの文字数を数える（前後の空白を取り除いてから数える）
fn title_length(title: &str) -> usize {
    title.trim().chars().count()
}
```

**▼ 実行結果**（`cargo test`）

```text
running 5 tests
test tests::counts_japanese_characters ... ok
test tests::eighty_characters_is_valid ... ok
test tests::eighty_one_characters_is_invalid ... ok
test tests::empty_title_is_invalid ... ok
test tests::spaces_only_title_is_invalid ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

なお、空白だけのタイトルを認めるかどうかは、アプリの**決めごと（仕様）**です。ここでは「認めない」と決めた場合の練習です。RustNote での扱いは第9巻で要件として決めます。

**問題3の解答（例）:** ①「`"牛乳を買う"` は 5 と出るはず」と予想した → ② `cargo run` と `cargo test` を実行した → ③ 15 と表示され、テストが2つ失敗したのを見た → ④ `len()` を `chars().count()` の1か所だけ変えた → ⑤「`len()` はバイト数、`chars().count()` は文字数」と説明できるようになり、テストとして残した。問題1・2は、2周目のサイクルです。

### まとめ

- 「予想 → 実行 → 観察 → 1か所変える → 説明・記録」を回すほど、理解が深くなります。
- `len()` は**バイト数**、`chars().count()` は**文字数**です。日本語を扱うときは特に注意します。
- テストは、分かったこと・直したことを**固定する記録**です。失敗したら `left`（実際）と `right`（期待）を比べ、まずコードを疑います。

## この先の学び方

### 巻の順番

基本は第0巻から第13巻まで、**番号順**に読みます。それぞれの巻が、前の巻の知識を使うように作られているからです。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 72 720 176" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-00-26" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<line x1="30" y1="120" x2="700" y2="120" stroke="#475569" stroke-width="3" marker-end="url(#ar-00-26)"/>
<circle cx="70" cy="120" r="9" fill="#f97316"/>
<circle cx="200" cy="120" r="9" fill="#3b82f6"/>
<circle cx="330" cy="120" r="9" fill="#3b82f6"/>
<circle cx="450" cy="120" r="9" fill="#f97316"/>
<circle cx="560" cy="120" r="9" fill="#10b981"/>
<circle cx="660" cy="120" r="9" fill="#10b981"/>
<text x="70" y="96" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">第0・1巻</text>
<text x="70" y="150" font-size="12" fill="#334155" text-anchor="middle">hello が動く</text>
<text x="70" y="168" font-size="12" fill="#334155" text-anchor="middle">道具がそろう</text>
<text x="200" y="96" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">第2〜3巻</text>
<text x="200" y="150" font-size="12" fill="#334155" text-anchor="middle">小さなプログラム</text>
<text x="200" y="168" font-size="12" fill="#334155" text-anchor="middle">が書ける</text>
<text x="330" y="96" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">第4〜5巻</text>
<text x="330" y="150" font-size="12" fill="#334155" text-anchor="middle">大きめのプログラム</text>
<text x="330" y="168" font-size="12" fill="#334155" text-anchor="middle">を組み立てられる</text>
<text x="450" y="96" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">第6〜8巻</text>
<text x="450" y="150" font-size="12" fill="#334155" text-anchor="middle">ページ・DB・</text>
<text x="450" y="168" font-size="12" fill="#334155" text-anchor="middle">ログインが作れる</text>
<text x="560" y="96" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">第9〜11巻</text>
<text x="560" y="150" font-size="12" fill="#334155" text-anchor="middle">RustNote が</text>
<text x="560" y="168" font-size="12" fill="#334155" text-anchor="middle">手元で完成</text>
<text x="660" y="96" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">第12〜13巻</text>
<text x="660" y="150" font-size="12" fill="#334155" text-anchor="middle">世界に公開</text>
<text x="660" y="168" font-size="12" fill="#334155" text-anchor="middle">全体を読み通す</text>
<rect x="150" y="196" width="420" height="40" rx="8" fill="#f8fafc" stroke="#cbd5e1"/>
<text x="360" y="221" font-size="12" fill="#334155" text-anchor="middle">第3巻（所有権）は難所。1回で分からなくても先へ進み、後で戻ってよい</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図10-4: 全14巻のマイルストーン。各段階で「できるようになること」</div>
</div>

- **第1巻（環境構築）は飛ばさない**でください。道具の版がそろっていないと、教材と違うエラーが出て、原因探しに時間を取られます。
- **第3巻（所有権・借用）は、多くの人がつまずく難所**です。1回で全部分からなくても大丈夫です。第4巻以降で何度も出てくるうちに分かってくるので、7割分かったら先へ進み、後で戻ってきましょう。
- 第13巻には RustNote の**完全なコード**が載っています。第9〜10巻で手が止まったら、答え合わせに使ってかまいません。ただし、まず自分で書いてから見るほうが、ずっと身に付きます。

### つまずいたときの調べ方

困ったときは、次の順番で調べると、たいていの問題は自分で解決できます。

1. **エラーメッセージを最初から読む** … UNIT 09 の読み方で、`-->` の場所と `help:` を確かめる。
2. **今いる場所と版を確かめる** … `Get-Location`（UNIT 02・03）、`cargo --version`。教材と同じ版か。
3. **最小の形にする** … 動かないコードから、関係なさそうな行を1つずつ消して、エラーが出る最小のコードを作る。これだけで原因が見えることが多い。
4. **公式の説明を読む** … `rustc --explain エラー番号`、Rust の公式の入門書「The Rust Programming Language」（日本語訳もあります）、クレートの説明サイト docs.rs、Topcoat の公式ドキュメント。
5. **エラー全文で検索する** … エラーの1行目（`error[E0308]: mismatched types` など）をそのまま検索する。自分のユーザー名などは消してから。
6. **人や AI に聞く** … 下の「AI の上手な使い方」のとおりに聞く。

> **ヒント:** 調べて解決したことは、短いメモに残しておきましょう。「何をしたら・どんなエラーが出て・何が原因で・どう直したか」の4行で十分です。同じ問題に2回目に出会ったとき、そのメモが一番の味方になります。

### Claude Code など AI の上手な使い方

Claude Code（クロード コード）のような AI のアシスタントは、Rust の学習でもとても頼りになります。ただし、**使い方しだいで、上達を大きく助けることも、逆に妨げることもあります**。

**上手な聞き方:** 次の4つをそろえて伝えると、的確な答えが返ってきやすくなります。

```text
【やりたいこと】RustNote のタイトルを 1〜80 文字に制限したい
【したこと】title.len() で長さを調べて、80 以下なら OK にした
【起きたこと】日本語のタイトルが 27 文字で「長すぎる」と言われる
【聞きたいこと】原因と、正しい数え方を、理由も含めて教えてください
```

- **エラーは全文を貼る** … 「エラーが出ました」だけでは、AI も推測するしかありません。`error[...]` から最後の行まで、そのまま貼ります。
- **「直して」より「なぜ？」と聞く** … 直ったコードだけをもらうと、次に同じ問題が起きたとき自分で直せません。「なぜこのエラーになるのか」「なぜその直し方でよいのか」を説明してもらいましょう。
- **答えは必ず自分で動かして確かめる** … AI の答えは、もっともらしいのに間違っていることがあります。特に Topcoat のような新しいフレームワークは、古い版の書き方を答えてしまうことがあります。`cargo check`・`cargo test` で確かめ、教材や公式ドキュメントと見比べます。
- **まず自分で考えてから聞く** … UNIT 10 のサイクルの①予想と③観察を自分でしてから聞くと、答えの意味がよく分かります。
- **秘密を貼らない** … パスワード、API キー、`.env` ファイルの中身、本物の利用者のデータは、AI にも検索にも貼りません。

> **注意:** AI に書いてもらったコードを、意味が分からないまま RustNote に入れるのはやめましょう。認証や認可（UNIT 08）のような安全に関わる部分は特にです。「このコードを1行ずつ説明して」と頼み、説明できるようになってから使います。

### この巻のあとにすること

この巻はこれで終わりです。次は**第1巻（環境構築）**で、Windows に PowerShell・Git・VS Code・Rust などの道具をそろえます。道具がそろったら、この巻に戻ってきて、各単元のコードを**実際に動かしながら**もう一度読んでみてください。1回目には見えなかったことが、きっと見えてきます。

## この巻のまとめ

各単元のゴールを、チェックリストにしました。自分の言葉で説明できるか、手を動かしてできるかを確かめて、できたものに印を付けましょう。説明できないものがあれば、その単元の「たとえ話」と「図」に戻ってください。

**コンピュータの基本（UNIT 01〜04）**

- □ CPU・メモリ・ストレージ・OS の役割を、台所のたとえで説明できる
- □ プロセスとは何かを説明でき、プロセス ID（PID）が起動のたびに変わる理由が分かる
- □ 標準出力・標準エラー出力・終了コードを区別でき、PowerShell で `$LASTEXITCODE` を確かめられる
- □ 絶対パスと相対パスを書き分けられ、`.` と `..` の意味が分かる
- □ 「ファイルが見つからない」と言われたら、カレントディレクトリを確かめられる
- □ PowerShell で `Get-Location`・`cd`・`ls`・`mkdir`・`cat`・`Copy-Item`・`Remove-Item` が使える
- □ コマンドを「コマンド名・サブコマンド・オプション・引数」に分けて読める
- □ コマンドライン引数と環境変数で、プログラムに値を渡せる
- □ Windows PowerShell 5.1 では `curl.exe` と書く理由、ファイルを UTF-8 で作る理由が分かる
- □ `cargo new`・`cargo run`・`cargo build`・`cargo check`・`--release` を使い分けられる
- □ コンパイル時の失敗と、実行時の失敗（パニック）を区別できる

**Web の基本（UNIT 05〜08）**

- □ URL をスキーム・ホスト・ポート・パス・クエリに分けられる
- □ URL を入れてから画面が出るまでの流れ（DNS → 接続 → リクエスト → 処理 → レスポンス → 描画）を説明できる
- □ HTTP のリクエストとレスポンスの形（1行目・ヘッダー・空行・本文）を説明できる
- □ GET と POST、200・303・404・500 の意味が分かる
- □ `curl.exe -v` で、やりとりがどこまで進んだかを確かめられる
- □ ポート・localhost（127.0.0.1）・0.0.0.0 の違いを説明できる
- □ Rust の標準ライブラリだけで、パスによって返事を変える小さなサーバーを動かせる
- □ 「ポートが使用中」「実行ファイルを消せない」エラーに対処できる
- □ メモリとストレージの違いを、カウンターの例で説明できる
- □ テーブル・行・列・主キー・UNIQUE・CHECK の意味が分かる
- □ 認証と認可の違いを説明でき、他人のノートに 404 を返す理由が分かる
- □ パスワードをハッシュ（Argon2id）で保存する理由が分かる

**学び方（UNIT 09〜10）**

- □ エラーメッセージを、エラー番号・`-->`・`^^^`・`note:`・`help:` の部品に分けて読める
- □ エラーが複数出ても、上から1つずつ直せる
- □ `rustc --explain` でエラー番号の説明を読める
- □ `cargo test` を実行し、`left` と `right` を比べて失敗の理由を読める
- □ `len()`（バイト数）と `chars().count()`（文字数）の違いが分かる
- □ 「予想 → 実行 → 観察 → 1か所変える → 説明・記録」のサイクルを回せる
- □ RustNote の完成像（画面・URL・ブラウザ／サーバー／データベースの構成）と、全14巻の順番を説明できる

## 用語ミニ辞典

この巻に出てきた用語を、五十音順（数字・英字は最後）に並べました。「初出の単元」は、その用語を最初に説明した UNIT の番号です。

| 用語 | 読み | 意味 | 初出の単元 |
|---|---|---|---|
| アプリ（プログラム） | アプリ | 決まった手順を書いたもの。動かしていない状態のもの | 01 |
| 暗号化 | あんごうか | 途中で盗み見や書き換えをされないように、データを変換すること。HTTPS で使う | 05 |
| 一方向 | いっぽうこう | ハッシュ値から元のデータに戻せない性質 | 08 |
| 永続化 | えいぞくか | プログラムが終わってもデータが残るように保存すること | 07 |
| エイリアス | エイリアス | コマンドの別名。PowerShell の `ls` は `Get-ChildItem` の別名 | 03 |
| エラー（error） | エラー | 直さないとコンパイルが終わらない問題 | 09 |
| 拡張子 | かくちょうし | ファイル名の最後の `.` から後ろ（`.rs` など）。中身の種類の目印 | 02 |
| カレントディレクトリ | カレントディレクトリ | 今いるフォルダ。相対パスの出発点 | 02 |
| 環境変数 | かんきょうへんすう | OS やシェルが持つ「名前＝値」のメモ。設定や秘密を渡すのに使う | 03 |
| 機械語 | きかいご | CPU がそのまま実行できる命令の並び | 04 |
| クライアント | クライアント | サーバーに頼みごとをする側（ブラウザなど） | 06 |
| クレート | クレート | Rust のプログラムや部品のひとまとまり | 04 |
| 警告（warning） | けいこく | コンパイルは通るが、間違いの可能性やより良い書き方を知らせる指摘 | 08 |
| 構造体（struct） | こうぞうたい | 関係のある値をまとめた、自分で作る型 | 08 |
| コマンドライン引数 | コマンドラインひきすう | コマンドの後ろに並べて、プログラムに渡す値 | 01 |
| コンパイル | コンパイル | ソースコードを機械語に翻訳すること。翻訳する道具がコンパイラ（Rust では `rustc`） | 04 |
| サーバー | サーバー | リクエストを待ち受けて、レスポンスを返す側のプログラム | 06 |
| シェル | シェル | コマンドを読んで、OS にプログラムの起動などを頼むプログラム（PowerShell・bash） | 03 |
| 終了コード | しゅうりょうコード | プロセスが終わるときに OS へ渡す数。0 は成功 | 01 |
| 主キー | しゅキー | テーブルの1行を特定するための列（`id` など）。同じ値は2つない | 07 |
| ステータスコード | ステータスコード | HTTP のレスポンスの結果を表す3桁の数（200・404 など） | 05 |
| ストレージ | ストレージ | 電源を切ってもデータが残る記憶装置（SSD など） | 01 |
| 制約 | せいやく | データベースの列に付けるルール（NOT NULL・UNIQUE・CHECK など） | 07 |
| セッション | セッション | ログインした状態を覚えておくしくみ。トークンを Cookie で受け渡す | 08 |
| 絶対パス | ぜったいパス | ドライブ名（`C:\`）から書いた、どこからでも同じ場所を指すパス | 02 |
| 相対パス | そうたいパス | 今いるフォルダから見た道順で書いたパス | 02 |
| ソルト | ソルト | パスワードのハッシュに混ぜる、人ごとに違う乱数 | 08 |
| ソースコード | ソースコード | 人が読み書きするプログラムの文章 | 04 |
| 端末（ターミナル） | たんまつ | 文字を入力して結果を表示する窓 | 03 |
| テスト | テスト | プログラムが期待どおりに動くかを確かめる小さなプログラム | 10 |
| テーブル | テーブル | データベースの表。行（1件）と列（項目）でできている | 07 |
| データベース（DB） | データベース | データを整理して保存し、探したり書き換えたりするプログラム | 07 |
| 認可 | にんか | その操作をしてよいかを判断すること | 08 |
| 認証 | にんしょう | 相手が誰かを確かめること（ログイン） | 08 |
| ハッシュ | ハッシュ | データから決まった長さの値を作る一方向の変換。パスワードの保存に使う | 08 |
| バイト | バイト | データの大きさの単位。UTF-8 では英数字が1バイト、日本語がふつう3バイト | 10 |
| パス | パス | ファイルやフォルダの場所を表す文字列。URL の中の `/notes` などもパスと呼ぶ | 02 |
| パニック | パニック | Rust のプログラムが実行中に問題を見つけて止まること。終了コードは 101 | 01 |
| 標準エラー出力 | ひょうじゅんエラーしゅつりょく | エラーの知らせを書く出口（`eprintln!`） | 01 |
| 標準出力 | ひょうじゅんしゅつりょく | ふつうの結果を書く出口（`println!`） | 01 |
| 標準ライブラリ | ひょうじゅんライブラリ | Rust に最初から付いてくる道具箱（`std`） | 01 |
| フォルダ（ディレクトリ） | フォルダ | ファイルやフォルダを入れる入れ物 | 02 |
| フレームワーク | フレームワーク | アプリでよく使う仕組みをまとめた部品集。Topcoat など | 巻頭 |
| プロセス | プロセス | 実行中のプログラム | 01 |
| プロトコル | プロトコル | やりとりの書き方の決まり。HTTP など | 05 |
| プロンプト | プロンプト | コマンドの入力を待っている印（`PS C:\Users\you>`） | 03 |
| ポート | ポート | 1台のコンピュータの中で、どのプロセスに届けるかを決める番号 | 06 |
| マイグレーション | マイグレーション | テーブルの形を変える手順を、順番に記録して適用するしくみ | 07 |
| メソッド（HTTP） | メソッド | リクエストで「何をしてほしいか」を表す言葉（GET・POST） | 05 |
| メモリ | メモリ | 作業中のデータを置く、速いが電源を切ると消える記憶装置 | 01 |
| 文字コード | もじコード | 文字と数値の対応表。UTF-8・Shift_JIS など | 03 |
| リクエスト | リクエスト | クライアントからサーバーへのお願い | 05 |
| ルーティング | ルーティング | メソッドとパスによって、処理を振り分けること | 06 |
| レスポンス | レスポンス | サーバーからの返事 | 05 |
| 0.0.0.0 | ゼロゼロゼロゼロ | 「すべての入口で待つ」という待ち受け側の指定 | 06 |
| 127.0.0.1（localhost） | ローカルホスト | 自分の PC を指す特別な住所 | 06 |
| Argon2id | アルゴンツー・アイディー | パスワード専用の、わざと遅いハッシュ関数 | 08 |
| bind / listen | バインド／リッスン | ポートを借りる届け出／来客を待つこと | 06 |
| Cargo | カーゴ | Rust のプロジェクト管理の道具。ビルド・実行・テストなどを取り仕切る | 04 |
| Cookie | クッキー | サーバーがブラウザに預ける小さなメモ。リクエストのたびに送り返される | 08 |
| CPU | シーピーユー | 命令を実行して計算する部品 | 01 |
| curl | カール | HTTP のリクエストを送り、結果を文字で見せる道具。Windows では `curl.exe` | 05 |
| DNS | ディーエヌエス | ホスト名を IP アドレスに引き直すしくみ | 05 |
| edition | エディション | Rust の文法の版。このシリーズは 2024 | 04 |
| HTML | エイチティーエムエル | Web ページの中身を書く言葉 | 05 |
| HTTP / HTTPS | エイチティーティーピー | ブラウザとサーバーのやりとりの決まり。HTTPS は暗号化版 | 05 |
| IDOR | アイドア | ID を書き換えるだけで他人のデータが見えてしまう不具合 | 08 |
| IP アドレス | アイピーアドレス | ネットワーク上のコンピュータの住所 | 06 |
| OS | オーエス | コンピュータの資源をプロセスに割り振る基本のソフト（Windows など） | 01 |
| PowerShell | パワーシェル | Windows のシェル。この教材の主な作業環境 | 03 |
| PRG | ピーアールジー | POST の後に 303 で GET のページへ移動させる作法 | 05 |
| SQL | エスキューエル | データベースにお願いするための言葉 | 07 |
| TCP | ティーシーピー | 2台の間で、データを順番どおり確実に届けるしくみ。HTTP はこの上を流れる | 05 |
| Turso / libSQL | ターソ／リブエスキューエル | RustNote が使うデータベースのサービスと、その中身のデータベース | 07 |
| URL | ユーアールエル | Web 上の宛先。スキーム・ホスト・ポート・パス・クエリでできている | 05 |
| UTF-8 | ユーティーエフエイト | 世界中の文字を表せる文字コード。Rust の文字列はこれ | 03 |

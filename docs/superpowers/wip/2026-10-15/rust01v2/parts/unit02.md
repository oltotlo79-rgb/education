## UNIT 02: Git・改行コード・秘密の除外

### この単元のゴール

- Git for Windows をインストールし、版と、設定がどのファイルに書かれているか（`--show-origin`）を確かめられる
- 名前・メールアドレス・最初のブランチ名を設定し、最初の**コミット**ができる
- `.gitignore` で、ビルドの成果物・データベースのファイル・秘密の値を書いた `.env` を、Git に入れないようにできる
- 改行コード（**CRLF** と **LF**）の違いと、`LF will be replaced by CRLF` という警告の意味を説明し、`.gitattributes` で改行コードを決められる
- うっかり秘密をコミットすると履歴に残ることを確かめ、そのときに何をすべきかを説明できる

### たとえ話でつかむ

**Git**（ギット）は、工房の「作業日誌つきの写真アルバム」です。

- 作業台の上の、今の状態が**作業フォルダ**です。
- 作業がひと区切りしたら、記録に残したい物を選んで台紙に並べます。これが **`git add`**（ステージに載せる）です。
- 台紙をアルバムに貼り、日付と「何をしたか」の一言を書き添えます。これが **`git commit`**（コミット）です。アルバムを後からめくれば、どの日に何がどう変わったかが分かり、昔の状態に戻すこともできます。
- 撮らない物もあります。作業で出た削りくず（ビルドの成果物）、毎日中身が変わる大きな帳簿（データベースのファイル）、そして**家の鍵**（パスワードや合い鍵の番号を書いたファイル）です。「アルバムに貼らない物リスト」が **`.gitignore`** です。
- 一度アルバムに貼った鍵の写真は、次のページで「鍵は外しました」と書いても、**前のページには残ったまま**です。アルバムを人に見せれば、鍵の形は分かってしまいます。

もう1つ、**改行コード**は「文の区切り方の方言」です。Windows では行の終わりを「CR と LF」の2文字で、Linux や macOS では「LF」の1文字で書くのが流儀です。Windows の方言のまま書いた手順書を Linux の係員に渡すと、行の最後に余計な CR がくっついた、知らない言葉に聞こえてしまいます。

| アルバムのたとえ | Git の言葉 | この単元で使うもの |
|---|---|---|
| 作業台の上 | 作業フォルダ（ワーキングツリー） | `~\rust-study\hello-git` |
| 台紙に並べる | ステージ（インデックス）に載せる | `git add` |
| アルバムに貼る | コミット | `git commit -m "…"` |
| アルバムそのもの | リポジトリ | `hello-git\.git` フォルダ |
| 貼らない物リスト | 無視するファイルの一覧 | `.gitignore` |
| 文の区切りの方言 | 改行コード（CRLF / LF） | `.gitattributes` |

> **用語:** リポジトリ（repository：Git が変更の記録をしまっておく場所。プロジェクトのフォルダの中の `.git` という隠しフォルダがその正体）

<!-- -->

> **用語:** コミット（commit：ある時点のファイルの状態を、メッセージ・作った人・日時と一緒に記録したもの。1つずつに `d779d78` のような番号（ハッシュ）が付く）

<!-- -->

> **用語:** 改行コード（行の終わりを表す見えない文字。**CR**（キャリッジ・リターン、`\r`、16進数で `0D`）と **LF**（ライン・フィード、`\n`、`0A`）がある。Windows の流儀は CR+LF の2文字で **CRLF**、Linux や macOS の流儀は **LF** の1文字）

### 図で理解する

まず、Git がファイルを記録するまでの流れです。ファイルは、作業フォルダ → ステージ → リポジトリ、の順に進みます。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-021" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="16" y="20" width="220" height="210" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="126" y="44" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">作業フォルダ</text>
<text x="32" y="72" font-size="12" fill="#0f172a" font-family="Consolas, monospace">Cargo.toml</text>
<text x="32" y="92" font-size="12" fill="#0f172a" font-family="Consolas, monospace">src/main.rs</text>
<text x="32" y="112" font-size="12" fill="#0f172a" font-family="Consolas, monospace">.env.example</text>
<text x="32" y="132" font-size="12" fill="#0f172a" font-family="Consolas, monospace">.gitignore</text>
<line x1="32" y1="146" x2="220" y2="146" stroke="#cbd5e1" stroke-width="1"/>
<text x="32" y="166" font-size="12" fill="#94a3b8" font-family="Consolas, monospace">.env</text>
<text x="32" y="186" font-size="12" fill="#94a3b8" font-family="Consolas, monospace">rustnote.db</text>
<text x="32" y="206" font-size="12" fill="#94a3b8" font-family="Consolas, monospace">target/</text>
<text x="210" y="186" font-size="12" fill="#b91c1c" text-anchor="end">✗ 無視</text>
<rect x="300" y="70" width="130" height="110" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="365" y="96" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">ステージ</text>
<text x="365" y="116" font-size="11" fill="#475569" text-anchor="middle">（インデックス）</text>
<text x="365" y="142" font-size="11" fill="#475569" text-anchor="middle">次のコミットに</text>
<text x="365" y="158" font-size="11" fill="#475569" text-anchor="middle">入れる物の台紙</text>
<rect x="504" y="40" width="200" height="170" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="604" y="64" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">リポジトリ（.git）</text>
<circle cx="540" cy="110" r="10" fill="#10b981"/>
<circle cx="604" cy="110" r="10" fill="#10b981"/>
<circle cx="668" cy="110" r="10" fill="#10b981"/>
<line x1="550" y1="110" x2="594" y2="110" stroke="#047857" stroke-width="2"/>
<line x1="614" y1="110" x2="658" y2="110" stroke="#047857" stroke-width="2"/>
<text x="604" y="146" font-size="11" fill="#475569" text-anchor="middle">コミットが順に並ぶ</text>
<text x="604" y="166" font-size="11" fill="#475569" text-anchor="middle">（いつでも前に戻れる）</text>
<line x1="236" y1="110" x2="296" y2="110" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-021)"/>
<text x="266" y="100" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">git add</text>
<line x1="430" y1="125" x2="500" y2="125" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-021)"/>
<text x="465" y="115" font-size="11" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">git commit</text>
<text x="126" y="256" font-size="12" fill="#b91c1c" text-anchor="middle">.gitignore に書いた物は、</text>
<text x="126" y="274" font-size="12" fill="#b91c1c" text-anchor="middle">git add の対象にならない</text>
<text x="604" y="256" font-size="12" fill="#475569" text-anchor="middle">git status で、3つの場所の</text>
<text x="604" y="274" font-size="12" fill="#475569" text-anchor="middle">違いを見られる</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: Git がファイルを記録する流れ。作業フォルダのファイルを git add でステージに載せ、git commit でリポジトリに記録する。.gitignore に書いたファイルは載らない</div>
</div>

次に、改行コードです。同じ `pwd` という1行でも、Windows の流儀と Linux の流儀では、ファイルの中身（バイト）が違います。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 330" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-022" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="20" y="34" font-size="13" font-weight="700" fill="#0f172a">CRLF（Windows の流儀）</text>
<rect x="210" y="16" width="60" height="28" rx="4" fill="#f8fafc" stroke="#64748b"/>
<text x="240" y="35" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">p 70</text>
<rect x="274" y="16" width="60" height="28" rx="4" fill="#f8fafc" stroke="#64748b"/>
<text x="304" y="35" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">w 77</text>
<rect x="338" y="16" width="60" height="28" rx="4" fill="#f8fafc" stroke="#64748b"/>
<text x="368" y="35" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">d 64</text>
<rect x="402" y="16" width="70" height="28" rx="4" fill="#fee2e2" stroke="#b91c1c"/>
<text x="437" y="35" font-size="12" fill="#b91c1c" text-anchor="middle" font-family="Consolas, monospace">\r 0D</text>
<rect x="476" y="16" width="70" height="28" rx="4" fill="#dbeafe" stroke="#3b82f6"/>
<text x="511" y="35" font-size="12" fill="#1e40af" text-anchor="middle" font-family="Consolas, monospace">\n 0A</text>
<text x="20" y="80" font-size="13" font-weight="700" fill="#0f172a">LF（Linux・macOS の流儀）</text>
<rect x="210" y="62" width="60" height="28" rx="4" fill="#f8fafc" stroke="#64748b"/>
<text x="240" y="81" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">p 70</text>
<rect x="274" y="62" width="60" height="28" rx="4" fill="#f8fafc" stroke="#64748b"/>
<text x="304" y="81" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">w 77</text>
<rect x="338" y="62" width="60" height="28" rx="4" fill="#f8fafc" stroke="#64748b"/>
<text x="368" y="81" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">d 64</text>
<rect x="402" y="62" width="70" height="28" rx="4" fill="#dbeafe" stroke="#3b82f6"/>
<text x="437" y="81" font-size="12" fill="#1e40af" text-anchor="middle" font-family="Consolas, monospace">\n 0A</text>
<text x="560" y="54" font-size="11" fill="#b91c1c">Linux は <tspan font-family="Consolas, monospace">\r</tspan> を</text>
<text x="560" y="70" font-size="11" fill="#b91c1c">名前の一部と思う</text>
<rect x="16" y="120" width="200" height="110" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="116" y="144" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">Windows の作業フォルダ</text>
<text x="116" y="170" font-size="12" fill="#0f172a" text-anchor="middle">ふつうのファイル: LF</text>
<text x="116" y="192" font-size="12" fill="#0f172a" text-anchor="middle">*.ps1・*.bat: CRLF</text>
<text x="116" y="216" font-size="11" fill="#475569" text-anchor="middle">VS Code も LF で保存（UNIT 03）</text>
<rect x="260" y="120" width="200" height="110" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="360" y="144" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">リポジトリの中</text>
<text x="360" y="170" font-size="12" fill="#0f172a" text-anchor="middle">テキストは全部 LF</text>
<text x="360" y="192" font-size="12" fill="#0f172a" text-anchor="middle">画像・DB は変換しない</text>
<rect x="504" y="120" width="200" height="110" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="604" y="144" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">Linux の世界</text>
<text x="604" y="170" font-size="12" fill="#0f172a" text-anchor="middle">WSL・Docker・CI</text>
<text x="604" y="192" font-size="12" fill="#0f172a" text-anchor="middle">Fly.io: LF のまま動く</text>
<line x1="216" y1="175" x2="256" y2="175" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-022)" marker-start="url(#ar-01-022)"/>
<line x1="460" y1="175" x2="500" y2="175" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-022)" marker-start="url(#ar-01-022)"/>
<rect x="160" y="252" width="400" height="64" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="360" y="276" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">.gitattributes（リポジトリに入れる決まり）</text>
<text x="260" y="300" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">* text=auto eol=lf</text>
<text x="460" y="300" font-size="12" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">*.ps1 text eol=crlf</text>
<line x1="360" y1="252" x2="360" y2="234" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-022)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: 改行コードの違いと、.gitattributes で決める置き方。リポジトリの中も作業フォルダも LF にそろえ、Windows 専用のスクリプトだけ CRLF にすると、Linux の世界へそのまま持っていける</div>
</div>

- CRLF のファイルを Linux で読むと、行の最後の `\r`（`0D`）が、コマンドの名前の一部として扱われます。`pwd` は `pwd\r` という知らないコマンドになってしまいます（「よくあるエラー」のエラー2で実際に見ます）。
- RustNote は、最後は Fly.io（Linux）の上で動きます。Windows で書いたファイルを、Linux の世界へ**そのまま**持っていけるように、最初から LF にそろえておくのが安全です。

最後に、秘密の値の扱いです。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 280" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-023" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="16" y="20" width="300" height="100" rx="10" fill="#fee2e2" stroke="#b91c1c" stroke-width="2"/>
<text x="166" y="44" font-size="13" font-weight="700" fill="#b91c1c" text-anchor="middle">.env（本物の値）</text>
<text x="32" y="70" font-size="12" fill="#0f172a" font-family="Consolas, monospace">DATABASE_AUTH_TOKEN=eyJhbG…</text>
<text x="166" y="100" font-size="12" fill="#b91c1c" text-anchor="middle">自分の PC だけに置く・Git に入れない</text>
<rect x="16" y="140" width="300" height="100" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="166" y="164" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">.env.example（見本）</text>
<text x="32" y="190" font-size="12" fill="#0f172a" font-family="Consolas, monospace">DATABASE_AUTH_TOKEN=</text>
<text x="166" y="220" font-size="12" fill="#047857" text-anchor="middle">名前だけ書いて Git で共有する</text>
<text x="530" y="34" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">うっかり .env をコミットした場合</text>
<circle cx="400" cy="90" r="12" fill="#10b981"/>
<circle cx="520" cy="90" r="12" fill="#b91c1c"/>
<circle cx="640" cy="90" r="12" fill="#10b981"/>
<line x1="412" y1="90" x2="508" y2="90" stroke="#475569" stroke-width="2"/>
<line x1="532" y1="90" x2="628" y2="90" stroke="#475569" stroke-width="2"/>
<text x="400" y="70" font-size="11" fill="#0f172a" text-anchor="middle">最初のコミット</text>
<text x="520" y="70" font-size="11" fill="#b91c1c" text-anchor="middle">.env を入れた</text>
<text x="640" y="70" font-size="11" fill="#0f172a" text-anchor="middle">.env を外した</text>
<rect x="356" y="150" width="348" height="90" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="530" y="176" font-size="12" fill="#c2410c" text-anchor="middle">最新のコミットに .env が無くても、</text>
<text x="530" y="198" font-size="12" fill="#c2410c" text-anchor="middle">真ん中のコミットの中には残っている</text>
<text x="530" y="222" font-size="12" fill="#0f172a" text-anchor="middle">→ 上げてしまったら、鍵（トークン）を無効にする</text>
<path d="M520,104 V146" fill="none" stroke="#b91c1c" stroke-width="2" stroke-dasharray="4 3" marker-end="url(#ar-01-023)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: 秘密の値の置き方。本物の値は .env に書いて自分の PC だけに置き、Git では見本の .env.example を共有する。一度コミットした秘密は、後で外しても履歴に残る</div>
</div>

### まず動かす最小コード

この単元の「最小コード」も、PowerShell のコマンドです。Git を入れて、最初の設定をし、練習用のリポジトリを作るところまで進めます。

#### 例2-1: Git をインストールして、版と置き場所を確かめる

> **▼ この手順がやること（先に日本語で）:** Git for Windows（Windows 用の Git）をインストールします。新しく開いた PowerShell で、Git の版・Git の本体の場所・インストーラーが書き込んだ設定を表示して、正しく入ったことを確かめます。

**手順1: インストールする前に確かめる。** PowerShell で `git --version` と打ちます。版が表示されたら、Git はもう入っています（手順2は飛ばしてかまいません）。第0巻 UNIT 03 で見た「用語 'git' は、…認識されません」のエラーになったら、まだ入っていません。

**手順2: インストーラーで入れる。** ブラウザで Git の公式サイト（`https://git-scm.com/`）を開き、Windows 用のダウンロードのページから、インストーラーを入手します。

- インストーラーには、x64 用と **ARM64 用**があります。ARM64 の PC では ARM64 用を選びます。間違えて x64 用を入れても動きますが、エミュレーションになります（UNIT 01）。
- インストーラーを実行すると、たくさんの画面が順に出てきます。**基本は既定のまま「Next」**で進めます。途中に次の画面があります（画面の名前は、この本の PC に入っている Git 2.53.0 のインストーラーのものです）。

| 画面の名前 | 既定の選択 | この本での扱い |
|---|---|---|
| Choosing the default editor used by Git | Use Vim (the ubiquitous text editor) as Git's default editor | 既定のままでよい。VS Code に変える方法は UNIT 03 で説明します |
| Adjusting the name of the initial branch in new repositories | Let Git decide | 既定のままでよい。例2-2 で `main` に設定します |
| Adjusting your PATH environment | Git from the command line and also from 3rd-party software | 既定のまま（PowerShell から `git` を使えるようにする選択） |
| Configuring the line ending conversions | Checkout Windows-style, commit Unix-style line endings | 既定のままでよい。改行コードは例2-5 で、リポジトリごとに決めます |

> **ヒント:** winget（ウィンゲット：Windows に入っている、コマンドでアプリを入れる道具）を使う方法もあります。`winget install --id Git.Git -e --source winget` と打つと、画面を出さずに既定の設定で入ります。この本の PC では winget での入れ方は試していないので、入れた後に、次の手順3で版と置き場所を必ず確かめてください。

**手順3: 新しい窓で確かめる。** インストールが終わったら、**PowerShell の窓をいったん閉じて、新しく開き直します**。そして次のコマンドを打ちます。

```powershell
# Git の版を表示する
git --version
# git というコマンドの本体が、どこにあるかを表示する
where.exe git
# インストーラーが書き込んだ、PC 全体（system）の設定を一覧で表示する
git config --system --list
```

**▼ 実行結果**

```text
git version 2.53.0.windows.2
C:\Program Files\Git\cmd\git.exe
diff.astextplain.textconv=astextplain
filter.lfs.clean=git-lfs clean -- %f
filter.lfs.smudge=git-lfs smudge -- %f
filter.lfs.process=git-lfs filter-process
filter.lfs.required=true
http.sslbackend=schannel
core.autocrlf=true
core.fscache=true
core.symlinks=false
pull.rebase=false
credential.helper=manager
credential.https://dev.azure.com.usehttppath=true
init.defaultbranch=master
```

#### ▼ 1つずつ分解して解説

##### 解説1: `git --version` の `windows.2`

- `2.53.0` が Git そのものの版、`.windows.2` は「Git 2.53.0 をもとにした、Windows 用の2回目の版」という Git for Windows の印です。あなたの PC では、もっと新しい版かもしれません。

##### 解説2: `where.exe git` は「どこの git が動くか」

- `where.exe` は、`PATH`（第0巻 UNIT 03）に書かれたフォルダを順に探して、その名前のプログラムがどこにあるかを教えてくれます。`C:\Program Files\Git\cmd\git.exe` が、PowerShell から `git` と打ったときに動く入口です。
- PowerShell では、`where` とだけ打つと、`Where-Object`（一覧から条件に合うものを選ぶコマンド）の別名として扱われてしまいます。場所を調べるときは、必ず `.exe` まで付けて `where.exe` と打ちます。
- 「情報: 与えられたパターンのファイルが見つかりませんでした。」と表示されるときは、`PATH` に Git のフォルダが入っていません。窓を開き直したか確かめましょう。

> **ヒント:** ARM64 版の Git が入っているかは、`Test-Path "C:\Program Files\Git\clangarm64"` で確かめられます。この本の PC（ARM64 版の Git）では `True` と表示されました。

##### 解説3: インストーラーが書き込んだ設定（system）

- `git config --system --list` は、**PC の全員**に効く設定の一覧です。インストーラーを既定のまま進めると、この内容が `C:\Program Files\Git\etc\gitconfig` というファイルに書き込まれます。
- 特に知っておきたいのは次の3つです。

| 設定 | 意味 |
|---|---|
| `core.autocrlf=true` | ファイルを取り出すときに改行を CRLF に、記録するときに LF に変換する（インストーラーの「Checkout Windows-style, commit Unix-style line endings」） |
| `credential.helper=manager` | GitHub などにログインした情報を、Windows に安全に覚えておく係（Git Credential Manager）を使う |
| `init.defaultbranch=master` | 新しいリポジトリの最初のブランチの名前を `master` にする（インストーラーの「Let Git decide」） |

- 設定がどのファイルに書かれているかは、`--show-origin` を付けると分かります。

```powershell
# core.autocrlf の値と、それが書かれているファイルを表示する
git config --show-origin --get core.autocrlf
```

**▼ 実行結果**

```text
file:C:/Program Files/Git/etc/gitconfig	true
```

#### 例2-2: 名前・メールアドレス・最初のブランチ名を設定する

> **▼ この手順がやること（先に日本語で）:** コミットに記録される「作った人の名前とメールアドレス」を設定します。さらに、新しいリポジトリの最初のブランチの名前を `main` にします。どれも、あなたのユーザーだけに効く設定（global）です。

```powershell
# コミットに記録する名前（ローマ字がおすすめ）
git config --global user.name "Taro Yamada"
# コミットに記録するメールアドレス
git config --global user.email "taro@example.com"
# 新しいリポジトリの最初のブランチの名前を main にする
git config --global init.defaultBranch main
```

`Taro Yamada` と `taro@example.com` は、あなたの名前とメールアドレスに置き換えてください。3行とも、成功すると何も表示されません。設定した内容は、次のコマンドで確かめます。

```powershell
# あなたのユーザーの設定（global）を一覧で表示する
git config --global --list
```

一覧の中に、`user.name=…`・`user.email=…`・`init.defaultbranch=main` の3行があれば成功です（この本の PC には別の名前が設定されているので、ここの表示は載せていません）。

#### ▼ 1つずつ分解して解説

##### 解説1: `--system`・`--global`・`--local` の3つの段

- Git の設定には3つの段があり、**下の段ほど優先**されます。

| 段 | 効く範囲 | 書かれているファイル |
|---|---|---|
| `--system` | PC の全員 | `C:\Program Files\Git\etc\gitconfig` |
| `--global` | あなたのユーザー | `C:\Users\（あなたの名前）\.gitconfig` |
| `--local` | そのリポジトリだけ | リポジトリの中の `.git\config` |

- 例2-1で見たとおり、system には `init.defaultbranch=master` が書かれています。global に `main` を書いたので、あなたが作るリポジトリでは `main` が使われます。
- `init.defaultBranch` のように、名前の大文字・小文字は区別されません。一覧では小文字（`init.defaultbranch`）で表示されます。

##### 解説2: 名前とメールアドレスは、コミットと一緒に公開される

- ここで設定した名前とメールアドレスは、**コミットのたびに記録**されます。GitHub などにコードを上げると、誰でも見られる状態になります（例2-5の後で、実際に記録された内容を見ます）。
- 本名やふだん使いのメールアドレスを公開したくない人は、GitHub が用意している「公開用のメールアドレス（noreply のアドレス）」を使う方法があります。GitHub のアカウントの設定で確かめてください。

##### 解説3: なぜ `main` にするのか

- ブランチ（branch：枝）は、変更の流れに付ける名前です。GitHub で新しく作るリポジトリの最初のブランチは `main` なので、手元もそろえておくと、後で GitHub につなぐときに迷いません。このシリーズでも `main` を使います。

#### 例2-3: 練習用のリポジトリ hello-git を作り、状態を見る

> **▼ この手順がやること（先に日本語で）:** `rust-study` の中に練習用のフォルダ `hello-git` を作り、Git のリポジトリにします。その中に、Rust のプロジェクトによくあるファイル（設定のファイル・プログラム・環境変数の見本と本物・データベースのファイル・ビルドの成果物）の**代わりになる小さなファイル**を作り、Git から見た状態を表示します。

**手順1: フォルダを作って、リポジトリにする。**

```powershell
# 作業フォルダへ移動して、練習用のフォルダを作る
cd ~\rust-study
mkdir hello-git
cd hello-git
# このフォルダを Git のリポジトリにする
git init
```

**▼ 実行結果**（`mkdir` の表示は省いています）

```text
Initialized empty Git repository in C:/Users/.../rust-study/hello-git/.git/
```

**手順2: 練習用のファイルを作る。** Rust のプロジェクトは UNIT 04 で `cargo new` を使って作りますが、ここでは同じ形のファイルを手で作ります。まず、ファイルを作るための小さな**関数**（いくつかのコマンドをまとめて名前を付けたもの）を、PowerShell に覚えさせます。

```powershell
# 1行ずつの文字列を LF（`n）でつないで、最後にも LF を付け、UTF-8（BOM なし）で書き出す関数
function New-LfFile($Path, [string[]]$Lines) {
    $text = ($Lines -join "`n") + "`n"
    New-Item -Path $Path -Value $text -Force | Out-Null
}
```

続けて、この関数でファイルを作ります。`'…'` で囲んだ1つずつが、ファイルの1行になります。

```powershell
# Rust のプロジェクトの設定ファイルの代わり
New-LfFile Cargo.toml '[package]', 'name = "hello-git"', 'version = "0.1.0"', 'edition = "2024"', '', '[dependencies]'
# プログラムの代わり（src フォルダも一緒に作られる）
New-LfFile src\main.rs 'fn main() {', '    println!("Hello, world!");', '}'
# 環境変数の見本（値は空のまま）
New-LfFile .env.example 'HOST=127.0.0.1', 'PORT=3000', 'DATABASE_URL=rustnote.db', 'DATABASE_AUTH_TOKEN='
# 本物の値を書いた環境変数のファイル（練習用の偽物のトークン）
New-LfFile .env 'HOST=127.0.0.1', 'PORT=3000', 'DATABASE_URL=libsql://rustnote-taro.turso.io', 'DATABASE_AUTH_TOKEN=eyJhbGciOi...secret'
# データベースのファイルの代わり
New-LfFile rustnote.db 'db'
# ビルドの成果物の代わり（空のファイル。target\debug フォルダも一緒に作られる）
New-Item target\debug\hello-git.exe -Force | Out-Null
```

**手順3: Git から見た状態を表示する。**

```powershell
# 作業フォルダ・ステージ・リポジトリの違いを表示する
git status
```

**▼ 実行結果**

```text
On branch main

No commits yet

Untracked files:
  (use "git add <file>..." to include in what will be committed)
	.env
	.env.example
	Cargo.toml
	rustnote.db
	src/
	target/

nothing added to commit but untracked files present (use "git add" to track)
```

#### ▼ 1つずつ分解して解説

##### 解説1: `git init` で、フォルダがリポジトリになる

- `git init` は、今いるフォルダの中に `.git` という隠しフォルダを作ります。これがリポジトリ（アルバム）の正体です。中身を手で触る必要はありません。
- 表示のパスが `C:/Users/...` のように `/` で区切られているのは、Git が Linux 生まれの道具だからです。Windows でも `/` のパスはたいてい通じます。

##### 解説2: `New-LfFile` 関数で、LF のファイルを作る

```powershell
function New-LfFile($Path, [string[]]$Lines) {
    $text = ($Lines -join "`n") + "`n"
    New-Item -Path $Path -Value $text -Force | Out-Null
}
```

- `function 名前(引数) { … }` で、PowerShell に新しいコマンドを覚えさせます。`$Path` にファイルの名前、`$Lines` に行の並び（`[string[]]` は「文字列の並び」という意味）を受け取ります。覚えさせたコマンドは、**その窓を閉じるまで**使えます。
- `` "`n" `` は、PowerShell で **LF**（改行の1文字）を表す書き方です。`` ` ``（バッククォート）は、日本語のキーボードでは Shift キーを押しながら `@` のキーを押すと打てます。`-join` で行の並びを LF でつなぎ、最後にも LF を1つ足しています。
- `New-Item -Path … -Value …` は、受け取った文字列を**そのまま** UTF-8（BOM なし）でファイルに書きます。`-Force` は「フォルダが無ければ作り、ファイルがあれば上書きする」という指定です。`| Out-Null` は、作ったファイルの一覧の表示を消しています。
- なぜわざわざ関数を作ったのかというと、Windows PowerShell 5.1 の `Set-Content` で書くと、改行が CRLF になり、`-Encoding utf8` では BOM も付くからです。Rust のプロジェクトのファイルは、`cargo new` が作るのと同じ「LF・BOM なし」にしておきたいのです。UNIT 03 で VS Code を LF で保存する設定にした後は、エディタで作れば済むので、この関数は使いません。

> **ヒント:** この本の PC で、この関数で作ったファイルを1バイトずつ比べ、後の例の実行結果を取ったときのファイルと、中身が完全に同じであることを確かめています。

##### 解説3: `git status` の読み方

- `On branch main` は「今は `main` というブランチにいます」、`No commits yet` は「まだコミットが1つもありません」という意味です。
- `Untracked files`（追跡していないファイル）は、「作業フォルダにあるけれど、Git がまだ記録の対象にしていないファイル」の一覧です。フォルダは中身をまとめて `src/` のように表示されます。
- 今は、**`.env`（秘密）・`rustnote.db`（データベース）・`target/`（ビルドの成果物）まで並んでいます**。このまま `git add .`（全部をステージに載せる）とすると、これらも記録されてしまいます。次の「もう一歩」で、これを防ぎます。

> **注意:** `.env` のトークン `eyJhbGciOi...secret` は、練習用の偽物です。本物のトークンは、絶対にこの本の手順のように画面に出したり、人に見せたりしないでください。

### もう一歩: 実務に近い形

#### 例2-4: .gitignore で、入れてはいけないファイルを外す

> **▼ この手順がやること（先に日本語で）:** 「Git に入れないファイル」の一覧 `.gitignore` を作ります。作った後の `git status` で、秘密・データベース・ビルドの成果物が一覧から消えたことを確かめます。どの行の決まりで外されたのかも表示し、最後に `.env` を無理に追加しようとして、Git に止められることを確かめます。

```powershell
# Git に入れないファイルの一覧を作る
New-LfFile .gitignore @(
    '# ビルドの成果物（cargo build が作る）'
    '/target/'
    ''
    '# ローカルの SQLite / libSQL データベース'
    '*.db'
    '*.db-shm'
    '*.db-wal'
    ''
    '# 秘密の値を書いた環境変数ファイル（見本の .env.example だけを共有する）'
    '.env'
)
# もう一度、状態を表示する
git status
```

`@(` の後で Enter キーを押すと、行の先頭に `>>` が出て、続きを入力できます。最後の `)` の行で Enter キーを押すと、ファイルが作られます。

**▼ 実行結果**

```text
On branch main

No commits yet

Untracked files:
  (use "git add <file>..." to include in what will be committed)
	.env.example
	.gitignore
	Cargo.toml
	src/

nothing added to commit but untracked files present (use "git add" to track)
```

`.env`・`rustnote.db`・`target/` が一覧から消えました。どの決まりで外されたのかは、`git check-ignore -v` で確かめられます。

```powershell
# 3つのファイルが、.gitignore のどの行で外されたかを表示する
git check-ignore -v .env rustnote.db target/debug/hello-git.exe
```

**▼ 実行結果**

```text
.gitignore:10:.env	.env
.gitignore:5:*.db	rustnote.db
.gitignore:2:/target/	target/debug/hello-git.exe
```

最後に、`.env` を名指しで追加しようとしてみます。

```powershell
# 無視しているファイルを、名指しで追加しようとする
git add .env
```

**▼ 実行結果**

```text
The following paths are ignored by one of your .gitignore files:
.env
hint: Use -f if you really want to add them.
hint: Disable this message with "git config set advice.addIgnoredFile false"
```

#### ▼ 1つずつ分解して解説

##### 解説1: `.gitignore` の書き方

| 書き方 | 意味 | この例で外れるもの |
|---|---|---|
| `/target/` | 先頭の `/` は「リポジトリのいちばん上の」、最後の `/` は「フォルダ」 | `target` フォルダの中身全部 |
| `*.db` | `*` は「何文字でもよい」。拡張子が `.db` のファイル全部 | `rustnote.db` |
| `.env` | この名前のファイル | `.env` |
| `#` で始まる行 | コメント（説明のメモ） | なし |

- `*.db-shm` と `*.db-wal` は、SQLite や libSQL（第7巻）がデータベースのファイルの横に作る、作業用のファイルです。まだ無くても、先に書いておきます。
- `.gitignore` そのものは、**Git に入れて共有**します。一緒に作業する人の PC でも、同じファイルが外されるようにするためです。

##### 解説2: `git check-ignore -v` の読み方

- `.gitignore:10:.env	.env` は、「`.gitignore` の10行目の `.env` という決まりで、`.env` が外されています」という意味です。左から「ファイル名:行番号:決まり」、空白（タブ）をはさんで「調べたパス」です。
- 「なぜか Git に入らない」「なぜか入ってしまう」と困ったときに、どの決まりが効いているのかを確かめるのに使います。

##### 解説3: `git add .env` が止められた

- 無視しているファイルを名指しで追加しようとすると、Git は「このパスは `.gitignore` で無視されています」と教えてくれて、追加しません。
- `hint: Use -f if you really want to add them.`（本当に追加したいなら `-f` を使って）と書かれていますが、**秘密のファイルで `-f` を使ってはいけません**。使うとどうなるかは、「よくあるエラー」のエラー3で見ます。

#### 例2-5: 改行コードを .gitattributes で決めて、最初のコミットをする

> **▼ この手順がやること（先に日本語で）:** まず、残ったファイルを全部ステージに載せ、Git が改行コードについて出す警告を読みます。次に、改行コードの決まり `.gitattributes` と、Windows 専用のスクリプトの例 `check.ps1` を作って、ステージの中の改行コードがどうなっているかを比べます。最後に、最初のコミットをします。

**手順1: ステージに載せて、警告を読む。**

```powershell
# 今いるフォルダの、無視していないファイルを全部ステージに載せる
git add .
```

**▼ 実行結果**

```text
warning: in the working copy of '.env.example', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of '.gitignore', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of 'Cargo.toml', LF will be replaced by CRLF the next time Git touches it
warning: in the working copy of 'src/main.rs', LF will be replaced by CRLF the next time Git touches it
```

ステージに載ったファイルの改行コードを、`git ls-files --eol` で表示します。

```powershell
# ステージのファイルと作業フォルダのファイルの、改行コードを並べて表示する
git ls-files --eol
```

**▼ 実行結果**

```text
i/lf    w/lf    attr/                 	.env.example
i/lf    w/lf    attr/                 	.gitignore
i/lf    w/lf    attr/                 	Cargo.toml
i/lf    w/lf    attr/                 	src/main.rs
```

**手順2: 改行コードの決まりを作る。**

```powershell
# 改行コードの決まりを作る
New-LfFile .gitattributes @(
    '# テキストファイルは、リポジトリにも作業フォルダにも LF で置く'
    '* text=auto eol=lf'
    ''
    '# Windows 専用のスクリプトは CRLF で置く'
    '*.ps1 text eol=crlf'
    '*.bat text eol=crlf'
    '*.cmd text eol=crlf'
    ''
    '# 画像やデータベースは文字ではないので、改行の変換をしない'
    '*.png binary'
    '*.jpg binary'
    '*.db binary'
)
# Windows 専用のスクリプトの例を作る
New-LfFile check.ps1 'Write-Output "check"'
# もう一度、全部をステージに載せる
git add .
```

**▼ 実行結果**

```text
warning: in the working copy of 'check.ps1', LF will be replaced by CRLF the next time Git touches it
```

```powershell
git ls-files --eol
```

**▼ 実行結果**

```text
i/lf    w/lf    attr/text=auto eol=lf 	.env.example
i/lf    w/lf    attr/text=auto eol=lf 	.gitattributes
i/lf    w/lf    attr/text=auto eol=lf 	.gitignore
i/lf    w/lf    attr/text=auto eol=lf 	Cargo.toml
i/lf    w/lf    attr/text eol=crlf    	check.ps1
i/lf    w/lf    attr/text=auto eol=lf 	src/main.rs
```

**手順3: 最初のコミットをする。**

```powershell
# ステージに載せた内容を、メッセージを付けて記録する
git commit -m "最初のコミット"
```

**▼ 実行結果**

```text
[main (root-commit) d779d78] 最初のコミット
 6 files changed, 36 insertions(+)
 create mode 100644 .env.example
 create mode 100644 .gitattributes
 create mode 100644 .gitignore
 create mode 100644 Cargo.toml
 create mode 100644 check.ps1
 create mode 100644 src/main.rs
```

#### ▼ 1つずつ分解して解説

##### 解説1: 「LF will be replaced by CRLF」は何の警告か

- 警告の意味は、「作業フォルダの `.env.example` の LF は、**次に Git がこのファイルに触るとき**（取り出し直すときなど）、CRLF に置き換えられます」です。今すぐファイルが変わるわけではありません。
- 原因は、例2-1で見た system の設定 `core.autocrlf=true` です。この設定は「記録するときは LF、取り出すときは CRLF」という変換をします。私たちのファイルは LF で作ったので、「取り出すときには CRLF になりますよ」と前もって知らせてくれたのです。
- このまま別の Windows の PC でこのリポジトリを取り出すと、作業フォルダのファイルは CRLF になります。それを WSL や Docker の中で使うと、図で見た「`\r` がくっつく」問題が起きます。

##### 解説2: `git ls-files --eol` の読み方

- `i/lf` は「ステージ（index）の中は LF」、`w/lf` は「作業フォルダ（working tree）のファイルは LF」、`attr/` は「`.gitattributes` で決めた決まり」です。
- 1回目は `attr/` の後ろが空でした。決まりが無いので、取り出すときの改行は、それぞれの PC の `core.autocrlf` しだいです。
- 2回目は、`attr/text=auto eol=lf` が付きました。これで、どの PC で取り出しても、作業フォルダのファイルは LF になります。`.gitattributes` の決まりは、`core.autocrlf` より優先されるからです。
- `check.ps1` だけは `attr/text eol=crlf` です。今の作業フォルダのファイルはまだ LF（`w/lf`）ですが、次に取り出したときに CRLF になります。手順2の警告は、そのことを知らせています。

##### 解説3: `.gitattributes` の書き方

| 書き方 | 意味 |
|---|---|
| `* text=auto eol=lf` | すべてのファイルについて、文字のファイルかどうかを Git に判断させ（`text=auto`）、文字のファイルなら作業フォルダでも LF にする（`eol=lf`） |
| `*.ps1 text eol=crlf` | `.ps1` は文字のファイルとして扱い、作業フォルダでは CRLF にする。後に書いた決まりが優先される |
| `*.png binary` | 文字のファイルではない（`binary`）。改行の変換も、差分の表示もしない |

- PowerShell のスクリプトは LF でも動きますが、Windows でしか使わないファイルなので、Windows の流儀（CRLF）にそろえています。`.bat` や `.cmd`（コマンドプロンプト用のスクリプト）は、LF だと正しく動かないことがあるので、CRLF にしておきます。
- この本では、PC 全体の設定（`core.autocrlf`）は既定のまま変えず、**リポジトリごとに `.gitattributes` で決めます**。`.gitattributes` はリポジトリに入るので、一緒に作業する人の PC でも、CI（自動テストの仕組み）や Docker の中でも、同じ決まりが使われるからです。

##### 解説4: コミットの表示の読み方

- `[main (root-commit) d779d78]` は、「`main` ブランチに、最初の（root）コミット `d779d78` を作りました」という意味です。`d779d78` はコミットの番号の先頭7文字で、あなたの PC では違う番号になります。
- `6 files changed, 36 insertions(+)` は「6つのファイルが変わり、36行が増えました」、`create mode 100644` は「ふつうのファイルとして新しく記録しました」という意味です。

記録された名前とメールアドレスは、`git log` で確かめられます。

```powershell
# コミットの番号・作った人・メッセージを1行ずつ表示する
git log --format="%h %an <%ae> %s"
```

**▼ 実行結果**（例2-5のすぐ後なら、この1行だけが表示されます）

```text
d779d78 Taro Yamada <taro@example.com> 最初のコミット
```

例2-2で設定した名前とメールアドレスが、コミットと一緒に記録されています。

### よくあるエラーと読み方

#### エラー1: 「Author identity unknown」（名前とメールアドレスの未設定）

例2-2を飛ばして `git commit` すると、次のエラーになります。

```text
Author identity unknown

*** Please tell me who you are.

Run

  git config --global user.email "you@example.com"
  git config --global user.name "Your Name"

to set your account's default identity.
Omit --global to set the identity only in this repository.

fatal: unable to auto-detect email address (got '...@...(none)')
```

- **何が起きたか:** 「作った人が分かりません。あなたは誰ですか？」というエラーです。コミットには作った人の名前とメールアドレスが必要なのに、どちらも設定されていませんでした。最後の行は「メールアドレスを自動で決められませんでした」という意味で、`...` の部分には Windows のユーザー名と PC の名前が入ります。
- **直し方:** エラーの中に書かれているとおり、`git config --global user.email "…"` と `git config --global user.name "…"` を実行してから、もう一度 `git commit` します。`Omit --global`（`--global` を付けないと、このリポジトリだけの設定になります）とも書かれていますが、ふつうは `--global` で設定します。

> **ヒント:** `-m "…"` を付けずに `git commit` と打つと、メッセージを書くためのエディタが開きます。既定のエディタは Vim（ヴィム）で、画面の左に `~` が並んだ黒い画面になります。慣れないうちは、`Esc` キーを押してから `:q!` と打って Enter キーを押し、いったん抜けてください（コミットは中止されます）。この本では、いつも `-m` を付けてコミットします。

#### エラー2: 「`$'\r': command not found`」（CRLF のシェルスクリプトを Linux で動かす）

Windows のエディタで作ったり、`core.autocrlf=true` の Git で取り出したりして、**CRLF になったシェルスクリプト**（bash の手順書）を、WSL の bash で動かした場合です。WSL は UNIT 07 で入れるので、今は結果を見るだけで大丈夫です。

使ったスクリプト `crlf.sh` は、次の5行です（中身は LF の `lf.sh` と同じで、改行だけが CRLF です）。

```bash
#!/bin/bash
set -e
echo "hello from WSL"
cd /tmp
pwd
```

WSL の bash で、見えない文字も表示するように（`cat -v`）して実行した結果です。

```bash
# エラーも一緒に流して（2>&1）、見えない文字を ^M のように表示する（cat -v）
bash crlf.sh 2>&1 | cat -v
```

**▼ 実行結果**

```text
crlf.sh: line 2: set: -^M: invalid option
set: usage: set [-abefhkmnptuvxBCEHPT] [-o option-name] [--] [-] [arg ...]
hello from WSL^M
crlf.sh: line 4: cd: $'/tmp\r': No such file or directory
crlf.sh: line 5: $'pwd\r': command not found
```

- **何が起きたか:** `^M` と `\r` は、どちらも CR（`0D`）のことです。bash は行の終わりを LF だけで判断するので、行の最後の CR を、**コマンドの一部**として読みました。
  - 2行目: `set -e` が `set -e\r` になり、「`-\r` という知らないオプションです」と断られました。
  - 4行目: `cd /tmp` が `/tmp\r` というフォルダへの移動になり、「そんなフォルダはありません」と言われました。
  - 5行目: `pwd` が `pwd\r` という知らないコマンドになりました。
- エラーの中の `$'pwd\r'` は、bash が「見えない文字を含む名前」を表示するときの書き方です。**`\r` が見えたら、改行コードを疑う**、と覚えておきましょう。
- **直し方:** ファイルの改行を LF に直します。WSL の bash なら、次のコマンドで行の最後の CR を取り除けます。直した後に実行すると、`hello from WSL` と `/tmp` が表示されることを、この本の PC で確かめました。

```bash
# 各行の最後の \r（CR）を取り除いて、ファイルを上書きする
sed -i 's/\r$//' crlf.sh
```

- そもそも CRLF にしないためには、この単元の `.gitattributes`（`* text=auto eol=lf`）をリポジトリに入れておき、UNIT 03 で VS Code を LF で保存する設定にします。

#### エラー3（エラーが出ない事故）: `.env` をコミットしてしまった

いちばん怖いのは、エラーが出ずに**秘密が記録されてしまう**事故です。ここでは、例2-4の注意を無視して `-f` で `.env` を追加し、コミットしてしまった場面を、わざと作って確かめます。

```powershell
# わざと失敗する実験: 無視しているはずの .env を、-f で無理やり追加してコミットする
git add -f .env
git commit -m "うっかり .env を入れた"
```

気づいて、`.env` を Git の管理から外します。`--cached` は「作業フォルダのファイルは消さずに、Git の記録からだけ外す」という指定です。

```powershell
# .env を Git の記録から外す（作業フォルダの .env は残る）
git rm --cached .env
git status --short
```

**▼ 実行結果**

```text
rm '.env'
D  .env
```

`D` は「次のコミットで、このファイルを消します（Delete）」という印です。このままコミットします。

```powershell
git commit -m ".env を Git の管理から外す"
```

これで、最新のコミットには `.env` がありません。ところが、履歴を調べると、`.env` は残っています。

```powershell
# コミットの一覧を表示する
git log --oneline
# .env に関わるコミットだけを表示する
git log --oneline -- .env
# 1つ前のコミット（HEAD~1）の中の .env を表示する
git show HEAD~1:.env
```

**▼ 実行結果**

```text
50956ca .env を Git の管理から外す
45dbcd4 うっかり .env を入れた
d779d78 最初のコミット
50956ca .env を Git の管理から外す
45dbcd4 うっかり .env を入れた
HOST=127.0.0.1
PORT=3000
DATABASE_URL=libsql://rustnote-taro.turso.io
DATABASE_AUTH_TOKEN=eyJhbGciOi...secret
```

- **何が起きたか:** `git rm --cached` で外したのは、「これからのコミット」からだけです。`45dbcd4` のコミットには、`.env` の中身がそのまま残っていて、`git show コミット:ファイル名` で誰でも読めます。図で見た「前のページに残った鍵の写真」です。
- **直し方:** 手元のリポジトリだけで、まだどこにも上げていなければ、上げる前に履歴を作り直す方法があります（少し難しいので、このシリーズでは扱いません）。**GitHub などに一度でも上げてしまったら、その秘密はもう漏れたものとして扱います**。まず、その**トークンやパスワードを無効にして、新しいものに取り替えます**。Turso のトークンを無効にする方法は、第7巻で扱います。履歴の掃除は、その後です。
- **防ぎ方:** `.gitignore` に `.env` を書くのは、**最初のコミットより前**にします。そして、`-f` は使いません。

### RustNote ではここで使う

RustNote のフォルダ（`Rust/rustnote`）の `.gitignore` は、この単元で作ったものとほとんど同じです。

```text
# ビルドの成果物（cargo build / topcoat asset bundle が作る）
/target/

# ローカルの SQLite / libSQL データベース（WAL などの付属ファイルも含む）
*.db
*.db-shm
*.db-wal
*.db-journal

# 秘密の値を書いた環境変数ファイル（.env.example だけを共有する）
.env

# OS やエディタが作るファイル
.DS_Store
Thumbs.db
```

- `*.db-journal` は、データベースのもう1つの作業用ファイルです。`.DS_Store`（macOS）と `Thumbs.db`（Windows）は、OS がフォルダの中に勝手に作るファイルです。
- `.env.example` は Git に入っていて、RustNote の README では、使う人が `Copy-Item .env.example .env` でコピーして使う手順になっています（手元で動かすだけなら、中身はそのままで動きます）。本番の Turso のトークンのような**本物の値は、コピーした `.env` にだけ書く**のです。`.env.example` の最後は次のようになっていて、トークンの欄は空です。

```text
DATABASE_URL=rustnote.db
# 本番（Turso）の例:
# DATABASE_URL=libsql://rustnote-<あなたの組織名>.turso.io
# DATABASE_AUTH_TOKEN=<turso db tokens create で作ったトークン>
DATABASE_AUTH_TOKEN=
```

一方、RustNote のフォルダには `.gitattributes` がありません。この本のリポジトリで、RustNote のファイルの改行コードを表示すると、次のようになります（最初の5つだけ載せています）。

```powershell
git ls-files --eol Rust/rustnote
```

**▼ 実行結果**（抜粋）

```text
i/lf    w/lf    attr/                 	Rust/rustnote/.dockerignore
i/lf    w/lf    attr/                 	Rust/rustnote/.env.example
i/lf    w/lf    attr/                 	Rust/rustnote/.gitignore
i/lf    w/lf    attr/                 	Rust/rustnote/Cargo.lock
i/lf    w/lf    attr/                 	Rust/rustnote/Cargo.toml
```

- 今はリポジトリの中も作業フォルダも LF ですが、`attr/` が空なので、`core.autocrlf=true` の Windows の PC で取り出すと、作業フォルダでは CRLF になります。
- RustNote は、最後は Fly.io（Linux）の上で、Docker を使って動かします（第12巻）。自分のリポジトリで RustNote を管理するときは、この単元の `.gitattributes` をいちばん上のフォルダに置いておくと、`Dockerfile` や `fly.toml` のような Linux の世界で使うファイルを、LF のまま保てます。

### 練習問題

**問1:** Rust のプロジェクトのフォルダに、次のファイルがあります。Git に入れる（コミットする）ものと、入れないものに分けてください。入れないものは、`.gitignore` のどの書き方で外せるかも答えてください。

`Cargo.toml`・`Cargo.lock`・`src/main.rs`・`target/debug/rustnote.exe`・`.env`・`.env.example`・`rustnote.db`・`rustnote.db-wal`・`README.md`

**問2:** 例2-5の手順1の `git ls-files --eol` では、どのファイルも `i/lf    w/lf    attr/` でした。

1. `i/lf`・`w/lf`・`attr/` は、それぞれ何を表していますか。
2. この状態（`.gitattributes` が無い）でコミットしたリポジトリを、`core.autocrlf=true` の別の Windows の PC で取り出すと、作業フォルダのファイルの改行は何になりますか。そのファイルを WSL の bash で使うと、何が起きる可能性がありますか。

**問3:** Turso のトークンを書いた `.env` をコミットして、GitHub に上げてしまったことに気づきました。何を、どの順番でしますか。

### 解答と解説

**問1の解答:**

| 入れる | 入れない（`.gitignore` の書き方） |
|---|---|
| `Cargo.toml`・`Cargo.lock`・`src/main.rs`・`.env.example`・`README.md` | `target/debug/rustnote.exe`（`/target/`）・`.env`（`.env`）・`rustnote.db`（`*.db`）・`rustnote.db-wal`（`*.db-wal`） |

- `Cargo.lock` は、使った部品（クレート）の正確な版を記録したファイルです（UNIT 04 で出てきます）。RustNote のような**アプリ**では、誰が・いつビルドしても同じ版の部品が使われるように、Git に入れます。RustNote のフォルダにも `Cargo.lock` が入っています。
- `target` の中は、`cargo build` でいつでも作り直せるので、入れません。データベースのファイルは、中身が毎回変わり、利用者のデータが入るので入れません。

**問2の解答:**

1. `i/lf` はステージ（index）の中が LF、`w/lf` は作業フォルダ（working tree）のファイルが LF、`attr/` は `.gitattributes` の決まりが何も無いことを表します。
2. 決まりが無いので、取り出す PC の `core.autocrlf=true` に従って、作業フォルダのファイルは **CRLF** になります。例2-5の手順1の警告「LF will be replaced by CRLF the next time Git touches it」が、まさにこのことを言っていました。そのファイルがシェルスクリプトなら、WSL の bash で動かしたときに、エラー2のように `$'\r': command not found` などのエラーになる可能性があります。

**問3の解答:**

1. **トークンを無効にして、新しいトークンに取り替える。** GitHub に上げた時点で、誰かに読まれたかもしれないものとして扱います。履歴を消すより先に、鍵そのものを使えなくするのがいちばん大事です。
2. **`.env` を Git の管理から外す。** `git rm --cached .env` をして、`.gitignore` に `.env` があることを確かめ、コミットします。新しいトークンは `.env` にだけ書きます。
3. **必要なら、履歴から消す。** ただし、消しても「一度漏れた」ことは取り消せません。1のトークンの取り替えを必ず先に済ませます。

### まとめ

- Git は、作業フォルダ → `git add`（ステージ）→ `git commit`（リポジトリ）の順に記録する。最初に名前・メールアドレス・`init.defaultBranch main` を global に設定し、設定の場所は `--show-origin` で確かめる。
- `.gitignore` には、最初のコミットより前に、`/target/`・`*.db`・`.env` を書く。一度コミットした秘密は、外しても履歴に残るので、上げてしまったら、まずトークンを取り替える。
- 改行コードは、リポジトリごとに `.gitattributes`（`* text=auto eol=lf`）で LF に決める。`\r` や `^M` が見えたら、CRLF を疑う。

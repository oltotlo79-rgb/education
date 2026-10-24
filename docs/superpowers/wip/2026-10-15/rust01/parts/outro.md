## この巻のまとめ

この巻では、教材のコードを自分の PC で動かすための土台として、シェル・Git・エディタの準備から始めました。次のチェックリストで、できるようになったことを確かめてください。自信がない項目は、括弧の中の単元に戻って、コマンドを打ち直しましょう。結果が教材と違ったときは、まず「シェルの版・今いるフォルダ・OS・CPU の種類」の4つを比べるのを忘れずに。

**Windows の土台**

- ☐ スタートメニューから Windows Terminal を開き、`$PSVersionTable.PSVersion` で PowerShell の版（5.1）を確かめられる（UNIT 01）
- ☐ `$env:PROCESSOR_ARCHITECTURE` で「このシェルの CPU の種類」を表示し、PC の CPU（ARM64 / x64）と食い違っていないかを確かめられる。x64 用のアプリから起動した PowerShell が `AMD64` になることを知っている（UNIT 01）
- ☐ 作業フォルダ `rust-study` を、短く・半角英数字だけで・OneDrive の外の場所（ホームフォルダの直下か `C:\dev`）に作れる（UNIT 01）
- ☐ `Set-Content -Encoding utf8` とヒア文字列（`@' … '@`）で BOM 付きのスクリプトを作り、`Set-ExecutionPolicy -Scope CurrentUser RemoteSigned` の後に `.\whereami.ps1` で実行できる（UNIT 01）
- ☐ 「スクリプトの実行が無効」（実行ポリシー）・「トークン '{' を使用できません」（BOM なし）・「位置指定パラメーターが見つかりません」（空白を含むパス）のエラーを読んで直せる（UNIT 01）
- ☐ パスが 260 文字に近づくと、`LNK1104` のようにリンカーが失敗することがあり、`LongPathsEnabled` に頼らず短い場所で作業するのが確実だと説明できる（UNIT 01）
- ☐ Git for Windows（ARM64 の PC では ARM64 版）を入れ、`git --version`・`where.exe git`・`git config --system --list` で確かめられる。`where` ではなく `where.exe` と打つ理由を知っている（UNIT 02）
- ☐ `user.name`・`user.email`・`init.defaultBranch main` を global に設定し、設定の段（system・global・local）と `--show-origin` を説明できる。名前とメールアドレスがコミットと一緒に公開されることを知っている（UNIT 02）
- ☐ 作業フォルダ → ステージ → リポジトリの流れを説明し、`git status`・`git add`・`git commit -m` で最初のコミットができる。「Author identity unknown」を直せる（UNIT 02）
- ☐ `.gitignore` に `/target/`・`*.db`・`.env` を書き、`git check-ignore -v` でどの決まりが効いているかを確かめられる（UNIT 02）
- ☐ CRLF と LF の違いと、`LF will be replaced by CRLF` の警告の意味を説明できる。`.gitattributes`（`* text=auto eol=lf`）で改行を決め、`git ls-files --eol` で確かめられる。`$'\r': command not found` を見て改行コードを疑える（UNIT 02）
- ☐ 一度コミットした `.env` が `git rm --cached` の後も履歴に残ることを `git show` で確かめ、上げてしまったら最初にトークンを取り替えると説明できる（UNIT 02）
- ☐ VS Code の Arm64 版を「PATH への追加」を選んで入れ、`code --version` の3行目で CPU の種類を確かめられる。`code --install-extension rust-lang.rust-analyzer` で拡張機能を入れられる（UNIT 03）
- ☐ `cd` してから `code .` で、`Cargo.toml` のあるフォルダそのものを開き、統合ターミナルを使える。「作成者を信頼しますか?」の意味を説明できる（UNIT 03）
- ☐ ユーザーの `settings.json` に `files.eol`・`editor.formatOnSave`・`"[rust]"` の整形の道具・`rust-analyzer.check.command`・`"[powershell]"` の `utf8bom` を書ける。既定 → ユーザー → ワークスペースの順に上書きされることを説明できる（UNIT 03）
- ☐ `.vscode/extensions.json` と `.vscode/settings.json` でプロジェクトの決まりを共有できる。rust-analyzer の「program not found」を `Caused by:` の下から読み、Rust がまだ無いことに気づける（UNIT 03）

> **お知らせ:** この版には UNIT 01〜03 を収録しています。UNIT 04（rustup・rustc・Cargo）・UNIT 05（MSVC Build Tools と CPU の種類）・UNIT 06（Cargo の品質コマンド）・UNIT 07（WSL2 と Docker）・UNIT 08（Topcoat 0.10 と CLI）・UNIT 09（Turso CLI と libSQL の確認）・UNIT 10（flyctl と環境の診断）は準備中で、次の更新から順に追加します。それまでの間、Rust のインストールは公式サイト（`https://www.rust-lang.org/`）の手順で行い、このシリーズの基準（Rust 1.98 以上、動作確認は 1.99.0）を満たしているかを `rustc --version` で確かめてください。

次の第2巻では、この巻でそろえた道具を使って、Rust の文法の土台を学びます。

## 用語ミニ辞典

この巻で出てきた主な用語です。

| 用語 | 読み・意味 | 出てくる単元 |
|---|---|---|
| シェル | 文字で打ったお願い（コマンド）を受け取って、プログラムを動かす係。Windows では PowerShell、Linux では bash | 巻頭・UNIT 01 |
| Windows PowerShell 5.1 | Windows に最初から入っている PowerShell。この本で使う版 | UNIT 01 |
| PowerShell 7 | 別にインストールする新しい PowerShell（コマンド名は `pwsh`）。文字コードの既定などが 5.1 と違う | UNIT 01 |
| Windows Terminal | シェルの窓を開くアプリ。スタートメニューで「ターミナル」と探す | UNIT 01 |
| CPU の種類（アーキテクチャ） | CPU の命令の体系。Windows の PC では x64 と ARM64 が主 | 巻頭・UNIT 01 |
| x64 / AMD64 | Intel や AMD の CPU の体系。Windows では `AMD64` と表示される | UNIT 01 |
| ARM64 | Snapdragon などの CPU の体系 | UNIT 01 |
| エミュレーション | ARM64 の PC で、x64 用のプログラムを通訳しながら動かす仕組み | UNIT 01 |
| ホームフォルダ | `C:\Users\（あなたの名前）`。PowerShell では `~` | UNIT 01 |
| 作業フォルダ | この本の練習をまとめて置くフォルダ。`~\rust-study` | UNIT 01 |
| OneDrive の同期フォルダ | 中のファイルをインターネット上に送り続けるフォルダ。作業フォルダには使わない | UNIT 01 |
| スクリプト | コマンドをファイルに並べて書いたもの。PowerShell では `.ps1` | UNIT 01 |
| ヒア文字列 | `@'` と `'@` ではさんだ、何行でもそのまま1つの文字列になる書き方 | UNIT 01 |
| 実行ポリシー | どんなスクリプトなら実行してよいかを決める PowerShell の設定。この本では `RemoteSigned` | UNIT 01 |
| BOM | ボム。ファイルの先頭の `EF BB BF` の印で、UTF-8 であることを知らせる | UNIT 01・03 |
| Shift_JIS | 日本語の Windows の古い文字コード。PowerShell 5.1 は BOM の無いスクリプトをこれで読む | UNIT 01 |
| MAX_PATH（パスの上限） | 多くの Windows のプログラムが扱えるパスの長さの上限。259 文字まで | UNIT 01 |
| リンカー | 部品をつないで実行ファイルを作る道具。Windows では `link.exe`（UNIT 05） | UNIT 01 |
| `rust-toolchain.toml` | そのフォルダの中で使う Rust の版を決めるファイル | UNIT 01・03 |
| Git | ファイルの変更の記録を残す道具 | UNIT 02 |
| リポジトリ | Git が記録をしまう場所。フォルダの中の `.git` | UNIT 02 |
| ステージ（インデックス） | 次のコミットに入れるものを並べておく台紙。`git add` で載せる | UNIT 02 |
| コミット | ある時点のファイルの状態を、メッセージ・作った人・日時と一緒に記録したもの | UNIT 02 |
| ブランチ | 変更の流れに付ける名前。このシリーズでは `main` | UNIT 02 |
| `.gitignore` | Git に入れないファイルの一覧 | UNIT 02 |
| `.gitattributes` | ファイルの種類ごとの扱い（改行コードなど）の決まり | UNIT 02 |
| 改行コード | 行の終わりを表す見えない文字。CR（`\r`・`0D`）と LF（`\n`・`0A`） | UNIT 02 |
| CRLF / LF | Windows の流儀の改行（CR+LF）と、Linux・macOS の流儀の改行（LF） | UNIT 02 |
| `core.autocrlf` | Git が改行を自動で変換する設定。Git for Windows の既定は `true` | UNIT 02 |
| `.env` / `.env.example` | 秘密の値を書く環境変数のファイルと、その見本。Git に入れるのは見本だけ | UNIT 02 |
| winget | ウィンゲット。Windows に入っている、コマンドでアプリを入れる道具 | UNIT 02 |
| VS Code | Visual Studio Code。この本で使うコードエディタ | UNIT 03 |
| 統合ターミナル | VS Code の中で開くターミナル。開いたフォルダから始まる | UNIT 03 |
| 拡張機能 | VS Code にあとから足す部品 | UNIT 03 |
| rust-analyzer | ラスト・アナライザー。Rust の補完・型の表示・チェックをする拡張機能 | UNIT 03 |
| コマンド パレット | VS Code の操作を名前で探して実行する入力欄。`Ctrl + Shift + P` | UNIT 03 |
| `settings.json` | VS Code の設定のファイル。ユーザーの設定とワークスペースの設定がある | UNIT 03 |
| ワークスペースの設定 | そのフォルダだけに効く設定。`.vscode/settings.json` | UNIT 03 |
| 制限モード | 信頼しなかったフォルダを開いたときの、拡張機能などを止めた状態 | UNIT 03 |
| 整形（フォーマット） | コードの字下げや空白を、決まった形に直すこと。Rust では rustfmt | UNIT 03 |
| clippy | クリッピー。Rust のよくある書き間違いや、もったいない書き方を指摘する道具 | UNIT 03 |

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

**UNIT 04: rustup・rustc・Cargo**

- ☐ rustup（版の管理）・rustc（コンパイラ）・cargo（ビルドと部品の段取り）の役目の違いと、`.cargo\bin` の受付が版を選ぶしくみを説明できる（UNIT 04）
- ☐ `https://rustup.rs/` から CPU に合った `rustup-init.exe` を入れ、新しい窓で `rustc --version`・`rustup show` を確かめられる（UNIT 04）
- ☐ `cargo new`・`cargo run` で最初のプログラムを動かし、`Cargo.toml`・`src\main.rs`・`target\debug` の役目を言える（UNIT 04）
- ☐ `rust-toolchain.toml` でフォルダごとに版を決め、`rustup install`・`rustup update` で版を入れたり新しくしたりできる。自動で入れる動きが非推奨なことを知っている（UNIT 04）

**UNIT 05: MSVC Build Tools と CPU の種類**

- ☐ Windows の Rust が、MSVC の `link.exe` で実行ファイルを綴じることと、VS Code だけでは足りないことを説明できる（UNIT 05）
- ☐ `vswhere.exe` と MSVC のフォルダで、Build Tools の名前・版・ARM64 用の道具（`Hostarm64`）を確かめられる（UNIT 05）
- ☐ 実行ファイルの印（ARM64 は `0xAA64`、x64 は `0x8664`）を読み、x64 用の道具がエミュレーションで動いていることに気づける（UNIT 05）
- ☐ `rustup target add` と `--target` で別の CPU 用の実行ファイルを作れる。「can't find crate for std」「linker not found」を読んで直せる（UNIT 05）

**UNIT 06: Cargo の品質コマンド**

- ☐ `cargo check`（型と文法）・`cargo fmt`（体裁）・`cargo clippy`（書き方の注意）・`cargo test`（答え合わせ）を、目的ごとに使い分けられる（UNIT 06）
- ☐ `cargo fmt --check` の差分、clippy の注意（`-->` の場所と `help:`）、テストの `left`・`right` を読んで、コードを直せる（UNIT 06）
- ☐ `-D warnings` で注意を失敗にでき、終了コード（0・1・101）を `$LASTEXITCODE` で確かめられる（UNIT 06）
- ☐ `'cargo-fmt.exe' is not installed` を読み、`rustup component add rustfmt clippy` で足せる（UNIT 06）

**UNIT 07: WSL2 と Docker**

- ☐ `wsl --install` で WSL2 と Ubuntu を入れ、`wsl --version`・`wsl -l -v` で確かめられる。文字化けしたら `$env:WSL_UTF8 = "1"` を付けられる（UNIT 07）
- ☐ Windows の `C:\` が Linux から `/mnt/c`、Linux の `~` が Windows から `\wsl.localhost\Ubuntu\home\…` に見えることを説明できる（UNIT 07）
- ☐ Linux の作業のファイルを Linux 側に置く理由を、測った数（小さなファイル 2000 個で約 3.6 秒と 0.1 秒）で言える（UNIT 07）
- ☐ Docker Desktop で `hello-world` を動かし、コンテナの CPU の種類（arm64・amd64）を確かめられる。「port is already allocated」を直せる（UNIT 07）

**UNIT 08: Topcoat 0.11 と CLI**

- ☐ `cargo install topcoat-cli --version 0.11.0 --locked` で CLI を入れ、`topcoat --version` で確かめられる。アプリの Topcoat と CLI の版をそろえる理由を言える（UNIT 08）
- ☐ `topcoat new` で新しいアプリを作り、`Cargo.toml`・`src\app.rs`・`styles.css` の役目を言える（UNIT 08）
- ☐ `cargo run` だけでは「no asset bundle」で止まる理由を説明し、`topcoat asset bundle` か `topcoat dev` で動かせる（UNIT 08）
- ☐ `requires rustc 1.99`（古い Rust）と `AddrInUse`・10048（使われているポート）のエラーを読んで直せる（UNIT 08）

**UNIT 09: Turso CLI と libSQL の確認**

- ☐ libSQL・sqld・Turso・Turso CLI の関係と、Turso CLI を WSL の中で `curl -sSfL https://get.tur.so/install.sh | bash` で入れる手順と、スクリプトがすることを説明できる（UNIT 09）
- ☐ Docker で libSQL のサーバー（`libsql-server`）を動かし、RustNote を `DATABASE_URL=http://127.0.0.1:8080` でつないで、`/health`・登録・SQL の結果を確かめられる（UNIT 09）
- ☐ Windows の開発用ビルドでリモートの DB につなぐと main のスタックがあふれることを知り、`cargo run --release` か `/STACK` で動かせる（UNIT 09）
- ☐ `http://` を忘れるとファイル（の別の流れ）に DB ができること、`libsql://` でトークンが要ることを読んで直せる（UNIT 09）

**UNIT 10: flyctl と環境の診断**

- ☐ flyctl を公式の `iwr https://fly.io/install.ps1 -useb | iex` で入れ、`fly version` で版と CPU の種類（`windows/amd64`）を確かめられる（UNIT 10）
- ☐ `fly version` はアカウント不要、`fly auth login`・`whoami`・`logout`・`signup` はアカウントにつながることを区別できる。トークンが `.fly\config.yml` に保存されることを知っている（UNIT 10）
- ☐ `doctor.ps1` で、この巻の道具を順に確かめ、NG の行の括弧の単元に戻れる。`-File` には `$HOME` を使い、ふつうの PowerShell で動かす（UNIT 10）

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
| rustup | ラストアップ。Rust の版（ツールチェーン）を入れたり選んだりする道具 | UNIT 04 |
| rustc | Rust のコンパイラ。ソースコードを機械語にする | UNIT 04 |
| cargo | カーゴ。Rust のビルドと部品（クレート）の管理をする道具 | UNIT 04 |
| ツールチェーン | ある版の Rust の道具一式。`stable-aarch64-pc-windows-msvc` のような名前 | UNIT 04 |
| stable | いちばん新しい安定版。`rustup update` で新しくなる | UNIT 04 |
| `Cargo.toml` | プロジェクトの表紙。名前・版・使う部品を書く | UNIT 04 |
| `target` フォルダ | cargo がビルドした物を置くフォルダ。Git には入れない | UNIT 04 |
| MSVC | Microsoft Visual C++。Windows で実行ファイルを作る道具一式。リンカー `link.exe` が入っている | UNIT 05 |
| Build Tools for Visual Studio | Visual Studio を開かずに、MSVC の道具だけを入れる製品 | UNIT 05 |
| vswhere.exe | 入っている Visual Studio・Build Tools を答える道具 | UNIT 05 |
| target | 作る実行ファイルが、どの CPU・OS 用か。`aarch64-pc-windows-msvc` など | UNIT 05 |
| host | rustc が動いている PC の target。`rustc -vV` の `host:` | UNIT 05 |
| `0xAA64`・`0x8664` | 実行ファイルの中の、CPU の種類の印。ARM64 と x64 | UNIT 05 |
| `cargo check` | 型と文法だけを確かめる。実行ファイルは作らないので速い | UNIT 06 |
| `cargo fmt` | rustfmt で、字下げや空白を決まった形に整える。`--check` は差分を見せるだけ | UNIT 06 |
| `cargo clippy` | よくある書き方の注意を出す。`-- -D warnings` で注意をエラーにする | UNIT 06 |
| `cargo test` | テストを動かして、結果が期待どおりかを確かめる | UNIT 06 |
| 終了コード | コマンドが終わるときに返す数。0 は成功。PowerShell では `$LASTEXITCODE` | UNIT 06 |
| `--profile minimal` | rustup で、rustc・cargo・標準ライブラリだけを入れる組み合わせ | UNIT 06 |
| WSL2 | Windows の中で、本物の Linux を動かすしくみ | UNIT 07 |
| ディストリビューション | Linux の種類。この本では Ubuntu | UNIT 07 |
| `/mnt/c` | Linux から見た、Windows の C ドライブ | UNIT 07 |
| `\wsl.localhost` | Windows から見た、WSL の Linux のフォルダの入口 | UNIT 07 |
| コンテナ | アプリと、その動きに要る物をまとめて、切り分けて動かす箱 | UNIT 07 |
| イメージ | コンテナの設計図。Docker Hub などから取ってくる | UNIT 07 |
| Docker Desktop | Windows でコンテナを動かすアプリ。WSL2 の上で動く | UNIT 07 |
| Topcoat | Rust で Web アプリを作るライブラリ。このシリーズは 0.11.0 | UNIT 08 |
| topcoat-cli | Topcoat の CLI（`topcoat` コマンド）のパッケージ名 | UNIT 08 |
| `cargo install` | crates.io からソースを取ってきてビルドし、`.cargo\bin` に置く | UNIT 08 |
| `--locked` | 作った人が確かめた `Cargo.lock` のとおりの版で作る指定 | UNIT 08 |
| アセットの束 | CSS などを、指紋付きの名前でまとめたもの。`target\debug\assets` | UNIT 08 |
| `topcoat dev` | ビルド・束・起動をまとめてして、保存のたびにやり直す開発用のサーバー | UNIT 08 |
| libSQL | SQLite をもとにした DB。ファイルとしても、サーバー越しにも使える | UNIT 09 |
| sqld | libSQL のサーバーのプログラム。HTTP で SQL を受け取る | UNIT 09 |
| Turso | libSQL の DB を貸すクラウドのサービス | UNIT 09 |
| Turso CLI | Turso を操作するコマンド（`turso`）。Windows 版は無く、WSL の中で使う | UNIT 09 |
| `DATABASE_URL` | RustNote の DB のつなぎ先。ファイルの名前・`http://`・`libsql://` | UNIT 09 |
| スタック | 関数を呼ぶたびに一時的な値を置く作業台。Windows の main は既定で 1 MB | UNIT 09 |
| 代替データストリーム | NTFS で、1つのファイルの中に持てる別の流れ。`名前:流れ` と書く | UNIT 09 |
| flyctl | Fly.io を操作するコマンド。`fly` と打っても同じ | UNIT 10 |
| `fly auth` | ログインまわりのコマンド。login・whoami・logout・signup | UNIT 10 |
| `.fly\config.yml` | ログインしたときのトークンが保存されるファイル。人に見せない | UNIT 10 |
| 環境の診断 | 道具がそろっているか・版が合っているかを、決まった順番で確かめること | UNIT 10 |
| `$HOME` | ホームフォルダ。PowerShell が先に `C:\Users\（名前）` に直す | UNIT 10 |

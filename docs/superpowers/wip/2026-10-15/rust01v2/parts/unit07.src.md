## UNIT 07: WSL2 と Docker

### この単元のゴール

- WSL2 と Ubuntu が入っているか・版はいくつかを、`wsl --version`・`wsl -l -v` で確かめられる
- Windows のフォルダと Linux のフォルダの対応（`/mnt/c`・`\\wsl.localhost`）を説明でき、Linux の作業のファイルを Linux 側に置く理由を、測った数で言える
- Docker Desktop で `hello-world` を動かし、コンテナの CPU の種類（arm64・amd64）を確かめられる
- 「ディストリビューションはありません」「port is already allocated」を読んで直せる

### たとえ話でつかむ

**一軒の家の中の、離れ**を考えます。

- Windows の家の庭に、**Linux の離れ**（WSL2）を建てます。離れの中では、Linux の道具が、そのまま使えます。Turso CLI のように、Windows 版が無い道具も、離れの中なら動きます（UNIT 09）。
- 母屋（Windows）と離れ（Linux）は、**渡り廊下**（`/mnt/c` と `\\wsl.localhost`）でつながっています。離れから母屋の部屋へも行けますが、渡り廊下は遠回りなので、行き来が多い仕事は遅くなります。**離れの仕事の書類は、離れに置く**のが速いです。
- **お弁当箱**（Docker のコンテナ）は、中身（アプリと、その動きに要る物全部）を詰めて、ふたをした箱です。どの家に持って行っても、同じように食べられます（同じように動きます）。Docker Desktop は、お弁当箱を温める電子レンジで、離れの仕組み（WSL2）の上で動いています。
- お弁当箱にも、**言葉の版**（arm64・amd64）があります。違う版の箱は、通訳（エミュレーション）を通して食べます（UNIT 05 と同じ）。

| 家のたとえ | この単元の言葉 |
|---|---|
| Linux の離れ | WSL2 と Ubuntu |
| 渡り廊下 | `/mnt/c`（Linux から Windows へ）・`\\wsl.localhost`（Windows から Linux へ） |
| 離れの書類は離れに置く | Linux の作業のファイルは、Linux 側（`~`）に置く（例7-3） |
| お弁当箱 | コンテナ（イメージから作る） |
| 電子レンジ | Docker Desktop |
| 箱の言葉の版 | イメージの CPU の種類（`linux/arm64`・`linux/amd64`） |

> **用語:** WSL2（Windows Subsystem for Linux 2：Windows の中で、本物の Linux を動かすしくみ。この本では Ubuntu を入れる）

<!-- -->

> **用語:** コンテナ（container：アプリと、その動きに要る物をまとめて、ほかから切り分けて動かす箱。箱の設計図を**イメージ**と呼ぶ）

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-071" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="20" width="300" height="260" rx="10" fill="#e0f2fe" stroke="#0284c7" stroke-width="1.5"/>
<text x="170" y="46" font-size="14" font-weight="700" fill="#0f172a" text-anchor="middle">Windows（母屋）</text>
<text x="40" y="76" font-size="12" fill="#0f172a">PowerShell・VS Code・rustc</text>
<text x="40" y="100" font-size="12" fill="#0f172a" font-family="Consolas, Yu Gothic UI, monospace">C:\Users\（名前）\rust-study</text>
<rect x="40" y="200" width="260" height="62" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.2"/>
<text x="170" y="224" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">Docker Desktop</text>
<text x="170" y="246" font-size="11" fill="#475569" text-anchor="middle">docker コマンド（Windows 用）</text>
<rect x="400" y="20" width="300" height="260" rx="10" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
<text x="550" y="46" font-size="14" font-weight="700" fill="#0f172a" text-anchor="middle">WSL2 の Linux（離れ）</text>
<text x="420" y="76" font-size="12" fill="#0f172a">Ubuntu 24.04（aarch64）</text>
<text x="420" y="100" font-size="12" fill="#0f172a">Linux 側のフォルダ: /home/（名前）</text>
<rect x="420" y="200" width="260" height="62" rx="8" fill="#fef3c7" stroke="#d97706" stroke-width="1.2"/>
<text x="550" y="224" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">docker-desktop（別の Linux）</text>
<text x="550" y="246" font-size="11" fill="#b45309" text-anchor="middle">ここでコンテナが動く</text>
<line x1="322" y1="120" x2="396" y2="120" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-071)"/>
<text x="360" y="112" font-size="11" fill="#475569" text-anchor="middle">wsl</text>
<line x1="398" y1="150" x2="324" y2="150" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-071)"/>
<text x="360" y="172" font-size="11" fill="#475569" text-anchor="middle">/mnt/c で</text>
<text x="360" y="188" font-size="11" fill="#475569" text-anchor="middle">Windows 側へ</text>
<line x1="302" y1="231" x2="416" y2="231" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-071)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: Windows と WSL2 と Docker。Linux 側から Windows のフォルダは /mnt/c で見えるが、行き来は遅い</div>
</div>

### まず動かす最小コード

#### 例7-1: WSL2 と Ubuntu を確かめる

> **▼ この手順がやること（先に日本語で）:** WSL の版と、入っている Linux の一覧を表示します。そのあと、Ubuntu の中でコマンドを1つずつ動かし、CPU の種類・Ubuntu の版・CPU の数・メモリを見ます。

**入れ方（まだ入っていないとき）:** Microsoft の公式のドキュメントでは、**管理者として開いた** PowerShell で、次のコマンドを打ち、PC を再起動するように案内されています（2026-10-31 に確かめたもの）。既定で Ubuntu が入ります。初めて Ubuntu を開いたときに、Linux 側のユーザー名とパスワードを決めます。

```powershell
wsl --install
```

この本の PC には、WSL2 と Ubuntu 24.04 が入っていました。

```powershell
# wsl の表示が文字化けしないようにする
$env:WSL_UTF8 = "1"
wsl --version
wsl -l -v
wsl -d Ubuntu -- uname -m
wsl -d Ubuntu -- bash -c 'head -4 /etc/os-release; nproc; free -h | head -2'
```

**▼ 実行結果**

```text
@@OUT:u07_wsl_version@@
```

```text
@@OUT:u07_wsl_list@@
```

```text
@@OUT:u07_ubuntu@@
```

#### ▼ 1つずつ分解して解説

##### 解説1: `wsl -l -v` の3つ

- `Ubuntu` が、この本で使う Linux です。`*` は既定（`wsl` とだけ打ったときに開く物）、`VERSION` の `2` は WSL2 で動いているという意味です。
- `docker-desktop` と `docker-desktop-data` は、Docker Desktop が自分で使うための Linux です。自分で開いて使うことはありません（図の黄色）。

##### 解説2: Ubuntu の中を見る

- `wsl -d Ubuntu -- コマンド` で、PowerShell から Ubuntu の中のコマンドを1つ動かせます。`--` のうしろが Linux のコマンドです。
- `uname -m` は `aarch64` でした。Windows が ARM64 なので、Linux も ARM64 で動いています。
- CPU は 12 個、メモリは 15Gi が見えました。WSL2 は、PC のメモリの一部を Linux に分けて使います。

##### 解説3: `WSL_UTF8` を付ける理由

- `wsl.exe` は、表示を UTF-16 という文字コードで出すので、PowerShell の設定によっては文字化けします。`$env:WSL_UTF8 = "1"` を付けると UTF-8 で出すので、日本語も読めます（この窓の中だけに効きます。UNIT 01）。

#### 例7-2: Windows と Linux の、フォルダの対応

> **▼ この手順がやること（先に日本語で）:** `rust-study` から Ubuntu のコマンドを動かし、Linux から見た「今いるフォルダ」を表示します。次に、Linux のホームフォルダと、それを Windows から見たときの名前を表示します。

```powershell
cd ~\rust-study
wsl -d Ubuntu -- pwd
wsl -d Ubuntu -- bash -c 'cd ~ && pwd && wslpath -w ~'
```

**▼ 実行結果**

```text
@@OUT:u07_paths@@
```

- Windows の `C:\Users\（名前）\rust-study` は、Linux からは `/mnt/c/Users/（名前）/rust-study` に見えます。`/mnt/c` が、Windows の C ドライブへの渡り廊下です。
- Linux のホームフォルダ `/home/（Linux のユーザー名）` は、Windows からは `\\wsl.localhost\Ubuntu\home\（Linux のユーザー名）` で見えます。エクスプローラーのアドレス欄に打つと、開けます。`wslpath -w` は、Linux のパスを Windows の書き方に直す道具です。

### もう一歩: 実務に近い形

#### 例7-3: Windows 側と Linux 側で、ファイルを作る速さを比べる

> **▼ この手順がやること（先に日本語で）:** 小さなファイルを 2000 個作る時間を、Windows 側のフォルダ（`/mnt/c/…/rust-study`）と、Linux 側のホームフォルダ（`~`）で比べるスクリプトを `rust-study` に置き、Ubuntu の中で3回動かします。

`speed.sh`（**改行は LF** で保存します。CRLF だと Linux の bash で動きません。UNIT 02・03）:

```bash
@@FILE:u07_speed.sh@@
```

```powershell
cd ~\rust-study
wsl -d Ubuntu -- bash speed.sh
wsl -d Ubuntu -- bash speed.sh
wsl -d Ubuntu -- bash speed.sh
```

**▼ 実行結果**

```text
@@OUT:u07_speed_all@@
```

- Windows 側のフォルダでは約 3.6 秒、Linux 側では 0.1 秒前後でした。**30 倍以上**の差です。Linux から `/mnt/c` のファイルを触るたびに、渡り廊下を通るからです。
- ビルドのように、小さなファイルをたくさん読み書きする仕事を Linux でするときは、プロジェクトを Linux 側（`~` の下）に置きます。逆に、Windows の道具（rustc・VS Code）で作業するプロジェクトは、Windows 側に置きます。**道具と同じ側に置く**のが決まりです。

#### 例7-4: Docker でコンテナを動かす

> **▼ この手順がやること（先に日本語で）:** Docker の版を表示してから、練習用のイメージ `hello-world` を動かします。次に、Debian のイメージで `uname -m` を動かし、コンテナの CPU の種類を見ます。最後に、x64 用（`linux/amd64`）を指定して、同じことをします。

**入れ方（まだ入っていないとき）:** Docker の公式サイトから Docker Desktop のインストーラーを入手して入れ、スタートメニューから Docker Desktop を起動します（入れたあと、自動では起動しません、と公式のドキュメントにあります）。公式のページの Arm 版（早期アクセス）の必要条件には、Windows の Enterprise・Pro・Education が挙げられていますが、この本の PC（Windows 11 Home の ARM64）でも、Docker Desktop 4.68.0 は動きました。動かないときは、公式のページの条件を確かめてください。また、大きな会社（従業員 250 人より多い、または売り上げが年 1,000 万ドルより多い）で仕事に使うときは、有料の契約が要ります（2026-10-31 に公式のページで確かめたもの）。

```powershell
docker version --format 'Client: {{.Client.Version}} {{.Client.Os}}/{{.Client.Arch}}'
docker version --format 'Server: {{.Server.Version}} {{.Server.Os}}/{{.Server.Arch}}'
docker run --rm hello-world
```

**▼ 実行結果**

```text
@@OUT:u07_docker_version@@
```

```text
@@OUT:u07_hello_world@@
```

```powershell
docker run --rm debian:trixie-slim uname -m
docker run --rm --platform linux/amd64 debian:trixie-slim uname -m
docker image ls --format '{{.Repository}}:{{.Tag}} {{.Size}}' hello-world
```

**▼ 実行結果**

```text
@@OUT:u07_uname_native@@
```

```text
@@OUT:u07_uname_amd64@@
```

```text
@@OUT:u07_images@@
```

#### ▼ 1つずつ分解して解説

##### 解説1: Client と Server

- `docker` コマンド（Client）は Windows 用（`windows/arm64`）、コンテナを動かす係（Server）は Linux 用（`linux/arm64`）でした。Server は、`docker-desktop` の Linux の中で動いています（図）。
- `hello-world` の説明のとおり、Client が Server に頼み、Server がイメージを Docker Hub から取ってきて（`Pulling`）、コンテナを作って動かし、表示を Client に送り返しました。2. の行の `(arm64v8)` は、ARM64 用のイメージが選ばれたという意味です。`--rm` は、終わったコンテナを自動で消す指定です。

##### 解説2: arm64 と amd64

- ふつうに動かすと `aarch64`、`--platform linux/amd64` を付けると `x86_64` でした。x64 用のコンテナも、通訳を通して動きます。ただし、遅くなることがあるので、ふだんは PC と同じ arm64 のイメージを使います。
- 第12巻で RustNote のイメージを作ったときも、この PC では `linux/arm64` のイメージができました。

##### 解説3: WSL の中からも docker が使える

```powershell
wsl -d Ubuntu -- bash -lc 'command -v docker && docker version --format {{.Server.Version}}'
```

```text
@@OUT:u07_wsl_docker@@
```

- Docker Desktop の設定で、Ubuntu との連携（WSL integration）が有効になっていると、Ubuntu の中でも `docker` コマンドが使えます。この本の PC では有効でした。つながる先は、同じ Docker Desktop の Server です。

### よくあるエラーと読み方

#### エラー1: 「指定された名前のディストリビューションはありません」

```powershell
wsl -d Ubunt -- uname -m
```

```text
@@OUT:u07_wsl_typo@@
```

- **何が起きたか:** `-d` のうしろの名前（`Ubunt`）の Linux がありませんでした。名前の打ち間違いです。
- **直し方:** `wsl -l -v` で、入っている名前を確かめます。名前は `Ubuntu` のように、一覧の `NAME` のとおりに打ちます。

#### エラー2: `port is already allocated`（ポートが、もう使われている）

コンテナを2つ、同じポート 18099 で動かそうとしました。

```powershell
docker run -d --name u07a -p 18099:80 debian:trixie-slim sleep 60 | Out-Null
docker run -d --name u07b -p 18099:80 debian:trixie-slim sleep 60
```

```text
@@OUT:u07_port_busy@@
```

- **何が起きたか:** `-p 18099:80` は「PC のポート 18099 を、コンテナの 80 につなぐ」という指定です。1つ目のコンテナ（`u07a`）が、もう 18099 を使っていたので、2つ目は `Bind for 0.0.0.0:18099 failed: port is already allocated` で作れませんでした（1行目は、1つ目のコンテナの ID です）。
- **直し方:** 使っていないポートにするか（`-p 18100:80`）、前のコンテナを止めます（`docker rm -f u07a`）。`docker ps` で、動いているコンテナと、使っているポートが分かります。

### RustNote ではここで使う

- RustNote の Dockerfile（第12巻 UNIT 02）は、Docker Desktop でビルドして、`linux/arm64` のイメージになりました。手元で `docker run -p 18080:8080 …` のように動かして確かめます。
- Turso CLI（UNIT 09）は Windows 版が無いので、WSL の Ubuntu に入れます。
- RustNote のプロジェクトは、Windows の道具（rustc・VS Code）で作業するので、Windows 側のフォルダに置きます（例7-3）。

### 練習問題

**問1:** Ubuntu の中で `cd /mnt/c/Users/（名前）/rust-study/hello && cargo build` のようにビルドすると、遅くなりそうなのはなぜでしょうか。

**問2:** エクスプローラーで Ubuntu のホームフォルダを開くには、アドレス欄に何と打てばよいでしょうか。

**問3:** `docker run --rm --platform linux/amd64 …` のコンテナが、この PC で動いたのに、遅く感じました。なぜでしょうか。

### 解答と解説

**問1の解答:**

- `/mnt/c` は、Linux から Windows のフォルダへの渡り廊下で、ファイルを触るたびに行き来が起きるからです。例7-3では、小さなファイル 2000 個で、Linux 側の 30 倍以上の時間がかかりました。ビルドは小さなファイルをたくさん読み書きするので、Linux でビルドするなら、プロジェクトを Linux 側に置きます。

**問2の解答:**

- 例7-2のとおり、`\\wsl.localhost\Ubuntu\home\（Linux のユーザー名）` です。Ubuntu の中で `wslpath -w ~` を打つと、この名前が表示されます。

**問3の解答:**

- この PC は ARM64 なので、`linux/amd64`（x64 用）のコンテナは、通訳（エミュレーション）を通して動くからです（UNIT 05 と同じ）。ふだんは、PC と同じ `linux/arm64` のイメージを使います。

### まとめ

- WSL2 は Windows の中の Linux。`wsl --install` で入れ、`wsl --version`・`wsl -l -v` で確かめる。この本の PC は WSL 2.6.3.0・Ubuntu 24.04.1（aarch64）。表示が化けるときは `$env:WSL_UTF8 = "1"`。
- Windows の `C:\` は Linux から `/mnt/c`、Linux の `~` は Windows から `\\wsl.localhost\Ubuntu\home\…`。行き来は遅く、小さなファイル 2000 個で約 3.6 秒と 0.1 秒の差。道具と同じ側にファイルを置く。
- Docker Desktop 4.68.0（Client は `windows/arm64`、Server は `linux/arm64`）で `hello-world` が動いた。`--platform linux/amd64` で x64 用も動くが、通訳になる。
- 名前の打ち間違いは「ディストリビューションはありません」、同じポートは `port is already allocated`。

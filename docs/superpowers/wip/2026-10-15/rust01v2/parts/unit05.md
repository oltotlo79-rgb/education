## UNIT 05: MSVC Build Tools と CPU の種類

### この単元のゴール

- Windows で Rust のプログラムを作るのに、MSVC のリンカー（`link.exe`）が要る理由を説明できる
- Build Tools が入っているか・どの版か・ARM64 用の道具があるかを、`vswhere.exe` とフォルダで確かめられる
- 実行ファイルがどの CPU 用かを読み分け、ARM64 の PC の中で、x64 用の道具がエミュレーションで動いていることに気づける
- target（作る実行ファイルの CPU と OS）を足して、別の CPU 用の実行ファイルを作れる。「can't find crate for `std`」「linker not found」を読んで直せる

### たとえ話でつかむ

**本の印刷所と製本所**を考えます。

- `rustc` は**印刷所**です。原稿（ソースコード）を読んで、ページ（機械語の部品）を刷ります。
- ページだけでは本になりません。**製本所**（リンカー `link.exe`）が、刷ったページと、Windows が用意している決まりの部品（`kernel32.lib` などの付録）を綴じて、1冊の本（`.exe`）にします。
- Windows の製本所は、Microsoft の **Build Tools**（MSVC）の中にあります。Rust を入れただけでは、製本所はありません（UNIT 04 の図の `link.exe`）。
- 本には**言葉の版**があります。ARM64 の言葉の本と、x64 の言葉の本です。ARM64 の PC は ARM64 の本を、そのまま読めます。x64 の本は、**通訳**（エミュレーション）を通して読むので、少し遅くなります。
- 印刷所に「x64 の言葉の本も刷って」と頼むには、x64 用の**活字のセット**（target の標準ライブラリ）を足します。

| 印刷と製本 | Rust と Windows |
|---|---|
| 印刷所 | `rustc` |
| 製本所 | `link.exe`（MSVC のリンカー） |
| 決まりの付録 | `kernel32.lib` などの Windows の部品（Windows SDK） |
| 言葉の版 | CPU の種類（ARM64・x64） |
| 通訳 | エミュレーション（UNIT 01） |
| 活字のセット | target の標準ライブラリ（`rustup target add`） |

> **用語:** MSVC（エム・エス・ブイ・シー。Microsoft Visual C++：Microsoft の C/C++ の道具一式。リンカー `link.exe` が入っている。Rust の Windows 版は、これを使って実行ファイルを作る）

<!-- -->

> **用語:** target（ターゲット：作る実行ファイルが、どの CPU・どの OS 用か。`aarch64-pc-windows-msvc` のような名前で表す）

### 図で理解する

`cargo build` の最後に、リンカーが実行ファイルを綴じます。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-051" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="30" width="140" height="70" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text font-family="Consolas, Yu Gothic UI, monospace" x="90" y="60" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">src\main.rs</text>
<text x="90" y="82" font-size="11" fill="#475569" text-anchor="middle">原稿</text>
<rect x="200" y="30" width="140" height="70" rx="8" fill="#e0f2fe" stroke="#0284c7" stroke-width="1.5"/>
<text x="270" y="60" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">rustc</text>
<text x="270" y="82" font-size="11" fill="#0369a1" text-anchor="middle">Rust に入っている</text>
<rect x="380" y="30" width="140" height="70" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="450" y="60" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">機械語の部品</text>
<text x="450" y="82" font-size="11" fill="#475569" text-anchor="middle">（.o・.rlib）</text>
<rect x="380" y="130" width="140" height="70" rx="8" fill="#fef3c7" stroke="#d97706" stroke-width="1.5"/>
<text x="450" y="158" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">kernel32.lib など</text>
<text x="450" y="180" font-size="11" fill="#b45309" text-anchor="middle">Windows SDK</text>
<rect x="560" y="80" width="140" height="70" rx="8" fill="#fef3c7" stroke="#d97706" stroke-width="1.5"/>
<text x="630" y="110" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">link.exe</text>
<text x="630" y="132" font-size="11" fill="#b45309" text-anchor="middle">Build Tools（MSVC）</text>
<line x1="162" y1="65" x2="196" y2="65" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-051)"/>
<line x1="342" y1="65" x2="376" y2="65" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-051)"/>
<line x1="522" y1="70" x2="556" y2="100" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-051)"/>
<line x1="522" y1="160" x2="556" y2="132" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-051)"/>
<line x1="630" y1="152" x2="630" y2="196" stroke="#475569" stroke-width="1.8" marker-end="url(#ar-01-051)"/>
<rect x="560" y="200" width="140" height="40" rx="8" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
<text x="630" y="225" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">hello.exe</text>
<text x="20" y="150" font-size="11" fill="#475569">黄色の2つは、Rust を入れても入らない。</text>
<text x="20" y="170" font-size="11" fill="#475569">Visual Studio の Build Tools で入れる。</text>
<text x="20" y="196" font-size="11" fill="#475569">VS Code（UNIT 03）は別の製品なので、</text>
<text x="20" y="216" font-size="11" fill="#475569">VS Code だけでは足りない。</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: Windows で実行ファイルができるまで。rustc が刷った部品を、MSVC の link.exe が Windows の部品と一緒に綴じる</div>
</div>

target の名前は、4つの部分に分かれています。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 200" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<text x="360" y="44" font-size="22" font-weight="700" fill="#0f172a" text-anchor="middle" font-family="Consolas, monospace">aarch64 - pc - windows - msvc</text>
<rect x="60" y="70" width="180" height="110" rx="8" fill="#e0f2fe" stroke="#0284c7" stroke-width="1.5"/>
<text x="150" y="94" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">aarch64</text>
<text x="150" y="118" font-size="11" fill="#0f172a" text-anchor="middle">CPU の種類</text>
<text x="150" y="140" font-size="11" fill="#0369a1" text-anchor="middle">ARM64 のこと</text>
<text x="150" y="160" font-size="11" fill="#0369a1" text-anchor="middle">x64 なら x86_64</text>
<rect x="260" y="70" width="110" height="110" rx="8" fill="#f8fafc" stroke="#64748b" stroke-width="1.5"/>
<text x="315" y="94" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">pc</text>
<text x="315" y="118" font-size="11" fill="#0f172a" text-anchor="middle">作った会社など</text>
<text x="315" y="140" font-size="11" fill="#475569" text-anchor="middle">（ここは気に</text>
<text x="315" y="160" font-size="11" fill="#475569" text-anchor="middle">しなくてよい）</text>
<rect x="390" y="70" width="120" height="110" rx="8" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
<text x="450" y="94" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">windows</text>
<text x="450" y="118" font-size="11" fill="#0f172a" text-anchor="middle">OS</text>
<text x="450" y="140" font-size="11" fill="#15803d" text-anchor="middle">Linux なら</text>
<text x="450" y="160" font-size="11" fill="#15803d" text-anchor="middle">unknown-linux</text>
<rect x="530" y="70" width="130" height="110" rx="8" fill="#fef3c7" stroke="#d97706" stroke-width="1.5"/>
<text x="595" y="94" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">msvc</text>
<text x="595" y="118" font-size="11" fill="#0f172a" text-anchor="middle">使う道具の種類</text>
<text x="595" y="140" font-size="11" fill="#b45309" text-anchor="middle">MSVC の link.exe</text>
<text x="595" y="160" font-size="11" fill="#b45309" text-anchor="middle">で綴じる</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: target の名前の読み方。この本の PC の host（rustc が動く PC）は aarch64-pc-windows-msvc</div>
</div>

### まず動かす最小コード

#### 例5-1: Build Tools が入っているかを確かめる

> **▼ この手順がやること（先に日本語で）:** Visual Studio の道具を探す `vswhere.exe` で、入っている Build Tools の名前と版を表示します。MSVC の道具のフォルダを開いて、ARM64 の PC で動く道具（`Hostarm64`）の中に、どの CPU 用の道具があるかを見ます。

**入れ方（まだ入っていないとき）:** Rust の公式のドキュメント（rustup の「Windows の MSVC の準備」）では、次の2つの方法が案内されています（2026-10-31 に確かめたもの）。

- `rustup-init.exe` を動かしたときに Visual Studio が見つからないと、rustup が準備（Visual Studio Community）を入れるかを聞いてくれます。
- 自分で入れるときは、Visual Studio Installer で「**C++ によるデスクトップ開発**」（Desktop Development with C++）を選びます。ARM64 の PC では、「MSVC の ARM64 用のビルドツール」も選びます。winget で入れるときのコマンドも、そのドキュメントにあります。Build Tools（Visual Studio を開かずに道具だけを入れる版）を使えるのは、Visual Studio のライセンスを持っている場合です、とドキュメントにあります。

この本の PC には、**Visual Studio Build Tools 2022** が入っていました。

```powershell
$vs = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
& $vs -products * -property displayName
& $vs -products * -property installationVersion
$msvc = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\"
Get-ChildItem $msvc -Name
Get-ChildItem "$msvc\14.44.35207\bin\Hostarm64" -Name
Test-Path "$msvc\14.44.35207\bin\Hostarm64\arm64\link.exe"
```

**▼ 実行結果**

```text
Visual Studio Build Tools 2022
17.14.37502.11
14.44.35207
arm64
x64
x86
True
```

#### ▼ 1つずつ分解して解説

##### 解説1: 名前・版・MSVC の版

- 1行目が製品の名前、2行目が Visual Studio の版（17.14）です。3行目の `14.44.35207` が、中の MSVC の道具の版です。
- `vswhere.exe` は、Visual Studio Installer と一緒に入る道具で、「この PC に、どの Visual Studio が入っているか」を答えます。`-products *` は、Build Tools も探す、という意味です。

##### 解説2: `Hostarm64` の中の3つ

- `bin\Hostarm64` は「ARM64 の PC で動く道具」の置き場です。その中の `arm64`・`x64`・`x86` は、**どの CPU 用の実行ファイルを作るか**で分かれています。
- この本の PC には3つともあり、`Hostarm64\arm64\link.exe` も見つかりました（`True`）。ARM64 の PC で ARM64 用を作るのは、この `link.exe` です。

#### 例5-2: できた実行ファイルが、どの CPU 用かを読む

> **▼ この手順がやること（先に日本語で）:** `rustc -vV` で、rustc が動いている PC の種類（host）を表示します。そのあと、UNIT 04 の `hello` を `cargo build` でビルドし（ソースが変わっていないので、作り直しはしません）、できた `hello.exe` がどの CPU 用かを、ファイルの先頭の印から読みます。

`.exe` のファイルには、先頭の近くに「どの CPU 用か」の印が書かれています。それを読む小さなスクリプト `machine.ps1` を、`rust-study` に置きました（BOM 付きの UTF-8 で保存します。UNIT 01）。

`machine.ps1`:

```powershell
# 実行ファイル（.exe）が、どの CPU 用に作られたかを表示する。
# 使い方: .\machine.ps1 ファイル1 ファイル2 …
foreach ($path in $args) {
    # 今いるフォルダからの道のりを、フルパスに直してから読む
    $full = (Resolve-Path $path).Path
    $bytes = [System.IO.File]::ReadAllBytes($full)
    # 0x3C の位置に「PE の見出しの場所」が書いてある。見出しの 4 バイト目から 2 バイトが CPU の種類
    $pe = [BitConverter]::ToInt32($bytes, 0x3C)
    $machine = [BitConverter]::ToUInt16($bytes, $pe + 4)
    $name = switch ($machine) {
        0xAA64 { "ARM64" }
        0x8664 { "x64" }
        0x014C { "x86（32 ビット）" }
        default { "そのほか" }
    }
    "{0,-6} 0x{1:X4}  {2}" -f $name, $machine, $path
}
```

```powershell
rustc -vV
cd ~\rust-study\hello
cargo build
& ..\machine.ps1 target\debug\hello.exe
```

**▼ 実行結果**

```text
rustc 1.99.1 (58cf8f9c3 2026-10-13)
binary: rustc
commit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67
commit-date: 2026-10-13
host: aarch64-pc-windows-msvc
release: 1.99.1
LLVM version: 23.1.1
```

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
ARM64  0xAA64  target\debug\hello.exe
```

#### ▼ 1つずつ分解して解説

##### 解説1: host は aarch64-pc-windows-msvc

- `host:` の行が、rustc が動いている PC の target です。何も指定しないと、cargo はこの host 用の実行ファイルを作ります。
- `LLVM version: 23.1.1` は、rustc の中で機械語を作る部品（LLVM）の版です。

##### 解説2: `0xAA64` は ARM64 の印

- `hello.exe` の印は `0xAA64` で、ARM64 用でした。x64 用なら `0x8664` です。
- 印は、ファイルの 0x3C（60）バイト目に書かれた場所の、4 バイト先にあります。Windows は、この印を見て、そのまま動かすか、通訳（エミュレーション）を使うかを決めます。

### もう一歩: 実務に近い形

#### 例5-3: x64 用の実行ファイルも作る

> **▼ この手順がやること（先に日本語で）:** `--target x86_64-pc-windows-msvc` を付けて、x64 用の `hello.exe` を作ります。最初は x64 用の標準ライブラリが無いので失敗します（エラー1）。`rustup target add` で足してから作り直し、ARM64 用と x64 用の印を比べ、x64 用を動かします。

```powershell
cd ~\rust-study\hello
rustup target add x86_64-pc-windows-msvc
cargo build --target x86_64-pc-windows-msvc
& ..\machine.ps1 target\debug\hello.exe target\x86_64-pc-windows-msvc\debug\hello.exe
.\target\x86_64-pc-windows-msvc\debug\hello.exe
```

**▼ 実行結果**

```text
info: downloading component rust-std
```

```text
   Compiling hello v0.1.0 (C:\Users\...\rust-study\hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.60s
ARM64  0xAA64  target\debug\hello.exe
x64    0x8664  target\x86_64-pc-windows-msvc\debug\hello.exe
Hello, world!
```

```powershell
Get-ChildItem target -Directory -Name
rustup target list --installed
```

**▼ 実行結果**

```text
debug
x86_64-pc-windows-msvc
```

```text
aarch64-pc-windows-msvc
x86_64-pc-windows-msvc
```

- `rustup target add` は、x64 用の標準ライブラリ（`rust-std`）を足しました。リンカーは、例5-1の `Hostarm64\x64` の道具が使われます。
- x64 用は `target\x86_64-pc-windows-msvc\debug\` に分けて置かれ、印は `0x8664` でした。ARM64 の PC でも、通訳を通して `Hello, world!` と動きました。
- **x64 用を作る場面:** x64 の PC の友だちに、実行ファイルを渡したいとき、などです。自分の PC で動かすだけなら、host 用（ARM64）のままが速く、いちばんです。

#### 例5-4: この PC の道具は、どの CPU 用か

> **▼ この手順がやること（先に日本語で）:** `machine.ps1` で、この巻で使う道具の実行ファイルの印を、まとめて読みます。

```powershell
& ~\rust-study\machine.ps1 (Get-Command rustc).Source "$env:LOCALAPPDATA\Programs\Microsoft VS Code\Code.exe" (Get-Command git).Source 'C:\Program Files\Git\usr\bin\bash.exe' (Get-Command docker).Source "$HOME\.fly\bin\fly.exe"
```

**▼ 実行結果**

```text
ARM64  0xAA64  C:\Users\...\.cargo\bin\rustc.exe
ARM64  0xAA64  C:\Users\...\AppData\Local\Programs\Microsoft VS Code\Code.exe
ARM64  0xAA64  C:\Program Files\Git\clangarm64\bin\git.exe
x64    0x8664  C:\Program Files\Git\usr\bin\bash.exe
ARM64  0xAA64  C:\Program Files\Docker\Docker\resources\bin\docker.exe
x64    0x8664  C:\Users\...\.fly\bin\fly.exe
```

- rustc・VS Code・Git・Docker のコマンドは ARM64 用でした。
- Git に入っている `usr\bin\bash.exe`（Git Bash）は x64 用です。UNIT 01 で、Git Bash から開いた PowerShell が `AMD64` と答えたのは、このためです。
- flyctl（`fly.exe`。UNIT 10）は x64 用です。Windows 用の公式の配布が x64 版だけなので、ARM64 の PC では通訳を通して動きます。
- **x64 用でも動く**ので、困ることは少ないです。ただ、x64 用の道具から呼んだプログラムは、x64 として動くことがあります（UNIT 01 のエラー4）。結果が教材と違うときは、この印を確かめる手がかりになります。

### よくあるエラーと読み方

#### エラー1: `can't find crate for std`（target の部品が無い）

例5-3で、`rustup target add` の前に `--target x86_64-pc-windows-msvc` で作ったときです。

```text
   Compiling hello v0.1.0 (C:\Users\...\rust-study\hello)
error[E0463]: can't find crate for `std`
  |
  = note: the `x86_64-pc-windows-msvc` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-pc-windows-msvc`

For more information about this error, try `rustc --explain E0463`.
error: could not compile `hello` (bin "hello") due to 1 previous error
```

- **何が起きたか:** x64 用の標準ライブラリ（`std`）が見つかりませんでした。`note:` に「その target が入っていないかもしれない」、`help:` に直し方が書かれています。
- **直し方:** `help:` のとおり、`rustup target add x86_64-pc-windows-msvc` を打ちます。

#### エラー2: `linker ... not found`（Build Tools が無い）

Build Tools が入っていない PC では、リンカーが見つからず、次のようなエラーになります。この本の PC には Build Tools があるので、**わざと「無い名前のリンカーを使う」設定**（`-C linker=link-missing.exe`）にして、同じ「見つからない」状態を作りました。

```powershell
$env:RUSTFLAGS = "-C linker=link-missing.exe"
cargo build
```

```text
   Compiling hello v0.1.0 (C:\Users\...\rust-study\hello)
error: linker `link-missing.exe` not found
  |
  = note: program not found

note: the msvc targets depend on the msvc linker but `link.exe` was not found

note: please ensure that Visual Studio 2017 or later, or Build Tools for Visual Studio were installed with the Visual C++ option

note: VS Code is a different product, and is not sufficient

error: could not compile `hello` (bin "hello") due to 1 previous error
```

- **何が起きたか:** リンカーが見つかりませんでした（`program not found`）。`note:` の3行が、MSVC の target のときの説明です。「msvc の target は msvc のリンカーを使うが、`link.exe` が見つからない」「Visual Studio 2017 以降か、Build Tools を Visual C++ 付きで入れたか確かめて」「**VS Code は別の製品で、それだけでは足りない**」。
- **直し方:** 例5-1の「入れ方」で Build Tools（C++ によるデスクトップ開発）を入れ、PowerShell の窓を開き直します。試したあとは、`$env:RUSTFLAGS` を消すか、窓を閉じます。

### RustNote ではここで使う

- RustNote を、この本の PC で `cargo build` すると、ARM64 用の `rustnote.exe` ができます。リンカーは、例5-1の `Hostarm64\arm64\link.exe` です。
- 第12巻の Docker では、Linux の中でビルドしました。そのときの target は Linux 用なので、Windows の Build Tools は使いません（Docker のイメージの中の道具を使います）。
- 結果が教材と違うときは、「シェルの版・今いるフォルダ・OS・CPU の種類」（UNIT 01）に加えて、`rustc -vV` の `host:` と、実行ファイルの印を比べます。

### 練習問題

**問1:** x64 の PC で `rustc -vV` を打つと、`host:` の行には何と表示されると考えられますか。

**問2:** 例5-3のあとで `cargo run` を打つと、動くのは ARM64 用と x64 用のどちらでしょうか。

**問3:** 友だちが「VS Code を入れたのに、`cargo build` で `link.exe` が見つからないと言われる」と困っています。何を伝えればよいでしょうか。

### 解答と解説

**問1の解答:**

- `host: x86_64-pc-windows-msvc` と考えられます。CPU の部分が `x86_64` になり、OS（`windows`）と道具（`msvc`）は同じです（図の target の読み方）。

**問2の解答:**

- **ARM64 用**です。`--target` を付けない `cargo run` は、host（`aarch64-pc-windows-msvc`）用の `target\debug\hello.exe` を作って動かします。x64 用を動かしたいときは、`cargo run --target x86_64-pc-windows-msvc` と打ちます。

**問3の解答（例）:**

- VS Code はエディタで、リンカーの入った Build Tools（MSVC）とは別の製品です。エラー2の `note:` にも「VS Code is a different product」と書かれています。
- 例5-1の入れ方で「C++ によるデスクトップ開発」を入れ、PowerShell の窓を開き直してから、もう一度 `cargo build` をしてもらいます。

### まとめ

- Windows の Rust（`*-pc-windows-msvc`）は、MSVC の `link.exe` で実行ファイルを綴じる。`link.exe` と Windows の部品は Build Tools で入れる。VS Code だけでは足りない。
- この本の PC は Visual Studio Build Tools 2022（17.14・MSVC 14.44.35207）。`Hostarm64` の中に、ARM64・x64・x86 用の道具がある。
- 実行ファイルの印は、ARM64 が `0xAA64`、x64 が `0x8664`。rustc・VS Code・Git は ARM64 用、Git Bash の `bash.exe` と `fly.exe` は x64 用で、通訳を通して動く。
- 別の CPU 用は `rustup target add` で足して `--target` で作る。足していないと `can't find crate for std`。

## UNIT 10: flyctl と環境の診断

### この単元のゴール

- 公開の道具 flyctl（`fly`）を公式の手順で入れ、`fly version` で版と CPU の種類を確かめられる
- `fly auth login`・`fly auth whoami` など、アカウントにつながるコマンドと、つながらないコマンドを区別できる
- この巻でそろえた道具を、1つのスクリプト（`doctor.ps1`）で順番に確かめ、NG の行から、戻る単元を見つけられる

### たとえ話でつかむ

**旅行の前の、持ち物の点検**を考えます。

- 旅行（RustNote の公開）の前に、持ち物の一覧表を見ながら、1つずつ「ある・ない」に印を付けます。財布・切符・充電器……。足りない物があれば、出発の前に気づけます。
- 点検は、**家の中でできること**（かばんの中を見る）と、**外に問い合わせないと分からないこと**（予約が本当に取れているか）に分かれます。家の中の点検は何度でも気軽にできますが、問い合わせには、自分の名前（アカウント）が要ります。
- flyctl は、旅行会社の窓口です。窓口の受付の版を確かめる（`fly version`）のは家の中でできますが、予約の確認（`fly auth whoami`）は、旅行会社に問い合わせることになります。

| 旅行の前の点検 | この単元 |
|---|---|
| 持ち物の一覧表 | `doctor.ps1`（環境の診断） |
| ある・ない の印 | `OK`・`NG` の行 |
| 家の中でできる点検 | 版の表示・ファイルがあるか（アカウント不要） |
| 外への問い合わせ | `fly auth whoami`・`fly deploy` など（アカウントが要る） |
| 旅行会社の窓口 | flyctl（`fly`） |

> **用語:** flyctl（フライ・シー・ティー・エル：Fly.io を操作するコマンド。`fly` と打っても、`flyctl` と打っても同じ）

<!-- -->

> **用語:** 環境の診断（道具がそろっているか・版が合っているかを、決まった順番で確かめること。結果が教材と違うときの最初の手がかりになる）

### 図で理解する

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<rect x="20" y="20" width="400" height="266" rx="8" fill="#f0fdf4" stroke="#16a34a" stroke-width="1.5"/>
<text x="220" y="44" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">家の中でできる点検（アカウント不要）</text>
<text x="36" y="72" font-size="12" fill="#0f172a">UNIT 01〜03: PowerShell・CPU・Git・VS Code</text>
<text x="36" y="98" font-size="12" fill="#0f172a">UNIT 04〜06: rustc・使われる版・rustfmt・clippy</text>
<text x="36" y="124" font-size="12" fill="#0f172a">UNIT 05: MSVC Build Tools（ARM64・x64 の道具）</text>
<text x="36" y="150" font-size="12" fill="#0f172a">UNIT 07: WSL の Ubuntu・Docker</text>
<text x="36" y="176" font-size="12" fill="#0f172a">UNIT 08: topcoat CLI（0.11.0）</text>
<text x="36" y="202" font-size="12" fill="#0f172a">UNIT 09: Turso CLI（WSL の中）</text>
<text x="36" y="228" font-size="12" fill="#0f172a">UNIT 10: flyctl（fly version）</text>
<text x="36" y="262" font-size="12" font-weight="700" fill="#15803d">→ doctor.ps1 で、まとめて OK・NG を出す</text>
<rect x="440" y="20" width="260" height="266" rx="8" fill="#fff7ed" stroke="#ea580c" stroke-width="1.5"/>
<text x="570" y="44" font-size="13" font-weight="700" fill="#0f172a" text-anchor="middle">外への問い合わせ</text>
<text x="570" y="64" font-size="12" fill="#0f172a" text-anchor="middle">（アカウントが要る）</text>
<text x="456" y="98" font-size="12" fill="#0f172a">fly auth login・whoami</text>
<text x="456" y="124" font-size="12" fill="#0f172a">fly launch・deploy・secrets</text>
<text x="456" y="150" font-size="12" fill="#0f172a">turso auth login</text>
<text x="456" y="176" font-size="12" fill="#0f172a">turso db create</text>
<text x="456" y="218" font-size="11" fill="#c2410c">この巻ではしない。</text>
<text x="456" y="238" font-size="11" fill="#c2410c">公開の手順は第12巻で、</text>
<text x="456" y="258" font-size="11" fill="#c2410c">公式のドキュメントにそって説明する</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: 環境の診断で確かめる物。版やファイルは家の中で確かめ、アカウントが要る操作は分けて考える</div>
</div>

### まず動かす最小コード

#### 例10-1: flyctl を入れて、版を確かめる

> **▼ この手順がやること（先に日本語で）:** Fly.io の公式の手順で flyctl を入れます。新しい窓で `fly version` を打ち、版と、どの CPU 用の flyctl かを確かめます。

**入れ方:** Fly.io の公式のドキュメントでは、PowerShell で次のコマンドを打つように案内されています（2026-10-31 に確かめたもの）。`pwsh`（PowerShell 7）が無いときは、`powershell` でもよい、とあります。入る場所は、ホームフォルダの `.fly` です。

```powershell
pwsh -Command "iwr https://fly.io/install.ps1 -useb | iex"
```

この本の PC には、すでに flyctl v0.5.1 が入っていたので、版を確かめた結果だけを載せます。

```powershell
fly version
where.exe fly flyctl
& ~\rust-study\machine.ps1 (Get-Command fly).Source
```

**▼ 実行結果**

```text
fly.exe v0.5.1 windows/amd64 Commit: fc5174919aea834930ca9aba72f271cc54409a06 BuildDate: 2026-10-29T17:46:27Z
C:\Users\...\.fly\bin\fly.exe
C:\Users\...\.fly\bin\flyctl.exe
x64    0x8664  C:\Users\...\.fly\bin\fly.exe
```

#### ▼ 1つずつ分解して解説

##### 解説1: 版と CPU の種類

- `v0.5.1` が版、`windows/amd64` が「Windows の x64 用」という意味です。Windows 用の公式の配布は x64 版だけなので、ARM64 の PC でも x64 版が入り、通訳（エミュレーション）で動きます（UNIT 05 の例5-4）。
- `Commit:` と `BuildDate:` は、その版を作ったときの番号と日時です。困ったことを調べるときに、版と一緒に伝えます。

##### 解説2: `fly` と `flyctl` は同じもの

- `where.exe` の結果のとおり、`.fly\bin` に `fly.exe` と `flyctl.exe` の2つがあります。どちらで打っても同じです。この本では短い `fly` を使います。

#### 例10-2: アカウントが要るコマンドと、要らないコマンド

flyctl のコマンドには、Fly.io のサーバーに問い合わせる物が多くあります。この巻では、アカウントにはログインしません（公開は第12巻で、手順の説明として扱います）。公式のドキュメント（`fly auth`）で確かめた、ログインまわりのコマンドは次のとおりです。

| コマンド | すること（公式のドキュメントより） | アカウント |
|---|---|---|
| `fly version` | flyctl の版を表示する | 要らない |
| `fly auth signup` | 新しい Fly.io のアカウントを作る | 作る |
| `fly auth login` | ログインする（既定はブラウザでの認証） | 要る |
| `fly auth whoami` | 今ログインしているアカウント（メールアドレスなど）を表示する | 要る |
| `fly auth logout` | ログアウトする | 要る |

- ログインすると、合い鍵（アクセストークン）が、ホームフォルダの `.fly\config.yml` に保存されます。このファイルは、人に見せたり、Git に入れたりしません（UNIT 02 の秘密の扱い）。
- `fly auth whoami` は、ログインしたあとに「どのアカウントで操作しているか」を確かめるコマンドです。会社と個人など、アカウントを使い分けるときは、公開の前に必ず確かめます。

### もう一歩: 実務に近い形

#### 例10-3: 道具をまとめて確かめる（doctor.ps1）

> **▼ この手順がやること（先に日本語で）:** この巻でそろえた道具を、1つずつ確かめるスクリプトを `rust-study` に置きます。RustNote のフォルダで動かし、全部 `OK` になるかを見ます。アカウントが要る確かめはしません。

`doctor.ps1`（BOM 付きの UTF-8 で保存します）:

```powershell
# この巻でそろえた道具を、順番に確かめる（環境の診断）。
# 使い方: RustNote のフォルダで  powershell -ExecutionPolicy Bypass -File $HOME\rust-study\doctor.ps1
# 1つでも NG なら、終了コード 1 で終わる。ログインが要る確かめ（fly auth whoami など）はしない。
[Console]::OutputEncoding = [Text.Encoding]::UTF8
$failed = 0

function Show([string]$Name, [bool]$Ok, [string]$Detail) {
    $mark = if ($Ok) { "OK" } else { "NG" }
    Write-Host ("{0,-3} {1,-22} {2}" -f $mark, $Name, $Detail)
    if (-not $Ok) { $script:failed++ }
}

# コマンドを動かして、最初の1行を返す（無ければ $null）
function FirstLine([scriptblock]$Command) {
    try {
        # 出力を全部受け取ってから、終了コードを見る（途中で切ると、終了コードが変わることがある）
        $out = @(& $Command 2>$null)
        if ($LASTEXITCODE -ne 0 -or $out.Count -eq 0) { return $null }
        return [string]$out[0]
    } catch {
        return $null
    }
}

# ---- Windows の土台（UNIT 01〜03）
Show "PowerShell" ($PSVersionTable.PSVersion.Major -eq 5) $PSVersionTable.PSVersion.ToString()
Show "このシェルの CPU" ($env:PROCESSOR_ARCHITECTURE -eq "ARM64" -or $env:PROCESSOR_ARCHITECTURE -eq "AMD64") $env:PROCESSOR_ARCHITECTURE
$git = FirstLine { git --version }
Show "Git" ($null -ne $git) $(if ($git) { $git } else { "見つからない（UNIT 02）" })
$code = @(code --version 2>$null)
Show "VS Code" ($code.Count -ge 3) $(if ($code.Count -ge 3) { "$($code[0])（$($code[2])）" } else { "見つからない（UNIT 03）" })

# ---- Rust（UNIT 04〜06）
$rustc = FirstLine { rustc --version }
Show "rustc" ($null -ne $rustc) $(if ($rustc) { $rustc } else { "見つからない（UNIT 04）" })
$active = FirstLine { rustup show active-toolchain }
Show "使われる版" ($null -ne $active) $(if ($active) { ($active -split " ")[0] } else { "分からない（UNIT 04）" })
$fmt = FirstLine { cargo fmt --version }
Show "rustfmt" ($null -ne $fmt) $(if ($fmt) { $fmt } else { "入っていない（UNIT 06）" })
$clippy = FirstLine { cargo clippy --version }
Show "clippy" ($null -ne $clippy) $(if ($clippy) { $clippy } else { "入っていない（UNIT 06）" })
$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
$vc = if (Test-Path $vswhere) { & $vswhere -products * -requires Microsoft.VisualStudio.Component.VC.Tools.ARM64 Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -requiresAny -property displayName } else { $null }
Show "MSVC Build Tools" ([bool]$vc) $(if ($vc) { $vc } else { "見つからない（UNIT 05）" })

# ---- Linux とコンテナ（UNIT 07）
$env:WSL_UTF8 = "1"
$wsl = FirstLine { wsl -d Ubuntu -- uname -m }
Show "WSL の Ubuntu" ($null -ne $wsl) $(if ($wsl) { "動く（$wsl）" } else { "動かない（UNIT 07）" })
$docker = FirstLine { docker version --format "{{.Server.Version}} {{.Server.Os}}/{{.Server.Arch}}" }
Show "Docker" ($null -ne $docker) $(if ($docker) { $docker } else { "Docker Desktop が動いていない（UNIT 07）" })

# ---- Topcoat・Turso・Fly.io（UNIT 08〜10）
$topcoat = FirstLine { topcoat --version }
Show "topcoat CLI" ($topcoat -eq "topcoat 0.11.0") $(if ($topcoat) { $topcoat } else { "見つからない（UNIT 08）" })
# Turso CLI は、Turso のクラウドを操作するときだけ使う（無くても RustNote は動く）ので、無くても NG にしない
$turso = FirstLine { wsl -d Ubuntu -- bash -ic "turso --version" }
if ($turso) {
    Show "Turso CLI（WSL）" $true $turso
} else {
    Write-Host ("{0,-3} {1,-22} {2}" -f "--", "Turso CLI（WSL）", "入っていない（UNIT 09。使うときに入れる）")
}
$fly = FirstLine { fly version }
Show "flyctl" ($null -ne $fly) $(if ($fly) { ($fly -split " ")[0..2] -join " " } else { "見つからない（UNIT 10）" })

Write-Host ""
if ($failed -gt 0) {
    Write-Host "NG の数: $failed"
    exit 1
}
Write-Host "全部 OK"
```

```powershell
cd （RustNote のフォルダ）
powershell -ExecutionPolicy Bypass -File $HOME\rust-study\doctor.ps1
"終了コード: $LASTEXITCODE"
```

**▼ 実行結果**

```text
OK  PowerShell             5.1.26100.9711
OK  このシェルの CPU             ARM64
OK  Git                    git version 2.53.0.windows.2
OK  VS Code                1.129.1（arm64）
OK  rustc                  rustc 1.99.1 (58cf8f9c3 2026-10-13)
OK  使われる版                  1.99.1-aarch64-pc-windows-msvc
OK  rustfmt                rustfmt 1.10.0-stable (58cf8f9c35 2026-10-13)
OK  clippy                 clippy 0.1.99 (58cf8f9c35 2026-10-13)
OK  MSVC Build Tools       Visual Studio Build Tools 2022
OK  WSL の Ubuntu           動く（aarch64）
OK  Docker                 29.3.1 linux/arm64
OK  topcoat CLI            topcoat 0.11.0
--  Turso CLI（WSL）         入っていない（UNIT 09。使うときに入れる）
OK  flyctl                 fly.exe v0.5.1 windows/amd64

全部 OK
終了コード: 0
```

#### ▼ 1つずつ分解して解説

##### 解説1: 何を確かめているか

- 上から、この巻の単元の順に並んでいます。行の最後の括弧の中は、NG のときに戻る単元です。
- `使われる版` は、今いるフォルダで選ばれる Rust です。RustNote のフォルダで動かしたので、`rust-toolchain.toml` の `1.99.1` が選ばれました（UNIT 04）。
- `Turso CLI（WSL）` は、WSL の中の対話のシェル（`bash -ic`）で確かめています。Turso CLI のインストールのスクリプトは、PATH を `.bashrc` に足すからです（UNIT 09）。この本の PC には入れていないので `--` でした。Turso CLI は、Turso のクラウドを操作するときだけ使う道具で、無くても RustNote は手元で動くので、NG にはしていません。
- `このシェルの CPU` は `ARM64` でした。Git Bash のような x64 の道具から開いた PowerShell では `AMD64` になり、PATH も変わることがあります（UNIT 01・05）。診断は、スタートメニューから開いた PowerShell で動かします。

##### 解説2: 終了コードで、ほかの道具からも使える

- 全部 OK なら終了コード 0、1つでも NG なら 1 で終わります。第12巻のスモークテストと同じ作りです。CI などのほかの道具から呼んでも、成功か失敗かが分かります。

### よくあるエラーと読み方

#### エラー1: NG の行が出た（道具が見つからない）

flyctl を入れる前の PC の代わりに、PATH から `.fly\bin` を抜いた PowerShell で `doctor.ps1` を動かしました。

```text
OK  PowerShell             5.1.26100.9711
OK  このシェルの CPU             ARM64
OK  Git                    git version 2.53.0.windows.2
OK  VS Code                1.129.1（arm64）
OK  rustc                  rustc 1.99.1 (58cf8f9c3 2026-10-13)
OK  使われる版                  1.99.1-aarch64-pc-windows-msvc
OK  rustfmt                rustfmt 1.10.0-stable (58cf8f9c35 2026-10-13)
OK  clippy                 clippy 0.1.99 (58cf8f9c35 2026-10-13)
OK  MSVC Build Tools       Visual Studio Build Tools 2022
OK  WSL の Ubuntu           動く（aarch64）
OK  Docker                 29.3.1 linux/arm64
OK  topcoat CLI            topcoat 0.11.0
--  Turso CLI（WSL）         入っていない（UNIT 09。使うときに入れる）
NG  flyctl                 見つからない（UNIT 10）

NG の数: 1
終了コード: 1
```

- **何が起きたか:** `flyctl` の行が `NG` になり、最後に `NG の数: 1`、終了コードは 1 でした。ほかの行は OK のままです。
- **直し方:** 行の括弧の中の単元（ここでは UNIT 10）に戻って、入れ直すか、PATH を確かめます（UNIT 04 のエラー1と同じく、新しい窓を開き直すのが最初の一手です）。

#### エラー2: `-File` に `~` を書いた

```powershell
powershell -ExecutionPolicy Bypass -File ~\rust-study\doctor.ps1
```

**▼ 実行結果**（エラーの文の所）

```text
The argument '~\rust-study\doctor.ps1' to the -File parameter does not exist. Provide the path to an existing '.ps1' file as an argument to the -File parameter.
Windows PowerShell
Copyright (C) Microsoft Corporation. All rights reserved.

終了コード: -196608
```

- **何が起きたか:** 「`-File` に渡した `~\rust-study\doctor.ps1` がありません」というエラーで、スクリプトは動きませんでした。`~`（ホームフォルダ）は、PowerShell の中で使える書き方で、`powershell.exe` という別のプログラムに渡すときには、ホームフォルダに直されないからです。
- **直し方:** `$HOME\rust-study\doctor.ps1` のように書きます。`$HOME` は、PowerShell が先に `C:\Users\（名前）` に直してから渡します。

### RustNote ではここで使う

- 第12巻で RustNote を公開する前に、`doctor.ps1` を RustNote のフォルダで動かし、全部 OK であることを確かめます。
- 公開の手順（`fly launch`・`fly deploy`・`fly secrets`）は、第12巻で、公式のドキュメントにそって説明します。そのとき、`fly auth whoami` で、使うアカウントを確かめてから進めます。

### 練習問題

**問1:** `doctor.ps1` を、ホームフォルダで動かしたときと、RustNote のフォルダで動かしたときとで、結果が変わる行はどれでしょうか。

**問2:** 次のうち、Fly.io のアカウントが無くても動くコマンドはどれでしょうか。`fly version`・`fly auth whoami`・`fly deploy`

**問3:** `doctor.ps1` に、「Rust が 1.99 以上か」を確かめる行を足すには、どうすればよいでしょうか。

### 解答と解説

**問1の解答:**

- `使われる版` の行です。RustNote のフォルダでは `rust-toolchain.toml` の `1.99.1`、ホームフォルダでは既定の `stable` が表示されます（UNIT 04）。この本の PC では、どちらも中身は 1.99.1 ですが、名前が違います。

**問2の解答:**

- `fly version` だけです。`fly auth whoami` はログインしているアカウントを問い合わせ、`fly deploy` は Fly.io にアプリを送るので、どちらもアカウントが要ります。

**問3の解答（例）:**

- `rustc --version` の2つ目の言葉（`1.99.1`）を `[version]` に変えて、`1.99` と比べます。

```powershell
$ver = [version](($rustc -split " ")[1])
Show "Rust の版" ($ver -ge [version]"1.99") $ver.ToString()
```

- Topcoat 0.11 は Rust 1.99 以上を求めるので（UNIT 08）、この行があると、古い Rust に早く気づけます。

### まとめ

- flyctl は、公式の `iwr https://fly.io/install.ps1 -useb | iex` で入れる。この本の PC は v0.5.1 の `windows/amd64` で、ARM64 の PC でも通訳で動く。`fly` と `flyctl` は同じ。
- `fly version` はアカウント不要。`fly auth login`・`whoami`・`logout`・`signup` はアカウントにつながる。トークンは `.fly\config.yml` に保存されるので、人に見せない。
- `doctor.ps1` で、PowerShell・Git・VS Code・Rust（rustfmt・clippy）・MSVC・WSL・Docker・topcoat CLI・flyctl を順に確かめる（Turso CLI は任意）。NG の行の括弧の単元に戻る。動かすときは `-File $HOME\…` と書き、x64 の道具からではなく、ふつうの PowerShell で動かす。

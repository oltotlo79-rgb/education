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

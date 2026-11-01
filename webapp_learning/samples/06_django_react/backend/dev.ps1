# 開発用のパソコンで、dev.env の設定を読み込んでから manage.py を動かす（必ず「UTF-8 (BOM 付き)」で保存する）
# 使い方: .\dev.ps1 migrate    .\dev.ps1 runserver    .\dev.ps1 test
# サーバーでは、これではなく onprem の 05 の manage.ps1（WinSW の XML を読む）を使う
param([Parameter(ValueFromRemainingArguments = $true)] [string[]] $ManageArgs)
$ErrorActionPreference = "Stop"

$envFile = Join-Path $PSScriptRoot "dev.env"
if (-not (Test-Path $envFile)) { throw "dev.env がありません。dev.env.example を写して作ってください。" }

$names = @()
foreach ($line in Get-Content $envFile -Encoding UTF8) {
    if ($line -match '^\s*#' -or $line -notmatch '=') { continue }
    $name, $value = $line -split '=', 2
    Set-Item -Path "Env:$($name.Trim())" -Value $value.Trim()
    $names += $name.Trim()
}

Push-Location $PSScriptRoot
try {
    & "$PSScriptRoot\.venv\Scripts\python.exe" manage.py @ManageArgs
    $code = $LASTEXITCODE
} finally {
    # パスワードなどを窓に残さない
    foreach ($n in $names) { Remove-Item -Path "Env:$n" -ErrorAction SilentlyContinue }
    Pop-Location
}
exit $code

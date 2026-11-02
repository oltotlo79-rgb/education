# 会社のサーバーと同じ形（waitress・DEBUG なし・WhiteNoise が画面の部品を配る）で、手元で動かして確かめる
# （必ず「UTF-8 (BOM 付き)」で保存する）
# 使い方: .\dev_serve.ps1   → http://127.0.0.1:8000/ を開く。止めるときは Ctrl+C
$ErrorActionPreference = "Stop"

$names = @()
foreach ($line in Get-Content (Join-Path $PSScriptRoot "dev.env") -Encoding UTF8) {
    if ($line -match '^\s*#' -or $line -notmatch '=') { continue }
    $name, $value = $line -split '=', 2
    Set-Item -Path "Env:$($name.Trim())" -Value $value.Trim()
    $names += $name.Trim()
}
$env:DJANGO_DEBUG = "false"   # 本番と同じ
$names += "DJANGO_DEBUG"

Push-Location $PSScriptRoot
try {
    $python = "$PSScriptRoot\.venv\Scripts\python.exe"
    & $python manage.py collectstatic --noinput   # 画面の部品を staticfiles に集める
    # onprem の 05 の 9.2 の XML の arguments と同じ（ポートだけ 8000）
    # 行末の ` は「次の行へ続く」という印（長い 1 行を 2 行に分けて書く）
    & $python -m waitress --listen=127.0.0.1:8000 --threads=8 `
        --trusted-proxy=127.0.0.1 --trusted-proxy-headers=x-forwarded-for config.wsgi:application
} finally {
    foreach ($n in $names) { Remove-Item -Path "Env:$n" -ErrorAction SilentlyContinue }
    Pop-Location
}

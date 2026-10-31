# 公開したあとの動作の確かめ（スモークテスト）。
# 使い方: powershell -File smoke.ps1 -Base https://あなたのアプリ.fly.dev
# 見るもの: /health・トップ・登録・一覧・ノートの作成と表示・ログアウト。1つでもだめなら終了コード 1
param([string]$Base = "http://127.0.0.1:8080")

$ErrorActionPreference = "Stop"
# 日本語の表示が文字化けしないよう、出力を UTF-8 にする
[Console]::OutputEncoding = [Text.Encoding]::UTF8
$jar = New-TemporaryFile
$failed = 0
$email = "smoke-$(Get-Date -Format 'yyyyMMddHHmmss')@example.com"

# 1つ確かめて、結果を1行で表示する（表示は Write-Host。関数の戻り値は curl の結果だけにする）
function Check([string]$Name, [string]$Expected, [string[]]$CurlArgs) {
    $out = & curl.exe -s -o NUL -w "%{http_code} %{redirect_url}" -b $jar.FullName -c $jar.FullName @CurlArgs
    $code = ($out -split " ")[0]
    if ($code -eq $Expected) {
        Write-Host "OK   $Name（$code）"
    } else {
        Write-Host "NG   $Name（期待 $Expected・実際 $code）"
        $script:failed++
    }
    return $out
}

Check "GET /health" "200" @("$Base/health") | Out-Null
Check "GET /（トップ）" "200" @("$Base/") | Out-Null
$form = "email=$([uri]::EscapeDataString($email))&password=correct+horse+battery&password_confirm=correct+horse+battery"
Check "POST /signup（登録）" "303" @("-d", $form, "$Base/signup") | Out-Null
Check "GET /notes（一覧）" "200" @("$Base/notes") | Out-Null
$created = Check "POST /notes（作成）" "303" @("-d", "title=smoke&body=ok", "$Base/notes")
$url = ($created -split " ")[1]
if ($url) { Check "GET 作ったノート" "200" @($url) | Out-Null }
Check "POST /logout" "303" @("-X", "POST", "$Base/logout") | Out-Null

Remove-Item $jar
if ($failed -gt 0) {
    Write-Output "だめだった数: $failed"
    exit 1
}
Write-Output "全部 OK"

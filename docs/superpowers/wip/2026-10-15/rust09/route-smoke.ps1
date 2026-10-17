# RustNote のルートを上から順に呼び、ステータスコードと移動先を表示する
# 使い方: 別の PowerShell で RustNote を起動しておき、.\route-smoke.ps1 を実行する
param([string]$Base = "http://127.0.0.1:3000")

# クッキー（ログイン状態）を保存するファイル
$jar = Join-Path $env:TEMP "rustnote-cookies.txt"
Remove-Item $jar -ErrorAction SilentlyContinue

# 1 回分のリクエストを送り、「メソッド パス -> 状態 → 移動先」を 1 行で表示する
function Call([string]$Method, [string]$Path, [string]$Form = "") {
    $curlArgs = @("-s", "-o", "NUL", "-b", $jar, "-c", $jar, "-X", $Method,
        "-w", "%{http_code} %{redirect_url}", "$Base$Path")
    if ($Form -ne "") {
        $curlArgs += @("--data", $Form)
    }
    $result = (& curl.exe @curlArgs).Split(" ", 2)
    $to = ""
    if ($result.Count -gt 1 -and $result[1] -ne "") {
        $to = "→ " + $result[1].Substring($Base.Length)
    }
    "{0,-6} {1,-20} -> {2} {3}" -f $Method, $Path, $result[0], $to
}

Call GET "/health"
Call GET "/"
Call GET "/notes"
Call POST "/signup" "email=alice%40example.com&password=correct-horse&password_confirm=correct-horse"
Call GET "/notes"
Call POST "/notes" "title=first&body=hello"
Call GET "/notes/1"
Call GET "/notes/1/edit"
Call POST "/notes/1/edit" "title=second&body=&version=1"
Call POST "/notes/1/toggle"
Call GET "/notes/abc"
Call GET "/notes/999"
Call DELETE "/notes"
Call POST "/notes/1/delete"
Call POST "/logout"
Call GET "/nope"

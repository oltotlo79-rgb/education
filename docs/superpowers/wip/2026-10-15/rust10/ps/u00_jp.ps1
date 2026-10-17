$base = "http://127.0.0.1:3310"
Set-Location $PSScriptRoot
$env:PATH = "C:\Windows\System32;" + $env:PATH
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
function pick($pattern) { process { [regex]::Matches($_, $pattern) | ForEach-Object { $_.Value } } }
function enc($text) { [uri]::EscapeDataString($text) }
(Get-Command curl.exe).Source
curl.exe --version | Select-Object -First 1
curl.exe -s -i -b alice.txt --data-urlencode "title=牛乳" -d "body=" "$base/notes" | pick '^location: .*'
curl.exe -s -i -b alice.txt -d "title=$(enc '卵')" -d "body=" "$base/notes" | pick '^location: .*'
curl.exe -s -b alice.txt "$base/notes" | pick '<a href="/notes/\d+">.*?</a>'

$base = "http://127.0.0.1:3310"
Set-Location $PSScriptRoot
Remove-Item alice.txt, bob.txt -ErrorAction SilentlyContinue
# ---- textbook commands start ----
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
function pick($pattern) { process { [regex]::Matches($_, $pattern) | ForEach-Object { $_.Value } } }
function enc($text) { [uri]::EscapeDataString($text) }
curl.exe -s -i -c alice.txt -d "email=alice%40example.com" -d "password=correct+horse+battery" -d "password_confirm=correct+horse+battery" "$base/signup"
"---"
curl.exe -s -b alice.txt -c alice.txt "$base/notes" | pick '<p class="flash[^>]*>.*?</p>|<h1>.*?</h1>|<p class="empty">.*?</p>'
"---"
curl.exe -s -b alice.txt "$base/notes" | Measure-Object -Line
"--- enc"
enc "買い物 100%"

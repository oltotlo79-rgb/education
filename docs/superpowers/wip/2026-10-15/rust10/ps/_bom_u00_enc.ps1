$base = "http://127.0.0.1:3310"
Set-Location $PSScriptRoot
"--- before UTF8 setting:"
[Console]::OutputEncoding.WebName
curl.exe -s -b alice.txt "$base/notes" | ForEach-Object { [regex]::Matches($_, '<h1>.*?</h1>') | ForEach-Object { $_.Value } }
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
function enc($text) { [uri]::EscapeDataString($text) }
"--- after:"
curl.exe -s -b alice.txt "$base/notes" | ForEach-Object { [regex]::Matches($_, '<h1>.*?</h1>') | ForEach-Object { $_.Value } }
enc "買い物 100%"
$PSVersionTable.PSVersion.ToString()

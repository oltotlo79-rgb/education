$node = "$PSScriptRoot\..\..\dl\node-v24.22.0-win-arm64\node.exe"
$env:PATH = "C:\Windows\system32;C:\Windows;C:\Program Files\Git\cmd;C:\Program Files\nodejs"
& $node find-command.mjs
& $node find-command.mjs git
& $node find-command.mjs nodee

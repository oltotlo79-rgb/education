# 実行ファイル（.exe）が、どの CPU 用に作られたかを表示する。
# 使い方: .\machine.ps1 ファイル1 ファイル2 …
foreach ($path in $args) {
    # 今いるフォルダからの道のりを、フルパスに直してから読む
    $full = (Resolve-Path $path).Path
    $bytes = [System.IO.File]::ReadAllBytes($full)
    # 0x3C の位置に「PE の見出しの場所」が書いてある。見出しの 4 バイト目から 2 バイトが CPU の種類
    $pe = [BitConverter]::ToInt32($bytes, 0x3C)
    $machine = [BitConverter]::ToUInt16($bytes, $pe + 4)
    $name = switch ($machine) {
        0xAA64 { "ARM64" }
        0x8664 { "x64" }
        0x014C { "x86（32 ビット）" }
        default { "そのほか" }
    }
    "{0,-6} 0x{1:X4}  {2}" -f $name, $machine, $path
}

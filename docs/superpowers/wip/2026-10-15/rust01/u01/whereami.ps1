# いま「どのシェルで・どこにいて・どんな PC か」を1画面にまとめて表示する
$os = Get-CimInstance Win32_OperatingSystem
$cpu = Get-CimInstance Win32_Processor | Select-Object -First 1

# CPU の種類の番号を名前に直す（9 = x64、12 = ARM64）
$machine = switch ($cpu.Architecture) {
    9 { "x64" }
    12 { "ARM64" }
    default { "その他 ($($cpu.Architecture))" }
}

Write-Output "シェル          : PowerShell $($PSVersionTable.PSVersion)"
Write-Output "現在のフォルダ  : $((Get-Location).Path)"
Write-Output "OS              : $($os.Caption) (ビルド $($os.BuildNumber))"
Write-Output "PC の CPU       : $machine"
Write-Output "このシェルの種類: $env:PROCESSOR_ARCHITECTURE"

# 作業フォルダのパスに空白や日本語（半角英数字と記号以外の文字）が入っていないかを調べる
$here = (Get-Location).Path
if ($here -match '[^\x21-\x7E]') {
    Write-Output "パスの文字      : 注意（空白や日本語が含まれています）"
} else {
    Write-Output "パスの文字      : OK（半角英数字と記号だけ）"
}

# OneDrive の同期フォルダの中かどうかを調べる
if ($env:OneDrive -and $here.StartsWith($env:OneDrive)) {
    Write-Output "OneDrive        : 注意（同期フォルダの中です）"
} else {
    Write-Output "OneDrive        : OK（同期フォルダの外です）"
}

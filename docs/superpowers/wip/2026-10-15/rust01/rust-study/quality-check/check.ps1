# コミットの前に流す品質チェック。1つでも失敗したら、そこで止める。
$steps = @(
    @{ Name = "整形の確認"; Args = @("fmt", "--check") },
    @{ Name = "Lint"; Args = @("clippy", "--all-targets", "--", "-D", "warnings") },
    @{ Name = "テスト"; Args = @("test", "--quiet") }
)

foreach ($step in $steps) {
    Write-Output "== $($step.Name): cargo $($step.Args -join ' ')"
    & cargo @($step.Args)
    # 直前に動かしたプログラムの終了コード。0 以外は失敗
    if ($LASTEXITCODE -ne 0) {
        Write-Output "NG: $($step.Name) で失敗しました（終了コード $LASTEXITCODE）"
        exit 1
    }
}

Write-Output "OK: すべてのチェックに通りました"

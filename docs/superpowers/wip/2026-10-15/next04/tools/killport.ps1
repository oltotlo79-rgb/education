param([int]$Port = 3000)
$c = Get-NetTCPConnection -LocalPort $Port -State Listen -ErrorAction SilentlyContinue
foreach ($p in $c) {
  $proc = Get-CimInstance Win32_Process -Filter "ProcessId=$($p.OwningProcess)"
  if ($proc.CommandLine -like "*scratchpad*next04*") { Stop-Process -Id $p.OwningProcess -Force; "killed $($p.OwningProcess)" }
}

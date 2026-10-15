# usage: python session.py START_DIR cmdfile  -> runs each line of cmdfile in one PowerShell session (ja-JP UI), prints transcript
import subprocess, sys, pathlib, tempfile, os
start = sys.argv[1]
cmds = pathlib.Path(sys.argv[2]).read_text(encoding="utf-8").splitlines()
lines = ["[System.Threading.Thread]::CurrentThread.CurrentUICulture = 'ja-JP'",
         "[System.Threading.Thread]::CurrentThread.CurrentCulture = 'ja-JP'",
         "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8",
         "$OutputEncoding = [System.Text.Encoding]::UTF8",
         f"Set-Location '{start}'"]
for c in cmds:
    if not c.strip():
        continue
    esc = c.replace("'", "''")
    lines.append(f"Write-Output ('PS ' + (Get-Location).Path + '> ' + '{esc}')")
    lines.append(f"try {{ . ([ScriptBlock]::Create('{esc}')) 2>&1 | Out-String -Width 100 -Stream | ForEach-Object {{ $_.TrimEnd() }} }} catch {{ $_ | Out-String -Width 100 -Stream | ForEach-Object {{ $_.TrimEnd() }} }}")
script = "\r\n".join(lines) + "\r\n"
p = pathlib.Path(tempfile.gettempdir()) / "session_tmp.ps1"
p.write_bytes(b"\xef\xbb\xbf" + script.encode("utf-8"))
out = subprocess.run(["powershell", "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", str(p)], capture_output=True)
text = out.stdout.decode("utf-8", errors="replace") + out.stderr.decode("utf-8", errors="replace")
sys.stdout.write(text)

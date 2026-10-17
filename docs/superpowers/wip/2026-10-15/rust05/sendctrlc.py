"""exe を新しいコンソールで起動し、wait 秒後にそのコンソールへ Ctrl+C を送って出力を表示する。
usage: python sendctrlc.py <exe> <wait_seconds>"""
import subprocess, sys, time
exe, wait = sys.argv[1], float(sys.argv[2])
# 子が Ctrl+C を無視する設定を引き継がないよう、ハンドラを既定に戻してから起動する
import ctypes
ctypes.windll.kernel32.SetConsoleCtrlHandler(None, False)
p = subprocess.Popen([exe], stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                     creationflags=subprocess.CREATE_NEW_CONSOLE)
time.sleep(wait)
helper = f"""
import ctypes, sys
k = ctypes.windll.kernel32
k.FreeConsole()
ok = k.AttachConsole({p.pid})
if not ok:
    sys.exit(10 + 0)
k.SetConsoleCtrlHandler(None, True)
r = k.GenerateConsoleCtrlEvent(0, 0)
sys.exit(0 if r else 3)
"""
r = subprocess.run([sys.executable, "-c", helper], creationflags=subprocess.CREATE_NO_WINDOW)
try:
    out, _ = p.communicate(timeout=10)
except subprocess.TimeoutExpired:
    p.kill()
    out, _ = p.communicate()
    print("[timeout: killed]")
sys.stdout.buffer.write(out)
print(f"[exit code {p.returncode}, helper {r.returncode}]")

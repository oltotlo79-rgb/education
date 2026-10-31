"""fly.toml と Dockerfile が、RustNote の待ち受けの設定とそろっているかを確かめる（Fly.io にはつながない）。
使い方: python check_fly.py [fly.toml のあるフォルダ] [確かめる URL（例: http://127.0.0.1:18080）]
"""
import re
import sys
import tomllib
import urllib.request
from pathlib import Path

folder = Path(sys.argv[1] if len(sys.argv) > 1 else ".")
base = sys.argv[2] if len(sys.argv) > 2 else None
fly = tomllib.loads((folder / "fly.toml").read_text(encoding="utf-8"))
docker = (folder / "Dockerfile").read_text(encoding="utf-8")
problems = 0


def check(ok, message):
    global problems
    print(("OK  " if ok else "NG  ") + message)
    if not ok:
        problems += 1


service = fly.get("http_service", {})
env = fly.get("env", {})
port = service.get("internal_port")
docker_port = re.search(r"PORT=(\d+)", docker)
docker_host = re.search(r"HOST=([0-9.]+)", docker)

check(port is not None, f"[http_service] internal_port = {port}")
check(env.get("PORT") == str(port), f"[env] PORT = {env.get('PORT')!r} と internal_port がそろっている")
check(docker_port is not None and docker_port.group(1) == str(port),
      f"Dockerfile の PORT = {docker_port.group(1) if docker_port else None} と internal_port がそろっている")
check(env.get("HOST") == "0.0.0.0", f"[env] HOST = {env.get('HOST')!r}（外から届くには 0.0.0.0）")
check(docker_host is not None and docker_host.group(1) == "0.0.0.0",
      f"Dockerfile の HOST = {docker_host.group(1) if docker_host else None}")
check(service.get("force_https") is True, "force_https = true（__Host-session クッキーは Secure なので HTTPS が要る）")
secrets_in_env = [k for k in env if "TOKEN" in k or "SECRET" in k or k == "DATABASE_URL"]
check(not secrets_in_env, f"[env] に秘密の値を書いていない（見つかったもの: {secrets_in_env}）")
checks = service.get("checks", [])
for c in checks:
    path = c.get("path")
    if base and path:
        try:
            with urllib.request.urlopen(base + path, timeout=5) as r:
                code = r.status
        except Exception as error:  # 届かない・4xx・5xx
            code = getattr(error, "code", type(error).__name__)
        check(code == 200, f"ヘルスチェック {c.get('method', 'GET')} {path} → {code}（{base}）")
    else:
        check(bool(path), f"ヘルスチェックの path = {path}")
print(f"だめだった数: {problems}")
sys.exit(1 if problems else 0)

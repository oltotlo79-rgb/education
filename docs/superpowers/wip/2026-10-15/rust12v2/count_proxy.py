# RustNote と sqld の間に置いて、RustNote が sqld に送った HTTP のリクエストを1行ずつ書き出す。
# 使い方: python count_proxy.py 待ち受けのポート 送り先のURL ログのファイル
import http.client
import json
import sys
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import urlsplit

listen_port = int(sys.argv[1])
upstream = urlsplit(sys.argv[2])
log_path = sys.argv[3]
lock = threading.Lock()


def short(sqls) -> str:
    text = " ".join(" ".join((s or "").split()) for s in sqls)
    return text[:32] + "…" if len(text) > 32 else text


def describe(body: bytes) -> str:
    """Hrana の中身を「describe『SQL』, close」のように短く書く（値は書かない）"""
    try:
        data = json.loads(body)
    except ValueError:
        return f"{len(body)} バイト"
    if "batch" in data and "requests" not in data:
        # /v3/cursor: 1つの batch を送り、結果を順に受け取る
        steps = data["batch"].get("steps", [])
        return f"cursor「{short(step.get('stmt', {}).get('sql') for step in steps)}」"
    parts = []
    for req in data.get("requests", []):
        kind = req.get("type", "?")
        sqls = []
        if kind == "execute":
            sqls = [req.get("stmt", {}).get("sql")]
        elif kind == "batch":
            sqls = [step.get("stmt", {}).get("sql") for step in req.get("batch", {}).get("steps", [])]
        elif kind in ("sequence", "describe"):
            sqls = [req.get("sql")]
        text = short(sqls)
        parts.append(f"{kind}「{text}」" if text else kind)
    return ", ".join(parts) if parts else "（requests なし）"


class Handler(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def log_message(self, *args):
        pass

    def read_body(self) -> bytes:
        if self.headers.get("Transfer-Encoding", "").lower() == "chunked":
            chunks = []
            while True:
                size = int(self.rfile.readline().strip().split(b";")[0], 16)
                if size == 0:
                    self.rfile.readline()
                    break
                chunks.append(self.rfile.read(size))
                self.rfile.readline()
            return b"".join(chunks)
        return self.rfile.read(int(self.headers.get("Content-Length", 0)))

    def forward(self):
        body = self.read_body()
        conn = http.client.HTTPConnection(upstream.hostname, upstream.port, timeout=30)
        headers = {k: v for k, v in self.headers.items() if k.lower() not in ("host", "transfer-encoding", "content-length", "connection")}
        conn.request(self.command, self.path, body=body if body else None, headers=headers)
        resp = conn.getresponse()
        data = resp.read()
        with lock, open(log_path, "a", encoding="utf-8") as log:
            line = f"{self.command} {self.path} → {resp.status}"
            if body:
                line += f": {describe(body)}"
            log.write(line + "\n")
        self.send_response(resp.status)
        for k, v in resp.getheaders():
            if k.lower() not in ("transfer-encoding", "content-length", "connection"):
                self.send_header(k, v)
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)
        conn.close()

    do_GET = forward
    do_POST = forward


ThreadingHTTPServer(("127.0.0.1", listen_port), Handler).serve_forever()

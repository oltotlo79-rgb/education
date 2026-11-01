"""練習用の「受け取るだけ」の SMTP サーバー。

届いた SMTP の会話を、そのまま画面に出し（C: は送ってきた側、S: はこのサーバー）、
メールを received/ フォルダに .eml で保存します。どこへも転送しません（本物の人には届きません）。
開発のパソコンだけで使います。Python の標準の部品だけで動きます（Python 3.11 以上）。

使い方:
  python smtp_sink.py                          127.0.0.1:1025 で待つ（暗号化なし）
  python smtp_sink.py --tls cert.pem key.pem   STARTTLS も使えるようにする（証明書は README の手順で作る）
止めるときは Ctrl+C。
"""

import argparse
import asyncio
import datetime
import pathlib
import ssl

HOST = "127.0.0.1"
OUT = pathlib.Path(__file__).with_name("received")


def show(who: str, line: str) -> None:
    print(f"{who}: {line}", flush=True)


async def handle(reader: asyncio.StreamReader, writer: asyncio.StreamWriter, tls_ctx: ssl.SSLContext | None) -> None:
    peer = writer.get_extra_info("peername")
    print(f"---- {peer[0]}:{peer[1]} からつながった ----", flush=True)

    async def send(line: str) -> None:
        show("S", line)
        writer.write((line + "\r\n").encode("ascii"))
        await writer.drain()

    async def recv() -> str | None:
        data = await reader.readline()
        if not data:
            return None
        line = data.decode("utf-8", "replace").rstrip("\r\n")
        show("C", line)
        return line

    tls = False
    mail_from, rcpts = None, []
    await send("220 localhost ESMTP practice-sink")
    while (line := await recv()) is not None:
        cmd = line.split(" ", 1)[0].upper()
        if cmd in ("EHLO", "HELO"):
            if cmd == "HELO":
                await send("250 localhost")
                continue
            lines = ["250-localhost", "250-SIZE 36700160", "250-8BITMIME"]
            if tls_ctx and not tls:
                lines.append("250-STARTTLS")  # 「暗号化に切りかえられます」という知らせ
            lines.append("250 SMTPUTF8")
            for item in lines:
                await send(item)
        elif cmd == "STARTTLS" and tls_ctx and not tls:
            await send("220 2.0.0 Ready to start TLS")
            try:
                await writer.start_tls(tls_ctx)
            except ssl.SSLError as e:
                # 送る側が、このサーバーの証明書を信じなかった（自分で作った証明書のときに起きる）
                print(f"---- 暗号化に切りかえられなかった: {e.reason} ----", flush=True)
                break
            print("---- ここから先は暗号化されている（途中の機器からは読めない）----", flush=True)
            tls, mail_from, rcpts = True, None, []  # 暗号化のあとは、EHLO からやり直す決まり
        elif cmd == "MAIL":
            mail_from, rcpts = line[10:].strip(), []
            await send("250 2.1.0 OK")
        elif cmd == "RCPT":
            rcpts.append(line[8:].strip())
            await send("250 2.1.5 OK")
        elif cmd == "DATA":
            if not mail_from or not rcpts:
                await send("503 5.5.1 MAIL and RCPT first")
                continue
            await send("354 End data with <CR><LF>.<CR><LF>")
            raw, in_header, body_lines = [], True, 0
            while True:
                data = await reader.readline()
                if data in (b".\r\n", b".\n", b""):
                    break
                if data.startswith(b".."):
                    data = data[1:]  # 行の頭の「.」を 2 つにして送る決まり（ドットの詰め直し）を戻す
                raw.append(data)
                text = data.decode("utf-8", "replace").rstrip("\r\n")
                if in_header:
                    show("C", text)
                    in_header = text != ""
                else:
                    body_lines += 1
            show("C", f"（本文 {body_lines} 行。表示は省く）")
            show("C", ".")
            OUT.mkdir(exist_ok=True)
            name = datetime.datetime.now().strftime("%Y%m%d-%H%M%S-%f") + ".eml"
            (OUT / name).write_bytes(b"".join(raw))
            await send(f"250 2.0.0 OK queued as {name}")
            mail_from, rcpts = None, []
        elif cmd == "RSET":
            mail_from, rcpts = None, []
            await send("250 2.0.0 OK")
        elif cmd == "NOOP":
            await send("250 2.0.0 OK")
        elif cmd == "QUIT":
            await send("221 2.0.0 Bye")
            break
        else:
            await send("502 5.5.2 Command not recognized")
    writer.close()
    print("---- 切れた ----", flush=True)


async def main() -> None:
    parser = argparse.ArgumentParser(description="練習用の、受け取るだけの SMTP サーバー")
    parser.add_argument("--port", type=int, default=1025)
    parser.add_argument("--tls", nargs=2, metavar=("CERT", "KEY"), help="STARTTLS に使う証明書と鍵")
    args = parser.parse_args()
    tls_ctx = None
    if args.tls:
        tls_ctx = ssl.create_default_context(ssl.Purpose.CLIENT_AUTH)
        tls_ctx.load_cert_chain(*args.tls)
    server = await asyncio.start_server(lambda r, w: handle(r, w, tls_ctx), HOST, args.port)
    print(f"{HOST}:{args.port} で待っています（STARTTLS: {'あり' if tls_ctx else 'なし'}）。Ctrl+C で止める。", flush=True)
    async with server:
        await server.serve_forever()


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        pass

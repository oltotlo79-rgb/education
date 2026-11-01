"""Python の smtplib で、テストのメールを 1 通送る（送る側から見た SMTP の会話を出す）。

使い方:
  python send_test.py                                  127.0.0.1:1025 へ、暗号化なしで
  python send_test.py --starttls --cafile cert.pem     STARTTLS で暗号化してから（自分で作った証明書を信じる）
"""

import argparse
import smtplib
import ssl
from email.message import EmailMessage


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--port", type=int, default=1025)
    parser.add_argument("--starttls", action="store_true")
    parser.add_argument("--cafile", help="信じる証明書（練習用に自分で作った物）")
    args = parser.parse_args()

    msg = EmailMessage()
    msg["From"] = "noreply@example.com"
    msg["To"] = "sato@example.com"
    msg["Subject"] = "【依頼・承認】送信テスト"
    msg.set_content("SMTP の練習のメールです。\n")

    with smtplib.SMTP(args.host, args.port, local_hostname="dev-pc", timeout=10) as smtp:
        smtp.set_debuglevel(1)  # やり取りを画面に出す
        smtp.ehlo()
        if args.starttls:
            smtp.starttls(context=ssl.create_default_context(cafile=args.cafile))
            smtp.ehlo()  # 暗号化のあとは、もう一度あいさつする
        smtp.send_message(msg)
    print("送りました。")


if __name__ == "__main__":
    main()

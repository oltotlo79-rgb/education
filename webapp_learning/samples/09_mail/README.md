# 09 メールと SMTP リレーの練習

開発のパソコンの中だけで、SMTP の会話を見るための練習のフォルダです。どこへも転送しないので、本物の人には届きません。

| ファイル | 中身 |
|---|---|
| `smtp_sink.py` | 練習用の「受け取るだけ」の SMTP サーバー。会話を画面に出し、メールを `received\` に `.eml` で保存する |
| `send_test.py` | Python の smtplib で、テストのメールを 1 通送る（送る側から見た会話を出す） |

## 使い方

```powershell
# 1. 暗号化なしの練習用サーバー（127.0.0.1:1025）。この窓は動かしたままにする
python smtp_sink.py

# 2. 別の窓で送る
python send_test.py
```

STARTTLS（暗号化への切りかえ）を試すときは、練習用の証明書を作ってから、別のポートで動かします。

```powershell
# Git for Windows の openssl を使う例（Git Bash なら、先頭に MSYS_NO_PATHCONV=1 を付ける）
openssl req -x509 -newkey rsa:2048 -nodes -keyout key.pem -out cert.pem -days 30 -subj "/CN=localhost" `
  -addext "subjectAltName=DNS:localhost,IP:127.0.0.1" -addext "basicConstraints=critical,CA:FALSE" `
  -addext "keyUsage=critical,digitalSignature,keyEncipherment" -addext "extendedKeyUsage=serverAuth"
python smtp_sink.py --port 1026 --tls cert.pem key.pem
python send_test.py --host localhost --port 1026 --starttls --cafile cert.pem
```

- `cert.pem`・`key.pem`・`received\` は Git に入れません（`.gitignore`）。
- 練習用の証明書は、自分で作った物なので、ほかの道具（アプリ）は信じません。それを確かめるのも練習の 1 つです（資料の 5.2）。

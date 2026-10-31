title: WSL2 と Docker
---checklist
- ☐ `wsl --install` で WSL2 と Ubuntu を入れ、`wsl --version`・`wsl -l -v` で確かめられる。文字化けしたら `$env:WSL_UTF8 = "1"` を付けられる（UNIT 07）
- ☐ Windows の `C:\` が Linux から `/mnt/c`、Linux の `~` が Windows から `\wsl.localhost\Ubuntu\home\…` に見えることを説明できる（UNIT 07）
- ☐ Linux の作業のファイルを Linux 側に置く理由を、測った数（小さなファイル 2000 個で約 3.6 秒と 0.1 秒）で言える（UNIT 07）
- ☐ Docker Desktop で `hello-world` を動かし、コンテナの CPU の種類（arm64・amd64）を確かめられる。「port is already allocated」を直せる（UNIT 07）
---glossary
| WSL2 | Windows の中で、本物の Linux を動かすしくみ |
| ディストリビューション | Linux の種類。この本では Ubuntu |
| `/mnt/c` | Linux から見た、Windows の C ドライブ |
| `\wsl.localhost` | Windows から見た、WSL の Linux のフォルダの入口 |
| コンテナ | アプリと、その動きに要る物をまとめて、切り分けて動かす箱 |
| イメージ | コンテナの設計図。Docker Hub などから取ってくる |
| Docker Desktop | Windows でコンテナを動かすアプリ。WSL2 の上で動く |

## UNIT 03: VS Code と rust-analyzer

### この単元のゴール

- Visual Studio Code（VS Code）の ARM64 版をインストールし、`code --version` で版と CPU の種類を確かめられる
- `code .` でフォルダを開き、VS Code の中のターミナル（統合ターミナル）で PowerShell を使える
- 拡張機能 **rust-analyzer** をコマンドで入れて、入ったことを一覧で確かめられる
- `settings.json` に「改行は LF」「保存のたびに整形」「clippy でチェック」の設定を書ける。ユーザーの設定とワークスペースの設定の違いを説明できる
- rust-analyzer が動かないときに、原因（Rust がまだ無い・開いたフォルダが違う）を切り分けられる

### たとえ話でつかむ

**VS Code** は、工房の**作業机**です。机の上には、ファイルの棚（エクスプローラー）、作業中の書類（エディタ）、受付の係員と話す窓口（ターミナル）が、1つにまとめて置かれています。

その机の横に、Rust にとても詳しい**先輩**に座ってもらいます。これが **rust-analyzer**（ラスト・アナライザー）です。

- あなたがコードを打っている間に、先輩は「その変数の型はこれだよ」「その名前のつづり、違うよ」と、横から教えてくれます。
- 書類を机に置く（保存する）たびに、先輩は書類をきれいに清書し直し（整形）、もっと良い書き方があれば赤ペンで書き込んでくれます（clippy のチェック）。
- ただし、先輩は**辞書と道具箱**（Rust の本体。UNIT 04 で入れます）を見ながら仕事をします。辞書と道具箱がまだ届いていないと、先輩は「道具が見つかりません」と言って、何もできません。
- 先輩は、**書類の束の表紙**（`Cargo.toml`）を見て、どこからどこまでが1つの仕事かを判断します。表紙の無い引き出しを渡しても、先輩は仕事を始めません。

机の使い方の決まり（「改行は LF で書く」「保存したら清書する」）は、**ルール表**（`settings.json`）に書いて、机に貼っておきます。

| 工房のたとえ | VS Code の言葉 | この単元で使うもの |
|---|---|---|
| 作業机 | VS Code の窓 | Visual Studio Code 1.129.1 |
| 机の上の窓口 | 統合ターミナル | PowerShell 5.1 |
| 横に座る先輩 | 拡張機能 rust-analyzer | rust-analyzer 0.3.3097 |
| 先輩の辞書と道具箱 | Rust の本体（cargo・rustc・rustfmt・clippy） | UNIT 04 で入れる |
| 書類の束の表紙 | `Cargo.toml` | 開いたフォルダの中にある必要がある |
| 机に貼るルール表 | 設定ファイル `settings.json` | ユーザーの設定・ワークスペースの設定 |

> **用語:** 拡張機能（extension：VS Code にあとから足す部品。言語ごとの補完やチェック、表示の日本語化などがある）

<!-- -->

> **用語:** rust-analyzer（ラスト・アナライザー：Rust のための拡張機能。打っている間の補完・型の表示・定義へのジャンプ・エラーの表示をする。中で `cargo` を呼び出して、プロジェクトの形を調べている）

### 図で理解する

まず、VS Code の画面の部分の名前です。この本では、次の名前で場所を指します。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 380" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-031" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="20" y="16" width="680" height="330" rx="8" fill="#ffffff" stroke="#475569" stroke-width="2"/>
<rect x="20" y="16" width="680" height="28" rx="8" fill="#e2e8f0"/>
<text x="360" y="35" font-size="12" fill="#0f172a" text-anchor="middle">hello-git - Visual Studio Code</text>
<rect x="20" y="44" width="40" height="276" fill="#334155"/>
<text x="40" y="74" font-size="11" fill="#ffffff" text-anchor="middle">①</text>
<text x="40" y="104" font-size="11" fill="#ffffff" text-anchor="middle">②</text>
<text x="40" y="134" font-size="11" fill="#ffffff" text-anchor="middle">③</text>
<rect x="60" y="44" width="170" height="276" fill="#f1f5f9"/>
<text x="72" y="66" font-size="12" font-weight="700" fill="#0f172a">エクスプローラー</text>
<text x="80" y="90" font-size="12" fill="#0f172a" font-family="Consolas, monospace">HELLO-GIT</text>
<text x="90" y="112" font-size="12" fill="#0f172a" font-family="Consolas, monospace">src</text>
<text x="104" y="132" font-size="12" fill="#0f172a" font-family="Consolas, monospace">main.rs</text>
<text x="90" y="152" font-size="12" fill="#0f172a" font-family="Consolas, monospace">.gitattributes</text>
<text x="90" y="172" font-size="12" fill="#0f172a" font-family="Consolas, monospace">.gitignore</text>
<text x="90" y="192" font-size="12" fill="#0f172a" font-family="Consolas, monospace">Cargo.toml</text>
<rect x="230" y="44" width="470" height="170" fill="#ffffff"/>
<text x="246" y="70" font-size="12" fill="#64748b" font-family="Consolas, monospace">main.rs</text>
<text x="246" y="98" font-size="12" fill="#1e40af" font-family="Consolas, monospace">fn main() {</text>
<text x="262" y="120" font-size="12" fill="#0f172a" font-family="Consolas, monospace">    println!("Hello, world!");</text>
<text x="246" y="142" font-size="12" fill="#1e40af" font-family="Consolas, monospace">}</text>
<rect x="230" y="214" width="470" height="106" fill="#0f172a"/>
<text x="246" y="236" font-size="12" fill="#e2e8f0">ターミナル</text>
<text x="246" y="262" font-size="12" fill="#facc15" font-family="Consolas, monospace">PS C:\Users\...\rust-study\hello-git&gt;</text>
<rect x="20" y="320" width="680" height="26" fill="#2563eb"/>
<text x="32" y="338" font-size="12" fill="#ffffff">main</text>
<text x="470" y="338" font-size="12" fill="#ffffff">UTF-8</text>
<text x="524" y="338" font-size="12" fill="#ffffff">LF</text>
<text x="554" y="338" font-size="12" fill="#ffffff">Rust</text>
<text x="600" y="338" font-size="12" fill="#ffffff">rust-analyzer</text>
<text x="40" y="370" font-size="11" fill="#475569">①エクスプローラー ②検索 ③ソース管理（Git）</text>
<text x="404" y="370" font-size="11" fill="#475569">左下: ブランチ　右下: 文字コード・改行コード・言語</text>
<line x1="465" y1="190" x2="465" y2="206" stroke="#475569" stroke-width="1.5"/>
<text x="465" y="186" font-size="11" fill="#475569" text-anchor="middle">エディタ</text>
<text x="665" y="236" font-size="11" fill="#94a3b8" text-anchor="end">パネル（統合ターミナル）</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: VS Code の画面イメージ。左端に切り替えのアイコン、その右にエクスプローラー、中央にエディタ、下にターミナル、いちばん下のステータスバーに改行コードや文字コードが出る</div>
</div>

- いちばん下の青い帯は**ステータスバー**です。右側の `UTF-8` は今のファイルの文字コード、`LF` は改行コードです。ここを押すと、文字コードや改行コードを変えられます（例3-4で使います）。
- 左端の縦の帯のアイコンで、エクスプローラー・検索・ソース管理（Git）・拡張機能などを切り替えます。

次に、rust-analyzer がどうやって仕事をしているかです。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 300" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-032" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="16" y="40" width="160" height="150" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="96" y="66" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">VS Code</text>
<text x="96" y="92" font-size="12" fill="#0f172a" text-anchor="middle">コードを打つ</text>
<text x="96" y="114" font-size="12" fill="#0f172a" text-anchor="middle">保存する</text>
<text x="96" y="146" font-size="12" fill="#b91c1c" text-anchor="middle">赤い波線</text>
<text x="96" y="168" font-size="12" fill="#1e40af" text-anchor="middle">型の表示・補完</text>
<rect x="236" y="40" width="200" height="150" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="336" y="66" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">rust-analyzer</text>
<text x="336" y="88" font-size="11" fill="#475569" text-anchor="middle">拡張機能に入っている</text>
<text x="336" y="104" font-size="11" fill="#475569" text-anchor="middle">rust-analyzer.exe（サーバー）</text>
<text x="336" y="134" font-size="12" fill="#0f172a" text-anchor="middle">プロジェクトの形を調べる</text>
<text x="336" y="156" font-size="12" fill="#0f172a" text-anchor="middle">保存のたびにチェック</text>
<rect x="496" y="40" width="208" height="150" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="600" y="66" font-size="13" font-weight="700" fill="#047857" text-anchor="middle">Rust の本体（UNIT 04）</text>
<text x="512" y="94" font-size="12" fill="#0f172a" font-family="Consolas, monospace">cargo metadata</text>
<text x="512" y="116" font-size="12" fill="#0f172a" font-family="Consolas, monospace">cargo check / clippy</text>
<text x="512" y="138" font-size="12" fill="#0f172a" font-family="Consolas, monospace">rustfmt</text>
<text x="600" y="172" font-size="11" fill="#475569" text-anchor="middle">Cargo.toml と src を読む</text>
<line x1="176" y1="100" x2="232" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-032)"/>
<line x1="232" y1="140" x2="180" y2="140" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-032)"/>
<line x1="436" y1="100" x2="492" y2="100" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-032)"/>
<line x1="492" y1="140" x2="440" y2="140" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-032)"/>
<text x="464" y="92" font-size="11" fill="#0f172a" text-anchor="middle">呼び出す</text>
<text x="464" y="158" font-size="11" fill="#0f172a" text-anchor="middle">結果</text>
<rect x="236" y="220" width="468" height="60" rx="10" fill="#fee2e2" stroke="#b91c1c" stroke-width="2"/>
<text x="470" y="244" font-size="12" fill="#b91c1c" text-anchor="middle">Rust が無いと: cargo が見つからない（program not found）</text>
<text x="470" y="266" font-size="12" fill="#b91c1c" text-anchor="middle">Cargo.toml が無いフォルダを開くと: そもそも動き始めない</text>
<line x1="600" y1="190" x2="600" y2="216" stroke="#b91c1c" stroke-width="2" stroke-dasharray="4 3" marker-end="url(#ar-01-032)"/>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: rust-analyzer の仕事の流れ。VS Code の拡張機能に入っているサーバーが、Rust の本体（cargo など）を呼び出してプロジェクトを調べ、結果を画面に返す</div>
</div>

- rust-analyzer の拡張機能には、本体のプログラム（`rust-analyzer.exe`）が入っています。ただし、プロジェクトの形を調べたり、チェックしたりするのには、**Rust の本体の `cargo` を呼び出します**。だから、Rust を入れる UNIT 04 までは、rust-analyzer は本当の力を出せません。
- この単元で拡張機能を先に入れておくと、UNIT 04 で Rust を入れた瞬間から、補完やチェックが使えるようになります。

最後に、設定の段です。UNIT 02 の Git の設定と同じように、VS Code の設定も段になっていて、**下の段ほど優先**されます。

<div style="max-width: 720px; margin: 18px auto; text-align: center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-01-033" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="60" y="16" width="600" height="56" rx="10" fill="#f8fafc" stroke="#64748b" stroke-width="2"/>
<text x="80" y="40" font-size="13" font-weight="700" fill="#0f172a">既定の設定</text>
<text x="80" y="60" font-size="12" fill="#475569">VS Code に最初から決まっている値（例: files.eol は "auto"）</text>
<rect x="60" y="92" width="600" height="56" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="80" y="116" font-size="13" font-weight="700" fill="#1e40af">ユーザーの設定</text>
<text x="80" y="136" font-size="12" fill="#475569">あなたが開くすべてのフォルダに効く（<tspan font-family="Consolas, monospace">%APPDATA%\Code\User\settings.json</tspan>）</text>
<rect x="60" y="168" width="600" height="56" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="80" y="192" font-size="13" font-weight="700" fill="#047857">ワークスペースの設定</text>
<text x="80" y="212" font-size="12" fill="#475569">そのフォルダだけに効く（フォルダの中の <tspan font-family="Consolas, monospace">.vscode\settings.json</tspan>）</text>
<line x1="690" y1="24" x2="690" y2="220" stroke="#475569" stroke-width="2" marker-end="url(#ar-01-033)"/>
<text x="704" y="90" font-size="12" fill="#475569" writing-mode="tb">下ほど優先</text>
</svg>
<div style="font-size: 12px; color: #64748b; margin-top: 6px;">図1-?: VS Code の設定の段。既定の設定をユーザーの設定が上書きし、さらにフォルダごとのワークスペースの設定が上書きする</div>
</div>

- 自分の好み（文字の大きさなど）や、どのプロジェクトでも同じにしたいこと（改行は LF）は、**ユーザーの設定**に書きます。
- そのプロジェクトの決まり（このプロジェクトでは clippy でチェックする、など）は、**ワークスペースの設定**に書いて、Git でチームと共有します。

### まず動かす最小コード

この単元の「最小コード」も、コマンドと画面の操作です。VS Code を入れて、拡張機能を入れて、UNIT 02 の `hello-git` を開くところまで進めます。

#### 例3-1: VS Code をインストールして、版と CPU の種類を確かめる

> **▼ この手順がやること（先に日本語で）:** VS Code の公式サイトからインストーラーを入手して、VS Code を入れます。新しく開いた PowerShell で、VS Code の版と、どの CPU 用の VS Code が入ったかを表示します。

**手順1: インストーラーを入手する。** ブラウザで VS Code の公式サイト（`https://code.visualstudio.com/`）を開き、ダウンロードのページから、Windows 用の **User Installer**（ユーザー用のインストーラー）を選びます。ARM64 の PC では **Arm64** 用、x64 の PC では x64 用を選びます。

**手順2: インストールする。** インストーラーを実行すると、次の順に画面が出ます（画面の名前は、この本の PC に入っている VS Code のインストーラーのものです）。

1. 「**使用許諾契約書の同意**」: 内容を読み、「**同意する**」を選んで「次へ」を押します。
2. インストール先や、スタートメニューのフォルダの画面は、既定のまま「次へ」を押します。
3. 「**追加タスクの選択**」: 次のように選びます。

| 項目 | 選び方 |
|---|---|
| デスクトップ上にアイコンを作成する | どちらでもよい |
| エクスプローラーのファイル コンテキスト メニューに [Code で開く] アクションを追加する | 入れておくと便利 |
| エクスプローラーのディレクトリ コンテキスト メニューに [Code で開く] アクションを追加する | 入れておくと便利 |
| サポートされているファイルの種類のエディターとして、Code を登録する | どちらでもよい |
| **PATH への追加（再起動後に使用可能）** | **必ず入れる** |

4. 「インストール」を押し、終わったら「完了」を押します。

「PATH への追加」は、PowerShell から `code` というコマンドで VS Code を動かせるようにする項目です。この本では、`code` コマンドを何度も使います。

**手順3: 新しい窓で確かめる。** PowerShell の窓をいったん閉じて、新しく開き直してから、次のコマンドを打ちます。

```powershell
# VS Code の版を表示する
code --version
# code というコマンドの本体が、どこにあるかを表示する
where.exe code
```

**▼ 実行結果**

```text
1.129.1
8a7abeba6e03ea3af87bfbce9a1b7e48fed567b8
arm64
C:\Users\...\AppData\Local\Programs\Microsoft VS Code\bin\code
C:\Users\...\AppData\Local\Programs\Microsoft VS Code\bin\code.cmd
```

#### ▼ 1つずつ分解して解説

##### 解説1: `code --version` の3行

- 1行目が VS Code の版、2行目はその版を作ったときのソースコードの番号（コミットの番号）、3行目が**どの CPU 用の VS Code か**です。
- ARM64 の PC で3行目が `arm64` なら、正しい版が入っています。`x64` と表示されたら、x64 用を入れてしまっています。動きはしますが、エミュレーションになり、その VS Code の中で開くターミナルも x64 になることがあります（UNIT 01 のエラー4と同じ問題）。いったんアンインストールして、Arm64 用を入れ直しましょう。

##### 解説2: User Installer は、あなたのユーザーのフォルダに入る

- `where.exe code` の結果のとおり、User Installer は VS Code を `C:\Users\（あなたの名前）\AppData\Local\Programs\Microsoft VS Code` に入れます。あなたのユーザーのフォルダの中なので、インストールにも更新にも管理者の権限が要りません。
- `bin` フォルダの `code` と `code.cmd` が、PowerShell から `code` と打ったときに動く入口です。「PATH への追加」で、このフォルダが `PATH` に足されました。

> **ヒント:** 表示を日本語にするには、拡張機能「Japanese Language Pack for Visual Studio Code」を入れます。PowerShell で `code --install-extension MS-CEINTL.vscode-language-pack-ja` と打ってから VS Code を開き直します。それでも英語のままなら、コマンド パレット（例3-4）で `Configure Display Language` を実行して `ja` を選びます。この本の VS Code の画面の名前（「ターミナル」「拡張機能」など）は、この日本語化を入れた状態のものです。

#### 例3-2: 拡張機能 rust-analyzer を入れて、一覧で確かめる

> **▼ この手順がやること（先に日本語で）:** PowerShell から、VS Code に拡張機能 rust-analyzer を入れます。入った拡張機能の名前と版を、一覧で表示して確かめます。

```powershell
# 拡張機能 rust-analyzer を入れる
code --install-extension rust-lang.rust-analyzer
```

**▼ 実行結果**

```text
Installing extensions...
(node:160520) [DEP0169] DeprecationWarning: `url.parse()` behavior is not standardized and prone to errors that have security implications. Use the WHATWG URL API instead. CVEs are not issued for `url.parse()` vulnerabilities.
(Use `Code --trace-deprecation ...` to show where the warning was created)
Installing extension 'rust-lang.rust-analyzer'...
Extension 'rust-lang.rust-analyzer' v0.3.3097 was successfully installed.
```

```powershell
# 入っている拡張機能の名前と版を一覧で表示する
code --list-extensions --show-versions
```

**▼ 実行結果**

```text
rust-lang.rust-analyzer@0.3.3097
```

ほかの拡張機能（日本語化など）を入れている人は、それも一緒に表示されます。

#### ▼ 1つずつ分解して解説

##### 解説1: `rust-lang.rust-analyzer` は「作った人.名前」

- 拡張機能には、`作った人の ID.拡張機能の ID` という形の名前が付いています。`rust-lang` は、Rust の開発チーム（The Rust Programming Language）の ID です。
- 似た名前の拡張機能がほかにもあるので、名前を正確に打つのが大切です。VS Code の画面から入れるときは、左端の「拡張機能」（または `Ctrl + Shift + X`）を開いて `rust-analyzer` で検索し、作った人が **The Rust Programming Language** になっているものの「インストール」を押します。

##### 解説2: 途中の DeprecationWarning は気にしなくてよい

- 2〜3行目の `DeprecationWarning`（非推奨の警告）は、VS Code のコマンドの**内部**で使われている部品についての、開発者向けのお知らせです。最後の行に `was successfully installed`（正しくインストールされました）と出ていれば成功です。
- `node:160520` の数字は、実行するたびに変わります。

##### 解説3: 拡張機能も CPU の種類ごとに作られている

- rust-analyzer の拡張機能には、本体のプログラム `rust-analyzer.exe` が入っているので、CPU の種類ごとに別のものが配られています。この本の PC では、入った拡張機能のフォルダの名前が `rust-lang.rust-analyzer-0.3.3097-win32-arm64` で、Windows の ARM64 用が自動で選ばれていました。
- VS Code そのものが ARM64 用なら、拡張機能も ARM64 用が選ばれます。例3-1で `code --version` の3行目を確かめたのは、このためでもあります。
- 拡張機能は、ふつうは `C:\Users\（あなたの名前）\.vscode\extensions` の中に、`作った人.名前-版` のフォルダとして入ります。本体のプログラムを含む拡張機能は、この本の PC では、フォルダの名前の最後に `-win32-arm64` が付いていました。拡張機能の調子がおかしいときに、どの CPU 用のものが入っているかを確かめる手がかりになります。

#### 例3-3: hello-git を VS Code で開き、統合ターミナルを使う

> **▼ この手順がやること（先に日本語で）:** UNIT 02 で作った `hello-git` フォルダを、PowerShell から VS Code で開きます。VS Code の中のターミナルを開いて、UNIT 01 のスクリプトで、そのターミナルが「どのシェルで・どこにいて・どの CPU か」を確かめます。

```powershell
# hello-git フォルダへ移動して、そのフォルダを VS Code で開く
cd ~\rust-study\hello-git
code .
```

VS Code が開き、次の順に進みます。

1. 初めて開くフォルダでは、「**このフォルダー内のファイルの作成者を信頼しますか?**」と尋ねる画面が出ます。自分で作ったフォルダなので、「**はい、作成者を信頼します**」を押します。
2. 左のエクスプローラーに、`src`・`.gitattributes`・`.gitignore`・`Cargo.toml` などが並びます。`.env` や `target` のように `.gitignore` で外したものは、薄い色で表示されます。
3. 上のメニューの「**ターミナル**」→「**新しいターミナル**」を押すと、下に統合ターミナルが開きます。

統合ターミナルで、UNIT 01 のスクリプトを実行します。

```powershell
# 1つ上のフォルダ（rust-study）にある whereami.ps1 を実行する
..\whereami.ps1
```

「現在のフォルダ」が `...\rust-study\hello-git` で、「PC の CPU」と「このシェルの種類」が同じ（ARM64 の PC なら両方 `ARM64`）なら、統合ターミナルは正しく使えています。

#### ▼ 1つずつ分解して解説

##### 解説1: `code .` の `.` は「今いるフォルダ」

- `code .` は「今いるフォルダ（`.`）を VS Code で開いて」というお願いです。`code src\main.rs` のようにファイルの名前を渡すと、そのファイルを開きます。
- VS Code では、**ファイル1つではなく、プロジェクトのフォルダを開いて**作業します。rust-analyzer も、開いたフォルダの中から `Cargo.toml` を探して仕事を始めるからです（「よくあるエラー」のエラー3）。

##### 解説2: 「作成者を信頼しますか?」の意味

- フォルダの中には、VS Code の設定や、拡張機能が自動で実行するプログラムを書けます。インターネットから取ってきた知らないフォルダを開いただけで、それが勝手に動くと危険です。
- 「いいえ、作成者を信頼しません」を選ぶと、**制限モード**で開きます。制限モードでは、rust-analyzer のような拡張機能が一部しか動きません。自分で作ったフォルダや、信頼できる人のフォルダだけ「はい」を選びましょう。

##### 解説3: 統合ターミナルは、開いたフォルダから始まる

- 統合ターミナルは、VS Code で開いたフォルダ（ここでは `hello-git`）を「今いるフォルダ」として始まります。`cd` しなくても、すぐにそのプロジェクトのコマンドを打てます。
- Windows の VS Code の統合ターミナルは、ふつうは PowerShell で開きます。中身は、Windows Terminal で開く PowerShell と同じです。

> **ヒント:** よく使う操作には、キーボードの近道があります。最初は次の5つだけ覚えておけば十分です。
>
> | キー | やること |
> |---|---|
> | `Ctrl + Shift + P` | コマンド パレットを開く（操作を名前で探す） |
> | `Ctrl + P` | ファイルを名前で探して開く |
> | `Ctrl + S` | 保存する |
> | `Ctrl + Shift + E` | エクスプローラーを開く |
> | `Ctrl + Shift + X` | 拡張機能の画面を開く |

### もう一歩: 実務に近い形

#### 例3-4: settings.json に、改行・整形・チェックの決まりを書く

> **▼ この手順がやること（先に日本語で）:** VS Code の**ユーザーの設定**のファイル `settings.json` を開き、「新しいファイルの改行は LF」「ファイルの最後に改行を1つ入れる」「行の最後の空白を消す」「保存のたびに整形する」「Rust の整形は rust-analyzer に任せる」「保存のたびのチェックを clippy で行う」「PowerShell のスクリプトは BOM 付きの UTF-8 で保存する」という決まりを書きます。

**手順1: settings.json を開く。** VS Code で `Ctrl + Shift + P` を押すと、画面の上に**コマンド パレット**（VS Code のすべての操作を、名前で探して実行できる入力欄）が開きます。`ユーザー設定を開く` と入力し、「**基本設定: ユーザー設定を開く (JSON)**」を選びます。

**手順2: 次の内容にする。** 初めて開いたときは `{}` だけか、空のことがあります。そのときは、全体を次の内容に置き換えます。すでに何か書かれているときは、`{` と `}` の内側に、次の行を書き足します（行と行の間は `,` で区切ります）。

```json
{
    "files.eol": "\n",
    "files.insertFinalNewline": true,
    "files.trimTrailingWhitespace": true,
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    },
    "rust-analyzer.check.command": "clippy",
    "[powershell]": {
        "files.encoding": "utf8bom"
    }
}
```

**手順3: 保存して、確かめる。** `Ctrl + S` で保存します。確かめ方は次のとおりです。

- 「ファイル」→「新しいテキスト ファイル」で新しいファイルを作り、画面の右下のステータスバーに **`LF`** と表示されることを確かめます。
- 左のエクスプローラーの「新しいファイル...」で `test.ps1` という名前のファイルを作って開き、ステータスバーに **`UTF-8 with BOM`** と表示されることを確かめます（確かめたら、`test.ps1` は消してかまいません）。
- Rust のファイルの整形とチェックは、UNIT 04 で Rust を入れた後に効き始めます。

#### ▼ 1つずつ分解して解説

##### 解説1: 改行と空白の決まり

| 設定 | 値 | 意味（既定の値） |
|---|---|---|
| `files.eol` | `"\n"` | 新しく作るファイルの改行を LF にする（既定は `"auto"` で、Windows では CRLF になる） |
| `files.insertFinalNewline` | `true` | 保存するとき、ファイルの最後に改行を1つ入れる（既定は `false`） |
| `files.trimTrailingWhitespace` | `true` | 保存するとき、行の最後の余分な空白を消す（既定は `false`） |

- `files.eol` は、UNIT 02 の `.gitattributes` と同じ考え方を、エディタの側でも守るための設定です。既定の `"auto"` は「OS 固有の改行文字を使用します」という意味で、Windows では CRLF になってしまいます。
- この設定が効くのは**新しく作るファイル**です。すでに CRLF のファイルを開いたときは、ステータスバーの `CRLF` を押して出てくる「**改行コードの選択**」で `LF` を選び、保存し直します。
- 設定の値（`"auto"` が既定であること、`"\n"`・`"\r\n"`・`"auto"` の3つから選べること）は、この本の PC の VS Code 1.129.1 の中身で確かめました。

##### 解説2: 保存したら整形する

- `editor.formatOnSave` を `true` にすると、保存のたびに**整形**（決まった形に、字下げや空白を直すこと）が走ります。
- `"[rust]": { … }` のように、`[言語の名前]` で囲んだ中に書いた設定は、**その言語のファイルにだけ**効きます。ここでは「Rust のファイルの整形は、rust-analyzer に任せる」（`editor.defaultFormatter`）と決めています。rust-analyzer は、Rust の標準の整形の道具 **rustfmt**（ラスト・フォーマット。UNIT 04 で Rust と一緒に入ります）を使って整形します。
- 整形の道具が決まっていない言語のファイルでは、保存しても整形はされません。

##### 解説3: 保存のたびに clippy でチェックする

- rust-analyzer は、保存するたびに、裏で `cargo check`（コンパイルできるかの確認）を実行して、エラーを赤い波線で表示します。`rust-analyzer.check.command` を `"clippy"` にすると、代わりに **`cargo clippy`**（よくある書き間違いや、もったいない書き方も指摘する道具。UNIT 06）が使われます。
- この設定の名前と、既定の値が `"check"` であることは、rust-analyzer 0.3.3097 の拡張機能の設定の一覧（`package.json`）で確かめました。

##### 解説4: PowerShell のスクリプトは BOM 付きで保存する

- UNIT 01 のエラー2のとおり、Windows PowerShell 5.1 は、BOM の無い `.ps1` の日本語を正しく読めません。VS Code の既定の文字コード（`files.encoding`）は `utf8`（BOM なし）なので、VS Code で新しく作った `.ps1` に日本語を書いて保存すると、同じ問題が起きます（「よくあるエラー」のエラー4）。
- `"[powershell]": { "files.encoding": "utf8bom" }` は、「PowerShell のファイルだけ、BOM 付きの UTF-8 で保存する」という設定です。
- すでに BOM なしで保存してしまったファイルは、ステータスバーの `UTF-8` を押し、「**エンコード付きで保存**」→「**UTF-8 with BOM**」を選ぶと、BOM 付きで保存し直せます。

> **注意:** `settings.json` は、JSON（ジェイソン）という形式で書きます。文字列は `"` で囲み、項目と項目の間は `,` で区切ります。`,` の付け忘れや、`"` の閉じ忘れがあると、その部分に波線が出て、設定が読み込まれません。

#### 例3-5: プロジェクトの決まりを .vscode フォルダに入れて共有する

> **▼ この手順がやること（先に日本語で）:** `hello-git` の中に `.vscode` フォルダを作り、「このプロジェクトでは rust-analyzer を使ってください」という**おすすめの拡張機能**の一覧（`extensions.json`）と、このプロジェクトだけの設定（`settings.json`）を置きます。最後に、Git から見るとどう見えるかを確かめます。

VS Code の左のエクスプローラーで、`HELLO-GIT` の右に出るアイコンの「新しいファイル...」を押し、`.vscode/extensions.json` と打って Enter キーを押します（`/` で区切ると、`.vscode` フォルダも一緒に作られます）。中身を次のようにして保存します。

```json
{
    "recommendations": [
        "rust-lang.rust-analyzer"
    ]
}
```

同じようにして、`.vscode/settings.json` を作ります。

```json
{
    "files.eol": "\n",
    "rust-analyzer.check.command": "clippy"
}
```

統合ターミナルで、Git から見た状態を確かめます。

```powershell
git status
```

**▼ 実行結果**

```text
On branch main
Untracked files:
  (use "git add <file>..." to include in what will be committed)
	.vscode/

nothing added to commit but untracked files present (use "git add" to track)
```

#### ▼ 1つずつ分解して解説

##### 解説1: `extensions.json` は「このプロジェクトのおすすめ」

- `.vscode/extensions.json` の `recommendations` に拡張機能の名前を並べておくと、このフォルダを開いた人の VS Code の「拡張機能」の画面に、「**ワークスペースの推奨事項**」という欄ができて、そこに並びます。チームの全員が、同じ道具をそろえやすくなります。
- 勝手にインストールされるわけではありません。入れるかどうかは、開いた人が決めます。

##### 解説2: ワークスペースの設定は、Git で共有する

- `.vscode/settings.json` は、図で見た「ワークスペースの設定」です。ユーザーの設定より優先されるので、このプロジェクトでは、誰の VS Code でも「改行は LF・clippy でチェック」になります。
- `git status` に `.vscode/` が出ているとおり、このフォルダは `.gitignore` で外していないので、`git add` してコミットすれば、チームと共有できます。
- 個人の好み（文字の大きさや色の設定など）は、ここではなく、ユーザーの設定に書きます。

##### 解説3: Git のメッセージを書くエディタを VS Code にする

- UNIT 02 で、`git commit` を `-m` なしで打つと Vim が開く、という話をしました。次の設定をすると、代わりに VS Code が開くようになります。

```powershell
# Git がメッセージを書いてもらうエディタを、VS Code にする
git config --global core.editor "code --wait"
```

- `--wait` は、「VS Code でそのファイルを閉じるまで、コマンドを待たせる」という指定です。メッセージを書いて保存し、タブを閉じると、コミットが続きます。成功しても何も表示されません。

#### 例3-6: 変更した内容を、VS Code のソース管理とターミナルの両方で見る

> **▼ この手順がやること（先に日本語で）:** `src/main.rs` の表示の文字を1か所だけ変えて保存し、VS Code の「ソース管理」の画面で、どこが変わったかを確かめます。同じことを、統合ターミナルの `git status --short` と `git diff` でも確かめ、最後に変更を元に戻します。

1. エクスプローラーで `src/main.rs` を開き、`"Hello, world!"` を `"Hello, Rust!"` に書き換えて、`Ctrl + S` で保存します。
2. 左端の「**ソース管理**」のアイコンを押します（`Ctrl + Shift + G` でも開きます）。「変更」の一覧に `main.rs` が並び、右に `M`（Modified：変更あり）の印が付きます。
3. 一覧の `main.rs` を押すと、変更前と変更後が左右に並んで表示され、変わった行に色が付きます。

同じことを、統合ターミナルで確かめます。

```powershell
# 変わったファイルを短い形で表示する
git status --short
# 変わった中身（差分）を表示する
git diff
```

**▼ 実行結果**（例3-5の `.vscode` フォルダを作った後の状態です）

```text
 M src/main.rs
?? .vscode/
warning: in the working copy of 'check.ps1', LF will be replaced by CRLF the next time Git touches it
diff --git a/src/main.rs b/src/main.rs
index e7a11a9..66449d7 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,3 +1,3 @@
 fn main() {
-    println!("Hello, world!");
+    println!("Hello, Rust!");
 }
```

確かめたら、変更を元に戻しておきます。

```powershell
# src/main.rs を、最後のコミットの状態に戻す（成功しても何も表示されない）
git restore src/main.rs
```

#### ▼ 1つずつ分解して解説

##### 解説1: `git status --short` の印

- `--short` を付けると、1つのファイルを1行で表示します。左の2文字が状態の印です。
- ` M`（左が空白で右が `M`）は「作業フォルダで変更したが、まだステージに載せていない」、`??` は「Git がまだ追跡していない」という意味です。UNIT 02 のエラー3で見た `D ` は「削除をステージに載せた」でした。左の文字がステージ、右の文字が作業フォルダの状態を表します。

##### 解説2: `git diff` の読み方

- `---` の行が変更前（`a/`）、`+++` の行が変更後（`b/`）のファイルです。
- `@@ -1,3 +1,3 @@` は「変更前の1行目から3行、変更後の1行目から3行の範囲を表示します」という意味です。
- 行の先頭の `-` が消えた行、`+` が増えた行、空白が変わっていない行です。VS Code のソース管理の左右の表示は、これを見やすく並べたものです。

##### 解説3: 途中の警告は、UNIT 02 で見たものと同じ

- `git diff` の最初に出た `check.ps1` の警告は、UNIT 02 の例2-5の手順2と同じものです。`.gitattributes` で `check.ps1` を CRLF にすると決めたのに、作業フォルダの `check.ps1` がまだ LF のままなので、Git が中を調べるたびに知らせてくれています。害はありません。
- `git restore` は、作業フォルダのファイルを、最後のコミットの状態に戻すコマンドです。保存していない変更ではなく、**保存済みの変更を捨てる**操作なので、本当に戻してよいかを確かめてから打ちましょう。

### よくあるエラーと読み方

#### エラー1: 「用語 'code' は、…認識されません」（PATH に入っていない）

```text
code : 用語 'code' は、コマンドレット、関数、スクリプト ファイル、または操作可能なプログラムの名前として認識されません。名前が正しく記述されていることを確認し、パスが含まれている場合はそのパスが正しいことを確認してから、再試行してください。
発生場所 行:1 文字:1
+ code --version
+ ~~~~
    + CategoryInfo          : ObjectNotFound: (code:String) [], CommandNotFoundException
    + FullyQualifiedErrorId : CommandNotFoundException
```

- **何が起きたか:** PowerShell が `PATH` のフォルダを探しましたが、`code` が見つかりませんでした。この本の PC で、`PATH` から VS Code の `bin` フォルダを抜いた PowerShell で `code --version` を打つと、このエラーになりました。
- **直し方:** 次の順に確かめます。
  1. **窓を開き直したか。** インストーラーが足した `PATH` は、インストールの後に新しく開いた窓から有効になります（インストーラーの項目の名前が「再起動後に使用可能」になっているのは、このためです）。
  2. **「PATH への追加」を選んだか。** 選び忘れたときは、もう一度インストーラーを実行して、「追加タスクの選択」でこの項目を選び直します。
  3. 開き直しても直らないときは、PC を再起動してみます。

#### エラー2: 「Failed to load the project」「program not found」（Rust がまだ無い）

rust-analyzer を入れて、Rust を入れる前に `Cargo.toml` のあるフォルダを開くと、rust-analyzer はプロジェクトを読み込めずにエラーになります。VS Code の画面では、`rust-analyzer failed to load workspace` のような知らせが出ます。何が起きているかを、rust-analyzer の本体をコマンドで直接動かして確かめると、次のようになりました（この本の PC で、`cargo` が見つからない状態にして、`hello-git` で実行したものです。長いので、最後の部分だけ載せています）。

```text
Error: Failed to load the project at ...\hello-git\.\Cargo.toml

Caused by:
    0: Failed to read Cargo metadata from Cargo.toml file ...\hello-git\.\Cargo.toml, None
    1: Failed to run `"cargo" "metadata" "--format-version" "1" "--manifest-path" "...\\hello-git\\.\\Cargo.toml"`
    2: program not found
```

- **何が起きたか:** 下から読みます。`program not found`（プログラムが見つかりません）が根っこの原因で、その上の行が「`cargo metadata` を実行できませんでした」、さらに上が「Cargo のメタデータ（プロジェクトの情報）を読めませんでした」、いちばん上が「プロジェクトを読み込めませんでした」です。`Caused by:`（原因は）の下に、原因の原因が順に並んでいます。
- 図で見たとおり、rust-analyzer はプロジェクトを調べるのに `cargo` を呼び出します。Rust がまだ入っていないので、`cargo` が見つからなかったのです。
- **直し方:** UNIT 04 で Rust を入れます。入れた後は、VS Code をいったん閉じて開き直すか、コマンド パレットで「`rust-analyzer: Restart server`」を実行すると、rust-analyzer が読み込み直します。
- VS Code の中で詳しい様子を見るには、「表示」→「出力」でパネルを開き、右上の一覧から「**rust-analyzer Language Server**」を選びます。

> **ヒント:** このエラーの読み方（`Caused by:` の下から読む）は、Rust の多くのツールや、Rust で書いたアプリのエラーで同じです。RustNote の起動のエラーも、同じ形で表示されます（第4巻で使う anyhow というクレートの表示の形です）。

#### エラー3（エラーが出ない落とし穴）: rust-analyzer が何もしない（開いたフォルダが違う）

`Cargo.toml` の無いフォルダを開くと、rust-analyzer は**エラーも出さずに、何もしません**。

- rust-analyzer の拡張機能の設定（`package.json`）を見ると、動き始める条件は「開いたフォルダの中に `Cargo.toml` がある」か「1つ下のフォルダに `Cargo.toml` がある」ことになっています（`workspaceContains:Cargo.toml` と `workspaceContains:*/Cargo.toml`）。
- たとえば、`C:\Users\（あなたの名前）` をまるごと開くと、`Cargo.toml` は `rust-study\hello-git\Cargo.toml` のように2つ下にあるので、rust-analyzer は動き始めません。
- **直し方:** `cd` でプロジェクトのフォルダ（`Cargo.toml` があるフォルダ）へ移動してから、`code .` で開きます。「ファイル」→「フォルダーを開く...」で開くときも、`Cargo.toml` があるフォルダそのものを選びます。

#### エラー4: VS Code で新しく作った .ps1 が、実行するとエラーになる

例3-4の `[powershell]` の設定を書く**前**の VS Code で、新しく `.ps1` のファイルを作り、日本語のコメントや表示の文字を書いて保存したとします。実行すると、UNIT 01 のエラー2と同じ「式またはステートメントのトークン '{' を使用できません」のようなエラーになります。

- **何が起きたか:** VS Code の既定の文字コードは BOM なしの UTF-8 なので、BOM の無いファイルができました。Windows PowerShell 5.1 は、BOM の無いファイルを Shift_JIS として読みます。ステータスバーを見ると、`UTF-8 with BOM` ではなく `UTF-8` と表示されているはずです。
- **直し方:** ステータスバーの `UTF-8` を押し、「エンコード付きで保存」→「UTF-8 with BOM」を選んで保存し直します。例3-4の設定を入れておけば、新しく作る `.ps1` は、最初から BOM 付きで保存されます。
- 例1-3で `Set-Content -Encoding utf8` で作った `whereami.ps1` のように、もともと BOM 付きのファイルは、VS Code が開いたときに BOM に気づき（ステータスバーに `UTF-8 with BOM` と出ます）、同じ形で保存し直します。

### RustNote ではここで使う

RustNote を VS Code で開くときは、`Cargo.toml` がある RustNote のフォルダ（`Rust/rustnote`）そのものを開きます。PowerShell で、そのフォルダへ `cd` で移動してから `code .` と打つのが確実です。

- RustNote のフォルダには `rust-toolchain.toml`（UNIT 01）があるので、rust-analyzer が呼び出す `cargo` も、そのフォルダで決めた Rust 1.99.1 になります。VS Code の赤い波線と、ターミナルの `cargo build` の結果が食い違わないのは、両方が同じ版の Rust を使うからです。
- RustNote の自動チェック（`Rust/rustnote/ci.yml`）では、次のように clippy を「警告もエラーとして扱う」設定で実行しています。

```yaml
      - name: Clippy（警告もエラーとして扱う）
        run: cargo clippy --all-targets --locked -- -D warnings
```

  例3-4で `rust-analyzer.check.command` を `"clippy"` にしておくと、自動チェックで指摘されるのと同じ種類の注意を、保存するたびに VS Code の中で先に見られます。
- RustNote のフォルダには `rustfmt.toml`（整形の決まり）があり、`edition = "2024"` と書かれています。保存のたびの整形（`editor.formatOnSave`）でも、この決まりが使われます。
- RustNote のフォルダには `.vscode` フォルダはありません。自分のリポジトリで RustNote を管理するときは、例3-5の `extensions.json` と `settings.json` を置いておくと、別の PC で開いたときにも同じ環境をすぐに作れます。

### 練習問題

**問1:** 例3-4の `settings.json` を書いた VS Code で、次の3つの操作をしました。それぞれ、どうなるでしょうか。

1. 「ファイル」→「新しいテキスト ファイル」で作ったファイルの改行コード
2. UNIT 02 の `.gitattributes` を開いて、最後の行の後ろに空白を3つ打ってから保存したときの、その空白
3. エクスプローラーの「新しいファイル...」で作った `hello.ps1` に、日本語のコメントを書いて保存したときの、ファイルの先頭の BOM

**問2:** 次のフォルダを VS Code で開いたとき、rust-analyzer が動き始めるのはどれですか（`hello-git` の中に `Cargo.toml` があります）。理由も答えてください。

1. `C:\Users\taro\rust-study\hello-git`
2. `C:\Users\taro\rust-study`
3. `C:\Users\taro`
4. `C:\Users\taro\rust-study\hello-git\src`

**問3:** ユーザーの設定に `"rust-analyzer.check.command": "check"`、ワークスペースの設定（`.vscode/settings.json`）に `"rust-analyzer.check.command": "clippy"` と書かれています。このワークスペースを開いたとき、保存のたびに使われるのはどちらですか。また、ほかのフォルダを開いたときはどうなりますか。

### 解答と解説

**問1の解答:**

1. **LF** になります。`files.eol` を `"\n"` にしたので、新しく作るファイルは LF です。ステータスバーに `LF` と表示されます。
2. **消えます**。`files.trimTrailingWhitespace` を `true` にしたので、保存するときに行の最後の空白が消されます。
3. **BOM が付きます**。`"[powershell]": { "files.encoding": "utf8bom" }` を書いたので、PowerShell のファイルは BOM 付きの UTF-8 で保存されます。この設定が無ければ BOM なしで保存され、エラー4のとおり、Windows PowerShell 5.1 で実行するとエラーになります。

**問2の解答:** 動き始めるのは **1** と **2** です。

- rust-analyzer は、開いたフォルダの中か、1つ下のフォルダに `Cargo.toml` があるときに動き始めます。
- **1** は、開いたフォルダの中に `Cargo.toml` があります。いちばんおすすめの開き方です。
- **2** は、1つ下の `hello-git` に `Cargo.toml` があるので動き始めます。ただし、`rust-study` の中のほかのフォルダも一緒に開くことになるので、ふつうはプロジェクトのフォルダ（1）を開きます。
- **3** は、`Cargo.toml` が2つ下（`rust-study\hello-git`）にあるので、動き始めません。
- **4** は、`Cargo.toml` が1つ**上**にあるので、動き始めません。

**問3の解答:** このワークスペースでは **clippy** が使われます。ワークスペースの設定は、ユーザーの設定より優先されるからです。ほかのフォルダ（`.vscode/settings.json` に書いていないフォルダ）を開いたときは、ユーザーの設定の **check** が使われます。

### まとめ

- VS Code は CPU に合った版（ARM64 の PC なら Arm64 の User Installer）を、「PATH への追加」を選んで入れ、新しい窓の `code --version` の3行目で確かめる。拡張機能は `code --install-extension rust-lang.rust-analyzer` で入れる。
- プロジェクトは `cd` してから `code .` で、`Cargo.toml` のあるフォルダそのものを開く。rust-analyzer は `cargo` を呼び出して働くので、本当に動くのは UNIT 04 で Rust を入れてから。エラーは `Caused by:` の下から読む。
- 設定は「既定 → ユーザー → ワークスペース」の順に上書きされる。ユーザーの設定に `files.eol`・`editor.formatOnSave`・`rust-analyzer.check.command` などを書き、プロジェクトの決まりは `.vscode` に入れて共有する。

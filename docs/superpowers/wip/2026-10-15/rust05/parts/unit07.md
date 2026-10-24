## UNIT 07: spawn_blocking と重い計算

### この単元のゴール

- 重い計算や同期の処理を async の中でそのまま行うと、同じスレッドで動くほかのタスクが止まることを、「心臓の鼓動」の表示で確かめられる
- `tokio::task::spawn_blocking` で、重い仕事を **blocking スレッド**（ブロッキング・スレッド）に任せ、結果を `Result<Result<…>, JoinError>` で受け取れる
- 別のスレッドへ渡す値を、借り物ではなく持ち主のいる値にできる（E0521 を直せる）
- 重い計算の同時実行数を `Semaphore` で抑えて、CPU とメモリを使い切らないようにできる
- `spawn_blocking` の仕事は途中で取り消せないこと、debug ビルドと release ビルドで重い計算の速さが大きく違うことを説明できる

### たとえ話でつかむ

UNIT 02 の大きな厨房を思い出してください。料理人（ワーカースレッド）は、たくさんの注文を少しずつ切り替えながら進める係でした。

- そこへ「生地を1時間こね続ける」仕事が来たとします。料理人がその場でこね始めると、こね終わるまで手が離せず、ほかの注文は全部止まります。待っているだけの仕事（`.await`）と違って、**こねる仕事は「席を譲る」ことができない**からです。
- そこで、厨房の奥に**パン職人の部屋**（blocking スレッド）を用意します。料理人は「この生地をこねておいて。できたら呼んで」と頼んで（`spawn_blocking`）、すぐにほかの注文に戻ります。
- パン職人は、必要に応じて増やせます（tokio の既定では最大 512 人）。でも、全員がいっせいにこね始めると、小麦粉（メモリ）も電気（CPU）も足りなくなります。だから、**整理券**（UNIT 05 の Semaphore）で「同時にこねるのは2人まで」と決めておきます。
- パン職人は、一度こね始めたら、途中で「やっぱりいいです」と言われてもやめません（`spawn_blocking` は取り消せない）。

> **用語:** blocking（ブロッキング：処理が終わるまで、スレッドを手放さずに占有し続けること。`.await` で席を譲る処理の反対）

<!-- -->

> **用語:** CPU バウンドと I/O バウンド（CPU バウンドは、計算そのものに時間がかかる処理。I/O バウンドは、ネットワークやディスクの返事を待つ時間がほとんどの処理。async が得意なのは I/O バウンド）

<!-- -->

> **用語:** blocking スレッド（tokio が、`spawn_blocking` で渡された同期の仕事のために用意するスレッド。必要なときに作られ、しばらく使われないと片付けられる。既定の上限は 512 本）

### 図で理解する

tokio のランタイムには、2種類のスレッドがあります。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 290" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-071" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="20" width="330" height="200" rx="12" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="179" y="46" font-size="13" font-weight="700" fill="#1e40af" text-anchor="middle">ワーカースレッド（料理人）</text>
<text x="179" y="66" font-size="11" fill="#475569" text-anchor="middle">ふつうは CPU のコア数だけ</text>
<rect x="34" y="82" width="290" height="36" rx="6" fill="#ffffff" stroke="#3b82f6"/>
<text x="179" y="105" font-size="11" fill="#0f172a" text-anchor="middle">async のタスクを、.await のたびに切り替える</text>
<rect x="34" y="126" width="290" height="36" rx="6" fill="#ffffff" stroke="#3b82f6"/>
<text x="179" y="149" font-size="11" fill="#0f172a" text-anchor="middle">得意: 待ち時間の多い仕事（DB・HTTP）</text>
<rect x="34" y="170" width="290" height="36" rx="6" fill="#fef2f2" stroke="#ef4444"/>
<text x="179" y="193" font-size="11" fill="#b91c1c" text-anchor="middle">苦手: 席を譲らない重い計算（全員が止まる）</text>
<rect x="376" y="20" width="330" height="200" rx="12" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="541" y="46" font-size="13" font-weight="700" fill="#c2410c" text-anchor="middle">blocking スレッド（パン職人）</text>
<text x="541" y="66" font-size="11" fill="#475569" text-anchor="middle">必要に応じて増える（既定の上限 512 本）</text>
<rect x="396" y="82" width="290" height="36" rx="6" fill="#ffffff" stroke="#f97316"/>
<text x="541" y="105" font-size="11" fill="#0f172a" text-anchor="middle">同期の仕事を1つ、最初から最後まで動かす</text>
<rect x="396" y="126" width="290" height="36" rx="6" fill="#ffffff" stroke="#f97316"/>
<text x="541" y="149" font-size="11" fill="#0f172a" text-anchor="middle">向いている: Argon2・圧縮・std::fs など</text>
<rect x="396" y="170" width="290" height="36" rx="6" fill="#fefce8" stroke="#eab308"/>
<text x="541" y="193" font-size="11" fill="#854d0e" text-anchor="middle">注意: 始まったら取り消せない</text>
<path d="M179,220 V252 H541 V224" fill="none" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-071)"/>
<text x="360" y="246" font-size="12" font-weight="700" fill="#0f172a" text-anchor="middle">spawn_blocking(|| 重い仕事)</text>
<text x="360" y="278" font-size="11" fill="#475569" text-anchor="middle">料理人は「できたら呼んで」と頼んで .await し、そのあいだ、ほかのタスクを進める</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: tokio の2種類のスレッド。async のタスクはワーカースレッドで切り替えながら進め、席を譲らない重い仕事は spawn_blocking で blocking スレッドに任せる</div>
</div>

次の図は、この後の例7-2の実行結果を、時間の線に並べたものです。0.2 秒ごとに「♥」を表示する「心臓の鼓動」のタスクと、パスワードのハッシュ（重い計算）を、ワーカースレッドが1本だけのランタイムで動かしました。

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 230" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-072" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<text x="20" y="24" font-size="12" font-weight="700" fill="#0f172a">心臓の鼓動（0.2 秒ごとの ♥）</text>
<text x="80" y="54" font-size="16" fill="#ef4444" text-anchor="middle">♥</text>
<text x="160" y="54" font-size="16" fill="#ef4444" text-anchor="middle">♥</text>
<text x="400" y="54" font-size="16" fill="#ef4444" text-anchor="middle">♥</text>
<text x="480" y="54" font-size="16" fill="#ef4444" text-anchor="middle">♥</text>
<text x="560" y="54" font-size="16" fill="#ef4444" text-anchor="middle">♥</text>
<text x="680" y="54" font-size="16" fill="#ef4444" text-anchor="middle">♥</text>
<rect x="200" y="72" width="200" height="40" rx="6" fill="#fee2e2" stroke="#ef4444" stroke-width="2"/>
<text x="300" y="90" font-size="11" font-weight="700" fill="#b91c1c" text-anchor="middle">① そのまま計算（0.3〜0.8 秒）</text>
<text x="300" y="105" font-size="10" fill="#b91c1c" text-anchor="middle">鼓動が止まる（0.4・0.6 が出ない）</text>
<rect x="520" y="72" width="200" height="40" rx="6" fill="#d1fae5" stroke="#10b981" stroke-width="2"/>
<text x="620" y="90" font-size="11" font-weight="700" fill="#047857" text-anchor="middle">② spawn_blocking（1.1〜1.6 秒）</text>
<text x="620" y="105" font-size="10" fill="#047857" text-anchor="middle">計算中も鼓動が続く（1.2・1.5）</text>
<line x1="40" y1="140" x2="710" y2="140" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-072)"/>
<line x1="80" y1="134" x2="80" y2="146" stroke="#475569" stroke-width="2"/>
<line x1="200" y1="134" x2="200" y2="146" stroke="#475569" stroke-width="2"/>
<line x1="400" y1="134" x2="400" y2="146" stroke="#475569" stroke-width="2"/>
<line x1="520" y1="134" x2="520" y2="146" stroke="#475569" stroke-width="2"/>
<text x="80" y="162" font-size="11" fill="#475569" text-anchor="middle">0 秒</text>
<text x="200" y="162" font-size="11" fill="#475569" text-anchor="middle">0.3</text>
<text x="400" y="162" font-size="11" fill="#475569" text-anchor="middle">0.8</text>
<text x="520" y="162" font-size="11" fill="#475569" text-anchor="middle">1.1</text>
<text x="360" y="194" font-size="12" fill="#475569" text-anchor="middle">①はワーカースレッドで直接計算したので、計算が終わるまで、ほかのタスクが1つも動けなかった</text>
<text x="360" y="216" font-size="12" fill="#475569" text-anchor="middle">②は blocking スレッドで計算したので、ワーカースレッドは鼓動のタスクを進め続けられた</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: 重い計算と「心臓の鼓動」。そのまま計算すると鼓動が止まり、spawn_blocking に任せると鼓動が続く。鼓動が止まっている間、サーバーならほかの人のリクエストも止まっている</div>
</div>

どんな処理が「席を譲らない」のかを、表にまとめます。

| 種類 | 例 | async の中でどうするか |
|---|---|---|
| CPU バウンドの重い計算 | パスワードのハッシュ（Argon2）・画像の縮小・圧縮・大きな JSON の解析 | `spawn_blocking` に任せる。同時実行数に上限を付ける |
| 同期の I/O | `std::fs` のファイル読み書き・`std::thread::sleep`・同期の DB ドライバ | async 版（`tokio::fs`・`tokio::time::sleep`・libsql など）を使う。無ければ `spawn_blocking` |
| 終わらない処理 | ずっと回り続ける監視のループなど | `spawn_blocking` ではなく `std::thread::spawn` で専用のスレッドを作る |
| 短い計算 | 数を足す・文字列を組み立てる・小さな JSON を作る | そのまま書いてよい（目安: 1回が 0.1 ミリ秒程度より短い） |

> **ヒント:** 「どれくらい重ければ `spawn_blocking` にするか」に決まった線はありません。表の「短い計算」の目安（0.1 ミリ秒）は、この本で使う大まかな目安です。迷ったら、例7-2の「心臓の鼓動」を自分の処理で試して、鼓動が止まらないかを確かめましょう。

### まず動かす最小コード

この単元では、パスワードのハッシュに argon2 クレートを使います。`async_lab` フォルダで次を実行します。

```powershell
cargo add argon2@=0.6.0
```

```text
    Updating crates.io index
      Adding argon2 =0.6.0 to dependencies
             Features:
             + alloc
             + getrandom
             + password-hash
             - kdf
             - parallel
             - rand_core
             - zeroize
    Updating crates.io index
     Locking 15 packages to highest Rust 1.98 compatible versions
      Adding argon2 v0.6.0
      Adding base64ct v1.8.3
      Adding blake2 v0.11.0
      Adding block-buffer v0.12.1
      Adding cmov v0.5.4
      Adding cpufeatures v0.3.1
      Adding crypto-common v0.2.2
      Adding ctutils v0.4.5
      Adding digest v0.11.3
      Adding getrandom v0.4.3
      Adding hybrid-array v0.4.15
      Adding password-hash v0.6.1
      Adding phc v0.6.1
      Adding r-efi v6.0.0
      Adding typenum v1.20.1
```

argon2 は、RustNote がパスワードを安全に保存するために使っているクレートで、版も同じ 0.6.0 です。パスワードの扱いそのもの（なぜハッシュにするのか、塩とは何か）は第8巻でくわしく学びます。この単元では「わざと重く作られた計算」の例として使います。

#### 例7-1: パスワードのハッシュは、どれくらい重いか

> **▼ このコードがやること（先に日本語で）:** パスワードを Argon2id（アルゴン・ツー・アイディー）の既定の設定でハッシュにし、かかった時間とできあがった文字列を表示します。続けて、そのハッシュとパスワードを照らし合わせ（照合）、結果と時間を表示します。async は使いません。

```rust
use std::time::Instant;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

fn main() {
    let start = Instant::now();
    // Argon2id（既定の設定）でハッシュ化する。塩（salt）は毎回ランダムに作られる
    let phc = Argon2::default()
        .hash_password(b"correct horse battery staple")
        .expect("ハッシュ化に失敗しました")
        .to_string();
    println!("ハッシュ化: {:.2} 秒", start.elapsed().as_secs_f64());
    println!("{phc}");

    let start = Instant::now();
    let parsed = PasswordHash::new(&phc).expect("PHC 文字列を読めません");
    let ok = Argon2::default()
        .verify_password(b"correct horse battery staple", &parsed)
        .is_ok();
    println!("照合: {ok}（{:.2} 秒）", start.elapsed().as_secs_f64());
}
```

`src/bin/u07_hash_time.rs` として保存して実行します。

```powershell
cargo run -q --bin u07_hash_time
```

**▼ 実行結果**

```text
ハッシュ化: 0.40 秒
$argon2id$v=19$m=19456,t=2,p=1$/zRp7pTcF+Q8kCqulTBc/A$6c1HL7P5KGLN4P8G+bTqWAtw7OcFHdJRLaYENekuIbU
照合: true（0.55 秒）
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: パスワードのハッシュは、わざと重く作られている

- 1回のハッシュ化に 0.40 秒、照合に 0.55 秒かかりました。DB への問い合わせ（数ミリ秒）と比べると、ずっと長い時間です。しかも、この時間は**待っているのではなく、CPU がずっと計算している**時間です。
- パスワードのハッシュは、もし DB の中身が盗まれても、総当たりでパスワードを当てられにくいように、**わざと時間とメモリがかかる**ように作られています。速くしてはいけない計算なのです。

##### 解説2: 長い文字列の中に、設定が書かれている

- 2行目は、保存用の文字列（PHC 文字列）です。`$argon2id` は方式、`v=19` は版、`m=19456,t=2,p=1` は設定（メモリを 19456 KiB＝約 19 MiB 使う・2回繰り返す・1本で計算する）、その後ろが塩とハッシュの値です。くわしくは第8巻で扱います。この設定は、OWASP（Web の安全の指針をまとめている団体）が推奨している値（m=19456・t=2・p=1）と同じです。
- `m=19456` のとおり、ハッシュ化を1回するたびに、約 19 MiB のメモリを使います。同時にたくさん計算すると、メモリもたくさん要ります（例7-4）。

> **注意:** この時間は、`cargo run`（debug ビルド）で測ったものです。最適化を有効にする release ビルドでは、ずっと速くなります。どれくらい変わるかは、練習問題1で確かめます。ただし「速くなる」と言っても、ワーカースレッドを止めてよい長さにはなりません。

#### 例7-2: そのまま計算すると、ほかのタスクが止まる

> **▼ このコードがやること（先に日本語で）:** 0.2 秒ごとに「♥ 経過時間」を表示する「心臓の鼓動」のタスクを動かします。ワーカースレッドが1本だけのランタイムで、①async の中でそのままハッシュを計算した場合と、②`spawn_blocking` に任せた場合で、鼓動の表示がどうなるかを比べます。

```rust
use std::time::{Duration, Instant};

use argon2::{Argon2, password_hash::PasswordHasher};

// CPU を使う重い処理: Argon2id でパスワードをハッシュ化する
fn hash(password: &str) -> String {
    Argon2::default()
        .hash_password(password.as_bytes())
        .expect("ハッシュ化に失敗しました")
        .to_string()
}

// 「心臓の鼓動」役: 200ms ごとに経過時間を表示する。止まったら、ほかのタスクが動けていない証拠
async fn heartbeat(start: Instant) {
    for _ in 0..12 {
        println!("  ♥ {:.1} 秒", start.elapsed().as_secs_f64());
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

// ワーカースレッドが1本だけのランタイム（違いが見やすいように）
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let start = Instant::now();
    let beat = tokio::spawn(heartbeat(start));
    tokio::time::sleep(Duration::from_millis(300)).await;

    // ① 悪い例: async の中で、そのまま重い計算をする
    println!(
        "① そのまま計算 開始 {:.1} 秒",
        start.elapsed().as_secs_f64()
    );
    let _ = hash("password-1");
    println!(
        "① そのまま計算 終了 {:.1} 秒",
        start.elapsed().as_secs_f64()
    );

    tokio::time::sleep(Duration::from_millis(300)).await;

    // ② 良い例: spawn_blocking で、重い計算専用の別スレッドへ任せる
    println!(
        "② spawn_blocking 開始 {:.1} 秒",
        start.elapsed().as_secs_f64()
    );
    let _ = tokio::task::spawn_blocking(|| hash("password-2"))
        .await
        .expect("計算のスレッドが panic しました");
    println!(
        "② spawn_blocking 終了 {:.1} 秒",
        start.elapsed().as_secs_f64()
    );

    beat.await.unwrap();
}
```

`src/bin/u07_blocking.rs` として保存して実行します。

```powershell
cargo run -q --bin u07_blocking
```

**▼ 実行結果**

```text
  ♥ 0.0 秒
  ♥ 0.2 秒
① そのまま計算 開始 0.3 秒
① そのまま計算 終了 0.8 秒
  ♥ 0.8 秒
  ♥ 1.0 秒
② spawn_blocking 開始 1.1 秒
  ♥ 1.2 秒
  ♥ 1.5 秒
② spawn_blocking 終了 1.6 秒
  ♥ 1.7 秒
  ♥ 1.9 秒
  ♥ 2.1 秒
  ♥ 2.3 秒
  ♥ 2.5 秒
  ♥ 2.7 秒
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: ①の間、心臓の鼓動が止まった

- 鼓動は 0.2 秒ごとのはずなので、0.4 秒と 0.6 秒にも「♥」が出るはずでした。ところが、①の計算をしていた 0.3〜0.8 秒の間は、1回も出ていません。
- `hash("password-1")` は `.await` の無い、ふつうの関数の呼び出しです。ワーカースレッドは、計算が終わるまでほかのタスクに席を譲れません。スレッドが1本しかないので、鼓動のタスクは 0.5 秒間、まったく動けませんでした。
- サーバーで言えば、**1人がログインしている間、ほかの全員の画面が止まる**ということです。ワーカースレッドが何本もあるランタイムでも、ログインが同時にスレッドの本数だけ来れば、同じことが起きます。

##### 解説2: ②の間は、鼓動が続いた

- ②の 1.1〜1.6 秒の間にも、1.2 秒と 1.5 秒に「♥」が出ています。`spawn_blocking` に渡した計算は、blocking スレッドという**別のスレッド**で動いたので、ワーカースレッドは空いたままでした。
- `main` の側は、`spawn_blocking(...).await` で計算の終わりを待っています。この `.await` は、ふつうの `.await` と同じく席を譲るので、待っている間に鼓動のタスクが進めます。
- 鼓動の間隔が 0.2 秒から少しずれている（1.2 → 1.5）のは、計算でパソコン全体が忙しくなっているためです。

##### 解説3: spawn_blocking は、JoinHandle を返す

- `tokio::task::spawn_blocking(クロージャ)` は、クロージャ（第2巻 UNIT 10）を blocking スレッドで動かし始め、UNIT 02 と同じ引換券（`JoinHandle`）をすぐに返します。
- 引換券を `.await` すると、`Result<クロージャの戻り値, JoinError>` が返ります。クロージャの中で panic したら `Err` です（「よくあるエラー2」）。
- 渡すのは `async` ブロックではなく、**ふつうのクロージャ**（`|| ...`）です。中で `.await` は書けません。同期の仕事を、同期のまま動かすための道具だからです。

### もう一歩: 実務に近い形

#### 例7-3: ハッシュ化と照合を、async の関数にまとめる

> **▼ このコードがやること（先に日本語で）:** RustNote と同じ形の2つの async 関数を作ります。`hash_password` はパスワードをハッシュにし、`verify_password` はパスワードと保存してあるハッシュを照らし合わせます。どちらも重い計算は `spawn_blocking` に任せ、失敗の種類を分けて `Err` で返します。正しいパスワード・違うパスワード・壊れたハッシュの3通りで試します。

```rust
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

// パスワードを Argon2id でハッシュ化する（重い計算は spawn_blocking で別スレッドへ）
async fn hash_password(password: &str) -> Result<String, String> {
    // 別スレッドへ渡すので、借り物（&str）ではなく持ち主のいる String にする
    let password = password.to_string();
    let joined = tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(password.as_bytes())
            .map(|hash| hash.to_string())
    })
    .await;
    // 外側の Result は「スレッドが panic しなかったか」、内側は「ハッシュ化に成功したか」
    match joined {
        Ok(Ok(phc)) => Ok(phc),
        Ok(Err(error)) => Err(format!("ハッシュ化に失敗: {error}")),
        Err(error) => Err(format!("計算のスレッドが異常終了: {error}")),
    }
}

// 入力されたパスワードが、保存してあるハッシュと一致するか調べる
async fn verify_password(password: &str, phc: &str) -> Result<bool, String> {
    let password = password.to_string();
    let phc = phc.to_string();
    let joined = tokio::task::spawn_blocking(move || {
        let parsed = PasswordHash::new(&phc)?;
        Ok::<bool, argon2::password_hash::Error>(
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed)
                .is_ok(),
        )
    })
    .await;
    match joined {
        Ok(Ok(matches)) => Ok(matches),
        Ok(Err(error)) => Err(format!("保存されたハッシュが読めない: {error}")),
        Err(error) => Err(format!("計算のスレッドが異常終了: {error}")),
    }
}

#[tokio::main]
async fn main() {
    let phc = hash_password("rust-is-fun").await.unwrap();
    println!("保存する文字列: {}...", &phc[..30]);
    println!(
        "正しいパスワード: {:?}",
        verify_password("rust-is-fun", &phc).await
    );
    println!(
        "違うパスワード:   {:?}",
        verify_password("rust-is-fum", &phc).await
    );
    println!(
        "壊れたハッシュ:   {:?}",
        verify_password("rust-is-fun", "abc").await
    );
}
```

`src/bin/u07_password.rs` として保存して実行します。

```powershell
cargo run -q --bin u07_password
```

**▼ 実行結果**

```text
保存する文字列: $argon2id$v=19$m=19456,t=2,p=1...
正しいパスワード: Ok(true)
違うパスワード:   Ok(false)
壊れたハッシュ:   Err("保存されたハッシュが読めない: invalid encoding")
```

#### ▼ コードを1つずつ分解して解説

##### 解説1: 別のスレッドへ渡す値は、持ち主のいる値にする

- `let password = password.to_string();` で、借り物の `&str` から、持ち主のいる `String` を作っています。`move` を付けたクロージャが、その `String` を blocking スレッドへ持っていきます。
- `spawn_blocking` も `tokio::spawn` と同じく、渡す仕事に `'static`（借り物を持たない）と `Send` を求めます。借り物のまま渡すと、コンパイルエラーになります（「よくあるエラー1」）。

##### 解説2: 失敗の種類を、3つに分ける

- 引換券の結果 `joined` は `Result<Result<…, argon2 のエラー>, JoinError>` です（UNIT 02 の練習問題3と同じ形）。
- `Ok(Ok(値))` は成功、`Ok(Err(…))` は「計算は最後まで動いたが、ハッシュ化や読み取りに失敗した」、`Err(…)` は「計算のスレッドが panic した」です。壊れたハッシュ `"abc"` を渡したときは、`PasswordHash::new` が `invalid encoding`（形式が正しくない）で失敗し、真ん中の場合になりました。
- 「違うパスワード」は、エラーではなく `Ok(false)` です。パスワードが合わないのは、ログインではよくあることなので、エラーではなく「一致しなかった」という結果として返しています。

##### 解説3: クロージャの中の ? と、Ok::<bool, …> の書き方

- `verify_password` のクロージャの中では、`PasswordHash::new(&phc)?` と `?` を使っています。クロージャの中の `?` は、クロージャ自身から `Err` を返します。
- そのため、クロージャの戻り値の型を Rust に教える必要があり、最後を `Ok::<bool, argon2::password_hash::Error>(...)` と書いています。「`Ok` の中身は `bool`、エラーは argon2 のエラー」という型の指定です（ターボフィッシュ `::<>`：第4巻 UNIT 03）。

#### 例7-4: 重い計算の同時実行数に、上限を付ける

> **▼ このコードがやること（先に日本語で）:** 同時に2つまでしかハッシュ化しない「ハッシュ係」`Hasher` を、Semaphore と `spawn_blocking` で作ります。6人が同時に登録ボタンを押した場面を、6つのタスクで再現し、それぞれのハッシュ化が終わった時刻を表示します。その間に、軽いリクエストがすぐ返せることも確かめます。

```rust
use std::sync::Arc;
use std::time::{Duration, Instant};

use argon2::{Argon2, password_hash::PasswordHasher};
use tokio::sync::Semaphore;

// 重い計算の同時実行数に上限を付けた「ハッシュ係」
#[derive(Clone)]
struct Hasher {
    permits: Arc<Semaphore>,
}

impl Hasher {
    fn new(max_concurrent: usize) -> Hasher {
        Hasher {
            permits: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    async fn hash(&self, password: String) -> Result<String, String> {
        // 許可証が空くまで待つ（待っている間はスレッドを使わない）
        let _permit = self.permits.acquire().await.map_err(|e| e.to_string())?;
        tokio::task::spawn_blocking(move || {
            Argon2::default()
                .hash_password(password.as_bytes())
                .map(|h| h.to_string())
                .map_err(|e| e.to_string())
        })
        .await
        .map_err(|e| e.to_string())?
    }
}

#[tokio::main]
async fn main() {
    let hasher = Hasher::new(2);
    let start = Instant::now();
    let mut handles = Vec::new();
    // 6人が同時に登録ボタンを押した
    for n in 1..=6 {
        let hasher = hasher.clone();
        handles.push(tokio::spawn(async move {
            let result = hasher.hash(format!("password-{n}")).await;
            println!(
                "{:.1} 秒: {n} 人目のハッシュ化が終わった（成功: {}）",
                start.elapsed().as_secs_f64(),
                result.is_ok()
            );
        }));
    }
    // その間も、ほかのリクエスト（ここでは軽い処理）は待たされずに動く
    tokio::time::sleep(Duration::from_millis(100)).await;
    println!(
        "{:.1} 秒: 軽いリクエストはすぐ返せる",
        start.elapsed().as_secs_f64()
    );
    for handle in handles {
        handle.await.unwrap();
    }
}
```

`src/bin/u07_limit.rs` として保存して実行します。

```powershell
cargo run -q --bin u07_limit
```

**▼ 実行結果**

```text
0.1 秒: 軽いリクエストはすぐ返せる
0.4 秒: 5 人目のハッシュ化が終わった（成功: true）
0.4 秒: 3 人目のハッシュ化が終わった（成功: true）
0.8 秒: 4 人目のハッシュ化が終わった（成功: true）
0.8 秒: 1 人目のハッシュ化が終わった（成功: true）
1.2 秒: 6 人目のハッシュ化が終わった（成功: true）
1.2 秒: 2 人目のハッシュ化が終わった（成功: true）
```

<div style="max-width:720px;margin:18px auto;text-align:center;">
<svg viewBox="0 0 720 250" width="100%" xmlns="http://www.w3.org/2000/svg" font-family="Yu Gothic UI, Yu Gothic, Meiryo, sans-serif">
<defs>
<marker id="ar-05-073" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#475569"/></marker>
</defs>
<rect x="14" y="30" width="150" height="150" rx="10" fill="#eff6ff" stroke="#3b82f6" stroke-width="2"/>
<text x="89" y="54" font-size="12" font-weight="700" fill="#1e40af" text-anchor="middle">登録のタスク 6 つ</text>
<text x="89" y="80" font-size="11" fill="#0f172a" text-anchor="middle">1・2・3・4・5・6</text>
<text x="89" y="104" font-size="10" fill="#475569" text-anchor="middle">許可証を待つ間は</text>
<text x="89" y="120" font-size="10" fill="#475569" text-anchor="middle">.await で席を譲る</text>
<text x="89" y="150" font-size="10" fill="#475569" text-anchor="middle">（スレッドも使わない）</text>
<rect x="214" y="70" width="140" height="70" rx="10" fill="#ecfdf5" stroke="#10b981" stroke-width="2"/>
<text x="284" y="96" font-size="12" font-weight="700" fill="#047857" text-anchor="middle">Semaphore</text>
<text x="284" y="116" font-size="11" fill="#0f172a" text-anchor="middle">許可証 2 枚</text>
<line x1="164" y1="105" x2="210" y2="105" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-073)"/>
<rect x="404" y="30" width="302" height="150" rx="10" fill="#fff7ed" stroke="#f97316" stroke-width="2"/>
<text x="555" y="54" font-size="12" font-weight="700" fill="#c2410c" text-anchor="middle">blocking スレッドで計算（同時に 2 つまで）</text>
<rect x="420" y="70" width="88" height="24" rx="4" fill="#ffedd5" stroke="#f97316"/>
<text x="464" y="87" font-size="10" fill="#0f172a" text-anchor="middle">5・3</text>
<rect x="510" y="70" width="88" height="24" rx="4" fill="#ffedd5" stroke="#f97316"/>
<text x="554" y="87" font-size="10" fill="#0f172a" text-anchor="middle">4・1</text>
<rect x="600" y="70" width="88" height="24" rx="4" fill="#ffedd5" stroke="#f97316"/>
<text x="644" y="87" font-size="10" fill="#0f172a" text-anchor="middle">6・2</text>
<text x="464" y="112" font-size="10" fill="#475569" text-anchor="middle">〜0.4 秒</text>
<text x="554" y="112" font-size="10" fill="#475569" text-anchor="middle">〜0.8 秒</text>
<text x="644" y="112" font-size="10" fill="#475569" text-anchor="middle">〜1.2 秒</text>
<text x="555" y="144" font-size="11" fill="#0f172a" text-anchor="middle">使うメモリは最大 約 19 MiB × 2</text>
<text x="555" y="164" font-size="11" fill="#0f172a" text-anchor="middle">CPU も 2 つ分まで</text>
<line x1="354" y1="105" x2="400" y2="105" stroke="#475569" stroke-width="2" marker-end="url(#ar-05-073)"/>
<rect x="14" y="196" width="692" height="44" rx="10" fill="#fef2f2" stroke="#ef4444"/>
<text x="360" y="216" font-size="11" fill="#b91c1c" text-anchor="middle">上限が無いと: 1000 人が同時にログインすれば、blocking スレッドが上限の 512 本まで増え、</text>
<text x="360" y="232" font-size="11" fill="#b91c1c" text-anchor="middle">約 19 MiB × 512 ＝ 約 9.5 GiB のメモリを一度に使おうとする</text>
</svg>
<div style="font-size:12px;color:#64748b;margin-top:6px;">図5-?: ハッシュ係のしくみ。許可証を持ったタスクだけが blocking スレッドで計算するので、CPU とメモリの使い方に上限が付く</div>
</div>

#### ▼ コードを1つずつ分解して解説

##### 解説1: 許可証を待つのは async、計算は blocking スレッド

- `self.permits.acquire().await` で、許可証が空くまで待ちます。この待ちは `.await` なので、待っている間はスレッドを使いません。許可証を持てたタスクだけが、`spawn_blocking` で計算を始めます。
- 許可証 `_permit` は、`hash` 関数の最後まで持たれ、計算が終わってから片付けられて返ります（UNIT 05 の例5-5）。だから、同時に計算しているのは、いつも2つまでです。
- 実行結果のとおり、2人ずつ約 0.4 秒ごとに終わり、6人で約 1.2 秒でした。どの2人が先に計算するかは、許可証を求めた順番で決まり、実行するたびに変わります。

##### 解説2: なぜ blocking スレッドの数だけに任せないのか

- `spawn_blocking` には、もともと上限（既定で 512 本）があります。tokio 1.53.3 のソース（`runtime/builder.rs`）でも、`max_blocking_threads` の既定は 512 です。しかし 512 は、「同時に 512 個の Argon2 を計算してよい」という意味ではありません。
- 例7-1で見たとおり、Argon2 は1回に約 19 MiB のメモリを使います。512 個を同時に計算すると、約 9.5 GiB です。CPU のコアも足りないので、1つ1つの計算も遅くなり、全員のログインが遅くなります。
- 上限を「CPU のコア数くらい」にしておけば、使うメモリと CPU が上限で決まり、混雑しても1人1人の待ち時間が読めるようになります。

##### 解説3: 軽いリクエストは、待たされない

- 0.1 秒の時点で、「軽いリクエストはすぐ返せる」が表示されています。ハッシュの計算は blocking スレッドで、許可証の待ちは `.await` で行われているので、ワーカースレッドは空いたままだからです。
- 「ログインが混んでいても、ノートの一覧はすぐ表示される」——これが、重い計算を正しく扱えているサーバーの姿です。

### よくあるエラーと読み方

#### エラー1: 借り物のまま、別のスレッドへ持ち込む（E0521）

```rust
use argon2::{Argon2, password_hash::PasswordHasher};

async fn hash_password(password: &str) -> String {
    // 借り物の password を、そのまま別スレッドへ持ち込もうとする
    tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(password.as_bytes())
            .unwrap()
            .to_string()
    })
    .await
    .unwrap()
}

#[tokio::main]
async fn main() {
    println!("{}", hash_password("rust-is-fun").await);
}
```

```text
   Compiling async_lab v0.1.0 (...\async_lab)
error[E0521]: borrowed data escapes outside of function
  --> src\bin\e07_borrow_escape.rs:5:5
   |
 3 |   async fn hash_password(password: &str) -> String {
   |                          --------  - let's call the lifetime of this reference `'1`
   |                          |
   |                          `password` is a reference that is only valid in the function body
 4 |       // 借り物の password を、そのまま別スレッドへ持ち込もうとする
 5 | /     tokio::task::spawn_blocking(move || {
 6 | |         Argon2::default()
 7 | |             .hash_password(password.as_bytes())
 8 | |             .unwrap()
 9 | |             .to_string()
10 | |     })
   | |      ^
   | |      |
   | |______`password` escapes the function body here
   |        argument requires that `'1` must outlive `'static`

For more information about this error, try `rustc --explain E0521`.
error: could not compile `async_lab` (bin "e07_borrow_escape") due to 1 previous error
```

- **何が起きたか:** 「借りたデータが、関数の外へ逃げ出しています」というエラーです。`password` は関数の中でだけ使える借り物（ライフタイム `'1`）なのに、`move` で別のスレッドへ持っていこうとしています。別のスレッドの仕事は関数より長生きするかもしれないので、`'1` が `'static` より長生きしなければならない（`'1 must outlive 'static`）という、無理な条件になりました。
- UNIT 02 の E0373（`tokio::spawn` で借り物を使う）と同じ理由のエラーです。今回は `move` を付けているので、「借り物そのものを移した」形でエラーになっています。
- **直し方:** 例7-3のとおり、`let password = password.to_string();` で持ち主のいる `String` を作ってから、`move` で渡します。

#### エラー2: spawn_blocking の中の panic（JoinError::Panic）

```rust
#[tokio::main]
async fn main() {
    let joined = tokio::task::spawn_blocking(|| {
        let sizes: Vec<u32> = Vec::new();
        // 空の Vec の最大値を unwrap して panic する
        sizes.iter().max().copied().unwrap()
    })
    .await;
    println!("{joined:?}");
}
```

```text

thread 'tokio-rt-worker' (66584) panicked at src\bin\e07_blocking_panic.rs:6:37:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Err(JoinError::Panic(Id(13), "called `Option::unwrap()` on a `None` value", ...))
```

（`cargo run` の `Compiling ...` などの行は省いています。）

- **何が起きたか:** 空の `Vec` の最大値は無い（`None`）ので、`unwrap` で panic しました。`spawn_blocking` の中の panic も、`tokio::spawn` と同じく、プログラム全体を落とさずに、引換券に `JoinError` として渡されます（UNIT 02 の例2-3）。`{:?}` で表示すると `JoinError::Panic(タスクの番号, "メッセージ", ...)` の形です。
- **直し方:** panic しないように、`Option` を `match` や `ok_or` で扱います。そのうえで、引換券の `Err` は、例7-3の `Err(error) => …` のように「計算のスレッドが異常終了」として扱い、サーバーなら 500 を返します。panic を `unwrap` で `main` まで持ち込まないようにしましょう。

#### エラー3（エラーが出ない間違い）: spawn_blocking は、途中で取り消せない

UNIT 06 の `timeout` で待つのをやめても、blocking スレッドの仕事は止まりません。

```rust
use std::time::{Duration, Instant};
use tokio::time::timeout;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // 1 秒かかる同期の処理を、別のスレッドで動かす
    let handle = tokio::task::spawn_blocking(move || {
        std::thread::sleep(Duration::from_secs(1));
        println!(
            "{:.1} 秒: 別スレッドの処理が最後まで終わった",
            start.elapsed().as_secs_f64()
        );
    });
    // 待つのは 0.1 秒まで
    match timeout(Duration::from_millis(100), handle).await {
        Ok(_) => println!("間に合いました"),
        Err(_) => println!(
            "{:.1} 秒: 待つのをやめました",
            start.elapsed().as_secs_f64()
        ),
    }
    println!("{:.1} 秒: main が終わります", start.elapsed().as_secs_f64());
}
```

`src/bin/u07_not_cancelled.rs` として実行した結果です。

```text
0.1 秒: 待つのをやめました
0.1 秒: main が終わります
1.0 秒: 別スレッドの処理が最後まで終わった
```

- **何が起きたか:** 0.1 秒で待つのをやめ、`main` も終わったのに、1.0 秒に「最後まで終わった」が表示され、プログラムはそこまで終わりませんでした。
- tokio のドキュメント（`task/blocking.rs`）には、「`spawn_blocking` の仕事は async ではないので、取り消せない。`abort` を呼んでも何も起きず、そのまま動き続ける」「ランタイムを片付けるときは、始まった `spawn_blocking` の仕事がすべて終わるまで待つ」と書かれています。この実行結果は、そのとおりの動きです。
- UNIT 06 で学んだとおり、async の仕事は `.await` の所で止められますが、同期のクロージャには `.await` がありません。だから止める場所がないのです。
- **気をつけること:** `spawn_blocking` に渡すのは、**いつか必ず終わる、長さに見当のつく仕事**だけにします（tokio のドキュメントも「最後には自分で終わる仕事のためのもの」としています）。終わらないループは `std::thread::spawn` の専用スレッドにし、止めたいときは、UNIT 04 の channel などで「止まって」の合図を送って、仕事の側で確かめて終わるようにします。サーバーを止めるとき（UNIT 06）に、長い `spawn_blocking` が残っていると、その分だけ止まるのが遅れることにも注意しましょう。

### RustNote ではここで使う

RustNote のパスワードの処理（`src/auth.rs`）は、例7-3とほぼ同じ形です。ハッシュ化の関数を抜き出します。

```rust
/// パスワードを Argon2id でハッシュ化し、PHC 文字列（$argon2id$v=19$...）にする。
/// 塩（salt）は argon2 クレートが毎回ランダムに作る。
pub async fn hash_password(password: &Password) -> Result<String, AppError> {
    let password = password.clone();
    // 重い計算なので、非同期の処理を止めないよう別スレッドで行う
    let joined = tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(password.as_str().as_bytes())
            .map(|hash| hash.to_string())
    })
    .await;
    match joined {
        Ok(Ok(phc)) => Ok(phc),
        Ok(Err(error)) => Err(AppError::internal(error)),
        Err(error) => Err(AppError::internal(error)),
    }
}
```

- `password.clone()` で持ち主のいる写しを作り、`move` で blocking スレッドへ渡しています（「よくあるエラー1」を避ける形）。`Password` は、RustNote の `src/domain.rs` で定義している、パスワード用の newtype です。
- 失敗は、どちらも `AppError::internal`（サーバーの中の問題、500）にしています。利用者には「エラーが起きました」とだけ見せ、くわしい理由はログに残す、という RustNote の方針です（第7巻 UNIT 10 でも見ました）。
- RustNote の `Dockerfile` は `cargo build --release --locked` で release ビルドを作っています。公開したアプリでは、ハッシュ化は練習問題1の release の速さで動きます。

ただし、この単元の目で見ると、RustNote には**直したほうがよい点が2つ**あります（どちらも RustNote のコードはまだ変えていません。引き継ぎの改善候補に記録しています）。

1. **ハッシュ化の同時実行数に上限が無い。** ログインと登録のたびに `spawn_blocking` しているだけなので、同時に大勢が来ると、blocking スレッドがどんどん増え、例7-4の図の下の段のように、CPU とメモリを使い切るおそれがあります。例7-4の `Hasher` のように、Semaphore で上限を付けるのが直し方です。
2. **ダミーのハッシュを、最初の1回だけワーカースレッドで計算している。** UNIT 05 で見た `DUMMY_HASH` は `LazyLock` で、最初に使われたときに作られます。使われる場所は、async 関数の `authenticate` の中の `verify_password(password, &DUMMY_HASH)` です。`&DUMMY_HASH` と書いた時点で（`verify_password` の中の `spawn_blocking` より前に）値が作られるので、最初の1回だけ、Argon2 の計算がワーカースレッドで行われます。直し方の例は、`auth.rs` に「`DUMMY_HASH` を先に作っておくだけの関数」（中で `LazyLock::force(&DUMMY_HASH)` を呼ぶ）を用意し、起動時（サーバーを始める前）に `spawn_blocking` の中で呼んでおくことです。

> **ヒント:** 2つ目は「1回だけ」なので、実際の影響は小さいものです。それでも、こうした「どのスレッドで重い計算が動くか」を1つずつ確かめる習慣が、混雑しても止まらないサーバーにつながります。

### 練習問題

**問1:** 例7-1の `u07_hash_time.rs` を、release ビルドで実行してください（`cargo run -q --release --bin u07_hash_time`）。ハッシュ化と照合の時間は、debug ビルドの 0.40 秒・0.55 秒から、どれくらい変わるでしょうか。予想してから実行しましょう。

**問2:** ファイルを読んで行数を数える同期の関数 `count_lines(path: &str) -> std::io::Result<usize>` を、async の中から呼べるようにした `count_lines_async(path: String) -> Result<usize, String>` を作ってください。10 万行のファイルを作って数え、無いファイルの場合のエラーも表示します（`src/bin/u07_q2.rs`）。

**問3:** 例7-4の `Hasher::new(2)` を `Hasher::new(3)` に変えると、6人のハッシュ化はそれぞれ何秒ごろに終わるでしょうか。予想してから実行してください（`src/bin/u07_q3.rs`）。

### 解答と解説

**問1の解答:**

```text
ハッシュ化: 0.02 秒
$argon2id$v=19$m=19456,t=2,p=1$x65/wsS7qC1329gDz82ZxQ$jvh1sOrv/5FqKUyXC3KppT5Gg2+1aYo6D6AhSUZ0f2c
照合: true（0.02 秒）
```

- ハッシュ化も照合も、約 0.02 秒になりました。debug ビルドの 20 分の 1 ほどです。release ビルドでは、コンパイラが計算を速くする工夫（最適化）をたくさん行うので、Argon2 のように計算ばかりの処理は、特に大きく速くなります。
- 塩が毎回ランダムなので、2行目の文字列は例7-1と違います（それでも照合は `true`）。
- **速くなっても、`spawn_blocking` は要ります。** 0.02 秒（20 ミリ秒）は、ワーカースレッドにとってはとても長い時間です（表の「短い計算」の目安の 0.1 ミリ秒と比べてみましょう）。また、時間を測るときは、debug と release のどちらで測ったかを必ず書き添えましょう。debug で測った時間で「遅すぎる」と判断すると、見当違いになります。

**問2の解答:**

```rust
use std::time::Instant;

// ファイルを読んで行数を数える（同期の処理。ファイルが大きいと時間がかかる）
fn count_lines(path: &str) -> std::io::Result<usize> {
    let text = std::fs::read_to_string(path)?;
    Ok(text.lines().count())
}

// async の中から呼べるように、spawn_blocking で別スレッドへ任せる
async fn count_lines_async(path: String) -> Result<usize, String> {
    tokio::task::spawn_blocking(move || count_lines(&path))
        .await
        .map_err(|e| format!("読むスレッドが異常終了: {e}"))?
        .map_err(|e| format!("読めません: {e}"))
}

#[tokio::main]
async fn main() {
    // 練習用に、10 万行のファイルを作っておく
    let lines: Vec<String> = (1..=100_000).map(|n| format!("{n} 行目")).collect();
    std::fs::write("u07_q2.txt", lines.join("\n")).expect("ファイルを作れません");

    let start = Instant::now();
    let count = count_lines_async(String::from("u07_q2.txt")).await;
    println!("{count:?}（{:.2} 秒）", start.elapsed().as_secs_f64());
    println!("{:?}", count_lines_async(String::from("no-such-file.txt")).await);
}
```

```text
Ok(100000)（0.08 秒）
Err("読めません: 指定されたファイルが見つかりません。 (os error 2)")
```

- 引数を `String` で受け取り、`move` でクロージャへ渡しています（「よくあるエラー1」を避ける形）。
- 結果は `Result<io::Result<usize>, JoinError>` です。1つ目の `map_err(...)?` で外側（panic）を、2つ目の `map_err` で内側（読めなかった）を、どちらも `String` のエラーにそろえています。無いファイルのときは、Windows の日本語のメッセージ「指定されたファイルが見つかりません。 (os error 2)」が、そのまま入っています。
- 実は、tokio にはファイル用の `tokio::fs`（`tokio::fs::read_to_string` など）があり、中では `spawn_blocking` を使っています（tokio 1.53.3 の `fs/mod.rs` の説明より）。ファイルを読むだけなら `tokio::fs::read_to_string(path).await` で十分です。読んだ後の重い処理（行を数える・解析する）までまとめて別スレッドで行いたいときは、この解答のように自分で `spawn_blocking` します。

**問3の解答:**

```text
0.1 秒: 軽いリクエストはすぐ返せる
0.4 秒: 1 人目のハッシュ化が終わった（成功: true）
0.4 秒: 2 人目のハッシュ化が終わった（成功: true）
0.4 秒: 3 人目のハッシュ化が終わった（成功: true）
0.7 秒: 4 人目のハッシュ化が終わった（成功: true）
0.8 秒: 5 人目のハッシュ化が終わった（成功: true）
0.8 秒: 6 人目のハッシュ化が終わった（成功: true）
```

- 3人ずつ2回に分かれて、約 0.4 秒と約 0.8 秒に終わりました。上限 2 のときの 1.2 秒より速くなっています。
- ただし、上限を上げれば上げるほど速くなるわけではありません。CPU のコアの数を超えると、同時に計算しても1つ1つが遅くなり、使うメモリだけが増えます。この PC は CPU のコア（論理プロセッサ）が12個ありますが、ほかの処理にも CPU を残したいので、上限は「コア数より少し少なめ」くらいから試すのがよいでしょう。上限の値は、`Config`（第4巻 UNIT 09）のような設定で変えられるようにしておくと、公開してから調整できます。

### まとめ

- 席を譲らない重い計算や同期の I/O を async の中でそのまま行うと、同じワーカースレッドのタスクが全部止まる。`spawn_blocking` で blocking スレッドに任せ、`.await` で結果を待つ。
- 別スレッドへ渡す値は、`to_string()` や `clone()` で持ち主のいる値にしてから `move` で渡す（E0521）。結果は `Ok(Ok)`・`Ok(Err)`・`Err(JoinError)` の3つに分けて扱う。
- 重い計算は Semaphore で同時実行数に上限を付け、CPU とメモリを使い切らないようにする。`spawn_blocking` は取り消せないので、必ず終わる仕事だけを渡す。速さは debug と release で大きく違う。

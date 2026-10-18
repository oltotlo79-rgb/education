use std::env;
use std::fmt;
use std::time::{Duration, Instant};

use libsql::{Builder, Database};

// 秘密の値。{:?} で表示されても中身を出さない
struct Secret(String);

impl Secret {
    // 中身が本当に必要な場所（DB を開くとき）だけで呼ぶ
    fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret(********)")
    }
}

// DB の置き場所の3つの形
#[derive(Debug)]
enum Mode {
    // 手元のファイル（開発・テスト）
    Local {
        path: String,
    },
    // ネットワークの向こうの DB に毎回問い合わせる
    Remote {
        url: String,
        token: Secret,
    },
    // 手元にコピーを持ち、読むのはコピー・書くのは向こう
    Replica {
        path: String,
        url: String,
        token: Secret,
    },
}

// 環境変数から、どの形で開くかを決める
fn mode_from_env() -> Result<Mode, String> {
    let url = env::var("DATABASE_URL").unwrap_or_else(|_| "u08_app.db".to_string());
    let token = Secret(env::var("DATABASE_AUTH_TOKEN").unwrap_or_default());
    let replica_path = env::var("DATABASE_REPLICA_PATH").ok();

    let is_network = ["libsql://", "https://", "http://"]
        .iter()
        .any(|prefix| url.starts_with(prefix));
    if !is_network {
        return Ok(Mode::Local { path: url });
    }
    // インターネット上の Turso にはトークンが必要（http:// は手元のテスト用サーバー）
    if !url.starts_with("http://") && token.expose().is_empty() {
        return Err(format!("{url} につなぐには DATABASE_AUTH_TOKEN が必要です"));
    }
    Ok(match replica_path {
        Some(path) => Mode::Replica { path, url, token },
        None => Mode::Remote { url, token },
    })
}

// 決めた形で DB を開く
async fn open(mode: Mode) -> Result<Database, libsql::Error> {
    match mode {
        Mode::Local { path } => Builder::new_local(path).build().await,
        Mode::Remote { url, token } => {
            Builder::new_remote(url, token.expose().to_string())
                .build()
                .await
        }
        Mode::Replica { path, url, token } => {
            Builder::new_remote_replica(path, url, token.expose().to_string())
                // 60 秒ごとに、裏で自動的に sync する
                .sync_interval(Duration::from_secs(60))
                .build()
                .await
        }
    }
}

#[tokio::main]
async fn main() {
    let mode = match mode_from_env() {
        Ok(mode) => mode,
        Err(message) => {
            eprintln!("設定エラー: {message}");
            std::process::exit(1);
        }
    };
    // 設定の中身をまるごと表示する
    println!("選んだ形: {mode:?}");

    let started = Instant::now();
    let result = async {
        let db = open(mode).await?;
        let conn = db.connect()?;
        let mut rows = conn.query("SELECT 1 + 1", ()).await?;
        let row = rows.next().await?.expect("1行ある");
        row.get::<i64>(0)
    }
    .await;
    let ms = started.elapsed().as_millis();
    match result {
        Ok(n) => println!("SELECT 1 + 1 = {n}（{ms} ミリ秒）"),
        Err(e) => println!("失敗（{ms} ミリ秒）: {e}"),
    }
}

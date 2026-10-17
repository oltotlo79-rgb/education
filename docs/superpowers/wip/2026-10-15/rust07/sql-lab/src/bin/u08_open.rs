use std::env;
use std::time::{Duration, Instant};

use libsql::{Builder, Database};

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
        token: String,
    },
    // 手元にコピーを持ち、読むのはコピー・書くのは向こう
    Replica {
        path: String,
        url: String,
        token: String,
    },
}

// 環境変数から、どの形で開くかを決める
fn mode_from_env() -> Result<Mode, String> {
    let url = env::var("DATABASE_URL").unwrap_or_else(|_| "u08_app.db".to_string());
    let token = env::var("DATABASE_AUTH_TOKEN").unwrap_or_default();
    let replica_path = env::var("DATABASE_REPLICA_PATH").ok();

    let is_network = ["libsql://", "https://", "http://"]
        .iter()
        .any(|prefix| url.starts_with(prefix));
    if !is_network {
        return Ok(Mode::Local { path: url });
    }
    // インターネット上の Turso にはトークンが必要（http:// は手元のテスト用サーバー）
    if !url.starts_with("http://") && token.is_empty() {
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
        Mode::Remote { url, token } => Builder::new_remote(url, token).build().await,
        Mode::Replica { path, url, token } => {
            Builder::new_remote_replica(path, url, token)
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
    // トークンが表示されないよう、形の名前だけを出す
    let name = match &mode {
        Mode::Local { .. } => "ローカル",
        Mode::Remote { .. } => "リモート",
        Mode::Replica { .. } => "組み込みレプリカ",
    };
    println!("選んだ形: {name}");

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

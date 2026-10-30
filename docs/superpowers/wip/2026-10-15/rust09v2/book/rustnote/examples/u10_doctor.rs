//! 第9巻 UNIT 10: 起動に要るものを、起動の順番どおりに1つずつ確かめて、どこでつまずくかを表示する。
//! 実行: cargo run --example u10_doctor

use rustnote::{
    config::{Config, listen_address},
    db::{Db, MIGRATIONS},
};
use topcoat::asset::AssetBundle;

/// 1つの確かめの結果を表示し、通ったかを返す
fn report(name: &str, result: Result<String, String>) -> bool {
    match result {
        Ok(detail) => {
            println!("✓ {name}: {detail}");
            true
        }
        Err(hint) => {
            println!("✗ {name}: {hint}");
            false
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let mut all_ok = true;

    // 1. 設定
    let config = match Config::from_env() {
        Ok(config) => {
            report(
                "設定",
                Ok(format!("DATABASE_URL = {}", config.database_url)),
            );
            config
        }
        Err(error) => {
            report("設定", Err(format!("{error}（環境変数か .env を直す）")));
            std::process::exit(1);
        }
    };

    // 2. アセットの束（アプリ本体 target/debug/rustnote.exe の隣の assets を見る）
    let bundle_dir = std::env::current_exe()
        .ok()
        .and_then(|exe| Some(exe.parent()?.parent()?.join("assets")));
    let bundle = match bundle_dir {
        Some(dir) => AssetBundle::load_dir(&dir)
            .map(|_| "ある".to_string())
            .map_err(|_| "無い（topcoat asset bundle を実行する）".to_string()),
        None => Err("場所が分からない".to_string()),
    };
    all_ok &= report("アセットの束", bundle);

    // 3〜5. DB を開く・届く・表が最新か
    match Db::open(&config).await {
        Ok(db) => {
            report("DB を開く", Ok("開けた".to_string()));
            let reached = db
                .ping()
                .await
                .map(|_| "届く".to_string())
                .map_err(|error| format!("届かない（{error}）"));
            if report("DB に届く", reached) {
                let applied = applied_count(&db).await;
                let result = if applied == MIGRATIONS.len() {
                    Ok(format!("{applied}/{} 適用済み", MIGRATIONS.len()))
                } else {
                    // 起動すれば migrate が流すので、失敗ではなく知らせるだけ
                    Ok(format!(
                        "{applied}/{} 適用済み（起動のときに残りが流れる）",
                        MIGRATIONS.len()
                    ))
                };
                report("マイグレーション", result);
            } else {
                all_ok = false;
            }
        }
        Err(error) => {
            all_ok &= report(
                "DB を開く",
                Err(format!("{error}（DATABASE_URL のフォルダを確かめる）")),
            );
        }
    }

    // 6. 待ち受ける場所（押さえられるかだけを試し、すぐに放す）
    let address = listen_address();
    let listen = tokio::net::TcpListener::bind(&address)
        .await
        .map(|_| format!("{address} は空いている"))
        .map_err(|error| {
            format!("{address} を使えない（{error}）。ほかのプログラムが使っていないか確かめる")
        });
    all_ok &= report("待ち受ける場所", listen);

    if !all_ok {
        std::process::exit(1);
    }
}

/// schema_migrations に記録された版の数（表が無ければ 0）
async fn applied_count(db: &Db) -> usize {
    let Ok(conn) = db.connect().await else {
        return 0;
    };
    let Ok(mut rows) = conn
        .query("SELECT COUNT(*) FROM schema_migrations", ())
        .await
    else {
        return 0;
    };
    match rows.next().await {
        Ok(Some(row)) => row.get::<i64>(0).map(|n| n as usize).unwrap_or(0),
        _ => 0,
    }
}

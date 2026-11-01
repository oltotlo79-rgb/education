//! webapp.exe の入り口。中身は lib.rs の側にある（テストからも使えるように）

use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match webapp::cli::run(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            // {:#} で、原因の連なり（「つなげません: ～」）まで出す
            eprintln!("エラー: {e:#}");
            ExitCode::FAILURE
        }
    }
}

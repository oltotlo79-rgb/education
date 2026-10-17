use tracing::{error, info};

fn main() {
    // 受け取り係（subscriber）を登録し忘れた
    info!("起動しました");
    error!("DB に接続できません");
    println!("main の最後まで来ました");
}

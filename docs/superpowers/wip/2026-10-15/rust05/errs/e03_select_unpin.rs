use std::time::Duration;

#[tokio::main]
async fn main() {
    // 全体の締め切り（1 秒後）
    let deadline = tokio::time::sleep(Duration::from_secs(1));
    let mut ticks = 0;
    loop {
        tokio::select! {
            // 同じ締め切りを、ループのたびに見張りたい
            _ = &mut deadline => {
                println!("締め切りです");
                break;
            }
            _ = tokio::time::sleep(Duration::from_millis(300)) => {
                ticks += 1;
                println!("{ticks} 回目の仕事");
            }
        }
    }
}

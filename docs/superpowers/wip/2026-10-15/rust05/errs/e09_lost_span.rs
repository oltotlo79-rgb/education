use tracing::{Instrument, info, info_span};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().without_time().init();

    let span = info_span!("request", request_id = 7);
    async {
        info!("リクエストの処理を始めます");
        // ① そのまま spawn: 新しいタスクには、今のスパンが引き継がれない
        tokio::spawn(async {
            info!("① 裏でメールを送りました");
        })
        .await
        .unwrap();
        // ② in_current_span: 今のスパンを付けてから spawn する
        tokio::spawn(
            async {
                info!("② 裏でメールを送りました");
            }
            .in_current_span(),
        )
        .await
        .unwrap();
    }
    .instrument(span)
    .await;
}

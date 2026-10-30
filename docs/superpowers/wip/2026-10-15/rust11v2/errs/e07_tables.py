# 第11巻 UNIT 07: RustNote のコピーに、tracing の構造化ログと /metrics を足す書き換え。

# src/routes.rs: /metrics のモジュールを足し、アクセスログを tracing と数えることに替える
ROUTES = [
    ("pub mod logout;\npub mod notes;\n", "pub mod logout;\npub mod metrics;\npub mod notes;\n"),
    (
        """/// アクセスログ: 「GET /notes -> 200 OK (1.2ms)」のように標準エラー出力へ書く
#[layer]
async fn access_log(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let started = std::time::Instant::now();
    let result = next.run(cx, body).await;
    // ヘルスチェックは数十秒ごとに来るので、ログに出さない
    if uri(cx).path() != "/health" {
        let status = match &result {
            Ok(response) => response.status().to_string(),
            // ページが Err(see_other(..)) で返したリダイレクトは、ここではまだ Err に見える
            Err(error) if error.downcast_ref::<SeeOther>().is_some() => "303 See Other".to_string(),
            Err(error) => format!("error ({error})"),
        };
        eprintln!(
            "{} {} -> {} ({:?})",
            method(cx),
            uri(cx).path(),
            status,
            started.elapsed()
        );
    }
    result
}
""",
        """/// アクセスログ: 1つのリクエストを、項目に分けた1件の記録（構造化ログ）にし、数える
#[layer]
async fn access_log(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let started = std::time::Instant::now();
    let result = next.run(cx, body).await;
    // ヘルスチェックと、数を読みに来る /metrics は、ログにも数にも入れない
    if uri(cx).path() != "/health" && uri(cx).path() != "/metrics" {
        let elapsed = started.elapsed();
        let status: u16 = match &result {
            Ok(response) => response.status().as_u16(),
            // ページが Err(see_other(..)) で返したリダイレクトは、ここではまだ Err に見える
            Err(error) if error.downcast_ref::<SeeOther>().is_some() => 303,
            Err(_) => 500,
        };
        tracing::info!(
            method = %method(cx),
            path = uri(cx).path(),
            status,
            elapsed_ms = elapsed.as_secs_f64() * 1000.0,
            "request"
        );
        metrics::METRICS.record(status, elapsed);
    }
    result
}
""",
    ),
]

# src/views.rs: 500 の原因も tracing で書く
VIEWS = [
    (
        """        eprintln!("[error] {}", causes.join(" <- "));""",
        """        tracing::error!(causes = causes.join(" <- "), "internal error");""",
    ),
]

# src/main.rs: 起動のときに、JSON で書く subscriber を入れる
MAIN = [
    (
        """    // .env が無くてもエラーにしない（本番では環境変数を直接設定する）
    dotenvy::dotenv().ok();
""",
        """    // .env が無くてもエラーにしない（本番では環境変数を直接設定する）
    dotenvy::dotenv().ok();
    // ログを JSON の1行ずつで標準出力へ。RUST_LOG で量を変えられる（既定は info）
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
""",
    ),
]

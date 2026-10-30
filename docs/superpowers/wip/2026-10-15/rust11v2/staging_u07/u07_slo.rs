//! 第11巻 UNIT 07: 1日分のつもりのリクエスト（1000 回）を送り、/metrics の数から、
//! SLO（守ると決めた目標）を守れたかと、エラーの予算（あと何回まで失敗してよいか）を計算する。
//! （RustNote のコピーに、tracing と /metrics を足した版で動かす）
//!
//! 実行: cargo test --test u07_slo -- --nocapture

mod common;

use common::{body_text, test_app};

/// /metrics の本文から、名前とラベルが合う行の数を読む
fn read(metrics: &str, name: &str) -> f64 {
    metrics
        .lines()
        .find(|line| line.starts_with(name) && line[name.len()..].starts_with(' '))
        .and_then(|line| line.rsplit(' ').next())
        .and_then(|value| value.parse().ok())
        .unwrap_or(0.0)
}

#[tokio::test]
async fn one_day_of_traffic() {
    let app = test_app().await;
    let alice = app.sign_up("alice@example.com", "correct horse battery").await;

    // 900 回は一覧を見る、97 回はノートを作る、3 回は NUL で始まる題（UNIT 04 のバグ）
    for i in 0..1000 {
        if i % 100 == 0 && i < 300 {
            app.post_form("/notes", "title=%00&body=", Some(&alice)).await;
        } else if i % 10 == 0 {
            let form = format!("title=memo{i}&body=");
            app.post_form("/notes", &form, Some(&alice)).await;
        } else {
            app.get("/notes", Some(&alice)).await;
        }
    }

    let metrics = body_text(app.get("/metrics", None).await).await;
    println!("{metrics}");

    let total = read(&metrics, "rustnote_request_duration_seconds_count");
    let errors = read(&metrics, r#"rustnote_requests_total{class="5xx"}"#);
    let under_50ms = read(&metrics, r#"rustnote_request_duration_seconds_bucket{le="0.05"}"#);
    let availability = (total - errors) / total * 100.0;
    let fast = under_50ms / total * 100.0;
    // SLO: 成功が 99.9% 以上・95% のリクエストが 50 ms 以内
    let budget = total * (1.0 - 0.999);
    println!("--- SLO の確かめ（登録の1回を含めて {total} 回）");
    println!("成功の割合: {availability:.2}%（目標 99.9% 以上）");
    println!("50 ms 以内の割合: {fast:.1}%（目標 95% 以上）");
    println!("エラーの予算: {budget:.0} 回まで → 使った回数 {errors} 回（{:.0}%）", errors / budget * 100.0);
}

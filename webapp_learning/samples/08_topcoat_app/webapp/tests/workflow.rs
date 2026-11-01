//! 依頼の流れのテスト（06 の Django 版の tests.py と同じ決まりを、Rust 版でも確かめる）
//!
//! `#[sqlx::test]` は、テストごとに空のデータベースを作り、migrations を流してから、`pool` を渡してくれる。
//! 動かし方: DATABASE_URL に、データベースを作ってよいユーザーの接続先を入れて `cargo test`

use std::{path::PathBuf, sync::Arc};

use sqlx::PgPool;
use time::macros::{date, datetime};
use topcoat::{
    context::{Cx, CxTestBuilder},
    mail::{MailConfig, MemoryTransport, TextBody},
};
use webapp::{
    config::{AppEnv, Config, MailSettings},
    jobs,
    models::{Employee, Status},
    state::AppState,
    upload::UploadedFile,
    workflow::{self, RequestInput, WorkflowError},
};

/// テストの舞台: 社員 4 人と、送ったメールをためる入れ物
struct World {
    cx: Cx,
    sent: MemoryTransport,
    pool: PgPool,
    file_dir: PathBuf,
    requester: Employee,
    a1: Employee,
    a2: Employee,
    worker: Employee,
}

async fn add_employee(pool: &PgPool, no: &str, name: &str, email: &str, role: &str) -> Employee {
    sqlx::query_as(
        "INSERT INTO employees (employee_no, name, dept, email, role) VALUES ($1, $2, '設計部', $3, $4)
         RETURNING id, employee_no, name, dept, job_title, email, role",
    )
    .bind(no)
    .bind(name)
    .bind(email)
    .bind(role)
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn world(pool: PgPool, redirect: Option<&str>) -> World {
    let requester = add_employee(&pool, "000001", "依頼 一郎", "req@example.com", "member").await;
    let a1 = add_employee(&pool, "000002", "承認 一子", "a1@example.com", "member").await;
    let a2 = add_employee(&pool, "000003", "承認 二郎", "a2@example.com", "member").await;
    let worker = add_employee(&pool, "000004", "担当 三郎", "w@example.com", "worker").await;

    let mut random = [0u8; 8];
    getrandom::fill(&mut random).unwrap();
    let file_dir = std::env::temp_dir().join(format!("webapp-test-{}", hex::encode(random)));
    let config = Config {
        app_env: AppEnv::Development,
        base_url: "http://webapp.example.local".into(),
        database_url: String::new(),
        file_dir: file_dir.clone(),
        mail: MailSettings::File("unused".into()),
        mail_from: "noreply@example.com".into(),
        mail_redirect_to: redirect.map(Into::into),
    };
    // メールは送らずに、メモリーにためる（あとで中身を確かめる）
    let sent = MemoryTransport::new();
    let cx = CxTestBuilder::new()
        .app_context(AppState { pool: pool.clone(), config: Arc::new(config) })
        .app_context(MailConfig::builder().transport(sent.clone()).build())
        .build();
    World { cx, sent, pool, file_dir, requester, a1, a2, worker }
}

/// 送ったメールの（宛先, 件名, 本文）
fn mails(w: &World) -> Vec<(String, String, String)> {
    w.sent
        .sent()
        .iter()
        .map(|m| {
            let body = match m.text() {
                TextBody::Text(t) => t.clone(),
                _ => String::new(),
            };
            (m.to()[0].address().to_string(), m.subject().to_string(), body)
        })
        .collect()
}

/// メールの本文から、承認の URL の文字列を取り出す
fn token_in(body: &str) -> String {
    body.split("/approve/").nth(1).expect("URL がありません").lines().next().unwrap().trim().to_string()
}

fn input(approvers: &[&Employee]) -> RequestInput {
    RequestInput {
        title: "治具の製作".into(),
        detail: "検査用".into(),
        due_date: date!(2026 - 11 - 30),
        approver_ids: approvers.iter().map(|a| a.id).collect(),
        drawing: None,
        files: Vec::new(),
    }
}

async fn submit(w: &World) -> i64 {
    workflow::submit(&w.cx, &w.requester, input(&[&w.a1, &w.a2])).await.unwrap()
}

async fn status(w: &World, id: i64) -> Status {
    let s: String = sqlx::query_scalar("SELECT status FROM requests WHERE id = $1").bind(id).fetch_one(&w.pool).await.unwrap();
    Status::try_from(s).unwrap()
}

fn rule_message<T: std::fmt::Debug>(r: Result<T, WorkflowError>) -> String {
    match r {
        Err(WorkflowError::Rule(m)) => m,
        other => panic!("Rule のエラーのはずが {other:?}"),
    }
}

#[sqlx::test]
async fn 出すと1人目にメールが届き回覧中になる(pool: PgPool) {
    let w = world(pool, None).await;
    let id = submit(&w).await;
    assert_eq!(status(&w, id).await, Status::Circulating);
    let m = mails(&w);
    assert_eq!(m.len(), 1);
    assert_eq!(m[0].0, "a1@example.com");
    assert_eq!(m[0].1, "【依頼・承認】承認のお願い: 治具の製作");
    assert!(m[0].2.contains("http://webapp.example.local/approve/"));
}

#[sqlx::test]
async fn 全員が承認すると回覧完了になり担当者に知らせる(pool: PgPool) {
    let w = world(pool, None).await;
    let id = submit(&w).await;
    workflow::approve(&w.cx, &token_in(&mails(&w)[0].2)).await.unwrap();
    assert_eq!(mails(&w)[1].0, "a2@example.com"); // 2 人目へ
    workflow::approve(&w.cx, &token_in(&mails(&w)[1].2)).await.unwrap();
    assert_eq!(status(&w, id).await, Status::Approved);
    let mut to: Vec<String> = mails(&w)[2..].iter().map(|m| m.0.clone()).collect();
    to.sort();
    assert_eq!(to, ["req@example.com", "w@example.com"]);
}

#[sqlx::test]
async fn urlは1回しか使えない(pool: PgPool) {
    let w = world(pool, None).await;
    submit(&w).await;
    let token = token_in(&mails(&w)[0].2);
    workflow::approve(&w.cx, &token).await.unwrap();
    assert_eq!(rule_message(workflow::approve(&w.cx, &token).await), "この依頼は、承認済みです。");
}

#[sqlx::test]
async fn 同時に2回押しても1回だけ進む(pool: PgPool) {
    let w = world(pool, None).await;
    let id = submit(&w).await;
    let token = token_in(&mails(&w)[0].2);
    // 2 つの承認を、同時に走らせる。鍵（FOR UPDATE）のおかげで、あとの方は「承認済み」になる
    let (r1, r2) = tokio::join!(workflow::approve(&w.cx, &token), workflow::approve(&w.cx, &token));
    assert_eq!([r1.is_ok(), r2.is_ok()].iter().filter(|ok| **ok).count(), 1);
    let approved: i64 = sqlx::query_scalar("SELECT count(*) FROM steps WHERE request_id = $1 AND decision = 'approved'")
        .bind(id)
        .fetch_one(&w.pool)
        .await
        .unwrap();
    assert_eq!(approved, 1);
    assert_eq!(mails(&w).len(), 2); // 1 人目へ 1 通・2 人目へ 1 通（2 人目へ 2 通にはならない）
}

#[sqlx::test]
async fn 期限が切れたurlは使えない(pool: PgPool) {
    let w = world(pool, None).await;
    submit(&w).await;
    sqlx::query("UPDATE steps SET token_expires_at = now() - interval '1 second'").execute(&w.pool).await.unwrap();
    let m = rule_message(workflow::approve(&w.cx, &token_in(&mails(&w)[0].2)).await);
    assert!(m.contains("期限が切れています"), "{m}");
}

#[sqlx::test]
async fn 差し戻すとコメントが追記され依頼者にメールが届く(pool: PgPool) {
    let w = world(pool, None).await;
    let id = submit(&w).await;
    workflow::return_request(&w.cx, &token_in(&mails(&w)[0].2), "寸法を追記してください").await.unwrap();
    assert_eq!(status(&w, id).await, Status::Returned);
    let comments: String =
        sqlx::query_scalar("SELECT return_comments FROM requests WHERE id = $1").bind(id).fetch_one(&w.pool).await.unwrap();
    // 【11/02 09:00 回覧者1 承認 一子】寸法を追記してください（日時の所は、動かした時刻になる）
    assert!(comments.starts_with('【'), "{comments}");
    assert!(comments.ends_with(" 回覧者1 承認 一子】寸法を追記してください"), "{comments}");
    assert_eq!(mails(&w).last().unwrap().0, "req@example.com");
}

#[sqlx::test]
async fn 理由の無い差し戻しは断る(pool: PgPool) {
    let w = world(pool, None).await;
    submit(&w).await;
    let m = rule_message(workflow::return_request(&w.cx, &token_in(&mails(&w)[0].2), "  ").await);
    assert_eq!(m, "差し戻すときは、理由を書いてください。");
}

#[sqlx::test]
async fn 引き戻すと承認待ちの人のurlが使えなくなる(pool: PgPool) {
    let w = world(pool, None).await;
    let id = submit(&w).await;
    let token = token_in(&mails(&w)[0].2);
    workflow::pull_back(&w.cx, &w.requester, id).await.unwrap();
    let mut conn = w.pool.acquire().await.unwrap();
    assert_eq!(rule_message(workflow::check_token(&mut conn, &token, false).await), "依頼者により引き戻されました。");
}

#[sqlx::test]
async fn 出し直すと最初の回覧者からやり直す(pool: PgPool) {
    let w = world(pool, None).await;
    let id = submit(&w).await;
    workflow::approve(&w.cx, &token_in(&mails(&w)[0].2)).await.unwrap(); // 1 人目は承認済み
    workflow::return_request(&w.cx, &token_in(&mails(&w)[1].2), "図面を直して").await.unwrap();
    let fixed = RequestInput { title: "治具の製作（直し）".into(), due_date: date!(2026 - 12 - 05), ..input(&[&w.a1, &w.a2]) };
    workflow::resubmit(&w.cx, &w.requester, id, fixed).await.unwrap();
    let (s, round): (String, i16) =
        sqlx::query_as("SELECT status, round FROM requests WHERE id = $1").bind(id).fetch_one(&w.pool).await.unwrap();
    assert_eq!((s.as_str(), round), ("circulating", 2));
    assert_eq!(mails(&w).last().unwrap().0, "a1@example.com");
}

#[sqlx::test]
async fn ほかの人の依頼は引き戻せない(pool: PgPool) {
    let w = world(pool, None).await;
    let id = submit(&w).await;
    assert_eq!(rule_message(workflow::pull_back(&w.cx, &w.a1, id).await), "自分の依頼ではありません。");
}

#[sqlx::test]
async fn 回覧者の決まり(pool: PgPool) {
    let w = world(pool, None).await;
    let twice = workflow::submit(&w.cx, &w.requester, input(&[&w.a1, &w.a1])).await;
    assert_eq!(rule_message(twice), "同じ人を 2 回は選べません。");
    let me = workflow::submit(&w.cx, &w.requester, input(&[&w.requester])).await;
    assert_eq!(rule_message(me), "自分自身は回覧者にできません。");
    let four = workflow::submit(&w.cx, &w.requester, input(&[&w.a1, &w.a2, &w.worker, &w.worker])).await;
    assert_eq!(rule_message(four), "回覧者は 1〜3 人にしてください。");
    assert!(mails(&w).is_empty()); // 断ったときは、メールも送らない
}

#[sqlx::test]
async fn 検証ではメールの宛先を書きかえる(pool: PgPool) {
    let w = world(pool, Some("it@example.com")).await;
    submit(&w).await;
    let m = &mails(&w)[0];
    assert_eq!(m.0, "it@example.com");
    assert!(m.1.starts_with("【検証】"));
    assert!(m.2.contains("本来の宛先: a1@example.com"));
}

#[sqlx::test]
async fn 担当を決めて完了にすると依頼者に知らせる(pool: PgPool) {
    let w = world(pool, None).await;
    let id = submit(&w).await;
    workflow::approve(&w.cx, &token_in(&mails(&w)[0].2)).await.unwrap();
    workflow::approve(&w.cx, &token_in(&mails(&w)[1].2)).await.unwrap();
    // 一般の人は担当になれない
    assert_eq!(rule_message(workflow::assign(&w.cx, &w.a1, id).await), "担当者だけができます。");
    workflow::assign(&w.cx, &w.worker, id).await.unwrap();
    workflow::complete(&w.cx, &w.worker, id).await.unwrap();
    assert_eq!(status(&w, id).await, Status::Done);
    let last = mails(&w).last().unwrap().clone();
    assert_eq!((last.0.as_str(), last.1.as_str()), ("req@example.com", "【依頼・承認】完了: 治具の製作"));
}

#[sqlx::test]
async fn 添付ファイルはフォルダに置き_失敗したら消す(pool: PgPool) {
    let w = world(pool, None).await;
    let png = UploadedFile { name: "写真.png".into(), bytes: b"\x89PNG\r\n\x1a\n....".to_vec() };
    let ok = RequestInput { files: vec![png.clone()], ..input(&[&w.a1]) };
    let id = workflow::submit(&w.cx, &w.requester, ok).await.unwrap();
    let stored: String =
        sqlx::query_scalar("SELECT stored_name FROM attachments WHERE request_id = $1").bind(id).fetch_one(&w.pool).await.unwrap();
    let dir = w.file_dir.join("attachments");
    assert!(dir.join(&stored).exists());

    // 回覧者の決まりで断られたら、書いたファイルも残さない
    let bad = RequestInput { files: vec![png], ..input(&[&w.requester]) };
    assert!(workflow::submit(&w.cx, &w.requester, bad).await.is_err());
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
    let _ = std::fs::remove_dir_all(&w.file_dir);
}

#[sqlx::test]
async fn 毎朝の通知は止まっている承認を送り直す(pool: PgPool) {
    let w = world(pool, None).await;
    submit(&w).await;
    let old = token_in(&mails(&w)[0].2);
    // 4 日前に送ったことにする
    sqlx::query("UPDATE steps SET sent_at = now() - interval '4 days' WHERE sent_at IS NOT NULL").execute(&w.pool).await.unwrap();
    let report = jobs::daily_reminders(&w.cx).await.unwrap();
    assert_eq!(report, jobs::Report { resent: 1, due_soon: 0 });
    let m = mails(&w);
    assert_eq!(m[1].0, "a1@example.com");
    // 前の URL は使えなくなり、新しい URL で承認できる
    assert_eq!(rule_message(workflow::approve(&w.cx, &old).await), "この URL は使えません。");
    workflow::approve(&w.cx, &token_in(&m[1].2)).await.unwrap();
}

#[sqlx::test]
async fn 毎朝の通知は8時を過ぎてから1日1回だけ動く(pool: PgPool) {
    let w = world(pool, None).await;
    assert!(jobs::run_daily_if_due(&w.cx, datetime!(2026-11-02 07:59 +9)).await.unwrap().is_none());
    assert!(jobs::run_daily_if_due(&w.cx, datetime!(2026-11-02 08:00 +9)).await.unwrap().is_some());
    assert!(jobs::run_daily_if_due(&w.cx, datetime!(2026-11-02 13:00 +9)).await.unwrap().is_none()); // 同じ日の 2 回目
    assert!(jobs::run_daily_if_due(&w.cx, datetime!(2026-11-03 08:01 +9)).await.unwrap().is_some()); // 次の日
}

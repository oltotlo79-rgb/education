//! 毎朝 8:00 の通知（onprem の 04 の 12.1）
//!
//! 1. 承認のお願いを送ってから 3 日たっても止まっている回覧 → 承認者に送り直す（前の URL は使えなくなる）
//! 2. 希望の納期まで 3 日以内なのに、まだ終わっていない依頼 → 依頼者と担当者に知らせる
//!
//! Django 版は、Windows のタスク スケジューラから動かした。Topcoat 版は **アプリの中で** 動かす:
//! 1 分ごとに「8:00 を過ぎたか・今日の分は済んだか」を見る。8:00 にアプリが止まっていても、
//! 起動したあとの最初の確かめで、今日の分を動かす（`job_runs` の表で、同じ日に 2 回は動かない）。

use std::{sync::Arc, time::Duration};

use time::{OffsetDateTime, Time};
use topcoat::context::{AppContext, Cx};

use crate::{
    mail::Outbox,
    models::{JST, Status, fmt_date, today_jst},
    state::state,
    workflow::{self, WfResult, WorkflowError},
};

const JOB: &str = "daily-reminders";
pub const STALE_DAYS: i32 = 3;
pub const DUE_SOON_DAYS: i64 = 3;
/// 何時に動かすか（日本の時刻）
pub const RUN_AT: Time = time::macros::time!(8:00);

/// 納期が近い依頼の 1 行
#[derive(sqlx::FromRow)]
struct DueSoon {
    title: String,
    due_date: time::Date,
    #[sqlx(try_from = "String")]
    status: Status,
    requester_email: String,
    assignee_email: Option<String>,
    assignee_name: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Report {
    pub resent: usize,
    pub due_soon: usize,
}

/// 通知を 1 回分おこなう（コマンドの send-daily-reminders からも呼ぶ）
pub async fn daily_reminders(cx: &Cx) -> WfResult<Report> {
    let pool = &state(cx).pool;

    // 1. 止まっている承認
    let stale: Vec<i64> = sqlx::query_scalar(
        "SELECT s.id FROM steps s JOIN requests r ON r.id = s.request_id
          WHERE s.decision = 'waiting' AND r.status = 'circulating'
            AND s.sent_at < now() - make_interval(days => $1)
          ORDER BY s.id",
    )
    .bind(STALE_DAYS)
    .fetch_all(pool)
    .await?;
    let mut resent = 0;
    for step_id in stale {
        match workflow::resend(cx, None, step_id).await {
            Ok(()) => resent += 1,
            // 調べてから送るまでの間に、承認されたなど。飛ばして次へ
            Err(WorkflowError::Rule(m)) => eprintln!("送り直しを飛ばしました（回覧 {step_id}）: {m}"),
            Err(e) => return Err(e),
        }
    }

    // 2. 納期が近い依頼
    let limit = today_jst() + time::Duration::days(DUE_SOON_DAYS);
    let rows: Vec<DueSoon> = sqlx::query_as(
        "SELECT r.title, r.due_date, r.status, q.email AS requester_email, w.email AS assignee_email, w.name AS assignee_name
           FROM requests r
           JOIN employees q ON q.id = r.requester_id
           LEFT JOIN employees w ON w.id = r.assignee_id
          WHERE r.status IN ('approved', 'in_progress') AND r.due_date <= $1
          ORDER BY r.due_date, r.id",
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;
    let mut outbox = Outbox::new();
    for r in &rows {
        let assignee = r.assignee_name.as_ref().map(|n| format!("（担当: {n}）")).unwrap_or_default();
        let body =
            format!("依頼「{}」の希望の納期は {} です。\n今の状態: {}{assignee}\n", r.title, fmt_date(r.due_date), r.status.label());
        let mut to = vec![r.requester_email.clone()];
        if let Some(a) = r.assignee_email.as_ref().filter(|a| **a != r.requester_email) {
            to.push(a.clone()); // 依頼者と担当者が同じ人なら、1 通だけ
        }
        for t in to {
            outbox.add(&t, &format!("納期が近づいています: {}", r.title), body.clone());
        }
    }
    outbox.send_all(cx).await;
    Ok(Report { resent, due_soon: rows.len() })
}

/// 8:00 を過ぎていて、今日の分がまだなら動かす。動かしたら Some
pub async fn run_daily_if_due(cx: &Cx, now: OffsetDateTime) -> WfResult<Option<Report>> {
    let now = now.to_offset(JST);
    if now.time() < RUN_AT {
        return Ok(None);
    }
    let pool = &state(cx).pool;
    // 「今日の分は自分がやる」と、先に 1 行入れる。もう入っていれば（済んでいれば）何もしない
    let claimed = sqlx::query("INSERT INTO job_runs (job, run_date) VALUES ($1, $2) ON CONFLICT DO NOTHING")
        .bind(JOB)
        .bind(now.date())
        .execute(pool)
        .await?
        .rows_affected()
        == 1;
    if !claimed {
        return Ok(None);
    }
    match daily_reminders(cx).await {
        Ok(report) => Ok(Some(report)),
        Err(e) => {
            // 失敗したら、印を消して、次の確かめ（1 分後）でやり直す
            sqlx::query("DELETE FROM job_runs WHERE job = $1 AND run_date = $2").bind(JOB).bind(now.date()).execute(pool).await?;
            Err(e)
        }
    }
}

/// サーバーの中で、1 分ごとに確かめ続ける（serve のときに 1 回だけ呼ぶ）
pub fn spawn_daily(app: Arc<AppContext>) {
    tokio::spawn(async move {
        let cx = Cx::new(app); // 画面のリクエストの外で、メールの設定などを使うための Cx
        loop {
            match run_daily_if_due(&cx, OffsetDateTime::now_utc()).await {
                Ok(Some(r)) => println!("毎朝の通知: 送り直した承認のお願い {} 件・納期が近い依頼 {} 件", r.resent, r.due_soon),
                Ok(None) => {}
                Err(e) => eprintln!("毎朝の通知に失敗しました（1 分後にやり直します）: {e}"),
            }
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });
}

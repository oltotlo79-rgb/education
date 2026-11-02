//! 依頼の状態を変える処理を、ここに集める（06 の Django 版の services.py と同じ動き）
//!
//! - 画面からは、ここの関数だけを呼ぶ。状態を直接書きかえない。
//! - どの関数も 1 つのトランザクションの中で動き、途中で失敗したら全部取り消す（`?` で抜けると、tx が捨てられて取り消しになる）。
//! - 同じ依頼を 2 人が同時に操作しても 1 回だけ進むように、行に鍵をかけてから（`FOR UPDATE`）読む。
//! - メールは「送る物の箱」に入れておき、`commit` が成功してから送る（mail.rs）。

use std::fmt;

use sha2::{Digest, Sha256};
use sqlx::{PgConnection, Postgres, Transaction, types::Json};
use time::{Date, OffsetDateTime};
use topcoat::context::Cx;

use crate::{
    config::Config,
    drawing::Drawing,
    mail::Outbox,
    models::{Decision, Employee, JST, Role, Status, fmt_date},
    state::state,
    upload::{self, SavedFile, UploadedFile},
};

/// 承認の URL が使える日数
pub const TOKEN_DAYS: i32 = 14;
/// 回覧者の人数の上限
pub const MAX_APPROVERS: usize = 3;

// ---------- エラー ----------

/// できない操作（Rule）と、思いがけない失敗（Internal）を分ける。
/// Rule の文は、そのまま画面に出してよい。Internal は画面には「エラー」とだけ出し、中身はログに書く
#[derive(Debug)]
pub enum WorkflowError {
    Rule(String),
    Internal(String),
}

impl fmt::Display for WorkflowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkflowError::Rule(m) => write!(f, "{m}"),
            WorkflowError::Internal(m) => write!(f, "内部のエラー: {m}"),
        }
    }
}

impl std::error::Error for WorkflowError {}

impl From<sqlx::Error> for WorkflowError {
    fn from(e: sqlx::Error) -> Self {
        WorkflowError::Internal(e.to_string())
    }
}

impl From<std::io::Error> for WorkflowError {
    fn from(e: std::io::Error) -> Self {
        WorkflowError::Internal(e.to_string())
    }
}

pub type WfResult<T> = Result<T, WorkflowError>;

fn rule<T>(message: impl Into<String>) -> WfResult<T> {
    Err(WorkflowError::Rule(message.into()))
}

// ---------- 入力 ----------

/// 依頼を出す・出し直すときに、画面から受け取る物（形はもう確かめてある）
#[derive(Debug, Clone)]
pub struct RequestInput {
    pub title: String,
    pub detail: String,
    pub due_date: Date,
    pub approver_ids: Vec<i64>,
    pub drawing: Option<Drawing>,
    pub files: Vec<UploadedFile>,
}

// ---------- 承認の URL ----------

/// URL に入れる、推測できない文字列（32 バイトの乱数を 16 進で 64 文字）
fn new_token() -> String {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).expect("乱数を作れません");
    hex::encode(bytes)
}

/// 文字列の指紋（SHA-256）。データベースには、これだけを入れる
pub fn hash_token(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

/// URL で開いた回覧の 1 人分
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TokenStep {
    pub step_id: i64,
    pub request_id: i64,
    pub step_order: i16,
    #[sqlx(try_from = "String")]
    pub decision: Decision,
    pub token_expires_at: Option<OffsetDateTime>,
    pub approver_id: i64,
    pub approver_name: String,
    #[sqlx(try_from = "String")]
    pub request_status: Status,
}

/// URL の文字列を調べる。使えなければ、理由の文を Rule で返す。
/// `lock` が true なら、回覧と依頼の行に鍵をかける（承認・差し戻しのとき）
pub async fn check_token(conn: &mut PgConnection, token: &str, lock: bool) -> WfResult<TokenStep> {
    let sql = format!(
        "SELECT s.id AS step_id, s.request_id, s.step_order, s.decision, s.token_expires_at,
                s.approver_id, a.name AS approver_name, r.status AS request_status
           FROM steps s
           JOIN requests r ON r.id = s.request_id
           JOIN employees a ON a.id = s.approver_id
          WHERE s.token_hash = $1 {}",
        if lock { "FOR UPDATE OF s, r" } else { "" }
    );
    // sqlx 0.9 は、組み立てた SQL の文字を、そのままでは受け取らない（SQL インジェクションを防ぐため）。
    // ここで足すのは決まった文字（FOR UPDATE ...）だけで、外から来た値は $1 で渡しているので、
    // 「確かめた」という印の AssertSqlSafe で包む
    let step: Option<TokenStep> =
        sqlx::query_as(sqlx::AssertSqlSafe(sql)).bind(hash_token(token)).fetch_optional(&mut *conn).await?;
    let Some(step) = step else {
        return rule("この URL は使えません。");
    };
    match step.decision {
        Decision::Approved => return rule("この依頼は、承認済みです。"),
        Decision::Returned => return rule("この依頼は、差し戻し済みです。"),
        Decision::Cancelled if step.request_status == Status::PulledBack => return rule("依頼者により引き戻されました。"),
        Decision::Cancelled => return rule("この URL は、取り消されました。"),
        Decision::Waiting => {}
    }
    if step.token_expires_at.is_some_and(|t| t < OffsetDateTime::now_utc()) {
        return rule("この URL は、期限が切れています。管理者に再送を頼んでください。");
    }
    Ok(step)
}

// ---------- 小さな部品 ----------

type Tx = Transaction<'static, Postgres>;

/// 鍵をかけて読んだ依頼
#[derive(Debug, sqlx::FromRow)]
struct Locked {
    id: i64,
    title: String,
    #[sqlx(try_from = "String")]
    status: Status,
    requester_id: i64,
    assignee_id: Option<i64>,
    round: i16,
    return_comments: String,
}

async fn lock_request(tx: &mut Tx, id: i64) -> WfResult<Locked> {
    let r: Option<Locked> = sqlx::query_as(
        "SELECT id, title, status, requester_id, assignee_id, round, return_comments
           FROM requests WHERE id = $1 FOR UPDATE",
    )
    .bind(id)
    .fetch_optional(&mut **tx)
    .await?;
    match r {
        Some(r) => Ok(r),
        None => rule("依頼がありません。"),
    }
}

/// 自分の依頼だけを、鍵をかけて読む
async fn lock_own(tx: &mut Tx, user: &Employee, id: i64) -> WfResult<Locked> {
    let r = lock_request(tx, id).await?;
    if r.requester_id != user.id {
        return rule("自分の依頼ではありません。");
    }
    Ok(r)
}

/// 記録（だれが・何をした）を 1 行足す
async fn log(tx: &mut Tx, request_id: i64, actor_id: Option<i64>, action: &str, note: &str) -> WfResult<()> {
    sqlx::query("INSERT INTO histories (request_id, actor_id, action, note) VALUES ($1, $2, $3, $4)")
        .bind(request_id)
        .bind(actor_id)
        .bind(action)
        .bind(note)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

/// 回覧者の決まり（1〜3 人・同じ人を 2 回は選べない・自分は選べない・いる社員だけ）
async fn check_approvers(tx: &mut Tx, requester_id: i64, ids: &[i64]) -> WfResult<()> {
    if ids.is_empty() || ids.len() > MAX_APPROVERS {
        return rule(format!("回覧者は 1〜{MAX_APPROVERS} 人にしてください。"));
    }
    let mut unique = ids.to_vec();
    unique.sort();
    unique.dedup();
    if unique.len() != ids.len() {
        return rule("同じ人を 2 回は選べません。");
    }
    if ids.contains(&requester_id) {
        return rule("自分自身は回覧者にできません。");
    }
    let found: i64 = sqlx::query_scalar("SELECT count(*) FROM employees WHERE id = ANY($1) AND is_active")
        .bind(ids)
        .fetch_one(&mut **tx)
        .await?;
    if found != ids.len() as i64 {
        return rule("回覧者に、いない社員が入っています。");
    }
    Ok(())
}

/// 承認のお願いのメールに書く物
#[derive(sqlx::FromRow)]
struct MailInfo {
    approver_name: String,
    approver_email: String,
    step_order: i16,
    title: String,
    requester_name: String,
    requester_dept: String,
    due_date: Date,
}

/// 新しい URL を作り（前の URL は使えなくなる）、承認のお願いのメールを箱に入れる
async fn send_approval_mail(tx: &mut Tx, outbox: &mut Outbox, config: &Config, step_id: i64) -> WfResult<()> {
    let token = new_token();
    sqlx::query(
        "UPDATE steps SET token_hash = $2, token_expires_at = now() + make_interval(days => $3), sent_at = now()
          WHERE id = $1",
    )
    .bind(step_id)
    .bind(hash_token(&token))
    .bind(TOKEN_DAYS)
    .execute(&mut **tx)
    .await?;

    let m: MailInfo = sqlx::query_as(
        "SELECT a.name AS approver_name, a.email AS approver_email, s.step_order,
                r.title, q.name AS requester_name, q.dept AS requester_dept, r.due_date
           FROM steps s
           JOIN employees a ON a.id = s.approver_id
           JOIN requests r ON r.id = s.request_id
           JOIN employees q ON q.id = r.requester_id
          WHERE s.id = $1",
    )
    .bind(step_id)
    .fetch_one(&mut **tx)
    .await?;

    let url = format!("{}/approve/{token}", config.base_url);
    let body = format!(
        "{} さん\n\n次の依頼の、承認をお願いします。（回覧者{}）\n\n件名: {}\n依頼者: {}（{}）\n希望の納期: {}\n\n\
         下の URL を開いて、「承認」か「差し戻し」を選んでください。\n{url}\n\n\
         ・この URL は、あなただけのものです。ほかの人に転送しないでください。\n\
         ・この URL は 1 回だけ使えます。{TOKEN_DAYS} 日で使えなくなります。\n",
        m.approver_name, m.step_order, m.title, m.requester_name, m.requester_dept, fmt_date(m.due_date),
    );
    outbox.add(&m.approver_email, &format!("承認のお願い: {}", m.title), body);
    Ok(())
}

/// 回覧を始める（回覧者を並べて、1 人目に承認のお願い）
async fn start_round(
    tx: &mut Tx,
    outbox: &mut Outbox,
    config: &Config,
    request_id: i64,
    round: i16,
    approver_ids: &[i64],
) -> WfResult<()> {
    let mut first = None;
    for (i, approver_id) in approver_ids.iter().enumerate() {
        let step_id: i64 = sqlx::query_scalar(
            "INSERT INTO steps (request_id, round, step_order, approver_id) VALUES ($1, $2, $3, $4) RETURNING id",
        )
        .bind(request_id)
        .bind(round)
        .bind(i as i16 + 1)
        .bind(approver_id)
        .fetch_one(&mut **tx)
        .await?;
        first.get_or_insert(step_id);
    }
    if let Some(step_id) = first {
        send_approval_mail(tx, outbox, config, step_id).await?;
    }
    Ok(())
}

/// 添付ファイルをフォルダに書き、表に 1 行ずつ足す。書いたファイルは `saved` に入れる（失敗したときに消すため）
async fn add_files(tx: &mut Tx, config: &Config, request_id: i64, files: &[UploadedFile], saved: &mut Vec<SavedFile>) -> WfResult<()> {
    for file in files {
        let f = upload::save(&config.file_dir, file).await?;
        saved.push(f.clone());
        sqlx::query(
            "INSERT INTO attachments (request_id, stored_name, original_name, content_type, size) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(request_id)
        .bind(&f.stored_name)
        .bind(&f.original_name)
        .bind(f.content_type)
        .bind(f.size as i32)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

/// 保存が確定したら送る。失敗したら、書いたファイルを消す（データベースは tx を捨てて取り消し）
async fn finish<T>(cx: &Cx, result: WfResult<(T, Tx, Outbox)>, saved: &[SavedFile]) -> WfResult<T> {
    let committed = match result {
        Ok((value, tx, outbox)) => tx.commit().await.map(|_| (value, outbox)).map_err(WorkflowError::from),
        Err(e) => Err(e),
    };
    match committed {
        Ok((value, outbox)) => {
            outbox.send_all(cx).await; // ここで初めて送る
            Ok(value)
        }
        Err(e) => {
            upload::remove(&state(cx).config.file_dir, saved).await;
            Err(e)
        }
    }
}

// ---------- 状態を変える操作 ----------

/// 依頼を出す → 回覧中。1 人目に承認のお願いのメール
pub async fn submit(cx: &Cx, requester: &Employee, input: RequestInput) -> WfResult<i64> {
    let st = state(cx);
    let mut saved = Vec::new();
    let result = async {
        let mut tx = st.pool.begin().await?;
        let mut outbox = Outbox::new();
        check_approvers(&mut tx, requester.id, &input.approver_ids).await?;
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO requests (title, detail, due_date, requester_id, drawing) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        )
        .bind(&input.title)
        .bind(&input.detail)
        .bind(input.due_date)
        .bind(requester.id)
        .bind(input.drawing.as_ref().map(Json))
        .fetch_one(&mut *tx)
        .await?;
        start_round(&mut tx, &mut outbox, &st.config, id, 1, &input.approver_ids).await?;
        add_files(&mut tx, &st.config, id, &input.files, &mut saved).await?;
        log(&mut tx, id, Some(requester.id), "依頼を出した", "").await?;
        Ok((id, tx, outbox))
    }
    .await;
    finish(cx, result, &saved).await
}

/// 承認 → 次の人へ。最後の人なら回覧完了（依頼者と、担当の部署へメール）。依頼の件名を返す
pub async fn approve(cx: &Cx, token: &str) -> WfResult<String> {
    let st = state(cx);
    let result = async {
        let mut tx = st.pool.begin().await?;
        let mut outbox = Outbox::new();
        let step = check_token(&mut tx, token, true).await?; // 鍵をかけて読む。同時に押されても 1 回だけ進む
        let r = lock_request(&mut tx, step.request_id).await?;
        // 指紋は消さずに残す。同じ URL をもう一度開いた人に「承認済みです」と知らせるため
        sqlx::query("UPDATE steps SET decision = 'approved', decided_at = now() WHERE id = $1")
            .bind(step.step_id)
            .execute(&mut *tx)
            .await?;
        log(&mut tx, r.id, Some(step.approver_id), &format!("承認（回覧者{}）", step.step_order), "").await?;

        let next: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM steps WHERE request_id = $1 AND round = $2 AND decision = 'waiting' ORDER BY step_order LIMIT 1",
        )
        .bind(r.id)
        .bind(r.round)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(next_id) = next {
            send_approval_mail(&mut tx, &mut outbox, &st.config, next_id).await?;
        } else {
            sqlx::query("UPDATE requests SET status = 'approved', updated_at = now() WHERE id = $1")
                .bind(r.id)
                .execute(&mut *tx)
                .await?;
            let (name, email): (String, String) = sqlx::query_as("SELECT name, email FROM employees WHERE id = $1")
                .bind(r.requester_id)
                .fetch_one(&mut *tx)
                .await?;
            outbox.add(
                &email,
                &format!("回覧完了: {}", r.title),
                format!(
                    "{name} さん\n\n依頼「{}」は、全員の承認が終わりました（回覧完了）。\n\
                     担当の部署と、打ち合わせの日を決めてください。\n",
                    r.title
                ),
            );
            let workers: Vec<String> = sqlx::query_scalar("SELECT email FROM employees WHERE role = 'worker' AND is_active")
                .fetch_all(&mut *tx)
                .await?;
            for to in workers {
                outbox.add(
                    &to,
                    &format!("新しい依頼: {}", r.title),
                    format!("新しい依頼が届きました。\n\n件名: {}\n\nアプリの「依頼の一覧」から、担当を決めてください。\n", r.title),
                );
            }
        }
        Ok((r.title, tx, outbox))
    }
    .await;
    finish(cx, result, &[]).await
}

/// 差し戻し → 差し戻し中。コメントは【日時 回覧者n 氏名】を付けて追記する。依頼の件名を返す
pub async fn return_request(cx: &Cx, token: &str, comment: &str) -> WfResult<String> {
    let comment = comment.trim();
    if comment.is_empty() {
        return rule("差し戻すときは、理由を書いてください。");
    }
    let st = state(cx);
    let result = async {
        let mut tx = st.pool.begin().await?;
        let mut outbox = Outbox::new();
        let step = check_token(&mut tx, token, true).await?;
        let r = lock_request(&mut tx, step.request_id).await?;
        sqlx::query("UPDATE steps SET decision = 'returned', decided_at = now() WHERE id = $1")
            .bind(step.step_id)
            .execute(&mut *tx)
            .await?;
        // まだ回ってきていない人の分は、取り消し（その人の URL は使えなくなる）
        sqlx::query("UPDATE steps SET decision = 'cancelled' WHERE request_id = $1 AND round = $2 AND decision = 'waiting'")
            .bind(r.id)
            .bind(r.round)
            .execute(&mut *tx)
            .await?;

        let now = OffsetDateTime::now_utc().to_offset(JST);
        let line = format!(
            "【{:02}/{:02} {:02}:{:02} 回覧者{} {}】{comment}",
            u8::from(now.month()), now.day(), now.hour(), now.minute(), step.step_order, step.approver_name
        );
        let comments = if r.return_comments.is_empty() { line.clone() } else { format!("{}\n{line}", r.return_comments) };
        sqlx::query("UPDATE requests SET status = 'returned', return_comments = $2, updated_at = now() WHERE id = $1")
            .bind(r.id)
            .bind(&comments)
            .execute(&mut *tx)
            .await?;
        log(&mut tx, r.id, Some(step.approver_id), &format!("差し戻し（回覧者{}）", step.step_order), comment).await?;

        let (name, email): (String, String) = sqlx::query_as("SELECT name, email FROM employees WHERE id = $1")
            .bind(r.requester_id)
            .fetch_one(&mut *tx)
            .await?;
        outbox.add(
            &email,
            &format!("差し戻し: {}", r.title),
            format!(
                "{name} さん\n\n依頼「{}」が、差し戻されました。\n\n{line}\n\n\
                 アプリで内容を直して、出し直してください。\n",
                r.title
            ),
        );
        Ok((r.title, tx, outbox))
    }
    .await;
    finish(cx, result, &[]).await
}

/// 引き戻し（回覧中だけ）→ 引き戻し中。承認待ちの人の URL は使えなくなる
pub async fn pull_back(cx: &Cx, user: &Employee, request_id: i64) -> WfResult<()> {
    let st = state(cx);
    let result = async {
        let mut tx = st.pool.begin().await?;
        let r = lock_own(&mut tx, user, request_id).await?;
        if r.status != Status::Circulating {
            return rule(format!("「{}」の依頼は引き戻せません。", r.status.label()));
        }
        sqlx::query("UPDATE steps SET decision = 'cancelled' WHERE request_id = $1 AND round = $2 AND decision = 'waiting'")
            .bind(r.id)
            .bind(r.round)
            .execute(&mut *tx)
            .await?;
        set_status(&mut tx, r.id, Status::PulledBack).await?;
        log(&mut tx, r.id, Some(user.id), "引き戻した", "").await?;
        Ok(((), tx, Outbox::new()))
    }
    .await;
    finish(cx, result, &[]).await
}

/// 直して出し直す → 回覧中。最初の回覧者からやり直す（回覧の回数を 1 増やす）
pub async fn resubmit(cx: &Cx, user: &Employee, request_id: i64, input: RequestInput) -> WfResult<()> {
    let st = state(cx);
    let mut saved = Vec::new();
    let result = async {
        let mut tx = st.pool.begin().await?;
        let mut outbox = Outbox::new();
        let r = lock_own(&mut tx, user, request_id).await?;
        if !matches!(r.status, Status::Returned | Status::PulledBack) {
            return rule(format!("「{}」の依頼は出し直せません。", r.status.label()));
        }
        check_approvers(&mut tx, user.id, &input.approver_ids).await?;
        let round = r.round + 1;
        sqlx::query(
            "UPDATE requests SET title = $2, detail = $3, due_date = $4, drawing = $5, round = $6,
                    status = 'circulating', updated_at = now()
              WHERE id = $1",
        )
        .bind(r.id)
        .bind(&input.title)
        .bind(&input.detail)
        .bind(input.due_date)
        .bind(input.drawing.as_ref().map(Json))
        .bind(round)
        .execute(&mut *tx)
        .await?;
        start_round(&mut tx, &mut outbox, &st.config, r.id, round, &input.approver_ids).await?;
        add_files(&mut tx, &st.config, r.id, &input.files, &mut saved).await?;
        log(&mut tx, r.id, Some(user.id), &format!("出し直した（{round} 回目）"), "").await?;
        Ok(((), tx, outbox))
    }
    .await;
    finish(cx, result, &saved).await
}

/// 取り下げ（差し戻し中・引き戻し中だけ）
pub async fn withdraw(cx: &Cx, user: &Employee, request_id: i64) -> WfResult<()> {
    let st = state(cx);
    let result = async {
        let mut tx = st.pool.begin().await?;
        let r = lock_own(&mut tx, user, request_id).await?;
        if !matches!(r.status, Status::Returned | Status::PulledBack) {
            return rule(format!("「{}」の依頼は取り下げられません。", r.status.label()));
        }
        set_status(&mut tx, r.id, Status::Withdrawn).await?;
        log(&mut tx, r.id, Some(user.id), "取り下げた", "").await?;
        Ok(((), tx, Outbox::new()))
    }
    .await;
    finish(cx, result, &[]).await
}

/// 担当を決める（回覧完了 → 対応中）
pub async fn assign(cx: &Cx, worker: &Employee, request_id: i64) -> WfResult<()> {
    if !worker.role.is_staff() {
        return rule("担当者だけができます。");
    }
    let st = state(cx);
    let result = async {
        let mut tx = st.pool.begin().await?;
        let r = lock_request(&mut tx, request_id).await?;
        if r.status != Status::Approved {
            return rule(format!("「{}」の依頼は、担当を決められません。", r.status.label()));
        }
        sqlx::query("UPDATE requests SET assignee_id = $2, status = 'in_progress', updated_at = now() WHERE id = $1")
            .bind(r.id)
            .bind(worker.id)
            .execute(&mut *tx)
            .await?;
        log(&mut tx, r.id, Some(worker.id), "担当になった", "").await?;
        Ok(((), tx, Outbox::new()))
    }
    .await;
    finish(cx, result, &[]).await
}

/// 引き渡した（対応中 → 完了）。依頼者にメール
pub async fn complete(cx: &Cx, worker: &Employee, request_id: i64) -> WfResult<()> {
    let st = state(cx);
    let result = async {
        let mut tx = st.pool.begin().await?;
        let mut outbox = Outbox::new();
        let r = lock_request(&mut tx, request_id).await?;
        if r.status != Status::InProgress || r.assignee_id != Some(worker.id) {
            return rule("担当している、対応中の依頼だけを完了にできます。");
        }
        set_status(&mut tx, r.id, Status::Done).await?;
        log(&mut tx, r.id, Some(worker.id), "完了にした", "").await?;
        let (name, email): (String, String) = sqlx::query_as("SELECT name, email FROM employees WHERE id = $1")
            .bind(r.requester_id)
            .fetch_one(&mut *tx)
            .await?;
        outbox.add(
            &email,
            &format!("完了: {}", r.title),
            format!("{name} さん\n\n依頼「{}」の対応が終わりました（完了）。\n", r.title),
        );
        Ok(((), tx, outbox))
    }
    .await;
    finish(cx, result, &[]).await
}

/// 承認のお願いを送り直す（前の URL は使えなくなる）。管理者の画面と、毎朝の処理で使う。
/// `actor` は、送り直した人（毎朝の処理なら None）
pub async fn resend(cx: &Cx, actor: Option<&Employee>, step_id: i64) -> WfResult<()> {
    let st = state(cx);
    let result = async {
        let mut tx = st.pool.begin().await?;
        let mut outbox = Outbox::new();
        let row: Option<(i64, i16, String, String)> = sqlx::query_as(
            "SELECT s.request_id, s.step_order, s.decision, r.status
               FROM steps s JOIN requests r ON r.id = s.request_id
              WHERE s.id = $1 FOR UPDATE OF s, r",
        )
        .bind(step_id)
        .fetch_optional(&mut *tx)
        .await?;
        let Some((request_id, order, decision, status)) = row else {
            return rule("回覧がありません。");
        };
        if decision != Decision::Waiting.as_str() || status != Status::Circulating.as_str() {
            return rule("承認を待っている回覧だけを送り直せます。");
        }
        send_approval_mail(&mut tx, &mut outbox, &st.config, step_id).await?;
        log(&mut tx, request_id, actor.map(|a| a.id), &format!("承認のお願いを送り直した（回覧者{order}）"), "").await?;
        Ok(((), tx, outbox))
    }
    .await;
    finish(cx, result, &[]).await
}

async fn set_status(tx: &mut Tx, request_id: i64, status: Status) -> WfResult<()> {
    sqlx::query("UPDATE requests SET status = $2, updated_at = now() WHERE id = $1")
        .bind(request_id)
        .bind(status.as_str())
        .execute(&mut **tx)
        .await?;
    Ok(())
}

/// 社員の役割を変える（サーバーの上のコマンドで使う）
pub async fn grant_role(pool: &sqlx::PgPool, employee_no: &str, role: Role) -> WfResult<()> {
    let n = sqlx::query("UPDATE employees SET role = $2 WHERE employee_no = $1")
        .bind(employee_no)
        .bind(role.as_str())
        .execute(pool)
        .await?
        .rows_affected();
    if n == 0 {
        return rule(format!("従業員番号 {employee_no} の社員がいません。"));
    }
    Ok(())
}

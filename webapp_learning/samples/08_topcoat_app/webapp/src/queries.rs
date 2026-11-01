//! 画面に出すために読むだけの SQL（状態は変えない）
//!
//! 「だれが、どの依頼を見られるか」の決まりも、ここに置く:
//! 担当者・管理者は全部。一般の人は、自分が出した依頼と、自分が回覧者に入っている依頼だけ。

use sqlx::PgPool;

use crate::models::{AttachmentRow, Employee, HistoryRow, RequestDetail, RequestRow, StepRow};

/// 一覧。`mine_only` なら、自分が出した依頼だけ
pub async fn list_requests(pool: &PgPool, user: &Employee, mine_only: bool) -> sqlx::Result<Vec<RequestRow>> {
    sqlx::query_as(
        "SELECT r.id, r.title, r.due_date, r.status, q.name AS requester_name,
                (SELECT a.name FROM steps s JOIN employees a ON a.id = s.approver_id
                  WHERE s.request_id = r.id AND s.round = r.round AND s.decision = 'waiting'
                  ORDER BY s.step_order LIMIT 1) AS waiting_for,
                r.created_at
           FROM requests r
           JOIN employees q ON q.id = r.requester_id
          WHERE ($1 OR r.requester_id = $2
                 OR EXISTS (SELECT 1 FROM steps s WHERE s.request_id = r.id AND s.approver_id = $2))
            AND (NOT $3 OR r.requester_id = $2)
          ORDER BY r.created_at DESC
          LIMIT 200",
    )
    .bind(user.role.is_staff())
    .bind(user.id)
    .bind(mine_only)
    .fetch_all(pool)
    .await
}

/// 依頼 1 件と、その回覧・添付・記録
pub struct FullRequest {
    pub request: RequestDetail,
    pub steps: Vec<StepRow>,
    pub attachments: Vec<AttachmentRow>,
    pub histories: Vec<HistoryRow>,
}

impl FullRequest {
    /// この人が見てよいか
    pub fn visible_to(&self, user: &Employee) -> bool {
        user.role.is_staff() || self.request.requester_id == user.id || self.steps.iter().any(|s| s.approver_id == user.id)
    }

    /// 今の回の回覧だけ
    pub fn current_steps(&self) -> impl Iterator<Item = &StepRow> {
        self.steps.iter().filter(|s| s.round == self.request.round)
    }
}

pub async fn find_request(pool: &PgPool, id: i64) -> sqlx::Result<Option<FullRequest>> {
    let request: Option<RequestDetail> = sqlx::query_as(
        "SELECT r.id, r.title, r.detail, r.due_date, r.status,
                r.requester_id, q.name AS requester_name, q.dept AS requester_dept,
                r.assignee_id, w.name AS assignee_name,
                r.drawing, r.return_comments, r.round, r.created_at
           FROM requests r
           JOIN employees q ON q.id = r.requester_id
           LEFT JOIN employees w ON w.id = r.assignee_id
          WHERE r.id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    let Some(request) = request else { return Ok(None) };

    let steps = sqlx::query_as(
        "SELECT s.id, s.round, s.step_order, s.approver_id, a.name AS approver_name, a.dept AS approver_dept,
                s.decision, s.decided_at
           FROM steps s JOIN employees a ON a.id = s.approver_id
          WHERE s.request_id = $1
          ORDER BY s.round DESC, s.step_order",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;
    let attachments = attachments_of(pool, id).await?;
    let histories = sqlx::query_as(
        "SELECT h.at, e.name AS actor_name, h.action, h.note
           FROM histories h LEFT JOIN employees e ON e.id = h.actor_id
          WHERE h.request_id = $1
          ORDER BY h.at, h.id",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;
    Ok(Some(FullRequest { request, steps, attachments, histories }))
}

pub async fn attachments_of(pool: &PgPool, request_id: i64) -> sqlx::Result<Vec<AttachmentRow>> {
    sqlx::query_as(
        "SELECT id, request_id, stored_name, content_type, original_name, size, uploaded_at
           FROM attachments WHERE request_id = $1 ORDER BY id",
    )
    .bind(request_id)
    .fetch_all(pool)
    .await
}

pub async fn find_attachment(pool: &PgPool, id: i64) -> sqlx::Result<Option<AttachmentRow>> {
    sqlx::query_as(
        "SELECT id, request_id, stored_name, content_type, original_name, size, uploaded_at
           FROM attachments WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

/// 回覧者に選べる人（自分以外の、いる社員）
pub async fn approver_choices(pool: &PgPool, user: &Employee) -> sqlx::Result<Vec<Employee>> {
    sqlx::query_as(
        "SELECT id, employee_no, name, dept, job_title, email, role
           FROM employees WHERE is_active AND id <> $1
          ORDER BY dept, employee_no",
    )
    .bind(user.id)
    .fetch_all(pool)
    .await
}

/// 集計: 状態ごとの件数と、担当者ごとの完了の件数
pub struct Stats {
    pub by_status: Vec<(String, i64)>,
    pub done_by_assignee: Vec<(String, i64)>,
}

pub async fn stats(pool: &PgPool) -> sqlx::Result<Stats> {
    let by_status = sqlx::query_as("SELECT status, count(*) FROM requests GROUP BY status").fetch_all(pool).await?;
    let done_by_assignee = sqlx::query_as(
        "SELECT w.name, count(*) FROM requests r JOIN employees w ON w.id = r.assignee_id
          WHERE r.status = 'done' GROUP BY w.name ORDER BY count(*) DESC, w.name",
    )
    .fetch_all(pool)
    .await?;
    Ok(Stats { by_status, done_by_assignee })
}

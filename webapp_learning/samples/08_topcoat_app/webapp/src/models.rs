//! データの形（表の 1 行）と、状態の名前。07 の status クレートを、データベースに合わせて広げた物

use time::{Date, OffsetDateTime, UtcOffset, macros::{format_description, offset}};

/// 日本の時刻（日本には夏時間が無いので、いつも +9 時間でよい）
pub const JST: UtcOffset = offset!(+9);

/// 日時を「2026-11-02 09:00」の形にする（日本の時刻で）
pub fn fmt_datetime(t: OffsetDateTime) -> String {
    t.to_offset(JST)
        .format(format_description!("[year]-[month]-[day] [hour]:[minute]"))
        .unwrap_or_default()
}

/// 日付を「2026-11-02」の形にする
pub fn fmt_date(d: Date) -> String {
    d.format(format_description!("[year]-[month]-[day]")).unwrap_or_default()
}

/// ファイルの大きさを「12 KB」「800 バイト」の形にする
pub fn fmt_size(bytes: i32) -> String {
    if bytes < 1024 { format!("{bytes} バイト") } else { format!("{} KB", bytes / 1024) }
}

/// 今日の日付（日本の時刻で）
pub fn today_jst() -> Date {
    OffsetDateTime::now_utc().to_offset(JST).date()
}

/// 文字（データベースの値）と enum を行き来させる、決まった書き方をまとめるマクロ
macro_rules! text_enum {
    ($name:ident { $($variant:ident => ($text:literal, $label:literal)),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name { $($variant),+ }

        impl $name {
            /// データベースに入れる文字
            pub fn as_str(self) -> &'static str {
                match self { $($name::$variant => $text),+ }
            }
            /// 画面に出す名前
            pub fn label(self) -> &'static str {
                match self { $($name::$variant => $label),+ }
            }
        }

        // データベースから読んだ文字を enum にする（知らない文字なら、読み込みのエラー）
        impl TryFrom<String> for $name {
            type Error = String;
            fn try_from(s: String) -> Result<Self, String> {
                match s.as_str() {
                    $($text => Ok($name::$variant),)+
                    _ => Err(format!("{} に、知らない値があります: {s}", stringify!($name))),
                }
            }
        }
    };
}

text_enum!(Status {
    Circulating => ("circulating", "回覧中"),
    Returned => ("returned", "差し戻し中"),
    PulledBack => ("pulled_back", "引き戻し中"),
    Approved => ("approved", "回覧完了"),
    InProgress => ("in_progress", "対応中"),
    Done => ("done", "完了"),
    Withdrawn => ("withdrawn", "取り下げ"),
});

impl Status {
    /// 全部の状態（流れの順）
    pub const ALL: [Status; 7] = [
        Status::Circulating,
        Status::Returned,
        Status::PulledBack,
        Status::Approved,
        Status::InProgress,
        Status::Done,
        Status::Withdrawn,
    ];

    /// 01 の CSS の札の class（badge-circulating など）
    pub fn css_class(self) -> String {
        format!("badge badge-{}", self.as_str().replace('_', "-"))
    }
}

text_enum!(Decision {
    Waiting => ("waiting", "待ち"),
    Approved => ("approved", "承認"),
    Returned => ("returned", "差し戻し"),
    Cancelled => ("cancelled", "取り消し"),
});

text_enum!(Role {
    Member => ("member", "一般"),
    Worker => ("worker", "担当者"),
    Admin => ("admin", "管理者"),
});

impl Role {
    /// 担当を決めたり、全部の依頼を見たりできるか
    pub fn is_staff(self) -> bool {
        matches!(self, Role::Worker | Role::Admin)
    }
}

/// 社員 1 人
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Employee {
    pub id: i64,
    pub employee_no: String,
    pub name: String,
    pub dept: String,
    pub job_title: String,
    pub email: String,
    #[sqlx(try_from = "String")]
    pub role: Role,
}

/// 一覧の 1 行
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RequestRow {
    pub id: i64,
    pub title: String,
    pub due_date: Date,
    #[sqlx(try_from = "String")]
    pub status: Status,
    pub requester_name: String,
    /// 回覧中なら、今だれの承認を待っているか
    pub waiting_for: Option<String>,
    pub created_at: OffsetDateTime,
}

/// 詳しい画面に出す、依頼 1 件
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RequestDetail {
    pub id: i64,
    pub title: String,
    pub detail: String,
    pub due_date: Date,
    #[sqlx(try_from = "String")]
    pub status: Status,
    pub requester_id: i64,
    pub requester_name: String,
    pub requester_dept: String,
    pub assignee_id: Option<i64>,
    pub assignee_name: Option<String>,
    /// JSONB の列を、図面の struct として読む（Json で包む）
    pub drawing: Option<sqlx::types::Json<crate::drawing::Drawing>>,
    pub return_comments: String,
    pub round: i16,
    pub created_at: OffsetDateTime,
}

/// 回覧の 1 人分（詳しい画面の表）
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct StepRow {
    pub id: i64,
    pub round: i16,
    pub step_order: i16,
    pub approver_id: i64,
    pub approver_name: String,
    pub approver_dept: String,
    #[sqlx(try_from = "String")]
    pub decision: Decision,
    pub decided_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AttachmentRow {
    pub id: i64,
    pub request_id: i64,
    pub stored_name: String,
    pub content_type: String,
    pub original_name: String,
    pub size: i32,
    pub uploaded_at: OffsetDateTime,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct HistoryRow {
    pub at: OffsetDateTime,
    pub actor_name: Option<String>,
    pub action: String,
    pub note: String,
}

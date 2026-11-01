//! 依頼の状態と、状態の移り変わり（02 の status.ts・04 の status.py を Rust で書いた物）

use std::fmt;

/// 依頼の状態。この 7 つ以外は作れない
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Circulating,
    Returned,
    PulledBack,
    Approved,
    InProgress,
    Done,
    Withdrawn,
}

impl Status {
    /// 画面に出す名前
    pub fn label(self) -> &'static str {
        match self {
            Status::Circulating => "回覧中",
            Status::Returned => "差し戻し中",
            Status::PulledBack => "引き戻し中",
            Status::Approved => "回覧完了",
            Status::InProgress => "対応中",
            Status::Done => "完了",
            Status::Withdrawn => "取り下げ",
        }
    }
}

/// 状態を変える操作
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    ApproveAll,
    Return,
    PullBack,
    Resubmit,
    Withdraw,
    Assign,
    HandOver,
}

/// できない操作をしようとしたときのエラー
#[derive(Debug, PartialEq, Eq)]
pub struct TransitionError {
    pub status: Status,
    pub action: Action,
}

impl fmt::Display for TransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "「{}」の依頼に {:?} はできません", self.status.label(), self.action)
    }
}

impl std::error::Error for TransitionError {}

/// 今の状態と操作から、次の状態を返す。できない操作なら Err
pub fn next_status(current: Status, action: Action) -> Result<Status, TransitionError> {
    use Action::*;
    use Status::*;
    match (current, action) {
        (Circulating, ApproveAll) => Ok(Approved),
        (Circulating, Return) => Ok(Returned),
        (Circulating, PullBack) => Ok(PulledBack),
        (Returned | PulledBack, Resubmit) => Ok(Circulating),
        (Returned | PulledBack, Withdraw) => Ok(Withdrawn),
        (Approved, Assign) => Ok(InProgress),
        (InProgress, HandOver) => Ok(Done),
        _ => Err(TransitionError { status: current, action }),
    }
}

/// 依頼 1 件
#[derive(Debug)]
pub struct Request {
    pub id: u32,
    pub title: String,
    pub status: Status,
    pub comments: Vec<String>,
}

impl Request {
    pub fn new(id: u32, title: &str) -> Request {
        Request { id, title: title.to_string(), status: Status::Circulating, comments: Vec::new() }
    }

    /// 操作をして、状態を進める。できなければ、状態は変えずに Err を返す
    pub fn apply(&mut self, action: Action) -> Result<(), TransitionError> {
        self.status = next_status(self.status, action)?; // ? は「Err ならここで返す」
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 差し戻してから出し直すと回覧中に戻る() {
        let mut r = Request::new(1, "治具の製作");
        r.apply(Action::Return).unwrap();
        assert_eq!(r.status, Status::Returned);
        r.apply(Action::Resubmit).unwrap();
        assert_eq!(r.status, Status::Circulating);
    }

    #[test]
    fn 完了した依頼は差し戻せない() {
        let err = next_status(Status::Done, Action::Return).unwrap_err();
        assert_eq!(err.to_string(), "「完了」の依頼に Return はできません");
    }

    #[test]
    fn できない操作では状態が変わらない() {
        let mut r = Request::new(2, "部品の追加工");
        assert!(r.apply(Action::HandOver).is_err());
        assert_eq!(r.status, Status::Circulating);
    }
}

//! アプリの「中身」（データと決まり）。画面（app.rs）からは、ここの関数を使う

/// 依頼の状態（status クレートと同じ考え方。ここでは使う所だけ）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Circulating,
    Returned,
    Approved,
    Done,
}

impl Status {
    pub fn label(self) -> &'static str {
        match self {
            Status::Circulating => "回覧中",
            Status::Returned => "差し戻し中",
            Status::Approved => "回覧完了",
            Status::Done => "完了",
        }
    }

    /// 01 の CSS の札の class
    pub fn css_class(self) -> &'static str {
        match self {
            Status::Circulating => "badge-circulating",
            Status::Returned => "badge-returned",
            Status::Approved => "badge-approved",
            Status::Done => "badge-done",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub id: u32,
    pub title: String,
    pub due_date: String,
    pub status: Status,
}

/// 依頼の置き場（練習なのでメモリーに持つ。08 では PostgreSQL に置く）
pub struct Store {
    pub requests: Vec<Request>,
}

impl Store {
    pub fn sample() -> Store {
        let row = |id, title: &str, due: &str, status| Request { id, title: title.into(), due_date: due.into(), status };
        Store {
            requests: vec![
                row(3, "棚の取り付け", "2026-11-20", Status::Returned),
                row(2, "部品の追加工", "2026-10-31", Status::Approved),
                row(1, "治具の製作", "2026-10-25", Status::Done),
            ],
        }
    }

    pub fn add(&mut self, title: &str, due_date: &str) {
        let id = self.requests.iter().map(|r| r.id).max().unwrap_or(0) + 1;
        self.requests.insert(0, Request { id, title: title.to_string(), due_date: due_date.to_string(), status: Status::Circulating });
    }
}

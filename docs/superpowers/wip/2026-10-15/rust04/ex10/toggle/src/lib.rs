/// ノートが完了しているかどうか（RustNote の done 列に対応）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Progress {
    Open,
    Done,
}

impl Progress {
    /// 完了と未完了を入れ替えます。
    #[must_use]
    pub fn toggle(self) -> Progress {
        match self {
            Progress::Open => Progress::Done,
            Progress::Done => Progress::Open,
        }
    }

    /// DB の done 列（0 / 1 の真偽値）から作ります。
    pub fn from_done(done: bool) -> Progress {
        if done { Progress::Done } else { Progress::Open }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_flips() {
        assert_eq!(Progress::Open.toggle(), Progress::Done);
        assert_eq!(Progress::Done.toggle(), Progress::Open);
    }

    #[test]
    fn toggle_twice_returns_to_start() {
        for start in [Progress::Open, Progress::Done] {
            assert_eq!(start.toggle().toggle(), start);
        }
    }

    #[test]
    fn from_done_column() {
        assert_eq!(Progress::from_done(true), Progress::Done);
        assert_eq!(Progress::from_done(false), Progress::Open);
    }
}

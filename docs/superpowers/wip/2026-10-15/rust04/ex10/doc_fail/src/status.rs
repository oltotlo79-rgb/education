use thiserror::Error;

/// ノートの状態です。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteStatus {
    /// 下書き。編集できる。
    Draft,
    /// 公開中。編集するには下書きに戻す。
    Published,
    /// アーカイブ済み。
    Archived,
}

/// 状態を変える操作です。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Publish,
    Unpublish,
    Archive,
    Restore,
}

/// 許されていない状態の移り変わりを表すエラーです。
#[derive(Debug, Error, PartialEq, Eq)]
#[error("{from:?} の状態では {action:?} できません")]
pub struct TransitionError {
    pub from: NoteStatus,
    pub action: Action,
}

impl NoteStatus {
    /// 操作を当てはめた「次の状態」を返します。今の状態は変えません。
    ///
    /// # Errors
    ///
    /// 許されていない組み合わせなら [`TransitionError`] を返します。
    ///
    /// # Examples
    ///
    /// ```
    /// use note_domain::{Action, NoteStatus};
    ///
    /// let next = NoteStatus::Draft.apply(Action::Publish).unwrap();
    /// assert_eq!(next, NoteStatus::Published);
    /// assert!(NoteStatus::Archived.apply(Action::Publish).is_err());
    /// ```
    pub fn apply(self, action: Action) -> Result<NoteStatus, TransitionError> {
        use Action::*;
        use NoteStatus::*;
        match (self, action) {
            (Draft, Publish) => Ok(Published),
            (Published, Unpublish) => Ok(Draft),
            (Draft | Published, Archive) => Ok(Archived),
            (Archived, Restore) => Ok(Draft),
            (from, action) => Err(TransitionError { from, action }),
        }
    }

    /// 編集できる状態なら `true` です。
    #[must_use]
    pub fn is_editable(self) -> bool {
        matches!(self, NoteStatus::Draft)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publish_then_archive_then_restore() {
        let s = NoteStatus::Draft;
        let s = s.apply(Action::Publish).unwrap();
        assert_eq!(s, NoteStatus::Published);
        let s = s.apply(Action::Archive).unwrap();
        assert_eq!(s, NoteStatus::Archived);
        let s = s.apply(Action::Restore).unwrap();
        assert_eq!(s, NoteStatus::Draft);
    }

    // すべての組み合わせ（3 状態 × 4 操作 = 12 通り）を表で確かめる
    #[test]
    fn transition_table() {
        use Action::*;
        use NoteStatus::*;
        let table = [
            (Draft, Publish, Some(Published)),
            (Draft, Unpublish, None),
            (Draft, Archive, Some(Archived)),
            (Draft, Restore, None),
            (Published, Publish, None),
            (Published, Unpublish, Some(Draft)),
            (Published, Archive, Some(Archived)),
            (Published, Restore, None),
            (Archived, Publish, None),
            (Archived, Unpublish, None),
            (Archived, Archive, None),
            (Archived, Restore, Some(Draft)),
        ];
        for (from, action, expected) in table {
            let got = from.apply(action).ok();
            assert_eq!(got, expected, "{from:?} に {action:?} を当てたとき");
        }
    }

    #[test]
    fn only_draft_is_editable() {
        assert!(NoteStatus::Draft.is_editable());
        assert!(!NoteStatus::Published.is_editable());
        assert!(!NoteStatus::Archived.is_editable());
    }
}

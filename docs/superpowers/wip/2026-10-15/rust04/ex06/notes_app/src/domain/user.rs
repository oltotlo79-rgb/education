// super:: で親（domain）、crate:: で crate のいちばん上からたどれる
use super::note::Note;

pub fn count_owned(notes: &[Note], user_id: i64) -> usize {
    notes.iter().filter(|n| n.owner_id == user_id).count()
}

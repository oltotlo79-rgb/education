use crate::domain::note::{Note, NoteTitle};

// crate の外に公開する保存先（中身の Vec は見せない）
#[derive(Default)]
pub struct NoteStore {
    notes: Vec<Note>,
}

impl NoteStore {
    pub fn add(&mut self, owner_id: i64, title: NoteTitle) -> &Note {
        let id = self.notes.len() as i64 + 1;
        self.notes.push(Note {
            id,
            title,
            owner_id,
        });
        self.notes.last().unwrap()
    }

    pub fn all(&self) -> &[Note] {
        &self.notes
    }
}

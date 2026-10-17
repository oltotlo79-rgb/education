// ノートをメモリ上の HashMap に保持するモジュール（プログラムを終えると消える）
use std::collections::HashMap;

use crate::note::{Note, NoteId, NoteTitle};

#[derive(Default)]
pub struct NoteStore {
    // pub を付けないので、外からは add や toggle を通してしか変えられない
    notes: HashMap<NoteId, Note>,
    last_id: i64,
}

impl NoteStore {
    pub fn new() -> Self {
        Self::default()
    }

    // ノートを追加して、振った ID を返す
    pub fn add(&mut self, title: NoteTitle) -> NoteId {
        self.last_id += 1;
        let id = NoteId(self.last_id);
        let note = Note {
            id,
            title,
            done: false,
        };
        self.notes.insert(id, note);
        id
    }

    // 完了を切り替え、切り替えた後の状態を返す。ID がなければ None
    pub fn toggle(&mut self, id: NoteId) -> Option<bool> {
        let note = self.notes.get_mut(&id)?;
        note.done = !note.done;
        Some(note.done)
    }

    // 削除したノートを返す。ID がなければ None
    pub fn remove(&mut self, id: NoteId) -> Option<Note> {
        self.notes.remove(&id)
    }

    // ID の小さい順に並べた一覧
    pub fn list(&self) -> Vec<&Note> {
        let mut notes: Vec<&Note> = self.notes.values().collect();
        notes.sort_by_key(|note| note.id);
        notes
    }

    // タイトルに query を含むノートだけの一覧
    pub fn search(&self, query: &str) -> Vec<&Note> {
        self.list()
            .into_iter()
            .filter(|note| note.title.as_str().contains(query))
            .collect()
    }
}

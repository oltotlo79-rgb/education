//! リポジトリ: domain だけを知っている

use crate::domain::NoteTitle;

/// ノートの保存先に求める約束
pub trait NoteRepository {
    /// 保存して、付けた番号を返す
    fn insert(&mut self, title: NoteTitle) -> usize;
    /// 保存したタイトルの一覧
    fn titles(&self) -> Vec<String>;
}

/// メモリの中に保存する実装
#[derive(Default)]
pub struct MemoryRepository {
    titles: Vec<NoteTitle>,
}

impl NoteRepository for MemoryRepository {
    fn insert(&mut self, title: NoteTitle) -> usize {
        self.titles.push(title);
        self.titles.len()
    }

    fn titles(&self) -> Vec<String> {
        self.titles.iter().map(|t| t.as_str().to_string()).collect()
    }
}

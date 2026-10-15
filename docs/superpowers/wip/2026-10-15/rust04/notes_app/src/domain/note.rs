// 1〜80 文字のタイトル（UNIT 01 の簡略版）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteTitle(String);

impl NoteTitle {
    pub fn parse(input: &str) -> Option<NoteTitle> {
        let t = input.trim();
        let len = t.chars().count();
        if (1..=80).contains(&len) {
            Some(NoteTitle(t.to_string()))
        } else {
            None
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Note {
    pub id: i64,
    pub title: NoteTitle,
    // crate の中（repo など）からは書き換えられるが、crate の外からは見えない
    pub(crate) owner_id: i64,
}

impl Note {
    pub fn owner_id(&self) -> i64 {
        self.owner_id
    }
}

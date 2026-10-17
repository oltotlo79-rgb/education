//! ドメイン: crate の中のほかの部品を use しない（いちばん内側）

/// 検査済みのタイトル（前後の空白を除いて 1〜80 文字）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteTitle(String);

impl NoteTitle {
    pub fn parse(input: &str) -> Result<NoteTitle, String> {
        let trimmed = input.trim();
        let len = trimmed.chars().count();
        if len == 0 || len > 80 {
            return Err(format!("タイトルは1〜80文字です（現在{len}文字）"));
        }
        Ok(NoteTitle(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

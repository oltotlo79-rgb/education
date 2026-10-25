/// 試し: dev-dependencies のクレートを src/ で使う
pub fn scratch_dir() -> tempfile::TempDir {
    tempfile::tempdir().expect("一時フォルダを作れません")
}

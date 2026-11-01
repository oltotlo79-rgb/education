//! 添付ファイル（JPG・PNG・PDF。1 ファイル 10MB まで・1 回に 5 ファイルまで）
//!
//! 外から来たファイルは信用しない。名前の拡張子はいくらでもごまかせるので、
//! **中身の先頭の数バイト（マジックナンバー）** でも種類を確かめる（06 の validators.py と同じ決まり）。
//! フォルダに置くときの名前は、アプリが乱数で決める（送られてきた名前は、表示とダウンロードにだけ使う）。

use std::path::{Path, PathBuf};

pub const MAX_FILE_BYTES: usize = 10 * 1024 * 1024;
pub const MAX_FILES: usize = 5;

/// 送られてきたファイル 1 つ（まだ確かめていない）
#[derive(Debug, Clone)]
pub struct UploadedFile {
    pub name: String,
    pub bytes: Vec<u8>,
}

/// フォルダに置いたファイル 1 つ
#[derive(Debug, Clone)]
pub struct SavedFile {
    pub stored_name: String,
    pub original_name: String,
    pub content_type: &'static str,
    pub size: usize,
}

/// 拡張子と、中身の先頭の印と、種類の名前（Content-Type）
const KINDS: &[(&str, &[u8], &str)] = &[
    ("jpg", b"\xff\xd8\xff", "image/jpeg"),
    ("jpeg", b"\xff\xd8\xff", "image/jpeg"),
    ("png", b"\x89PNG\r\n\x1a\n", "image/png"),
    ("pdf", b"%PDF-", "application/pdf"),
];

/// ブラウザによっては「C:\Users\...\図面.pdf」のように場所ごと送ってくるので、最後の名前だけにする
pub fn clean_name(name: &str) -> String {
    let base = name.rsplit(['/', '\\']).next().unwrap_or("").trim();
    base.chars().filter(|c| !c.is_control()).take(200).collect()
}

/// 1 ファイルを確かめる。よければ、拡張子と種類の名前を返す。だめなら、画面に出す文を返す
pub fn check(file: &UploadedFile) -> Result<(&'static str, &'static str), String> {
    let ext = file.name.rsplit_once('.').map(|(_, e)| e.to_ascii_lowercase()).unwrap_or_default();
    let Some(&(ext, magic, content_type)) = KINDS.iter().find(|(e, _, _)| *e == ext) else {
        return Err(format!("「{}」: 添付できるのは JPG・PNG・PDF だけです。", file.name));
    };
    if file.bytes.len() > MAX_FILE_BYTES {
        return Err(format!("「{}」: 添付ファイルは 1 つ 10MB までです。", file.name));
    }
    if !file.bytes.starts_with(magic) {
        return Err(format!("「{}」: ファイルの中身が、拡張子と合いません。", file.name));
    }
    Ok((ext, content_type))
}

/// 全部を確かめる（数と、1 つずつ）。だめな物の文を全部返す
pub fn check_all(files: &[UploadedFile]) -> Vec<String> {
    let mut errors = Vec::new();
    if files.len() > MAX_FILES {
        errors.push(format!("添付ファイルは、1 回に {MAX_FILES} つまでです。"));
    }
    errors.extend(files.iter().filter_map(|f| check(f).err()));
    errors
}

/// 添付ファイルを置くフォルダ
pub fn attachment_dir(file_dir: &Path) -> PathBuf {
    file_dir.join("attachments")
}

/// フォルダに書く。名前は乱数（32 文字の 16 進）＋拡張子
pub async fn save(file_dir: &Path, file: &UploadedFile) -> std::io::Result<SavedFile> {
    let (ext, content_type) = check(file).map_err(std::io::Error::other)?; // 念のため、もう一度確かめる
    let dir = attachment_dir(file_dir);
    tokio::fs::create_dir_all(&dir).await?;
    let mut random = [0u8; 16];
    getrandom::fill(&mut random).map_err(|e| std::io::Error::other(e.to_string()))?;
    let stored_name = format!("{}.{ext}", hex::encode(random));
    tokio::fs::write(dir.join(&stored_name), &file.bytes).await?;
    Ok(SavedFile { stored_name, original_name: file.name.clone(), content_type, size: file.bytes.len() })
}

/// 保存に失敗したときの後片付け（書いたファイルを消す）
pub async fn remove(file_dir: &Path, saved: &[SavedFile]) {
    for f in saved {
        let _ = tokio::fs::remove_file(attachment_dir(file_dir).join(&f.stored_name)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(name: &str, bytes: &[u8]) -> UploadedFile {
        UploadedFile { name: name.into(), bytes: bytes.to_vec() }
    }

    #[test]
    fn 中身がpdfならpdfとして通る() {
        assert_eq!(check(&file("図面.PDF", b"%PDF-1.7 ...")), Ok(("pdf", "application/pdf")));
    }

    #[test]
    fn 拡張子をpngに変えたexeは断る() {
        let err = check(&file("photo.png", b"MZ\x90\x00")).unwrap_err();
        assert!(err.contains("中身が、拡張子と合いません"));
    }

    #[test]
    fn 場所ごとの名前は最後だけにする() {
        assert_eq!(clean_name(r"C:\Users\sato\Desktop\図面.pdf"), "図面.pdf");
    }
}

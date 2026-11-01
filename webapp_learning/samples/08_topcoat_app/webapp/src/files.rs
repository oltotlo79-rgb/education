//! 添付ファイルを、ブラウザへ返す（ダウンロード）

use topcoat::router::{Body, response::Response};

use crate::{models::AttachmentRow, upload};

/// フォルダからファイルを読み、ダウンロードの返事を作る
pub async fn download(file_dir: &std::path::Path, a: &AttachmentRow) -> topcoat::Result<Response> {
    let bytes = tokio::fs::read(upload::attachment_dir(file_dir).join(&a.stored_name)).await?;
    Ok(Response::builder()
        .header("Content-Type", a.content_type.as_str())
        // 開かずに保存させる。日本語の名前は filename* に、%XX の形で書く（RFC 6266）
        .header("Content-Disposition", content_disposition(&a.original_name))
        // ブラウザに、中身から種類を推測させない（画像のふりをした HTML を開かせない）
        .header("X-Content-Type-Options", "nosniff")
        .body(Body::from(bytes))?)
}

fn content_disposition(name: &str) -> String {
    // 古いブラウザ向けの filename には、ASCII の文字だけを残す
    let ascii: String = name.chars().map(|c| if c.is_ascii_alphanumeric() || ".-_".contains(c) { c } else { '_' }).collect();
    format!("attachment; filename=\"{ascii}\"; filename*=UTF-8''{}", percent_encode(name))
}

/// URL などに書けない文字を、%XX の形にする
fn percent_encode(s: &str) -> String {
    s.bytes()
        .map(|b| if b.is_ascii_alphanumeric() || b"-._~".contains(&b) { (b as char).to_string() } else { format!("%{b:02X}") })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 日本語の名前は_xxの形にする() {
        assert_eq!(
            content_disposition("図面 A.pdf"),
            "attachment; filename=\"___A.pdf\"; filename*=UTF-8''%E5%9B%B3%E9%9D%A2%20A.pdf"
        );
    }
}

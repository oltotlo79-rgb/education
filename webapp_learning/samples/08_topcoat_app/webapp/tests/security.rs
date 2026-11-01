//! 安全のためのテスト（10 の巻）。データベースは使わない

use topcoat::{context::Cx, mail::mail};
use webapp::{drawing, upload};

/// 件名に改行を入れて、ヘッダーを増やそうとする（ヘッダー インジェクション）。
/// メールの部品が、改行を「ヘッダーの区切り」として扱わないことを確かめる
#[tokio::test]
async fn 件名に改行を入れても_ヘッダーは増えない() {
    // 依頼の件名は社員が入れる。悪い人が、件名に「改行＋Bcc:」を入れたとする
    let title = "治具の製作\r\nBcc: evil@example.com";
    let m = mail! {
        from: "noreply@example.com",
        to: "sato@example.com",
        subject: format!("【依頼・承認】承認のお願い: {title}"),
        text: "本文",
    }
    .expect("メールを作れる");
    assert!(m.bcc().is_empty());
    let raw = String::from_utf8(m.formatted(&Cx::default()).expect("形にできる")).unwrap();
    // 改行のあとの「Bcc:」が、本物のヘッダーの行になっていない
    assert!(!raw.contains("\r\nBcc:"), "Bcc のヘッダーが足されてしまった:\n{raw}");
}

/// 図面の文字に、HTML や script を入れても、図面の形として読めるだけ（画面では文字として出る）
#[test]
fn 図面の文字にscriptを入れても_ただの文字() {
    let d = drawing::parse(
        r#"{"version":1,"widthMm":297,"heightMm":210,"shapes":[{"kind":"text","x":1,"y":1,"text":"<script>alert(1)</script>"}]}"#,
    )
    .unwrap()
    .unwrap();
    assert!(matches!(&d.shapes[0], drawing::Shape::Text { text, .. } if text == "<script>alert(1)</script>"));
}

/// 送られてきた名前に「..\」などが入っていても、フォルダに置く名前には使わない
#[tokio::test]
async fn ファイル名で別のフォルダに書かせない() {
    let mut random = [0u8; 8];
    getrandom::fill(&mut random).unwrap();
    let dir = std::env::temp_dir().join(format!("webapp-sec-{}", hex::encode(random)));
    let file = upload::UploadedFile { name: r"..\..\Windows\evil.png".into(), bytes: b"\x89PNG\r\n\x1a\n....".to_vec() };
    let saved = upload::save(&dir, &file).await.unwrap();
    // 置いた名前は、アプリが決めた乱数の名前（32 文字の 16 進＋.png）
    assert_eq!(saved.stored_name.len(), 32 + ".png".len());
    assert!(saved.stored_name.chars().all(|c| c.is_ascii_hexdigit() || c == '.' || c.is_ascii_lowercase()));
    assert!(dir.join("attachments").join(&saved.stored_name).exists());
    let _ = std::fs::remove_dir_all(&dir);
}

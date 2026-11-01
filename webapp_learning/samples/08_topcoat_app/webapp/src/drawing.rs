//! 図面（線・四角・文字）の JSON。06 の React 版と同じ形
//!
//! ```json
//! {"version":1,"widthMm":297,"heightMm":210,"shapes":[{"kind":"rect","x":10,"y":10,"width":50,"height":30}]}
//! ```
//!
//! Python では、届いた JSON の形を 1 つずつ if で確かめた（06 の validators.py）。
//! Rust では、**形を struct と enum で書いておき、serde に読ませる**。形が違えば、読み込みがエラーになる。

use serde::{Deserialize, Serialize};

pub const MAX_DRAWING_BYTES: usize = 5 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)] // widthMm のような名前に合わせる。知らない項目はエラー
pub struct Drawing {
    pub version: u32,
    pub width_mm: f64,
    pub height_mm: f64,
    pub shapes: Vec<Shape>,
}

/// 図形。"kind" の値で、どの形かを見分ける
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Shape {
    Rect { x: f64, y: f64, width: f64, height: f64 },
    Line { x1: f64, y1: f64, x2: f64, y2: f64 },
    Text { x: f64, y: f64, text: String },
}

/// フォームの隠し項目の文字を、図面にする。空なら「図面なし」（None）
pub fn parse(text: &str) -> Result<Option<Drawing>, String> {
    let text = text.trim();
    if text.is_empty() {
        return Ok(None);
    }
    if text.len() > MAX_DRAWING_BYTES {
        return Err("図面は 5MB までです。".into());
    }
    let drawing: Drawing = serde_json::from_str(text).map_err(|_| "図面の形が違います。".to_string())?;
    if drawing.version != 1 {
        return Err("図面の版が違います。".into());
    }
    if !(drawing.width_mm > 0.0 && drawing.height_mm > 0.0) {
        return Err("図面の大きさが違います。".into());
    }
    // 図形が 1 つも無いなら、図面なしと同じに扱う
    Ok(if drawing.shapes.is_empty() { None } else { Some(drawing) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 四角と線を読める() {
        let d = parse(r#"{"version":1,"widthMm":297,"heightMm":210,"shapes":[
            {"kind":"rect","x":10,"y":10,"width":50,"height":30},
            {"kind":"line","x1":0,"y1":0,"x2":100,"y2":50}]}"#)
        .unwrap()
        .unwrap();
        assert_eq!(d.shapes.len(), 2);
        assert!(matches!(d.shapes[0], Shape::Rect { width: 50.0, .. }));
    }

    #[test]
    fn 知らない図形はエラー() {
        let err = parse(r#"{"version":1,"widthMm":297,"heightMm":210,"shapes":[{"kind":"circle","r":5}]}"#);
        assert_eq!(err, Err("図面の形が違います。".to_string()));
    }

    #[test]
    fn 空なら図面なし() {
        assert_eq!(parse(""), Ok(None));
    }
}

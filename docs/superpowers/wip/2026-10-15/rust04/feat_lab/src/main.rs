// json 機能が ON のときだけ、Serialize を derive する
#[cfg_attr(feature = "json", derive(serde::Serialize))]
struct Note {
    title: String,
    body: String,
}

// いつでも使える書き出し
fn export_text(note: &Note) -> String {
    format!("{} / {}", note.title, note.body)
}

// markdown 機能が ON のときだけコンパイルされる
#[cfg(feature = "markdown")]
fn export_markdown(note: &Note) -> String {
    format!("# {}\n{}", note.title, note.body)
}

// json 機能が ON のときだけコンパイルされる
#[cfg(feature = "json")]
fn export_json(note: &Note) -> String {
    serde_json::to_string(note).expect("Note は必ず JSON にできる")
}

fn main() {
    let note = Note {
        title: "牛乳を買う".to_string(),
        body: "低脂肪のもの".to_string(),
    };
    println!("[text] {}", export_text(&note));

    #[cfg(feature = "markdown")]
    println!("[markdown] {}", export_markdown(&note));

    #[cfg(feature = "json")]
    println!("[json] {}", export_json(&note));

    // cfg! は、その機能が ON かどうかを true / false で返す
    println!(
        "markdown={} json={}",
        cfg!(feature = "markdown"),
        cfg!(feature = "json")
    );
}

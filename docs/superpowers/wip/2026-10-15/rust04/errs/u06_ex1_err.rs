mod app {
    pub mod domain {
        pub struct Note {
            // どこからでも読み書きできる
            pub title: String,
            // この crate の中からだけ読み書きできる
            pub(crate) owner_id: i64,
            // この module（domain）の中からだけ
            version: i64,
        }

        impl Note {
            pub fn new(title: &str, owner_id: i64) -> Note {
                Note {
                    title: title.to_string(),
                    owner_id,
                    version: 1,
                }
            }

            // version を外から「読むだけ」の窓口
            pub fn version(&self) -> i64 {
                self.version
            }

            pub fn bump(&mut self) {
                self.version = next_version(self.version);
            }
        }

        // pub なし: domain の中だけで使う補助関数
        fn next_version(v: i64) -> i64 {
            v + 1
        }
    }

    pub mod service {
        // super は「1つ上の module」（ここでは app）
        use super::domain::Note;

        pub fn rename(note: &mut Note, title: &str) {
            note.title = title.to_string();
            note.bump();
        }

        // 親の module（app）の中からだけ呼べる
        pub(super) fn audit(note: &Note) -> String {
            format!("owner={} version={}", note.owner_id, note.version())
        }
    }

    pub fn run() -> String {
        let mut note = domain::Note::new("牛乳", 7);
        service::rename(&mut note, "牛乳を買う");
        // app の中なので pub(super) の audit を呼べる
        service::audit(&note)
    }
}

// crate:: は「この crate のいちばん上」からの道のり
use crate::app::domain::Note;

fn main() {
    let mut note = Note::new("本を返す", 1);
    app::service::rename(&mut note, "図書館に本を返す");
    println!("{} (version {})", note.title, note.version());
    println!("owner_id は同じ crate なので読める: {}", note.owner_id);
    println!("{}", app::run());
    // main（crate のいちばん上）から pub(super) の関数を呼ぶ
    println!("{}", app::service::audit(&note));
}

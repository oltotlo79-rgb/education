#[derive(Debug)]
struct Note {
    id: u64,
    title: String,
    done: bool,
}

// 引数と同じ名前のフィールドは、名前だけ書けばよい（省略記法）
fn make_note(id: u64, title: &str) -> Note {
    Note {
        id,
        title: title.to_string(),
        done: false,
    }
}

fn main() {
    // フィールドを書き換えるので mut を付ける（mut はインスタンス全体に付く）
    let mut note = make_note(1, "牛乳を買う");
    note.done = true;
    println!("{note:?}");

    // ..note は「残りのフィールドは note と同じ値にする」（構造体更新記法）
    let another = Note {
        id: 2,
        title: String::from("卵を買う"),
        ..note
    };
    println!(
        "#{} {}（完了: {}）",
        another.id, another.title, another.done
    );
}

struct Note {
    title: String,
    done: bool,
}

// 読むだけなので &Note を受け取る
fn describe(note: &Note) -> String {
    let mark = if note.done { "[x]" } else { "[ ]" };
    format!("{mark} {}", note.title)
}

// 書き換えるので &mut Note を受け取る
fn toggle(note: &mut Note) {
    note.done = !note.done;
}

// 書き換える。新しいタイトルは読むだけなので &str で受け取る
fn rename(note: &mut Note, new_title: &str) {
    note.title = new_title.to_string();
}

fn main() {
    let mut note = Note {
        title: String::from("牛乳を買う"),
        done: false,
    };
    println!("{}", describe(&note));

    toggle(&mut note);
    rename(&mut note, "牛乳と卵を買う");
    println!("{}", describe(&note));

    // 可変参照を変数に入れておき、続けて使うこともできる
    let editor = &mut note;
    toggle(editor);
    editor.title.push_str("（急ぎ）");
    println!("{}", describe(&note));
}

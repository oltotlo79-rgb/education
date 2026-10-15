struct Note {
    title: String,
    done: bool,
}

fn main() {
    let mut notes = vec![Note {
        title: String::from("買い物"),
        done: false,
    }];
    let total = notes.len();
    let first = &mut notes[0];
    first.done = true;
    println!("{} / {total} 件 / {}", first.title, first.done);
}

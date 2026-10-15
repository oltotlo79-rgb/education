use std::collections::HashMap;

struct Note {
    id: u64,
    title: String,
    done: bool,
}

// 方法1: 位置（ただの数値）を先に求め、借用をいったん終わらせてから書き換える
fn toggle_by_id(notes: &mut [Note], id: u64) -> bool {
    let Some(index) = notes.iter().position(|n| n.id == id) else {
        return false;
    };
    notes[index].done = !notes[index].done;
    true
}

// 方法2: iter_mut で「書き換え用の参照」を1つだけ取り出す
fn rename_by_id(notes: &mut [Note], id: u64, new_title: &str) -> bool {
    match notes.iter_mut().find(|n| n.id == id) {
        Some(note) => {
            note.title = new_title.to_string();
            true
        }
        None => false,
    }
}

// 方法3: HashMap は entry で「探す」と「書く」を1回で済ませる
fn count_words(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

struct Editor {
    draft: String,
    history: Vec<String>,
}

fn main() {
    let mut notes = vec![
        Note {
            id: 1,
            title: String::from("買い物"),
            done: false,
        },
        Note {
            id: 2,
            title: String::from("掃除"),
            done: false,
        },
    ];
    println!("toggle 2: {}", toggle_by_id(&mut notes, 2));
    println!("rename 1: {}", rename_by_id(&mut notes, 1, "買い出し"));
    println!("toggle 9: {}", toggle_by_id(&mut notes, 9));
    for n in &notes {
        println!("  #{} {} done={}", n.id, n.title, n.done);
    }

    let counts = count_words("rust is fun rust is safe");
    println!("rust = {}, is = {}, fun = {}", counts["rust"], counts["is"], counts["fun"]);

    // 方法4: struct の別々のフィールドなら、同時に可変借用できる
    let mut editor = Editor {
        draft: String::from("下書き1"),
        history: Vec::new(),
    };
    let draft = &mut editor.draft;
    let history = &mut editor.history;
    history.push(draft.clone());
    draft.clear();
    draft.push_str("下書き2");
    println!("draft = {}, history = {:?}", editor.draft, editor.history);
}

// 操作の種類ごとに、持つデータの形が違う
enum Command {
    List,
    Add(String),
    Toggle(u64),
    Rename { id: u64, title: String },
    Quit,
}

fn describe(cmd: &Command) -> String {
    match cmd {
        Command::List => String::from("一覧を表示"),
        // ( ) の中の値に名前を付けて取り出す
        Command::Add(title) => format!("「{title}」を追加"),
        Command::Toggle(id) => format!("#{id} の完了を切り替え"),
        // { } の形のものは、フィールド名で取り出す
        Command::Rename { id, title } => format!("#{id} を「{title}」に改名"),
        Command::Quit => String::from("終了"),
    }
}

fn main() {
    let commands = vec![
        Command::Add(String::from("牛乳を買う")),
        Command::Toggle(1),
        Command::Rename {
            id: 1,
            title: String::from("低脂肪乳を買う"),
        },
        Command::List,
        Command::Quit,
    ];
    for cmd in &commands {
        println!("{}", describe(cmd));
    }
}

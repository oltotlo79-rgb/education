enum Command {
    List,
    Add(String),
    Done(u64),
    Quit,
}

// 1行の入力を Command に変換する。解釈できなければ None
fn parse_command(line: &str) -> Option<Command> {
    let line = line.trim();
    // 最初の空白で「命令」と「残り」に分ける。空白がなければ残りは ""
    let (word, rest) = line.split_once(' ').unwrap_or((line, ""));
    let rest = rest.trim();
    match word {
        "list" => Some(Command::List),
        "quit" => Some(Command::Quit),
        // タイトルが空の add は受け付けない
        "add" if !rest.is_empty() => Some(Command::Add(rest.to_string())),
        "done" => match rest.parse::<u64>() {
            Ok(id) => Some(Command::Done(id)),
            Err(_) => None,
        },
        _ => None,
    }
}

fn main() {
    let inputs = [
        "list",
        "add 牛乳を買う",
        "add",
        "done 1",
        "done abc",
        "quit",
        "hello",
    ];
    for line in inputs {
        // Some の中の Command の中まで、一度にパターンで分解する
        let text = match parse_command(line) {
            Some(Command::List) => String::from("一覧を表示"),
            Some(Command::Add(title)) => format!("「{title}」を追加"),
            Some(Command::Done(id)) => format!("#{id} を完了にする"),
            Some(Command::Quit) => String::from("終了"),
            None => String::from("解釈できません"),
        };
        println!("{line:?} → {text}");
    }
}

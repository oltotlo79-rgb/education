// src/note.rs と src/store.rs を、このクレートのモジュールとして読み込む
mod note;
mod store;

use std::io::{self, Write};

use note::{Note, NoteId, NoteTitle};
use store::NoteStore;

// 利用者が入力できる命令
enum Command {
    List,
    Add(String),
    Done(i64),
    Delete(i64),
    Search(String),
    Help,
    Quit,
}

// 1行の入力を命令に変換する。解釈できなければ None
fn parse_command(line: &str) -> Option<Command> {
    let line = line.trim();
    let (word, rest) = line.split_once(' ').unwrap_or((line, ""));
    let rest = rest.trim();
    match word {
        "list" => Some(Command::List),
        "add" => Some(Command::Add(rest.to_string())),
        "done" => rest.parse().ok().map(Command::Done),
        "del" => rest.parse().ok().map(Command::Delete),
        "find" if !rest.is_empty() => Some(Command::Search(rest.to_string())),
        "help" => Some(Command::Help),
        "quit" => Some(Command::Quit),
        _ => None,
    }
}

fn print_notes(notes: &[&Note]) {
    if notes.is_empty() {
        println!("（ノートはありません）");
        return;
    }
    for note in notes {
        let mark = if note.done { "[x]" } else { "[ ]" };
        println!("{mark} #{} {}", note.id.0, note.title.as_str());
    }
}

fn print_help() {
    println!("list          一覧を表示");
    println!("add タイトル  ノートを追加（1〜80文字）");
    println!("done 番号     完了／未完了を切り替え");
    println!("del 番号      ノートを削除");
    println!("find 語       タイトルで検索");
    println!("quit          終了");
}

fn prompt() -> io::Result<()> {
    print!("> ");
    // print! は改行しないので、すぐ画面に出すために flush する
    io::stdout().flush()
}

fn main() -> io::Result<()> {
    let mut store = NoteStore::new();
    println!("RustNote ミニ版です。help で使い方を表示します。");
    prompt()?;

    // 1行ずつ読む。入力が終わる（Ctrl+Z / Ctrl+D）か quit で抜ける
    for line in io::stdin().lines() {
        let line = line?;
        let Some(command) = parse_command(&line) else {
            println!("解釈できません（help で使い方を表示）");
            prompt()?;
            continue;
        };
        match command {
            Command::List => print_notes(&store.list()),
            Command::Add(raw) => match NoteTitle::parse(&raw) {
                Ok(title) => {
                    let id = store.add(title);
                    println!("#{} を追加しました", id.0);
                }
                Err(e) => println!("入力エラー: {e}"),
            },
            Command::Done(id) => match store.toggle(NoteId(id)) {
                Some(true) => println!("#{id} を完了にしました"),
                Some(false) => println!("#{id} を未完了に戻しました"),
                None => println!("#{id} は見つかりません"),
            },
            Command::Delete(id) => match store.remove(NoteId(id)) {
                Some(note) => println!("「{}」を削除しました", note.title.as_str()),
                None => println!("#{id} は見つかりません"),
            },
            Command::Search(query) => print_notes(&store.search(&query)),
            Command::Help => print_help(),
            Command::Quit => break,
        }
        prompt()?;
    }
    println!("終了します");
    Ok(())
}

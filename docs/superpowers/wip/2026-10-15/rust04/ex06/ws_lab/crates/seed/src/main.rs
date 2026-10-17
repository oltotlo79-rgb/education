use domain::NoteTitle;

fn main() {
    let titles = ["牛乳を買う", "本を返す"];
    let ok = titles
        .iter()
        .filter(|t| NoteTitle::parse(t).is_ok())
        .count();
    println!("{ok} 件のタイトルが正しい形です");
}

// #[derive(Debug)] を付けると、{:?} で中身を表示できるようになる
#[derive(Debug)]
struct Note {
    id: u64,
    title: String,
    body: String,
    done: bool,
}

fn main() {
    // 「フィールド名: 値」を並べて、Note の値（インスタンス）を作る
    let note = Note {
        id: 1,
        title: String::from("牛乳を買う"),
        body: String::from("低脂肪のもの"),
        done: false,
    };
    // 値.フィールド名 で取り出す
    println!("#{} {}（完了: {}）", note.id, note.title, note.done);
    println!("本文: {}", note.body);
    // {:?} は1行で、{:#?} は複数行で見やすく表示する
    println!("{note:?}");
    println!("{note:#?}");
}

// 実行ファイル: 部品を組み立てて動かすだけ。
// ライブラリの部品は crate:: ではなく、パッケージ名（note_layers::）で呼ぶ
use note_layers::repository::{MemoryRepository, NoteRepository};
use note_layers::routes::create_note;

fn main() {
    let mut repo = MemoryRepository::default();
    println!("{}", create_note(&mut repo, "  牛乳を買う  "));
    println!("{}", create_note(&mut repo, "   "));
    println!("保存されたタイトル: {:?}", repo.titles());
}

// ファイルの場所（src/repository/notes.rs）のとおりに道のりを書いた
use rustnote::repository::notes::LibsqlNoteRepository;

fn main() {
    println!("{}", std::any::type_name::<LibsqlNoteRepository>());
}

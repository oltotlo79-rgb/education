// repository.rs の pub use が作った入口から読み込む
use rustnote::repository::LibsqlNoteRepository;

fn main() {
    println!("{}", std::any::type_name::<LibsqlNoteRepository>());
}

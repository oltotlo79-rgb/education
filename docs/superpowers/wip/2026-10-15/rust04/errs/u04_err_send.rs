use std::sync::Arc;
use std::thread;

trait NoteRepository {
    fn count(&self) -> usize;
}

struct InMemory;

impl NoteRepository for InMemory {
    fn count(&self) -> usize {
        0
    }
}

fn main() {
    // + Send + Sync を付け忘れた
    let repo: Arc<dyn NoteRepository> = Arc::new(InMemory);
    let r = Arc::clone(&repo);
    let handle = thread::spawn(move || r.count());
    println!("{}", handle.join().unwrap());
}

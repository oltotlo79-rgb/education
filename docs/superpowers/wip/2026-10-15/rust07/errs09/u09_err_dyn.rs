use std::sync::Arc;

struct Note {
    title: String,
}

// async fn をそのまま trait に書いた（#[async_trait] なし）
trait NoteRepository {
    async fn find(&self, id: i64) -> Option<Note>;
}

struct MemoryRepo;

impl NoteRepository for MemoryRepo {
    async fn find(&self, id: i64) -> Option<Note> {
        (id == 1).then(|| Note {
            title: "買い物リスト".to_string(),
        })
    }
}

#[tokio::main]
async fn main() {
    // 実装を dyn（trait の型）として持とうとする
    let repo: Arc<dyn NoteRepository + Send + Sync> = Arc::new(MemoryRepo);
    if let Some(note) = repo.find(1).await {
        println!("{}", note.title);
    }
}

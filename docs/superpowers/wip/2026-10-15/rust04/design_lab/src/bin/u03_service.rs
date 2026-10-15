use std::fmt::Debug;

trait NoteRepository {
    fn titles(&self) -> Vec<String>;
}

#[derive(Debug)]
struct InMemory(Vec<String>);

impl NoteRepository for InMemory {
    fn titles(&self) -> Vec<String> {
        self.0.clone()
    }
}

// 保存先の型 R をパラメータとして持つサービス
struct NoteService<R> {
    repo: R,
}

// where 句: 「R が NoteRepository と Debug を実装しているなら」使えるメソッド
impl<R> NoteService<R>
where
    R: NoteRepository + Debug,
{
    fn new(repo: R) -> Self {
        Self { repo }
    }

    // 戻り値の impl Iterator: 「何かのイテレータ」を返す（具体的な型名は隠す）
    fn search(&self, word: &str) -> impl Iterator<Item = String> {
        self.repo
            .titles()
            .into_iter()
            .filter(move |t| t.contains(word))
    }

    fn debug_repo(&self) -> String {
        format!("{:?}", self.repo)
    }
}

fn main() {
    let repo = InMemory(vec![
        "牛乳を買う".to_string(),
        "本を返す".to_string(),
        "牛乳パックを捨てる".to_string(),
    ]);
    let service = NoteService::new(repo);
    for title in service.search("牛乳") {
        println!("見つかった: {title}");
    }
    println!("{}", service.debug_repo());
}

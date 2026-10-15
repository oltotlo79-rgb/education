use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(i64);

// &self で使え、スレッドをまたいで共有できる保存先の約束
trait NoteRepository {
    fn insert(&self, owner: UserId, title: &str) -> Result<i64, String>;
    fn count(&self) -> usize;
    fn kind(&self) -> &'static str;
}

// ---- メモリ版 ----
#[derive(Default)]
struct InMemoryNoteRepository {
    titles: Mutex<Vec<(UserId, String)>>,
}

impl NoteRepository for InMemoryNoteRepository {
    fn insert(&self, owner: UserId, title: &str) -> Result<i64, String> {
        let mut titles = self.titles.lock().unwrap();
        titles.push((owner, title.to_string()));
        Ok(titles.len() as i64)
    }

    fn count(&self) -> usize {
        self.titles.lock().unwrap().len()
    }

    fn kind(&self) -> &'static str {
        "memory"
    }
}

// ---- 読み取り専用版（メンテナンス中を想定）----
struct ReadOnlyRepository;

impl NoteRepository for ReadOnlyRepository {
    fn insert(&self, _owner: UserId, _title: &str) -> Result<i64, String> {
        Err("ただいまメンテナンス中のため保存できません".to_string())
    }

    fn count(&self) -> usize {
        0
    }

    fn kind(&self) -> &'static str {
        "read-only"
    }
}

// 共有する型に名前を付けておく
type DynRepo = Arc<dyn NoteRepository + Send + Sync>;

// アプリ全体で共有する状態
#[derive(Clone)]
struct AppState {
    repo: DynRepo,
}

// 設定の文字列を見て、実行時に実装を選ぶ
fn build_repo(kind: &str) -> Result<DynRepo, String> {
    match kind {
        "memory" => Ok(Arc::new(InMemoryNoteRepository::default())),
        "read-only" => Ok(Arc::new(ReadOnlyRepository)),
        other => Err(format!("知らない保存先です: {other}")),
    }
}

fn main() {
    for kind in ["memory", "read-only", "postgres"] {
        let repo = match build_repo(kind) {
            Ok(repo) => repo,
            Err(e) => {
                println!("起動失敗: {e}");
                continue;
            }
        };
        let state = AppState { repo };

        // 2 つのスレッドで同じ状態を使う（Web サーバーの同時リクエストの代わり）
        let mut handles = Vec::new();
        for user in 1..=2 {
            let state = state.clone();
            handles.push(thread::spawn(move || {
                state.repo.insert(UserId(user), "牛乳を買う")
            }));
        }
        for h in handles {
            let result = h.join().unwrap();
            println!("[{}] {:?}", state.repo.kind(), result);
        }
        println!("[{}] 件数 = {}", state.repo.kind(), state.repo.count());
    }
}

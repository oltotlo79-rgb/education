use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UserId(i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NoteId(i64);

#[derive(Debug, Clone)]
struct Note {
    id: NoteId,
    owner: UserId,
    title: String,
}

// 保存に失敗したことを表す型（DB に接続できない、など）
#[derive(Debug)]
struct RepoError(String);

// ノートの保存場所に求める「約束」
trait NoteRepository {
    fn insert(&self, owner: UserId, title: &str) -> Result<Note, RepoError>;
    fn list_by_owner(&self, owner: UserId) -> Result<Vec<Note>, RepoError>;
}

// ---- 実装1: メモリの中に保存する ----
struct Inner {
    next_id: i64,
    notes: Vec<Note>,
}

struct InMemoryNoteRepository {
    // &self のまま中身を書き換えられるよう Mutex で包む（第3巻）
    inner: Mutex<Inner>,
}

impl InMemoryNoteRepository {
    fn new() -> Self {
        let inner = Inner {
            next_id: 1,
            notes: Vec::new(),
        };
        Self {
            inner: Mutex::new(inner),
        }
    }
}

impl NoteRepository for InMemoryNoteRepository {
    fn insert(&self, owner: UserId, title: &str) -> Result<Note, RepoError> {
        let mut inner = self.inner.lock().unwrap();
        let note = Note {
            id: NoteId(inner.next_id),
            owner,
            title: title.to_string(),
        };
        inner.next_id += 1;
        inner.notes.push(note.clone());
        Ok(note)
    }

    fn list_by_owner(&self, owner: UserId) -> Result<Vec<Note>, RepoError> {
        let inner = self.inner.lock().unwrap();
        let mine = inner
            .notes
            .iter()
            .filter(|n| n.owner == owner)
            .cloned()
            .collect();
        Ok(mine)
    }
}

// ---- 実装2: いつも失敗する偽物（テスト用の Fake）----
struct BrokenRepository;

impl NoteRepository for BrokenRepository {
    fn insert(&self, _owner: UserId, _title: &str) -> Result<Note, RepoError> {
        Err(RepoError("DB に接続できません".to_string()))
    }

    fn list_by_owner(&self, _owner: UserId) -> Result<Vec<Note>, RepoError> {
        Err(RepoError("DB に接続できません".to_string()))
    }
}

// ユースケース「ノートを作る」。保存先が何なのかは知らない
fn create_note(repo: &impl NoteRepository, owner: UserId, title: &str) -> Result<NoteId, String> {
    let title = title.trim();
    if title.is_empty() {
        return Err("タイトルが空です".to_string());
    }
    let note = repo
        .insert(owner, title)
        .map_err(|e| format!("保存に失敗しました: {}", e.0))?;
    Ok(note.id)
}

fn main() {
    let repo = InMemoryNoteRepository::new();
    let alice = UserId(1);
    let bob = UserId(2);

    println!("{:?}", create_note(&repo, alice, "牛乳を買う"));
    println!("{:?}", create_note(&repo, bob, "本を返す"));
    println!("{:?}", create_note(&repo, alice, "   "));
    println!("{:?}", create_note(&repo, alice, "電気代を払う"));

    for note in repo.list_by_owner(alice).unwrap() {
        println!("alice のノート: {:?} {}", note.id, note.title);
    }

    // 同じユースケースに、壊れた保存先を渡してみる
    let broken = BrokenRepository;
    println!("{:?}", create_note(&broken, alice, "牛乳を買う"));
}

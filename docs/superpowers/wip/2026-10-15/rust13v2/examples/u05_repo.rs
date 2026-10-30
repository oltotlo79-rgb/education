//! 第13巻 UNIT 05: 3つのリポジトリを、一時フォルダの本物の DB で、AppState を通して動かす。
//! 実行: cargo run -q --example u05_repo

use std::time::{Duration, SystemTime};

use rustnote::db::{self, Db};
use rustnote::domain::{Email, NewNote, NoteId, NoteQuery};
use rustnote::error::RepoError;
use rustnote::state::AppState;
use topcoat::session::TokenHash;

/// RepoError の種類の名前（中身は短くして表示する）
fn kind(error: &RepoError) -> String {
    match error {
        RepoError::UniqueViolation(message) => format!("UniqueViolation({message})"),
        RepoError::VersionMismatch => "VersionMismatch".to_string(),
        RepoError::NotFound => "NotFound".to_string(),
        RepoError::InvalidData(message) => format!("InvalidData({message})"),
        RepoError::Database(error) => format!("Database({error})"),
    }
}

/// 正しいと分かっている入力から NewNote を作る（NoteInputErrors は Error を実装していないので ? を使わない）
fn new_note(title: &str, body: &str) -> NewNote {
    NewNote::parse(title, body).expect("正しい入力")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let db = Db::open_local(dir.path().join("u05.db")).await?;
    db::migrate(&db).await?;
    // 本番と同じ AppState。中の3つは Arc<dyn …Repository + Send + Sync>
    let state = AppState::new(db);

    println!("--- users");
    let alice_email = Email::parse("alice@example.com")?;
    // 本物は Argon2id のハッシュ（UNIT 06）。ここでは仮の文字列を入れる
    let alice = state.users.create(&alice_email, "$argon2id$dummy").await?;
    println!("登録: {alice:?}");
    let again = state.users.create(&alice_email, "$argon2id$dummy").await;
    println!("同じメールで登録: {:?}", again.map_err(|e| kind(&e)));
    let bob = state
        .users
        .create(&Email::parse("bob@example.com")?, "$argon2id$dummy")
        .await?;
    let record = state.users.find_by_email(&alice_email).await?;
    println!(
        "メールで探す: {:?}",
        record.map(|r| (r.user.id, r.password_hash))
    );

    println!("--- notes（持ち主ごと）");
    let note = state
        .notes
        .insert(alice.id, &new_note("買い物", "牛乳"))
        .await?;
    println!(
        "追加: id {}・version {}・done {}",
        note.id, note.version, note.done
    );
    println!(
        "alice が探す: {:?}",
        state.notes.find(alice.id, note.id).await?.map(|n| n.title)
    );
    println!(
        "bob が探す: {:?}",
        state.notes.find(bob.id, note.id).await?.map(|n| n.title)
    );
    let missing = NoteId::new(999)?;
    println!(
        "無い ID を探す: {:?}",
        state.notes.find(alice.id, missing).await?.map(|n| n.title)
    );

    println!("--- update（楽観的ロック）");
    let edit = new_note("買い物（土曜）", "牛乳と卵");
    let updated = state.notes.update(alice.id, note.id, &edit, 1).await?;
    println!(
        "version 1 で更新: title {:?}・version {}",
        updated.title, updated.version
    );
    let stale = state.notes.update(alice.id, note.id, &edit, 1).await;
    println!(
        "古い version 1 で更新: {:?}",
        stale.map(|n| n.version).map_err(|e| kind(&e))
    );
    let others = state.notes.update(bob.id, note.id, &edit, 2).await;
    println!(
        "bob が更新: {:?}",
        others.map(|n| n.version).map_err(|e| kind(&e))
    );
    let toggled = state.notes.toggle_done(alice.id, note.id).await?;
    println!(
        "完了に切り替え: done {}・version {}",
        toggled.done, toggled.version
    );
    // 編集の画面を version 2 のときに開いていた、という場面
    let after_toggle = state.notes.update(alice.id, note.id, &edit, 2).await;
    println!(
        "切り替えのあと、version 2 で更新: {:?}",
        after_toggle.map(|n| n.version).map_err(|e| kind(&e))
    );

    println!("--- search（キーワードとページ送り）");
    for i in 1..=23 {
        state
            .notes
            .insert(alice.id, &new_note(&format!("メモ {i}"), ""))
            .await?;
    }
    state
        .notes
        .insert(alice.id, &new_note("セール", "100%オフ"))
        .await?;
    state
        .notes
        .insert(alice.id, &new_note("値段", "1000円"))
        .await?;
    for (keyword, page) in [
        (None, None),
        (None, Some(2)),
        (Some("100%"), None),
        (Some("100"), None),
    ] {
        let found = state
            .notes
            .search(alice.id, &NoteQuery::new(keyword, page))
            .await?;
        let titles: Vec<&str> = found
            .notes
            .iter()
            .take(3)
            .map(|n| n.title.as_str())
            .collect();
        println!(
            "q={keyword:?} page={page:?} → 総数 {}・このページ {} 件・{}/{} ページ・先頭 {titles:?}",
            found.total_count,
            found.notes.len(),
            found.page,
            found.total_pages()
        );
    }
    let bob_page = state
        .notes
        .search(bob.id, &NoteQuery::new(None, None))
        .await?;
    println!("bob の一覧: 総数 {}", bob_page.total_count);

    println!("--- delete");
    println!(
        "1回目: {:?}",
        state
            .notes
            .delete(alice.id, note.id)
            .await
            .map_err(|e| kind(&e))
    );
    println!(
        "2回目: {:?}",
        state
            .notes
            .delete(alice.id, note.id)
            .await
            .map_err(|e| kind(&e))
    );

    println!("--- sessions");
    let now = SystemTime::now();
    let hour = Duration::from_secs(3600);
    let active = TokenHash::new([1; 32]);
    let expired = TokenHash::new([2; 32]);
    state.sessions.create(&active, alice.id, now + hour).await?;
    state.sessions.create(&expired, bob.id, now - hour).await?;
    let user = state.sessions.find_user(&active, now).await?;
    println!("期限内のトークン: {:?}", user.map(|u| u.email.to_string()));
    let user = state.sessions.find_user(&expired, now).await?;
    println!(
        "期限切れのトークン: {:?}",
        user.map(|u| u.email.to_string())
    );
    println!(
        "期限切れの掃除: {} 件",
        state.sessions.delete_expired(now).await?
    );
    state.sessions.delete(&active).await?;
    let user = state.sessions.find_user(&active, now).await?;
    println!("ログアウトのあと: {:?}", user.map(|u| u.email.to_string()));
    Ok(())
}

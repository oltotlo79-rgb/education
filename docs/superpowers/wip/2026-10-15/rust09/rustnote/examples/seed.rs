//! 開発用のお試しデータ（シード）を入れる。本番では使わない。
//! 実行: cargo run --example seed

use anyhow::Context;
use rustnote::{
    auth::hash_password,
    config::Config,
    db::{self, Db},
    domain::{Email, NewNote, Password},
    repository::{LibsqlNoteRepository, LibsqlUserRepository, NoteRepository, UserRepository},
};

/// お試し用のノート（タイトル, 本文）
const DEMO_NOTES: [(&str, &str); 3] = [
    ("買い物", "牛乳と卵"),
    ("Rust の復習", "第4巻 UNIT 01 を読み直す"),
    ("100% やる", ""),
];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env().context("設定を読み込めませんでした")?;
    // 本番（Turso）の DB には、お試しデータを入れない
    if config.is_remote() {
        anyhow::bail!("シードはローカルの DB にだけ入れます: {}", config.database_url);
    }
    let db = Db::open(&config).await.context("DB を開けませんでした")?;
    db::migrate(&db).await.context("マイグレーションに失敗しました")?;

    let users = LibsqlUserRepository::new(db.clone());
    let notes = LibsqlNoteRepository::new(db.clone());
    let email = Email::parse("demo@example.com")?;

    // 2 回目からは何もしない（何度実行しても同じ結果になるようにする）
    if users.find_by_email(&email).await?.is_some() {
        println!("{email} は登録済みなので、何もしません");
        return Ok(());
    }

    let password = Password::parse("demo-password")?;
    let hash = hash_password(&password).await?;
    let user = users.create(&email, &hash).await?;
    for (title, body) in DEMO_NOTES {
        let new_note = NewNote::parse(title, body).expect("見本のノートは検査に合格する");
        let note = notes.insert(user.id, &new_note).await?;
        println!("ノートを作りました: id={} {}", note.id.get(), note.title);
    }
    println!("ログイン: {email} / demo-password");
    Ok(())
}

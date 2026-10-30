//! 見本のデータを入れる前に、マイグレーションを呼び忘れた
use rustnote::{
    auth::hash_password,
    config::Config,
    db::Db,
    domain::{Email, Password},
    repository::{LibsqlUserRepository, UserRepository},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = Db::open(&Config::from_env()?).await?;
    // db::migrate(&db).await?; を書き忘れた
    let users = LibsqlUserRepository::new(db.clone());
    let hash = hash_password(&Password::parse("correct horse battery")?).await?;
    users.create(&Email::parse("alice@example.com")?, &hash).await?;
    println!("alice@example.com を入れました");
    Ok(())
}

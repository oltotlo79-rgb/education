//! 第9巻 UNIT 07: 開発用の見本のデータ（seed）を入れる。本番の DB には入れない。
//! 実行: $env:DATABASE_URL = "dev.db"; cargo run --example u07_seed

use rustnote::{
    auth::hash_password,
    config::Config,
    db::{self, Db},
    domain::{Email, NewNote, Password},
    error::RepoError,
    repository::{LibsqlNoteRepository, LibsqlUserRepository, NoteRepository, UserRepository},
};

/// 見本のユーザーと、そのノートのタイトル
const SAMPLES: &[(&str, &[&str])] = &[
    (
        "alice@example.com",
        &["牛乳を買う", "本を返す", "歯医者の予約"],
    ),
    ("bob@example.com", &["会議の資料を作る"]),
];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    // ネットワークの向こうの DB（本番の Turso など）には、見本のデータを入れない
    if config.is_remote() {
        anyhow::bail!(
            "{} はリモートの DB です。見本のデータは、手元の DB にだけ入れます",
            config.database_url
        );
    }
    let db = Db::open(&config).await?;
    // 本番と同じマイグレーションで、表を作る（済んでいれば何もしない）
    let applied = db::migrate(&db).await?;
    println!("適用したマイグレーション: {applied:?}");

    let users = LibsqlUserRepository::new(db.clone());
    let notes = LibsqlNoteRepository::new(db.clone());
    // パスワードは、アプリと同じ関数でハッシュにする（SQL に直接書かない）
    let password = Password::parse("correct horse battery")?;

    for (email, titles) in SAMPLES {
        let hash = hash_password(&password).await?;
        match users.create(&Email::parse(email)?, &hash).await {
            Ok(user) => {
                for title in *titles {
                    // 見本のタイトルは、ここで決めた正しい値なので、検査に落ちることはない
                    let note = NewNote::parse(title, "").expect("見本のタイトルが不正です");
                    notes.insert(user.id, &note).await?;
                }
                println!("{email} を作り、ノートを {} 件入れました", titles.len());
            }
            // もういれば飛ばす（何度動かしても、同じ状態になる）
            Err(RepoError::UniqueViolation(_)) => println!("{email} はもういるので、飛ばします"),
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}
